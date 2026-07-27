"""부록 5 Step 0 보조 — 신규 모델 베어링 객체 속성명 전수 덤프 (C·Cu·Dpw·e 탐색)"""
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)

import masta_clr_legacy  # noqa: F401,E402
import mastapy  # noqa: E402
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design  # noqa: E402
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType  # noqa: E402

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_프리로드 적용_온도_50도_260726.Masta")
RPM2RADS = 2 * math.pi / 60


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception as e:
        return f"<ERR {str(e).splitlines()[0][:50]}>"


def dump(obj, title, only=None):
    print(f"\n--- {title}  ({type(obj).__name__}) ---")
    for name in sorted(dir(obj)):
        if name.startswith("_"):
            continue
        if only and not any(k in name.lower() for k in only):
            continue
        v = safe(obj, name)
        if callable(v):
            continue
        s = repr(v)
        if len(s) > 80:
            s = s[:77] + "..."
        print(f"  {name}: {type(v).__name__} = {s}")


design = Design.load(MODEL)
asm = design.all_parts_of_type_root_assembly()[0]
pl = list(asm.all_parts_of_type_point_load())[0]
bearings = list(asm.all_parts_of_type_bearing())
ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
lc = next(c for c in asm.design_properties.static_loads
          if getattr(c, "name", "") == "Load Case 1")

p = lc.inputs_for_point_load(pl)
p.force_x.force = 4965600.0; p.force_y.force = 126450.0; p.axial_load.force = 3017070.0
p.moment_x.moment = -4814750.0; p.moment_y.moment = -13666500.0
lc.inputs_for_power_load(ipl).speed = 5.09 * RPM2RADS
sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
sd.perform_analysis()

KEY = ("rating", "fatigue", "limit", "diameter", "angle", "row", "number_of",
       "element", "roller", "pitch", "mean", "dynamic", "static", "cu", "e_",
       "clearance", "preload", "width", "length", "bore", "outer")

for b in bearings:
    print("\n" + "=" * 78)
    print(f"[{b}]")
    dump(b, "bearing part", KEY)
    det = safe(b, "bearing_detail") if not isinstance(safe(b, "bearing_detail"), str) else None
    if det is not None:
        dump(det, "b.bearing_detail (카탈로그 기하/정격)", KEY)
    d = sd.results_for(b).component_detailed_analysis
    dump(d, "detail (해석결과)", KEY)
    iso = safe(d, "iso2812007")
    if not isinstance(iso, str) and iso is not None:
        dump(iso, "detail.iso2812007", None)

print("\n완료")
