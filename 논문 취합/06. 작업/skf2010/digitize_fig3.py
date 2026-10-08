"""SKF 2010 Fig 3 (a)(b)(c) 예측 패널 — NN 곡선 자동 디지타이징 (작업내역서 §4.4 ①).

Fig 3 은 흑백이라 Fig 2 처럼 색으로 곡선을 가를 수 없다. NN 은 굵은 실선, N 은 가는 점선이므로
**선 굵기**로 가른다: 어두운 픽셀 → 3×3 열림 연산(가는 선·글자 제거) → 작은 조각 제거.
남은 굵은 선에서 압력(위)·막두께(아래) 두 곡선을 x = 0 열에서 시작해 좌우로 추적한다
(열마다 이전 값에 가장 가까운 무리). 무리 높이가 JUMP_PX 를 넘으면 수직 급변 → NaN.

축: x −200 ~ 200 μm(접촉 중심 기준) · Δh = h − h_s −0.3 ~ 0.4 μm · Δp = p − p_s −5 ~ 2 GPa.
    python digitize_fig3.py  ->  ref_data/V1b_leeds_fig3_{a,b,c}.csv · figures/V1b_digitize_check_{a,b,c}.png
"""
from __future__ import annotations

import os

import numpy as np
from PIL import Image, ImageDraw
from scipy import ndimage as ndi

SRC = "../../03. 정리/2010_SKF_Micro_Geometry_Effects_on_the_S/"
FIGS = {"a": ("img_0008.jpg", 0.0), "b": ("img_0010.jpg", 1.0), "c": ("img_0012.jpg", -1.0)}
X_LIM, DH_LIM, DP_LIM = (-200.0, 200.0), (-0.3, 0.4), (-5.0, 2.0)
DARK = 150          # JPEG 번짐 — 120 이면 굵은 선 일부가 얇아져 열림 연산에 지워진다
JUMP_PX = 16       # 굵은 선(약 4 px)이 기울면 열 안 무리가 길어진다 — 무리 중심 오차 ≤ 8 px(Δh 0.013 μm)
MIN_SPAN = 25          # 이 폭(px)보다 좁은 조각은 글자·점선 잔재로 본다


def frame(g):
    d = g < DARK
    cs, rs = d.sum(0), d.sum(1)
    w, h = g.shape[1], g.shape[0]
    left = int(np.argmax(cs[: w // 2]))
    right = w // 2 + int(np.argmax(cs[w // 2:]))
    top = int(np.argmax(rs[: h // 2]))
    bottom = h // 2 + int(np.argmax(rs[h // 2:]))
    return left, right, top, bottom


def thick_mask(g, box):
    left, right, top, bottom = box
    m = np.zeros_like(g, bool)
    m[top + 4: bottom - 3, left + 4: right - 3] = g[top + 4: bottom - 3, left + 4: right - 3] < DARK
    m = ndi.binary_opening(m, structure=np.ones((3, 3)))
    lab, n = ndi.label(m, structure=np.ones((3, 3)))
    for k, sl in enumerate(ndi.find_objects(lab), 1):
        if sl[1].stop - sl[1].start < MIN_SPAN:
            m[lab == k] = False
    return m


def runs(rows):
    if len(rows) == 0:
        return []
    cut = np.where(np.diff(rows) > 1)[0]
    return list(zip(np.r_[rows[0], rows[cut + 1]], np.r_[rows[cut], rows[-1]]))


def trace(m, cols, start):
    """start 행에서 출발해 cols 순서로 열마다 가장 가까운 무리의 중심을 따라간다."""
    out = np.full(len(cols), np.nan)
    prev = start
    for i, c in enumerate(cols):
        rr = runs(np.where(m[:, c])[0])
        if not rr:
            continue
        s, e = min(rr, key=lambda t: min(abs(t[0] - prev), abs(t[1] - prev), abs(0.5 * (t[0] + t[1]) - prev)))
        if min(abs(s - prev), abs(e - prev)) > 25:      # 너무 멀면 끊김으로 본다
            continue
        prev = 0.5 * (s + e)
        if e - s <= JUMP_PX:
            out[i] = prev
    return out


def digitize(key):
    fname, S = FIGS[key]
    img = Image.open(SRC + fname).convert("RGB")
    g = np.asarray(img.convert("L")).astype(int)
    left, right, top, bottom = frame(g)
    m = thick_mask(g, (left, right, top, bottom))
    H = bottom - top
    row_of = lambda v, lim: top + (lim[1] - v) / (lim[1] - lim[0]) * H
    cols = np.arange(left + 5, right - 4)
    c0 = int(np.argmin(np.abs(cols - (left + right) / 2)))
    # x = 0 열에서 두 곡선의 시작 행: 압력은 Δp=0 근처, 막두께는 그보다 아래 첫 무리
    rr = runs(np.where(m[:, cols[c0]])[0])
    rp0 = row_of(0.0, DP_LIM)
    p_run = min(rr, key=lambda t: abs(0.5 * (t[0] + t[1]) - rp0))
    h_run = min((t for t in rr if t[0] > p_run[1] + 5), key=lambda t: t[0])
    res = {}
    for nm, run in (("p", p_run), ("h", h_run)):
        st = 0.5 * (run[0] + run[1])
        rgt = trace(m, cols[c0:], st)
        lft = trace(m, cols[:c0 + 1][::-1], st)[::-1]
        res[nm] = np.r_[lft[:-1], rgt]
    x = X_LIM[0] + (cols - left) / (right - left) * (X_LIM[1] - X_LIM[0])
    dh = DH_LIM[1] - (res["h"] - top) / H * (DH_LIM[1] - DH_LIM[0])
    dp = DP_LIM[1] - (res["p"] - top) / H * (DP_LIM[1] - DP_LIM[0])

    os.makedirs("ref_data", exist_ok=True)
    path = f"ref_data/V1b_leeds_fig3_{key}.csv"
    with open(path, "w", encoding="utf-8") as f:
        f.write(f"# SKF 2010 Fig 3({key}) NN, S={S:+.0f}, {fname}; frame px L{left} R{right} T{top} B{bottom}; "
                f"NaN = 수직 급변(>{JUMP_PX}px) 또는 미검출 · x 는 접촉 중심 기준\n")
        f.write("x_um,dh_um,dp_GPa\n")
        for a, b, c in zip(x, dh, dp):
            f.write(f"{a:.2f},{b:.4f},{c:.4f}\n")
    chk = img.copy()
    d = ImageDraw.Draw(chk)
    for c, yh, yp in zip(cols, res["h"], res["p"]):
        if np.isfinite(yh):
            d.ellipse((int(c) - 1, int(round(yh)) - 1, int(c) + 1, int(round(yh)) + 1), fill=(255, 0, 0))
        if np.isfinite(yp):
            d.ellipse((int(c) - 1, int(round(yp)) - 1, int(c) + 1, int(round(yp)) + 1), fill=(0, 90, 255))
    d.rectangle((left, top, right, bottom), outline=(0, 200, 0))
    os.makedirs("figures", exist_ok=True)
    chk = chk.resize((chk.width * 2, chk.height * 2))
    chk.save(f"figures/V1b_digitize_check2_{key}.png")
    print(f"Fig 3({key}) S={S:+.0f}: {len(x)}열 · 유효 dh {np.isfinite(dh).mean():.0%} dp {np.isfinite(dp).mean():.0%} · "
          f"dh {np.nanmin(dh):+.3f}~{np.nanmax(dh):+.3f} μm · dp {np.nanmin(dp):+.2f}~{np.nanmax(dp):+.2f} GPa -> {path}")


if __name__ == "__main__":
    for k in FIGS:
        digitize(k)
