"""
부록 10 · 검토용 예시 MASTA 파일 생성
=====================================
부록 8 모델(50°C)에 시계열 index 0~9 (t=0.0~0.9 s) 10개 로드케이스를 생성·입력하고
전용 duty cycle 'Batch_10' 으로 묶어 **새 파일로 저장** (원본은 건드리지 않음).
케이스명: batch_pt<i>_t<시간>s
"""
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)

import masta_clr_legacy  # noqa: F401,E402
import mastapy           # noqa: E402
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design  # noqa: E402

SRC = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
       r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안"
       r"_피로하중 반영_유연체_FE_온도_50도_260721.Masta")
OUT = os.path.join(HERE, "배치예시_10케이스_260722.Masta")
RPM2RADS = 2 * math.pi / 60
N = 10

import c1_pin  # noqa: E402
data = c1_pin.parse_dlc(c1_pin.DLC)

design = Design.load(SRC)
asm = design.all_parts_of_type_root_assembly()[0]
dp = asm.design_properties
pl = list(asm.all_parts_of_type_point_load())[0]
ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
ds = lc0.design_state_load_case_group
print(f"원본 로드: {os.path.basename(SRC)}")
print(f"DesignState: {ds.name!r} · 기존 static_loads {len(list(dp.static_loads))}개")

made = []
for i in range(N):
    rec = data[i]
    name = f"batch_pt{i}_t{rec['t']:.1f}s".replace("-0.0", "0.0")
    lc = lc0.duplicate(ds, name)
    p = lc.inputs_for_point_load(pl)
    p.force_x.force = -rec["Fz"] * 1e3
    p.force_y.force = rec["Fy"] * 1e3
    p.axial_load.force = rec["Fx"] * 1e3
    p.moment_x.moment = -rec["Mz"] * 1e3
    p.moment_y.moment = rec["My"] * 1e3
    pw = lc.inputs_for_power_load(ipl)
    pw.speed = rec["rpm"] * RPM2RADS
    pw.torque = rec["Mx"] * 1e3
    made.append(lc)
    print(f"  [{i}] {name:20}  F_X={-rec['Fz']*1e3:12,.0f} N  "
          f"M_X={-rec['Mz']*1e3:14,.0f} N·m  rpm={rec['rpm']:.4f}")

duty = dp.add_duty_cycle("Batch_10")
for lc in made:
    duty.add_static_load(lc)
print(f"\nduty cycle 'Batch_10' 구성: {duty.number_of_load_cases:.0f}케이스")
print(f"최종 static_loads: {len(list(dp.static_loads))}개 (기존 17 + 신규 {len(made)})")

# 새 파일로 저장 (save 메서드 시그니처 확인 후 호출)
import inspect  # noqa: E402
for m in ("save", "save_as", "save_to"):
    if hasattr(design, m):
        try:
            print(f"design.{m}{inspect.signature(getattr(design, m))}")
        except Exception:
            print(f"design.{m}(...)")
saved = False
for call in (lambda: design.save(OUT, False),):
    try:
        call()
        saved = True
        break
    except Exception as e:
        print("  저장 시도 실패:", str(e).splitlines()[0][:70])
print(f"\n[저장] {'성공' if saved else '실패'}: {OUT}")
if saved and os.path.exists(OUT):
    print(f"  파일 크기: {os.path.getsize(OUT)/1e6:.1f} MB")
