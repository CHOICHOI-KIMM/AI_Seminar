"""
전량 파이프라인 완주 후 — 불합격 DLC 일괄 재처리 (실측 2점 중점 타겟팅)
=====================================================================
※ 반드시 run_fullfleet 완주(프로세스 종료) 후 단독 실행 — MASTA 동시 점유 금지.

대상: masta_best_summary.csv 의 pass==0 DLC
방법(진동 회피):
  현 선정 로직은 0.05 그리드 + 0.01 반올림이라 폭 ~0.01 합격창을 못 맞힘.
  → 참값(masta_ref_dmg.csv, 기확보) 재사용 + 조합만 재해석하여
    ① 기록된 dt에서 실측 2점(k±Δ)으로 ε_UW(k)·ε_Sys(k) 선형화
    ② 실측 합격창 [k: ε_Sys=0] ~ [k: ε_UW=3] 의 중점 k* 산출 → k*에서 1회 검증
    ③ 실패(창 없음/미명중) 시 Δ 좁혀 1회 재시도, 그래도 없으면 dt 한 단계 축소 후 반복
    ④ dt 소진 시 불합격 유지 기록
각 DLC ≈ 조합 6~8회 × 6s ≈ 1분. 갱신: best_summary + masta_cmb_detail + §5-4 행.
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
E_W = 9.0 / 8.0
MEM_LIMIT = 90.0
DTS = [20, 10, 4, 2, 1, 0.6]
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")
DOC = os.path.join(ROOT, "DLC기반_피로해석_DLC별해석.md")


def load_meta():
    return {r["DLC"]: r for r in csv.DictReader(
        open(os.path.join(HERE, "dlc_meta.csv"), encoding="utf-8-sig"))}


def load_raw(name):
    return [{k: float(v) for k, v in r.items()} for r in csv.DictReader(
        open(os.path.join(HERE, name, "raw.csv"), encoding="utf-8-sig"))]


def lsys(lu, ld):
    return (lu ** -E_W + ld ** -E_W) ** (-1.0 / E_W)


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


def main():
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

    meta = load_meta()
    failed = []
    for d in sorted(os.listdir(HERE)):
        p = os.path.join(HERE, d, "masta_best_summary.csv")
        if not os.path.isfile(p):
            continue
        r = list(csv.DictReader(open(p, encoding="utf-8-sig")))[0]
        if int(float(r["pass"])) == 0:
            failed.append((d, float(r["dt"]), float(r["k"])))
    print(f"[재처리 대상] 불합격 {len(failed)}개: {[f[0] for f in failed]}")
    if not failed:
        print("불합격 없음 — 종료")
        return

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

    def analyze(reps, tagp):
        dmg = {"UW": [], "DW": []}
        t_anal = 0.0
        for b0 in range(0, len(reps), NBATCH):
            chunk = reps[b0:b0 + NBATCH]
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
            for key, b in bearings.items():
                subs = list(list(csd.results_for(b))[0].component_analysis_cases)
                for (cid, t_s, rev, rec), sub in zip(chunk, subs):
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
                raise MemoryError("메모리 90% 초과")
        return dmg, t_anal

    for name, dt0_rec, k0_rec in failed:
        sf = float(meta[name]["ScaleFactor"])
        data = load_raw(name)
        refrows = list(csv.DictReader(open(
            os.path.join(HERE, name, "masta_ref_dmg.csv"), encoding="utf-8-sig")))
        DrefU = sum(float(r["dmg_UW"]) for r in refrows)
        DrefD = sum(float(r["dmg_DW"]) for r in refrows)
        lifeU = 30.0 / (DrefU * sf)
        lifeD = 30.0 / (DrefD * sf)
        lifeS = lsys(lifeU, lifeD)
        print(f"\n===== {name} (기록 dt={dt0_rec:g}, k={k0_rec:g}) =====")

        def measure(dt, k):
            reps = bin_reps(data, dt, k)
            dmg, t = analyze(reps, f"rp_{name[-4:]}")
            DU, DD = sum(dmg["UW"]), sum(dmg["DW"])
            eU = (DU / DrefU - 1) * 100
            lU, lD = 30.0 / (DU * sf), 30.0 / (DD * sf)
            eS = (lifeS / lsys(lU, lD) - 1) * 100
            eD = (DD / DrefD - 1) * 100
            return eU, eS, eD, DU * sf, t

        solved = None
        neval = 0
        for dt in [d for d in DTS if d <= max(dt0_rec, 20)]:
            if dt > dt0_rec:
                continue
            # 실측 2점 (k±0.02, 그리드 근처)
            klo, khi = max(k0_rec - 0.02, 0.01), k0_rec + 0.02
            eUlo, eSlo, _, _, t1 = measure(dt, klo); neval += 1
            eUhi, eShi, _, _, t2 = measure(dt, khi); neval += 1
            sU = (eUhi - eUlo) / (khi - klo)
            sS = (eShi - eSlo) / (khi - klo)
            print(f"  [dt={dt:g}] k={klo:.3f}:(U{eUlo:+.2f},S{eSlo:+.2f}) "
                  f"k={khi:.3f}:(U{eUhi:+.2f},S{eShi:+.2f}) sU={sU:.0f} sS={sS:.0f}")
            if abs(sU) < 1e-6 or abs(sS) < 1e-6:
                continue
            # 합격창: εSys=0 → k_S0, εUW=3 → k_U3
            k_S0 = klo + (0 - eSlo) / sS
            k_U3 = klo + (3 - eUlo) / sU
            k_lo_win, k_hi_win = min(k_S0, k_U3), max(k_S0, k_U3)
            # UW 하한(0)·Sys 상한(3)도 고려
            k_U0 = klo + (0 - eUlo) / sU
            k_S3 = klo + (3 - eSlo) / sS
            lo = max(k_lo_win, min(k_U0, k_S3) if False else 0.0)
            win_lo = max(k_S0, k_U0)         # 둘 다 ≥0 시작
            win_hi = min(k_U3, k_S3)         # 둘 다 ≤3 끝
            if win_hi <= win_lo:
                print(f"    창 없음(win[{win_lo:.3f},{win_hi:.3f}]) → dt 축소")
                continue
            kstar = round((win_lo + win_hi) / 2, 3)
            eU, eS, eD, d30c, t3 = measure(dt, kstar); neval += 1
            ok = (0 <= eU <= 3) and (0 <= eS <= 3)
            print(f"    k*={kstar:.3f} → εUW={eU:+.2f} εSys={eS:+.2f} εDW={eD:+.2f} "
                  f"{'✅' if ok else '❌ 재시도'}")
            if not ok:
                # 1회 미세 조정: 중앙 재타겟
                adj = kstar + (1.5 - eS) / sS if not (0 <= eS <= 3) else \
                      kstar + (1.5 - eU) / sU
                kstar = round(min(max(adj, win_lo), win_hi), 3)
                eU, eS, eD, d30c, t4 = measure(dt, kstar); neval += 1
                ok = (0 <= eU <= 3) and (0 <= eS <= 3)
                print(f"    재시도 k*={kstar:.3f} → εUW={eU:+.2f} εSys={eS:+.2f} "
                      f"{'✅' if ok else '❌'}")
            if ok:
                solved = (dt, kstar, eU, eS, eD, d30c)
                break
        if solved:
            dt, kstar, eU, eS, eD, d30c = solved
            lU, lD = lifeU, lifeD
            with open(os.path.join(HERE, name, "masta_best_summary.csv"), "w",
                      newline="", encoding="utf-8-sig") as fo:
                wo = csv.writer(fo)
                wo.writerow(["dt", "k", "ncorr", "eps_UW", "eps_DW", "eps_Sys",
                             "life_UW_ref", "life_DW_ref", "life_Sys_ref",
                             "D30_UW_ref", "D30_DW_ref", "D30_UW_cmb",
                             "t_ref_min", "t_cmb_s", "pass"])
                wo.writerow([dt, kstar, f"R{neval}", eU, eD, eS, lifeU, lifeD, lifeS,
                             DrefU * sf, DrefD * sf, d30c, "", "", 1])
            print(f"  ✅ 해결: dt={dt:g} k={kstar:.3f} (평가 {neval}회)")
        else:
            print(f"  ❌ 미해결 유지 (평가 {neval}회)")

    print("\n[재처리 완료] best_summary 갱신됨 → make_dlc_master_xlsx.py + §5-4 재생성 권장")


if __name__ == "__main__":
    main()
