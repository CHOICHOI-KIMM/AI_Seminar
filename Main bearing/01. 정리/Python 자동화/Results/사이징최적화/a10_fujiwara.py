# -*- coding: utf-8 -*-
"""§10-12.6 무인 수행 — Fujiwara 수정 로그 프로파일 적용

  S0  교정 게이트   USERSPECIFIED 로 DIN Lundberg 를 재현할 수 있는가
  S1  대칭 격자     K1 x K2 x zm  (6 x 5 x 6 = 180 / 설계)
  S2  비대칭 확장   좌(하중측) 깊게 · 우 얕게
  S3  국소 정밀화   좌표하강 (파라미터별 ± 반간격)
  S4  방향 확인     극한 LC 15건에서 치우침 방향이 항상 같은가
  S5  정리          CSV · 요약

  python a10_fujiwara.py            전체
  python a10_fujiwara.py s0 s1      일부 단계만
"""
import csv
import json
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import a10_profile_lib as L      # noqa: E402

OUTD = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "fujiwara")
RANKS = (1, 130, 210)
NPTS = 61

# A 를 "실제 롤러하중" 으로 정의했으므로 문헌 범위(1.0~2.8)보다 넓게 연다.
# S0 에서 유망했던 Johns-Gohar x4 가 K1 ~ 4 에 해당한다.
K1S = (1.0, 2.0, 3.0, 4.0, 6.0, 8.0)
K2S = (0.4, 0.55, 0.7, 0.85, 1.0)
ZMS = (40e-6, 80e-6, 150e-6, 250e-6, 400e-6, 600e-6)
RATIOS = (0.15, 0.3, 0.5, 0.7, 1.0)      # zmR / zmL
SCALES = (1.0, 1.3, 1.6, 2.0)            # zmL 증폭
LCS = ("Mx_max", "Mx_min", "My_max", "My_min", "Mz_max", "Mz_min",
       "Myz_max", "Myz_min", "Fx_max", "Fx_min", "Fy_max", "Fy_min",
       "Fz_max", "Fz_min", "Fyz_max", "Fyz_min")

COLS = ["rank", "stage", "tag", "K1L", "K2L", "zmL_um", "K1R", "K2R",
        "zmR_um", "sigma_MPa", "edge_L_MPa", "margin_L_mm", "y_star_mm",
        "y_star_pct", "edge_R_MPa", "margin_R_mm", "contact_mm", "score",
        "feasible"]


def log(m):
    print(m, flush=True)


def score_of(m, b):
    """기준 대비 개선도 합. σ 위반이면 None."""
    if m["y_star_mm"] is None or m["sigma_MPa"] > b["sigma_MPa"] + 1e-9:
        return None
    half = b["L_we_mm"] / 2.0
    r1 = (b["edge_L_MPa"] - m["edge_L_MPa"]) / max(b["edge_L_MPa"], 1.0)
    r2 = ((m["margin_L_mm"] - b["margin_L_mm"])
          / max(half - b["margin_L_mm"], 1.0))
    r3 = ((abs(b["y_star_mm"]) - abs(m["y_star_mm"]))
          / max(abs(b["y_star_mm"]), 1.0))
    return round(r1 + r2 + r3, 5)


def row_of(rk, stage, tag, p, m, b):
    s = score_of(m, b)
    um = lambda v: None if v is None else round(v * 1e6, 1)   # noqa: E731
    return dict(rank=rk, stage=stage, tag=tag,
                K1L=p[0], K2L=p[1], zmL_um=um(p[2]),
                K1R=p[3], K2R=p[4], zmR_um=um(p[5]),
                sigma_MPa=m["sigma_MPa"], edge_L_MPa=m["edge_L_MPa"],
                margin_L_mm=m["margin_L_mm"], y_star_mm=m["y_star_mm"],
                y_star_pct=m["y_star_pct"], edge_R_MPa=m["edge_R_MPa"],
                margin_R_mm=m["margin_R_mm"], contact_mm=m["contact_mm"],
                score=s, feasible=int(s is not None))


FAIL = dict(L_we_mm=None, sigma_MPa=9e9, edge_row_MPa=None, n_contact=0,
            P_max_N=None, tilt_mrad=None, y_star_mm=None, y_star_pct=None,
            edge_L_MPa=9e9, edge_R_MPa=9e9, margin_L_mm=0.0, margin_R_mm=0.0,
            contact_mm=0.0)


def run_p(rig, Lwe, Q, p, tag):
    """한 조합을 해석한다. 프로파일이 부적합해 해석이 깨지면 실패행."""
    try:
        f = L.profile_fn(Lwe, p[0], p[1], p[2],
                         K1R=p[3], K2R=p[4], zmR=p[5], Q=Q)
        rig.set_user(f, NPTS)
        return rig.solve(tag)[0]
    except Exception as e:
        log(f"    [실패] {tag} {str(e).splitlines()[0][:56]}")
        return dict(FAIL)


# ══════════════════════════════════════════════════════════════════
def main(stages):
    global RANKS
    os.makedirs(OUTD, exist_ok=True)
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    rig = L.Rig()
    rig.load_case()
    t0 = time.perf_counter()
    rows, base, best = [], {}, {}

    # ── 기준값 ────────────────────────────────────────────────
    log("=" * 74)
    log("기준값 — 현행 DIN Lundberg (axial_offset = 0)")
    log("=" * 74)
    for rk in RANKS:
        rig.build(P[rk])
        rig.set_din(0.0)
        m, _ = rig.solve(f"base{rk}")
        base[rk] = m
        log(f"  #{rk:<4} L_we {m['L_we_mm']:6.1f} · σ {m['sigma_MPa']:7.1f} "
            f"@ y* {m['y_star_mm']:+8.2f} ({m['y_star_pct']:4.1f}%) · "
            f"edge_L {m['edge_L_MPa']:7.1f} · margin_L "
            f"{m['margin_L_mm']:5.2f} · Q {m['P_max_N']/1e6:.2f} MN")
        rows.append(row_of(rk, "base", "DIN off 0",
                           (None,) * 6, m, m))
    json.dump({str(k): v for k, v in base.items()},
              open(os.path.join(OUTD, "baseline.json"), "w"), indent=1)

    # ── S0 교정 게이트 ────────────────────────────────────────
    gate = {}
    if "s0" in stages:
        log("\n" + "=" * 74)
        log("S0 — 교정 게이트 : 우리 수식이 MASTA 내장 Johns-Gohar 와 같은가")
        log("   (Johns-Gohar = Fujiwara 의 K1 = K2 = 1 특수해)")
        log("=" * 74)
        ok_all = True
        for rk in RANKS:
            rig.build(P[rk])
            b = base[rk]
            Lwe = b["L_we_mm"] / 1e3
            for f_ in (1.0, 4.0, 8.0):
                Q = b["P_max_N"] * f_
                try:
                    ed = rig.set_jg(Q)
                    mj, _ = rig.solve(f"cal_jg{rk}_{f_}")
                except Exception as e:
                    log(f"  #{rk:<4} ×{f_:g} JG 해석 실패 "
                        f"{str(e).splitlines()[0][:50]}")
                    ok_all = False
                    continue
                rig.set_user(L.profile_fn(Lwe, 1.0, 1.0, ed, Q=Q), NPTS)
                mu, _ = rig.solve(f"cal_us{rk}_{f_}")
                d1 = mu["sigma_MPa"] - mj["sigma_MPa"]
                d2 = (mu["margin_L_mm"] or 0) - (mj["margin_L_mm"] or 0)
                d3 = mu["edge_L_MPa"] - mj["edge_L_MPa"]
                ok = abs(d1) <= 10.0 and abs(d2) <= 1.0
                ok_all = ok_all and ok
                gate[f"{rk}_x{f_:g}"] = dict(
                    end_drop_um=round(ed * 1e6, 2),
                    jg_sigma=mj["sigma_MPa"], us_sigma=mu["sigma_MPa"],
                    d_sigma=round(d1, 1), d_edge=round(d3, 1),
                    d_margin=round(d2, 2), ok=ok)
                log(f"  #{rk:<4} ×{f_:<4} 낙차 {ed*1e6:7.1f} μm │ JG σ "
                    f"{mj['sigma_MPa']:7.1f} edge {mj['edge_L_MPa']:7.1f} "
                    f"margin {mj['margin_L_mm'] or 0:6.2f} │ 우리 σ "
                    f"{mu['sigma_MPa']:7.1f} edge {mu['edge_L_MPa']:7.1f} "
                    f"margin {mu['margin_L_mm'] or 0:6.2f} │ Δσ {d1:+6.1f}"
                    f"  {'통과' if ok else '실패'}")
                rows.append(row_of(rk, "S0", f"JG 대조 ×{f_:g}",
                                   (1.0, 1.0, ed, 1.0, 1.0, ed), mu, b))
        json.dump(gate, open(os.path.join(OUTD, "s0_gate.json"), "w"),
                  indent=1)
        log("  [게이트] " + ("통과 — 수식·좌표주입·부호규약이 모두 검증됐다"
                            if ok_all else
                            "실패 — 원인 규명 전까지 결과를 신뢰할 수 없다"))

    # ── S1 대칭 격자 ──────────────────────────────────────────
    if "s1" in stages:
        log("\n" + "=" * 74)
        log(f"S1 — 대칭 Fujiwara 격자  {len(K1S)}×{len(K2S)}×{len(ZMS)} = "
            f"{len(K1S)*len(K2S)*len(ZMS)} / 설계")
        log("=" * 74)
        for rk in RANKS:
            rig.build(P[rk])
            b = base[rk]
            Lwe, Q = b["L_we_mm"] / 1e3, b["P_max_N"]
            got, n = [], 0
            for K1 in K1S:
                for K2 in K2S:
                    for zm in ZMS:
                        p = (K1, K2, zm, K1, K2, zm)
                        m = run_p(rig, Lwe, Q, p, f"g{rk}_{n}")
                        r = row_of(rk, "S1", f"sym K1={K1} K2={K2} "
                                             f"zm={zm*1e6:.0f}", p, m, b)
                        rows.append(r)
                        got.append((r, p, m))
                        n += 1
                fe = [g for g in got if g[0]["feasible"]]
                log(f"  #{rk:<4} K1={K1:<4} 누적 {n:3d} · 가능 {len(fe):3d} · "
                    f"최고 score "
                    f"{max([g[0]['score'] for g in fe], default=float('nan')):.4f}"
                    f"  {time.perf_counter()-t0:5.0f}s")
            fe = [g for g in got if g[0]["feasible"]]
            if fe:
                bst = max(fe, key=lambda g: g[0]["score"])
                best[(rk, "S1")] = bst
                log(f"  #{rk:<4} ▶ S1 최적 {bst[0]['tag']} · score "
                    f"{bst[0]['score']:.4f} · σ {bst[2]['sigma_MPa']:.1f} · "
                    f"edge_L {bst[2]['edge_L_MPa']:.1f} · margin_L "
                    f"{bst[2]['margin_L_mm']:.2f} · y* "
                    f"{bst[2]['y_star_mm']:+.2f}")
            else:
                log(f"  #{rk:<4} ▶ S1 가능해 없음 — σ 제약을 만족하는 대칭 "
                    f"조합이 격자에 없다")
            _flush(rows)

    # ── S2 비대칭 ─────────────────────────────────────────────
    if "s2" in stages:
        log("\n" + "=" * 74)
        log("S2 — 비대칭 확장 (좌 깊게 · 우 얕게)")
        log("=" * 74)
        for rk in RANKS:
            rig.build(P[rk])
            b = base[rk]
            Lwe, Q = b["L_we_mm"] / 1e3, b["P_max_N"]
            seeds = _seeds(rows, rk)
            if not seeds:
                log(f"  #{rk:<4} 씨앗 없음 — 격자 중앙값으로 시작")
                seeds = [(1.8, 0.7, 150e-6)]
            got, n = [], 0
            for (K1, K2, zm) in seeds:
                for sc_ in SCALES:
                    for ra in RATIOS:
                        p = (K1, K2, zm * sc_, K1, K2, zm * sc_ * ra)
                        m = run_p(rig, Lwe, Q, p, f"a{rk}_{n}")
                        r = row_of(rk, "S2", f"asym K1={K1} K2={K2} "
                                             f"zmL={zm*sc_*1e6:.0f} "
                                             f"r={ra}", p, m, b)
                        rows.append(r)
                        got.append((r, p, m))
                        n += 1
            fe = [g for g in got if g[0]["feasible"]]
            log(f"  #{rk:<4} 씨앗 {len(seeds)} · 시행 {n} · 가능 {len(fe)}"
                f"  {time.perf_counter()-t0:5.0f}s")
            if fe:
                bst = max(fe, key=lambda g: g[0]["score"])
                best[(rk, "S2")] = bst
                log(f"  #{rk:<4} ▶ S2 최적 {bst[0]['tag']} · score "
                    f"{bst[0]['score']:.4f} · σ {bst[2]['sigma_MPa']:.1f} · "
                    f"edge_L {bst[2]['edge_L_MPa']:.1f} · margin_L "
                    f"{bst[2]['margin_L_mm']:.2f} · y* "
                    f"{bst[2]['y_star_mm']:+.2f}")
            _flush(rows)

    # ── S3 국소 정밀화 ────────────────────────────────────────
    if "s3" in stages:
        log("\n" + "=" * 74)
        log("S3 — 국소 정밀화 (좌표하강 · 3라운드)")
        log("=" * 74)
        for rk in RANKS:
            rig.build(P[rk])
            b = base[rk]
            Lwe, Q = b["L_we_mm"] / 1e3, b["P_max_N"]
            cand = [best.get((rk, s)) for s in ("S2", "S1")]
            cand = [c for c in cand if c]
            if not cand:
                log(f"  #{rk:<4} 정밀화할 후보 없음")
                continue
            cur = max(cand, key=lambda g: g[0]["score"])
            p = list(cur[1])
            sbest = cur[0]["score"]
            step = [0.4, 0.15, 0.30, 0.4, 0.15, 0.30]   # K1,K2,zm 상대/절대
            n = 0
            for rnd in range(3):
                moved = False
                for i in range(6):
                    for sgn in (+1, -1):
                        q = list(p)
                        if i in (0, 3):
                            q[i] = round(max(0.5, min(16.0,
                                        q[i] * (1.0 + sgn * step[i]))), 3)
                        elif i in (1, 4):
                            q[i] = round(max(0.2, min(1.0,
                                        q[i] + sgn * step[i])), 3)
                        else:
                            q[i] = max(5e-6, min(1200e-6,
                                       q[i] * (1.0 + sgn * step[i])))
                        if q == p:
                            continue
                        m = run_p(rig, Lwe, Q, tuple(q), f"r{rk}_{n}")
                        r = row_of(rk, "S3", f"refine r{rnd}", tuple(q), m, b)
                        rows.append(r)
                        n += 1
                        if r["feasible"] and (sbest is None
                                              or r["score"] > sbest):
                            sbest, p, moved = r["score"], q, True
                            best[(rk, "S3")] = (r, tuple(q), m)
                step = [x * 0.5 for x in step]
                log(f"  #{rk:<4} 라운드 {rnd} · 시행 {n} · score "
                    f"{sbest:.4f}" + ("" if moved else "  (개선 없음)")
                    + f"  {time.perf_counter()-t0:5.0f}s")
                if not moved and rnd >= 1:
                    break
            bb = best.get((rk, "S3"))
            if bb:
                log(f"  #{rk:<4} ▶ S3 최적 σ {bb[2]['sigma_MPa']:.1f} · "
                    f"edge_L {bb[2]['edge_L_MPa']:.1f} · margin_L "
                    f"{bb[2]['margin_L_mm']:.2f} · y* "
                    f"{bb[2]['y_star_mm']:+.2f} · score {bb[0]['score']:.4f}")
            _flush(rows)

    # ── S4 방향 확인 ──────────────────────────────────────────
    if "s4" in stages:
        log("\n" + "=" * 74)
        log("S4 — 정렬오차 방향 확인 (극한 LC 16건 · 현행 프로파일)")
        log("=" * 74)
        dirs = []
        for rk in RANKS:
            rig.build(P[rk])
            rig.set_din(0.0)
            for nm in LCS:
                try:
                    rig.load_case(nm)
                except StopIteration:
                    continue
                try:
                    m, off = rig.solve(f"d{rk}_{nm}")
                except Exception as e:
                    log(f"  #{rk:<4} {nm:<9} 해석 실패 "
                        f"{str(e).splitlines()[0][:40]}")
                    continue
                if m["y_star_mm"] is None:
                    sgn = 0
                else:
                    sgn = 1 if m["y_star_mm"] > 0 else -1
                dirs.append(dict(rank=rk, lc=nm, y_star_mm=m["y_star_mm"],
                                 sign=sgn, sigma_MPa=m["sigma_MPa"],
                                 tilt_mrad=m["tilt_mrad"],
                                 margin_L_mm=m["margin_L_mm"],
                                 margin_R_mm=m["margin_R_mm"],
                                 edge_L_MPa=m["edge_L_MPa"],
                                 edge_R_MPa=m["edge_R_MPa"]))
                log(f"  #{rk:<4} {nm:<9} y* {str(m['y_star_mm']):>9s} "
                    f"({sgn:+d}) · σ {m['sigma_MPa']:7.1f} · margin L "
                    f"{m['margin_L_mm']:6.2f} / R {m['margin_R_mm']:6.2f}")
        rig.load_case()
        with open(os.path.join(OUTD, "s4_direction.csv"), "w", newline="",
                  encoding="utf-8-sig") as f:
            w = csv.DictWriter(f, fieldnames=list(dirs[0]))
            w.writeheader()
            w.writerows(dirs)
        neg = sum(1 for d in dirs if d["sign"] < 0)
        pos = sum(1 for d in dirs if d["sign"] > 0)
        log(f"\n  [방향] 좌측(−) {neg}건 · 우측(+) {pos}건 → "
            + ("전건 동일 — 비대칭 적용 가능"
               if 0 in (neg, pos) else "부호 반전 존재 — 비대칭은 위험"))

    _flush(rows)
    log(f"\n[완료] {len(rows)}행 · {(time.perf_counter()-t0)/60:.1f}분 · "
        f"{OUTD}")


def _seeds(rows, rk, k=3):
    fe = [r for r in rows if r["rank"] == rk and r["stage"] == "S1"
          and r["feasible"]]
    fe.sort(key=lambda r: -r["score"])
    out, seen = [], set()
    for r in fe:
        key = (r["K1L"], r["K2L"])
        if key in seen:
            continue
        seen.add(key)
        out.append((r["K1L"], r["K2L"], r["zmL_um"] / 1e6))
        if len(out) >= k:
            break
    return out


def _flush(rows):
    with open(os.path.join(OUTD, "fujiwara_all.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=COLS)
        w.writeheader()
        w.writerows(rows)


if __name__ == "__main__":
    av = [a.lower() for a in sys.argv[1:]]
    rk = [a for a in av if a.startswith("ranks=")]
    if rk:
        RANKS = tuple(int(x) for x in rk[0].split("=")[1].split(","))
        av = [a for a in av if not a.startswith("ranks=")]
    st = av or ["s0", "s1", "s2", "s3", "s4"]
    main(st)
