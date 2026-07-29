"""
MASTA 조립폭 T 거동 실측 (부록 4-7 미결 #1)
==============================================
후속안은 B·C 를 입력하고 T 를 MASTA 자동값으로 쓰려 한다. 성립 여부를 실측한다.

  A. width 관련 속성 탐색 — 읽기전용/파생 속성이 따로 있는가
  B. B·C 변경 시 T 자동 갱신 여부
  C. T 설정 제약 규칙 — 하한이 무엇인가
  D. 후속안 비율(B=1.26025·L_we · C=1.06281·L_we)로 L_we 스윕 시 T 거동

출력: P1_극한응력/diag_masta_width.csv
"""
import csv
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
OUTDIR = os.path.join(HERE, "P1_극한응력")
OUT = os.path.join(OUTDIR, "diag_masta_width.csv")
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg   # noqa: E402

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")

V13 = dict(D_pw=3.3309, alpha=19.0, D_we=0.11051, L_we=0.238048)
BL, CL = 0.300 / 0.238048, 0.253 / 0.238048     # 후속안 B/L_we · C/L_we
SWEEP_LWE = [0.175, 0.250, 0.325, 0.400, 0.475, 0.550]


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception:
        return None


def sc(o, n):
    v = safe(o, n)
    if isinstance(v, (int, float)) and not isinstance(v, bool):
        return float(v)
    for a in ("value", "wrapped"):
        w = safe(v, a)
        if isinstance(w, (int, float)) and not isinstance(w, bool):
            return float(w)
    return None


def trySet(det, name, val):
    """설정 시도 → (성공여부, 오류메시지)"""
    try:
        setattr(det, name, val)
        return True, ""
    except Exception as e:
        return False, str(e).splitlines()[0][:90]


def snap(det):
    return {k: sc(det, k) for k in
            ("width", "inner_ring_width", "outer_ring_width",
             "element_diameter", "roller_length", "bore", "outer_diameter",
             "effective_centre_from_front_face")}


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design
    from mastapy.bearings import RollerBearingProfileTypes as RP

    os.makedirs(OUTDIR, exist_ok=True)
    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    uw = [b for b in asm.all_parts_of_type_bearing() if "UW" in str(b)][0]
    det = uw.detail
    det.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG
    rows = []

    # ── A. width 관련 속성 탐색 ──
    print("=" * 78)
    print("A. width 관련 속성 탐색")
    print("=" * 78)
    cands = sorted(n for n in dir(det)
                   if not n.startswith("_") and
                   any(t in n.lower() for t in ("width", "assembl")))
    for n in cands:
        v = sc(det, n)
        ok, err = (True, "") if v is None else trySet(det, n, v)
        print(f"  {n:44} = {('%.6f' % v) if v is not None else 'None':>12}"
              f"   {'설정가능' if ok else '읽기전용/거부'}")
    print()

    # ── 기준 상태: v1.3 제원 주입 ──
    g = sg.bearing(V13["D_pw"], V13["alpha"], V13["D_we"], V13["L_we"])
    sg.apply_to_masta(det, g)
    base = snap(det)
    print(f"[기준] v1.3 주입 후  T={base['width']*1e3:.3f}  "
          f"B={base['inner_ring_width']*1e3:.3f}  C={base['outer_ring_width']*1e3:.3f}")

    # ── B. B·C 변경 시 T 자동 갱신 여부 ──
    print("\n" + "=" * 78)
    print("B. B·C 변경 시 T 자동 갱신 여부")
    print("=" * 78)
    print(f"{'시험':34}{'B 전→후':>20}{'C 전→후':>20}{'T 전→후':>20}  판정")
    tests = [
        ("B +100mm",            dict(inner_ring_width=+0.100)),
        ("C +100mm",            dict(outer_ring_width=+0.100)),
        ("B·C 동시 +100mm",     dict(inner_ring_width=+0.100, outer_ring_width=+0.100)),
        ("B −50mm",             dict(inner_ring_width=-0.050)),
        ("B +400mm (T 초과)",   dict(inner_ring_width=+0.400)),
    ]
    for tag, delta in tests:
        sg.apply_to_masta(det, g)                    # 매번 기준 복원
        b0 = snap(det)
        errs = []
        for k, d in delta.items():
            ok, e = trySet(det, k, b0[k] + d)
            if not ok:
                errs.append(f"{k}:{e}")
        b1 = snap(det)
        auto = abs(b1["width"] - b0["width"]) > 1e-9
        print(f"  {tag:32}{b0['inner_ring_width']*1e3:8.1f}→{b1['inner_ring_width']*1e3:<8.1f}"
              f"{b0['outer_ring_width']*1e3:9.1f}→{b1['outer_ring_width']*1e3:<8.1f}"
              f"{b0['width']*1e3:9.1f}→{b1['width']*1e3:<8.1f}"
              f"  {'자동갱신' if auto else '불변'}"
              f"{('  !! ' + errs[0]) if errs else ''}")
        rows.append(dict(phase="B", test=tag,
                         B0=round(b0["inner_ring_width"] * 1e3, 3),
                         B1=round(b1["inner_ring_width"] * 1e3, 3),
                         C0=round(b0["outer_ring_width"] * 1e3, 3),
                         C1=round(b1["outer_ring_width"] * 1e3, 3),
                         T0=round(b0["width"] * 1e3, 3),
                         T1=round(b1["width"] * 1e3, 3),
                         auto=int(auto), err="|".join(errs)))

    # ── C. T 설정 제약 규칙 ──
    print("\n" + "=" * 78)
    print("C. T 설정 하한 — B=300 · C=253 상태에서 T 를 낮춰 본다")
    print("=" * 78)
    sg.apply_to_masta(det, g)
    print(f"{'시도 T [mm]':>14}{'결과 T':>12}  판정 / 메시지")
    for t_mm in (400, 320, 310, 300, 299, 280, 253, 252, 200):
        sg.apply_to_masta(det, g)
        ok, err = trySet(det, "width", t_mm / 1e3)
        got = sc(det, "width") * 1e3
        print(f"{t_mm:14d}{got:12.1f}  {'OK' if ok else 'REJECT'}"
              f"{('  ' + err) if err else ''}")
        rows.append(dict(phase="C", test=f"T={t_mm}", T1=round(got, 3),
                         auto=int(ok), err=err))

    # ── D. 후속안 비율로 L_we 스윕 ──
    print("\n" + "=" * 78)
    print("D. 후속안 비율 (B=1.26025·L_we · C=1.06281·L_we) 로 L_we 스윕")
    print("=" * 78)
    print(f"{'L_we':>7}{'B_set':>9}{'C_set':>9}{'T(B·C만)':>11}"
          f"{'T=1.30226L':>12}{'차이':>9}  비고")
    for lwe in SWEEP_LWE:
        g2 = sg.bearing(V13["D_pw"], V13["alpha"], V13["D_we"], lwe)
        sg.apply_to_masta(det, g2)                 # 현행식(T 포함) 주입
        # 후속안: B·C 만 덮어쓰고 T 는 건드리지 않는다
        Bn, Cn = BL * lwe, CL * lwe
        e1 = trySet(det, "inner_ring_width", Bn)[1]
        e2 = trySet(det, "outer_ring_width", Cn)[1]
        T_auto = sc(det, "width")
        T_rule = sg.T_OVER_LWE * lwe
        note = " | ".join(x for x in (e1, e2) if x)
        print(f"{lwe*1e3:7.0f}{Bn*1e3:9.1f}{Cn*1e3:9.1f}{T_auto*1e3:11.1f}"
              f"{T_rule*1e3:12.1f}{(T_auto-T_rule)*1e3:9.1f}"
              f"  {note or ('T 불변' if abs(T_auto - T_rule) < 1e-9 else '')}")
        rows.append(dict(phase="D", test=f"L_we={lwe*1e3:.0f}",
                         B1=round(Bn * 1e3, 3), C1=round(Cn * 1e3, 3),
                         T1=round(T_auto * 1e3, 3),
                         T0=round(T_rule * 1e3, 3), err=note))

    keys = []
    for r in rows:
        for k in r:
            if k not in keys:
                keys.append(k)
    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=keys)
        w.writeheader()
        w.writerows(rows)
    print(f"\n[저장] {OUT}")


if __name__ == "__main__":
    main()
