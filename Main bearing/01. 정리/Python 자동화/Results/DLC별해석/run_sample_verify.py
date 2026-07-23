"""
DLC별 해석 · 2단계 표본 선행 검증 (표본 2·3) — 무인, 배치 N=20
================================================================
표본: DLC1.2-d-s1 (20, 0.25) · DLC6.4-a-s3 (20, 0.05)
각 표본: 참값(dt=0.1, 6001점) + 최적조합(30빈) MASTA 해석 → ε_UW/ε_Sys/ε_DW·수명·시간
→ DLC별해석.md §5-2 표의 해당 행 실시간 입력, 산출 CSV는 DLC 폴더 저장
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
SAMPLES = [("DLC1.2-d-s1", 20, 0.25), ("DLC6.4-a-s3", 20, 0.05)]
NBATCH = 20
DT0 = 0.1
E_W = 9.0 / 8.0
MEM_LIMIT = 90.0
DOC = os.path.join(ROOT, "DLC기반_피로해석_DLC별해석.md")
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")


def load_raw(name):
    rows = []
    for r in csv.DictReader(open(os.path.join(HERE, name, "raw.csv"),
                                 encoding="utf-8-sig")):
        rows.append({k: float(v) for k, v in r.items()})
    return rows


def load_sf(name):
    for r in csv.DictReader(open(os.path.join(HERE, "dlc_meta.csv"),
                                 encoding="utf-8-sig")):
        if r["DLC"] == name:
            return float(r["ScaleFactor"])
    raise KeyError(name)


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
        rec["My"] = rec.get("My", 0.0)   # KSIG 에 이미 포함
        out.append((bi, rec["rpm"] / 60.0 * (m * DT0), rec))
    return out


def update_row(name, text_cells):
    """§5-2 표에서 '| ... | {name} | ...' 행을 찾아 값 칸 교체."""
    lines = open(DOC, encoding="utf-8").read().split("\n")
    for i, ln in enumerate(lines):
        if f"| {name} |" in ln and ln.startswith("|"):
            parts = [p.strip() for p in ln.split("|")]
            # parts: '', 유형, DLC, (dt,k), εUW, εSys, εDW, 수명, 시간, 판정, ''
            parts[4:10] = text_cells
            lines[i] = "| " + " | ".join(parts[1:10]) + " |"
            break
    open(DOC, "w", encoding="utf-8").write("\n".join(lines))


def note_progress(msg):
    MSx, MEx = "<!-- DLC_PROGRESS_START -->", "<!-- DLC_PROGRESS_END -->"
    txt = open(DOC, encoding="utf-8").read()
    block = f"{MSx}\n\n> 1단계 스크리닝: **111/111 DLC 완료** · κ클립 45개 · 스킵 0개\n> 2단계 표본 검증: {msg}\n\n{MEx}"
    txt = txt.split(MSx)[0] + block + txt.split(MEx, 1)[1]
    open(DOC, "w", encoding="utf-8").write(txt)


def main():
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

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

    def analyze_batch(cases_recs, tagp):
        """[(id, rev, rec)] → {'UW':[dmg...], 'DW':[...]} + 해석시간"""
        dmg = {"UW": [], "DW": []}
        t_anal = 0.0
        for b0 in range(0, len(cases_recs), NBATCH):
            chunk = cases_recs[b0:b0 + NBATCH]
            lcs = []
            for cid, rev, rec in chunk:
                lc = lc0.duplicate(ds, f"{tagp}_{cid}")
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

    # 웜업
    d0 = load_raw(SAMPLES[0][0])
    mf.set_loads(lc0, pl, ipl, d0[0])
    lc0.analysis_of(AnalysisType.SYSTEM_DEFLECTION).perform_analysis()
    print("[웜업] 완료")

    for name, dt, k in SAMPLES:
        sf = load_sf(name)
        data = load_raw(name)
        print(f"\n===== {name} (dt={dt}, k={k}, SF={sf:,.0f}) =====")
        note_progress(f"{name} 참값(dt=0.1) 해석 중…")
        # ── 참값 (점별) ──
        cases = [(i, r["rpm"] / 60.0 * DT0, r) for i, r in enumerate(data)]
        t0 = time.perf_counter()
        dref, t_ref = analyze_batch(cases, f"rf{name[-4:]}")
        DrefU, DrefD = sum(dref["UW"]), sum(dref["DW"])
        print(f"[참값] {len(cases)}점 해석 {t_ref:.0f}s  DrefU={DrefU:.4e} DrefD={DrefD:.4e}")
        with open(os.path.join(HERE, name, "masta_ref_summary.csv"), "w",
                  newline="", encoding="utf-8-sig") as f:
            w = csv.writer(f)
            w.writerow(["bearing", "sample_damage", "D30", "life_yr"])
            for key, D in (("UW", DrefU), ("DW", DrefD)):
                w.writerow([key, D, D * sf, 30.0 / (D * sf) if D > 0 else ""])
        note_progress(f"{name} 최적조합 (dt={dt}, k={k}) 해석 중…")
        # ── 최적조합 (빈) ──
        reps = bin_reps(data, dt, k)
        dcmb, t_cmb = analyze_batch(reps, f"bs{name[-4:]}")
        DU, DD = sum(dcmb["UW"]), sum(dcmb["DW"])
        eU = (DU / DrefU - 1) * 100 if DrefU > 0 else float("nan")
        eD = (DD / DrefD - 1) * 100 if DrefD > 0 else float("nan")
        lifeU = 30.0 / (DU * sf)
        lifeD = 30.0 / (DD * sf)
        lifeUr = 30.0 / (DrefU * sf)
        lifeDr = 30.0 / (DrefD * sf)
        ls = (lifeU ** -E_W + lifeD ** -E_W) ** (-1 / E_W)
        lsr = (lifeUr ** -E_W + lifeDr ** -E_W) ** (-1 / E_W)
        eS = (lsr / ls - 1) * 100
        passed = (0 <= eU <= 3) and (0 <= eS <= 3)
        print(f"[조합] {len(reps)}빈 {t_cmb:.1f}s  εUW={eU:+.2f}% εSys={eS:+.2f}% "
              f"εDW={eD:+.2f}% → {'합격' if passed else '불합격'}")
        with open(os.path.join(HERE, name, "masta_best_summary.csv"), "w",
                  newline="", encoding="utf-8-sig") as f:
            w = csv.writer(f)
            w.writerow(["dt", "k", "eps_UW", "eps_DW", "eps_Sys",
                        "life_UW", "life_DW", "life_Sys", "anal_s"])
            w.writerow([dt, k, eU, eD, eS, lifeU, lifeD, ls, t_cmb])
        # ── 표 행 입력 ──
        # 기존 셀에서 예측값 추출 유지: 행 재구성 시 예측은 스크리닝 맵에서 다시 읽음
        E = {}
        for m in csv.DictReader(open(os.path.join(HERE, name, "screen_eps_map.csv"),
                                     encoding="utf-8-sig")):
            E[(float(m["k"]), float(m["dt_s"]))] = (float(m["eps_UW_pct"]),
                                                    float(m["eps_Sys_pct"]))
        pU, pS = E.get((k, float(dt)), (float("nan"), float("nan")))
        mark = "**합격 ✅**" if passed else "**불합격 ❌**"
        update_row(name, [
            f"**{eU:+.2f}%**({pU:+.2f})",
            f"**{eS:+.2f}%**({pS:+.2f})",
            f"{eD:+.2f}%",
            f"{lifeU:,.1f} / {lifeD:,.1f} / {ls:,.1f}",
            f"{t_cmb:.1f} s",
            mark])
        note_progress(f"{name} 완료 ({'합격' if passed else '불합격'})")

    note_progress("표본 2·3 완료")
    print("\n표본 검증 완료")


if __name__ == "__main__":
    main()
