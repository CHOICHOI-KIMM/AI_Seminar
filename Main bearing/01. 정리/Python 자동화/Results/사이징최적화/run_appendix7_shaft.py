"""
부록 7 — 샤프트 사양 및 성능 검토
====================================
§6-11.7 의 40건(a·b)과 v1.3 기준선에 대해 두 가지를 정리한다.

  ① 샤프트 사양   무게 · 내경 · 외경 · 두께 · 외경/두께비 · 길이 · z1 · z2
  ② DIN 743 안전율  지배 LC `Myz_max` (토크 22,673 kNm · speed 0 — P1 과 동일)

**①은 MASTA 가 필요 없다.** 내·외경과 길이는 `sizing_geom.shaft()` 가 보어와
`z2` 로부터 내고, 무게는 이미 `p2d_targets.csv` 에 MASTA 실측으로 있다.

**②는 설계마다 모델을 재구성해 해석한다**(P1 배치와 같은 순서). 모델의 정격
방법은 이미 `ShaftRatingMethod.DIN_743201212` 로 잡혀 있고, 안전율은
`ShaftSystemDeflection.safety_factors.items[]` 에서 읽는다 —
`description` · `safety_factor` · `minimum_required_safety_factor`.

주의 — `Myz_max` 는 **토크가 `nan`** 인 상태로 저장돼 있어 실행 시점에 넣어야
한다(P1 도 그랬다). 온도는 이 LC 의 설계 기본값 **80 °C** 를 그대로 쓴다.

산출: 부록7_샤프트/shaft_spec.csv · din743.csv
"""
import csv
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg          # noqa: E402

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
TGT = os.path.join(HERE, "P2_피로수명_S4", "p2d_targets.csv")
OUT = os.path.join(HERE, "부록7_샤프트")
GOV, TQ = "Myz_max", 22673.0          # 지배 극한 LC (§8-4.2)

# v1.3 기준선 — 격자점이 아니라 별도 지정 (§8-3.1 과 같은 방식)
BASE = dict(rank_mass="base", set="—", rank="—", z1=0.5, z2=3.0,
            D_pw_mm=3330.9, alpha=19.0, D_we_mm=110.51, L_we_mm=238.048,
            mass_shaft_kg=43225.8)

SPEC_COLS = ["tag", "set", "rank", "mass_shaft_kg", "OD_mm", "ID_mm",
             "t_mm", "OD_over_t", "length_mm", "z1_m", "z2_m"]

DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
MARK = {"a": "<!-- A7:SPEC_A -->", "b": "<!-- A7:SPEC_B -->"}
HDR = ("| 태그 | 프론트 # | 무게 [t] | 내경 [mm] | 외경 [mm] | 두께 [mm] | "
       "외경/두께 | 길이 [mm] | z1 [m] | z2 [m] | **피로 안전율** |")
SEP = "|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|"


def spec_line(r, sf):
    """사양 1행. `sf` 는 DIN 743 무한수명 피로 안전율(없으면 —)."""
    f = lambda k: float(r[k])                                   # noqa: E731
    return (f"| {r['tag']} | {r['rank']} | {f('mass_shaft_kg')/1e3:.1f} | "
            f"{f('ID_mm'):,.0f} | {f('OD_mm'):,.0f} | {f('t_mm'):.1f} | "
            f"{f('OD_over_t'):.2f} | {f('length_mm'):,.0f} | "
            f"{f('z1_m'):.1f} | {f('z2_m'):.1f} | "
            + (f"**{sf:.3f}**" if sf is not None else "—") + " |")


def write_doc(specs, sfmap):
    """§7-2 의 a·b 표를 짝 마커 사이에 채운다"""
    import io
    import re
    by = {r["tag"]: r for r in specs}
    base = by["base"]
    s = io.open(DOC, encoding="utf-8").read()
    for pre, mark in MARK.items():
        rows, prev = [], None
        for r in specs:
            if r["tag"] == "base" or r["set"] != pre:
                continue
            n = int(r["rank"])
            if prev is not None and prev <= 10 < n:
                rows.append("| **— 총질량 최경량 —** |" + " |" * 10)
            rows.append(spec_line(r, sfmap.get(r["tag"])))
            prev = n
        blk = "\n".join([HDR, SEP,
                         spec_line(dict(base, tag="**기준선** (v1.3)",
                                        rank="—"), sfmap.get("base"))] + rows)
        close = mark.replace("<!-- ", "<!-- /")
        pat = re.compile(re.escape(mark) + r"\n.*?\n" + re.escape(close), re.S)
        if not pat.search(s):
            raise RuntimeError(f"{mark} … {close} 자리표를 찾지 못했다")
        s = pat.sub(lambda m: f"{mark}\n{blk}\n{close}", s, count=1)
    io.open(DOC, "w", encoding="utf-8").write(s)


def spec_rows():
    """샤프트 사양 — MASTA 없이 계산한다"""
    with open(TGT, encoding="utf-8-sig") as f:
        rows = [BASE] + [dict(r) for r in csv.DictReader(f)]
    out = []
    for r in rows:
        z2 = float(r["z2"])
        g = sg.bearing(float(r["D_pw_mm"]) / 1e3, float(r["alpha"]),
                       float(r["D_we_mm"]) / 1e3, float(r["L_we_mm"]) / 1e3)
        s = sg.shaft(g["bore"], z2)
        od, idm = s["outer_diameter"] * 1e3, s["inner_diameter"] * 1e3
        t = (od - idm) / 2.0
        out.append(dict(tag=r["rank_mass"], set=r["set"], rank=r["rank"],
                        mass_shaft_kg=round(float(r["mass_shaft_kg"]), 1),
                        OD_mm=round(od, 1), ID_mm=round(idm, 1),
                        t_mm=round(t, 1), OD_over_t=round(od / t, 2),
                        length_mm=round(s["length"] * 1e3, 1),
                        z1_m=float(r["z1"]), z2_m=z2))
    return out


def main():
    os.makedirs(OUT, exist_ok=True)
    specs = spec_rows()
    p = os.path.join(OUT, "shaft_spec.csv")
    with open(p, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=SPEC_COLS)
        w.writeheader()
        w.writerows(specs)
    print(f"[사양] {len(specs)}건 → {os.path.basename(p)}")

    # ── DIN 743 ────────────────────────────────────────────────────
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import (
        AnalysisType)
    from mastapy.bearings import RollerBearingProfileTypes as RP

    d = Design.load(MODEL)
    asm = d.all_parts_of_type_root_assembly()[0]
    sh = list(asm.all_parts_of_type_shaft())[0]
    bs = list(asm.all_parts_of_type_bearing())
    uw = [b for b in bs if "UW" in str(b)][0]
    dw = [b for b in bs if "DW" in str(b)][0]
    for b in bs:
        b.detail.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG
    dp = asm.design_properties
    ipl = next(pl for pl in asm.all_parts_of_type_power_load()
               if "input" in str(pl).lower())
    lc = next(c for c in dp.static_loads if c.name == GOV)
    # 저장된 LC 는 토크가 nan 이다 — 여기서 넣는다 (P1 과 동일)
    q = lc.inputs_for_power_load(ipl)
    for a, v in (("speed", 0.0), ("torque", TQ * 1e3)):
        try:
            setattr(q, a, v)
        except Exception:
            pass
    t0 = lc.temperatures
    print(f"[LC] {GOV} 토크 {q.torque/1e3:,.0f} kNm · speed {q.speed} · "
          f"베어링 {t0.rolling_bearing_element:.0f} °C "
          f"(use_default={lc.use_default_temperatures})")

    with open(TGT, encoding="utf-8-sig") as f:
        tg = [dict(r) for r in csv.DictReader(f)]
    rows, t_all = [], time.perf_counter()
    for i, r in enumerate([BASE] + tg, 1):
        tag = r["rank_mass"]
        z1, z2 = float(r["z1"]), float(r["z2"])
        g = sg.bearing(float(r["D_pw_mm"]) / 1e3, float(r["alpha"]),
                       float(r["D_we_mm"]) / 1e3, float(r["L_we_mm"]) / 1e3)
        for b in bs:
            try:
                if b.inner_connection is not None:
                    b.inner_connection.delete()
            except Exception:
                pass
        s = sg.shaft(g["bore"], z2)
        sh.remove_all_sections()
        sh.add_section(0.0, s["length"], s["outer_diameter"],
                       s["inner_diameter"], s["outer_diameter"],
                       s["inner_diameter"])
        for b in bs:
            sg.apply_to_masta(b.detail, g)
        for b, z in ((uw, z1), (dw, z2)):
            b.try_mount_on(sh, z)

        duty = dp.add_duty_cycle(f"a7_{i}")
        duty.add_static_load(lc)
        csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
        csd.perform_analysis()
        sub = list(list(csd.results_for(sh))[0].component_analysis_cases)[0]
        # `mass_including_connected_components` 는 베어링까지 포함하므로
        # 샤프트 질량으로 쓰면 안 된다 — 사양표의 MASTA 실측을 쓴다
        row = dict(tag=tag)
        for it in sub.safety_factors.items:
            key = ("fatigue_inf" if "Infinite" in it.description else
                   "static" if "Permanent Deformation" in it.description else
                   it.description[:24])
            row[key] = round(float(it.safety_factor), 4)
            row[key + "_min"] = round(float(it.minimum_required_safety_factor), 2)
            row.setdefault("descriptions", []).append(it.description)
        duty.delete()
        rows.append(row)
        if i <= 2 or i % 10 == 0:
            print(f"  [{i}/{len(tg)+1}] {tag}  " +
                  " · ".join(f"{k} {v}" for k, v in row.items()
                             if k.endswith("inf") or k == "static"), flush=True)

    keys = ["tag"] + sorted({k for r in rows for k in r
                             if k not in ("tag", "descriptions")})
    p = os.path.join(OUT, "din743.csv")
    with open(p, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=keys, extrasaction="ignore")
        w.writeheader()
        w.writerows(rows)
    print(f"\n[DIN 743] {len(rows)}건 · {(time.perf_counter()-t_all)/60:.1f}분 "
          f"→ {os.path.basename(p)}")
    print("  항목: " + " / ".join(rows[0].get("descriptions", [])))

    # ── 문서 §7-2 표 갱신 ──────────────────────────────────────────
    sfmap = {r["tag"]: r.get("fatigue_inf") for r in rows}
    write_doc(specs, sfmap)
    na = sum(1 for r in specs if r["set"] == "a")
    print(f"[문서] §7-2 a {na}행 · b {len(specs)-na-1}행 갱신")


if __name__ == "__main__":
    main()
