"""
§10-12 — 롤러 프로파일 검토 (DIN Lundberg `axial_offset` · Johns-Gohar)
=========================================================================
부록 1 은 프로파일을 **최대 접촉응력** 관점에서 DIN Lundberg 로 충분하다고
확인했다. 그런데 `Myz_max` 에서 모멘트에 의한 정렬오차가 커 **응력분포가 한쪽
끝으로 쏠리고 롤러 끝단까지 접촉**한다 — 모서리 접촉 위험이다.

세 지표를 MASTA 에서 직접 읽어 프로파일을 비교한다.

  ① σ_max          `maximum_normal_stress_inner`      — 최적화 제약 (≤ 2,100)
  ② 끝단 응력      `maximum_normal_edge_stress_inner` — 모서리 접촉 위험
  ③ 접촉 손실      `L_we − contact_length_inner`      — 하중을 잃은 길이

대상: §10-11.2.2 의 `#1`·`#103`·`#210` · 지배 LC `Myz_max` 1건.

  python a10_profile.py            전체 (설계 3 × 조건 ~15)
  python a10_profile.py 1 210      일부 설계만
"""
import csv
import math
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import a8_eval                    # noqa: E402
import nsga_eval as ne            # noqa: E402
import sizing_geom as sg          # noqa: E402
import run_appendix7_shaft as a7  # noqa: E402

SRC = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "a10_pareto.csv")
OUT = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "profile_study.csv")
RANKS = (1, 103, 210)
GOV, TQ = "Myz_max", 22673.0
OFFSETS = (-60, -50, -40, -30, -20, -10, 0, 10, 20)     # mm — 프로파일 이동
JG_FACTORS = (1.0, 2.0, 4.0, 8.0, 16.0)  # design_load 배수
LB_FACTORS = (1.0, 2.0, 4.0, 8.0)        # LUNDBERG load 배수


def sc(o, n):
    try:
        v = getattr(o, n)
    except Exception:
        return None
    return float(v) if isinstance(v, (int, float)) else None


def main(ranks):
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import (
        AnalysisType)
    from mastapy.bearings import RollerBearingProfileTypes as RP

    F = {int(r["rank"]): r for r in csv.DictReader(
        open(SRC, encoding="utf-8-sig"))}
    d = Design.load(a7.MODEL)
    asm = d.all_parts_of_type_root_assembly()[0]
    sh = list(asm.all_parts_of_type_shaft())[0]
    bs = list(asm.all_parts_of_type_bearing())
    uw = [b for b in bs if "UW" in str(b)][0]
    dw = [b for b in bs if "DW" in str(b)][0]
    dp = asm.design_properties
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    lc = next(c for c in dp.static_loads if c.name == GOV)
    q = lc.inputs_for_power_load(ipl)
    for a, v in (("speed", 0.0), ("torque", TQ * 1e3)):
        try:
            setattr(q, a, v)
        except Exception:
            pass
    ds = lc.design_state_load_case_group

    rows, t0, n = [], time.perf_counter(), 0
    for rk in ranks:
        r = F[rk]
        z1, z2 = float(r["z1"]), float(r["z2"])
        Lwe = float(r["L_w_mm"]) - 8.6                  # 유효 롤러 길이 [mm]
        g = ne.geom(float(r["D_pw_mm"]) / 1e3, float(r["alpha"]),
                    float(r["D_we_mm"]) / 1e3, float(r["L_w_mm"]) / 1e3, True)
        od, idm = g["bore"], a8_eval.shaft_id(g["bore"])

        def build():
            for b in bs:
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
                    setattr(b.detail, a, a8_eval.R_CORNER)
            for b, z in ((uw, z1), (dw, z2)):
                b.try_mount_on(sh, z)

        def solve(tag):
            dup = lc.duplicate(ds, f"pf_{tag}")
            duty = dp.add_duty_cycle(f"pfd_{tag}")
            duty.add_static_load(dup)
            csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            csd.perform_analysis()
            det = list(list(csd.results_for(uw))[0]
                       .component_analysis_cases)[0].component_detailed_analysis
            row = det.rows[0]
            els = list(row.elements)
            cl = [sc(e, "contact_length_inner") for e in els]
            cl = [c for c in cl if c]
            out = dict(
                sigma=sc(det, "maximum_normal_stress_inner") / 1e6,
                edge=sc(row, "maximum_normal_edge_stress_inner") / 1e6,
                contact_len=max(cl) * 1e3 if cl else None,
                n_contact=sc(det, "number_of_elements_in_contact"),
                Pmax=sc(det, "maximum_normal_load_inner"),
                tilt=max((sc(e, "element_tilt") or 0) for e in els) * 1e3)
            for x in (dup, duty):
                try:
                    x.delete()
                except Exception:
                    pass
            return out

        # ── DIN Lundberg · axial_offset 스윙 ──────────────────────
        base_P = None
        for off in OFFSETS:
            build()
            for b in bs:
                ps = b.detail.roller_profile_set
                ps.active_profile_type = RP.DIN_LUNDBERG
                ps.active_profile.axial_offset = off / 1e3
            n += 1
            v = solve(f"{rk}_dl{off}")
            if off == 0:
                base_P = v["Pmax"]
            loss = Lwe - v["contact_len"] if v["contact_len"] else None
            rows.append(dict(rank=rk, profile="DIN_LUNDBERG",
                             param=f"axial_offset={off}", L_we=round(Lwe, 1),
                             sigma_MPa=round(v["sigma"], 1),
                             edge_MPa=round(v["edge"], 1),
                             contact_mm=round(v["contact_len"], 1),
                             loss_mm=round(loss, 1),
                             loss_pct=round(100 * loss / Lwe, 1),
                             n_contact=int(v["n_contact"]),
                             tilt_mrad=round(v["tilt"], 4)))
            print(f"  #{rk:<4} DIN off {off:+4d}  σ {v['sigma']:7.1f} · "
                  f"끝단 {v['edge']:7.1f} · 접촉 {v['contact_len']:6.1f}/"
                  f"{Lwe:.1f} (손실 {loss:5.1f} · "
                  f"{100*loss/Lwe:4.1f}%)  {time.perf_counter()-t0:5.0f}s",
                  flush=True)

        # ── Johns-Gohar · design_load 배수 ────────────────────────
        for f_ in JG_FACTORS:
            build()
            for b in bs:
                ps = b.detail.roller_profile_set
                ps.active_profile_type = RP.JOHNS_GOHAR
                try:
                    ps.active_profile.design_load = base_P * f_
                except Exception as e:
                    print(f"    design_load 설정 실패 "
                          f"{str(e).splitlines()[0][:50]}")
            jg = uw.detail.roller_profile_set.active_profile
            ed = sc(jg, "end_drop")
            n += 1
            v = solve(f"{rk}_jg{f_}")
            loss = Lwe - v["contact_len"] if v["contact_len"] else None
            rows.append(dict(rank=rk, profile="JOHNS_GOHAR",
                             param=f"design_load={base_P*f_/1e6:.2f}MN "
                                   f"(x{f_}) · end_drop="
                                   f"{ed*1e6:.0f}um" if ed else "",
                             L_we=round(Lwe, 1),
                             sigma_MPa=round(v["sigma"], 1),
                             edge_MPa=round(v["edge"], 1),
                             contact_mm=round(v["contact_len"], 1),
                             loss_mm=round(loss, 1),
                             loss_pct=round(100 * loss / Lwe, 1),
                             n_contact=int(v["n_contact"]),
                             tilt_mrad=round(v["tilt"], 4)))
            print(f"  #{rk:<4} JG   ×{f_:<4}  σ {v['sigma']:7.1f} · "
                  f"끝단 {v['edge']:7.1f} · 접촉 {v['contact_len']:6.1f}/"
                  f"{Lwe:.1f} (손실 {loss:5.1f} · "
                  f"{100*loss/Lwe:4.1f}%) · end_drop "
                  f"{ed*1e6 if ed else 0:.0f} um", flush=True)

        # ── Lundberg · load 스윙 (크라운 깊이를 직접 키운다) ─────
        for f_ in LB_FACTORS:
            build()
            for b in bs:
                ps = b.detail.roller_profile_set
                ps.active_profile_type = RP.LUNDBERG
                try:
                    ps.active_profile.use_bearing_dynamic_capacity = False
                    ps.active_profile.load = base_P * f_
                except Exception as e:
                    print(f"    LUNDBERG 설정 실패 "
                          f"{str(e).splitlines()[0][:50]}")
            n += 1
            v = solve(f"{rk}_lb{f_}")
            loss = Lwe - v["contact_len"] if v["contact_len"] else None
            rows.append(dict(rank=rk, profile="LUNDBERG",
                             param=f"load={base_P*f_/1e6:.2f}MN (x{f_})",
                             L_we=round(Lwe, 1),
                             sigma_MPa=round(v["sigma"], 1),
                             edge_MPa=round(v["edge"], 1),
                             contact_mm=round(v["contact_len"], 1),
                             loss_mm=round(loss, 1),
                             loss_pct=round(100 * loss / Lwe, 1),
                             n_contact=int(v["n_contact"]),
                             tilt_mrad=round(v["tilt"], 4)))
            print(f"  #{rk:<4} LB   x{f_:<4}  s {v['sigma']:7.1f} · "
                  f"edge {v['edge']:7.1f} · contact {v['contact_len']:6.1f}/"
                  f"{Lwe:.1f} (loss {loss:5.1f} · "
                  f"{100*loss/Lwe:4.1f}%)", flush=True)

        # 원상 복구
        for b in bs:
            ps = b.detail.roller_profile_set
            ps.active_profile_type = RP.DIN_LUNDBERG
            ps.active_profile.axial_offset = 0.0

    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)
    print(f"\n[저장] {OUT} · {n}회 해석 · {(time.perf_counter()-t0)/60:.1f}분")


if __name__ == "__main__":
    main([int(x) for x in sys.argv[1:]] or list(RANKS))
