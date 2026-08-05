"""
부록 7 Step 0 — 샤프트 DIN 743 평가 경로 탐색
================================================
`Myz_max` 하중조건에서 샤프트 피로안전율을 어떻게 꺼내는지 확인한다.
읽기만 하고 모델은 저장하지 않는다.

찾는 것
  ① 설계에 걸린 샤프트 정격 방법(DIN 743 / FKM …)과 재질·표면 설정
  ② 해석 결과에서 안전율을 들고 있는 객체·속성
  ③ `Myz_max` 의 토크 입력 상태 (P1 은 power load 로 22,673 kNm 를 넣었다)
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
GOV, TQ = "Myz_max", 22673.0
KEYS = ("din", "safety", "fatigue", "rating", "material", "surface",
        "notch", "stress", "factor")


def hits(o, label, extra=()):
    out = []
    for n in dir(o):
        if n.startswith("_"):
            continue
        low = n.lower()
        if not (any(k in low for k in KEYS) or n in extra):
            continue
        try:
            v = getattr(o, n)
        except Exception as e:
            out.append((n, f"<err {str(e).splitlines()[0][:34]}>"))
            continue
        if callable(v):
            continue
        if isinstance(v, (int, float, str, bool)) or v is None:
            out.append((n, v))
        else:
            out.append((n, f"<{type(v).__name__}>"))
    if out:
        print(f"\n[{label}] {type(o).__name__}")
        for n, v in out:
            print(f"    {n:52} {v}")


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import (
        AnalysisType)

    d = Design.load(MODEL)
    asm = d.all_parts_of_type_root_assembly()[0]
    sh = list(asm.all_parts_of_type_shaft())[0]
    dp = asm.design_properties
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())

    print(f"[모델] {os.path.basename(MODEL)}")
    print(f"[샤프트] {sh}")
    hits(sh, "Shaft 부품", ("length", "mass_of_shaft_body"))
    for nm in ("shaft_rating_method", "shaft_settings", "material",
               "material_database_selector"):
        o = getattr(sh, nm, None) or getattr(d, nm, None)
        if o is not None and not isinstance(o, (int, float, str, bool)):
            hits(o, f"·{nm}")

    lc = next(c for c in dp.static_loads if c.name == GOV)
    q = lc.inputs_for_power_load(ipl)
    print(f"\n[LC {GOV}] use_default_temp {lc.use_default_temperatures} · "
          f"베어링 {lc.temperatures.rolling_bearing_element} °C")
    for a in ("torque", "speed", "target_torque", "power"):
        print(f"    power_load.{a:14} {getattr(q, a, '<없음>')}")

    # 토크를 넣고 해석 (P1 과 같은 경로)
    for a, v in (("speed", 0.0), ("torque", TQ * 1e3)):
        try:
            setattr(q, a, v)
        except Exception:
            pass
    duty = dp.add_duty_cycle("din743_probe")
    duty.add_static_load(lc)
    csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
    csd.perform_analysis()
    res = list(csd.results_for(sh))[0]
    print(f"\n[결과 객체] {type(res).__name__}")
    hits(res, "Shaft 결과")
    for sub in list(getattr(res, "component_analysis_cases", []) or [])[:1]:
        hits(sub, "component_analysis_cases[0]")
        print(f"\n  shaft_rating_method = {sub.shaft_rating_method}")
        for nm in ("shaft_section_end_with_worst_fatigue_safety_factor",
                   "shaft_section_end_with_worst_fatigue_safety_factor"
                   "_for_infinite_life",
                   "shaft_section_end_with_worst_static_safety_factor"):
            o = getattr(sub, nm, None)
            if o is None:
                continue
            print(f"\n  ── {nm}")
            for a in sorted(dir(o)):
                if a.startswith("_"):
                    continue
                try:
                    v = getattr(o, a)
                except Exception:
                    continue
                if isinstance(v, (int, float, str, bool)):
                    print(f"      {a:46} {v}")
        sf = getattr(sub, "safety_factors", None)
        if sf is not None:
            print("\n  ── safety_factors")
            for a in sorted(dir(sf)):
                if a.startswith("_"):
                    continue
                try:
                    v = getattr(sf, a)
                except Exception:
                    continue
                if isinstance(v, (int, float, str, bool)):
                    print(f"      {a:46} {v}")
        for nm in ("shaft_section_results",):
            items = list(getattr(sub, nm, []) or [])
            if not items:
                continue
            s0 = items[0]
            print(f"\n  ── {nm}[0] = {type(s0).__name__}")
            for a in sorted(dir(s0)):
                if a.startswith("_"):
                    continue
                try:
                    v = getattr(s0, a)
                except Exception:
                    continue
                if isinstance(v, (int, float, str, bool)):
                    print(f"      {a:46} {v}")
                elif not callable(v) and v is not None:
                    print(f"      {a:46} <{type(v).__name__}>")
            for side in ("left_end", "right_end"):
                e = getattr(s0, side, None)
                if e is None:
                    continue
                print(f"\n  ── {nm}[0].{side} = {type(e).__name__}")
                for a in sorted(dir(e)):
                    if a.startswith("_"):
                        continue
                    try:
                        v = getattr(e, a)
                    except Exception:
                        continue
                    if isinstance(v, (int, float, str, bool)):
                        print(f"      {a:46} {v}")
        for nm in ("sections", "shaft_rating"):
            o = getattr(sub, nm, None)
            if o is None:
                continue
            try:
                items = list(o)
                print(f"\n  {nm}: {len(items)}개")
                if items:
                    hits(items[0], f"{nm}[0]")
            except TypeError:
                hits(o, nm)
    duty.delete()


if __name__ == "__main__":
    main()
