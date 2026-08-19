"""
부록 1-9 — 유효하중중심(L_eff) 강체정역학 기반 피로수명 검토 (무인·순수 Python)
================================================================================
1-8에서 L_eff 반영 시 반경반력이 MASTA와 0.1% 일치함을 확인. 본 절은 그 L_eff 정역학을
1-6/1-7과 동일한 피로 경로(DLC1.2-d-s1·dt=20·k=0.26·30빈)에 적용해, 반경반력 정확도가
'피로손상'을 얼마나 재현하는지(축력 R_Z 잔존오차의 손상 전파) 검증한다.
반력식 = 사이징 부록 3-7.1.1 Pin.full(L_eff+자중) · MASTA/기존기하 = appendix1_life.csv 재사용.
"""
import csv
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
NAME = "DLC1.2-d-s1"
DT, K, DT0 = 20.0, 0.26, 0.1
SF = 38180.0
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")

# ── L_eff 기하 (사이징 부록 3-7.1.1, α=19°) ──
L_EFF, A_EFF, B_EFF = 3.616661, 0.058331, 3.558331
ZA, ZB, Y1 = -0.058331, 3.558331, 1.1617
# ── 피로 상수 (v1.3 50°C · pinstat 동일) ──
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


def bin_reps(data, dt, k):
    kp = int(round(dt / DT0))
    n = len(data)
    nb = n // kp
    edges = [(b * kp, (b + 1) * kp) for b in range(nb)]
    if edges[-1][1] < n:
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


def hub_masta(rec):
    return dict(FX=-rec["Fz"] * 1e3, FY=rec["Fy"] * 1e3, FZ=rec["Fx"] * 1e3,
                MX=-rec["Mz"] * 1e3, MY=rec["My"] * 1e3)


class Pin:
    def __init__(self, L_, A_, B_, zA, zB, Y1):
        self.L, self.A, self.B, self.zA, self.zB, self.Y1 = L_, A_, B_, zA, zB, Y1
        self.W = self.zW = None

    def base(self, u):
        rax = u["FX"] * (self.B / self.L) - u["MY"] / self.L
        ray = u["FY"] * (self.B / self.L) + u["MX"] / self.L
        rbx = u["FX"] * (self.A / self.L) + u["MY"] / self.L
        rby = u["FY"] * (self.A / self.L) - u["MX"] / self.L
        return rax, ray, rbx, rby

    def calibrate(self, u0, mUW_rx, mDW_rx):
        self.W = mUW_rx + mDW_rx - u0["FX"]
        _, _, rbx0, _ = self.base(u0)
        self.zW = self.zA + self.L * (mDW_rx - rbx0) / self.W
        return self.W, self.zW

    def full(self, u):
        rax, ray, rbx, rby = self.base(u)
        rax += self.W * (self.zB - self.zW) / self.L
        rbx += self.W * (self.zW - self.zA) / self.L
        fra, frb = math.hypot(rax, ray), math.hypot(rbx, rby)
        sa, sb = 0.5 * fra / self.Y1, 0.5 * frb / self.Y1
        ka = u["FZ"]
        faa, fab = (ka + sb, -sb) if ka >= sa - sb else (sa, -(sa - ka))
        return {"UW": (rax, ray, faa, fra), "DW": (rbx, rby, fab, frb)}


def damage_of(brg, rev, rpm):
    fr, fa = brg[3], brg[2]
    P = equiv_load(fr, fa)
    kap = kappa(rpm)
    ai = a_iso(kap, EC50 * CU_N / max(P, 1.0))
    l10m = ai * (C_N / max(P, 1.0)) ** P_EXP * 1e6
    return P, rev / l10m if l10m > 0 else 0.0


def main():
    reps = bin_reps(load_raw(NAME), DT, K)
    hubs = [hub_masta(rec) for _, _, rec in reps]
    D = os.path.join(HERE, NAME)
    react = list(csv.DictReader(open(os.path.join(D, "appendix1_reactions.csv"),
                                     encoding="utf-8-sig")))
    life = list(csv.DictReader(open(os.path.join(D, "appendix1_life.csv"),
                                    encoding="utf-8-sig")))
    # 자중 캘리브레이션 (빈1 MASTA R_X)
    pin = Pin(L_EFF, A_EFF, B_EFF, ZA, ZB, Y1)
    W, zW = pin.calibrate(hubs[0], float(react[0]["UW_RX_masta_kN"]) * 1e3,
                          float(react[0]["DW_RX_masta_kN"]) * 1e3)
    print(f"[L_eff 자중] W={W/1e3:,.1f} kN · z_W={zW:.4f} m (L={L_EFF:.4f})")

    rows = []
    tot = {"pin": [0.0, 0.0], "masta": [0.0, 0.0], "leff": [0.0, 0.0]}
    for (bi, rev, rec), u, lf in zip(reps, hubs, life):
        r = pin.full(u)
        row = {"bin": bi + 1, "rev": rev, "kappa": float(lf["kappa"])}
        for k, key in (("UW", 0), ("DW", 1)):
            P, dl = damage_of(r[k], rev, rec["rpm"])
            dp = float(lf[f"{k}_dmg_pin"]); dm = float(lf[f"{k}_dmg_masta"])
            tot["pin"][key] += dp; tot["masta"][key] += dm; tot["leff"][key] += dl
            row[f"{k}_P"] = P / 1e3
            row[f"{k}_d_pin"] = dp; row[f"{k}_d_leff"] = dl; row[f"{k}_d_masta"] = dm
            row[f"{k}_eps_pin"] = (dp/dm-1)*100 if dm else float("nan")
            row[f"{k}_eps_leff"] = (dl/dm-1)*100 if dm else float("nan")
        rows.append(row)

    with open(os.path.join(D, "appendix1_9_leff.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader(); w.writerows(rows)

    print("\n빈별 UW 손상오차 [(모델−MASTA)/MASTA %]:")
    import statistics as st
    for tag, key in (("기존기하(pin)", "eps_pin"), ("L_eff", "eps_leff")):
        for bk in ("UW", "DW"):
            v = [r[f"{bk}_{key}"] for r in rows]
            print(f"  {bk} {tag:14}: 평균 {st.mean(v):+7.2f}% ({min(v):+.1f}~{max(v):+.1f})")

    print("\n총량 D30 (×SF) · 수명:")
    E_W = 9 / 8
    for tag in ("pin", "masta", "leff"):
        dU, dD = tot[tag][0]*SF, tot[tag][1]*SF
        lU, lD = 30/dU, 30/dD
        lS = (lU**-E_W + lD**-E_W)**(-1/E_W)
        eU = (dU/(tot['masta'][0]*SF)-1)*100
        print(f"  {tag:6}: D30 UW {dU:.4f} DW {dD:.4f} · 수명 UW {lU:.1f} DW {lD:.1f} "
              f"Sys {lS:.1f} yr · UW편향 {eU:+.1f}%")
    print(f"\n[완료] appendix1_9_leff.csv ({len(rows)}빈)")


if __name__ == "__main__":
    main()
