# -*- coding: utf-8 -*-
"""§10-12.7 무인 수행 — 비대칭 DIN Lundberg (k_L · k_R · δ · f/a)

프로파일 (§10-12.7.3)
    |y−δ| ≤ f      : z = 0
    |y−δ| > f      : z = k_s·A·ln[1/(1−((|y−δ|−f)/(a_s−f))²)]
    |y−δ| ≥ a_s    : z = k_s·A·K            (MASTA 와 같은 끝단 절단)
  a_L = L_we/2 + δ · a_R = L_we/2 − δ · A = 4.5e−4·D_we · K = 4.912010

단계 — 한 변수씩 연다
  T0 교정   k=1·δ=0·f=0 이 현행 DIN 을 재현하는가
  T1 k_R    「우측을 얕게 하면 더 지지한다」 직접 확인
  T2 k_L×k_R 격자
  T3 δ 추가
  T4 f 추가
  T5 국소 정밀화 (좌표하강)
  T6 Fujiwara 대조

각 단계가 끝날 때마다 문서의 <!-- A10:ASYMRUN --> 구간을 덮어쓴다.
"""
import csv
import io
import json
import math
import os
import re
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import a10_profile_lib as L      # noqa: E402

DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
OUT = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "asymdin")
FUJI = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "fujiwara")
RANKS = (1, 103, 210)
# MASTA 자신이 DIN 프로파일을 201점으로 들고 있다(outer_race_and_roller_
# profiles). **같은 격자를 써야** 재현된다 — 61점은 끝단을 분해하지 못해
# end_L 이 +73 MPa 어긋나고, 301·401 점은 격자가 어긋나 다시 나빠진다.
NPTS = 201
K_CUT = 4.912010                 # §10-12.7.1 ④
A_OVER_DWE = 4.5e-4              # §10-12.7.1 ⑴
N_LAM = 38                       # MASTA 는 접촉구간을 38등분
EPS = 1.0                        # 접촉 판정 하한 [MPa]

K_R1 = (0.2, 0.3, 0.4, 0.6, 0.8, 1.0, 1.2, 1.5)
K_L2 = (1.0, 1.5, 2.0, 2.5, 3.0, 4.0)
K_R2 = (0.2, 0.4, 0.6, 0.8, 1.0, 1.5)
DELTAS = (-40, -30, -20, -10, 0, 10, 20, 30, 40)
FRACS = (0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6)

COLS = ["rank", "stage", "k_L", "k_R", "delta_mm", "f_over_a", "sigma_MPa",
        "end_L_MPa", "end_L_pct", "margin_L_pct", "margin_L_lam",
        "end_R_MPa", "end_R_pct", "margin_R_pct", "margin_R_lam",
        "y_star_pct", "y_star_mm", "score", "feasible"]


def log(m):
    print(m, flush=True)


# ── 프로파일 ──────────────────────────────────────────────────────
def asym_din(Lwe, D_we, kL, kR, delta_mm, f_over_a):
    """위치 y[m] → 낙차 z[m]"""
    a = Lwe / 2.0
    d = delta_mm / 1e3
    A = A_OVER_DWE * D_we
    f = f_over_a * a

    def z(y):
        t = abs(y - d)
        if t <= f:
            return 0.0
        k = kL if y < d else kR
        a_s = (a + d) if y < d else (a - d)
        if a_s <= f + 1e-9:
            return k * A * K_CUT
        u = (t - f) / (a_s - f)
        if u >= 1.0:
            return k * A * K_CUT
        v = k * A * math.log(1.0 / (1.0 - u * u))
        return min(v, k * A * K_CUT)
    return z


# ── 지표 (§10-12.7.2) ─────────────────────────────────────────────
def metrics(m, off, Lwe):
    half = Lwe * 1e3 / 2.0
    lam = Lwe * 1e3 / N_LAM
    smax = m["sigma_MPa"]
    live = [(p, s) for p, s in off if s > EPS]
    if not live or not smax:
        return None
    ys = max(live, key=lambda t: t[1])
    return dict(
        sigma_MPa=smax,
        end_L_MPa=round(live[0][1], 1),
        end_L_pct=round(100 * live[0][1] / smax, 2),
        margin_L_pct=round(100 * (live[0][0] + half) / half, 3),
        margin_L_lam=round((live[0][0] + half) / lam, 3),
        end_R_MPa=round(live[-1][1], 1),
        end_R_pct=round(100 * live[-1][1] / smax, 2),
        margin_R_pct=round(100 * (half - live[-1][0]) / half, 3),
        margin_R_lam=round((half - live[-1][0]) / lam, 3),
        y_star_mm=round(ys[0], 2),
        y_star_pct=round(100 * abs(ys[0]) / half, 2))


def score_of(g, b):
    if g is None or g["sigma_MPa"] > b["sigma_MPa"] + 1e-9:
        return None
    r1 = (b["end_L_MPa"] - g["end_L_MPa"]) / max(b["end_L_MPa"], 1.0)
    r2 = ((g["margin_L_pct"] - b["margin_L_pct"])
          / max(100.0 - b["margin_L_pct"], 1.0))
    r3 = ((b["y_star_pct"] - g["y_star_pct"]) / max(b["y_star_pct"], 1.0))
    d0 = abs(b["end_L_MPa"] - b["end_R_MPa"])
    d1 = abs(g["end_L_MPa"] - g["end_R_MPa"])
    r4 = (d0 - d1) / max(d0, 1.0)
    return round(r1 + r2 + r3 + r4, 5)


def row_of(rk, st, p, g, b):
    sc = score_of(g, b)
    r = dict(rank=rk, stage=st, k_L=p[0], k_R=p[1], delta_mm=p[2],
             f_over_a=p[3], score=sc, feasible=int(sc is not None))
    r.update({k: (g or {}).get(k) for k in COLS
              if k not in r and k in (g or {})})
    for c in COLS:
        r.setdefault(c, None)
    return r


# ── 문서 실시간 갱신 ──────────────────────────────────────────────
def fmt(v, n=1):
    return "—" if v is None else f"{v:,.{n}f}"


def write_doc(prog, base, best, t0, note=""):
    B = []
    A = B.append
    A(f"*실행 중 — 단계가 끝날 때마다 갱신된다. 경과 "
      f"{(time.perf_counter()-t0)/60:.1f}분.*{note}")
    A("")
    A("**진행**")
    A("")
    A("| 단계 | 여는 변수 | 시행 | 가능해 | 경과 | 비고 |")
    A("|---|---|--:|--:|--:|---|")
    for p in prog:
        A(f"| **{p['id']}** | {p['var']} | {p['n']} | {p['ok']} | "
          f"{p['t']:.1f}분 | {p['note']} |")
    A("")
    A("**현재 최적해** — 각 설계의 현행(DIN 대칭) 대비")
    A("")
    A("| 설계 | 구분 | `k_L` | `k_R` | `δ` | `f/a` | `σ_max` | "
      "`end_L` | `end_L%` | `margin_L%` | 라미나 | `end_R` | `end_R%` | "
      "`margin_R%` | 라미나 | `y*%` | score |")
    A("|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
    for rk in RANKS:
        b = base[rk]
        A(f"| **`#{rk}`** | 현행 | 1 | 1 | 0 | 0 | {fmt(b['sigma_MPa'])} | "
          f"{fmt(b['end_L_MPa'])} | {fmt(b['end_L_pct'],1)} % | "
          f"{fmt(b['margin_L_pct'],2)} % | {fmt(b['margin_L_lam'],2)} | "
          f"{fmt(b['end_R_MPa'])} | {fmt(b['end_R_pct'],1)} % | "
          f"{fmt(b['margin_R_pct'],2)} % | {fmt(b['margin_R_lam'],2)} | "
          f"{fmt(b['y_star_pct'],1)} % | — |")
        w = best.get(rk)
        if w:
            p, g = w["p"], w["g"]
            A(f"| | **최적** | {p[0]:g} | {p[1]:g} | {p[2]:g} | {p[3]:g} | "
              f"**{fmt(g['sigma_MPa'])}** | **{fmt(g['end_L_MPa'])}** | "
              f"**{fmt(g['end_L_pct'],1)} %** | "
              f"**{fmt(g['margin_L_pct'],2)} %** | "
              f"**{fmt(g['margin_L_lam'],2)}** | {fmt(g['end_R_MPa'])} | "
              f"{fmt(g['end_R_pct'],1)} % | {fmt(g['margin_R_pct'],2)} % | "
              f"{fmt(g['margin_R_lam'],2)} | **{fmt(g['y_star_pct'],1)} %** | "
              f"{w['score']:.4f} |")
        else:
            A("| | 최적 | — | — | — | — | — | — | — | — | — | — | — | — | — "
              "| — | — |")
    A("")
    A("*score = 끝단응력 감소율 + 좌단여유 증가율 + `y*` 중앙화율 + "
      "좌우 끝단응력 격차 축소율 (σ ≤ 현행 만족 시에만 정의).*")
    A("")
    blk = "\n".join(B)
    try:
        s = io.open(DOC, encoding="utf-8").read()
        pat = re.compile(r"(<!-- A10:ASYMRUN -->\n).*?(<!-- /A10:ASYMRUN -->)",
                         re.S)
        if not pat.search(s):
            return
        out = pat.sub(lambda m: m.group(1) + blk + "\n" + m.group(2), s,
                      count=1)
        io.open(DOC, "w", encoding="utf-8").write(out)
    except Exception as e:
        log(f"    [문서 갱신 실패] {str(e).splitlines()[0][:60]}")


# ══════════════════════════════════════════════════════════════════
def main():
    os.makedirs(OUT, exist_ok=True)
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    rig = L.Rig()
    rig.load_case()
    t0 = time.perf_counter()
    rows, base, best, prog, geo = [], {}, {}, [], {}

    def run(rk, p, tag):
        g0 = geo[rk]
        f = asym_din(g0["Lwe"], g0["D_we"], p[0], p[1], p[2], p[3])
        try:
            rig.set_user(f, NPTS)
            m, off = rig.solve(tag)
        except Exception as e:
            log(f"    [실패] {tag} {str(e).splitlines()[0][:50]}")
            return None
        return metrics(m, off, g0["Lwe"])

    def record(rk, st, p, g):
        r = row_of(rk, st, p, g, base[rk])
        rows.append(r)
        if r["feasible"] and (rk not in best
                              or r["score"] > best[rk]["score"]):
            best[rk] = dict(p=p, g=g, score=r["score"], stage=st)
        return r

    def stage(sid, var, note, t_start, n, ok):
        prog.append(dict(id=sid, var=var, n=n, ok=ok,
                         t=(time.perf_counter() - t_start) / 60.0, note=note))
        write_doc(prog, base, best, t0)
        _flush(rows)

    # ── 기준값 ────────────────────────────────────────────────
    log("=" * 74)
    log("기준값 — 현행 DIN Lundberg (대칭 · axial_offset 0)")
    log("=" * 74)
    for rk in RANKS:
        rig.build(P[rk])
        d = rig.uw.detail
        geo[rk] = dict(Lwe=d.effective_roller_length,
                       D_we=d.element_diameter)
        rig.set_din(0.0)
        m, off = rig.solve(f"b{rk}")
        base[rk] = metrics(m, off, geo[rk]["Lwe"])
        b = base[rk]
        log(f"  #{rk:<4} σ {b['sigma_MPa']:7.1f} · end_L {b['end_L_MPa']:7.1f}"
            f" ({b['end_L_pct']:5.2f}%) · margin_L {b['margin_L_pct']:5.3f}% "
            f"({b['margin_L_lam']:.3f} 라미나) · end_R {b['end_R_MPa']:7.1f} "
            f"({b['end_R_pct']:5.2f}%) · y* {b['y_star_pct']:5.2f}%")
    json.dump({str(k): v for k, v in base.items()},
              open(os.path.join(OUT, "baseline.json"), "w"), indent=1)

    # ── T0 교정 게이트 ────────────────────────────────────────
    log("\n" + "=" * 74)
    log("T0 — 교정 게이트 : k=1 · δ=0 · f=0 이 현행 DIN 을 재현하는가")
    log("=" * 74)
    ts, okall, nn = time.perf_counter(), True, 0
    for rk in RANKS:
        rig.build(P[rk])
        g = run(rk, (1.0, 1.0, 0.0, 0.0), f"cal{rk}")
        nn += 1
        if g is None:
            okall = False
            continue
        b = base[rk]
        d1 = g["sigma_MPa"] - b["sigma_MPa"]
        d2 = g["end_L_MPa"] - b["end_L_MPa"]
        d3 = g["margin_L_pct"] - b["margin_L_pct"]
        ok = abs(d1) <= 3.0 and abs(d2) <= 15.0 and abs(d3) <= 0.05
        okall = okall and ok
        log(f"  #{rk:<4} σ {g['sigma_MPa']:7.1f} (Δ{d1:+6.1f}) · end_L "
            f"{g['end_L_MPa']:7.1f} (Δ{d2:+7.1f}) · margin_L "
            f"{g['margin_L_pct']:6.3f}% (Δ{d3:+6.3f}) "
            f"{'통과' if ok else '실패'}")
        record(rk, "T0", (1.0, 1.0, 0.0, 0.0), g)
    log("  [게이트] " + ("통과" if okall else "실패 — 결과 신뢰 불가"))
    stage("T0", "—", "교정 " + ("통과" if okall else "**실패**"), ts, nn,
          sum(1 for r in rows if r["stage"] == "T0" and r["feasible"]))

    # ── T1 k_R 단독 ───────────────────────────────────────────
    log("\n" + "=" * 74)
    log("T1 — k_R 단독 (k_L=1 · δ=0 · f=0)")
    log("=" * 74)
    ts, nn = time.perf_counter(), 0
    for rk in RANKS:
        rig.build(P[rk])
        for kr in K_R1:
            g = run(rk, (1.0, kr, 0.0, 0.0), f"t1_{rk}_{kr}")
            nn += 1
            if g:
                r = record(rk, "T1", (1.0, kr, 0.0, 0.0), g)
                log(f"  #{rk:<4} k_R {kr:<4} σ {g['sigma_MPa']:7.1f} · "
                    f"end_L {g['end_L_MPa']:7.1f} · end_R "
                    f"{g['end_R_MPa']:7.1f} · mL {g['margin_L_pct']:6.3f}% · "
                    f"mR {g['margin_R_pct']:6.3f}% · y* "
                    f"{g['y_star_pct']:5.2f}% · "
                    f"{'sc %.3f' % r['score'] if r['feasible'] else '위반'}")
    stage("T1", "`k_R`", "우측 계수 단독", ts, nn,
          sum(1 for r in rows if r["stage"] == "T1" and r["feasible"]))

    # ── T2 k_L × k_R ──────────────────────────────────────────
    log("\n" + "=" * 74)
    log(f"T2 — k_L × k_R 격자 ({len(K_L2)}×{len(K_R2)})")
    log("=" * 74)
    ts, nn = time.perf_counter(), 0
    for rk in RANKS:
        rig.build(P[rk])
        for kl in K_L2:
            for kr in K_R2:
                g = run(rk, (kl, kr, 0.0, 0.0), f"t2_{rk}_{kl}_{kr}")
                nn += 1
                if g:
                    record(rk, "T2", (kl, kr, 0.0, 0.0), g)
            fe = [r for r in rows if r["rank"] == rk and r["stage"] == "T2"
                  and r["feasible"]]
            log(f"  #{rk:<4} k_L {kl:<4} 누적 가능 {len(fe):3d} · 최고 "
                f"{max([r['score'] for r in fe], default=float('nan')):.4f}"
                f"  {(time.perf_counter()-t0)/60:.1f}분")
    stage("T2", "`k_L`·`k_R`", "좌우 배율 격자", ts, nn,
          sum(1 for r in rows if r["stage"] == "T2" and r["feasible"]))

    # ── T3 δ ──────────────────────────────────────────────────
    log("\n" + "=" * 74)
    log("T3 — δ 추가")
    log("=" * 74)
    ts, nn = time.perf_counter(), 0
    for rk in RANKS:
        rig.build(P[rk])
        for (kl, kr) in _seeds(rows, rk, "T2"):
            for dd in DELTAS:
                g = run(rk, (kl, kr, dd, 0.0), f"t3_{rk}_{kl}_{kr}_{dd}")
                nn += 1
                if g:
                    record(rk, "T3", (kl, kr, dd, 0.0), g)
        w = best.get(rk)
        log(f"  #{rk:<4} 시행 {nn} · 최적 score "
            f"{w['score'] if w else float('nan'):.4f} · "
            f"{(time.perf_counter()-t0)/60:.1f}분")
    stage("T3", "`δ`", "오프셋 추가", ts, nn,
          sum(1 for r in rows if r["stage"] == "T3" and r["feasible"]))

    # ── T4 f ──────────────────────────────────────────────────
    log("\n" + "=" * 74)
    log("T4 — 직선 구간 f/a 추가")
    log("=" * 74)
    ts, nn = time.perf_counter(), 0
    for rk in RANKS:
        rig.build(P[rk])
        for (kl, kr, dd) in _seeds3(rows, rk, "T3"):
            for fr in FRACS:
                g = run(rk, (kl, kr, dd, fr), f"t4_{rk}_{kl}_{kr}_{dd}_{fr}")
                nn += 1
                if g:
                    r = record(rk, "T4", (kl, kr, dd, fr), g)
                    log(f"  #{rk:<4} k {kl:g}/{kr:g} δ {dd:+g} f/a {fr:.1f} → "
                        f"σ {g['sigma_MPa']:7.1f} · end_L "
                        f"{g['end_L_MPa']:7.1f} · mL "
                        f"{g['margin_L_pct']:6.3f}% ({g['margin_L_lam']:.2f}) "
                        f"· y* {g['y_star_pct']:5.2f}% · "
                        f"{'sc %.3f' % r['score'] if r['feasible'] else '위반'}")
    stage("T4", "**`f/a`**", "직선 구간 추가", ts, nn,
          sum(1 for r in rows if r["stage"] == "T4" and r["feasible"]))

    # ── T5 국소 정밀화 ────────────────────────────────────────
    log("\n" + "=" * 74)
    log("T5 — 국소 정밀화 (좌표하강 3라운드)")
    log("=" * 74)
    ts, nn = time.perf_counter(), 0
    for rk in RANKS:
        rig.build(P[rk])
        if rk not in best:
            log(f"  #{rk:<4} 후보 없음")
            continue
        p = list(best[rk]["p"])
        sb = best[rk]["score"]
        step = [0.5, 0.2, 8.0, 0.08]
        for rnd in range(3):
            moved = False
            for i in range(4):
                for sg in (+1, -1):
                    q = list(p)
                    lim = [(0.3, 6.0), (0.1, 2.0), (-60.0, 60.0), (0.0, 0.7)]
                    q[i] = round(min(lim[i][1],
                                     max(lim[i][0], q[i] + sg * step[i])), 4)
                    if q == p:
                        continue
                    g = run(rk, tuple(q), f"t5_{rk}_{nn}")
                    nn += 1
                    if g is None:
                        continue
                    r = record(rk, "T5", tuple(q), g)
                    if r["feasible"] and r["score"] > sb:
                        sb, p, moved = r["score"], q, True
            step = [x * 0.5 for x in step]
            log(f"  #{rk:<4} 라운드 {rnd} · 시행 {nn} · score {sb:.4f}"
                + ("" if moved else "  (개선 없음)"))
            if not moved and rnd >= 1:
                break
    stage("T5", "전체", "국소 정밀화", ts, nn,
          sum(1 for r in rows if r["stage"] == "T5" and r["feasible"]))

    # ── T6 Fujiwara 대조 ──────────────────────────────────────
    log("\n" + "=" * 74)
    log("T6 — §10-12.6 Fujiwara 최적과 대조")
    log("=" * 74)
    ts, nn = time.perf_counter(), 0
    comp = {}
    QB = json.load(open(os.path.join(FUJI, "baseline.json"), encoding="utf-8"))
    FR = list(csv.DictReader(open(os.path.join(FUJI, "fujiwara_all.csv"),
                                  encoding="utf-8-sig")))
    for rk in RANKS:
        rig.build(P[rk])
        S = [r for r in FR if int(r["rank"]) == rk and r["stage"] == "S3"
             and r["feasible"] == "1"]
        if not S:
            continue
        t = max(S, key=lambda r: float(r["score"]))
        f = L.profile_fn(geo[rk]["Lwe"], float(t["K1L"]), float(t["K2L"]),
                         float(t["zmL_um"]) / 1e6, K1R=float(t["K1R"]),
                         K2R=float(t["K2R"]), zmR=float(t["zmR_um"]) / 1e6,
                         Q=QB[str(rk)]["P_max_N"])
        rig.set_user(f, NPTS)
        m, off = rig.solve(f"t6_{rk}")
        nn += 1
        comp[rk] = metrics(m, off, geo[rk]["Lwe"])
        log(f"  #{rk:<4} Fujiwara → σ {comp[rk]['sigma_MPa']:7.1f} · end_L "
            f"{comp[rk]['end_L_MPa']:7.1f} ({comp[rk]['end_L_pct']:.1f}%) · "
            f"mL {comp[rk]['margin_L_pct']:.3f}% "
            f"({comp[rk]['margin_L_lam']:.2f} 라미나) · y* "
            f"{comp[rk]['y_star_pct']:.2f}%")
    json.dump({str(k): v for k, v in comp.items()},
              open(os.path.join(OUT, "fuji_compare.json"), "w"), indent=1)
    stage("T6", "—", "Fujiwara 대조", ts, nn, len(comp))

    json.dump({str(k): dict(p=v["p"], score=v["score"], stage=v["stage"],
                            g=v["g"]) for k, v in best.items()},
              open(os.path.join(OUT, "best.json"), "w"), indent=1)
    write_doc(prog, base, best, t0,
              f"  **완료 — 총 {len(rows)}회.**")
    log(f"\n[완료] {len(rows)}행 · {(time.perf_counter()-t0)/60:.1f}분 · {OUT}")


def _seeds(rows, rk, st, k=3):
    fe = [r for r in rows if r["rank"] == rk and r["stage"] == st
          and r["feasible"]]
    fe.sort(key=lambda r: -r["score"])
    out, seen = [], set()
    for r in fe:
        key = (r["k_L"], r["k_R"])
        if key in seen:
            continue
        seen.add(key)
        out.append(key)
        if len(out) >= k:
            break
    return out or [(2.0, 0.6)]


def _seeds3(rows, rk, st, k=3):
    fe = [r for r in rows if r["rank"] == rk and r["stage"] == st
          and r["feasible"]]
    fe.sort(key=lambda r: -r["score"])
    out, seen = [], set()
    for r in fe:
        key = (r["k_L"], r["k_R"], r["delta_mm"])
        if key in seen:
            continue
        seen.add(key)
        out.append(key)
        if len(out) >= k:
            break
    return out or [(2.0, 0.6, 0.0)]


def _flush(rows):
    with open(os.path.join(OUT, "asymdin_all.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=COLS)
        w.writeheader()
        w.writerows(rows)


if __name__ == "__main__":
    main()
