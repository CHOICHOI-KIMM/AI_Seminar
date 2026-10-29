"""§6.6.5 — 건접촉(M1) 진단: V2 범프(S = 0 · N 128) 의 떨림이 '압력 자체' 인지 '성분 선택 마스크' 인지 가름.

D1 선택 마스크 링잉: 건접촉 압력 변동분 p_dry − mean 을 그대로(①) · 식 (4.2-5) 선택 마스크를 씌워(②) 역변환.
   ① 은 접촉 밖에서 상수(−mean)라 변동이 없다. ② 가 접촉 밖에 변동을 만들면 마스크 링잉(C1).
   지표는 공간 표준편차(고역 필터를 쓰면 필터 자체가 링잉을 만들어 접촉 밖에 새므로 쓰지 않음).
D2 접촉 영역 해상도: 접촉 격자점 수 · 등가 반경/Δx · 중앙단면 접촉 폭(격자점) (C2).
방법 변경 없음 — load_sharing 해(paper, 기본값)를 그대로 분해한다.
    python m1_diag.py  ->  ref_data/M1_diag.json · figures/M1_diag_mask.png
"""
import json

import matplotlib
import numpy as np

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

import load_sharing as LS  # noqa: E402
from v2_direct import CASE, bump, grid  # noqa: E402
from v2_n256 import hf_rms  # noqa: E402


def main():
    a, L, x, (X, Y) = grid()
    xb, j, dx, ph = x / a, len(x) // 2, x[1] - x[0], CASE.p_h
    out, lines = {}, {"xb": xb}
    for shape in ("ellipsoid", "cap"):
        r_c = -bump(shape, np.hypot(X, Y))
        o = LS.solve(r_c, L, CASE)
        s = LS._state(o["phi_bl"], np.fft.ifftshift(r_c), L, CASE, 0.0, o["h_mean"], "complex", None, "hooke_li_all")
        p = s["dry"]["p"]
        con = p > 0
        Pd = np.fft.fft2(p)
        Pd[0, 0] = 0.0
        full = np.fft.ifft2(Pd).real                                  # ① 마스크 없음 = p − mean
        masked = np.fft.ifft2(np.where(s["sel_dry"], Pd, 0.0)).real   # ② 선택 마스크
        out_ = ~con
        far = (np.abs(xb) >= 0.15) & (np.abs(xb) <= 0.5)
        Pl = np.fft.fft2(s["dp_lub"])
        Pl[0, 0] = 0.0
        lub = np.fft.ifft2(Pl).real
        row = con[j]
        out[shape] = dict(
            phi_bl=o["phi_bl"], sel_dry_frac=float(s["sel_dry"].mean()),
            h_tran_redo_max_diff_nm=float(np.abs(s["h_tran"] - o["h_tran"]).max() * 1e9),
            # D1 — 중앙단면(|x/b| ≤ 0.5) 짧은 파장 RMS [p_h]
            hf_line_full=hf_rms(xb, full[j] / ph), hf_line_masked=hf_rms(xb, masked[j] / ph),
            hf_line_paper=hf_rms(xb, o["p_tran_paper"][j] / ph),
            # D1 — 접촉 밖 공간 변동(표준편차, 고역 필터 없이) [p_h]: ① 은 접촉 밖 상수라 0
            out_std_full=float(full[out_].std() / ph), out_std_masked=float(masked[out_].std() / ph),
            max_out_masked=float(np.abs(masked[out_] - masked[out_].mean()).max() / ph),
            # D1 — 중앙단면 접촉에서 떨어진 구간(0.15 ≤ |x/b| ≤ 0.5) 표준편차 [p_h]: paper · 마스크 건접촉 · 유막
            far_std_paper=float(o["p_tran_paper"][j][far].std() / ph), far_std_masked=float(masked[j][far].std() / ph),
            far_std_lub=float(lub[j][far].std() / ph), far_std_full=float(full[j][far].std() / ph),
            # D2 — 접촉 영역 해상도
            n_contact=int(con.sum()), r_eq_over_dx=float(np.sqrt(con.sum() / np.pi)),
            r_eq_um=float(np.sqrt(con.sum() * dx * dx / np.pi) * 1e6), dx_um=float(dx * 1e6),
            n_line=int(row.sum()), line_extent_xb=[float(xb[row].min()), float(xb[row].max())] if row.any() else None,
            p_dry_max_ph=float(p.max() / ph), plastic_frac=float(s["dry"]["plastic"]))
        lines[f"{shape}_full"], lines[f"{shape}_masked"], lines[f"{shape}_con"] = full[j] / ph, masked[j] / ph, row
        print(shape, {k: (round(v, 5) if isinstance(v, float) else v) for k, v in out[shape].items()}, flush=True)
    json.dump(out, open("ref_data/M1_diag.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)

    plt.rcParams["font.family"] = "Malgun Gothic"
    plt.rcParams["axes.unicode_minus"] = False
    fig, axs = plt.subplots(2, 1, figsize=(9.0, 6.8), sharex=True)
    for ax, (shape, nm) in zip(axs, (("ellipsoid", "타원 반구"), ("cap", "구면 캐프"))):
        c = lines[f"{shape}_con"]
        if c.any():
            ax.axvspan(xb[c].min() - (xb[1] - xb[0]) / 2, xb[c].max() + (xb[1] - xb[0]) / 2, color="0.88", lw=0, label="접촉 영역(중앙단면)")
        ax.plot(xb, lines[f"{shape}_full"], color="k", lw=1.3, label="① 건접촉 압력 변동분(마스크 없음)")
        ax.plot(xb, lines[f"{shape}_masked"], color="#d62728", lw=1.1, label="② 식 (4.2-5) 선택 마스크 적용")
        r = out[shape]
        ax.set(xlim=(-1, 1), ylabel="p/p_h", title=f"{nm} · N 128 · 접촉 밖 표준편차(2D) ① {r['out_std_full']:.3f} · ② {r['out_std_masked']:.3f} p_h")
        ax.grid(alpha=0.3, ls=":")
        ax.legend(fontsize=8, loc="upper right")
    axs[-1].set_xlabel("x/b")
    fig.tight_layout()
    fig.savefig("figures/M1_diag_mask.png", dpi=150)


if __name__ == "__main__":
    main()
