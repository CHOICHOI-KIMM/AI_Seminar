"""
부록 5 Step 1 — 신규 모델(v1.4 프리로드) 상수로 스크리닝 재실행 (dt=20 전용)
============================================================================
run_dlc_screening.py 와 동일 경로(성분 +kσ → 핀지지 정역학 → P → ISO281 손상)이나
Step 0 실측 상수로 C_N·CU_N 만 갱신(그 외 전부 불변)하고 dt=20 만 평가한다.

Step 0 실측(260727, probe_const_v14_preload / probe_rating_v14):
  C   22,227.98 kN → 40,697.42 kN   (+83.1%)  롤러 φ110.51→φ140.0, 유효길이 238.05→350.0
  Cu   3,929.02 kN →  7,255.11 kN   (+84.7%)
  불변: PCD 3330.9 · α 19.00° · e 0.5164914 · Y1 1.161684 · ν50 295 · e_C 0.888378
        기하 L=2.5 / A=-0.5 / B=3.0 (PL z=0, UW z=0.5, DW z=3.0)
  ※ 롤러 수 Z=87 > 이론최대 74.6 (모델 입력 그대로 사용 — 부록 5 한계로 명기)

k 선정: dt=20 ε(k)에서 중앙타겟(ε_UW·ε_Sys ∈ [0,3] 중 ε_Sys→1.5),
        창 없으면 ε_Sys≥0 중 최소(보수측), 전부 음수면 |ε_Sys| 최소.
산출: 부록5_preload_dt20/screen_eps_map_dt20.csv · screen_k_dt20.csv
      (기존 DLC 폴더의 screen_eps_map.csv 는 건드리지 않는다 — 부록3/4 권위자료 보존)
"""
import csv
import os
import sys
import time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)

OUTDIR = os.path.join(HERE, "부록5_preload_dt20")

# ── 상수 (Step 0 실측 갱신분만 반영) ──
L_, A_, B_ = 2.5, -0.5, 3.0
E_LIM, Y1 = 0.5165, 1.1617
C_N, CU_N, P_EXP = 40697.42e3, 7255.11e3, 10.0 / 3.0      # ← 갱신
NU50, EC50, DPW = 294.637, 0.888378, 3328.6
E_W = 9.0 / 8.0
DT0 = 0.1
DT = 20.0
KS = np.round(np.arange(0.0, 3.0001, 0.05), 2)
DESIGN_YEARS = 30.0


def a_iso(kap, ratio):
    k = np.clip(kap, 0.1, 4.0)
    term = np.where(k < 0.4, 1.5859 - 1.3993 / k ** 0.054381,
                    np.where(k < 1.0, 1.5859 - 1.2348 / k ** 0.19087,
                             1.5859 - 1.2348 / k ** 0.071739))
    inner = 1.0 - term * np.minimum(ratio, 5.0) ** 0.4
    return np.clip(np.where(inner > 0, 0.1 * np.maximum(inner, 1e-12) ** -9.185, 50.0),
                   None, 50.0)


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
        return np.where(r <= E_LIM, fr, 0.4 * fr + Y1 * np.abs(fa))
    return P(fra, faa), P(frb, fab)


def damage(P, rpm, rev):
    kap = NU50 / (45000.0 * np.maximum(np.abs(rpm), 1e-6) ** -0.83 * DPW ** -0.5)
    ai = a_iso(kap, EC50 * CU_N / np.maximum(P, 1.0))
    return rev / (ai * (C_N / np.maximum(P, 1.0)) ** P_EXP * 1e6)


def hub(A):
    Mx, My, Mz = A[:, 2] * 1e3, A[:, 3] * 1e3, A[:, 4] * 1e3
    Fx, Fy, Fz = A[:, 5] * 1e3, A[:, 6] * 1e3, A[:, 7] * 1e3
    return (-Fz, Fy, Fx, -Mz, My), A[:, 1]


def screen_one(A, sf):
    (FX, FY, FZ, MX, MY), rpm = hub(A)
    H = np.stack([FX, FY, FZ, MX, MY], axis=1)
    n = len(rpm)
    kap_pts = NU50 / (45000.0 * np.maximum(np.abs(rpm), 1e-6) ** -0.83 * DPW ** -0.5)
    n_kclip = int((kap_pts < 0.1).sum())
    Pu, Pd = statics_P(FX, FY, FZ, MX, MY)
    rev = np.abs(rpm) / 60.0 * DT0
    DrefU, DrefD = float(damage(Pu, rpm, rev).sum()), float(damage(Pd, rpm, rev).sum())
    if DrefU <= 0 or DrefD <= 0:
        return [], dict(valid=False, n_kclip=n_kclip)
    d30U, d30D = DrefU * sf, DrefD * sf
    lifeU, lifeD = DESIGN_YEARS / d30U, DESIGN_YEARS / d30D
    lsys_ref = (lifeU ** -E_W + lifeD ** -E_W) ** (-1 / E_W)

    kp = int(round(DT / DT0))
    nb = n // kp
    if nb < 2:
        return [], dict(valid=False, n_kclip=n_kclip, reason="nbin<2")
    Hb = H[:nb * kp].reshape(nb, kp, 5)
    rb = rpm[:nb * kp].reshape(nb, kp).mean(1)
    mu, sd = Hb.mean(1), Hb.std(1)
    rev_b = np.abs(rb) / 60.0 * (kp * DT0)
    if nb * kp < n:
        seg = np.vstack([Hb[-1], H[nb * kp:]])
        mu[-1], sd[-1] = seg.mean(0), seg.std(0)

    rows = []
    for k in KS:
        rep = mu + np.sign(mu) * k * sd
        Pub, Pdb = statics_P(*[rep[:, i] for i in range(5)])
        eU = float(damage(Pub, rb, rev_b).sum() / DrefU - 1) * 100
        eD = float(damage(Pdb, rb, rev_b).sum() / DrefD - 1) * 100
        lu = DESIGN_YEARS / (DrefU * (1 + eU / 100) * sf)
        ld = DESIGN_YEARS / (DrefD * (1 + eD / 100) * sf)
        ls = (lu ** -E_W + ld ** -E_W) ** (-1 / E_W)
        eS = (lsys_ref / ls - 1) * 100
        rows.append((float(k), DT, eU, eD, eS))
    return rows, dict(valid=True, n_kclip=n_kclip, nbin=nb,
                      d30U=d30U, d30D=d30D, lifeU=lifeU, lifeD=lifeD,
                      lsys=lsys_ref,
                      kap_min=float(kap_pts.min()), kap_max=float(kap_pts.max()))


def select_k(rows):
    ks = np.array([r[0] for r in rows])
    eU = np.array([r[2] for r in rows])
    eS = np.array([r[4] for r in rows])
    kf = np.linspace(ks.min(), ks.max(), 3001)
    u, s = np.interp(kf, ks, eU), np.interp(kf, ks, eS)
    ok = (u >= 0) & (u <= 3) & (s >= 0) & (s <= 3)
    if ok.any():
        idx = np.where(ok)[0]
        j = idx[np.argmin(np.abs(s[idx] - 1.5))]
        return round(float(kf[j]), 2), "center", float(u[j]), float(s[j])
    cons = s >= 0
    if cons.any():
        idx = np.where(cons)[0]
        j = idx[np.argmin(s[idx])]
        return round(float(kf[j]), 2), "cons", float(u[j]), float(s[j])
    j = int(np.argmin(np.abs(s)))
    return round(float(kf[j]), 2), "abs", float(u[j]), float(s[j])


def main():
    os.makedirs(OUTDIR, exist_ok=True)
    meta = {r["DLC"]: r for r in csv.DictReader(open(
        os.path.join(HERE, "dlc_meta.csv"), encoding="utf-8-sig"))}
    dlcs = sorted(meta)
    print(f"[부록5 Step1] 대상 {len(dlcs)} DLC · dt={DT:g} 전용 · "
          f"C={C_N/1e3:,.0f} kN · Cu={CU_N/1e3:,.0f} kN")
    t0 = time.time()

    fmap = open(os.path.join(OUTDIR, "screen_eps_map_dt20.csv"), "w", newline="",
                encoding="utf-8-sig")
    wmap = csv.writer(fmap)
    wmap.writerow(["DLC", "k", "dt_s", "eps_UW_pct", "eps_DW_pct", "eps_Sys_pct"])

    krows = []
    n_bad = 0
    for i, name in enumerate(dlcs, 1):
        A = np.genfromtxt(os.path.join(HERE, name, "raw.csv"), delimiter=",",
                          skip_header=1, encoding="utf-8-sig")
        sf = float(meta[name]["ScaleFactor"])
        rows, sm = screen_one(A, sf)
        if not sm["valid"]:
            n_bad += 1
            krows.append(dict(DLC=name, valid=0, k="", ksel="", eps_UW="",
                              eps_Sys="", nbin="", D30_UW_scr="", D30_DW_scr="",
                              life_Sys_scr_yr="", kappa_min="", kappa_max="",
                              n_kappa_clip=sm["n_kclip"]))
            continue
        for r in rows:
            wmap.writerow([name] + list(r))
        k, tag, eu, es = select_k(rows)
        krows.append(dict(DLC=name, valid=1, k=k, ksel=tag, eps_UW=eu, eps_Sys=es,
                          nbin=sm["nbin"], D30_UW_scr=sm["d30U"],
                          D30_DW_scr=sm["d30D"], life_Sys_scr_yr=sm["lsys"],
                          kappa_min=sm["kap_min"], kappa_max=sm["kap_max"],
                          n_kappa_clip=sm["n_kclip"]))
        if i % 20 == 0 or i == len(dlcs):
            print(f"  … {i}/{len(dlcs)}  ({time.time()-t0:.0f}s)")
    fmap.close()

    with open(os.path.join(OUTDIR, "screen_k_dt20.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(krows[0]))
        w.writeheader(); w.writerows(krows)

    from collections import Counter
    kc = Counter(r["ksel"] for r in krows if r["valid"] == 1)
    ok = [r for r in krows if r["valid"] == 1]
    print(f"\n[완료] {len(ok)}/{len(dlcs)} DLC 유효 (무효 {n_bad}) · {time.time()-t0:.0f}s")
    print(f"  k 선정: {dict(kc)}  (center=창내중앙 · cons=보수측 · abs=|ε|최소)")
    if ok:
        kv = np.array([r["k"] for r in ok], dtype=float)
        print(f"  k 범위 {kv.min():.2f}~{kv.max():.2f} · 평균 {kv.mean():.2f}")
    print(f"[저장] {OUTDIR}")


if __name__ == "__main__":
    main()
