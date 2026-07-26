"""
부록 4 — 전 DLC dt=20 강제 + 오차최소 k(중앙타겟) 무보정 함대 수명 vs 참값
============================================================================
부록 3와 동일하되: 모든 DLC를 dt=20으로 통일하고, k는 스크리닝 dt=20 ε(k)에서
  중앙타겟(ε_Sys→1.5, [0,3]창) — 창 없으면 ε_Sys≥0 중 최소(보수측)로 선정.
목적: '가장 굵은 dt=20으로 통일'해도 함대 수명이 맞는가 + 최고속 시간 검증.
기록: 부록 3 양식(Σ D30·수명·편향·수렴) + DLC별 해석시간 t_s + 총 해석시간.
산출: 부록4_screening_dt20/ (per_dlc·fleet_summary·convergence)
"""
import csv
import math
import os
import sys
import time

import numpy as np
import psutil

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안"
         r"_피로하중 반영_유연체_FE_온도_50도_260721.Masta")
OUTDIR = os.path.join(HERE, "부록4_screening_dt20")
NBATCH = 20
DT0, DT = 0.1, 20.0
E_W = 9.0 / 8.0
MEM_LIMIT = 95.0
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")


def load_master():
    return {r["DLC"]: r for r in csv.DictReader(
        open(os.path.join(HERE, "dlc_master_summary.csv"), encoding="utf-8-sig"))}


def load_meta():
    return {r["DLC"]: r for r in csv.DictReader(
        open(os.path.join(HERE, "dlc_meta.csv"), encoding="utf-8-sig"))}


def load_raw(name):
    return [{k: float(v) for k, v in r.items()} for r in csv.DictReader(
        open(os.path.join(HERE, name, "raw.csv"), encoding="utf-8-sig"))]


def ref_d30(name, sf):
    p = os.path.join(HERE, name, "masta_ref_dmg.csv")
    if not os.path.isfile(p):
        return None
    u = d = 0.0
    for r in csv.DictReader(open(p, encoding="utf-8-sig")):
        u += float(r["dmg_UW"]); d += float(r["dmg_DW"])
    return u * sf, d * sf


def select_k_dt20(name):
    """스크리닝 dt=20 ε(k)에서 중앙타겟 k. 창 없으면 ε_Sys≥0 중 최소(보수측)."""
    p = os.path.join(HERE, name, "screen_eps_map.csv")
    rows = [(float(r["k"]), float(r["eps_UW_pct"]), float(r["eps_Sys_pct"]))
            for r in csv.DictReader(open(p, encoding="utf-8-sig"))
            if float(r["dt_s"]) == DT]
    if len(rows) < 2:
        return None, "no-map"
    rows.sort()
    ks = np.array([r[0] for r in rows])
    eU = np.array([r[1] for r in rows])
    eS = np.array([r[2] for r in rows])
    kf = np.linspace(ks.min(), ks.max(), 3001)
    u, s = np.interp(kf, ks, eU), np.interp(kf, ks, eS)
    ok = (u >= 0) & (u <= 3) & (s >= 0) & (s <= 3)
    if ok.any():
        idx = np.where(ok)[0]
        j = idx[np.argmin(np.abs(s[idx] - 1.5))]
        return round(float(kf[j]), 2), "center"
    cons = s >= 0
    if cons.any():
        idx = np.where(cons)[0]
        j = idx[np.argmin(s[idx])]           # ε_Sys≥0 중 최소(0 최근접, 보수)
        return round(float(kf[j]), 2), "cons"
    j = int(np.argmin(np.abs(s)))            # 전부 음수 → |ε_Sys| 최소
    return round(float(kf[j]), 2), "abs"


def bin_reps(data, k):
    kp = int(round(DT / DT0))
    n = len(data)
    nb = n // kp
    if nb < 2:
        nb = 1
    edges = [(b * kp, (b + 1) * kp) for b in range(nb)]
    if edges and edges[-1][1] < n:
        edges[-1] = (edges[-1][0], n)
    out = []
    for bi, (i0, i1) in enumerate(edges):
        m = i1 - i0
        rec = {key: sum(data[i][key] for i in range(i0, i1)) / m
               for key in ("rpm", "Mx")}
        for key in KSIG:
            mu = sum(data[i][key] for i in range(i0, i1)) / m
            var = sum((data[i][key] - mu) ** 2 for i in range(i0, i1)) / m
            rec[key] = mu + math.copysign(1.0, mu) * k * math.sqrt(var)
        out.append((bi, abs(rec["rpm"]) / 60.0 * (m * DT0), rec))
    return out


def lsys(lu, ld):
    return (lu ** -E_W + ld ** -E_W) ** (-1.0 / E_W)


def main():
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

    os.makedirs(OUTDIR, exist_ok=True)
    master, meta = load_master(), load_meta()
    targets = [name for name, m in master.items()
               if m.get("valid") == "1"
               and os.path.isfile(os.path.join(HERE, name, "masta_ref_dmg.csv"))]
    print(f"[부록4] 대상 {len(targets)} DLC (전 DLC dt=20 강제)")

    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    dp = asm.design_properties
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
    bearings = {("UW" if "UW" in str(b) else "DW"): b
                for b in asm.all_parts_of_type_bearing()}
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    ds = lc0.design_state_load_case_group
    print("[모델] 로드 완료")

    def analyze(reps, tag):
        dmg = {"UW": [], "DW": []}
        t_anal = 0.0
        for b0 in range(0, len(reps), NBATCH):
            chunk = reps[b0:b0 + NBATCH]
            lcs = []
            for cid, rev, rec in chunk:
                lc = lc0.duplicate(ds, f"ap4_{tag}_{cid}")
                mf.set_loads(lc, pl, ipl, rec)
                lcs.append(lc)
            duty = dp.add_duty_cycle(f"ap4dc_{tag}_{b0}")
            for lc in lcs:
                duty.add_static_load(lc)
            t0 = time.perf_counter()
            csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            csd.perform_analysis()
            t_anal += time.perf_counter() - t0
            for key, b in bearings.items():
                subs = list(list(csd.results_for(b))[0].component_analysis_cases)
                for (cid, rev, rec), sub in zip(chunk, subs):
                    l10 = (sub.component_detailed_analysis
                           .isots162812008.modified_reference_rating_life_cycles)
                    dmg[key].append(rev / l10 if (l10 and l10 > 0) else 0.0)
            for lc in lcs:
                try:
                    lc.delete()
                except Exception:
                    pass
            try:
                duty.delete()
            except Exception:
                pass
            if psutil.virtual_memory().percent > MEM_LIMIT:
                raise MemoryError("메모리 95% 초과")
        return dmg, t_anal

    rows = []
    t0 = time.time()
    for i, name in enumerate(targets, 1):
        sf = float(meta[name]["ScaleFactor"])
        rd = ref_d30(name, sf)
        if rd is None or rd[0] <= 0:
            continue
        d30rU, d30rD = rd
        k, ktag = select_k_dt20(name)
        if k is None:
            continue
        reps = bin_reps(load_raw(name), k)
        dmg, t_anal = analyze(reps, name[-6:].replace(".", ""))
        d30cU, d30cD = sum(dmg["UW"]) * sf, sum(dmg["DW"]) * sf
        eU = (d30cU / d30rU - 1) * 100
        eD = (d30cD / d30rD - 1) * 100 if d30rD > 0 else float("nan")
        lU_r, lD_r = 30.0 / d30rU, 30.0 / d30rD
        lU_c, lD_c = 30.0 / d30cU, 30.0 / d30cD
        eS = (lsys(lU_r, lD_r) / lsys(lU_c, lD_c) - 1) * 100
        rows.append(dict(DLC=name, dt=20, k=k, ksel=ktag, nbin=len(reps),
                         D30_ref_UW=d30rU, D30_ref_DW=d30rD,
                         D30_scr_UW=d30cU, D30_scr_DW=d30cD,
                         eps_UW=eU, eps_DW=eD, eps_Sys=eS,
                         life_ref_UW=lU_r, life_scr_UW=lU_c, t_s=t_anal))
        if i % 10 == 0 or i == len(targets):
            print(f"  [{i}/{len(targets)}] {name} k={k:g}({ktag}) {len(reps)}빈 "
                  f"εUW={eU:+.2f}% t={t_anal:.1f}s ({time.time()-t0:.0f}s)")

    with open(os.path.join(OUTDIR, "per_dlc.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader(); w.writerows(rows)

    sU_r = sum(r["D30_ref_UW"] for r in rows)
    sD_r = sum(r["D30_ref_DW"] for r in rows)
    sU_c = sum(r["D30_scr_UW"] for r in rows)
    sD_c = sum(r["D30_scr_DW"] for r in rows)
    LUr, LDr = 30.0 / sU_r, 30.0 / sD_r
    LUc, LDc = 30.0 / sU_c, 30.0 / sD_c
    t_total = sum(r["t_s"] for r in rows)
    with open(os.path.join(OUTDIR, "fleet_summary.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.writer(f)
        w.writerow(["n_dlc", "sumD30_UW_ref", "sumD30_DW_ref", "sumD30_UW_scr",
                    "sumD30_DW_scr", "life_UW_ref", "life_Sys_ref", "life_UW_scr",
                    "life_Sys_scr", "bias_UW_pct", "bias_Sys_life_pct",
                    "total_analysis_s", "mean_per_dlc_s"])
        w.writerow([len(rows), sU_r, sD_r, sU_c, sD_c, LUr, lsys(LUr, LDr),
                    LUc, lsys(LUc, LDc), (sU_c / sU_r - 1) * 100,
                    (lsys(LUr, LDr) / lsys(LUc, LDc) - 1) * 100,
                    t_total, t_total / len(rows)])

    order = sorted(rows, key=lambda r: -r["D30_ref_UW"])
    Lsys_scr_final = lsys(LUc, LDc)
    cU_r = cD_r = cU_c = cD_c = 0.0
    with open(os.path.join(OUTDIR, "convergence.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.writer(f)
        w.writerow(["n", "life_UW_ref", "life_Sys_ref", "life_UW_scr",
                    "life_Sys_scr", "bias_Sys_life_pct", "vs_final_scr_Sys_pct"])
        for n, r in enumerate(order, 1):
            cU_r += r["D30_ref_UW"]; cD_r += r["D30_ref_DW"]
            cU_c += r["D30_scr_UW"]; cD_c += r["D30_scr_DW"]
            lur, ldr = 30.0 / cU_r, 30.0 / cD_r
            luc, ldc = 30.0 / cU_c, 30.0 / cD_c
            w.writerow([n, lur, lsys(lur, ldr), luc, lsys(luc, ldc),
                        (lsys(lur, ldr) / lsys(luc, ldc) - 1) * 100,
                        (lsys(luc, ldc) / Lsys_scr_final - 1) * 100])

    from collections import Counter
    kc = Counter(r["ksel"] for r in rows)
    print(f"\n[완료] {len(rows)} DLC · 총 {time.time()-t0:.0f}s")
    print(f"  k 선정: {dict(kc)} (center=창내중앙 · cons=보수측 · abs=절대최소)")
    print(f"  총 해석시간 {t_total:.0f}s · DLC당 평균 {t_total/len(rows):.1f}s")
    print(f"  Σ D30_UW 참값 {sU_r:.3f} vs 스크리닝 {sU_c:.3f} → 편향 {(sU_c/sU_r-1)*100:+.2f}%")
    print(f"  함대 Sys 참값 {lsys(LUr,LDr):.2f} vs 스크리닝 {lsys(LUc,LDc):.2f} yr "
          f"(편향 {(lsys(LUr,LDr)/lsys(LUc,LDc)-1)*100:+.2f}%)")


if __name__ == "__main__":
    main()
