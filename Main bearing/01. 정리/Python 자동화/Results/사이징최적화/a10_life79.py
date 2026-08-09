# -*- coding: utf-8 -*-
"""§10-12.7.9 — 피로수명 검토 (오프셋 · 오프셋+비대칭 DIN · 70 °C)

  조건  ① 오프셋만            e_off = rule_v2 · 양쪽 DIN
        ② 오프셋 + 비대칭 DIN  + UW 에만 비대칭 DIN (확장 최적)
  대상  #1 · #103 · #210 · 111 DLC · 베어링 70 °C (§6-11.7 조건)
  지표  ΣD30 (UW · DW) → 수명 [년] · 손상 상위 15 DLC

부록 1 의 `run_fatigue` 는 111 DLC 를 통째로 돌고 끝에 반환한다. 실시간
갱신을 위해 **20 DLC 마다 콜백**하는 형태로 다시 썼다.
"""
import csv
import io
import json
import os
import re
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import a10_profile_lib as L      # noqa: E402
import a10_asymdin2 as A2        # noqa: E402
import a10_eoff_v2 as EV2        # noqa: E402
import run_appendix1_profile as A1   # noqa: E402

DOC = A2.DOC
OUT = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "life79")
RANKS = (1, 103, 210)
CASES = (("base", "현행"), ("off", "오프셋만"),
         ("adin", "오프셋 + 비대칭 DIN"))
TICK = 20
TOPN = 15
EXP = 9.0 / 8.0


def log(m):
    print(m, flush=True)


def set_70c(asm):
    lc0 = next(c for c in asm.design_properties.static_loads
               if c.name == "Load Case 1")
    t = lc0.temperatures
    for a in ("rolling_bearing_element", "rolling_bearing_inner_race",
              "rolling_bearing_outer_race"):
        setattr(t, a, 70.0)
    log(f"[온도] 베어링 {t.rolling_bearing_element:.0f} °C · 샤프트 "
        f"{t.shaft:.0f} °C · 하우징 {t.housing:.0f} °C")
    return lc0


def set_smoothed(brg):
    """피로 전용 — 프로파일 평가를 SMOOTHED 로 전환 (§10-12.7.9 ⑸)."""
    from mastapy.bearings.roller_bearing_profiles import ProfileDataToUse
    up = brg.detail.roller_profile_set.active_profile
    up.data_to_use = ProfileDataToUse.SMOOTHED
    log(f"[프로파일] data_to_use = {up.data_to_use} (피로 전용 · 형상 동일)")


def load_k70():
    p = os.path.join(HERE, "P2_피로수명_S4_70C", "screen_k.csv")
    out = {}
    for r in csv.DictReader(io.open(p, encoding="utf-8-sig")):
        if r.get("design") not in (None, "", "a01"):
            continue
        out[r["DLC"]] = (float(r["k"]), r["ksel"])
    return out


def fatigue(asm, bearings, kmap, meta, tag, lc0, cb=None):
    """부록 1 의 루프 + 20 DLC 마다 콜백. per-DLC 손상을 남긴다."""
    import masta_fatigue as mf
    import psutil
    from mastapy.system_model.analyses_and_results.static_loads import (
        AnalysisType)
    dp = asm.design_properties
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    bmap = {("UW" if "UW" in str(b) else "DW"): b for b in bearings}
    ds = lc0.design_state_load_case_group
    targets = [n for n in sorted(meta) if n in kmap]
    rows, t0 = {}, time.time()
    for i, name in enumerate(targets, 1):
        sf = float(meta[name]["ScaleFactor"])
        k, ktag = kmap[name]
        reps = A1.bin_reps(A1.load_raw(name), k)
        dmg = {"UW": [], "DW": []}
        short = name[-6:].replace(".", "")
        for b0 in range(0, len(reps), A1.NBATCH):
            chunk = reps[b0:b0 + A1.NBATCH]
            lcs = []
            for cid, rev, rec in chunk:
                lc = lc0.duplicate(ds, f"L79{tag}_{short}_{cid}")
                mf.set_loads(lc, pl, ipl, rec)
                lcs.append(lc)
            duty = dp.add_duty_cycle(f"L79{tag}d_{short}_{b0}")
            for lc in lcs:
                duty.add_static_load(lc)
            csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            csd.perform_analysis()
            for key, b in bmap.items():
                subs = list(list(csd.results_for(b))[0]
                            .component_analysis_cases)
                for (cid, rev, rec), sub in zip(chunk, subs):
                    l10 = (sub.component_detailed_analysis
                           .isots162812008
                           .modified_reference_rating_life_cycles)
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
            if psutil.virtual_memory().percent > A1.MEM_LIMIT:
                raise MemoryError("메모리 95% 초과")
        rows[name] = dict(D30_UW=sum(dmg["UW"]) * sf,
                          D30_DW=sum(dmg["DW"]) * sf, k=k, sf=sf)
        if i % TICK == 0 or i == len(targets):
            log(f"    [{tag}] {i}/{len(targets)}  ({time.time()-t0:.0f}s)")
            if cb:
                cb(rows, i, len(targets))
    return rows


def summarize(rows):
    sU = sum(v["D30_UW"] for v in rows.values())
    sD = sum(v["D30_DW"] for v in rows.values())
    lU = 30.0 / sU if sU > 0 else float("inf")
    lD = 30.0 / sD if sD > 0 else float("inf")
    lS = (lU ** -EXP + lD ** -EXP) ** (-1.0 / EXP)
    return sU, sD, lU, lD, lS


def fmt(v, n=2):
    return "—" if v is None else f"{v:,.{n}f}"


def write_doc(state, t0):
    B = []
    A = B.append
    A(f"*실행 중 — 설계 완료마다 · 20 DLC 마다 갱신. 경과 "
      f"{(time.perf_counter()-t0)/60:.1f}분.*")
    A("")
    A("**진행 · 수명**")
    A("")
    A("| 설계 | 조건 | DLC | ΣD30 UW | ΣD30 DW | 수명 UW | 수명 DW | "
      "**수명 Sys** |")
    A("|---|---|--:|--:|--:|--:|--:|--:|")
    for rk in RANKS:
        for cid, lab in CASES:
            st = state.get((rk, cid))
            if not st:
                A(f"| `#{rk}` | {lab} | — | — | — | — | — | — |")
                continue
            sU, sD, lU, lD, lS = summarize(st["rows"])
            done = "**" if st["done"] else ""
            A(f"| `#{rk}` | {lab} | {st['i']}/{st['n']} | {fmt(sU, 4)} | "
              f"{fmt(sD, 4)} | {fmt(lU, 1)} | {fmt(lD, 1)} | "
              f"{done}{fmt(lS, 1)}{done} |")
    A("")
    for rk in RANKS:
        b = state.get((rk, "adin"))
        if not b or not b["rows"]:
            continue
        a = (state.get((rk, "base")) or state.get((rk, "off"))
             or dict(rows={}, done=False))
        cmp_ = bool(a["rows"])
        A(f"**`#{rk}` 손상 상위 {TOPN} DLC** — UW 손상 기준 정렬"
          + ("" if cmp_ else " (오프셋 + 비대칭 DIN)"))
        A("")
        if cmp_:
            A("| # | DLC | 기준 UW | + 비대칭 UW | 변화 | 기준 DW | "
              "+ 비대칭 DW | 누적 UW 비율 |")
            A("|--:|---|--:|--:|---|--:|--:|--:|")
        else:
            A("| # | DLC | UW 손상 | DW 손상 | 누적 UW 비율 |")
            A("|--:|---|--:|--:|--:|")
        tot = sum(v["D30_UW"] for v in b["rows"].values())
        top = sorted(b["rows"].items(), key=lambda kv: -kv[1]["D30_UW"])
        acc = 0.0
        for j, (nm, v) in enumerate(top[:TOPN], 1):
            o = a["rows"].get(nm, {})
            acc += v["D30_UW"]
            d = (v["D30_UW"] - o.get("D30_UW", 0))
            sign = "▲" if d > 0 else "▼"
            if cmp_:
                A(f"| {j} | `{nm}` | {fmt(o.get('D30_UW'), 5)} | "
                  f"{fmt(v['D30_UW'], 5)} | {sign} {fmt(abs(d), 5)} | "
                  f"{fmt(o.get('D30_DW'), 5)} | {fmt(v['D30_DW'], 5)} | "
                  f"{100*acc/tot:.1f} % |")
            else:
                A(f"| {j} | `{nm}` | {fmt(v['D30_UW'], 5)} | "
                  f"{fmt(v['D30_DW'], 5)} | {100*acc/tot:.1f} % |")
        A("")
    blk = "\n".join(B)
    try:
        s = io.open(DOC, encoding="utf-8").read()
        pat = re.compile(r"(<!-- A10:LIFE79 -->\n).*?(<!-- /A10:LIFE79 -->)",
                         re.S)
        if not pat.search(s):
            return
        out = pat.sub(lambda m: m.group(1) + blk + "\n" + m.group(2), s,
                      count=1)
        io.open(DOC, "w", encoding="utf-8").write(out)
    except Exception as e:
        log(f"    [문서 갱신 실패] {str(e).splitlines()[0][:60]}")


def main():
    global RANKS, CASES
    os.makedirs(OUT, exist_ok=True)
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    b3 = json.load(open(os.path.join(A2.OUT, "best3.json"), encoding="utf-8"))
    rig = L.Rig()
    lc0 = set_70c(rig.asm)
    meta = A1.load_meta()
    kmap = load_k70()
    log(f"[대상] 설계 {list(RANKS)} · 조건 {[c[0] for c in CASES]} · DLC {len(kmap)}건")
    state, t0 = {}, time.perf_counter()
    for rk in RANKS:
        p = b3[str(rk)]["p"]
        for cid, lab in CASES:
            rig.build(P[rk])
            d = rig.uw.detail
            e = float(d.element_offset) * 1e3        # 현행 20.742
            if cid != "base":                        # 현행은 오프셋도 그대로
                e = EV2.rule_v2(d)[0]
                for b in rig.bs:
                    b.detail.element_offset = e / 1e3
            rig.set_din(0.0)
            if cid == "adin":
                rig.set_user(A2.asym_din2(d.effective_roller_length,
                                          d.element_diameter, *p),
                             A2.NPTS, targets=[rig.uw])
                # §10-12.7.9 ⑸ — 피로는 SMOOTHED. 형상은 201점 전부 동일한데
                # (최대차 0.000000 um) 평가가 8.6배 빠르다. 응력·형상 판정은
                # ACTUAL_DATA 를 그대로 쓴다.
                set_smoothed(rig.uw)
            log(f"\n=== #{rk} · {lab} · e_off {e:.2f} "
                + (f"· k {p[0]:g}/{p[1]:g} δ {p[2]:g}" if cid == "adin"
                   else "") + " ===")
            key = (rk, cid)
            state[key] = dict(rows={}, i=0, n=len(kmap), done=False)

            def cb(rows, i, n, _k=key):
                state[_k].update(rows=dict(rows), i=i, n=n)
                write_doc(state, t0)

            t1 = time.perf_counter()
            rows = fatigue(rig.asm, rig.bs, kmap, meta, f"{rk}{cid}", lc0, cb)
            state[key].update(rows=rows, i=len(rows), done=True)
            sU, sD, lU, lD, lS = summarize(rows)
            log(f"  ΣD30 UW {sU:.4f} · DW {sD:.4f} → 수명 UW {lU:.1f} · "
                f"DW {lD:.1f} · Sys **{lS:.1f}년** "
                f"({(time.perf_counter()-t1)/60:.1f}분)")
            write_doc(state, t0)
            with io.open(os.path.join(OUT, "life79_perdlc.csv"), "w",
                         newline="", encoding="utf-8-sig") as f:
                w = csv.writer(f)
                w.writerow(["rank", "case", "DLC", "D30_UW", "D30_DW", "k"])
                for (r2, c2), st in state.items():
                    for nm, v in st["rows"].items():
                        w.writerow([r2, c2, nm, v["D30_UW"], v["D30_DW"],
                                    v["k"]])
    log(f"\n[완료] {(time.perf_counter()-t0)/60:.1f}분 · {OUT}")


if __name__ == "__main__":
    av = sys.argv[1:]
    rk = [int(x) for x in av if x.isdigit()]
    cs = [x for x in av if x in ("base", "off", "adin")]
    if rk:
        RANKS = tuple(rk)
    if cs:
        CASES = tuple(c for c in CASES if c[0] in cs)
    main()
