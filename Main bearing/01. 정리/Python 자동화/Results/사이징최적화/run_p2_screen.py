"""
P2 Phase 1 Step 1 — L_eff 기반 스크리닝으로 설계별 111 DLC k 산출 (문서 §8-3.2)
================================================================================
run_appendix5_screen.py 와 동일 경로이나 세 곳이 다르다.

  1) 기하   : 상수 L 2.5 / A −0.5 / B 3.0  →  설계별 L_eff · A · B (부록 3-7.1.1)
  2) 상수   : Y1 · e · C · C_u · D_pw 를 설계마다 주입 (p2_constants.csv)
  3) e_C    : 고정값  →  빈별 ISO 281 Annex A.12 (그리스 · slight~typical · D_pw≥500)
              e_C = a·(1 − 1.677/D_pw^(1/3)),  a = min(1, 0.0177·κ^0.68·D_pw^0.55)
              MASTA 실측(0.888378)과 0.08% 이내 일치 확인됨

k 선정: dt=20 ε(k) 에서 중앙타겟(ε_UW·ε_Sys ∈ [0,3] 중 ε_Sys→1.5),
        창 없으면 ε_Sys≥0 중 최소(보수측), 전부 음수면 |ε_Sys| 최소. (부록 4·5 동일)

산출: P2_피로수명_Phase1/screen_k.csv · screen_eps_map.csv
"""
import csv
import os
import sys
import time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
DLCDIR = os.path.join(RES, "DLC별해석")
CONST = os.path.join(HERE, "P1_극한응력_Phase2", "p2_constants.csv")
OUTDIR = os.path.join(HERE, "P2_피로수명_Phase1")

NU50 = 294.637                 # Mobilith SHC 460 @ 50°C [mm²/s] — MASTA 실측 일치
P_EXP = 10.0 / 3.0
E_W = 9.0 / 8.0
DT0, DT = 0.1, 20.0
KS = np.round(np.arange(0.0, 3.0001, 0.01), 2)   # 260729 세분화 — 0.05 는 너무 성김(§8-3.2)
DESIGN_YEARS = 30.0
AISO_MAX = 50.0                # MASTA maximum_bearing_life_modification_factor
EC_K1, EC_C = 0.0177, 1.677    # ISO 281 Fig. A.12 (D_pw ≥ 500 mm)


def a_iso(kap, ratio):
    k = np.clip(kap, 0.1, 4.0)
    term = np.where(k < 0.4, 1.5859 - 1.3993 / k ** 0.054381,
                    np.where(k < 1.0, 1.5859 - 1.2348 / k ** 0.19087,
                             1.5859 - 1.2348 / k ** 0.071739))
    inner = 1.0 - term * np.minimum(ratio, 5.0) ** 0.4
    return np.clip(np.where(inner > 0, 0.1 * np.maximum(inner, 1e-12) ** -9.185,
                            AISO_MAX), None, AISO_MAX)


class Design:
    """설계 1건의 스크리닝 상수 묶음"""

    def __init__(self, r):
        self.tag = str(r["rank_mass"])
        self.L = float(r["L_eff_m"])
        self.A = float(r["A_m"])
        self.B = float(r["B_m"])
        self.Y1 = float(r["Y1"])
        self.e = float(r["e_lim"])
        self.C = float(r["C_kN"]) * 1e3
        self.Cu = float(r["Cu_kN"]) * 1e3
        self.Dpw = float(r["D_pw_mm"])          # [mm]
        self.meta = {k: r[k] for k in ("D_pw_mm", "alpha", "D_we_mm", "L_we_mm",
                                       "z1", "z2", "Z")}

    def kappa(self, rpm):
        return NU50 / (45000.0 * np.maximum(np.abs(rpm), 1e-6) ** -0.83
                       * self.Dpw ** -0.5)

    def eC(self, kap):
        a = np.minimum(1.0, EC_K1 * kap ** 0.68 * self.Dpw ** 0.55)
        return a * (1.0 - EC_C / self.Dpw ** (1.0 / 3.0))

    def statics_P(self, FX, FY, FZ, MX, MY):
        rax = FX * (self.B / self.L) - MY / self.L
        ray = FY * (self.B / self.L) + MX / self.L
        rbx = FX * (self.A / self.L) + MY / self.L
        rby = FY * (self.A / self.L) - MX / self.L
        fra, frb = np.hypot(rax, ray), np.hypot(rbx, rby)
        sa, sb = 0.5 * fra / self.Y1, 0.5 * frb / self.Y1
        case1 = FZ >= sa - sb
        faa = np.where(case1, FZ + sb, sa)
        fab = np.where(case1, -sb, -(sa - FZ))

        def P(fr, fa):
            r = np.abs(fa) / np.maximum(fr, 1e-9)
            return np.where(r <= self.e, fr, 0.4 * fr + self.Y1 * np.abs(fa))
        return P(fra, faa), P(frb, fab)

    def damage(self, P, rpm, rev):
        kap = self.kappa(rpm)
        ai = a_iso(kap, self.eC(kap) * self.Cu / np.maximum(P, 1.0))
        return rev / (ai * (self.C / np.maximum(P, 1.0)) ** P_EXP * 1e6)


def load_raw(name):
    return np.loadtxt(os.path.join(DLCDIR, name, "raw.csv"),
                      delimiter=",", skiprows=1)


def hub(A):
    Mx, My, Mz = A[:, 2] * 1e3, A[:, 3] * 1e3, A[:, 4] * 1e3
    Fx, Fy, Fz = A[:, 5] * 1e3, A[:, 6] * 1e3, A[:, 7] * 1e3
    return (-Fz, Fy, Fx, -Mz, My), A[:, 1]


def screen_one(dz, A, sf):
    (FX, FY, FZ, MX, MY), rpm = hub(A)
    H = np.stack([FX, FY, FZ, MX, MY], axis=1)
    n = len(rpm)
    Pu, Pd = dz.statics_P(FX, FY, FZ, MX, MY)
    rev = np.abs(rpm) / 60.0 * DT0
    DrefU = float(dz.damage(Pu, rpm, rev).sum())
    DrefD = float(dz.damage(Pd, rpm, rev).sum())
    if DrefU <= 0 or DrefD <= 0:
        return [], None
    lifeU, lifeD = DESIGN_YEARS / (DrefU * sf), DESIGN_YEARS / (DrefD * sf)
    lsys_ref = (lifeU ** -E_W + lifeD ** -E_W) ** (-1 / E_W)

    kp = int(round(DT / DT0))
    nb = n // kp
    if nb < 2:
        return [], None
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
        Pub, Pdb = dz.statics_P(*[rep[:, i] for i in range(5)])
        eU = float(dz.damage(Pub, rb, rev_b).sum() / DrefU - 1) * 100
        eD = float(dz.damage(Pdb, rb, rev_b).sum() / DrefD - 1) * 100
        lu = DESIGN_YEARS / (DrefU * (1 + eU / 100) * sf)
        ld = DESIGN_YEARS / (DrefD * (1 + eD / 100) * sf)
        ls = (lu ** -E_W + ld ** -E_W) ** (-1 / E_W)
        rows.append((float(k), eU, eD, (lsys_ref / ls - 1) * 100))
    return rows, nb


def pick_k(rows):
    """중앙타겟 — 부록 4·5 동일 규칙"""
    win = [r for r in rows if 0.0 <= r[1] <= 3.0 and 0.0 <= r[3] <= 3.0]
    if win:
        return min(win, key=lambda r: abs(r[3] - 1.5)), "center"
    pos = [r for r in rows if r[3] >= 0.0]
    if pos:
        return min(pos, key=lambda r: r[3]), "min_pos"
    return min(rows, key=lambda r: abs(r[3])), "min_abs"


def main():
    os.makedirs(OUTDIR, exist_ok=True)
    with open(CONST, encoding="utf-8-sig") as f:
        designs = [Design(r) for r in csv.DictReader(f)]
    with open(os.path.join(DLCDIR, "dlc_meta.csv"), encoding="utf-8-sig") as f:
        meta = {r["DLC"]: r for r in csv.DictReader(f)}
    dlcs = sorted(meta)
    print(f"[P2 Step 1] 설계 {len(designs)}건 × DLC {len(dlcs)}개 × k {len(KS)}수준")
    print(f"  모델: L_eff 기반 강체정역학 (부록 3-7.1.1) · e_C = ISO 281 A.12(빈별)")

    krows, maprows = [], []
    t0 = time.time()
    for di, dz in enumerate(designs, 1):
        t1 = time.time()
        nfail = 0
        for name in dlcs:
            sf = float(meta[name]["ScaleFactor"])
            rows, nb = screen_one(dz, load_raw(name), sf)
            if not rows:
                nfail += 1
                continue
            best, tag = pick_k(rows)
            krows.append(dict(design=dz.tag, DLC=name, k=best[0], ksel=tag,
                              nbin=nb, ScaleFactor=sf,
                              eps_UW=round(best[1], 4), eps_DW=round(best[2], 4),
                              eps_Sys=round(best[3], 4)))
            for r in rows:
                maprows.append(dict(design=dz.tag, DLC=name, k=r[0],
                                    eps_UW=round(r[1], 4), eps_DW=round(r[2], 4),
                                    eps_Sys=round(r[3], 4)))
        ks = [r["k"] for r in krows if r["design"] == dz.tag]
        es = [r["eps_Sys"] for r in krows if r["design"] == dz.tag]
        print(f"  [{di}/{len(designs)}] #{dz.tag:<4} L_eff {dz.L:.3f} "
              f"C {dz.C/1e3:>8,.0f} kN  →  k {min(ks):.2f}~{max(ks):.2f} "
              f"(평균 {sum(ks)/len(ks):.2f}) · ε_Sys {min(es):+.2f}~{max(es):+.2f} "
              f"· 실패 {nfail} · {time.time()-t1:.0f}s", flush=True)

    with open(os.path.join(OUTDIR, "screen_k.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(krows[0]))
        w.writeheader()
        w.writerows(krows)
    with open(os.path.join(OUTDIR, "screen_eps_map.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(maprows[0]))
        w.writeheader()
        w.writerows(maprows)
    print(f"\n[완료] {len(krows):,}행 · {(time.time()-t0)/60:.1f}분")
    print(f"[저장] {OUTDIR}/screen_k.csv · screen_eps_map.csv")


if __name__ == "__main__":
    main()
