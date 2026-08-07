"""
부록 10 S3 — 3목적 최적화 (외경 · 베어링 · 총질량)
=====================================================
부록 9 의 `a9_s3_run.py` 와 같은 방식으로 `nsga_s3_run.py` 를 재사용하고
§10-2 ~ §10-6 의 변경만 갈아끼운다.

  · **목적**  `(D, 베어링)` → **`(D ↓, 베어링 1개 질량 ↓, 총질량 ↓)`**
  · `z2`   6.0 고정 → **`Choice` 3.5 ~ 6.0 · 0.1 m (26수준)**
  · 개체수 100 → **182** (= 7 × 26) · 세대 150
  · HV 기준점 `(5.5, 45)` → **`(5.5 m, 45 t, 250 t)`**

그 밖(두께 규칙 · 코너 반경 4.3 · 정수화 · 밴드 재매개화 · `C6` = `D ≤ 5,000`
· `C4`·`C5` 삭제 · `z1` 1.0 · α `Integer` · `D_pw` [3,300, 4,800])은 부록 9
그대로다.

**`f3` 가 `f2` 를 포함한다**(`f3 = 2·f2 + m_shaft`) — §10-2 에 한계로 적었다.
결과에서 `(D, 베어링, 샤프트)` 정식화로 재판정해 비교한다(§10-7 ①).

  python a10_s3_run.py dry     드라이런 182 × 5 (S3-b)
  python a10_s3_run.py         본 최적화 182 × 150 (S3-c)
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
S3.OUTROOT = "부록10_NSGA"
S3.BASE_EXP = dict(S3.BASE_EXP, sigma_max_MPa=3407.9, mass_shaft_kg=58384.5)

# ── §10-3 설계변수 ──────────────────────────────────────────────────
S3.Z1_OPT = [1.0]
S3.Z2_OPT = [round(3.5 + 0.1 * i, 1) for i in range(26)]
S3.AL_OPT = list(range(15, 31))
S3.DPW_HI = 4800
S3.N_CAT = len(S3.Z2_OPT)                      # 26 — `z2` 만 범주다
S3.POP, S3.GEN = 182, 150
S3.HV_REF = np.array([5.5, 45.0, 250.0])       # [D m · 베어링 t · 총질량 t]
DCAP = 5.0                                     # m — `C6`

# ── §10-5 제약 (부록 9 와 동일) ─────────────────────────────────────
_ORIG = S3.sg.constraints


def _cons(g, z1, z2):
    """`C4`·`C5` 삭제 · `C6` 를 외경 기준으로 대체"""
    v = [x for x in _ORIG(g, z1, z2)
         if not x.startswith(("C4", "C5", "C6"))]
    if g["outer_diameter"] > DCAP:
        v.append("C6 D >5000mm")
    return v


S3.sg.constraints = _cons

_BASE = S3.SizingS3
_SEED = S3.CategorySeeding


class Sizing10(_BASE):
    """변수 정의와 **3목적**을 §10-2 로 바꾼다"""

    def __init__(self, ev):
        self.ev = ev
        self.n_call = self.n_geom_out = self.n_edge = 0
        super(_BASE, self).__init__(
            vars=dict(z1=Choice(options=S3.Z1_OPT),
                      z2=Choice(options=S3.Z2_OPT),
                      alpha=Integer(bounds=(S3.AL_OPT[0], S3.AL_OPT[-1])),
                      D_pw=Integer(bounds=(S3.DPW_LO, S3.DPW_HI)),
                      D_we=Integer(bounds=(S3.DWE_LO, S3.DWE_HI)),
                      u=Integer(bounds=(0, S3.U_MAX))),
            n_obj=3, n_ieq_constr=2)

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
            out["F"] = [1e6, 1e6, 1e6]
            out["G"] = [float(len(bad)), 1.0]
            return
        r = self.ev.evaluate([pt])[0]
        s = float(r["sigma_max_MPa"])
        mb = float(r["mass_brg_kg"]) / 1000.0
        mt = float(r["mass_total_kg"]) / 1000.0
        out["F"] = [g["outer_diameter"], mb, mt]
        out["G"] = [0.0, (s - S3.LIMIT) / S3.LIMIT if s > 0 else 1.0]


class Seed10(_SEED):
    """`z2` 26수준 전수 시딩 + 수치변수 무작위 (§6-8.1.2)"""

    def _do(self, problem, n_samples, **kw):
        rng = np.random.default_rng(S3.SEED)
        out = []
        for i in range(n_samples):
            out.append(dict(
                z1=S3.Z1_OPT[0], z2=S3.Z2_OPT[i % len(S3.Z2_OPT)],
                alpha=int(rng.integers(S3.AL_OPT[0], S3.AL_OPT[-1] + 1)),
                D_pw=int(rng.integers(S3.DPW_LO, S3.DPW_HI + 1)),
                D_we=int(rng.integers(S3.DWE_LO, S3.DWE_HI + 1)),
                u=int(rng.integers(0, S3.U_MAX + 1))))
        return np.array(out, dtype=object)


def dump10(prob, algo, path):
    """프론트 덤프 — 캐시 행을 그대로 싣고 질량 셋·σ 여유를 더한다"""
    rows, seen = [], set()
    for x in algo.opt.get("X"):
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


class Log10(S3.GenLog):
    """세대 로그 — 목적이 3개라 `f3_min`(총질량)을 한 열 더 남긴다"""

    def notify(self, algo):
        F = algo.opt.get("F")
        F = np.atleast_2d(F) if F is not None else np.empty((0, 3))
        ok = F[(F < S3.HV_REF).all(axis=1)] if len(F) else F
        row = dict(gen=int(algo.n_gen), n_front=int(len(ok)),
                   hv=round(float(self.hv(ok)), 4) if len(ok) else 0.0,
                   f1_min=round(float(ok[:, 0].min()), 4) if len(ok) else None,
                   f2_min=round(float(ok[:, 1].min()), 4) if len(ok) else None,
                   f3_min=round(float(ok[:, 2].min()), 4) if len(ok) else None,
                   budget=self.prob.n_call, masta=self.prob.ev.n_masta,
                   cache_hit=self.prob.ev.n_hit,
                   geom_out=self.prob.n_geom_out, edge=self.prob.n_edge,
                   t_s=round(__import__("time").perf_counter() - self.t0, 1))
        self.rows.append(row)
        with open(self.logf, "w", newline="", encoding="utf-8-sig") as f:
            w = csv.DictWriter(f, fieldnames=list(row))
            w.writeheader()
            w.writerows(self.rows)
        dump10(self.prob, algo, os.path.join(self.outdir, "s3_checkpoint.csv"))
        print(f"  gen {row['gen']:3}/{self.n_gen}  프론트 {row['n_front']:3}  "
              f"HV {row['hv']:9.3f}  D {row['f1_min']}  베어링 {row['f2_min']}  "
              f"총 {row['f3_min']}  MASTA {row['masta']:6,}  "
              f"탈락 {row['geom_out']:5,}  {row['t_s']:7.1f}s", flush=True)


def dry10(prob, front, outdir):
    """드라이런 점검 (§10-9). `z2` 분포는 **기록만** 하고 판정에서 뺀다 —
    5세대로 판정하기엔 이르다(부록 8·9 모두 초반에 흔들렸다)."""
    print("\n[S3-b] 드라이런 점검")
    res = {}
    pts = [r for k, r in prob.ev.cache.items()
           if k != S3.ne.key_of(*S3.BASE_PT)]

    seed = Seed10()._do(prob, S3.POP)
    zz = sorted({x["z2"] for x in seed})
    al = sorted({int(x["alpha"]) for x in seed})
    res["z2 전수 시딩"] = (len(zz) == S3.N_CAT,
                           f"{len(zz)}/{S3.N_CAT}수준 {min(zz)} ~ {max(zz)}")
    res["초기집단 α"] = (len(al) >= 12 and al[0] <= 16 and al[-1] >= 29,
                         f"{len(al)}종 {al[0]} ~ {al[-1]}")

    off = [r for r in pts
           if abs(round(float(r["L_w_mm"]) * 10) - float(r["L_w_mm"]) * 10)
           > 1e-9]
    res["L_w 0.1 mm 격자"] = (not off, f"평가 {len(pts):,}점 중 이탈 {len(off)}")

    sl = [float(r["slenderness"]) for r in pts]
    edge = sum(1 for v in sl if v <= 1.5005 or v >= 2.4995)
    res["세장비 밴드"] = (all(1.5 - 1e-9 <= v <= 2.5 + 1e-9 for v in sl),
                          f"{min(sl):.4f} ~ {max(sl):.4f} · "
                          f"경계집중 {100*edge/len(sl):.1f}%")

    D = [float(r["D_mm"]) for r in pts]
    res["C6 외경 상한"] = (max(D) <= 5000 + 1e-6,
                           f"D {min(D):,.0f} ~ {max(D):,.0f} (≤ 5,000)")

    # C7 — `z2` 하한 3.5 에서 축방향 간섭 여유 (§10-7 ③)
    worst = min((float(r["z2"]) - 1.0) - (float(r["T_mm"]) / 1e3 + 0.1)
                for r in pts)
    res["C7 여유"] = (worst > 0, f"최소 {worst:.3f} m (z2−z1 − (T+0.1))")

    res["프론트"] = (len(front) > 1,
                     f"{len(front)}건 · D {min(float(r['D_mm']) for r in front):,.0f}"
                     f" ~ {max(float(r['D_mm']) for r in front):,.0f}")

    ok_all = True
    for k, (ok, msg) in res.items():
        print(f"  {'O' if ok else 'x'}  {k:16} {msg}")
        ok_all &= ok

    fz = sorted({float(r["z2"]) for r in front})
    print(f"  ·  {'z2 분포(기록)':16} 프론트 {len(fz)}수준 "
          f"{min(fz):.1f} ~ {max(fz):.1f} · 평가 "
          f"{len({float(r['z2']) for r in pts})}수준")
    ck = os.path.join(outdir, "s3_checkpoint.csv")
    print(f"  {'O' if os.path.isfile(ck) else 'x'}  {'체크포인트':16} "
          f"{'생성됨' if os.path.isfile(ck) else '없음'}")
    print(f"\n  판정: {'통과 — 본런 진행 가능' if ok_all else '실패 — 본런 보류'}")
    return ok_all


EVERY, MARK = 10, "A10:GENLOG"
DOC = os.path.join(os.path.dirname(os.path.dirname(HERE)),
                   "DLC기반_피로해석_사이징_최적화.md")


class LogDoc(Log10):
    """10세대마다 §10-11.1 실행요약표를 다시 쓴다"""

    def notify(self, algo):
        super().notify(algo)
        g = int(algo.n_gen)
        if g % EVERY and g != self.n_gen:
            return
        self._snap = getattr(self, "_snap", [])
        self._snap.append(dict(self.rows[-1]))
        # ΔHV 는 목적의 물리 단위가 섞인 부피라 절대값에 의미가 없다 —
        # **직전 표시 행 대비 백분율**을 함께 낸다.
        body = ["", "| 세대 | 프론트 | HV | **ΔHV** | **ΔHV / HV** | "
                "**최소 `D`** [mm] | **최소 베어링** [t] | "
                "**최소 총질량** [t] |",
                "|--:|--:|--:|--:|--:|--:|--:|--:|"]
        prev = None
        for r in self._snap:
            d = "—" if prev is None else f"{r['hv']-prev:+,.1f}"
            pct = "—" if prev is None else f"**{100*(r['hv']-prev)/prev:+.3f}%**"
            body.append(f"| {r['gen']} | {r['n_front']} | {r['hv']:,.1f} | {d} | "
                        f"{pct} | **{r['f1_min']*1e3:,.0f}** | "
                        f"**{r['f2_min']:.2f}** | **{r['f3_min']:.2f}** |")
            prev = r["hv"]
        body += ["", f"*{time.strftime('%m-%d %H:%M')} 기준 · "
                 f"{self._snap[-1]['gen']}/{self.n_gen} 세대 · HV 기준점 "
                 f"(5.5 m, 45 t, 250 t) · 셋 다 목적이다*"]
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


S3.GenLog = LogDoc
S3.SizingS3 = Sizing10
S3.CategorySeeding = Seed10
S3.dump_front = dump10
S3.dry_checks = dry10

if __name__ == "__main__":
    sys.exit(S3.main())
