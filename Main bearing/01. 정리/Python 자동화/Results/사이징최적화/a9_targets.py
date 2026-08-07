"""
S4-d(부록 9) Step 0 — 피로 검토 대상 3건 목록 (§9-10)
========================================================
§9-9.2.1 의 세 설계를 그대로 대상으로 삼는다.

  c50   `D` ≤ 5,000 의 최소 베어링   (D 4,975 · 베어링 16.69 t)
  c45   `D` ≤ 4,500 의 최소 베어링   (D 4,500 · 베어링 20.52 t)
  cmin  프론트가 도달한 최소 외경     (D 4,051 · 베어링 27.59 t)

**`L_we_mm` 열에는 롤러 전장 `L_w` 를 넣는다** — 하류 스크립트가 이 열을
`roller_length` 로 주입하기 때문이다(§8-2.1).

산출: P2_피로수명_A9/p2f_targets.csv
"""
import csv
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import a8_eval  # noqa: E402

SRC = os.path.join(HERE, "부록9_NSGA", "S3_본최적화", "a9_pareto.csv")
DIR = os.path.join(HERE, "P2_피로수명_A9")
OUT = os.path.join(DIR, "p2f_targets.csv")
CAPS = ((5000, "c50"), (4500, "c45"), (None, "cmin"))

COLS = ["rank_mass", "set", "rank", "z1", "z2", "D_pw_mm", "alpha", "D_we_mm",
        "L_we_mm", "slenderness", "Z", "bore_mm", "D_mm", "T_mm", "B_mm",
        "C_mm", "L_eff_m", "mass_brg_kg", "mass_shaft_kg", "mass_total_kg",
        "sigma_max_MPa"]


def main():
    P = list(csv.DictReader(open(SRC, encoding="utf-8-sig")))
    os.makedirs(DIR, exist_ok=True)

    rows, seen = [], set()
    for cap, tag in CAPS:
        if cap is None:
            r = min(P, key=lambda x: float(x["D_mm"]))
        else:
            cand = [x for x in P if float(x["D_mm"]) <= cap + 1e-6]
            r = min(cand, key=lambda x: (float(x["mass_brg_t"]),
                                         float(x["D_mm"])))
        if r["key"] in seen:
            print(f"  [건너뜀] {tag} — 앞 항목과 같은 설계다")
            continue
        seen.add(r["key"])
        d = {k: r[k] for k in COLS[3:] if k != "L_we_mm"}
        d["L_we_mm"] = r["L_w_mm"]              # 롤러 전장 (§8-2.1)
        rows.append(dict(rank_mass=tag, set="a", rank=int(r["rank"]), **d))

    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=COLS)
        w.writeheader()
        w.writerows(rows)
    print(f"[대상] {len(rows)}건 → {OUT}")
    for r in rows:
        d_ = float(r["bore_mm"])
        th = (d_ - a8_eval.shaft_id(d_ / 1e3) * 1e3) / 2
        print(f"  {r['rank_mass']:5} #{r['rank']:2}  D {float(r['D_mm']):,.0f}"
              f" · d {d_:,.0f} · t {th:.1f} · D_we {float(r['D_we_mm']):.1f}"
              f" · L_w {float(r['L_we_mm']):.1f} · Z {int(float(r['Z']))}"
              f" · 베어링 {float(r['mass_brg_kg'])/1e3:.2f} t"
              f" · σ {float(r['sigma_max_MPa']):,.1f}")


if __name__ == "__main__":
    main()
