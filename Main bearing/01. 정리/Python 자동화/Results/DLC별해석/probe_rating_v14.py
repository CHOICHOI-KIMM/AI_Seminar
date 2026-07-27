"""부록 5 Step 0 보조 2 — 베어링 카탈로그 설계객체(C·Cu·Z·롤러치수) 접근자 탐색.
해석 불필요(모델 로드만) — Bearing 파트의 design/detail 계열 속성을 훑는다."""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)

import masta_clr_legacy  # noqa: F401,E402
import mastapy  # noqa: E402
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design  # noqa: E402

MODELS = {
    "v1.4 프리로드(신규)": (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
                      r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
                      r"_베어링 크기 확대_롤러 확대_프리로드 적용_온도_50도_260726.Masta"),
    "v1.3 50도(기준선)": (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
                     r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안"
                     r"_피로하중 반영_유연체_FE_온도_50도_260721.Masta"),
}

WANT = ("rating", "fatigue", "limit", "number_of", "element", "roller",
        "pitch", "diameter", "length", "angle", "width", "row", "cu")


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception as e:
        return f"<ERR {str(e).splitlines()[0][:45]}>"


def dump(obj, title):
    print(f"\n  --- {title}  ({type(obj).__name__}) ---")
    for name in sorted(dir(obj)):
        if name.startswith("_"):
            continue
        if not any(k in name.lower() for k in WANT):
            continue
        v = safe(obj, name)
        if callable(v):
            continue
        s = repr(v)
        if len(s) > 70:
            s = s[:67] + "..."
        print(f"    {name}: {type(v).__name__} = {s}")


for tag, path in MODELS.items():
    print("\n" + "#" * 78)
    print(f"### {tag}")
    print("#" * 78)
    design = Design.load(path)
    asm = design.all_parts_of_type_root_assembly()[0]
    for b in asm.all_parts_of_type_bearing():
        print(f"\n[{b}]")
        # 설계객체 후보 탐색
        cands = [n for n in dir(b)
                 if not n.startswith("_")
                 and any(k in n.lower() for k in ("detail", "design", "catalog"))]
        print(f"  설계객체 후보 속성: {cands}")
        for n in cands:
            v = safe(b, n)
            if isinstance(v, str) or v is None or callable(v):
                print(f"  {n}: {v if isinstance(v, str) else type(v).__name__} (건너뜀)")
                continue
            dump(v, f"b.{n}")

print("\n완료")
