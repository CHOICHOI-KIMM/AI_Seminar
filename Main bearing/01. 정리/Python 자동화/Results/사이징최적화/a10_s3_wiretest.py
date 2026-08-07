"""
부록 10 S3-a — 배선 시험 (대용 평가기 · MASTA 0회)
=====================================================
MASTA 를 결정론적 대용식으로 갈아끼우고 `182 × 4` 를 돌려 배선만 확인한다.

**대용식에 `z2` 항을 넣는다.** 부록 9 의 1차 배선시험은 σ 에 `D_pw` 항이 없어
작은 `D` 와 가벼운 베어링이 동시에 가능해졌고 프론트가 1점으로 무너졌다(§9-7 ▸).
이번에는 `z2` 가 새 교환 차원이므로 같은 함정이 `z2` 에서 생긴다 — 스팬이
짧으면 샤프트가 가벼워 총질량이 좋아지는데, σ 에 벌점이 없으면 `z2` 가 하한에
전부 몰린다. **실제 기전(스팬이 짧으면 베어링 하중이 커진다)을 넣어야** 3목적이
작동하는지 볼 수 있다.

점검 항목
  ① 목적이 3개이고 `(D, 베어링, 총질량)` 인가
  ② `z2` 26수준 전수 시딩
  ③ α `Integer` · `D_pw` 4,800 · `C6`(D ≤ 5,000)
  ④ `C7` 여유 — `z2` 하한 3.5 에서도 성립하는가 (§10-7 ③)
  ⑤ 밴드 · 두께 규칙 · 3D HV · 프론트 덤프

산출: 부록10_NSGA/_wiretest/
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
import a10_s3_run as A10          # noqa: E402  (설정 주입)
import nsga_eval as ne            # noqa: E402
import nsga_s3_run as S3          # noqa: E402
import sizing_geom as sg          # noqa: E402

OUT = os.path.join(HERE, "부록10_NSGA", "_wiretest")


class Surrogate:
    """MASTA 자리에 끼우는 결정론적 대용식"""

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
            # 상충을 만드는 두 항이 반드시 있어야 한다
            #   (4.5/D_pw)^1.6 — 피치원이 크면 롤러 하중이 준다 (§8-8.5.2)
            #   (6.0/z2)^0.9   — 스팬이 짧으면 베어링 하중이 커진다 (§8-6.5.2)
            sig = (2600.0 * (0.11 / D_we) ** 0.5 * (0.238 / L_we) ** 0.25
                   * (0.1 / t) ** 0.08 * (4.5 / D_pw) ** 1.6
                   * (6.0 / z2) ** 0.9)
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
    print(f"[부록 10 S3-a] 배선 시험 · MASTA 0회 · 개체 {S3.POP} · "
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
    F = res.algorithm.opt.get("F")
    seed = A10.Seed10()._do(prob, S3.POP)
    zz = sorted({x["z2"] for x in seed})
    al = [float(r["alpha"]) for r in C]
    dpw = np.array([float(r["D_pw_mm"]) for r in C])
    D = np.array([float(r["D_mm"]) for r in C])
    sl = [float(r["slenderness"]) for r in C]
    off = [r for r in C if abs(round(float(r["L_w_mm"]) * 10)
                               - float(r["L_w_mm"]) * 10) > 1e-9]
    c7 = min((float(r["z2"]) - 1.0) - (float(r["T_mm"]) / 1e3 + 0.1)
             for r in C)
    with open(os.path.join(OUT, "front.csv"), encoding="utf-8-sig") as f:
        cols = next(csv.reader(f))

    ok = True

    def chk(tag, cond, msg):
        nonlocal ok
        ok &= bool(cond)
        print(f"  {'O' if cond else 'x'} {tag}  {msg}")

    chk("①", F.shape[1] == 3 and F[:, 0].max() <= 5.0 + 1e-9 and len(F) > 2,
        f"목적 3개 · D {F[:,0].min():.3f}~{F[:,0].max():.3f} m · "
        f"베어링 {F[:,1].min():.2f}~{F[:,1].max():.2f} t · "
        f"총 {F[:,2].min():.1f}~{F[:,2].max():.1f} t · 프론트 {len(F)}점")
    chk("②", len(zz) == S3.N_CAT,
        f"z2 전수 시딩 {len(zz)}/{S3.N_CAT}수준 {min(zz)} ~ {max(zz)}")
    chk("③", all(float(a).is_integer() for a in al) and dpw.max() > 4500
        and D.max() <= 5000 + 1e-6,
        f"α {min(al):.0f}~{max(al):.0f}({len(set(al))}종) · D_pw 최대 "
        f"{dpw.max():,.0f} · D 최대 {D.max():,.0f}")
    chk("④", c7 > 0, f"C7 여유 최소 {c7:.3f} m (z2−z1 − (T+0.1))")
    chk("⑤", not off and 1.5 <= min(sl) and max(sl) <= 2.5 and front
        and {"D_mm", "mass_brg_t", "mass_total_t", "sigma_margin"} <= set(cols),
        f"L_w 이탈 {len(off)} · 세장비 {min(sl):.4f}~{max(sl):.4f} · "
        f"프론트 {len(front)}건 · 열 정상")

    fz = sorted({float(r["z2"]) for r in front})
    ez = sorted({float(r["z2"]) for r in C})
    print(f"  ·  z2 분포(기록)  프론트 {len(fz)}수준 {min(fz):.1f}~{max(fz):.1f}"
          f" · 평가 {len(ez)}수준 {min(ez):.1f}~{max(ez):.1f}")
    print(f"\n  pymoo 배선  4세대 완주 · 예산 {prob.n_call:,} · "
          f"평가 {ev.n_masta:,} · 캐시 {ev.n_hit} · 해석식탈락 "
          f"{prob.n_geom_out:,} ({100*prob.n_geom_out/prob.n_call:.1f}%)")
    print(f"\n  판정: {'통과 — S3-b 진행 가능' if ok else '실패'}")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
