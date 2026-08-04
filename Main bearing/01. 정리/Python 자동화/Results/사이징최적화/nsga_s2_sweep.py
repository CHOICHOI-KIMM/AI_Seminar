"""
부록 6 S2 — 파라미터 스윕 (MASTA 0회)
=======================================
`nsga_s2_replay.py` 가 캐시 주입으로 MASTA 호출을 0 으로 만들었으므로,
(개체수 · 세대수 · 시드) 를 자유롭게 훑어 §6-8 파라미터를 **실증으로** 고를 수 있다.

평가 척도
  재현율   전수 파레토 10건 중 정확히 일치한 개수
  고유평가 서로 다른 설계점 수 — 실제 수행 시 MASTA 호출 횟수와 같다
  전수대비 8,700 (P1 Phase 3 실제 평가수) 대비 비율

산출: 부록6_NSGA/S2_재현시험/s2_sweep.csv
"""
import csv
import os
import sys

import numpy as np
from pymoo.algorithms.moo.nsga2 import NSGA2, RankAndCrowdingSurvival
from pymoo.core.mixed import MixedVariableGA, MixedVariableMating
from pymoo.optimize import minimize

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import nsga_eval as ne              # noqa: E402
import nsga_s2_replay as R          # noqa: E402

CONFIGS = [(100, 60), (100, 150), (150, 100), (200, 100),
           (224, 150), (300, 150)]
SEEDS = (1, 2, 3)
EXHAUSTIVE_EVALS = 8700


class Counted(R.Sizing):
    """실제 MASTA 를 부르는 고유 설계점 수를 센다.

    해석식 제약(z1<z2 · 세장비 · C2·C4~C9·C12)을 **전부** 통과한 점만
    센다 — 하나라도 걸리면 MASTA 를 부르지 않기 때문이다. 세장비만 보고
    세면 나머지 제약 탈락분을 과대계상한다(260804 정정).
    """

    def __init__(self, ev):
        super().__init__(ev)
        self.seen = set()
        self.n_eval = 0

    def _evaluate(self, x, out, *a, **kw):
        self.n_eval += 1
        super()._evaluate(x, out, *a, **kw)
        if out["F"][0] < 1e5:              # MASTA 를 실제로 거친 점
            self.seen.add(ne.key_of(x["z1"], x["z2"], x["D_pw"],
                                    x["alpha"], x["D_we"], x["L_we"]))


def one(ev, pop, gen, seed):
    prob = Counted(ev)
    algo = NSGA2(pop_size=pop,
                 sampling=MixedVariableGA(pop_size=pop).initialization.sampling,
                 mating=MixedVariableMating(eliminate_duplicates=None),
                 survival=RankAndCrowdingSurvival(),
                 eliminate_duplicates=None)
    res = minimize(prob, algo, ("n_gen", gen), seed=seed, verbose=False)
    F = np.atleast_2d(res.F)
    ex = np.array([[float(r["mass_brg_kg"]) / 1000,
                    float(r["mass_total_kg"]) / 1000]
                   for r in R.exhaustive_front()])
    hit = sum(1 for e in ex if np.abs(F - e).sum(axis=1).min() < 1e-6)
    return hit, len(ex), len(F), len(prob.seen), prob.n_eval


def main():
    ev = ne.Evaluator(R.OUT, integerize=False, verbose=False)
    n = R.seed_cache(ev)
    print(f"[스윕] 캐시 {n:,}점 주입 · MASTA 0회 · 전수 실평가 {EXHAUSTIVE_EVALS:,}\n",
          flush=True)
    print("   개체 x 세대   예산    재현율(시드1/2/3)   평균   고유평가   전수대비")
    rows = []
    for pop, gen in CONFIGS:
        hs, us = [], []
        evs = []
        for sd in SEEDS:
            h, ne_x, nf, uq, nv = one(ev, pop, gen, sd)
            hs.append(h); us.append(uq); evs.append(nv)
        avg = sum(hs) / len(hs)
        uavg = sum(us) / len(us)
        rows.append(dict(pop=pop, gen=gen, budget=int(sum(evs)/len(evs)),
                         hits="/".join(map(str, hs)), avg=round(avg, 2),
                         of=ne_x, uniq=int(uavg),
                         vs_exh=round(100 * uavg / EXHAUSTIVE_EVALS, 1)))
        print(f"  {pop:4} x {gen:4}  {int(sum(evs)/len(evs)):6,}   {hs[0]}/{hs[1]}/{hs[2]} of {ne_x}"
              f"      {avg:4.2f}   {int(uavg):7,}    {100*uavg/EXHAUSTIVE_EVALS:5.1f}%",
              flush=True)
    ev.close()
    p = os.path.join(R.OUT, "s2_sweep.csv")
    with open(p, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader(); w.writerows(rows)
    print(f"\n[저장] {os.path.basename(p)}")


if __name__ == "__main__":
    main()
