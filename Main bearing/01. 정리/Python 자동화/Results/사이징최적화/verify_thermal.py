# -*- coding: utf-8 -*-
"""
샤프트·하우징 온도가 결과를 바꾸는가 — 실측 확인
====================================================
`include_thermal_expansion_effects = False` 라면 샤프트·하우징 온도는 해석에
들어갈 경로가 없어야 한다. 같은 설계·같은 DLC 를 세 조건으로 돌려 확인한다.

  ⒜ 베어링 70 · 샤프트 40 · 하우징 40   ← S4-d 70 °C 가 쓴 조건
  ⒝ 베어링 70 · 샤프트 70 · 하우징 70   ← 전체를 올린 조건
  ⒞ 베어링 50 · 샤프트 40 · 하우징 40   ← 50 °C 대조

⒜ = ⒝ 이면 샤프트·하우징 온도는 무관하고, 손상 증가는 전부 점도 효과다.
플래그를 켠 ⒟ 도 함께 돌려 「끄면 같고 켜면 다르다」를 보인다.
"""
import os
import sys

H = (r"d:\AI\AI_Seminar\Main bearing\01. 정리\Python 자동화"
     r"\Results\사이징최적화")
sys.path.insert(0, H)
sys.path.insert(0, os.path.dirname(os.path.dirname(H)))

import csv                       # noqa: E402
import a8_build                  # noqa: F401,E402  (부록 8 규칙 주입)
import sizing_geom as sg         # noqa: E402
import run_p2_fatigue as p2      # noqa: E402

TAG, DLC = "a01", "DLC1.2-k-s5"


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import (
        AnalysisType)
    from mastapy.bearings import RollerBearingProfileTypes as RP

    c = next(r for r in csv.DictReader(
        open(os.path.join(H, "P2_피로수명_A8", "p2e_constants.csv"),
             encoding="utf-8-sig")) if r["rank_mass"] == TAG)
    k = float(next(r for r in csv.DictReader(
        open(os.path.join(H, "P2_피로수명_A8", "screen_k.csv"),
             encoding="utf-8-sig"))
        if r["design"] == TAG and r["DLC"] == DLC)["k"])

    d = Design.load(p2.MODEL)
    asm = d.all_parts_of_type_root_assembly()[0]
    sh = list(asm.all_parts_of_type_shaft())[0]
    bs = list(asm.all_parts_of_type_bearing())
    uw = [b for b in bs if "UW" in str(b)][0]
    dw = [b for b in bs if "DW" in str(b)][0]
    for b in bs:
        b.detail.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG
    dp = asm.design_properties
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    lc0 = next(x for x in dp.static_loads if x.name == "Load Case 1")
    ds = lc0.design_state_load_case_group

    z1, z2 = float(c["z1"]), float(c["z2"])
    g = sg.bearing(float(c["D_pw_mm"]) / 1e3, float(c["alpha"]),
                   float(c["D_we_mm"]) / 1e3, float(c["L_we_mm"]) / 1e3)
    for b in bs:
        try:
            if b.inner_connection is not None:
                b.inner_connection.delete()
        except Exception:
            pass
    s = sg.shaft(g["bore"], z2)
    sh.remove_all_sections()
    sh.add_section(0.0, s["length"], s["outer_diameter"], s["inner_diameter"],
                   s["outer_diameter"], s["inner_diameter"])
    for b in bs:
        sg.apply_to_masta(b.detail, g)
    for b, z in ((uw, z1), (dw, z2)):
        b.try_mount_on(sh, z)
    print(f"[모델] {TAG} · 내경 {s['inner_diameter']*1e3:,.0f} mm · "
          f"코너 {uw.detail.left_element_corner_radius*1e3:.1f} mm · "
          f"L_we {uw.detail.effective_roller_length*1e3:.2f} mm")

    reps = p2.bin_reps(p2.load_raw(DLC), k)
    t = lc0.temperatures
    print(f"[플래그] include_thermal_expansion_effects = "
          f"{lc0.include_thermal_expansion_effects}")

    def run(tag, brg, shf, hsg, expand):
        lc0.include_thermal_expansion_effects = expand
        for a in ("rolling_bearing_element", "rolling_bearing_inner_race",
                  "rolling_bearing_outer_race"):
            setattr(t, a, brg)
        t.shaft, t.housing = shf, hsg
        lcs = []
        for cid, rev, rec in reps:
            lc = lc0.duplicate(ds, f"th_{tag}_{cid}")
            mf.set_loads(lc, pl, ipl, rec)
            lcs.append(lc)
        duty = dp.add_duty_cycle(f"thd_{tag}")
        for lc in lcs:
            duty.add_static_load(lc)
        csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
        csd.perform_analysis()
        tot = 0.0
        subs = list(list(csd.results_for(uw))[0].component_analysis_cases)
        for (cid, rev, rec), sub in zip(reps, subs):
            l10 = (sub.component_detailed_analysis
                   .isots162812008.modified_reference_rating_life_cycles)
            tot += rev / l10 if (l10 and l10 > 0) else 0.0
        for x in lcs + [duty]:
            try:
                x.delete()
            except Exception:
                pass
        print(f"  {tag:26s} 베어링 {brg:.0f} · 샤프트 {shf:.0f} · 하우징 "
              f"{hsg:.0f} · 열팽창 {str(expand):5s} → Σ1/L = {tot:.6e}")
        return tot

    print(f"\n[대조] {DLC} · k = {k} · 빈 {len(reps)}개  (UW 손상 합)")
    a = run("(a) S4-d 70C 조건", 70, 40, 40, False)
    b = run("(b) 전체 70C", 70, 70, 70, False)
    c_ = run("(c) 50C 대조", 50, 40, 40, False)
    e = run("(d) 전체 70C + 열팽창 켬", 70, 70, 70, True)
    f = run("(e) 70/40/40 + 열팽창 켬", 70, 40, 40, True)

    print(f"\n  (b)/(a) = {b/a:.9f}   ← 1.000000000 이면 샤프트·하우징 무관")
    print(f"  (a)/(c) = {a/c_:.6f}   ← 온도(점도) 효과")
    print(f"  (d)/(b) = {e/b:.6f}   ← 열팽창을 켜면 달라지는가")
    print(f"  (e)/(a) = {f/a:.6f}   ← 같은 온도에서 열팽창만 켠 경우")


if __name__ == "__main__":
    main()
