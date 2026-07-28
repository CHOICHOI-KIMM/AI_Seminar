"""베어링 형상 이미지 저장 — 2D 단면도 + 3D 뷰 + 리포트 경로 확인"""
import os, sys
HERE=os.path.dirname(os.path.abspath(__file__)); RES=os.path.dirname(HERE); ROOT=os.path.dirname(RES)
sys.path.insert(0,ROOT)
import masta_clr_legacy  # noqa
import mastapy
mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
from mastapy.system_model import Design
MODEL=(r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
       r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
       r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
OUT=os.path.join(HERE,"_img_test"); os.makedirs(OUT,exist_ok=True)
IMGS=["two_d_drawing","two_d_drawing_full_model","three_d_view","three_d_isometric_view",
      "three_d_view_orientated_in_xy_plane_with_z_axis_pointing_into_the_screen",
      "three_d_view_orientated_in_xz_plane_with_y_axis_pointing_into_the_screen"]
d=Design.load(MODEL); asm=d.all_parts_of_type_root_assembly()[0]
b=list(asm.all_parts_of_type_bearing())[0]
print("[베어링 이미지]")
for n in IMGS:
    try:
        v=getattr(b,n)
        fp=os.path.join(OUT,f"bearing_{n}.png"); v.save(fp)
        print(f"  OK  {n:70} {str(v.size):>12}  {os.path.getsize(fp):,} B")
    except Exception as e:
        print(f"  실패 {n}: {str(e).splitlines()[0][:60]}")
print("\n[어셈블리 이미지]")
for n in ("two_d_drawing_full_model","three_d_isometric_view"):
    try:
        v=getattr(asm,n); fp=os.path.join(OUT,f"asm_{n}.png"); v.save(fp)
        print(f"  OK  {n:70} {str(v.size):>12}  {os.path.getsize(fp):,} B")
    except Exception as e:
        print(f"  실패 {n}: {str(e).splitlines()[0][:60]}")
print("\n[리포트 목록]")
print("  bearing:", getattr(b,"report_names",None))
print("  detail :", getattr(b.detail,"report_names",None))
print("\n[리포트 출력 시험 — Default CAD Report]")
for tgt,tag in ((b,"bearing"),(b.detail,"detail")):
    for rn in ("Default CAD Report","All Properties Report"):
        try:
            fp=os.path.join(OUT,f"{tag}_{rn.replace(' ','_')}")
            tgt.output_named_report_to(rn, fp)
            found=[f for f in os.listdir(OUT) if f.startswith(f"{tag}_{rn.replace(' ','_')}")]
            print(f"  OK  {tag}.{rn:24} -> {found[:3]}")
        except Exception as e:
            print(f"  실패 {tag}.{rn:24}: {str(e).splitlines()[0][:55]}")
