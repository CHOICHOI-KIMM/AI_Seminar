# -*- coding: utf-8 -*-
"""
digitize_lit2010.py -- Graph-reading digitizer for three figures of
  Ventura/SKF 2010, "Micro-Geometry Effects on the Sliding Friction Transition in EHL"
  (MinerU JPG extraction):
    img_0017.jpg  Fig 7(a)  Deolalikar et al. pressure (blue) + film thickness (green)
    img_0018.jpg  Fig 7(b)  present 2010 model pressure (blue) + clearance (green)
    img_0019.jpg  Fig 8     contact load ratio (blue) + contact area ratio (green, 'o' markers)

Run:  python digitize_lit2010.py        ->  ../lit2010_oracle.json

Method
  1. Frame = longest non-white rows/cols. Axis calibration is a least-squares fit to
     TICK-MARK pixel positions paired with the printed labels (CAL below); the frame
     and every listed tick are re-detected and asserted at run time (+-1.5 / +-1 px).
     NOTE Fig 7(a): the left axis ticks (0,2,4,6 at rows 339/260/181/102) give 39.5 px
     per unit, so the frame top corresponds to p/P_H = 7.48, not 8.
  2. Colour masks (blue / green, tolerant to JPG blending) restricted to the frame
     interior (2 px margin) and excluding the Fig 8 legend box. Per pixel column the
     mask run that continues the previous column is taken: short runs -> run centre;
     near-vertical runs (> 6 px tall) -> several sub-pixel points so spikes are kept.
     Columns with no mask pixel (occlusion by the other curve / curve on the frame
     line) are gaps -> null. Both curves of a panel are then sampled on one shared x
     grid (every column near the bump, every 3rd/4th column elsewhere, plus the
     sub-pixel x of steep columns).
  3. Fig 8 markers: filled-disk score on the green mask (r = 4.5 px; marker ~55-60,
     plain line ~25), non-max suppression, centre refined to the enclosed bright
     ring interior.
  4. Cross-check: the agent read landmarks by eye from the original images (VISUAL
     'init'), then, where |auto-visual| > 3 % fs, re-read the pixel row against a
     pixel ruler on a 6x zoom ('v'); the auto trace was also overlaid on the images.
     Both readings are stored in the JSON crossCheck table.

All numbers are GRAPH READINGS (pixel resolution ~0.3-1 % of full scale), not
published tabulated values.
"""
import json, os
import numpy as np
from PIL import Image

HERE = os.path.dirname(os.path.abspath(__file__))
IMG_DIR = os.path.normpath(os.path.join(HERE, '..', '..', '..',
                                        '2010_SKF_Micro_Geometry_Effects_on_the_S'))
OUT_JSON = os.path.normpath(os.path.join(HERE, '..', 'lit2010_oracle.json'))

# ---------------------------------------------------------------- calibration
# (pixel, value) pairs measured from tick marks; frame limits for the run-time check.
CAL = {
    'img_0017.jpg': dict(
        frame=dict(top=43.5, bottom=339, left=46, right=406.5),
        x=[(46.0, -1.5), (106.0, -1.0), (406.5, 1.5)],                  # 120.17 px per x/b
        yL=[(339.0, 0.0), (260.0, 2.0), (181.0, 4.0), (102.0, 6.0)],    # 39.5 px/unit, top = 7.48
        yR=[(339.0, 0.0), (43.5, 1.0)],                                 # 0..1 = full frame (label 0.5 @ row 191)
        ticks_x=[106], ticks_yL=[102, 181, 260],
    ),
    'img_0018.jpg': dict(
        frame=dict(top=15, bottom=333.5, left=56.5, right=567),
        x=[(141.5, -1.0), (311.5, 0.0), (481.5, 1.0)],                  # 170 px per x/b; +-1.5 = frame
        yL=[(333.5, 0.0), (288.0, 1.0), (60.5, 6.0), (15.0, 7.0)],      # 45.5 px/unit
        yR=[(333.5, 0.0), (174.0, 0.5), (15.0, 1.0)],                   # 318.5 px/unit (ticks 94/174/254)
        ticks_x=[141.5, 226.5, 396.5, 481.5], ticks_yL=[60, 106, 151, 197, 242, 288],
    ),
    'img_0019.jpg': dict(
        frame=dict(top=33.5, bottom=391.5, left=56.5, right=583.5),
        x=[(56.5, 0.0), (232.0, 0.5), (408.0, 1.0), (583.5, 1.5)],      # 351.3 px per m/s
        yL=[(391.5, 0.0), (302.0, 1.0), (212.5, 2.0), (123.0, 3.0), (33.5, 4.0)],  # 89.5 px/unit
        yR=[(391.5, -0.2), (272.2, 0.0), (153.0, 0.2), (33.5, 0.4)],    # 596.7 px/unit
        ticks_x=[232, 408], ticks_yL=[123, 212.5, 302],
        legend=(345, 40, 552, 112),                                     # x0,y0,x1,y1 exclusion box
    ),
}
FULL_SCALE = {'7a': dict(pressure=7.5, film=1.0), '7b': dict(pressure=7.0, clearance=1.0),
              '8': dict(load=4.0, area=0.6)}

# Visual landmark readings by the agent (data units).
#   init : first read straight from the original image (before running the extraction)
#   v    : final visual value; where it differs from init it is a re-read of the pixel row
#          on a 6x zoom with a pixel ruler, converted with the same tick calibration
#   win  : compare against the auto window-maximum in [lo,hi] instead of the value at x
#   note : free text
#   cross: landmark is an x position (where the curve crosses level v); auto = first x with
#          curve >= v, compared in % of the x full scale
V = lambda x, v, init=None, win=None, note=None, cross=False: dict(x=x, v=v, init=v if init is None else init, win=win, note=note, cross=cross)
VISUAL = {
    ('7a', 'pressure'): [
        V(-1.48, 0.05, note='curve lies on the frame line here; not separable from it -> auto null'),
        V(-1.0, 0.20, init=None),
        V(-0.5, 0.84, init=0.48, note='init misread; ruler: blue centre row 306'),
        V(-0.085, 1.5, init=1.45, cross=True, note='init read a "local hump of 1.45 at x=-0.10" left of the spike; the mask shows no such feature: it is the 4 px thick base of the near-vertical spike flank (blue rows 270-297 at px 215-218, nothing below row 271 from px 219 on). Compared instead as the x/b where the left flank crosses p/P_H = 1.5: visual px 216 -> x/b = -0.085; diff in % of the x full scale (3.0)'),
        V(0.0, 7.10, win=(-0.05, 0.05), note='spike apex, row 59'),
        V(0.5, 0.76, init=0.90, note='ruler: row 309'),
        V(1.0, 0.05, note='curve merges with the frame line -> auto null expected'),
    ],
    ('7a', 'film'): [
        V(-1.48, 0.51, init=0.50, note='green reaches px 48 (row ~186); x=-1.5 itself is on the frame'),
        V(-1.0, 0.162, init=0.19, note='ruler: row 291'),
        V(-0.5, 0.125, init=0.155, note='ruler: row 302'),
        V(0.5, 0.115, init=0.15, note='ruler: row 305'),
        V(1.0, 0.112, init=0.19, note='ruler: row 306'),
        V(1.46, 0.475, init=0.50, note='init estimated from the small image; green mask rows 195-199 at px 402-404 -> 0.475 (green reaches px 403; last grid column px 402)'),
    ],
    ('7b', 'pressure'): [
        V(-0.97, 0.505, init=0.47, note='curves start at px 145 (x/b ~ -0.98); blue mask rows 309-311 -> 0.505'),
        V(-0.5, 0.66, init=0.63, note='ruler: row 303.5'),
        V(0.08, 5.40, win=(0.0, 0.2), note='peak, row ~88'),
        V(0.5, 0.69, init=0.78, note='ruler: row 302'),
        V(0.98, 0.505, init=0.49, note='ruler: row 310.5'),
    ],
    ('7b', 'clearance'): [
        V(-0.97, 0.033, init=0.049, note='init read row ~318 from the small image; green mask rows 322-324 at px 145-156 -> 0.033'),
        V(-0.5, 0.046, init=0.096, note='init misread the blue curve as green; ruler: green row 319'),
        V(-0.094, 0.196, init=0.20, win=(-0.2, -0.05), note='left cusp apex at px 295.5 (x/b=-0.094), row 271'),
        V(0.0, 0.0, note='zero clearance over the bump'),
        V(0.08, 0.35, win=(0.0, 0.2), note='right cusp, row ~222'),
        V(0.5, 0.052, init=0.07, note='ruler: row 317'),
        V(0.98, 0.035, init=0.05, note='ruler: row 322.5'),
    ],
    ('8', 'load'): [
        V(0.009, 1.47, init=1.95, note='init read the dark marker cluster as the line start; blue is hidden behind the green markers above ~1.47 (rows 212-257); first visible blue at px 59, row 260'),
        V(0.07, 0.92), V(0.12, 0.95), V(0.5, 0.83), V(1.0, 0.62),
        V(1.487, 0.40, note='last unoccluded column (px 581); frame line at 1.5'),
    ],
    ('8', 'areaMarker'): [   # (speed, area%) of the ring centres, matched to nearest detected marker
        V(0.011, 0.096, init=0.0975, note='init speed 0.01; ring centre ~(60.5,215)'),
        V(0.011, 0.078, init=0.079, note='init speed 0.02; ring centre ~(60.5,225.5); two lowest rings are ~1 px apart in x, unresolvable (+-0.003 m/s)'),
        V(0.021, 0.031, init=0.030, note='init speed 0.035; ring centre ~(64,253.5)'),
        V(0.064, 0.0146, init=0.014, note='init speed 0.07'),
        V(0.18, 0.007), V(0.44, 0.005), V(0.89, 0.005),
    ],
}


def linmap(pairs):
    p = np.array([a for a, _ in pairs], float); v = np.array([b for _, b in pairs], float)
    m, c = np.linalg.lstsq(np.vstack([p, np.ones_like(p)]).T, v, rcond=None)[0]
    return (lambda q: m * np.asarray(q, float) + c), m, c


def load(name):
    return np.asarray(Image.open(os.path.join(IMG_DIR, name)).convert('RGB')).astype(int)


def runs(idx):
    out = []
    for i in idx:
        if out and i == out[-1][1] + 1:
            out[-1][1] = i
        else:
            out.append([i, i])
    return out


def verify_frame_and_ticks(name, im, cal):
    H, W, _ = im.shape
    nw = im.sum(axis=2) < 500
    rows = nw.sum(axis=1); cols = nw.sum(axis=0)
    fr = runs([i for i in range(H) if rows[i] > 0.6 * W]); fc = runs([i for i in range(W) if cols[i] > 0.6 * H])
    det = dict(top=(fr[0][0] + fr[0][1]) / 2, bottom=(fr[-1][0] + fr[-1][1]) / 2,
               left=(fc[0][0] + fc[0][1]) / 2, right=(fc[-1][0] + fc[-1][1]) / 2)
    for k, v in det.items():
        assert abs(v - cal['frame'][k]) <= 1.5, f'{name}: frame {k} detected {v} vs cal {cal["frame"][k]}'
    band = nw[int(det['bottom']) - 8:int(det['bottom']) - 1, :].sum(axis=0)
    for tx in cal['ticks_x']:
        assert band[int(round(tx)) - 1:int(round(tx)) + 2].max() >= 4, f'{name}: x tick at {tx} not found'
    band = nw[:, int(det['left']) + 1:int(det['left']) + 9].sum(axis=1)
    for ty in cal['ticks_yL']:
        assert band[int(round(ty)) - 1:int(round(ty)) + 2].max() >= 4, f'{name}: y tick at {ty} not found'
    return det


def colour_masks(im):
    r, g, b = im[..., 0], im[..., 1], im[..., 2]
    blue = (b > r + 35) & (b > g + 22) & (b > 80)
    green = (g > r + 28) & (g > b + 10) & (g > 80)
    return blue, green


def masks(im, frame, legend=None):
    blue, green = colour_masks(im)
    H, W = blue.shape
    inside = np.zeros((H, W), bool)
    # 1 px margin top/bottom (frame lines are dark, never blue/green; the zero-clearance
    # green of Fig 7 runs 1-2 px above the bottom line), 2 px left/right (Fig 7b/8 axis
    # lines are themselves blue/green).
    y0, y1 = int(np.ceil(frame['top'])) + 1, int(np.floor(frame['bottom'])) - 1
    x0, x1 = int(np.ceil(frame['left'])) + 2, int(np.floor(frame['right'])) - 2
    inside[y0:y1 + 1, x0:x1 + 1] = True
    if legend is not None:
        lx0, ly0, lx1, ly1 = legend
        inside[ly0:ly1 + 1, lx0:lx1 + 1] = False
    return blue & inside, green & inside, (x0, x1, y0, y1)


def trace(mask, xr, steep_px=6, merge_gap=6, max_sub=8, new_tol=4):
    """Column-wise trace. Returns list of (px_x, px_y, run_extent_px) with x strictly
    increasing (sub-pixel x inside columns whose mask run is taller than steep_px, i.e.
    near-vertical segments); run_extent_px = 0 for ordinary columns. None marks a
    column without mask pixels (gap).
    (Skeletonising the strokes first was tried and rejected: it erodes the spike apex,
    the clearance cusps and the curve ends by ~half a stroke width.)"""
    x0, x1, y0, y1 = xr

    def column_runs(x):
        ys = np.where(mask[:, x])[0]
        if len(ys) == 0:
            return []
        rr = runs(list(ys)); merged = [rr[0]]
        for a, b in rr[1:]:
            if a - merged[-1][1] <= merge_gap:
                merged[-1][1] = b
            else:
                merged.append([a, b])
        return merged

    pts = []; prev_y = None; prev_run = None; direction = 0   # -1 = travelling up (rows decreasing)
    for x in range(x0, x1 + 1):
        merged = column_runs(x)
        if not merged:
            pts.append(None); prev_y = None; prev_run = None; direction = 0; continue
        if prev_y is None:
            run = max(merged, key=lambda ab: ab[1] - ab[0])
        else:
            run = min(merged, key=lambda ab: min(abs(ab[0] - prev_y), abs(ab[1] - prev_y), abs((ab[0] + ab[1]) / 2 - prev_y)))
        a, b = run
        if b - a <= steep_px:
            y = (a + b) / 2
            pts.append((float(x), y, 0))
            if prev_y is not None:
                direction = int(np.sign(y - prev_y))
            prev_y = y
        else:
            # Steep column (near-vertical stroke several px thick): the part of the run that
            # overlaps the previous column's run is stroke thickness; the part beyond it
            # (by more than new_tol, the stroke-edge jitter) is where the curve travels in
            # this column. The curve continues from the previous y (clipped into this run)
            # to the new extreme. No significant new territory -> follow the stroke's
            # leading edge in the current direction of travel (this is what carries the
            # trace up to a spike apex), else hold. Both directions new (a whole narrow
            # spike inside one column) -> previous y -> nearer extreme -> farther extreme.
            if prev_run is None:
                # re-entry after a gap / first column: pick the direction by looking at the
                # next column's run (the path must end at the end nearest to it)
                nxt = column_runs(x + 1) if x + 1 <= x1 else []
                if nxt:
                    nr = min(nxt, key=lambda ab: min(abs(ab[0] - a), abs(ab[1] - b), abs(ab[0] - b), abs(ab[1] - a)))
                    yc = (nr[0] + nr[1]) / 2
                    path = [b, a] if abs(a - yc) < abs(b - yc) else [a, b]
                elif prev_y is not None:
                    path = [a, b] if abs(a - prev_y) <= abs(b - prev_y) else [b, a]
                else:
                    path = [b, a]
            else:
                pa, pb = prev_run
                start = min(max(prev_y, a), b)
                new_up, new_down = a < pa - new_tol, b > pb + new_tol
                if new_up and not new_down:
                    path = [start, a]
                elif new_down and not new_up:
                    path = [start, b]
                elif new_up and new_down:
                    path = [start, a, b] if abs(a - start) <= abs(b - start) else [start, b, a]
                elif direction < 0 and a < pa:
                    path = [float(a)]          # leading (top) edge while travelling up
                elif direction > 0 and b > pb:
                    path = [float(b)]          # leading (bottom) edge while travelling down
                else:
                    path = [start]
            if len(path) == 1:
                pts.append((float(x), float(path[0]), int(b - a)))
                if prev_y is not None and path[0] != prev_y:
                    direction = int(np.sign(path[0] - prev_y))
                prev_y = path[0]
            else:
                direction = int(np.sign(path[-1] - path[0]))
                total = sum(abs(path[i + 1] - path[i]) for i in range(len(path) - 1))
                n = min(max_sub, int(total / 3) + 2)
                # distribute n sub-points along the path, x spread across the column
                dist = [0.0]
                for i in range(len(path) - 1):
                    dist.append(dist[-1] + abs(path[i + 1] - path[i]))
                for k in range(n):
                    s = total * k / (n - 1)
                    j = max(i for i in range(len(path) - 1) if dist[i] <= s + 1e-9)
                    seg = dist[j + 1] - dist[j]
                    yv = path[j] + (path[j + 1] - path[j]) * ((s - dist[j]) / seg if seg > 0 else 0)
                    pts.append((x - 0.45 + 0.9 * k / (n - 1), float(yv), int(b - a)))
                prev_y = path[-1]
        prev_run = (a, b)
    return pts


def to_data(pts, fx, fy):
    return [None if p is None else (float(fx(p[0])), float(fy(p[1])), p[2]) for p in pts]


def sample(curve, xs, max_gap):
    """linear interpolation along the traced curve; None where the grid x falls in a
    gap (traced neighbours further apart than max_gap) or outside the trace."""
    valid = [p for p in curve if p is not None]
    cx = np.array([p[0] for p in valid]); cy = np.array([p[1] for p in valid])
    res = []
    for x in xs:
        i = int(np.searchsorted(cx, x))
        if i < len(cx) and abs(cx[i] - x) < 1e-9:
            res.append(float(cy[i])); continue
        if i == 0 or i == len(cx):
            res.append(None); continue
        xa, xb = cx[i - 1], cx[i]
        if xb - xa > max_gap:
            res.append(None); continue
        t = (x - xa) / (xb - xa)
        res.append(float(cy[i - 1] + t * (cy[i] - cy[i - 1])))
    return res


def build_grid(xr, fx, dense_lo, dense_hi, sparse_step, traces, spike_px=15):
    """shared x grid: every column inside [dense_lo, dense_hi], every sparse_step-th
    column elsewhere, plus the sub-pixel x of genuinely steep columns (mask run taller
    than spike_px, i.e. the pressure spike flanks / clearance cusps)."""
    x0, x1, _, _ = xr
    xs = []
    for i, c in enumerate(range(x0, x1 + 1)):
        xd = float(fx(c))
        if dense_lo <= xd <= dense_hi or i % sparse_step == 0:
            xs.append(xd)
    for tr in traces:
        xs += [p[0] for p in tr if p is not None and p[2] >= spike_px]
    return sorted(set(round(v, 5) for v in xs))


def find_markers(im, frame, legend, radius=4.5, min_score=38, nms=5):
    """'o' marker detection (Fig 8): filled-disk score on the green mask, local maxima,
    centre refined to the centroid of bright pixels enclosed by green on four sides
    (ring interior). Mask clipped to the frame only, because the lowest-speed markers
    touch the axis. Returns [(px_x, px_y, score, nInteriorPx)] sorted by x."""
    from numpy.lib.stride_tricks import sliding_window_view
    r, g, b = im[..., 0], im[..., 1], im[..., 2]
    _, green = colour_masks(im)
    green = green.copy()
    green[:int(frame['top']) + 1, :] = False; green[int(frame['bottom']):, :] = False
    green[:, :int(frame['left']) + 1] = False; green[:, int(frame['right']):] = False
    if legend is not None:
        lx0, ly0, lx1, ly1 = legend
        green[ly0:ly1 + 1, lx0:lx1 + 1] = False
    n = int(np.ceil(radius)) + 1
    yy, xx = np.mgrid[-n:n + 1, -n:n + 1]
    disk = np.hypot(yy, xx) <= radius
    score = (sliding_window_view(np.pad(green, n), disk.shape) * disk).sum(axis=(2, 3))
    cand = sorted(np.argwhere(score >= min_score).tolist(), key=lambda p: -score[p[0], p[1]])
    keep = []
    for cy, cx in cand:
        if all((cy - ky) ** 2 + (cx - kx) ** 2 > nms ** 2 for ky, kx in keep):
            keep.append((cy, cx))
    bright = (g > 170) & (r > 140) & (b > 140)
    out = []
    for cy, cx in keep:
        pts = []
        for y in range(cy - 5, cy + 6):
            for x in range(cx - 5, cx + 6):
                if bright[y, x] and green[y, x + 1:x + 6].any() and green[y, x - 5:x].any() \
                        and green[y + 1:y + 6, x].any() and green[y - 5:y, x].any():
                    pts.append((x, y))
        if pts:
            c = (float(np.mean([p[0] for p in pts])), float(np.mean([p[1] for p in pts])), int(score[cy, cx]), len(pts))
        else:
            c = (float(cx), float(cy), int(score[cy, cx]), 0)
        if all(np.hypot(c[0] - o[0], c[1] - o[1]) > 3 for o in out):   # dedupe refined centres
            out.append(c)
    return sorted(out)


def interp_at(xs, ys, x):
    xa = np.array([v for v, w in zip(xs, ys) if w is not None]); ya = np.array([w for w in ys if w is not None])
    if len(xa) == 0 or x < xa[0] or x > xa[-1]:
        return None
    i = int(np.searchsorted(xa, x))
    if i > 0 and i < len(xa) and xa[i] - xa[i - 1] > 0.05 and abs(xa[i] - x) > 1e-9:
        return None   # inside a gap
    return float(np.interp(x, xa, ya))


def window_max(xs, ys, lo, hi):
    vals = [(x, y) for x, y in zip(xs, ys) if y is not None and lo <= x <= hi]
    return max(vals, key=lambda t: t[1]) if vals else (None, None)


def digitize_fig7(name, dense):
    cal = CAL[name]; im = load(name)
    frame = verify_frame_and_ticks(name, im, cal)
    fx, mx, cx = linmap(cal['x']); fyL, mL, cL = linmap(cal['yL']); fyR, mR, cR = linmap(cal['yR'])
    blue, green, xr = masks(im, frame)
    tb = to_data(trace(blue, xr), fx, fyL)
    tg = to_data(trace(green, xr), fx, fyR)
    # stroke-edge (top-most blue pixel) value of the pressure spike, as an upper bound
    ys, xs_ = np.where(blue)
    edge_peak = float(fyL(ys.min()))
    xs = build_grid(xr, fx, dense[0], dense[1], 3, [tb, tg])
    gap = 2.5 * mx
    p = sample(tb, xs, gap); h = sample(tg, xs, gap)
    idx = [i for i in range(len(xs)) if p[i] is not None or h[i] is not None]
    xs = [xs[i] for i in idx]; p = [p[i] for i in idx]; h = [h[i] for i in idx]
    mapping = (f'x/b = {mx:.5f}*px + ({cx:.4f}); left y = {mL:.5f}*py + ({cL:.4f}); '
               f'right y = {mR:.6f}*py + ({cR:.4f}); frame px top/bottom/left/right = '
               f'{frame["top"]}/{frame["bottom"]}/{frame["left"]}/{frame["right"]}; interior cols {xr[0]}..{xr[1]}')
    return dict(xOverB=[round(v, 4) for v in xs],
                pOverPh=[None if v is None else round(v, 3) for v in p],
                hDimless=[None if v is None else round(v, 4) for v in h], mapping=mapping, edgePeak=edge_peak)


def digitize_fig8():
    name = 'img_0019.jpg'; cal = CAL[name]; im = load(name)
    frame = verify_frame_and_ticks(name, im, cal)
    fx, mx, cx = linmap(cal['x']); fyL, mL, cL = linmap(cal['yL']); fyR, mR, cR = linmap(cal['yR'])
    blue, green, xr = masks(im, frame, cal['legend'])
    tb = to_data(trace(blue, xr, merge_gap=3), fx, fyL)
    tg = to_data(trace(green, xr, merge_gap=8), fx, fyR)
    xs = build_grid(xr, fx, 0.0, 0.25, 4, [tb, tg])
    gap = 2.5 * mx
    ld = sample(tb, xs, gap); ar = sample(tg, xs, gap)
    idx = [i for i in range(len(xs)) if ld[i] is not None or ar[i] is not None]
    xs = [xs[i] for i in idx]; ld = [ld[i] for i in idx]; ar = [ar[i] for i in idx]
    mk = find_markers(im, frame, cal['legend'])
    mapping = (f'speed = {mx:.6f}*px + ({cx:.4f}); load% = {mL:.5f}*py + ({cL:.4f}); '
               f'area% = {mR:.6f}*py + ({cR:.5f}); frame px top/bottom/left/right = '
               f'{frame["top"]}/{frame["bottom"]}/{frame["left"]}/{frame["right"]}; interior cols {xr[0]}..{xr[1]}; '
               f'legend box excluded {cal["legend"]}')
    # extrapolate the load line to the frame (1.5 m/s) from the last 8 columns
    tail = [(x, y) for x, y in zip(xs, ld) if y is not None and x > 1.45]
    m, c = np.polyfit([t[0] for t in tail], [t[1] for t in tail], 1)
    return dict(speeds=[round(v, 4) for v in xs],
                loadRatioPct=[None if v is None else round(v, 3) for v in ld],
                areaRatioPct=[None if v is None else round(v, 4) for v in ar],
                areaMarkers=dict(speeds=[round(float(fx(a)), 4) for a, b, s, k in mk],
                                 areaRatioPct=[round(float(fyR(b)), 4) for a, b, s, k in mk],
                                 pixelCentres=[[round(a, 1), round(b, 1)] for a, b, s, k in mk]),
                load_at_1p5_extrap=float(m * 1.5 + c), mapping=mapping)


def bump_half_width(f7b):
    """zero-clearance region = contiguous run of grid points around x/b=0 whose clearance
    is < 0.02 or null (null = green lying on the frame bottom line, i.e. also ~0)."""
    xs = np.array(f7b['xOverB']); h = np.array([np.nan if v is None else v for v in f7b['hDimless']])
    win = (xs > -0.4) & (xs < 0.4)
    z = win & ((h < 0.02) | np.isnan(h))
    i0 = int(np.argmin(np.abs(xs)))
    assert z[i0], 'no zero clearance at x/b=0'
    a = i0
    while a - 1 >= 0 and z[a - 1]: a -= 1
    b = i0
    while b + 1 < len(xs) and z[b + 1]: b += 1
    zero = np.zeros_like(z); zero[a:b + 1] = True
    xz = xs[zero]
    left = win & (xs < xz.min()); right = win & (xs > xz.max())
    xl = xs[left][np.nanargmax(h[left])]; xr_ = xs[right][np.nanargmax(h[right])]
    return dict(zeroSpan=(float(xz.min()), float(xz.max())), zeroHalf=float((xz.max() - xz.min()) / 2),
                cuspLeft=float(xl), cuspRight=float(xr_), cuspHalf=float((xr_ - xl) / 2), minH=float(np.nanmin(h[win])))


def main():
    f7a = digitize_fig7('img_0017.jpg', (-0.35, 0.35))
    f7b = digitize_fig7('img_0018.jpg', (-0.30, 0.30))
    f8 = digitize_fig8()

    cross = []
    def cc(fig, curve, xs, ys, fs, xkey, xfs):
        for L in VISUAL[(fig, curve)]:
            if L['cross']:
                # x where the curve first reaches level v (left flank crossing)
                a = next((x for x, y in zip(xs, ys) if y is not None and y >= L['v'] and x > L['x'] - 0.1), None)
                row = {'fig': fig, 'curve': curve, 'level': L['v'], 'auto': None if a is None else round(a, 4),
                       'visual': L['x'], 'diffPctFullScale': None if a is None else round(abs(a - L['x']) / xfs * 100, 2),
                       'visualInitial': L['init'], 'compare': f'{xkey} at which curve crosses level (x full scale {xfs})',
                       'note': L['note']}
                cross.append(row); continue
            if L['win']:
                ax, a = window_max(xs, ys, *L['win'])
            else:
                ax, a = L['x'], interp_at(xs, ys, L['x'])
            row = {'fig': fig, 'curve': curve, xkey: L['x'], 'auto': None if a is None else round(a, 3),
                   'visual': L['v'], 'diffPctFullScale': None if a is None else round(abs(a - L['v']) / fs * 100, 2),
                   'visualInitial': L['init'],
                   'diffInitialPctFullScale': None if (a is None or L['init'] is None) else round(abs(a - L['init']) / fs * 100, 2)}
            if L['win']:
                row['autoX'] = ax; row['window'] = list(L['win'])
            if L['note']:
                row['note'] = L['note']
            cross.append(row)
    cc('7a', 'pressure', f7a['xOverB'], f7a['pOverPh'], FULL_SCALE['7a']['pressure'], 'xOverB', 3.0)
    cc('7a', 'film', f7a['xOverB'], f7a['hDimless'], FULL_SCALE['7a']['film'], 'xOverB', 3.0)
    cc('7b', 'pressure', f7b['xOverB'], f7b['pOverPh'], FULL_SCALE['7b']['pressure'], 'xOverB', 3.0)
    cc('7b', 'clearance', f7b['xOverB'], f7b['hDimless'], FULL_SCALE['7b']['clearance'], 'xOverB', 3.0)
    cc('8', 'load', f8['speeds'], f8['loadRatioPct'], FULL_SCALE['8']['load'], 'speed', 1.5)
    mks = f8['areaMarkers']; used = set()
    for L in VISUAL[('8', 'areaMarker')]:
        cand = [(np.hypot((s - L['x']) / 1.5, (a - L['v']) / 0.6), j) for j, (s, a) in enumerate(zip(mks['speeds'], mks['areaRatioPct'])) if j not in used]
        _, j = min(cand); used.add(j)
        cross.append({'fig': '8', 'curve': 'areaMarker', 'speed': L['x'], 'autoSpeed': mks['speeds'][j],
                      'auto': mks['areaRatioPct'][j], 'visual': L['v'],
                      'diffPctFullScale': round(abs(mks['areaRatioPct'][j] - L['v']) / FULL_SCALE['8']['area'] * 100, 2),
                      'visualInitial': L['init'], 'note': L['note']})

    bw = bump_half_width(f7b)
    pk7a = window_max(f7a['xOverB'], f7a['pOverPh'], -0.05, 0.05)
    flank7a = next((x for x, y in zip(f7a['xOverB'], f7a['pOverPh']) if y is not None and y >= 1.5 and x > -0.2), None)
    pk7b = window_max(f7b['xOverB'], f7b['pOverPh'], 0.0, 0.2)
    pk7bh = window_max(f7b['xOverB'], f7b['hDimless'], 0.0, 0.2)
    cusp7b = window_max(f7b['xOverB'], f7b['hDimless'], -0.2, -0.05)
    ld = f8['loadRatioPct']; sp = f8['speeds']
    first = next(i for i in range(len(ld)) if ld[i] is not None)
    last = max(i for i in range(len(ld)) if ld[i] is not None)

    out = {
        '_provenance': (
            'Graph readings digitized from MinerU JPG extractions of the 2010 SKF paper "Micro-Geometry '
            'Effects on the Sliding Friction Transition in EHL": img_0017.jpg = Fig 7(a) Deolalikar et al. '
            '(pure rolling, p_h = 0.63 GPa), img_0018.jpg = Fig 7(b) present 2010 model, img_0019.jpg = Fig 8 '
            '(pure sliding, stationary bump, p_h = 1 GPa). Values come from colour-mask pixel tracing with '
            'tick-mark axis calibration (tools/digitize_lit2010.py), cross-checked against visual landmark '
            'readings and an overlay of the trace on the images. They are NOT tabulated values from the paper: '
            'pixel resolution alone is 0.3-1 % of full scale, JPG blending and curve overlap add more near the '
            'bump, and the Fig 7(a) left axis top is 7.48 (tick-based), not 8. null marks columns where a curve '
            'is hidden behind the other curve, lies on the frame line, or is absent.'),
        'method': {
            'auto': ('Frame = longest non-white rows/cols; axis maps fitted to tick-mark pixels (re-detected and '
                     'asserted at run time). Blue/green RGB masks inside the frame (2 px margin; Fig 8 legend box '
                     'excluded). Per pixel column the mask run continuing the previous column is used: short runs -> '
                     'centre; near-vertical runs -> sub-pixel points (spikes kept). Both curves sampled on one shared '
                     'x grid (every column near the bump, every 3rd/4th column elsewhere, plus steep sub-pixel x); '
                     'gaps -> null. Fig 8 markers: filled-disk score (r=4.5 px) on the green mask, centre refined to '
                     'the enclosed ring interior.'),
            'crossCheck': ('Agent read landmarks by eye from the original images before the extraction '
                           '(visualInitial). Where |auto - visualInitial| > 3 % fs the pixel row was re-read on a 6x '
                           'zoom with a pixel ruler (or from the colour-mask dump) and converted with the same tick '
                           'calibration (visual); the auto trace was also overlaid on the images at 2-4x and '
                           'inspected, including the spike flanks and cusps. Every >3 % case was traced to an '
                           'eyeball misread (curve mis-identified, wrong x, a non-existent "hump" that is the thick '
                           'base of the spike flank, or a line hidden behind markers), not to a calibration or mask '
                           'error. Peaks/cusps compare window maxima (autoX = x of max); the 7a flank row compares '
                           'the x/b at which the pressure crosses 1.5.'),
            'calibration': {'fig7a': f7a['mapping'], 'fig7b': f7b['mapping'], 'fig8': f8['mapping']},
            'bumpHalfWidth': ('Fig 7(b) clearance trace: half the x/b span where h*Rx/b^2 < 0.02 around x/b=0 '
                              f'(zero-clearance contact over the bump), span {bw["zeroSpan"][0]:.3f}..{bw["zeroSpan"][1]:.3f} '
                              f'-> {bw["zeroHalf"]:.3f} (primary). Alternative: half the distance between the two clearance '
                              f'cusps (local maxima at {bw["cuspLeft"]:.3f} and {bw["cuspRight"]:.3f}) = {bw["cuspHalf"]:.3f}. '
                              'Resolution 1 px = 0.006 x/b.'),
        },
        'uncertainty': {
            'fig7': ('±1.5% full scale on the flanks (pixel 0.3-0.4% fs; up to ~3% where blue and green overlap, '
                     '|x/b|<0.15, with nulls where one curve hides the other). Near-vertical stroke segments are '
                     'followed along the stroke edge, so spike/cusp peak values are upper bounds by <= half a '
                     'stroke width (~0.05 p/P_H in 7a, ~0.04 in 7b, ~0.005 in h) and the x/b of a vertical flank '
                     'is uncertain by ±0.012 (±1.5 px); elsewhere x/b ±0.006 (1 px).'),
            'fig8': ('±1% full scale for the load line (±0.04 %-points); its start above ~1.47 % is hidden behind '
                     'the markers. Markers ±0.005 %-points area ratio (0.8% fs), speeds ±0.005 m/s; the two '
                     'lowest-speed rings (~0.007-0.012 m/s) are 1-2 px apart in x and not resolvable.'),
        },
        'fig7': {
            'deolalikar': {k: f7a[k] for k in ('xOverB', 'pOverPh', 'hDimless')},
            'model2010': {k: f7b[k] for k in ('xOverB', 'pOverPh', 'hDimless')},
            'bumpHalfWidthOverB': round(bw['zeroHalf'], 3),
            'bumpHalfWidthOverB_cuspBased': round(bw['cuspHalf'], 3),
            'landmarks': {
                'deolalikarPeakPOverPh': round(pk7a[1], 2), 'deolalikarPeakX': pk7a[0],
                'deolalikarPeakStrokeEdge': round(f7a['edgePeak'], 2),
                'model2010PeakStrokeEdge': round(f7b['edgePeak'], 2),
                'peakNote': 'Peak = maximum of the centre-line trace; StrokeEdge = top-most blue pixel of the spike (upper bound, +~half a stroke width)',
                'deolalikarLeftFlankXAtPOverPh1p5': flank7a,
                'deolalikarMinHDimless': round(float(min(h for h in f7a['hDimless'] if h is not None)), 3),
                'model2010PeakPOverPh': round(pk7b[1], 2), 'model2010PeakX': pk7b[0],
                'model2010MinHDimless': round(bw['minH'], 3),
                'model2010ClearanceCuspLeft': round(cusp7b[1], 3), 'model2010ClearanceCuspLeftX': cusp7b[0],
                'model2010ClearancePeakRight': round(pk7bh[1], 3), 'model2010ClearancePeakRightX': pk7bh[0],
            },
        },
        'fig8': {
            'speeds': f8['speeds'], 'loadRatioPct': f8['loadRatioPct'], 'areaRatioPct': f8['areaRatioPct'],
            'areaMarkers': {'speeds': mks['speeds'], 'areaRatioPct': mks['areaRatioPct']},
            'areaMarkerPixelCentres': mks['pixelCentres'],
            'landmarks': {
                'loadRatioAtLowestSpeed': ld[first], 'lowestSpeed': sp[first],
                'loadRatioAtLowestSpeedNote': 'first column where the blue line is visible; above ~1.47 % it is hidden behind the green markers (rows 212-257), so the true start value is unreadable',
                'loadRatioAt1p5': round(f8['load_at_1p5_extrap'], 3),
                'loadRatioAt1p5Note': f'linear extrapolation of the last 8 columns to 1.5 m/s; last unoccluded column {sp[last]} m/s = {ld[last]} %',
                'loadRatioAtLastColumn': ld[last], 'lastColumnSpeed': sp[last],
            },
        },
        'crossCheck': cross,
    }
    with open(OUT_JSON, 'w', encoding='utf-8') as fh:
        json.dump(out, fh, ensure_ascii=False, indent=1)

    print('wrote', OUT_JSON)
    for k, v in out['method']['calibration'].items():
        print(k, ':', v)
    print('points 7a/7b/8:', len(f7a['xOverB']), len(f7b['xOverB']), len(f8['speeds']))
    print('nulls 7a p/h:', f7a['pOverPh'].count(None), f7a['hDimless'].count(None),
          ' 7b p/h:', f7b['pOverPh'].count(None), f7b['hDimless'].count(None),
          ' 8 load/area:', f8['loadRatioPct'].count(None), f8['areaRatioPct'].count(None))
    print('bump:', bw)
    print('landmarks fig7:', out['fig7']['landmarks'])
    print('landmarks fig8:', out['fig8']['landmarks'])
    print('markers:', mks)
    print('%-4s %-10s %-7s %-8s %-8s %-8s %-8s %s' % ('fig', 'curve', 'x', 'auto', 'visual', 'diff%', 'init', 'diffInit%'))
    for r in cross:
        print('%-4s %-10s %-7s %-8s %-8s %-8s %-8s %s' % (r['fig'], r['curve'], r.get('xOverB', r.get('speed', 'lvl%s' % r.get('level'))), r['auto'],
                                                       r['visual'], r['diffPctFullScale'], r['visualInitial'], r.get('diffInitialPctFullScale', '')))


if __name__ == '__main__':
    main()
