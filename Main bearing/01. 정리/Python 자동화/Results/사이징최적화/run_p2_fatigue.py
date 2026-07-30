"""
P2 Phase 1 Step 2 — 설계 13건 × 111 DLC dt=20 MASTA 피로해석 (문서 §8-3.4)
==========================================================================
run_appendix5.py 와 동일 해석 경로(LC 복제 + 듀티사이클 + ISO/TS 16281)이나,
설계마다 MASTA 모델을 재구성한다(P1 Phase 2 와 동일 순서).

  분리 → 샤프트 재구성 → 제원 주입(B→C→T) → 재장착 → 111 DLC × dt=20

체크포인트: (설계, DLC) 단위 append — 중단 시 최대 6초 손실
문서 갱신 : 설계 1건 완료마다 §8-3.6 표 재생성
MASTA 저장: 최경량 #1 · 기준선 — 피로해석 완료 후 (결정 260729)

산출: P2_피로수명_Phase1/fatigue_per_dlc.csv · fatigue_summary.csv · p2_selfcheck.txt
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
# argv[1] == "2" → P2 Phase 2 (24건 · A/B) · 없으면 Phase 1 (13건)
_PH2 = len(sys.argv) > 1 and sys.argv[1] == "2"
if _PH2:
    DIR = os.path.join(HERE, "P2_피로수명_Phase2")
    CONST = os.path.join(DIR, "p2b_constants.csv")
else:
    DIR = os.path.join(HERE, "P2_피로수명_Phase1")
    CONST = os.path.join(HERE, "P1_극한응력_Phase2", "p2_constants.csv")
PERDLC = os.path.join(DIR, "fatigue_per_dlc.csv")
SUMMARY = os.path.join(DIR, "fatigue_summary.csv")
SAVEDIR = os.path.join(DIR, "MASTA")
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg          # noqa: E402
import update_p2_table            # noqa: E402
import update_p2b_table           # noqa: E402

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
SAVE_TAGS = ({"A1", "A2", "A3", "B1", "B2", "B3"} if _PH2
             else {"1", "base"})
NBATCH = 20
DT0, DT = 0.1, 20.0
E_W = 9.0 / 8.0
DESIGN_YEARS = 30.0
MEM_LIMIT = 95.0
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")
FIELDS = ["design", "DLC", "k", "ksel", "nbin", "ScaleFactor",
          "eps_UW", "eps_Sys", "D30_UW", "D30_DW",
          "life_UW_yr", "life_DW_yr", "life_Sys_yr", "t_s"]


def load_raw(name):
    with open(os.path.join(DLCDIR, name, "raw.csv"), encoding="utf-8-sig") as f:
        return [{k: float(v) for k, v in r.items()} for r in csv.DictReader(f)]


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


def done_keys():
    if not os.path.isfile(PERDLC):
        return set()
    with open(PERDLC, encoding="utf-8-sig") as f:
        return {(r["design"], r["DLC"]) for r in csv.DictReader(f)}


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
    from mastapy.bearings import RollerBearingProfileTypes as RP

    os.makedirs(DIR, exist_ok=True)
    os.makedirs(SAVEDIR, exist_ok=True)
    with open(CONST, encoding="utf-8-sig") as f:
        specs = list(csv.DictReader(f))
    with open(os.path.join(DIR, "screen_k.csv"), encoding="utf-8-sig") as f:
        kmap = {}
        for r in csv.DictReader(f):
            kmap.setdefault(r["design"], {})[r["DLC"]] = r
    dk = done_keys()
    print(f"[P2 Step 2] 설계 {len(specs)}건 × DLC 111 · 기존 완료 {len(dk):,}건")

    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    sh = list(asm.all_parts_of_type_shaft())[0]
    bs = list(asm.all_parts_of_type_bearing())
    uw = [b for b in bs if "UW" in str(b)][0]
    dw = [b for b in bs if "DW" in str(b)][0]
    bmap = {"UW": uw, "DW": dw}
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    dp = asm.design_properties
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    ds = lc0.design_state_load_case_group
    for b in bs:
        b.detail.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG
    print(f"[모델] {os.path.basename(MODEL)}")

    new = not os.path.isfile(PERDLC)
    fh = open(PERDLC, "a", newline="", encoding="utf-8-sig")
    wr = csv.DictWriter(fh, fieldnames=FIELDS)
    if new:
        wr.writeheader()

    def analyze(reps, tag):
        dmg = {"UW": [], "DW": []}
        t_anal = 0.0
        for b0 in range(0, len(reps), NBATCH):
            chunk = reps[b0:b0 + NBATCH]
            lcs = []
            for cid, rev, rec in chunk:
                lc = lc0.duplicate(ds, f"p2_{tag}_{cid}")
                mf.set_loads(lc, pl, ipl, rec)
                lcs.append(lc)
            duty = dp.add_duty_cycle(f"p2dc_{tag}_{b0}")
            for lc in lcs:
                duty.add_static_load(lc)
            t0 = time.perf_counter()
            csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            csd.perform_analysis()
            t_anal += time.perf_counter() - t0
            for key, b in bmap.items():
                subs = list(list(csd.results_for(b))[0].component_analysis_cases)
                for (cid, rev, rec), sub in zip(chunk, subs):
                    l10 = (sub.component_detailed_analysis
                           .isots162812008.modified_reference_rating_life_cycles)
                    dmg[key].append(rev / l10 if (l10 and l10 > 0) else 0.0)
            for x in lcs + [duty]:
                try:
                    x.delete()
                except Exception:
                    pass
            if psutil.virtual_memory().percent > MEM_LIMIT:
                raise MemoryError("메모리 95% 초과")
        return dmg, t_anal

    t_all = time.time()
    for si, c in enumerate(specs, 1):
        tag = c["rank_mass"]
        z1, z2 = float(c["z1"]), float(c["z2"])
        g = sg.bearing(float(c["D_pw_mm"]) / 1e3, float(c["alpha"]),
                       float(c["D_we_mm"]) / 1e3, float(c["L_we_mm"]) / 1e3)
        # ── 모델 재구성 (P1 Phase 2 와 동일 순서) ──
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
        for b, z in ((uw, z1), (dw, z2)):
            b.try_mount_on(sh, z)
        chk = [k for k, v in (("bore", g["bore"] * 1e3),
                              ("width", g["width"] * 1e3),
                              ("number_of_elements", g["number_of_elements"]))
               if abs(float(getattr(uw.detail, k)) * (1e3 if k != "number_of_elements"
                                                      else 1) - v) > 0.01]
        t_d = time.time()
        print(f"\n[{si}/{len(specs)}] 설계 #{tag}  D_pw {float(c['D_pw_mm']):,.0f} "
              f"α {float(c['alpha']):.0f} D_we {float(c['D_we_mm']):.0f} "
              f"L_we {float(c['L_we_mm']):.0f} z {z1}/{z2}"
              + (f"  !! 주입불일치 {chk}" if chk else ""), flush=True)

        rows = []
        todo = [n for n in sorted(kmap[tag]) if (tag, n) not in dk]
        for i, name in enumerate(todo, 1):
            kr = kmap[tag][name]
            k, sf = float(kr["k"]), float(kr["ScaleFactor"])
            reps = bin_reps(load_raw(name), k)
            dmg, t_anal = analyze(reps, f"{tag}_{i}")
            d30U, d30D = sum(dmg["UW"]) * sf, sum(dmg["DW"]) * sf
            lU = DESIGN_YEARS / d30U if d30U > 0 else float("inf")
            lD = DESIGN_YEARS / d30D if d30D > 0 else float("inf")
            row = dict(design=tag, DLC=name, k=k, ksel=kr["ksel"],
                       nbin=len(reps), ScaleFactor=sf,
                       eps_UW=kr["eps_UW"], eps_Sys=kr["eps_Sys"],
                       D30_UW=d30U, D30_DW=d30D, life_UW_yr=lU, life_DW_yr=lD,
                       life_Sys_yr=lsys(lU, lD), t_s=round(t_anal, 3))
            wr.writerow(row); fh.flush()
            rows.append(row)
            if i % 30 == 0 or i == len(todo):
                print(f"    [{i}/{len(todo)}] {name} k={k:g} {len(reps)}빈 "
                      f"D30_UW={d30U:.4f}  ({time.time()-t_d:.0f}s)", flush=True)

        # ── 설계 집계 ──
        with open(PERDLC, encoding="utf-8-sig") as f:
            allr = [r for r in csv.DictReader(f) if r["design"] == tag]
        sU = sum(float(r["D30_UW"]) for r in allr)
        sD = sum(float(r["D30_DW"]) for r in allr)
        LU = DESIGN_YEARS / sU if sU > 0 else float("inf")
        LD = DESIGN_YEARS / sD if sD > 0 else float("inf")
        LS = lsys(LU, LD)
        sS = DESIGN_YEARS / LS
        srow = dict(design=tag, n_dlc=len(allr), D30_UW=sU, D30_DW=sD,
                    D30_Sys=sS, life_UW_yr=LU, life_DW_yr=LD, life_Sys_yr=LS,
                    pass_UW=int(sU <= 0.5), pass_Sys=int(sS <= 0.5),
                    t_s=round(sum(float(r["t_s"]) for r in allr), 1))
        exist = []
        if os.path.isfile(SUMMARY):
            with open(SUMMARY, encoding="utf-8-sig") as f:
                exist = [r for r in csv.DictReader(f) if r["design"] != tag]
        with open(SUMMARY, "w", newline="", encoding="utf-8-sig") as f:
            w = csv.DictWriter(f, fieldnames=list(srow))
            w.writeheader(); w.writerows(exist + [srow])
        print(f"  → ΣD30_UW {sU:.4f} · ΣD30_DW {sD:.4f} · Sys {sS:.4f} "
              f"· life_Sys {LS:,.2f} yr · "
              f"{'합격' if (sU <= 0.5 and sS <= 0.5) else '불합격'} "
              f"({(time.time()-t_d)/60:.1f}분)", flush=True)

        if tag in SAVE_TAGS:
            p = os.path.join(SAVEDIR, f"P2_design_{tag}.masta")
            try:
                st = design.save(p, False)      # (file_name, save_results)
                print(f"  [저장] {os.path.basename(p)}  {st}")
            except Exception as e:
                print(f"  !! MASTA 저장 실패: {str(e).splitlines()[0][:70]}")
        upd = update_p2b_table if _PH2 else update_p2_table
        d, t = upd.main()
        print(f"  [문서] §8-3.6 갱신 {d}/{t}", flush=True)

    fh.close()
    print(f"\n[완료] {(time.time()-t_all)/60:.1f}분")


if __name__ == "__main__":
    main()
