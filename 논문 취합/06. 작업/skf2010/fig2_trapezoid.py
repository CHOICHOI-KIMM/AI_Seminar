"""§3.9 — §3.4 의 Fig 2 형식 4열 그림을 사다리꼴 형상(§4.6.3 · Leeds Fig 1 Simulated)으로 다시 만든다.

시험 비교만 한다(기본 입력은 계획서 K9 수직 모서리 그대로 · 사용자 결정 2026-10-08).
위상 s 는 §3.4 값을 그대로 쓴다(S=0 176.0 · +1 2.0 · −1 복소 0.5 / 크기 175.5 μm).
산출: figures/V1_fig2_{initial_z, ours_complex, ours_magnitude}_trap.png  (논문 그림은 §3.4 것을 그대로 씀)
"""
import numpy as np

import conditions as C
import ehl_ripple as M2
import run_cases as RC
from fig3_shape_test import trapezoid_patch, A_MID, W_RAMP

S_34 = {(0.0, "complex"): 176.0, (1.0, "complex"): 2.0, (1.0, "magnitude"): 2.0,
        (-1.0, "complex"): 0.5, (-1.0, "magnitude"): 175.5}


def z_edge(q):
    """결함 중심에서 변까지 수직거리 q [m] → 높이 [μm] (사다리꼴)."""
    return 0.30 * np.clip((A_MID + W_RAMP / 2 - q) / W_RAMP, 0.0, 1.0)


def main():
    case, h = C.LEEDS, 0.302e-6
    r, L = trapezoid_patch()
    xs = np.linspace(-case.a_x, case.a_x, 1201)
    fits = {}
    for (S, mode), s_um in S_34.items():
        dh, dp, _, _ = M2.ripple_line(r, L, case, S, h, xs, mode, shifts=np.array([s_um * 1e-6]))
        fits[(S, mode)] = dict(xs=xs, dh=dh[0], dp=dp[0], s=s_um * 1e-6)
        print(f"S={S:+.0f} {mode:9s} s {s_um:5.1f} μm · Δh {dh.min()*1e6:+.3f} ~ {dh.max()*1e6:+.3f} μm", flush=True)
    for src in ("roughness", "complex", "magnitude"):
        print(RC.plot_fig2_style(src, fits, suffix="_trap", z_edge=z_edge if src == "roughness" else None))


if __name__ == "__main__":
    main()
