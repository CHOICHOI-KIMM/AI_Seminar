"""Hooke et al. 2007 Part 1 Fig 12 — 보완파 진폭비 h_c/A vs 파장비 Λ = λx/b S^-2 P^1.5 (가로 roughness, ν/u < 1).

원본: ../../03. 정리/2007_SKF_Rapidcalculation_of_thepressure/img_0013.jpg (445×351 흑백 JPEG)

축 보정 (digitize_hl06.tick_cands · calib 재사용)
  · 가로(로그): 위·아래 프레임 눈금(주 10^-1·10^0·10^1 긴 눈금, 부 k·10^n 짧은 눈금) → log10 값 배정 → px = a + b·log10Λ 회귀.
  · 세로(선형): 좌·우 프레임 눈금 0.25 간격(1.0·0.5 긴 눈금, 0.75·0.25 짧은 눈금) → px = a + b·(h_c/A) 회귀.
기호 (범례 상자 행 85–152 · 열 280–437 제외)
  · ■(ν/u 0.5): 5×5 창이 모두 어두운 자리(core) — + 의 팔(1–2 px)·곡선(≤ 3 px)은 5×5 를 채우지 못한다.
      core 연결요소가 사각 하나 크기(≤ 4×4)면 그 평균 = 사각 중심 → kind=square.
      더 크면 사각 여러 개가 겹친 덩어리 → 개수·위치를 하나로 정할 수 없어 core 를 4 px 간격 NMS 로 나눈 자리를 kind=unknown 으로 둔다.
  · +(ν/u 0.95): 중심에서 상하·좌우 팔(±2–4 px) 이 어둡고(각 5/6 이상) 네 대각 2×2 모서리가 밝은(어두운 화소 ≤ PLUS_K/16) 자리.
      ■ 는 모서리가 어두워 걸리지 않는다. ■ 와 겹친 + 는 모서리가 어두워 검출되지 않는다(미검출로 보고).
곡선 (선 종류: 점선 = 왼쪽 + 무리를 따르는 선, 실선 = ■ 무리를 따르는 선 — 범례에는 선이 없어 계열 대응은 원문 서술 기준)
  · 기호 자리(검출 + 의 팔·■ 덩어리)를 지운 나머지 얇은 화소(열마다 연속 무리 높이 ≤ 4)를 앞 열 예측값 ±3 px 안에서 추적.
    두 곡선이 3 px 안으로 붙거나 기호 덩어리에 묻힌 열은 비운다(못 읽음).

    python digitize_h07_fig12.py
    -> ref_data/H07P1_fig12_digitized.csv, figures/H07P1_fig12_digitize_check.png
"""
from __future__ import annotations

import os

import numpy as np
from PIL import Image, ImageDraw
from numpy.lib.stride_tricks import sliding_window_view as swv
from scipy import ndimage

from digitize_hl06 import tick_cands, calib, runs

SRC = "../../03. 정리/2007_SKF_Rapidcalculation_of_thepressure/img_0013.jpg"
OUT_CSV = "ref_data/H07P1_fig12_digitized.csv"
OUT_PNG = "figures/H07P1_fig12_digitize_check.png"
DARK = 140
SCALE = 3
LEGEND = (85, 152, 280, 437)          # 행0, 행1, 열0, 열1 (범례 글자·기호)
SQ_CORE = 4          # 사각 하나의 core 폭 상한(px) — 실측 고립 ■ 5–8 px 폭 → core 1–4
PLUS_K = 6           # + 판정: 네 대각 2×2 모서리(16 화소) 중 어두운 화소 상한 — 지나는 곡선이 모서리 2 곳을 지나는 경우 포함


def load():
    g = np.asarray(Image.open(SRC).convert("L")).astype(float)
    return g, g < DARK


def frame(d):
    """프레임 직선(어두운 비율 > 0.5 행/열) → 안쪽 가장자리 (top, bottom, left, right)."""
    r = runs(np.where(d.sum(1) > 0.5 * d.shape[1])[0])
    c = runs(np.where(d.sum(0) > 0.5 * d.shape[0])[0])
    return r[0][1] + 1, r[-1][0] - 1, c[0][1] + 1, c[-1][0] - 1


def cal_x(d, top, bot, left, right):
    """로그 가로축: 긴 눈금 3 개(10^-1, 10^0, 10^1, 왼쪽부터)로 초기 직선 → 모든 눈금을 가장 가까운 log10(k·10^n) 에 배정 → 회귀."""
    major = tick_cands(d, "T", top, left + 2, right, lmin=14, lmax=22)
    assert len(major) == 3, major
    minor = (tick_cands(d, "T", top, left + 2, right, lmin=8, lmax=13)
             + tick_cands(d, "B", bot, left + 2, right, lmin=8, lmax=13))
    b0, a0 = np.polyfit([-1, 0, 1], major, 1)
    grid = np.log10(np.outer(np.arange(1, 10), 10.0 ** np.arange(-2, 3)).ravel())
    pv = [(p, v) for p, v in zip(major, (-1, 0, 1))]
    for p in minor:
        v = grid[np.argmin(np.abs(a0 + b0 * grid - p))]
        pv.append((p, float(v)))
    for _ in range(2):
        P, V = np.array(pv).T
        b, a = np.polyfit(V, P, 1)
        pv = [t for t, k in zip(pv, np.abs(P - (a + b * V)) < 1.5) if k]
    P, V = np.array(pv).T
    b, a = np.polyfit(V, P, 1)
    res = P - (a + b * V)
    return a, b, float(np.sqrt(np.mean(res ** 2))), pv


def cal_y(d, top, bot, left, right):
    cands = []
    for side, edge in (("L", left), ("R", right)):
        cands += tick_cands(d, side, edge, top + 6, bot - 4, lmin=8, lmax=22)
    return calib(cands, cands[int(np.argmin(np.abs(np.array(cands) - 0.5 * (top + bot))))], 0.5, 0.25, (top - bot) / 4.0)


def report_cal(name, cal):
    a, b, rms, pv = cal
    res = [p - (a + b * v) for p, v in pv]
    print(f"[보정 {name}] 눈금 {len(pv)} 개, px = {a:.3f} + {b:.3f}·v, rms {rms:.3f} px, 최대 |잔차| {max(map(abs, res)):.3f} px")


# ---------------------------------------------------------------- 기호 -------
def in_plot(r, c, top, bot, left, right):
    l0, l1, c0, c1 = LEGEND
    return top + 3 <= r <= bot - 1 and left + 3 <= c <= right - 3 and not (l0 <= r <= l1 and c0 <= c <= c1)


def squares(d, box):
    """5×5 전부 어두운 core → 연결요소. 반환 [(r, c, kind)] 와 덩어리 마스크(곡선 추적에서 지움)."""
    f = swv(d.astype(float), (5, 5)).sum((2, 3)) >= 25
    core = np.zeros(d.shape, bool)
    core[2:2 + f.shape[0], 2:2 + f.shape[1]] = f
    lab, n = ndimage.label(core, np.ones((3, 3)))
    out = []
    for k, sl in enumerate(ndimage.find_objects(lab), 1):
        rr, cc = np.where(lab[sl] == k)
        rr, cc = rr + sl[0].start, cc + sl[1].start
        if not in_plot(rr.mean(), cc.mean(), *box):
            continue
        if np.ptp(rr) <= SQ_CORE - 1 and np.ptp(cc) <= SQ_CORE - 1:
            out.append((rr.mean(), cc.mean(), "square"))
            continue
        # 덩어리: 7×7 창 어두움이 큰 core 부터 4 px(체비셰프) 간격 NMS
        dens = [d[r - 3:r + 4, c - 3:c + 4].sum() for r, c in zip(rr, cc)]
        taken = []
        for i in np.argsort(dens)[::-1]:
            if all(max(abs(rr[i] - r), abs(cc[i] - c)) >= 4 for r, c in taken):
                taken.append((rr[i], cc[i]))
        out += [(float(r), float(c), "unknown") for r, c in taken]
    blob = ndimage.binary_dilation(core, np.ones((5, 5)))       # core 를 사각 크기로 되돌린 자리
    return out, blob


def pluses(d, box):
    top, bot, left, right = box
    D = d.astype(float)
    arm = (-4, -3, -2, 2, 3, 4)
    cand = []
    for r in range(top + 5, bot - 3):
        for c in range(left + 5, right - 4):
            if not d[r, c] or not in_plot(r, c, *box):
                continue
            V = np.mean([D[r + k, c] for k in arm])
            H = np.mean([D[r, c + k] for k in arm])
            K = (D[r - 3:r - 1, c - 3:c - 1].sum() + D[r - 3:r - 1, c + 2:c + 4].sum()
                 + D[r + 2:r + 4, c - 3:c - 1].sum() + D[r + 2:r + 4, c + 2:c + 4].sum())
            if V >= 5 / 6 and H >= 5 / 6 and K <= PLUS_K:
                cand.append(((V + H) / 2 - K / 16, r, c))
    cand.sort(reverse=True)
    taken = []
    for s, r, c in cand:
        if all(max(abs(r - q), abs(c - p)) >= 3 for q, p in taken):
            taken.append((r, c))
    out = []
    for r, c in taken:
        # 부화소 중심: 가로 팔 행들의 평균 행, 세로 팔 열들의 평균 열 (팔 두께 1–2 px)
        rows = [q for q in range(r - 1, r + 2) if D[q, c - 4:c + 5].sum() >= 7]
        cols = [p for p in range(c - 1, c + 2) if D[r - 4:r + 5, p].sum() >= 7]
        out.append((float(np.mean(rows)) if rows else float(r), float(np.mean(cols)) if cols else float(c), "plus"))
    return out


def plus_mask(shape, pts):
    m = np.zeros(shape, bool)
    for r, c, _ in pts:
        r, c = int(round(r)), int(round(c))
        m[r - 5:r + 6, c - 1:c + 2] = True
        m[r - 1:r + 2, c - 5:c + 6] = True
    return m


# ---------------------------------------------------------------- 곡선 -------
# 추적 시작점(Λ, h_c/A): 원본에서 두 곡선이 기호와 떨어져 홀로 지나는 자리. 시작점 자체는 데이터로 쓰지 않고
# 그 열의 실제 얇은 무리에 붙는지(±3 px)만 확인한 뒤 양쪽으로 추적한다.
SEEDS = [("line_dash", 0.146, 0.444),     # 왼쪽 오르막(+ 무리 왼편)
         ("line_solid", 0.265, 0.395)]    # 왼쪽 오르막(■ 무리 사이)
# 봉우리(Λ ≈ 0.3–0.8)는 기호 덩어리에 묻혀 끊긴다. 오른쪽 내리막(Λ > 0.8)은 두 선이 4–6 px 간격으로 기호 띠 속을 함께 지나
# 어느 화소가 어느 선인지 못 정하므로(시험 추적이 + 팔·옆 선으로 건너뜀) 추적하지 않는다.
RUN_MAX = 7          # 열당 무리 높이 상한(px) — 가파른 구간의 선 두께 포함, 기호 잔여·눈금(≥ 10) 제외
GAP_MAX = 14         # 연속 빈 열 상한 — 넘으면 추적 끝
FIT_N = 6            # 기울기 추정에 쓰는 최근 점 수


def thin_runs(res, x, r0, r1):
    return [(s, e) for s, e in runs(np.where(res[r0:r1, x])[0] + r0) if e - s + 1 <= RUN_MAX]


def dist(run, y):
    """예측 행 y 와 무리 [s, e] 사이 거리 — 가파른 구간은 한 열의 무리가 기울기만큼 길어 중심보다 구간으로 비교한다."""
    return max(0.0, run[0] - y, y - run[1])


def track(res, x0, y0, step, x_lim, r0, r1):
    pts = [(x0, y0)]
    x, gap = x0, 0
    while gap <= GAP_MAX:
        x += step
        if not (x_lim[0] <= x <= x_lim[1]):
            break
        P = np.array(pts[-FIT_N:])
        slope = np.polyfit(P[:, 0], P[:, 1], 1)[0] if len(P) >= 3 else 0.0
        pred = pts[-1][1] + slope * (x - pts[-1][0])
        tol = 3 + abs(slope) * min(0.5 * (gap + 1), 1.0)   # 빈 열이 길수록 기울기 오차가 쌓인다(상한 3 + |기울기| — 옆 곡선으로 건너뛰지 않게)
        cand = [q for q in thin_runs(res, x, r0, r1) if dist(q, pred) <= tol]
        if not cand:
            gap += 1
            continue
        q = min(cand, key=lambda q: dist(q, pred))
        pts.append((x, 0.5 * (q[0] + q[1])))
        gap = 0
    return pts[1:]


def lines(res, box, cx, cy):
    top, bot, left, right = box
    r0, r1, xl = top + 18, bot - 5, (left + 3, right - 3)
    out = {"line_dash": {}, "line_solid": {}}
    for kind, lam, h in SEEDS:
        x0 = int(round(cx[0] + cx[1] * np.log10(lam)))
        y_seed = cy[0] + cy[1] * h
        near = [q for q in thin_runs(res, x0, r0, r1) if dist(q, y_seed) <= 3]
        assert near, (kind, x0, y_seed, thin_runs(res, x0, r0, r1))
        q = min(near, key=lambda q: dist(q, y_seed))
        y0 = 0.5 * (q[0] + q[1])
        pts = track(res, x0, y0, -1, xl, r0, r1)[::-1] + [(x0, y0)] + track(res, x0, y0, +1, xl, r0, r1)
        out[kind].update(pts)
    # 두 곡선이 같은 무리(3 px 안)를 잡은 열은 어느 곡선인지 못 정하므로 둘 다 뺀다
    a, b = out["line_dash"], out["line_solid"]
    for x in set(a) & set(b):
        if abs(a[x] - b[x]) <= 3:
            del a[x], b[x]
    return out


# ---------------------------------------------------------------- 실행 -------
def main():
    g, d = load()
    box = frame(d)
    cx, cy = cal_x(d, *box), cal_y(d, *box)
    report_cal("x (log10 Λ)", cx)
    report_cal("y (h_c/A)", cy)
    lam = lambda c: 10.0 ** ((np.asarray(c, float) - cx[0]) / cx[1])
    hca = lambda r: (np.asarray(r, float) - cy[0]) / cy[1]

    sq, blob = squares(d, box)
    pl = pluses(d, box)
    sym = pl + sq
    res = d & ~blob & ~plus_mask(d.shape, pl)
    ln = lines(res, box, cx, cy)

    rows = [(k, float(lam(c)), float(hca(r))) for r, c, k in sym]
    for kind, pts in ln.items():
        rows += [(kind, float(lam(x)), float(hca(y))) for x, y in sorted(pts.items())]
    order = ["plus", "square", "unknown", "line_dash", "line_solid"]
    rows.sort(key=lambda t: (order.index(t[0]), t[1]))
    os.makedirs("ref_data", exist_ok=True)
    with open(OUT_CSV, "w", encoding="utf-8") as f:
        f.write("kind,Lambda,hc_A\n")
        for k, L, h in rows:
            f.write(f"{k},{L:.5g},{h:.4f}\n")

    # 확인 그림: 원본(연회색) 위 + 파랑 · ■ 빨강 · unknown 주황 · 점선 초록 · 실선 자주 · 범례 제외 상자 회색
    im = Image.fromarray((150 + 0.41 * g).astype(np.uint8)).convert("RGB")
    im = im.resize((im.width * SCALE, im.height * SCALE), Image.NEAREST)
    dr = ImageDraw.Draw(im)
    l0, l1, c0, c1 = LEGEND
    dr.rectangle((c0 * SCALE, l0 * SCALE, (c1 + 1) * SCALE, (l1 + 1) * SCALE), outline=(120, 120, 120))
    col = {"plus": (0, 90, 255), "square": (230, 0, 0), "unknown": (255, 140, 0),
           "line_dash": (0, 170, 0), "line_solid": (200, 0, 200)}
    for kind, pts in ln.items():
        for x, y in pts.items():
            X, Y = (x + 0.5) * SCALE, (y + 0.5) * SCALE
            dr.ellipse((X - 1.5, Y - 1.5, X + 1.5, Y + 1.5), fill=col[kind])
    for r, c, k in sym:
        X, Y = (c + 0.5) * SCALE, (r + 0.5) * SCALE
        if k == "plus":
            dr.line((X - 6, Y, X + 6, Y), fill=col[k], width=2)
            dr.line((X, Y - 6, X, Y + 6), fill=col[k], width=2)
        else:
            dr.rectangle((X - 4, Y - 4, X + 4, Y + 4), outline=col[k], width=2)
    os.makedirs("figures", exist_ok=True)
    im.save(OUT_PNG)

    # 정량 점검: 계열별 개수 · 최대값 위치/크기
    print(f"[출력] {OUT_CSV} ({len(rows)} 행), {OUT_PNG}")
    for k in order:
        sel = [(L, h) for kk, L, h in rows if kk == k]
        if not sel:
            print(f"[{k}] 0 개")
            continue
        L, h = np.array(sel).T
        i = int(np.argmax(h))
        top3 = ", ".join(f"({L[j]:.3g}, {h[j]:.3f})" for j in np.argsort(h)[::-1][:3])
        print(f"[{k}] {len(sel)} 개, Λ {L.min():.3g}–{L.max():.3g}, 최대 h_c/A = {h[i]:.3f} @ Λ = {L[i]:.3g}; 상위 3 (Λ, h_c/A): {top3}")
    return rows


if __name__ == "__main__":
    main()
