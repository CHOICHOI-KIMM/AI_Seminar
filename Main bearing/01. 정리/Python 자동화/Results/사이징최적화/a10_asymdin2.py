# -*- coding: utf-8 -*-
"""§10-12.7.7 재탐색 — 양측 여유 제약 · 좌우 독립 직선 구간

1차(§10-12.7.6)의 결론은 「좌측은 벌었으나 우측이 대신 물린다」였다. 원인은
score 가 좌측 항만 셋이라 우측을 방치한 것이다. 여기서는

  제약  σ ≤ 현행 · margin_L ≥ 1 라미나 · margin_R ≥ 1 라미나
  목적  score = s1 + s3   (동일 가중치)
        s1 중앙화      1 − y*% / y*%₀
        s3 좌우균형    1 − |end_L − end_R| / σ_max
      참고로만 기록 (목적 아님)
        s2 최소여유    min(mL, mR) [라미나] / 3   ← 제약으로 올라갔다
        s4 진입응력    1 − max(end_L, end_R) / σ_max

로 바꾸고, 직선 구간을 **좌우 독립**(f_L · f_R)으로 연다. 설계변수 5개.

  U0  기존 324행 재판정 (MASTA 0회)
  U1  k_L × k_R × δ 격자
  U2  f_L × f_R 추가
  U3  국소 정밀화
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
import a10_asymdin as AD         # noqa: E402

DOC = AD.DOC
OUT = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "asymdin")
RANKS = AD.RANKS
NPTS = AD.NPTS
LAM_MIN = 1.0                    # 양측 여유 하한 [라미나]
LAM_FULL = 3.0                   # s2 만점 기준

K_L = (1.5, 2.0, 2.5, 3.0, 4.0)
K_R = (0.3, 0.4, 0.5, 0.6, 0.8)
DEL = (0, 10, 20, 30, 40)
FS = (0.0, 0.1, 0.2, 0.3)

COLS = ["rank", "stage", "k_L", "k_R", "delta_mm", "f_L", "f_R", "sigma_MPa",
        "end_L_MPa", "end_L_pct", "margin_L_pct", "margin_L_lam",
        "end_R_MPa", "end_R_pct", "margin_R_pct", "margin_R_lam",
        "y_star_pct", "s1", "s2", "s3", "s4", "score", "feasible"]


def log(m):
    print(m, flush=True)


def asym_din2(Lwe, D_we, kL, kR, delta_mm, fL, fR):
    """좌우 독립 직선 구간까지 여는 비대칭 DIN Lundberg"""
    a = Lwe / 2.0
    d = delta_mm / 1e3
    A = AD.A_OVER_DWE * D_we

    def z(y):
        left = y < d
        k = kL if left else kR
        a_s = (a + d) if left else (a - d)
        f = (fL if left else fR) * a_s
        t = abs(y - d)
        cap = k * A * AD.K_CUT
        if t <= f:
            return 0.0
        if a_s <= f + 1e-9:
            return cap
        u = (t - f) / (a_s - f)
        if u >= 1.0:
            return cap
        return min(k * A * math.log(1.0 / (1.0 - u * u)), cap)
    return z


def score2(g, b):
    """제약 만족 시 (s1..s4, score), 아니면 None"""
    if g is None:
        return None
    if g["sigma_MPa"] > b["sigma_MPa"] + 1e-9:
        return None
    if g["margin_L_lam"] < LAM_MIN or g["margin_R_lam"] < LAM_MIN:
        return None
    sm = g["sigma_MPa"]
    s1 = 1.0 - g["y_star_pct"] / max(b["y_star_pct"], 1e-9)
    s2 = min(g["margin_L_lam"], g["margin_R_lam"]) / LAM_FULL
    s3 = 1.0 - abs(g["end_L_MPa"] - g["end_R_MPa"]) / sm
    s4 = 1.0 - max(g["end_L_MPa"], g["end_R_MPa"]) / sm
    # score = s1 + s3 (동일 가중치).  s2 는 이미 제약(≥ 1 라미나)이라
    # 목적에서 빼 중복을 없앴고, s4 는 참고로만 기록한다.
    return (round(s1, 4), round(s2, 4), round(s3, 4), round(s4, 4),
            round(s1 + s3, 5))


def row2(rk, st, p, g, b):
    sc = score2(g, b)
    r = dict(rank=rk, stage=st, k_L=p[0], k_R=p[1], delta_mm=p[2],
             f_L=p[3], f_R=p[4],
             s1=sc[0] if sc else None, s2=sc[1] if sc else None,
             s3=sc[2] if sc else None, s4=sc[3] if sc else None,
             score=sc[4] if sc else None, feasible=int(sc is not None))
    for k in COLS:
        if k not in r:
            r[k] = (g or {}).get(k)
    return r


def fmt(v, n=1):
    return "—" if v is None else f"{v:,.{n}f}"


def write_doc(prog, base, best, t0, note=""):
    B = []
    A = B.append
    A(f"*실행 중 — 단계가 끝날 때마다 갱신된다. 경과 "
      f"{(time.perf_counter()-t0)/60:.1f}분.*{note}")
    A("")
    A("| 단계 | 여는 변수 | 시행 | 가능해 | 경과 | 비고 |")
    A("|---|---|--:|--:|--:|---|")
    for p in prog:
        A(f"| **{p['id']}** | {p['var']} | {p['n']} | {p['ok']} | "
          f"{p['t']:.1f}분 | {p['note']} |")
    A("")
    A("| 설계 | 구분 | `k_L` | `k_R` | `δ` | `f_L` | `f_R` | `σ_max` | "
      "`margin_L` 라미나 | `margin_R` 라미나 | `end_L` | `end_R` | `y*%` | "
      "`s₁` | `s₂` | `s₃` | `s₄` | **score** |")
    A("|---|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|"
      "--:|")
    for rk in RANKS:
        b = base[rk]
        A(f"| **`#{rk}`** | 현행 | 1 | 1 | 0 | 0 | 0 | {fmt(b['sigma_MPa'])} "
          f"| {fmt(b['margin_L_lam'],2)} | {fmt(b['margin_R_lam'],2)} | "
          f"{fmt(b['end_L_MPa'])} | {fmt(b['end_R_MPa'])} | "
          f"{fmt(b['y_star_pct'],1)} | — | — | — | — | — |")
        w = best.get(rk)
        if w:
            p, g, s = w["p"], w["g"], w["s"]
            A(f"| | **최적** | {p[0]:g} | {p[1]:g} | {p[2]:g} | {p[3]:g} | "
              f"{p[4]:g} | **{fmt(g['sigma_MPa'])}** | "
              f"**{fmt(g['margin_L_lam'],2)}** | "
              f"**{fmt(g['margin_R_lam'],2)}** | {fmt(g['end_L_MPa'])} | "
              f"{fmt(g['end_R_MPa'])} | **{fmt(g['y_star_pct'],1)}** | "
              f"{s[0]:.3f} | {s[1]:.3f} | {s[2]:.3f} | {s[3]:.3f} | "
              f"**{s[4]:.4f}** |")
        else:
            A("| | 최적 | — | — | — | — | — | — | **가능해 없음** | — | — | "
              "— | — | — | — | — | — | — |")
    A("")
    blk = "\n".join(B)
    try:
        s = io.open(DOC, encoding="utf-8").read()
        pat = re.compile(r"(<!-- A10:ASYMRUN2 -->\n).*?"
                         r"(<!-- /A10:ASYMRUN2 -->)", re.S)
        if not pat.search(s):
            return
        out = pat.sub(lambda m: m.group(1) + blk + "\n" + m.group(2), s,
                      count=1)
        io.open(DOC, "w", encoding="utf-8").write(out)
    except Exception as e:
        log(f"    [문서 갱신 실패] {str(e).splitlines()[0][:60]}")


def main():
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    base = {int(k): v for k, v in json.load(
        open(os.path.join(OUT, "baseline.json"), encoding="utf-8")).items()}
    rig = L.Rig()
    rig.load_case()
    t0 = time.perf_counter()
    rows, best, prog, geo = [], {}, [], {}

    def run(rk, p, tag):
        g0 = geo[rk]
        f = asym_din2(g0["Lwe"], g0["D_we"], *p)
        try:
            rig.set_user(f, NPTS)
            m, off = rig.solve(tag)
        except Exception as e:
            log(f"    [실패] {tag} {str(e).splitlines()[0][:46]}")
            return None
        return AD.metrics(m, off, g0["Lwe"])

    def record(rk, st, p, g):
        r = row2(rk, st, p, g, base[rk])
        rows.append(r)
        if r["feasible"] and (rk not in best
                              or r["score"] > best[rk]["s"][4]):
            best[rk] = dict(p=p, g=g, s=score2(g, base[rk]), stage=st)
        return r

    def tick(sid, var, note, ts, n, st):
        """설계 하나가 끝날 때마다 문서를 갱신한다 (같은 단계는 덮어쓴다)."""
        ent = dict(id=sid, var=var, n=n,
                   ok=sum(1 for r in rows
                          if r["stage"] == st and r["feasible"]),
                   t=(time.perf_counter() - ts) / 60.0, note=note)
        for i, q in enumerate(prog):
            if q["id"] == sid:
                prog[i] = ent
                break
        else:
            prog.append(ent)
        write_doc(prog, base, best, t0)

    def stage(sid, var, note, ts, n, st):
        tick(sid, var, note, ts, n, st)
        with open(os.path.join(OUT, "asymdin2_all.csv"), "w", newline="",
                  encoding="utf-8-sig") as f:
            w = csv.DictWriter(f, fieldnames=COLS)
            w.writeheader()
            w.writerows(rows)

    for rk in RANKS:
        rig.build(P[rk])
        d = rig.uw.detail
        geo[rk] = dict(Lwe=d.effective_roller_length, D_we=d.element_diameter)

    # ── U0 기존 자료 재판정 (MASTA 0회) ───────────────────────
    log("=" * 74)
    log("U0 — 1차 324행을 새 제약으로 재판정 (MASTA 0회)")
    log("=" * 74)
    ts = time.perf_counter()
    old = list(csv.DictReader(open(os.path.join(OUT, "asymdin_all.csv"),
                                   encoding="utf-8-sig")))
    n_ok = 0
    for r in old:
        try:
            g = {k: (float(r[k]) if r[k] not in ("", "None") else None)
                 for k in ("sigma_MPa", "end_L_MPa", "end_L_pct",
                           "margin_L_pct", "margin_L_lam", "end_R_MPa",
                           "end_R_pct", "margin_R_pct", "margin_R_lam",
                           "y_star_pct")}
        except Exception:
            continue
        if g["sigma_MPa"] is None:
            continue
        rk = int(r["rank"])
        sc = score2(g, base[rk])
        if sc:
            n_ok += 1
    log(f"  1차 {len(old)}행 중 새 제약(양측 ≥ 1 라미나)을 만족하는 해 "
        f"**{n_ok}건**")
    prog.append(dict(id="U0", var="—", n=len(old), ok=n_ok, t=0.0,
                     note=f"1차 자료 재판정 · 만족 {n_ok}건"))
    write_doc(prog, base, best, t0)

    # ── U1 격자 ───────────────────────────────────────────────
    log("\n" + "=" * 74)
    log(f"U1 — k_L × k_R × δ  ({len(K_L)}×{len(K_R)}×{len(DEL)})")
    log("=" * 74)
    ts, nn = time.perf_counter(), 0
    for rk in RANKS:
        rig.build(P[rk])
        for kl in K_L:
            for kr in K_R:
                for dd in DEL:
                    p = (kl, kr, dd, 0.0, 0.0)
                    g = run(rk, p, f"u1_{rk}_{nn}")
                    nn += 1
                    if g:
                        record(rk, "U1", p, g)
            fe = [r for r in rows if r["rank"] == rk and r["stage"] == "U1"
                  and r["feasible"]]
            log(f"  #{rk:<4} k_L {kl:<4} 누적 가능 {len(fe):3d} · 최고 "
                f"{max([r['score'] for r in fe], default=float('nan')):.4f}"
                f"  {(time.perf_counter()-t0)/60:.1f}분")
        tick("U1", "`k_L`·`k_R`·`δ`", f"`#{rk}` 까지", ts, nn, "U1")
    stage("U1", "`k_L`·`k_R`·`δ`", "양측 제약 격자", ts, nn, "U1")

    # ── U2 f_L × f_R ──────────────────────────────────────────
    log("\n" + "=" * 74)
    log("U2 — 좌우 독립 직선 구간 f_L × f_R")
    log("=" * 74)
    ts, nn = time.perf_counter(), 0
    for rk in RANKS:
        rig.build(P[rk])
        seeds = _seeds(rows, rk, "U1")
        for (kl, kr, dd) in seeds:
            for fl in FS:
                for fr in FS:
                    if fl == 0.0 and fr == 0.0:
                        continue
                    p = (kl, kr, dd, fl, fr)
                    g = run(rk, p, f"u2_{rk}_{nn}")
                    nn += 1
                    if g:
                        record(rk, "U2", p, g)
        tick("U2", "`f_L`·`f_R`", f"`#{rk}` 까지", ts, nn, "U2")
        w = best.get(rk)
        log(f"  #{rk:<4} 씨앗 {len(seeds)} · 시행 {nn} · 최고 "
            f"{w['s'][4] if w else float('nan'):.4f} "
            f"(f {w['p'][3]:g}/{w['p'][4]:g})" if w else "")
    stage("U2", "`f_L`·`f_R`", "좌우 독립 직선 구간", ts, nn, "U2")

    # ── U3 국소 정밀화 ────────────────────────────────────────
    log("\n" + "=" * 74)
    log("U3 — 국소 정밀화")
    log("=" * 74)
    ts, nn = time.perf_counter(), 0
    for rk in RANKS:
        rig.build(P[rk])
        if rk not in best:
            log(f"  #{rk:<4} 가능해 없음 — 정밀화 생략")
            continue
        p = list(best[rk]["p"])
        sb = best[rk]["s"][4]
        step = [0.4, 0.1, 6.0, 0.06, 0.06]
        lim = [(0.5, 6.0), (0.1, 1.5), (-40.0, 60.0), (0.0, 0.5), (0.0, 0.5)]
        for rnd in range(3):
            moved = False
            for i in range(5):
                for sg in (+1, -1):
                    q = list(p)
                    q[i] = round(min(lim[i][1],
                                     max(lim[i][0], q[i] + sg * step[i])), 4)
                    if q == p:
                        continue
                    g = run(rk, tuple(q), f"u3_{rk}_{nn}")
                    nn += 1
                    if g is None:
                        continue
                    r = record(rk, "U3", tuple(q), g)
                    if r["feasible"] and r["score"] > sb:
                        sb, p, moved = r["score"], q, True
            step = [x * 0.5 for x in step]
            log(f"  #{rk:<4} 라운드 {rnd} · 시행 {nn} · score {sb:.4f}"
                + ("" if moved else "  (개선 없음)"))
            if not moved and rnd >= 1:
                break
        tick("U3", "전체", f"`#{rk}` 까지", ts, nn, "U3")
    stage("U3", "전체", "국소 정밀화", ts, nn, "U3")

    json.dump({str(k): dict(p=v["p"], s=v["s"], stage=v["stage"], g=v["g"])
               for k, v in best.items()},
              open(os.path.join(OUT, "best2.json"), "w"), indent=1)
    write_doc(prog, base, best, t0, f"  **완료 — 총 {len(rows)}회.**")
    log(f"\n[완료] {len(rows)}행 · {(time.perf_counter()-t0)/60:.1f}분")


def _seeds(rows, rk, st, k=3):
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
    return out or [(2.5, 0.5, 20)]


if __name__ == "__main__":
    main()
