"""
DLC별해석 대시보드용 데이터 빌드 — CSV 원천 → dashboard_data.json (MASTA 불필요)
================================================================================
원천: dlc_master_summary.csv(스크리닝) + DLC별 masta_best_summary.csv(실측) +
      screen_eps_map.csv(예측 보간) + dlc_meta.csv(가동시간) + fleet_masta_partial.csv(부분합)
산출: 빈별 D30 · ε 예측vs실측 산점 · 손상/가동시간 커버리지 · KPI · 판정 통계
"""
import csv
import glob
import json
import os
from datetime import datetime

HERE = os.path.dirname(os.path.abspath(__file__))
E_W = 9.0 / 8.0
DTS = [20, 10, 4, 2, 1, 0.6]


def bin_of(name):
    core = name[3:] if name.startswith("DLC") else name
    fam = core.rsplit("-s", 1)[0]      # 1.2-d
    return fam


def load_csv(path):
    return list(csv.DictReader(open(path, encoding="utf-8-sig")))


def interp_pred(name, dt, k):
    """screen_eps_map.csv에서 (dt,k) 예측 εUW·εSys 선형보간."""
    p = os.path.join(HERE, name, "screen_eps_map.csv")
    if not os.path.isfile(p):
        return None, None
    pts = [(float(r["k"]), float(r["eps_UW_pct"]), float(r["eps_Sys_pct"]))
           for r in load_csv(p) if float(r["dt_s"]) == dt]
    if len(pts) < 2:
        return None, None
    pts.sort()
    ks = [x[0] for x in pts]
    if k <= ks[0]:
        lo, hi = pts[0], pts[1]
    elif k >= ks[-1]:
        lo, hi = pts[-2], pts[-1]
    else:
        i = max(i for i in range(len(ks)) if ks[i] <= k)
        lo, hi = pts[i], pts[i + 1]
    t = (k - lo[0]) / (hi[0] - lo[0]) if hi[0] != lo[0] else 0.0
    return lo[1] + t * (hi[1] - lo[1]), lo[2] + t * (hi[2] - lo[2])


def build_validation(master, meta):
    """부록1 강체모델 검증 (DLC1.2-d-s1): 반력오차 통계 + 총량비교."""
    name = "DLC1.2-d-s1"
    rp = os.path.join(HERE, name, "appendix1_reactions.csv")
    lp = os.path.join(HERE, name, "appendix1_life.csv")
    bp = os.path.join(HERE, name, "masta_best_summary.csv")
    if not (os.path.isfile(rp) and os.path.isfile(lp)):
        return None
    rows = load_csv(rp)
    stats = {}
    for bk in ("UW", "DW"):
        for c in ("RX", "RY", "RZ", "Fr"):
            vals = [float(r[f"{bk}_{c}_err_pct"]) for r in rows]
            stats[f"{bk}_{c}"] = [round(sum(vals) / len(vals), 2),
                                  round(min(vals), 1), round(max(vals), 1)]
    # 총량 (핀지지 vs MASTA), 참값은 best_summary
    lrows = load_csv(lp)
    sf = float(meta[name]["ScaleFactor"])
    dU_pin = sum(float(r["UW_dmg_pin"]) for r in lrows) * sf
    dU_ma = sum(float(r["UW_dmg_masta"]) for r in lrows) * sf
    dD_pin = sum(float(r["DW_dmg_pin"]) for r in lrows) * sf
    dD_ma = sum(float(r["DW_dmg_masta"]) for r in lrows) * sf
    b = load_csv(bp)[0]
    dU_ref, dD_ref = float(b["D30_UW_ref"]), float(b["D30_DW_ref"])

    def life(d30U, d30D):
        lU, lD = 30.0 / d30U, 30.0 / d30D
        return lU, lD, (lU ** -E_W + lD ** -E_W) ** (-1 / E_W)

    lp_ = life(dU_pin, dD_pin)
    lm_ = life(dU_ma, dD_ma)
    lr_ = life(dU_ref, dD_ref)
    return {
        "dlc": name, "dt": 20, "k": 0.26, "nbin": len(rows),
        "W_kN": 424.0, "zW_m": 18.78,
        "react": stats,
        "totals": {
            "D30_UW": [round(dU_pin, 4), round(dU_ma, 4), round(dU_ref, 4)],
            "D30_DW": [round(dD_pin, 4), round(dD_ma, 4), round(dD_ref, 4)],
            "life_UW": [round(lp_[0], 1), round(lm_[0], 1), round(lr_[0], 1)],
            "life_DW": [round(lp_[1], 1), round(lm_[1], 1), round(lr_[1], 1)],
            "life_Sys": [round(lp_[2], 1), round(lm_[2], 1), round(lr_[2], 1)],
        },
        "bias_UW_pct": round((dU_pin / dU_ma - 1) * 100, 1),
        "bias_DW_pct": round((dD_pin / dD_ma - 1) * 100, 1),
    }


def main():
    master = {r["DLC"]: r for r in load_csv(os.path.join(HERE, "dlc_master_summary.csv"))}
    meta = {r["DLC"]: r for r in load_csv(os.path.join(HERE, "dlc_meta.csv"))}

    # ── 실측 완료 DLC ──
    done = {}
    for p in glob.glob(os.path.join(HERE, "*", "masta_best_summary.csv")):
        name = os.path.basename(os.path.dirname(p))
        r = load_csv(p)[0]
        done[name] = {k: v for k, v in r.items()}

    # ── 빈별 집계 (실측) ──
    bins = {}
    for name, r in done.items():
        b = bin_of(name)
        d30u = float(r["D30_UW_ref"])
        d30d = float(r["D30_DW_ref"])
        e = bins.setdefault(b, {"bin": b, "D30_UW": 0.0, "D30_DW": 0.0,
                                "n": 0, "n_pass": 0})
        e["D30_UW"] += d30u
        e["D30_DW"] += d30d
        e["n"] += 1
        e["n_pass"] += int(float(r["pass"]))
    bin_list = sorted(bins.values(), key=lambda x: x["bin"])

    # ── ε 예측 vs 실측 산점 ──
    scatter = []
    for name, r in done.items():
        dt, k = float(r["dt"]), float(r["k"])
        pU, pS = interp_pred(name, dt, k)
        if pU is None:
            continue
        scatter.append({
            "dlc": name, "bin": bin_of(name),
            "pred_UW": round(pU, 3), "meas_UW": round(float(r["eps_UW"]), 3),
            "pred_Sys": round(pS, 3), "meas_Sys": round(float(r["eps_Sys"]), 3),
            "pass": int(float(r["pass"])), "ncorr": str(r["ncorr"])})

    # ── 커버리지 (스크리닝 D30 내림차순) ──
    valid = [(n, float(m["D30_UW_scr"]),
              float(m["ScaleFactor"]) * (int(float(meta[n]["n_pts"])) - 1)
              * float(meta[n]["dt0_s"]))
             for n, m in master.items()
             if m.get("valid") == "1" and m.get("D30_UW_scr")]
    valid.sort(key=lambda x: -x[1])
    totD = sum(v[1] for v in valid)
    totH = sum(v[2] for v in valid)
    cov = []
    cD = cH = 0.0
    for i, (n, d, h) in enumerate(valid, 1):
        cD += d
        cH += h
        cov.append({"n": i, "dmg_pct": round(cD / totD * 100, 2),
                    "time_pct": round(cH / totH * 100, 2)})

    # ── KPI ──
    part = None
    pp = os.path.join(HERE, "fleet_masta_partial.csv")
    if os.path.exists(pp):
        part = load_csv(pp)[0]
    accs = [abs(s["meas_UW"] - s["pred_UW"]) for s in scatter]
    sumU = sum(float(r["D30_UW_ref"]) for r in done.values())
    sumUc = sum(float(r["D30_UW_cmb"]) for r in done.values())
    n_pass = sum(int(float(r["pass"])) for r in done.values())
    n_corr = sum(1 for r in done.values()
                 if str(r["ncorr"]) not in ("0",) and int(float(r["pass"])) == 1)
    n_fail = sum(1 for r in done.values() if int(float(r["pass"])) == 0)
    kpi = {
        "n_done": len(done), "n_total": len(master),
        "n_pass": n_pass, "n_corr": n_corr, "n_fail": n_fail,
        "life_UW": round(float(part["life_UW"]), 2) if part else None,
        "life_Sys": round(float(part["life_Sys"]), 2) if part else None,
        "life_DW": round(float(part["life_DW"]), 2) if part else None,
        "sumD30_UW": round(sumU, 3),
        "acc_mean_pp": round(sum(accs) / len(accs), 3) if accs else None,
        "acc_max_pp": round(max(accs), 3) if accs else None,
        "combo_bias_pct": round((sumUc / sumU - 1) * 100, 2) if sumU else None,
        "cov_dmg_pct": round(sumU / totD * 100 / (totD / totD), 2) if False else None,
    }
    # 완료분이 차지하는 스크리닝 손상 커버리지
    done_scr = sum(float(master[n]["D30_UW_scr"]) for n in done
                   if master.get(n, {}).get("D30_UW_scr"))
    kpi["done_cov_pct"] = round(done_scr / totD * 100, 1)

    # ── 부록1 강체모델 검증 (DLC1.2-d-s1) ──
    validation = build_validation(master, meta)

    data = {
        "generated": datetime.now().strftime("%Y-%m-%d %H:%M"),
        "kpi": kpi, "bins": bin_list, "scatter": scatter,
        "coverage": cov,
        "coverage_marks": {"50": None, "80": None, "90": None, "95": None},
        "validation": validation,
    }
    for tgt in (50, 80, 90, 95):
        for c in cov:
            if c["dmg_pct"] >= tgt:
                data["coverage_marks"][str(tgt)] = c
                break

    out = os.path.join(HERE, "dashboard_data.json")
    json.dump(data, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(f"[저장] {out}")
    print(f"  완료 {kpi['n_done']}/{kpi['n_total']} · 합격 {n_pass} "
          f"(보정 {n_corr}) · 불합격 {n_fail}")
    print(f"  부분수명 UW {kpi['life_UW']} / Sys {kpi['life_Sys']} yr · "
          f"정확도 평균 {kpi['acc_mean_pp']}%p 최대 {kpi['acc_max_pp']}%p")
    print(f"  빈 {len(bin_list)}종 · 산점 {len(scatter)}점 · 완료 손상커버 "
          f"{kpi['done_cov_pct']}%")


if __name__ == "__main__":
    main()
