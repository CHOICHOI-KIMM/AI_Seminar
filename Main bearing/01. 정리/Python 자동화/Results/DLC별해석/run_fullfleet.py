"""
DLC별 해석 · 전 111 DLC 본해석 파이프라인 §5-4 (무인, 배치 N=20)
=================================================================
- 기존 25개(masta_best_summary 보유)는 결과 재사용(§5-4 표 선기입), 잔여 86개 신규 해석
- 잔여 순서: 빈별 편향배율(상위 25 실측 참값/스크리닝) 보정 D30 내림차순
- DLC당: ① 참값(dt=0.1, 체크포인트) + masta_ref_detail.csv(부록9 양식)
         ② 조합 + masta_cmb_detail.csv ③ 판정·보정(§4-1: k≤3회→dt축소)
- 매 DLC 완료: §5-4 실시간 갱신 + fleet_masta_partial.csv 갱신
- 신규 5개 완료마다 + 종료 시: 총괄 엑셀 재생성(잠금 시 스킵)
"""
import csv
import math
import os
import subprocess
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
NBATCH = 20
DT0 = 0.1
E_W = 9.0 / 8.0
MEM_LIMIT = 90.0
DTS = [20, 10, 4, 2, 1, 0.6]
DOC = os.path.join(ROOT, "DLC기반_피로해석_DLC별해석.md")
MS4, ME4 = "<!-- DLC_FLEET_RESULTS_START -->", "<!-- DLC_FLEET_RESULTS_END -->"
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")
XLSX_SCRIPT = os.path.join(HERE, "make_dlc_master_xlsx.py")
RESULTS = {}


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


def load_best(name):
    p = os.path.join(HERE, name, "masta_best_summary.csv")
    if not os.path.isfile(p):
        return None
    r = list(csv.DictReader(open(p, encoding="utf-8-sig")))[0]
    return {k: float(v) for k, v in r.items()}


def bin_of(name):
    """DLC1.2-d-s1 → '1.2-d' (패밀리-풍속빈)."""
    core = name[3:] if name.startswith("DLC") else name
    return core.rsplit("-s", 1)[0]


def slope_of(E, dt, k, which):
    ks = sorted({kk for (kk, dd) in E if dd == dt})
    lo = max([kk for kk in ks if kk <= k], default=ks[0])
    hi = min([kk for kk in ks if kk > lo], default=ks[-1])
    if hi == lo:
        lo = ks[-2]; hi = ks[-1]
    return (E[(hi, dt)][which] - E[(lo, dt)][which]) / (hi - lo)


def reselect(E, dt):
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
        out.append((bi, i0 * DT0, abs(rec["rpm"]) / 60.0 * (m * DT0), rec))
    return out


def lsys(lu, ld):
    return (lu ** -E_W + ld ** -E_W) ** (-1.0 / E_W)


def write_partial():
    done = [R for R in RESULTS.values() if R.get("stage") == "done"]
    if not done:
        return
    sU = sum(R["D30U"] for R in done)
    sD = sum(R["D30D"] for R in done)
    LU, LD = 30.0 / sU, 30.0 / sD
    with open(os.path.join(HERE, "fleet_masta_partial.csv"), "w", newline="",
              encoding="utf-8-sig") as fo:
        wo = csv.writer(fo)
        wo.writerow(["n_dlc", "sumD30_UW", "sumD30_DW",
                     "life_UW", "life_DW", "life_Sys"])
        wo.writerow([len(done), sU, sD, LU, LD, lsys(LU, LD)])


def regen_xlsx():
    try:
        r = subprocess.run([sys.executable, "-X", "utf8", XLSX_SCRIPT],
                           capture_output=True, timeout=120)
        print(f"  [엑셀] 재생성 {'완료' if r.returncode == 0 else '스킵(잠금/오류)'}")
    except Exception as e:
        print(f"  [엑셀] 스킵: {str(e)[:50]}")


def update_doc(note=""):
    ndone = sum(1 for R in RESULTS.values() if R.get("stage") == "done")
    L = [MS4, "",
         f"> 전 111 DLC 본해석 · 순서 = 기존 25(완료분 선기입) → 잔여 86(빈별 편향배율 "
         f"보정 D30 내림차순) · 판정 = UW∧Sys 0~+3% (참값 = dt=0.1 MASTA) · "
         f"진행 {ndone}/111 · {note}", "",
         "| # | DLC | (dt, k) 최종 | 보정 | ε_UW | ε_Sys | ε_DW | "
         "UW/DW/Sys 수명 [yr] (참값) | D30_UW(참값) | 참값/조합 시간 | 판정 |",
         "|--:|-----|------------:|----:|-----:|-----:|-----:|---------------------------:|"
         "------------:|--------------:|:----:|"]
    for i, (name, R) in enumerate(RESULTS.items(), 1):
        if R.get("stage") != "done":
            L.append(f"| {i} | {name} | {R.get('combo','–')} | – | – | – | – | – | "
                     f"{R.get('d30','–')} | {R.get('note','대기')} | ⏳ |")
            continue
        L.append(f"| {i} | {name} | ({R['dt']:g}, {R['k']:g}) | {R['ncorr']:g} | "
                 f"**{R['eU']:+.2f}%** | **{R['eS']:+.2f}%** | {R['eD']:+.2f}% | "
                 f"{R['lifeU']:,.1f} / {R['lifeD']:,.1f} / {R['lifeS']:,.1f} | "
                 f"{R['D30U']:.3f} | {R['t_ref']:.0f}분 / {R['t_cmb']:.0f}초 | {R['mark']} |")
    done = [R for R in RESULTS.values() if R.get("stage") == "done"]
    if done:
        sU = sum(R["D30U"] for R in done)
        sD = sum(R["D30D"] for R in done)
        LU, LD = 30.0 / sU, 30.0 / sD
        cU = sum(R["D30U_cmb"] for R in done)
        tag = "**전량 합산 (전 111 DLC, 참값 기반)**" if len(done) == len(RESULTS) \
              else f"**부분합 ({len(done)} DLC, 참값 기반)**"
        L += ["",
              f"{tag}: ΣD30_UW = **{sU:.3f}** · ΣD30_DW = {sD:.3f} → "
              f"수명 UW **{LU:.2f}** / DW {LD:.2f} / Sys **{lsys(LU, LD):.2f}** yr",
              f"- 조합(빈) 합산 검증: ΣD30_UW(조합) = {cU:.3f} → 참값 대비 "
              f"**{(cU/sU-1)*100:+.2f}%** (방법론 전체 편향)"]
    L += ["", ME4]
    txt = open(DOC, encoding="utf-8").read()
    if MS4 not in txt:
        sec = ("\n### 5-4. 전 111 DLC 본해석 결과 (실시간)\n\n"
               + MS4 + "\n" + ME4 + "\n\n")
        anchor = "\n---\n\n# 부록 1."
        if anchor in txt:
            txt = txt.replace(anchor, "\n" + sec + anchor, 1)
        else:
            txt = txt.rstrip() + "\n" + sec
    txt = txt.split(MS4)[0] + "\n".join(L) + txt.split(ME4, 1)[1]
    open(DOC, "w", encoding="utf-8").write(txt)


def main():
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

    master = load_master()
    meta = load_meta()

    # ── 순서 구성: 완료 25 → 잔여 86 (빈별 배율 보정 순위) ──
    prefilled, ratios = [], {}
    for name in sorted(master):
        b = load_best(name)
        if b is None:
            continue
        scr = float(master[name].get("D30_UW_scr") or 0)
        if scr > 0:
            ratios.setdefault(bin_of(name), []).append(b["D30_UW_ref"] / scr)
        prefilled.append((name, b))
    prefilled.sort(key=lambda nb: -nb[1]["D30_UW_ref"])
    bin_r = {bb: float(np.mean(v)) for bb, v in ratios.items()}
    r_default = float(np.mean([r for v in ratios.values() for r in v]))
    remaining = []
    for name, m in master.items():
        if load_best(name) is not None or m.get("valid") != "1":
            continue
        scr = float(m.get("D30_UW_scr") or 0)
        r = bin_r.get(bin_of(name), r_default)
        remaining.append((name, scr * r))
    remaining.sort(key=lambda x: -x[1])
    print(f"[순서] 완료 {len(prefilled)} + 잔여 {len(remaining)} "
          f"(빈 배율 {len(bin_r)}종, 기본 {r_default:.3f})")

    for name, b in prefilled:
        RESULTS[name] = dict(
            stage="done", dt=b["dt"], k=b["k"], ncorr=b["ncorr"],
            eU=b["eps_UW"], eS=b["eps_Sys"], eD=b["eps_DW"],
            lifeU=b["life_UW_ref"], lifeD=b["life_DW_ref"], lifeS=b["life_Sys_ref"],
            D30U=b["D30_UW_ref"], D30D=b["D30_DW_ref"], D30U_cmb=b["D30_UW_cmb"],
            t_ref=b["t_ref_min"], t_cmb=b["t_cmb_s"],
            mark="**합격 ✅**" if b["pass"] else "**불합격 ❌**")
    for name, d30c in remaining:
        RESULTS[name] = {"stage": "wait", "note": "대기", "d30_corr": d30c}
    update_doc("파이프라인 초기화")

    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    dp = asm.design_properties
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
    bearings = {("UW" if "UW" in str(b) else "DW"): b
                for b in asm.all_parts_of_type_bearing()}
    bkeys = list(bearings)
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    dstate = lc0.design_state_load_case_group
    print("[모델] 로드 완료")

    def detail_row(rec, bi, t_s, rev, bname_, d):
        loads = {"force_x_N": -rec["Fz"] * 1e3, "force_y_N": rec["Fy"] * 1e3,
                 "axial_load_N": rec["Fx"] * 1e3, "moment_x_Nm": -rec["Mz"] * 1e3,
                 "moment_y_Nm": rec["My"] * 1e3, "Moment_z_Nm": rec["Mx"] * 1e3}
        lv = [loads[c] for c in mf.LOAD_COLS]
        sin = mf.fnum(mf.g(d, "maximum_normal_stress_inner"))
        sout = mf.fnum(mf.g(d, "maximum_normal_stress_outer"))
        dm = [mf.damage(rev, mf.g(d, pth)) for _, pth, _, _ in mf.DAMAGE_DEFS]
        return ([bi, t_s, rec["rpm"], rev] + lv + [bname_,
                mf.num(sin / 1e6 if sin is not None else None),
                mf.num(sout / 1e6 if sout is not None else None),
                mf.num(mf.g(d, "iso762006.safety_factor")),
                mf.num(mf.g(d, "iso2812007.basic_rating_life_cycles")),
                mf.num(mf.g(d, "iso2812007.modified_rating_life_cycles")),
                mf.num(mf.g(d, "isots162812008.basic_reference_rating_life_cycles")),
                mf.num(mf.g(d, "isots162812008.modified_reference_rating_life_cycles"))]
                + [mf.num(x) for x in dm] + ["", ""])

    def analyze(cases, tagp, det_writer=None):
        """cases=[(cid, t_s, rev, rec)] → ({'UW': [...], 'DW': [...]}, t_anal)"""
        dmg = {"UW": [], "DW": []}
        t_anal = 0.0
        for b0 in range(0, len(cases), NBATCH):
            chunk = cases[b0:b0 + NBATCH]
            lcs = []
            for cid, t_s, rev, rec in chunk:
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
            subs = {key: list(list(csd.results_for(b))[0].component_analysis_cases)
                    for key, b in bearings.items()}
            for j, (cid, t_s, rev, rec) in enumerate(chunk):
                for key in bkeys:
                    d = mf.g(subs[key][j], "component_detailed_analysis")
                    l10mr = mf.fnum(mf.g(
                        d, "isots162812008.modified_reference_rating_life_cycles"))
                    dmg[key].append(rev / l10mr if (l10mr and l10mr > 0) else 0.0)
                    if det_writer is not None:
                        det_writer.writerow(detail_row(
                            rec, cid, t_s, rev, mf.bname(bearings[key]), d))
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
    first = remaining[0][0] if remaining else prefilled[0][0]
    d0 = load_raw(first)
    mf.set_loads(lc0, pl, ipl, d0[0])
    sd = lc0.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
    sd.perform_analysis()
    l10s = (sd.results_for(bearings["UW"]).component_detailed_analysis
            .isots162812008.modified_reference_rating_life_cycles)
    dmgb, _ = analyze([(0, 0.0, 1.0, d0[0])], "chk")
    rel = abs((1.0 / dmgb["UW"][0]) / l10s - 1)
    print(f"[자체검증] 단일 vs 배치 상대오차 {rel:.2e} {'✅' if rel <= 1e-6 else '❌ 중단'}")
    if rel > 1e-6:
        return
    update_doc("자체검증 통과, 시작")

    n_new_done = 0
    for oi, (name, _) in enumerate(remaining):
        R = RESULTS[name]
        R["stage"] = "run"
        sf = float(meta[name]["ScaleFactor"])
        data = load_raw(name)
        E = load_map(name)
        dt = float(master[name]["best_dt_s"])
        k = float(master[name]["best_k"])
        R["combo"] = f"({dt:g}, {k:g})"
        print(f"\n===== {name} (dt={dt:g}, k={k:g}, {len(data)}점) =====")

        # ── ① 참값 (체크포인트 + 상세) ──
        refp = os.path.join(HERE, name, "masta_ref_dmg.csv")
        detp = os.path.join(HERE, name, "masta_ref_detail.csv")
        done = 0
        if os.path.exists(refp):
            done = max(sum(1 for _ in open(refp, encoding="utf-8-sig")) - 1, 0)
        R["note"] = "참값 해석중"
        update_doc(f"{name} 참값 {done}/{len(data)}점부터")
        t0 = time.perf_counter()
        f = open(refp, "a", newline="", encoding="utf-8-sig")
        w = csv.writer(f)
        fd = open(detp, "a", newline="", encoding="utf-8-sig")
        wd = csv.writer(fd)
        if done == 0:
            w.writerow(["idx", "dmg_UW", "dmg_DW"])
            wd.writerow(mf.DATA_HEADER)
        CH = 400
        for s0 in range(done, len(data), CH):
            seg = [(i, i * DT0, abs(data[i]["rpm"]) / 60.0 * DT0, data[i])
                   for i in range(s0, min(s0 + CH, len(data)))]
            dmg, _ = analyze(seg, f"rf{oi}", det_writer=wd)
            for j, (i, t_s, rev, rec) in enumerate(seg):
                w.writerow([i, dmg["UW"][j], dmg["DW"][j]])
            f.flush()
            fd.flush()
        f.close()
        fd.close()
        t_ref = (time.perf_counter() - t0) / 60.0
        rows = list(csv.DictReader(open(refp, encoding="utf-8-sig")))
        DrefU = sum(float(r["dmg_UW"]) for r in rows)
        DrefD = sum(float(r["dmg_DW"]) for r in rows)
        if DrefU <= 0 or DrefD <= 0:
            print(f"[참값] 손상 0 — 판정 불가, 스킵 기록")
            R.update(stage="done", dt=dt, k=k, ncorr=0, eU=float("nan"),
                     eS=float("nan"), eD=float("nan"), lifeU=float("inf"),
                     lifeD=float("inf"), lifeS=float("inf"), D30U=0.0, D30D=0.0,
                     D30U_cmb=0.0, t_ref=t_ref, t_cmb=0.0, mark="손상≈0")
            update_doc(f"{name} 손상≈0 스킵")
            continue
        lifeU = 30.0 / (DrefU * sf)
        lifeD = 30.0 / (DrefD * sf)
        lifeS = lsys(lifeU, lifeD)
        print(f"[참값] D30_UW={DrefU*sf:.4f} 수명 UW={lifeU:.1f} ({t_ref:.0f}분)")

        # ── ② 조합 + ③ 판정·보정 ──
        R["note"] = "조합 해석중"
        update_doc(f"{name} 조합 (dt={dt:g}, k={k:g})")
        ncorr = 0
        t_cmb_tot = 0.0
        dt_cur, k_cur = dt, k
        while True:
            reps = bin_reps(data, dt_cur, k_cur)
            cdetp = os.path.join(HERE, name, "masta_cmb_detail.csv")
            fc = open(cdetp, "w", newline="", encoding="utf-8-sig")
            wc = csv.writer(fc)
            wc.writerow(mf.DATA_HEADER)
            dmgc, t_c = analyze(reps, f"bs{oi}_{ncorr}", det_writer=wc)
            fc.close()
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
            if ncorr < 3:
                which = 2 if not (0 <= eS <= 3) else 0
                sl = slope_of(E, dt_cur, k_cur, which)
                tgt = eS if which == 2 else eU
                if abs(sl) < 1e-6:
                    ncorr = 3
                    continue
                k_cur = round(min(max(k_cur + (1.5 - tgt) / sl, 0.0), 3.0), 2)
                ncorr += 1
            else:
                i_dt = DTS.index(dt_cur) if dt_cur in DTS else 0
                if i_dt + 1 < len(DTS):
                    dt_cur = DTS[i_dt + 1]
                    kk = reselect(E, dt_cur)
                    k_cur = kk if kk is not None else k_cur
                ncorr += 1
        mark = "**합격 ✅**" if ok else "**불합격 ❌**"
        R.update(stage="done", dt=dt_cur, k=k_cur, ncorr=ncorr,
                 eU=eU, eS=eS, eD=eD, lifeU=lifeU, lifeD=lifeD, lifeS=lifeS,
                 D30U=DrefU * sf, D30D=DrefD * sf, D30U_cmb=DU * sf,
                 t_ref=t_ref, t_cmb=t_cmb_tot, mark=mark)
        with open(os.path.join(HERE, name, "masta_best_summary.csv"), "w",
                  newline="", encoding="utf-8-sig") as fo:
            wo = csv.writer(fo)
            wo.writerow(["dt", "k", "ncorr", "eps_UW", "eps_DW", "eps_Sys",
                         "life_UW_ref", "life_DW_ref", "life_Sys_ref",
                         "D30_UW_ref", "D30_DW_ref", "D30_UW_cmb",
                         "t_ref_min", "t_cmb_s", "pass"])
            wo.writerow([dt_cur, k_cur, ncorr, eU, eD, eS, lifeU, lifeD, lifeS,
                         DrefU * sf, DrefD * sf, DU * sf, t_ref, t_cmb_tot, int(ok)])
        write_partial()
        n_new_done += 1
        update_doc(f"{name} 완료 (신규 {n_new_done}/{len(remaining)})")
        if n_new_done % 5 == 0:
            regen_xlsx()

    write_partial()
    update_doc("전량 완료")
    regen_xlsx()
    done = [R for R in RESULTS.values() if R.get("stage") == "done"]
    sU = sum(R["D30U"] for R in done)
    sD = sum(R["D30D"] for R in done)
    LU, LD = 30.0 / sU, 30.0 / sD
    print(f"\n[전량합산] {len(done)} DLC ΣD30_UW={sU:.3f} → "
          f"UW {LU:.2f} yr · Sys {lsys(LU, LD):.2f} yr")
    print("완료")


if __name__ == "__main__":
    main()
