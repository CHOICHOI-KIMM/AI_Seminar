"""
§8-4.4 — 베어링 단독 질량 기준 최경량 가능해
=============================================
P1 Phase 3 가능해 771건을 **베어링 1개 질량**으로만 정렬해 상위 12건을 뽑는다
(기존 §8-4.3.2·§8-4.3.3 은 총질량 = 2×베어링 + 샤프트 기준).

동일 베어링 기하(D_pw·α·D_we·L_we)는 배치(z1·z2)만 달라도 질량이 같으므로
**총질량이 최소인 배치 하나로 대표**시킨다 — 안 그러면 z1 ≥ 1.5 표의 고유 기하가
6/12 로 줄어 서로 다른 베어링을 12종 보여주지 못한다.

산출: P1_극한응력_Phase3/p1_brgmass.csv + 문서 §8-4.4.1·§8-4.4.2 표
표 위치는 절 제목 + 정규식으로 찾고 패턴에 헤더 내용을 넣지 않는다(260730 사고).
탐색 범위를 **다음 ##### 까지로 제한**한다 — 제한이 없으면 절에 표가 아직
없을 때 아래쪽 다른 절의 표를 잡아 덮어쓴다(260802 사고, §8-5.5.1 피해).
"""
import csv
import io
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "P1_극한응력_Phase3", "p1_grid.csv")
OUT = os.path.join(HERE, "P1_극한응력_Phase3", "p1_brgmass.csv")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
SETS = (("z1>=1.0", "##### 8-4.4.1", 1.0),
        ("z1>=1.5", "##### 8-4.4.2", 1.5))
N = 12
# 기준선 v1.3 — 제원은 §4-3 MASTA 실측. z1 0.5 라 두 집합 어디에도 없다.
BASE = dict(D_pw_mm=3330.9, bore_mm=3055.0, D_mm=3600.0, B_mm=300.0, C_mm=253.0,
            T_mm=310.0, alpha=19.0, D_we_mm=110.51, L_we_mm=238.048, Z=87,
            z1=0.5, z2=3.0, L_eff_m=3.616662,
            mass_brg_kg=5600.5, mass_shaft_kg=43225.8, sigma_max_MPa=3424.2)
HDR = ("| # | D_pw [mm] | d [mm] | D [mm] | B [mm] | C [mm] | T [mm] | α [°] | "
       "D_we [mm] | L_we [mm] | 세장비 | z1 [m] | z2 [m] | Z [개] | L_eff [m] | "
       "**베어링** [t] | 샤프트 [t] | 합계 [t] | 총질량 순위 | σ_max [MPa] |")
SEP = "|--:" * 19 + "|--:|"

def _next_head(s, base):
    """base 다음 제목 줄의 위치. 없으면 문서 끝.
    레벨을 고정하면 절이 문서 마지막일 때 실패한다(260803)."""
    m = re.search(r"^#{1,6} ", s[base + 5:], re.M)
    return base + 5 + m.start() if m else len(s)

PAT = r"^\| # \| D_pw.*?(?=\n\n)"


def load():
    with open(SRC, encoding="utf-8-sig") as f:
        return [r for r in csv.DictReader(f)
                if r["feasible"] in ("1", "True", "TRUE")]


def pick(rows, zmin):
    """베어링 질량 오름차순 · 동일 기하는 총질량 최소 배치로 대표"""
    sub = [r for r in rows if float(r["z1"]) >= zmin]
    trank = {r["idx"]: i + 1 for i, r in
             enumerate(sorted(sub, key=lambda r: float(r["mass_total_kg"])))}
    best = {}
    for r in sub:
        g = (r["D_pw_mm"], r["alpha"], r["D_we_mm"], r["L_we_mm"])
        if g not in best or float(r["mass_total_kg"]) < float(best[g]["mass_total_kg"]):
            best[g] = r
    out = sorted(best.values(), key=lambda r: float(r["mass_brg_kg"]))[:N]
    return out, trank, len(sub), len(best)


def row(i, r, trank):
    f = lambda k: float(r[k])                                   # noqa: E731
    mb, ms = f("mass_brg_kg") / 1000, f("mass_shaft_kg") / 1000
    tr = trank.get(r["idx"], "")
    return (f"| {i} | {f('D_pw_mm'):,.0f} | {f('bore_mm'):,.0f} | {f('D_mm'):,.0f} | "
            f"{f('B_mm'):,.1f} | {f('C_mm'):,.1f} | {f('T_mm'):,.1f} | "
            f"{f('alpha'):.0f} | {f('D_we_mm'):.0f} | {f('L_we_mm'):.0f} | "
            f"{f('L_we_mm')/f('D_we_mm'):.3f} | {f('z1'):.1f} | {f('z2'):.1f} | "
            f"{int(float(r['Z']))} | {f('L_eff_m'):.3f} | **{mb:.1f}** | {ms:.1f} | "
            f"{2*mb+ms:.1f} | {tr} | {f('sigma_max_MPa'):,.1f} |")


def base_row():
    b = BASE
    mb, ms = b["mass_brg_kg"] / 1000, b["mass_shaft_kg"] / 1000
    return (f"| **기준선** | {b['D_pw_mm']:,.0f} | {b['bore_mm']:,.0f} | "
            f"{b['D_mm']:,.0f} | {b['B_mm']:,.1f} | {b['C_mm']:,.1f} | "
            f"{b['T_mm']:,.1f} | {b['alpha']:.0f} | {b['D_we_mm']:.0f} | "
            f"{b['L_we_mm']:.0f} | {b['L_we_mm']/b['D_we_mm']:.3f} | "
            f"{b['z1']:.1f} | {b['z2']:.1f} | {b['Z']} | {b['L_eff_m']:.3f} | "
            f"**{mb:.1f}** | {ms:.1f} | {2*mb+ms:.1f} | — | "
            f"{b['sigma_max_MPa']:,.1f} |")


def main():
    rows = load()
    tbls, dump, meta = {}, [], {}
    for lab, sect, zmin in SETS:
        top, trank, nsub, nuniq = pick(rows, zmin)
        meta[lab] = (nsub, nuniq)
        lines = [HDR, SEP] + [row(i, r, trank) for i, r in enumerate(top, 1)]
        lines.append(base_row())
        tbls[sect] = "\n".join(lines)
        for i, r in enumerate(top, 1):
            dump.append(dict(subset=lab, rank_brg=i,
                             rank_total=trank.get(r["idx"], ""), **r))
    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(dump[0]))
        w.writeheader(); w.writerows(dump)

    s = io.open(DOC, encoding="utf-8").read()
    for sect in sorted(tbls, reverse=True):        # 뒤에서 앞으로 — 인덱스 밀림 방지
        base = s.index(sect)
        nxt = _next_head(s, base)                 # 다음 제목까지로 탐색 제한
        m = re.search(PAT, s[base:nxt], re.S | re.M)
        if not m:
            raise RuntimeError(f"{sect} 표를 찾지 못했다 — 자리표 표가 있어야 한다")
        s = s[:base + m.start()] + tbls[sect] + s[base + m.end():]
    io.open(DOC, "w", encoding="utf-8").write(s)
    return meta


if __name__ == "__main__":
    for k, (n, u) in main().items():
        print(f"[{k}] 가능해 {n}건 · 고유 기하 {u}종 → 상위 {N} 갱신")
