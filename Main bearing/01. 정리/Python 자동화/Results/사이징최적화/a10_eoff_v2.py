# -*- coding: utf-8 -*-
"""§10-11.3.6 v2 — 사용자 유도 수식으로 element_offset 재산출 · 영향 실측

    e_off = o_off − (D_we/2)·sin(α − β)

  o_off  외륜 중심의 row 중심 대비 위치 = T − (B+C)/2   (MASTA 실측 항등식)
  α      궤도 경사각 = cup_angle = contact_angle
  β      롤러 반각 = element_taper_angle / 2  (= (cup − cone)/2)

보정항 (D_we/2)·sin(α−β) 는 **롤러 중심에서 접촉선까지의 축방향 성분**이다
— 롤러 축의 경사가 정확히 α−β 이고, 접촉선은 그 축에서 반지름 D_we/2 만큼
수직으로 떨어져 있다.

지배 LC Myz_max 에서 현행 · 신규 · (신규 + 비대칭 프로파일) σ 를 잰다.
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
FUJI = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "fujiwara")
RANKS = (1, 103, 210)
NPTS = 61


def rule_v2(d):
    """MASTA detail → (신규 e_off [mm], 중간값 dict)"""
    T = d.width * 1e3
    Bw = d.inner_ring_width * 1e3
    C = d.outer_ring_width * 1e3
    o_off = T - (Bw + C) / 2.0
    D_we = d.element_diameter * 1e3
    al = d.cup_angle                       # [rad]
    be = d.element_taper_angle / 2.0       # 롤러 반각 [rad]
    corr = D_we / 2.0 * math.sin(al - be)
    return o_off - corr, dict(
        T=T, B=Bw, C=C, o_off=round(o_off, 3), D_we=round(D_we, 2),
        alpha_deg=round(math.degrees(al), 4),
        two_beta_deg=round(math.degrees(d.element_taper_angle), 4),
        beta_deg=round(math.degrees(be), 4),
        axis_deg=round(math.degrees(al - be), 4),
        corr=round(corr, 3))


def best_asym(rk):
    R = list(csv.DictReader(open(os.path.join(FUJI, "fujiwara_all.csv"),
                                 encoding="utf-8-sig")))
    S = [r for r in R if int(r["rank"]) == rk and r["stage"] == "S3"
         and r["feasible"] == "1"]
    if not S:
        return None
    t = max(S, key=lambda r: float(r["score"]))
    return (float(t["K1L"]), float(t["K2L"]), float(t["zmL_um"]) / 1e6,
            float(t["K1R"]), float(t["K2R"]), float(t["zmR_um"]) / 1e6)


def main():
    os.makedirs(OUT, exist_ok=True)
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    QB = json.load(open(os.path.join(FUJI, "baseline.json"), encoding="utf-8"))
    rig = L.Rig()
    rig.load_case()                       # Myz_max
    res = {}

    print("=" * 78)
    print("① 규칙  e_off = o_off − (D_we/2)·sin(α − β)")
    print("=" * 78)
    for rk in RANKS:
        rig.build(P[rk])
        e_new, g = rule_v2(rig.uw.detail)
        g["e_cur"] = round(rig.uw.detail.element_offset * 1e3, 4)
        g["e_new"] = round(e_new, 3)
        g["de"] = round(e_new - g["e_cur"], 3)
        res[rk] = g
        print(f"  #{rk:<4} T {g['T']:.0f} · B {g['B']:.0f} · C {g['C']:.0f} "
              f"→ o_off {g['o_off']:6.2f}")
        print(f"        D_we {g['D_we']:.1f} · α {g['alpha_deg']:.4f}° · "
              f"2β {g['two_beta_deg']:.4f}° · β {g['beta_deg']:.4f}° · "
              f"롤러축 α−β {g['axis_deg']:.4f}°")
        print(f"        보정 {g['corr']:6.2f} → **e_off "
              f"{g['e_new']:6.2f}** (현행 {g['e_cur']:.2f} · "
              f"Δ {g['de']:+.2f})")

    print("\n" + "=" * 78)
    print("② 영향 — 지배 LC Myz_max")
    print("=" * 78)
    print(f"{'설계':>6s} {'조건':<20s} {'e_off':>8s} {'a':>9s} {'L_eff':>9s} "
          f"{'σ_max':>9s} {'제약':>6s}")
    for rk in RANKS:
        g = res[rk]
        p = best_asym(rk)
        z1, z2 = float(P[rk]["z1"]), float(P[rk]["z2"])
        for tag, e, prof in (("현행", g["e_cur"], False),
                             ("신규 e_off", g["e_new"], False),
                             ("신규 + 프로파일", g["e_new"], True)):
            rig.build(P[rk])
            for b in rig.bs:
                b.detail.element_offset = e / 1e3
            if prof and p:
                f = L.profile_fn(rig.Lwe, p[0], p[1], p[2], K1R=p[3],
                                 K2R=p[4], zmR=p[5], Q=QB[str(rk)]["P_max_N"])
                rig.set_user(f, NPTS)
            else:
                rig.set_din(0.0)
            d = rig.uw.detail
            a = d.effective_centre_from_front_face * 1e3
            Le = (z2 - z1) * 1e3 + 2 * (a - g["T"] / 2.0)
            m, _ = rig.solve(f"v2_{rk}_{len(g)}_{tag[:4]}")
            key = {"현행": "cur", "신규 e_off": "new",
                   "신규 + 프로파일": "new_prof"}[tag]
            g[f"a_{key}"] = round(a, 2)
            g[f"Leff_{key}"] = round(Le, 1)
            g[f"sigma_{key}"] = m["sigma_MPa"]
            g[f"edgeL_{key}"] = m["edge_L_MPa"]
            g[f"ystar_{key}"] = m["y_star_mm"]
            print(f"{rk:6d} {tag:<20s} {e:8.2f} {a:9.2f} {Le:9.1f} "
                  f"{m['sigma_MPa']:9.1f} "
                  f"{'OK' if m['sigma_MPa'] <= 2100 else '위반':>6s}",
                  flush=True)
        g["dLeff_pct"] = round(100 * (g["Leff_new"] - g["Leff_cur"])
                               / g["Leff_cur"], 2)
        g["dsigma"] = round(g["sigma_new"] - g["sigma_cur"], 1)

    json.dump({str(k): v for k, v in res.items()},
              open(os.path.join(OUT, "eoff_v2.json"), "w"), indent=1)
    print("\n[저장]", os.path.join(OUT, "eoff_v2.json"))


if __name__ == "__main__":
    main()
