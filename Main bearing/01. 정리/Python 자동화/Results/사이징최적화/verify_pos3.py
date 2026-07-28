"""베어링 개별 축위치 변경 경로 탐색 — mount_on / 소켓 오프셋 / 샤프트 단면"""
import os, sys, inspect
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
    except Exception as e: return f"<ERR {str(e).splitlines()[0][:45]}>"
d=Design.load(MODEL); asm=d.all_parts_of_type_root_assembly()[0]
bs=list(asm.all_parts_of_type_bearing()); sh=list(asm.all_parts_of_type_shaft())[0]
uw=[b for b in bs if "UW" in str(b)][0]

print("=== mount_on / try_mount_on 시그니처 ===")
for n in ("mount_on","try_mount_on"):
    f=safe(uw,n)
    try: print(f"  {n}{inspect.signature(f)}")
    except Exception as e: print(f"  {n} 조회실패 {str(e)[:50]}")

print("\n=== Shaft 파트 속성 (길이/단면/설계) ===")
for n in sorted(dir(sh)):
    if n.startswith("_"): continue
    if not any(k in n.lower() for k in ("length","section","design","body","outer","inner","profile","diameter")): continue
    v=safe(sh,n)
    if callable(v): 
        try: print(f"  {n}{inspect.signature(v)}  [method]")
        except Exception: print(f"  {n}()  [method]")
        continue
    s=repr(v)
    print(f"  {n} = {s[:80]}")

print("\n=== 샤프트 연결/소켓 오프셋 ===")
try:
    for c in sh.connections:
        print(f"  conn {type(c).__name__}: {safe(c,'owner_a')} <-> {safe(c,'owner_b')}")
        for sk in ("socket_a","socket_b"):
            s=safe(c,sk)
            if s is None or isinstance(s,str): continue
            offs=[n for n in dir(s) if not n.startswith("_") and "offset" in n.lower()]
            print(f"    {sk}={s}  offset속성={offs}")
            for n in offs: print(f"       {n} = {safe(s,n)}")
except Exception as e: print("  실패", str(e)[:70])

print("\n=== ShaftDesign 후보 ===")
for n in ("active_definition","design","shaft_body","component_design","detail"):
    v=safe(sh,n)
    print(f"  sh.{n} = {type(v).__name__ if not isinstance(v,str) else v}")
    if not isinstance(v,str) and v is not None:
        cands=[m for m in dir(v) if not m.startswith("_") and
               any(k in m.lower() for k in ("length","section","outer","inner","diameter"))]
        print(f"     후보: {cands[:25]}")
print("완료")
