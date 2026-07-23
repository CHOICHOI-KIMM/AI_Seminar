"""
부록 9 · 1단계 통계적 사전 스크리닝 (MASTA 불필요)
==================================================
경로(본해석과 동일 벡터 경로): 성분 +kσ → 핀지지 정역학 → P → 손상적분
- 5분력(F_X,F_Y,F_Z,M_X,M_Y)에만 +kσ. M_Z·rpm 은 빈 산술평균 (M_Z 는 반력 무영향)
- 참값 = dt 0.1 점별(빈당 1점, σ=0 → k 무관)을 동일 경로로 계산
- p승평균 미사용. 후보 k 를 그리드로 직접 대입하는 brute-force
산출: ε(k,dt) 맵 CSV + 그림 + 프로세스_v1.md §9-6 갱신
"""
import os
import sys
import csv

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))          # Results/부록9
ROOT = os.path.dirname(os.path.dirname(HERE))              # Python 자동화
sys.path.insert(0, ROOT)
from c1_pin import parse_dlc, hub, DLC                     # noqa: E402

import matplotlib                                          # noqa: E402
matplotlib.use("Agg")
import matplotlib.pyplot as plt                            # noqa: E402
import matplotlib.ticker as mticker                        # noqa: E402

# ── 상수 ──
Z_P, Z_A, Z_B = 0.0, 0.5, 3.0
L, A_, B_ = Z_B - Z_A, Z_P - Z_A, Z_B - Z_P
E_LIM, Y1 = 0.5165, 1.1617
C_N, CU_N, P_EXP = 22228e3, 3929e3, 10.0 / 3.0
NU50, EC50, DPW = 294.637, 0.888378, 3328.6                # 50°C (§8-3)
SCALE = 45040.0
DT0 = 0.1
DTS = [60, 20, 10, 6, 4, 2, 1, 0.6, 0.4, 0.3, 0.2]
KS = np.round(np.arange(0.0, 1.5001, 0.05), 2)
TARGET_DTS = [10, 6, 4, 2, 1, 0.6, 0.4, 0.3, 0.2]          # 1차 목표 dt ≤ 10
DOC = os.path.join(ROOT, "DLC기반_피로해석_프로세스_v1.md")
MARK_S, MARK_E = "<!-- APPENDIX_9_START -->", "<!-- APPENDIX_9_END -->"


def a_iso(kap, ratio):
    """ISO 281 식(34)~(36) 반경 롤러 — 벡터화."""
    k = np.clip(kap, None, 4.0)
    term = np.where(k < 0.4, 1.5859 - 1.3993 / k ** 0.054381,
                    np.where(k < 1.0, 1.5859 - 1.2348 / k ** 0.19087,
                             1.5859 - 1.2348 / k ** 0.071739))
    inner = 1.0 - term * ratio ** 0.4
    out = np.where(inner > 0, 0.1 * np.maximum(inner, 1e-12) ** -9.185, 50.0)
    return np.clip(out, None, 50.0)


def statics_P(u):
    """u: (...,5)=[FX,FY,FZ,MX,MY] → P_UW, P_DW."""
    FX, FY, FZ, MX, MY = (u[..., i] for i in range(5))
    rax = FX * (B_ / L) - MY / L
    ray = FY * (B_ / L) + MX / L
    rbx = FX * (A_ / L) + MY / L
    rby = FY * (A_ / L) - MX / L
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
    kap = NU50 / (45000.0 * rpm ** -0.83 * DPW ** -0.5)
    ai = a_iso(kap, EC50 * CU_N / P)
    l10m = ai * (C_N / P) ** P_EXP * 1e6
    return rev / l10m


def main():
    data = parse_dlc(DLC)
    H = np.array([[hub(r)[k] for k in ("FX", "FY", "FZ", "MX", "MY")] for r in data])
    rpm = np.array([r["rpm"] for r in data])
    n = len(data)
    print(f"[시계열] {n}점  | k 그리드 {len(KS)}개 (0~1.5, 0.05)  | dt {len(DTS)}종")

    # 참값: dt=0.1 점별 (동일 경로)
    Pu, Pd = statics_P(H)
    rev_pt = rpm / 60.0 * DT0
    Dref = {"UW": damage(Pu, rpm, rev_pt).sum(), "DW": damage(Pd, rpm, rev_pt).sum()}
    print(f"[참값] D_UW={Dref['UW']:.6e}  D_DW={Dref['DW']:.6e} (표본, 핀지지 경로)")

    rows = []          # (k, dt, eps_UW, eps_DW)
    for dt in DTS:
        kpts = int(round(dt / DT0))
        nb = n // kpts
        Hb = H[:nb * kpts].reshape(nb, kpts, 5)
        rb = rpm[:nb * kpts].reshape(nb, kpts).mean(1)
        mu, sd = Hb.mean(1), Hb.std(1)                    # (nb,5) 모표준편차
        rev_b = rb / 60.0 * (kpts * DT0)
        # 잔여점 마지막 빈 편입
        if nb * kpts < n:
            seg = H[nb * kpts:]
            mu[-1] = np.vstack([Hb[-1], seg]).mean(0)
            sd[-1] = np.vstack([Hb[-1], seg]).std(0)
        for k in KS:
            rep = mu + np.sign(mu) * k * sd
            Pu_b, Pd_b = statics_P(rep)
            eU = (damage(Pu_b, rb, rev_b).sum() / Dref["UW"] - 1) * 100
            eD = (damage(Pd_b, rb, rev_b).sum() / Dref["DW"] - 1) * 100
            rows.append((k, dt, eU, eD))

    # CSV 저장
    out_csv = os.path.join(HERE, "ksigma_screen_eps_map.csv")
    with open(out_csv, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.writer(f)
        w.writerow(["k", "dt_s", "eps_UW_pct", "eps_DW_pct"])
        w.writerows(rows)
    print(f"[저장] {out_csv}  ({len(rows)}행)")

    E = {(k, dt): (eU, eD) for k, dt, eU, eD in rows}

    # dt별 최소 k (0 ≤ ε_UW ≤ 3)
    kmin = {}
    for dt in DTS:
        ok = [k for k in KS if 0.0 <= E[(k, dt)][0] <= 3.0]
        kmin[dt] = min(ok) if ok else None

    # 전역 k (TARGET_DTS 전부 만족)
    kglob = None
    for k in KS:
        if all(0.0 <= E[(k, dt)][0] <= 3.0 for dt in TARGET_DTS):
            kglob = k
            break

    print("\n[dt별 ε_UW(k=0) → 최소 k]")
    for dt in DTS:
        print(f"  dt={dt:<5g} ε(k=0)={E[(0.0, dt)][0]:+7.2f}%   k_min={kmin[dt]}")
    print(f"\n[전역 최적 k (dt≤10 전부 0≤ε≤3%)] k* = {kglob}")
    if kglob is not None:
        print("  검증:", {dt: round(E[(kglob, dt)][0], 2) for dt in TARGET_DTS})

    # 그림: ε vs k 곡선 (dt별)
    plt.rcParams["font.family"] = "Malgun Gothic"
    plt.rcParams["axes.unicode_minus"] = False
    fig, ax = plt.subplots(figsize=(9.5, 5.8))
    cmap = plt.get_cmap("viridis")
    for i, dt in enumerate(DTS):
        ax.plot(KS, [E[(k, dt)][0] for k in KS], "-",
                color=cmap(i / (len(DTS) - 1)), label=f"dt={dt:g}s", lw=1.6)
    ax.axhspan(0, 3, color="#2ecc71", alpha=0.15, label="목표 0~+3%")
    ax.axhline(0, color="k", lw=1)
    if kglob is not None:
        ax.axvline(kglob, color="#c0392b", ls="--", lw=1.5)
        ax.text(kglob, ax.get_ylim()[1], f" k*={kglob}", color="#c0392b", va="top")
    ax.set_xlabel("k  (대표값 = μ + sign(μ)·k·σ)")
    ax.set_ylabel("손상 편향 ε_UW [%]  (참값 dt=0.1, 핀지지 경로)")
    ax.set_title("부록 9 · 1단계 스크리닝 — ε(k, dt) 맵 (UW, 50°C 상수)")
    ax.legend(fontsize=8, ncol=2)
    ax.grid(alpha=0.3)
    png = os.path.join(HERE, "ksigma_screen_map.png")
    fig.tight_layout(); fig.savefig(png, dpi=140); plt.close(fig)
    print(f"[저장] {png}")

    # 문서 §9-6 갱신
    Lm = [MARK_S, "", "#### 9-6.1 [1단계 완료] 사전 스크리닝 결과 (MASTA 0회)", "",
          f"> 경로: 성분 +kσ → 핀지지 정역학 → P → 손상(ISO281 식34~36, 50°C 상수). "
          f"참값 = dt 0.1 점별 동일 경로. k 그리드 0~1.5(0.05).", ""]
    Lm.append("| dt (s) | ε(k=0) 빈평균 | k_min (0≤ε≤3%) | ε(k*) |")
    Lm.append("|-------:|-------------:|---------------:|------:|")
    for dt in DTS:
        e0 = E[(0.0, dt)][0]
        ek = E[(kglob, dt)][0] if kglob is not None else float("nan")
        km = f"{kmin[dt]:.2f}" if kmin[dt] is not None else "불가"
        Lm.append(f"| {dt:g} | {e0:+.2f}% | {km} | {ek:+.2f}% |")
    Lm += ["", f"**전역 최적 k\\*** (dt≤10 전 구간 0≤ε≤+3% 동시만족 최소값): **{kglob}**", "",
           "**표 데이터의 정의 및 조건 (개조식)**", "",
           "1. **공통 조건**",
           "   - 시계열: DLC1.2-c-s1 (6,001점, Δt₀=0.1 s) · 주지표: **UW** 손상",
           "   - 대표값: 5분력(F_X,F_Y,F_Z,M_X,M_Y) 각각 `μ + sign(μ)·k·σ` (σ=모표준편차 ÷n). "
           "M_Z·rpm은 빈 산술평균",
           "   - 계산 경로: 대표하중 → **핀지지 정역학**(§3-3.3, L=2.5 m) → 등가하중 P(카탈로그 X·Y) "
           "→ 손상. **MASTA 0회**",
           "   - 손상 모델: `D = Σ N_b / [a_ISO·(C/P)^p·10⁶]`, C=22,228 kN, p=10/3, "
           "a_ISO=ISO 281 식(34~36) 해석식 + **50 °C 상수**(ν=294.637 mm²/s, e_C=0.8884, D_pw=3,328.6 mm)",
           "   - 사이클: `N_b = (r̄_b/60)·dt` (빈 평균 rpm)",
           "2. **참값 (분모)**",
           "   - dt=0.1 **점별**(빈당 1점 → σ=0 → k와 무관하게 고정)을 **동일 경로**로 계산",
           "   - 핀지지 정역학의 절대편향(UW Fr +32%)은 분자·분모에 공통 → ε에서는 근사적으로 상쇄"
           " (따라서 ε는 '빈 압축+kσ 보정'의 상대 효과만 반영, 최종 확정은 2단계 MASTA)",
           "3. **열 정의**",
           "   - `dt (s)`: 빈 폭. 빈수 = 600/dt (예: dt=1 → 600빈)",
           "   - `ε(k=0) 빈평균`: k=0, 즉 **부록 8 빈 산술평균과 동일 방법**의 손상 편향"
           " `ε = (D−D_ref)/D_ref×100`. **음수 = 손상 과소 = 비보수측**",
           "   - `k_min (0≤ε≤3%)`: 해당 dt에서 **보수측(ε≥0)이면서 +3% 이내**를 만족하는 최소 k"
           " (그리드 0~1.5, 0.05 간격). '불가' = 만족하는 k 없음",
           "   - `ε(k*)`: 전역 최적 k\\*를 일괄 적용했을 때의 편향 — 전부 +부호(보수측)이며 ≤3%인지 확인용",
           "",
           f"![스크리닝 맵](Results/부록9/ksigma_screen_map.png)", "",
           "- ε(k)는 k에 대해 단조증가 → 최소 k 탐색이 유일해를 가짐",
           "- 2단계(MASTA) k 후보: k\\* 및 인접값 검토 후 추후 결정(§9-5 Q3)", "", MARK_E]
    txt = open(DOC, encoding="utf-8").read()
    if MARK_S in txt and MARK_E in txt:
        txt = txt.split(MARK_S)[0] + "\n".join(Lm) + txt.split(MARK_E, 1)[1]
        open(DOC, "w", encoding="utf-8").write(txt)
        print("[갱신] §9-6")


if __name__ == "__main__":
    main()
