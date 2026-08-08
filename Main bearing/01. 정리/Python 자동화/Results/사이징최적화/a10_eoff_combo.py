# -*- coding: utf-8 -*-
"""§10-11.3.6 보강 — v1.3 대조 + (신 element_offset x 비대칭 프로파일) 합산

element_offset 을 궤도 중앙으로 옮기면 σ 가 +9 ~ +18 MPa 올라 제약을 넘는다.
§10-12.6 의 비대칭 Fujiwara 프로파일은 σ 를 8 ~ 32 MPa 낮춘다.
둘을 동시에 걸면 상쇄되는지 — 단순 합이 아니므로 실측한다.
"""
import csv
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import a10_profile_lib as L        # noqa: E402
import a10_element_offset as EO    # noqa: E402
import sizing_geom as sg           # noqa: E402

OUT = EO.OUT
FUJI = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "fujiwara",
                    "fujiwara_all.csv")
RANKS = (1, 103, 210)
NPTS = 61


def best_asym(R, rk):
    S = [r for r in R if int(r["rank"]) == rk and r["stage"] == "S3"
         and r["feasible"] == "1"]
    if not S:
        return None
    t = max(S, key=lambda r: float(r["score"]))
    return (float(t["K1L"]), float(t["K2L"]), float(t["zmL_um"]) / 1e6,
            float(t["K1R"]), float(t["K2R"]), float(t["zmR_um"]) / 1e6)


def main():
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    R = list(csv.DictReader(open(FUJI, encoding="utf-8-sig")))
    rule = json.load(open(os.path.join(OUT, "eoff_rule.json"),
                          encoding="utf-8"))
    # 프로파일 진폭 A 의 기준 하중은 설계별 실측 최대 롤러하중이다
    QB = json.load(open(os.path.join(HERE, "부록10_NSGA", "S3_본최적화",
                                     "fujiwara", "baseline.json"),
                        encoding="utf-8"))
    rig = L.Rig()
    rig.load_case()

    # ── v1.3 대조 ────────────────────────────────────────────────
    print("=" * 76)
    print("① v1.3 기준선 — 같은 잣대로 재면 돌출이 얼마인가")
    print("=" * 76)
    v = sg.V13
    g13 = sg.bearing(v["D_pw"], v["alpha"], v["D_we"], v["L_we"])
    row13 = dict(z1=v["z1"], z2=v["z2"], D_pw_mm=v["D_pw"] * 1e3,
                 alpha=v["alpha"], D_we_mm=v["D_we"] * 1e3,
                 L_w_mm=v["L_we"] * 1e3)
    rig.build(row13)
    g = EO.geo(rig.uw.detail)
    print(f"   L_we {g['L_we']:.1f} · T {g['T']:.0f} · B {g['B']:.0f} · "
          f"C {g['C']:.0f} · cup {g['cup_deg']:.2f}° · 모따기 "
          f"{g['r_f']:.2f}")
    print(f"   궤도 축방향 {g['race_axial']:7.2f} · 롤러 투영 "
          f"{g['roller_axial']:7.2f} · 편측 여유 {g['room']:6.2f}")
    print(f"   e_off {g['e_off']:.3f} · o_off {g['o_off']:.2f} → 편심 "
          f"{g['ecc']:.2f} · **돌출 {g['over']:+.2f} mm**")
    print(f"   가설 B e_off = {g['o_off']:.2f} (Δ {g['o_off']-g['e_off']:+.2f})")
    v13 = dict(g)

    # ── 합산 ─────────────────────────────────────────────────────
    print("\n" + "=" * 76)
    print("② 신 element_offset × 비대칭 Fujiwara 프로파일 (지배 LC)")
    print("=" * 76)
    print(f"{'설계':>6s} {'현행':>9s} {'e_off만':>9s} {'프로파일만':>11s} "
          f"{'둘 다':>9s} {'단순합 예측':>11s} {'제약':>6s}")
    res = {}
    for rk in RANKS:
        gg = rule[str(rk)]
        p = best_asym(R, rk)
        out = {}
        for tag, e_mm, prof in (("현행", gg["e_off"], False),
                                ("e_off만", gg["o_off"], False),
                                ("프로파일만", gg["e_off"], True),
                                ("둘 다", gg["o_off"], True)):
            rig.build(P[rk])
            for b in rig.bs:
                b.detail.element_offset = e_mm / 1e3
            if prof and p:
                f = L.profile_fn(rig.Lwe, p[0], p[1], p[2], K1R=p[3],
                                 K2R=p[4], zmR=p[5],
                                 Q=QB[str(rk)]["P_max_N"])
                rig.set_user(f, NPTS)
            else:
                rig.set_din(0.0)
            m, _ = rig.solve(f"cb{rk}_{tag}")
            out[tag] = m
        pred = (out["현행"]["sigma_MPa"]
                + (out["e_off만"]["sigma_MPa"] - out["현행"]["sigma_MPa"])
                + (out["프로파일만"]["sigma_MPa"] - out["현행"]["sigma_MPa"]))
        ok = "OK" if out["둘 다"]["sigma_MPa"] <= 2100.0 else "위반"
        print(f"{rk:6d} {out['현행']['sigma_MPa']:9.1f} "
              f"{out['e_off만']['sigma_MPa']:9.1f} "
              f"{out['프로파일만']['sigma_MPa']:11.1f} "
              f"{out['둘 다']['sigma_MPa']:9.1f} {pred:11.1f} {ok:>6s}")
        res[rk] = {k: dict(sigma=w["sigma_MPa"], edge_L=w["edge_L_MPa"],
                           y_star=w["y_star_mm"],
                           margin_L=w["margin_L_mm"]) for k, w in out.items()}
        res[rk]["pred_sum"] = round(pred, 1)
    json.dump(dict(v13=v13, combo=res),
              open(os.path.join(OUT, "eoff_combo.json"), "w"), indent=1)
    print("\n[저장]", os.path.join(OUT, "eoff_combo.json"))


if __name__ == "__main__":
    main()
