"""
부록 9 S3 — 외경 `D` 제약 · 스팬 세분 최적화
================================================
부록 8 의 `a8_s3_run.py` 와 같은 방식으로 `nsga_s3_run.py` 를 재사용하고
**§9-2 ~ §9-5 의 변경만 갈아끼운다.**

  · **목적**  `(베어링, 총질량)` → **`(외경 D ↓, 베어링 1개 질량 ↓)`**
  · `z1`   `Choice [1.0, 1.5]` → **1.0 고정**
  · `z2`   `Choice`(7) → **6.0 고정** — 아래 참조
  · α      `Choice` → **`Integer(15, 30)`**
  · `D_pw` 상한 4,500 → **4,800**
  · `C6`   `D_pw ≤ 4,500` → **`D ≤ 5,000`** · `C4`·`C5` 삭제
  · 개체수 224 → **100** · 세대 150

**`z2` 를 고정하는 이유** — `z2` 는 `D` 에 전혀 들어가지 않고 베어링 질량과는
음의 상관(−0.403)이다. 길수록 베어링 하중이 줄어 `f2` 만 좋아지므로 **상한에
박힌다.** 기존 가능해 17,714점에서 `(D, 베어링)` 프론트를 뽑으면 `z2` 가
**33/33 전부 6.0** 이었다. 변수로 두면 26수준 중 25개가 지배당해 예산만 쓴다.

**총질량·샤프트 질량은 목적이 아니다** — 프론트 표에 열로 실어 선택 단계에서
본다(§8-6.5.7 ⑹ 과 같은 운영).

평가기·종속변수(두께 규칙 · 코너 반경 4.3 · 정수화)는 부록 8 그대로다.

  python a9_s3_run.py dry     드라이런 130 × 5 (S3-b)
  python a9_s3_run.py         본 최적화 130 × 150 (S3-c)
"""
import csv
import io
import os
import re
import sys
import time

import numpy as np
from pymoo.core.variable import Choice, Integer

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import a8_eval                    # noqa: E402
import nsga_s3_run as S3          # noqa: E402

S3.ne = a8_eval
S3.OUTROOT = "부록9_NSGA"
S3.BASE_EXP = dict(S3.BASE_EXP, sigma_max_MPa=3407.9, mass_shaft_kg=58384.5)

# ── §9-2 설계변수 ───────────────────────────────────────────────────
S3.Z1_OPT = [1.0]
S3.Z2_OPT = [6.0]
S3.AL_OPT = list(range(15, 31))
S3.DPW_HI = 4800
S3.N_CAT = 1                                   # 범주가 없다 — 전부 Integer
S3.POP, S3.GEN = 100, 150
S3.HV_REF = np.array([5.5, 45.0])              # [D m · 베어링 t] — 목적 변경
DCAP = 5.0                                     # m — 새 `C6`

# ── §9-4 제약 ───────────────────────────────────────────────────────
_ORIG = S3.sg.constraints


def _cons(g, z1, z2):
    """`C4`·`C5` 삭제 · `C6` 를 외경 기준으로 대체 (§9-4)"""
    v = [x for x in _ORIG(g, z1, z2)
         if not x.startswith(("C4", "C5", "C6"))]
    if g["outer_diameter"] > DCAP:
        v.append("C6 D >5000mm")
    return v


S3.sg.constraints = _cons


# ── α 를 `Integer` 로 · `z1` 고정 ───────────────────────────────────
# 아래에서 `S3.SizingS3` 를 갈아끼우므로 원본을 먼저 붙잡아 둔다 —
# 그러지 않으면 `super(S3.SizingS3, ...)` 가 자기 자신을 가리킨다.
_BASE = S3.SizingS3
_SEED = S3.CategorySeeding


class Sizing9(_BASE):
    """변수 정의와 **목적함수**를 §9-2 로 바꾼다.

    `f1` 외경 `D` [m] ↓ · `f2` 베어링 1개 질량 [t] ↓
    """

    def __init__(self, ev):
        self.ev = ev
        self.n_call = self.n_geom_out = self.n_edge = 0
        # `ElementwiseProblem.__init__` 을 직접 부른다 — 부모의 vars 정의를
        # 건너뛰기 위함이다(`SizingS3.__init__` 은 Choice 로 α 를 잡는다).
        super(_BASE, self).__init__(
            vars=dict(z1=Choice(options=S3.Z1_OPT),
                      z2=Choice(options=S3.Z2_OPT),
                      alpha=Integer(bounds=(S3.AL_OPT[0], S3.AL_OPT[-1])),
                      D_pw=Integer(bounds=(S3.DPW_LO, S3.DPW_HI)),
                      D_we=Integer(bounds=(S3.DWE_LO, S3.DWE_HI)),
                      u=Integer(bounds=(0, S3.U_MAX))),
            n_obj=2, n_ieq_constr=2)

    def _evaluate(self, x, out, *a, **kw):
        self.n_call += 1
        pt, _ = self.design_of(x)
        if x["u"] in (0, S3.U_MAX):
            self.n_edge += 1
        g = S3.ne.geom(*pt[2:], True)
        try:
            bad = S3.sg.constraints(g, pt[0], pt[1])
        except Exception:
            bad = ["err"]
        if bad:                                    # 해석식 탈락 → MASTA 생략
            self.n_geom_out += 1
            out["F"] = [1e6, 1e6]
            out["G"] = [float(len(bad)), 1.0]
            return
        r = self.ev.evaluate([pt])[0]
        s = float(r["sigma_max_MPa"])
        out["F"] = [g["outer_diameter"], float(r["mass_brg_kg"]) / 1000.0]
        out["G"] = [0.0, (s - S3.LIMIT) / S3.LIMIT if s > 0 else 1.0]


class Seed9(_SEED):
    """`z1`·`z2` 가 고정이라 범주가 없다 — 수치변수만 무작위로 뿌린다"""

    def _do(self, problem, n_samples, **kw):
        rng = np.random.default_rng(S3.SEED)
        out = []
        for _ in range(n_samples):
            out.append(dict(
                z1=S3.Z1_OPT[0], z2=S3.Z2_OPT[0],
                alpha=int(rng.integers(S3.AL_OPT[0], S3.AL_OPT[-1] + 1)),
                D_pw=int(rng.integers(S3.DPW_LO, S3.DPW_HI + 1)),
                D_we=int(rng.integers(S3.DWE_LO, S3.DWE_HI + 1)),
                u=int(rng.integers(0, S3.U_MAX + 1))))
        return np.array(out, dtype=object)


def dump9(prob, algo, path):
    """프론트 덤프 — 부록 6 판은 `F[0]=베어링·F[1]=총질량` 을 가정하므로 쓸 수
    없다. 목적값 대신 **캐시 행을 그대로** 싣고 질량 셋을 함께 낸다."""
    X = algo.opt.get("X")
    rows, seen = [], set()
    for x in X:
        pt, _ = prob.design_of(x)
        k = S3.ne.key_of(*pt)
        r = prob.ev.cache.get(k)
        if r is None or k in seen:
            continue
        seen.add(k)
        mb = float(r["mass_brg_kg"]) / 1e3
        ms = float(r["mass_shaft_kg"]) / 1e3
        rows.append(dict(r, mass_brg_t=round(mb, 4),
                         mass_shaft_t=round(ms, 4),
                         mass_total_t=round(2 * mb + ms, 4),
                         sigma_margin=round(S3.LIMIT
                                            - float(r["sigma_max_MPa"]), 1)))
    rows.sort(key=lambda q: (float(q["D_mm"]), q["mass_brg_t"]))
    for i, r in enumerate(rows, 1):
        r["rank"] = i
    if rows:
        with open(path, "w", newline="", encoding="utf-8-sig") as f:
            w = csv.DictWriter(f, fieldnames=["rank"] + [c for c in rows[0]
                                                         if c != "rank"])
            w.writeheader()
            w.writerows(rows)
    return rows


def dry9(prob, front, outdir):
    """드라이런 점검 (§9-8).

    부록 6 판의 「시딩 커버리지」는 `(z1, z2, α)` 조합 수를 `N_CAT` 과 비교한다.
    부록 9 는 α 가 `Integer` 로 빠지고 `z1`·`z2` 가 고정이라 **범주가 없어**
    그 검사가 「16 / 1」 같은 무의미한 값을 내고 항상 실패한다. 그 자리에
    **α 가 `Integer` 로서 폭넓게 뿌려지는지**를 넣는다. 나머지는 그대로다.
    """
    print("\n[S3-b] 드라이런 점검")
    res = {}
    pts = [r for k, r in prob.ev.cache.items()
           if k != S3.ne.key_of(*S3.BASE_PT)]

    seed = Seed9()._do(prob, S3.POP)
    al = sorted({int(x["alpha"]) for x in seed})
    zz = {(x["z1"], x["z2"]) for x in seed}
    res["초기집단 α"] = (len(zz) == 1 and len(al) >= 12 and al[0] <= 16
                         and al[-1] >= 29,
                         f"{len(al)}종 {al[0]} ~ {al[-1]} · "
                         f"(z1, z2) {len(zz)}조합 고정")

    off = [r for r in pts
           if abs(round(float(r["L_w_mm"]) * 10) - float(r["L_w_mm"]) * 10)
           > 1e-9]
    res["L_w 0.1 mm 격자"] = (not off, f"평가 {len(pts):,}점 중 이탈 {len(off)}"
                                       f" (기준선 1점 제외)")

    sl = [float(r["slenderness"]) for r in pts]
    edge = sum(1 for v in sl if v <= 1.5005 or v >= 2.4995)
    res["세장비 밴드"] = (all(1.5 - 1e-9 <= v <= 2.5 + 1e-9 for v in sl),
                          f"{min(sl):.4f} ~ {max(sl):.4f} · "
                          f"경계집중 {100*edge/len(sl):.1f}%")

    D = [float(r["D_mm"]) for r in pts]
    res["C6 외경 상한"] = (max(D) <= 5000 + 1e-6,
                           f"D {min(D):,.0f} ~ {max(D):,.0f} mm (≤ 5,000)")

    fD = [float(r["D_mm"]) for r in front]
    fb = [float(r["mass_brg_t"]) for r in front]
    res["프론트 단조성"] = (len(front) > 1,
                            f"{len(front)}건 · D {min(fD):,.0f} ~ "
                            f"{max(fD):,.0f} · 베어링 {min(fb):.2f} ~ "
                            f"{max(fb):.2f} t")

    ok_all = True
    for k, (ok, msg) in res.items():
        print(f"  {'O' if ok else 'x'}  {k:16} {msg}")
        ok_all &= ok
    ck = os.path.join(outdir, "s3_checkpoint.csv")
    print(f"  {'O' if os.path.isfile(ck) else 'x'}  {'체크포인트':16} "
          f"{'생성됨' if os.path.isfile(ck) else '없음'}")
    print(f"\n  판정: {'통과 — 본런 진행 가능' if ok_all else '실패 — 본런 보류'}")
    return ok_all


EVERY, MARK = 10, "A9:GENLOG"
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")


class Log9(S3.GenLog):
    """세대 로그 + **10세대마다** §9-9 마커에 실행요약표를 다시 쓴다"""

    def notify(self, algo):
        super().notify(algo)
        g = int(algo.n_gen)
        if g % EVERY and g != self.n_gen:
            return
        F = algo.opt.get("F")
        X = algo.opt.get("X")
        tot = []
        for x in X:
            pt, _ = self.prob.design_of(x)
            r = self.prob.ev.cache.get(S3.ne.key_of(*pt))
            if r:
                tot.append((2 * float(r["mass_brg_kg"])
                            + float(r["mass_shaft_kg"])) / 1e3)
        row = self.rows[-1]
        self._snap = getattr(self, "_snap", [])
        self._snap.append(dict(
            gen=g, n_front=row["n_front"], hv=row["hv"],
            D_min=float(F[:, 0].min()) * 1e3, b_min=float(F[:, 1].min()),
            t_min=min(tot) if tot else None))
        self._doc()

    def _doc(self):
        body = ["", "| 세대 | 프론트 | HV | ΔHV | **최소 `D`** [mm] | "
                "**최소 베어링** [t] | 최소 총질량 [t] |",
                "|--:|--:|--:|--:|--:|--:|--:|"]
        prev = None
        for r in self._snap:
            d = "—" if prev is None else f"{r['hv']-prev:+.3f}"
            body.append(f"| {r['gen']} | {r['n_front']} | {r['hv']:,.3f} | {d} | "
                        f"**{r['D_min']:,.0f}** | **{r['b_min']:.2f}** | "
                        + (f"{r['t_min']:.2f} |" if r["t_min"] else "— |"))
            prev = r["hv"]
        body += ["", f"*{time.strftime('%m-%d %H:%M')} 기준 · "
                 f"{self._snap[-1]['gen']}/{self.n_gen} 세대 · HV 기준점 "
                 f"(5.5 m, 45 t) · 총질량은 목적이 아니라 참고값이다*"]
        try:
            s = io.open(DOC, encoding="utf-8").read()
            a, b = f"<!-- {MARK} -->", f"<!-- /{MARK} -->"
            pat = re.compile(re.escape(a) + r".*?" + re.escape(b), re.S)
            if pat.search(s):
                blk = a + "\n" + "\n".join(body) + "\n" + b
                out = pat.sub(lambda _m: blk, s, count=1)
                io.open(DOC, "w", encoding="utf-8").write(out)
        except Exception as e:
            print(f"  [문서] 갱신 실패: {str(e).splitlines()[0][:60]}")


S3.GenLog = Log9
S3.SizingS3 = Sizing9
S3.CategorySeeding = Seed9
S3.dump_front = dump9
S3.N_CAT = 1
S3.dry_checks = dry9

if __name__ == "__main__":
    sys.exit(S3.main())
