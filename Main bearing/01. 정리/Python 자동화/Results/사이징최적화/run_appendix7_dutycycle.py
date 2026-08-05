"""
부록 7-6 — 극한 LC 대 시계열 듀티사이클 (샤프트 DIN 743)
===========================================================
§7-5.2 가 제기한 「극한 하중으로 피로 안전율을 보는 것이 맞는가」를 실측으로
답한다. 대상은 `a01`(베어링 최경량 · §6-11.5a #1).

  (A) 극한 16 LC — 엑셀의 `Mx` 를 토크로 넣고 각각 DIN 743.
      저장된 모델은 **14개가 토크 미설정(`nan`)** 이라 넣어야 한다(§8-1.4 표).
      지배 LC 가 정말 `Myz_max` 인지 확인한다.

  (B) 손상 지배 DLC — `DLC1.2-d-s1`(a01 총손상의 3.7% · 111 DLC 중 최대)을
      `dt=20` · μ+kσ 대표하중 30빈으로 바꿔 **듀티사이클** 로 돌린다.
      변환은 피로 해석과 같은 `bin_reps()` 를 그대로 쓰고 `k` 는
      `screen_k.csv` 의 값(0.19)을 그대로 쓴다 — 새로 만들지 않는다.

**`Myz_max` 가 난 `DLC2.2.4-b-s04` 는 쓸 수 없다.** 극한하중은 엑셀의 극값
1세트뿐이고 그 DLC 의 시계열이 없다(피로 111 DLC 는 DLC1.2·2.4.1·2.4.2·4.1·6.4
다섯 계열뿐이다). 그래서 (B)는 손상을 지배하는 DLC 로 대신한다.

산출: 부록7_샤프트/dutycycle.csv
"""
import csv
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
DLCDIR = os.path.join(RES, "DLC별해석")
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg          # noqa: E402
import run_p2_fatigue as p2       # noqa: E402  (bin_reps · load_raw 재사용)
import run_appendix7_shaft as a7  # noqa: E402  (geom_of 재사용)

MODEL = a7.MODEL
TAG, DLC = "a01", "DLC1.2-d-s1"
OUT = os.path.join(HERE, "부록7_샤프트", "dutycycle.csv")
NBATCH = 30

# 극한 16 LC 의 토크 = 엑셀 Mx [kNm] (§8-1.4 표 · 좌표변환 TQ = +Mx)
EXTREME = {
    "Mx_max": 61192.0, "Mx_min": -22925.0, "My_max": 23670.0,
    "My_min": 27453.0, "Mz_max": 10308.0, "Mz_min": 17013.0,
    "Myz_max": 22673.0, "Myz_min": 58208.0, "Fx_max": 51143.0,
    "Fx_min": 90.3, "Fy_max": -40.3, "Fy_min": 33.3,
    "Fz_max": -2.0, "Fz_min": 61059.0, "Fyz_max": 55311.0,
}


def sf_of(sub):
    return {("fatigue_inf" if "Infinite" in i.description else "static"):
            round(float(i.safety_factor), 4)
            for i in sub.safety_factors.items}


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
        r = next(x for x in csv.DictReader(f) if x["rank_mass"] == TAG)
    with open(os.path.join(HERE, "P2_피로수명_S4", "screen_k.csv"),
              encoding="utf-8-sig") as f:
        kr = next(x for x in csv.DictReader(f)
                  if x["design"] == TAG and x["DLC"] == DLC)
    k, sf_scale = float(kr["k"]), float(kr["ScaleFactor"])
    z1, z2 = float(r["z1"]), float(r["z2"])

    d = Design.load(MODEL)
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

    # ── a01 제원 주입 (CSV 기록값 그대로) ──────────────────────────
    g, s = a7.geom_of(r)
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
    print(f"[설계] {TAG} · bore {g['bore']*1e3:,.0f} · "
          f"샤프트 OD {s['outer_diameter']*1e3:,.0f} / "
          f"ID {s['inner_diameter']*1e3:,.0f} · L {s['length']*1e3:,.0f}")

    rows = []

    # ── (A) 극한 16 LC ────────────────────────────────────────────
    print("\n[A] 극한 LC — 엑셀 Mx 를 토크로 주입")
    for name, tq in EXTREME.items():
        lc = next((c for c in dp.static_loads if c.name == name), None)
        if lc is None:
            print(f"  !! {name} 없음")
            continue
        q = lc.inputs_for_power_load(ipl)
        for a, v in (("speed", 0.0), ("torque", tq * 1e3)):
            try:
                setattr(q, a, v)
            except Exception:
                pass
        duty = dp.add_duty_cycle(f"ex_{name}")
        duty.add_static_load(lc)
        csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
        csd.perform_analysis()
        sub = list(list(csd.results_for(sh))[0].component_analysis_cases)[0]
        v = sf_of(sub)
        duty.delete()
        rows.append(dict(case=name, kind="극한 LC", n_lc=1,
                         torque_kNm=tq, **v))
        print(f"  {name:9} TQ {tq:>10,.1f} kNm · 피로 {v['fatigue_inf']:7.3f} "
              f"· 정적 {v['static']:7.3f}")

    # ── (B) 손상 지배 DLC 듀티사이클 ──────────────────────────────
    print(f"\n[B] {DLC} — dt=20 · μ+{k}σ 대표하중 듀티사이클")
    reps = p2.bin_reps(p2.load_raw(DLC), k)
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    ds = lc0.design_state_load_case_group
    t0 = time.perf_counter()
    lcs = []
    for cid, rev, rec in reps:
        lc = lc0.duplicate(ds, f"a7dc_{cid}")
        mf.set_loads(lc, pl, ipl, rec)
        lcs.append(lc)
    duty = dp.add_duty_cycle("a7_dutycycle")
    for lc in lcs:
        duty.add_static_load(lc)
    csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
    csd.perform_analysis()
    comp = list(csd.results_for(sh))[0]
    subs = list(comp.component_analysis_cases)
    each = [sf_of(x) for x in subs]
    worst = min(each, key=lambda v: v["fatigue_inf"])
    grp = sf_of(comp)                     # 듀티사이클 레벨 집계
    for x in lcs + [duty]:
        try:
            x.delete()
        except Exception:
            pass
    print(f"  빈 {len(reps)}개 · {time.perf_counter()-t0:.1f}s")
    print(f"  빈별 피로 최소 {worst['fatigue_inf']:.3f} · "
          f"최대 {max(v['fatigue_inf'] for v in each):.3f}")
    print(f"  듀티사이클 집계 피로 {grp['fatigue_inf']:.3f} · "
          f"정적 {grp['static']:.3f}")
    rows.append(dict(case=f"{DLC} (빈별 최악)", kind="듀티사이클",
                     n_lc=len(reps), torque_kNm="", **worst))
    rows.append(dict(case=f"{DLC} (듀티 집계)", kind="듀티사이클",
                     n_lc=len(reps), torque_kNm="", **grp))

    keys = ["case", "kind", "n_lc", "torque_kNm", "fatigue_inf", "static"]
    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=keys, extrasaction="ignore")
        w.writeheader()
        w.writerows(rows)
    print(f"\n[저장] {os.path.basename(OUT)} · {len(rows)}행")


if __name__ == "__main__":
    main()
