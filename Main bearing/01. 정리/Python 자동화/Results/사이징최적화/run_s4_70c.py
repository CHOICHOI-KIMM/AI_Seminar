"""
§6-11.7 — 베어링 70 °C 피로수명 재검토 (무인 실행기)
=======================================================
S4 와 **같은 40건**을 베어링 온도만 50 → 70 °C 로 바꿔 다시 돌린다.

변경 지점은 두 곳뿐이다(`probe_temperature.py` 로 확정).

  ① MASTA  `Load Case 1`.temperatures 의 베어링 3항
           (element · inner_race · outer_race) 50 → 70
           — 이 LC 를 복제해 111 DLC 를 만들므로 여기만 바꾸면 전파된다.
             `Design.default_system_temperatures`(80 °C)는 이 LC 에 영향 없다.
  ② 스크리닝 상수  ν(50) 294.637 → ν(70) **137.178** mm²/s
           — `probe_nu70.py` 가 MASTA 결과에서 읽은 값(−53.4%)

**상수 실측(probe)은 생략한다.** `a`·`Y1`·`e`·`C`·`C_u` 는 기하와 정격이라
온도와 무관하므로 S4 의 `p2d_constants.csv` 를 그대로 쓴다. 온도가 바꾸는
것은 점도 → κ → `a_ISO` → 손상이다.

  ① run_p2_screen.py 5    ν(70) 로 DLC별 k 재선정
  ② run_p2_fatigue.py 5   LC 온도 70 °C · 111 DLC × dt=20 MASTA

로그: P2_피로수명_S4_70C/run_s4_70c.log
"""
import os
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
DIR = os.path.join(HERE, "P2_피로수명_S4_70C")
LOG = os.path.join(DIR, "run_s4_70c.log")
STEPS = (("스크리닝(70°C)", "run_p2_screen.py", "5"),
         ("피로 본해석(70°C)", "run_p2_fatigue.py", "5"))


def run(script, arg, tag):
    with open(LOG, "a", encoding="utf-8") as f:
        f.write(f"\n{'='*70}\n[{tag}] {script} {arg} · "
                f"{time.strftime('%Y-%m-%d %H:%M:%S')}\n")
        f.flush()
        env = dict(os.environ, PYTHONIOENCODING="utf-8", PYTHONUNBUFFERED="1")
        p = subprocess.run([sys.executable, script, arg], cwd=HERE, env=env,
                           stdout=f, stderr=subprocess.STDOUT)
    return p.returncode


def tail(n=25):
    if not os.path.isfile(LOG):
        return "(로그 없음)"
    with open(LOG, encoding="utf-8", errors="replace") as f:
        return "".join(f.readlines()[-n:])


def main():
    os.makedirs(DIR, exist_ok=True)
    t0 = time.perf_counter()
    for name, script, arg in STEPS:
        t1 = time.perf_counter()
        rc = run(script, arg, name)
        if rc != 0:
            print(f"[재시도] {name} 실패 (exit {rc}) — 산출물을 살려 1회 재실행",
                  flush=True)
            rc = run(script, arg, f"{name} 재시도")
        if rc != 0:
            print(tail())
            print(f"\n[70°C] {name} 실패 (exit {rc}) — 중단 · 로그 {LOG}")
            return rc
        print(f"[70°C] {name} 완료 · {(time.perf_counter()-t1)/60:.1f}분",
              flush=True)

    print(tail())
    print(f"\n{'='*70}")
    print(f"[70°C] 2단 완료 · 총 {(time.perf_counter()-t0)/3600:.2f}시간")
    return 0


if __name__ == "__main__":
    sys.exit(main())
