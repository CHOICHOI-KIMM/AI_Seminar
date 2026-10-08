"""§4.4 ② — Leeds 원문 등고선도에서 스냅숏 위상 s 판독 + Δh 맞춤 s 대조.

등고선도(계산 막두께)의 중앙 행(y = 0)에서 굵은 윤곽선 무리를 찾아 결함(마름모)의 좌·우
꼭짓점을 읽는다. 척도는 결함 피치(대각선 방향 x 주기 125√2 = 176.8 μm), 원점은 등고선
영역(원)의 중심. s = 중앙 결함 중심의 x 위치 (= 본 계산 무늬 r(x − s) 의 윗면 중심).
대조: 같은 S 의 본 계산 Δh 를 s 0 ~ 176.8 μm(0.5 μm)로 옮겨 Leeds NN Δh 와 RMS 최소인 s.
      두 곡선은 Δh 기준이 달라(h − h_s 대 평균 기준, §4.2) 각자 평균을 빼고 비교한다.
"""
import io
import json
import os

import numpy as np
from PIL import Image
from scipy import ndimage as ndi

import conditions as C
import ehl_ripple as M2
import run_cases as RC

LEEDS = "../../03. 정리/2005_Leeds_3D_FlatTop_EHL/"
PANELS = {"a": ("img_0004.jpg", 0.0, "Fig 3(a) t=0"), "b": ("img_0034.jpg", 1.0, "Fig 6(d) t=3/500 s"),
          "c": ("img_0038.jpg", -1.0, "Fig 7(a) t=0")}
P_UM = 125.0 * np.sqrt(2.0)
TOL_UM = 5.0


def runs(idx):
    cut = np.where(np.diff(idx) > 1)[0]
    return list(zip(np.r_[idx[0], idx[cut + 1]], np.r_[idx[cut], idx[-1]]))


def read_contour(fname):
    g = np.asarray(Image.open(LEEDS + fname).convert("L")).astype(int)
    d = g < 150
    lab, n = ndi.label(d, structure=np.ones((3, 3)))
    k = int(np.argmax(ndi.sum(d, lab, range(1, n + 1)))) + 1
    ys, _ = np.where(lab == k)
    cy = (ys.min() + ys.max()) // 2
    row = d[cy - 1: cy + 2].any(0)
    rr = [(a, b) for a, b in runs(np.where(row)[0])]
    xc = 0.5 * (rr[0][0] + rr[-1][1])                 # 등고선 영역(원) 양 끝의 중점
    thick = [0.5 * (a + b) for a, b in rr[1:-1] if b - a >= 4]   # 굵은 윤곽(꼭짓점) — 얇은 등고선 제외
    # 중앙 결함: 원 중심을 사이에 둔 두 꼭짓점
    left = max(t for t in thick if t < xc + 25 and any(u > t + 40 for u in thick))
    cand = [t for t in thick if t > left + 40]
    right = min(cand)
    nxt = min(t for t in thick if t > right + 15)          # 다음 결함의 왼 꼭짓점
    pitch_px = nxt - left
    um_px = P_UM / pitch_px
    s = (0.5 * (left + right) - xc) * um_px
    return dict(xc_px=xc, left_px=left, right_px=right, next_px=nxt, pitch_px=pitch_px,
                um_per_px=um_px, diag_um=(right - left) * um_px, s_um=s)


def read_contour2(fname):
    """정밀 판독 — 중앙 결함 4변을 직선으로 잡아 중심을 구한다.

    굵은 윤곽(3×3 열림 후)을 u = x + y, v = x − y 로 투영하면 마름모 각 변이 u·v 축에서
    봉우리가 된다. 원 중심 근처의 u·v 봉우리 양쪽 하나씩이 중앙 결함의 네 변이다.
    중심 x = (U + V)/2 (U·V = 각 축 두 변의 중점). 척도는 같은 축에서 이웃 결함의 같은 쪽
    변 간격 = 피치 125 μm(변 수직 거리) → Δu = 125·√2 / (μm/px).
    원점(접촉 중심)은 등고선 영역 외곽을 행별 좌우 끝점으로 원 맞춤해 구한다.
    """
    g = np.asarray(Image.open(LEEDS + fname).convert("L")).astype(int)
    d = g < 150
    lab, n = ndi.label(d, structure=np.ones((3, 3)))
    k = int(np.argmax(ndi.sum(d, lab, range(1, n + 1)))) + 1
    main = lab == k
    ys, xs = np.where(main)
    y0, y1 = ys.min(), ys.max()
    pts = []
    for y in range(int(y0 + 0.2 * (y1 - y0)), int(y1 - 0.2 * (y1 - y0))):
        xr = np.where(main[y])[0]
        if xr.size:
            pts += [(xr[0], y), (xr[-1], y)]
    P = np.array(pts, float)
    A = np.c_[2 * P[:, 0], 2 * P[:, 1], np.ones(len(P))]
    (cx, cy, c0), *_ = np.linalg.lstsq(A, (P ** 2).sum(1), rcond=None)
    R = np.sqrt(c0 + cx ** 2 + cy ** 2)
    th = ndi.binary_opening(d, structure=np.ones((3, 3)))
    yy, xx = np.where(th)
    sel = (xx - cx) ** 2 + (yy - cy) ** 2 < (0.8 * R) ** 2
    u, v = (xx + yy)[sel], (xx - yy)[sel]

    def peaks(q, q0):
        h, e = np.histogram(q, bins=np.arange(q.min() - 1, q.max() + 2, 1.0))
        hs = ndi.uniform_filter1d(h.astype(float), 3)
        cen = 0.5 * (e[:-1] + e[1:])
        pk = [cen[i] for i in range(1, len(hs) - 1) if hs[i] >= hs[i - 1] and hs[i] >= hs[i + 1]
              and hs[i] > 0.35 * hs.max()]
        lo = max(p for p in pk if p < q0)
        hi = min(p for p in pk if p > q0)
        lo2 = max((p for p in pk if p < lo - 10), default=None)
        hi2 = min((p for p in pk if p > hi + 10), default=None)
        return lo, hi, lo2, hi2

    ul, uh, ul2, uh2 = peaks(u, cx + cy)
    vl, vh, vl2, vh2 = peaks(v, cx - cy)
    side_px = np.mean([(uh - ul), (vh - vl)]) / np.sqrt(2)          # 결함 한 변(수직 거리) [px]
    pitch = [x for x in ((ul - ul2) if ul2 else None, (uh2 - uh) if uh2 else None,
                         (vl - vl2) if vl2 else None, (vh2 - vh) if vh2 else None) if x]
    um_px = 125.0 * np.sqrt(2) / np.mean(pitch) if pitch else 90.0 / side_px
    xc_d = 0.5 * (0.5 * (ul + uh) + 0.5 * (vl + vh))
    yc_d = 0.5 * (0.5 * (ul + uh) - 0.5 * (vl + vh))
    return dict(cx_px=cx, cy_px=cy, R_px=R, xc_defect_px=xc_d, yc_defect_px=yc_d,
                um_per_px=um_px, side_um=side_px * um_px, n_pitch=len(pitch),
                s_um=(xc_d - cx) * um_px, y_off_um=(yc_d - cy) * um_px)


def read_contour3(fname):
    """정밀 판독 — 결함 안쪽 흰 영역(윤곽선으로 둘러싸인 구멍)의 무게중심을 쓴다.

    윤곽선 두께가 결함 둘레에서 고르므로 안쪽 구멍의 무게중심이 결함 중심이다(수백 px 평균).
    척도: 중앙 결함과 대각 이웃 4개(완전한 결함만, 면적 ≥ 0.7 × 중앙) 중심 간 거리 = 125 μm.
    원점: 등고선 영역(어두운 픽셀의 최대 덩어리, 구멍 채움) 외곽을 원으로 맞춤 —
          잔차 큰 점(측엽·잘린 결함)을 3회 걸러 다시 맞춘다.
    """
    g = np.asarray(Image.open(LEEDS + fname).convert("L")).astype(int)
    d = g < 150
    lab, n = ndi.label(d, structure=np.ones((3, 3)))
    k = int(np.argmax(ndi.sum(d, lab, range(1, n + 1)))) + 1
    filled = ndi.binary_fill_holes(lab == k)
    edge = filled & ~ndi.binary_erosion(filled)
    ys, xs = np.where(edge)
    P = np.c_[xs, ys].astype(float)
    for _ in range(4):
        A = np.c_[2 * P[:, 0], 2 * P[:, 1], np.ones(len(P))]
        (cx, cy, c0), *_ = np.linalg.lstsq(A, (P ** 2).sum(1), rcond=None)
        R = np.sqrt(c0 + cx ** 2 + cy ** 2)
        res = np.hypot(P[:, 0] - cx, P[:, 1] - cy) - R
        P = P[np.abs(res) < max(3.0, 2.0 * res.std())]
    th = ndi.binary_opening(d, structure=np.ones((3, 3)))       # 굵은 윤곽만(얇은 등고선 제거)
    holes = filled & ~th
    hl, hn = ndi.label(holes)
    areas = ndi.sum(holes, hl, range(1, hn + 1))
    cents = ndi.center_of_mass(holes, hl, range(1, hn + 1))
    jmax = int(np.argmax(areas))                                  # 가장 큰 흰 영역 = 홈 그물 → 제외
    rest = [a for j, a in enumerate(areas) if j != jmax]
    big = [(a, c) for j, (a, c) in enumerate(zip(areas, cents))
           if j != jmax and a > 0.3 * max(rest)]
    a0, (y0, x0) = min(big, key=lambda t: np.hypot(t[1][1] - cx, t[1][0] - cy))
    nb = sorted((np.hypot(c[1] - x0, c[0] - y0), a) for a, c in big if (a, c) != (a0, (y0, x0)))
    diag = [dist for dist, a in nb[:4] if a >= 0.7 * a0]
    um_px = 125.0 / np.mean(diag)
    return dict(cx_px=cx, cy_px=cy, R_px=R, R_um=R * um_px, xc_defect_px=x0, yc_defect_px=y0,
                area_px=a0, n_nb=len(diag), nb_px=[round(x, 2) for x in diag], um_per_px=um_px,
                s_um=(x0 - cx) * um_px, y_off_um=(y0 - cy) * um_px)


def fit_s(key, S, mode):
    d = np.genfromtxt(f"ref_data/V1b_leeds_fig3_{key}.csv", delimiter=",", skip_header=2, encoding="utf-8")
    case, h = C.LEEDS, 0.302e-6
    x = d[:, 0] * 1e-6
    m = (np.abs(x) <= case.a_x) & np.isfinite(d[:, 1])
    x, ref = x[m], d[m, 1] * 1e-6
    r, L = RC.flat_top_patch()
    shifts = np.arange(0.0, P_UM * 1e-6, 0.5e-6)
    dh, _, _, _ = M2.ripple_line(r, L, case, S, h, x, mode, shifts=shifts)
    e = np.sqrt(np.mean(((dh - dh.mean(1, keepdims=True)) - (ref - ref.mean())) ** 2, axis=1))
    k = int(np.argmin(e))
    return float(shifts[k] * 1e6), float(e[k] * 1e6)


def main():
    out = {}
    for key, (fname, S, label) in PANELS.items():
        c = read_contour(fname)
        modes = ("complex",) if S == 0 else ("complex", "magnitude")
        fits = {m: fit_s(key, S, m) for m in modes}
        row = dict(panel=label, S=S, contour=c, fit={m: dict(s_um=v[0], rms_um=v[1]) for m, v in fits.items()})
        for m, (sf, _) in fits.items():
            dd = (sf - c["s_um"] + P_UM / 2) % P_UM - P_UM / 2
            row["fit"][m]["diff_um"] = dd
        out[key] = row
        print(f"({key}) {label} · 판독 s {c['s_um']:+.1f} μm (꼭짓점 {c['left_px']:.1f}/{c['right_px']:.1f} px · "
              f"원 중심 {c['xc_px']:.1f} · 피치 {c['pitch_px']:.1f} px = {c['um_per_px']:.3f} μm/px · "
              f"대각선 {c['diag_um']:.1f} μm)", flush=True)
        for m, v in row["fit"].items():
            print(f"     맞춤({m}) s {v['s_um']:.1f} μm · RMS {v['rms_um']:.4f} μm · 판독과 차 {v['diff_um']:+.1f} μm "
                  f"→ {'OK' if abs(v['diff_um']) <= TOL_UM else '기준 초과'}", flush=True)
    json.dump(out, io.open("ref_data/V1b_phase.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
