"""
§8-4.4.4 — 베어링 질량 · 총질량 2변수 파레토 프론트
=====================================================
P1 Phase 3 가능해를 두 목적(베어링 1개 질량 ↓ · 총질량 ↓)으로 보고
비지배해를 뽑아 산점도에 얹는다. z1 ≥ 1.0(전지역) · z1 ≥ 1.5 각각 별도 그림.

프론트 양 끝점은 단일목적 최적해와 일치해야 한다 —
  좌단 = §8-4.4.1/§8-4.4.2 의 1위(베어링 최소)
  우단 = §8-4.3.2/§8-4.3.3 의 1위(총질량 최소)
스크립트가 이를 검사한다(불일치면 예외).

기준선(베어링 5.6 t · 총 54.4 t)은 두 축 모두에서 프론트를 지배하나
σ 3,424 로 불합격이라 가능해가 아니다. 축이 3배로 늘어나 프론트가
뭉개지므로 그림에 넣지 않고 캡션으로만 밝힌다.

산출: figures/pareto_z1_10.{png,svg} · pareto_z1_15.{png,svg}
      P1_극한응력_Phase3/p1_pareto.csv + 문서 §8-4.4.4 표
"""
import csv
import io
import os
import re

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt          # noqa: E402
from matplotlib.ticker import MultipleLocator  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "P1_극한응력_Phase3", "p1_grid.csv")
OUT = os.path.join(HERE, "P1_극한응력_Phase3", "p1_pareto.csv")
FIG = os.path.join(HERE, "figures")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
SETS = (("z1>=1.0", "##### 8-4.4.4a", 1.0, "pareto_z1_10", "z₁ ≥ 1.0 m (전지역)"),
        ("z1>=1.5", "##### 8-4.4.4b", 1.5, "pareto_z1_15", "z₁ ≥ 1.5 m"))
DPI = 500

def _next_head(s, base):
    """base 다음 제목 줄의 위치. 없으면 문서 끝.
    레벨을 고정하면 절이 문서 마지막일 때 실패한다(260803)."""
    m = re.search(r"^#{1,6} ", s[base + 5:], re.M)
    return base + 5 + m.start() if m else len(s)

# 편집기가 표를 정렬하면 `| # |` 이 `|  # |` 로 패딩된다 — 여백을 허용해야
# 재실행이 깨지지 않는다(260804).
PAT = r"^\|\s*#\s*\|\s*D_pw.*?(?=\n\n)"
# 열 구성은 §6-11.5a·b(`plot_pareto_s3.py`)와 **같아야 한다** — 두 프론트를
# 좌우로 대조하는 것이 부록 6 의 핵심이라 열이 어긋나면 비교가 안 된다.
HDR = ("| # | D_pw [mm] | d [mm] | D [mm] | T [mm] | B [mm] | C [mm] | "
       "α [°] | D_we [mm] | L_we [mm] | 세장비 | z1 [m] | z2 [m] | Z [개] | "
       "L_eff [m] | **베어링** [t] | 샤프트 [t] | **합계** [t] | σ_max [MPa] |")
SEP = "|--:" * 18 + "|--:|"

plt.rcParams.update({"font.family": "Malgun Gothic", "axes.unicode_minus": False,
                     "font.size": 10.5})


def load():
    with open(SRC, encoding="utf-8-sig") as f:
        return [r for r in csv.DictReader(f)
                if r["feasible"] in ("1", "True", "TRUE")]


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
    ax.scatter(bx, by, s=13, c="#c8ccd4", edgecolors="none", zorder=1,
               label=f"가능해 {len(sub):,}건")
    ax.step(fx, fy, where="post", lw=1.4, c="#c0392b", alpha=.55, zorder=2)
    # 마지막 점 오른쪽 — 베어링을 더 키워도 총질량이 줄지 않는 구간.
    # 이어주지 않으면 프론트가 중간에 끊긴 것처럼 보인다.
    ax.plot([fx[-1], max(bx)], [fy[-1]] * 2, ls=(0, (5, 4)), lw=1.1,
            c="#c0392b", alpha=.42, zorder=2)
    ax.scatter(fx, fy, s=52, c="#c0392b", edgecolors="white", linewidths=.9,
               zorder=3, label=f"파레토 프론트 {len(front)}건")
    # 번호는 **점 아래**에 둔다 — 계단선은 각 점에서 오른쪽으로 간 뒤 아래로
    # 꺾이므로 점의 위·오른쪽은 선이 지나간다. 아래는 항상 비어 있다.
    # x 가 가까운 점끼리 묶어 **좌우로 벌린다**. 세로로 쌓으면 아래 라벨의
    # 지시선이 위 라벨을 뚫고 지나간다(260803).
    span = max(fx) - min(fx)
    grp, cur = [], [0]
    for i in range(1, len(fx)):
        if fx[i] - fx[i - 1] < .05 * span:
            cur.append(i)
        else:
            grp.append(cur)
            cur = [i]
    grp.append(cur)
    # 벌리는 폭은 이웃 점까지의 간격으로 제한한다 — 안 그러면 밀려난 라벨이
    # 다음 점의 라벨 자리를 침범한다(260803, 2·3 겹침).
    fig.canvas.draw()
    x0, x1 = ax.get_xlim()
    ppu = ax.get_window_extent().width / fig.dpi * 72 / (x1 - x0)  # pt / t
    for gi, g in enumerate(grp):
        pitch = 13.0
        if len(g) > 1:
            room = min((fx[g[0]] - fx[g[0] - 1]) * ppu if g[0] else 1e9,
                       (fx[g[-1] + 1] - fx[g[-1]]) * ppu
                       if g[-1] + 1 < len(fx) else 1e9)
            pitch = min(pitch, max(7.0, 2 * .42 * room / (len(g) - 1)))
        for k, j in enumerate(g):
            dx = (k - (len(g) - 1) / 2) * pitch
            arw = (dict(arrowstyle="-", lw=.75, color="#c0392b", alpha=.6,
                        shrinkA=1, shrinkB=5) if len(g) > 1 else None)
            ax.annotate(str(j + 1), (fx[j], fy[j]), textcoords="offset points",
                        xytext=(dx, -18), ha="center", va="center",
                        fontsize=8.5, color="#c0392b", weight="bold",
                        arrowprops=arw)
    ax.annotate("베어링 최소", (fx[0], fy[0]), textcoords="offset points",
                xytext=(9, 15), fontsize=9.5, color="#c0392b", ha="left")
    ax.annotate("총질량 최소", (fx[-1], fy[-1]), textcoords="offset points",
                xytext=(14, 7), fontsize=9.5, color="#c0392b", ha="left",
                va="bottom")   # 도달선(점선) 위로 — 글자 관통 방지
    ax.margins(x=.07, y=.10)

    ax.set_xlabel("베어링 1개 질량 [t]")
    ax.set_ylabel("총질량 = 2 × 베어링 + 샤프트 [t]")
    ax.set_title(f"베어링 질량 · 총질량 파레토 프론트 — {title}", pad=11)
    ax.xaxis.set_major_locator(MultipleLocator(5))
    ax.yaxis.set_major_locator(MultipleLocator(20))
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
            f"{f('C_mm'):,.0f} | {f('alpha'):.0f} | {f('D_we_mm'):.0f} | "
            f"{f('L_we_mm'):.0f} | {f('L_we_mm')/f('D_we_mm'):.3f} | "
            f"{f('z1'):.1f} | {f('z2'):.1f} | {int(float(r['Z']))} | "
            f"{f('L_eff_m'):.3f} | **{mb:.2f}** | {ms:.1f} | "
            f"**{2*mb+ms:.1f}** | {f('sigma_max_MPa'):,.1f} |")


def main():
    rows = load()
    tbls, dump, info = {}, [], {}
    for lab, sect, zmin, stem, title in SETS:
        sub = [r for r in rows if float(r["z1"]) >= zmin]
        F = pareto(sub)
        # 양 끝점 = 단일목적 최적해 검사
        assert F[0] is min(sub, key=lambda r: float(r["mass_brg_kg"])), lab
        assert F[-1] is min(sub, key=lambda r: float(r["mass_total_kg"])), lab
        info[lab] = (len(sub), len(F), draw(sub, F, stem, title))
        tbls[sect] = "\n".join([HDR, SEP] + [row(i, r) for i, r in enumerate(F, 1)])
        for i, r in enumerate(F, 1):
            dump.append(dict(subset=lab, rank_pareto=i, **r))
    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(dump[0]))
        w.writeheader(); w.writerows(dump)

    s = io.open(DOC, encoding="utf-8").read()
    for sect in sorted(tbls, reverse=True):     # 뒤에서 앞으로 — 인덱스 밀림 방지
        base = s.index(sect)
        nxt = _next_head(s, base)      # 다음 제목까지로 제한(260802 사고)
        m = re.search(PAT, s[base:nxt], re.S | re.M)
        if not m:
            raise RuntimeError(f"{sect} 표를 찾지 못했다 — 자리표 표가 있어야 한다")
        s = s[:base + m.start()] + tbls[sect] + s[base + m.end():]
    io.open(DOC, "w", encoding="utf-8").write(s)
    return info


if __name__ == "__main__":
    for k, (n, nf, ps) in main().items():
        sz = " · ".join(f"{a} {b:,.0f} KB" for a, b in ps)
        print(f"[{k}] 가능해 {n} → 파레토 {nf}건 · {sz}")
