"""
사이징 최적화 부록 3 Stage 1 — α=30° 에서 S1 두 기하 변형 비교 (참값 없이 dt=20)
=================================================================================
α 를 19° → 30° 로 바꾸면 e = 1.5tanα 와 Y1 = 0.4cotα 가 직접 바뀌어 분기 구조가
크게 달라진다. Stage 0 (α=19°) 에서 두 S1 변형의 차이가 0.83% 였는데,
α=30° 에서도 그 결론이 유지되는지 확인한다.

절차
 1. MASTA 에서 α=30° 설정 후 e·Y1·C·C_u·a 를 실측 (자동 갱신 여부 확인)
 2. 실측값으로 S1 상수를 동기화하고 두 기하 변형의 k 를 산출
      A 현행기하 : L=2.500  A=-0.500  B=3.000        (α 무관, 고정)
      B L_eff    : L, A, B 를 실측 a 로부터 산출      (α 반영)
 3. 각 k 로 dt=20 MASTA 실행 → 함대 수명 비교
"""
import csv
import math
import os
import sys
import time

import numpy as np
import psutil

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
DLCDIR = os.path.join(RES, "DLC별해석")
OUTDIR = os.path.join(HERE, "부록3_S1기하검증")
sys.path.insert(0, ROOT)

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
V13_SEQ = [("element_diameter", 0.11051), ("roller_length", 0.238048),
           ("outer_diameter", 3.6), ("inner_ring_width", 0.3),
           ("outer_ring_width", 0.253), ("width", 0.31),
           ("number_of_elements", 87)]
ALPHA_DEG = 30.0
Z1, Z2 = 0.5, 3.0

NU50, EC50, DPW = 294.637, 0.888378, 3328.6
P_EXP, E_W = 10.0 / 3.0, 9.0 / 8.0
DT0, DT = 0.1, 20.0
KS = np.round(np.arange(0.0, 3.0001, 0.05), 2)
DESIGN_YEARS = 30.0
NBATCH, MEM_LIMIT = 20, 95.0
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")


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


def a_iso(kap, ratio):
    k = np.clip(kap, 0.1, 4.0)
    term = np.where(k < 0.4, 1.5859 - 1.3993 / k ** 0.054381,
                    np.where(k < 1.0, 1.5859 - 1.2348 / k ** 0.19087,
                             1.5859 - 1.2348 / k ** 0.071739))
    inner = 1.0 - term * np.minimum(ratio, 5.0) ** 0.4
    return np.clip(np.where(inner > 0, 0.1 * np.maximum(inner, 1e-12) ** -9.185, 50.0),
                   None, 50.0)


def build(L_, A_, B_, E_LIM, Y1, C_N, CU_N):
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

    def damage(P, rpm, rev):
        kap = NU50 / (45000.0 * np.maximum(np.abs(rpm), 1e-6) ** -0.83 * DPW ** -0.5)
        ai = a_iso(kap, EC50 * CU_N / np.maximum(P, 1.0))
        return rev / (ai * (C_N / np.maximum(P, 1.0)) ** P_EXP * 1e6)
    return statics_P, damage


def hub(A):
    Mx, My, Mz = A[:, 2] * 1e3, A[:, 3] * 1e3, A[:, 4] * 1e3
    Fx, Fy, Fz = A[:, 5] * 1e3, A[:, 6] * 1e3, A[:, 7] * 1e3
    return (-Fz, Fy, Fx, -Mz, My), A[:, 1]


def screen_k(name, sf, statics_P, damage, E_LIM):
    A = np.genfromtxt(os.path.join(DLCDIR, name, "raw.csv"), delimiter=",",
                      skip_header=1, encoding="utf-8-sig")
    (FX, FY, FZ, MX, MY), rpm = hub(A)
    H = np.stack([FX, FY, FZ, MX, MY], axis=1)
    n = len(rpm)
    Pu, Pd, ru, _ = statics_P(FX, FY, FZ, MX, MY)
    branch = float((ru > E_LIM).mean()) * 100
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
    u, s = np.array(eU), np.array(eS)
    kf = np.linspace(KS.min(), KS.max(), 3001)
    ui, si = np.interp(kf, KS, u), np.interp(kf, KS, s)
    ok = (ui >= 0) & (ui <= 3) & (si >= 0) & (si <= 3)
    if ok.any():
        idx = np.where(ok)[0]; j = idx[np.argmin(np.abs(si[idx] - 1.5))]; tag = "center"
    else:
        cons = si >= 0
        if cons.any():
            idx = np.where(cons)[0]; j = idx[np.argmin(si[idx])]; tag = "cons"
        else:
            j = int(np.argmin(np.abs(si))); tag = "abs"
    return dict(k=round(float(kf[j]), 2), ksel=tag, nbin=nb, branch_pct=branch)


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
    names = sorted(meta)

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
    print("[1] α 변경 전 (19°) 실측")
    def snap(d):
        return dict(alpha=math.degrees(sc(d, "contact_angle")),
                    cone=math.degrees(sc(d, "cone_angle")),
                    e=sc(d, "limiting_value_for_axial_load_ratio"),
                    Y1=sc(d, "dynamic_axial_load_factor_for_high_axial_radial_load_ratios"),
                    C=sc(d, "basic_dynamic_load_rating"), Cu=sc(d, "fatigue_load_limit"),
                    a=sc(d, "effective_centre_from_front_face"), T=sc(d, "width"),
                    drop=max(float(safe(q, "total_deviation")) for q in
                             list(safe(d, "inner_race_and_roller_profiles"))) * 1e6)
    s19 = snap(det)
    for k_, v_ in s19.items():
        print(f"    {k_:6} = {v_:,.6f}")

    print(f"\n[2] α = {ALPHA_DEG}° 설정")
    for b in bearings:
        try:
            b.detail.contact_angle = math.radians(ALPHA_DEG)
        except Exception as e:
            print("  !! contact_angle 설정 실패:", str(e).splitlines()[0][:70])
    s30 = snap(bearings[0].detail)
    print(f"    {'항목':6} {'19°':>16} {'30°':>16} {'변화':>10}")
    for k_ in s19:
        a_, b_ = s19[k_], s30[k_]
        print(f"    {k_:6} {a_:16,.6f} {b_:16,.6f} {(b_/a_-1)*100:+9.2f}%")
    e_th, y_th = 1.5 * math.tan(math.radians(ALPHA_DEG)), 0.4 / math.tan(math.radians(ALPHA_DEG))
    print(f"    이론식 e = 1.5tanα = {e_th:.6f} · Y1 = 0.4cotα = {y_th:.6f}")

    # L_eff (α=30 실측 a 사용)
    c = s30["a"] - s30["T"] / 2
    L_eff, A_eff, B_eff = (Z2 - Z1) + 2 * c, (Z1 - c) * -1, Z2 + c
    GEOM = {"A_현행기하": (2.500000, -0.500000, 3.000000),
            "B_Leff반영": (L_eff, A_eff, B_eff)}
    print(f"\n[3] S1 기하  c = a - T/2 = {c*1000:.3f} mm")
    for t, (L_, A_, B_) in GEOM.items():
        print(f"    {t:12} L={L_:.6f}  A={A_:+.6f}  B={B_:.6f}  1/L={1/L_:.4f}")

    E_LIM, Y1, C_N, CU_N = s30["e"], s30["Y1"], s30["C"], s30["Cu"]
    kres = {}
    for tag, (L_, A_, B_) in GEOM.items():
        st, dm = build(L_, A_, B_, E_LIM, Y1, C_N, CU_N)
        kres[tag] = {n: r for n in names
                     if (r := screen_k(n, float(meta[n]["ScaleFactor"]), st, dm, E_LIM))}
        ks = [v["k"] for v in kres[tag].values()]
        br = [v["branch_pct"] for v in kres[tag].values()]
        from collections import Counter
        print(f"    [{tag}] k {min(ks):.2f}~{max(ks):.2f} 평균 {sum(ks)/len(ks):.3f} "
              f"{dict(Counter(v['ksel'] for v in kres[tag].values()))} "
              f"분기율 {sum(br)/len(br):.1f}%")
    common = sorted(set(kres["A_현행기하"]) & set(kres["B_Leff반영"]))
    dk = [kres["B_Leff반영"][n]["k"] - kres["A_현행기하"][n]["k"] for n in common]
    same = sum(1 for d_ in dk if abs(d_) < 1e-9)
    print(f"    k 동일 {same}/{len(common)} · Δk 평균 {sum(dk)/len(dk):+.3f} "
          f"범위 {min(dk):+.2f}~{max(dk):+.2f}")

    # ── MASTA dt=20 (두 변형) ──
    dp = asm.design_properties
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
    bmap = {("UW" if "UW" in str(b) else "DW"): b for b in bearings}
    lc0 = next(cc for cc in dp.static_loads if cc.name == "Load Case 1")
    ds = lc0.design_state_load_case_group
    RAW = {n: [{a: float(b) for a, b in r.items()} for r in csv.DictReader(
        open(os.path.join(DLCDIR, n, "raw.csv"), encoding="utf-8-sig"))] for n in common}

    out = {}
    for tag in GEOM:
        print(f"\n[4] MASTA dt=20 · {tag}")
        res, t0 = {}, time.time()
        for i, name in enumerate(common, 1):
            sf = float(meta[name]["ScaleFactor"])
            reps = bin_reps(RAW[name], kres[tag][name]["k"])
            dmg = {"UW": [], "DW": []}
            short = name[-6:].replace(".", "")
            for b0 in range(0, len(reps), NBATCH):
                chunk = reps[b0:b0 + NBATCH]
                lcs = []
                for cid, rev, rec in chunk:
                    lc = lc0.duplicate(ds, f"a3s1{tag[0]}_{short}_{cid}")
                    mf.set_loads(lc, pl, ipl, rec)
                    lcs.append(lc)
                duty = dp.add_duty_cycle(f"a3s1{tag[0]}dc_{short}_{b0}")
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
            res[name] = (sum(dmg["UW"]) * sf, sum(dmg["DW"]) * sf)
            if i % 20 == 0 or i == len(common):
                print(f"    {i}/{len(common)}  ({time.time()-t0:.0f}s)", flush=True)
        out[tag] = res

    with open(os.path.join(OUTDIR, "stage1_alpha30_per_dlc.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.writer(f)
        w.writerow(["DLC", "k_A", "k_B", "dk", "branch_A_pct", "branch_B_pct",
                    "D30_UW_A", "D30_UW_B", "d_pct", "D30_DW_A", "D30_DW_B"])
        for n in sorted(common, key=lambda x: -out["A_현행기하"][x][0]):
            a_, b_ = kres["A_현행기하"][n], kres["B_Leff반영"][n]
            ua, ub = out["A_현행기하"][n][0], out["B_Leff반영"][n][0]
            w.writerow([n, a_["k"], b_["k"], round(b_["k"] - a_["k"], 2),
                        f"{a_['branch_pct']:.1f}", f"{b_['branch_pct']:.1f}",
                        f"{ua:.6f}", f"{ub:.6f}", f"{(ub/ua-1)*100:+.3f}",
                        f"{out['A_현행기하'][n][1]:.6f}", f"{out['B_Leff반영'][n][1]:.6f}"])

    print("\n" + "=" * 64)
    print(f"  α = {ALPHA_DEG}°   {'A 현행기하':>14} {'B L_eff':>14} {'차이':>10}")
    S = {t: (sum(v[0] for v in out[t].values()), sum(v[1] for v in out[t].values()))
         for t in GEOM}
    for lab, f_ in (("Σ D30_UW", lambda t: S[t][0]), ("Σ D30_DW", lambda t: S[t][1]),
                    ("함대 UW [yr]", lambda t: DESIGN_YEARS / S[t][0]),
                    ("함대 DW [yr]", lambda t: DESIGN_YEARS / S[t][1]),
                    ("함대 Sys [yr]", lambda t: lsys(DESIGN_YEARS / S[t][0],
                                                    DESIGN_YEARS / S[t][1]))):
        a_, b_ = f_("A_현행기하"), f_("B_Leff반영")
        print(f"  {lab:14} {a_:14.4f} {b_:14.4f} {(b_/a_-1)*100:+9.2f}%")
    print(f"[저장] {OUTDIR}")


if __name__ == "__main__":
    main()
