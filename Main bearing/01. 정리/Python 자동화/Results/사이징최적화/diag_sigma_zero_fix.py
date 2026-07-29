"""
sigma=0 원인 검증 — 원뿔 테이퍼 보정 가설
==========================================
가설: sizing_geom.bearing() 이 bore/OD 를 '중앙면' 궤도반경 기준으로 산출하여
      원뿔 테이퍼를 무시한다. 롤러가 길어지면 내륜 궤도가 소단(小端)에서
      보어 아래로 내려가 형상이 무효가 된다.

  현행 : d = D_pw − D_we·cosα − 2·t_i
  보정 : d = D_pw − D_we·cosα − L_we·sinα − 2·t_i      (소단 기준)
         D = D_pw + D_we·cosα + L_we·sinα + 2·t_o      (대단 기준)

검증: 현행식으로 FAIL 이던 점들을 보정식으로 재해석하여 sigma>0 이 되는지 확인.
"""
import csv
import math
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
OUTDIR = os.path.join(HERE, "P1_극한응력")
OUT = os.path.join(OUTDIR, "diag_sigma_zero_fix.csv")
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg   # noqa: E402
from diag_sigma_zero import GOV, Rig, sc, safe   # noqa: E402

# (D_pw, alpha, D_we, L_we) — A/C 단계에서 FAIL 이던 점 + 대조용 OK 점
CASES = [
    (3.3309, 24.0, 0.170, 0.380, "OK(대조)"),
    (3.3309, 24.0, 0.170, 0.400, "FAIL"),
    (3.3309, 24.0, 0.170, 0.500, "FAIL"),
    (3.900, 24.0, 0.170, 0.460, "OK(대조)"),
    (3.900, 24.0, 0.170, 0.480, "FAIL"),
    (3.900, 24.0, 0.170, 0.560, "FAIL"),
    (3.900, 30.0, 0.170, 0.440, "FAIL"),
    (3.900, 30.0, 0.170, 0.500, "FAIL"),
    (4.500, 24.0, 0.170, 0.560, "FAIL"),
    (4.500, 30.0, 0.200, 0.500, "FAIL"),
    (4.500, 30.0, 0.110, 0.415, "FAIL(프로브)"),
    (4.200, 30.0, 0.110, 0.415, "FAIL(프로브)"),
]
Z1, Z2 = 0.5, 4.0


def bearing_fix(D_pw, alpha_deg, D_we, L_we):
    """소단/대단 기준 bore·OD 보정본"""
    ra = math.radians(alpha_deg)
    ca, sa = math.cos(ra), math.sin(ra)
    t_i, t_o = sg.TI_OVER_DPW * D_pw, sg.TO_OVER_DPW * D_pw
    d = round((D_pw - D_we * ca - L_we * sa - 2 * t_i) * 1000) / 1000
    D = round((D_pw + D_we * ca + L_we * sa + 2 * t_o) * 1000) / 1000
    T = sg.T_OVER_LWE * L_we
    return dict(D_pw=D_pw, alpha_deg=alpha_deg, D_we=D_we, L_we=L_we,
                bore=d, outer_diameter=D, width=T,
                inner_ring_width=sg.B_OVER_T * T,
                outer_ring_width=sg.C_OVER_T * T,
                number_of_elements=int(sg.ETA * math.pi * D_pw / D_we))


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
    from mastapy.bearings import RollerBearingProfileTypes as RP

    os.makedirs(OUTDIR, exist_ok=True)
    rig = Rig(mastapy, Design, AnalysisType, RP)
    t0 = time.time()
    rows = []
    print("=" * 96)
    print("원뿔 테이퍼 보정 검증  (z 0.5/4.0)")
    print("=" * 96)
    print(f"{'D_pw':>7}{'a':>4}{'D_we':>6}{'L_we':>6}{'기존판정':>12}"
          f"{'현행bore':>10}{'보정bore':>10}{'Δ':>7}"
          f"{'현행σ':>10}{'보정σ':>10}  결과")

    for dpw, al, dwe, lwe, tag in CASES:
        g0 = sg.bearing(dpw, al, dwe, lwe)
        g1 = bearing_fix(dpw, al, dwe, lwe)
        res = {}
        for lbl, g in (("cur", g0), ("fix", g1)):
            for b in rig.bs:
                try:
                    if b.inner_connection is not None:
                        b.inner_connection.delete()
                except Exception:
                    pass
            s = sg.shaft(g["bore"], Z2)
            try:
                rig.sh.remove_all_sections()
                rig.sh.add_section(0.0, s["length"], s["outer_diameter"],
                                   s["inner_diameter"], s["outer_diameter"],
                                   s["inner_diameter"])
            except Exception:
                pass
            for b in rig.bs:
                sg.apply_to_masta(b.detail, g)
            for b, z in ((rig.uw, Z1), (rig.dw, Z2)):
                try:
                    b.try_mount_on(rig.sh, z)
                except Exception:
                    pass
            sig, _, err = rig.solve()
            res[lbl] = (sig, err)
        s0, s1 = res["cur"][0], res["fix"][0]
        verdict = ("복구" if (s0 <= 0 < s1) else
                   "유지" if (s0 > 0 and s1 > 0) else
                   "여전히실패" if s1 <= 0 else "역행")
        print(f"{dpw*1e3:7.0f}{al:4.0f}{dwe*1e3:6.0f}{lwe*1e3:6.0f}{tag:>12}"
              f"{g0['bore']*1e3:10.0f}{g1['bore']*1e3:10.0f}"
              f"{(g1['bore']-g0['bore'])*1e3:7.0f}"
              f"{s0:10.1f}{s1:10.1f}  {verdict}", flush=True)
        rows.append(dict(D_pw_mm=dpw * 1e3, alpha=al, D_we_mm=dwe * 1e3,
                         L_we_mm=lwe * 1e3, prev=tag,
                         bore_cur_mm=round(g0["bore"] * 1e3, 1),
                         bore_fix_mm=round(g1["bore"] * 1e3, 1),
                         D_cur_mm=round(g0["outer_diameter"] * 1e3, 1),
                         D_fix_mm=round(g1["outer_diameter"] * 1e3, 1),
                         T_mm=round(g0["width"] * 1e3, 1),
                         sigma_cur=round(s0, 1), sigma_fix=round(s1, 1),
                         verdict=verdict, err=res["fix"][1]))

    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)
    n_fix = sum(1 for r in rows if r["verdict"] == "복구")
    n_bad = sum(1 for r in rows if r["verdict"] in ("여전히실패", "역행"))
    print(f"\n[결과] 복구 {n_fix} · 유지 {sum(1 for r in rows if r['verdict']=='유지')}"
          f" · 미해결 {n_bad}   ({time.time()-t0:.0f}s)")
    print(f"[저장] {OUT}")


if __name__ == "__main__":
    main()
