"""
부록 8 S4-d — 프론트 14건 피로수명 무인 실행기 (§8-7)
========================================================
부록 6 S4 와 **같은 파이프라인**을 phase `6`(50 °C) · `7`(70 °C)로 돌린다.
모델 구성만 `a8_build` 가 부록 8 규칙(두께 규칙 · 코너 반경 4.3 · 정수화)으로
갈아끼운다.

  0  목록      `a8_targets.py`            (완료)
  1  상수      `probe_p2_constants.py 6`  a·Y1·e·C·C_u  — 코너 반경 반영
  2  스크리닝  `run_p2_screen.py 6|7`     DLC별 k (건당 7초)
  3  피로      `run_p2_fatigue.py 6|7`    111 DLC × dt=20

  python run_a8_s4.py 50     상수 → 스크리닝(50) → 피로(50)
  python run_a8_s4.py 70     스크리닝(70) → 피로(70)

각 단계는 체크포인트를 남기므로 중단 후 다시 부르면 이어서 돈다.
로그 전량은 `P2_피로수명_A8[_70C]/run_a8_s4.log`.
"""
import os
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
T = sys.argv[1] if len(sys.argv) > 1 else "50"
PH = "6" if T == "50" else "7"
OUT = os.path.join(HERE, "P2_피로수명_A8" + ("_70C" if PH == "7" else ""))
LOG = os.path.join(OUT, "run_a8_s4.log")
STEPS = ([("상수 실측", "probe_p2_constants.py", "6")] if PH == "6" else []) + [
    ("스크리닝", "run_p2_screen.py", PH),
    ("피로해석", "run_p2_fatigue.py", PH)]


def run(name, script, ph):
    print(f"\n{'='*68}\n[{T} °C] {name} — {script} {ph}  "
          f"{time.strftime('%H:%M:%S')}", flush=True)
    with open(LOG, "a", encoding="utf-8") as f:
        f.write(f"\n{'='*68}\n[{name}] {time.strftime('%Y-%m-%d %H:%M:%S')}\n")
        f.flush()
        env = dict(os.environ, PYTHONIOENCODING="utf-8", PYTHONUNBUFFERED="1")
        p = subprocess.run([sys.executable, script, ph], cwd=HERE, env=env,
                           stdout=f, stderr=subprocess.STDOUT)
    return p.returncode


def main():
    os.makedirs(OUT, exist_ok=True)
    t0 = time.perf_counter()
    for name, script, ph in STEPS:
        rc = run(name, script, ph)
        if rc != 0:
            print(f"  !! {name} 실패 (exit {rc}) — 중단. 로그 {LOG}")
            return rc
        print(f"  완료 ({(time.perf_counter()-t0)/60:.1f}분 누적)", flush=True)
    print(f"\n[S4-d {T} °C] 전 단계 완료 · 총 "
          f"{(time.perf_counter()-t0)/3600:.2f}시간")
    return 0


if __name__ == "__main__":
    sys.exit(main())
