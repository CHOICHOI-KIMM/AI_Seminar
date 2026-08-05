"""
S5-0 — 온도 설정 위치 탐색 (§6-11.7 준비)
============================================
현재 모델은 파일명이 `…_온도_50도_…` 이고 스크리닝 상수도 `NU50 = 294.637`
로 박혀 있다. **70 °C 로 바꾸려면 정확히 어디를 건드려야 하는지**를 먼저
확정한다 — 한 곳이라도 놓치면 50 °C 와 70 °C 가 섞인 해석이 된다.

찾는 것
  ① MASTA 모델 안에서 온도를 들고 있는 객체·속성 (베어링·윤활·하중조건)
  ② 그 온도가 실제로 점도(ν)·κ 로 이어지는 경로
  ③ 결과 객체에서 읽을 수 있는 점도·κ (검증용)

읽기만 한다 — 아무것도 바꾸지 않는다.
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
KEYS = ("temperature", "viscosity", "lubric", "oil", "grease", "kappa",
        "viscosit")


def hits(obj, label, seen=None):
    """온도·점도 관련 속성만 골라 값과 함께 출력"""
    out = []
    for n in dir(obj):
        if n.startswith("_"):
            continue
        if not any(k in n.lower() for k in KEYS):
            continue
        try:
            v = getattr(obj, n)
        except Exception as e:
            out.append((n, f"<err {str(e).splitlines()[0][:40]}>"))
            continue
        if callable(v):
            continue
        if isinstance(v, (int, float, str, bool)) or v is None:
            out.append((n, v))
        else:
            w = getattr(v, "value", None)
            out.append((n, f"{type(v).__name__}"
                           + (f" = {w}" if isinstance(w, (int, float)) else "")))
    if out:
        print(f"\n[{label}] {type(obj).__name__}")
        for n, v in out:
            print(f"    {n:52} {v}")
    return out


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design

    d = Design.load(MODEL)
    asm = d.all_parts_of_type_root_assembly()[0]
    bs = list(asm.all_parts_of_type_bearing())
    uw = [b for b in bs if "UW" in str(b)][0]
    dp = asm.design_properties

    print(f"[모델] {os.path.basename(MODEL)}")
    hits(d, "Design")
    hits(asm, "RootAssembly")
    hits(dp, "DesignProperties")
    hits(uw, "Bearing(UW) 부품")
    hits(uw.detail, "Bearing(UW).detail")
    for nm in ("lubrication_detail", "lubrication", "bearing_settings"):
        o = getattr(uw.detail, nm, None) or getattr(d, nm, None)
        if o is not None and not isinstance(o, (int, float, str)):
            hits(o, f"detail.{nm}")

    lc = next(iter(dp.static_loads), None)
    if lc is not None:
        hits(lc, f"StaticLoadCase '{lc.name}'")
        try:
            blc = lc.bearings_load_case if hasattr(lc, "bearings_load_case") \
                else None
            if blc:
                hits(blc, "LoadCase.bearings")
        except Exception:
            pass
        # 베어링별 하중조건 객체
        for attr in ("load_case_for", "bearing_load_cases"):
            f = getattr(lc, attr, None)
            if callable(f):
                try:
                    hits(f(uw), f"LoadCase.{attr}(UW)")
                except Exception:
                    pass

    # ── 온도 집합 상세 (여기가 실제 설정 지점) ──────────────────────
    def dump(o, label):
        print(f"\n[{label}] {type(o).__name__}")
        for n in sorted(dir(o)):
            if n.startswith("_"):
                continue
            try:
                v = getattr(o, n)
            except Exception:
                continue
            if callable(v):
                continue
            if isinstance(v, (int, float, str, bool)) or v is None:
                print(f"    {n:46} {v}")
                continue
            try:                                  # 래핑된 스칼라만 풀어본다
                w = v.value
            except Exception:
                w = None
            print(f"    {n:46} " + (f"{w}  <{type(v).__name__}>" if
                                    isinstance(w, (int, float)) else
                                    f"<{type(v).__name__}>"))

    dump(d.default_system_temperatures, "Design.default_system_temperatures")
    if lc is not None:
        dump(lc.temperatures, f"LoadCase '{lc.name}'.temperatures")
        print(f"\n  use_default_temperatures = {lc.use_default_temperatures}")
        print(f"  → 하중조건이 **자체 온도**를 쓴다"
              if not lc.use_default_temperatures else
              "  → 하중조건이 설계 기본 온도를 쓴다")

    # 윤활유 상세 (점도-온도 관계가 여기 있다)
    ld = uw.overridden_lubrication_detail
    print(f"\n  Bearing.override_design_lubrication_detail = "
          f"{uw.override_design_lubrication_detail}")
    dump(ld, "Bearing.overridden_lubrication_detail")

    # 하중조건 개수와 온도 사용 현황
    lcs = list(dp.static_loads)
    ud = sum(1 for c in lcs if getattr(c, "use_default_temperatures", None))
    print(f"\n[요약] 정적 하중조건 {len(lcs)}개 · "
          f"설계기본온도 사용 {ud}개 · 자체온도 {len(lcs)-ud}개")
    for c in lcs[:6]:
        t = c.temperatures
        print(f"    '{c.name}' use_default={c.use_default_temperatures} · "
              f"element {t.rolling_bearing_element} · inner "
              f"{t.rolling_bearing_inner_race} · outer "
              f"{t.rolling_bearing_outer_race} · shaft {t.shaft}")

    # ── 실제로 쓰이는 윤활 = 결과에서 읽는다 (설계 레벨 객체는 접근 불가) ──
    from mastapy.system_model.analyses_and_results.static_loads import (
        AnalysisType)
    duty = dp.add_duty_cycle("probe_temp_dc")
    duty.add_static_load(lc)
    csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
    csd.perform_analysis()
    sub = list(list(csd.results_for(uw))[0].component_analysis_cases)[0]
    det = sub.component_detailed_analysis
    print("\n[결과에서 읽은 윤활·온도 (UW · Load Case 1)]")
    for n in sorted(dir(det)):
        if n.startswith("_") or not any(k in n.lower() for k in
                                        ("viscos", "temperature", "kappa",
                                         "lubric", "film")):
            continue
        try:
            v = getattr(det, n)
        except Exception:
            continue
        if isinstance(v, (int, float)):
            print(f"    {n:56} {v}")
    duty.delete()


if __name__ == "__main__":
    main()
