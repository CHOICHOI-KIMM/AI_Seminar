"""
Claude Code 환경 export — 필수 파일을 한 zip으로 묶어 'Python 자동화' 폴더에 생성
==============================================================================
포함(승인 260726): skills · settings.json · plugins · projects\\d--AI-AI-Seminar(세션16+memory)
                    · D:\\AI\\CLAUDE.md · D:\\AI\\.env(GEMINI 키)
제외(인증·런타임): .credentials.json · cache · debug · sessions · shell-snapshots · session-env
비밀정보: settings.json·.env 그대로 포함 → ⚠ 산출 zip은 외부 공유 금지(.gitignore 처리)
산출: Python 자동화\\claude_env_export_YYMMDD.zip (+ 내부 _manifest.json)
대상 PC에서 claude_env_deploy.py 로 자동 배치.
"""
import datetime
import json
import os
import zipfile

USERHOME = os.path.expanduser("~")
CLAUDE = os.path.join(USERHOME, ".claude")
OUTDIR = os.path.dirname(os.path.abspath(__file__))
DATE = datetime.datetime.now().strftime("%y%m%d")
OUT = os.path.join(OUTDIR, f"claude_env_export_{DATE}.zip")

# (name, source_abs, dest_spec[%USERPROFILE% 치환], type)
ITEMS = [
    ("skills",    os.path.join(CLAUDE, "skills"),        r"%USERPROFILE%\.claude\skills", "dir"),
    ("settings",  os.path.join(CLAUDE, "settings.json"), r"%USERPROFILE%\.claude\settings.json", "file"),
    ("plugins",   os.path.join(CLAUDE, "plugins"),       r"%USERPROFILE%\.claude\plugins", "dir"),
    ("sessions",  os.path.join(CLAUDE, "projects", "d--AI-AI-Seminar"),
     r"%USERPROFILE%\.claude\projects\d--AI-AI-Seminar", "dir"),
    ("claude_md", r"D:\AI\CLAUDE.md", r"D:\AI\CLAUDE.md", "file"),
    ("dotenv",    r"D:\AI\.env",      r"D:\AI\.env",      "file"),
]


def main():
    manifest = []
    n_files = 0
    total = 0
    print(f"[export] → {OUT}")
    with zipfile.ZipFile(OUT, "w", zipfile.ZIP_DEFLATED, allowZip64=True) as z:
        for name, src, dest, typ in ITEMS:
            if not os.path.exists(src):
                print(f"  [건너뜀] 원본 없음: {src}")
                continue
            if typ == "file":
                arc = f"payload/{name}/{os.path.basename(src)}"
                z.write(src, arc)
                sz = os.path.getsize(src)
                total += sz; n_files += 1
                manifest.append({"name": name, "arc": arc, "dest": dest, "type": "file"})
                print(f"  [파일] {name}: {os.path.basename(src)} ({sz/1024:.0f} KB)")
            else:
                root = f"payload/{name}"
                cnt = 0; sub = 0
                for dp, _, fns in os.walk(src):
                    for fn in fns:
                        full = os.path.join(dp, fn)
                        rel = os.path.relpath(full, src).replace("\\", "/")
                        z.write(full, f"{root}/{rel}")
                        sub += os.path.getsize(full); cnt += 1
                total += sub; n_files += cnt
                manifest.append({"name": name, "arc": root, "dest": dest, "type": "dir"})
                print(f"  [폴더] {name}: {cnt}개 파일 ({sub/1048576:.1f} MB)")
        meta = {"created": datetime.datetime.now().strftime("%Y-%m-%d %H:%M"),
                "source_user_home": USERHOME, "n_files": n_files,
                "note": "deploy: python claude_env_deploy.py <이폴더> [--apply]",
                "items": manifest}
        z.writestr("_manifest.json", json.dumps(meta, ensure_ascii=False, indent=1))
    zsz = os.path.getsize(OUT)
    print(f"[완료] {n_files}개 파일 · 원본 {total/1048576:.0f} MB → 압축 {zsz/1048576:.0f} MB")
    print(f"       {OUT}")
    print("  ⚠ 비밀정보 포함 — 외부 공유·git 커밋 금지 (.gitignore 처리 권장)")


if __name__ == "__main__":
    main()
