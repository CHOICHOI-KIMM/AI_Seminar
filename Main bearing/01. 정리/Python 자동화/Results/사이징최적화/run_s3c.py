"""
부록 6 S3-c — 본 최적화 무인 실행기
======================================
`nsga_s3_run.py` (드라이런 아님) 를 자식 프로세스로 돌린다. 약 2.76시간이
걸리므로 사람이 지켜보지 않는 실행을 전제로 한다.

  0  정상 종료
  4  v1.3 기준선 불일치 (모델·설정 변질) → 재시도하지 않는다
  그 외 (예외·크래시·MASTA 세션 끊김)  → **1회만** 재시도한다

**재시도가 싼 이유는 `eval_cache.csv` 다.** 이미 평가한 설계는 MASTA 를
다시 부르지 않으므로, 2차 시도는 끊긴 지점까지를 초 단위로 재생한 뒤
거기서부터 실제 계산을 잇는다. S3-b 에서 실증됐다 — 재실행 시 MASTA 0회로
세대별 하이퍼볼륨이 완전히 동일하게 재현됐다(§6-11.4).

진행 상황은 `s3_genlog.csv` 에 세대마다 기록되고, 집단은 매 세대
`s3_checkpoint.csv` 로 덤프된다. 로그 전량은 `run_s3c.log`.
"""
import os
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "부록6_NSGA", "S3_본최적화")
LOG = os.path.join(OUT, "run_s3c.log")
TAIL = 30
NO_RETRY = {0: "정상 종료", 3: "게이트 불합격", 4: "기준선 불일치"}


def attempt(n):
    with open(LOG, "a", encoding="utf-8") as f:
        f.write(f"\n{'='*70}\n[시도 {n}] {time.strftime('%Y-%m-%d %H:%M:%S')}\n")
        f.flush()
        env = dict(os.environ, PYTHONIOENCODING="utf-8", PYTHONUNBUFFERED="1")
        p = subprocess.run([sys.executable, "nsga_s3_run.py"],
                           cwd=HERE, env=env, stdout=f,
                           stderr=subprocess.STDOUT)
    return p.returncode


def tail(n=TAIL):
    if not os.path.isfile(LOG):
        return "(로그 없음)"
    with open(LOG, encoding="utf-8", errors="replace") as f:
        return "".join(f.readlines()[-n:])


def main():
    os.makedirs(OUT, exist_ok=True)
    t0 = time.perf_counter()
    rc = attempt(1)
    if rc not in NO_RETRY:
        print(f"[재시도] 1차 실패 (exit {rc}) — 캐시를 살려 1회만 다시 돈다",
              flush=True)
        rc = attempt(2)

    print(tail())
    dt = time.perf_counter() - t0
    print(f"\n{'='*70}")
    print(f"[S3-c] {NO_RETRY.get(rc, f'실행 실패 (exit {rc})')} · "
          f"총 {dt/3600:.2f}시간 · 로그 {LOG}")
    return rc


if __name__ == "__main__":
    sys.exit(main())
