"""
§6-11.7 Step 0 — 베어링 70 °C 의 점도 실측
=============================================
`Load Case 1` 의 베어링 온도 3항(element·inner_race·outer_race)만 50 → 70 으로
바꾸고 해석해 **MASTA 가 산출하는 동점도**를 읽는다. 스크리닝 상수 `NU70` 은
이 값을 쓴다 — `NU50 = 294.637` 이 같은 경로에서 얻어졌음이 확인됐으므로
(결과의 `kinematic_viscosity` 2.9464e-4 m²/s) 동일 경로가 정합적이다.

**모델은 저장하지 않는다.** 온도 변경은 메모리 안에서만 일어난다.

확인 항목
  ① 온도 3항이 실제로 반영되는가 (`element_temperature` 가 70 인가)
  ② ν(70) [mm²/s]
  ③ 복제 하중조건이 온도를 상속하는가 — 본해석 파이프라인과 같은 경로
"""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
T_NEW = 70.0
FIELDS = ("element_temperature", "kinematic_viscosity", "dynamic_viscosity",
          "lubricant_film_temperature", "oil_sump_temperature")


def read(det):
    return {n: getattr(det, n, None) for n in FIELDS}


def show(tag, d):
    nu = d["kinematic_viscosity"]
    print(f"  [{tag}] element {d['element_temperature']} °C · "
          f"ν {nu*1e6:,.3f} mm²/s · η {d['dynamic_viscosity']:.6f} Pa·s · "
          f"film {d['lubricant_film_temperature']} °C")
    return nu * 1e6


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import (
        AnalysisType)

    d = Design.load(MODEL)
    asm = d.all_parts_of_type_root_assembly()[0]
    uw = [b for b in asm.all_parts_of_type_bearing() if "UW" in str(b)][0]
    dp = asm.design_properties
    lc = next(c for c in dp.static_loads if c.name == "Load Case 1")
    ds = lc.design_state_load_case_group

    def run(tag, lc_use):
        duty = dp.add_duty_cycle(f"nu70_{tag}")
        duty.add_static_load(lc_use)
        csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
        csd.perform_analysis()
        sub = list(list(csd.results_for(uw))[0].component_analysis_cases)[0]
        out = read(sub.component_detailed_analysis)
        duty.delete()
        return show(tag, out)

    print("[1] 변경 전 (모델 그대로)")
    nu50 = run("50 °C", lc)

    print(f"\n[2] Load Case 1 의 베어링 3항만 {T_NEW:.0f} °C 로")
    t = lc.temperatures
    print(f"  변경 전: element {t.rolling_bearing_element} · "
          f"inner {t.rolling_bearing_inner_race} · "
          f"outer {t.rolling_bearing_outer_race} · shaft {t.shaft}")
    t.rolling_bearing_element = T_NEW
    t.rolling_bearing_inner_race = T_NEW
    t.rolling_bearing_outer_race = T_NEW
    print(f"  변경 후: element {t.rolling_bearing_element} · "
          f"inner {t.rolling_bearing_inner_race} · "
          f"outer {t.rolling_bearing_outer_race} · shaft {t.shaft} (유지)")
    nu70 = run(f"{T_NEW:.0f} °C", lc)

    print(f"\n[3] 복제 하중조건이 온도를 상속하는가 "
          f"(본해석 파이프라인과 같은 경로)")
    dup = lc.duplicate(ds, "nu70_dup")
    td = dup.temperatures
    print(f"  복제본: use_default {dup.use_default_temperatures} · "
          f"element {td.rolling_bearing_element}")
    nud = run("복제본", dup)
    dup.delete()

    print(f"\n[결과] ν(50) {nu50:,.3f} → ν(70) {nu70:,.3f} mm²/s "
          f"({100*(nu70-nu50)/nu50:+.1f}%)")
    print(f"  κ ∝ ν 이므로 κ 도 같은 비율로 낮아진다 → a_ISO 하락 → 손상 증가")
    print(f"  복제본 일치: {'예' if abs(nud-nu70) < 1e-6 else '아니오 — 확인 필요'}")
    print(f"\n  run_p2_screen.py 에 넣을 값:  NU70 = {nu70:.3f}")


if __name__ == "__main__":
    main()
