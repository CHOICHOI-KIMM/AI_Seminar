"""
부록 10 — 프론트 설계의 MASTA 파일 저장
==========================================
§10-11.2.2 표의 지정 순번을 `.masta` 로 남긴다.

저장되는 상태
  · 베어링 제원은 `a10_pareto.csv` 기록값(정수화 포함) 그대로
  · 샤프트 내경은 **두께 규칙** `ID = floor((d⁴ − 32·W·d/π)^¼)` · W = 1.393e9
  · 샤프트 길이는 `z2 + 0.5 m` — 설계마다 다르다(`z2` 가 변수다 · §10-3)
  · **롤러 코너 반경 좌·우 각 4.3 mm** 주입 (§8-3 ②) → `L_we = L_w − 8.6`
  · 지배 LC `Myz_max` 에 토크 22,673 kNm · speed 0 을 넣은 상태
  · 해석 결과는 제외 (`save(path, False)` — 기존 관행)

**부록 7·9 의 저장 스크립트와 다른 점은 코너 반경**이다. 부록 8 이래의 종속
변수이므로 빼면 유효 롤러 길이가 8.6 mm 길어져 다른 모델이 된다.

사용법
  python a10_save_masta.py 1 103 210            현행 e_off
  python a10_save_masta.py 1 103 210 offset     §10-11.3.6 신규
"""
import csv
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import a8_eval                         # noqa: E402
import sizing_geom as sg               # noqa: E402
import run_appendix7_shaft as a7       # noqa: E402

SRC = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "a10_pareto.csv")
OUTDIR = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "MASTA")
GOV, TQ = "Myz_max", 22673.0


# §10-11.3.6 규칙은 `a10_eoff_v2.rule_v2` 단일 소스를 쓴다
#   e_off = o_off - (D_we/2)*sin(alpha - beta)


def main(ranks, offset=False):
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design
    from mastapy.bearings import RollerBearingProfileTypes as RP

    with open(SRC, encoding="utf-8-sig") as f:
        F = {int(r["rank"]): r for r in csv.DictReader(f)}
    miss = [n for n in ranks if n not in F]
    if miss:
        raise SystemExit(f"프론트에 없는 순번: {miss}")

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
        od = float(r["bore_mm"])                       # 보어 = 샤프트 외경
        idm = a8_eval.shaft_id(od / 1e3) * 1e3         # 두께 규칙 + floor
        g = dict(D_pw=float(r["D_pw_mm"]) / 1e3, alpha_deg=float(r["alpha"]),
                 D_we=float(r["D_we_mm"]) / 1e3,
                 L_we=float(r["L_w_mm"]) / 1e3,        # MASTA 주입은 롤러 전장
                 bore=od / 1e3,
                 outer_diameter=float(r["D_mm"]) / 1e3,
                 width=float(r["T_mm"]) / 1e3,
                 inner_ring_width=float(r["B_mm"]) / 1e3,
                 outer_ring_width=float(r["C_mm"]) / 1e3,
                 number_of_elements=int(float(r["Z"])))

        for b in bs:                                   # 재장착 전 연결 제거
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
            for a in ("left_element_corner_radius",
                      "right_element_corner_radius"):
                setattr(b.detail, a, a8_eval.R_CORNER)  # §8-3 ②
        for b, z in ((uw, z1), (dw, z2)):
            b.try_mount_on(sh, z)

        e_new = None
        if offset:                                   # §10-11.3.6
            import a10_eoff_v2 as EV2
            e_new = EV2.rule_v2(uw.detail)[0] / 1e3  # [mm] -> [m]
            for b in bs:
                b.detail.element_offset = e_new

        det = uw.detail
        tag = "_offset" if offset else ""
        p = os.path.join(OUTDIR, f"A10_front{n:03d}{tag}.masta")
        try:
            d.save(p, False)
            ok = os.path.isfile(p)
            print(f"  #{n:<4} D {float(r['D_mm']):,.0f} · d {od:,.0f} · "
                  f"ID {idm:,.0f} · t {(od-idm)/2:.1f} · L {z2+0.5:.1f} m · "
                  f"L_we {det.effective_roller_length*1e3:.1f} · "
                  f"Z {int(det.number_of_elements)} · e_off "
                  f"{det.element_offset*1e3:.2f}"
                  + (f" (신규 · Δ{e_new*1e3-20.7416:+.2f})" if e_new
                     else " (현행)") + " → "
                  + (f"{os.path.basename(p)} {os.path.getsize(p)/1e6:.1f} MB"
                     if ok else "!! 저장 안 됨"))
        except Exception as e:
            print(f"  !! #{n} 저장 실패: {str(e).splitlines()[0][:70]}")
    print(f"\n[저장 위치] {OUTDIR}")


if __name__ == "__main__":
    av = [x for x in sys.argv[1:]]
    off = "offset" in av
    rk = [int(x) for x in av if x.isdigit()] or [1, 103, 210]
    main(rk, offset=off)
