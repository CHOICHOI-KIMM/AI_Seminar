"""
부록 7-6.7 — 샤프트 두께 비율 검토 (`a01`)
=============================================
두 가지를 낸다.

  ① `DLC1.2-k-s5`(§7-6.5 최악 DLC)에서 **최저 피로안전율이 나는 빈**과 그
     빈의 6자유도 대표하중
  ② 샤프트 내경 비율 `ID = floor(r · OD)` 의 `r` 을 키우며(= 벽을 얇게 하며)
     **피로 안전율 5.0 이상**을 만족하는 한계 비율·두께

스윙 하중은 ①에서 찾은 **최악 빈 1개**만 쓴다. 듀티사이클 집계가 최악 LC
기준이므로(§7-6.3) DLC 전체를 돌린 것과 값이 같고 30배 빠르다.

`r` 은 0.885 ~ 0.97 을 0.005 로 훑은 뒤, 5.0 교차 구간만 0.001 로 좁힌다.
질량과 정적 안전율을 함께 실어 **얇게 해서 얻는 이득**이 보이게 한다.

산출: 부록7_샤프트/thickness.csv · worst_bin.csv + 문서 §7-6.7 표
"""
import csv
import io
import math
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

TAG, DLC = "a01", "DLC1.2-k-s5"
TARGET = 5.0                      # 요구 피로 안전율
R0 = sg.ID_OVER_OD                # 0.88543 (현행)
COARSE = [round(0.885 + 0.005 * i, 4) for i in range(18)]   # ~0.970
DOC = a7.DOC
OUTDIR = os.path.join(HERE, "부록7_샤프트")
MARK_BIN, MARK_SWEEP = "<!-- A7:WORSTBIN -->", "<!-- A7:THICK -->"
KSIG = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")


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

    with open(os.path.join(HERE, "P2_피로수명_S4", "p2d_targets.csv"),
              encoding="utf-8-sig") as f:
        spec = next(x for x in csv.DictReader(f) if x["rank_mass"] == TAG)
    with open(os.path.join(HERE, "P2_피로수명_S4", "screen_k.csv"),
              encoding="utf-8-sig") as f:
        k = float(next(x for x in csv.DictReader(f)
                       if x["design"] == TAG and x["DLC"] == DLC)["k"])

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

    g, s0 = a7.geom_of(spec)
    z1, z2 = float(spec["z1"]), float(spec["z2"])
    OD = s0["outer_diameter"]
    for b in bs:
        try:
            if b.inner_connection is not None:
                b.inner_connection.delete()
        except Exception:
            pass

    def build(ratio):
        """내경 비율을 바꿔 모델 재구성 — 베어링 제원은 그대로다.

        **재장착 전에 기존 연결을 반드시 지운다.** 안 지우면 두 번째 호출부터
        `ConnectionException: Attempt to connect incompatible sockets` 가 난다.
        """
        for b in bs:
            try:
                if b.inner_connection is not None:
                    b.inner_connection.delete()
            except Exception:
                pass
        idm = math.floor(OD * ratio * 1000) / 1000
        sh.remove_all_sections()
        sh.add_section(0.0, s0["length"], OD, idm, OD, idm)
        for b in bs:
            sg.apply_to_masta(b.detail, g)
        for b, z in ((uw, z1), (dw, z2)):
            b.try_mount_on(sh, z)
        return idm

    def run(reps, tag):
        lcs = []
        for cid, rev, rec in reps:
            lc = lc0.duplicate(ds, f"th_{tag}_{cid}")
            mf.set_loads(lc, pl, ipl, rec)
            lcs.append(lc)
        duty = dp.add_duty_cycle(f"thd_{tag}")
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

    # ── ① 최악 빈 찾기 (현행 비율) ─────────────────────────────
    build(R0)
    reps = p2.bin_reps(p2.load_raw(DLC), k)
    each, _ = run(reps, "base")
    wi = min(range(len(each)), key=lambda i: each[i]["fat"])
    wbin = reps[wi]
    print(f"[①] {DLC} · {len(reps)}빈 · k={k:g}")
    print(f"  최악 빈 #{wbin[0]} · 피로 {each[wi]['fat']:.3f} · "
          f"정적 {each[wi]['sta']:.3f}")
    print(f"  빈별 피로 {min(x['fat'] for x in each):.3f} ~ "
          f"{max(x['fat'] for x in each):.3f}")
    rec = wbin[2]
    with open(os.path.join(OUTDIR, "worst_bin.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.writer(f)
        w.writerow(["DLC", "bin", "k", "rpm", "rev"] + list(KSIG)
                   + ["fatigue_inf", "static"])
        w.writerow([DLC, wbin[0], k, rec["rpm"], wbin[1]]
                   + [rec[x] for x in KSIG]
                   + [each[wi]["fat"], each[wi]["sta"]])
    blk = ["| 항목 | 값 |", "|---|--:|",
           f"| DLC | `{DLC}` |", f"| 빈 번호 | **#{wbin[0]}** / {len(reps)} |",
           f"| `k` | {k:g} |",
           f"| 회전수 | {rec['rpm']:,.3f} rpm |",
           f"| 30년 회전수 | {wbin[1]:,.3f} rev |"]
    for x in KSIG:
        u = "kN" if x.startswith("F") else "kNm"
        blk.append(f"| `{x}` | {rec[x]/1e3:,.1f} {u} |")
    blk += [f"| **무한수명 피로** | **{each[wi]['fat']:.3f}** |",
            f"| 영구변형 | {each[wi]['sta']:.3f} |"]
    doc = swap(io.open(DOC, encoding="utf-8").read(), MARK_BIN,
               "\n".join(blk))
    io.open(DOC, "w", encoding="utf-8").write(doc)

    # ── ② 비율 스윙 (최악 빈 1개만) ────────────────────────────
    one = [wbin]
    rows, t0 = [], time.perf_counter()

    def probe(r):
        idm = build(r)
        e, ms = run(one, f"r{int(r*1e4)}")
        t = (OD - idm) / 2 * 1e3
        rows.append(dict(ratio=r, ID_mm=idm * 1e3, t_mm=t,
                         OD_over_t=OD * 1e3 / t, mass_shaft_t=ms / 1e3,
                         fatigue_inf=e[0]["fat"], static=e[0]["sta"]))
        print(f"  r {r:.4f} · ID {idm*1e3:,.0f} · t {t:6.1f} · "
              f"질량 {ms/1e3:6.1f} t · 피로 {e[0]['fat']:6.3f} · "
              f"정적 {e[0]['sta']:6.3f}", flush=True)
        return e[0]["fat"]

    print(f"\n[②] 비율 스윙 (최악 빈 #{wbin[0]} 1개) · 목표 피로 ≥ {TARGET}")
    probe(R0)
    for r in COARSE:
        probe(r)
    ok = [x for x in rows if x["fatigue_inf"] >= TARGET]
    ng = [x for x in rows if x["fatigue_inf"] < TARGET]
    if ok and ng:
        lo = max(x["ratio"] for x in ok)
        hi = min(x["ratio"] for x in ng)
        print(f"\n  경계 정밀화 {lo:.4f} ~ {hi:.4f} (0.001)")
        r = lo + 0.001
        while r < hi - 1e-9:
            probe(round(r, 4))
            r += 0.001
    rows.sort(key=lambda x: x["ratio"])
    with open(os.path.join(OUTDIR, "thickness.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)

    ok = [x for x in rows if x["fatigue_inf"] >= TARGET]
    lim = max(ok, key=lambda x: x["ratio"]) if ok else None
    body = ["| 비율 `r` | 내경 [mm] | 두께 [mm] | `OD/t` | 샤프트 [t] | "
            "**피로** | 영구변형 | 판정 |", "|--:|--:|--:|--:|--:|--:|--:|:-:|"]
    for x in rows:
        mark = " ← **현행**" if abs(x["ratio"] - R0) < 1e-9 else (
            " ← **한계**" if lim and x["ratio"] == lim["ratio"] else "")
        body.append(
            f"| {x['ratio']:.4f}{mark} | {x['ID_mm']:,.0f} | {x['t_mm']:.1f} | "
            f"{x['OD_over_t']:.2f} | {x['mass_shaft_t']:.1f} | "
            f"**{x['fatigue_inf']:.3f}** | {x['static']:.3f} | "
            f"{'✅' if x['fatigue_inf'] >= TARGET else '❌'} |")
    doc = swap(io.open(DOC, encoding="utf-8").read(), MARK_SWEEP,
               "\n".join(body))
    io.open(DOC, "w", encoding="utf-8").write(doc)
    print(f"\n[완료] {len(rows)}점 · {time.perf_counter()-t0:.0f}s")
    if lim:
        b = rows[0]
        print(f"  한계 r {lim['ratio']:.4f} · 두께 {lim['t_mm']:.1f} mm · "
              f"질량 {lim['mass_shaft_t']:.1f} t "
              f"({lim['mass_shaft_t']-b['mass_shaft_t']:+.1f} t)")


if __name__ == "__main__":
    main()
