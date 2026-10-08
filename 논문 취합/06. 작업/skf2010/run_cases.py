"""SKF(2010) 모델 검증 실행 — 계획서 §6.

    python run_cases.py        # V0 (M2 단위검사)
    python run_cases.py v1     # V1 (Leeds flat-top, SKF Fig 2 / Leeds Fig 3 대조)
"""
from __future__ import annotations

import csv
import os
import sys
import warnings

import numpy as np
import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

import conditions as C
import ehl_ripple as M2

RESULTS: list[tuple[str, str, str, bool]] = []   # (ID, 항목, 관측, 합격)


def rec(tid, label, observed, ok):
    RESULTS.append((tid, label, observed, bool(ok)))


# ---------------------------------------------------------------- V0-1


def v0_1_dimensionless():
    """Q, C 가 단위계에 무관한 무차원량인지 수치로 확인.

    SI(m, Pa, s) 와 (mm, MPa, s) 두 단위계에서 같은 값이 나와야 한다.
    """
    Ls, Ps = 1e3, 1e-6          # m -> mm, Pa -> MPa
    base = dict(r_a=0.3e-6, wx=2 * np.pi / 100e-6, wy=2 * np.pi / 150e-6,
                h=0.2e-6, E_eff=219e9, B_bulk=8.7e9,
                u2=0.20, u_bar=0.15, eta_x=1.2, eta_y=1.2)
    si = M2.particular_integral(**base)

    scaled = dict(
        r_a=base["r_a"] * Ls, wx=base["wx"] / Ls, wy=base["wy"] / Ls,
        h=base["h"] * Ls, E_eff=base["E_eff"] * Ps, B_bulk=base["B_bulk"] * Ps,
        u2=base["u2"] * Ls, u_bar=base["u_bar"] * Ls,
        eta_x=base["eta_x"] * Ps, eta_y=base["eta_y"] * Ps,
    )
    sc = M2.particular_integral(**scaled)

    eq = max(abs(si["Q"] - sc["Q"]) / abs(si["Q"]),
             abs(si["C"] - sc["C"]) / abs(si["C"]))
    # h_a 는 길이이므로 Ls 배가 되어야 한다
    eh = abs(sc["h_a"] - si["h_a"] * Ls) / abs(si["h_a"] * Ls)
    rec("V0-1", "Q, C 단위계 불변 (무차원)", f"상대차 {eq:.2e}", eq < 1e-12)
    rec("V0-1", "h_a 길이차원 스케일링", f"상대차 {eh:.2e}", eh < 1e-12)
    return si


# ---------------------------------------------------------------- V0-2


def v0_2_newtonian():
    """tau_m -> 0 에서 Eyring 등가점도가 Newtonian 으로 환원되는가 (식 19)."""
    eta = 1.2
    ex, ey = M2.eta_equivalent(eta, 0.0, 8e6)
    e1 = max(abs(ex - eta), abs(ey - eta)) / eta
    rec("V0-2", "식(19) tau_m=0 -> eta_x=eta_y=eta", f"상대차 {e1:.2e}", e1 < 1e-14)

    ex2, ey2 = M2.eta_equivalent(eta, 1.0e3, 8e6)      # tau_m/tau0 = 1.25e-4
    e2 = max(abs(ex2 - eta), abs(ey2 - eta)) / eta
    rec("V0-2", "식(19) tau_m/tau0 << 1 연속성", f"상대차 {e2:.2e}", e2 < 1e-6)

    ex3, ey3 = M2.eta_equivalent(eta, 8e6, 8e6)        # tau_m/tau0 = 1
    ok = ex3 < eta and ey3 < eta and ex3 < ey3
    rec("V0-2", "식(19) tau_m=tau0 전단박화 (eta_x<eta_y<eta)",
        f"eta_x={ex3:.4f}, eta_y={ey3:.4f}, eta={eta}", ok)


# ---------------------------------------------------------------- V0-3


def v0_3_pure_rolling():
    """순수구름 S=0 (u2=u_bar) -> Q=0, p_a=0, h_a=r_a (변형 없음)."""
    r_a = 0.3e-6
    kw = dict(r_a=r_a, wx=2 * np.pi / 100e-6, wy=2 * np.pi / 150e-6,
              h=0.2e-6, E_eff=219e9, B_bulk=8.7e9, u_bar=0.15,
              eta_x=1.2, eta_y=1.2)
    pr = M2.particular_integral(u2=0.15, **kw)
    ok = pr["Q"] == 0.0 and pr["p_a"] == 0 and abs(pr["h_a"] - r_a) < 1e-30
    rec("V0-3", "순수구름 -> Q=0, p_a=0, h_a=r_a",
        f"Q={pr['Q']:.1e}, |p_a|={abs(pr['p_a']):.1e}, h_a/r_a={pr['h_a'].real/r_a:.6f}", ok)

    # Q -> 0 연속성: p_a 가 (u2-u_bar) 에 1차 비례하는가
    amp = []
    for du in (1e-4, 1e-5, 1e-6):
        p = M2.particular_integral(u2=0.15 + du, **kw)
        amp.append(abs(p["p_a"]) / du)
    lin = max(abs(a / amp[-1] - 1.0) for a in amp)
    rec("V0-3", "Q->0 에서 p_a ∝ (u2-u_bar) 1차 수렴", f"기울기 편차 {lin:.2e}", lin < 1e-3)


# ---------------------------------------------------------------- V0-4


def v0_4_flattening_limit():
    """식 (18),(10),(11) 을 손으로 정리한 닫힌형과 대조하고 극한 거동을 본다.

        h_a / r_a = (1 - i Q C) / (1 - i Q (1 + C))

    이 항등식은 Q, C 전 범위에서 성립해야 한다. 극한은
      Q -> 0   : h_a -> r_a          (변형 없음)
      Q -> inf : h_a -> r_a C/(1+C)  (압축성이 남기는 잔류 리플)
      C = 0    : |h_a|/r_a = 1/sqrt(1+Q^2)  (비압축성, 1/Q 로 평탄화)
    """
    r_a = 0.3e-6
    kw = dict(r_a=r_a, wx=2 * np.pi / 100e-6, wy=0.0, h=0.2e-6,
              E_eff=219e9, u_bar=0.15, eta_x=1.2, eta_y=1.2)

    worst = 0.0
    for B in (8.7e9, 1.7e9, 1e30):
        for du in (1e-3, 1e-1, 1e1, 1e3, 1e5):
            p = M2.particular_integral(B_bulk=B, u2=0.15 + du, **kw)
            Q, Cv = p["Q"], p["C"]
            closed = (1.0 - 1j * Q * Cv) / (1.0 - 1j * Q * (1.0 + Cv))
            worst = max(worst, abs(p["h_a"] / r_a - closed))
    rec("V0-4", "h_a/r_a = (1-iQC)/(1-iQ(1+C)) 닫힌형 항등",
        f"최대 편차 {worst:.2e} (Q, C 15조합)", worst < 1e-12)

    p = M2.particular_integral(B_bulk=8.7e9, u2=0.15 + 1e5, **kw)
    Cv = p["C"]
    err = abs(abs(p["h_a"]) / r_a - Cv / (1 + Cv)) / (Cv / (1 + Cv))
    rec("V0-4", "Q->inf 에서 h_a/r_a -> C/(1+C)",
        f"h_a/r_a={abs(p['h_a'])/r_a:.6f}, C/(1+C)={Cv/(1+Cv):.6f}, 상대차 {err:.1e}",
        err < 1e-6)

    # 비압축성: |h_a|/r_a 가 1/Q 로 감소하는가 (du 10배 -> 비 10배)
    rs = []
    for du in (1e3, 1e4, 1e5):
        p2 = M2.particular_integral(B_bulk=1e30, u2=0.15 + du, **kw)
        rs.append(abs(p2["h_a"]) / r_a)
    decade = [rs[i] / rs[i + 1] for i in range(len(rs) - 1)]
    rec("V0-4", "비압축성(C=0): |h_a|/r_a ∝ 1/Q 로 평탄화",
        f"속도 10배당 비 {decade[0]:.3f}, {decade[1]:.3f}",
        all(abs(d - 10.0) < 0.05 for d in decade))


# ---------------------------------------------------------------- V0-5


def v0_5_complementary_wave():
    """식 (21) 수렴성과 물리적 타당성: beta = Im(psi) > 0 (감쇠), beta ∝ h^3 경향."""
    case = C.DEOLALIKAR
    eta = case.eta0 * np.exp(case.alpha * case.p_h)
    B = C.bulk_modulus(case.p_h)
    wx, wy = 2 * np.pi / 100e-6, 2 * np.pi / 150e-6

    betas, oks, its = [], [], []
    for h in (0.10e-6, 0.15e-6, 0.20e-6, 0.30e-6):
        psi, nit, ok = M2.solve_psi(wx, wy, h, case.E_eff, B, case.u_bar,
                                    case.u_bar, eta, eta)
        betas.append(psi.imag)
        oks.append(ok)
        its.append(nit)
    rec("V0-5", "식(21) 축차대입 수렴", f"반복 {min(its)}~{max(its)}회", all(oks))
    rec("V0-5", "감쇠율 beta = Im(psi) > 0", f"beta = {min(betas):.3e}~{max(betas):.3e} 1/m",
        all(b > 0 for b in betas))
    rec("V0-5", "beta 가 h 증가에 따라 단조증가",
        "단조증가" if all(np.diff(betas) > 0) else "비단조",
        bool(all(np.diff(betas) > 0)))

    # 순수미끄럼(정지 거친면 u2=0) -> omega_x' = 0, psi 는 순허수(전파 없이 감쇠만)
    psi0, _, ok0 = M2.solve_psi(wx, wy, 0.15e-6, case.E_eff, B, case.u_bar,
                                0.0, eta, eta)
    rec("V0-5", "u2=0 (정지 거친면) -> Re(psi)=0, 순수감쇠",
        f"Re={psi0.real:.2e}, Im={psi0.imag:.3e}", abs(psi0.real) < 1e-6 and psi0.imag > 0 and ok0)


# ---------------------------------------------------------------- V0-6


def v0_6_amplitude_reduction():
    """식 (24) 진폭감쇠 곡선의 극한과 단조성, 식 (22) 정의역."""
    nab = np.logspace(-2, 2, 200)
    ht = M2.total_amplitude(1.0, nab)
    rec("V0-6", "nabla->0 에서 h_t/r_a -> 1", f"{ht[0]:.6f}", abs(ht[0] - 1) < 2e-3)
    rec("V0-6", "nabla=100 에서 h_t/r_a -> 0", f"{ht[-1]:.2e}", ht[-1] < 1e-2)
    rec("V0-6", "h_t/r_a 단조감소", "단조감소" if all(np.diff(ht) < 0) else "비단조",
        bool(all(np.diff(ht) < 0)))

    # 식 (22) 밑수 1 + S/2 (U1): 논문 그림 조건 S = 0 (Fig 1/2a/7/8), +1, -1 (Fig 2b,c) 에서
    # 유한·비음수, 밑수가 음수인 S < -2 는 오류. (Newtonian, Q = 0 -> K = 1, nabla = 10)
    vals = {S: M2.ar_param_nn(10.0, 0.0, S)[0] for S in (0.0, 1.0, -1.0)}
    finite = all(np.isfinite(v) and v >= 0.0 for v in vals.values())
    try:
        M2.ar_param_nn(10.0, 0.0, -3.0)
        guarded = False
    except ValueError:
        guarded = True
    rec("V0-6", "식(22): S=0,+1,-1 에서 유한·비음수, S<-2 는 오류",
        f"S=0: {vals[0.0]:.3f}, +1: {vals[1.0]:.3f}, -1: {vals[-1.0]:.3f}, "
        f"S=-3 {'오류' if guarded else '통과(잘못)'}", finite and guarded)
    return nab, ht


# ---------------------------------------------------------------- V0-7


def v0_7_ar_param_crosscheck():
    """nabla = lam sqrt(M)/(a_x sqrt(L)) 정의 검산 (계획서 K10 flat-top 주기)."""
    lam = (90e-6 + 35e-6) * np.sqrt(2.0)            # = 176.8 um
    got = C.LEEDS.ar_param(lam)
    _, _, _, M_, L_ = C.LEEDS.dims
    hand = lam * np.sqrt(M_) / (C.LEEDS.a_x * np.sqrt(L_))
    rec("V0-7", "nabla 정의 검산 (Leeds, lam=176.8um)",
        f"nabla={got:.4f} (M={M_:.2f}, L={L_:.2f}, a_x={C.LEEDS.a_x*1e6:.1f}um)",
        abs(got - hand) < 1e-12)

    d = C.DEOLALIKAR
    rec("V0-7", "nabla (Deolalikar, lam=2*18um 범프폭)",
        f"nabla={d.ar_param(36e-6):.4f}", True)


# ---------------------------------------------------------------- V0-8


def v0_8_assembly():
    """조립 일관성: h_a + h_cf = h_t (계획서 §2.2(c))."""
    case = C.DEOLALIKAR
    out = M2.ripple_component(r_a=0.6e-6, wx=2 * np.pi / 100e-6,
                              wy=2 * np.pi / 150e-6, h=case.h_cen, case=case, S=0.0)
    resid = abs(out["h_a"] + out["h_cf"] - out["h_t"])
    rec("V0-8", "h_a + h_cf = h_t 항등", f"잔차 {resid:.2e} m", resid < 1e-18)
    rec("V0-8", "순수구름에서 h_a = r_a (복소부 0)",
        f"h_a={out['h_a'].real*1e6:.4f}+{out['h_a'].imag*1e6:.1e}j um",
        abs(out["h_a"].imag) < 1e-20)
    return out


# ---------------------------------------------------------------- 그림


def plot_v0(nab, ht, path="figures/V0_amplitude_reduction.png"):
    import os
    os.makedirs(os.path.dirname(path), exist_ok=True)
    fig, ax = plt.subplots(figsize=(7, 4.4))
    ax.semilogx(nab, ht, "k-", lw=2,
                label=r"eq.(24) with $\nabla$  (Venner-Lubrecht)")
    for S, ls in zip((0.0, 1.0, -1.0), ("--", "-.", ":")):
        nn = np.array([M2.ar_param_nn(v, 0.0, S)[0] for v in nab])
        ax.semilogx(nab, M2.total_amplitude(1.0, nn), ls, lw=1.6,
                    label=f"eq.(22) $\\nabla_{{nn}}$, S={S:+.0f}")
    ax.set_xlabel(r"$\nabla$")
    ax.set_ylabel(r"$h_{\rm t}/r_{\rm a}$")
    ax.set_title("V0-6  amplitude reduction: eq.(24) with eq.(22), Newtonian Q=0")
    ax.grid(True, which="both", alpha=0.3)
    ax.legend(fontsize=8)
    fig.tight_layout()
    fig.savefig(path, dpi=150)
    plt.close(fig)
    return path


# ---------------------------------------------------------------- main


def run_v0():
    v0_1_dimensionless()
    v0_2_newtonian()
    v0_3_pure_rolling()
    v0_4_flattening_limit()
    v0_5_complementary_wave()
    nab, ht = v0_6_amplitude_reduction()
    v0_7_ar_param_crosscheck()
    v0_8_assembly()
    png = plot_v0(nab, ht)

    print(f"{'ID':<7}{'항목':<44}{'관측':<46}판정")
    print("-" * 110)
    for tid, label, obs, ok in RESULTS:
        print(f"{tid:<7}{label:<44}{obs:<46}{'PASS' if ok else 'FAIL'}")
    npass = sum(r[3] for r in RESULTS)
    print("-" * 110)
    print(f"{npass}/{len(RESULTS)} PASS    그림: {png}")
    return npass == len(RESULTS)


# ================================================================= V1


def flat_top_patch(n=255, periods=3):
    """Leeds 정사각 flat-top 배열(계획서 K9): 윗변 90 um, 간격 35 um, 높이 0.30 um,
    구름방향 대비 45 도. 격자 주기는 x·y 모두 125*sqrt(2) um 이므로 한 변을 그 정수배로
    잡으면 FFT 가 정확히 주기적이다. 중심 결함은 원점(접촉 중심)에 놓는다.
    반환: (간극 섭동 r[iy, ix] = -(z - mean z), 한 변 L)"""
    P, half = 125e-6, 45e-6
    L = periods * np.sqrt(2.0) * P
    g = np.arange(n) * L / n
    X, Y = np.meshgrid(g, g)
    uc = ((X + Y) / np.sqrt(2.0) + P / 2) % P - P / 2
    vc = ((X - Y) / np.sqrt(2.0) + P / 2) % P - P / 2
    z = np.where((np.abs(uc) <= half) & (np.abs(vc) <= half), 0.30e-6, 0.0)
    return -(z - z.mean()), L


SKF_CENTRE_UM = 250.0   # 가정: SKF Fig 1 도메인(0~500 um) 중앙 = 접촉 중심 (사용자 결정 2026-09-29)


def load_fig2(key):
    """디지타이징한 SKF Fig 2 (digitize_fig2.py). 반환: 접촉중심 기준 x [m], dh [m], dp [Pa]."""
    d = np.genfromtxt(f"ref_data/V1_skf_fig2_{key}.csv", delimiter=",", skip_header=2,
                      encoding="utf-8")
    return (d[:, 0] - SKF_CENTRE_UM) * 1e-6, d[:, 1] * 1e-6, d[:, 2] * 1e9


def run_v1():
    """V1: Leeds flat-top (계획서 K9·K11), SKF Fig 2 와 점별 비교 — 판정 없이 보고.

    사용자 결정(2026-09-29): 식 (21) 은 논문 풀이법(Hooke 교대 풀이, U11), 곡선은 이미지 색상
    자동 추출, 스냅숏은 접촉중심 가정 + 거칠기 위상 1개를 Delta h 최소제곱으로 맞춤
    (Delta p 는 독립 비교). S = +-1 은 U9 두 안을 모두 계산해 보고만 한다(선택은 사용자).
    """
    warnings.simplefilter("ignore", RuntimeWarning)
    case, h = C.LEEDS, 0.302e-6                 # h: Leeds/SKF Table 1 중앙유막
    r, L = flat_top_patch()
    n = r.shape[0]
    w = 2.0 * np.pi * np.fft.fftfreq(n, d=L / n)
    wy, wx = np.meshgrid(w, w, indexing="ij")
    live = (wx != 0) | (wy != 0)
    eta = case.eta0 * np.exp(case.alpha * case.p_h)
    B = C.bulk_modulus(case.p_h)

    # ---- V1-1 식 (21) 논문 풀이법 수렴 + 독립 풀이(뉴턴+연속법) 근 대조
    for S in (0.0, 1.0, -1.0):
        tau = M2.tau_mean_eyring(eta, S * case.u_bar, h, case.tau0)
        ex, ey = M2.eta_equivalent(eta, tau, case.tau0)
        u2 = case.u_bar * (1 + S / 2)
        ps, it, ok = M2.solve_psi(wx, wy, h, case.E_eff, B, case.u_bar, u2, ex, ey)
        pn, _, _ = M2.solve_psi_newton(wx, wy, h, case.E_eff, B, case.u_bar, u2, ex, ey)
        diff = np.max(np.abs(ps - pn)[live] / np.maximum(np.abs(pn[live]), 1.0))
        rec("V1-1", f"S={S:+.0f}: 식(21) 논문 풀이 수렴, 뉴턴 근과 일치",
            f"{it}회 {'수렴' if ok else '미수렴'}, 차 {diff:.0e}, min Im {ps[live].imag.min():.1e}",
            ok and diff < 1e-9 and ps[live].imag.min() > 0)

    # ---- V1-2 점별 비교 (판정 없음)
    shifts = np.arange(0.0, 125e-6 * np.sqrt(2.0), 0.5e-6)   # y = 0 선의 한 주기
    fits = {}
    for S, key in ((0.0, "a"), (1.0, "b"), (-1.0, "c")):
        xs, dh_s, dp_s = load_fig2(key)
        m = np.abs(xs) <= case.a_x
        xs, dh_s, dp_s = xs[m], dh_s[m], dp_s[m]
        fh, fp = np.isfinite(dh_s), np.isfinite(dp_s)
        for mode in (("complex",) if S == 0 else ("complex", "magnitude")):
            dh, dp, resid, _ = M2.ripple_line(r, L, case, S, h, xs, mode, shifts=shifts)
            rec("V1-2", f"S={S:+.0f} {mode}: 단면 실수성", f"허수/실수 {resid:.0e}", resid < 1e-10)
            rms_h = np.sqrt(np.mean((dh[:, fh] - dh_s[fh]) ** 2, axis=1))
            k = int(np.argmin(rms_h))
            rms_p = np.sqrt(np.mean((dp[k, fp] - dp_s[fp]) ** 2))
            fits[(S, mode)] = dict(
                xs=xs, dh=dh[k], dp=dp[k], s=shifts[k], n_h=int(fh.sum()), n_p=int(fp.sum()),
                nr_h=rms_h[k] / np.ptp(dh_s[fh]), nr_p=rms_p / np.ptp(dp_s[fp]),
                cor_h=np.corrcoef(dh[k, fh], dh_s[fh])[0, 1],
                cor_p=np.corrcoef(dp[k, fp], dp_s[fp])[0, 1],
                pp_h=np.ptp(dh[k]), pp_h_s=np.ptp(dh_s[fh]),
                pp_p=np.ptp(dp[k]), pp_p_s=np.ptp(dp_s[fp]))

    fig2 = [plot_fig2_style("paper")] + [plot_fig2_style(m, fits) for m in ("complex", "magnitude")]
    fig2.append(plot_fig2_style("roughness", fits))

    print(f"{'ID':<7}{'항목':<42}{'관측':<48}판정")
    print("-" * 105)
    for tid, label, obs, ok in RESULTS:
        print(f"{tid:<7}{label:<42}{obs:<48}{'PASS' if ok else 'FAIL'}")
    print("-" * 105)
    print(f"\n[점별 비교 — 판정 없음]  접촉중심 x_SKF = {SKF_CENTRE_UM:.0f} um 가정, 위상 s 는 Delta h RMS 최소")
    print(f"{'S':>3} {'U9':<10}{'s[um]':>7} | {'Δh 정규화RMS':>11} {'상관':>6} {'진폭(본/SKF)':>16} | "
          f"{'Δp 정규화RMS':>11} {'상관':>6} {'진폭(본/SKF) GPa':>18}")
    for (S, mode), f in fits.items():
        print(f"{S:+3.0f} {mode:<10}{f['s']*1e6:7.1f} | {f['nr_h']:11.1%} {f['cor_h']:6.3f} "
              f"{f['pp_h']*1e6:7.3f}/{f['pp_h_s']*1e6:.3f} um | {f['nr_p']:11.1%} {f['cor_p']:6.3f} "
              f"{f['pp_p']/1e9:8.3f}/{f['pp_p_s']/1e9:.3f}")
    print("Fig 2 형식 그림: " + ", ".join(fig2))
    return all(r_[3] for r_ in RESULTS)


SKF_RED, SKF_BLUE = "#ef2b1d", "#3a68a4"     # SKF Fig 2 곡선 색 (이미지에서 추출)


def plot_fig2_style(source, fits=None, suffix="", z_edge=None):
    """SKF 2010 Fig 2 와 같은 형식의 3패널 그림 (사용자 결정 2026-09-29).

    source = "paper": 디지타이징 곡선(급변 구간 NaN 은 앞뒤 유효점을 이어 수직 급변 재현)
             "complex" / "magnitude": 본 계산(U9 안별). 곡선은 접촉 내부(|x| <= a_x)만.
             "roughness": 같은 위치의 y = 0 초기 거칠기 표면 높이 z (flat_top_patch 와 같은
             형상을 해석적으로, 모서리 수직). 위상 s 는 fits 값, 무늬 이동 r(x - s) 와 같음.
             z = 0.30 um (윗면) / 0 (홈), Delta h 와 부호 반대(Delta h > 0 = 홈).
    축은 논문과 같다: x 50~450 um (SKF 좌표 = 접촉중심 기준 x + 250 um, 위·아래),
    좌 Delta h -0.4~0.4 um, 우 Delta p -5~2 GPa, 점선 격자. 네 그림의 패널 위치가 같도록
    거칠기 그림도 같은 축 구성을 쓰고 오른쪽 축은 보이지 않게 둔다.
    """
    name = {"paper": "paper", "roughness": "initial_z"}.get(source, "ours_" + source)
    path = f"figures/V1_fig2_{name}{suffix}.png"   # suffix: 형상 시험 등 별도 그림(§3.9)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    fig, axes = plt.subplots(3, 1, figsize=(6.8, 15.0))
    caps = ("(a) Middle-plane profile, S = 0", "(b) Middle-plane profile, S = 1",
            "(c) Middle-plane profile, S = -1")
    P, half = 125e-6, 45e-6
    for ax, key, S, cap in zip(axes, ("a", "b", "c"), (0.0, 1.0, -1.0), caps):
        ax2 = ax.twinx()
        if source == "roughness":
            x = np.linspace(-200e-6, 200e-6, 4001)
            xu = x * 1e6 + SKF_CENTRE_UM
            modes = ("complex",) if S == 0 else ("complex", "magnitude")
            for mode, ls in zip(modes, ("-", ":")):
                s = fits[(S, mode)]["s"]
                uc = ((x - s) / np.sqrt(2.0) + P / 2) % P - P / 2      # y = 0: uc = vc
                # z_edge(q): 결함 중심에서 변까지 수직거리 q[m] → 높이[μm] (None 이면 수직 모서리)
                z = z_edge(np.abs(uc)) if z_edge else np.where(np.abs(uc) <= half, 0.30, 0.0)
                if mode == "complex":
                    ax.fill_between(xu, 0.0, z, where=z > 0, color="0.85")
                ax.plot(xu, z, color="k", lw=1.4, ls=ls,
                        label=f"s = {s * 1e6:.1f} um" + ("" if S == 0 else f" ({mode})"))
            ax.legend(fontsize=8, loc="lower right")
        else:
            if source == "paper":
                xk, hk, pk = load_fig2(key)
                mh, mp = np.isfinite(hk), np.isfinite(pk)
                xh, dh, xp, dp = xk[mh], hk[mh], xk[mp], pk[mp]
            else:
                f = fits[(S, "complex" if S == 0 else source)]
                xh = xp = f["xs"]
                dh, dp = f["dh"], f["dp"]
            xh_um, xp_um = xh * 1e6 + SKF_CENTRE_UM, xp * 1e6 + SKF_CENTRE_UM
            ax.plot(xh_um, dh * 1e6, color=SKF_RED, lw=1.6)
            ax2.plot(xp_um, dp / 1e9, color=SKF_BLUE, lw=1.6)
            i = int(np.argmin(dh))
            ax.annotate(r"$\Delta$ h", (xh_um[i] + 12, dh[i] * 1e6 + 0.02), fontsize=10)
            j = int(np.argmax(dp))
            ax2.annotate(r"$\Delta$ p", (xp_um[j] + 12, dp[j] / 1e9 + 0.35), fontsize=10)
        ax.set_xlim(50, 450)
        ax.set_ylim(-0.4, 0.4)
        ax2.set_ylim(-5, 2)
        ax.set_xticks(np.arange(50, 451, 50))
        ax.set_yticks(np.arange(-0.4, 0.41, 0.1))
        ax2.set_yticks(np.arange(-5, 3, 1))
        ax.grid(True, ls=":", color="0.4", lw=0.6)
        top = ax.twiny()
        top.set_xlim(ax.get_xlim())
        top.set_xticks(np.arange(50, 451, 50))
        if source == "roughness":
            lc, cap = "k", cap.replace("Middle-plane profile", "Initial roughness")
            ax.set_ylabel(r"z, [$\mu$ m]", color=lc)
            ax2.tick_params(axis="y", colors=(0, 0, 0, 0))            # 자리만 차지(정렬용)
            ax2.set_ylabel(r"$\Delta$ p, [GPa]", color=(0, 0, 0, 0))
        else:
            lc = SKF_RED
            ax.set_ylabel(r"$\Delta$ h, [$\mu$ m]", color=lc)
            ax2.tick_params(axis="y", colors=SKF_BLUE)
            ax2.set_ylabel(r"$\Delta$ p, [GPa]", color=SKF_BLUE)
        tc = "k" if source == "roughness" else SKF_BLUE
        top.set_xlabel(r"x, [$\mu$ m]", color=tc)
        top.tick_params(axis="x", colors=tc)
        ax.set_xlabel(r"x, [$\mu$ m]" + f"\n{cap}", color=lc)
        ax.tick_params(axis="x", colors=lc)
        ax.tick_params(axis="y", colors=lc)
        if source != "paper":
            ax.axvline(SKF_CENTRE_UM - C.LEEDS.a_x * 1e6, color="0.6", lw=0.8, ls="--")
            ax.axvline(SKF_CENTRE_UM + C.LEEDS.a_x * 1e6, color="0.6", lw=0.8, ls="--")
    title = {"paper": "SKF 2010 Fig 2 (digitized)",
             "roughness": "Initial roughness height z at y = 0 (shaded = flat top; sign opposite to Delta h)"
             }.get(source, f"This work (U9: {source}; S = 0 common), same format as SKF Fig 2; "
                           "dashed = contact edges")
    # 거칠기 그림은 제목을 보이지 않게 두고 자리만 차지(다른 그림과 패널 위치 일치)
    fig.suptitle(title, fontsize=9, color=(0, 0, 0, 0) if source == "roughness" else "k")
    fig.tight_layout()
    fig.savefig(path, dpi=150)
    plt.close(fig)
    return path


def report_psi(path="figures/V1_psi_map.png"):
    """식 (21) 의 해 psi = omega_x' + i beta (Leeds V1 패치, S = 0, +1, -1).
    omega_y = 0 선의 대표 파장 표 + 전 성분 (omega_x, omega_y) 분포 그림 (작업내역서 §3.6.4)."""
    case, h = C.LEEDS, 0.302e-6
    r, L = flat_top_patch()
    n = r.shape[0]
    w = 2.0 * np.pi * np.fft.fftfreq(n, d=L / n)
    wy, wx = np.meshgrid(w, w, indexing="ij")
    eta = case.eta0 * np.exp(case.alpha * case.p_h)
    B = C.bulk_modulus(case.p_h)
    picks = [(3, 0), (6, 0), (15, 0), (51, 0), (127, 0), (3, 3)]   # (kx, ky): 파장 L/k
    fig, axes = plt.subplots(2, 3, figsize=(13.5, 8.0), constrained_layout=True)
    ext = np.array([w.min(), w.max(), w.min(), w.max()]) * 1e-6
    print(f"{'S':>3} {'kx,ky':>7} {'lam_x[um]':>9} {'wx*u2/u_bar[1/m]':>17} {'omega_x_[1/m]':>13} "
          f"{'beta[1/m]':>11} {'omega_x_/(wx*u2/u_bar)':>22} {'beta/(wx*u2/u_bar)':>19}".replace("_[", "'[").replace("_/", "'/"))
    for j, S in enumerate((0.0, 1.0, -1.0)):
        tau = M2.tau_mean_eyring(eta, S * case.u_bar, h, case.tau0)
        ex, ey = M2.eta_equivalent(eta, tau, case.tau0)
        u2 = case.u_bar * (1 + S / 2)
        psi, _, ok = M2.solve_psi(wx, wy, h, case.E_eff, B, case.u_bar, u2, ex, ey)
        wx_u2 = wx * u2 / case.u_bar
        for kx, ky in picks:
            p, c = psi[ky, kx], wx_u2[ky, kx]
            print(f"{S:+3.0f} {kx:>3},{ky:<3} {L / kx * 1e6:9.1f} {c:17.4e} {p.real:13.4e} "
                  f"{p.imag:11.4e} {p.real / c:22.6f} {p.imag / c:19.4e}")
        with np.errstate(divide="ignore", invalid="ignore"):
            ratio = np.where(wx_u2 != 0, psi.real / wx_u2, np.nan)
            lb = np.log10(np.where(psi.imag > 0, psi.imag, np.nan))
        for i, (v, lab) in enumerate(((ratio, r"$\omega_x'\,\bar u/(\omega_x u_2)$"),
                                      (lb, r"$\log_{10}\beta$ [1/m]"))):
            im = axes[i, j].imshow(np.fft.fftshift(v), origin="lower", extent=ext, cmap="viridis")
            cb = fig.colorbar(im, ax=axes[i, j], label=lab)
            cb.formatter.set_useOffset(False)
            cb.update_ticks()
            axes[i, j].set_title(f"S = {S:+.0f}  " + lab)
            axes[i, j].set_xlabel(r"$\omega_x$ [1/$\mu$m]")
            axes[i, j].set_ylabel(r"$\omega_y$ [1/$\mu$m]")
    fig.savefig(path, dpi=130)
    plt.close(fig)
    print(f"그림: {path}")
    return True


if __name__ == "__main__":
    ok = {"v1": run_v1, "psi": report_psi}.get(sys.argv[1] if sys.argv[1:] else "", run_v0)()
    raise SystemExit(0 if ok else 1)
