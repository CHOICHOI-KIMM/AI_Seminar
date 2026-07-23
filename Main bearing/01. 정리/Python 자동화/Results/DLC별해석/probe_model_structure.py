"""부록 1 결론 재검토용: 50°C 모델의 구조 확인
— FE 파트(샤프트/하우징), 베어링 외륜 지지(ground 여부), 연결 관계"""
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)

import masta_clr_legacy  # noqa: F401,E402
import mastapy  # noqa: E402
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안"
         r"_피로하중 반영_유연체_FE_온도_50도_260721.Masta")


def safe(o, n):
    try:
        v = getattr(o, n)
        return v
    except Exception as e:
        return f"<ERR {str(e).splitlines()[0][:40]}>"


design = Design.load(MODEL)
asm = design.all_parts_of_type_root_assembly()[0]

print("=== 전체 파트 목록 (타입별) ===")
for p in asm.all_parts():
    print(f"  {type(p).__name__:28} | {p}")

print("\n=== FE 파트 ===")
try:
    fes = list(asm.all_parts_of_type_fe_part())
    for fe in fes:
        print(f"  {fe}")
        for attr in ("fe_substructures", "is_housing", "component_name"):
            print(f"    {attr} = {safe(fe, attr)}")
except Exception as e:
    print("  조회 실패:", e)

print("\n=== 샤프트 FE 치환 여부 ===")
for sh in asm.all_parts_of_type_shaft():
    print(f"  {sh}: is_replaced_by_fe = {safe(sh, 'is_replaced_by_fe')}")

print("\n=== 베어링 외륜 지지 ===")
for b in asm.all_parts_of_type_bearing():
    print(f"  [{b}]")
    for attr in ("is_outer_race_grounded", "outer_race_mounting",
                 "mounting_options"):
        print(f"    {attr} = {safe(b, attr)}")
    try:
        for conn in b.connections:
            oa = safe(conn, "owner_a")
            ob = safe(conn, "owner_b")
            print(f"    연결: {oa}  <->  {ob}  ({type(conn).__name__})")
    except Exception as e:
        print("    연결 조회 실패:", e)

print("\n완료")
