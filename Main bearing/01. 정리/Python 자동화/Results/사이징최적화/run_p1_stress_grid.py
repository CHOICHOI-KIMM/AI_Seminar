"""
P1 — 극한 응력 전수 격자 · Phase 3 (문서 §8-4)
=====================================
유효 설계점 × 지배 극한 LC × 2 베어링 → σ_max 가능영역 확정.

  정렬  : 응력 유리도 복합지표 S = L_we × Z × L_eff 내림차순 (큰 것부터)
  체크포인트 : 매 점 CSV append — 재시작 시 기존 행 건너뜀
  프루닝 : 사용하지 않음 (α 비단조 · Z=floor 계단형)

Phase 3 (260730) — 배치 변수 확대. 베어링 제원 격자는 Phase 2 와 동일
  · z1 0.3~1.0 → 1.0~3.0 (5) · z2 3.0~5.0 → 3.0~6.0 (7) → 유효 8,700점
  · (C8) 비활성화 · (C4) 스팬 하한이 실효 제약으로 (1,800점 탈락)
  · Phase 2 중복 1,500점(z1 1.0 x z2 3.0~5.0)은 시딩해 재사용
  · 극한 LC 는 지배 1건(Myz_max) · 500점마다 문서 §8-4.3 자동 갱신

사용:
    python run_p1_stress_grid.py --probe      # 예비측정 10점
    python run_p1_stress_grid.py              # 본실행 (재개 가능)
"""
import csv
import itertools
import math
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
OUTDIR = os.path.join(HERE, "P1_극한응력_Phase3")
GRID_CSV = os.path.join(OUTDIR, "p1_grid.csv")
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg      # noqa: E402
import update_p1_table       # noqa: E402

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")

# ── 격자 (§8-1.1) ──
# 260730 Phase 3 — 배치 변수 확대 (§8-4.1)
Z1 = [1.0, 1.5, 2.0, 2.5, 3.0]
Z2 = [3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0]
DPW = [3.300, 3.600, 3.900, 4.200, 4.500]
ALPHA = [15.0, 19.0, 23.0, 27.0, 31.0]   # 260729 개정 — 4도 등간격
# 260729 개정 — 세장비 상한 2.5 반영 + D_we 확대 + L_we 75mm 절대격자
DWE = [0.110, 0.140, 0.170, 0.200, 0.230]
LWE = [0.175, 0.250, 0.325, 0.400, 0.475, 0.550]

# ── 지배 극한 LC (§8-1.2) : 이름 → 축토크 [kNm] ──
GOV = {"Myz_max": 22673.0}          # Phase 2 — 지배 1 LC 만
LIMIT = 2100.0            # MPa

FIELDS = ["idx", "S_rank", "z1", "z2", "D_pw_mm", "alpha", "D_we_mm", "L_we_mm",
          "slenderness",
          "Z", "bore_mm", "D_mm", "T_mm", "B_mm", "C_mm",
          "t_i_mm", "t_o_mm", "c12_margin",
          "L_eff_m", "S_index", "mass_brg_kg", "mass_shaft_kg", "mass_total_kg",
          "Myz_max_UW", "Myz_max_DW",
          "sigma_max_MPa", "governing", "feasible", "t_s", "warn"]


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception:
        return None


def sc(o, n):
    v = safe(o, n)
    if isinstance(v, (int, float)) and not isinstance(v, bool):
        return float(v)
    for a in ("value", "wrapped"):
        w = safe(v, a)
        if isinstance(w, (int, float)) and not isinstance(w, bool):
            return float(w)
    return None


def build_grid():
    """제약 통과 설계점 + 정렬지표 S. a(유효하중중심)는 근사식으로 정렬용만 산출."""
    pts = []
    for z1, z2, dpw, al, dwe, lwe in itertools.product(Z1, Z2, DPW, ALPHA, DWE, LWE):
        g = sg.bearing(dpw, al, dwe, lwe)
        if sg.constraints(g, z1, z2, cone_deg=al):
            continue
        T = g["width"]
        a_appx = T / 2 + (dpw / 2) * math.tan(math.radians(al))   # §4-5 근사
        L_eff = (z2 - z1) + 2 * (a_appx - T / 2)
        S = lwe * g["number_of_elements"] * L_eff
        pts.append(dict(z1=z1, z2=z2, D_pw=dpw, alpha=al, D_we=dwe, L_we=lwe,
                        g=g, L_eff=L_eff, S=S))
    pts.sort(key=lambda r: -r["S"])          # 큰 것부터
    for i, r in enumerate(pts, 1):
        r["S_rank"] = i
    return pts


def done_keys():
    if not os.path.isfile(GRID_CSV):
        return set()
    with open(GRID_CSV, encoding="utf-8-sig") as f:
        return {r["idx"] for r in csv.DictReader(f)}


def key_of(r):
    return (f"{r['z1']:.2f}_{r['z2']:.2f}_{r['D_pw']*1e3:.0f}_{r['alpha']:.0f}"
            f"_{r['D_we']*1e3:.0f}_{r['L_we']*1e3:.0f}")


def main(probe=False):
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
    from mastapy.bearings import RollerBearingProfileTypes as RP

    os.makedirs(OUTDIR, exist_ok=True)
    pts = build_grid()
    print(f"[P1 Phase 3] 유효 설계점 {len(pts):,} · 지배 LC {list(GOV)} · 정렬 S = L_we x Z x L_eff")
    if probe:
        pts = pts[:10]
        print("     [예비측정] 상위 10점만 수행")
    dk = done_keys()
    todo = [r for r in pts if key_of(r) not in dk]
    print(f"     기존 완료 {len(dk):,} · 이번 실행 {len(todo):,}")
    if not todo:
        print("     남은 점 없음"); return

    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    sh = list(asm.all_parts_of_type_shaft())[0]
    bs = list(asm.all_parts_of_type_bearing())
    uw = [b for b in bs if "UW" in str(b)][0]
    dw = [b for b in bs if "DW" in str(b)][0]
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    for b in bs:
        b.detail.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG
    dp = asm.design_properties
    lc_tpl = {}
    for nm, tq in GOV.items():
        lc = next(c for c in dp.static_loads if c.name == nm)
        q = lc.inputs_for_power_load(ipl)
        for a_, v_ in (("speed", 0.0), ("torque", tq * 1e3)):
            try:
                setattr(q, a_, v_)
            except Exception:
                pass
        lc_tpl[nm] = lc
    ds = lc_tpl["Myz_max"].design_state_load_case_group
    print("[모델] 로드 완료")

    new = not os.path.isfile(GRID_CSV)
    f = open(GRID_CSV, "a", newline="", encoding="utf-8-sig")
    w = csv.DictWriter(f, fieldnames=FIELDS)
    if new:
        w.writeheader()

    cur_z = (None, None)
    t0 = time.time()
    for n, r in enumerate(todo, 1):
        t1 = time.perf_counter()
        g = r["g"]
        warn = []
        # ① 두 베어링 분리 → ② 샤프트 재구성 → ③ 베어링 제원 → ④ 재장착
        for b in bs:
            try:
                if b.inner_connection is not None:
                    b.inner_connection.delete()
            except Exception as e:
                warn.append(f"unmount:{str(e).splitlines()[0][:30]}")
        s = sg.shaft(g["bore"], r["z2"])
        try:
            sh.remove_all_sections()
            sh.add_section(0.0, s["length"], s["outer_diameter"],
                           s["inner_diameter"], s["outer_diameter"],
                           s["inner_diameter"])
        except Exception as e:
            warn.append(f"shaft:{str(e).splitlines()[0][:35]}")
        for b in bs:
            bad = sg.apply_to_masta(b.detail, g)
            if bad:
                warn.append("spec:" + bad[0])
        for b, z in ((uw, r["z1"]), (dw, r["z2"])):
            try:
                res = b.try_mount_on(sh, z)
                if res is not None and safe(res, "was_connection_created") is False:
                    warn.append(f"mount{z}:{safe(res, 'failure_message')}"[:45])
            except Exception as e:
                warn.append(f"mount{z}:{str(e).splitlines()[0][:30]}")
        for b, tg in ((uw, "UW"), (dw, "DW")):
            if safe(b, "is_mounted") is not True:
                warn.append(f"{tg} 미장착")
        cur_z = (r["z1"], r["z2"])

        # L_eff — MASTA 실측 a 로 산출 (§8-1.7.3.1 결정)
        a_m = sc(uw.detail, "effective_centre_from_front_face")
        T_m = sc(uw.detail, "width")
        L_meas = ((r["z2"] - r["z1"]) + 2 * (a_m - T_m / 2)
                  if (a_m is not None and T_m is not None) else None)

        # ── 해석: LC 복제 + 듀티사이클 (기하 변경 시 캐시 회피 — 필수) ──
        sig, best, who = {}, 0.0, ""
        names, dups = [], []
        for nm, tq in GOV.items():
            lc = lc_tpl[nm].duplicate(ds, f"p1_{n}_{nm}")
            q = lc.inputs_for_power_load(ipl)
            for a_, v_ in (("speed", 0.0), ("torque", tq * 1e3)):
                try:
                    setattr(q, a_, v_)
                except Exception:
                    pass
            names.append(nm); dups.append(lc)
        duty = dp.add_duty_cycle(f"p1dc_{n}")
        for lc in dups:
            duty.add_static_load(lc)
        csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
        csd.perform_analysis()
        for b, tag in ((uw, "UW"), (dw, "DW")):
            subs = list(list(csd.results_for(b))[0].component_analysis_cases)
            for nm, sub in zip(names, subs):
                v = sc(sub.component_detailed_analysis, "maximum_normal_stress")
                v = None if v is None else v / 1e6
                sig[f"{nm}_{tag}"] = v
                if v and v > best:
                    best, who = v, f"{nm}/{tag}"
        for lc in dups:
            try:
                lc.delete()
            except Exception:
                pass
        try:
            duty.delete()
        except Exception:
            pass
        if best <= 0.0:
            warn.append("sigma=0 (해석 무효)")
        if warn:
            print(f"  !! [{n}] {r['S_rank']}: {warn[:3]}", flush=True)
        mb = sc(uw.detail, "mass") or 0.0
        ms = sc(sh, "mass_of_shaft_body") or 0.0
        row = dict(idx=key_of(r), S_rank=r["S_rank"], z1=r["z1"], z2=r["z2"],
                   D_pw_mm=g["D_pw"] * 1e3, alpha=g["alpha_deg"],
                   D_we_mm=g["D_we"] * 1e3, L_we_mm=g["L_we"] * 1e3,
                   slenderness=round(g["L_we"] / g["D_we"], 3),
                   Z=g["number_of_elements"], bore_mm=g["bore"] * 1e3,
                   D_mm=g["outer_diameter"] * 1e3, T_mm=g["width"] * 1e3,
                   B_mm=g["inner_ring_width"] * 1e3,
                   C_mm=g["outer_ring_width"] * 1e3,
                   t_i_mm=round(g["t_i"] * 1e3, 2), t_o_mm=round(g["t_o"] * 1e3, 2),
                   c12_margin=round(min(g["t_i"], g["t_o"]) / (0.20 * g["D_we"]), 3),
                   L_eff_m=None if L_meas is None else round(L_meas, 5),
                   S_index=round(r["S"], 3),
                   mass_brg_kg=round(mb, 1), mass_shaft_kg=round(ms, 1),
                   mass_total_kg=round(2 * mb + ms, 1),
                   sigma_max_MPa=round(best, 1), governing=who,
                   feasible=1 if (0 < best < LIMIT) else 0,
                   warn="|".join(warn) if warn else "",
                   t_s=round(time.perf_counter() - t1, 2))
        for k_, v_ in sig.items():
            row[k_] = None if v_ is None else round(v_, 1)
        w.writerow(row); f.flush()
        if n % 500 == 0 or n == len(todo):        # 문서 §8-4.3 실시간 갱신
            try:
                update_p1_table.main((time.time() - t0) / 60.0)
            except Exception as e:
                print("  [warn] 문서 갱신 실패:", str(e).splitlines()[0][:60])
        if n <= 10 or n % 50 == 0 or n == len(todo):
            el = time.time() - t0
            eta = el / n * (len(todo) - n) / 3600
            print(f"  [{n}/{len(todo)}] S#{r['S_rank']} "
                  f"D_pw{g['D_pw']*1e3:.0f} a{g['alpha_deg']:.0f} "
                  f"Dwe{g['D_we']*1e3:.0f} Lwe{g['L_we']*1e3:.0f} "
                  f"z{r['z1']:.1f}/{r['z2']:.1f} -> {best:7.1f} MPa "
                  f"{'OK' if best < LIMIT else '  '} "
                  f"({row['t_s']:.1f}s, ETA {eta:.2f}h)", flush=True)
    f.close()
    print(f"\n[완료] {len(todo):,}점 · {(time.time()-t0)/3600:.2f} h")
    print(f"[저장] {GRID_CSV}")


if __name__ == "__main__":
    main(probe="--probe" in sys.argv)
