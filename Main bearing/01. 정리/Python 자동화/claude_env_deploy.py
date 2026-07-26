#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""
claude_env_deploy.py

claude_env_export_*/ 아카이브를 현재 컴퓨터에 복원(배포)한다.
_manifest.json 의 items 정의를 읽어 payload -> dest 로 복사하며,
덮어쓰기 전에 기존 파일/폴더를 항상 백업한다.

사용법:
    python claude_env_deploy.py <export폴더>                  # 미리보기(dry-run, 기본값)
    python claude_env_deploy.py <export폴더> --apply          # 실제 배포
    python claude_env_deploy.py <export폴더> --apply --replace
    python claude_env_deploy.py <export폴더> --apply --only skills,settings
    python claude_env_deploy.py <export폴더> --list

기본 동작:
  - dry-run: --apply 를 붙이지 않으면 아무것도 변경하지 않고 계획만 출력한다.
  - 백업: 덮어쓰는 대상이 이미 존재하면 --backup-dir 아래에 원본 그대로 보존한다.
          (기본 위치: %USERPROFILE%\\.claude\\backups\\env_deploy_<타임스탬프>)
  - 폴더 병합(merge, 기본): 같은 이름 파일만 덮어쓰고 대상에만 있는 파일은 남긴다.
  - 폴더 교체(--replace): 대상 폴더를 통째로 지우고 payload 내용으로 대체한다.
"""

import argparse
import json
import os
import shutil
import sys
from datetime import datetime
from pathlib import Path

if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")

MANIFEST_NAME = "_manifest.json"
BACKUP_MANIFEST = "_restore_info.json"


# ---------------------------------------------------------------- 유틸

def expand(path_str):
    """%USERPROFILE% 등 환경변수를 확장한 절대경로 Path 반환."""
    return Path(os.path.expandvars(os.path.expanduser(str(path_str)))).resolve()


def count_files(path):
    """경로 하위 파일 수. 파일이면 1, 없으면 0."""
    p = Path(path)
    if p.is_file():
        return 1
    if p.is_dir():
        return sum(1 for f in p.rglob("*") if f.is_file())
    return 0


def dir_size(path):
    """경로 하위 총 바이트."""
    p = Path(path)
    if p.is_file():
        return p.stat().st_size
    if p.is_dir():
        return sum(f.stat().st_size for f in p.rglob("*") if f.is_file())
    return 0


def human(nbytes):
    for unit in ("B", "KB", "MB", "GB"):
        if nbytes < 1024 or unit == "GB":
            return "%.1f%s" % (nbytes, unit) if unit != "B" else "%dB" % nbytes
        nbytes /= 1024.0


def free_space(path):
    """대상 드라이브의 여유 공간(바이트). 존재하는 상위 폴더 기준."""
    p = Path(path)
    while not p.exists() and p.parent != p:
        p = p.parent
    try:
        return shutil.disk_usage(str(p)).free
    except OSError:
        return None


# ---------------------------------------------------------------- 계획 수립

class Action(object):
    """항목 1개에 대한 배포 계획."""

    def __init__(self, name, src, dest, kind, mode):
        self.name = name
        self.src = src            # payload 안의 원본 경로
        self.dest = dest          # 이 컴퓨터의 대상 경로
        self.kind = kind          # 'file' | 'dir'
        self.mode = mode          # 'merge' | 'replace'
        self.status = "OK"        # OK | SKIP
        self.reason = ""
        self.dest_exists = False
        self.n_src_files = 0
        self.src_bytes = 0

    @property
    def verb(self):
        if self.status == "SKIP":
            return "건너뜀"
        if not self.dest_exists:
            return "신규 생성"
        return "백업 후 교체" if self.mode == "replace" else "백업 후 병합"


def build_plan(export_dir, manifest, only, replace_dirs):
    """manifest items -> Action 목록."""
    actions = []
    for item in manifest.get("items", []):
        name = item.get("name", "?")
        if only and name not in only:
            continue

        src = (export_dir / item["arc"]).resolve()
        dest = expand(item["dest"])
        kind = item.get("type", "file")
        mode = "replace" if (replace_dirs and kind == "dir") else "merge"

        act = Action(name, src, dest, kind, mode)

        if not src.exists():
            act.status = "SKIP"
            act.reason = "payload 원본 없음: %s" % src
        elif kind == "dir" and not src.is_dir():
            act.status = "SKIP"
            act.reason = "type=dir 이지만 원본이 폴더가 아님"
        elif kind == "file" and not src.is_file():
            act.status = "SKIP"
            act.reason = "type=file 이지만 원본이 파일이 아님"
        else:
            act.n_src_files = count_files(src)
            act.src_bytes = dir_size(src)

        act.dest_exists = dest.exists()
        actions.append(act)

    return actions


# ---------------------------------------------------------------- 백업 / 복사

def backup_dest(act, backup_root, apply_changes):
    """
    덮어쓰기 전에 기존 dest 를 backup_root 안에 보존.
    반환: 백업 경로(Path) 또는 None(백업 대상 없음).
    """
    if not act.dest_exists:
        return None

    # 항목 이름으로 구분하고 원래 이름을 유지해 복원 시 헷갈리지 않게 한다.
    target = backup_root / act.name / act.dest.name

    if not apply_changes:
        return target

    target.parent.mkdir(parents=True, exist_ok=True)
    if act.dest.is_dir():
        shutil.copytree(str(act.dest), str(target), dirs_exist_ok=True, symlinks=True)
    else:
        shutil.copy2(str(act.dest), str(target))
    return target


def deploy_one(act, apply_changes):
    """실제 복사 수행. 복사된 파일 수 반환."""
    if not apply_changes:
        return act.n_src_files

    act.dest.parent.mkdir(parents=True, exist_ok=True)

    if act.kind == "file":
        shutil.copy2(str(act.src), str(act.dest))
        return 1

    # dir
    if act.mode == "replace" and act.dest.exists():
        shutil.rmtree(str(act.dest))
    shutil.copytree(str(act.src), str(act.dest), dirs_exist_ok=True, symlinks=True)
    return act.n_src_files


# ---------------------------------------------------------------- 출력

def print_plan(actions, export_dir, backup_root, apply_changes, replace_dirs):
    mode_label = "실제 배포 (--apply)" if apply_changes else "미리보기 (dry-run)"
    print("=" * 78)
    print("Claude 환경 배포  -  %s" % mode_label)
    print("=" * 78)
    print("원본 아카이브 : %s" % export_dir)
    print("백업 위치     : %s" % backup_root)
    print("폴더 처리방식 : %s" % ("교체(replace)" if replace_dirs else "병합(merge)"))
    print("-" * 78)
    print("%-10s %-14s %8s %9s  %s" % ("항목", "처리", "파일수", "크기", "대상 경로"))
    print("-" * 78)
    for a in actions:
        print("%-10s %-14s %8s %9s  %s"
              % (a.name, a.verb,
                 a.n_src_files if a.status == "OK" else "-",
                 human(a.src_bytes) if a.status == "OK" else "-",
                 a.dest))
        if a.status == "SKIP":
            print("%-10s   -> %s" % ("", a.reason))
    print("-" * 78)


def print_warnings(actions):
    """민감 정보 / 용량 관련 사전 경고."""
    warns = []
    for a in actions:
        if a.status != "OK":
            continue
        if a.src.name == ".env" or a.name == "dotenv":
            warns.append("%s: API 키가 평문으로 들어있다. 배포 후 %s 의 공유/커밋 여부를 확인할 것."
                         % (a.name, a.dest))
        if a.src_bytes > 100 * 1024 * 1024:
            warns.append("%s: %s 로 용량이 크다. 복사에 시간이 걸리고 백업까지 하면 2배를 쓴다."
                         % (a.name, human(a.src_bytes)))
        if a.mode == "replace" and a.dest_exists:
            warns.append("%s: --replace 이므로 %s 의 기존 내용은 백업 후 전부 삭제된다."
                         % (a.name, a.dest))

    total = sum(a.src_bytes for a in actions if a.status == "OK")
    backup_need = sum(a.src_bytes for a in actions if a.status == "OK" and a.dest_exists)
    free = free_space(Path.home())
    if free is not None and free < (total + backup_need) * 1.2:
        warns.append("여유 공간 부족 가능: 필요 약 %s, 여유 %s"
                     % (human(total + backup_need), human(free)))

    if warns:
        print("[주의]")
        for w in warns:
            print("  - %s" % w)
        print("-" * 78)


# ---------------------------------------------------------------- 메인

def main():
    ap = argparse.ArgumentParser(
        description="claude_env_export_* 아카이브를 이 컴퓨터에 백업 후 덮어쓰기로 배포한다.")
    ap.add_argument("export_dir", help="_manifest.json 이 들어있는 export 폴더")
    ap.add_argument("--apply", action="store_true",
                    help="실제로 복사한다. 없으면 dry-run(변경 없음)")
    ap.add_argument("--replace", action="store_true",
                    help="폴더 항목을 병합하지 않고 통째로 교체한다(기존 파일 삭제)")
    ap.add_argument("--only", default="",
                    help="배포할 항목만 콤마로 지정 (예: skills,settings,plugins)")
    ap.add_argument("--skip", default="",
                    help="제외할 항목을 콤마로 지정 (예: sessions)")
    ap.add_argument("--backup-dir", default="",
                    help="백업 위치. 기본 %%USERPROFILE%%\\.claude\\backups\\env_deploy_<타임스탬프>")
    ap.add_argument("--no-backup", action="store_true",
                    help="백업을 생략한다(권장하지 않음)")
    ap.add_argument("--list", action="store_true",
                    help="manifest 항목만 출력하고 종료")
    ap.add_argument("--yes", "-y", action="store_true",
                    help="확인 프롬프트 없이 진행")
    args = ap.parse_args()

    export_dir = Path(args.export_dir).resolve()
    manifest_path = export_dir / MANIFEST_NAME
    if not manifest_path.is_file():
        print("[오류] %s 를 찾을 수 없다: %s" % (MANIFEST_NAME, manifest_path))
        return 2

    with open(str(manifest_path), "r", encoding="utf-8") as f:
        manifest = json.load(f)

    if args.list:
        print("생성 시각 : %s" % manifest.get("created", "?"))
        print("원본 홈   : %s" % manifest.get("source_user_home", "?"))
        print("파일 수   : %s" % manifest.get("n_files", "?"))
        for it in manifest.get("items", []):
            print("  %-10s %-28s -> %s" % (it.get("name"), it.get("arc"), it.get("dest")))
        return 0

    only = set(x.strip() for x in args.only.split(",") if x.strip())
    skip = set(x.strip() for x in args.skip.split(",") if x.strip())

    actions = build_plan(export_dir, manifest, only, args.replace)
    if skip:
        for a in actions:
            if a.name in skip:
                a.status = "SKIP"
                a.reason = "--skip 으로 제외됨"

    if not actions:
        print("[오류] 배포할 항목이 없다. --only 값을 확인할 것.")
        return 2

    stamp = datetime.now().strftime("%y%m%d_%H%M%S")
    if args.backup_dir:
        backup_root = expand(args.backup_dir)
    else:
        backup_root = expand(r"%USERPROFILE%\.claude\backups") / ("env_deploy_%s" % stamp)

    if args.no_backup:
        backup_root_label = "(--no-backup: 백업 안 함)"
    else:
        backup_root_label = backup_root

    print_plan(actions, export_dir, backup_root_label, args.apply, args.replace)
    print_warnings(actions)

    todo = [a for a in actions if a.status == "OK"]
    if not todo:
        print("실행할 항목이 없다.")
        return 0

    if not args.apply:
        print("dry-run 이므로 아무것도 변경하지 않았다.")
        print("실제 배포하려면 동일 명령에 --apply 를 붙일 것.")
        return 0

    if not args.yes:
        overwrite = [a.name for a in todo if a.dest_exists]
        if overwrite:
            print("덮어쓸 항목: %s" % ", ".join(overwrite))
        try:
            answer = input("계속 진행? [y/N] ").strip().lower()
        except EOFError:
            answer = ""
        if answer not in ("y", "yes"):
            print("취소했다.")
            return 1

    # ---- 실행
    print()
    restore_info = {
        "created": datetime.now().strftime("%Y-%m-%d %H:%M:%S"),
        "export_dir": str(export_dir),
        "mode": "replace" if args.replace else "merge",
        "entries": [],
    }

    failed = []
    for a in todo:
        try:
            backup_path = None
            if not args.no_backup:
                backup_path = backup_dest(a, backup_root, True)
                if backup_path:
                    print("[백업] %-10s %s" % (a.name, backup_path))

            n = deploy_one(a, True)
            print("[배포] %-10s %s (%d개 파일)" % (a.name, a.dest, n))

            restore_info["entries"].append({
                "name": a.name,
                "dest": str(a.dest),
                "backup": str(backup_path) if backup_path else None,
                "existed_before": a.dest_exists,
                "mode": a.mode,
                "n_files": n,
            })
        except Exception as exc:  # 한 항목 실패가 나머지를 막지 않게 한다
            failed.append((a.name, exc))
            print("[실패] %-10s %s" % (a.name, exc))

    if not args.no_backup and restore_info["entries"]:
        backup_root.mkdir(parents=True, exist_ok=True)
        with open(str(backup_root / BACKUP_MANIFEST), "w", encoding="utf-8") as f:
            json.dump(restore_info, f, ensure_ascii=False, indent=1)

    print()
    print("=" * 78)
    ok = len(todo) - len(failed)
    print("완료: %d개 항목 배포, %d개 실패" % (ok, len(failed)))
    if failed:
        for name, exc in failed:
            print("  실패 %s: %s" % (name, exc))
    if not args.no_backup:
        print("백업 위치: %s" % backup_root)
        print("되돌리려면 %s 의 내용을 원래 dest 경로로 다시 복사할 것." % BACKUP_MANIFEST)
    print("Claude Code 를 재시작해야 skills/settings/plugins 변경이 반영된다.")
    print("=" * 78)

    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
