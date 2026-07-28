"""기준 모델 — 하우징 FE 적용 여부 및 베어링 외륜 지지 방식 직접 확인"""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")

def safe(o, n):
    try: return getattr(o, n)
    except Exception as e: return f"<ERR {str(e).splitlines()[0][:45]}>"

d = Design.load(MODEL)
asm = d.all_parts_of_type_root_assembly()[0]

print("=== 전체 파트 ===")
for p in asm.all_parts():
    print(f"  {type(p).__name__:26} | {p}")

print("\n=== FE Part 상세 ===")
for fe in asm.all_parts_of_type_fe_part():
    print(f"  [{fe}]")
    for a in ("fe_substructures", "component_name", "mass", "is_housing",
              "material", "included_in_analysis", "is_active"):
        print(f"    {a} = {safe(fe, a)}")
    try:
        subs = list(safe(fe, "fe_substructures") or [])
        print(f"    substructure 수 = {len(subs)}")
        for s in subs:
            print(f"      - {s}  nodes={safe(s,'number_of_nodes')} "
                  f"reduced={safe(s,'is_reduced')} file={safe(s,'external_fe_file_path')}")
    except Exception as e:
        print("    substructure 조회 실패:", str(e)[:70])
    try:
        print(f"    connections = {[str(c) for c in fe.connections]}")
    except Exception as e:
        print("    connections 실패:", str(e)[:60])

print("\n=== 베어링 외륜 지지 ===")
for b in asm.all_parts_of_type_bearing():
    print(f"  [{b}]")
    for a in ("outer_component", "outer_connection", "inner_component",
              "inner_connection", "is_outer_race_grounded", "outer_socket"):
        print(f"    {a} = {safe(b, a)}")
    try:
        for c in b.connections:
            print(f"    연결: {safe(c,'owner_a')}  <->  {safe(c,'owner_b')}  ({type(c).__name__})")
    except Exception as e:
        print("    연결 조회 실패:", str(e)[:60])

print("\n=== 전체 연결 ===")
try:
    for c in asm.all_connections():
        print(f"  {type(c).__name__:34} {safe(c,'owner_a')} <-> {safe(c,'owner_b')}")
except Exception as e:
    print("  실패:", str(e)[:70])
print("완료")
