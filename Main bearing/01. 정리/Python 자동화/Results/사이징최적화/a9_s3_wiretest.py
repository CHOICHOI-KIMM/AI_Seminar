"""
부록 9 S3-a — 배선 시험 (대용 평가기 · MASTA 0회)
====================================================
부록 8 이 S3-a 에서 한 것과 같은 점검이다(§8-4). MASTA 를 결정론적 대용식으로
갈아끼우고 `130 × 4` 를 돌려 **배선만** 확인한다.

점검 항목
  ① `z1` = 1.0 · `z2` = 6.0 으로 고정됐는가
  ② 목적이 `(외경 D, 베어링 1개 질량)` 인가
  ③ α 가 `Integer` 로 15 ~ 30 정수를 내는가
  ④ `D_pw` 가 4,800 까지 나오고 `C6`(D ≤ 5,000)가 걸러 주는가
  ⑤ `C4`·`C5` 가 제거됐는가 (위반 목록에 등장하지 않아야 한다)
  ⑥ 세장비 밴드 · 두께 규칙 · pymoo 배선

산출: 부록9_NSGA/_wiretest/
"""
import csv
import os
import sys

import numpy as np
from pymoo.algorithms.moo.nsga2 import NSGA2, RankAndCrowdingSurvival
from pymoo.core.mixed import MixedVariableMating
from pymoo.optimize import minimize

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import a8_eval                    # noqa: E402
import a9_s3_run as A9            # noqa: E402  (설정 주입)
import nsga_eval as ne            # noqa: E402
import nsga_s3_run as S3          # noqa: E402
import sizing_geom as sg          # noqa: E402

OUT = os.path.join(HERE, "부록9_NSGA", "_wiretest")


class Surrogate:
    """MASTA 자리에 끼우는 결정론적 대용식 (§8-4 와 같은 형태)"""

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
            t = (g["bore"] - idm) / 2
            # **`D_pw` 항이 있어야 상충이 생긴다** — 피치원이 커지면 롤러
            # 하중이 `M/D_pw` 로 줄어 σ 가 내려간다(§8-8.5.2 실측 기전).
            # 이것이 없으면 작은 `D` 와 가벼운 베어링이 동시에 가능해져
            # 프론트가 한 점으로 무너진다.
            sig = (2600.0 * (0.11 / D_we) ** 0.5 * (0.238 / L_we) ** 0.25
                   * (0.1 / t) ** 0.08 * (4.5 / D_pw) ** 1.6)
            r = dict(key=k, z1=z1, z2=z2, alpha=al,
                     D_pw_mm=round(D_pw * 1e3, 1),
                     slenderness=round(L_we / D_we, 4),
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
                     taper_2beta_deg=1.2, cone_angle_deg=17.8)
            self.cache[k] = r
            out.append(r)
        return out

    def close(self):
        pass


def main():
    os.makedirs(OUT, exist_ok=True)
    print(f"[부록 9 S3-a] 배선 시험 · MASTA 0회 · 개체 {S3.POP} · "
          f"범주 {S3.N_CAT}")

    ev = Surrogate()
    prob = S3.SizingS3(ev)
    algo = NSGA2(pop_size=S3.POP, sampling=S3.CategorySeeding(),
                 mating=MixedVariableMating(eliminate_duplicates=None),
                 survival=RankAndCrowdingSurvival(), eliminate_duplicates=None)
    res = minimize(prob, algo, ("n_gen", 4), seed=S3.SEED, verbose=False,
                   callback=S3.GenLog(prob, OUT, 4), save_history=False)
    front = S3.dump_front(prob, res.algorithm, os.path.join(OUT, "front.csv"))

    C = list(ev.cache.values())
    z1s = {float(r["z1"]) for r in C}
    z2s = sorted({float(r["z2"]) for r in C})
    als = [float(r["alpha"]) for r in C]
    dpw = np.array([float(r["D_pw_mm"]) for r in C])
    D = np.array([float(r["D_mm"]) for r in C])
    sl = [float(r["slenderness"]) for r in C]
    off = [r for r in C if abs(round(float(r["L_w_mm"]) * 10)
                               - float(r["L_w_mm"]) * 10) > 1e-9]

    # C4·C5 가 제거됐는지 — 격자 표본에서 위반 목록을 훑는다
    names = set()
    rng = np.random.default_rng(1)
    for _ in range(4000):
        z2 = float(rng.choice(S3.Z2_OPT))
        g = ne.geom(int(rng.integers(S3.DPW_LO, S3.DPW_HI + 1)) / 1e3,
                    int(rng.integers(15, 31)),
                    int(rng.integers(S3.DWE_LO, S3.DWE_HI + 1)) / 1e4,
                    S3.lwe_of(int(rng.integers(S3.DWE_LO, S3.DWE_HI + 1)),
                              int(rng.integers(0, S3.U_MAX + 1))) / 1e4, True)
        names |= {x.split()[0] for x in sg.constraints(g, 1.0, z2)}

    with open(os.path.join(OUT, "front.csv"), encoding="utf-8-sig") as f:
        cols = next(csv.reader(f))
    z1s = {float(r["z1"]) for r in C}

    ok = True
    def chk(tag, cond, msg):
        nonlocal ok
        ok &= bool(cond)
        print(f"  {'O' if cond else 'x'} {tag}  {msg}")

    F = res.algorithm.opt.get("F")
    Dm = np.array([float(r["D_mm"]) for r in C]) / 1e3
    bm = np.array([float(r["mass_brg_kg"]) for r in C]) / 1e3
    chk("①", z1s == {1.0} and z2s == [6.0],
        f"z1 = {sorted(z1s)} · z2 = {z2s}")
    chk("②", F.shape[1] == 2 and F[:, 0].max() <= 5.0 + 1e-9
        and 3.0 < F[:, 0].min() and 0 < F[:, 1].min() < 60 and len(F) > 1,
        f"목적 F1(D) {F[:,0].min():.3f}~{F[:,0].max():.3f} m · "
        f"F2(베어링) {F[:,1].min():.2f}~{F[:,1].max():.2f} t · "
        f"프론트 {len(F)}점")
    chk("③", all(float(a).is_integer() for a in als)
        and 15 <= min(als) and max(als) <= 30,
        f"α {min(als):.0f} ~ {max(als):.0f} · 서로 다른 값 {len(set(als))}종")
    chk("④", dpw.max() > 4500 and D.max() <= 5000 + 1e-6,
        f"D_pw 최대 {dpw.max():,.0f} (>4,500) · D 최대 {D.max():,.0f} (≤5,000)")
    chk("⑤", not (names & {"C4", "C5", "C6"}) or "C6" in names,
        f"활성 제약 {sorted(names) or '없음'}")
    chk("⑥", not off and 1.5 <= min(sl) and max(sl) <= 2.5 and front
        and {"D_mm", "mass_brg_t", "mass_total_t", "sigma_margin"} <= set(cols),
        f"L_w 격자 이탈 {len(off)} · 세장비 {min(sl):.4f}~{max(sl):.4f} · "
        f"프론트 {len(front)}건 · 질량·진단 열 "
        f"{'D_mm' in cols and 'mass_total_t' in cols}")

    print(f"\n  pymoo 배선  4세대 완주 · 예산 {prob.n_call:,} · "
          f"평가 {ev.n_masta:,} · 캐시 {ev.n_hit} · 해석식탈락 "
          f"{prob.n_geom_out:,} ({100*prob.n_geom_out/prob.n_call:.1f}%)")
    print(f"\n  판정: {'통과 — S3-b 진행 가능' if ok else '실패'}")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
