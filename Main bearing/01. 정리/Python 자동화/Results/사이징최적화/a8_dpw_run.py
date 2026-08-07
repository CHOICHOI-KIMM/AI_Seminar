"""
§8-8 — `D_pw` 상한 효과 확인 (224 × 40 · 5세대 실시간 갱신)
==============================================================
부록 8 S3-c 를 **`D_pw` 범위만 바꿔** 다시 돌린다.

    `D_pw` ∈ [3,300, 4,500]  →  **[4,400, 5,500]** · 1 mm 격자

나머지는 전부 그대로다 — 설계변수 구조·제약·목적함수·비용모델·평가기(두께
규칙 · 코너 반경 · 정수화)·개체수 224·시드 1·범주 전수 시딩·연산자. `D_pw` 는
`Choice` 가 아니라 스케일 `Integer` 라 **범주 조합은 224 로 그대로**이고, 따라서
개체수도 시딩도 손댈 필요가 없다.

**목적은 최적해가 아니라 판정이다.** 새 프론트의 `D_pw` 가 내부에 맺히면
4,500 이 답을 자르고 있었다는 것이 확정되고 참 최적 위치도 나온다. 여전히
상한(5,500)에 붙으면 문제는 최적화가 아니라 **두께 규칙에 상한 기전이 없다**는
쪽으로 옮겨간다.

5세대마다 `D_pw` 분포와 질량 분포를 문서 §8-8 마커와 CSV 에 함께 쓴다.

  python a8_dpw_run.py            224 × 40 (약 46분)
"""
import csv
import io
import os
import re
import sys
import time

import numpy as np
from pymoo.algorithms.moo.nsga2 import NSGA2, RankAndCrowdingSurvival
from pymoo.core.mixed import MixedVariableMating
from pymoo.optimize import minimize

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import a8_eval                    # noqa: E402
import nsga_s3_run as S3          # noqa: E402

S3.ne = a8_eval
S3.OUTROOT = "부록8_DPW"

# ── C6 해제 ─────────────────────────────────────────────────────────
# 1차 실행(260806)은 아무것도 재지 못했다. `D_pw` ≤ 4,500 은 탐색 상자가 아니라
# **명시적 설계제약 `C6`**(제작·운송 상한 · §3)이고, `sizing_geom.constraints`
# 안에 있어 변수 범위를 열어도 평가 전에 걸러진다 — 4,500 초과가 한 점도 MASTA
# 에 가지 못했다(탈락 1,539건).
#
# **제약을 버리는 것이 아니라 대가를 재는 것이다.** C6 만 떼고 돌려
# 「4,500 을 넘기면 총질량이 얼마나 줄고 그때 벽두께가 얼마인가」를 낸다.
# 제작·운송 완화를 협의할 때 근거가 되는 수치다. 나머지 제약은 그대로다.
_ORIG_CONS = S3.sg.constraints


def _cons_no_c6(g, z1, z2):
    return [v for v in _ORIG_CONS(g, z1, z2) if not v.startswith("C6")]


S3.sg.constraints = _cons_no_c6
S3.BASE_EXP = dict(S3.BASE_EXP, sigma_max_MPa=3407.9, mass_shaft_kg=58384.5)

DPW_LO, DPW_HI = 4400, 5500       # mm — 이 스크립트의 유일한 변경점
# `SizingS3` 의 변수 정의(152행)와 `CategorySeeding`(127행) 이 둘 다 이 전역을
# **호출 시점에** 읽으므로, 여기서 갈아끼우면 문제 정의와 시딩이 함께 따라온다.
S3.DPW_LO, S3.DPW_HI = DPW_LO, DPW_HI
GEN = 40
EVERY = 5
OUT = os.path.join(HERE, "부록8_DPW", "S3_C6해제")
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")
MARK = "A8:DPWLOG"


class Log(S3.GenLog):
    """부록 6 세대 로그 + 5세대마다 D_pw·질량 분포 기입"""

    def __init__(self, prob, outdir, gen):
        super().__init__(prob, outdir, gen)
        self.snap = []

    def notify(self, algo):
        super().notify(algo)
        g = algo.n_gen
        if g % EVERY and g != GEN:
            return
        ev = self.prob.ev
        fe = [r for r in ev.cache.values()
              if 0 < float(r["sigma_max_MPa"]) < S3.LIMIT
              and float(r["z1"]) >= 1.0]
        if not fe:
            return
        F = algo.opt.get("F")
        dpw_f = np.array([float(x["D_pw"]) for x in algo.opt.get("X")])
        dpw_a = np.array([float(r["D_pw_mm"]) for r in fe])
        mb = np.array([float(r["mass_brg_kg"]) for r in fe]) / 1e3
        mt = np.array([float(r["mass_total_kg"]) for r in fe]) / 1e3
        self.snap.append(dict(
            gen=g, n_front=len(F), hv=self.rows[-1]["hv"],
            f_lo=dpw_f.min(), f_med=float(np.median(dpw_f)), f_hi=dpw_f.max(),
            a_lo=100 * float((dpw_a <= DPW_LO + 10).mean()),
            a_hi=100 * float((dpw_a >= DPW_HI - 10).mean()),
            a_med=float(np.median(dpw_a)), n_fe=len(fe),
            mb=mb.min(), mt=mt.min()))
        self._dump()

    def _dump(self):
        cols = list(self.snap[0])
        with open(os.path.join(self.outdir, "dpw_progress.csv"), "w",
                  newline="", encoding="utf-8-sig") as f:
            w = csv.DictWriter(f, fieldnames=cols)
            w.writeheader()
            w.writerows(self.snap)
        body = [
            "", "| 세대 | 프론트 | HV | 프론트 `D_pw` 최소 | 중앙 | 최대 | "
            "가능해 | 하한 4,400 부착 | 상한 5,500 부착 | 가능해 `D_pw` 중앙 | "
            "최소 베어링 [t] | 최소 총질량 [t] |",
            "|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|"]
        for r in self.snap:
            body.append(
                f"| {r['gen']} | {r['n_front']} | {r['hv']:,.1f} | "
                f"{r['f_lo']:,.0f} | {r['f_med']:,.0f} | **{r['f_hi']:,.0f}** | "
                f"{r['n_fe']:,} | {r['a_lo']:.1f}% | **{r['a_hi']:.1f}%** | "
                f"{r['a_med']:,.0f} | {r['mb']:.2f} | **{r['mt']:.2f}** |")
        body.append("")
        body.append(f"*{time.strftime('%m-%d %H:%M')} 기준 · "
                    f"{self.snap[-1]['gen']}/{GEN} 세대 · `C6` 해제 · 부착 "
                    f"판정은 경계에서 10 mm 이내 · 전량은 "
                    f"`{os.path.relpath(self.outdir, HERE)}/`*")
        try:
            s = io.open(DOC, encoding="utf-8").read()
            a, b = f"<!-- {MARK} -->", f"<!-- /{MARK} -->"
            pat = re.compile(re.escape(a) + r".*?" + re.escape(b), re.S)
            if pat.search(s):
                io.open(DOC, "w", encoding="utf-8").write(
                    pat.sub(a + "\n" + "\n".join(body) + "\n" + b, s, count=1))
        except Exception as e:
            print(f"  [문서] 갱신 실패: {str(e).splitlines()[0][:60]}")


def main():
    os.makedirs(OUT, exist_ok=True)
    ev = S3.ne.Evaluator(OUT, integerize=True, verbose=False)
    print(f"[§8-8] D_pw {DPW_LO:,} ~ {DPW_HI:,} · {S3.POP} × {GEN} = "
          f"{S3.POP*GEN:,} 평가 · 캐시 {len(ev.cache):,}점", flush=True)

    if not S3.baseline_check(ev):
        ev.close()
        return 4

    prob = S3.SizingS3(ev)
    algo = NSGA2(pop_size=S3.POP, sampling=S3.CategorySeeding(),
                 mating=MixedVariableMating(eliminate_duplicates=None),
                 survival=RankAndCrowdingSurvival(), eliminate_duplicates=None)
    t0 = time.perf_counter()
    res = minimize(prob, algo, ("n_gen", GEN), seed=S3.SEED, verbose=False,
                   callback=Log(prob, OUT, GEN), save_history=False)
    front = S3.dump_front(prob, res.algorithm, os.path.join(OUT, "s3_front.csv"))
    dt = time.perf_counter() - t0
    print(f"\n[결과] 프론트 {len(front)}건 · MASTA {ev.n_masta:,} · "
          f"{dt/60:.1f}분 · {dt/max(ev.n_masta,1):.3f} s/MASTA")
    ev.close()
    return 0


if __name__ == "__main__":
    sys.exit(main())
