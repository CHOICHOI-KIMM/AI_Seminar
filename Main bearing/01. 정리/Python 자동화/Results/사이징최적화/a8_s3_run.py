"""
부록 8 S3 — 두께 규칙을 반영한 재최적화 (부록 6 스크립트 재사용)
===================================================================
부록 6 의 `nsga_s3_run.py` 를 **그대로 쓰고 두 곳만 갈아끼운다.**

  · 평가기   `nsga_eval.Evaluator` → `a8_eval.Evaluator`
             (샤프트 내경 두께 규칙 · 2β 기록)
  · 산출     `부록6_NSGA/` → `부록8_NSGA/`

설계변수·제약·목적함수·파라미터(224 × 150 · 밴드 재매개화 · 정수화 · 범주 전수
시딩 · 중복제거 끔 · 세대수 고정)는 **손대지 않는다.** §6-11.5 의 as-run 구성이
그대로 적용된다.

사용법
  python a8_s3_run.py dry     드라이런 224×5 (S3-b)
  python a8_s3_run.py         본 최적화 224×150 (S3-c)
"""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import a8_eval                     # noqa: E402
import nsga_s3_run as S3           # noqa: E402

S3.ne = a8_eval                    # 평가기 교체 (geom·key_of 는 재노출됨)
S3.OUTROOT = "부록8_NSGA"

# 사전 게이트의 기댓값 — 부록 6 값을 그대로 쓰면 **당연히** 걸린다. §8-3 의 변경
# 셋이 v1.3 기준선에서 σ 와 샤프트 질량을 바꾸기 때문이다(§8-3.1 분해 표).
#   σ            3,424.2 → 3,407.9  (두께 규칙 −63.5 · 코너 반경 +47.2)
#   샤프트 질량  43,225.8 → 58,384.5 kg (보어 3,055 는 교차점 3,328 보다 작다)
# 게이트를 푸는 것이 아니라 **새 값으로 다시 못박는다** — 나머지 여섯 항목은
# 부록 6 값 그대로이므로 배선이 어긋나면 여전히 여기서 걸린다.
S3.BASE_EXP = dict(S3.BASE_EXP, sigma_max_MPa=3407.9, mass_shaft_kg=58384.5)

if __name__ == "__main__":
    sys.exit(S3.main())
