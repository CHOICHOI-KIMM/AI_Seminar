"""
§10-11.2 — 부록 10 S3-c 결과 정리 (3목적)
============================================
평가 전량에서 `(D, 베어링, 총질량)` 3목적 비지배집합을 다시 계산하고,
`D` 상한 4.0 · 4.5 · 5.0 m 각각의 **두 최적**(최소 베어링 · 최소 총질량)을 뽑는다.

그림은 **2패널 + 제3목적 색상**이다 — 왼쪽 `(D, 베어링)`·색 총질량, 오른쪽
`(D, 총질량)`·색 베어링. 가로축이 둘 다 `D` 라 §9-9.2 그림과 직접 겹쳐 볼 수
있고 `D` 상한 3선을 그대로 쓴다.

산출: figures/pareto_a10.{png,svg} · 부록10_NSGA/S3_본최적화/a10_pareto.csv
      + 문서 §10-11.2
"""
import csv
import io
import os
import re
import sys

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt                  # noqa: E402
import numpy as np                               # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import a8_eval                                   # noqa: E402

OUT = os.path.join(HERE, "부록10_NSGA", "S3_본최적화")
FIG = os.path.join(HERE, "figures")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
CAPS = (4000, 4500, 5000)
LIMIT = 2100.0
STEP = 50                                        # mm — 프론트 대표점 간격

plt.rcParams.update({"font.family": "Malgun Gothic",
                     "axes.unicode_minus": False, "font.size": 10.5})


def rd(p):
    with open(p, encoding="utf-8-sig") as f:
        return list(csv.DictReader(f))


def nd3(A):
    """3목적 최소화 비지배 인덱스"""
    k = []
    for i in range(len(A)):
        d = ((A <= A[i]).all(axis=1) & (A < A[i]).any(axis=1))
        if not d.any():
            k.append(i)
    return k


def f(r, k):
    return float(r[k])


def main():
    C = rd(os.path.join(OUT, "eval_cache.csv"))
    fe = [r for r in C if 0 < f(r, "sigma_max_MPa") < LIMIT
          and f(r, "z1") >= 1.0 and f(r, "D_mm") <= 5000 + 1e-6]
    print(f"평가 {len(C):,} · 가능해 {len(fe):,} ({100*len(fe)/len(C):.1f}%)")

    D = np.array([f(r, "D_mm") for r in fe])
    b = np.array([f(r, "mass_brg_kg") for r in fe]) / 1e3
    sh = np.array([f(r, "mass_shaft_kg") for r in fe]) / 1e3
    t = 2 * b + sh
    z2 = np.array([f(r, "z2") for r in fe])
    k = nd3(np.c_[D, b, t])
    k.sort(key=lambda i: (D[i], b[i]))
    print(f"프론트 {len(k)}건 · D {D[k].min():,.0f}~{D[k].max():,.0f} · "
          f"베어링 {b[k].min():.2f}~{b[k].max():.2f} · "
          f"총질량 {t[k].min():.2f}~{t[k].max():.2f} · "
          f"z2 {z2[k].min():.1f}~{z2[k].max():.1f}")

    # ── D 상한별 두 최적 ──────────────────────────────────────────
    best = {}
    for c in CAPS:
        cand = [i for i in k if D[i] <= c + 1e-6]
        n = int((D <= c + 1e-6).sum())
        if not cand:
            best[c] = (None, None, n)
            print(f"  D <= {c:,}  가능해 {n}건 — 프론트 해 없음")
            continue
        jb = min(cand, key=lambda i: (b[i], D[i]))
        jt = min(cand, key=lambda i: (t[i], D[i]))
        best[c] = (jb, jt, n)
        print(f"  D <= {c:,}  가능해 {n:5,} · 최소 베어링 {b[jb]:6.2f} t "
              f"(z2 {z2[jb]:.1f} · 총 {t[jb]:7.2f}) · 최소 총질량 "
              f"{t[jt]:7.2f} t (z2 {z2[jt]:.1f} · 베어링 {b[jt]:.2f})")

    # ── 그림 ─────────────────────────────────────────────────────
    fig, axes = plt.subplots(1, 2, figsize=(13.2, 5.4))
    for ax, (yy, cc, ylab, clab) in zip(axes, (
            (b, t, "베어링 1개 질량 [t]", "총질량 [t]"),
            (t, b, "총질량 [t]", "베어링 1개 질량 [t]"))):
        ax.scatter(D / 1e3, yy, s=5, c="#d5d9e0", edgecolors="none", zorder=1)
        o = sorted(k, key=lambda i: D[i])
        sc = ax.scatter(D[o] / 1e3, yy[o], c=cc[o], s=26, cmap="viridis",
                        edgecolors="white", linewidths=.4, zorder=3)
        for c in CAPS:
            ax.axvline(c / 1e3, lw=1.1, ls=(0, (5, 4)), c="#2c3e50", alpha=.7,
                       zorder=2)
        for c in CAPS:
            jb, jt, _ = best[c]
            j = jb if yy is b else jt
            if j is None:
                continue
            ax.scatter([D[j] / 1e3], [yy[j]], s=140, marker="*", c="#c0392b",
                       edgecolors="white", linewidths=.8, zorder=5)
            ax.annotate(f"{yy[j]:.2f}", (D[j] / 1e3, yy[j]),
                        textcoords="offset points", xytext=(8, 8),
                        fontsize=9, color="#c0392b")
        ax.set_xlabel("외경 D [m]")
        ax.set_ylabel(ylab)
        ax.grid(True, ls=":", lw=.6, c="#b6bcc6", alpha=.7)
        ax.set_axisbelow(True)
        for sp in ("top", "right"):
            ax.spines[sp].set_visible(False)
        fig.colorbar(sc, ax=ax, label=clab, pad=.02)
    axes[0].set_title("최소 베어링 — 색은 총질량", pad=10)
    axes[1].set_title("최소 총질량 — 색은 베어링", pad=10)
    fig.suptitle("부록 10 S3 3목적 파레토 프론트 — 외경 · 베어링 · 총질량",
                 y=1.00, fontsize=12.5)
    fig.tight_layout()
    os.makedirs(FIG, exist_ok=True)
    for ext, kw in (("png", dict(dpi=450)), ("svg", {})):
        fig.savefig(os.path.join(FIG, f"pareto_a10.{ext}"),
                    bbox_inches="tight", facecolor="white", **kw)
    plt.close(fig)

    # ── CSV ──────────────────────────────────────────────────────
    rows = []
    for n, i in enumerate(k, 1):
        r = fe[i]
        rows.append(dict(r, rank=n, mass_brg_t=round(b[i], 4),
                         mass_shaft_t=round(sh[i], 4),
                         mass_total_t=round(t[i], 4),
                         sigma_margin=round(LIMIT - f(r, "sigma_max_MPa"), 1)))
    with open(os.path.join(OUT, "a10_pareto.csv"), "w", newline="",
              encoding="utf-8-sig") as fh:
        w = csv.DictWriter(fh, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)

    # ── 문서 ─────────────────────────────────────────────────────
    def geo(i, nb=None, nt=None):
        """`nb`·`nt` 를 주면 질량 옆에 정규화 열이 붙는다(대표점 표 전용)"""
        r = fe[i]
        d_ = f(r, "bore_mm")
        idm = a8_eval.shaft_id(d_ / 1e3) * 1e3
        th = (d_ - idm) / 2
        area = np.pi / 4 * (d_ ** 2 - idm ** 2) / 1e6        # m²
        return (f"**{b[i]:.2f}** | " + (f"{nb:.2f} | " if nb is not None else "")
                + f"{sh[i]:.1f} | **{t[i]:.2f}** | "
                + (f"{nt:.2f} | " if nt is not None else "")
                + f"{f(r,'D_pw_mm'):,.0f} | {d_:,.0f} | {th:.1f} | {d_/th:.1f} | "
                # 단면적 · 길이 — 샤프트 [t] ≈ A · L · 7.85 로 검산된다
                f"{area:.2f} | {f(r,'z2') + 0.5:.1f} | "
                f"{f(r,'alpha'):.0f} | {f(r,'D_we_mm'):.1f} | "
                f"{f(r,'L_we_mm'):.1f} | {f(r,'slenderness'):.3f} | "
                f"{int(f(r,'Z'))} | **{f(r,'z2'):.1f}** | "
                f"{f(r,'sigma_max_MPa'):,.1f} | "
                f"{LIMIT-f(r,'sigma_max_MPa'):.1f}")

    COLS = ("**베어링** [t] | 샤프트 [t] | **총질량** [t] | `D_pw` [mm] | "
            "d [mm] | t [mm] | d/t | **A** [m²] | **L** [m] | α [°] | "
            "`D_we` [mm] | "
            "`L_we` [mm] | 세장비 | Z | **z2** [m] | σ_max [MPa] | "
            "σ 여유 [MPa]")
    COLS_N = COLS.replace("**베어링** [t] |", "**베어링** [t] | 정규화 |") \
                 .replace("**총질량** [t] |", "**총질량** [t] | 정규화 |")
    NC = 3 + COLS.count("|") + 1

    body = ["", "![부록 10 3목적 파레토 프론트]"
            "(Results/사이징최적화/figures/pareto_a10.png)", "",
            "왼쪽은 `(D, 베어링)`·색이 총질량, 오른쪽은 `(D, 총질량)`·색이 "
            "베어링이다. 회색 점선이 `D` 상한 4.0 · 4.5 · 5.0 m, 빨간 별이 각 "
            "상한에서의 **그 패널 목적의 최적**이다.", "",
            "#### 10-11.2.1. `D` 상한별 두 최적", "",
            "**목적이 셋이라 상한마다 최적이 둘이다** — 최소 베어링 설계와 최소 "
            "총질량 설계가 갈린다. 둘이 얼마나 다른지(특히 `z2`·샤프트)가 "
            "3목적의 핵심 산출이다.", "",
            "*맨 아래 행은 상한이 아니라 **프론트가 도달한 최소 외경**이다 — "
            "그 이하로는 가능해가 없다. 그 `D` 에는 프론트 점이 하나뿐이라 두 "
            "기준이 갈리지 않는다.*", "",
            f"| `D` 상한 [mm] | 기준 | 그때 `D` [mm] | {COLS} |",
            "|--:" * NC + "|"]
    for c in CAPS:
        jb, jt, n = best[c]
        if jb is None:
            body.append(f"| **{c:,}** | — | **해 없음** |" + " — |" * (NC - 3))
            continue
        for tag, j in (("**최소 베어링**", jb), ("**최소 총질량**", jt)):
            body.append(f"| **{c:,}** | {tag} | {D[j]:,.0f} | {geo(j)} |")
    jlo = k[0]                                   # 프론트 최소 D (k 는 D 오름차순)
    body.append(f"| *(도달 하한)* | — | **{D[jlo]:,.0f}** | {geo(jlo)} |")

    # ── 프론트 대표점 — D 50 mm 구간마다 두 점 ────────────────────
    # **어느 기준으로 뽑혔는지**와 **어느 구간에서 뽑혔는지**를 함께 남긴다.
    # 구간마다 두 기준이 같은 설계를 가리키는 일이 잦아(19구간 중 8) 표시가
    # 없으면 행 수가 들쭉날쭉한 이유를 읽을 수 없다.
    must = {k[0], k[-1]}
    for c in CAPS:
        must |= {x for x in best[c][:2] if x is not None}
    why, seg_of = {}, {}
    lo = D[k].min()
    while lo <= D[k].max() + 1e-6:
        seg = [i for i in k if lo <= D[i] < lo + STEP]
        if seg:
            jt = min(seg, key=lambda i: t[i])
            jb = min(seg, key=lambda i: b[i])
            for j, tag in ((jt, "총질량↓"), (jb, "베어링↓")):
                why[j] = "**둘 다**" if j in why and why[j] != tag else tag
                seg_of[j] = (lo, len(seg))
        lo += STEP
    for i in must:
        why.setdefault(i, "—")
        seg_of.setdefault(i, (D[i] - D[i] % STEP, 0))
    pick = sorted(why, key=lambda i: (D[i], b[i]))
    rank = {i: n for n, i in enumerate(k, 1)}

    body += ["", f"#### 10-11.2.2. 프론트 대표점 {len(pick)}건", "",
             f"프론트 **{len(k)}건**을 `D` **{STEP} mm 구간마다 총질량 최소 · "
             f"베어링 최소 두 점씩** 솎았다. `D` 상한 6점과 양 끝은 구간과 "
             f"무관하게 넣었다(★). `#` 는 전량 기준 순번이다.", "",
             "**「선정」 열이 그 행이 뽑힌 기준**이다 — `둘 다` 는 그 구간에서 "
             "총질량 최소와 베어링 최소가 **같은 설계**라는 뜻이고, 그런 구간은 "
             "행이 하나뿐이다(19구간 중 **8구간**). `★` 는 상한·양 끝 강제 "
             "포함이다.", "",
             "**`A` 는 샤프트 중공 단면적**이다.", "",
             # LaTeX 는 **원시 문자열**로 둔다 — `\f`·`\r` 이 제어문자로
             # 해석되면 수식이 깨진다(`\frac` → 폼피드 + `rac`).
             r"$$A = \frac{\pi}{4}\left(d^{2} - ID^{2}\right), \qquad "
             r"ID = \left\lfloor \left(d^{4} - \frac{32\,W\,d}{\pi}"
             r"\right)^{1/4} \right\rfloor, \qquad "
             r"W = 1.393\times10^{9}\ \mathrm{mm^{3}}$$", "",
             "`ID` 는 두께 규칙이 정하는 샤프트 내경이다(§7-6.7.5 · 목표 DIN 743 안전율 5.2 · `floor` 는 안전측). 벽두께로 쓰면 "
             "`A = π·t·(d − t)` 로도 같다.", "",
             "`샤프트 [t] ≈ A × L × 7.85` 로 검산되므로, 샤프트 질량이 "
             "**단면과 길이 중 어느 쪽에서 왔는지**를 표에서 바로 "
             "분해할 수 있다(§8-6.5.1 과 같은 분해).", "",
             "**「정규화」는 이 표 33행 안에서의 min-max 값**이다 — "
             "`(최대 − 값) / (최대 − 최소)` 로, **최소가 1.00 · 최대가 0.00** "
             "이다(작을수록 좋으므로 클수록 우수). 기준집합이 이 표이므로 "
             "대표점을 다르게 솎으면 값도 달라진다 — 프론트 전량 기준 값은 "
             "`a10_pareto.csv` 에서 직접 계산하면 된다.", "",
             f"| # | **선정** | `D` 구간 [mm] | 구간 점수 | `D` [mm] | {COLS_N} |",
             "|:-:" * 2 + "|--:" * (3 + COLS_N.count("|") + 1) + "|"]
    pb = np.array([b[i] for i in pick])
    pt_ = np.array([t[i] for i in pick])

    def nz(v, arr):
        return 1.0 if arr.max() == arr.min() else (arr.max() - v) / (
            arr.max() - arr.min())

    for i in pick:
        s0, ns = seg_of[i]
        body.append(f"| {rank[i]}{' ★' if i in must else ''} | {why[i]} | "
                    f"{s0:,.0f} ~ {s0+STEP-1:,.0f} | "
                    f"{ns if ns else '—'} | **{D[i]:,.0f}** | "
                    f"{geo(i, nz(b[i], pb), nz(t[i], pt_))} |")
    body += ["", f"*프론트 **전량 {len(k)}건**은 "
             "`부록10_NSGA/S3_본최적화/a10_pareto.csv` · 정리 `a10_result.py`*"]

    s = io.open(DOC, encoding="utf-8").read()
    a, b_ = "<!-- A10:RES -->", "<!-- /A10:RES -->"
    pat = re.compile(re.escape(a) + r".*?" + re.escape(b_), re.S)
    # 본문에 LaTeX 백슬래시가 있으므로 **치환문자열이 아니라 함수**로 넘긴다 —
    # 문자열로 넘기면 `\p` 같은 것이 정규식 이스케이프로 해석돼 터진다.
    blk = a + "\n" + "\n".join(body) + "\n" + b_
    io.open(DOC, "w", encoding="utf-8").write(pat.sub(lambda m: blk, s, count=1))
    print(f"[문서] §10-11.2 기입 · 대표점 {len(pick)}행")


if __name__ == "__main__":
    main()
