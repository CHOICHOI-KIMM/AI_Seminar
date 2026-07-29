"""
사이징 최적화 — 종속변수 산출 단일 소스 (SSOT)
================================================
DLC기반_피로해석_사이징_최적화.md §4-3 종속변수 표의 구현체.
모든 탐색·검증 스크립트는 반드시 이 모듈을 통해 제원을 산출한다.
(스크립트마다 규칙이 달라지는 표류를 막기 위함 — 260728 통일)

자유변수 : D_pw, alpha, D_we, L_we, z1, z2
종속변수 : Z, t_i, t_o, d, D, T, B, C, 샤프트 OD/ID/길이, 케이지(자동)
"""
import math

# ── v1.3 실측에서 확정한 비율 상수 (결정 #7 · #14e · 260728 축방향 확정) ──
ETA = 0.92                    # 롤러 수 케이지 여유율 (v1.3 실증 87/94.61)
# 260729 개정 — 원뿔 테이퍼 반영 (diag_sigma_zero.py 진단)
#   구식은 '중앙면' 궤도반경에서 일정 두께를 뺐다. 테이퍼 롤러의 궤도는 원뿔이라
#   L_we 가 커지면 내륜 궤도가 소단에서 보어 아래로 내려가 형상이 무효가 되고
#   MASTA 가 최대응력을 반환하지 않았다(격자의 약 1/3). 실패 임계는 소단 벽두께
#   약 3 mm. 따라서 벽두께를 '소단(내륜)·대단(외륜)' 에서 정의하도록 바꾼다.
#   비율은 v1.3 실측(bore 3055 · OD 3600)에서 역산했고 검산이 정확히 재현된다.
# 260729 재개정 — 링 두께를 D_pw 가 아니라 링 폭 기준으로 (부록 4)
#   D_pw 기준은 작은 피치경에 큰 롤러를 올릴 때 JIS 관행(살두께 >= 0.20·D_we)을
#   지키지 못한다 — P1 격자 240종 중 12종 미달(부록 4-6). 링 폭 기준은 세장비
#   하한 1.5 와 결합해 t/D_we >= 0.274 를 구조적으로 보장한다.
TI_OVER_B = 0.15652           # 내륜 소단 벽두께 / B  (v1.3 실측 46.95/300)
TO_OVER_C = 0.17215           # 외륜 대단 벽두께 / C  (v1.3 실측 43.55/253)
T_OVER_LWE = 1.30226          # 조립폭 T / L_we
B_OVER_LWE = 1.26025          # 내륜폭 B / L_we  (= 0.96774 · T/L_we)
C_OVER_LWE = 1.06281          # 외륜폭 C / L_we  (= 0.81613 · T/L_we)
ID_OVER_OD = 0.88543          # 샤프트 내경 / 외경 (벽두께 비율)
SHAFT_TAIL = 0.5              # 샤프트 후단 여유 [m] — L_shaft = z2 + 0.5

# v1.3 출발점 (검산용)
V13 = dict(D_pw=3.3309, alpha=19.0, D_we=0.11051, L_we=0.238048,
           z1=0.5, z2=3.0)


def bearing(D_pw, alpha_deg, D_we, L_we):
    """자유변수 4개 → 베어링 종속제원 dict [m, 개]

    bore/OD 는 원뿔 궤도의 소단/대단에서 벽두께를 확보하도록 산출한다.
    궤도 반경은 중앙면에서 ±(L_we/2)·sinα 만큼 변하므로 지름으로는 L_we·sinα.
    """
    ra = math.radians(alpha_deg)
    ca, sa = math.cos(ra), math.sin(ra)
    T = T_OVER_LWE * L_we
    B = B_OVER_LWE * L_we
    C = C_OVER_LWE * L_we
    t_i, t_o = TI_OVER_B * B, TO_OVER_C * C
    d = round((D_pw - D_we * ca - L_we * sa - 2 * t_i) * 1000) / 1000   # 1mm (#11)
    D = round((D_pw + D_we * ca + L_we * sa + 2 * t_o) * 1000) / 1000
    Z = int(ETA * math.pi * D_pw / D_we)                       # floor
    return dict(D_pw=D_pw, alpha_deg=alpha_deg, D_we=D_we, L_we=L_we,
                t_i=t_i, t_o=t_o, bore=d, outer_diameter=D,
                width=T, inner_ring_width=B, outer_ring_width=C,
                number_of_elements=Z)


def shaft(bore, z2):
    """bore·z2 → 샤프트 종속제원 [m]"""
    return dict(outer_diameter=bore, inner_diameter=bore * ID_OVER_OD,
                length=z2 + SHAFT_TAIL)


def constraints(g, z1, z2, cone_deg=None):
    """제약 위반 목록 반환. cone_deg 인자는 하위호환용으로 남기며 사용하지 않는다.

    260729 정리 (문서 §4-4) — 격자 12,000 조합 전수 감사에서 한 번도 활성화되지
    않고 규칙상 자동 만족인 (C1)(C3)(C10)(C11) 을 삭제했다. 번호는 결번으로 두어
    기존 문서 참조를 보존한다. (C12) 는 JIS 관행 살두께 기준으로 신설.
    """
    D_we, L_we, T = g["D_we"], g["L_we"], g["width"]
    B, C = g["inner_ring_width"], g["outer_ring_width"]
    v = []
    # 260729 재조사 — 세장비 상한 4.0 → 2.5
    if not (1.5 <= L_we / D_we <= 2.5): v.append("C2 세장비 이탈")
    if z2 - z1 < 1.5:                v.append("C4 스팬 <1.5m")
    if z1 < 0.3:                     v.append("C5 z1 <0.3m")
    if g["D_pw"] > 4.5:              v.append("C6 D_pw >4500mm")
    if z2 - z1 < T + 0.1:            v.append("C7 축방향 간섭")
    if z1 - T / 2 < 0:               v.append("C8 UW 가 하중점 앞으로")
    if z2 + T / 2 > z2 + SHAFT_TAIL: v.append("C9 DW 샤프트 이탈")
    # C12 — 궤도륜 살두께 >= 전동체 지름의 20% (JIS 관행, Hertz 반무한체 가정)
    if min(TI_OVER_B * B, TO_OVER_C * C) < 0.20 * D_we:
        v.append("C12 살두께 <0.20 D_we")
    return v


def apply_to_masta(detail, g):
    """MASTA bearing.detail 에 주입 (링 폭 → T 순서 필수)"""
    seq = [("element_diameter", g["D_we"]), ("roller_length", g["L_we"]),
           ("bore", g["bore"]), ("outer_diameter", g["outer_diameter"]),
           ("inner_ring_width", g["inner_ring_width"]),
           ("outer_ring_width", g["outer_ring_width"]),
           ("width", g["width"]), ("number_of_elements", g["number_of_elements"])]
    bad = []
    for k, v in seq:
        try:
            setattr(detail, k, v)
        except Exception as e:
            bad.append(f"{k}:{str(e).splitlines()[0][:40]}")
    try:
        detail.pitch_circle_diameter = g["D_pw"]
    except Exception:
        pass
    try:
        detail.contact_angle = math.radians(g["alpha_deg"])
    except Exception:
        pass
    return bad


if __name__ == "__main__":
    g = bearing(**{k: V13[k] for k in ("D_pw", "D_we", "L_we")},
                alpha_deg=V13["alpha"])
    print("v1.3 검산 (실측 대비)")
    exp = dict(bore=3.055, outer_diameter=3.600, width=0.310,
               inner_ring_width=0.300, outer_ring_width=0.253,
               number_of_elements=87)
    for k, e in exp.items():
        a = g[k]
        print(f"  {k:20} = {a*1e3 if k!='number_of_elements' else a:10.3f}"
              f"   실측 {e*1e3 if k!='number_of_elements' else e:10.3f}"
              f"   {'OK' if abs(a-e) < (0.6e-3 if k!='number_of_elements' else 0.5) else '!! 불일치'}")
    s = shaft(g["bore"], V13["z2"])
    print(f"  샤프트 OD {s['outer_diameter']*1e3:.1f} / ID {s['inner_diameter']*1e3:.1f}"
          f" / L {s['length']*1e3:.0f}   실측 3055.0 / 2705.0 / 3500")
    print("  제약 위반:", constraints(g, V13["z1"], V13["z2"], cone_deg=17.8) or "없음")
