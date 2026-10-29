"""§6.6.4 — V2 압력 떨림의 출처 분해(구면 캐프 · S = 0 · N 128): p_tran = 유막 성분 + 건접촉 성분.

direct(§7.4.1) 의 압력 계수를 두 부분으로 나눠 중앙단면 짧은 파장 RMS(λ < 10 μm, |x/b| ≤ 0.5)를 비교한다.
  유막 부분   : 유막 쪽이 선택된 성분의 P_lub + 건접촉 쪽이 선택된 성분의 P_lub
  건접촉 부분 : 건접촉 쪽이 선택된 성분의 P_dry (DC 제외)
paper − direct = w⁻¹ ν_lub − P_lub (보완파의 κ_c ≠ κ 차이) 도 같은 지표로 보고한다.
    python v2_direct_parts.py  ->  ref_data/V2_direct_parts.json
"""
import json

import numpy as np

import load_sharing as LS
from v2_direct import CASE, bump, grid
from v2_n256 import hf_rms


def main():
    a, L, x, (X, Y) = grid()
    xb, j = x / a, len(x) // 2
    r_c = -bump("cap", np.hypot(X, Y))
    o = LS.solve(r_c, L, CASE)
    s = LS._state(o["phi_bl"], np.fft.ifftshift(r_c), L, CASE, 0.0, o["h_mean"], "complex", None, "hooke_li_all")
    Pl, Pdry, sel = np.fft.fft2(s["dp_lub"]), np.fft.fft2(s["dry"]["p"]), s["sel_dry"]
    Pdry = np.where(sel, Pdry, 0.0)
    Pdry[0, 0] = 0.0
    Pl[0, 0] = 0.0
    lub, dry = np.fft.ifft2(Pl).real, np.fft.ifft2(Pdry).real
    ph = CASE.p_h
    out = dict(phi_bl=o["phi_bl"], sel_dry_frac=float(sel.mean()),
               hf_rms_lub=hf_rms(xb, lub[j] / ph), hf_rms_dry=hf_rms(xb, dry[j] / ph),
               hf_rms_direct=hf_rms(xb, o["p_tran_direct"][j] / ph), hf_rms_paper=hf_rms(xb, o["p_tran_paper"][j] / ph),
               hf_rms_paper_minus_direct=hf_rms(xb, (o["p_tran_paper"] - o["p_tran_direct"])[j] / ph),
               max_abs_paper_minus_direct=float(np.abs(o["p_tran_paper"] - o["p_tran_direct"]).max() / ph),
               h_tran_redo_max_diff_nm=float(np.abs(s["h_tran"] - o["h_tran"]).max() * 1e9))   # 같은 φ 재계산 확인(M1 초기값만 다름)
    json.dump(out, open("ref_data/V2_direct_parts.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(out)


if __name__ == "__main__":
    main()
