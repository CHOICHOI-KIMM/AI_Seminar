"""
부록 8 S3-c — 본 최적화 무인 실행기
=====================================
부록 6 의 `run_s3c.py` 와 **같은 실행기**이고, 자식 프로세스만 `a8_s3_run.py`
(평가기·산출 폴더 교체판)로 바뀐다. 약 2.96시간이 걸리므로 사람이 지켜보지
않는 실행을 전제로 한다.

  0  정상 종료
  3  드라이런 게이트 불합격      → 재시도하지 않는다
  4  v1.3 기준선 불일치           → 재시도하지 않는다
  그 외 (예외·크래시·MASTA 세션 끊김)  → **1회만** 재시도한다

**재시도가 싼 이유는 `eval_cache.csv` 다.** 이미 평가한 설계는 MASTA 를 다시
부르지 않으므로, 2차 시도는 끊긴 지점까지를 초 단위로 재생한 뒤 거기서부터
실제 계산을 잇는다(§6-11.4 에서 실증).

진행 상황은 `s3_genlog.csv` 에 세대마다 기록되고, 집단은 매 세대
`s3_checkpoint.csv` 로 덤프된다. 로그 전량은 `run_a8c.log`.
"""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import run_s3c as R                # noqa: E402

R.OUT = os.path.join(HERE, "부록8_NSGA", "S3_본최적화")
R.LOG = os.path.join(R.OUT, "run_a8c.log")
_orig = R.attempt


def attempt(n):
    """자식만 부록 8 스크립트로 바꾼다"""
    import subprocess
    import time
    with open(R.LOG, "a", encoding="utf-8") as f:
        f.write(f"\n{'='*70}\n[시도 {n}] {time.strftime('%Y-%m-%d %H:%M:%S')}\n")
        f.flush()
        env = dict(os.environ, PYTHONIOENCODING="utf-8", PYTHONUNBUFFERED="1")
        p = subprocess.run([sys.executable, "a8_s3_run.py"],
                           cwd=HERE, env=env, stdout=f,
                           stderr=subprocess.STDOUT)
    return p.returncode


R.attempt = attempt

if __name__ == "__main__":
    sys.exit(R.main())
