"""
부록 5 검증용 — DLC1.2-d-s1 단일 MASTA 파일 생성 (30 로드케이스 전량 삽입)
=========================================================================
부록 5(run_appendix5.py)가 이 DLC에 대해 실제로 MASTA에 인가한 것과
**완전히 동일한** 대표하중 30개(dt=20 · k=0.27)를 하나의 .Masta 파일에
로드케이스로 삽입하고, 듀티사이클로 묶어 해석한 뒤 결과와 함께 저장한다.

목적: 스크립트가 산출한 per_dlc.csv / 총괄 엑셀 값과
      MASTA GUI에서 직접 확인하는 값이 일치하는지 대조.

산출:
  02. 자료/MASTA/..._프리로드_검증_DLC1.2-d-s1_30LC_260727.Masta
  부록5_preload_dt20/verify_DLC1.2-d-s1_bins.csv / .xlsx   (빈별 대조표)
"""
import csv
import math
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)

DLC = "DLC1.2-d-s1"
DT0, DT = 0.1, 20.0
E_W = 9.0 / 8.0
DESIGN_YEARS = 30.0
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_프리로드 적용_온도_50도_260726.Masta")
SAVE_AS = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
           r"\26MW_메인베어링_v1.4_프리로드_검증_DLC1.2-d-s1_30LC_260727.Masta")
OUTDIR = os.path.join(HERE, "부록5_preload_dt20")


def load_meta():
    return {r["DLC"]: r for r in csv.DictReader(
        open(os.path.join(HERE, "dlc_meta.csv"), encoding="utf-8-sig"))}


def load_k():
    for r in csv.DictReader(open(os.path.join(OUTDIR, "screen_k_dt20.csv"),
                                 encoding="utf-8-sig")):
        if r["DLC"] == DLC:
            return float(r["k"]), r["ksel"]
    raise SystemExit(f"{DLC} k 없음")


def load_raw():
    return [{k: float(v) for k, v in r.items()} for r in csv.DictReader(
        open(os.path.join(HERE, DLC, "raw.csv"), encoding="utf-8-sig"))]


def bin_reps(data, k):
    """run_appendix5.bin_reps 와 동일 (성분 평균 + sign*k*표준편차)"""
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
        out.append((bi, abs(rec["rpm"]) / 60.0 * (m * DT0), rec, m))
    return out


def lsys(lu, ld):
    return (lu ** -E_W + ld ** -E_W) ** (-1.0 / E_W)


def main():
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

    meta = load_meta()
    sf = float(meta[DLC]["ScaleFactor"])
    k, ktag = load_k()
    reps = bin_reps(load_raw(), k)
    print(f"[{DLC}] k={k}({ktag}) · dt={DT:g} · {len(reps)}빈 · ScaleFactor={sf:,.0f}")

    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    dp = asm.design_properties
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
    bearings = {("UW" if "UW" in str(b) else "DW"): b
                for b in asm.all_parts_of_type_bearing()}
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    ds = lc0.design_state_load_case_group
    print("[모델] 로드 완료 —", os.path.basename(MODEL))

    # ── 로드케이스 30개 생성 (삭제하지 않고 파일에 남긴다) ──
    lcs = []
    applied = []
    for bi, rev, rec, m in reps:
        lc = lc0.duplicate(ds, f"{DLC}_b{bi:02d}")
        loads = mf.set_loads(lc, pl, ipl, rec)
        lcs.append(lc)
        applied.append((bi, rev, rec, m, loads))
    duty = dp.add_duty_cycle(f"{DLC}_dt20_k{k:g}_30LC")
    for lc in lcs:
        duty.add_static_load(lc)
    print(f"[생성] 로드케이스 {len(lcs)}개 + 듀티사이클 '{DLC}_dt20_k{k:g}_30LC'")

    # ── 해석 ──
    t0 = time.perf_counter()
    csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
    csd.perform_analysis()
    t_anal = time.perf_counter() - t0
    print(f"[해석] 완료 {t_anal:.1f}s")

    res = {}
    for key, b in bearings.items():
        subs = list(list(csd.results_for(b))[0].component_analysis_cases)
        res[key] = []
        for sub in subs:
            d = sub.component_detailed_analysis
            l10 = d.isots162812008.modified_reference_rating_life_cycles
            try:
                p_eq = d.iso2812007.dynamic_equivalent_load
            except Exception:
                p_eq = float("nan")
            res[key].append((l10, p_eq))

    # ── 빈별 대조표 ──
    rows = []
    sumU = sumD = 0.0
    for i, (bi, rev, rec, m, loads) in enumerate(applied):
        l10U, pU = res["UW"][i]
        l10D, pD = res["DW"][i]
        dU = rev / l10U if (l10U and l10U > 0) else 0.0
        dD = rev / l10D if (l10D and l10D > 0) else 0.0
        sumU += dU; sumD += dD
        rows.append(dict(
            bin=bi, LoadCase=f"{DLC}_b{bi:02d}", n_pts=m,
            rpm=rec["rpm"], rev=rev,
            Fx_kN=rec["Fx"], Fy_kN=rec["Fy"], Fz_kN=rec["Fz"],
            Mx_kNm=rec["Mx"], My_kNm=rec["My"], Mz_kNm=rec["Mz"],
            MASTA_force_x_N=loads["force_x_N"], MASTA_force_y_N=loads["force_y_N"],
            MASTA_axial_N=loads["axial_load_N"],
            MASTA_moment_x_Nm=loads["moment_x_Nm"],
            MASTA_moment_y_Nm=loads["moment_y_Nm"],
            MASTA_torque_Nm=loads["Moment_z_Nm"],
            P_UW_N=pU, P_DW_N=pD,
            L10_UW_cycles=l10U, L10_DW_cycles=l10D,
            dmg_UW=dU, dmg_DW=dD))

    d30U, d30D = sumU * sf, sumD * sf
    lU, lD = DESIGN_YEARS / d30U, DESIGN_YEARS / d30D

    csvp = os.path.join(OUTDIR, f"verify_{DLC}_bins.csv")
    with open(csvp, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader(); w.writerows(rows)
    print(f"[저장] {csvp}")

    # ── 엑셀 (대조용) ──
    try:
        from openpyxl import Workbook
        wb = Workbook()
        ws = wb.active; ws.title = "bins"
        ws.append(list(rows[0]))
        for r in rows:
            ws.append([r[c] for c in rows[0]])
        s = wb.create_sheet("summary")
        for line in [
            ["DLC", DLC], ["dt [s]", DT], ["k", k], ["k 선정", ktag],
            ["빈 수", len(rows)], ["ScaleFactor", sf],
            ["Σ dmg_UW (1회)", sumU], ["Σ dmg_DW (1회)", sumD],
            ["D30_UW", d30U], ["D30_DW", d30D],
            ["수명 UW [yr]", lU], ["수명 DW [yr]", lD],
            ["수명 Sys [yr]", lsys(lU, lD)],
            ["해석시간 [s]", t_anal],
            ["", ""],
            ["부록5 per_dlc.csv 대조", "아래 값과 일치해야 함"],
            ["D30_UW (부록5)", 0.2324715364053867],
            ["D30_DW (부록5)", 0.04348126448929959],
            ["수명 Sys (부록5) [yr]", 113.82425209599418],
        ]:
            s.append(line)
        xp = os.path.join(OUTDIR, f"verify_{DLC}_bins.xlsx")
        wb.save(xp)
        print(f"[저장] {xp}")
    except ImportError:
        print("[skip] openpyxl 없음 — xlsx 생략")

    # ── 모델 저장 (결과 포함) ──
    st = design.save(SAVE_AS, True)
    print(f"[저장] {SAVE_AS}\n  status={st}")

    print(f"\n=== 대조 요약 ({DLC}) ===")
    print(f"  Σ dmg_UW(1회) {sumU:.6e} × SF {sf:,.0f} → D30_UW = {d30U:.6f}")
    print(f"  Σ dmg_DW(1회) {sumD:.6e} × SF {sf:,.0f} → D30_DW = {d30D:.6f}")
    print(f"  수명 UW {lU:,.2f} yr · DW {lD:,.2f} yr · Sys {lsys(lU, lD):,.2f} yr")
    print(f"  부록5 per_dlc.csv: D30_UW 0.232472 · D30_DW 0.043481 · Sys 113.82 yr")
    print(f"  차이: D30_UW {(d30U/0.2324715364053867-1)*100:+.4f}% · "
          f"D30_DW {(d30D/0.04348126448929959-1)*100:+.4f}%")


if __name__ == "__main__":
    main()
