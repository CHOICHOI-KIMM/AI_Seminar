"""
부록 7-6.5 — 111 DLC 전수 샤프트 안전율 (`a01`)
================================================================
`a01` 의 **111 DLC 전부**를 `dt=20` · μ+kσ 대표하중으로 바꿔 듀티사이클로
돌리고, 샤프트 DIN 743 두 안전율을 뽑는다. 표에는 계열별 요약과 최악
10건을 싣는다 — 손상 순위로 표본을 고르면 최악을 24% 놓친다(§7-6.5 ②).

  · 대표하중 변환은 피로 해석과 **같은 `bin_reps()`** 를 쓰고 `k` 는
    `screen_k.csv` 값을 그대로 쓴다 — 새로 만들지 않는다
  · 듀티사이클 집계는 **최악 LC 기준**임이 §7-6.3 에서 확인됐으므로
    표에는 그 최악값을 싣는다
  · 비교 기준은 극한 LC `Myz_max` 의 **4.253**(§7-6.1)

**CSV 는 DLC 하나가 끝날 때마다** 갱신하고(중단 대비) **문서는 완료 후 한 번**
갱신한다.

산출: 부록7_샤프트/all111_dlc.csv + 문서 §7-6.5 표
"""
import csv
import io
import os
import re
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg          # noqa: E402
import run_p2_fatigue as p2       # noqa: E402
import run_appendix7_shaft as a7  # noqa: E402

TAG = "a01"
EXTREME_SF = 4.253                # §7-6.1 Myz_max 피로 안전율
# 인자 `all` → 111 DLC 전수(§7-6.6). 없으면 손상 상위 10건(§7-6.5).
ALL = len(sys.argv) > 1 and sys.argv[1].lower().startswith("all")
NTOP = 111 if ALL else 10
OUT = os.path.join(HERE, "부록7_샤프트",
                   "all111_dlc.csv" if ALL else "top10_dlc.csv")
DOC = a7.DOC
MARK = "<!-- A7:ALL111 -->" if ALL else "<!-- A7:TOP10 -->"
HDR = ("| # | DLC | 베어링 ΣD30_UW | 비중 | `k` | 빈 | "
       "**무한수명 피로** | 영구변형 | 극한 LC 대비 |")
SEP = "|--:|---|--:|--:|--:|--:|--:|--:|--:|"


def targets():
    d = os.path.join(HERE, "P2_피로수명_S4")
    with open(os.path.join(d, "fatigue_per_dlc.csv"), encoding="utf-8-sig") as f:
        R = [r for r in csv.DictReader(f) if r["design"] == TAG]
    tot = sum(float(r["D30_UW"]) for r in R)
    R.sort(key=lambda r: -float(r["D30_UW"]))
    with open(os.path.join(d, "p2d_targets.csv"), encoding="utf-8-sig") as f:
        spec = next(x for x in csv.DictReader(f) if x["rank_mass"] == TAG)
    return R[:NTOP], tot, spec


def line(i, r, tot):
    return (f"| {i} | `{r['DLC']}` | {r['D30_UW']:.5f} | "
            f"{100*r['D30_UW']/tot:.1f}% | {r['k']:g} | {r['nbin']} | "
            f"**{r['fatigue_inf']:.3f}** | {r['static']:.3f} | "
            f"{r['fatigue_inf']/EXTREME_SF:.2f}배 |")


def write_doc(rows, tot):
    if not ALL:
        body = [HDR, SEP] + [line(i, r, tot) for i, r in enumerate(rows, 1)]
        if len(rows) < NTOP:
            body.append(f"| … | *(수행 중 {len(rows)}/{NTOP})* |" + " |" * 7)
    else:
        # 111행은 문서에 싣지 않는다 — 계열별 요약 + 최악 10건
        fam = {}
        for r in rows:
            fam.setdefault(r["DLC"].split("-")[0], []).append(r)
        body = ["**계열별 요약** (111 DLC · 2,646빈)", "",
                "| 계열 | DLC 수 | 빈 | 베어링 손상 비중 | "
                "**피로 안전율** | 영구변형 | 극한 LC 대비 |",
                "|---|--:|--:|--:|---|---|---|"]
        for k in sorted(fam):
            g = fam[k]
            f = [x["fatigue_inf"] for x in g]
            st = [x["static"] for x in g]
            body.append(
                f"| `{k}` | {len(g)} | {sum(x['nbin'] for x in g):,} | "
                f"{100*sum(x['D30_UW'] for x in g)/tot:.1f}% | "
                f"**{min(f):.2f} ~ {max(f):.2f}** | "
                f"{min(st):.1f} ~ {max(st):.1f} | "
                f"{min(f)/EXTREME_SF:.2f} ~ {max(f)/EXTREME_SF:.2f}배 |")
        worst = sorted(rows, key=lambda r: r["fatigue_inf"])[:10]
        body += ["", "**샤프트 안전율 최악 10건**", "", HDR, SEP]
        body += [line(i, r, tot) for i, r in enumerate(worst, 1)]
        body += ["", f"*전 111건은 `부록7_샤프트/all111_dlc.csv`.*"]
    blk = "\n".join(body)
    close = MARK.replace("<!-- ", "<!-- /")
    s = io.open(DOC, encoding="utf-8").read()
    pat = re.compile(re.escape(MARK) + r"\n.*?\n" + re.escape(close), re.S)
    if not pat.search(s):
        raise RuntimeError(f"{MARK} … {close} 자리표를 찾지 못했다")
    io.open(DOC, "w", encoding="utf-8").write(
        pat.sub(lambda m: f"{MARK}\n{blk}\n{close}", s, count=1))


def main():
    top, tot, spec = targets()
    print(f"[{TAG}] 베어링 손상 상위 {len(top)} DLC · 총 ΣD30_UW {tot:.4f}")

    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import (
        AnalysisType)
    from mastapy.bearings import RollerBearingProfileTypes as RP

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

    g, s = a7.geom_of(spec)            # CSV 기록 제원 (정수화 포함)
    z1, z2 = float(spec["z1"]), float(spec["z2"])
    for b in bs:
        try:
            if b.inner_connection is not None:
                b.inner_connection.delete()
        except Exception:
            pass
    sh.remove_all_sections()
    sh.add_section(0.0, s["length"], s["outer_diameter"], s["inner_diameter"],
                   s["outer_diameter"], s["inner_diameter"])
    for b in bs:
        sg.apply_to_masta(b.detail, g)
    for b, z in ((uw, z1), (dw, z2)):
        b.try_mount_on(sh, z)
    print(f"[설계] bore {g['bore']*1e3:,.0f} · 샤프트 OD "
          f"{s['outer_diameter']*1e3:,.0f} / ID {s['inner_diameter']*1e3:,.0f}")

    rows, t0 = [], time.perf_counter()
    for i, r in enumerate(top, 1):
        name, k = r["DLC"], float(r["k"])
        reps = p2.bin_reps(p2.load_raw(name), k)
        lcs = []
        for cid, rev, rec in reps:
            lc = lc0.duplicate(ds, f"t10_{i}_{cid}")
            mf.set_loads(lc, pl, ipl, rec)
            lcs.append(lc)
        duty = dp.add_duty_cycle(f"t10_{i}")
        for lc in lcs:
            duty.add_static_load(lc)
        csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
        csd.perform_analysis()
        comp = list(csd.results_for(sh))[0]
        v = {("fatigue_inf" if "Infinite" in it.description else "static"):
             float(it.safety_factor) for it in comp.safety_factors.items}
        for x in lcs + [duty]:
            try:
                x.delete()
            except Exception:
                pass
        rows.append(dict(DLC=name, D30_UW=float(r["D30_UW"]), k=k,
                         nbin=len(reps), **v))
        # ── CSV 는 매 건, 문서는 상위10 모드에서만 실시간 갱신 ──
        with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
            w = csv.DictWriter(f, fieldnames=list(rows[0]))
            w.writeheader()
            w.writerows(rows)
        if not ALL:
            write_doc(rows, tot)
        print(f"  [{i}/{len(top)}] {name:20} 빈 {len(reps):3} · "
              f"피로 {v['fatigue_inf']:7.3f} · 정적 {v['static']:7.3f} "
              f"· 극한 대비 {v['fatigue_inf']/EXTREME_SF:.2f}배 "
              f"({time.perf_counter()-t0:.0f}s)", flush=True)

    if ALL:
        write_doc(rows, tot)          # 전수는 완료 후 한 번만 갱신
    f = [r["fatigue_inf"] for r in rows]
    w = min(rows, key=lambda r: r["fatigue_inf"])
    print(f"\n[완료] {len(rows)}건 · {time.perf_counter()-t0:.0f}s · "
          f"피로 {min(f):.3f} ~ {max(f):.3f} · "
          f"극한 LC({EXTREME_SF}) 대비 {min(f)/EXTREME_SF:.2f} ~ "
          f"{max(f)/EXTREME_SF:.2f}배")
    print(f"  최악 {w['DLC']} 피로 {w['fatigue_inf']:.3f} · "
          f"정적 {w['static']:.3f}")


if __name__ == "__main__":
    main()
