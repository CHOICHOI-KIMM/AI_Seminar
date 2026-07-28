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
TI_OVER_DPW = 0.025674        # 내륜 반경두께 / D_pw
TO_OVER_DPW = 0.024654        # 외륜 반경두께 / D_pw
T_OVER_LWE = 1.30226          # 조립폭 T / L_we
B_OVER_T = 0.96774            # 내륜폭 B / T
C_OVER_T = 0.81613            # 외륜폭 C / T
ID_OVER_OD = 0.88543          # 샤프트 내경 / 외경 (벽두께 비율)
SHAFT_TAIL = 0.5              # 샤프트 후단 여유 [m] — L_shaft = z2 + 0.5

# v1.3 출발점 (검산용)
V13 = dict(D_pw=3.3309, alpha=19.0, D_we=0.11051, L_we=0.238048,
           z1=0.5, z2=3.0)


def bearing(D_pw, alpha_deg, D_we, L_we):
    """자유변수 4개 → 베어링 종속제원 dict [m, 개]"""
    ca = math.cos(math.radians(alpha_deg))
    t_i, t_o = TI_OVER_DPW * D_pw, TO_OVER_DPW * D_pw
    d = round((D_pw - D_we * ca - 2 * t_i) * 1000) / 1000     # 1 mm 반올림 (#11)
    D = round((D_pw + D_we * ca + 2 * t_o) * 1000) / 1000
    T = T_OVER_LWE * L_we
    B = B_OVER_T * T
    C = C_OVER_T * T
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
    """(C1)~(C11) 위반 목록 반환. cone_deg 미지정 시 alpha 로 근사."""
    D_we, L_we, T = g["D_we"], g["L_we"], g["width"]
    B, C, Z = g["inner_ring_width"], g["outer_ring_width"], g["number_of_elements"]
    cone = math.radians(cone_deg if cone_deg is not None else g["alpha_deg"])
    L_ax = L_we * math.cos(cone)                 # 롤러 축투영
    v = []
    if Z < 20:                       v.append("C1 롤러수 <20")
    if not (1.5 <= L_we / D_we <= 4.0): v.append("C2 세장비 이탈")
    if g["bore"] <= 0:               v.append("C3 bore<=0")
    if z2 - z1 < 1.5:                v.append("C4 스팬 <1.5m")
    if z1 < 0.3:                     v.append("C5 z1 <0.3m")
    if g["D_pw"] > 4.5:              v.append("C6 D_pw >4500mm")
    if z2 - z1 < T + 0.1:            v.append("C7 축방향 간섭")
    if z1 - T / 2 < 0:               v.append("C8 UW 가 하중점 앞으로")
    if z2 + T / 2 > z2 + SHAFT_TAIL: v.append("C9 DW 샤프트 이탈")
    if B < L_ax:                     v.append("C10 콘이 롤러 축투영 미포함")
    if C < L_ax:                     v.append("C11 컵이 롤러 축투영 미포함")
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
