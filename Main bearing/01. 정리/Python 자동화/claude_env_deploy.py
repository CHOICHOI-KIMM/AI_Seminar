"""
Claude Code 환경 deploy — 압축 푼 폴더의 파일을 대상 PC의 목적지로 자동 배치
==========================================================================
사용:  python claude_env_deploy.py <압축푼폴더경로> [--apply]
  · 기본은 dry-run (복사 계획만 출력) · --apply 로 실제 배치
  · 목적지 기존 파일/폴더는 .bak_YYMMDD 로 백업 후 교체
  · %USERPROFILE% 는 대상 PC 사용자로 런타임 치환 (사용자명 달라도 동작)
전제: 대상 PC 폴더 경로가 원본과 동일하게 생성되어 있음 (D:\\AI 등).
"""
import datetime
import json
import os
import shutil
import sys

DATE = datetime.datetime.now().strftime("%y%m%d")


def resolve(dest):
    return os.path.expandvars(dest)   # %USERPROFILE%, %VAR% 치환


def backup(path):
    base = f"{path}.bak_{DATE}"
    cand = base
    i = 1
    while os.path.exists(cand):
        cand = f"{base}_{i}"; i += 1
    os.rename(path, cand)
    return cand


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    apply = "--apply" in sys.argv
    if not args:
        print("사용: python claude_env_deploy.py <압축푼폴더경로> [--apply]")
        sys.exit(1)
    root = os.path.abspath(args[0])
    mpath = os.path.join(root, "_manifest.json")
    if not os.path.isfile(mpath):
        print(f"[오류] _manifest.json 없음: {mpath}")
        print("      압축을 푼 폴더(내부에 _manifest.json·payload\\ 존재)를 지정하세요.")
        sys.exit(1)
    meta = json.load(open(mpath, encoding="utf-8"))
    mode = "적용(APPLY)" if apply else "미리보기(DRY-RUN)"
    print(f"[deploy] {mode} · manifest {meta.get('created')} · {len(meta['items'])}개 항목\n")

    done = 0
    for it in meta["items"]:
        src = os.path.join(root, it["arc"].replace("/", os.sep))
        dest = resolve(it["dest"])
        exists = os.path.exists(dest)
        tag = "교체(백업)" if exists else "신규"
        print(f"  [{it['type']:4}] {it['name']:10} → {dest}   [{tag}]")
        if not os.path.exists(src):
            print(f"         ⚠ 압축 내 원본 없음: {src}")
            continue
        if not apply:
            continue
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        if exists:
            bak = backup(dest)
            print(f"         백업: {os.path.basename(bak)}")
        if it["type"] == "dir":
            shutil.copytree(src, dest)
        else:
            shutil.copy2(src, dest)
        done += 1

    if apply:
        print(f"\n[완료] {done}개 항목 배치")
        print("  다음: 대상 PC에서 claude 재로그인 · MASTA/파이썬 경로·.env 확인 후")
        print("        cd D:\\AI\\AI_Seminar  &&  claude --resume <세션ID>")
    else:
        print("\n[미리보기 종료] 실제 배치하려면 --apply 추가:")
        print(f"  python claude_env_deploy.py \"{root}\" --apply")


if __name__ == "__main__":
    main()
