"""
S4 — 파레토 프론트 40건 피로수명 검토 (무인 실행기)
=====================================================
§6-11.5a·b 네 표의 설계 40건에 P2 본해석을 건다. 기존 P2 3단 파이프라인을
`Phase 4` 로 재사용한다.

  ① probe_p2_constants.py 4   MASTA 로 a·Y1·e·C·C_u 실측 → p2d_constants.csv
  ② run_p2_screen.py 4        핀지지 강체정역학으로 DLC별 k 선정 (MASTA 불필요)
  ③ run_p2_fatigue.py 4       111 DLC × dt=20 MASTA 피로 → fatigue_summary.csv

**단계마다 산출물이 남고, ③ 은 (설계, DLC) 단위로 append 되므로** 중간에
끊겨도 다시 돌리면 남은 것부터 이어간다. 그래서 재시도는 1회만 둔다.

판정: ΣD30_UW ≤ 0.5 ∧ ΣD30_Sys ≤ 0.5 (30년 손상 · 직렬 와이블 e = 9/8)

로그: P2_피로수명_S4/run_s4.log
"""
import os
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
DIR = os.path.join(HERE, "P2_피로수명_S4")
LOG = os.path.join(DIR, "run_s4.log")
STEPS = (("상수 실측", "probe_p2_constants.py", "4"),
         ("스크리닝", "run_p2_screen.py", "4"),
         ("피로 본해석", "run_p2_fatigue.py", "4"))


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
            print(f"\n[S4] {name} 실패 (exit {rc}) — 중단 · 로그 {LOG}")
            return rc
        print(f"[S4] {name} 완료 · {(time.perf_counter()-t1)/60:.1f}분",
              flush=True)

    print(tail())
    print(f"\n{'='*70}")
    print(f"[S4] 3단 완료 · 총 {(time.perf_counter()-t0)/3600:.2f}시간 · "
          f"로그 {LOG}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
