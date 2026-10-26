"""Hooke & Li 2006 Part 1 Fig 6 — 회색 맞춤 곡선(식 19–25)만 디지타이징 (θ = 0°, 45°, 90°).

원본: 03. 정리/2006_SKF_Rapid_PureRolling_Part1/img_0017.jpg (θ=0), img_0018 (45), img_0019 (90) — 431×276 흑백 JPEG.
축: 가로 Λ = (λ/b)P^1.5 S^-2 로그 10^-2–10^2, 왼축 a/A (눈금 0.25), 오른축 Pλ/E'A (눈금 0.5).

1. 축 보정: 프레임 안쪽으로 뻗은 눈금(digitize_hl06.tick_cands)을 픽셀로 검출 → 초기 추정(프레임 = 10^-2·10^2,
   아래 프레임 = 0, 오른축 위 프레임 = 3, 왼축 눈금 간격 = 후보 간격 중앙값)으로 눈금 값에 맞추고(≤ 3 px)
   최소제곱 직선 px = a + b·v (x 는 v = log10 Λ) 회귀, |잔차| > 1.5 px 제외 후 재회귀. 잔차를 보고한다.
2. 회색 마스크: GRAY_LO ≤ 명도 ≤ GRAY_HI 이고, 3×3 이웃에 검정(명도 < DARK) 픽셀이 없는 픽셀.
   검정 기호·프레임·눈금·글자의 번짐 가장자리와 속 빈 □ 의 속(번져서 회색)은 검정에 붙어 있으므로 빠진다.
   곡선이 기호와 겹친 곳은 그래서 비어 있다(추정으로 채우지 않음). 8-연결 성분 MIN_CC 픽셀 미만은 잡음으로 지운다.
3. 열마다 회색 무리(run) 중심을 모두 기록(점 구름).
   curve 판별(classify): 이웃 열 무리를 1:1 로 이은 사슬(갈라짐·교차에서 끊김)의 기울기로 — 상승 = pres,
   하강 = amp(θ=45·90 만; θ=0 은 떨어지는 압력 가지와 구분 불가라 unknown), 평탄 = 단파장 구간(log10Λ ≤ −0.5)에서만
   a/A ≥ 0.5 → amp, 아래 → pres. 5 열 미만 사슬과 나머지는 unknown.
   branch: 압력 곡선, Λ > 1 열에서 pres 무리가 정확히 7 개일 때만 위→아래 1..7 (그 외 빈칸).

    python digitize_hl06_fig6.py
    -> ref_data/HL06_fig6_fit_digitized.csv, ref_data/HL06_fig6_calib.json, figures/HL06_fig6_digitize_check_{0,45,90}.png
"""
from __future__ import annotations

import json
import os

import numpy as np
from PIL import Image, ImageDraw
from scipy import ndimage

from digitize_fig2 import runs
from digitize_hl06 import tick_cands

SRC = "../../03. 정리/2006_SKF_Rapid_PureRolling_Part1/"
FIGS = {0: "img_0017.jpg", 45: "img_0018.jpg", 90: "img_0019.jpg"}
P15S2 = [0.35, 0.62, 1.4, 2.49, 5.59, 9.92, 22.4]     # 캡션: 장파장 가지 왼쪽→오른쪽
DARK = 100            # 검정(기호·프레임·눈금·글자)
GRAY_LO, GRAY_HI = 100, 190
MIN_CC = 4
SCALE = 3
MIN_SPAN = 5          # 판별할 조각의 최소 열 수
SLOPE = 0.2           # 상승/하강 판정 기울기(px/px)
SEED_LOG = -0.5       # log10 Λ ≤ 이 값: a/A ≥ 0.5 영역 = amp, 아래 = pres (그림상 두 곡선이 a/A≈1, ≈0 근처로 떨어져 있음)
# θ 라벨 글자 상자(행0, 행1, 열0, 열1) — 원본에서 잉크 범위를 재어 정함, 이 안의 회색 픽셀은 지움
LABEL_BOX = {0: (32, 52, 241, 299), 45: (32, 52, 216, 285), 90: (31, 52, 208, 276)}


# ---------------------------------------------------------------- 축 보정 ------
def frame(d):
    """>50 % 검정인 행·열 무리 → (위 프레임, 아래 프레임, 왼 프레임, 오른 프레임) 각 (s, e)."""
    rr = runs(np.where(d.sum(1) > 0.5 * d.shape[1])[0])
    cc = runs(np.where(d.sum(0) > 0.5 * d.shape[0])[0])
    return rr[0], rr[-1], cc[0], cc[-1]


def fit_ticks(cands, grid, a0, b0, tol=3.0):
    """눈금 후보 px → 초기 직선 a0 + b0·v 로 grid 값에 맞춤(값마다 가장 가까운 후보 하나, ≤ tol px)
    → 회귀, |잔차| > 1.5 px 제외 후 재회귀. 반환 dict(a, b, rms, max, pairs)."""
    cands = np.asarray(cands, float)
    pv = []
    for v in grid:
        p0 = a0 + b0 * v
        if len(cands) and np.min(np.abs(cands - p0)) <= tol:
            pv.append((float(cands[np.argmin(np.abs(cands - p0))]), float(v)))
    P, V = np.array(pv).T
    b, a = np.polyfit(V, P, 1)
    keep = np.abs(P - (a + b * V)) <= 1.5
    P, V = P[keep], V[keep]
    b, a = np.polyfit(V, P, 1)
    res = P - (a + b * V)
    return dict(a=float(a), b=float(b), n=int(len(P)), rms_px=round(float(np.sqrt(np.mean(res ** 2))), 3),
                max_abs_res_px=round(float(np.max(np.abs(res))), 3),
                ticks=[[round(p, 1), round(v, 4)] for p, v in zip(P, V)])


def v_of(px, c):
    return (np.asarray(px, float) - c["a"]) / c["b"]


def calibrate(d):
    top, bot, lf, rf = frame(d)
    cl, cr = lf[1] + 1, rf[0] - 1                 # 프레임 안쪽 첫 열
    rt, rb = top[1] + 1, bot[0] - 1               # 프레임 안쪽 첫 행
    # x: 아래 프레임에서 위로 난 눈금, 로그 격자 log10(k·10^n)
    xc = tick_cands(d, "B", rb, cl + 2, cr - 1, 3, 20)
    xgrid = np.log10(np.outer(np.arange(1, 10), 10.0 ** np.arange(-2, 2)).ravel())
    xgrid = np.r_[xgrid, 2.0]
    x = fit_ticks(xc, xgrid, cl - 0.5 + 2 * (cr - cl + 1) / 4, (cr - cl + 1) / 4)
    # 오른축: 0.5 간격, 아래 프레임 = 0, 위 프레임 = 3 (초기 추정)
    y0, y3 = 0.5 * (bot[0] + bot[1]), 0.5 * (top[0] + top[1])
    R = fit_ticks(tick_cands(d, "R", cr, rt + 1, rb, 3, 20), np.arange(0.5, 3.0, 0.5), y0, (y3 - y0) / 3)
    # 왼축: 0.25 간격, 아래 프레임 = 0, 간격은 후보 간격 중앙값
    lc = tick_cands(d, "L", cl, rt + 1, rb, 3, 20)
    dl = np.diff(lc)
    L = fit_ticks(lc, np.arange(0.25, 1.21, 0.25), y0, -float(np.median(dl[(dl > 20) & (dl < 80)])) / 0.25)
    fr = dict(top=[int(top[0]), int(top[1])], bottom=[int(bot[0]), int(bot[1])], left=[int(lf[0]), int(lf[1])], right=[int(rf[0]), int(rf[1])])
    # 보정 점검(눈금과 독립인 프레임 위치): 왼·오른 프레임 log10Λ, 아래 프레임 값, 위 프레임 오른축 값
    chk = dict(frame_left_log10=round(float(v_of(0.5 * (lf[0] + lf[1]), x)), 3),
               frame_right_log10=round(float(v_of(0.5 * (rf[0] + rf[1]), x)), 3),
               bottom_frame_left_axis=round(float(v_of(y0, L)), 4), bottom_frame_right_axis=round(float(v_of(y0, R)), 4),
               top_frame_right_axis=round(float(v_of(y3, R)), 4), top_frame_left_axis=round(float(v_of(y3, L)), 4))
    return dict(x=x, left=L, right=R, frame=fr, frame_check=chk), (cl, cr, rt, rb)


# ---------------------------------------------------------------- 회색 마스크 --
def gray_mask(g, box, label_box):
    cl, cr, rt, rb = box
    dark = g < DARK
    near = ndimage.binary_dilation(dark, structure=np.ones((3, 3), bool))
    m = (g >= GRAY_LO) & (g <= GRAY_HI) & ~near
    m[:rt + 1] = False
    m[rb:] = False
    m[:, :cl + 1] = False
    m[:, cr:] = False
    r0, r1, c0, c1 = label_box
    m[r0:r1 + 1, c0:c1 + 1] = False
    lab, n = ndimage.label(m, structure=np.ones((3, 3), bool))
    sz = ndimage.sum(m, lab, np.arange(1, n + 1))
    return m & np.isin(lab, np.where(sz >= MIN_CC)[0] + 1)


# ---------------------------------------------------------------- 곡선 판별 ----
def chains(cols, col_runs):
    """이웃 열 무리끼리 8-연결(행 범위가 1 px 안으로 닿음)이고 서로 유일한 짝이면 잇는다 → 사슬 목록 [[(열, 무리 번호)]].
    갈라짐·합쳐짐·교차(짝이 둘 이상)에서는 사슬이 끊긴다."""
    def touch(a, b):
        return a[0] <= b[1] + 1 and b[0] <= a[1] + 1
    nxt, prv = {}, {}
    for c in cols[:-1]:
        A, B = col_runs[c], col_runs[c + 1]
        for i, a in enumerate(A):
            js = [j for j, b in enumerate(B) if touch(a, b)]
            if len(js) == 1 and sum(touch(A[k], B[js[0]]) for k in range(len(A))) == 1:
                nxt[(c, i)] = (c + 1, js[0])
                prv[(c + 1, js[0])] = (c, i)
    out = []
    for c in cols:
        for i in range(len(col_runs[c])):
            if (c, i) in prv:
                continue
            ch = [(c, i)]
            while ch[-1] in nxt:
                ch.append(nxt[ch[-1]])
            out.append(ch)
    return out


def classify(cols, col_runs, theta, seed):
    """사슬별 라벨 → {(열, 무리 번호): 라벨}.
    전제(사용자 지정): a/A 곡선은 Λ 증가에 따라 1 → 0 감소, Pλ/E'A 는 θ=45·90 에서 단조 상승, θ=0 은 상승 후 감소.
    · 열 수 ≥ MIN_SPAN 인 사슬만 판별, 기울기 = 무리 중심 행의 직선 회귀(px/px, 그림 아래 방향 +).
    · 상승(기울기 ≤ −SLOPE) → pres (a/A 는 오르지 않음).
    · 하강(기울기 ≥ SLOPE): θ=45·90 → amp (압력은 내리지 않음), θ=0 → unknown (떨어지는 압력 가지와 구분 불가).
    · 평탄: 사슬 전체가 seed 영역(단파장 구간)에서 한 라벨일 때만, 그 밖은 unknown."""
    lab = {}
    for ch in chains(cols, col_runs):
        cc = np.array([c for c, _ in ch], float)
        yy = np.array([0.5 * sum(col_runs[c][i]) for c, i in ch])
        tag = "unknown"
        if len(ch) >= MIN_SPAN:
            sl = float(np.polyfit(cc, yy, 1)[0])
            if sl <= -SLOPE:
                tag = "pres"
            elif sl >= SLOPE:
                tag = "amp" if theta in (45, 90) else "unknown"
            else:
                sd = {seed(c, y) for c, y in zip(cc, yy)}
                tag = sd.pop() if len(sd) == 1 and None not in sd else "unknown"
        for key in ch:
            lab[key] = tag
    return lab


# ---------------------------------------------------------------- 그림 하나 ----
COL = {"amp": (230, 0, 0), "pres": (0, 90, 255), "unknown": (0, 170, 0)}


def one(theta):
    g = np.asarray(Image.open(SRC + FIGS[theta]).convert("L")).astype(float)
    d = g < 140
    cal, box = calibrate(d)
    cl, cr, rt, rb = box
    m = gray_mask(g, box, LABEL_BOX[theta])
    cols = list(range(cl + 2, cr - 1))
    col_runs = {c: runs(np.where(m[:, c])[0]) for c in cols}
    lab = classify(cols, col_runs, theta, lambda c, y: None if v_of(c, cal["x"]) > SEED_LOG else ("amp" if v_of(y, cal["left"]) >= 0.5 else "pres"))
    rows = []
    for c in cols:
        lg = float(v_of(c, cal["x"]))
        cand = [(i, 0.5 * (s + e), e - s + 1) for i, (s, e) in enumerate(col_runs[c])]
        pres_y = sorted(y for i, y, _ in cand if lab[(c, i)] == "pres")
        br = {y: k + 1 for k, y in enumerate(pres_y)} if lg > 0 and len(pres_y) == 7 else {}
        for i, y, n in cand:
            rows.append(dict(theta=theta, Lambda=10 ** lg, col=c, y_px=y, amp=float(v_of(y, cal["left"])),
                             pres=float(v_of(y, cal["right"])), run_len=n, curve=lab[(c, i)], branch=br.get(y, "")))
    # 확인 그림: 원본 확대 위에 추출점(라벨 색)
    im = Image.fromarray(g.astype(np.uint8)).convert("RGB")
    im = im.resize((im.width * SCALE, im.height * SCALE), Image.NEAREST)
    dr = ImageDraw.Draw(im)
    for r in rows:
        X, Y = (r["col"] + 0.5) * SCALE, (r["y_px"] + 0.5) * SCALE
        dr.ellipse((X - 1.5, Y - 1.5, X + 1.5, Y + 1.5), fill=COL[r["curve"]])
    path = f"figures/HL06_fig6_digitize_check_{theta}.png"
    im.save(path)
    return rows, cal, path


def endpoint(rows, curve, lg0, ref, half=0.3, tol=0.05):
    """정량 점검(끝점, 사용자 지정 기준값 ref): log10Λ ∈ lg0 ± half 안 점의 해당 축 값.
    labelled = curve 라벨 점, unknown = 라벨 없는 점(어느 곡선인지 미정 — 참고용). pass = labelled 평균이 ref ± tol."""
    out = {}
    for key, want in (("labelled", curve), ("unknown", "unknown")):
        sel = [r for r in rows if r["curve"] == want and abs(np.log10(r["Lambda"]) - lg0) <= half]
        if not sel:
            out[key] = dict(n=0)
            continue
        v = np.array([r["amp" if curve == "amp" else "pres"] for r in sel])
        lg = np.log10([r["Lambda"] for r in sel])
        out[key] = dict(n=int(len(v)), min=round(float(v.min()), 4), max=round(float(v.max()), 4), mean=round(float(v.mean()), 4),
                        log10Lambda=[round(float(lg.min()), 3), round(float(lg.max()), 3)])
    lab = out["labelled"]
    out["ref"] = ref
    out["pass"] = None if lab["n"] == 0 else bool(abs(lab["mean"] - ref) <= tol)
    return out


def main():
    os.makedirs("ref_data", exist_ok=True)
    allr, cals, checks = [], {}, {}
    for th in FIGS:
        rows, cal, path = one(th)
        allr += rows
        cals[th] = cal
        # 끝점 점검: 프레임 끝 열 근처(±0.1 decade)는 비어 있을 수 있어 가장 가까운 0.2 decade 도 함께
        checks[th] = {
            "amp@0.01": endpoint(rows, "amp", -2.0, 1.0),
            "amp@100": endpoint(rows, "amp", 2.0, 0.0),
            "pres@0.01": endpoint(rows, "pres", -2.0, 0.0),
            "max_pres_runs_per_col(Lambda>1)": max([sum(1 for r in rows if r["col"] == c and r["curve"] == "pres")
                                                    for c in {r["col"] for r in rows if r["Lambda"] > 1}] or [0]),
            "counts": {k: sum(1 for r in rows if r["curve"] == k) for k in COL},
        }
        print(th, path)
    with open("ref_data/HL06_fig6_fit_digitized.csv", "w", encoding="utf-8") as f:
        f.write("# Hooke & Li 2006 Part 1 Fig 6 grey fitted curves (eqs 19-25), digitize_hl06_fig6.py. Point cloud: every grey run centre per pixel column.\n")
        f.write("# value_left_axis = a/A (left-axis calibration), value_right_axis = P lambda/E'A (right-axis); both given for every point — use the one matching 'curve'.\n")
        f.write("# curve: amp | pres from continuity tracking from the left end; unknown = not attributable. Grey pixels next to black symbols/frame/ticks are blank (not interpolated).\n")
        f.write("# branch: pres only, Lambda>1 columns with exactly 7 pres runs, 1..7 top->bottom (= P^1.5S^-2 0.35..22.4 per caption order), else blank.\n")
        f.write("theta,Lambda,y_px,value_left_axis,value_right_axis,run_len,curve,branch\n")
        for r in allr:
            f.write(f"{r['theta']},{r['Lambda']:.6g},{r['y_px']:.1f},{r['amp']:.4f},{r['pres']:.4f},{r['run_len']},{r['curve']},{r['branch']}\n")
    with open("ref_data/HL06_fig6_calib.json", "w", encoding="utf-8") as f:
        json.dump(dict(calibration={str(k): v for k, v in cals.items()}, checks={str(k): v for k, v in checks.items()},
                       params=dict(DARK=DARK, GRAY=[GRAY_LO, GRAY_HI], MIN_CC=MIN_CC, MIN_SPAN=MIN_SPAN, SLOPE=SLOPE, SEED_LOG=SEED_LOG, LABEL_BOX={str(k): v for k, v in LABEL_BOX.items()})), f, ensure_ascii=False, indent=1)
    for th in FIGS:
        c = cals[th]
        print(th, {k: (c[k]["n"], c[k]["rms_px"], c[k]["max_abs_res_px"]) for k in ("x", "left", "right")}, c["frame_check"])
        print("  ", checks[th])


if __name__ == "__main__":
    main()
