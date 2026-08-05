"""
§6-11.7 — 지정 설계의 70 °C MASTA 파일 저장
==============================================
S4-70C 대상 40건 중 **일부만** 골라 `.masta` 로 남긴다. 피로 결과는 이미
`fatigue_per_dlc.csv` 에 다 있으므로 **재해석 없이 모델만 재구성**한다 —
`run_p2_fatigue.py 5` 는 완료된 (설계, DLC) 를 건너뛰기 때문이다.

저장되는 상태
  · 해당 설계의 제원(베어링 · 샤프트 · 배치)
  · `Load Case 1` 의 베어링 온도 **70 °C** (샤프트·하우징 40 °C 유지)
  · 해석 결과는 제외 (`save(path, False)` — Phase 1~3 과 같다)

40건을 다 훑지 않도록 **지정 태그만 담은 임시 상수 파일**을 만들어 넘긴다.
`fatigue_summary.csv` 는 건드리지 않는다.

사용법
  python save_s4_70c_masta.py a01 a55 a61
"""
import csv
import os
import shutil
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
DIR = os.path.join(HERE, "P2_피로수명_S4_70C")
SRC_CONST = os.path.join(HERE, "P2_피로수명_S4", "p2d_constants.csv")
TMP_CONST = os.path.join(DIR, "_save_constants.csv")
BAK = os.path.join(DIR, "_fatigue_summary.bak.csv")
SUMMARY = os.path.join(DIR, "fatigue_summary.csv")
SCRIPT = os.path.join(HERE, "run_p2_fatigue.py")


def main(tags):
    with open(SRC_CONST, encoding="utf-8-sig") as f:
        rows = list(csv.DictReader(f))
    pick = [r for r in rows if r["rank_mass"] in tags]
    missing = set(tags) - {r["rank_mass"] for r in pick}
    if missing:
        raise SystemExit(f"대상에 없는 태그: {sorted(missing)}")

    os.makedirs(DIR, exist_ok=True)
    with open(TMP_CONST, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(pick)
    shutil.copy2(SUMMARY, BAK)          # summary 는 원상복구한다

    # run_p2_fatigue.py 5 를 상수 파일·저장 태그만 바꿔 호출
    env = dict(os.environ, PYTHONIOENCODING="utf-8", PYTHONUNBUFFERED="1",
               P2_CONST_OVERRIDE=TMP_CONST, P2_SAVE_TAGS=",".join(tags))
    rc = subprocess.run([sys.executable, SCRIPT, "5"], cwd=HERE, env=env).returncode

    shutil.move(BAK, SUMMARY)           # 동일값이지만 행 순서를 보존한다
    os.remove(TMP_CONST)
    print(f"\n[저장] {os.path.join(DIR, 'MASTA')} · exit {rc}")
    for t in tags:
        p = os.path.join(DIR, "MASTA", f"P2_design_{t}.masta")
        print(f"  {'O' if os.path.isfile(p) else 'x'} P2_design_{t}.masta"
              + (f"  {os.path.getsize(p)/1e6:.1f} MB"
                 if os.path.isfile(p) else ""))
    return rc


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:] or ["a01", "a55", "a61"]))
