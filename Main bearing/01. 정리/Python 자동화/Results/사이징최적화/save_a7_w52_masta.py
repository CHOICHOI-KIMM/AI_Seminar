"""
부록 7-6.7.6 — 두께 규칙 적용 설계의 MASTA 파일 저장
=======================================================
§7-6.7.6 표의 프론트 상위 설계를 `.masta` 로 남긴다.

저장되는 상태
  · 베어링 제원은 `s3_pareto.csv` 기록값(정수화 포함) 그대로
  · 샤프트 내경은 **두께 규칙** `ID = floor((OD⁴ − 32·W·OD/π)^¼)` · W = 1.393e9
  · 지배 LC `Myz_max` 에 토크 22,673 kNm · speed 0 을 넣은 상태
  · 해석 결과는 제외 (`save(path, False)` — 기존 관행)

**이 설계들은 σ 2,100 MPa 를 넘는다**(§7-6.7.6). 두께 규칙을 사후 적용하면
어떻게 되는지 보기 위한 파일이지 채택안이 아니다.

사용법
  python save_a7_w52_masta.py 1 2
"""
import csv
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg               # noqa: E402
import run_appendix7_shaft as a7       # noqa: E402
import run_appendix7_front52 as f52    # noqa: E402

SRC = f52.SRC
OUTDIR = os.path.join(HERE, "부록7_샤프트", "MASTA")
GOV, TQ = f52.GOV, f52.TQ


def main(ranks):
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design
    from mastapy.bearings import RollerBearingProfileTypes as RP

    with open(SRC, encoding="utf-8-sig") as f:
        F = {int(r["rank_pareto"]): r for r in csv.DictReader(f)
             if r["subset"] == "z1>=1.0"}
    missing = [n for n in ranks if n not in F]
    if missing:
        raise SystemExit(f"프론트에 없는 순번: {missing}")

    os.makedirs(OUTDIR, exist_ok=True)
    d = Design.load(a7.MODEL)
    asm = d.all_parts_of_type_root_assembly()[0]
    sh = list(asm.all_parts_of_type_shaft())[0]
    bs = list(asm.all_parts_of_type_bearing())
    uw = [b for b in bs if "UW" in str(b)][0]
    dw = [b for b in bs if "DW" in str(b)][0]
    for b in bs:
        b.detail.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG
    dp = asm.design_properties
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    lc = next(c for c in dp.static_loads if c.name == GOV)
    q = lc.inputs_for_power_load(ipl)
    for a, v in (("speed", 0.0), ("torque", TQ * 1e3)):
        try:
            setattr(q, a, v)
        except Exception:
            pass
    print(f"[LC] {GOV} 토크 {TQ:,.0f} kNm · speed 0")

    for n in ranks:
        r = F[n]
        z1, z2 = float(r["z1"]), float(r["z2"])
        od = float(r["bore_mm"])
        idm = f52.new_id(od)
        g = dict(D_pw=float(r["D_pw_mm"]) / 1e3, alpha_deg=float(r["alpha"]),
                 D_we=float(r["D_we_mm"]) / 1e3,
                 L_we=float(r["L_we_mm"]) / 1e3, bore=od / 1e3,
                 outer_diameter=float(r["D_mm"]) / 1e3,
                 width=float(r["T_mm"]) / 1e3,
                 inner_ring_width=float(r["B_mm"]) / 1e3,
                 outer_ring_width=float(r["C_mm"]) / 1e3,
                 number_of_elements=int(float(r["Z"])))
        for b in bs:
            try:
                if b.inner_connection is not None:
                    b.inner_connection.delete()
            except Exception:
                pass
        sh.remove_all_sections()
        sh.add_section(0.0, z2 + sg.SHAFT_TAIL, od / 1e3, idm / 1e3,
                       od / 1e3, idm / 1e3)
        for b in bs:
            sg.apply_to_masta(b.detail, g)
        for b, z in ((uw, z1), (dw, z2)):
            b.try_mount_on(sh, z)

        p = os.path.join(OUTDIR, f"A7_W52_front{n:02d}.masta")
        try:
            st = d.save(p, False)
            ok = os.path.isfile(p)
            print(f"  #{n:2} OD {od:,.0f} · ID {math.floor(od*sg.ID_OVER_OD):,.0f}"
                  f"→{idm:,.0f} · t {(od-idm)/2:.1f} · D_we {float(r['D_we_mm']):.1f}"
                  f" · z {z1}/{z2} → {os.path.basename(p)}"
                  f" {os.path.getsize(p)/1e6:.1f} MB" if ok else f"  !! 실패 {st}")
        except Exception as e:
            print(f"  !! #{n} 저장 실패: {str(e).splitlines()[0][:70]}")
    print(f"\n[저장 위치] {OUTDIR}")


if __name__ == "__main__":
    main([int(x) for x in sys.argv[1:]] or [1, 2])
