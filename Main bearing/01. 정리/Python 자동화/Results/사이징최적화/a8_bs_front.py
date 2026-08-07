"""
§8-6.5.7 ⑻ — 부록 8 S3-c 를 `(베어링, 샤프트)` 로 재판정한 프론트
====================================================================
평가 자료는 그대로 두고 **목적함수만 바꿔 비지배 판정을 다시** 한다. MASTA
재해석은 0회다.

  현행  `(f1 베어링, f2 총질량)`   →  8건 · 6건
  재판정 `(g1 베어링, g2 샤프트)`  →  **27건 · 19건**

총질량은 `2·g1 + g2` 로 정확히 복원되므로, 프론트에서 **총질량 최소점을 고르는
것**이 §8-6.5.7 ⑸ 의 선택 단계다. 그림에 총질량 등고선 `2b + s = const` 를
겹쳐 그 등고선이 프론트에 접하는 점이 곧 그 선택임을 보인다.

산출: figures/pareto_bs_z1_10.{png,svg} · pareto_bs_z1_15.{png,svg}
      부록8_NSGA/S3_본최적화/bs_pareto.csv + 문서 §8-6.5.7 ⑻ 표
"""
import csv
import io
import os
import re

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt                  # noqa: E402
import numpy as np                               # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "부록8_NSGA", "S3_본최적화", "eval_cache.csv")
OUT = os.path.join(HERE, "부록8_NSGA", "S3_본최적화", "bs_pareto.csv")
FIG = os.path.join(HERE, "figures")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
SETS = (("a", 1.0, "pareto_bs_z1_10", "z₁ ≥ 1.0 m (전지역)"),
        ("b", 1.5, "pareto_bs_z1_15", "z₁ ≥ 1.5 m"))
DPI = 500

# ── 검토 후 제외한 점 (자동 규칙이 아니라 **기록된 판단**이다) ──────────
# `z2` = 3.0 은 2,453점을 평가해 가능해가 2점(0.08%)뿐이고 그 2점이 그대로
# 프론트에 올랐다. 두 점 모두 σ 여유가 32.9 ~ 52.3 MPa 로 제약을 소진하지
# 못했고(나머지 25건은 0.1 ~ 12.5) `D_pw` 도 `C6` 상한에서 62 ~ 109 mm
# 떨어져 있다 — 세 신호 모두 **그 슬라이스가 탐색되지 않았다**고 말한다.
# 비지배인 것은 사실이나 그 구간의 참 프론트로 보고할 수 없어 제외한다.
EXCLUDE = {("z1>=1.0", 26), ("z1>=1.0", 27)}

plt.rcParams.update({"font.family": "Malgun Gothic",
                     "axes.unicode_minus": False, "font.size": 10.5})


def rd(p):
    with open(p, encoding="utf-8-sig") as f:
        return list(csv.DictReader(f))


def nd(A):
    """2목적 최소화 비지배 인덱스 (목적값 중복은 하나만)"""
    k = []
    for i in range(len(A)):
        d = ((A[:, 0] <= A[i, 0]) & (A[:, 1] <= A[i, 1])
             & ((A[:, 0] < A[i, 0]) | (A[:, 1] < A[i, 1])))
        if not d.any():
            k.append(i)
    k.sort(key=lambda i: (A[i, 0], A[i, 1]))
    out, seen = [], set()
    for i in k:
        t = (round(A[i, 0], 6), round(A[i, 1], 6))
        if t not in seen:
            seen.add(t)
            out.append(i)
    return out


def draw(b, s, cur, jmin, drop, stem, title):
    """b·s 프론트 · 총질량 등고선 · 총질량 최소점 · 현행 프론트 · 제외점"""
    t = 2 * b + s
    keep = ~drop
    fig, ax = plt.subplots(figsize=(7.4, 5.6))

    # 총질량 등고선 — 기울기 −2 의 직선족
    lo, hi = t.min(), t.max()
    for c in np.arange(np.floor(lo / 5) * 5, hi + 5.01, 5):
        xs = np.array([b.min() * .93, b.max() * 1.07])
        ax.plot(xs, c - 2 * xs, lw=.7, ls=(0, (4, 4)), c="#9aa4b2",
                alpha=.75, zorder=1)
        ax.annotate(f"{c:,.0f} t", (xs[1], c - 2 * xs[1]), fontsize=8,
                    color="#7c8798", ha="right", va="bottom", zorder=1)

    ax.step(b[keep], s[keep], where="post", lw=1.4, c="#c0392b", alpha=.5,
            zorder=2)
    ax.scatter(b[keep], s[keep], s=30, c="#c0392b", edgecolors="white",
               linewidths=.7, zorder=3,
               label=f"파레토 프론트 {int(keep.sum())}건")
    if drop.any():
        ax.scatter(b[drop], s[drop], s=54, marker="x", c="#7f8c8d",
                   linewidths=1.6, zorder=3,
                   label=f"미수렴으로 제외 {int(drop.sum())}건")
        ax.annotate("가능해 2점뿐 · σ 여유 33~52 MPa",
                    (b[drop].mean(), s[drop].mean()),
                    textcoords="offset points", xytext=(-14, 16),
                    fontsize=9, color="#7f8c8d", ha="right")
    ax.scatter(b[cur & keep], s[cur & keep], s=96, facecolors="none",
               edgecolors="#2c3e50", linewidths=1.5, zorder=4,
               label=f"현행 (베어링, 총질량) 프론트 {int((cur & keep).sum())}건")
    ax.scatter([b[jmin]], [s[jmin]], s=150, marker="*", c="#1e8449",
               edgecolors="white", linewidths=.8, zorder=5,
               label=f"총질량 최소 {t[jmin]:.2f} t")

    ax.set_xlabel("베어링 1개 질량 [t]")
    ax.set_ylabel("샤프트 질량 [t]")
    ax.set_title(f"베어링 · 샤프트 목적 파레토 프론트 — {title}", pad=11)
    ax.grid(True, ls=":", lw=.6, c="#b6bcc6", alpha=.55)
    ax.set_axisbelow(True)
    for sp in ("top", "right"):
        ax.spines[sp].set_visible(False)
    ax.legend(frameon=False, loc="upper right", fontsize=9.3)
    ax.margins(x=.09, y=.12)
    fig.tight_layout()

    os.makedirs(FIG, exist_ok=True)
    out = []
    for ext, kw in (("png", dict(dpi=DPI)), ("svg", {})):
        p = os.path.join(FIG, f"{stem}.{ext}")
        fig.savefig(p, bbox_inches="tight", facecolor="white", **kw)
        out.append((os.path.basename(p), os.path.getsize(p) / 1024))
    plt.close(fig)
    return out


def main():
    C = rd(SRC)
    rows_all, blocks = [], {}
    for tag, lo, stem, title in SETS:
        R = [r for r in C if 0 < float(r["sigma_max_MPa"]) < 2100
             and float(r["z1"]) >= lo]
        M = np.array([[float(r["mass_brg_kg"]), float(r["mass_shaft_kg"])]
                      for r in R]) / 1e3
        k = nd(M)
        F = [R[i] for i in k]
        b, s = M[k, 0], M[k, 1]
        t = 2 * b + s
        jmin = int(np.argmin(t))

        # 현행 (베어링, 총질량) 프론트에도 드는가
        T = np.c_[M[:, 0], 2 * M[:, 0] + M[:, 1]]
        kcur = set(nd(T))
        cur = np.array([i in kcur for i in k])

        # 검토 판단으로 제외 · 슬라이스 표본수는 판단 근거로 함께 낸다
        sub = f"z1>={lo}"
        drop = np.array([(sub, i + 1) in EXCLUDE for i in range(len(F))])
        nslice = {}
        for r in R:
            nslice[float(r["z2"])] = nslice.get(float(r["z2"]), 0) + 1

        keep = ~drop
        tk = np.where(keep, t, np.inf)
        jmin = int(np.argmin(tk))
        rank = {int(v): n + 1 for n, v in enumerate(np.argsort(tk))
                if keep[int(v)]}
        paths = draw(b, s, cur, jmin, drop, stem, title)
        print(f"[{tag}] 가능해 {len(R):,} · 프론트 {len(F)}건 "
              f"(제외 {int(drop.sum())} → {int(keep.sum())}) · 현행 포함 "
              f"{int((cur & keep).sum())} · 총질량 최소 {t[jmin]:.2f} t · "
              + " · ".join(f"{n} {kb:,.0f} KB" for n, kb in paths))

        hdr = ("| # | D_pw [mm] | d [mm] | D [mm] | T [mm] | α [°] | "
               "2β [°] | D_we [mm] | L_we [mm] | 세장비 | z1 [m] | z2 [m] | "
               "Z [개] | **베어링** [t] | **샤프트** [t] | 총질량 [t] | "
               "총질량 순위 | σ_max [MPa] | **σ 여유** [MPa] | "
               "**슬라이스 표본** | 현행 |")
        sep = "|--:" * 20 + "|:-:|"

        def row(i, n):
            r = F[i]
            star = " ★" if i == jmin else ""
            return (
                f"| {n}{star} | {float(r['D_pw_mm']):,.0f} | "
                f"{float(r['bore_mm']):,.0f} | {float(r['D_mm']):,.0f} | "
                f"{float(r['T_mm']):,.0f} | {float(r['alpha']):.0f} | "
                f"{float(r['taper_2beta_deg']):.3f} | "
                f"{float(r['D_we_mm']):.1f} | {float(r['L_we_mm']):.1f} | "
                f"{float(r['slenderness']):.3f} | {float(r['z1']):.1f} | "
                f"{float(r['z2']):.1f} | {int(float(r['Z']))} | "
                f"**{b[i]:.2f}** | **{s[i]:.2f}** | "
                f"{'**' if i == jmin else ''}{t[i]:.2f}"
                f"{'**' if i == jmin else ''} | "
                f"{rank.get(i, '—')} | {float(r['sigma_max_MPa']):,.1f} | "
                f"{2100-float(r['sigma_max_MPa']):.1f} | "
                f"{nslice[float(r['z2'])]:,} | "
                f"{'●' if cur[i] else '—'} |")

        body = [hdr, sep]
        n = 0
        for i in range(len(F)):
            if keep[i]:
                n += 1
                body.append(row(i, n))
        if drop.any():
            body += ["", "**검토 후 제외** — 아래 두 점은 비지배이나 그 "
                     "슬라이스가 탐색되지 않았다(§8-6.5.7 ⑻ 읽을 것 ⑤)", "",
                     hdr, sep]
            for i in range(len(F)):
                if drop[i]:
                    body.append(row(i, f"~~{i+1}~~"))
        blocks[tag] = "\n".join(body)

        for i, r in enumerate(F):
            rows_all.append(dict(subset=sub, rank=i + 1,
                                 mass_brg_t=round(b[i], 4),
                                 mass_shaft_t=round(s[i], 4),
                                 mass_total_t=round(t[i], 4),
                                 rank_total=rank.get(i, ""),
                                 sigma_margin=round(2100 - float(
                                     r["sigma_max_MPa"]), 1),
                                 n_slice=nslice[float(r["z2"])],
                                 excluded=int(drop[i]),
                                 on_current_front=int(cur[i]), **r))

    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows_all[0]))
        w.writeheader()
        w.writerows(rows_all)

    s_ = io.open(DOC, encoding="utf-8").read()
    for tag in ("a", "b"):
        m = f"A8:BS_{tag.upper()}"
        pat = re.compile(re.escape(f"<!-- {m} -->") + r".*?"
                         + re.escape(f"<!-- /{m} -->"), re.S)
        if pat.search(s_):
            s_ = pat.sub(f"<!-- {m} -->\n" + blocks[tag] + f"\n<!-- /{m} -->",
                         s_, count=1)
        else:
            print(f"  [건너뜀] {m} 자리표 없음")
    io.open(DOC, "w", encoding="utf-8").write(s_)
    print(f"[저장] {OUT} · 문서 표 2개")


if __name__ == "__main__":
    main()
