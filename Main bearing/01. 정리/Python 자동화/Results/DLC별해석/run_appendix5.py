"""
부록 5 Step 2 — v1.4 프리로드 모델 전 111 DLC dt=20 MASTA 본해석 (참값 없음)
============================================================================
부록 4와 동일 절차(dt=20 통일 + 스크리닝 중앙타겟 k)를 신규 모델에 적용한다.
참값(dt=0.1 점별) 해석은 사용자 지시로 수행하지 않으므로 편향 검증은 없고,
절대 손상·수명·기여도만 산출한다.

모델: v1.4 3안 · 베어링 크기 확대 · 롤러 확대 · 프리로드(SOLID, 축변위 0.5 mm) · 50°C
k   : 부록5_preload_dt20/screen_k_dt20.csv (Step 1, 신규 C·Cu 반영 스크리닝)
산출: 부록5_preload_dt20/ (per_dlc·fleet_summary·convergence)
"""
import csv
import math
import os
import sys
import time

import psutil

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_프리로드 적용_온도_50도_260726.Masta")
OUTDIR = os.path.join(HERE, "부록5_preload_dt20")
NBATCH = 20
DT0, DT = 0.1, 20.0
E_W = 9.0 / 8.0
DESIGN_YEARS = 30.0
MEM_LIMIT = 95.0
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")


def load_meta():
    return {r["DLC"]: r for r in csv.DictReader(
        open(os.path.join(HERE, "dlc_meta.csv"), encoding="utf-8-sig"))}


def load_k():
    out = {}
    for r in csv.DictReader(open(os.path.join(OUTDIR, "screen_k_dt20.csv"),
                                 encoding="utf-8-sig")):
        if r["valid"] != "1":
            continue
        out[r["DLC"]] = (float(r["k"]), r["ksel"],
                         float(r["eps_UW"]), float(r["eps_Sys"]))
    return out


def load_raw(name):
    return [{k: float(v) for k, v in r.items()} for r in csv.DictReader(
        open(os.path.join(HERE, name, "raw.csv"), encoding="utf-8-sig"))]


def bin_reps(data, k):
    kp = int(round(DT / DT0))
    n = len(data)
    nb = max(n // kp, 1)
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
    meta, kmap = load_meta(), load_k()
    targets = [n for n in sorted(meta) if n in kmap]
    print(f"[부록5] 대상 {len(targets)} DLC (dt=20 통일 · 참값 없음)")
    print(f"[모델] {os.path.basename(MODEL)}")

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
                lc = lc0.duplicate(ds, f"ap5_{tag}_{cid}")
                mf.set_loads(lc, pl, ipl, rec)
                lcs.append(lc)
            duty = dp.add_duty_cycle(f"ap5dc_{tag}_{b0}")
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
        k, ktag, e_u, e_s = kmap[name]
        reps = bin_reps(load_raw(name), k)
        dmg, t_anal = analyze(reps, name[-6:].replace(".", ""))
        d30U, d30D = sum(dmg["UW"]) * sf, sum(dmg["DW"]) * sf
        lU = DESIGN_YEARS / d30U if d30U > 0 else float("inf")
        lD = DESIGN_YEARS / d30D if d30D > 0 else float("inf")
        rows.append(dict(DLC=name, dt=20, k=k, ksel=ktag, nbin=len(reps),
                         ScaleFactor=sf,
                         scr_eps_UW=e_u, scr_eps_Sys=e_s,
                         D30_UW=d30U, D30_DW=d30D,
                         life_UW_yr=lU, life_DW_yr=lD, life_Sys_yr=lsys(lU, lD),
                         t_s=t_anal))
        if i % 10 == 0 or i == len(targets):
            print(f"  [{i}/{len(targets)}] {name} k={k:g}({ktag}) {len(reps)}빈 "
                  f"D30_UW={d30U:.4f} t={t_anal:.1f}s ({time.time()-t0:.0f}s)",
                  flush=True)

    sU = sum(r["D30_UW"] for r in rows)
    sD = sum(r["D30_DW"] for r in rows)
    LU, LD = DESIGN_YEARS / sU, DESIGN_YEARS / sD
    LS = lsys(LU, LD)
    for r in rows:
        r["contrib_UW_pct"] = r["D30_UW"] / sU * 100
        r["contrib_DW_pct"] = r["D30_DW"] / sD * 100

    with open(os.path.join(OUTDIR, "per_dlc.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader(); w.writerows(rows)

    t_total = sum(r["t_s"] for r in rows)
    with open(os.path.join(OUTDIR, "fleet_summary.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.writer(f)
        w.writerow(["n_dlc", "sumD30_UW", "sumD30_DW", "life_UW_yr", "life_DW_yr",
                    "life_Sys_yr", "total_analysis_s", "mean_per_dlc_s",
                    "min_per_dlc_s", "max_per_dlc_s"])
        ts = [r["t_s"] for r in rows]
        w.writerow([len(rows), sU, sD, LU, LD, LS, t_total, t_total / len(rows),
                    min(ts), max(ts)])

    order = sorted(rows, key=lambda r: -r["D30_UW"])
    cU = cD = 0.0
    with open(os.path.join(OUTDIR, "convergence.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.writer(f)
        w.writerow(["n", "DLC", "cum_D30_UW", "life_UW_yr", "life_Sys_yr",
                    "vs_final_Sys_pct", "cum_contrib_UW_pct"])
        for n, r in enumerate(order, 1):
            cU += r["D30_UW"]; cD += r["D30_DW"]
            lu, ld = DESIGN_YEARS / cU, DESIGN_YEARS / cD
            w.writerow([n, r["DLC"], cU, lu, lsys(lu, ld),
                        (lsys(lu, ld) / LS - 1) * 100, cU / sU * 100])

    from collections import Counter
    kc = Counter(r["ksel"] for r in rows)
    print(f"\n[완료] {len(rows)} DLC · 경과 {time.time()-t0:.0f}s")
    print(f"  k 선정: {dict(kc)}")
    print(f"  총 해석시간 {t_total:.0f}s · DLC당 평균 {t_total/len(rows):.1f}s "
          f"(범위 {min(ts):.1f}~{max(ts):.1f}s)")
    print(f"  Σ D30_UW = {sU:.4f} · Σ D30_DW = {sD:.4f}")
    print(f"  함대 수명  UW {LU:.2f} yr · DW {LD:.2f} yr · Sys {LS:.2f} yr")


if __name__ == "__main__":
    main()
