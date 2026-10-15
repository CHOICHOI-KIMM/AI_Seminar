"""Hooke & Li 2006 Part 1 — Fig 3 · Fig 4 흑백 원문 그림 디지타이징.

축 보정: 각 축의 눈금(프레임에 붙은 짧은 선)을 픽셀로 검출 → 눈금 값(그림 라벨을 눈으로 읽어 지정)과
최소제곱 직선(px = a + b·value) 회귀. 회귀 잔차(px)를 정량 검사로 출력한다.

Fig 3 (δh/A · δp b/E'A, 3단 = λ/b 0.1·0.7·3.0)
  · λ/b 0.7·3.0 : 열마다 곡선 픽셀 무리 중 이전 값에 가장 가까운 무리 중심(kind=curve).
                  프레임·눈금·0 기준선은 지운다. 곡선이 0선과 겹치면(이전 값이 0선 근처, 다른 무리 없음) 0.
                  수직 급변(무리 높이 > JUMP_PX) 열은 무리 중심을 curve 로, 무리 양끝을 env_hi/env_lo 로 함께 기록.
  · λ/b 0.1     : 열마다 곡선 픽셀 최상·최하(env_hi/env_lo). 0선 행은 제외, 0선 외 픽셀이 없으면 0.
Fig 4 (기호 ▲=H/A 왼축, △=P b/E'A 오른축)
  · x 는 원문 41점 격자 λ/b = 10^(-4+0.15k) 로 고정하고, 격자 열 근처에서 ▲·△ 견본(같은 그림에서 오려낸
    템플릿)과의 정규화 상호상관(NCC) 봉우리 → 기호 bbox 중심 = 데이터 점(Grace 식 기호 ±s 대칭 가정).
  · 같은 종류 봉우리는 격자점당 하나, 다른 종류는 서로 6 px 안이면 점수가 낮은 쪽을 가려진 것으로 보고 NaN(flag=hidden).
  · smooth: 간극 h(왼축)·압력 p(오른축) 두 곡선을 x/b=0 에서 좌우로 추적(무리 배정: 예측값과 가까운 쪽).

    python digitize_hl06.py
    -> ref_data/HL06_fig3.csv, ref_data/HL06_fig4.csv, ref_data/HL06_fig4_smooth.csv, ref_data/HL06_fig4_hcen.json,
       figures/HL06_digitize_check_*.png
"""
from __future__ import annotations

import json
import os

import numpy as np
from numpy.lib.stride_tricks import sliding_window_view as swv
from PIL import Image, ImageDraw

from digitize_fig2 import runs

REF = "figures/ref/"
DARK = 140
JUMP_PX = 12
SCALE = 3          # 확인 그림 확대 배율


# ---------------------------------------------------------------- 공통 ---------
def load(name):
    g = np.asarray(Image.open(REF + name + ".jpg").convert("L")).astype(float)
    return g, g < DARK


def lines(d, axis, frac):
    """긴 직선(프레임·0선): 어두운 픽셀 비율 > frac 인 행(axis=1)/열(axis=0)의 연속 무리 [(s,e)]."""
    cnt = d.sum(axis)
    n = d.shape[axis]
    return runs(np.where(cnt > frac * n)[0])


def tick_cands(d, side, edge, lo, hi, lmin=6, lmax=17):
    """프레임 안쪽 가장자리 edge(px)에서 안으로 뻗은 짧은 선 → 중심 위치 목록.
    side: 'L','R' (행 방향 눈금), 'B','T' (열 방향 눈금). lo..hi 는 눈금을 찾을 범위."""
    out = []
    for p in range(lo, hi):
        n = 0
        while n < 40:
            if side == "L":
                v = d[p, edge + n]
            elif side == "R":
                v = d[p, edge - n]
            elif side == "B":
                v = d[edge - n, p]
            else:
                v = d[edge + n, p]
            if not v:
                break
            n += 1
        out.append(lmin <= n <= lmax)
    idx = np.where(out)[0] + lo
    return [0.5 * (s + e) for s, e in runs(idx) if e - s <= 3]


def calib(cands, anchor_px, anchor_val, step, spacing):
    """눈금 후보 → 값 배정(anchor 기준 step 배수) → px = a + b·v 회귀. 반환 (a, b, rms, 사용 (px,v))."""
    pv = []
    for p in cands:
        q = (p - anchor_px) / spacing
        if abs(q - round(q)) < 0.25:
            pv.append((p, anchor_val + step * round(q)))
    for _ in range(2):
        P, V = np.array(pv).T
        b, a = np.polyfit(V, P, 1)
        res = P - (a + b * V)
        keep = np.abs(res) < 1.5
        pv = [t for t, k in zip(pv, keep) if k]
    P, V = np.array(pv).T
    b, a = np.polyfit(V, P, 1)
    rms = float(np.sqrt(np.mean((P - (a + b * V)) ** 2)))
    return a, b, rms, pv


def v_of(px, cal):
    return (np.asarray(px, float) - cal[0]) / cal[1]


def px_of(v, cal):
    return cal[0] + cal[1] * np.asarray(v, float)


CAL_LOG = []


def log_cal(img, axis, cal):
    a, b, rms, pv = cal
    CAL_LOG.append(dict(image=img, axis=axis, n_ticks=len(pv), px_at_0=round(a, 2), px_per_unit=round(b, 3), rms_px=round(rms, 3),
                        max_abs_res_px=round(float(np.max(np.abs([p - (a + b * v) for p, v in pv]))), 3)))


def upscale(g):
    im = Image.fromarray((150 + 0.41 * g).astype(np.uint8)).convert("RGB")   # 원본을 연회색으로 — 겹친 점이 보이게
    return im.resize((im.width * SCALE, im.height * SCALE), Image.NEAREST)


def dot(dr, x, y, col, r=2):
    X, Y = (x + 0.5) * SCALE, (y + 0.5) * SCALE
    dr.ellipse((X - r, Y - r, X + r, Y + r), fill=col)


# ---------------------------------------------------------------- Fig 3 --------
FIG3 = {  # quantity: (파일, 세로 눈금 간격 값, 프레임 ±한계 참고)
    "dh": ("HL06_fig3_dh", 0.25),
    "dp": ("HL06_fig3_dp", 0.5),
}
LAMS = [0.1, 0.7, 3.0]
FRAME_LIM = {"dh": 1.2, "dp": 2.0}     # 프레임 ± 한계(눈금 배정용 근사, 회귀가 실제 값을 정함)


def fig3_one(q):
    name, ystep = FIG3[q]
    g, d = load(name)
    hl = lines(d, 1, 0.75)       # 프레임 위·아래와 0선 (각 2 px)
    vl = lines(d, 0, 0.75)
    cl, cr = vl[0][1] + 1, vl[-1][0] - 1          # 프레임 안쪽 첫 열
    hl_c = [0.5 * (s + e) for s, e in hl]
    assert len(hl) == 9, hl                       # 패널 3개 × (위, 0선, 아래)
    # x 축: 맨 아래 패널 아래 프레임에서 위로 난 눈금 (−2~2, 0.5 간격)
    bot = hl[8][0] - 1
    xc = tick_cands(d, "B", bot, cl + 2, cr - 1)
    xcal = calib(xc, 0.5 * (cl + cr), 0.0, 0.5, (cr - cl) / 8)
    log_cal(name, "x (bottom ticks)", xcal)
    rows, chk = [], upscale(g)
    dr = ImageDraw.Draw(chk)
    tick_cols = px_of(np.arange(-1.5, 1.6, 0.5), xcal)     # 가로 눈금 열(위·아래 프레임) — ±1 열은 건너뜀
    for ip, lam in enumerate(LAMS):
        top_r, zero_r, bot_r = hl[3 * ip], hl[3 * ip + 1], hl[3 * ip + 2]
        yc = tick_cands(d, "L", cl, top_r[1] + 2, bot_r[0] - 1)
        sp = (hl_c[3 * ip + 2] - hl_c[3 * ip]) / (2 * FRAME_LIM[q] / ystep)   # 눈금 간격 px 근사(배정용)
        ycal = calib(yc, 0.5 * (zero_r[0] + zero_r[1]), 0.0, ystep, -sp)     # 세로축: 값↑ = px↓
        log_cal(name, f"y lam_b={lam}", ycal)
        # 지울 픽셀: 프레임 밖, 0선, 눈금
        m = d.copy()
        m[:, :cl] = False
        m[:, cr + 1:] = False
        m[: top_r[1] + 1] = False
        m[bot_r[0]:] = False
        lim = FRAME_LIM[q]
        nt = int(lim / ystep + 1e-9)
        for p in px_of(np.arange(-nt, nt + 1) * ystep, ycal):
            r = int(round(p))            # 세로 눈금 행(검출 여부와 무관하게 값 격자 전부)
            m[r - 1: r + 2, cl: cl + 17] = False
            m[r - 1: r + 2, cr - 16: cr + 1] = False
        z0, z1 = zero_r
        zc = 0.5 * (z0 + z1)
        r_in0, r_in1 = top_r[1] + 1, bot_r[0] - 1      # 패널 안 행 범위(양 끝 포함)
        cols = np.array([c for c in range(cl + 2, cr - 1) if np.min(np.abs(tick_cols - c)) > 1.5])   # 프레임 번짐 2 열 제외
        xb = v_of(cols, xcal)
        if lam == 0.1:
            for c, x in zip(cols, xb):
                rr = np.where(m[r_in0: r_in1 + 1, c])[0] + r_in0
                nz = rr[(rr < z0 - 0) | (rr > z1 + 0)]
                if len(rr) == 0:
                    continue
                if len(nz) == 0:
                    hi = lo = zc
                else:
                    hi, lo = nz.min(), nz.max()
                    # 0선을 지나는 곡선이면 0선이 곡선의 일부일 수 있다 — 극값만 기록하므로 영향 없음
                for kind, r in (("env_hi", hi), ("env_lo", lo)):
                    rows.append((q, lam, kind, x, float(v_of(r, ycal))))
                    dot(dr, c, r, (255, 0, 0) if kind == "env_hi" else (0, 0, 255))
            continue
        prev = None
        for c, x in zip(cols, xb):
            col = m[r_in0: r_in1 + 1, c]
            rr = runs(np.where(col)[0] + r_in0)
            cand, zero_only = [], False
            for s, e in rr:
                if s >= z0 and e <= z1:
                    zero_only = True
                    continue
                if s <= z1 and e >= z0:              # 0선을 포함한 무리
                    up, dn = z0 - s, e - z1          # 0선 위·아래로 뻗은 길이
                    if up > 2 and dn <= 0:
                        s2, e2 = s, z0 - 1
                    elif dn > 2 and up <= 0:
                        s2, e2 = z1 + 1, e
                    else:
                        s2, e2 = s, e
                    cand.append((s2, e2))
                else:
                    cand.append((s, e))
            if not cand:
                if zero_only and (prev is None or abs(prev - zc) <= 3):
                    cand = [(z0, z1)]                # 곡선이 0선과 겹침 → 0
                else:
                    continue
            if prev is None:
                s, e = max(cand, key=lambda t: t[1] - t[0])
            else:
                s, e = min(cand, key=lambda t: min(abs(t[0] - prev), abs(t[1] - prev), abs(0.5 * (t[0] + t[1]) - prev)))
            prev = 0.5 * (s + e)
            rows.append((q, lam, "curve", x, float(v_of(prev, ycal))))
            dot(dr, c, prev, (255, 0, 0))
            if e - s > JUMP_PX:                      # 수직 급변 — 무리 양 끝도 기록
                rows.append((q, lam, "env_hi", x, float(v_of(s, ycal))))
                rows.append((q, lam, "env_lo", x, float(v_of(e, ycal))))
                dot(dr, c, s, (0, 160, 0))
                dot(dr, c, e, (0, 160, 0))
    path = f"figures/HL06_digitize_check_fig3_{q}.png"
    chk.save(path)
    return rows, path


# ---------------------------------------------------------------- Fig 4 기호 ---
KGRID = np.arange(41)
LGRID = -4 + 0.15 * KGRID          # log10(λ/b)

# 오른축 눈금 간격(값)과 긴 눈금 간격: 그림 라벨을 눈으로 읽음
#   trans/inline D: 0.4 라벨, 0.2 눈금 · E: 1 라벨, 0.5 눈금 · F: 10 라벨, 5 눈금
RSTEP = {"D": 0.2, "E": 0.5, "F": 5.0}


def ncc_map(k, T, w=None):
    """가중 정규화 상호상관. w=0 인 화소(프레임·0선·눈금)는 창·견본 모두에서 뺀다 — 선 위 기호도 잡히게.
    출력 좌표 = 견본 bbox 중심(정수 부분); 소수 부분 (oy, ox) 를 함께 반환."""
    th, tw = T.shape
    H, W = k.shape
    if w is None:
        w = np.ones_like(k)
    hh, ww = H - th + 1, W - tw + 1
    S = {n: np.zeros((hh, ww)) for n in ("w", "wT", "wT2", "wI", "wI2", "wIT")}
    for i in range(th):
        for j in range(tw):
            wi, Ii, t = w[i:i + hh, j:j + ww], k[i:i + hh, j:j + ww], T[i, j]
            S["w"] += wi
            S["wT"] += wi * t
            S["wT2"] += wi * t * t
            wI = wi * Ii
            S["wI"] += wI
            S["wI2"] += wI * Ii
            S["wIT"] += wI * t
    sw = np.maximum(S["w"], 1e-9)
    num = S["wIT"] - S["wI"] * S["wT"] / sw
    vI = S["wI2"] - S["wI"] ** 2 / sw
    vT = S["wT2"] - S["wT"] ** 2 / sw
    r = num / np.sqrt(np.maximum(vI * vT, 1e-9))
    r[S["w"] < 0.6 * th * tw] = -1.0                 # 6 할 이상 가려진 창은 판정 안 함
    out = np.full(k.shape, -1.0)
    oy, ox = (th - 1) / 2, (tw - 1) / 2
    out[int(oy): int(oy) + hh, int(ox): int(ox) + ww] = r
    return out, oy - int(oy), ox - int(ox)


def tmpl_from(k, r0, r1, c0, c1, thr=100):
    sub = k[r0:r1, c0:c1] > thr
    rr, cc = np.where(sub.any(1))[0], np.where(sub.any(0))[0]
    return k[r0 + rr[0]: r0 + rr[-1] + 1, c0 + cc[0]: c0 + cc[-1] + 1].copy()


# 견본 기호: trans_D 실수부 패널의 고립된 ▲(λ/b≈1, h) · △(λ/b≈2, p) — 모든 그림에 같은 견본 사용
def templates():
    g, _ = load("HL06_fig4_trans_D")
    k = 255 - g
    return tmpl_from(k, 195, 210, 268, 288), tmpl_from(k, 289, 304, 284, 304)


def peaks(prof, r0, r1, thr):
    out = []
    for r in range(max(r0, 1), min(r1, len(prof) - 1)):
        if prof[r] > thr and prof[r] >= prof[r - 1] and prof[r] >= prof[r + 1]:
            # 포물선 보간으로 행 부화소 위치
            den = prof[r - 1] - 2 * prof[r] + prof[r + 1]
            dlt = 0.5 * (prof[r - 1] - prof[r + 1]) / den if den < 0 else 0.0
            out.append((r + dlt, float(prof[r])))
    return out


THR = {"F": 0.50, "O": 0.45}     # 상관 하한(낮게 두고 속·bbox 어두움 검사로 가짜 봉우리를 거른다)
SEP = 6.0
BOX_INK = 85       # 기호 bbox(9×9, 선 제외) 평균 어두움 하한 — 빈 선·글자(imaginary)·프레임 가짜 봉우리(≤ 70) 제거, 실제 기호 ≥ 91
INK = (190, 110)   # 속 어두움(0~255) 판정: ▲ ≥ 190 (실측 ≈ 215~229, △ 에 대한 ▲ 견본 교차응답 120~173), △ < 110 (실측 ≈ 0~90), 사이는 판정 불가


def symbols(nF, nO, cx, r0, r1, offs, inner):
    """격자 열 cx 근처에서 ▲(F)·△(O) 위치(bbox 중심 행)·점수·flag.
    상관만으로는 0선 위 △ 가 ▲ 견본에 더 높게 맞기도 하므로, 후보마다 기호 속(중심~아래 2 행, 가운데 3 열)
    어두움 inner(r, c) 로 종류를 확인한다: ▲ 는 속이 어둡고 △ 는 밝다. 확인 실패 후보는 버린다."""
    cands, rej = [], []
    for typ, n, (oy, ox) in (("F", nF, offs[0]), ("O", nO, offs[1])):
        c = int(round(cx - ox))
        for r, s in peaks(n[:, c - 1: c + 2].max(1), r0, r1, THR[typ]):
            rr = int(round(r))
            cc = c - 1 + int(np.argmax(n[rr, c - 1: c + 2]))
            ink, box = inner(r + oy, cc + ox)
            ok = (ink >= INK[0] if typ == "F" else ink < INK[1]) and box >= BOX_INK
            (cands if ok else rej).append((s, typ, r + oy, cc + ox))
    cands.sort(reverse=True)
    got, flags = {}, {"F": "", "O": ""}
    for s, typ, r, c in cands:
        if typ in got:
            continue
        other = "O" if typ == "F" else "F"
        if other in got and abs(got[other][0] - r) < SEP:
            flags[typ] = "hidden"     # 다른 기호와 겹친 자리의 약한 봉우리 → 못 읽음
            continue
        got[typ] = (r, s, c)
    for typ in ("F", "O"):
        other = "O" if typ == "F" else "F"
        if typ in got:
            if other in got and abs(got[other][0] - got[typ][0]) < 2 * SEP:
                flags[typ] = flags[typ] or "near_other"
        elif not flags[typ]:
            # 종류 확인에 실패한 이 종류 후보가 다른 기호 자리에 있었다 → 겹쳐 쌓인 기호(못 읽음)
            stacked = other in got and any(t == typ and abs(r - got[other][0]) < 1.5 * SEP for _, t, r, _ in rej)
            flags[typ] = "hidden" if stacked else "not_found"
    return got, flags


def fig4_sym(case, kind, TF, TO):
    name = f"HL06_fig4_{kind}_{case}"
    g, d = load(name)
    k = 255 - g
    H, W = d.shape
    hl = lines(d, 1, 0.5)
    vl = lines(d, 0, 0.3)
    vl = [t for t in vl if t[1] - t[0] < 4 and (t[0] > 40 and t[1] < W - 40)]
    cl, cr = vl[0][1] + 1, vl[-1][0] - 1
    if kind == "trans":
        # hl: 위패널 위, 위패널 아래(=허수 0, 기호 겹쳐 두꺼움), 아래패널 위, 0선, (기호 바닥 행), 아래
        mid = 0.5 * (hl[2][1] + hl[-1][0])          # 실수 패널 0선 = 패널 가운데(±1.2 프레임)
        zr = min(hl[3:-1], key=lambda t: abs(0.5 * (t[0] + t[1]) - mid))
        top_i, imz, top_r, bot = hl[0], hl[1], hl[2], hl[-1]
        panels = [("im", top_i, imz, imz), ("re", top_r, zr, bot)]
    else:
        panels = [("re", hl[0], hl[1], hl[2])]
    # x 축: 아래 프레임 위로 난 10 년 긴 눈금 (길이 13~17) — 10^-3..10^1
    bot_edge = hl[-1][0] - 1
    xc = tick_cands(d, "B", bot_edge, cl + 3, cr - 2, 13, 17)
    xcal = calib(xc, 0.5 * (cl + cr), -1.0, 1.0, (cr - cl) / 6)
    log_cal(name, "x log10 (decade ticks)", xcal)
    xchk = (float(v_of(cl - 0.5, xcal)), float(v_of(cr + 0.5, xcal)))   # 프레임 = 10^-4, 10^2 이어야 함
    CAL_LOG[-1]["frame_log10"] = [round(xchk[0], 3), round(xchk[1], 3)]
    # 상관 계산에서 뺄 화소: 프레임·0선 행(무리 중 가장 진한 2 행), 가로축 로그 눈금 열(프레임에서 16 px 이내)
    w = np.ones_like(k)
    cnt = d.sum(1)
    for s_, e_ in hl:
        rr = np.arange(s_, e_ + 1)
        for r_ in rr[np.argsort(cnt[rr])[-2:]] if e_ - s_ >= 2 else rr:
            w[r_] = 0
    tick_lg = np.log10(np.outer(np.arange(1, 10), 10.0 ** np.arange(-4, 2)).ravel())
    tick_px = px_of(tick_lg, xcal)
    edges = [hl[-1][0]]
    if kind == "trans":       # 허수 패널 아래 프레임(=0) 의 진한 두 행 중 위쪽
        rr = np.arange(hl[1][0], hl[1][1] + 1)
        edges.append(int(rr[np.argsort(cnt[rr])[-2:]].min()))
    for e_ in edges:
        for c_ in tick_px:
            c_ = int(round(c_))
            w[e_ - 16: e_, c_ - 1: c_ + 2] = 0
    w[:, : cl + 1] = 0
    w[:, cr:] = 0
    nF, oyF, oxF = ncc_map(k, TF, w)
    nO, oyO, oxO = ncc_map(k, TO, w)

    def inner(rc, cc):
        """기호 속 어두움: bbox 중심 ~ 아래 2 행 × 가운데 3 열(선·눈금 화소 제외) 평균."""
        r_, c_ = int(round(rc)), int(round(cc))
        blk, wb = k[r_: r_ + 3, c_ - 1: c_ + 2], w[r_: r_ + 3, c_ - 1: c_ + 2]
        ins = float((blk * wb).sum() / wb.sum()) if wb.sum() > 0 else np.nan
        bb, wbb = k[r_ - 4: r_ + 5, c_ - 4: c_ + 5], w[r_ - 4: r_ + 5, c_ - 4: c_ + 5]
        box = float((bb * wbb).sum() / wbb.sum()) if wbb.sum() > 0 else 0.0
        return ins, box
    offs = ((oyF, oxF), (oyO, oxO))
    rows, chk = [], upscale(g)
    dr = ImageDraw.Draw(chk)
    xdev = []
    yr_all = tick_cands(d, "R", cr, hl[0][1] + 2, hl[-1][0] - 1)      # 오른축 눈금 간격 근사용(그림 전체)
    for part, ptop, pzero, pbot in panels:
        zc = 0.5 * (pzero[0] + pzero[1])
        # 왼축: 0 기준선에서 0.25 간격 눈금
        if part == "im":
            yl = tick_cands(d, "L", cl, ptop[1] + 2, pzero[0] - 1)
            yr = tick_cands(d, "R", cr, ptop[1] + 2, pzero[0] - 1)
            zpx = pzero[0] + 0.5        # 허수 패널 0 = 아래 프레임(첫 2 행이 선, 그 아래는 기호)
        else:
            yl = tick_cands(d, "L", cl, ptop[1] + 2, pbot[0] - 1)
            yr = tick_cands(d, "R", cr, ptop[1] + 2, pbot[0] - 1)
            zpx = zc
        lsp = 22.3          # 0.25 당 px 근사(배정용) — 회귀가 실제 값을 정함
        lcal = calib(yl, zpx, 0.0, 0.25, -lsp)
        log_cal(name, f"{part} left H/A", lcal)
        # 오른축 눈금 간격(px) 근사: 같은 그림에서 오른쪽 프레임 눈금 후보 간 최빈 간격
        dif = np.diff(yr_all)
        rsp = float(np.median(dif[(dif > 12) & (dif < 40)]))
        rcal = calib(yr, zpx, 0.0, RSTEP[case], -rsp)
        log_cal(name, f"{part} right PbE'A", rcal)
        # 탐색 행 범위: 프레임 안(허수는 0 아래 0.2 까지 허용)
        r0 = int(ptop[1] + 3)
        # 실수 패널 아래 6 px 는 가로축 눈금 숲 — 기호 중심이 그 안에 올 수 없다(값 < −1.13)
        r1 = int(px_of(-0.2, lcal)) if part == "im" else int(pbot[0] - 6)
        for kk, lg in zip(KGRID, LGRID):
            cx = float(px_of(lg, xcal))
            got, flags = symbols(nF, nO, cx, r0, r1, offs, inner)
            for typ, qn, cal, col in (("F", "H", lcal, (230, 0, 0)), ("O", "P", rcal, (0, 90, 255))):
                if typ in got:
                    r = got[typ][0]
                    val = float(v_of(r, cal))
                    dot(dr, cx, r, col, 3)
                    # 기호의 실제 열 위치(격자 확인용): 봉우리 행에서 열 방향 점수 최대
                    n = nF if typ == "F" else nO
                    ox = offs[0][1] if typ == "F" else offs[1][1]
                    c0 = int(round(cx - ox))
                    rr = int(round(r - (offs[0][0] if typ == "F" else offs[1][0])))
                    seg = n[rr, c0 - 3: c0 + 4]
                    xdev.append(int(np.argmax(seg)) - 3 + (c0 + ox - cx))
                else:
                    val = np.nan
                    dr.line(((cx + .5) * SCALE - 4, 2 * SCALE, (cx + .5) * SCALE + 4, 2 * SCALE + 8), fill=col, width=2)
                rows.append(dict(case=case, theta=0 if kind == "trans" else 90, part=part, quantity=qn,
                                 k=int(kk), lam_b=10 ** lg, value=val, flag="frame" if kk in (0, 40) and not np.isfinite(val) else flags[typ], px_row=got.get(typ, (np.nan,))[0]))
    # 격자선 표시
    for lg in LGRID:
        x = (float(px_of(lg, xcal)) + 0.5) * SCALE
        dr.line((x, 0, x, 4), fill=(0, 170, 0))
    path = f"figures/HL06_digitize_check_fig4_{kind}_{case}.png"
    chk.save(path)
    return rows, path, float(np.mean(np.abs(xdev))) if xdev else np.nan


# ---------------------------------------------------------------- Fig 4 smooth -
HSTEP = {"D": 0.1, "E": 0.025, "F": 0.005}
HTOP = {"D": 1.0, "E": 0.15, "F": 0.03}      # 왼축 프레임 위 라벨 값(배정용)   # 왼축 눈금 간격(라벨 1.0/0.5 · 0.15/0.10/0.05 · 0.03/0.02/0.01)


def fig4_smooth(case):
    name = f"HL06_fig4_smooth_{case}"
    g, d = load(name)
    hl = lines(d, 1, 0.6)
    vl = [t for t in lines(d, 0, 0.6) if t[1] - t[0] < 4]
    top, bot = hl[0], hl[-1]
    cl, cr = vl[0][1] + 1, vl[-1][0] - 1
    xc = tick_cands(d, "B", bot[0] - 1, cl + 3, cr - 2, 8, 18)
    # x: 눈금 간격 0.5, 왼 프레임 = −2 를 기준점으로 배정
    xcal = calib(xc, cl - 0.5, -2.0, 0.5, float(np.median(np.diff(xc))))
    log_cal(name, "x", xcal)
    yl = tick_cands(d, "L", cl, top[1] + 2, bot[0] - 1)
    yr = tick_cands(d, "R", cr, top[1] + 2, bot[0] - 1)
    span = bot[0] - 0.5 * (top[0] + top[1])       # 프레임 위 = 왼축 HTOP · 오른축 1.5, 아래 = 0
    lcal = calib(yl, bot[0] + 0.5, 0.0, HSTEP[case], -span * HSTEP[case] / HTOP[case])
    rcal = calib(yr, bot[0] + 0.5, 0.0, 0.25, -span * 0.25 / 1.5)
    log_cal(name, "left hR/b2", lcal)
    log_cal(name, "right p/pH", rcal)
    m = d.copy()
    m[: top[1] + 1] = False
    m[bot[0]:] = False
    m[:, :cl] = False
    m[:, cr + 1:] = False
    for p, _ in xcal[3]:
        c = int(round(p))
        m[bot[0] - 18: bot[0], c - 1: c + 2] = False
        m[top[1] + 1: top[1] + 19, c - 1: c + 2] = False
    # 세로 눈금 행: 검출 여부와 무관하게 값 격자 전부(긴 눈금 ≈ 15 px)
    for p in px_of(np.arange(1, round(HTOP[case] / HSTEP[case])) * HSTEP[case], lcal):
        r = int(round(p))
        m[r - 1: r + 2, cl: cl + 17] = False
    for p in px_of(np.arange(1, 6) * 0.25, rcal):
        r = int(round(p))
        m[r - 1: r + 2, cr - 16: cr + 1] = False
    r_in0, r_in1 = top[1] + 1, bot[0] - 1
    c0 = int(round(px_of(0.0, xcal)))

    def col_runs(c):
        return runs(np.where(m[r_in0: r_in1 + 1, c])[0] + r_in0)

    # x=0 열: 위 무리 = p(돔 정점), 아래 무리 = h
    rr = col_runs(c0)
    assert len(rr) == 2, (case, rr)
    start = {"p": 0.5 * sum(rr[0]), "h": 0.5 * sum(rr[1])}
    res = {"h": {}, "p": {}}
    flag = {}
    def gap(t, v):           # 예측 행 v 와 무리 t=(s,e) 사이 거리(무리 안이면 0)
        return 0.0 if t[0] <= v <= t[1] else min(abs(t[0] - v), abs(t[1] - v))

    for direction in (+1, -1):
        # hist: 두 곡선이 갈라져 있던 열의 (열, 행) — 교차·겹침 열은 넣지 않아 교차 뒤에도 기울기가 이어진다
        hist = {key: [(c0, start[key])] for key in start}
        miss = {"p": 0, "h": 0}
        c = c0 + direction
        while cl + 1 <= c <= cr - 1:
            rr = col_runs(c)
            pred, tol = {}, {}
            for key in hist:
                if miss[key] > 6:                       # 7 열 넘게 못 따라가면 이 방향 추적 중단
                    pred[key] = None
                    continue
                pts = np.array(hist[key][-4:], float)
                if len(pts) >= 2:
                    sl, ic = np.polyfit(pts[:, 0], pts[:, 1], 1)
                else:
                    sl, ic = 0.0, pts[0, 1]
                pred[key] = sl * c + ic if len(pts) >= 2 else ic
                tol[key] = 4.0 + 1.5 * abs(sl) * (1 + miss[key])
            live = [key for key in ("p", "h") if pred[key] is not None]
            assign = {}
            if pred["h"] is None and pred["p"] is not None and pred["p"] >= bot[0] - 4 and len(rr) == 1:
                # 겹침 구간 뒤 h 를 놓쳤고 p 는 바닥(0)에 붙어 있다 → 남은 유일한 무리가 h (재포착)
                s_, e_ = rr[0]
                hist["h"] = [(c, 0.5 * (s_ + e_))]
                miss["h"] = 0
                flag[c] = "h_reacquired"
                live = ["p"]
                assign["h"] = rr[0]
                res["h"][c] = 0.5 * (s_ + e_)
            if len(live) == 2 and len(rr) >= 2:
                best = None
                for a in rr:
                    for b in rr:
                        if a == b:
                            continue
                        cost = gap(a, pred["p"]) + gap(b, pred["h"])
                        if best is None or cost < best[0]:
                            best = (cost, a, b)
                for key, t in (("p", best[1]), ("h", best[2])):
                    if gap(t, pred[key]) <= tol[key]:
                        assign[key] = t
            elif len(rr) >= 1 and live:
                for key in live:
                    t = min(rr, key=lambda t: gap(t, pred[key]))
                    if gap(t, pred[key]) <= tol[key]:
                        assign[key] = t
                if len(assign) == 2 and assign["p"] == assign["h"]:
                    s_, e_ = assign["p"]
                    if e_ - s_ <= 4:
                        flag[c] = "cross"                   # 교차점: 두 곡선 같은 값(기울기 기록 안 함)
                    else:
                        flag[c] = "merged"                  # 겹친 긴 무리 — 둘 다 못 읽음
                        assign = {}
                    for key in assign:
                        res[key][c] = 0.5 * (s_ + e_)
                    miss = {k_: miss[k_] + 1 for k_ in miss}
                    c += direction
                    continue
            for key in ("p", "h"):
                if key in assign:
                    s_, e_ = assign[key]
                    v = 0.5 * (s_ + e_)
                    res[key][c] = v
                    if flag.get(c) != "h_reacquired" or key != "h":
                        hist[key].append((c, v))
                    miss[key] = 0
                elif key == "p" and pred["p"] is not None and pred["p"] >= bot[0] - 4:
                    # p 가 바닥 프레임(=0)과 겹침 → 0 (프레임 행은 지웠으므로 무리가 없다)
                    res["p"][c] = bot[0] + 0.5
                    hist["p"].append((c, bot[0] + 0.5))
                    miss["p"] = 0
                    flag.setdefault(c, "p_on_frame")
                else:
                    miss[key] += 1
            c += direction
    rows, chk = [], upscale(g)
    dr = ImageDraw.Draw(chk)
    for key, cal, colr in (("h", lcal, (230, 0, 0)), ("p", rcal, (0, 90, 255))):
        for c in sorted(res[key]):
            v = res[key][c]
            rows.append((case, key, float(v_of(c, xcal)), float(v_of(v, cal))))
            dot(dr, c, v, colr)
    # 중심 간극: x/b = 0 를 끼는 두 열의 선형보간 + ±2 열 평균 비교
    xs = np.array([c for c in sorted(res["h"])])
    hv = np.array([v_of(res["h"][c], lcal) for c in xs])
    xb = v_of(xs, xcal)
    hcen = float(np.interp(0.0, xb, hv))
    near = hv[np.abs(xb) <= 3.5 / xcal[1]]          # x/b=0 ±3 열
    path = f"figures/HL06_digitize_check_fig4_smooth_{case}.png"
    chk.save(path)
    px_unit = lcal[1]
    return rows, path, dict(case=case, h_center_hR_b2=round(hcen, 5),
                            h_center_pm_3col_min_max=[round(float(near.min()), 5), round(float(near.max()), 5)],
                            pixel_resolution_hR_b2=round(1.0 / abs(px_unit), 6),
                            p_center_p_pHertz=round(float(np.interp(0.0, v_of(np.array(sorted(res["p"])), xcal),
                                                                     [v_of(res["p"][c], rcal) for c in sorted(res["p"])])), 4),
                            flags={k: int(sum(1 for f in flag.values() if f == k)) for k in set(flag.values())})


# ---------------------------------------------------------------- main ---------
def main():
    os.makedirs("ref_data", exist_ok=True)
    checks = []
    # Fig 3
    allr = []
    for q in FIG3:
        r, p = fig3_one(q)
        allr += r
        checks.append(p)
    with open("ref_data/HL06_fig3.csv", "w", encoding="utf-8") as f:
        f.write("# Hooke & Li 2006 Part 1 Fig 3 digitized (digitize_hl06.py). dh = delta h/A, dp = delta p b/E'A.\n")
        f.write("# lam_b 0.7/3.0: curve = 열마다 곡선 무리 중심; 수직 급변(무리>12px) 열은 env_hi/env_lo 로 무리 양끝 추가.\n")
        f.write("# lam_b 0.1: env_hi/env_lo = 열마다 곡선 픽셀 최상/최하. 프레임 한계(dh +-1.2, dp +-2)에 닿은 값은 잘린 값(clipped).\n")
        f.write("quantity,lam_b,kind,x_b,value\n")
        for q, lam, kind, x, v in allr:
            f.write(f"{q},{lam},{kind},{x:.4f},{v:.4f}\n")
    # Fig 4 symbols
    TF, TO = templates()
    sym, xdevs = [], {}
    for kind in ("trans", "inline"):
        for case in "DEF":
            r, p, xd = fig4_sym(case, kind, TF, TO)
            sym += r
            checks.append(p)
            xdevs[f"{kind}_{case}"] = round(xd, 2)
    with open("ref_data/HL06_fig4.csv", "w", encoding="utf-8") as f:
        f.write("# Hooke & Li 2006 Part 1 Fig 4 digitized (digitize_hl06.py). theta 0 = transverse, 90 = in-line (원문 Fig 4 캡션).\n")
        f.write("# H = H/A (filled triangle, left axis), P = P b/E'A (open triangle, right axis); lam_b = 10^(-4+0.15k).\n")
        f.write("# NaN flag: hidden = 다른 기호와 겹쳐 쌓여 못 읽음, not_found = 눈금 숲/선 위 등에서 기호 미검출, frame = 프레임 위(k=0,40). 값 있는 flag near_other = 다른 기호와 12px 이내(겹침 기록).\n")
        f.write("case,theta,part,quantity,lam_b,value,flag,k\n")
        for t in sym:
            f.write(f"{t['case']},{t['theta']},{t['part']},{t['quantity']},{t['lam_b']:.6g},"
                    f"{'NaN' if not np.isfinite(t['value']) else format(t['value'], '.4f')},{t['flag']},{t['k']}\n")
    # Fig 4 smooth
    sm, hcen = [], {}
    for case in "DEF":
        r, p, info = fig4_smooth(case)
        sm += r
        checks.append(p)
        hcen[case] = info
    with open("ref_data/HL06_fig4_smooth.csv", "w", encoding="utf-8") as f:
        f.write("# Hooke & Li 2006 Part 1 Fig 4 smooth solutions: h = hR/b^2 (left axis), p = p/p_Hertz (right axis).\n")
        f.write("case,curve,x_b,value\n")
        for case, key, x, v in sm:
            f.write(f"{case},{key},{x:.4f},{v:.5f}\n")
    with open("ref_data/HL06_fig4_hcen.json", "w", encoding="utf-8") as f:
        json.dump(dict(source="Hooke & Li 2006 Part 1 Fig 4 smooth panels, digitize_hl06.py", cases=hcen), f,
                  ensure_ascii=False, indent=1)
    chk_known = verify(sym, hcen)
    with open("ref_data/HL06_calib.json", "w", encoding="utf-8") as f:
        json.dump(dict(calibration=CAL_LOG, symbol_x_dev_from_grid_px=xdevs, known_point_checks=chk_known), f,
                  ensure_ascii=False, indent=1)
    return sym, sm, hcen, xdevs, checks


def verify(sym, hcen):
    """정량 검사: 물리적으로 알려진 점근값과 디지타이징 값 비교(축 보정과 독립인 기준).
    trans/inline 실수부 H: 단파장 → 1, 장파장(k=35..39) → 0 · 허수부 H, P: 단파장 → 0 · 실수부 P: 단파장 → 0.
    각 항목 (읽은 점 수, 평균, 최대|편차|) 와 px 환산 평균 편차(+ = 그림에서 위쪽)."""
    out = []

    def sel(case, theta, part, q, ks):
        return np.array([t["value"] for t in sym if t["case"] == case and t["theta"] == theta and t["part"] == part
                         and t["quantity"] == q and t["k"] in ks and np.isfinite(t["value"])])

    pxu = {c["image"] + "|" + c["axis"]: abs(c["px_per_unit"]) for c in CAL_LOG}
    for case in "DEF":
        for theta, kind in ((0, "trans"), (90, "inline")):
            img = f"HL06_fig4_{kind}_{case}"
            # 평탄 구간: D·E 는 k=1..10 이 그림상 평탄, F 는 실수부 h·p 가 일찍 변하므로 k=1..3 (h) · k=1 (p) 만
            kh, kp = (range(1, 11), range(1, 11)) if case != "F" else (range(1, 4), range(1, 2))
            items = [("re", "H", kh, 1.0, "re left H/A"), ("re", "H", range(35, 40), 0.0, "re left H/A"),
                     ("re", "P", kp, 0.0, "re right PbE'A")]
            if theta == 0:
                items += [("im", "H", range(1, 11), 0.0, "im left H/A"), ("im", "P", range(1, 11), 0.0, "im right PbE'A")]
            for part, q, ks, ref, ax in items:
                v = sel(case, theta, part, q, ks)
                if len(v) == 0:
                    out.append(dict(image=img, part=part, q=q, k=f"{ks.start}-{ks.stop - 1}", ref=ref, n=0))
                    continue
                dev = v - ref
                out.append(dict(image=img, part=part, q=q, k=f"{ks.start}-{ks.stop - 1}", ref=ref, n=int(len(v)),
                                mean=round(float(v.mean()), 4), max_abs_dev=round(float(np.abs(dev).max()), 4),
                                mean_dev_px=round(float(dev.mean()) * pxu[img + "|" + ax], 2)))
    for case, info in hcen.items():
        out.append(dict(image=f"HL06_fig4_smooth_{case}", check="p(0)/p_Hertz", value=info["p_center_p_pHertz"]))
    for o in out:
        print("  check", o)
    return out


if __name__ == "__main__":
    sym, sm, hcen, xdevs, checks = main()
    for c in CAL_LOG:
        print(c)
    print("x dev", xdevs)
    print(json.dumps(hcen, ensure_ascii=False))
