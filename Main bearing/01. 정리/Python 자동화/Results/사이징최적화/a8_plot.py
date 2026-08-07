"""
§8-6.3a·b — 부록 8 S3-c 파레토 프론트
========================================
`plot_pareto_s3.py`(§6-11.5a·b)와 **같은 양식**이다 — 색·마커·도달선·축 라벨을
맞춰 두 결과를 나란히 읽을 수 있게 했다.

부록 6 과 다른 점 하나. **프론트 점만 그린다** — 가능해 산점도(10,722건)와 부록 6
프론트를 함께 얹으면 y축이 105 ~ 180 t 로 벌어져, 정작 이번 프론트의 폭
**1.7 t** 가 바닥에 눌려 보이지 않는다. 그래서 §7-6.7.6 처럼 프론트만 남겨
계단 구조가 드러나게 한다. 부록 6 과의 대조는 §8-6.4 의 표가 맡는다.

프론트는 `a8_analyze.py` 가 평가 전량에서 다시 계산한 `a8_pareto.csv` 를 쓴다.
기준선(`z1` = 0.5)은 가능해가 아니므로 자동으로 빠진다.

산출: figures/pareto_a8_z1_10.{png,svg} · pareto_a8_z1_15.{png,svg}
"""
import csv
import io
import os
import re

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt                  # noqa: E402
from matplotlib.ticker import MultipleLocator    # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "부록8_NSGA", "S3_본최적화", "eval_cache.csv")
PAR = os.path.join(HERE, "부록8_NSGA", "S3_본최적화", "a8_pareto.csv")
REF = os.path.join(HERE, "부록6_NSGA", "S3_본최적화", "s3_pareto.csv")
FIG = os.path.join(HERE, "figures")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
DPI = 500
SETS = (("z1>=1.0", "#### 8-6.3a", 1.0, "pareto_a8_z1_10",
         "z₁ ≥ 1.0 m (전지역)"),
        ("z1>=1.5", "#### 8-6.3b", 1.5, "pareto_a8_z1_15", "z₁ ≥ 1.5 m"))

plt.rcParams.update({"font.family": "Malgun Gothic",
                     "axes.unicode_minus": False, "font.size": 10.5})


def rd(p):
    with open(p, encoding="utf-8-sig") as f:
        return list(csv.DictReader(f))


def draw(sub, front, ref, stem, title):
    fx = [float(r["mass_brg_kg"]) / 1e3 for r in front]
    fy = [float(r["mass_total_kg"]) / 1e3 for r in front]

    fig, ax = plt.subplots(figsize=(7.2, 5.4))
    ax.step(fx, fy, where="post", lw=1.4, c="#c0392b", alpha=.55, zorder=2)
    ax.scatter(fx, fy, s=34, c="#c0392b", edgecolors="white", linewidths=.7,
               zorder=3, label=f"파레토 프론트 {len(front)}건")
    ax.annotate(f"베어링 최소\n{fx[0]:.2f} t", (fx[0], fy[0]),
                textcoords="offset points", xytext=(12, 11), fontsize=9.5,
                color="#c0392b", ha="left", va="bottom")
    ax.annotate(f"총질량 최소\n{fy[-1]:.2f} t", (fx[-1], fy[-1]),
                textcoords="offset points", xytext=(-12, -12), fontsize=9.5,
                color="#c0392b", ha="right", va="top")
    ax.margins(x=.12, y=.18)

    ax.set_xlabel("베어링 1개 질량 [t]")
    ax.set_ylabel("총질량 = 2 × 베어링 + 샤프트 [t]")
    ax.set_title(f"부록 8 S3 파레토 프론트 — {title}", pad=11)
    # 프론트만 그리면 y 폭이 1 t 안팎이라 §6-11.5 의 5/10 t 눈금은 쓸 수 없다.
    # 갈래마다 폭이 다르므로(a 1.70 t · b 0.80 t) 눈금을 폭에서 정한다.
    def tick(span):
        return next(s for s in (5, 2, 1, .5, .25, .1, .05) if span / s >= 3)
    ax.xaxis.set_major_locator(MultipleLocator(tick(max(fx) - min(fx))))
    ax.yaxis.set_major_locator(MultipleLocator(tick(max(fy) - min(fy))))
    ax.grid(True, ls=":", lw=.6, c="#b6bcc6", alpha=.8)
    ax.set_axisbelow(True)
    for s in ("top", "right"):
        ax.spines[s].set_visible(False)
    ax.legend(frameon=False, loc="upper right", fontsize=9.5)
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
    cache = [r for r in rd(SRC)
             if r["feasible"] in ("1", "True", "TRUE") and float(r["z1"]) >= 1.0]
    par = rd(PAR)
    ref = rd(REF) if os.path.isfile(REF) else []
    s = io.open(DOC, encoding="utf-8").read()

    for tag, head, lo, stem, title in SETS:
        sub = [r for r in cache if float(r["z1"]) >= lo]
        front = [r for r in par if r["subset"] == tag]
        rf = [r for r in ref if r.get("subset") == tag]
        paths = draw(sub, front, rf, stem, title)
        print(f"[{tag}] 가능해 {len(sub):,} · 프론트 {len(front)} · "
              f"부록6 {len(rf)} · " +
              " · ".join(f"{n} {k:,.0f} KB" for n, k in paths))

        # 자리표 문구를 그림 링크로 교체
        i = s.index(head)
        j = s.index("\n", i)
        blk = s[j:j + 400]
        m = re.search(r"\*파레토 프론트 그림 —[^\n]*\*", blk)
        if not m:
            print(f"  [건너뜀] {head} 자리표 없음")
            continue
        img = (f"![부록 8 S3 파레토 프론트 {title}]"
               f"(Results/사이징최적화/figures/{stem}.png)")
        s = s[:j + m.start()] + img + s[j + m.end():]

    io.open(DOC, "w", encoding="utf-8").write(s)
    print("[문서] 그림 2개 삽입")


if __name__ == "__main__":
    main()
