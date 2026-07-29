"""
sigma=0 원인 진단 (P1 선행)
============================
P1 격자의 약 1/3 이 최대응력 0 으로 반환되는 원인을 규명한다.
프로브 3점에서 T/D_pw ≳ 0.124 가 의심되나 근거가 빈약하다.

  A. 경계 스캔   : D_pw 3종 × L_we 미세 스윕 → 실패 경계 정밀 측정
  B. 심층 해부   : 실패 1점에서 어느 단계가 None 인지 추적 + 형상 유효성 조회
  C. 교차 확인   : alpha · D_we 가 경계를 이동시키는지

출력: P1_극한응력/diag_sigma_zero.csv
"""
import csv
import math
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
OUTDIR = os.path.join(HERE, "P1_극한응력")
OUT = os.path.join(OUTDIR, "diag_sigma_zero.csv")
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg   # noqa: E402

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
GOV = {"Myz_max": 22673.0, "My_min": 27453.0, "Mz_min": 17013.0}

Z1, Z2 = 0.5, 4.0
# A: 경계 스캔 — L_we 20 mm 간격
SCAN_DPW = [3.3309, 3.900, 4.500]
SCAN_LWE = [round(0.200 + 0.020 * i, 3) for i in range(19)]     # 200~560
SCAN_DWE, SCAN_AL = 0.170, 24.0
# C: 교차 확인
CROSS = ([(al, 0.170) for al in (12.0, 18.0, 30.0)]
         + [(24.0, d) for d in (0.110, 0.140, 0.200)])
CROSS_DPW = 3.900


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


class Rig:
    """모델 1회 로드 후 제원 주입·해석 반복"""

    def __init__(self, mastapy, Design, AnalysisType, RP):
        self.AT = AnalysisType
        self.design = Design.load(MODEL)
        asm = self.design.all_parts_of_type_root_assembly()[0]
        self.asm = asm
        self.sh = list(asm.all_parts_of_type_shaft())[0]
        self.bs = list(asm.all_parts_of_type_bearing())
        self.uw = [b for b in self.bs if "UW" in str(b)][0]
        self.dw = [b for b in self.bs if "DW" in str(b)][0]
        self.ipl = next(p for p in asm.all_parts_of_type_power_load()
                        if "input" in str(p).lower())
        for b in self.bs:
            b.detail.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG
        self.dp = asm.design_properties
        self.tpl = {}
        for nm, tq in GOV.items():
            lc = next(c for c in self.dp.static_loads if c.name == nm)
            q = lc.inputs_for_power_load(self.ipl)
            for a_, v_ in (("speed", 0.0), ("torque", tq * 1e3)):
                try:
                    setattr(q, a_, v_)
                except Exception:
                    pass
            self.tpl[nm] = lc
        self.ds = self.tpl["Myz_max"].design_state_load_case_group
        self.n = 0

    def apply(self, dpw, al, dwe, lwe, z1=Z1, z2=Z2):
        g = sg.bearing(dpw, al, dwe, lwe)
        warn = []
        for b in self.bs:
            try:
                if b.inner_connection is not None:
                    b.inner_connection.delete()
            except Exception as e:
                warn.append("unmount:" + str(e).splitlines()[0][:25])
        s = sg.shaft(g["bore"], z2)
        try:
            self.sh.remove_all_sections()
            self.sh.add_section(0.0, s["length"], s["outer_diameter"],
                                s["inner_diameter"], s["outer_diameter"],
                                s["inner_diameter"])
        except Exception as e:
            warn.append("shaft:" + str(e).splitlines()[0][:30])
        for b in self.bs:
            bad = sg.apply_to_masta(b.detail, g)
            if bad:
                warn.append("spec:" + bad[0])
        for b, z in ((self.uw, z1), (self.dw, z2)):
            try:
                b.try_mount_on(self.sh, z)
            except Exception as e:
                warn.append("mount:" + str(e).splitlines()[0][:25])
        for b, tg in ((self.uw, "UW"), (self.dw, "DW")):
            if safe(b, "is_mounted") is not True:
                warn.append(tg + "미장착")
        return g, warn

    def solve(self, deep=False):
        """해석 후 (sigma_max, 단계별 진단) 반환"""
        self.n += 1
        names, dups = [], []
        for nm, tq in GOV.items():
            lc = self.tpl[nm].duplicate(self.ds, f"dg_{self.n}_{nm}")
            q = lc.inputs_for_power_load(self.ipl)
            for a_, v_ in (("speed", 0.0), ("torque", tq * 1e3)):
                try:
                    setattr(q, a_, v_)
                except Exception:
                    pass
            names.append(nm)
            dups.append(lc)
        duty = self.dp.add_duty_cycle(f"dgdc_{self.n}")
        for lc in dups:
            duty.add_static_load(lc)
        diag = {}
        best, err = 0.0, ""
        try:
            csd = duty.analysis_of(self.AT.SYSTEM_DEFLECTION)
            csd.perform_analysis()
            for b, tag in ((self.uw, "UW"), (self.dw, "DW")):
                rr = list(csd.results_for(b))
                diag[tag + "_nres"] = len(rr)
                if not rr:
                    continue
                subs = list(rr[0].component_analysis_cases)
                diag[tag + "_nsub"] = len(subs)
                for nm, sub in zip(names, subs):
                    cda = safe(sub, "component_detailed_analysis")
                    if tag == "UW" and nm == "Myz_max":
                        diag["cda_type"] = type(cda).__name__ if cda else "None"
                    v = sc(cda, "maximum_normal_stress")
                    if v and v / 1e6 > best:
                        best = v / 1e6
        except Exception as e:
            err = str(e).splitlines()[0][:70]
        for x in dups + [duty]:
            try:
                x.delete()
            except Exception:
                pass
        return best, diag, err

    def detail_probe(self):
        """베어링 detail 의 형상·유효성 관련 값 수집"""
        d = self.uw.detail
        keys = ["width", "bore", "outer_diameter", "inner_ring_width",
                "outer_ring_width", "element_diameter", "roller_length",
                "pitch_circle_diameter", "number_of_elements",
                "effective_centre_from_front_face", "contact_angle",
                "element_taper_angle", "inner_race_cone_angle",
                "outer_race_cone_angle", "cup_angle", "cone_angle",
                "element_effective_length", "roller_effective_length",
                "basic_dynamic_load_rating", "basic_static_load_rating",
                "mass", "cage_bridge_width", "element_surface_velocity"]
        out = {}
        for k in keys:
            v = sc(d, k)
            if v is not None:
                out[k] = v
        msgs = []
        for k in ("geometry_validity", "is_valid", "validity", "warnings",
                  "design_warnings", "geometry_warnings"):
            v = safe(d, k)
            if v is not None:
                msgs.append(f"{k}={str(v)[:60]}")
        out["_msgs"] = " | ".join(msgs)
        return out


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
    from mastapy.bearings import RollerBearingProfileTypes as RP

    os.makedirs(OUTDIR, exist_ok=True)
    rig = Rig(mastapy, Design, AnalysisType, RP)
    rows = []
    t0 = time.time()

    # ── A. 경계 스캔 ──
    print("=" * 78)
    print("A. 경계 스캔  (D_we 170 · alpha 24 · z 0.5/4.0)")
    print("=" * 78)
    print(f"{'D_pw':>7}{'L_we':>7}{'T':>8}{'T/D_pw':>9}{'B/L_ax':>8}"
          f"{'Z':>5}{'sigma':>10}  판정")
    bound = {}
    for dpw in SCAN_DPW:
        prev_ok = None
        for lwe in SCAN_LWE:
            g, warn = rig.apply(dpw, SCAN_AL, SCAN_DWE, lwe)
            sig, diag, err = rig.solve()
            T, r = g["width"], g["width"] / dpw
            L_ax = lwe * math.cos(math.radians(SCAN_AL))
            ok = sig > 0
            if prev_ok is True and not ok and dpw not in bound:
                bound[dpw] = (prev_lwe, lwe)
            prev_ok, prev_lwe = ok, lwe
            rows.append(dict(phase="A", D_pw_mm=dpw * 1e3, alpha=SCAN_AL,
                             D_we_mm=SCAN_DWE * 1e3, L_we_mm=lwe * 1e3,
                             T_mm=round(T * 1e3, 1), T_over_Dpw=round(r, 5),
                             B_over_Lax=round(g["inner_ring_width"] / L_ax, 4),
                             Z=g["number_of_elements"],
                             sigma_max=round(sig, 1), ok=int(ok),
                             warn="|".join(warn), err=err,
                             cda=diag.get("cda_type", "")))
            print(f"{dpw*1e3:7.0f}{lwe*1e3:7.0f}{T*1e3:8.1f}{r:9.4f}"
                  f"{g['inner_ring_width']/L_ax:8.3f}{g['number_of_elements']:5d}"
                  f"{sig:10.1f}  {'OK' if ok else 'FAIL ' + (err or diag.get('cda_type',''))}",
                  flush=True)
        print()
    print("[경계]", {f"{k*1e3:.0f}": f"{v[0]*1e3:.0f}~{v[1]*1e3:.0f}mm"
                     for k, v in bound.items()})

    # ── B. 심층 해부 ──
    print("\n" + "=" * 78)
    print("B. 심층 해부  (성공점 vs 실패점 detail 비교)")
    print("=" * 78)
    pairs = []
    dpw = SCAN_DPW[1]
    lo, hi = bound.get(dpw, (0.300, 0.420))
    for tag, lwe in (("성공", lo), ("실패", hi)):
        rig.apply(dpw, SCAN_AL, SCAN_DWE, lwe)
        det = rig.detail_probe()
        sig, diag, err = rig.solve(deep=True)
        det.update(_tag=tag, _lwe=lwe * 1e3, _sigma=round(sig, 1),
                   _err=err, _diag=str(diag)[:120])
        pairs.append(det)
    keys = sorted(set(pairs[0]) | set(pairs[1]))
    print(f"{'속성':38}{'성공':>16}{'실패':>16}")
    for k in keys:
        a, b = pairs[0].get(k), pairs[1].get(k)
        if isinstance(a, float) and isinstance(b, float):
            d = "" if abs(a - b) < 1e-9 else "  <<<"
            print(f"{k:38}{a:16.5f}{b:16.5f}{d}")
        else:
            print(f"{k:38}{str(a)[:16]:>16}{str(b)[:16]:>16}")

    # ── C. 교차 확인 ──
    print("\n" + "=" * 78)
    print(f"C. 교차 확인  (D_pw {CROSS_DPW*1e3:.0f} 고정, 경계 부근 L_we 스윕)")
    print("=" * 78)
    lo2, hi2 = bound.get(CROSS_DPW, (0.300, 0.420))
    lws = [round(lo2 - 0.02 + 0.02 * i, 3) for i in range(4)]
    print(f"{'alpha':>7}{'D_we':>7}" + "".join(f"{int(x*1e3):>9}" for x in lws))
    for al, dwe in CROSS:
        cells = []
        for lwe in lws:
            g, _ = rig.apply(CROSS_DPW, al, dwe, lwe)
            sig, diag, err = rig.solve()
            cells.append("OK" if sig > 0 else "FAIL")
            rows.append(dict(phase="C", D_pw_mm=CROSS_DPW * 1e3, alpha=al,
                             D_we_mm=dwe * 1e3, L_we_mm=lwe * 1e3,
                             T_mm=round(g["width"] * 1e3, 1),
                             T_over_Dpw=round(g["width"] / CROSS_DPW, 5),
                             Z=g["number_of_elements"],
                             sigma_max=round(sig, 1), ok=int(sig > 0), err=err))
        print(f"{al:7.0f}{dwe*1e3:7.0f}" + "".join(f"{c:>9}" for c in cells),
              flush=True)

    keys = []
    for r in rows:
        for k in r:
            if k not in keys:
                keys.append(k)
    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=keys)
        w.writeheader()
        w.writerows(rows)
    print(f"\n[저장] {OUT}  ({len(rows)}행 · {(time.time()-t0)/60:.1f}분)")


if __name__ == "__main__":
    main()
