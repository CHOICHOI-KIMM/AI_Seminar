"""
부록 8 §8-6.2 후처리 — UW 미스얼라인먼트 · 축 피로 SF
========================================================
S3-c 프론트(`z1 ≥ 1.0` 8건)에 대해 표의 마지막 두 열을 채운다. 본런에 해석을
얹지 않고 사후에 뽑는 이유는 §8-6.2 에 적었다.

  · **UW 미스얼라인먼트** — 지배 LC `Myz_max` 로 재해석해 UW 베어링의
    `relative_misalignment` 를 읽는다. σ 와 **같은 하중**이어야 σ 열을 설명할 수
    있으므로 지배 LC 를 쓴다(§7-6.7.6 과 같은 기준).
  · **축 피로 SF** — `DLC1.2-k-s5` 최약 빈(#11)의 6자유도 대표하중을 걸고
    DIN 743 `Fatigue Safety Factor for Infinite Life` 를 읽는다. 두께 규칙
    `W` = 1.393×10⁹ mm³ 를 유도한 바로 그 하중이라, 목표 5.2 대비 실제 값이
    얼마나 나오는지가 이 열의 요지다(§7-6.7.5).

  σ 도 함께 다시 읽어 본런 값과 대조한다 — 재현되지 않으면 배선이 어긋난 것이다.

산출: 부록8_NSGA/S3_본최적화/a8_post.csv + 문서 §8-6.3a 표
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

import a8_eval                    # noqa: E402
import sizing_geom as sg          # noqa: E402
import run_appendix7_shaft as a7  # noqa: E402

OUT = os.path.join(HERE, "부록8_NSGA", "S3_본최적화")
SRC = os.path.join(OUT, "a8_pareto.csv")
BIN = os.path.join(HERE, "부록7_샤프트", "worst_bin.csv")
DOC = a7.DOC
GOV, TQ = "Myz_max", 22673.0
# 대상 갈래 — 인수 `a`(z1 ≥ 1.0) 또는 `b`(z1 ≥ 1.5) · 기본 `a`
_T = (sys.argv[1] if len(sys.argv) > 1 else "a").lower()
SUBSET = "z1>=1.0" if _T == "a" else "z1>=1.5"
MARK = f"A8:5{_T.upper()}_BRG"
DST = os.path.join(OUT, f"a8_post_{_T}.csv")


def sc(o, n):
    try:
        v = getattr(o, n)
    except Exception:
        return None
    if isinstance(v, (int, float)) and not isinstance(v, bool):
        return float(v)
    for a in ("value", "wrapped"):
        try:
            w = getattr(v, a)
        except Exception:
            continue
        if isinstance(w, (int, float)) and not isinstance(w, bool):
            return float(w)
    return None


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import (
        AnalysisType)
    from mastapy.bearings import RollerBearingProfileTypes as RP

    F = [r for r in csv.DictReader(open(SRC, encoding="utf-8-sig"))
         if r["subset"] == SUBSET]
    rec = next(csv.DictReader(open(BIN, encoding="utf-8-sig")))
    rec = {k: float(v) for k, v in rec.items() if k not in ("DLC",)}
    print(f"[대상] 프론트 {len(F)}건 · 최약 빈 #{int(rec['bin'])} "
          f"(DLC1.2-k-s5) · {rec['rpm']:.4f} rpm")

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

    lc_gov = next(c for c in dp.static_loads if c.name == GOV)
    q = lc_gov.inputs_for_power_load(ipl)
    for a, v in (("speed", 0.0), ("torque", TQ * 1e3)):
        try:
            setattr(q, a, v)
        except Exception:
            pass
    ds_gov = lc_gov.design_state_load_case_group
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    ds0 = lc0.design_state_load_case_group

    rows, t0 = [], time.perf_counter()
    for i, r in enumerate(F, 1):
        z1, z2 = float(r["z1"]), float(r["z2"])
        g = dict(D_pw=float(r["D_pw_mm"]) / 1e3, alpha_deg=float(r["alpha"]),
                 D_we=float(r["D_we_mm"]) / 1e3,
                 L_we=float(r["L_w_mm"]) / 1e3,      # MASTA 주입은 롤러 전장
                 bore=float(r["bore_mm"]) / 1e3,
                 outer_diameter=float(r["D_mm"]) / 1e3,
                 width=float(r["T_mm"]) / 1e3,
                 inner_ring_width=float(r["B_mm"]) / 1e3,
                 outer_ring_width=float(r["C_mm"]) / 1e3,
                 number_of_elements=int(float(r["Z"])))
        od = g["bore"]
        idm = a8_eval.shaft_id(od)                   # 두께 규칙 + floor

        for b in bs:                                 # 재장착 전 연결 제거 필수
            try:
                if b.inner_connection is not None:
                    b.inner_connection.delete()
            except Exception:
                pass
        sh.remove_all_sections()
        sh.add_section(0.0, z2 + sg.SHAFT_TAIL, od, idm, od, idm)
        for b in bs:
            sg.apply_to_masta(b.detail, g)
            for a in ("left_element_corner_radius",
                      "right_element_corner_radius"):
                setattr(b.detail, a, a8_eval.R_CORNER)   # §8-3 ②
        for b, z in ((uw, z1), (dw, z2)):
            b.try_mount_on(sh, z)

        # ── ⒜ 지배 LC — σ 재현 + UW 미스얼라인먼트 ──────────────
        dup = lc_gov.duplicate(ds_gov, f"a8p_{i}")
        duty = dp.add_duty_cycle(f"a8pd_{i}")
        duty.add_static_load(dup)
        csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
        csd.perform_analysis()
        smax, mis = 0.0, None
        for b in (uw, dw):
            for sub in list(
                    list(csd.results_for(b))[0].component_analysis_cases):
                det = sub.component_detailed_analysis
                v = sc(det, "maximum_normal_stress")
                if v and v / 1e6 > smax:
                    smax = v / 1e6
                if b is uw:
                    m = sc(det, "relative_misalignment")
                    if m is not None:
                        mis = abs(m) * 1e3
        for x in (dup, duty):
            try:
                x.delete()
            except Exception:
                pass

        # ── ⒝ 최약 빈 대표하중 — DIN 743 ────────────────────────
        lcf = lc0.duplicate(ds0, f"a8f_{i}")
        mf.set_loads(lcf, pl, ipl, rec)
        dutyf = dp.add_duty_cycle(f"a8fd_{i}")
        dutyf.add_static_load(lcf)
        csf = dutyf.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
        csf.perform_analysis()
        comp = list(csf.results_for(sh))[0]
        sf = {}
        for x in comp.component_analysis_cases:
            for it in x.safety_factors.items:
                sf["fat" if "Infinite" in it.description else "sta"] = \
                    float(it.safety_factor)
        for x in (lcf, dutyf):
            try:
                x.delete()
            except Exception:
                pass

        rows.append(dict(
            rank=int(r["rank"]), bore_mm=round(od * 1e3, 1),
            ID_mm=round(idm * 1e3, 1), t_mm=round((od - idm) / 2 * 1e3, 1),
            sigma_run=float(r["sigma_max_MPa"]), sigma_rerun=round(smax, 1),
            misalign_mrad=round(mis, 4) if mis is not None else None,
            SF_fatigue=round(sf.get("fat", 0.0), 3),
            SF_static=round(sf.get("sta", 0.0), 3)))
        x = rows[-1]
        print(f"  [{i}/{len(F)}] #{x['rank']} d {x['bore_mm']:,.0f} · t "
              f"{x['t_mm']:.1f} · σ {x['sigma_run']:,.1f}→"
              f"{x['sigma_rerun']:,.1f} · 미스 {x['misalign_mrad']} mrad · "
              f"SF {x['SF_fatigue']:.3f} / {x['SF_static']:.2f} "
              f"({time.perf_counter()-t0:.0f}s)", flush=True)

    with open(DST, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)

    ds_ = max(abs(r["sigma_rerun"] - r["sigma_run"]) for r in rows)
    print(f"\n[결과] σ 재현 최대 오차 {ds_:,.1f} MPa")
    print(f"  미스얼라인먼트 {min(r['misalign_mrad'] for r in rows):.3f} ~ "
          f"{max(r['misalign_mrad'] for r in rows):.3f} mrad")
    print(f"  축 피로 SF {min(r['SF_fatigue'] for r in rows):.3f} ~ "
          f"{max(r['SF_fatigue'] for r in rows):.3f} (목표 5.2)")

    # ── 문서 표의 마지막 두 열 채우기 ──────────────────────────
    by = {r["rank"]: r for r in rows}
    s = io.open(DOC, encoding="utf-8").read()
    a, b = f"<!-- {MARK} -->", f"<!-- /{MARK} -->"
    pat = re.compile(re.escape(a) + r"(.*?)" + re.escape(b), re.S)
    blk = pat.search(s).group(1)
    lines = []
    for ln in blk.split("\n"):
        m = re.match(r"^\| (\d+) \|", ln)
        if m and int(m.group(1)) in by:
            r = by[int(m.group(1))]
            # 마지막 셀은 `" | "` 로 쪼개면 `"— |"` 처럼 닫는 파이프를 물고
            # 나온다. 뒤에서 두 번째가 미스얼라인먼트, 마지막이 축 SF 다.
            cells = ln.split(" | ")
            cells[-2] = f"{r['misalign_mrad']:.3f}"
            cells[-1] = f"**{r['SF_fatigue']:.2f}** |"
            ln = " | ".join(cells)
        lines.append(ln)
    blk2 = "\n".join(lines).replace(
        "*UW 미스얼라인먼트·축 피로 SF 는 후처리 대기 — §8-6.2 "
        "(DLC1.2-k-s5 최약 빈 기준).*",
        "*UW 미스얼라인먼트는 지배 LC `Myz_max` 기준 · 축 피로 SF 는 "
        "`DLC1.2-k-s5` 최약 빈(#11) 대표하중 기준 DIN 743 무한수명 안전율 "
        "(§8-6.2).*")
    io.open(DOC, "w", encoding="utf-8").write(
        pat.sub(lambda m: a + blk2 + b, s, count=1))
    print(f"[문서] §8-6.3a 표 {len(rows)}행 갱신")


if __name__ == "__main__":
    main()
