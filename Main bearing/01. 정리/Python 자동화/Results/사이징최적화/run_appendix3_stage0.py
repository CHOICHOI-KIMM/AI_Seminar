"""
사이징 최적화 부록 3 Stage 0 — S1 기하 모델 변경의 영향 분리
=============================================================
동일 MASTA 모델(빔 3안 + v1.3 제원 + DIN, α=19°)에서 S1 의 기하 상수만 바꿔
k 를 각각 산출하고 dt=20 MASTA 결과를 비교한다.

  조합 A : 현행 기하  L=2.500  A=-0.500  B=3.000   → 부록 1-A 결과 재사용
  조합 B : L_eff 반영 L=3.6167 A=+0.0583 B=3.5583  → 본 스크립트가 실행

유효 지지점: c = a - T/2 = 713.331 - 155.0 = 558.331 mm (O 배열)
"""
import csv
import math
import os
import sys

import numpy as np
import psutil

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
DLCDIR = os.path.join(RES, "DLC별해석")
OUTDIR = os.path.join(HERE, "부록3_S1기하검증")
APP1 = os.path.join(HERE, "부록1_롤러프로파일", "per_dlc_compare.csv")
sys.path.insert(0, ROOT)

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
V13_SEQ = [("element_diameter", 0.11051), ("roller_length", 0.238048),
           ("outer_diameter", 3.6), ("inner_ring_width", 0.3),
           ("outer_ring_width", 0.253), ("width", 0.31),
           ("number_of_elements", 87)]

# ── S1 기하 변형 ──
GEOM = {
    "A_현행기하": dict(L=2.500000, A=-0.500000, B=3.000000),
    "B_Leff반영": dict(L=3.616662, A=+0.058331, B=3.558331),
}
# ── S1 공통 상수 (v1.3) ──
E_LIM, Y1 = 0.5165, 1.1617
C_N, CU_N, P_EXP = 22227.98e3, 3929.02e3, 10.0 / 3.0
NU50, EC50, DPW = 294.637, 0.888378, 3328.6
E_W = 9.0 / 8.0
DT0, DT = 0.1, 20.0
KS = np.round(np.arange(0.0, 3.0001, 0.05), 2)
DESIGN_YEARS = 30.0
NBATCH, MEM_LIMIT = 20, 95.0
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")


def a_iso(kap, ratio):
    k = np.clip(kap, 0.1, 4.0)
    term = np.where(k < 0.4, 1.5859 - 1.3993 / k ** 0.054381,
                    np.where(k < 1.0, 1.5859 - 1.2348 / k ** 0.19087,
                             1.5859 - 1.2348 / k ** 0.071739))
    inner = 1.0 - term * np.minimum(ratio, 5.0) ** 0.4
    return np.clip(np.where(inner > 0, 0.1 * np.maximum(inner, 1e-12) ** -9.185, 50.0),
                   None, 50.0)


def make_statics(L_, A_, B_):
    def statics_P(FX, FY, FZ, MX, MY):
        rax = FX * (B_ / L_) - MY / L_
        ray = FY * (B_ / L_) + MX / L_
        rbx = FX * (A_ / L_) + MY / L_
        rby = FY * (A_ / L_) - MX / L_
        fra, frb = np.hypot(rax, ray), np.hypot(rbx, rby)
        sa, sb = 0.5 * fra / Y1, 0.5 * frb / Y1
        case1 = FZ >= sa - sb
        faa = np.where(case1, FZ + sb, sa)
        fab = np.where(case1, -sb, -(sa - FZ))

        def P(fr, fa):
            r = np.abs(fa) / np.maximum(fr, 1e-9)
            return np.where(r <= E_LIM, fr, 0.4 * fr + Y1 * np.abs(fa)), r
        Pu, ru = P(fra, faa)
        Pd, rd = P(frb, fab)
        return Pu, Pd, ru, rd
    return statics_P


def damage(P, rpm, rev):
    kap = NU50 / (45000.0 * np.maximum(np.abs(rpm), 1e-6) ** -0.83 * DPW ** -0.5)
    ai = a_iso(kap, EC50 * CU_N / np.maximum(P, 1.0))
    return rev / (ai * (C_N / np.maximum(P, 1.0)) ** P_EXP * 1e6)


def hub(A):
    Mx, My, Mz = A[:, 2] * 1e3, A[:, 3] * 1e3, A[:, 4] * 1e3
    Fx, Fy, Fz = A[:, 5] * 1e3, A[:, 6] * 1e3, A[:, 7] * 1e3
    return (-Fz, Fy, Fx, -Mz, My), A[:, 1]


def screen_k(name, sf, statics_P):
    """dt=20 중앙타겟 k + 분기율(Fa/Fr>e 비율) 반환"""
    A = np.genfromtxt(os.path.join(DLCDIR, name, "raw.csv"), delimiter=",",
                      skip_header=1, encoding="utf-8-sig")
    (FX, FY, FZ, MX, MY), rpm = hub(A)
    H = np.stack([FX, FY, FZ, MX, MY], axis=1)
    n = len(rpm)
    Pu, Pd, ru, rd = statics_P(FX, FY, FZ, MX, MY)
    branch = float((ru > E_LIM).mean()) * 100          # UW 분기 초과 비율 [%]
    rev = np.abs(rpm) / 60.0 * DT0
    DrefU, DrefD = float(damage(Pu, rpm, rev).sum()), float(damage(Pd, rpm, rev).sum())
    if DrefU <= 0 or DrefD <= 0:
        return None
    lifeU, lifeD = DESIGN_YEARS / (DrefU * sf), DESIGN_YEARS / (DrefD * sf)
    lsys_ref = (lifeU ** -E_W + lifeD ** -E_W) ** (-1 / E_W)
    kp = int(round(DT / DT0)); nb = n // kp
    if nb < 2:
        return None
    Hb = H[:nb * kp].reshape(nb, kp, 5)
    rb = rpm[:nb * kp].reshape(nb, kp).mean(1)
    mu, sd = Hb.mean(1), Hb.std(1)
    rev_b = np.abs(rb) / 60.0 * (kp * DT0)
    if nb * kp < n:
        seg = np.vstack([Hb[-1], H[nb * kp:]])
        mu[-1], sd[-1] = seg.mean(0), seg.std(0)
    eU, eS = [], []
    for k in KS:
        rep = mu + np.sign(mu) * k * sd
        Pub, Pdb, _, _ = statics_P(*[rep[:, i] for i in range(5)])
        du = float(damage(Pub, rb, rev_b).sum() / DrefU - 1) * 100
        dd = float(damage(Pdb, rb, rev_b).sum() / DrefD - 1) * 100
        lu = DESIGN_YEARS / (DrefU * (1 + du / 100) * sf)
        ld = DESIGN_YEARS / (DrefD * (1 + dd / 100) * sf)
        eU.append(du)
        eS.append((lsys_ref / (lu ** -E_W + ld ** -E_W) ** (-1 / E_W) - 1) * 100)
    ks = KS; u = np.array(eU); s = np.array(eS)
    kf = np.linspace(ks.min(), ks.max(), 3001)
    ui, si = np.interp(kf, ks, u), np.interp(kf, ks, s)
    ok = (ui >= 0) & (ui <= 3) & (si >= 0) & (si <= 3)
    if ok.any():
        idx = np.where(ok)[0]; j = idx[np.argmin(np.abs(si[idx] - 1.5))]; tag = "center"
    else:
        cons = si >= 0
        if cons.any():
            idx = np.where(cons)[0]; j = idx[np.argmin(si[idx])]; tag = "cons"
        else:
            j = int(np.argmin(np.abs(si))); tag = "abs"
    return dict(k=round(float(kf[j]), 2), ksel=tag, nbin=nb, branch_pct=branch,
                eps_UW=float(ui[j]), eps_Sys=float(si[j]))


def bin_reps(data, k):
    kp = int(round(DT / DT0)); n = len(data); nb = max(n // kp, 1)
    edges = [(b * kp, (b + 1) * kp) for b in range(nb)]
    if edges and edges[-1][1] < n:
        edges[-1] = (edges[-1][0], n)
    out = []
    for bi, (i0, i1) in enumerate(edges):
        m = i1 - i0
        rec = {key: sum(data[i][key] for i in range(i0, i1)) / m for key in ("rpm", "Mx")}
        for key in KSIG:
            mu = sum(data[i][key] for i in range(i0, i1)) / m
            var = sum((data[i][key] - mu) ** 2 for i in range(i0, i1)) / m
            rec[key] = mu + math.copysign(1.0, mu) * k * math.sqrt(var)
        out.append((bi, abs(rec["rpm"]) / 60.0 * (m * DT0), rec))
    return out


def lsys(lu, ld):
    return (lu ** -E_W + ld ** -E_W) ** (-1.0 / E_W)


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
    from mastapy.bearings import RollerBearingProfileTypes as RP

    os.makedirs(OUTDIR, exist_ok=True)
    meta = {r["DLC"]: r for r in csv.DictReader(
        open(os.path.join(DLCDIR, "dlc_meta.csv"), encoding="utf-8-sig"))}
    app1 = {r["DLC"]: r for r in csv.DictReader(open(APP1, encoding="utf-8-sig"))}
    names = sorted(n for n in meta if n in app1)
    print(f"[부록3 Stage0] 대상 {len(names)} DLC · dt=20 · α=19° · DIN Lundberg")

    # ── S1 두 변형의 k 산출 ──
    kres = {}
    for tag, g in GEOM.items():
        st = make_statics(g["L"], g["A"], g["B"])
        kres[tag] = {}
        for n in names:
            r = screen_k(n, float(meta[n]["ScaleFactor"]), st)
            if r:
                kres[tag][n] = r
        ks = [v["k"] for v in kres[tag].values()]
        br = [v["branch_pct"] for v in kres[tag].values()]
        from collections import Counter
        c = Counter(v["ksel"] for v in kres[tag].values())
        print(f"  [{tag}] L={g['L']:.4f} A={g['A']:+.4f} B={g['B']:.4f}"
              f"  k {min(ks):.2f}~{max(ks):.2f} 평균 {sum(ks)/len(ks):.3f}"
              f"  {dict(c)}  분기율 평균 {sum(br)/len(br):.1f}%")
    same = sum(1 for n in names if kres["A_현행기하"][n]["k"] == kres["B_Leff반영"][n]["k"])
    dk = [kres["B_Leff반영"][n]["k"] - kres["A_현행기하"][n]["k"] for n in names]
    print(f"  k 동일 {same}/{len(names)} · Δk 평균 {sum(dk)/len(dk):+.3f} "
          f"범위 {min(dk):+.2f}~{max(dk):+.2f}")

    # ── 조합 B 만 MASTA 실행 (A 는 부록 1-A 재사용) ──
    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    bearings = list(asm.all_parts_of_type_bearing())
    for b in bearings:
        det = b.detail
        for k_, v_ in V13_SEQ:
            try:
                setattr(det, k_, v_)
            except Exception as e:
                print(f"  !! {k_}: {str(e).splitlines()[0][:50]}")
        det.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG
    det = bearings[0].detail
    print(f"  [모델] C={det.basic_dynamic_load_rating:,.0f} "
          f"Cu={det.fatigue_load_limit:,.0f} mass={det.mass:,.1f}")

    dp = asm.design_properties
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
    bmap = {("UW" if "UW" in str(b) else "DW"): b for b in bearings}
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    ds = lc0.design_state_load_case_group

    import time
    rows = []
    t0 = time.time()
    for i, name in enumerate(names, 1):
        sf = float(meta[name]["ScaleFactor"])
        kk = kres["B_Leff반영"][name]["k"]
        reps = bin_reps([{a: float(b) for a, b in r.items()} for r in csv.DictReader(
            open(os.path.join(DLCDIR, name, "raw.csv"), encoding="utf-8-sig"))], kk)
        dmg = {"UW": [], "DW": []}
        short = name[-6:].replace(".", "")
        for b0 in range(0, len(reps), NBATCH):
            chunk = reps[b0:b0 + NBATCH]
            lcs = []
            for cid, rev, rec in chunk:
                lc = lc0.duplicate(ds, f"a3s0_{short}_{cid}")
                mf.set_loads(lc, pl, ipl, rec)
                lcs.append(lc)
            duty = dp.add_duty_cycle(f"a3s0dc_{short}_{b0}")
            for lc in lcs:
                duty.add_static_load(lc)
            csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            csd.perform_analysis()
            for key, b in bmap.items():
                subs = list(list(csd.results_for(b))[0].component_analysis_cases)
                for (cid, rev, rec), sub in zip(chunk, subs):
                    l10 = (sub.component_detailed_analysis
                           .isots162812008.modified_reference_rating_life_cycles)
                    dmg[key].append(rev / l10 if (l10 and l10 > 0) else 0.0)
            for lc in lcs:
                try:
                    lc.delete()
                except Exception:
                    pass
            try:
                duty.delete()
            except Exception:
                pass
            if psutil.virtual_memory().percent > MEM_LIMIT:
                raise MemoryError("메모리 95% 초과")
        rows.append(dict(DLC=name, D30_UW=sum(dmg["UW"]) * sf, D30_DW=sum(dmg["DW"]) * sf))
        if i % 20 == 0 or i == len(names):
            print(f"    [B] {i}/{len(names)}  ({time.time()-t0:.0f}s)", flush=True)

    # ── 비교 ──
    B = {r["DLC"]: r for r in rows}
    sUA = sum(float(app1[n]["D30_UW_DIN"]) for n in names)
    sDA = sum(float(app1[n]["D30_DW_DIN"]) for n in names)
    sUB = sum(B[n]["D30_UW"] for n in names)
    sDB = sum(B[n]["D30_DW"] for n in names)
    LUA, LDA = DESIGN_YEARS / sUA, DESIGN_YEARS / sDA
    LUB, LDB = DESIGN_YEARS / sUB, DESIGN_YEARS / sDB
    with open(os.path.join(OUTDIR, "stage0_per_dlc.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.writer(f)
        w.writerow(["DLC", "k_A", "k_B", "dk", "branch_A_pct", "branch_B_pct",
                    "D30_UW_A", "D30_UW_B", "d_pct", "D30_DW_A", "D30_DW_B"])
        for n in sorted(names, key=lambda x: -float(app1[x]["D30_UW_DIN"])):
            a, b = kres["A_현행기하"][n], kres["B_Leff반영"][n]
            ua, ub = float(app1[n]["D30_UW_DIN"]), B[n]["D30_UW"]
            w.writerow([n, a["k"], b["k"], round(b["k"] - a["k"], 2),
                        f"{a['branch_pct']:.1f}", f"{b['branch_pct']:.1f}",
                        f"{ua:.6f}", f"{ub:.6f}", f"{(ub/ua-1)*100:+.3f}",
                        app1[n]["D30_DW_DIN"], f"{B[n]['D30_DW']:.6f}"])
    print("\n" + "=" * 62)
    print(f"  {'':16} {'A 현행기하':>14} {'B L_eff':>14} {'차이':>10}")
    for lab, a, b in (("Σ D30_UW", sUA, sUB), ("Σ D30_DW", sDA, sDB),
                      ("함대 UW [yr]", LUA, LUB), ("함대 DW [yr]", LDA, LDB),
                      ("함대 Sys [yr]", lsys(LUA, LDA), lsys(LUB, LDB))):
        print(f"  {lab:16} {a:14.4f} {b:14.4f} {(b/a-1)*100:+9.2f}%")
    print(f"[저장] {OUTDIR}")


if __name__ == "__main__":
    main()
