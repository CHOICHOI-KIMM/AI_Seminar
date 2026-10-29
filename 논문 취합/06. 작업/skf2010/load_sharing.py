"""M3 하중분할 (계획서 §2.3 · 논문 §4.2).

절차(논문 step 1–7):
  1. h_mean = h_cen (Hamrock–Dowson), p_mean = p_h (패치 = 접촉 중심)
  2. phi_bl 가정 → F_dry = phi_bl F (평균압력 phi_bl p_mean)
  3. M2: 평균간극 c_rho h_mean 로 리플 → h_lub = c_rho h_mean + Δh
       c_rho = (0.59 + 1.34 phi p)/(0.59 + phi p) [GPa] = density_ratio(phi p_mean)
  4. M1: 간극 h_lub, 평균압력 phi p_mean → h_dry, p_dry
  5. h_tran = IFFT{성분마다 |h̃| 가 큰 쪽 복소계수} (4.2-5)
  6. p_tran = IFFT{w⁻¹ FFT(h_tran − r)} (4.2-6, 변위 u = h − r: M1·M2 부호 규약)
  7. mean⟨h_tran⟩ = c_rho h_mean 이 되는 phi_bl (유량평형, Johnson et al. [4])

사용자 결정(2026-10-09, 원문·후속 SKF·Johnson 1972 문헌 검토 후):
  - (5) 위상: 크기가 큰 쪽 복소계수를 위상째 고른다. DC(평균간극)는 F_dry 가 정한 건접촉
    강체접근(h_dry 평균)을 쓴다.
  - (7) 'free space' = Johnson 부록 II 의 Macaulay 평균 mean(max(h_tran, 0)).
    phi_bl 은 이분법으로 맞춘다(2010 step 7). 갱신법·허용오차는 원문에 없어 tol_phi 로 둔다.
  - M2 의 η·B 는 p_mean(= p_h) 에서 평가한다(2007 Part 1 'nominal Hertz pressure').
    F_lub 는 c_rho 로만 들어간다.
  - p_tran 구성은 두 안을 모두 낸다(V2 에서 비교):
      "hertz": 변동분(DC 0) + 매끈면 Hertz 분포 (2017·2018·2021 SKF)
      "mean" : 변동분 + p_mean (2011, 국부 압력)
    둘 다 0 ≤ p ≤ p_lim 로 절단한다(2011 음압 0 고정 · 2010 p_lim).

§6.6.4 재검토(사용자 결정 2026-10-29):
  - M2 보완파만 웹 계산기와 통일 — s0_mode 기본 "hooke_li_all"(S = 0 Hooke & Li 복소 H/A,
    S ≠ 0 H/A 근사 h_c = H/A − h_a(S), §5 V1c). 나머지는 원문 §4.2 그대로.
  - p_mode: "paper" = 식 (4.2-6) w⁻¹ 역산(기본) · "direct" = §7.4.1, 식 (4.2-5) 에서 고른 쪽의 압력
    계수를 직접 사용 — 유막 성분 P_lub, 건접촉 성분 P_lub + P_dry(h_dry − r = ν_lub + u_dry 와 같은 힘 구성).
    결과에는 두 안을 모두 담는다(p_tran = 선택 p_mode, p_tran_paper · p_tran_direct).
"""
from __future__ import annotations

import numpy as np

import conditions as CO
import dry_contact as DC
import ehl_ripple as ER
import kernels as K


def _state(phi, r, L, case, S, h_mean, hc_mode, p0, s0_mode):
    """주어진 phi_bl 에서 step 3–5. 반환 dict(잔차 포함)."""
    c_rho = CO.density_ratio(phi * case.p_h)
    dh, dp, x = ER.ripple_field(r, L, case, S, c_rho * h_mean, hc_mode, s0_mode)
    h_lub = c_rho * h_mean + dh
    dry = DC.solve(h_lub, L, L, case.E_eff, phi * case.p_h, p_lim=case.p_lim, p0=p0)
    Fl, Fd = np.fft.fft2(h_lub), np.fft.fft2(dry["h"])
    Ft = np.where(np.abs(Fd) > np.abs(Fl), Fd, Fl)
    Ft[0, 0] = Fd[0, 0]                                       # 평균간극 = 건접촉 강체접근
    h_tran = np.fft.ifft2(Ft).real
    free = float(np.mean(np.maximum(h_tran, 0.0)))
    return dict(phi=phi, c_rho=c_rho, h_lub=h_lub, dp_lub=dp, dry=dry, h_tran=h_tran, x=x,
                sel_dry=np.abs(Fd) > np.abs(Fl), resid=free - c_rho * h_mean, free=free)


def solve(r_c, L, case, S=0.0, h_mean=None, hc_mode="complex", tol_phi=1e-3, phi0=1e-4, itmax=60,
          s0_mode="hooke_li_all", p_mode="paper"):
    """중심 좌표 패치 r_c[iy, ix] (간극 섭동, 한 변 L) 의 하중분할 해.

    반환 dict: phi_bl, c_rho, h_lub, h_tran, p_tran(변동분), p_hertz, p_mean (전체압력 두 안),
               x, dry(M1 결과), history[(phi, 잔차, M1 수렴)], converged
    """
    h_mean = case.h_cen if h_mean is None else h_mean
    r = np.fft.ifftshift(r_c)                                 # ripple_field 규약(인덱스 0 = 중심)
    hist, p0 = [], None

    def ev(phi):
        nonlocal p0
        s = _state(phi, r, L, case, S, h_mean, hc_mode, p0, s0_mode)
        p0 = s["dry"]["p"]                                    # 다음 phi 의 M1 초기값
        hist.append((phi, s["resid"], s["dry"]["converged"]))
        return s

    # 괄호: 잔차는 phi 에 대해 감소(건접촉 하중↑ → 접근↑, c_rho↑). 작은 phi 에서 시작해 2배씩.
    lo = ev(phi0)
    if lo["resid"] <= 0.0:
        raise ValueError(f"phi = {phi0} 에서 이미 잔차 ≤ 0 — phi0 를 줄일 것")
    hi = lo
    while hi["resid"] > 0.0:
        lo = hi
        if lo["phi"] >= 1.0:
            raise ValueError("phi = 1 까지 잔차 > 0 — 전 하중 건접촉")
        hi = ev(min(2.0 * lo["phi"], 1.0))
    for _ in range(itmax):
        if hi["phi"] - lo["phi"] <= tol_phi * hi["phi"]:
            break
        mid = ev(0.5 * (lo["phi"] + hi["phi"]))
        lo, hi = (mid, hi) if mid["resid"] > 0.0 else (lo, mid)
    s = hi if abs(hi["resid"]) < abs(lo["resid"]) else lo

    # step 6: p_tran (변동분) — "paper" 식 (4.2-6) · "direct" §7.4.1 — 과 전체압력 두 안
    w = K.w_frf(r_c.shape, L, L, case.E_eff)
    Pv = np.fft.fft2(s["h_tran"] - r_c)
    p_tran = np.zeros_like(Pv)
    nz = w > 0
    p_tran[nz] = Pv[nz] / w[nz]
    p_paper = np.fft.ifft2(p_tran).real
    Pl = np.fft.fft2(s["dp_lub"])
    Pd = np.where(s["sel_dry"], Pl + np.fft.fft2(s["dry"]["p"]), Pl)
    Pd[0, 0] = 0.0                                            # 변동분(DC 0) — paper 와 같은 정의
    p_direct = np.fft.ifft2(Pd).real
    p_tran = {"paper": p_paper, "direct": p_direct}[p_mode]
    x = s["x"]
    X, Y = np.meshgrid(x, x)
    p_hz = case.p_h * np.sqrt(np.clip(1.0 - (X / case.a_x) ** 2 - (Y / case.a_y) ** 2, 0.0, None))
    clip = lambda p: np.clip(p, 0.0, case.p_lim)              # noqa: E731
    return dict(phi_bl=s["phi"], c_rho=s["c_rho"], resid=s["resid"], free=s["free"],
                h_mean=h_mean, h_lub=s["h_lub"], h_tran=s["h_tran"], dry=s["dry"], x=x,
                p_tran=p_tran, p_tran_paper=p_paper, p_tran_direct=p_direct, p_mode=p_mode,
                p_hertz=clip(p_hz + p_tran), p_mean=clip(case.p_h + p_tran),
                history=hist, converged=bool(hi["phi"] - lo["phi"] <= tol_phi * hi["phi"]
                                             and all(h[2] for h in hist)))
