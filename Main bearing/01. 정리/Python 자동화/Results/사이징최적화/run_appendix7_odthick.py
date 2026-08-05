"""
부록 7-6.7.3 — OD별 한계 두께 (40건)
=======================================
설계마다 **피로 안전율 5.0 을 만족하는 최대 내경 비율 `r*`** 을 찾고, 그때의
두께 `t*` 가 외경 `OD` 와 어떤 관계인지 본다.

  · 하중은 설계마다 **자기 `k`** 로 `DLC1.2-k-s5`(§7-6.5 최악 DLC)를 30빈으로
    변환해 쓴다. 먼저 30빈을 다 돌려 그 설계의 **최악 빈**을 찾고, 이후 비율
    탐색은 그 빈 하나로 한다(§7-6.3 · 값이 같고 30배 빠르다)
  · `r*` 은 이분탐색 12회로 0.0005 까지 좁힌다
  · 마지막에 `r*` 에서 30빈을 다시 돌려 **최악 빈이 바뀌지 않았는지** 확인한다

예상 형태는 `t* ∝ 1/OD²` 다 — 얇은 링의 단면계수가 `≈ π·OD²·t/4` 이고 하중이
같다면 `OD²·t` 가 일정해야 한다. 실제로 그런지, 아니면 스팬(`z2`)이 더 지배적
인지를 회귀로 가른다.

산출: 부록7_샤프트/od_thickness.csv + 문서 §7-6.7.3 표·회귀
"""
import csv
import io
import math
import os
import re
import sys
import time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg          # noqa: E402
import run_p2_fatigue as p2       # noqa: E402
import run_appendix7_shaft as a7  # noqa: E402

DLC, TARGET = "DLC1.2-k-s5", 5.0
R_LO, R_HI, NBI = 0.80, 0.985, 12      # 이분탐색 범위·횟수
OUT = os.path.join(HERE, "부록7_샤프트", "od_thickness.csv")
DOC = a7.DOC
MARK_T, MARK_F = "<!-- A7:ODTHICK -->", "<!-- A7:ODFIT -->"


def swap(doc, mark, blk):
    close = mark.replace("<!-- ", "<!-- /")
    pat = re.compile(re.escape(mark) + r"\n.*?\n" + re.escape(close), re.S)
    if not pat.search(doc):
        raise RuntimeError(f"{mark} … {close} 자리표를 찾지 못했다")
    return pat.sub(lambda m: f"{mark}\n{blk}\n{close}", doc, count=1)


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import (
        AnalysisType)
    from mastapy.bearings import RollerBearingProfileTypes as RP

    D = os.path.join(HERE, "P2_피로수명_S4")
    with open(os.path.join(D, "p2d_targets.csv"), encoding="utf-8-sig") as f:
        specs = list(csv.DictReader(f))
    with open(os.path.join(D, "screen_k.csv"), encoding="utf-8-sig") as f:
        kmap = {r["design"]: float(r["k"]) for r in csv.DictReader(f)
                if r["DLC"] == DLC}

    d = Design.load(a7.MODEL)
    asm = d.all_parts_of_type_root_assembly()[0]
    sh = list(asm.all_parts_of_type_shaft())[0]
    bs = list(asm.all_parts_of_type_bearing())
    uw = [b for b in bs if "UW" in str(b)][0]
    dw = [b for b in bs if "DW" in str(b)][0]
    for b in bs:
        b.detail.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG
    dp = asm.design_properties
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    ds = lc0.design_state_load_case_group

    rows, t0 = [], time.perf_counter()
    for si, spec in enumerate(specs, 1):
        tag = spec["rank_mass"]
        g, s0 = a7.geom_of(spec)
        z1, z2 = float(spec["z1"]), float(spec["z2"])
        OD, L = s0["outer_diameter"], s0["length"]
        reps = p2.bin_reps(p2.load_raw(DLC), kmap[tag])

        def build(r):
            for b in bs:
                try:
                    if b.inner_connection is not None:
                        b.inner_connection.delete()
                except Exception:
                    pass
            idm = math.floor(OD * r * 1000) / 1000
            sh.remove_all_sections()
            sh.add_section(0.0, L, OD, idm, OD, idm)
            for b in bs:
                sg.apply_to_masta(b.detail, g)
            for b, z in ((uw, z1), (dw, z2)):
                b.try_mount_on(sh, z)
            return idm

        def run(rp, tg):
            lcs = []
            for cid, rev, rec in rp:
                lc = lc0.duplicate(ds, f"od_{tg}_{cid}")
                mf.set_loads(lc, pl, ipl, rec)
                lcs.append(lc)
            duty = dp.add_duty_cycle(f"odd_{tg}")
            for lc in lcs:
                duty.add_static_load(lc)
            csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            csd.perform_analysis()
            comp = list(csd.results_for(sh))[0]
            each = [{("fat" if "Infinite" in i.description else "sta"):
                     float(i.safety_factor) for i in x.safety_factors.items}
                    for x in comp.component_analysis_cases]
            ms = float(sh.mass_of_shaft_body)
            for x in lcs + [duty]:
                try:
                    x.delete()
                except Exception:
                    pass
            return each, ms

        build(sg.ID_OVER_OD)                      # 현행 비율에서 최악 빈 탐색
        each, ms0 = run(reps, f"{si}b")
        wi = min(range(len(each)), key=lambda i: each[i]["fat"])
        one, sf0 = [reps[wi]], each[wi]["fat"]

        lo, hi = R_LO, R_HI                       # 이분탐색 (r↑ → SF↓)
        for _ in range(NBI):
            mid = (lo + hi) / 2
            build(mid)
            e, _ = run(one, f"{si}p")
            if e[0]["fat"] >= TARGET:
                lo = mid
            else:
                hi = mid
        rstar = math.floor(lo * 1000) / 1000      # 0.001 단위로 내림(안전측)
        idm = build(rstar)
        chk, ms = run(reps, f"{si}c")             # r* 에서 30빈 재확인
        wj = min(range(len(chk)), key=lambda i: chk[i]["fat"])
        t = (OD - idm) / 2 * 1e3

        rows.append(dict(tag=tag, OD_mm=OD * 1e3, z1=z1, z2=z2,
                         L_mm=L * 1e3, k=kmap[tag], worst_bin=reps[wi][0],
                         sf_now=sf0, mass_now_t=ms0 / 1e3,
                         r_star=rstar, ID_star_mm=idm * 1e3, t_star_mm=t,
                         OD_over_t=OD * 1e3 / t, sf_star=chk[wj]["fat"],
                         static_star=chk[wj]["sta"], mass_star_t=ms / 1e3,
                         bin_shift=int(reps[wj][0] != reps[wi][0])))
        with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
            w = csv.DictWriter(f, fieldnames=list(rows[0]))
            w.writeheader()
            w.writerows(rows)
        print(f"  [{si:2}/{len(specs)}] {tag} OD {OD*1e3:,.0f} · "
              f"현행 SF {sf0:6.3f} → r* {rstar:.3f} · t* {t:6.1f} · "
              f"SF {chk[wj]['fat']:.3f} · 질량 {ms0/1e3:6.1f}→{ms/1e3:6.1f} t"
              + ("  !빈이동" if reps[wj][0] != reps[wi][0] else "")
              + f"  ({time.perf_counter()-t0:.0f}s)", flush=True)

    # ── 회귀 ──────────────────────────────────────────────────
    od = np.array([r["OD_mm"] for r in rows])
    ts = np.array([r["t_star_mm"] for r in rows])
    z2 = np.array([r["z2"] for r in rows])
    lt = np.log(ts)

    def fit(X, names):
        A = np.column_stack([np.ones(len(od))] + X)
        c, *_ = np.linalg.lstsq(A, lt, rcond=None)
        pred = A @ c
        r2 = 1 - ((lt - pred) ** 2).sum() / ((lt - lt.mean()) ** 2).sum()
        return c, r2, names

    f1 = fit([np.log(od)], ["ln OD"])
    f2 = fit([np.log(od), np.log(z2)], ["ln OD", "ln z2"])
    body = [
        "| 모형 | 식 | R² |", "|---|---|--:|",
        f"| OD 단독 | `t* = {math.exp(f1[0][0]):.3g} · OD^{f1[0][1]:+.2f}` | "
        f"{f1[1]:.3f} |",
        f"| OD + 스팬 | `t* = {math.exp(f2[0][0]):.3g} · OD^{f2[0][1]:+.2f} · "
        f"z2^{f2[0][2]:+.2f}` | {f2[1]:.3f} |",
        f"| *물리 예상* | `t* ∝ OD^−2` (단면계수 `≈ π·OD²·t/4`) | — |"]
    doc = io.open(DOC, encoding="utf-8").read()
    doc = swap(doc, MARK_F, "\n".join(body))

    tb = ["| 태그 | OD [mm] | z2 [m] | 현행 SF | **`r*`** | 내경* [mm] | "
          "**`t*` [mm]** | `OD/t*` | SF* | 질량 [t] |",
          "|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|"]
    for r in sorted(rows, key=lambda x: -x["OD_mm"]):
        tb.append(
            f"| `{r['tag']}` | {r['OD_mm']:,.0f} | {r['z2']:.1f} | "
            f"{r['sf_now']:.2f} | **{r['r_star']:.3f}** | "
            f"{r['ID_star_mm']:,.0f} | **{r['t_star_mm']:.1f}** | "
            f"{r['OD_over_t']:.1f} | {r['sf_star']:.2f} | "
            f"{r['mass_now_t']:.1f} → {r['mass_star_t']:.1f} |")
    doc = swap(doc, MARK_T, "\n".join(tb))
    io.open(DOC, "w", encoding="utf-8").write(doc)

    print(f"\n[완료] {len(rows)}건 · {time.perf_counter()-t0:.0f}s")
    print(f"  t* {ts.min():.1f} ~ {ts.max():.1f} mm · "
          f"r* {min(r['r_star'] for r in rows):.3f} ~ "
          f"{max(r['r_star'] for r in rows):.3f}")
    print(f"  OD 단독 지수 {f1[0][1]:+.2f} (R² {f1[1]:.3f}) · "
          f"OD+z2 {f2[0][1]:+.2f}/{f2[0][2]:+.2f} (R² {f2[1]:.3f})")
    print(f"  빈 이동 {sum(r['bin_shift'] for r in rows)}건")


if __name__ == "__main__":
    main()
