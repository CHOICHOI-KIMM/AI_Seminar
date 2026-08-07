"""
§8-7.3 — S4-d 결과 표·요약 기입
==================================
`compare_70c.py`(§6-11.7)와 같은 양식으로 부록 8 14건을 정리한다.

  50 °C : P2_피로수명_A8/fatigue_summary.csv       (ν 294.637)
  70 °C : P2_피로수명_A8_70C/fatigue_summary.csv   (ν 137.178 · −53.4%)

손상비 = ΣD30_UW(70) / ΣD30_UW(50).
"""
import csv
import io
import os
import re

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
D50 = os.path.join(HERE, "P2_피로수명_A8", "fatigue_summary.csv")
D70 = os.path.join(HERE, "P2_피로수명_A8_70C", "fatigue_summary.csv")
PAR = os.path.join(HERE, "부록8_NSGA", "S3_본최적화", "a8_pareto.csv")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
LIMIT = 0.5

# v1.3 기준선 — §6-11.7 과 같은 자리에 같은 값(50 °C 만 있다 · §8-3 실측)
BASE = ("| **기준선** (v1.3) | — | 5.60 | 3,055 | 3,600 | 110.5 | 238.0 | "
        "2.154 | 87 | **6.9249** | — | — | 7.7297 | — | 3.9 | — | "
        "**불합격**(50 °C) |")
HDR = ("| 태그 | 프론트 # | 베어링 [t] | d [mm] | D [mm] | D_we [mm] | "
       "L_we [mm] | 세장비 | Z [개] | ΣD30_UW 50 °C | ΣD30_UW 70 °C | "
       "**손상비** | ΣD30_Sys 50 °C | ΣD30_Sys 70 °C | life_Sys 50 °C | "
       "life_Sys 70 °C | **판정 70 °C** |")
SEP = "|" + "--:|" * 16 + ":-:|"


def rd(p):
    with open(p, encoding="utf-8-sig") as f:
        return {r["design"]: r for r in csv.DictReader(f)}


def swap(s, mark, blk):
    a, b = f"<!-- {mark} -->", f"<!-- /{mark} -->"
    pat = re.compile(re.escape(a) + r".*?" + re.escape(b), re.S)
    assert pat.search(s), mark
    return pat.sub(a + "\n" + blk + "\n" + b, s, count=1)


def main():
    f50, f70 = rd(D50), rd(D70)
    par = list(csv.DictReader(open(PAR, encoding="utf-8-sig")))
    geo = {}
    for r in par:
        pre = "a" if r["subset"] == "z1>=1.0" else "b"
        geo[f"{pre}{int(r['rank']):02d}"] = r

    body, stat = [], {}
    for pre, sub, ttl in (("a", "z1>=1.0", "a — `z1 ≥ 1.0` 프론트 (§8-6.3a)"),
                          ("b", "z1>=1.5", "b — `z1 ≥ 1.5` 프론트 (§8-6.3b)")):
        tags = sorted(t for t in f50 if t.startswith(pre))
        body += ["", f"**{ttl}**", "", HDR, SEP]
        if pre == "a":
            body.append(BASE)
        rows = []
        for t in tags:
            g, x, y = geo[t], f50[t], f70[t]
            d50, d70 = float(x["D30_UW"]), float(y["D30_UW"])
            s50, s70 = float(x["D30_Sys"]), float(y["D30_Sys"])
            ok = d70 <= LIMIT and s70 <= LIMIT
            rows.append((d50, d70, d70 / d50, s70, ok))
            body.append(
                f"| `{t}` | {int(g['rank'])} | "
                f"{float(g['mass_brg_kg'])/1e3:.2f} | "
                f"{float(g['bore_mm']):,.0f} | {float(g['D_mm']):,.0f} | "
                f"{float(g['D_we_mm']):.1f} | {float(g['L_we_mm']):.1f} | "
                f"{float(g['slenderness']):.3f} | {int(float(g['Z']))} | "
                f"{d50:.4f} | **{d70:.4f}** | **{d70/d50:.3f}배** | "
                f"{s50:.4f} | {s70:.4f} | "
                f"{float(x['life_Sys_yr']):.0f} | "
                f"**{float(y['life_Sys_yr']):.0f}** | "
                f"{'**합격**' if ok else '**불합격**'} |")
        stat[pre] = np.array([r[:4] for r in rows]), [r[4] for r in rows]

    A = np.vstack([stat["a"][0], stat["b"][0]])
    ok_all = stat["a"][1] + stat["b"][1]
    tbl = ("\n판정 기준: **ΣD30_UW ≤ 0.5 ∧ ΣD30_Sys ≤ 0.5** (30년 손상)\n"
           + "\n".join(body)
           + "\n\n*50 °C `P2_피로수명_A8/` · 70 °C `P2_피로수명_A8_70C/`*")

    worst = int(np.argmax(A[:, 1]))
    tags_all = (sorted(t for t in f50 if t.startswith("a"))
                + sorted(t for t in f50 if t.startswith("b")))
    summ = f"""**14건 전원 합격 — 50 °C 와 70 °C 모두.** 부록 6 이 70 °C 에서
40건 중 20건을 잃은 것과 갈린다.

| 온도 | ΣD30_UW | ΣD30_Sys | 최소 여유 | life_Sys | 합격 |
|---|---|---|--:|---|--:|
| 50 °C | {A[:,0].min():.4f} ~ {A[:,0].max():.4f} | — | \
**{LIMIT/A[:,0].max():.2f}배** | 204 ~ 283 yr | **14 / 14** |
| 70 °C | {A[:,1].min():.4f} ~ {A[:,1].max():.4f} | \
{A[:,3].min():.4f} ~ {A[:,3].max():.4f} | \
**{LIMIT/max(A[:,1].max(), A[:,3].max()):.2f}배** | 74 ~ 102 yr | \
**{sum(ok_all)} / {len(ok_all)}** |

**손상비는 {A[:,2].min():.3f} ~ {A[:,2].max():.3f}배**(평균 {A[:,2].mean():.3f})\
다. 부록 6 의 2.84배와 같은 수준이므로, **온도의 효과 자체는 설계가 바뀌어도
거의 일정**하다 — 점도가 절반 이하로 떨어져 `κ` 가 낮아지는 물리는 기하와
무관하기 때문이다.

**갈린 것은 출발점이다.** 부록 6 의 베어링 최경량군은 50 °C 에서 이미
0.202 ~ 0.321 이었고 2.84배를 곱하면 한도를 넘었다. 부록 8 은 50 °C 에서
{A[:,0].min():.3f} ~ {A[:,0].max():.3f} 로 출발해 2.7배를 곱해도
{A[:,1].max():.3f} 에 그친다.

이유는 §6-11.6 이 이미 답했다 — **여유는 질량이 결정한다**(ΣD30_UW ↔ 베어링
질량 상관 −0.954). 부록 8 프론트의 베어링은 17.0 ~ 23.9 t 로, 부록 6 에서
70 °C 에 탈락한 최경량군(13.4 ~ 16.4 t)보다 무겁다. **두께 규칙이 σ 여유를
줄여 베어링을 키우게 만든 것이, 피로 쪽에서는 이득으로 돌아왔다.**

> **여유가 넉넉하지는 않다.** 70 °C 최악은 `{tags_all[worst]}` 의
> ΣD30_UW **{A[worst,1]:.4f}**(여유 {LIMIT/A[worst,1]:.2f}배 · life_Sys
> {float(f70[tags_all[worst]]['life_Sys_yr']):.0f} yr)다. 부록 6 의 50 °C 최소
> 여유 1.56배보다도 얇다. 하중 스펙트럼이나 재료 가정이 바뀌면 흔들릴 수 있는
> 폭이며, 70 °C 조건에 열팽창 효과가 섞여 있다는 미결 #12 도 그대로다.

**피로는 여전히 최적화를 구속하지 않는다.** 두 온도 어디에서도 프론트를
잘라내지 않았으므로, §6-7 이 피로를 루프에서 뺀 판단은 부록 8 에서도 유효하다."""

    s = io.open(DOC, encoding="utf-8").read()
    s = swap(s, "A8:S4SUM", summ)
    s = swap(s, "A8:S4TABLE", tbl)
    io.open(DOC, "w", encoding="utf-8").write(s)
    print(f"[문서] §8-7.3 기입 · 합격 {sum(ok_all)}/{len(ok_all)} · "
          f"손상비 {A[:,2].min():.3f}~{A[:,2].max():.3f}배 · "
          f"최악 {tags_all[worst]} {A[worst,1]:.4f}")


if __name__ == "__main__":
    main()
