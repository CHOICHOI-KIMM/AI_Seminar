"""
MASTA 메인베어링 배치 결과 추출
================================
지정한 .masta 모델을 열어, 모든 정적 로드케이스(static_loads) × 베어링에 대해
System Deflection 해석을 수행하고, 베어링 수명/정격/응력 결과를 CSV로 저장한다.

* 외부 파이썬(mastapy 14.1.1)용. HASP 혼합모드 문제는 masta_clr_legacy 가 해결.
* 필드명은 masta_inspect.py 의 조사 결과(inspect_report.txt)로 확정한 것.

실행:  PYTHONUTF8=1 python masta_batch.py
출력:  batch_results.csv  (스크립트와 같은 폴더, UTF-8-BOM = 엑셀 한글 호환)

주의(수명 해석):
  이 모델의 정적 로드케이스(예: Mx_max)는 회전속도=0 인 '극한하중' 케이스라
  수명-시간(*_time)이 inf 로 나온다. 실사용 L10h(시간 수명)가 필요하면 회전속도가
  있는 듀티사이클 로드케이스를 사용해야 한다(RUN_DUTY_CYCLE 참고).
"""
import csv
import math
import os
import traceback

# ── 사용자 수정 구역 ──────────────────────────────────────
MASTA_DIR  = r"C:\Program Files\SMT\MASTA 14.1.1"
MODEL_PATH = r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA\26MW_메인베어링_기본설계_v1.2_샤프트 두께,형상 2안_강체,롤러프로파일만_모멘트 수정_260720.Masta"
OUT_CSV    = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                          "batch_results_모멘트수정.csv")
# ─────────────────────────────────────────────────────────

import masta_clr_legacy  # noqa: F401  ← import mastapy 前: CLR Legacy V2 선기동
import mastapy
mastapy.init(MASTA_DIR)
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType


def bname(b):
    """Bearing 표시명. .name 속성이 없으면 str(b) 폴백."""
    n = getattr(b, "name", None)
    return n if isinstance(n, str) else str(b)


def g(obj, path, default=None):
    """'a.b.c' 속성 체인을 안전하게 읽는다. 실패 시 default."""
    cur = obj
    for part in path.split("."):
        try:
            cur = getattr(cur, part)
        except Exception:
            return default
        if cur is None:
            return default
    return cur


# 추출할 필드: (CSV 컬럼명, "detail 로부터의 속성경로")
FIELDS = [
    # 기본
    ("num_elements_in_contact",    "number_of_elements_in_contact"),
    ("axial_to_radial_load_ratio", "axial_to_radial_load_ratio"),
    ("max_normal_stress_inner_Pa", "maximum_normal_stress_inner"),
    ("max_normal_stress_outer_Pa", "maximum_normal_stress_outer"),
    # ISO 281:2007  (피로수명)
    ("P_dynamic_equiv_load_N",     "iso2812007.dynamic_equivalent_load"),
    ("L10_basic_life_cycles",      "iso2812007.basic_rating_life_cycles"),
    ("L10_basic_life_time_h",      "iso2812007.basic_rating_life_time"),
    ("L10m_modified_life_cycles",  "iso2812007.modified_rating_life_cycles"),
    ("L10m_modified_life_time_h",  "iso2812007.modified_rating_life_time"),
    ("a1_reliability_factor",      "iso2812007.life_modification_factor_for_reliability"),
    ("aISO_systems_factor",        "iso2812007.life_modification_factor_for_systems_approach"),
    ("basic_life_safety_factor",   "iso2812007.basic_rating_life_safety_factor"),
    # ISO 76:2006  (정적 안전율)
    ("P0_static_equiv_load_N",     "iso762006.static_equivalent_load"),
    ("s0_static_safety_factor",    "iso762006.safety_factor"),
    # ISO/TS 16281:2008  (수정 기준 정격수명 - 롤러)
    ("Cr_dyn_load_rating_inner_N", "isots162812008.load_for_the_basic_dynamic_load_rating_of_the_inner_ring_or_shaft_washer"),
    ("Cr_dyn_load_rating_outer_N", "isots162812008.load_for_the_basic_dynamic_load_rating_of_the_outer_ring_or_housing_washer"),
    ("ref_basic_life_cycles",      "isots162812008.basic_reference_rating_life_cycles"),
    ("ref_basic_life_time_h",      "isots162812008.basic_reference_rating_life_time"),
    ("ref_modified_life_cycles",   "isots162812008.modified_reference_rating_life_cycles"),
    ("ref_modified_life_time_h",   "isots162812008.modified_reference_rating_life_time"),
]


def find_load_cases(design, assembly):
    """정적 로드케이스 컬렉션을 여러 경로 후보로 탐색 (inspect 결과 기준)."""
    for root, path in [(assembly, "design_properties.static_loads"),
                       (design,   "static_loads"),
                       (assembly, "static_loads"),
                       (design,   "design_properties.static_loads")]:
        v = g(root, path)
        if v is None:
            continue
        try:
            lst = list(v)
        except TypeError:
            continue
        if lst:
            return lst
    return []


def num(v):
    """숫자/inf/nan/None 을 CSV 친화적으로."""
    if v is None:
        return ""
    if isinstance(v, float):
        if math.isinf(v):
            return "inf"
        if math.isnan(v):
            return "nan"
    return v


def main():
    print("모델 로드:", MODEL_PATH)
    design = Design.load(MODEL_PATH)
    assembly = design.all_parts_of_type_root_assembly()[0]

    bearings = list(assembly.all_parts_of_type_bearing())
    print(f"베어링 {len(bearings)}개:", [bname(b) for b in bearings])

    load_cases = find_load_cases(design, assembly)
    if not load_cases:
        raise RuntimeError("정적 로드케이스를 찾지 못함 (경로 후보 실패)")
    print(f"정적 로드케이스 {len(load_cases)}개:",
          [getattr(lc, "name", str(lc)) for lc in load_cases])

    header = ["load_case", "bearing", "detailed_result_type",
              "has_converged"] + [c for c, _ in FIELDS]
    rows = []

    for lc in load_cases:
        lc_name = getattr(lc, "name", str(lc))
        try:
            sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            sd.perform_analysis()
        except Exception:
            print(f"  [{lc_name}] 해석 실패:")
            traceback.print_exc()
            continue

        for b in bearings:
            res = sd.results_for(b)
            detail = g(res, "component_detailed_analysis")
            row = [lc_name, bname(b),
                   type(detail).__name__ if detail is not None else "",
                   num(g(res, "has_converged"))]
            for _, path in FIELDS:
                row.append(num(g(detail, path)))
            rows.append(row)
        print(f"  [{lc_name}] 완료 ({len(bearings)} 베어링)")

    with open(OUT_CSV, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.writer(f)
        w.writerow(header)
        w.writerows(rows)

    print(f"\n저장: {OUT_CSV}  ({len(rows)} 행)")
    print("주의: 정적 극한하중 케이스는 속도=0 이라 *_time 이 inf 일 수 있음"
          " (실사용 시간수명은 회전 듀티사이클 필요).")


if __name__ == "__main__":
    main()
