"""
G-8.2c: 베어링 반력 영향계수 [T] 동정 (MASTA 단위하중 7회, 1회성 특성화)
========================================================================
배경: 핀지지 정역학은 베어링 틸팅모멘트 반력을 무시하여 부정확(UW +29%, DW +81%).
      실제 계는 부정정(베어링 회전강성 의존)이나, 반력은 hub 하중의 **완전 선형**
      (§C-3.6 R²=1.000000) → 영향계수로 정확히 표현 가능.

  R_b,c = T[b][c][k]·u_k + R0_b,c        u = (F_X, F_Y, F_Z, M_X, M_Y, M_Z)
  R0 = 자중 등 하중 무관 항 (영하중 케이스에서 획득)

MASTA 7회(영하중 1 + 단위하중 6) → JSON 저장 → 이후 6001점은 순수 산술.
"""
import json
import math
import os

import masta_clr_legacy  # noqa: F401
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.3"
         r"_샤프트 두께,형상 2안_피로하중 반영_유연체_FE_260721.masta")
OUT = "c1_influence.json"
RPM2RADS = 2 * math.pi / 60
RPM_REF = 4.7581          # DLC 평균 회전수 (rpm) — 속도는 상수로 고정

INPUTS = ["F_X", "F_Y", "F_Z", "M_X", "M_Y", "M_Z"]
UNIT = {"F_X": 1e6, "F_Y": 1e6, "F_Z": 1e6, "M_X": 1e7, "M_Y": 1e7, "M_Z": 1e7}


def apply(lc, pl, ipl, u):
    p = lc.inputs_for_point_load(pl)
    p.force_x.force = u["F_X"]
    p.force_y.force = u["F_Y"]
    p.axial_load.force = u["F_Z"]
    p.moment_x.moment = u["M_X"]
    p.moment_y.moment = u["M_Y"]
    pll = lc.inputs_for_power_load(ipl)
    pll.speed = RPM_REF * RPM2RADS
    pll.torque = u["M_Z"]


def react(sd, bearings):
    out = {}
    for b in bearings:
        try:
            f = sd.results_for(b).internal_force
            out[str(b)] = [float(f.x), float(f.y), float(f.z)]
        except Exception:
            out[str(b)] = [float("nan")] * 3
    return out


def main():
    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    pl = list(asm.all_parts_of_type_point_load())[0]
    bearings = list(asm.all_parts_of_type_bearing())
    ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
    lc = next(c for c in asm.design_properties.static_loads
              if getattr(c, "name", "") == "Load Case 1")
    names = [str(b) for b in bearings]
    print(f"[베어링] {names}")

    def run(u):
        apply(lc, pl, ipl, u)
        sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
        sd.perform_analysis()
        return react(sd, bearings)

    zero = {k: 0.0 for k in INPUTS}
    print("\n[0] 영하중 (자중 등 상수항 R0)")
    R0 = run(zero)
    for n in names:
        print(f"    {n:20} R0 = [{R0[n][0]/1e3:9.2f}, {R0[n][1]/1e3:9.2f}, "
              f"{R0[n][2]/1e3:9.2f}] kN")

    T = {n: {c: {} for c in "XYZ"} for n in names}
    for k in INPUTS:
        u = dict(zero); u[k] = UNIT[k]
        R = run(u)
        print(f"\n[{k}] 단위 {UNIT[k]:.0e}")
        for n in names:
            for ci, c in enumerate("XYZ"):
                T[n][c][k] = (R[n][ci] - R0[n][ci]) / UNIT[k]
            print(f"    {n:20} dR = [{(R[n][0]-R0[n][0])/1e3:9.2f}, "
                  f"{(R[n][1]-R0[n][1])/1e3:9.2f}, {(R[n][2]-R0[n][2])/1e3:9.2f}] kN")

    data = {"model": os.path.basename(MODEL), "rpm_ref": RPM_REF,
            "inputs": INPUTS, "bearings": names,
            "R0": R0, "T": T}
    json.dump(data, open(OUT, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    print(f"\n[저장] {OUT}")

    # 물리 점검
    print("\n[점검] 힘평형 — 각 입력에 대한 두 베어링 반력합 계수")
    for k in INPUTS:
        for c in "XYZ":
            s = sum(T[n][c][k] for n in names)
            if abs(s) > 1e-3:
                print(f"    Σ dR_{c}/d{k} = {s:+.4f}")


if __name__ == "__main__":
    main()
