# -*- coding: utf-8 -*-
"""§10-11.3.6 — element_offset 규칙(가설 B: 궤도 중앙)과 그 영향 실측

MASTA 정의
  element_offset      롤러 중심과 row 중심 사이의 축방향 거리
  outer_ring_offset   외륜 중심과 row 중심 사이의 축방향 거리
                      (실측 항등식 o_off = T − (B+C)/2 · 4설계에서 mm 일치)

`mass_properties_of_*_from_geometry` 가 세 물체를 같은 프레임으로 주며
롤러 중심 z = element_offset 임을 확인했다. 따라서 가설 B 는

      element_offset = outer_ring_offset = T − (B+C)/2

이고, SSOT 비율을 넣으면  e = (1.30226 − (1.26025+1.06281)/2)·L_we
                          = 0.14073 · L_we  이다.

영향: a 가 1:1 로 줄고 L_eff = (z2−z1) + 2(a − T/2) 가 따라 줄어든다.
극한 LC 16건에서 σ_max 변화를 잰다.
"""
import csv
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import a10_profile_lib as L      # noqa: E402

OUT = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "element_offset")
RANKS = (1, 103, 210)
LCS = ("Mx_max", "Mx_min", "My_max", "My_min", "Mz_max", "Mz_min",
       "Myz_max", "Myz_min", "Fx_max", "Fx_min", "Fy_max", "Fy_min",
       "Fz_max", "Fz_min", "Fyz_max", "Fyz_min")

T_R, B_R, C_R = 1.30226, 1.26025, 1.06281     # sizing_geom SSOT 비율
E_OVER_LWE = T_R - (B_R + C_R) / 2.0          # = 0.140725


def geo(d):
    """MASTA 실측 기하 [mm]"""
    g = dict(L_we=d.effective_roller_length * 1e3,
             L_w=d.roller_length * 1e3,
             T=d.width * 1e3, B=d.inner_ring_width * 1e3,
             C=d.outer_ring_width * 1e3,
             e_off=d.element_offset * 1e3,
             o_off=d.outer_ring_offset * 1e3,
             a=d.effective_centre_from_front_face * 1e3,
             cup_deg=d.cup_angle * 180 / math.pi,
             D_pw=d.pitch_circle_diameter * 1e3)
    for nm, at in (("r_f", "outer_ring_front_face_corner_radius"),
                   ("r_b", "outer_ring_back_face_corner_radius")):
        try:
            g[nm] = float(getattr(d, at)) * 1e3
        except Exception:
            g[nm] = None
    # 외륜 궤도의 축방향 길이 / 모선 길이
    ch = (g["r_f"] or 0) + (g["r_b"] or 0)
    g["race_axial"] = g["C"] - ch
    g["race_slant"] = g["race_axial"] / math.cos(math.radians(g["cup_deg"]))
    g["roller_axial"] = g["L_we"] * math.cos(math.radians(g["cup_deg"]))
    g["room"] = (g["race_axial"] - g["roller_axial"]) / 2.0   # 편측 여유
    g["ecc"] = g["o_off"] - g["e_off"]                        # 편심량
    g["over"] = g["ecc"] - g["room"]                          # 초과(양수=돌출)
    return g


def main():
    os.makedirs(OUT, exist_ok=True)
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    rig = L.Rig()
    res, rows = {}, []

    print("=" * 78)
    print("① 규칙 — 현행 대 가설 B")
    print("=" * 78)
    for rk in RANKS:
        rig.build(P[rk])
        d = rig.uw.detail
        g = geo(d)
        e_new = g["o_off"]
        g["e_new"] = e_new
        g["de"] = e_new - g["e_off"]
        g["ratio_new"] = e_new / g["L_we"]
        res[rk] = g
        print(f"\n  #{rk} · L_we {g['L_we']:.1f} · T {g['T']:.0f} · "
              f"B {g['B']:.0f} · C {g['C']:.0f} · cup {g['cup_deg']:.2f}° · "
              f"모따기 {g['r_f']:.2f}/{g['r_b']:.2f}")
        print(f"     궤도 축방향 {g['race_axial']:7.2f} · 롤러 투영 "
              f"{g['roller_axial']:7.2f} · 편측 여유 {g['room']:6.2f}")
        print(f"     현행 e_off {g['e_off']:7.3f} · o_off {g['o_off']:6.2f} "
              f"→ 편심 {g['ecc']:6.2f} · **돌출 {g['over']:+6.2f} mm**")
        print(f"     가설 B  e_off = {e_new:7.2f}  (Δ {g['de']:+7.2f} · "
              f"L_we 대비 {g['ratio_new']:.5f})")

    print("\n" + "=" * 78)
    print("② 영향 — 극한 LC 16건 σ_max (현행 대 신규)")
    print("=" * 78)
    for rk in RANKS:
        g = res[rk]
        for tag, e in (("현행", g["e_off"]), ("가설B", g["e_new"])):
            rig.build(P[rk])
            rig.set_din(0.0)
            for b in rig.bs:
                b.detail.element_offset = e / 1e3
            d = rig.uw.detail
            a_now = d.effective_centre_from_front_face * 1e3
            z1, z2 = float(P[rk]["z1"]), float(P[rk]["z2"])
            L_eff = (z2 - z1) * 1e3 + 2 * (a_now - g["T"] / 2.0)
            g[f"a_{tag}"] = round(a_now, 2)
            g[f"Leff_{tag}"] = round(L_eff, 1)
            for nm in LCS:
                rig.load_case(nm)
                try:
                    m, _ = rig.solve(f"eo{rk}_{tag}_{nm}")
                except Exception as ex:
                    print(f"   #{rk} {tag} {nm} 실패 "
                          f"{str(ex).splitlines()[0][:36]}")
                    continue
                rows.append(dict(rank=rk, case=tag, e_off=round(e, 3),
                                 lc=nm, a_mm=round(a_now, 2),
                                 L_eff_mm=round(L_eff, 1),
                                 sigma_MPa=m["sigma_MPa"],
                                 edge_L_MPa=m["edge_L_MPa"],
                                 y_star_mm=m["y_star_mm"],
                                 tilt_mrad=m["tilt_mrad"]))
            s = [r["sigma_MPa"] for r in rows
                 if r["rank"] == rk and r["case"] == tag]
            print(f"  #{rk:<4} {tag:<5} e_off {e:7.2f} · a {a_now:8.2f} · "
                  f"L_eff {L_eff:8.1f} · σ 최대 {max(s):7.1f} "
                  f"(LC {LCS[s.index(max(s))]})", flush=True)

    print("\n" + "=" * 78)
    print("③ 요약 — 지배 LC Myz_max")
    print("=" * 78)
    print(f"{'설계':>6s} {'Δe_off':>9s} {'Δa':>9s} {'ΔL_eff':>9s} "
          f"{'σ 현행':>9s} {'σ 신규':>9s} {'Δσ':>8s} {'σ 최대증가':>10s}")
    for rk in RANKS:
        g = res[rk]
        sc = [r for r in rows if r["rank"] == rk and r["case"] == "현행"]
        sn = [r for r in rows if r["rank"] == rk and r["case"] == "가설B"]
        gc = {r["lc"]: r["sigma_MPa"] for r in sc}
        gn = {r["lc"]: r["sigma_MPa"] for r in sn}
        dmax = max(gn[k] - gc[k] for k in gc if k in gn)
        print(f"{rk:6d} {g['de']:+9.2f} "
              f"{g['a_가설B']-g['a_현행']:+9.2f} "
              f"{g['Leff_가설B']-g['Leff_현행']:+9.1f} "
              f"{gc['Myz_max']:9.1f} {gn['Myz_max']:9.1f} "
              f"{gn['Myz_max']-gc['Myz_max']:+8.1f} {dmax:+10.1f}")
        g["dsigma_gov"] = round(gn["Myz_max"] - gc["Myz_max"], 1)
        g["dsigma_max"] = round(dmax, 1)

    with open(os.path.join(OUT, "eoff_lc.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)
    json.dump({str(k): v for k, v in res.items()},
              open(os.path.join(OUT, "eoff_rule.json"), "w"), indent=1)
    print("\n[저장]", OUT)


if __name__ == "__main__":
    main()
