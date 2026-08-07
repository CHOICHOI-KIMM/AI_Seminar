"""
부록 8 S3-a — 배선 시험 (대용 평가기 · MASTA 0회)
====================================================
부록 6 이 S3-a 에서 한 것과 같은 점검이다(§6-11.1). MASTA 를 결정론적 대용식으로
갈아끼우고 `224 × 4` 를 돌려 **배선만** 확인한다. 기하·제약·정수화·두께 규칙은
실제 코드 그대로이고 σ·질량만 가짜다.

점검 항목
  ① 두께 규칙이 전 격자에서 성립하는가 — 보어 전 구간에서 `ID` 가 실수해를 갖고
     `floor` 후에도 양수 두께인가
  ② 시딩 커버리지 224/224
  ③ `L_we` 가 0.1 mm 격자 · 세장비 밴드 안
  ④ 프론트 CSV 에 2β 열이 실리는가
  ⑤ pymoo 배선 (세대 로그·체크포인트)

산출: 부록8_NSGA/_wiretest/
"""
import csv
import math
import os
import sys

import numpy as np
from pymoo.algorithms.moo.nsga2 import NSGA2, RankAndCrowdingSurvival
from pymoo.core.mixed import MixedVariableMating
from pymoo.optimize import minimize

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import a8_eval                     # noqa: E402
import nsga_eval as ne             # noqa: E402
import nsga_s3_run as S3           # noqa: E402
import sizing_geom as sg           # noqa: E402

S3.ne = a8_eval
OUT = os.path.join(HERE, "부록8_NSGA", "_wiretest")


class Surrogate:
    """MASTA 자리에 끼우는 결정론적 대용식 (`nsga_s3_paramtest` 와 같은 형태).

    두께 규칙이 실제로 적용되는지 보려면 **샤프트 내경이 질량·응력에 반영**
    되어야 하므로, `a8_eval.shaft_id` 를 그대로 불러 쓴다.
    """

    FIELDS = a8_eval.Evaluator.FIELDS

    def __init__(self):
        self.cache, self.n_masta, self.n_hit = {}, 0, 0

    def evaluate(self, pts):
        out = []
        for p in pts:
            k = ne.key_of(*p)
            if k in self.cache:
                self.n_hit += 1
                out.append(self.cache[k])
                continue
            self.n_masta += 1
            z1, z2, D_pw, al, D_we, L_we = p
            g = ne.geom(D_pw, al, D_we, L_we, True)
            idm = a8_eval.shaft_id(g["bore"])
            vol = (np.pi * (g["outer_diameter"] ** 2 - g["bore"] ** 2) / 4
                   * g["width"])
            mb = vol * 7850 * 0.55
            ms = (np.pi * (g["bore"] ** 2 - idm ** 2) / 4
                  * (z2 + sg.SHAFT_TAIL) * 7850)
            # 얇을수록 σ 가 오르는 경향을 흉내 낸다(§7-6.7.6 실측 방향)
            t = (g["bore"] - idm) / 2
            sig = (2600.0 * (0.11 / D_we) ** 0.5 * (0.238 / L_we) ** 0.25
                   * (0.1 / t) ** 0.08)
            r = dict(key=k, slenderness=round(L_we / D_we, 4),
                     Z=g["number_of_elements"], L_w_mm=round(L_we * 1e3, 2),
                     L_we_mm=round(L_we * 1e3 - 8.6, 2),
                     D_we_mm=round(D_we * 1e3, 2),
                     bore_mm=round(g["bore"] * 1e3, 1),
                     D_mm=round(g["outer_diameter"] * 1e3, 1),
                     T_mm=round(g["width"] * 1e3, 1),
                     B_mm=round(g["inner_ring_width"] * 1e3, 1),
                     C_mm=round(g["outer_ring_width"] * 1e3, 1),
                     L_eff_m=None, mass_brg_kg=round(mb, 1),
                     mass_shaft_kg=round(ms, 1),
                     mass_total_kg=round(2 * mb + ms, 1),
                     sigma_max_MPa=round(sig, 1),
                     shaft_ID_mm=round(idm * 1e3, 1),
                     taper_2beta_deg=1.2, cone_angle_deg=17.8)
            self.cache[k] = r
            out.append(r)
        return out

    def close(self):
        pass


def main():
    os.makedirs(OUT, exist_ok=True)
    print("[부록 8 S3-a] 배선 시험 · MASTA 0회")

    # ── ① 두께 규칙이 전 격자에서 성립하는가 ──────────────────────
    bad = []
    for dpw in range(S3.DPW_LO, S3.DPW_HI + 1, 5):
        for dwe_i in (S3.DWE_LO, (S3.DWE_LO + S3.DWE_HI) // 2, S3.DWE_HI):
            for al in (S3.AL_OPT[0], S3.AL_OPT[-1]):
                lwe_i = S3.lwe_of(dwe_i, S3.U_MAX // 2)
                g = ne.geom(dpw / 1e3, al, dwe_i / 1e4, lwe_i / 1e4, True)
                idm = a8_eval.shaft_id(g["bore"])
                if idm is None or idm <= 0 or idm >= g["bore"]:
                    bad.append((dpw, dwe_i, al, g["bore"]))
    print(f"  ① 두께 규칙  격자 표본 위반 {len(bad)}건"
          + (f" 예: {bad[:2]}" if bad else " (전 구간 해 존재)"))

    # 보어 범위와 두께 범위
    bores, ths = [], []
    for dpw in range(S3.DPW_LO, S3.DPW_HI + 1, 25):
        for dwe_i in (S3.DWE_LO, S3.DWE_HI):
            lwe_i = S3.lwe_of(dwe_i, S3.U_MAX // 2)
            for al in (S3.AL_OPT[0], S3.AL_OPT[-1]):
                g = ne.geom(dpw / 1e3, al, dwe_i / 1e4, lwe_i / 1e4, True)
                idm = a8_eval.shaft_id(g["bore"])
                bores.append(g["bore"] * 1e3)
                ths.append((g["bore"] - idm) / 2 * 1e3)
    b, t = np.array(bores), np.array(ths)
    cur = b * (1 - sg.ID_OVER_OD) / 2
    print(f"     보어 {b.min():,.0f}~{b.max():,.0f} · 두께 {t.min():.0f}~"
          f"{t.max():.0f} mm (현행 규칙 {cur.min():.0f}~{cur.max():.0f})")
    x = b[np.argmin(np.abs(t - cur))]
    print(f"     두 규칙 교차 보어 ≈ {x:,.0f} mm "
          f"(이보다 작으면 두꺼워지고 크면 얇아진다)")

    # ── ②~⑤ pymoo 배선 ──────────────────────────────────────────
    ev = Surrogate()
    prob = S3.SizingS3(ev)
    algo = NSGA2(pop_size=S3.POP, sampling=S3.CategorySeeding(),
                 mating=MixedVariableMating(eliminate_duplicates=None),
                 survival=RankAndCrowdingSurvival(), eliminate_duplicates=None)
    res = minimize(prob, algo, ("n_gen", 4), seed=1, verbose=False,
                   callback=S3.GenLog(prob, OUT, 4), save_history=False)
    front = S3.dump_front(prob, res.algorithm,
                          os.path.join(OUT, "front.csv"))

    seen = set()
    for xx in S3.CategorySeeding()._do(prob, S3.POP):
        seen.add((xx["z1"], xx["z2"], xx["alpha"]))
    off = [r for r in ev.cache.values()
           if abs(round(float(r["L_w_mm"]) * 10)
                  - float(r["L_w_mm"]) * 10) > 1e-9]
    sl = [float(r["slenderness"]) for r in ev.cache.values()]
    with open(os.path.join(OUT, "front.csv"), encoding="utf-8-sig") as f:
        cols = next(csv.reader(f))

    print(f"  ② 시딩 커버리지  {len(seen)}/{S3.N_CAT}")
    print(f"  ③ L_w 격자 이탈 {len(off)} · 세장비(L_w/D_we) "
          f"{min(sl):.4f}~{max(sl):.4f}")
    need = ("L_w_mm", "L_we_mm", "taper_2beta_deg")
    print(f"  ④ 프론트 열 {'·'.join(need)} "
          f"{'모두 포함' if all(c in cols for c in need) else '누락'}"
          f" · 프론트 {len(front)}건")
    print(f"  ⑤ pymoo 배선  4세대 완주 · 예산 {prob.n_call:,} · "
          f"평가 {ev.n_masta:,} · 캐시 {ev.n_hit} · 탈락 {prob.n_geom_out}")
    ok = (not bad and len(seen) == S3.N_CAT and not off
          and all(c in cols for c in need) and front)
    print(f"\n  판정: {'통과 — S3-b 진행 가능' if ok else '실패'}")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
