# -*- coding: utf-8 -*-
"""열팽창 관련 옵션이 어디에 있고 무엇으로 설정돼 있는지 훑는다"""
import os
import sys

H = (r"d:\AI\AI_Seminar\Main bearing\01. 정리\Python 자동화"
     r"\Results\사이징최적화")
sys.path.insert(0, H)
sys.path.insert(0, os.path.dirname(os.path.dirname(H)))

import run_p2_fatigue as p2  # noqa: E402

KEY = ("thermal", "expansion", "temperature", "temp")


def val(o, a):
    try:
        v = getattr(o, a)
    except Exception as e:
        return f"<ERR {str(e).splitlines()[0][:44]}>"
    return v


def scan(o, label, depth=0):
    try:
        names = sorted(dir(o))
    except Exception:
        return
    hits = []
    for a in names:
        if a.startswith("_"):
            continue
        if not any(k in a.lower() for k in KEY):
            continue
        v = val(o, a)
        if callable(v):
            continue
        hits.append((a, v))
    if hits:
        print(f"\n### {label}  ({type(o).__name__})")
        for a, v in hits:
            print(f"  {a:60s} = {v}")


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design

    d = Design.load(p2.MODEL)
    asm = d.all_parts_of_type_root_assembly()[0]
    dp = asm.design_properties
    sh = list(asm.all_parts_of_type_shaft())[0]
    bs = list(asm.all_parts_of_type_bearing())
    uw = [b for b in bs if "UW" in str(b)][0]
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    gov = next(c for c in dp.static_loads if c.name == "Myz_max")

    targets = [(d, "Design"), (dp, "design_properties"),
               (lc0, "LC 'Load Case 1'"), (gov, "LC 'Myz_max'"),
               (uw, "Bearing UW"), (uw.detail, "Bearing UW .detail"),
               (sh, "Shaft")]
    for o, lab in targets:
        scan(o, lab)

    # 설정 컨테이너들 — 이름에 settings 가 들어간 속성을 한 단계 더 판다
    for o, lab in list(targets):
        for a in sorted(dir(o)):
            if a.startswith("_") or "setting" not in a.lower():
                continue
            v = val(o, a)
            if v is None or isinstance(v, (str, int, float, bool)):
                continue
            scan(v, f"{lab} ▸ {a}")

    # 해석 설정 트리 (있으면)
    for a in ("analysis_settings", "system_deflection_settings",
              "bearing_settings", "design_settings"):
        for o, lab in ((d, "Design"), (dp, "design_properties")):
            v = val(o, a)
            if v is not None and not isinstance(v, str):
                scan(v, f"{lab}.{a}")


if __name__ == "__main__":
    main()
