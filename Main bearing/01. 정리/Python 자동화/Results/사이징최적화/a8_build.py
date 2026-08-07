"""
부록 8 — 모델 구성 규칙 주입 (`import` 만으로 적용된다)
=========================================================
P2 파이프라인(`probe_p2_constants` · `run_p2_screen` · `run_p2_fatigue`)은
기하를 `sizing_geom` 의 세 함수로 만든다. 부록 8 은 그 셋만 감싸면 파이프라인
전체가 부록 8 규칙으로 돌아간다 — 스크립트 본문은 손대지 않는다.

  ① `sg.bearing`        → `nsga_eval.geom(…, integerize=True)`
       S3 가 실제로 MASTA 에 넣은 **정수화된** T·B·C·d·D 를 그대로 재현한다.
       원래 `sg.bearing` 은 T·B·C 를 반올림하지 않아 S3 결과와 어긋난다.
  ② `sg.shaft`          → 내경을 §7-6.7.5 두께 규칙으로 교체
       `floor((OD⁴ − 32·W·OD/π)^¼)` · `W` = 1.393×10⁹ mm³
  ③ `sg.apply_to_masta` → 주입 뒤 롤러 코너 반경 4.3 mm 를 좌·우에 넣는다

**③ 은 피로에 직접 들어간다.** 코너 반경이 유효 롤러 길이를 8.6 mm 줄이므로
기본동정격하중 `C` 가 낮아지고, 같은 하중에서 손상이 커진다. `C`·`C_u` 를
실측하는 상수 단계부터 적용돼야 하는 이유다.
"""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import a8_eval                    # noqa: E402
import nsga_eval as ne            # noqa: E402
import sizing_geom as sg          # noqa: E402

_ORIG = (sg.bearing, sg.shaft, sg.apply_to_masta)
_DONE = False


def _bearing(D_pw, alpha_deg, D_we, L_we):
    """`L_we` 인수는 **롤러 전장** `L_w` 다 (부록 1~7 의 이름 그대로)"""
    return ne.geom(D_pw, alpha_deg, D_we, L_we, True)


def _shaft(bore, z2):
    s = _ORIG[1](bore, z2)
    idm = a8_eval.shaft_id(bore)
    if idm is None:
        raise ValueError(f"두께 규칙 적용 불가 — bore {bore*1e3:,.0f} mm")
    s["inner_diameter"] = idm
    return s


def _apply(detail, g):
    bad = _ORIG[2](detail, g)
    for a in ("left_element_corner_radius", "right_element_corner_radius"):
        try:
            setattr(detail, a, a8_eval.R_CORNER)
        except Exception as e:
            bad.append(f"{a}:{str(e).splitlines()[0][:32]}")
    return bad


def patch():
    global _DONE
    if _DONE:
        return
    sg.bearing, sg.shaft, sg.apply_to_masta = _bearing, _shaft, _apply
    _DONE = True
    print(f"[부록 8] 모델 규칙 주입 — 두께 규칙 W {a8_eval.W52/1e9:.4f}e9 mm³ · "
          f"코너 반경 {a8_eval.R_CORNER*1e3:.1f} mm · 정수화 켬", flush=True)


patch()
