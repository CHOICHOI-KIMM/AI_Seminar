"""
부록 8 — 평가기 (부록 6 `nsga_eval` 계승 + 세 곳 변경)
=========================================================
해석 경로·정수화·캐시·자기검증은 부록 6 것을 그대로 쓴다(`nsga_eval.Evaluator`).
바뀌는 것은 아래 셋뿐이다.

  ① 샤프트 내경 — §7-6.7.5 의 두께 규칙
     `ID = floor((OD⁴ − 32·W·OD/π)^¼)` · `W = 1.393 × 10⁹ mm³`
     (목표 DIN 743 무한수명 안전율 5.2 · `floor` 는 §6-4.2 와 같은 안전측)
     현행 `ID = floor(0.88543·OD)` 를 대체한다.

  ② 롤러 코너 반경 — 좌·우 각 **4.3 mm** 주입
     그 결과 MASTA 가 계산하는 유효 롤러 길이가 `L_we = L_w − 8.6 mm` 가 된다.
     이 값을 읽어 **`L_we_mm`** 로 기록한다(주입값이 아니라 산출값).

  ③ 롤러 테이퍼각 2β — MASTA 자동 산출값을 읽어서 기록(`element_taper_angle`).
     주입하지 않으므로 해석 결과는 바뀌지 않는다. `cone_angle` 도 같이 뺀다.

명명
    `L_w`  롤러 전장 — **설계변수**. `u` 밴드 재매개화가 정하고 MASTA
           `roller_length` 로 주입된다. 세장비 제약 1.5 ≤ L_w/D_we ≤ 2.5 와
           T·B·C 비율은 모두 이 값을 기준으로 한다(부록 6 그대로).
    `L_we` 유효 롤러 길이 — **종속변수**. MASTA `effective_roller_length`.

    부록 1~7 스크립트는 `L_we` 라는 이름으로 롤러 전장을 다뤄 왔다(실제로는
    `roller_length` 주입). 그 40여 개 as-run 기록을 건드리지 않으려고 개명은
    부록 8 계층에서만 한다 — 기반 평가기가 넘긴 `L_we_mm` 을 `L_w_mm` 으로
    바꾸고, 새로 읽은 유효 길이를 `L_we_mm` 에 넣는다(`finish`).

**σ 가 오른다는 것을 전제로 한다.** §7-6.7.6 에서 기존 프론트 64건이 새 내경으로
전부 σ 2,100 을 넘었고(미스얼라인먼트 1.30배 · 상관 +0.995), 코너 반경도 접촉
길이를 8.6 mm 줄이는 같은 방향이다. 부록 8 의 최적화는 **응력을 낮추는 방향으로
설계를 다시 찾는 것**이 목적이다.
"""
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import nsga_eval as ne             # noqa: E402

W52 = 1.3930e9                     # mm³ — §7-6.7.5 목표 SF 5.2
R_CORNER = 0.0043                  # m — 롤러 좌·우 코너 반경

# 부록 6 스크립트가 `ne.<이름>` 으로 쓰는 것들 — 드롭인 대체가 되게 재노출한다
geom = ne.geom
key_of = ne.key_of
sc = ne.sc
LIMIT = ne.LIMIT


def shaft_id(bore_m):
    """두께 규칙 — 입력·출력 모두 m. `floor` 는 1 mm 단위(안전측)."""
    od = bore_m * 1e3
    inner = od ** 4 - 32 * W52 * od / math.pi
    if inner <= 0:                 # 물리적으로 불가능한 대구경 — 상위에서 걸린다
        return None
    return math.floor(inner ** 0.25) / 1e3


def _fields():
    """기반 열의 `L_we_mm` 을 `L_w_mm` 으로 바꾸고 종속 열 셋을 뒤에 붙인다"""
    out = ["L_w_mm" if c == "L_we_mm" else c for c in ne.FIELDS]
    return out + ["L_we_mm", "taper_2beta_deg", "cone_angle_deg"]


class Evaluator(ne.Evaluator):
    """부록 6 평가기 + 두께 규칙 + 코너 반경 + 유효 길이·테이퍼각 기록"""

    FIELDS = _fields()

    def shaft_of(self, bore, z2, integerize):
        s = ne.sg.shaft(bore, z2)
        idm = shaft_id(bore)
        if idm is None:
            raise ValueError(f"두께 규칙 적용 불가 — bore {bore*1e3:,.0f} mm")
        s["inner_diameter"] = idm
        return s

    def tweak(self, detail):
        for a in ("left_element_corner_radius", "right_element_corner_radius"):
            setattr(detail, a, R_CORNER)

    def finish(self, row, detail):
        row["L_w_mm"] = row.pop("L_we_mm")          # 기반이 준 값은 롤러 전장
        v = ne.sc(detail, "effective_roller_length")
        row["L_we_mm"] = round(v * 1e3, 2) if v is not None else None
        for col, attr in (("taper_2beta_deg", "element_taper_angle"),
                          ("cone_angle_deg", "cone_angle")):
            a = ne.sc(detail, attr)
            row[col] = round(math.degrees(a), 4) if a is not None else None
        return row


if __name__ == "__main__":        # 자기검증 — v1.3 기준선 1점
    ev = Evaluator(os.path.join(HERE, "부록8_NSGA", "_selftest"),
                   integerize=False)
    b = 3.055                     # v1.3 보어 [m]
    print(f"[두께 규칙] bore {b*1e3:,.0f} → ID {shaft_id(b)*1e3:,.0f} mm "
          f"(현행 {math.floor(b*1e3*ne.sg.ID_OVER_OD):,.0f})")
    r = ev.evaluate([(0.5, 3.0, 3.3309, 19.0, 0.11051, 0.238048)])[0]
    ev.close()
    print(f"  bore {r['bore_mm']} · D {r['D_mm']} · T {r['T_mm']} · Z {r['Z']}")
    print(f"  L_w {r['L_w_mm']} → L_we {r['L_we_mm']} mm "
          f"(세장비 {r['slenderness']} = L_w/D_we)")
    print(f"  σ {r['sigma_max_MPa']} MPa · 베어링 "
          f"{float(r['mass_brg_kg'])/1000:.3f} t · 샤프트 "
          f"{float(r['mass_shaft_kg'])/1000:.3f} t")
    print(f"  2β {r['taper_2beta_deg']}° · cone {r['cone_angle_deg']}°")
