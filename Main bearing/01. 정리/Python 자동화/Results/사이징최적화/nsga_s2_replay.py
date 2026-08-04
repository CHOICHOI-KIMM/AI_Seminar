"""
부록 6 S2 — 재현 시험 (전수 파레토 대조)
==========================================
탐색공간을 **P1 Phase 3 격자로 제한**하고 NSGA-II 를 돌려, 전수 탐색이
찾은 파레토(§8-4.4.4a 10건)를 다시 찾아내는지 본다. 답을 아는 문제에서
알고리즘을 검증해야 S3 의 격자 밖 결과를 신뢰할 수 있다(§6-9).

**MASTA 호출 0회** — `p1_grid.csv` 8,700점을 평가기 캐시에 주입하면
격자 안의 모든 점이 캐시 적중이다. 순수 알고리즘 시험이 된다.

정수화는 끈다(§6-4.3) — 전수 결과와 같은 기준이어야 비교가 성립한다.
**세장비 수리도 끈다.** 전수 탐색은 위반을 수리하지 않고 탈락시켰으므로,
수리를 켜면 다른 문제를 푸는 셈이 되어 프론트 차이의 원인을 가릴 수 없다.
게다가 클립은 격자 밖 L_we 를 만들어 캐시를 무력화한다(260804 실측:
10분간 약 1,400점을 헛되이 MASTA 로 평가). 수리 효과는 S3 에서 본다.
"""
import csv
import json
import os
import sys

import numpy as np
from pymoo.algorithms.moo.nsga2 import NSGA2, RankAndCrowdingSurvival
from pymoo.core.mixed import MixedVariableGA, MixedVariableMating
from pymoo.core.problem import ElementwiseProblem
from pymoo.core.variable import Choice
from pymoo.optimize import minimize

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import nsga_eval as ne   # noqa: E402

OUT = os.path.join(HERE, "부록6_NSGA", "S2_재현시험")
GRID = os.path.join(HERE, "P1_극한응력_Phase3", "p1_grid.csv")
PARETO = os.path.join(HERE, "P1_극한응력_Phase3", "p1_pareto.csv")
LIMIT = 2100.0
# P1 Phase 3 격자 (§8-4.1) — S2 는 이 안에서만 탐색한다
LV = dict(z1=[1.0, 1.5, 2.0, 2.5, 3.0],
          z2=[3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0],
          D_pw=[3.300, 3.600, 3.900, 4.200, 4.500],
          alpha=[15.0, 19.0, 23.0, 27.0, 31.0],
          D_we=[0.110, 0.140, 0.170, 0.200, 0.230],
          L_we=[0.175, 0.250, 0.325, 0.400, 0.475, 0.550])
POP, GEN, SEED = 100, 60, 1


def seed_cache(ev):
    """p1_grid.csv → 평가기 캐시. 격자 안은 MASTA 없이 즉시 응답."""
    n = 0
    with open(GRID, encoding="utf-8-sig") as f:
        for r in csv.DictReader(f):
            k = ne.key_of(float(r["z1"]), float(r["z2"]),
                          float(r["D_pw_mm"]) / 1e3, float(r["alpha"]),
                          float(r["D_we_mm"]) / 1e3, float(r["L_we_mm"]) / 1e3)
            ev.cache[k] = dict(r, key=k)
            n += 1
    return n


class Sizing(ElementwiseProblem):
    """2목적(베어링 질량 ↓ · 총질량 ↓) · 제약 3개"""

    def __init__(self, ev):
        self.ev = ev
        self.miss = 0
        super().__init__(vars={k: Choice(options=v) for k, v in LV.items()},
                         n_obj=2, n_ieq_constr=3)

    def _evaluate(self, x, out, *a, **kw):
        z1, z2 = x["z1"], x["z2"]
        D_we = x["D_we"]
        L_we = x["L_we"]                         # 수리 없음 — 전수와 동일
        pt = (z1, z2, x["D_pw"], x["alpha"], D_we, L_we)

        g1 = z1 - z2 + 1e-9                      # z1 < z2
        sl = L_we / D_we
        g2 = max(1.5 - sl, sl - 2.5)             # 세장비 1.5~2.5
        gg = ne.geom(*pt[2:], False)
        bad = sg_bad(gg, z1, z2)
        if g1 > 0 or g2 > 0 or bad:              # 해석식 위반 → MASTA 생략
            out["F"] = [1e6, 1e6]
            out["G"] = [max(g1, 0.0), max(g2, 0.0) + len(bad), 1.0]
            return
        r = self.ev.evaluate([pt])[0]
        if r["key"] not in self.ev.cache:
            self.miss += 1
        s = float(r["sigma_max_MPa"])
        mb, mt = float(r["mass_brg_kg"]), float(r["mass_total_kg"])
        out["F"] = [mb / 1000.0, mt / 1000.0]
        out["G"] = [0.0, 0.0, (s - LIMIT) / LIMIT if s > 0 else 1.0]


def sg_bad(g, z1, z2):
    import sizing_geom as sg
    try:
        return sg.constraints(g, z1, z2)
    except Exception:
        return ["err"]


def exhaustive_front():
    with open(PARETO, encoding="utf-8-sig") as f:
        return [r for r in csv.DictReader(f) if r["subset"] == "z1>=1.0"]


def main():
    os.makedirs(OUT, exist_ok=True)
    ev = ne.Evaluator(OUT, integerize=False, verbose=False)
    n = seed_cache(ev)
    print(f"[S2] 캐시 주입 {n:,}점 (p1_grid.csv) · 격자 안은 MASTA 0회",
          flush=True)

    prob = Sizing(ev)
    algo = NSGA2(pop_size=POP,
                 sampling=MixedVariableGA(pop_size=POP).initialization.sampling,
                 mating=MixedVariableMating(
                     eliminate_duplicates=None),
                 survival=RankAndCrowdingSurvival(),
                 eliminate_duplicates=None)
    res = minimize(prob, algo, ("n_gen", GEN), seed=SEED, verbose=False,
                   save_history=False)
    ev.close()

    F = np.atleast_2d(res.F)
    X = res.X if isinstance(res.X, np.ndarray) else np.array([res.X])
    order = np.argsort(F[:, 0])
    F, X = F[order], X[order]

    exh = exhaustive_front()
    ex = np.array([[float(r["mass_brg_kg"]) / 1000,
                    float(r["mass_total_kg"]) / 1000] for r in exh])

    print(f"\n[결과] NSGA 프론트 {len(F)}건 · 전수 프론트 {len(ex)}건 · "
          f"MASTA 신규호출 {ev.n_masta} · 캐시적중 {ev.n_hit:,}")
    print(f"  격자 밖 평가(캐시 미스) {prob.miss}")
    print("\n  전수 프론트 재현 대조 (베어링 t / 총 t)")
    print("   #  전수                NSGA 최근접        일치")
    hit = 0
    for i, e in enumerate(ex, 1):
        d = np.abs(F - e).sum(axis=1)
        j = int(np.argmin(d))
        ok = d[j] < 1e-6
        hit += ok
        print(f"  {i:2}  {e[0]:7.3f} {e[1]:8.3f}   {F[j,0]:7.3f} {F[j,1]:8.3f}   "
              f"{'O' if ok else f'x (Δ{d[j]:.3f})'}")
    print(f"\n  재현율 {hit}/{len(ex)} = {100*hit/len(ex):.0f}%")

    with open(os.path.join(OUT, "s2_front.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        w = csv.writer(f)
        w.writerow(["rank", "mass_brg_t", "mass_total_t"] + list(LV))
        for i, (f_, x_) in enumerate(zip(F, X), 1):
            w.writerow([i, round(f_[0], 4), round(f_[1], 4)] +
                       [x_[k] for k in LV])
    json.dump(dict(pop=POP, gen=GEN, seed=SEED, n_front=int(len(F)),
                   n_exhaustive=int(len(ex)), hit=int(hit),
                   masta_calls=int(ev.n_masta), cache_hits=int(ev.n_hit),
                   grid_miss=int(prob.miss)),
              open(os.path.join(OUT, "s2_meta.json"), "w"), indent=1)
    return hit, len(ex)


if __name__ == "__main__":
    main()
