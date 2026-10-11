"""SKF 2010 Fig 7(b) (본 모델, 순수구름, p_h 0.63 GPa) 곡선 자동 디지타이징 — 색상 추출 (작업내역서 §5.3 ④).

축: x/b −1.5 ~ 1.5 (프레임 좌우) · p/p_h 0 ~ 7 (좌축) · hR_x/b² 0 ~ 1 (우축). 곡선은 x/b ±1 에만 있다.
곡선: 파랑 = p/p_h, 초록 = hR_x/b². 열마다 이전 점에 가장 가까운 색 무리를 따른다.
급경사·수직 급변 열도 무리 중심을 그대로 기록해 끊김 없이 잇는다(사용자 결정 2026-10-09):
급변 열의 중심은 앞뒤 열 값의 중간이라 선이 이어지면서 봉우리(다음 열)도 보존된다. 값 0 의 곡선은 프레임 아랫선에
겹치므로 아랫선 행까지 읽는다.

    python digitize_fig7.py  ->  ref_data/V2_skf_fig7b.csv (curve,x_b,value) · figures/V2_digitize_check.png
"""
from __future__ import annotations

import os

import numpy as np
from PIL import Image, ImageDraw

from digitize_fig2 import frame, runs

SRC = "../../03. 정리/2010_SKF_Micro_Geometry_Effects_on_the_S/img_0018.jpg"
X_LIM, P_LIM, H_LIM = (-1.5, 1.5), (0.0, 7.0), (0.0, 1.0)


def trace(mask, cols, top, bottom):
    """폴리라인 (열, 행) 배열. 첫 열은 가장 긴 무리에서 출발."""
    pts, prev = [], None
    for c in cols:
        rr = runs(np.where(mask[top + 3: bottom + 2, c])[0] + top + 3)
        if not rr:
            continue
        if prev is None:
            s, e = max(rr, key=lambda t: t[1] - t[0])
            prev = 0.5 * (s + e)
        else:
            s, e = min(rr, key=lambda t: min(abs(t[0] - prev), abs(t[1] - prev)))
        prev = 0.5 * (s + e)
        pts.append((c, prev))
    return np.array(pts, float)


def main():
    img = Image.open(SRC).convert("RGB")
    im = np.asarray(img).astype(int)
    R, G, B = im[..., 0], im[..., 1], im[..., 2]
    blue = (B - R > 35) & (B - G > 25) & (B > 90)
    green = (G - R > 25) & (G - B > 5) & (G > 70)
    left, right, top, bottom = frame(im)
    col_of = lambda xb: left + (xb - X_LIM[0]) / (X_LIM[1] - X_LIM[0]) * (right - left)  # noqa: E731
    cols = np.arange(int(np.ceil(col_of(-1.0))), int(np.floor(col_of(1.0))) + 1)
    curves = {}
    for nm, mask, lim in (("p_ph", blue, P_LIM), ("hRx_b2", green, H_LIM)):
        pts = trace(mask, cols, top, bottom)
        x = X_LIM[0] + (pts[:, 0] - left) / (right - left) * (X_LIM[1] - X_LIM[0])
        v = np.maximum(lim[1] - (pts[:, 1] - top) / (bottom - top) * (lim[1] - lim[0]), 0.0)
        curves[nm] = (pts, x, v)

    os.makedirs("ref_data", exist_ok=True)
    path = "ref_data/V2_skf_fig7b.csv"
    with open(path, "w", encoding="utf-8") as f:
        f.write(f"# SKF 2010 Fig 7(b) present model, pure rolling, p_h 0.63 GPa; img_0018.jpg; frame px "
                f"L{left} R{right} T{top} B{bottom}; 열마다 색 무리 중심\n")
        f.write("curve,x_b,value\n")
        for nm, (_, x, v) in curves.items():
            for a, b in zip(x, v):
                f.write(f"{nm},{a:.4f},{b:.4f}\n")
    chk = img.copy()
    d = ImageDraw.Draw(chk)
    for nm, colr in (("p_ph", (255, 0, 0)), ("hRx_b2", (255, 140, 0))):
        d.line([tuple(p) for p in curves[nm][0]], fill=colr, width=1)
    d.rectangle((left, top, right, bottom), outline=(255, 0, 255))
    os.makedirs("figures", exist_ok=True)
    chk.resize((chk.width * 2, chk.height * 2)).save("figures/V2_digitize_check.png")
    for nm, (pts, x, v) in curves.items():
        print(f"{nm}: 점 {len(v)} · 열 건너뜀 최대 {int(np.diff(pts[:, 0]).max())} · "
              f"최대 {v.max():.3f} @ x/b {x[np.argmax(v)]:+.3f}")
    print(f"-> {path}")


if __name__ == "__main__":
    main()
