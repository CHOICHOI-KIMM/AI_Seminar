"""
P2 Phase 1 Step 0 — 최경량 가능해 12건의 스크리닝 상수 실측
============================================================
부록 3 L_eff 기반 강체정역학 스크리닝에 필요한 설계별 상수를 뽑는다.

  기하   : a(유효하중중심) · T → c = a − T/2 → L · A · B
  베어링 : Y1 · e · C · C_u · (오염계수 e_C)
  기타   : D_pw (κ 산출용)

출력: P1_극한응력_Phase2/p2_constants.csv
"""
import csv
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
# argv[1] "2" → Phase 2(24건) · "3" → Phase 3(6건 · 베어링질량 기준)
# 없으면 Phase 1(12건 + 기준선)
_PH = sys.argv[1] if len(sys.argv) > 1 else "1"
PH2 = _PH in ("2", "3", "4", "6", "8")   # 기준선 행을 붙이지 않는 단계
if _PH == "8":                   # S4-d(부록 9) — D 상한 3건 (§9-10)
    DIR = os.path.join(HERE, "P2_피로수명_A9")
    SRCF, OUT = "p2f_targets.csv", os.path.join(DIR, "p2f_constants.csv")
elif _PH == "6":                   # S4-d — 부록 8 S3-c 프론트 14건 (§8-7)
    DIR = os.path.join(HERE, "P2_피로수명_A8")
    SRCF, OUT = "p2e_targets.csv", os.path.join(DIR, "p2e_constants.csv")
elif _PH == "4":                 # S4 — 부록 6 S3-c 프론트 40건 (§6-11.5)
    DIR = os.path.join(HERE, "P2_피로수명_S4")
    SRCF, OUT = "p2d_targets.csv", os.path.join(DIR, "p2d_constants.csv")
elif _PH == "3":
    DIR = os.path.join(HERE, "P2_피로수명_Phase3")
    SRCF, OUT = "p2c_targets.csv", os.path.join(DIR, "p2c_constants.csv")
elif _PH == "2":
    DIR = os.path.join(HERE, "P2_피로수명_Phase2")
    SRCF, OUT = "p2b_targets.csv", os.path.join(DIR, "p2b_constants.csv")
else:
    DIR = os.path.join(HERE, "P1_극한응력_Phase2")
    SRCF, OUT = "p1_feasible.csv", os.path.join(DIR, "p2_constants.csv")
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg   # noqa: E402

if _PH in ("6", "8"):      # 두께 규칙·코너 반경·정수화 (§8-3)
    import a8_build        # noqa: F401,E402

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
NTOP = 12

CAND = {
    "Y1": ["dynamic_axial_load_factor_for_high_axial_radial_load_ratios"],
    "e": ["limiting_value_for_axial_load_ratio"],
    "C": ["basic_dynamic_load_rating", "basic_dynamic_radial_load_rating"],
    "C_u": ["fatigue_load_limit", "fatigue_limit_load",
            "basic_static_load_rating"],
    "a": ["effective_centre_from_front_face"],
    "T": ["width"],
    "e_C": ["contamination_factor"],
}


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception:
        return None


def sc(o, n):
    v = safe(o, n)
    if isinstance(v, (int, float)) and not isinstance(v, bool):
        return float(v)
    for a in ("value", "wrapped"):
        w = safe(v, a)
        if isinstance(w, (int, float)) and not isinstance(w, bool):
            return float(w)
    return None


def pick(det, names):
    for n in names:
        v = sc(det, n)
        if v is not None:
            return v, n
    return None, ""


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design
    from mastapy.bearings import RollerBearingProfileTypes as RP

    with open(os.path.join(DIR, SRCF), encoding="utf-8-sig") as f:
        top = list(csv.DictReader(f))
    if not PH2:
        top = top[:NTOP]
        # 기준선 (v1.3 제원 · z 0.5/3.0) — 격자점이 아니므로 별도 지정 (§8-3.1)
        top.append(dict(rank_mass="base", D_pw_mm="3330.9", alpha="19.0",
                        D_we_mm="110.51", L_we_mm="238.048", z1="0.5", z2="3.0"))

    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    sh = list(asm.all_parts_of_type_shaft())[0]
    bs = list(asm.all_parts_of_type_bearing())
    uw = [b for b in bs if "UW" in str(b)][0]
    dw = [b for b in bs if "DW" in str(b)][0]
    for b in bs:
        b.detail.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG

    # 후보 속성 탐색 (1회)
    first = True
    rows = []
    for r in top:
        dpw = float(r["D_pw_mm"]) / 1e3
        al = float(r["alpha"])
        dwe = float(r["D_we_mm"]) / 1e3
        lwe = float(r["L_we_mm"]) / 1e3
        z1, z2 = float(r["z1"]), float(r["z2"])
        g = sg.bearing(dpw, al, dwe, lwe)
        for b in bs:
            try:
                if b.inner_connection is not None:
                    b.inner_connection.delete()
            except Exception:
                pass
        s = sg.shaft(g["bore"], z2)
        try:
            sh.remove_all_sections()
            sh.add_section(0.0, s["length"], s["outer_diameter"],
                           s["inner_diameter"], s["outer_diameter"],
                           s["inner_diameter"])
        except Exception as e:
            print("  !! shaft", str(e)[:50])
        for b in bs:
            sg.apply_to_masta(b.detail, g)
        for b, z in ((uw, z1), (dw, z2)):
            try:
                b.try_mount_on(sh, z)
            except Exception as e:
                print("  !! mount", str(e)[:40])

        det = uw.detail
        if first:
            first = False
            print("속성 탐색 (rating·마찰 관련):")
            for n in sorted(x for x in dir(det) if not x.startswith("_") and any(
                    t in x.lower() for t in ("rating", "fatigue", "limit",
                                             "contamin", "factor"))):
                v = sc(det, n)
                if v is not None:
                    print(f"   {n:58} = {v:,.6g}")
            print()

        vals, srcs = {}, {}
        for key, names in CAND.items():
            v, nm = pick(det, names)
            vals[key], srcs[key] = v, nm
        a_, T_ = vals["a"], vals["T"]
        c = a_ - T_ / 2 if (a_ is not None and T_ is not None) else None
        row = dict(rank_mass=r["rank_mass"], D_pw_mm=round(dpw * 1e3, 1),
                   alpha=al, D_we_mm=round(dwe * 1e3, 1),
                   L_we_mm=round(lwe * 1e3, 1), z1=z1, z2=z2,
                   Z=g["number_of_elements"],
                   a_mm=None if a_ is None else round(a_ * 1e3, 3),
                   T_mm=round(T_ * 1e3, 3),
                   c_mm=None if c is None else round(c * 1e3, 3),
                   L_eff_m=None if c is None else round((z2 - z1) + 2 * c, 6),
                   A_m=None if c is None else round(c - z1, 6),
                   B_m=None if c is None else round(z2 + c, 6),
                   Y1=None if vals["Y1"] is None else round(vals["Y1"], 6),
                   e_lim=None if vals["e"] is None else round(vals["e"], 6),
                   C_kN=None if vals["C"] is None else round(vals["C"] / 1e3, 1),
                   Cu_kN=None if vals["C_u"] is None else round(vals["C_u"] / 1e3, 1),
                   e_C=None if vals["e_C"] is None else round(vals["e_C"], 6))
        rows.append(row)
        print(f"  #{row['rank_mass']:>2}  D_pw{row['D_pw_mm']:>6.0f} a{al:>3.0f} "
              f"D_we{row['D_we_mm']:>5.0f} L_we{row['L_we_mm']:>5.0f}  "
              f"a={row['a_mm']:>8.1f} T={row['T_mm']:>7.1f}  "
              f"L_eff={row['L_eff_m']:>7.4f} A={row['A_m']:>+8.4f} B={row['B_m']:>7.4f}  "
              f"Y1={row['Y1']} e={row['e_lim']}  "
              f"C={row['C_kN']:>10,.0f} Cu={row['Cu_kN']:>9,.0f}", flush=True)

    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)
    print(f"\n[저장] {OUT}")
    print("속성 출처:", {k: v for k, v in srcs.items() if v})


if __name__ == "__main__":
    main()
