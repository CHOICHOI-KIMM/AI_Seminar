"""
DLC별 해석 · 1단계 — 전 DLC 사전 스크리닝 (MASTA 불필요)
========================================================
부록 9 스크리닝 경로 준용: 5분력 +kσ → 핀지지 정역학 → P → 손상(ISO281 식34~36, 50°C).
참값 = dt 0.1 점별 동일 경로. k 0~1.5(0.05) × dt {20,10,4,2,1,0.6}.
- κ < 0.1 은 0.1 로 클립하고 발생 점수 기록 (ISO 281 범위 밖 → §9-5 Q6)
- 최적 (dt,k): §4-1 정책스택 — 연속 k 중앙 타겟팅(ε_Sys→1.5), 여유<0.5%p면 dt 축소
산출: DLC폴더/screen_eps_map.csv + dlc_master_summary.csv + 총괄 xlsx + md 진행 마커
"""
import csv
import glob
import math
import os
import sys
import time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)

# ── 상수 (프로세스_v1 §3-3.3, §8-3) ──
L_, A_, B_ = 2.5, -0.5, 3.0
E_LIM, Y1 = 0.5165, 1.1617
C_N, CU_N, P_EXP = 22228e3, 3929e3, 10.0 / 3.0
NU50, EC50, DPW = 294.637, 0.888378, 3328.6
E_W = 9.0 / 8.0
DT0 = 0.1
DTS = [20, 10, 4, 2, 1, 0.6]
KS = np.round(np.arange(0.0, 3.0001, 0.05), 2)   # 극저속 DLC 대응 상한 3.0 (260723)
DESIGN_YEARS = 30.0
DOC = os.path.join(ROOT, "DLC기반_피로해석_DLC별해석.md")
MS, ME = "<!-- DLC_PROGRESS_START -->", "<!-- DLC_PROGRESS_END -->"
MASTER = os.path.join(HERE, "dlc_master_summary.csv")


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
    """raw 배열 → (FX,FY,FZ,MX,MY) [N·Nm], rpm."""
    Mx, My, Mz = A[:, 2] * 1e3, A[:, 3] * 1e3, A[:, 4] * 1e3
    Fx, Fy, Fz = A[:, 5] * 1e3, A[:, 6] * 1e3, A[:, 7] * 1e3
    return (-Fz, Fy, Fx, -Mz, My), A[:, 1]


def screen_one(A, sf):
    """→ (rows[list], summary[dict])"""
    (FX, FY, FZ, MX, MY), rpm = hub(A)
    H = np.stack([FX, FY, FZ, MX, MY], axis=1)
    n = len(rpm)
    kap_pts = NU50 / (45000.0 * np.maximum(np.abs(rpm), 1e-6) ** -0.83 * DPW ** -0.5)
    n_kclip = int((kap_pts < 0.1).sum())
    # 참값 (dt0.1 점별)
    Pu, Pd = statics_P(FX, FY, FZ, MX, MY)
    rev = np.abs(rpm) / 60.0 * DT0          # |rpm| — 역회전도 회전수 적산 (MASTA |n| 실측 정합)
    DrefU, DrefD = float(damage(Pu, rpm, rev).sum()), float(damage(Pd, rpm, rev).sum())
    out_rows = []
    if DrefU <= 0 or DrefD <= 0:
        return out_rows, dict(valid=False, n_kclip=n_kclip, DrefU=DrefU, DrefD=DrefD)
    d30U, d30D = DrefU * sf, DrefD * sf
    lifeU = DESIGN_YEARS / d30U if d30U > 0 else float("inf")
    lifeD = DESIGN_YEARS / d30D if d30D > 0 else float("inf")
    lsys_ref = (lifeU ** -E_W + lifeD ** -E_W) ** (-1 / E_W)

    eps = {}
    for dt in DTS:
        kp = int(round(dt / DT0))
        nb = n // kp
        if nb < 2:
            continue
        Hb = H[:nb * kp].reshape(nb, kp, 5)
        rb = rpm[:nb * kp].reshape(nb, kp).mean(1)
        mu, sd = Hb.mean(1), Hb.std(1)
        rev_b = np.abs(rb) / 60.0 * (kp * DT0)   # |빈 평균 rpm|
        # 잔여점 마지막 빈 편입
        if nb * kp < n:
            seg = np.vstack([Hb[-1], H[nb * kp:]])
            mu[-1], sd[-1] = seg.mean(0), seg.std(0)
        for k in KS:
            rep = mu + np.sign(mu) * k * sd
            Pub, Pdb = statics_P(*[rep[:, i] for i in range(5)])
            eU = float(damage(Pub, rb, rev_b).sum() / DrefU - 1) * 100
            eD = float(damage(Pdb, rb, rev_b).sum() / DrefD - 1) * 100
            # 시스템: 손상→수명→조합
            lu = DESIGN_YEARS / (DrefU * (1 + eU / 100) * sf)
            ld = DESIGN_YEARS / (DrefD * (1 + eD / 100) * sf)
            ls = (lu ** -E_W + ld ** -E_W) ** (-1 / E_W)
            eS = (lsys_ref / ls - 1) * 100
            eps[(k, dt)] = (eU, eD, eS)
            out_rows.append((k, dt, eU, eD, eS))

    # 최적 (dt,k) — §4-1 정책스택 (260723 개정):
    #  연속 k(선형보간) + 중앙 타겟팅(ε_Sys→1.5) · ε_Sys 여유<0.5%p(창 협소)면 dt 한 단계 축소
    best = None
    fallback = None
    for dt in DTS:
        ks = np.array(sorted({kk for (kk, dd) in eps if dd == dt}))
        if len(ks) < 2:
            continue
        eU = np.array([eps[(kk, dt)][0] for kk in ks])
        eD = np.array([eps[(kk, dt)][1] for kk in ks])
        eS = np.array([eps[(kk, dt)][2] for kk in ks])
        kf = np.linspace(ks.min(), ks.max(), 3001)
        u, d_, sv = np.interp(kf, ks, eU), np.interp(kf, ks, eD), np.interp(kf, ks, eS)
        okm = (u >= 0) & (u <= 3) & (sv >= 0) & (sv <= 3)
        if not okm.any():
            continue
        idx = np.where(okm)[0]
        j = idx[np.argmin(np.abs(sv[idx] - 1.5))]      # 중앙 타겟
        cand = (dt, round(float(kf[j]), 2),
                float(u[j]), float(d_[j]), float(sv[j]))
        if fallback is None:
            fallback = cand                              # 가장 큰 dt의 최선값 보관
        if sv[j] >= 0.5:                                 # 여유 확보 시 채택 (dt 보존)
            best = cand
            break
    if best is None:
        best = fallback                                  # 전 dt 창 협소 → 최대 dt 최선값
    return out_rows, dict(valid=True, n_kclip=n_kclip,
                          d30U=d30U, d30D=d30D, lifeU=lifeU, lifeD=lifeD,
                          lsys=lsys_ref, best=best,
                          kap_min=float(kap_pts.min()), kap_max=float(kap_pts.max()))


def update_md(done, total, t0, notes):
    L = [MS, "", f"> 1단계 스크리닝 진행: **{done}/{total} DLC** "
         f"(경과 {time.time()-t0:.0f}s) · κ클립 발생 DLC {notes['kclip']}개 · "
         f"참값손상 무의미(스킵) {notes['skip']}개", "", ME]
    txt = open(DOC, encoding="utf-8").read()
    if MS in txt and ME in txt:
        txt = txt.split(MS)[0] + chr(10).join(L) + txt.split(ME, 1)[1]
        open(DOC, "w", encoding="utf-8").write(txt)


def main():
    meta = {r["DLC"]: r for r in csv.DictReader(open(
        os.path.join(HERE, "dlc_meta.csv"), encoding="utf-8-sig"))}
    dlcs = sorted(meta)
    print(f"[대상] {len(dlcs)} DLC")
    t0 = time.time()
    master = []
    notes = {"kclip": 0, "skip": 0}
    for i, name in enumerate(dlcs, 1):
        d = os.path.join(HERE, name)
        A = np.genfromtxt(os.path.join(d, "raw.csv"), delimiter=",",
                          skip_header=1, encoding="utf-8-sig")
        sf = float(meta[name]["ScaleFactor"])
        rows, sm = screen_one(A, sf)
        with open(os.path.join(d, "screen_eps_map.csv"), "w", newline="",
                  encoding="utf-8-sig") as f:
            w = csv.writer(f)
            w.writerow(["k", "dt_s", "eps_UW_pct", "eps_DW_pct", "eps_Sys_pct"])
            w.writerows(rows)
        if sm["n_kclip"] > 0:
            notes["kclip"] += 1
        row = {"DLC": name, "ScaleFactor": sf,
               "rpm_mean": float(meta[name]["rpm_mean"]),
               "rpm_CV_pct": float(meta[name]["rpm_CV_pct"]),
               "T3P_s": float(meta[name]["T3P_s"]),
               "dt_rule_max_s": float(meta[name]["dt_rule_max_s"]),
               "n_rpm_lt1": int(meta[name]["n_rpm_lt1"]),
               "n_kappa_clip": sm["n_kclip"]}
        if not sm["valid"]:
            notes["skip"] += 1
            row.update({"valid": 0})
        else:
            dt_b, k_b, eU, eD, eS = (sm["best"] if sm["best"]
                                     else (None, None, None, None, None))
            row.update({"valid": 1,
                        "D30_UW_scr": sm["d30U"], "D30_DW_scr": sm["d30D"],
                        "life_UW_scr_yr": sm["lifeU"], "life_DW_scr_yr": sm["lifeD"],
                        "life_Sys_scr_yr": sm["lsys"],
                        "best_dt_s": dt_b, "best_k": k_b,
                        "eps_UW_at_best": eU, "eps_DW_at_best": eD,
                        "eps_Sys_at_best": eS,
                        "kappa_min": sm["kap_min"], "kappa_max": sm["kap_max"]})
        master.append(row)
        if i % 10 == 0 or i == len(dlcs):
            print(f"  … {i}/{len(dlcs)}  ({time.time()-t0:.0f}s)")
            update_md(i, len(dlcs), t0, notes)
    # master csv
    keys = []
    for r in master:
        for kk in r:
            if kk not in keys:
                keys.append(kk)
    with open(MASTER, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=keys)
        w.writeheader()
        w.writerows(master)
    print(f"[저장] {MASTER} ({len(master)}행)")
    update_md(len(dlcs), len(dlcs), t0, notes)


if __name__ == "__main__":
    main()
