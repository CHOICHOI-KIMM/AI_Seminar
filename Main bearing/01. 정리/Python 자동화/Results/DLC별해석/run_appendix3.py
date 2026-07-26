"""
부록 3 — 스크리닝 채택 (dt,k) 무보정 조합 수명 vs 참값 함대 비교 (동시 실행 안전)
================================================================================
목적: 각 DLC를 '1차 스크리닝이 채택한 (dt,k)'로 조합 수명 해석(보정 없이) →
      참값(dt=0.1, 기존 masta_ref_dmg.csv 재사용) 대비 함대 편향 검증.
      "MASTA 보정을 생략하고 스크리닝 (dt,k)만 써도 함대 수명이 맞는가"에 답.
동시 실행: 별도 프로세스(자체 Design)·고유 듀티명(pid)·전용 출력. .md는 미기록(완주 후 작성).
출력: 부록3_screening_life/ (per_dlc.csv · fleet_summary.csv · convergence.csv) + 자체 로그.
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
         r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안"
         r"_피로하중 반영_유연체_FE_온도_50도_260721.Masta")
OUTDIR = os.path.join(HERE, "부록3_screening_life")
NBATCH = 20
DT0 = 0.1
E_W = 9.0 / 8.0
MEM_LIMIT = 95.0
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")
PID = os.getpid()


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
    """기존 참값 손상 재사용 → (D30_UW, D30_DW)."""
    p = os.path.join(HERE, name, "masta_ref_dmg.csv")
    if not os.path.isfile(p):
        return None
    u = d = 0.0
    for r in csv.DictReader(open(p, encoding="utf-8-sig")):
        u += float(r["dmg_UW"]); d += float(r["dmg_DW"])
    return u * sf, d * sf


def bin_reps(data, dt, k):
    kp = int(round(dt / DT0))
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
    master = load_master()
    meta = load_meta()
    # 참값 확보(완료) DLC = masta_ref_dmg.csv 존재 & 스크리닝 (dt,k) 있음
    targets = []
    for name, m in master.items():
        if m.get("valid") != "1" or m.get("best_dt_s") in ("", "None", None):
            continue
        if not os.path.isfile(os.path.join(HERE, name, "masta_ref_dmg.csv")):
            continue
        targets.append((name, float(m["best_dt_s"]), float(m["best_k"])))
    print(f"[부록3] 대상 {len(targets)} DLC · PID {PID}")

    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    dp = asm.design_properties
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
    bearings = {("UW" if "UW" in str(b) else "DW"): b
                for b in asm.all_parts_of_type_bearing()}
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    ds = lc0.design_state_load_case_group
    print("[모델] 로드 완료 (별도 세션)")

    def analyze(reps, tag):
        dmg = {"UW": [], "DW": []}
        for b0 in range(0, len(reps), NBATCH):
            chunk = reps[b0:b0 + NBATCH]
            lcs = []
            for cid, rev, rec in chunk:
                lc = lc0.duplicate(ds, f"ap3_{PID}_{tag}_{cid}")
                mf.set_loads(lc, pl, ipl, rec)
                lcs.append(lc)
            duty = dp.add_duty_cycle(f"ap3dc_{PID}_{tag}_{b0}")
            for lc in lcs:
                duty.add_static_load(lc)
            csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            csd.perform_analysis()
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
        return dmg

    # 병합모드: 기존 per_dlc.csv 재사용, 누락분만 신규 해석
    existing = {}
    perp = os.path.join(OUTDIR, "per_dlc.csv")
    if os.path.isfile(perp):
        numf = ("dt", "k", "nbin", "D30_ref_UW", "D30_ref_DW", "D30_scr_UW",
                "D30_scr_DW", "eps_UW", "eps_DW", "eps_Sys", "life_ref_UW",
                "life_scr_UW", "t_s")
        for r in csv.DictReader(open(perp, encoding="utf-8-sig")):
            existing[r["DLC"]] = {k: (float(v) if k in numf else v)
                                  for k, v in r.items()}
        print(f"[병합] 기존 {len(existing)}개 재사용, 누락분만 해석")

    rows = []
    t0 = time.time()
    for i, (name, dt, k) in enumerate(targets, 1):
        if name in existing:
            rows.append(existing[name])
            continue
        sf = float(meta[name]["ScaleFactor"])
        rd = ref_d30(name, sf)
        if rd is None or rd[0] <= 0:
            continue
        d30rU, d30rD = rd
        data = load_raw(name)
        reps = bin_reps(data, dt, k)
        tt = time.perf_counter()
        dmg = analyze(reps, name[-6:].replace(".", ""))
        dtc = time.perf_counter() - tt
        d30cU = sum(dmg["UW"]) * sf
        d30cD = sum(dmg["DW"]) * sf
        eU = (d30cU / d30rU - 1) * 100
        eD = (d30cD / d30rD - 1) * 100 if d30rD > 0 else float("nan")
        lU_r, lD_r = 30.0 / d30rU, 30.0 / d30rD
        lU_c, lD_c = 30.0 / d30cU, 30.0 / d30cD
        eS = (lsys(lU_r, lD_r) / lsys(lU_c, lD_c) - 1) * 100
        rows.append(dict(DLC=name, dt=dt, k=k, nbin=len(reps),
                         D30_ref_UW=d30rU, D30_ref_DW=d30rD,
                         D30_scr_UW=d30cU, D30_scr_DW=d30cD,
                         eps_UW=eU, eps_DW=eD, eps_Sys=eS,
                         life_ref_UW=lU_r, life_scr_UW=lU_c, t_s=dtc))
        if i % 5 == 0 or i == len(targets):
            print(f"  [{i}/{len(targets)}] {name} dt={dt:g} k={k:g} "
                  f"{len(reps)}빈 εUW={eU:+.2f}% ({time.time()-t0:.0f}s)")
        # 중간 저장(중단 대비)
        with open(os.path.join(OUTDIR, "per_dlc.csv"), "w", newline="",
                  encoding="utf-8-sig") as f:
            w = csv.DictWriter(f, fieldnames=list(rows[0]))
            w.writeheader(); w.writerows(rows)

    # per_dlc 최종 일괄 저장 (병합 시 existing 스킵으로 누락 방지)
    with open(os.path.join(OUTDIR, "per_dlc.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader(); w.writerows(rows)

    # ── 함대 요약 ──
    sU_r = sum(r["D30_ref_UW"] for r in rows)
    sD_r = sum(r["D30_ref_DW"] for r in rows)
    sU_c = sum(r["D30_scr_UW"] for r in rows)
    sD_c = sum(r["D30_scr_DW"] for r in rows)
    LUr, LDr = 30.0 / sU_r, 30.0 / sD_r
    LUc, LDc = 30.0 / sU_c, 30.0 / sD_c
    with open(os.path.join(OUTDIR, "fleet_summary.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.writer(f)
        w.writerow(["n_dlc", "sumD30_UW_ref", "sumD30_DW_ref",
                    "sumD30_UW_scr", "sumD30_DW_scr",
                    "life_UW_ref", "life_Sys_ref", "life_UW_scr", "life_Sys_scr",
                    "bias_UW_pct", "bias_Sys_life_pct"])
        w.writerow([len(rows), sU_r, sD_r, sU_c, sD_c,
                    LUr, lsys(LUr, LDr), LUc, lsys(LUc, LDc),
                    (sU_c / sU_r - 1) * 100,
                    (lsys(LUr, LDr) / lsys(LUc, LDc) - 1) * 100])

    # ── DLC 개수별 수명 수렴 (참값 D30 내림차순 누적) ──
    order = sorted(rows, key=lambda r: -r["D30_ref_UW"])
    Lsys_scr_final = lsys(LUc, LDc)          # 최종(111) 스크리닝 Sys
    cU_r = cD_r = cU_c = cD_c = 0.0
    conv = []
    for n, r in enumerate(order, 1):
        cU_r += r["D30_ref_UW"]; cD_r += r["D30_ref_DW"]
        cU_c += r["D30_scr_UW"]; cD_c += r["D30_scr_DW"]
        lur, ldr = 30.0 / cU_r, 30.0 / cD_r
        luc, ldc = 30.0 / cU_c, 30.0 / cD_c
        conv.append((n, lur, lsys(lur, ldr), luc, lsys(luc, ldc),
                     (lsys(lur, ldr) / lsys(luc, ldc) - 1) * 100,
                     (lsys(luc, ldc) / Lsys_scr_final - 1) * 100))
    with open(os.path.join(OUTDIR, "convergence.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.writer(f)
        w.writerow(["n", "life_UW_ref", "life_Sys_ref", "life_UW_scr",
                    "life_Sys_scr", "bias_Sys_life_pct", "vs_final_scr_Sys_pct"])
        w.writerows(conv)

    print(f"\n[완료] {len(rows)} DLC · {time.time()-t0:.0f}s")
    print(f"  Σ D30_UW 참값 {sU_r:.3f} vs 스크리닝 {sU_c:.3f} → "
          f"편향 {(sU_c/sU_r-1)*100:+.2f}%")
    print(f"  함대 수명 UW 참값 {LUr:.2f} vs 스크리닝 {LUc:.2f} yr")
    print(f"  함대 수명 Sys 참값 {lsys(LUr,LDr):.2f} vs 스크리닝 {lsys(LUc,LDc):.2f} yr "
          f"(편향 {(lsys(LUr,LDr)/lsys(LUc,LDc)-1)*100:+.2f}%)")
    print(f"  → 부록3_screening_life/ (per_dlc·fleet_summary·convergence)")


if __name__ == "__main__":
    main()
