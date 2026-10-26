"""§6.6.3 4.4 — 패치 크기(U5) 축소 영향 (구면 캐프 · Hooke & Li 기본값 · N 128 · 사용자 결정 2026-10-11).

L = 2a_x(현 기준) · a_x · 0.5a_x 에서 M3 를 다시 풀어 먼 쪽 간극·간극 봉우리·최대 p/p_h·φ_bl·F_dry 를 비교한다.
    python v2_patch_size.py  ->  ref_data/V2_patch_size.json
"""
import io
import json

import numpy as np

import conditions as CO
import load_sharing as LS
from v2_fig7 import CASE, N, bump


def main():
    a = CASE.a_x
    hs = a**2 / CASE.Rx
    out = {}
    for frac in (2.0, 1.0, 0.5):
        L = frac * a
        x = (np.arange(N) - N // 2) * L / N
        X, Y = np.meshgrid(x, x)
        o = LS.solve(-bump("cap", np.hypot(X, Y)), L, CASE)
        xb, j = x / a, N // 2
        p = o["p_hertz"][j] / CASE.p_h
        hh = o["h_tran"][j] / hs
        far_mask = np.abs(xb) >= 0.4 * frac               # 창 가장자리 20 % (|x| ≥ 0.4 L)
        res = dict(L_um=L * 1e6, L_over_a=frac, dx_um=L / N * 1e6, image_gap_um=(L - 2 * 18e-6) * 1e6,
                   phi_bl=o["phi_bl"], F_dry_N=o["phi_bl"] * CASE.p_h * L**2, c_rho=o["c_rho"],
                   converged=o["converged"], contact_area_um2=float(np.mean(o["dry"]["p"] > 0) * L**2 * 1e12),
                   p_max=float(p.max()), x_pmax=float(xb[np.argmax(p)]),
                   h_mean_patch=float(np.mean(np.maximum(o["h_tran"], 0.0)) / hs),
                   h_far=float(np.mean(hh[far_mask])),
                   h_max_lead=float(hh[xb < 0].max()), h_max_trail=float(hh[xb > 0].max()))
        out[f"L={frac:g}a"] = res
        print({k: (round(v, 4) if isinstance(v, float) else v) for k, v in res.items()}, flush=True)
    json.dump(out, io.open("ref_data/V2_patch_size.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
