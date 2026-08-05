"""
§6-11.7 — 베어링 50 °C 대 70 °C 피로 대조
============================================
같은 40건을 두 온도에서 돌린 결과를 설계별로 맞대어 §6-11.7 의 자리표
두 곳(`<!-- T70:SUMMARY -->` · `<!-- T70:TABLE -->`)을 채운다.

  50 °C : P2_피로수명_S4/fatigue_summary.csv       (ν 294.637)
  70 °C : P2_피로수명_S4_70C/fatigue_summary.csv   (ν 137.178 · −53.4%)

**손상비 = ΣD30_UW(70) / ΣD30_UW(50)** 가 온도의 효과다. 점도가 절반 이하로
떨어지면 κ 가 같은 비율로 낮아지고 `a_ISO` 가 작아져 손상이 커진다.

70 °C 가 아직 다 끝나지 않았으면 **끝난 것까지만** 대조하고 그 사실을 표에
적는다 — 진행 중에도 중간 확인이 되게 한다.
"""
import csv
import io
import os
import re

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
D50 = os.path.join(HERE, "P2_피로수명_S4", "fatigue_summary.csv")
D70 = os.path.join(HERE, "P2_피로수명_S4_70C", "fatigue_summary.csv")
TGT = os.path.join(HERE, "P2_피로수명_S4", "p2d_targets.csv")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
MARK = {"summary": "<!-- T70:SUMMARY -->", "table": "<!-- T70:TABLE -->"}
LIMIT = 0.5
NU50, NU70 = 294.637, 137.178


def rd(p):
    if not os.path.isfile(p):
        return {}
    with open(p, encoding="utf-8-sig") as f:
        return {r["design"]: r for r in csv.DictReader(f)}


def grp_of(tag):
    n = int(tag[1:])
    return tag[0] + ("-베어링최경량" if n <= 10 else "-총질량최경량")


def main():
    a, b = rd(D50), rd(D70)
    tg = {r["rank_mass"]: r for r in csv.DictReader(
        open(TGT, encoding="utf-8-sig"))}
    keys = [k for k in a if k in b]
    keys.sort(key=lambda t: (t[0], int(t[1:])))
    if not keys:
        raise RuntimeError("70 °C 결과가 아직 없다")

    d50 = np.array([float(a[k]["D30_UW"]) for k in keys])
    d70 = np.array([float(b[k]["D30_UW"]) for k in keys])
    ratio = d70 / d50
    npass = sum(1 for k in keys
                if b[k]["pass_UW"] == "1" and b[k]["pass_Sys"] == "1")
    flip = [k for k in keys
            if (a[k]["pass_UW"] == "1" and a[k]["pass_Sys"] == "1")
            and not (b[k]["pass_UW"] == "1" and b[k]["pass_Sys"] == "1")]

    # ── 요약 ────────────────────────────────────────────────────────
    done = len(b) >= 40
    S = ([] if done else
         [f"> **수행 중 — {len(b)}/40. 아래 값은 잠정이며 완료 시 "
          f"`compare_70c.py` 가 40건으로 덮어쓴다.**", ""])
    S += [f"**{len(keys)}건 대조**"
          + ("" if done else f" (70 °C 완료 {len(b)}/40)")
          + f". 점도 ν {NU50:,.1f} → {NU70:,.1f} mm²/s"
          f" ({100*(NU70-NU50)/NU50:+.1f}%).", "",
         "| 군 | 건수 | ΣD30_UW 50 °C | ΣD30_UW 70 °C | **손상비** | "
         "여유 70 °C | 판정 |", "|---|--:|---|---|--:|--:|:-:|"]
    for g in ("a-베어링최경량", "a-총질량최경량",
              "b-베어링최경량", "b-총질량최경량"):
        idx = [i for i, k in enumerate(keys) if grp_of(k) == g]
        if not idx:
            continue
        x, y, r = d50[idx], d70[idx], ratio[idx]
        ok = all(b[keys[i]]["pass_UW"] == "1" and b[keys[i]]["pass_Sys"] == "1"
                 for i in idx)
        S.append(f"| {g} | {len(idx)} | {x.min():.4f} ~ {x.max():.4f} | "
                 f"{y.min():.4f} ~ {y.max():.4f} | **{r.mean():.2f}배** | "
                 f"{LIMIT/y.max():.2f} ~ {LIMIT/y.min():.2f}배 | "
                 f"{'전원 합격' if ok else '**불합격 있음**'} |")
    S += ["", f"**손상비 {ratio.min():.2f} ~ {ratio.max():.2f}배 "
              f"(평균 {ratio.mean():.2f})** · 70 °C 합격 **{npass}/{len(keys)}**"
              + (f" · 50 → 70 에서 뒤집힌 설계 **{len(flip)}건** "
                 f"({', '.join(flip)})" if flip else " · 판정이 뒤집힌 설계 없음")]
    worst = keys[int(np.argmax(d70))]
    S.append(f" · 최소 여유는 `{worst}` 의 **{LIMIT/d70.max():.2f}배**"
             f"(ΣD30_UW {d70.max():.4f} · life_Sys "
             f"{float(b[worst]['life_Sys_yr']):,.0f} yr).")

    # ── 설계별 표 ──────────────────────────────────────────────────
    T = [f"판정 기준: **ΣD30_UW ≤ {LIMIT} ∧ ΣD30_Sys ≤ {LIMIT}** (30년 손상)",
         "",
         "| 태그 | 프론트 # | 베어링 [t] | d [mm] | D [mm] | D_we [mm] | "
         "L_we [mm] | 세장비 | Z [개] | ΣD30_UW 50 °C | ΣD30_UW 70 °C | "
         "**손상비** | life_Sys 50 °C | life_Sys 70 °C | **판정 70 °C** |",
         "|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|"]
    for k in keys:
        t, x, y = tg[k], a[k], b[k]
        ok = y["pass_UW"] == "1" and y["pass_Sys"] == "1"
        T.append(
            f"| `{k}` | {t['rank']} | {float(t['mass_brg_kg'])/1e3:.2f} | "
            f"{float(t['bore_mm']):,.0f} | {float(t['D_mm']):,.0f} | "
            f"{float(t['D_we_mm']):.1f} | {float(t['L_we_mm']):.1f} | "
            f"{float(t['slenderness']):.3f} | {int(float(t['Z']))} | "
            f"{float(x['D30_UW']):.4f} | "
            f"**{float(y['D30_UW']):.4f}** | "
            f"{float(y['D30_UW'])/float(x['D30_UW']):.2f} | "
            f"{float(x['life_Sys_yr']):,.0f} | {float(y['life_Sys_yr']):,.0f} | "
            f"{'합격' if ok else '**불합격**'} |")

    s = io.open(DOC, encoding="utf-8").read()
    for key, mark in MARK.items():
        # 다음 **마커나 제목**까지를 통째로 교체한다. 첫 빈 줄까지만 잡으면
        # 블록 안에 빈 줄이 생기는 순간(주석 + 표) 뒷부분이 남아 표가
        # 중복된다 — 260805 실제로 발생.
        pat = re.compile(re.escape(mark) + r"\n(?:.*?)(?=\n<!--|\n#{2,6} |\Z)",
                         re.S)
        if not pat.search(s):
            raise RuntimeError(f"{mark} 자리표를 찾지 못했다")
        blk = "\n".join(S if key == "summary" else T)
        s = pat.sub(lambda m: mark + "\n" + blk, s, count=1)
    io.open(DOC, "w", encoding="utf-8").write(s)

    print(f"[§6-11.7] {len(keys)}건 대조 · 손상비 {ratio.min():.2f}~"
          f"{ratio.max():.2f}배(평균 {ratio.mean():.2f}) · "
          f"70 °C 합격 {npass}/{len(keys)}"
          + (f" · 뒤집힘 {len(flip)}건: {', '.join(flip)}" if flip else ""))


if __name__ == "__main__":
    main()
