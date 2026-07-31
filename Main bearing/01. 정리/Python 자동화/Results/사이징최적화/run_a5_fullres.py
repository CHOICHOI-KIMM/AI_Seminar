"""
부록 5 §5-9 — 빈 대표화 검증 (전점 실치 대조)
================================================
`DLC2.4.2-a-s1` 1건에 대해 A1·B1 두 모델의 κ·λ·a_ISO 를 세 가지 방식으로 구해
dt=20 빈 대표화가 얼마나 편향되는지 정량한다.

  C0  빈 산술평균 rpm + 빈 μ+kσ 하중      15 LC   ← 현행 (부록 5 재사용)
  C1  점별 실치 rpm + 빈 μ+kσ 하중     3,001 LC   ← rpm 효과 단독
  C2  점별 실치 rpm + 점별 실치 하중    3,001 LC   ← 참값

κ ∝ n^0.83 · h_min ∝ n^0.68 로 둘 다 오목함수라 Jensen 부등식상 빈 평균은
κ·λ 를 과대평가한다. C1−C0 이 그 순효과, C2−C1 이 하중 대표화의 순효과다.

500점마다 update_a5_fullres.main() 을 불러 문서 §5-9.3 표를 갱신한다.
산출: 부록5_윤활조건/fullres_per_point.csv
"""
import csv
import math
import os
import sys
import time

import psutil

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
DLCDIR = os.path.join(RES, "DLC별해석")
OUTDIR = os.path.join(HERE, "부록5_윤활조건")
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg              # noqa: E402
import update_a5_fullres as upd       # noqa: E402
from run_appendix5_lub import (MODEL, NBATCH, DT0, DT, MEM_LIMIT, KSIG,
                               K_PH2, safe, sc, load_raw, extract)  # noqa: E402

DLC = "DLC2.4.2-a-s1"
TARGETS = [("A1", 3.900, 23.0, 0.230, 0.475, 1.0, 5.0),
           ("B1", 3.900, 27.0, 0.230, 0.475, 1.5, 5.5)]
CASES = ("C1", "C2")                  # C0 는 lub_per_bin.csv 재사용
EVERY = 500                           # 문서 갱신 주기 [점]
FIELDS = ["model", "case", "idx", "bin", "rpm", "rev", "brg",
          "kappa", "lambda_in", "lambda_out", "hmin_in_um", "hmin_out_um",
          "nu_mm2s", "nu1_mm2s", "a_iso", "a_iso_capped"]


def bin_edges(n):
    kp = int(round(DT / DT0))
    nb = max(n // kp, 1)
    e = [(b * kp, (b + 1) * kp) for b in range(nb)]
    if e and e[-1][1] < n:
        e[-1] = (e[-1][0], n)
    return e


def bin_loads(data, k):
    """빈별 μ+kσ 하중 (rpm·Mx 는 산술평균) — run_appendix5_lub.bin_reps 와 동일"""
    out = []
    for i0, i1 in bin_edges(len(data)):
        m = i1 - i0
        rec = {key: sum(data[i][key] for i in range(i0, i1)) / m
               for key in ("rpm", "Mx")}
        for key in KSIG:
            mu = sum(data[i][key] for i in range(i0, i1)) / m
            var = sum((data[i][key] - mu) ** 2 for i in range(i0, i1)) / m
            rec[key] = mu + math.copysign(1.0, mu) * k * math.sqrt(var)
        out.append(rec)
    return out


def load_k(design):
    with open(K_PH2, encoding="utf-8-sig") as f:
        return {r["DLC"]: float(r["k"]) for r in csv.DictReader(f)
                if r["design"] == design}[DLC]


def build_points(data, k):
    """(case, idx, bin, rec) — C1 은 빈 하중 + 점 rpm, C2 는 전부 점 실치"""
    reps = bin_loads(data, k)
    owner = {}
    for b, (i0, i1) in enumerate(bin_edges(len(data))):
        for i in range(i0, i1):
            owner[i] = b
    jobs = {"C1": [], "C2": []}
    for i, row in enumerate(data):
        b = owner[i]
        c1 = dict(reps[b])
        c1["rpm"] = row["rpm"]
        jobs["C1"].append((i, b, c1))
        jobs["C2"].append((i, b, {key: row[key]
                                  for key in ("rpm", "Mx") + KSIG}))
    return jobs


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
    from mastapy.bearings import RollerBearingProfileTypes as RP

    os.makedirs(OUTDIR, exist_ok=True)
    out = os.path.join(OUTDIR, "fullres_per_point.csv")
    done = set()
    if os.path.isfile(out):
        with open(out, encoding="utf-8-sig") as f:
            done = {(r["model"], r["case"], int(r["idx"]))
                    for r in csv.DictReader(f)}
    data = load_raw(DLC)
    print(f"[§5-9] {DLC} · {len(data):,}점 · 모델 {len(TARGETS)} × 케이스 "
          f"{len(CASES)} · 기존 완료 {len(done):,}", flush=True)

    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    sh = list(asm.all_parts_of_type_shaft())[0]
    bs = list(asm.all_parts_of_type_bearing())
    bmap = {"UW": [b for b in bs if "UW" in str(b)][0],
            "DW": [b for b in bs if "DW" in str(b)][0]}
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    dp = asm.design_properties
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    ds = lc0.design_state_load_case_group
    for b in bs:
        b.detail.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG

    new = not os.path.isfile(out)
    fh = open(out, "a", newline="", encoding="utf-8-sig")
    wr = csv.DictWriter(fh, fieldnames=FIELDS)
    if new:
        wr.writeheader()

    t_all = time.time()
    for tag, dpw, al, dwe, lwe, z1, z2 in TARGETS:
        g = sg.bearing(dpw, al, dwe, lwe)
        for b in bs:
            try:
                if b.inner_connection is not None:
                    b.inner_connection.delete()
            except Exception:
                pass
        s = sg.shaft(g["bore"], z2)
        sh.remove_all_sections()
        sh.add_section(0.0, s["length"], s["outer_diameter"], s["inner_diameter"],
                       s["outer_diameter"], s["inner_diameter"])
        for b in bs:
            sg.apply_to_masta(b.detail, g)
        for b, z in ((bmap["UW"], z1), (bmap["DW"], z2)):
            b.try_mount_on(sh, z)
        k = load_k(tag)
        jobs = build_points(data, k)
        print(f"\n[{tag}] α {al:.0f} k {k:.2f} · 합성거칠기 "
              f"{sc(bmap['UW'].detail,'combined_surface_roughness_inner')*1e6:.4f} µm",
              flush=True)

        for case in CASES:
            todo = [j for j in jobs[case] if (tag, case, j[0]) not in done]
            if not todo:
                print(f"  {case} 이미 완료", flush=True)
                continue
            t_u, since = time.time(), 0
            for b0 in range(0, len(todo), NBATCH):
                chunk = todo[b0:b0 + NBATCH]
                lcs = []
                for idx, bi, rec in chunk:
                    lc = lc0.duplicate(ds, f"fr_{tag}_{case}_{idx}")
                    mf.set_loads(lc, pl, ipl, rec)
                    lcs.append(lc)
                duty = dp.add_duty_cycle(f"frdc_{tag}_{case}_{b0}")
                for lc in lcs:
                    duty.add_static_load(lc)
                csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
                csd.perform_analysis()
                for key, b in bmap.items():
                    subs = list(list(csd.results_for(b))[0].component_analysis_cases)
                    for (idx, bi, rec), sub in zip(chunk, subs):
                        wr.writerow(dict(
                            model=tag, case=case, idx=idx, bin=bi, brg=key,
                            rpm=round(rec["rpm"], 5),
                            rev=round(abs(rec["rpm"]) / 60.0 * DT0, 8),
                            **{kk: (None if vv is None else
                                    (vv if isinstance(vv, int) else round(vv, 6)))
                               for kk, vv in
                               extract(sub.component_detailed_analysis).items()}))
                for x in lcs + [duty]:
                    try:
                        x.delete()
                    except Exception:
                        pass
                since += len(chunk)
                if since >= EVERY or b0 + NBATCH >= len(todo):
                    fh.flush()
                    since = 0
                    try:
                        upd.main()
                    except Exception as e:
                        print(f"    [갱신 실패] {e}", flush=True)
                    el = time.time() - t_u
                    n = b0 + len(chunk)
                    print(f"    {case} [{n}/{len(todo)}] {el:.0f}s "
                          f"(ETA {el/n*(len(todo)-n):.0f}s)", flush=True)
                if psutil.virtual_memory().percent > MEM_LIMIT:
                    raise MemoryError("메모리 95% 초과")
            print(f"  → {tag}·{case} 완료 {(time.time()-t_u)/60:.1f}분", flush=True)

    fh.close()
    upd.main()
    print(f"\n[완료] {(time.time()-t_all)/60:.1f}분 · {out}", flush=True)


if __name__ == "__main__":
    main()
