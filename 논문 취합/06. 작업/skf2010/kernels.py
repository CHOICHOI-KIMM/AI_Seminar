"""FFT 탄성 커널 (계획서 §2.1 식 (3)(5′) · M1·M2 공유).

w(κ) = 2/(E*·κ) = 4/(E′·κ),  w(0) = 0 (평균 압력은 강체 접근으로 흡수)
u(p) = IFFT{ w · FFT(p) }   (식 (3) · 주기 단위셀)
변수명은 작업계획서 §2.0 대응표를 따른다.
"""
from __future__ import annotations

import numpy as np


def kappa_grid(shape, Lx, Ly):
    """주기 패치(ny, nx) · 크기 Lx × Ly 의 파수 크기 κ = √(ωx² + ωy²) [1/m]."""
    ny, nx = shape
    wx = 2.0 * np.pi * np.fft.fftfreq(nx, d=Lx / nx)
    wy = 2.0 * np.pi * np.fft.fftfreq(ny, d=Ly / ny)
    WX, WY = np.meshgrid(wx, wy)
    return np.hypot(WX, WY)


def w_frf(shape, Lx, Ly, E_eff):
    """주파수응답함수 w = 4/(E′κ), DC 성분 0."""
    kappa = kappa_grid(shape, Lx, Ly)
    w = np.zeros_like(kappa)
    nz = kappa > 0
    w[nz] = 4.0 / (E_eff * kappa[nz])
    return w


def displacement(p, w):
    """법선 탄성변위 u(p) = IFFT{w·FFT(p)} (평균 0)."""
    return np.fft.ifft2(w * np.fft.fft2(p)).real
