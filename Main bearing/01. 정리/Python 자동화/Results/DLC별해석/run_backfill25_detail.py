"""
상위 25 DLC 조합(빈) 해석 상세 CSV 소급 생성 — 무인, 배치 N=20
================================================================
각 DLC의 확정 (dt, k)로 빈 대표하중을 재구성해 MASTA 재해석하고,
부록 9 데이터 양식(mf.DATA_HEADER: 인가하중·응력·s0·L10 4종·손상 4종·샤프트 SF)으로
<DLC>/masta_cmb_detail.csv 저장. 상세 추출 오버헤드(ms/case)를 기존 t_cmb_s와 대조 실측.
※ 참값 상세(masta_ref_detail.csv)는 향후 참값 전량 배치에서 동일 양식으로 생성 예정.
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
NBATCH = 20
DT0 = 0.1
MEM_LIMIT = 90.0
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")


def load_raw(name):
    rows = []
    for r in csv.DictReader(open(os.path.join(HERE, name, "raw.csv"),
                                 encoding="utf-8-sig")):
        rows.append({k: float(v) for k, v in r.items()})
    return rows


def bin_reps(data, dt, k):
    """→ [(bi, t_s, rpm, rev, rec)] — run_top25와 동일 규약 + 빈 시작시각."""
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
        rev = abs(rec["rpm"]) / 60.0 * (m * DT0)
        out.append((bi, i0 * DT0, rec["rpm"], rev, rec))
    return out


def main():
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

    # 대상: masta_best_summary.csv 보유 DLC (상위 25)
    targets = []
    for d in sorted(os.listdir(HERE)):
        p = os.path.join(HERE, d, "masta_best_summary.csv")
        if os.path.isfile(p):
            r = list(csv.DictReader(open(p, encoding="utf-8-sig")))[0]
            targets.append((d, float(r["dt"]), float(r["k"]), float(r["t_cmb_s"])))
    print(f"[대상] {len(targets)}개 DLC")

    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    dp = asm.design_properties
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    bearings = list(asm.all_parts_of_type_bearing())
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    ds = lc0.design_state_load_case_group
    print("[모델] 로드 완료 (50°C)")

    # 웜업
    d0 = load_raw(targets[0][0])
    mf.set_loads(lc0, pl, ipl, d0[0])
    lc0.analysis_of(AnalysisType.SYSTEM_DEFLECTION).perform_analysis()
    print("[웜업] 완료")

    grand_t = grand_n = 0
    for name, dt, k, t_prev in targets:
        data = load_raw(name)
        reps = bin_reps(data, dt, k)
        out_csv = os.path.join(HERE, name, "masta_cmb_detail.csv")
        f = open(out_csv, "w", newline="", encoding="utf-8-sig")
        w = csv.writer(f)
        w.writerow(mf.DATA_HEADER)
        t_anal = 0.0
        t_all0 = time.perf_counter()
        for b0 in range(0, len(reps), NBATCH):
            chunk = reps[b0:b0 + NBATCH]
            cases = []
            for bi, t_s, rpm, rev, rec in chunk:
                lc = lc0.duplicate(ds, f"cd_{name[-4:]}_{bi}")
                mf.set_loads(lc, pl, ipl, rec)
                cases.append(lc)
            duty = dp.add_duty_cycle(f"cddc_{name[-4:]}_{b0}")
            for lc in cases:
                duty.add_static_load(lc)
            t0 = time.perf_counter()
            csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            csd.perform_analysis()
            t_anal += time.perf_counter() - t0
            subs = {mf.bname(bb): list(list(csd.results_for(bb))[0]
                                       .component_analysis_cases)
                    for bb in bearings}
            for j, (bi, t_s, rpm, rev, rec) in enumerate(chunk):
                loads = {"force_x_N": -rec["Fz"] * 1e3, "force_y_N": rec["Fy"] * 1e3,
                         "axial_load_N": rec["Fx"] * 1e3, "moment_x_Nm": -rec["Mz"] * 1e3,
                         "moment_y_Nm": rec["My"] * 1e3, "Moment_z_Nm": rec["Mx"] * 1e3}
                lv = [loads[c] for c in mf.LOAD_COLS]
                for bb in bearings:
                    d = mf.g(subs[mf.bname(bb)][j], "component_detailed_analysis")
                    sin = mf.fnum(mf.g(d, "maximum_normal_stress_inner"))
                    sout = mf.fnum(mf.g(d, "maximum_normal_stress_outer"))
                    dm = [mf.damage(rev, mf.g(d, pth))
                          for _, pth, _, _ in mf.DAMAGE_DEFS]
                    w.writerow([bi, t_s, rpm, rev] + lv + [mf.bname(bb),
                               mf.num(sin / 1e6 if sin is not None else None),
                               mf.num(sout / 1e6 if sout is not None else None),
                               mf.num(mf.g(d, "iso762006.safety_factor")),
                               mf.num(mf.g(d, "iso2812007.basic_rating_life_cycles")),
                               mf.num(mf.g(d, "iso2812007.modified_rating_life_cycles")),
                               mf.num(mf.g(d, "isots162812008.basic_reference_rating_life_cycles")),
                               mf.num(mf.g(d, "isots162812008.modified_reference_rating_life_cycles"))]
                               + [mf.num(x) for x in dm] + ["", ""])
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
                raise MemoryError("메모리 90% 초과")
        f.close()
        t_all = time.perf_counter() - t_all0
        grand_t += t_all
        grand_n += len(reps)
        ovh = (t_all / t_prev - 1) * 100 if t_prev > 0 else float("nan")
        print(f"[{name}] {len(reps)}빈  해석 {t_anal:.1f}s / 전체 {t_all:.1f}s "
              f"({t_all / len(reps) * 1000:.0f} ms/case, 기존 {t_prev:.1f}s 대비 {ovh:+.0f}%)")

    print(f"\n[완료] 총 {grand_n}빈  {grand_t:.0f}s  평균 {grand_t / grand_n * 1000:.0f} ms/case")


if __name__ == "__main__":
    main()
