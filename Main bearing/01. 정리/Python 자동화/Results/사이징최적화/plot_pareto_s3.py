"""
§6-11.5a·b — S3-c 파레토 프론트 (베어링 질량 · 총질량)
========================================================
`plot_pareto_brgmass.py`(§8-4.4.4)와 **같은 양식**으로 그린다. 두 결과를
나란히 놓고 읽을 수 있어야 하므로 색·마커·도달선·축 라벨을 맞췄다.

§8-4.4.4 와 다른 점 둘.
  ① 대상이 격자 8,700점이 아니라 **NSGA 가 평가한 33,161점**이다
  ② 프론트 점이 64·41건으로 많아 **점 번호를 생략**한다(라벨이 서로 덮인다)

**프론트는 평가 전량에서 다시 계산한다.** NSGA 가 보고한 최종 프론트(65건)는
집단 안에서만 비지배를 판정한 결과라, 집단에서 밀려난 과거 세대 설계에
지배되는 점이 하나 섞인다. §8-4.4.4 가 격자 전량 기준이므로 같은 기준으로
맞춘다.

기준선(베어링 5.6 t · 총 54.4 t)은 σ 3,424 로 불합격이라 그리지 않는다.
사전 검증용으로 캐시에 들어간 기준선 행(`z1 = 0.5`)도 제외한다.

산출: figures/pareto_s3_z1_10.{png,svg} · pareto_s3_z1_15.{png,svg}
      부록6_NSGA/S3_본최적화/s3_pareto.csv + 문서 §6-11.5a·b 표(상위 10건)
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
SRC = os.path.join(HERE, "부록6_NSGA", "S3_본최적화", "eval_cache.csv")
OUT = os.path.join(HERE, "부록6_NSGA", "S3_본최적화", "s3_pareto.csv")
FIG = os.path.join(HERE, "figures")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
SETS = (("z1>=1.0", "##### 6-11.5a", 1.0, "pareto_s3_z1_10",
         "z₁ ≥ 1.0 m (전지역)"),
        ("z1>=1.5", "##### 6-11.5b", 1.5, "pareto_s3_z1_15", "z₁ ≥ 1.5 m"))
TOP = 10          # 문서 표에 싣는 건수 (전체는 s3_pareto.csv)
DPI = 500

# 편집기가 표를 정렬하면 `| # |` 이 `|  # |` 로 패딩된다 — 여백을 허용해야
# 재실행이 깨지지 않는다(260804).
PAT = r"^\|\s*#\s*\|\s*D_pw.*?(?=\n\n)"
HDR = ("| # | D_pw [mm] | d [mm] | D [mm] | T [mm] | B [mm] | C [mm] | "
       "α [°] | D_we [mm] | L_we [mm] | 세장비 | z1 [m] | z2 [m] | Z [개] | "
       "L_eff [m] | **베어링** [t] | 샤프트 [t] | **합계** [t] | σ_max [MPa] |")
SEP = "|--:" * 18 + "|--:|"

plt.rcParams.update({"font.family": "Malgun Gothic",
                     "axes.unicode_minus": False, "font.size": 10.5})


def _next_head(s, base):
    m = re.search(r"^#{1,6} ", s[base + 5:], re.M)
    return base + 5 + m.start() if m else len(s)


def load():
    with open(SRC, encoding="utf-8-sig") as f:
        return [r for r in csv.DictReader(f)
                if r["feasible"] in ("1", "True", "TRUE")
                and float(r["z1"]) >= 1.0]        # 기준선(z1=0.5) 제외


def pareto(rows):
    """두 목적 최소화 — 베어링 오름차순 주사, 총질량이 갱신될 때만 비지배"""
    P = sorted(rows, key=lambda r: (float(r["mass_brg_kg"]),
                                    float(r["mass_total_kg"])))
    out, best = [], float("inf")
    for r in P:
        t = float(r["mass_total_kg"])
        if t < best:
            out.append(r)
            best = t
    return out


def draw(sub, front, stem, title):
    bx = [float(r["mass_brg_kg"]) / 1000 for r in sub]
    by = [float(r["mass_total_kg"]) / 1000 for r in sub]
    fx = [float(r["mass_brg_kg"]) / 1000 for r in front]
    fy = [float(r["mass_total_kg"]) / 1000 for r in front]

    fig, ax = plt.subplots(figsize=(7.2, 5.4))
    ax.scatter(bx, by, s=9, c="#c8ccd4", edgecolors="none", zorder=1,
               label=f"가능해 {len(sub):,}건")
    ax.step(fx, fy, where="post", lw=1.4, c="#c0392b", alpha=.55, zorder=2)
    # 마지막 점 오른쪽 — 베어링을 더 키워도 총질량이 줄지 않는 구간(도달선)
    ax.plot([fx[-1], max(bx)], [fy[-1]] * 2, ls=(0, (5, 4)), lw=1.1,
            c="#c0392b", alpha=.42, zorder=2)
    # 점이 64·41개라 §8-4.4.4 보다 작게 — 계단선이 점에 가리지 않게 한다
    ax.scatter(fx, fy, s=26, c="#c0392b", edgecolors="white", linewidths=.7,
               zorder=3, label=f"파레토 프론트 {len(front)}건")
    ax.annotate(f"베어링 최소\n{fx[0]:.2f} t", (fx[0], fy[0]),
                textcoords="offset points", xytext=(11, 13), fontsize=9.5,
                color="#c0392b", ha="left")
    ax.annotate(f"총질량 최소\n{fy[-1]:.1f} t", (fx[-1], fy[-1]),
                textcoords="offset points", xytext=(-8, 12), fontsize=9.5,
                color="#c0392b", ha="right", va="bottom")
    ax.margins(x=.07, y=.10)

    ax.set_xlabel("베어링 1개 질량 [t]")
    ax.set_ylabel("총질량 = 2 × 베어링 + 샤프트 [t]")
    ax.set_title(f"S3 파레토 프론트 — {title}", pad=11)
    ax.xaxis.set_major_locator(MultipleLocator(5))
    ax.yaxis.set_major_locator(MultipleLocator(10))
    ax.grid(True, ls=":", lw=.6, c="#b6bcc6", alpha=.8)
    ax.set_axisbelow(True)
    for s in ("top", "right"):
        ax.spines[s].set_visible(False)
    ax.legend(frameon=False, loc="upper right", fontsize=9.5)
    fig.tight_layout()

    os.makedirs(FIG, exist_ok=True)
    paths = []
    for ext, kw in (("png", dict(dpi=DPI)), ("svg", {})):
        p = os.path.join(FIG, f"{stem}.{ext}")
        fig.savefig(p, bbox_inches="tight", facecolor="white", **kw)
        paths.append((os.path.basename(p), os.path.getsize(p) / 1024))
    plt.close(fig)
    return paths


def row(i, r):
    f = lambda k: float(r[k])                                   # noqa: E731
    mb, ms = f("mass_brg_kg") / 1000, f("mass_shaft_kg") / 1000
    return (f"| {i} | {f('D_pw_mm'):,.0f} | {f('bore_mm'):,.0f} | "
            f"{f('D_mm'):,.0f} | {f('T_mm'):,.0f} | {f('B_mm'):,.0f} | "
            f"{f('C_mm'):,.0f} | {f('alpha'):.0f} | {f('D_we_mm'):.1f} | "
            f"{f('L_we_mm'):.1f} | {f('L_we_mm')/f('D_we_mm'):.3f} | "
            f"{f('z1'):.1f} | {f('z2'):.1f} | {int(float(r['Z']))} | "
            f"{f('L_eff_m'):.3f} | **{mb:.2f}** | {ms:.1f} | "
            f"**{2*mb+ms:.1f}** | {f('sigma_max_MPa'):,.1f} |")


def main():
    rows = load()
    tbls, dump, info = {}, [], {}
    for lab, sect, zmin, stem, title in SETS:
        sub = [r for r in rows if float(r["z1"]) >= zmin]
        F = pareto(sub)
        # 양 끝점 = 단일목적 최적해여야 한다 (§8-4.4.4 와 같은 검사)
        assert F[0] is min(sub, key=lambda r: float(r["mass_brg_kg"])), lab
        assert F[-1] is min(sub, key=lambda r: float(r["mass_total_kg"])), lab
        info[lab] = (len(sub), len(F), draw(sub, F, stem, title))
        tbls[sect] = "\n".join(
            [HDR, SEP] + [row(i, r) for i, r in enumerate(F[:TOP], 1)]
            + ([f"| … | *(이하 {len(F)-TOP}건은 `s3_pareto.csv`)* |"
                + " |" * 17] if len(F) > TOP else []))
        for i, r in enumerate(F, 1):
            dump.append(dict(subset=lab, rank_pareto=i, **r))
    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(dump[0]))
        w.writeheader()
        w.writerows(dump)

    s = io.open(DOC, encoding="utf-8").read()
    for sect in sorted(tbls, reverse=True):   # 뒤에서 앞으로 — 인덱스 밀림 방지
        base = s.index(sect)
        nxt = _next_head(s, base)
        m = re.search(PAT, s[base:nxt], re.S | re.M)
        if not m:
            raise RuntimeError(f"{sect} 표를 찾지 못했다 — 자리표 표가 있어야 한다")
        s = s[:base + m.start()] + tbls[sect] + s[base + m.end():]
    io.open(DOC, "w", encoding="utf-8").write(s)
    return info


if __name__ == "__main__":
    for k, (n, nf, ps) in main().items():
        sz = " · ".join(f"{a} {b:,.0f} KB" for a, b in ps)
        print(f"[{k}] 가능해 {n:,} → 파레토 {nf}건 · {sz}")
