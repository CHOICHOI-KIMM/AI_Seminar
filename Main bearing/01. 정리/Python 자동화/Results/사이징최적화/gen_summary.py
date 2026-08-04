"""
§6-11.5 — S3-c 세대별 진행 요약표
====================================
`s3_genlog.csv` 150행을 문서용으로 추린다.

**10세대 간격 + 변경점**을 싣는다. 균등 간격만 실으면 최소 질량이 갱신된
세대(예: 97 ~ 104)가 행 사이에 가려져, 정작 `G` = 150 고정을 정당화한
사건이 보이지 않는다.

  변경점 = 최소 베어링질량 또는 최소 총질량이 직전 세대보다 개선된 세대

`ΔHV` 는 **직전에 표시된 행 대비** 증분이다(연속 세대 차가 아니다) —
표시 간격이 일정하지 않으므로 이 정의가 읽기에 맞다.

산출: 문서 §6-11.5 실행 요약 아래 (마커 `<!-- GENLOG -->`)
"""
import csv
import io
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "부록6_NSGA", "S3_본최적화", "s3_genlog.csv")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
MARK = "<!-- GENLOG -->"
STEP = 10


def main():
    with open(SRC, encoding="utf-8-sig") as f:
        G = list(csv.DictReader(f))

    # 변경점 — 두 목적의 최솟값이 갱신된 세대
    改 = set()
    for i, g in enumerate(G):
        if i == 0:
            continue
        for k in ("f1_min", "f2_min"):
            if float(g[k]) < float(G[i - 1][k]) - 1e-9:
                改.add(int(g["gen"]))
    keep = sorted({1, len(G)} | {int(g["gen"]) for g in G
                                 if int(g["gen"]) % STEP == 0} | 改)

    rows, prev_hv, prev_f = [], None, None
    for g in G:
        n = int(g["gen"])
        if n not in keep:
            continue
        hv = float(g["hv"])
        d = "—" if prev_hv is None else f"{hv - prev_hv:+,.1f}"
        f1, f2 = float(g["f1_min"]), float(g["f2_min"])
        tag = " ★" if n in 改 else ""
        # 갱신된 값만 굵게 — 어느 목적이 움직였는지 한눈에 보이게
        b1 = f"**{f1:.3f}**" if prev_f and f1 < prev_f[0] - 1e-9 else f"{f1:.3f}"
        b2 = f"**{f2:.3f}**" if prev_f and f2 < prev_f[1] - 1e-9 else f"{f2:.3f}"
        rows.append(f"| {n}{tag} | {int(g['n_front'])} | {hv:,.1f} | {d} | "
                    f"{b1} | {b2} |")
        prev_hv, prev_f = hv, (f1, f2)

    tbl = "\n".join(
        ["| 세대 | 프론트 | HV | ΔHV | 최소 베어링 [t] | 최소 총질량 [t] |",
         "|--:|--:|--:|--:|--:|--:|"] + rows
        + ["", "★ = 두 목적의 최솟값 중 하나가 갱신된 세대 · `ΔHV` 는 "
           "**직전 표시 행** 대비 · 전체 150세대는 `s3_genlog.csv`."])

    s = io.open(DOC, encoding="utf-8").read()
    pat = re.compile(re.escape(MARK) + r"\n(?:.*?)(?=\n\n#)", re.S)
    if not pat.search(s):
        raise RuntimeError(f"{MARK} 자리표를 찾지 못했다")
    io.open(DOC, "w", encoding="utf-8").write(
        pat.sub(lambda m: MARK + "\n" + tbl, s, count=1))
    print(f"[§6-11.5] 150세대 → {len(rows)}행 (10세대 간격 + 변경점 {len(改)}개)")


if __name__ == "__main__":
    main()
