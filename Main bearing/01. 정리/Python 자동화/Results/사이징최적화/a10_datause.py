# -*- coding: utf-8 -*-
"""§10-12.7.9 ⑷ — data_to_use 별 지표·형상·속도 비교 (#1)

  ACTUAL_DATA              우리가 넣은 201점 그대로 (현행)
  SMOOTHED                 평활
  FITTED_STANDARD_PROFILE  표준 프로파일(AUTO)로 되피팅

세 설정이 ⑴ 같은 것을 푸는가(지표·형상) ⑵ 빨라지는가(DLC 5건 피로)를
함께 본다. 빨라도 다른 것을 풀면 쓸 수 없다.
"""
import csv
import json
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import a10_profile_lib as L      # noqa: E402
import a10_asymdin as AD         # noqa: E402
import a10_asymdin2 as A2        # noqa: E402
import a10_eoff_v2 as EV2        # noqa: E402
import a10_save_combo as SC      # noqa: E402
import a10_life79 as L79         # noqa: E402
import run_appendix1_profile as A1   # noqa: E402

OUT = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "datause")
RANK = 1
NDLC = 5


def main():
    os.makedirs(OUT, exist_ok=True)
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    b3 = json.load(open(os.path.join(A2.OUT, "best3.json"), encoding="utf-8"))
    p = b3[str(RANK)]["p"]
    rig = L.Rig()
    lc0 = L79.set_70c(rig.asm)
    rig.load_case()                       # Myz_max (지표용)
    meta = A1.load_meta()
    kmap = L79.load_k70()
    short = [n for n in sorted(meta) if n in kmap][:NDLC]
    kmap5 = {n: kmap[n] for n in short}
    meta5 = {n: meta[n] for n in short}
    print(f"[속도 측정] DLC {NDLC}건 — {', '.join(short)}")

    from mastapy.bearings.roller_bearing_profiles import (ProfileDataToUse,
                                                          ProfileToFit)
    MODES = (("FITTED(AUTO)", ProfileDataToUse.FITTED_STANDARD_PROFILE,
              ProfileToFit.AUTO),)
    rows, shapes = [], {}
    for nm, mode, fit in MODES:
        rig.build(P[RANK])
        d = rig.uw.detail
        e = EV2.rule_v2(d)[0]
        for b in rig.bs:
            b.detail.element_offset = e / 1e3
        rig.set_din(0.0)
        rig.set_user(A2.asym_din2(d.effective_roller_length,
                                  d.element_diameter, *p),
                     A2.NPTS, targets=[rig.uw])
        up = rig.uw.detail.roller_profile_set.active_profile
        try:
            up.data_to_use = mode          # 먼저 모드를 바꿔야
            if fit is not None:            # profile_to_fit 가 유효해진다
                up.profile_to_fit = fit
        except Exception as ex:
            print(f"  {nm}: 설정 실패 {str(ex).splitlines()[0][:50]}")
            continue

        # ① 실제 적용된 형상
        shp = [(round(float(q.offset_from_roller_centre) * 1e3, 3),
                round(float(q.roller_deviation) * 1e6, 3))
               for q in rig.uw.detail.outer_race_and_roller_profiles]
        shapes[nm] = shp
        zL = shp[0][1]
        zR = shp[-1][1]
        asym = abs(zL - zR) / max(zL, zR, 1e-9) * 100

        # ② 지표
        rig.load_case()
        dup, duty = SC.solve_keep(rig, f"du_{nm[:4]}")
        mu, md = SC.probe(rig, rig.uw), SC.probe(rig, rig.dw)
        for x in (dup, duty):
            try:
                x.delete()
            except Exception:
                pass

        # ③ 속도 (DLC 5건)
        t1 = time.perf_counter()
        try:
            L79.fatigue(rig.asm, rig.bs, kmap5, meta5, f"du{nm[:4]}", lc0)
            dt = time.perf_counter() - t1
        except Exception as ex:
            print(f"  {nm}: 피로 실패 {str(ex).splitlines()[0][:50]}")
            dt = float("nan")

        rows.append(dict(mode=nm, z_end_L=zL, z_end_R=zR,
                         asym_pct=round(asym, 1),
                         UW_sigma=mu["sigma_MPa"], UW_end_L=mu["end_L_MPa"],
                         UW_end_R=mu["end_R_MPa"],
                         UW_mL=mu["margin_L_lam"], UW_mR=mu["margin_R_lam"],
                         UW_y=mu["y_star_pct"],
                         DW_sigma=md["sigma_MPa"], DW_end_L=md["end_L_MPa"],
                         DW_end_R=md["end_R_MPa"],
                         DW_mL=md["margin_L_lam"], DW_mR=md["margin_R_lam"],
                         DW_y=md["y_star_pct"],
                         sec_5dlc=round(dt, 1)))
        print(f"\n=== {nm} ===")
        print(f"  형상 끝단낙차 좌 {zL:8.1f} / 우 {zR:8.1f} um · 비대칭도 "
              f"{asym:5.1f} %")
        print(f"  UW  σ {mu['sigma_MPa']:7.1f} · end {mu['end_L_MPa']:7.1f}/"
              f"{mu['end_R_MPa']:7.1f} · margin {mu['margin_L_lam']:5.2f}/"
              f"{mu['margin_R_lam']:5.2f} · y* {mu['y_star_pct']:5.1f}%")
        print(f"  DW  σ {md['sigma_MPa']:7.1f} · end {md['end_L_MPa']:7.1f}/"
              f"{md['end_R_MPa']:7.1f} · margin {md['margin_L_lam']:5.2f}/"
              f"{md['margin_R_lam']:5.2f} · y* {md['y_star_pct']:5.1f}%")
        print(f"  속도 DLC {NDLC}건 {dt:6.1f}초", flush=True)

    with open(os.path.join(OUT, "datause_fitted.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)
    json.dump(shapes, open(os.path.join(OUT, "shapes_fitted.json"), "w"), indent=1)
    print(f"\n[저장] {OUT}")


if __name__ == "__main__":
    main()
