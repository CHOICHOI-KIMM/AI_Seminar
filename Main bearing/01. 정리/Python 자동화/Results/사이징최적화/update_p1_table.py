"""
P1 Phase 3 — 문서 §8-4.3 진행 요약·최경량 표 자동 재생성
=========================================================
p1_grid.csv 를 읽어 두 표를 다시 쓴다. run_p1_stress_grid.py 가 500점마다 호출한다.

표 위치는 **절 제목 + 정규식**으로 찾는다 — 헤더 문자열 완전일치는 열을 늘릴 때
실행 중인 프로세스를 죽인다(260729 사고).
"""
import csv
import io
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import sizing_geom as sg   # noqa: E402

DIR = os.path.join(HERE, "P1_극한응력_Phase3")
SRC = os.path.join(DIR, "p1_grid.csv")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
S_PROG = "##### 8-4.3.1"
S_TOP = "##### 8-4.3.2"
S_TOP15 = "##### 8-4.3.3"      # z1 >= 1.5 m 만
Z1_MIN = 1.5
LIMIT = 2100.0
NTOP = 12
TOTAL = 8700

H_PROG = "| 진행 | 가능해 | 가능률 | σ 최소 [MPa] | 최경량 [t] | 경과 |"
P_PROG = "|---:|---:|---:|---:|---:|---:|"
H_TOP = ("| # | D_pw [mm] | d [mm] | D [mm] | B [mm] | C [mm] | T [mm] | "
         "α [°] | D_we [mm] | L_we [mm] | "
         "세장비 | z1 [m] | z2 [m] | Z [개] | L_eff [m] | "
         "베어링 [t] | 샤프트 [t] | **합계** [t] | σ_max [MPa] |")
P_TOP = ("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|"
         "--:|--:|--:|--:|")


def build(elapsed_min=None):
    with open(SRC, encoding="utf-8-sig") as f:
        rows = list(csv.DictReader(f))
    for r in rows:
        r["_s"] = float(r["sigma_max_MPa"] or 0)
        r["_m"] = float(r["mass_total_kg"] or 0)
    ok = sorted((r for r in rows if 0 < r["_s"] < LIMIT), key=lambda r: r["_m"])
    sg_ = [r["_s"] for r in rows if r["_s"] > 0]
    el = "·" if elapsed_min is None else f"{elapsed_min:.0f}분"
    prog = [H_PROG, P_PROG, (
        f"| **{len(rows):,}** / {TOTAL:,} | **{len(ok):,}** | "
        f"{100.0*len(ok)/len(rows):.1f}% | {min(sg_):,.1f} | "
        f"{(ok[0]['_m']/1000 if ok else 0):.1f} | {el} |"
        if rows else "| · | · | · | · | · | · |")]

    def mk(rows_ok):
        t = [H_TOP, P_TOP]
        if not rows_ok:
            t.append("| · " * 19 + "|")
        return t

    def row_of(i, r):
        g = sg.bearing(float(r["D_pw_mm"]) / 1e3, float(r["alpha"]),
                       float(r["D_we_mm"]) / 1e3, float(r["L_we_mm"]) / 1e3)
        return (
            f"| {i} | {float(r['D_pw_mm']):,.0f} | {g['bore']*1e3:,.0f} | "
            f"{g['outer_diameter']*1e3:,.0f} | "
            f"{g['inner_ring_width']*1e3:,.1f} | {g['outer_ring_width']*1e3:,.1f} | "
            f"{g['width']*1e3:,.1f} | {float(r['alpha']):.0f} | "
            f"{float(r['D_we_mm']):.0f} | {float(r['L_we_mm']):.0f} | "
            f"{float(r['slenderness']):.3f} | {float(r['z1']):.1f} | "
            f"{float(r['z2']):.1f} | {r['Z']} | {float(r['L_eff_m']):.3f} | "
            f"{float(r['mass_brg_kg'])/1000:.1f} | "
            f"{float(r['mass_shaft_kg'])/1000:.1f} | "
            f"**{r['_m']/1000:.1f}** | {r['_s']:,.1f} |")

    top = mk(ok)
    for i, r in enumerate(ok[:NTOP], 1):
        top.append(row_of(i, r))

    ok15 = [r for r in ok if float(r["z1"]) >= Z1_MIN]      # §8-4.3.3
    top15 = mk(ok15)
    for i, r in enumerate(ok15[:NTOP], 1):
        top15.append(row_of(i, r))

    return ("\n".join(prog), "\n".join(top), "\n".join(top15),
            len(rows), len(ok), len(ok15))


def swap(s, sect, tbl, pat):
    base = s.index(sect)
    m = re.search(pat, s[base:], re.S | re.M)
    if not m:
        raise RuntimeError(f"{sect} 표를 찾지 못했다")
    return s[:base + m.start()] + tbl + s[base + m.end():]


def main(elapsed_min=None):
    prog, top, top15, n, nok, nok15 = build(elapsed_min)
    s = io.open(DOC, encoding="utf-8").read()
    s = swap(s, S_PROG, prog, r"^\| 진행 \|.*?(?=\n\n)")
    # 뒤쪽(8-4.3.3)부터 교체 — 앞을 먼저 바꾸면 인덱스가 밀린다
    s = swap(s, S_TOP15, top15, r"^\| # \| D_pw.*?(?=\n\n)")
    s = swap(s, S_TOP, top, r"^\| # \| D_pw.*?(?=\n\n)")
    txt = (f"*(수행 중 {n:,}/{TOTAL:,} — 500점마다 자동 갱신)*" if n < TOTAL
           else f"*(완료 {TOTAL:,}점)*")
    i = s.index("#### 8-4.3 결과")
    s = s[:i] + re.sub(r"^\*\((수행|완료)[^\n]*\)\*$", txt, s[i:], count=1,
                       flags=re.M)
    io.open(DOC, "w", encoding="utf-8").write(s)
    return n, nok, nok15


if __name__ == "__main__":
    n, nok, nok15 = main()
    print(f"[문서 갱신] §8-4.3  {n:,}점 · 가능해 {nok:,}건 (z1>=1.5: {nok15:,}건)")
