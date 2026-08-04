"""
§6-11.6 — 제작 제약을 고려한 설계안 선정
===========================================
S3-c 파레토 프론트(`z1 ≥ 1.0` · 64건)에 **제작 상한** 네 개를 걸어 제작
가능 후보군을 추린다.

  d     보어 = 샤프트 외경   링 선반 스윙 · 열처리로 · 운송 폭
  D     베어링 외경         동상
  D_we  롤러 외경           롤러 소재·연삭 설비
  L_we  롤러 길이           동상

**한계값은 아직 미지정이다.** `LIMITS` 에 값을 넣고 다시 돌리면 개별 효과
표와 통과 설계 표가 문서에 채워진다. 값이 하나도 없으면 분포 표만 갱신하고
나머지는 자리표로 남긴다 — 근거 없는 숫자를 문서에 남기지 않기 위함이다.

`z1 ≥ 1.5` 프론트는 대상에서 뺀다 — 같은 베어링 질량대에서 총질량이 약
10 t 무거워 전 구간이 `z1 ≥ 1.0` 프론트에 지배된다(§6-11.5a·b).

산출: 문서 §6-11.6 의 표 3개
"""
import csv
import io
import os
import re

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "부록6_NSGA", "S3_본최적화", "s3_pareto.csv")
OUT = os.path.join(HERE, "부록6_NSGA", "S3_본최적화", "s3_mfg.csv")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
SUBSET = "z1>=1.0"

# ── 제작 상한 [mm] — **사용자 지정 대기** ────────────────────────────
# 값을 넣는 순간 그 값으로 문서가 갱신된다. None 은 "제한 없음"이다.
LIMITS = dict(bore_mm=None, D_mm=None, D_we_mm=None, L_we_mm=None)
LABEL = dict(bore_mm="d (보어 = 샤프트 외경)", D_mm="D (베어링 외경)",
             D_we_mm="D_we (롤러 외경)", L_we_mm="L_we (롤러 길이)")

MARK = {"dist": "<!-- MFG:DIST -->", "each": "<!-- MFG:EACH -->",
        "pass": "<!-- MFG:PASS -->"}
HDR = ("| # | D_pw [mm] | d [mm] | D [mm] | T [mm] | B [mm] | C [mm] | "
       "α [°] | D_we [mm] | L_we [mm] | 세장비 | z1 [m] | z2 [m] | Z [개] | "
       "L_eff [m] | **베어링** [t] | 샤프트 [t] | **합계** [t] | σ_max [MPa] |")
SEP = "|--:" * 18 + "|--:|"
TODO = "*(제작 상한이 지정되면 스크립트가 채운다 — `select_mfg.py` 의 `LIMITS`)*"


def load():
    with open(SRC, encoding="utf-8-sig") as f:
        return [r for r in csv.DictReader(f) if r["subset"] == SUBSET]


def row(r):
    f = lambda k: float(r[k])                                   # noqa: E731
    mb, ms = f("mass_brg_kg") / 1000, f("mass_shaft_kg") / 1000
    return (f"| {r['rank_pareto']} | {f('D_pw_mm'):,.0f} | {f('bore_mm'):,.0f} | "
            f"{f('D_mm'):,.0f} | {f('T_mm'):,.0f} | {f('B_mm'):,.0f} | "
            f"{f('C_mm'):,.0f} | {f('alpha'):.0f} | {f('D_we_mm'):.1f} | "
            f"{f('L_we_mm'):.1f} | {f('L_we_mm')/f('D_we_mm'):.3f} | "
            f"{f('z1'):.1f} | {f('z2'):.1f} | {int(float(r['Z']))} | "
            f"{f('L_eff_m'):.3f} | **{mb:.2f}** | {ms:.1f} | "
            f"**{2*mb+ms:.1f}** | {f('sigma_max_MPa'):,.1f} |")


def dist_table(F):
    """프론트 치수 분포 — 한계값을 고를 때 쓰는 근거 자료"""
    out = ["| 변수 | 최소 | p25 | 중앙 | p75 | 최대 | 지정 상한 |",
           "|---|--:|--:|--:|--:|--:|--:|"]
    for k, nm in LABEL.items():
        v = np.array([float(r[k]) for r in F])
        q = np.percentile(v, [0, 25, 50, 75, 100])
        lim = f"**{LIMITS[k]:,.0f}**" if LIMITS[k] else "*미지정*"
        out.append(f"| {nm} | " + " | ".join(f"{x:,.1f}" for x in q)
                   + f" | {lim} |")
    return "\n".join(out)


def each_table(F):
    """제약을 하나씩 걸었을 때의 생존 — 어느 제약이 병목인지 본다"""
    if not any(LIMITS.values()):
        return TODO
    out = ["| 제약 | 상한 | 통과 | 탈락 | 통과분 최소 베어링 [t] | "
           "통과분 최소 총질량 [t] |", "|---|--:|--:|--:|--:|--:|"]
    for k, nm in LABEL.items():
        if LIMITS[k] is None:
            out.append(f"| {nm} | *제한 없음* | {len(F)} | 0 | — | — |")
            continue
        ok = [r for r in F if float(r[k]) <= LIMITS[k]]
        mb = min((float(r["mass_brg_kg"]) for r in ok), default=None)
        mt = min((float(r["mass_total_kg"]) for r in ok), default=None)
        out.append(f"| {nm} | {LIMITS[k]:,.0f} | **{len(ok)}** | "
                   f"{len(F)-len(ok)} | "
                   f"{mb/1e3:.2f} | {mt/1e3:.1f} |" if ok else
                   f"| {nm} | {LIMITS[k]:,.0f} | **0** | {len(F)} | — | — |")
    return "\n".join(out)


def pass_table(F):
    """네 제약 동시(AND) 통과 설계"""
    if not any(LIMITS.values()):
        return TODO
    ok = [r for r in F
          if all(LIMITS[k] is None or float(r[k]) <= LIMITS[k]
                 for k in LIMITS)]
    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        if ok:
            w = csv.DictWriter(f, fieldnames=list(ok[0]))
            w.writeheader()
            w.writerows(ok)
    if not ok:
        return ("**통과 설계가 없다.** 상한을 완화하거나, 프론트 밖에서 "
                "제약을 만족하는 설계를 다시 찾아야 한다(재최적화).")
    return "\n".join([HDR, SEP] + [row(r) for r in ok])


def main():
    F = load()
    blocks = {"dist": dist_table(F), "each": each_table(F),
              "pass": pass_table(F)}
    s = io.open(DOC, encoding="utf-8").read()
    for key, mark in MARK.items():
        # 마커 다음 줄부터 다음 빈 줄+마커/제목 전까지를 교체한다
        pat = re.compile(re.escape(mark) + r"\n(?:.*?)(?=\n\n)", re.S)
        if not pat.search(s):
            raise RuntimeError(f"{mark} 자리표를 찾지 못했다")
        s = pat.sub(lambda m: mark + "\n" + blocks[key], s, count=1)
    io.open(DOC, "w", encoding="utf-8").write(s)
    n_pass = blocks["pass"].count("\n| ") if any(LIMITS.values()) else 0
    print(f"[§6-11.6] 프론트 {len(F)}건 · 상한 "
          + " · ".join(f"{k}={v}" for k, v in LIMITS.items())
          + (f" → 통과 {n_pass}건" if any(LIMITS.values()) else " → 자리표 유지"))


if __name__ == "__main__":
    main()
