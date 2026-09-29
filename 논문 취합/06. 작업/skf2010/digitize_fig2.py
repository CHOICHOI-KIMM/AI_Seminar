"""SKF 2010 Fig 2 (a)(b)(c) 곡선 자동 디지타이징 — 이미지 색상 추출 (사용자 결정 2026-09-29).

축: x 50~450 um (프레임 좌우), Delta h -0.4~0.4 um (좌축), Delta p -5~2 GPa (우축).
곡선: 빨강 = Delta h, 파랑 = Delta p. 열마다 곡선색 픽셀 무리를 찾고, 본문 라벨 글자를
피하려고 이전 열 값에 가장 가까운 무리를 따른다. 무리 높이가 JUMP_PX 를 넘으면
수직 급변 구간이라 값이 하나로 정해지지 않으므로 NaN 으로 둔다.

    python digitize_fig2.py   ->  ref_data/V1_skf_fig2_{a,b,c}.csv, figures/V1_digitize_check_{a,b,c}.png
"""
from __future__ import annotations

import os

import numpy as np
from PIL import Image, ImageDraw

SRC = "../../03. 정리/2010_SKF_Micro_Geometry_Effects_on_the_S/"
FIGS = {"a": ("img_0004.jpg", 0.0), "b": ("img_0005.jpg", 1.0), "c": ("img_0006.jpg", -1.0)}
X_LIM, DH_LIM, DP_LIM = (50.0, 450.0), (-0.4, 0.4), (-5.0, 2.0)
JUMP_PX = 8


def frame(im):
    """축 상자: 비백색 픽셀이 가장 많은 좌우 열·상하 행."""
    nw = im.sum(2) < 600
    cs, rs = nw.sum(0), nw.sum(1)
    w, h = im.shape[1], im.shape[0]
    left = int(np.argmax(cs[: w // 2]))
    right = w // 2 + int(np.argmax(cs[w // 2:]))
    top = int(np.argmax(rs[: h // 2]))
    bottom = h // 2 + int(np.argmax(rs[h // 2:]))
    return left, right, top, bottom


def runs(rows):
    """정렬된 행 번호를 연속 무리로 묶어 (시작, 끝) 목록 반환."""
    if len(rows) == 0:
        return []
    cut = np.where(np.diff(rows) > 1)[0]
    starts = np.r_[rows[0], rows[cut + 1]]
    ends = np.r_[rows[cut], rows[-1]]
    return list(zip(starts, ends))


def trace(mask, left, right, top, bottom):
    """열마다 이전 값에 가장 가까운 무리의 중심 행. 급변·누락은 NaN."""
    cols = np.arange(left + 3, right - 2)
    out = np.full(len(cols), np.nan)
    prev = None
    for i, c in enumerate(cols):
        rr = runs(np.where(mask[top + 3: bottom - 2, c])[0] + top + 3)
        if not rr:
            continue
        if prev is None:
            s, e = max(rr, key=lambda t: t[1] - t[0])
        else:
            s, e = min(rr, key=lambda t: min(abs(t[0] - prev), abs(t[1] - prev)))
        prev = 0.5 * (s + e)
        if e - s <= JUMP_PX:
            out[i] = prev
    return cols, out


def digitize(key):
    fname, S = FIGS[key]
    img = Image.open(SRC + fname).convert("RGB")
    im = np.asarray(img).astype(int)
    R, G, B = im[..., 0], im[..., 1], im[..., 2]
    red = (R > 170) & (G < 110) & (B < 110)
    blue = (B > 120) & (R < 110) & (G < 150) & (B - R > 60)
    left, right, top, bottom = frame(im)

    cols, rh = trace(red, left, right, top, bottom)
    _, rp = trace(blue, left, right, top, bottom)
    fx = (cols - left) / (right - left)
    x = X_LIM[0] + fx * (X_LIM[1] - X_LIM[0])
    dh = DH_LIM[1] - (rh - top) / (bottom - top) * (DH_LIM[1] - DH_LIM[0])
    dp = DP_LIM[1] - (rp - top) / (bottom - top) * (DP_LIM[1] - DP_LIM[0])

    os.makedirs("ref_data", exist_ok=True)
    path = f"ref_data/V1_skf_fig2_{key}.csv"
    with open(path, "w", encoding="utf-8") as f:
        f.write(f"# SKF 2010 Fig 2({key}), S={S:+.0f}, {fname}; frame px L{left} R{right} T{top} B{bottom}; "
                f"NaN = 수직 급변(>{JUMP_PX}px) 또는 미검출\n")
        f.write("x_um,dh_um,dp_GPa\n")
        for a, b, c in zip(x, dh, dp):
            f.write(f"{a:.2f},{b:.4f},{c:.4f}\n")

    chk = img.copy()
    d = ImageDraw.Draw(chk)
    for c, yh, yp in zip(cols, rh, rp):
        if np.isfinite(yh):
            d.point((int(c), int(round(yh)) - 6), fill=(0, 160, 0))
        if np.isfinite(yp):
            d.point((int(c), int(round(yp)) - 6), fill=(255, 140, 0))
    d.rectangle((left, top, right, bottom), outline=(0, 200, 0))
    os.makedirs("figures", exist_ok=True)
    chk_path = f"figures/V1_digitize_check_{key}.png"
    chk.save(chk_path)
    ok_h, ok_p = np.isfinite(dh).mean(), np.isfinite(dp).mean()
    print(f"Fig 2({key}) S={S:+.0f}: {len(x)}열, 유효 dh {ok_h:.0%} dp {ok_p:.0%}, "
          f"dh {np.nanmin(dh):+.3f}~{np.nanmax(dh):+.3f} um, dp {np.nanmin(dp):+.2f}~{np.nanmax(dp):+.2f} GPa "
          f"-> {path}, 검수 {chk_path}")


if __name__ == "__main__":
    for k in FIGS:
        digitize(k)
