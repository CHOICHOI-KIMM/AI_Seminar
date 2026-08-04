"""
S4 Step 0 — 피로 검토 대상 40건 목록 작성
============================================
§6-11.5a·b 의 네 표에 실린 설계를 그대로 대상으로 삼는다.

  a01 ~ a10   `z1 ≥ 1.0` 프론트 #1 ~ #10   (베어링 최경량)
  a55 ~ a64   `z1 ≥ 1.0` 프론트 #55 ~ #64  (총질량 최경량)
  b01 ~ b10   `z1 ≥ 1.5` 프론트 #1 ~ #10   (베어링 최경량)
  b32 ~ b41   `z1 ≥ 1.5` 프론트 #32 ~ #41  (총질량 최경량)

태그의 숫자는 **프론트 순번**이다 — 표·그림·`s3_pareto.csv` 와 그대로 대조된다.

열 구성은 `p2c_targets.csv`(Phase 3)와 같게 맞춘다. `probe_p2_constants.py 4`
가 이 파일을 읽어 MASTA 로 `a`·`Y1`·`e`·`C`·`C_u` 를 실측한다.

산출: P2_피로수명_S4/p2d_targets.csv
"""
import csv
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "부록6_NSGA", "S3_본최적화", "s3_pareto.csv")
DIR = os.path.join(HERE, "P2_피로수명_S4")
OUT = os.path.join(DIR, "p2d_targets.csv")
TOP = 10

COLS = ["rank_mass", "set", "rank", "z1", "z2", "D_pw_mm", "alpha", "D_we_mm",
        "L_we_mm", "slenderness", "Z", "bore_mm", "D_mm", "T_mm", "B_mm",
        "C_mm", "L_eff_m", "mass_brg_kg", "mass_shaft_kg", "mass_total_kg",
        "sigma_max_MPa"]


def main():
    with open(SRC, encoding="utf-8-sig") as f:
        P = list(csv.DictReader(f))

    rows = []
    for sub, pre in (("z1>=1.0", "a"), ("z1>=1.5", "b")):
        F = [r for r in P if r["subset"] == sub]
        pick = F[:TOP] + F[-TOP:]          # 앞 10 = 베어링 최경량 · 뒤 10 = 총질량
        for r in pick:
            n = int(r["rank_pareto"])
            rows.append(dict(
                rank_mass=f"{pre}{n:02d}", set=pre, rank=n,
                **{k: r[k] for k in COLS[3:]}))

    os.makedirs(DIR, exist_ok=True)
    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=COLS)
        w.writeheader()
        w.writerows(rows)

    print(f"[S4] 대상 {len(rows)}건 → {os.path.relpath(OUT, HERE)}")
    for pre in ("a", "b"):
        g = [r for r in rows if r["set"] == pre]
        print(f"  {pre}: {len(g)}건 · 프론트 #"
              + ", #".join(str(r["rank"]) for r in g))


if __name__ == "__main__":
    main()
