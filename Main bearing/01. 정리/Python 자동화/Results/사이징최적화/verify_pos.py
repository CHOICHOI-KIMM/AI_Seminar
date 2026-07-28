"""베어링 축방향 위치 이동 시험 + 소켓 오프셋 기준 확인"""
import os, sys, math
HERE=os.path.dirname(os.path.abspath(__file__)); RES=os.path.dirname(HERE); ROOT=os.path.dirname(RES)
sys.path.insert(0,ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
MODEL=(r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
       r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
       r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
def safe(o,n):
    try: return getattr(o,n)
    except Exception as e: return f"<ERR {str(e).splitlines()[0][:40]}>"
d=Design.load(MODEL); asm=d.all_parts_of_type_root_assembly()[0]
bs=list(asm.all_parts_of_type_bearing()); sh=list(asm.all_parts_of_type_shaft())[0]
dw=[b for b in bs if "DW" in str(b)][0]
print("현재 위치:", safe(dw,"position"))
print("socket_offset 관련:")
for n in ("available_socket_offsets","socket_offset","offset_type","inner_socket","outer_socket"):
    print(f"   {n} = {safe(dw,n)}")
conn=safe(dw,"inner_connection")
for n in ("socket_a","socket_b","drawing_position"):
    s=safe(conn,n)
    print(f"   conn.{n} = {s}")
    if s is not None and not isinstance(s,str):
        for m in dir(s):
            if m.startswith("_") or callable(safe(s,m)): continue
            if any(k in m.lower() for k in ("offset","position","name")):
                print(f"       {m} = {safe(s,m)}")

print("\n[시험] set_position_of_component_and_connected_components(3.5, 0, 0)")
import inspect
f=safe(dw,"set_position_of_component_and_connected_components")
try: print("  시그니처:", inspect.signature(f))
except Exception as e: print("  시그니처 조회 실패:", str(e)[:60])
for args in ([3.5,0.0,0.0], [(3.5,0.0,0.0)],):
    try:
        f(*args) if isinstance(args[0],float) else f(args[0])
        print(f"  호출 {args} -> 성공, 위치 = {safe(dw,'position')}")
        break
    except Exception as e:
        print(f"  호출 {args} 실패: {str(e).splitlines()[0][:80]}")
try:
    from mastapy.math_utility import Vector3D
    f(Vector3D(3.5,0.0,0.0)); print("  Vector3D 호출 성공, 위치 =", safe(dw,"position"))
except Exception as e:
    print("  Vector3D 시도 실패:", str(e).splitlines()[0][:80])
print("샤프트 길이:", safe(sh,"length"))
print("완료")
