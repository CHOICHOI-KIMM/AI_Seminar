"""
부록 1-9 후반 — 유효하중중심(L_eff) 강체정역학 피로수명 함대 적용 (111 DLC · 무인·순수 Python)
================================================================================
1-9 전반(단일 DLC)의 L_eff+자중 정역학을, 전 111 DLC에 점별(dt=0.1) 적용하여 함대 피로수명을
산출한다. 동일 파이프라인에서 '기존기하 스팬'(1-6/1-7)도 함께 계산해 통제된 3-way 비교
(기존기하 / L_eff / MASTA 참값)를 만든다. 자중(W·z_W)은 DLC 무관 상수 → DLC1.2-d-s1 빈1
MASTA R_X로 두 기하 각각 1회 캘리브레이션 후 전 DLC 공용. MASTA 참값 = 부록4 per_dlc.csv(dt=0.1).
"""
import csv
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
DT0 = 0.1
E_W = 9.0 / 8.0
SF_KEY = "ScaleFactor"
CAL_DLC = "DLC1.2-d-s1"
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")

# ── 기하 ──
Y1 = 1.1617
GEO = {  # 기존기하 스팬 (1-6/1-7)
    "geo": dict(L=2.5, A=-0.5, B=3.0, zA=0.5, zB=3.0),
    # L_eff (사이징 부록 3-7.1.1, α=19°)
    "leff": dict(L=3.616661, A=0.058331, B=3.558331, zA=-0.058331, zB=3.558331),
}
# ── 피로 상수 (v1.3 50°C) ──
C_N, CU_N, P_EXP = 22228e3, 3929e3, 10.0 / 3.0
NU50, EC50, DPW, E_LIM = 294.637, 0.888378, 3328.6, 0.5165


def a_iso(kap, ratio):
    k = min(max(kap, 0.1), 4.0)
    if k < 0.4:
        term = 1.5859 - 1.3993 / k ** 0.054381
    elif k < 1.0:
        term = 1.5859 - 1.2348 / k ** 0.19087
    else:
        term = 1.5859 - 1.2348 / k ** 0.071739
    inner = 1.0 - term * min(ratio, 5.0) ** 0.4
    return min(0.1 * max(inner, 1e-12) ** -9.185 if inner > 0 else 50.0, 50.0)


def kappa(rpm):
    return NU50 / (45000.0 * max(abs(rpm), 1e-6) ** -0.83 * DPW ** -0.5)


def equiv_load(fr, fa):
    if fr <= 0:
        return abs(fa) * Y1
    return fr if abs(fa) / fr <= E_LIM else 0.4 * fr + Y1 * abs(fa)


def load_raw(name):
    return [{k: float(v) for k, v in r.items()} for r in csv.DictReader(
        open(os.path.join(HERE, name, "raw.csv"), encoding="utf-8-sig"))]


def hub(rec):
    return (-rec["Fz"] * 1e3, rec["Fy"] * 1e3, rec["Fx"] * 1e3,
            -rec["Mz"] * 1e3, rec["My"] * 1e3)  # FX,FY,FZ(axial),MX,MY


class Pin:
    def __init__(self, g):
        self.L, self.A, self.B = g["L"], g["A"], g["B"]
        self.zA, self.zB = g["zA"], g["zB"]
        self.W = self.zW = None

    def base(self, FX, FY, MX, MY):
        rax = FX * (self.B / self.L) - MY / self.L
        ray = FY * (self.B / self.L) + MX / self.L
        rbx = FX * (self.A / self.L) + MY / self.L
        rby = FY * (self.A / self.L) - MX / self.L
        return rax, ray, rbx, rby

    def calibrate(self, u0, mUW_rx, mDW_rx):
        FX, FY, _, MX, MY = u0
        self.W = mUW_rx + mDW_rx - FX
        _, _, rbx0, _ = self.base(FX, FY, MX, MY)
        self.zW = self.zA + self.L * (mDW_rx - rbx0) / self.W
        return self.W, self.zW

    def damage_pair(self, u, rev):
        FX, FY, FZ, MX, MY = u
        rax, ray, rbx, rby = self.base(FX, FY, MX, MY)
        rax += self.W * (self.zB - self.zW) / self.L
        rbx += self.W * (self.zW - self.zA) / self.L
        fra, frb = math.hypot(rax, ray), math.hypot(rbx, rby)
        sa, sb = 0.5 * fra / Y1, 0.5 * frb / Y1
        faa, fab = (FZ + sb, -sb) if FZ >= sa - sb else (sa, -(sa - FZ))
        return fra, faa, frb, fab


def dmg_inc(fr, fa, rev, kap):
    P = equiv_load(fr, fa)
    ai = a_iso(kap, EC50 * CU_N / max(P, 1.0))
    l10m = ai * (C_N / max(P, 1.0)) ** P_EXP * 1e6
    return rev / l10m if l10m > 0 else 0.0


def main():
    meta = {r["DLC"]: r for r in csv.DictReader(
        open(os.path.join(HERE, "dlc_meta.csv"), encoding="utf-8-sig"))}
    ref = {r["DLC"]: r for r in csv.DictReader(
        open(os.path.join(HERE, "부록4_screening_dt20", "per_dlc.csv"),
             encoding="utf-8-sig"))}

    # ── 자중 캘리브레이션 (두 기하, DLC1.2-d-s1 빈1 MASTA R_X) ──
    react = list(csv.DictReader(open(os.path.join(HERE, CAL_DLC,
                 "appendix1_reactions.csv"), encoding="utf-8-sig")))
    mUW = float(react[0]["UW_RX_masta_kN"]) * 1e3
    mDW = float(react[0]["DW_RX_masta_kN"]) * 1e3
    # 캘리브레이션 대표하중 = 1-9 전반과 동일: dt=20,k=0.26 빈1
    craw = load_raw(CAL_DLC)
    kp = 200  # dt=20/dt0
    m = kp
    u0rec = {}
    for key in KSIG:
        mu = sum(craw[i][key] for i in range(m)) / m
        var = sum((craw[i][key] - mu) ** 2 for i in range(m)) / m
        u0rec[key] = mu + math.copysign(1.0, mu) * 0.26 * math.sqrt(var)
    u0 = hub(u0rec)
    pins = {}
    for tag, g in GEO.items():
        p = Pin(g)
        W, zW = p.calibrate(u0, mUW, mDW)
        pins[tag] = p
        print(f"[{tag:5}] W={W/1e3:,.1f} kN · z_W={zW:.4f} m (L={p.L:.4f})")

    # ── 111 DLC 점별 손상 ──
    names = sorted(d for d in os.listdir(HERE)
                   if d.startswith("DLC") and os.path.isfile(
                       os.path.join(HERE, d, "raw.csv")))
    tot = {"geo": [0.0, 0.0], "leff": [0.0, 0.0], "ref": [0.0, 0.0]}
    rows = []
    for name in names:
        if name not in meta:
            print(f"  ! meta 없음: {name}"); continue
        sf = float(meta[name][SF_KEY])
        data = load_raw(name)
        dsamp = {"geo": [0.0, 0.0], "leff": [0.0, 0.0]}
        for rec in data:
            rpm = rec["rpm"]
            rev = abs(rpm) / 60.0 * DT0
            if rev <= 0:
                continue
            kap = kappa(rpm)
            u = hub(rec)
            for tag in ("geo", "leff"):
                fra, faa, frb, fab = pins[tag].damage_pair(u, rev)
                dsamp[tag][0] += dmg_inc(fra, faa, rev, kap)
                dsamp[tag][1] += dmg_inc(frb, fab, rev, kap)
        dg = [dsamp["geo"][0] * sf, dsamp["geo"][1] * sf]
        dl = [dsamp["leff"][0] * sf, dsamp["leff"][1] * sf]
        rr = ref.get(name)
        dr = ([float(rr["D30_ref_UW"]), float(rr["D30_ref_DW"])]
              if rr and rr.get("D30_ref_UW") else [float("nan"), float("nan")])
        for i in (0, 1):
            tot["geo"][i] += dg[i]; tot["leff"][i] += dl[i]
            if not math.isnan(dr[i]):
                tot["ref"][i] += dr[i]

        def sysd(u_, d_):
            lu, ld = 30.0 / u_, 30.0 / d_
            ls = (lu ** -E_W + ld ** -E_W) ** (-1.0 / E_W)
            return 30.0 / ls
        eL_U = (dl[0] / dr[0] - 1) * 100 if dr[0] > 0 else float("nan")
        eG_U = (dg[0] / dr[0] - 1) * 100 if dr[0] > 0 else float("nan")
        if dr[0] > 0 and dr[1] > 0:
            eL_S = (sysd(*dl) / sysd(*dr) - 1) * 100
            eG_S = (sysd(*dg) / sysd(*dr) - 1) * 100
        else:
            eL_S = eG_S = float("nan")
        rows.append(dict(DLC=name, sf=sf, D30_ref_UW=dr[0], D30_ref_DW=dr[1],
                         D30_geo_UW=dg[0], D30_geo_DW=dg[1],
                         D30_leff_UW=dl[0], D30_leff_DW=dl[1],
                         eps_geo_UW=eG_U, eps_geo_Sys=eG_S,
                         eps_leff_UW=eL_U, eps_leff_Sys=eL_S))

    with open(os.path.join(HERE, "부록1_9_fleet_per_dlc.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(sorted(rows, key=lambda r: -(r["D30_ref_UW"]
                    if r["D30_ref_UW"] == r["D30_ref_UW"] else 0)))

    print(f"\n[함대 총량] ({len(rows)} DLC · 참값 유효 "
          f"{sum(1 for r in rows if r['D30_ref_UW']==r['D30_ref_UW'])})")
    print(f"{'구분':6} {'ΣD30_UW':>9} {'ΣD30_DW':>9} {'수명UW':>7} {'수명DW':>7} "
          f"{'수명Sys':>8} {'UW편향':>8} {'Sys편향':>8}")
    refU, refD = tot["ref"]
    lsr = (( 30/refU) ** -E_W + (30/refD) ** -E_W) ** (-1/E_W)
    for tag, lab in (("geo", "기존기하"), ("leff", "L_eff"), ("ref", "MASTA참값")):
        u, d = tot[tag]
        lu, ld = 30/u, 30/d
        ls = (lu ** -E_W + ld ** -E_W) ** (-1/E_W)
        eu = (u / refU - 1) * 100
        es = (lsr / ls - 1) * 100
        print(f"{lab:6} {u:9.4f} {d:9.4f} {lu:7.2f} {ld:7.2f} {ls:8.2f} "
              f"{eu:+7.1f}% {es:+7.1f}%")
    print("\n[완료] 부록1_9_fleet_per_dlc.csv")


if __name__ == "__main__":
    main()
