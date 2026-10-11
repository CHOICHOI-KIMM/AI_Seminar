"""M1 건접촉 — Stanley–Kato 변분해 + 완전소성 절단 (계획서 §2.1 · 논문 §2.1–2.2).

알고리즘(Stanley & Kato 1997 §6 · SKF 2010 §2.1):
  1. p′ = p − grad f,  grad f = u(p) + g
  2. p′ 을 균일하게 평행이동해 양압의 평균이 p_target 이 되게 한다
  3. p′ < 0 을 0 으로 자른다      → 4. p = p′, 수렴까지 반복
  "gradient 에 1 을 곱한다"는 원문의 단위계(단위셀 2π · 압력 p/E*)를 그대로 따른다:
      p̃′ = p̃ − (ũ + g̃),  p̃ = p/E*,  ũ, g̃ = (u, g)·2π/Lx
  → 물리량으로  p′ = p − E*·(2π/Lx)·(u + g),  E* = E′/2.
완전소성(Tian–Bhushan · SKF §2.2): 단계 2 에서 p_lim 상한까지 함께 걸어 평행이동량을 정한다
  (평균 하중 조건 식 (1) 을 소성 절단 후에도 지키기 위함).
수렴: 접촉점(p > 0)의 u + g 가 한 평면 위(범위 ≤ tol_h)에 있고(Stanley–Kato), 평균 |Δp|/p_target ≤ tol_p(SKF §2.2).
간극: h = u + g − δ,  δ = 접촉점 u + g 의 평균(강체 접근). 소성점은 h = 0 으로 둔다.
적용범위: 원문(Stanley–Kato §6) 대로 **부분접촉**에만 쓴다. 완전접촉에서는 가장 긴 파장 성분의
  반복계수가 −1 이라 진동해 수렴하지 않는다(V0b 확인) — 결과의 full_contact 로 알린다.
"""
from __future__ import annotations

import numpy as np

import kernels as K


def _shift_to_target(q, p_target, p_lim):
    """mean(clip(q + c, 0, p_lim)) = p_target 이 되는 c (이분법)."""
    lo, hi = -q.max() - 1.0, p_target + abs(q).max() + 1.0
    if not np.isfinite(p_lim):
        hi = p_target - q.min() + 1.0
    for _ in range(200):
        c = 0.5 * (lo + hi)
        if np.mean(np.clip(q + c, 0.0, p_lim)) > p_target:
            hi = c
        else:
            lo = c
    return 0.5 * (lo + hi)


def solve(g, Lx, Ly, E_eff, p_target, p_lim=np.inf, tol_h=1e-12, tol_p=1e-10, itmax=5000, p0=None):
    """주기 패치의 건접촉.  g: 강체 평면–비변형면 간극[m] (ny, nx).

    반환 dict: p[Pa] · u[m] · h[m] · delta[m] · iters · converged · plastic(소성점 비율) · rng_h · err_p
    """
    w = K.w_frf(g.shape, Lx, Ly, E_eff)
    E_star = 0.5 * E_eff
    scale = E_star * 2.0 * np.pi / Lx
    p = np.full(g.shape, float(p_target)) if p0 is None else p0.copy()
    ok, it, rng, err = False, 0, np.inf, np.inf
    for it in range(1, itmax + 1):
        u = K.displacement(p, w)
        q = p - scale * (u + g)
        c = _shift_to_target(q, p_target, p_lim)
        pn = np.clip(q + c, 0.0, p_lim)
        err = np.mean(np.abs(pn - p)) / p_target
        p = pn
        u = K.displacement(p, w)
        el = (p > 0) & (p < p_lim)                      # 탄성 접촉점
        s = (u + g)[el] if el.any() else (u + g)[p > 0]
        rng = s.max() - s.min()
        if rng <= tol_h and err <= tol_p:
            ok = True
            break
    u = K.displacement(p, w)
    el = (p > 0) & (p < p_lim)
    delta = float(np.mean((u + g)[el])) if el.any() else float(np.min(u + g))
    h = np.maximum(u + g - delta, 0.0)
    h[p > 0] = 0.0
    return dict(p=p, u=u, h=h, delta=delta, iters=it, converged=ok,
                plastic=float(np.mean(p >= p_lim)) if np.isfinite(p_lim) else 0.0,
                rng_h=float(rng), err_p=float(err), full_contact=bool(np.all(p > 0)))
