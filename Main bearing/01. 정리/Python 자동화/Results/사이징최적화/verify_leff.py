"""
L_eff 분리 검증 (문서 §8-1.7.3.1)
==================================
σ 가 유효스팬 L_eff 만의 함수인가, 아니면 z1·z2·α 가 독립 경로를 갖는가.

  A. 동일 L_eff · z1 변화     (z2−z1 = 4.0 고정)
  B. L_eff 스윕 — z1 경로     (z2 5.0 고정)
  C. L_eff 스윕 — z2 경로     (z1 0.3 고정)
  D. L_eff 스윕 — α 경로      (z 0.3/5.0 고정)

판정: σ 를 L_eff 에 대해 그렸을 때 네 시리즈가 한 곡선으로 모이면 L_eff 가 지배변수.
L_eff 는 MASTA 실측 a 로 산출: L_eff = (z2 − z1) + 2·(a − T/2)

공통: D_pw 4,500 · D_we 110 · L_we 295 (T/D_pw = 0.0854 로 해석 가능 영역)
"""
import csv
import math
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
OUT = os.path.join(HERE, "P1_극한응력", "leff_verify.csv")
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg   # noqa: E402

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
DPW, DWE, LWE = 4.500, 0.110, 0.295
GOV = {"Myz_max": 22673.0, "My_min": 27453.0, "Mz_min": 17013.0}

SERIES = []
for z1, z2 in ((0.3, 4.3), (0.5, 4.5), (0.7, 4.7), (1.0, 5.0)):
    SERIES.append(("A_동일Leff", 30.0, z1, z2))
for z1 in (0.3, 0.5, 0.7, 1.0):
    SERIES.append(("B_z1경로", 30.0, z1, 5.0))
for z2 in (3.0, 3.5, 4.0, 4.5, 5.0):
    SERIES.append(("C_z2경로", 30.0, 0.3, z2))
for al in (12.0, 18.0, 24.0, 30.0):
    SERIES.append(("D_alpha경로", al, 0.3, 5.0))


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


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
    from mastapy.bearings import RollerBearingProfileTypes as RP

    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    sh = list(asm.all_parts_of_type_shaft())[0]
    bs = list(asm.all_parts_of_type_bearing())
    uw = [b for b in bs if "UW" in str(b)][0]
    dw = [b for b in bs if "DW" in str(b)][0]
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    dp = asm.design_properties
    for b in bs:
        b.detail.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG
    tpl = {nm: next(c for c in dp.static_loads if c.name == nm) for nm in GOV}
    ds = tpl["Myz_max"].design_state_load_case_group
    print(f"[L_eff 검증] {len(SERIES)}점 · D_pw {DPW*1e3:.0f} · D_we {DWE*1e3:.0f} "
          f"· L_we {LWE*1e3:.0f}")

    rows = []
    for n, (ser, al, z1, z2) in enumerate(SERIES, 1):
        g = sg.bearing(DPW, al, DWE, LWE)
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
            print("  !! shaft", str(e)[:40])
        for b in bs:
            sg.apply_to_masta(b.detail, g)
        for b, z in ((uw, z1), (dw, z2)):
            try:
                b.try_mount_on(sh, z)
            except Exception as e:
                print("  !! mount", str(e)[:40])
        # MASTA 실측 a 로 L_eff
        a_m = sc(uw.detail, "effective_centre_from_front_face")
        T = sc(uw.detail, "width")
        L_meas = (z2 - z1) + 2 * (a_m - T / 2) if a_m else None
        L_appx = (z2 - z1) + DPW * math.tan(math.radians(al))

        names, dups = [], []
        for nm, tq in GOV.items():
            lc = tpl[nm].duplicate(ds, f"lv{n}_{nm}")
            q = lc.inputs_for_power_load(ipl)
            for a_, v_ in (("speed", 0.0), ("torque", tq * 1e3)):
                try:
                    setattr(q, a_, v_)
                except Exception:
                    pass
            names.append(nm); dups.append(lc)
        duty = dp.add_duty_cycle(f"lvd{n}")
        for lc in dups:
            duty.add_static_load(lc)
        csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
        csd.perform_analysis()
        sig, best, who = {}, 0.0, ""
        for b, tag in ((uw, "UW"), (dw, "DW")):
            subs = list(list(csd.results_for(b))[0].component_analysis_cases)
            for nm, sub in zip(names, subs):
                v = sc(sub.component_detailed_analysis, "maximum_normal_stress")
                v = None if v is None else v / 1e6
                sig[f"{nm}_{tag}"] = v
                if v and v > best:
                    best, who = v, f"{nm}/{tag}"
        for x in dups + [duty]:
            try:
                x.delete()
            except Exception:
                pass
        rows.append(dict(series=ser, alpha=al, z1=z1, z2=z2, span=round(z2 - z1, 3),
                         a_mm=None if a_m is None else round(a_m * 1e3, 2),
                         T_mm=round(T * 1e3, 2),
                         L_eff_meas=None if L_meas is None else round(L_meas, 5),
                         L_eff_appx=round(L_appx, 5),
                         sigma_max=round(best, 1), governing=who,
                         **{k: (None if v is None else round(v, 1))
                            for k, v in sig.items()}))
        print(f"  [{n:2d}/{len(SERIES)}] {ser:12} a{al:4.0f} z{z1:.1f}/{z2:.1f} "
              f"L_eff {L_meas:.4f} (근사 {L_appx:.4f}) -> {best:7.1f} MPa")

    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader(); w.writerows(rows)
    print(f"\n[저장] {OUT}")

    # ── 판정 ──
    print("\n=== A. 동일 L_eff · z1 변화 ===")
    A = [r for r in rows if r["series"] == "A_동일Leff"]
    b0 = A[0]
    for r in A:
        print(f"  z1 {r['z1']:.1f} L_eff {r['L_eff_meas']:.4f} "
              f"σ {r['sigma_max']:7.1f}  ({(r['sigma_max']/b0['sigma_max']-1)*100:+.2f}%)")
    sp = max(r["sigma_max"] for r in A) / min(r["sigma_max"] for r in A) - 1
    print(f"  → 동일 L_eff 내 σ 편차 {sp*100:.2f}%")

    print("\n=== 경로별 L_eff 기울기 [%/m] ===")
    for ser in ("B_z1경로", "C_z2경로", "D_alpha경로"):
        S = sorted([r for r in rows if r["series"] == ser],
                   key=lambda r: r["L_eff_meas"])
        if len(S) < 2:
            continue
        dL = S[-1]["L_eff_meas"] - S[0]["L_eff_meas"]
        dS = S[-1]["sigma_max"] / S[0]["sigma_max"] - 1
        print(f"  {ser:12} L_eff {S[0]['L_eff_meas']:.3f}→{S[-1]['L_eff_meas']:.3f} "
              f"σ {S[0]['sigma_max']:.1f}→{S[-1]['sigma_max']:.1f}  "
              f"{dS*100/dL:+7.2f} %/m")


if __name__ == "__main__":
    main()
