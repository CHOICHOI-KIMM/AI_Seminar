"""
부록 6 S3-a2 — 세장비 처리 3안 비교 (대용 평가기 · MASTA 0회)
================================================================
세장비 1.5 ~ 2.5 를 어떻게 다룰지 세 방식을 같은 조건에서 비교한다.

  ⑴ 수리 (clip)      밴드 밖 `L_we` 를 가장 가까운 경계로 끌어당긴다 (초안)
  ⑵ 제약만            밴드 밖이면 탈락시킨다 (전수·S2 와 동일 기준)
  ⑶ 밴드 재매개화      `L_we` 대신 밴드 내 상대위치 `u` 를 뽑는다 (채택안)

**MASTA 를 쓰지 않는다.** σ·질량을 결정론적 대용식으로 대체하고 기하·제약·
정수화는 실제 코드(`nsga_eval.geom` · `sizing_geom.constraints`)를 그대로 쓴다.
따라서 **HV 절대값에는 의미가 없고 세 구성의 상대 비교만 유효**하다. 반면
경계집중률·탈락률·MASTA 호출 비율은 목적함수와 무관한 기하·제약 통계라
실제 수행에서도 같은 값이 나온다.

산출: 부록6_NSGA/S3_파라미터화/paramtest.csv
"""
import csv
import os
import sys

import numpy as np
from pymoo.algorithms.moo.nsga2 import NSGA2, RankAndCrowdingSurvival
from pymoo.core.mixed import MixedVariableMating
from pymoo.core.problem import ElementwiseProblem
from pymoo.core.sampling import Sampling
from pymoo.core.variable import Choice, Integer
from pymoo.indicators.hv import HV
from pymoo.optimize import minimize

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import nsga_eval as ne          # noqa: E402
import nsga_s3_run as S3        # noqa: E402
import sizing_geom as sg        # noqa: E402

OUT = os.path.join(HERE, "부록6_NSGA", "S3_파라미터화")
SEEDS = (1, 2, 3)
GENS = 50


# ── 대용 평가기 ──────────────────────────────────────────────────────
class Surrogate:
    """MASTA 자리에 끼우는 결정론적 대용식.

    링 체적에서 질량을, 롤러 치수에서 응력을 낸다. 실제 거동을 흉내내려는
    것이 아니라 **모든 구성에 같은 지형을 주기 위한** 장치다.
    """

    def __init__(self):
        self.cache = {}
        self.n_masta = 0
        self.n_hit = 0

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
            vol = (np.pi * (g["outer_diameter"] ** 2 - g["bore"] ** 2) / 4
                   * g["width"])
            mb = vol * 7850 * 0.55
            ms = (np.pi * (g["bore"] ** 2 - (g["bore"] * 0.885) ** 2) / 4
                  * (z2 + 0.5) * 7850)
            sig = 2600.0 * (0.11 / D_we) ** 0.5 * (0.238 / L_we) ** 0.25
            r = dict(key=k, slenderness=round(L_we / D_we, 4),
                     Z=g["number_of_elements"], L_we_mm=round(L_we * 1e3, 2),
                     D_we_mm=round(D_we * 1e3, 2),
                     bore_mm=round(g["bore"] * 1e3, 1),
                     D_mm=round(g["outer_diameter"] * 1e3, 1),
                     T_mm=round(g["width"] * 1e3, 1),
                     B_mm=round(g["inner_ring_width"] * 1e3, 1),
                     C_mm=round(g["outer_ring_width"] * 1e3, 1), L_eff_m=None,
                     mass_brg_kg=round(mb, 1), mass_shaft_kg=round(ms, 1),
                     mass_total_kg=round(2 * mb + ms, 1),
                     sigma_max_MPa=round(sig, 1))
            self.cache[k] = r
            out.append(r)
        return out

    def close(self):
        pass


# ── ⑴ 수리 / ⑵ 제약만 — `L_we` 를 직접 뽑는 구성 ─────────────────────
class LweSeeding(Sampling):
    def __init__(self, seed):
        super().__init__()
        self.seed = seed

    def _do(self, problem, n, **kw):
        rng = np.random.default_rng(self.seed)
        cats = [(a, b, c) for a in S3.Z1_OPT for b in S3.Z2_OPT
                for c in S3.AL_OPT]
        rng.shuffle(cats)
        return np.array([dict(z1=cats[i % len(cats)][0],
                              z2=cats[i % len(cats)][1],
                              alpha=cats[i % len(cats)][2],
                              D_pw=int(rng.integers(S3.DPW_LO, S3.DPW_HI + 1)),
                              D_we=int(rng.integers(S3.DWE_LO, S3.DWE_HI + 1)),
                              L_we=int(rng.integers(S3.LWE_LO, S3.LWE_HI + 1)))
                         for i in range(n)], dtype=object)


class LweProblem(ElementwiseProblem):
    """`repair=True` 면 클립, `False` 면 세장비를 제약으로 되돌린다."""

    def __init__(self, ev, repair):
        self.ev, self.repair = ev, repair
        self.n_call = self.n_geom_out = self.n_edge = 0
        super().__init__(
            vars=dict(z1=Choice(options=S3.Z1_OPT),
                      z2=Choice(options=S3.Z2_OPT),
                      alpha=Choice(options=S3.AL_OPT),
                      D_pw=Integer(bounds=(S3.DPW_LO, S3.DPW_HI)),
                      D_we=Integer(bounds=(S3.DWE_LO, S3.DWE_HI)),
                      L_we=Integer(bounds=(S3.LWE_LO, S3.LWE_HI))),
            n_obj=2, n_ieq_constr=2)

    def _evaluate(self, x, out, *a, **kw):
        self.n_call += 1
        lo, hi = S3.band(x["D_we"])
        lwe_i = x["L_we"]
        if self.repair:
            lwe_i = min(max(lwe_i, lo), hi)
            if lwe_i != x["L_we"]:
                self.n_edge += 1
        elif not (lo <= lwe_i <= hi):              # 제약만 → 탈락
            self.n_geom_out += 1
            out["F"] = [1e6, 1e6]
            out["G"] = [1.0, 1.0]
            return
        pt = S3.to_si(x["z1"], x["z2"], x["D_pw"], x["alpha"], x["D_we"],
                      lwe_i)
        _common_eval(self, pt, out)


# ── ⑶ 밴드 재매개화 — `nsga_s3_run` 의 실제 구성 ─────────────────────
def _common_eval(prob, pt, out):
    g = ne.geom(*pt[2:], True)
    try:
        bad = sg.constraints(g, pt[0], pt[1])
    except Exception:
        bad = ["err"]
    if bad:
        prob.n_geom_out += 1
        out["F"] = [1e6, 1e6]
        out["G"] = [float(len(bad)), 1.0]
        return
    r = prob.ev.evaluate([pt])[0]
    s = float(r["sigma_max_MPa"])
    out["F"] = [float(r["mass_brg_kg"]) / 1e3, float(r["mass_total_kg"]) / 1e3]
    out["G"] = [0.0, (s - S3.LIMIT) / S3.LIMIT if s > 0 else 1.0]


CONFIGS = [
    ("수리 (clip)", lambda ev: LweProblem(ev, True), LweSeeding),
    ("제약만", lambda ev: LweProblem(ev, False), LweSeeding),
    ("밴드 재매개화 u", lambda ev: S3.SizingS3(ev), S3.CategorySeeding),
]


def one(make_prob, make_samp, seed):
    ev = Surrogate()
    prob = make_prob(ev)
    algo = NSGA2(pop_size=S3.POP, sampling=make_samp(seed),
                 mating=MixedVariableMating(eliminate_duplicates=None),
                 survival=RankAndCrowdingSurvival(), eliminate_duplicates=None)
    res = minimize(prob, algo, ("n_gen", GENS), seed=seed, verbose=False)
    F = np.atleast_2d(res.F)
    F = F[F[:, 0] < 1e5] if len(F) else F
    hv = float(HV(ref_point=S3.HV_REF)(F)) if len(F) else 0.0
    sl = [float(r["slenderness"]) for r in ev.cache.values()]
    edge = sum(1 for v in sl if v <= 1.502 or v >= 2.498)
    return dict(hv=hv, n_front=len(F), budget=prob.n_call,
                masta=ev.n_masta, geom_out=prob.n_geom_out,
                edge_pct=100 * edge / len(sl) if sl else 0.0,
                sl_min=min(sl) if sl else None, sl_max=max(sl) if sl else None)


def main():
    os.makedirs(OUT, exist_ok=True)
    print(f"[S3-a2] 세장비 3안 × 시드 {len(SEEDS)}개 × {GENS}세대 · MASTA 0회\n")
    print("  구성              MASTA호출   탈락    프론트      HV(평균)     "
          "경계집중   세장비 범위")
    rows = []
    for name, mp, ms in CONFIGS:
        rs = [one(mp, ms, sd) for sd in SEEDS]
        avg = {k: float(np.mean([r[k] for r in rs]))
               for k in ("hv", "n_front", "budget", "masta", "geom_out",
                         "edge_pct")}
        row = dict(config=name, seeds=len(SEEDS), gens=GENS,
                   budget=int(avg["budget"]),
                   masta=int(avg["masta"]),
                   masta_pct=round(100 * avg["masta"] / avg["budget"], 1),
                   geom_out_pct=round(100 * avg["geom_out"] / avg["budget"], 1),
                   n_front=round(avg["n_front"], 1),
                   hv=round(avg["hv"], 1),
                   hv_sd=round(float(np.std([r["hv"] for r in rs])), 1),
                   edge_pct=round(avg["edge_pct"], 1),
                   sl_min=round(min(r["sl_min"] for r in rs), 3),
                   sl_max=round(max(r["sl_max"] for r in rs), 3))
        rows.append(row)
        print(f"  {name:16} {row['masta']:6,}({row['masta_pct']:4.1f}%) "
              f"{row['geom_out_pct']:5.1f}%  {row['n_front']:5.1f}  "
              f"{row['hv']:10,.1f}±{row['hv_sd']:<6,.1f} "
              f"{row['edge_pct']:6.1f}%   {row['sl_min']:.3f}~{row['sl_max']:.3f}",
              flush=True)

    p = os.path.join(OUT, "paramtest.csv")
    with open(p, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)
    print(f"\n[저장] {p}")
    best = max(rows, key=lambda r: r["hv"])
    print(f"  HV 최고 {best['config']} · 경계집중 최저 "
          f"{min(rows, key=lambda r: r['edge_pct'])['config']}")


if __name__ == "__main__":
    main()
