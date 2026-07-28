"""
사이징 최적화 부록 1 — 롤러 프로파일 모델 영향 검토 (DIN Lundberg vs Johns Gohar)
=================================================================================
동일 파일(빔 샤프트 3안) + 동일 v1.3 베어링 제원에서 롤러 프로파일만 바꿔
  1-A/1-B : 피로 111 DLC (dt=20, 부록 4 표준, k 는 부록 4 스크리닝 결과 재사용)
  1-C/1-D : 극한 16 LC (엑셀 Mx 를 축토크로 인가, 속도 0)
를 수행하고 부록 4 값과 대조한다.

산출: Results/사이징최적화/부록1_롤러프로파일/
  per_dlc_compare.csv   DLC별 수명 비교 (111행)
  fleet_summary.csv     함대 요약 + 부록 4 대조 + 극한 응력
"""
import csv
import math
import os
import sys
import time

import psutil

HERE = os.path.dirname(os.path.abspath(__file__))          # .../Results/사이징최적화
RES = os.path.dirname(HERE)                                # .../Results
ROOT = os.path.dirname(RES)                                # .../Python 자동화
DLCDIR = os.path.join(RES, "DLC별해석")
OUTDIR = os.path.join(HERE, "부록1_롤러프로파일")
sys.path.insert(0, ROOT)

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")

# v1.3 제원 (설정 순서 중요: 링 폭을 먼저 낮춘 뒤 T)
V13_SEQ = [("element_diameter", 0.11051), ("roller_length", 0.238048),
           ("outer_diameter", 3.6), ("inner_ring_width", 0.3),
           ("outer_ring_width", 0.253), ("width", 0.31),
           ("number_of_elements", 87)]
C_TARGET, CU_TARGET, MASS_TARGET = 22227978.9, 3929016.7, 5600.48

DT0, DT = 0.1, 20.0
E_W = 9.0 / 8.0
DESIGN_YEARS = 30.0
NBATCH = 20
MEM_LIMIT = 95.0
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")

# 부록 4 참조값 (v1.3 제원 + FE 샤프트 2안, dt=20 무보정)
APP4 = dict(sumD30_UW=6.918, life_UW=4.337, life_Sys=3.870)

# 극한 LC 축토크 [kNm] = 엑셀 Mx 열
EXT_TQ = {
    "Mx_max": 61192.0, "Mx_min": -22925.0, "My_max": 23670.0, "My_min": 27453.0,
    "Mz_max": 10308.0, "Mz_min": 17013.0, "Myz_max": 22673.0, "Myz_min": 58208.0,
    "Fx_max": 51143.0, "Fx_min": 90.3, "Fy_max": -40.3, "Fy_min": 33.3,
    "Fz_max": -2.0, "Fz_min": 61059.0, "Fyz_max": 55311.0, "Fyz_min": -2.0,
}
STRESS_LIMIT = 2100.0   # MPa


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception:
        return None


def sc(o, n):
    v = safe(o, n)
    if isinstance(v, (int, float)) and not isinstance(v, bool):
        return float(v)
    for a in ("value", "wrapped"):
        w = safe(v, a)
        if isinstance(w, (int, float)) and not isinstance(w, bool):
            return float(w)
    return None


def load_meta():
    return {r["DLC"]: r for r in csv.DictReader(
        open(os.path.join(DLCDIR, "dlc_meta.csv"), encoding="utf-8-sig"))}


def load_k_from_app4():
    """부록 4 스크리닝 결과의 (dt=20, 중앙타겟 k) 재사용 — C/Cu 가 동일하므로 유효"""
    p = os.path.join(DLCDIR, "부록4_screening_dt20", "per_dlc.csv")
    return {r["DLC"]: (float(r["k"]), r["ksel"]) for r in
            csv.DictReader(open(p, encoding="utf-8-sig"))}


def load_raw(name):
    return [{k: float(v) for k, v in r.items()} for r in csv.DictReader(
        open(os.path.join(DLCDIR, name, "raw.csv"), encoding="utf-8-sig"))]


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


def build_model(profile_type):
    """모델 로드 → v1.3 제원 이식 → 프로파일 설정 → 검증"""
    from mastapy.system_model import Design
    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    bearings = list(asm.all_parts_of_type_bearing())
    for b in bearings:
        det = safe(b, "detail")
        for key, val in V13_SEQ:
            try:
                setattr(det, key, val)
            except Exception as e:
                print(f"  !! {b} {key} 설정 실패: {str(e).splitlines()[0][:70]}")
        try:
            safe(det, "roller_profile_set").active_profile_type = profile_type
        except Exception as e:
            print(f"  !! {b} 프로파일 설정 실패: {str(e).splitlines()[0][:70]}")
    det = safe(bearings[0], "detail")
    C, Cu, mass = (sc(det, "basic_dynamic_load_rating"),
                   sc(det, "fatigue_load_limit"), sc(det, "mass"))
    pts = list(safe(det, "inner_race_and_roller_profiles"))
    drop = max(float(safe(q, "total_deviation")) for q in pts) * 1e6
    ptype = safe(safe(det, "roller_profile_set"), "active_profile_type")
    print(f"  [검증] {ptype}  단부드롭 {drop:.2f} um")
    print(f"         C={C:,.0f}(목표 {C_TARGET:,.0f})  Cu={Cu:,.0f}(목표 {CU_TARGET:,.0f})"
          f"  mass={mass:,.1f}(목표 {MASS_TARGET:,.1f})")
    ok = (abs(C / C_TARGET - 1) < 1e-4 and abs(Cu / CU_TARGET - 1) < 1e-4
          and abs(mass / MASS_TARGET - 1) < 1e-3)
    print(f"         제원 이식 {'OK' if ok else '!! 불일치'}")
    return design, asm, bearings, drop, ok


def run_fatigue(asm, bearings, kmap, meta, tag):
    import masta_fatigue as mf
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
    dp = asm.design_properties
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    bmap = {("UW" if "UW" in str(b) else "DW"): b for b in bearings}
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    ds = lc0.design_state_load_case_group

    targets = [n for n in sorted(meta) if n in kmap]
    rows = {}
    t0 = time.time()
    for i, name in enumerate(targets, 1):
        sf = float(meta[name]["ScaleFactor"])
        k, ktag = kmap[name]
        reps = bin_reps(load_raw(name), k)
        dmg = {"UW": [], "DW": []}
        t_an = 0.0
        short = name[-6:].replace(".", "")
        for b0 in range(0, len(reps), NBATCH):
            chunk = reps[b0:b0 + NBATCH]
            lcs = []
            for cid, rev, rec in chunk:
                lc = lc0.duplicate(ds, f"a1{tag}_{short}_{cid}")
                mf.set_loads(lc, pl, ipl, rec)
                lcs.append(lc)
            duty = dp.add_duty_cycle(f"a1{tag}dc_{short}_{b0}")
            for lc in lcs:
                duty.add_static_load(lc)
            t1 = time.perf_counter()
            csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            csd.perform_analysis()
            t_an += time.perf_counter() - t1
            for key, b in bmap.items():
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
        d30U, d30D = sum(dmg["UW"]) * sf, sum(dmg["DW"]) * sf
        rows[name] = dict(k=k, ksel=ktag, nbin=len(reps), sf=sf,
                          D30_UW=d30U, D30_DW=d30D, t_s=t_an)
        if i % 20 == 0 or i == len(targets):
            print(f"    [{tag}] {i}/{len(targets)}  ({time.time()-t0:.0f}s)", flush=True)
    return rows


def run_extreme(asm, bearings, tag):
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
    dp = asm.design_properties
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    bmap = {("UW" if "UW" in str(b) else "DW"): b for b in bearings}
    out = {}
    for nm, tq_knm in EXT_TQ.items():
        lc = next((c for c in dp.static_loads if c.name == nm), None)
        if lc is None:
            print(f"    !! 로드케이스 {nm} 없음")
            continue
        q = lc.inputs_for_power_load(ipl)
        for attr, val in (("speed", 0.0), ("torque", tq_knm * 1e3)):
            try:
                setattr(q, attr, val)
            except Exception as e:
                print(f"    !! {nm} {attr}: {str(e).splitlines()[0][:50]}")
        sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
        sd.perform_analysis()
        rec = {}
        for key, b in bmap.items():
            d = sd.results_for(b).component_detailed_analysis
            rec[key] = dict(mx=sc(d, "maximum_normal_stress"),
                            inner=sc(d, "maximum_normal_stress_inner"),
                            outer=sc(d, "maximum_normal_stress_outer"))
        out[nm] = rec
    return out


def main():
    import masta_clr_legacy  # noqa: F401  (HASP 혼합모드 우회)
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.bearings import RollerBearingProfileTypes
    os.makedirs(OUTDIR, exist_ok=True)
    meta, kmap = load_meta(), load_k_from_app4()
    print(f"[부록1] 대상 {len([n for n in meta if n in kmap])} DLC · dt={DT:g} · "
          f"k 는 부록 4 스크리닝 재사용")

    CASES = [("DIN", RollerBearingProfileTypes.DIN_LUNDBERG),
             ("GOH", RollerBearingProfileTypes.JOHNS_GOHAR)]
    fat, ext, drops = {}, {}, {}
    for tag, ptype in CASES:
        print(f"\n=== {tag} ===")
        t0 = time.time()
        design, asm, bearings, drop, ok = build_model(ptype)
        drops[tag] = drop
        ext[tag] = run_extreme(asm, bearings, tag)
        print(f"  극한 16 LC 완료 ({time.time()-t0:.0f}s)")
        fat[tag] = run_fatigue(asm, bearings, kmap, meta, tag)
        print(f"  피로 111 DLC 완료 (누적 {time.time()-t0:.0f}s)")

    # ── per_dlc_compare.csv ──
    names = sorted(fat["DIN"], key=lambda n: -fat["DIN"][n]["D30_UW"])
    sU = {t: sum(fat[t][n]["D30_UW"] for n in names) for t, _ in CASES}
    sD = {t: sum(fat[t][n]["D30_DW"] for n in names) for t, _ in CASES}
    with open(os.path.join(OUTDIR, "per_dlc_compare.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.writer(f)
        w.writerow(["rank", "DLC", "k", "ksel", "nbin",
                    "D30_UW_DIN", "D30_UW_GOH", "dD30_UW_pct",
                    "D30_DW_DIN", "D30_DW_GOH", "dD30_DW_pct",
                    "life_UW_DIN_yr", "life_UW_GOH_yr",
                    "life_Sys_DIN_yr", "life_Sys_GOH_yr", "dlife_Sys_pct",
                    "contrib_UW_DIN_pct", "t_s_DIN", "t_s_GOH"])
        for i, n in enumerate(names, 1):
            a, b = fat["DIN"][n], fat["GOH"][n]
            lu_a, ld_a = DESIGN_YEARS / a["D30_UW"], DESIGN_YEARS / a["D30_DW"]
            lu_b, ld_b = DESIGN_YEARS / b["D30_UW"], DESIGN_YEARS / b["D30_DW"]
            ls_a, ls_b = lsys(lu_a, ld_a), lsys(lu_b, ld_b)
            w.writerow([i, n, a["k"], a["ksel"], a["nbin"],
                        f"{a['D30_UW']:.6f}", f"{b['D30_UW']:.6f}",
                        f"{(b['D30_UW']/a['D30_UW']-1)*100:+.3f}",
                        f"{a['D30_DW']:.6f}", f"{b['D30_DW']:.6f}",
                        f"{(b['D30_DW']/a['D30_DW']-1)*100:+.3f}",
                        f"{lu_a:.3f}", f"{lu_b:.3f}",
                        f"{ls_a:.3f}", f"{ls_b:.3f}",
                        f"{(ls_b/ls_a-1)*100:+.3f}",
                        f"{a['D30_UW']/sU['DIN']*100:.3f}",
                        f"{a['t_s']:.2f}", f"{b['t_s']:.2f}"])

    # ── fleet_summary.csv ──
    LU = {t: DESIGN_YEARS / sU[t] for t, _ in CASES}
    LD = {t: DESIGN_YEARS / sD[t] for t, _ in CASES}
    LS = {t: lsys(LU[t], LD[t]) for t, _ in CASES}

    def pct(a, b):
        return "" if (a is None or b in (None, 0)) else f"{(a/b-1)*100:+.3f}"

    lines = [["category", "item", "unit", "DIN", "GOHAR", "GOH_vs_DIN_pct",
              "appendix4", "DIN_vs_app4_pct", "GOH_vs_app4_pct"]]
    lines.append(["프로파일", "단부드롭", "um", f"{drops['DIN']:.2f}",
                  f"{drops['GOH']:.2f}", pct(drops['GOH'], drops['DIN']), "", "", ""])
    for item, unit, a, b, ref in (
            ("Sigma D30_UW", "-", sU["DIN"], sU["GOH"], APP4["sumD30_UW"]),
            ("Sigma D30_DW", "-", sD["DIN"], sD["GOH"], None),
            ("fleet life UW", "yr", LU["DIN"], LU["GOH"], APP4["life_UW"]),
            ("fleet life DW", "yr", LD["DIN"], LD["GOH"], None),
            ("fleet life Sys", "yr", LS["DIN"], LS["GOH"], APP4["life_Sys"])):
        lines.append(["함대", item, unit, f"{a:.4f}", f"{b:.4f}", pct(b, a),
                      "" if ref is None else f"{ref:.4f}",
                      pct(a, ref), pct(b, ref)])
    tt = {t: sum(r["t_s"] for r in fat[t].values()) for t, _ in CASES}
    lines.append(["함대", "total analysis time", "s", f"{tt['DIN']:.0f}",
                  f"{tt['GOH']:.0f}", pct(tt['GOH'], tt['DIN']), "", "", ""])

    worst = {"DIN": (None, 0.0), "GOH": (None, 0.0)}
    for nm in EXT_TQ:
        for brg in ("UW", "DW"):
            va = ext["DIN"].get(nm, {}).get(brg, {}).get("mx")
            vb = ext["GOH"].get(nm, {}).get(brg, {}).get("mx")
            if va is None or vb is None:
                continue
            va, vb = va / 1e6, vb / 1e6
            if va > worst["DIN"][1]:
                worst["DIN"] = (f"{nm}/{brg}", va)
            if vb > worst["GOH"][1]:
                worst["GOH"] = (f"{nm}/{brg}", vb)
            lines.append([f"극한:{brg}", nm, "MPa", f"{va:.1f}", f"{vb:.1f}",
                          pct(vb, va), "", "", ""])
    for t, _ in CASES:
        pass
    lines.append(["극한", "worst case", "-", worst["DIN"][0], worst["GOH"][0],
                  "", "", "", ""])
    lines.append(["극한", "max stress", "MPa", f"{worst['DIN'][1]:.1f}",
                  f"{worst['GOH'][1]:.1f}", pct(worst['GOH'][1], worst['DIN'][1]),
                  f"{STRESS_LIMIT:.0f}",
                  f"{(worst['DIN'][1]/STRESS_LIMIT-1)*100:+.1f}",
                  f"{(worst['GOH'][1]/STRESS_LIMIT-1)*100:+.1f}"])
    lines.append(["극한", "margin to 2100MPa", "MPa",
                  f"{STRESS_LIMIT-worst['DIN'][1]:.1f}",
                  f"{STRESS_LIMIT-worst['GOH'][1]:.1f}", "", "", "", ""])
    with open(os.path.join(OUTDIR, "fleet_summary.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        csv.writer(f).writerows(lines)

    print("\n" + "=" * 66)
    print(f"  단부드롭       DIN {drops['DIN']:8.2f} um   GOH {drops['GOH']:8.2f} um")
    print(f"  Sigma D30_UW   DIN {sU['DIN']:8.4f}      GOH {sU['GOH']:8.4f}"
          f"   ({(sU['GOH']/sU['DIN']-1)*100:+.2f}%)")
    print(f"  fleet UW [yr]  DIN {LU['DIN']:8.3f}      GOH {LU['GOH']:8.3f}")
    print(f"  fleet Sys [yr] DIN {LS['DIN']:8.3f}      GOH {LS['GOH']:8.3f}"
          f"   ({(LS['GOH']/LS['DIN']-1)*100:+.2f}%)")
    print(f"  부록4 대조     SumD30_UW {APP4['sumD30_UW']:.3f} -> DIN "
          f"{(sU['DIN']/APP4['sumD30_UW']-1)*100:+.2f}%  "
          f"GOH {(sU['GOH']/APP4['sumD30_UW']-1)*100:+.2f}%")
    print(f"  극한 최대응력  DIN {worst['DIN'][1]:7.1f} MPa ({worst['DIN'][0]})"
          f"   GOH {worst['GOH'][1]:7.1f} MPa ({worst['GOH'][0]})")
    print(f"  2100 MPa 대비  DIN {(worst['DIN'][1]/STRESS_LIMIT-1)*100:+.1f}%   "
          f"GOH {(worst['GOH'][1]/STRESS_LIMIT-1)*100:+.1f}%")
    print(f"[저장] {OUTDIR}")


if __name__ == "__main__":
    main()
