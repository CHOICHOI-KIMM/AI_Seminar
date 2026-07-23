"""
DLC별 해석 · 상위 25 DLC 본해석 파이프라인 (무인, 배치 N=20)
=============================================================
DLC당: ① 참값(dt=0.1 점별, 체크포인트 재개) → ② 최적조합 배치 → ③ 판정·보정(≤3회,
실패 시 dt 1단계 축소 1회) → §5-3 실시간 기록 → 다음 DLC.
전체 완료 후: 상위 25 부분합(참값 기반) → fleet_masta_partial.csv → 총괄 엑셀 반영.
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
N_TOP = 25
NBATCH = 20
DT0 = 0.1
E_W = 9.0 / 8.0
MEM_LIMIT = 90.0
DTS = [20, 10, 4, 2, 1, 0.6]
DOC = os.path.join(ROOT, "DLC기반_피로해석_DLC별해석.md")
MS3, ME3 = "<!-- DLC_TOP_RESULTS_START -->", "<!-- DLC_TOP_RESULTS_END -->"
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")
RESULTS = {}          # {DLC: dict(...)}  §5-3 표 원천


# ── 데이터 로드 ──
def load_master():
    return {r["DLC"]: r for r in csv.DictReader(
        open(os.path.join(HERE, "dlc_master_summary.csv"), encoding="utf-8-sig"))}


def load_meta():
    return {r["DLC"]: r for r in csv.DictReader(
        open(os.path.join(HERE, "dlc_meta.csv"), encoding="utf-8-sig"))}


def load_raw(name):
    return [{k: float(v) for k, v in r.items()} for r in csv.DictReader(
        open(os.path.join(HERE, name, "raw.csv"), encoding="utf-8-sig"))]


def load_map(name):
    E = {}
    for m in csv.DictReader(open(os.path.join(HERE, name, "screen_eps_map.csv"),
                                 encoding="utf-8-sig")):
        E[(float(m["k"]), float(m["dt_s"]))] = (
            float(m["eps_UW_pct"]), float(m["eps_DW_pct"]), float(m["eps_Sys_pct"]))
    return E


def slope_of(E, dt, k, which):
    """스크리닝 맵 국소 기울기 [%p/k] (which: 0=UW, 2=Sys)."""
    ks = sorted({kk for (kk, dd) in E if dd == dt})
    lo = max([kk for kk in ks if kk <= k], default=ks[0])
    hi = min([kk for kk in ks if kk > lo], default=ks[-1])
    if hi == lo:
        lo = ks[-2]; hi = ks[-1]
    return (E[(hi, dt)][which] - E[(lo, dt)][which]) / (hi - lo)


def reselect(E, dt):
    """해당 dt에서 연속 k 중앙 타겟팅 (§4-1)."""
    ks = np.array(sorted({kk for (kk, dd) in E if dd == dt}))
    if len(ks) < 2:
        return None
    eU = np.array([E[(kk, dt)][0] for kk in ks])
    eS = np.array([E[(kk, dt)][2] for kk in ks])
    kf = np.linspace(ks.min(), ks.max(), 3001)
    u, sv = np.interp(kf, ks, eU), np.interp(kf, ks, eS)
    ok = (u >= 0) & (u <= 3) & (sv >= 0) & (sv <= 3)
    if not ok.any():
        return None
    idx = np.where(ok)[0]
    j = idx[np.argmin(np.abs(sv[idx] - 1.5))]
    return round(float(kf[j]), 2)


def bin_reps(data, dt, k):
    kp = int(round(dt / DT0))
    n = len(data)
    nb = n // kp
    edges = [(b * kp, (b + 1) * kp) for b in range(nb)]
    if edges[-1][1] < n:
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


# ── §5-3 실시간 표 ──
def lsys(lu, ld):
    return (lu ** -E_W + ld ** -E_W) ** (-1.0 / E_W)


def update_doc(note=""):
    L = [MS3, "",
         f"> 상위 {N_TOP} DLC(참값 손상순) · DLC당 참값→조합→판정·보정 순차 수행 · "
         f"판정 = UW∧Sys 0~+3% (참값 = 각 DLC dt=0.1 MASTA) · {note}", "",
         "| # | DLC | (dt, k) 최종 | 보정 | ε_UW | ε_Sys | ε_DW | "
         "UW/DW/Sys 수명 [yr] (참값) | D30_UW(참값) | 참값/조합 시간 | 판정 |",
         "|--:|-----|------------:|----:|-----:|-----:|-----:|---------------------------:|"
         "------------:|--------------:|:----:|"]
    for i, (name, R) in enumerate(RESULTS.items(), 1):
        if R.get("stage") != "done":
            L.append(f"| {i} | {name} | {R.get('combo','–')} | – | – | – | – | – | "
                     f"{R.get('d30','–')} | {R.get('note','진행중')} | ⏳ |")
            continue
        L.append(f"| {i} | {name} | ({R['dt']:g}, {R['k']:g}) | {R['ncorr']} | "
                 f"**{R['eU']:+.2f}%** | **{R['eS']:+.2f}%** | {R['eD']:+.2f}% | "
                 f"{R['lifeU']:,.1f} / {R['lifeD']:,.1f} / {R['lifeS']:,.1f} | "
                 f"{R['d30']:.3f} | {R['t_ref']:.0f}분 / {R['t_cmb']:.0f}초 | {R['mark']} |")
    if all(R.get("stage") == "done" for R in RESULTS.values()) and RESULTS:
        sU = sum(R["D30U"] for R in RESULTS.values())
        sD = sum(R["D30D"] for R in RESULTS.values())
        LU, LD = 30.0 / sU, 30.0 / sD
        LS = lsys(LU, LD)
        cU = sum(R["D30U_cmb"] for R in RESULTS.values())
        L += ["",
              f"**상위 {len(RESULTS)} 부분합 (참값 기반)**: ΣD30_UW = **{sU:.3f}** · "
              f"ΣD30_DW = {sD:.3f} → 부분 수명 UW **{LU:.2f}** / DW {LD:.2f} / "
              f"Sys **{LS:.2f}** yr",
              f"- 조합(빈) 합산 검증: ΣD30_UW(조합) = {cU:.3f} → 참값 대비 "
              f"**{(cU/sU-1)*100:+.2f}%** (방법론 전체 편향)"]
    L += ["", ME3]
    txt = open(DOC, encoding="utf-8").read()
    if MS3 not in txt:
        txt = txt.rstrip() + "\n\n### 5-3. 상위 25 DLC 본해석 결과 (실시간)\n\n" \
              + MS3 + "\n" + ME3 + "\n"
    txt = txt.split(MS3)[0] + "\n".join(L) + txt.split(ME3, 1)[1]
    open(DOC, "w", encoding="utf-8").write(txt)


def main():
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

    master = load_master()
    meta = load_meta()
    top = sorted([r for r in master.values() if r["valid"] == "1"],
                 key=lambda r: -float(r["D30_UW_scr"]))[:N_TOP]
    order = [r["DLC"] for r in top]
    for name in order:
        RESULTS[name] = {"stage": "wait", "note": "대기"}
    print(f"[대상] 상위 {N_TOP}:", order[:5], "…")

    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    dp = asm.design_properties
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
    bearings = {("UW" if "UW" in str(b) else "DW"): b
                for b in asm.all_parts_of_type_bearing()}
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    dstate = lc0.design_state_load_case_group
    print("[모델] 로드 완료")

    def analyze(cases, tagp):
        """[(cid, rev, rec)] → ({'UW': [...], 'DW': [...]}, t_anal)"""
        dmg = {"UW": [], "DW": []}
        t_anal = 0.0
        for b0 in range(0, len(cases), NBATCH):
            chunk = cases[b0:b0 + NBATCH]
            lcs = []
            for cid, rev, rec in chunk:
                lc = lc0.duplicate(dstate, f"{tagp}_{cid}")
                mf.set_loads(lc, pl, ipl, rec)
                lcs.append(lc)
            duty = dp.add_duty_cycle(f"dc_{tagp}_{b0}")
            for lc in lcs:
                duty.add_static_load(lc)
            t0 = time.perf_counter()
            csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            csd.perform_analysis()
            t_anal += time.perf_counter() - t0
            for key, b in bearings.items():
                subs = list(list(csd.results_for(b))[0].component_analysis_cases)
                for (cid, rev, rec), sub in zip(chunk, subs):
                    l10mr = (sub.component_detailed_analysis
                             .isots162812008.modified_reference_rating_life_cycles)
                    dmg[key].append(rev / l10mr if (l10mr and l10mr > 0) else 0.0)
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
                raise MemoryError("메모리 90% 초과")
        return dmg, t_anal

    # 웜업 + 자체검증
    d0 = load_raw(order[0])
    mf.set_loads(lc0, pl, ipl, d0[0])
    sd = lc0.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
    sd.perform_analysis()
    l10s = (sd.results_for(bearings["UW"]).component_detailed_analysis
            .isots162812008.modified_reference_rating_life_cycles)
    dmgb, _ = analyze([(0, 1.0, d0[0])], "chk")
    rel = abs((1.0 / dmgb["UW"][0]) / l10s - 1)
    print(f"[자체검증] 단일 vs 배치 상대오차 {rel:.2e} {'✅' if rel <= 1e-6 else '❌ 중단'}")
    if rel > 1e-6:
        return
    update_doc("자체검증 통과, 시작")

    for name in order:
        R = RESULTS[name]
        R["stage"] = "run"
        sf = float(meta[name]["ScaleFactor"])
        data = load_raw(name)
        E = load_map(name)
        dt = float(master[name]["best_dt_s"])
        k = float(master[name]["best_k"])
        R["combo"] = f"({dt:g}, {k:g})"
        print(f"\n===== {name} (dt={dt:g}, k={k:g}, {len(data)}점) =====")

        # ── ① 참값 (체크포인트 재개) ──
        refp = os.path.join(HERE, name, "masta_ref_dmg.csv")
        done = 0
        if os.path.exists(refp):
            done = sum(1 for _ in open(refp, encoding="utf-8-sig")) - 1
            done = max(done, 0)
        R["note"] = "참값 해석중"
        update_doc(f"{name} 참값 {done}/{len(data)}점부터")
        t0 = time.perf_counter()
        f = open(refp, "a", newline="", encoding="utf-8-sig")
        w = csv.writer(f)
        if done == 0:
            w.writerow(["idx", "dmg_UW", "dmg_DW"])
        CH = 400                                    # 체크포인트 단위
        for s0 in range(done, len(data), CH):
            seg = [(i, abs(data[i]["rpm"]) / 60.0 * DT0, data[i])
                   for i in range(s0, min(s0 + CH, len(data)))]
            dmg, _ = analyze(seg, f"rf{order.index(name)}")
            for j, (i, rev, rec) in enumerate(seg):
                w.writerow([i, dmg["UW"][j], dmg["DW"][j]])
            f.flush()
        f.close()
        t_ref = (time.perf_counter() - t0) / 60.0
        rows = list(csv.DictReader(open(refp, encoding="utf-8-sig")))
        DrefU = sum(float(r["dmg_UW"]) for r in rows)
        DrefD = sum(float(r["dmg_DW"]) for r in rows)
        lifeU = 30.0 / (DrefU * sf)
        lifeD = 30.0 / (DrefD * sf)
        lifeS = lsys(lifeU, lifeD)
        print(f"[참값] D30_UW={DrefU*sf:.3f} 수명 UW={lifeU:.1f} ({t_ref:.0f}분)")

        # ── ② 조합 + ③ 판정·보정 ──
        R["note"] = "조합 해석중"
        update_doc(f"{name} 조합 (dt={dt:g}, k={k:g})")
        ncorr = 0
        t_cmb_tot = 0.0
        dt_cur, k_cur = dt, k
        while True:
            reps = bin_reps(data, dt_cur, k_cur)
            dmgc, t_c = analyze(reps, f"bs{order.index(name)}_{ncorr}")
            t_cmb_tot += t_c
            DU, DD = sum(dmgc["UW"]), sum(dmgc["DW"])
            eU = (DU / DrefU - 1) * 100
            eD = (DD / DrefD - 1) * 100
            lU, lD = 30.0 / (DU * sf), 30.0 / (DD * sf)
            eS = (lifeS / lsys(lU, lD) - 1) * 100
            ok = (0 <= eU <= 3) and (0 <= eS <= 3)
            print(f"  [시도{ncorr}] dt={dt_cur:g} k={k_cur:g} εUW={eU:+.2f} "
                  f"εSys={eS:+.2f} εDW={eD:+.2f} {'✅' if ok else '❌'}")
            if ok or ncorr >= 4:
                break
            if ncorr < 3:                          # k 보정 (≤3회)
                which = 2 if not (0 <= eS <= 3) else 0
                sl = slope_of(E, dt_cur, k_cur, which)
                tgt = eS if which == 2 else eU
                if abs(sl) < 1e-6:
                    ncorr = 3
                    continue
                k_cur = round(min(max(k_cur + (1.5 - tgt) / sl, 0.0), 3.0), 2)
                ncorr += 1
            else:                                   # dt 1단계 축소
                i_dt = DTS.index(dt_cur) if dt_cur in DTS else 0
                if i_dt + 1 < len(DTS):
                    dt_cur = DTS[i_dt + 1]
                    kk = reselect(E, dt_cur)
                    k_cur = kk if kk is not None else k_cur
                ncorr += 1
        mark = "**합격 ✅**" if ok else "**불합격 ❌**"
        R.update(stage="done", dt=dt_cur, k=k_cur, ncorr=ncorr,
                 eU=eU, eS=eS, eD=eD, lifeU=lifeU, lifeD=lifeD, lifeS=lifeS,
                 d30=DrefU * sf, D30U=DrefU * sf, D30D=DrefD * sf,
                 D30U_cmb=DU * sf, t_ref=t_ref, t_cmb=t_cmb_tot, mark=mark)
        with open(os.path.join(HERE, name, "masta_best_summary.csv"), "w",
                  newline="", encoding="utf-8-sig") as fo:
            wo = csv.writer(fo)
            wo.writerow(["dt", "k", "ncorr", "eps_UW", "eps_DW", "eps_Sys",
                         "life_UW_ref", "life_DW_ref", "life_Sys_ref",
                         "D30_UW_ref", "D30_DW_ref", "D30_UW_cmb",
                         "t_ref_min", "t_cmb_s", "pass"])
            wo.writerow([dt_cur, k_cur, ncorr, eU, eD, eS, lifeU, lifeD, lifeS,
                         DrefU * sf, DrefD * sf, DU * sf, t_ref, t_cmb_tot, int(ok)])
        update_doc(f"{name} 완료")

    # ── ④ 부분 합산 (참값 기반) ──
    sU = sum(R["D30U"] for R in RESULTS.values())
    sD = sum(R["D30D"] for R in RESULTS.values())
    LU, LD = 30.0 / sU, 30.0 / sD
    with open(os.path.join(HERE, "fleet_masta_partial.csv"), "w", newline="",
              encoding="utf-8-sig") as fo:
        wo = csv.writer(fo)
        wo.writerow(["n_dlc", "sumD30_UW", "sumD30_DW",
                     "life_UW", "life_DW", "life_Sys"])
        wo.writerow([len(RESULTS), sU, sD, LU, LD, lsys(LU, LD)])
    update_doc("전체 완료")
    print(f"\n[부분합] ΣD30_UW={sU:.3f} → UW {LU:.2f} yr · Sys {lsys(LU, LD):.2f} yr")
    print("완료")


if __name__ == "__main__":
    main()
