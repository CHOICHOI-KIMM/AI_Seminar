"""
부록 10 · 3단계 추가 — N=20, 30 측정 후 기존 표(§10-6.2)·CSV에 정렬 삽입
측정 방식·컬럼은 run_batch_study.py 와 동일. 오차 판정용 기준 L10 은 재측정(행 추가 없음).
"""
import csv
import math
import os
import sys
import threading
import time

import psutil

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)

import masta_clr_legacy  # noqa: F401,E402
import mastapy           # noqa: E402
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design                                    # noqa: E402
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType  # noqa: E402
import c1_pin  # noqa: E402

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안"
         r"_피로하중 반영_유연체_FE_온도_50도_260721.Masta")
RPM2RADS = 2 * math.pi / 60
N_PTS = 100
NS_NEW = [20, 30]
MEM_LIMIT = 90.0
DOC = os.path.join(ROOT, "DLC기반_피로해석_프로세스_v1.md")
MS, ME = "<!-- BATCH_STUDY_START -->", "<!-- BATCH_STUDY_END -->"
CSV_OUT = os.path.join(HERE, "batch_study_results.csv")
FIELDS = ["condition", "batches", "prep_s", "analysis_s", "ms_per_point",
          "eff_cores", "peak_rss_MB", "peak_sysmem_pct", "max_rel_err", "status"]
PROC = psutil.Process()


class MemSampler(threading.Thread):
    def __init__(self):
        super().__init__(daemon=True)
        self.stop_ev = threading.Event()
        self.peak_rss = 0.0
        self.peak_sys = 0.0

    def run(self):
        while not self.stop_ev.is_set():
            try:
                self.peak_rss = max(self.peak_rss, PROC.memory_info().rss / 2**20)
                self.peak_sys = max(self.peak_sys, psutil.virtual_memory().percent)
            except Exception:
                pass
            self.stop_ev.wait(0.2)

    def stop(self):
        self.stop_ev.set()
        self.join(timeout=1)


def nkey(cond):
    try:
        return int(cond.split("=")[1].split(" ")[0])
    except Exception:
        return 0


def load_rows():
    rows = []
    if os.path.exists(CSV_OUT):
        for r in csv.DictReader(open(CSV_OUT, encoding="utf-8-sig")):
            for k in ("prep_s", "analysis_s", "ms_per_point", "eff_cores",
                      "peak_rss_MB", "peak_sysmem_pct"):
                r[k] = float(r[k])
            r["max_rel_err"] = (None if r["max_rel_err"] in ("", "None")
                                else float(r["max_rel_err"]))
            r["batches"] = int(r["batches"])
            rows.append(r)
    return rows


def update_doc(rows, note=""):
    rows = sorted(rows, key=lambda r: nkey(r["condition"]))
    L = [MS, "",
         f"> 대상: 50 °C 모델 · 시계열 index 0~{N_PTS-1} ({N_PTS}점) · RAM 16 GB / 12코어 · "
         f"메모리 상한 {MEM_LIMIT:.0f}% · 일치 기준 상대오차 ≤1e-6. {note}", "",
         "| 조건 | 배치수 | 준비 [s] | 해석 [s] | **점당 [ms]** | 유효코어 | 피크 RSS [MB] "
         "| 피크 시스템 메모리 | 최대 상대오차 | 상태 |",
         "|------|-------:|--------:|--------:|-------------:|--------:|-------------:"
         "|------------------:|-------------:|:----:|"]
    for r in rows:
        err = r["max_rel_err"]
        errs = "–" if err is None else f"{err:.1e} {'✅' if err <= 1e-6 else '❌'}"
        L.append(f"| {r['condition']} | {r['batches']} | {r['prep_s']:.1f} | "
                 f"{r['analysis_s']:.1f} | **{r['ms_per_point']:.0f}** | "
                 f"{r['eff_cores']:.1f} | {r['peak_rss_MB']:,.0f} | "
                 f"{r['peak_sysmem_pct']:.1f}% | {errs} | {r['status']} |")
    L += ["", ME]
    txt = open(DOC, encoding="utf-8").read()
    txt = txt.split(MS)[0] + "\n".join(L) + txt.split(ME, 1)[1]
    open(DOC, "w", encoding="utf-8").write(txt)
    with open(CSV_OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=FIELDS)
        w.writeheader()
        w.writerows(rows)


def set_loads(lc, rec, pl, ipl):
    p = lc.inputs_for_point_load(pl)
    p.force_x.force = -rec["Fz"] * 1e3
    p.force_y.force = rec["Fy"] * 1e3
    p.axial_load.force = rec["Fx"] * 1e3
    p.moment_x.moment = -rec["Mz"] * 1e3
    p.moment_y.moment = rec["My"] * 1e3
    pw = lc.inputs_for_power_load(ipl)
    pw.speed = rec["rpm"] * RPM2RADS
    pw.torque = rec["Mx"] * 1e3


def cpu_secs():
    t = PROC.cpu_times()
    return t.user + t.system


def main():
    rows = load_rows()
    print(f"[기존 표] {len(rows)}행 로드")
    data = c1_pin.parse_dlc(c1_pin.DLC)[:N_PTS]
    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    dp = asm.design_properties
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
    uw = next(b for b in asm.all_parts_of_type_bearing() if "UW" in str(b))
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    ds = lc0.design_state_load_case_group

    # 웜업 + 오차 판정용 기준 L10 재측정 (행 추가 없음)
    base = []
    set_loads(lc0, data[0], pl, ipl)
    lc0.analysis_of(AnalysisType.SYSTEM_DEFLECTION).perform_analysis()
    t0 = time.perf_counter()
    for i in range(N_PTS):
        set_loads(lc0, data[i], pl, ipl)
        sd = lc0.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
        sd.perform_analysis()
        base.append(sd.results_for(uw).component_detailed_analysis
                    .iso2812007.basic_rating_life_cycles)
    print(f"[기준 L10 재측정] {time.perf_counter()-t0:.1f}s (행 미추가)")

    for N in NS_NEW:
        nb = math.ceil(N_PTS / N)
        prep = anal = dc_tot = 0.0
        errs = []
        status = "완료 ✅"
        smp = MemSampler(); smp.start()
        for b in range(nb):
            idxs = list(range(b * N, min((b + 1) * N, N_PTS)))
            t0 = time.perf_counter()
            cases = []
            for j, i in enumerate(idxs):
                lc = lc0.duplicate(ds, f"ad{N}_{b}_{j}")
                set_loads(lc, data[i], pl, ipl)
                cases.append(lc)
            duty = dp.add_duty_cycle(f"ad{N}_dc{b}")
            for lc in cases:
                duty.add_static_load(lc)
            prep += time.perf_counter() - t0

            c0, t0 = cpu_secs(), time.perf_counter()
            csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            csd.perform_analysis()
            anal += time.perf_counter() - t0
            dc_tot += cpu_secs() - c0

            res = list(csd.results_for(uw))[0]
            sub = list(res.component_analysis_cases)
            if len(sub) != len(idxs):
                status = f"결과수 불일치({len(sub)}/{len(idxs)}) ❌"
            else:
                for j, i in enumerate(idxs):
                    v = (sub[j].component_detailed_analysis
                         .iso2812007.basic_rating_life_cycles)
                    errs.append(abs(v / base[i] - 1))
            for lc in cases:
                try:
                    lc.delete()
                except Exception:
                    pass
            try:
                duty.delete()
            except Exception:
                pass
            if psutil.virtual_memory().percent > MEM_LIMIT:
                status = f"메모리 초과 중단 ⛔ (배치 {b+1}/{nb})"
                break
        smp.stop()
        done = len(errs) if errs else max(1, N * (b + 1))
        rows.append(dict(condition=f"N={N}", batches=nb, prep_s=prep, analysis_s=anal,
                         ms_per_point=anal / done * 1000,
                         eff_cores=dc_tot / anal if anal else 0,
                         peak_rss_MB=smp.peak_rss, peak_sysmem_pct=smp.peak_sys,
                         max_rel_err=max(errs) if errs else None, status=status))
        update_doc(rows, f"N=20·30 추가 (260722)")
        print(f"[N={N}] 준비 {prep:.1f}s 해석 {anal:.1f}s ({anal/done*1000:.0f} ms/점) "
              f"유효코어 {dc_tot/anal:.1f} 오차max {max(errs) if errs else -1:.1e} → {status}")

    print("\n추가 측정 완료")


if __name__ == "__main__":
    main()
