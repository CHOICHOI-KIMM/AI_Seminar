"""
§9-10 표 갱신기 — 설계 1건이 끝날 때마다 호출된다
====================================================
`run_p2_fatigue.py` 가 phase 8·9 에서 설계 하나를 마칠 때마다 부른다. 그때까지
나온 `fatigue_summary.csv` 만 읽어 §9-10 마커를 다시 쓴다 — **끝난 것만 채우고
나머지는 `—`** 로 두므로 진행 중에도 문서가 일관된다.

양식은 §8-7.3 을 계승한다(50/70 병기 · 손상비 · 판정).
"""
import csv
import io
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
D50 = os.path.join(HERE, "P2_피로수명_A9", "fatigue_summary.csv")
D70 = os.path.join(HERE, "P2_피로수명_A9_70C", "fatigue_summary.csv")
TGT = os.path.join(HERE, "P2_피로수명_A9", "p2f_targets.csv")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
MARK = "A9:S4"
LIMIT = 0.5
NAME = {"c50": "`D` ≤ 5,000", "c45": "`D` ≤ 4,500", "cmin": "도달 하한"}


def rd(p, key="design"):
    if not os.path.isfile(p):
        return {}
    with open(p, encoding="utf-8-sig") as f:
        return {r[key]: r for r in csv.DictReader(f)}


def main():
    import a8_eval
    tg = rd(TGT, "rank_mass")
    f50, f70 = rd(D50), rd(D70)

    body = ["", "판정 기준: **ΣD30_UW ≤ 0.5 ∧ ΣD30_Sys ≤ 0.5** (30년 손상)", "",
            "| 태그 | `D` [mm] | 베어링 [t] | d [mm] | t [mm] | D_we [mm] | "
            "L_we [mm] | Z | ΣD30_UW 50 °C | ΣD30_UW 70 °C | **손상비** | "
            "ΣD30_Sys 70 °C | life_Sys 50 °C | life_Sys 70 °C | "
            "**판정 70 °C** |",
            "|" + "--:|" * 14 + ":-:|"]
    done = 0
    for tag in ("cmin", "c45", "c50"):
        t = tg.get(tag)
        if t is None:
            continue
        d_ = float(t["bore_mm"])
        th = (d_ - a8_eval.shaft_id(d_ / 1e3) * 1e3) / 2
        head = (f"| `{tag}` ({NAME[tag]}) | {float(t['D_mm']):,.0f} | "
                f"**{float(t['mass_brg_kg'])/1e3:.2f}** | {d_:,.0f} | "
                f"{th:.1f} | {float(t['D_we_mm']):.1f} | "
                f"{float(t['L_we_mm'])-8.6:.1f} | {int(float(t['Z']))} |")
        a, b = f50.get(tag), f70.get(tag)
        if a is None:
            body.append(head + " *대기* | — | — | — | — | — | — |")
            continue
        d50 = float(a["D30_UW"])
        if b is None:
            body.append(head + f" {d50:.4f} | *진행 중* | — | — | "
                        f"{float(a['life_Sys_yr']):.0f} | — | — |")
            done += 1
            continue
        d70, s70 = float(b["D30_UW"]), float(b["D30_Sys"])
        ok = d70 <= LIMIT and s70 <= LIMIT
        body.append(head + f" {d50:.4f} | **{d70:.4f}** | "
                    f"**{d70/d50:.3f}배** | {s70:.4f} | "
                    f"{float(a['life_Sys_yr']):.0f} | "
                    f"**{float(b['life_Sys_yr']):.0f}** | "
                    f"{'**합격**' if ok else '**불합격**'} |")
        done += 1

    body += ["", f"*50 °C {len(f50)}/3 · 70 °C {len(f70)}/3 완료 · "
             "설계 1건이 끝날 때마다 갱신된다 · `t` 는 두께 규칙이 만든 "
             "샤프트 벽두께*"]

    s = io.open(DOC, encoding="utf-8").read()
    a_, b_ = f"<!-- {MARK} -->", f"<!-- /{MARK} -->"
    pat = re.compile(re.escape(a_) + r".*?" + re.escape(b_), re.S)
    if pat.search(s):
        io.open(DOC, "w", encoding="utf-8").write(
            pat.sub(a_ + "\n" + "\n".join(body) + "\n" + b_, s, count=1))
    return len(f50), len(f70)


if __name__ == "__main__":
    import sys
    sys.path.insert(0, HERE)
    print(main())
