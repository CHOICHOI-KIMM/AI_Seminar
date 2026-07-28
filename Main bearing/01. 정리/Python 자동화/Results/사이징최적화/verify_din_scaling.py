"""
부록 2-6 검증 — DIN Lundberg 의 L_we 무관성이 문제가 되는가
=============================================================
설계공간 모서리에서 크라우닝(end_drop)을 DIN 값 대비 배율로 스윕하고,
지배 극한 4 LC 최대응력이 최소가 되는 지점과 DIN 값의 거리를 측정한다.

Johns-Gohar 의 end_drop 은 design_load 에 정확히 선형이므로(부록 2-2 실측),
설계마다 Qd 1점으로 비례상수를 보정한 뒤 목표 end_drop 을 직접 겨냥한다.
"""
import csv
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
sys.path.insert(0, ROOT)

import masta_clr_legacy  # noqa: F401,E402
import mastapy  # noqa: E402
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design  # noqa: E402
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType  # noqa: E402
from mastapy.bearings import RollerBearingProfileTypes as RP  # noqa: E402

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
OUT = os.path.join(HERE, "부록1_롤러프로파일", "verify_din_scaling.csv")

# 지배 극한 4 LC (부록 1-5.3) — 이름: 축토크 [kNm]
GOV = {"Myz_max": 22673.0, "My_min": 27453.0, "Mz_min": 17013.0, "Mz_max": 10308.0}
DIN_RATIO = 0.00221          # end_drop / D_we (부록 2-6 실측)
SWEEP = [0.25, 0.5, 0.75, 1.0, 1.5, 2.0, 3.0]   # DIN 대비 배율

# 설계 모서리: (라벨, D_we[m], L_we[m], D_pw[m], z1[m], z2[m])
CORNERS = [
    ("C0 기준",         0.11051, 0.238048, 3.3309, 0.5, 3.0),
    ("C1 L_we최대",     0.11051, 0.500,    3.3309, 0.5, 3.0),
    ("C2 D_we최대",     0.200,   0.238048, 3.3309, 0.5, 3.0),
    ("C3 대형조합",     0.200,   0.500,    4.5000, 0.5, 3.0),
    ("C4 L_we+스팬",    0.11051, 0.500,    3.3309, 0.4, 4.5),
]
COSA = math.cos(math.radians(19.0))


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


def end_drop(det):
    pts = list(safe(det, "inner_race_and_roller_profiles"))
    return max(float(safe(q, "total_deviation")) for q in pts) * 1e6   # µm


def set_geom(bs, sh, D_we, L_we, D_pw, z1, z2):
    t_i, t_o = 0.025674 * D_pw, 0.024654 * D_pw
    d = round((D_pw - D_we * COSA - 2 * t_i) * 1000) / 1000
    D = round((D_pw + D_we * COSA + 2 * t_o) * 1000) / 1000
    Z = int(0.92 * math.pi * D_pw / D_we)
    T = L_we + 0.072
    warn = []
    for b in bs:
        det = safe(b, "detail")
        for k, v in (("element_diameter", D_we), ("roller_length", L_we),
                     ("bore", d), ("outer_diameter", D),
                     ("inner_ring_width", T - 0.010), ("outer_ring_width", T - 0.057),
                     ("width", T), ("number_of_elements", Z)):
            try:
                setattr(det, k, v)
            except Exception as e:
                warn.append(f"{k}:{str(e).splitlines()[0][:30]}")
        try:
            det.pitch_circle_diameter = D_pw
        except Exception:
            pass
    # 샤프트 길이 먼저 늘린 뒤 위치 이동
    if z2 > 3.0:
        try:
            sh.length = z2 + 0.5
        except Exception as e:
            warn.append(f"shaft.length:{str(e).splitlines()[0][:40]}")
    for b in bs:
        z = z1 if "UW" in str(b) else z2
        try:
            b.set_position_of_component_and_connected_components((z, 0.0, 0.0))
        except Exception as e:
            warn.append(f"pos:{str(e).splitlines()[0][:40]}")
    return d, D, Z, T, warn


def sigma_max(asm, bs, lcmap):
    """지배 4 LC · 2 베어링 최대 접촉응력 [MPa] 과 지배 케이스"""
    best, who = 0.0, None
    for nm, lc in lcmap.items():
        sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
        sd.perform_analysis()
        for b in bs:
            v = sc(sd.results_for(b).component_detailed_analysis, "maximum_normal_stress")
            if v is None:
                continue
            v /= 1e6
            if v > best:
                best, who = v, f"{nm}/{'UW' if 'UW' in str(b) else 'DW'}"
    return best, who


def main():
    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    bs = list(asm.all_parts_of_type_bearing())
    sh = list(asm.all_parts_of_type_shaft())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    lcmap = {}
    for nm, tq in GOV.items():
        lc = next(c for c in asm.design_properties.static_loads if c.name == nm)
        q = lc.inputs_for_power_load(ipl)
        for a, v in (("speed", 0.0), ("torque", tq * 1e3)):
            try:
                setattr(q, a, v)
            except Exception:
                pass
        lcmap[nm] = lc
    print(f"[검증] 모서리 {len(CORNERS)}개 × 배율 {len(SWEEP)}개 × 지배 4 LC")

    rows = []
    for label, D_we, L_we, D_pw, z1, z2 in CORNERS:
        d, D, Z, T, warn = set_geom(bs, sh, D_we, L_we, D_pw, z1, z2)
        if warn:
            print(f"  [{label}] 경고: {warn[:3]}")
        det = safe(bs[0], "detail")
        # DIN 기준
        for b in bs:
            safe(safe(b, "detail"), "roller_profile_set").active_profile_type = RP.DIN_LUNDBERG
        din_drop = end_drop(det)
        din_sig, din_who = sigma_max(asm, bs, lcmap)
        print(f"\n[{label}] D_we={D_we*1e3:.1f} L_we={L_we*1e3:.1f} D_pw={D_pw*1e3:.0f} "
              f"z={z1}/{z2} Z={Z} d={d*1e3:.0f} D={D*1e3:.0f}")
        print(f"   DIN  end_drop {din_drop:8.2f} um (ratio {din_drop/(D_we*1e6):.5f})"
              f"  sigma {din_sig:8.1f} MPa  [{din_who}]")
        # Johns-Gohar 배율 스윕 — Qd 1점 보정 후 목표 end_drop 겨냥
        for b in bs:
            safe(safe(b, "detail"), "roller_profile_set").active_profile_type = RP.JOHNS_GOHAR
        ap0 = safe(safe(det, "roller_profile_set"), "active_profile")
        ap0.design_load = 1.0e6
        k_lin = end_drop(det) / 1.0e6          # µm per N (선형)
        print(f"   {'배율':>6} {'목표drop':>10} {'실제drop':>10} {'Qd[MN]':>8} "
              f"{'sigma':>9}  지배")
        best = (1e9, None, None)
        for r in SWEEP:
            tgt = din_drop * r
            qd = tgt / k_lin
            for b in bs:
                safe(safe(safe(b, "detail"), "roller_profile_set"),
                     "active_profile").design_load = qd
            act = end_drop(det)
            sig, who = sigma_max(asm, bs, lcmap)
            print(f"   {r:6.2f} {tgt:10.2f} {act:10.2f} {qd/1e6:8.2f} {sig:9.1f}  {who}")
            rows.append(dict(corner=label, D_we_mm=D_we*1e3, L_we_mm=L_we*1e3,
                             D_pw_mm=D_pw*1e3, z1=z1, z2=z2, Z=Z,
                             ratio_vs_DIN=r, end_drop_um=act,
                             drop_over_Dwe=act/(D_we*1e6),
                             drop_over_Lwe=act/(L_we*1e6),
                             design_load_MN=qd/1e6, sigma_MPa=sig, governing=who))
            if sig < best[0]:
                best = (sig, r, act)
        pen = (din_sig / best[0] - 1) * 100
        print(f"   -> 최적 배율 {best[1]:.2f} (end_drop {best[2]:.2f} um, "
              f"sigma {best[0]:.1f} MPa) · DIN 손해 {pen:+.2f}%")
        rows.append(dict(corner=label, D_we_mm=D_we*1e3, L_we_mm=L_we*1e3,
                         D_pw_mm=D_pw*1e3, z1=z1, z2=z2, Z=Z,
                         ratio_vs_DIN="DIN", end_drop_um=din_drop,
                         drop_over_Dwe=din_drop/(D_we*1e6),
                         drop_over_Lwe=din_drop/(L_we*1e6),
                         design_load_MN="", sigma_MPa=din_sig, governing=din_who))

    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader(); w.writerows(rows)
    print(f"\n[저장] {OUT}")


if __name__ == "__main__":
    main()
