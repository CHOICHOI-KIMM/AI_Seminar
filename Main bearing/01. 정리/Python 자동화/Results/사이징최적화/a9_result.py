"""
§9-9.2 — 부록 9 S3-c 결과 정리
=================================
평가 전량에서 `(외경 D, 베어링 1개 질량)` 프론트를 다시 계산하고, `D` 상한
4.0 · 4.5 · 5.0 m 각각의 최소 베어링 설계를 뽑는다.

**프론트를 최종 집단이 아니라 평가 전량에서 계산하는 이유**는 §6-11.5 와 같다 —
NSGA 는 비지배를 집단 안에서만 판정하므로 과거 세대의 더 나은 설계에 지배되는
점이 섞인다.

산출: figures/pareto_a9_D.{png,svg} · 부록9_NSGA/S3_본최적화/a9_pareto.csv
      + 문서 §9-9.2
"""
import csv
import io
import os
import re

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt                  # noqa: E402
import numpy as np                               # noqa: E402

import sys                                       # noqa: E402
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import a8_eval                                   # noqa: E402

OUT = os.path.join(HERE, "부록9_NSGA", "S3_본최적화")
FIG = os.path.join(HERE, "figures")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
CAPS = (4000, 4500, 5000)
LIMIT = 2100.0

plt.rcParams.update({"font.family": "Malgun Gothic",
                     "axes.unicode_minus": False, "font.size": 10.5})


def rd(p):
    with open(p, encoding="utf-8-sig") as f:
        return list(csv.DictReader(f))


def nd(A):
    k = []
    for i in range(len(A)):
        d = ((A[:, 0] <= A[i, 0]) & (A[:, 1] <= A[i, 1])
             & ((A[:, 0] < A[i, 0]) | (A[:, 1] < A[i, 1])))
        if not d.any():
            k.append(i)
    k.sort(key=lambda i: (A[i, 0], A[i, 1]))
    o, s = [], set()
    for i in k:
        t = (round(A[i, 0], 4), round(A[i, 1], 4))
        if t not in s:
            s.add(t)
            o.append(i)
    return o


def f(r, k):
    return float(r[k])


def main():
    C = rd(os.path.join(OUT, "eval_cache.csv"))
    fe = [r for r in C if 0 < f(r, "sigma_max_MPa") < LIMIT
          and f(r, "z1") >= 1.0 and f(r, "D_mm") <= 5000 + 1e-6]
    print(f"평가 {len(C):,} · 가능해 {len(fe):,} "
          f"({100*len(fe)/len(C):.1f}%)")

    D = np.array([f(r, "D_mm") for r in fe])
    b = np.array([f(r, "mass_brg_kg") for r in fe]) / 1e3
    s = np.array([f(r, "mass_shaft_kg") for r in fe]) / 1e3
    t = 2 * b + s
    k = nd(np.c_[D, b])
    F = [fe[i] for i in k]
    fd, fb, ft, fs = D[k], b[k], t[k], s[k]
    print(f"프론트 {len(F)}건 · D {fd.min():,.0f} ~ {fd.max():,.0f} · "
          f"베어링 {fb.min():.2f} ~ {fb.max():.2f} t · "
          f"총질량 {ft.min():.2f} ~ {ft.max():.2f} t")

    # ── D 상한별 최소 베어링 ───────────────────────────────────────
    # **프론트 위에서** 고른다 — 상한별 최소 베어링점은 정의상 비지배이고,
    # 전량에서 고르면 동률 처리 때문에 프론트 밖 인덱스가 나올 수 있다.
    best = {}
    for c in CAPS:
        m = D <= c + 1e-6
        cand = [i for i in k if D[i] <= c + 1e-6]
        if not cand:
            best[c] = None
            print(f"  D ≤ {c:,}  가능해 {int(m.sum())}건 — 프론트 해 없음")
            continue
        j = min(cand, key=lambda i: (b[i], D[i]))
        best[c] = j
        print(f"  D ≤ {c:,}  가능해 {int(m.sum()):5,} · 최소 베어링 "
              f"{b[j]:6.2f} t (D {D[j]:,.0f} · 총질량 {t[j]:7.2f})")

    # ── 그림 ───────────────────────────────────────────────────────
    fig, ax = plt.subplots(figsize=(7.6, 5.6))
    ax.scatter(D / 1e3, b, s=7, c="#c8ccd4", edgecolors="none", zorder=1,
               label=f"가능해 {len(fe):,}건")
    ax.step(fd / 1e3, fb, where="post", lw=1.4, c="#c0392b", alpha=.55,
            zorder=3)
    ax.scatter(fd / 1e3, fb, s=24, c="#c0392b", edgecolors="white",
               linewidths=.6, zorder=4, label=f"파레토 프론트 {len(F)}건")
    for c in CAPS:
        ax.axvline(c / 1e3, lw=1.1, ls=(0, (5, 4)), c="#2c3e50", alpha=.75,
                   zorder=2)
        j = best[c]
        if j is None:
            ax.annotate(f"D ≤ {c/1e3:.1f} m\n가능해 없음", (c / 1e3, b.max()),
                        textcoords="offset points", xytext=(6, -6),
                        fontsize=9, color="#2c3e50", va="top")
            continue
        ax.scatter([D[j] / 1e3], [b[j]], s=130, marker="*", c="#1e8449",
                   edgecolors="white", linewidths=.8, zorder=5)
        ax.annotate(f"D ≤ {c/1e3:.1f} m\n{b[j]:.2f} t",
                    (D[j] / 1e3, b[j]), textcoords="offset points",
                    xytext=(9, 9), fontsize=9.5, color="#1e8449")
    ax.set_xlabel("외경 D [m]")
    ax.set_ylabel("베어링 1개 질량 [t]")
    ax.set_title("부록 9 S3 파레토 프론트 — 외경 대 베어링 질량", pad=11)
    ax.grid(True, ls=":", lw=.6, c="#b6bcc6", alpha=.7)
    ax.set_axisbelow(True)
    for sp in ("top", "right"):
        ax.spines[sp].set_visible(False)
    ax.legend(frameon=False, loc="upper right", fontsize=9.5)
    ax.margins(x=.05, y=.10)
    fig.tight_layout()
    os.makedirs(FIG, exist_ok=True)
    for ext, kw in (("png", dict(dpi=500)), ("svg", {})):
        fig.savefig(os.path.join(FIG, f"pareto_a9_D.{ext}"),
                    bbox_inches="tight", facecolor="white", **kw)
    plt.close(fig)

    # ── CSV ────────────────────────────────────────────────────────
    rows = []
    for n, i in enumerate(k, 1):
        r = fe[i]
        rows.append(dict(r, rank=n, mass_brg_t=round(b[i], 4),
                         mass_shaft_t=round(s[i], 4),
                         mass_total_t=round(t[i], 4),
                         sigma_margin=round(LIMIT - f(r, "sigma_max_MPa"), 1)))
    with open(os.path.join(OUT, "a9_pareto.csv"), "w", newline="",
              encoding="utf-8-sig") as fh:
        w = csv.DictWriter(fh, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)

    # ── 문서 ───────────────────────────────────────────────────────
    star = {best[c] for c in CAPS if best[c] is not None}
    # 질량 셋을 앞으로 빼고 제원을 뒤에 둔다 — 두 표가 같은 정의를 공유한다
    def mass(i):
        return f"**{b[i]:.2f}** | {s[i]:.1f} | {t[i]:.2f}"

    def geo(i):
        r = fe[i]
        d_ = f(r, "bore_mm")
        th = (d_ - a8_eval.shaft_id(d_ / 1e3) * 1e3) / 2
        return (f"{f(r,'D_pw_mm'):,.0f} | {d_:,.0f} | **{th:.1f}** | "
                f"{d_/th:.1f} | {f(r,'alpha'):.0f} | "
                f"{f(r,'taper_2beta_deg'):.3f} | {f(r,'D_we_mm'):.1f} | "
                f"{f(r,'L_we_mm'):.1f} | {f(r,'slenderness'):.3f} | "
                f"{int(f(r,'Z'))} | {f(r,'z1'):.1f} | {f(r,'z2'):.1f} | "
                f"{f(r,'sigma_max_MPa'):,.1f} | "
                f"{LIMIT-f(r,'sigma_max_MPa'):.1f}")

    MASS = "**베어링** [t] | 샤프트 [t] | 총질량 [t]"
    COLS = ("`D_pw` [mm] | d [mm] | **t** [mm] | **d/t** | α [°] | 2β [°] | "
            "`D_we` [mm] | `L_we` [mm] | 세장비 | Z | z1 [m] | z2 [m] | "
            "σ_max [MPa] | **σ 여유** [MPa]")

    jmin = k[0]                                   # 프론트 최소 D (k 는 D 오름차순)
    body = ["", "![부록 9 파레토 프론트](Results/사이징최적화/figures/"
            "pareto_a9_D.png)", "",
            "회색 점선이 `D` 상한 4.0 · 4.5 · 5.0 m, 초록 별이 각 상한에서의 "
            "**최소 베어링 질량**이다.", "",
            "#### 9-9.2.1. `D` 상한별 최소 베어링", "",
            "*맨 아래 행은 상한이 아니라 **프론트가 도달한 최소 외경**이다 — "
            "그 이하로는 가능해가 없다.*", "",
            f"| `D` 상한 [mm] | 가능해 | 그때 `D` [mm] | {MASS} | {COLS} |",
            "|--:" * (3 + MASS.count("|") + 1 + COLS.count("|") + 1) + "|"]
    for c in CAPS:
        j = best[c]
        if j is None:
            body.append(f"| **{c:,}** | **0** | **해 없음** |"
                        + " — |" * (MASS.count("|") + COLS.count("|") + 2))
            continue
        body.append(f"| **{c:,}** | {int((D <= c + 1e-6).sum()):,} | "
                    f"{D[j]:,.0f} | {mass(j)} | {geo(j)} |")
    body.append(f"| *(도달 하한)* | 1 | **{D[jmin]:,.0f}** | {mass(jmin)} | {geo(jmin)} |")

    # ── 대표점 추출 — `D` 50 mm 간격 · 양 끝과 상한 3점은 강제 포함 ──
    STEP = 50
    must = {k[0], k[-1]} | {j for j in best.values() if j is not None}
    pick, last = [], None
    for i in k:                                   # k 는 D 오름차순이다
        if i in must or last is None or D[i] - last >= STEP:
            pick.append(i)
            last = D[i]
    for i in sorted(must, key=lambda j: D[j]):
        if i not in pick:
            pick.append(i)
    pick.sort(key=lambda i: D[i])
    rank = {i: n for n, i in enumerate(k, 1)}

    HDR = f"| # | {MASS} | `D` [mm] | {COLS} |"
    NCOL = HDR.count("|") - 1                     # 구분줄 칸수를 헤더에서 센다
    body += ["", f"#### 9-9.2.2. 프론트 대표점 {len(pick)}건", "",
             f"프론트 **{len(F)}건**을 `D` **{STEP} mm 간격**으로 솎았다 — "
             f"양 끝과 `D` 상한 3점은 간격과 무관하게 넣었다. `#` 는 전량 "
             f"기준 순번이다. ★ = `D` 상한 3점 · `t` 는 두께 규칙이 만든 "
             f"샤프트 벽두께 · σ 여유 = 2,100 − σ_max", "", HDR,
             "|--:" * NCOL + "|"]
    for i in pick:
        body.append(f"| {rank[i]}{' ★' if i in star else ''} | "
                    f"{mass(i)} | **{D[i]:,.0f}** | {geo(i)} |")
    body += ["", f"*프론트 **전량 {len(F)}건**은 "
             "`부록9_NSGA/S3_본최적화/a9_pareto.csv` · 정리 `a9_result.py`*"]

    doc = io.open(DOC, encoding="utf-8").read()
    a, b_ = "<!-- A9:RES -->", "<!-- /A9:RES -->"
    pat = re.compile(re.escape(a) + r".*?" + re.escape(b_), re.S)
    blk = a + "\n" + "\n".join(body) + "\n" + b_
    out = pat.sub(lambda _m: blk, doc, count=1)
    io.open(DOC, "w", encoding="utf-8").write(out)
    print(f"[문서] §9-9.2 기입 · 프론트 {len(F)}행")


if __name__ == "__main__":
    main()
