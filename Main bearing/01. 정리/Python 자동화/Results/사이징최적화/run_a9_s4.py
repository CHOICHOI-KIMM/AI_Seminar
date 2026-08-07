"""
부록 9 S4-d — `D` 상한 3건 피로수명 무인 실행기 (§9-10)
==========================================================
`run_a8_s4.py` 와 같은 구조이고 phase 만 `8`(50 °C) · `9`(70 °C)다.
설계 1건이 끝날 때마다 §9-10 표가 갱신된다.

  0  목록      `a9_targets.py`            (완료)
  1  상수      `probe_p2_constants.py 8`  a·Y1·e·C·C_u — 코너 반경 반영
  2  스크리닝  `run_p2_screen.py 8|9`     DLC별 k
  3  피로      `run_p2_fatigue.py 8|9`    111 DLC × dt=20

  python run_a9_s4.py        50 °C → 70 °C 연속 (약 40분)
"""
import os
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "P2_피로수명_A9")
LOG = os.path.join(OUT, "run_a9_s4.log")
STEPS = [("상수 실측", "probe_p2_constants.py", "8"),
         ("스크리닝 50°C", "run_p2_screen.py", "8"),
         ("피로해석 50°C", "run_p2_fatigue.py", "8"),
         ("스크리닝 70°C", "run_p2_screen.py", "9"),
         ("피로해석 70°C", "run_p2_fatigue.py", "9")]


def run(name, script, ph):
    print(f"\n{'='*66}\n[{name}] {script} {ph}  {time.strftime('%H:%M:%S')}",
          flush=True)
    with open(LOG, "a", encoding="utf-8") as f:
        f.write(f"\n{'='*66}\n[{name}] {time.strftime('%Y-%m-%d %H:%M:%S')}\n")
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
    print(f"\n[S4-d 부록 9] 전 단계 완료 · 총 "
          f"{(time.perf_counter()-t0)/60:.1f}분")
    return 0


if __name__ == "__main__":
    sys.exit(main())
