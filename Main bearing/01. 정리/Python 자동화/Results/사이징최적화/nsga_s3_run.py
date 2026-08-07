"""
부록 6 S3 — 본 최적화 (NSGA-II · 격자 밖 탐색)
================================================
S2 는 P1 Phase 3 격자(26,250 조합) 위에서 알고리즘의 **정확성**을 검증했다.
S3 는 그 격자를 벗어나 **1.21 × 10¹² 조합**을 탐색한다(§6-3).

S2 와 달라지는 것 네 가지 — 전부 §6-8 에서 채택한 구성이다.
  ① 수치 3변수를 **스케일 `Integer`** 로 선언 (0.1 mm 단위 · §6-3.1)
  ② 종속변수 **정수화 켬** (`INTEGERIZE=True` · §6-4)
  ③ 세장비 **밴드 재매개화** — `L_we` 대신 밴드 내 상대위치 `u` (§6-11.3)
  ④ 초기집단 **범주 224 조합 전수 시딩** (§6-8.1.4 ⒝)

③ 은 초안의 수리(clip)를 대체한 것이다. 클립은 개체의 51.6 ~ 70.8% 를
세장비 경계선 위에 박아 넣어 `L_we` 를 사실상 종속변수로 만들었다(§6-11.2).
재매개화는 제약을 **구조적으로** 만족시키면서 `L_we` 를 밴드 안에서
자유롭게 둔다 — 경계에 닿는 것은 `u` 가 0 또는 U_MAX 일 때뿐이다.

종료는 **세대수 150 고정**이다(미결 #3 종결 · R14 항목 24). 하이퍼볼륨은
세대별로 기록만 하고 종료 판정에 쓰지 않는다 — 사후 수렴 확인용이다.

중복 제거는 pymoo 에 맡기지 않는다(`eliminate_duplicates=None` · R14 항목 25).
`u` 는 밴드 폭보다 촘촘해 여러 `u` 가 같은 `L_we` 로 떨어지는데, 이런
중복은 `x` 공간에서 보이지 않는다. MASTA 절약은 `eval_cache` 가 담당한다.

사용법
  python nsga_s3_run.py dry     드라이런 224×5 (약 6분) — S3-b 점검 4항목
  python nsga_s3_run.py         본 최적화 224×150 (약 2.95시간)
"""
import csv
import json
import os
import sys
import time

import numpy as np
from pymoo.algorithms.moo.nsga2 import NSGA2, RankAndCrowdingSurvival
from pymoo.core.callback import Callback
from pymoo.core.mixed import MixedVariableMating
from pymoo.core.problem import ElementwiseProblem
from pymoo.core.sampling import Sampling
from pymoo.core.variable import Choice, Integer
from pymoo.indicators.hv import HV
from pymoo.optimize import minimize

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import nsga_eval as ne          # noqa: E402
import sizing_geom as sg        # noqa: E402

LIMIT = 2100.0                  # MPa (§6-5)
POP, GEN, SEED = 224, 150, 1    # §6-8 — S2 에서 재현율 100% 의 최소 구성
DRY_GEN = 5                     # S3-b 드라이런
# 부록 8 이 평가기와 산출 폴더만 갈아끼워 이 스크립트를 재사용한다
OUTROOT = "부록6_NSGA"

# ── 설계변수 (§6-3 · §6-11.3) ────────────────────────────────────────
# 수치변수는 **0.1 mm 정수 단위**로 다룬다. D_pw 만 1 mm 단위이므로 10배
# 해서 같은 눈금에 올린다 — 밴드 경계 산출이 전부 정수 연산이 된다.
Z1_OPT = [1.0, 1.5]                                    # m
Z2_OPT = [3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0]           # m
AL_OPT = list(range(15, 31))                           # deg (1° 간격)
DPW_LO, DPW_HI = 3300, 4500                            # mm (1 mm 단위)
DWE_LO, DWE_HI = 1100, 2300                            # 0.1 mm 단위 → 110~230 mm
LWE_LO, LWE_HI = 1750, 5500                            # 0.1 mm 단위 → 175~550 mm
U_MAX = 10000                                          # 밴드 내 상대위치 눈금
N_CAT = len(Z1_OPT) * len(Z2_OPT) * len(AL_OPT)        # 224

# 하이퍼볼륨 기준점 [베어링 t · 총 t] — P1 Phase 3 최대(23 t / 195 t) 밖에 둔다.
# 세대 간 비교용이므로 값 자체가 아니라 **고정되어 있다는 점**이 중요하다.
HV_REF = np.array([40.0, 250.0])


# ── 세장비 밴드 재매개화 (§6-11.3) ───────────────────────────────────
def band(dwe_i):
    """`D_we` 에 대응하는 `L_we` 허용 밴드를 0.1 mm 격자로 돌려준다.

    입력·출력 모두 **0.1 mm 정수 단위**다. 하한은 올림·상한은 내림이라
    두 방향 모두 구간 안쪽이고, 따라서 세장비 1.5 ~ 2.5 를 벗어나지 않는다.
      lo = ceil (1.5·dwe_i) = (3·dwe_i + 1) // 2
      hi = floor(2.5·dwe_i) = (5·dwe_i) // 2
    변수 범위 [175, 550] mm 를 넘으면 그쪽으로 자른다 — 그래도 세장비는
    만족한다(하한 절단은 `D_we` < 116.7 mm, 상한 절단은 > 220 mm 일 때만).
    """
    return (max((3 * dwe_i + 1) // 2, LWE_LO),
            min((5 * dwe_i) // 2, LWE_HI))


def lwe_of(dwe_i, u):
    """밴드 내 상대위치 `u` → `L_we` (0.1 mm 정수 단위)

    `u` 는 0 ~ U_MAX 의 정수다. 밴드 폭이 1,100 ~ 1,200 스텝이므로 여러
    `u` 가 같은 `L_we` 로 떨어진다(약 9:1) — 중복은 `eval_cache` 가 흡수한다.
    경계값은 `u` 가 정확히 0 또는 U_MAX 일 때만 나온다.
    """
    lo, hi = band(dwe_i)
    return lo + round(u * (hi - lo) / U_MAX)


def to_si(z1, z2, dpw_mm, alpha, dwe_i, lwe_i):
    """평가기 입력(SI · m·deg)으로 변환"""
    return (float(z1), float(z2), dpw_mm / 1e3, float(alpha),
            dwe_i / 1e4, lwe_i / 1e4)


# ── 초기집단 — 범주 224 조합 전수 시딩 (§6-8.1.4 ⒝) ──────────────────
class CategorySeeding(Sampling):
    """범주 조합 224개를 빠짐없이 한 번씩 배치하고 수치는 무작위로 채운다.

    무작위 224 개체는 조합의 63% 만 덮는다(§6-8.1.2 ③). 미탐색 조합에
    프론트가 있으면 그대로 놓친다.

    `n_samples` 가 224 를 넘으면 조합을 순환시킨다(수치는 매번 새로 뽑음).
    """

    def __init__(self, seed=SEED):
        super().__init__()
        self.seed = seed

    def _do(self, problem, n_samples, **kw):
        rng = np.random.default_rng(self.seed)
        cats = [(a, b, c) for a in Z1_OPT for b in Z2_OPT for c in AL_OPT]
        rng.shuffle(cats)
        out = []
        for i in range(n_samples):
            z1, z2, al = cats[i % len(cats)]
            out.append(dict(z1=z1, z2=z2, alpha=al,
                            D_pw=int(rng.integers(DPW_LO, DPW_HI + 1)),
                            D_we=int(rng.integers(DWE_LO, DWE_HI + 1)),
                            u=int(rng.integers(0, U_MAX + 1))))
        return np.array(out, dtype=object)


# ── 문제 정의 ────────────────────────────────────────────────────────
class SizingS3(ElementwiseProblem):
    """2목적(베어링 질량 ↓ · 총질량 ↓) · 제약 2개

    제약은 둘만 둔다(§6-5).
      g1  기하 제약 위반 개수 (C2·C4~C9·C12 — `sizing_geom.constraints`)
      g2  응력 `(σ − 2,100) / 2,100`
    `z1 < z2` 는 범주 정의상 항상 만족하고(`z1 ≤ 1.5` · `z2 ≥ 3.0`),
    세장비는 재매개화로 구조적으로 만족한다.
    """

    def __init__(self, ev):
        self.ev = ev
        self.n_call = 0          # `_evaluate` 호출 총수 = 예산
        self.n_geom_out = 0      # 해석식 탈락 (MASTA 미호출)
        self.n_edge = 0          # 세장비 경계(u=0 또는 U_MAX)에 놓인 개체
        super().__init__(
            vars=dict(z1=Choice(options=Z1_OPT), z2=Choice(options=Z2_OPT),
                      alpha=Choice(options=AL_OPT),
                      D_pw=Integer(bounds=(DPW_LO, DPW_HI)),
                      D_we=Integer(bounds=(DWE_LO, DWE_HI)),
                      u=Integer(bounds=(0, U_MAX))),
            n_obj=2, n_ieq_constr=2)

    def design_of(self, x):
        """설계변수 dict → SI 설계점. 프론트 기록도 이걸 쓴다."""
        lwe_i = lwe_of(x["D_we"], x["u"])
        return to_si(x["z1"], x["z2"], x["D_pw"], x["alpha"],
                     x["D_we"], lwe_i), lwe_i

    def _evaluate(self, x, out, *a, **kw):
        self.n_call += 1
        pt, _ = self.design_of(x)
        if x["u"] in (0, U_MAX):
            self.n_edge += 1

        g = ne.geom(*pt[2:], True)                 # 정수화 켬 (§6-4)
        try:
            bad = sg.constraints(g, pt[0], pt[1])
        except Exception:
            bad = ["err"]
        if bad:                                    # 해석식 탈락 → MASTA 생략
            self.n_geom_out += 1
            out["F"] = [1e6, 1e6]
            out["G"] = [float(len(bad)), 1.0]
            return

        r = self.ev.evaluate([pt])[0]
        s = float(r["sigma_max_MPa"])
        mb, mt = float(r["mass_brg_kg"]), float(r["mass_total_kg"])
        out["F"] = [mb / 1000.0, mt / 1000.0]
        out["G"] = [0.0, (s - LIMIT) / LIMIT if s > 0 else 1.0]


# ── 세대 로그 · 체크포인트 ───────────────────────────────────────────
class GenLog(Callback):
    """세대마다 하이퍼볼륨·MASTA 호출수·경과를 남기고 집단을 덤프한다.

    HV 는 **종료 판정에 쓰지 않는다**(R14 항목 24) — 사후 수렴 확인용이다.
    체크포인트는 중단 시 진행 상황을 읽기 위한 것이며, NSGA 집단 상태를
    복원하지는 않는다. 재시작하면 세대는 다시 돌지만 `eval_cache` 적중으로
    MASTA 는 다시 부르지 않는다.
    """

    def __init__(self, prob, outdir, n_gen=GEN):
        super().__init__()
        self.prob, self.outdir, self.n_gen = prob, outdir, n_gen
        self.t0 = time.perf_counter()
        self.rows = []
        self.hv = HV(ref_point=HV_REF)
        self.logf = os.path.join(outdir, "s3_genlog.csv")

    def notify(self, algo):
        F = algo.opt.get("F")
        F = np.atleast_2d(F) if F is not None else np.empty((0, 2))
        ok = F[(F[:, 0] < HV_REF[0]) & (F[:, 1] < HV_REF[1])] if len(F) else F
        row = dict(gen=int(algo.n_gen),
                   n_front=int(len(ok)),
                   hv=round(float(self.hv(ok)), 4) if len(ok) else 0.0,
                   f1_min=round(float(ok[:, 0].min()), 4) if len(ok) else None,
                   f2_min=round(float(ok[:, 1].min()), 4) if len(ok) else None,
                   budget=self.prob.n_call,
                   masta=self.prob.ev.n_masta,
                   cache_hit=self.prob.ev.n_hit,
                   geom_out=self.prob.n_geom_out,
                   edge=self.prob.n_edge,
                   t_s=round(time.perf_counter() - self.t0, 1))
        self.rows.append(row)
        with open(self.logf, "w", newline="", encoding="utf-8-sig") as f:
            w = csv.DictWriter(f, fieldnames=list(row))
            w.writeheader()
            w.writerows(self.rows)
        dump_front(self.prob, algo, os.path.join(self.outdir,
                                                 "s3_checkpoint.csv"))
        print(f"  gen {row['gen']:3}/{self.n_gen}  프론트 {row['n_front']:3}  "
              f"HV {row['hv']:9.3f}  MASTA {row['masta']:6,}  "
              f"탈락 {row['geom_out']:6,}  {row['t_s']:7.1f}s", flush=True)


FRONT_COLS = ["rank", "mass_brg_t", "mass_total_t", "z1", "z2", "D_pw_mm",
              "alpha", "D_we_mm", "L_we_mm", "u", "slenderness", "Z",
              "sigma_max_MPa", "bore_mm", "D_mm", "T_mm", "B_mm", "C_mm",
              "L_eff_m"]


def dump_front(prob, algo, path):
    """비지배 집합을 CSV 로. **`u` 에서 환산한 `L_we`** 를 적는다.

    설계변수 `u` 자체는 도면 치수가 아니므로 환산값을 함께 남긴다 —
    §6-3.1 이 요구한 "보고된 설계 = 실제 평가된 설계"를 지키기 위함이다.
    """
    F = np.atleast_2d(algo.opt.get("F"))
    X = algo.opt.get("X")
    rows = []
    for f_, x_ in zip(F, X):
        if f_[0] >= 1e5:
            continue
        pt, _ = prob.design_of(x_)
        r = prob.ev.cache.get(ne.key_of(*pt), {})
        # 평가기가 더 기록한 열이 있으면(부록 8 의 2β 등) 그대로 실어 준다
        add = {k: r.get(k) for k in getattr(prob.ev, "FIELDS", [])
               if k not in FRONT_COLS and k in r}
        rows.append(dict(**add, mass_brg_t=round(f_[0], 4),
                         mass_total_t=round(f_[1], 4),
                         z1=pt[0], z2=pt[1], D_pw_mm=round(pt[2] * 1e3, 1),
                         alpha=pt[3], D_we_mm=round(pt[4] * 1e3, 2),
                         L_we_mm=round(pt[5] * 1e3, 2), u=int(x_["u"]),
                         slenderness=r.get("slenderness"), Z=r.get("Z"),
                         sigma_max_MPa=r.get("sigma_max_MPa"),
                         bore_mm=r.get("bore_mm"), D_mm=r.get("D_mm"),
                         T_mm=r.get("T_mm"), B_mm=r.get("B_mm"),
                         C_mm=r.get("C_mm"), L_eff_m=r.get("L_eff_m")))
    rows.sort(key=lambda r: r["mass_brg_t"])
    for i, r in enumerate(rows, 1):
        r["rank"] = i
    cols = FRONT_COLS + [k for k in (rows[0] if rows else {})
                         if k not in FRONT_COLS]
    with open(path, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=cols)
        w.writeheader()
        w.writerows(rows)
    return rows


# ── 사전 점검 — v1.3 기준선 재검증 ───────────────────────────────────
# S1 자기검증(§6-10.1)과 **같은 1점**을 본런 직전에 다시 돌린다. 모델 파일이
# 바뀌었거나 MASTA 설정이 달라졌으면 여기서 먼저 걸린다.
BASE_PT = (0.5, 3.0, 3.3309, 19.0, 0.11051, 0.238048)
BASE_EXP = dict(bore_mm=3055.0, D_mm=3600.0, T_mm=310.0, Z=87,
                sigma_max_MPa=3424.2, mass_brg_kg=5600.5,
                mass_shaft_kg=43225.8, L_eff_m=3.61666)
BASE_TOL = dict(sigma_max_MPa=0.001, mass_brg_kg=0.002,      # 상대오차
                mass_shaft_kg=0.002, L_eff_m=0.001)


def baseline_check(ev):
    """v1.3 기준선 1점을 재현하는지 확인한다 (약 0.3 s).

    정수화는 **끈다** — §6-10.1 의 기댓값이 정수화 이전 기준이기 때문이다.
    (`ID_shaft` 가 `floor` 냐 `round` 냐에서 1 mm 갈려 샤프트 질량이 달라진다.)
    평가 후 원래 설정으로 되돌린다.
    """
    keep = ev.integerize
    ev.integerize = False
    try:
        r = ev.evaluate([BASE_PT])[0]
    finally:
        ev.integerize = keep

    ok, lines = True, []
    for k in ("bore_mm", "D_mm", "T_mm", "Z"):                # 정확 일치
        got, exp = float(r[k]), float(BASE_EXP[k])
        hit = got == exp
        ok &= hit
        lines.append((k, got, exp, hit, ""))
    for k, tol in BASE_TOL.items():                           # 상대오차
        got, exp = float(r[k]), float(BASE_EXP[k])
        e = abs(got - exp) / abs(exp)
        hit = e <= tol
        ok &= hit
        lines.append((k, got, exp, hit, f"{100*e:.3f}%"))

    print("\n[사전] v1.3 기준선 재검증")
    for k, got, exp, hit, extra in lines:
        print(f"  {'O' if hit else 'x'}  {k:14} {got:>12,.4g}  기대 {exp:>12,.4g}"
              f"  {extra}")
    print(f"  판정: {'일치' if ok else '불일치 — 본런 보류'}")
    return ok


# ── S3-b 드라이런 점검 (§6-9) ────────────────────────────────────────
def dry_checks(prob, front, outdir):
    """네 항목을 실측으로 확인하고 통과 여부를 돌려준다.

    **기준선(v1.3) 은 점검 대상에서 뺀다** — `L_we` 238.048 mm 는 0.1 mm 격자에
    없는 실측값이고, 최적화가 만든 설계가 아니라 사전 검증용으로 캐시에
    들어간 것이기 때문이다(§6-11.1 재검증).
    """
    print("\n[S3-b] 드라이런 점검")
    res = {}
    base_key = ne.key_of(*BASE_PT)
    pts = [r for k, r in prob.ev.cache.items() if k != base_key]

    seen = set()
    for x in CategorySeeding()._do(prob, POP):
        seen.add((x["z1"], x["z2"], x["alpha"]))
    res["시딩 커버리지"] = (len(seen) == N_CAT, f"{len(seen)}/{N_CAT}")

    off = [r for r in pts
           if abs(round(float(r["L_we_mm"]) * 10) - float(r["L_we_mm"]) * 10)
           > 1e-9]
    res["L_we 0.1 mm 격자"] = (not off,
                               f"평가 {len(pts):,}점 중 이탈 {len(off)}"
                               f" (기준선 1점 제외)")

    sl = [float(r["slenderness"]) for r in pts]
    ok_sl = all(1.5 - 1e-9 <= v <= 2.5 + 1e-9 for v in sl)
    edge = sum(1 for v in sl if v <= 1.5005 or v >= 2.4995)
    res["세장비 밴드"] = (ok_sl, f"{min(sl):.4f} ~ {max(sl):.4f} · "
                                 f"경계집중 {100*edge/len(sl):.1f}%" if sl else "—")

    bad = [r for r in front
           if abs(float(r["L_we_mm"]) * 10 - round(float(r["L_we_mm"]) * 10))
           > 1e-9 or not (1.5 - 1e-9 <= float(r["slenderness"]) <= 2.5 + 1e-9)]
    res["프론트 환산값"] = (not bad,
                            f"프론트 {len(front)}건 · 격자·밴드 이탈 {len(bad)}")

    ok_all = True
    for k, (ok, msg) in res.items():
        print(f"  {'O' if ok else 'x'}  {k:16} {msg}")
        ok_all &= ok
    ckpt = os.path.join(outdir, "s3_checkpoint.csv")
    print(f"  {'O' if os.path.isfile(ckpt) else 'x'}  {'체크포인트':16} "
          f"{'생성됨' if os.path.isfile(ckpt) else '없음'} — 재시작 시 "
          f"eval_cache 적중으로 MASTA 재호출 없음")
    print(f"\n  판정: {'통과 — 본런 진행 가능' if ok_all else '실패 — 본런 보류'}")
    return ok_all


def main():
    dry = len(sys.argv) > 1 and sys.argv[1].lower().startswith("dry")
    gen = DRY_GEN if dry else GEN
    outdir = os.path.join(HERE, OUTROOT,
                          "S3_드라이런" if dry else "S3_본최적화")
    os.makedirs(outdir, exist_ok=True)

    ev = ne.Evaluator(outdir, integerize=True, verbose=True)
    prob = SizingS3(ev)
    print(f"[S3{'-b 드라이런' if dry else ''}] {POP} × {gen} = "
          f"{POP*gen:,} 평가 · 조합 {2*7*16*1201*1201*3751:.2e} · "
          f"캐시 {len(ev.cache):,}점", flush=True)

    if not baseline_check(ev):          # 사전 게이트 — 재시도 대상 아님
        ev.close()
        return 4

    algo = NSGA2(pop_size=POP,
                 sampling=CategorySeeding(),
                 mating=MixedVariableMating(eliminate_duplicates=None),
                 survival=RankAndCrowdingSurvival(),
                 eliminate_duplicates=None)
    t0 = time.perf_counter()
    res = minimize(prob, algo, ("n_gen", gen), seed=SEED, verbose=False,
                   callback=GenLog(prob, outdir, gen), save_history=False)
    dt = time.perf_counter() - t0
    ev.close()

    front = dump_front(prob, res.algorithm,
                       os.path.join(outdir, "s3_front.csv"))
    meta = dict(pop=POP, gen=gen, seed=SEED, dry=dry,
                budget=prob.n_call, masta=ev.n_masta, cache_hit=ev.n_hit,
                geom_out=prob.n_geom_out, edge=prob.n_edge,
                n_front=len(front), elapsed_s=round(dt, 1),
                s_per_masta=round(dt / ev.n_masta, 3) if ev.n_masta else None,
                hv_ref=HV_REF.tolist())
    json.dump(meta, open(os.path.join(outdir, "s3_meta.json"), "w"), indent=1)

    print(f"\n[결과] 프론트 {len(front)}건 · 예산 {prob.n_call:,} · "
          f"MASTA {ev.n_masta:,} · 캐시적중 {ev.n_hit:,} · "
          f"해석식탈락 {prob.n_geom_out:,} · 밴드경계 {prob.n_edge:,}")
    print(f"       소요 {dt/60:.1f}분"
          + (f" · {dt/ev.n_masta:.3f} s/MASTA" if ev.n_masta else ""))
    if front:
        print(f"       최경량 베어링 {front[0]['mass_brg_t']:.3f} t / "
              f"총 {front[0]['mass_total_t']:.3f} t")
    if dry:
        ok = dry_checks(prob, front, outdir)
        if ev.n_masta:
            print(f"\n  본런 예산 추정: MASTA {ev.n_masta/gen*GEN:,.0f}회 × "
                  f"{dt/ev.n_masta:.3f} s ≈ "
                  f"{ev.n_masta/gen*GEN*(dt/ev.n_masta)/3600:.2f}시간")
        return 0 if ok else 3          # 3 = 게이트 불합격 (재시도 대상 아님)
    return 0


if __name__ == "__main__":
    sys.exit(main())
