"""
부록 6 S3-b — 드라이런 무인 실행기
=====================================
`nsga_s3_run.py dry` 를 자식 프로세스로 돌리고 결과를 판정한다.
사람이 지켜보지 않는 실행을 전제로 하므로 **실패의 종류를 구분**한다.

  0  통과 — 본런 진행 가능
  3  게이트 불합격 (계산·구현 문제)  → 재시도하지 않는다
  4  v1.3 기준선 불일치 (모델·설정 변질) → 재시도하지 않는다
  그 외 (예외·크래시·MASTA 세션 끊김)  → **1회만** 재시도한다

재시도가 싼 이유는 `eval_cache.csv` 다 — 이미 평가한 설계는 MASTA 를 다시
부르지 않으므로 두 번째 시도는 남은 부분만 돈다.

로그는 `부록6_NSGA/S3_드라이런/run_s3b.log` 에 전량 기록하고, 화면에는
꼬리 부분만 보여준다.
"""
import os
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "부록6_NSGA", "S3_드라이런")
LOG = os.path.join(OUT, "run_s3b.log")
TAIL = 40
NO_RETRY = {0: "통과", 3: "게이트 불합격", 4: "기준선 불일치"}


def attempt(n):
    with open(LOG, "a", encoding="utf-8") as f:
        f.write(f"\n{'='*70}\n[시도 {n}] {time.strftime('%Y-%m-%d %H:%M:%S')}\n")
        f.flush()
        env = dict(os.environ, PYTHONIOENCODING="utf-8", PYTHONUNBUFFERED="1")
        p = subprocess.run([sys.executable, "nsga_s3_run.py", "dry"],
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
    verdict = NO_RETRY.get(rc, f"실행 실패 (exit {rc})")
    print(f"\n{'='*70}")
    print(f"[S3-b] {verdict} · 총 {dt/60:.1f}분 · 로그 {LOG}")
    print(f"       {'본런(S3-c) 진행 가능' if rc == 0 else '본런 보류'}")
    return rc


if __name__ == "__main__":
    sys.exit(main())
