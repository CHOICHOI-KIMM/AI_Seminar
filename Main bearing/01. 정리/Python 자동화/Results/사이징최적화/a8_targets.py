"""
S4-d Step 0 — 부록 8 프론트 14건 목록 작성 (§8-7)
====================================================
`make_s4_targets.py` 와 같은 역할이고 대상만 다르다. 부록 8 프론트는 8·6건뿐이라
부록 6 처럼 앞뒤 10건씩 자르지 않고 **전량**을 싣는다.

  a01 ~ a08   `z1 ≥ 1.0` 프론트 전량
  b01 ~ b06   `z1 ≥ 1.5` 프론트 전량

**`L_we_mm` 열에는 롤러 전장 `L_w` 를 넣는다.** 하류 스크립트가 이 열을
`roller_length` 로 주입하기 때문이다(부록 1~7 의 이름 그대로 · §8-2.1).
유효 길이는 코너 반경 4.3 mm 를 넣으면 MASTA 가 스스로 계산한다.

산출: P2_피로수명_A8/p2e_targets.csv
"""
import csv
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "부록8_NSGA", "S3_본최적화", "a8_pareto.csv")
DIR = os.path.join(HERE, "P2_피로수명_A8")
OUT = os.path.join(DIR, "p2e_targets.csv")

COLS = ["rank_mass", "set", "rank", "z1", "z2", "D_pw_mm", "alpha", "D_we_mm",
        "L_we_mm", "slenderness", "Z", "bore_mm", "D_mm", "T_mm", "B_mm",
        "C_mm", "L_eff_m", "mass_brg_kg", "mass_shaft_kg", "mass_total_kg",
        "sigma_max_MPa"]


def main():
    with open(SRC, encoding="utf-8-sig") as f:
        P = list(csv.DictReader(f))

    os.makedirs(DIR, exist_ok=True)
    rows = []
    for sub, pre in (("z1>=1.0", "a"), ("z1>=1.5", "b")):
        for r in [x for x in P if x["subset"] == sub]:
            n = int(r["rank"])
            d = {k: r[k] for k in COLS[3:] if k != "L_we_mm"}
            d["L_we_mm"] = r["L_w_mm"]          # 롤러 전장 (§8-2.1)
            rows.append(dict(rank_mass=f"{pre}{n:02d}", set=pre, rank=n, **d))

    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=COLS)
        w.writeheader()
        w.writerows(rows)
    print(f"[대상] {len(rows)}건 → {OUT}")
    for r in rows:
        print(f"  {r['rank_mass']}  D_pw {float(r['D_pw_mm']):,.0f} · "
              f"D_we {float(r['D_we_mm']):.1f} · L_w {float(r['L_we_mm']):.1f} "
              f"· z {r['z1']}/{r['z2']} · 베어링 "
              f"{float(r['mass_brg_kg'])/1e3:.2f} t")


if __name__ == "__main__":
    main()
