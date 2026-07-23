"""
G-8.2: 핀지지 정역학 기반 방법 C-1 — 반력 백데이터 + 수명 오차
================================================================
A. 백데이터: Point Load / Input Power Load 인가값 + UW·DW 전 반력성분(핀지지 vs MASTA) + 오차
B. 핀지지 정역학 값으로 C-1 수명(L10m, D, 등가수명) 산출 → MASTA 방법 B 대비 오차

핀지지 정역학 (§C-3.3, 기하 §C-3.6): 하중점 P@z=0, UW(A)@z=0.5, DW(B)@z=3.0
  L=2.5, a=z_P−z_A=−0.5, b=z_B−z_P=3.0
  R_A,X = F_X·(b/L) − M_Y/L      R_B,X = F_X·(a/L) + M_Y/L
  R_A,Y = F_Y·(b/L) + M_X/L      R_B,Y = F_Y·(a/L) − M_X/L
축분배(TRB쌍 유도축력, §C-3.3(2)): S=0.5·Fr/Y1, K_a=F_Z
  K_a ≥ S_A−S_B : Fa_A=+(K_a+S_B), Fa_B=−S_B
  K_a <  S_A−S_B : Fa_A=+S_A,       Fa_B=−(S_A−K_a)
"""
import csv
import glob
import math
import os
import statistics as st
import sys
import time

from c1_aiso import C_N, CU_N, E_C, DM_MM, NU, P_EXP, kappa, a_iso_radial_roller

DLC = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\260714 유니슨 피로하중\DLC1.2-c-s1.$150")
Z_P, Z_A, Z_B = 0.0, 0.5, 3.0
L, A_, B_ = Z_B - Z_A, Z_P - Z_A, Z_B - Z_P
E_LIM = 0.5165
Y1 = 0.4 / math.tan(math.atan(E_LIM / 1.5))     # 1.1617
SCALE_FACTOR = 45040.0
DESIGN_YEARS = 30.0
NAME_A, NAME_B = "Main Bearing_UW", "Main Bearing_DW"
N_BACK = 100        # 백데이터용 MASTA 해석 점수


def parse_dlc(path):
    rows = []
    for ln in open(path, encoding="latin-1").readlines()[4:]:
        p = ln.split()
        if len(p) < 8:
            continue
        try:
            v = [float(x) for x in p[:8]]
        except ValueError:
            continue
        rows.append(dict(t=v[0], rpm=v[1], Mx=v[2], My=v[3], Mz=v[4],
                         Fx=v[5], Fy=v[6], Fz=v[7]))
    return rows


def hub(rec):
    """시계열 → MASTA 인가하중 [N, N·m] (§4.2)."""
    return dict(FX=-rec["Fz"] * 1e3, FY=rec["Fy"] * 1e3, FZ=rec["Fx"] * 1e3,
                MX=-rec["Mz"] * 1e3, MY=rec["My"] * 1e3, MZ=rec["Mx"] * 1e3)


def pin_statics(rec):
    """→ {name: dict(RX,RY,RZ,Fr,Fa)} 핀지지 정역학 + TRB쌍 축분배."""
    u = hub(rec)
    rax = u["FX"] * (B_ / L) - u["MY"] / L
    ray = u["FY"] * (B_ / L) + u["MX"] / L
    rbx = u["FX"] * (A_ / L) + u["MY"] / L
    rby = u["FY"] * (A_ / L) - u["MX"] / L
    fra, frb = math.hypot(rax, ray), math.hypot(rbx, rby)
    sa, sb = 0.5 * fra / Y1, 0.5 * frb / Y1
    ka = u["FZ"]
    if ka >= sa - sb:
        faa, fab = ka + sb, -sb
    else:
        faa, fab = sa, -(sa - ka)
    return {NAME_A: dict(RX=rax, RY=ray, RZ=faa, Fr=fra, Fa=faa),
            NAME_B: dict(RX=rbx, RY=rby, RZ=fab, Fr=frb, Fa=fab)}


def equiv_load(fr, fa):
    if fr <= 0:
        return abs(fa) * Y1
    return fr if abs(fa) / fr <= E_LIM else 0.4 * fr + Y1 * abs(fa)


# ══════════════════ A. 백데이터 (MASTA) ══════════════════
def backdata(data):
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

    MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3"
             r"_샤프트 두께,형상 2안_피로하중 반영_유연체_FE_260721.masta")
    RPM2RADS = 2 * math.pi / 60
    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    pl = list(asm.all_parts_of_type_point_load())[0]
    bearings = {str(b): b for b in asm.all_parts_of_type_bearing()}
    ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
    lc = next(c for c in asm.design_properties.static_loads
              if getattr(c, "name", "") == "Load Case 1")

    idxs = [int(round(j * (len(data) - 1) / (N_BACK - 1))) for j in range(N_BACK)]
    out = []
    for i in idxs:
        rec = data[i]
        u = hub(rec)
        p = lc.inputs_for_point_load(pl)
        p.force_x.force = u["FX"]; p.force_y.force = u["FY"]
        p.axial_load.force = u["FZ"]
        p.moment_x.moment = u["MX"]; p.moment_y.moment = u["MY"]
        pll = lc.inputs_for_power_load(ipl)
        pll.speed = rec["rpm"] * RPM2RADS
        pll.torque = u["MZ"]
        sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
        sd.perform_analysis()
        m = {}
        for nm, b in bearings.items():
            f = sd.results_for(b).internal_force
            x, y, z = float(f.x), float(f.y), float(f.z)
            m[nm] = dict(RX=x, RY=y, RZ=z, Fr=math.hypot(x, y), Fa=z)
        out.append((i, rec, u, pin_statics(rec), m))
    return out


def report_backdata(bd):
    print("\n" + "=" * 100)
    print("A. 반력 백데이터 — 핀지지 정역학 vs MASTA")
    print("=" * 100)

    i, rec, u, pin, m = bd[0]
    print(f"\n[대표점 index={i}, t={rec['t']:.1f}s, rpm={rec['rpm']:.4f}]")
    print("\n  ── 인가 하중 ──")
    print(f"  Point Load        force_x     = {u['FX']:>15,.1f} N")
    print(f"                    force_y     = {u['FY']:>15,.1f} N")
    print(f"                    axial_load  = {u['FZ']:>15,.1f} N")
    print(f"                    moment_x    = {u['MX']:>15,.1f} N·m")
    print(f"                    moment_y    = {u['MY']:>15,.1f} N·m")
    print(f"  Input Power Load  speed       = {rec['rpm']*2*math.pi/60:>15.6f} rad/s "
          f"({rec['rpm']:.4f} rpm)")
    print(f"                    torque      = {u['MZ']:>15,.1f} N·m")

    print("\n  ── 반력 [kN] ──")
    print(f"  {'베어링':18} {'성분':>5} {'핀지지 정역학':>15} {'MASTA':>15} "
          f"{'차이':>13} {'오차%':>10}")
    print("  " + "-" * 82)
    for nm in (NAME_A, NAME_B):
        for c in ("RX", "RY", "RZ", "Fr", "Fa"):
            pv, mv = pin[nm][c], m[nm][c]
            err = (pv - mv) / mv * 100 if mv else float("nan")
            print(f"  {nm if c=='RX' else '':18} {c:>5} {pv/1e3:15,.1f} {mv/1e3:15,.1f} "
                  f"{(pv-mv)/1e3:13,.1f} {err:+10.2f}")
        print("  " + "-" * 82)

    print(f"\n  ── 힘평형 확인 [kN] ──")
    for c in ("RX", "RY", "RZ"):
        ps = pin[NAME_A][c] + pin[NAME_B][c]
        ms = m[NAME_A][c] + m[NAME_B][c]
        ap = u["FX"] if c == "RX" else (u["FY"] if c == "RY" else u["FZ"])
        print(f"  Σ{c}: 핀지지 {ps/1e3:10,.1f}   MASTA {ms/1e3:10,.1f}   "
              f"인가 {ap/1e3:10,.1f}")

    print(f"\n[전체 {len(bd)}점 오차 통계 — (핀지지−MASTA)/MASTA × 100]")
    print(f"  {'베어링':18} {'성분':>5} {'평균%':>10} {'중앙%':>10} {'최소%':>10} {'최대%':>10}")
    print("  " + "-" * 68)
    for nm in (NAME_A, NAME_B):
        for c in ("RX", "RY", "RZ", "Fr", "Fa"):
            e = [(p[nm][c] - mm[nm][c]) / mm[nm][c] * 100
                 for _, _, _, p, mm in bd if mm[nm][c]]
            print(f"  {nm if c=='RX' else '':18} {c:>5} {st.mean(e):+10.2f} "
                  f"{st.median(e):+10.2f} {min(e):+10.2f} {max(e):+10.2f}")
        print("  " + "-" * 68)


# ══════════════════ B. 핀지지 기반 C-1 수명 ══════════════════
def method_c1(data, idxs, weights):
    """→ {bearing: dict(Peq, n_eq, kappa, a_iso, L10, L10m, D, D30, life)}"""
    out = {}
    for nm in (NAME_A, NAME_B):
        num = den = spd = 0.0
        for i, w in zip(idxs, weights):
            rec = data[i]
            n = (rec["rpm"] / 60.0) * w          # 해당 점 회전수 [rev]
            s = pin_statics(rec)[nm]
            P = equiv_load(s["Fr"], s["Fa"])
            num += n * P ** P_EXP
            den += n
            spd += n * rec["rpm"]
        peq = (num / den) ** (1.0 / P_EXP)
        neq = spd / den
        kap = kappa(neq)
        ai = a_iso_radial_roller(kap, E_C * CU_N / peq)
        l10 = (C_N / peq) ** P_EXP * 1e6
        l10m = ai * l10
        D = den / l10m
        D30 = D * SCALE_FACTOR
        out[nm] = dict(Peq=peq, n_eq=neq, kappa=kap, a_iso=ai, L10=l10, L10m=l10m,
                       N=den, D=D, D30=D30, life=DESIGN_YEARS / D30 if D30 else float("inf"))
    return out


def ref_from_summary(path):
    """MASTA 방법 B(ISO281 modified) 표본손상·30년손상·등가수명."""
    out = {}
    for r in csv.reader(open(path, encoding="utf-8-sig")):
        if len(r) > 8 and r[1] == "ISO281" and r[2] == "modified":
            try:
                out[r[0]] = dict(D=float(r[3]), D30=float(r[5]), life=float(r[7]))
            except ValueError:
                pass
    return out


def main():
    data = parse_dlc(DLC)
    print(f"[시계열] {os.path.basename(DLC)}  {len(data)}점")
    print(f"[기하]  z_P=0, z_UW={Z_A}, z_DW={Z_B} → L={L}, a={A_}, b={B_}")
    print(f"[TRB]   e={E_LIM}  α={math.degrees(math.atan(E_LIM/1.5)):.2f}°  Y1={Y1:.4f}")

    if "--nomasta" not in sys.argv:
        report_backdata(backdata(data))

    # B. 수명 비교 — 가장 큰 MASTA 결과셋과 동일 점군 사용
    files = [f for f in sorted(glob.glob(os.path.join("Results", "*_dt_*.csv")))
             if "summary" not in f]
    path = max(files, key=lambda f: sum(1 for _ in open(f, encoding="utf-8-sig")))
    spath = os.path.splitext(path)[0] + "_summary.csv"
    idxs = sorted({int(r["index"]) for r in csv.DictReader(open(path, encoding="utf-8-sig"))
                   if r.get("index", "").strip().isdigit()})
    w = [1.0] * len(idxs)
    w[0] = w[-1] = 0.5
    step = (idxs[-1] - idxs[0]) / (len(idxs) - 1) * 0.1     # Δt [s]
    weights = [x * step for x in w]

    t0 = time.perf_counter()
    c1 = method_c1(data, idxs, weights)
    el = time.perf_counter() - t0
    ref = ref_from_summary(spath) if os.path.exists(spath) else {}

    print("\n" + "=" * 100)
    print(f"B. 핀지지 정역학 기반 방법 C-1 수명  ({os.path.basename(path)}, {len(idxs)}점, "
          f"계산 {el*1000:.1f} ms)")
    print("=" * 100)
    for nm in (NAME_A, NAME_B):
        c = c1[nm]
        print(f"\n[{nm}]")
        print(f"  P_eq      = {c['Peq']/1e3:12,.1f} kN")
        print(f"  n_eq      = {c['n_eq']:12.4f} rpm   → κ = {c['kappa']:.5f}")
        print(f"  e_C·C_u/P_eq = {E_C*CU_N/c['Peq']:9.5f}   → a_ISO = {c['a_iso']:.5f}")
        print(f"  L10       = {c['L10']:12.4e} Rev")
        print(f"  L10m      = {c['L10m']:12.4e} Rev")
        print(f"  ΣN(표본)  = {c['N']:12.4e} Rev")
        print(f"  D(표본)   = {c['D']:12.4e}")
        print(f"  D(30년)   = {c['D30']:12.4f}")
        print(f"  등가수명  = {c['life']:12.2f} yr")
        if nm in ref:
            r = ref[nm]
            print(f"  ── 방법 B(MASTA ISO281 per-step) 대비 ──")
            print(f"  D(표본) B = {r['D']:12.4e}   → C-1 오차 {(c['D']/r['D']-1)*100:+8.2f}%")
            print(f"  D(30년) B = {r['D30']:12.4f}   → C-1 오차 {(c['D30']/r['D30']-1)*100:+8.2f}%")
            print(f"  등가수명 B= {r['life']:12.2f} yr → C-1 오차 "
                  f"{(c['life']/r['life']-1)*100:+8.2f}%")


if __name__ == "__main__":
    main()
