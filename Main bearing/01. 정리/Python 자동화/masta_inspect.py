"""
MASTA 모델 구조 자동탐색(Discovery) 스크립트
=============================================
목적: 배치 스크립트의 미확정 부분(로드케이스 경로 / 베어링 목록 / 베어링 수명
      결과 필드명)을 사용자 PC에서 '한 번' 실행해 자동으로 알아냅니다.

사용법:
  1) 아래 MASTA_DIR, MODEL_PATH 두 줄만 본인 환경에 맞게 수정
  2) 커맨드라인에서:  py masta_inspect.py > inspect_report.txt
  3) 생성된 inspect_report.txt 를 확인(또는 저에게 붙여넣기)하면
     masta_batch.py 의 TODO 를 정확한 코드로 확정합니다.

* mastapy 는 리플렉션으로 실제 객체를 조사하므로, 제가 API 이름을 몰라도
  이 스크립트가 설치 버전의 정확한 속성/경로를 찾아 출력합니다.
* 모델을 수정/저장하지 않는 '읽기 전용' 조사 스크립트입니다.
"""

import traceback

# ── 사용자 수정 구역 ──────────────────────────────────────
MASTA_DIR  = r"C:\Program Files\SMT\MASTA 14.1.1"   # ← 실제 설치 경로
MODEL_PATH = r"D:\AI\AI_Seminar\Main bearing\02. 자료\260720 검증용 자료_롤러 프로파일\26MW_메인베어링_기본설계_v1.2_샤프트 두께,형상 2안_강체,롤러프로파일만_260720.Masta"
# ─────────────────────────────────────────────────────────

import masta_clr_legacy  # noqa: F401  ← import mastapy 前에: CLR을 Legacy V2로 선기동(HASP 혼합모드 로드용)
import mastapy
mastapy.init(MASTA_DIR)
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType


def hr(t=""):
    print("\n" + "=" * 70)
    if t:
        print(t)
        print("-" * 70)


def public_attrs(obj, max_items=60):
    """객체의 호출불가 public 속성명과 값(짧게)을 나열."""
    out = []
    for name in dir(obj):
        if name.startswith("_"):
            continue
        try:
            v = getattr(obj, name)
        except Exception:
            continue
        if callable(v):
            continue
        s = repr(v)
        if len(s) > 60:
            s = s[:57] + "..."
        out.append((name, type(v).__name__, s))
        if len(out) >= max_items:
            break
    return out


def try_paths(root, paths):
    """여러 후보 속성경로를 시도해 (경로문자열, 값) 중 성공한 것 반환."""
    found = []
    for p in paths:
        cur = root
        ok = True
        for part in p.split("."):
            try:
                cur = getattr(cur, part)
            except Exception:
                ok = False
                break
        if ok and cur is not None:
            try:
                n = len(list(cur))
                found.append((p, f"컬렉션 (len={n})", cur))
            except TypeError:
                found.append((p, type(cur).__name__, cur))
    return found


def main():
    hr("1. 모델 로드")
    design = Design.load(MODEL_PATH)
    print("Design:", getattr(design, "name", design))
    assembly = design.all_parts_of_type_root_assembly()[0]
    print("Root assembly:", getattr(assembly, "name", assembly))

    # 2. 부품/베어링 목록 -----------------------------------
    hr("2. 베어링 목록")
    bearings = []
    for getter in ("all_parts_of_type_bearing",
                   "all_parts_of_type_rolling_bearing",
                   "bearings"):
        g = getattr(assembly, getter, None)
        try:
            items = list(g() if callable(g) else g) if g else []
        except Exception:
            items = []
        if items:
            print(f"  [{getter}] → {len(items)}개")
            for b in items:
                print("     -", getattr(b, "name", b))
            bearings = items
            break
    if not bearings:
        print("  베어링 접근자를 못 찾음. assembly 의 all_parts_of_type_* 후보:")
        for n in dir(assembly):
            if n.startswith("all_parts_of_type"):
                print("     ?", n)

    # 3. 로드케이스 경로 자동탐색 ---------------------------
    hr("3. 로드케이스 / 듀티사이클 경로")
    dp = getattr(assembly, "design_properties", None) or design
    candidates = [
        "design_states", "duty_cycles", "load_cases", "static_loads",
        "design_properties.design_states", "design_properties.duty_cycles",
        "design_properties.static_loads",
    ]
    hits = try_paths(assembly, candidates) + try_paths(design, candidates)
    load_case = None
    for path, kind, val in hits:
        print(f"  [OK] {path}  →  {kind}")
        try:
            first = list(val)[0]
            print("        first item:", getattr(first, "name", first))
            # design_state 안의 load_cases 까지 한 단계 더
            for sub in ("load_cases", "static_loads", "load_case"):
                s = getattr(first, sub, None)
                if s:
                    lcs = list(s)
                    if lcs:
                        print(f"        → {path}[0].{sub} : {len(lcs)}개 "
                              f"(예: {getattr(lcs[0],'name',lcs[0])})")
                        load_case = load_case or lcs[0]
            if load_case is None and hasattr(first, "analysis_of"):
                load_case = first  # first item 자체가 load case일 수도
        except Exception as e:
            print("        (열거 실패:", e, ")")
    if not hits:
        print("  후보 경로 실패. design_properties 의 public 속성:")
        for n, t, s in public_attrs(dp, 80):
            print(f"     {n}: {t} = {s}")

    # 4. System Deflection 실행 + 결과 필드명 ---------------
    hr("4. System Deflection 결과 필드 (베어링 수명 확인)")
    if load_case is None:
        print("  로드케이스를 확정 못 해 해석을 건너뜀. 3번 출력 확인 필요.")
    else:
        print("  사용할 load case:", getattr(load_case, "name", load_case))
        try:
            sd = load_case.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            sd.perform_analysis()
            print("  results_ready:", getattr(sd, "results_ready", "?"))
            if bearings:
                res = sd.results_for(bearings[0])
                print(f"\n  베어링 '{getattr(bearings[0],'name','?')}' 결과 객체:",
                      type(res).__name__)
                print("  ── 사용 가능한 결과 속성 (이 이름을 batch 스크립트에 사용) ──")
                for n, t, s in public_attrs(res, 120):
                    # 수명/손상 관련만 강조
                    mark = "  ★" if any(k in n.lower() for k in
                            ("life", "l10", "damage", "reliab", "rating",
                             "safety", "load", "force")) else ""
                    print(f"     {n}: {t} = {s}{mark}")

                # 4-2. 상세 결과(수명/정격) 객체 파고들기 ----------
                life_kw = ("life", "l10", "l_10", "damage", "reliab", "rating",
                           "iso", "din", "fatigue", "adjust", "modification",
                           "static", "dynamic", "equivalent", "hours",
                           "revolution", "safety")
                detail = getattr(res, "component_detailed_analysis", None)
                if detail is not None:
                    hr(f"4-2. component_detailed_analysis: {type(detail).__name__}")
                    for n, t, s in public_attrs(detail, 250):
                        mark = "  ★" if any(k in n.lower() for k in life_kw) else ""
                        print(f"     {n}: {t} = {s}{mark}")
                    # 수명/정격 관련 '중첩 객체'를 한 단계 더 덤프
                    for n in dir(detail):
                        if n.startswith("_"):
                            continue
                        try:
                            v = getattr(detail, n)
                        except Exception:
                            continue
                        if callable(v) or v is None:
                            continue
                        tn = type(v).__name__
                        if any(k in tn.lower() for k in
                               ("life", "rating", "iso", "reliab", "damage")):
                            hr(f"4-2-nested: detail.{n} → {tn}")
                            for nn, tt, ss in public_attrs(v, 120):
                                m2 = "  ★" if any(k in nn.lower()
                                                  for k in life_kw) else ""
                                print(f"       {nn}: {tt} = {ss}{m2}")
        except Exception:
            print("  해석/결과 조사 중 예외:")
            traceback.print_exc()

    # 5. AnalysisType 목록 ---------------------------------
    hr("5. 사용 가능한 AnalysisType 멤버")
    for n in dir(AnalysisType):
        if n.isupper():
            print("   -", n)

    hr("완료")
    print("이 출력을 저장/공유해 주시면 batch 스크립트를 확정합니다.")


if __name__ == "__main__":
    main()
