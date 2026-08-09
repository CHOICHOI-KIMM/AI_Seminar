# -*- coding: utf-8 -*-
"""§10-11.3.6 ⑹ + §10-12.7.8 — 두 변경을 함께 건 MASTA 파일

  ① element_offset  = o_off − (D_we/2)·sin(α−β)   (§10-11.3.6 · rule_v2)
  ② 비대칭 DIN 프로파일 (확장 최적 · §10-12.7.8) — **UW 에만**

DW 는 y* 가 +2.8 ~ +18.6 % 로 거의 중앙이고 쏠림 방향도 UW 와 반대라
좌측을 깊게 판 프로파일이 해롭다. 그래서 DW 는 현행 DIN 을 유지한다.

저장 전에 `Myz_max` 로 UW·DW 의 σ·margin·y* 를 실측해 기록한다 — 두 변경을
동시에 건 상태는 측정된 적이 없다.

  python a10_save_combo.py
"""
import csv
import json
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import a10_profile_lib as L      # noqa: E402
import a10_asymdin as AD         # noqa: E402
import a10_asymdin2 as A2        # noqa: E402
import a10_eoff_v2 as EV2        # noqa: E402

OUTDIR = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "MASTA")
OUT = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "combo")
RANKS = (1, 103, 210)


def probe(rig, b):
    """한 베어링의 지표를 마지막 해석 결과에서 읽는다."""
    det = list(list(rig._csd.results_for(b))[0]
               .component_analysis_cases)[0].component_detailed_analysis
    Lwe = b.detail.effective_roller_length
    row = det.rows[0]
    els = list(row.elements)
    e = max(els, key=lambda x: x.normal_load_inner)
    off = sorted([(float(o.offset) * 1e3,
                   float(o.normal_stress_inner) / 1e6)
                  for o in e.results_at_roller_offsets])
    m = dict(sigma_MPa=round(float(det.maximum_normal_stress_inner) / 1e6, 1))
    return AD.metrics(m, off, Lwe)


def solve_keep(rig, tag):
    """duty 를 지우지 않고 결과 객체를 남긴다 (UW·DW 를 모두 읽어야 한다)."""
    dup = rig.lc.duplicate(rig.lc.design_state_load_case_group, f"cb_{tag}")
    duty = rig.dp.add_duty_cycle(f"cbd_{tag}")
    duty.add_static_load(dup)
    csd = duty.analysis_of(rig.AT.SYSTEM_DEFLECTION)
    csd.perform_analysis()
    rig._csd = csd
    return dup, duty


def main():
    os.makedirs(OUT, exist_ok=True)
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    b3 = json.load(open(os.path.join(A2.OUT, "best3.json"), encoding="utf-8"))
    rig = L.Rig()
    rig.load_case()
    rows, t0 = [], time.perf_counter()

    print(f"{'설계':>6s} {'조건':<14s} {'e_off':>8s} {'k_L/k_R':>10s} "
          f"{'δ':>5s} | {'UW σ':>8s} {'mL':>5s} {'mR':>5s} {'y*%':>6s} | "
          f"{'DW σ':>8s} {'mL':>5s} {'mR':>5s} | 제약")
    for rk in RANKS:
        g = None
        p = b3[str(rk)]["p"]
        for tag in ("offset", "offset_adin"):
            rig.build(P[rk])
            d = rig.uw.detail
            Lwe, Dwe = d.effective_roller_length, d.element_diameter
            e_new = EV2.rule_v2(d)[0]                 # [mm]
            for b in rig.bs:
                b.detail.element_offset = e_new / 1e3
            rig.set_din(0.0)                          # DW 는 끝까지 이대로
            if tag == "offset_adin":
                fn = A2.asym_din2(Lwe, Dwe, *p)
                rig.set_user(fn, A2.NPTS, targets=[rig.uw])
            dup, duty = solve_keep(rig, f"{rk}{tag}")
            mu, md = probe(rig, rig.uw), probe(rig, rig.dw)
            for x in (dup, duty):
                try:
                    x.delete()
                except Exception:
                    pass
            smax = max(mu["sigma_MPa"], md["sigma_MPa"])
            ok = "OK" if smax <= 2100.0 else f"위반 +{smax-2100:.1f}"
            kk = "1 / 1" if tag == "offset" else f"{p[0]:g} / {p[1]:g}"
            dd = 0 if tag == "offset" else p[2]
            print(f"{rk:6d} {tag:<14s} {e_new:8.2f} {kk:>10s} {dd:5g} | "
                  f"{mu['sigma_MPa']:8.1f} {mu['margin_L_lam']:5.2f} "
                  f"{mu['margin_R_lam']:5.2f} {mu['y_star_pct']:6.1f} | "
                  f"{md['sigma_MPa']:8.1f} {md['margin_L_lam']:5.2f} "
                  f"{md['margin_R_lam']:5.2f} | {ok}", flush=True)
            rows.append(dict(rank=rk, case=tag, e_off_mm=round(e_new, 2),
                             k_L=(1 if tag == "offset" else p[0]),
                             k_R=(1 if tag == "offset" else p[1]),
                             delta_mm=dd,
                             UW_sigma=mu["sigma_MPa"],
                             UW_mL_lam=mu["margin_L_lam"],
                             UW_mR_lam=mu["margin_R_lam"],
                             UW_end_L=mu["end_L_MPa"],
                             UW_end_R=mu["end_R_MPa"],
                             UW_y_star_pct=mu["y_star_pct"],
                             DW_sigma=md["sigma_MPa"],
                             DW_mL_lam=md["margin_L_lam"],
                             DW_mR_lam=md["margin_R_lam"],
                             DW_y_star_pct=md["y_star_pct"],
                             sigma_max=smax, feasible=int(smax <= 2100.0)))
            if tag == "offset_adin":
                path = os.path.join(OUTDIR, f"A10_front{rk:03d}_"
                                            f"offset_adin.masta")
                try:
                    rig.d.save(path, False)
                    print(f"       → {os.path.basename(path)} "
                          f"{os.path.getsize(path)/1e6:.1f} MB", flush=True)
                except Exception as e:
                    print(f"       !! 저장 실패 "
                          f"{str(e).splitlines()[0][:60]}")
    with open(os.path.join(OUT, "combo_check.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)
    print(f"\n[완료] {len(rows)}건 · {(time.perf_counter()-t0)/60:.1f}분")


if __name__ == "__main__":
    main()
