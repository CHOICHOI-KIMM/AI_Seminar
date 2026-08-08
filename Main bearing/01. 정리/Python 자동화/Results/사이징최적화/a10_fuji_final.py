# -*- coding: utf-8 -*-
"""§10-12.6 마무리 — 유효 여유 실측 · 분포/형상 수집 · 그림

  ① 기준(DIN) · 대칭 최적 · 비대칭 최적 · K2<=0.5 최적 을 각각 다시 풀어
     길이방향 분포를 통째로 저장한다.
  ② 「유효 여유」 = 롤러 좌단에서 응력이 σ_max 의 10/25/50 % 를 처음 넘는
     지점까지의 거리. 1 MPa 임계의 margin_L 이 0 이어도 이 값은 살아 있다.
  ③ 프로파일 형상 + 응력분포 2단 그림.
"""
import csv
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import a10_profile_lib as L      # noqa: E402

OUTD = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "fujiwara")
SRC = os.path.join(OUTD, "fujiwara_all.csv")
RANKS = (1, 103, 210)
NPTS = 61
FRACS = (0.10, 0.25, 0.50)


def margins(off, half):
    """유효 여유 — 좌단에서 σ_max 의 f 배를 처음 넘는 지점까지 [mm]"""
    if not off:
        return {}
    smax = max(s for _, s in off)
    out = {}
    for f in FRACS:
        lim = f * smax
        hit = [p for p, s in off if s >= lim]
        out[f"m{int(f*100)}_L"] = round(hit[0] + half, 2) if hit else None
        out[f"m{int(f*100)}_R"] = round(half - hit[-1], 2) if hit else None
    return out


def best_row(R, rk, stage, k2max=None):
    S = [r for r in R if int(r["rank"]) == rk and r["stage"] == stage
         and r["feasible"] == "1"]
    if k2max is not None:
        S = [r for r in S if float(r["K2L"]) <= k2max + 1e-9
             and float(r["K2R"]) <= k2max + 1e-9]
    if not S:
        return None
    return max(S, key=lambda r: float(r["score"]))


def par(r):
    return (float(r["K1L"]), float(r["K2L"]), float(r["zmL_um"]) / 1e6,
            float(r["K1R"]), float(r["K2R"]), float(r["zmR_um"]) / 1e6)


def main():
    R = list(csv.DictReader(open(SRC, encoding="utf-8-sig")))
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    rig = L.Rig()
    rig.load_case()
    out = {}
    for rk in RANKS:
        rig.build(P[rk])
        half = None
        cases = [("기준 (DIN Lundberg)", None)]
        for tag, st, k2 in (("대칭 최적", "S1", None),
                            ("비대칭 최적", "S3", None),
                            ("비대칭 · K2<=0.5", None, 0.5)):
            if k2 is not None:
                cand = [best_row(R, rk, s, 0.5) for s in ("S3", "S2", "S1")]
                cand = [c for c in cand if c]
                r = (max(cand, key=lambda x: float(x["score"]))
                     if cand else None)
            else:
                r = best_row(R, rk, st)
            if r:
                cases.append((tag, r))
        rec = []
        for tag, r in cases:
            if r is None:
                rig.set_din(0.0)
                prof = None
            else:
                p = par(r)
                Q = None
                f = L.profile_fn(rig.Lwe, p[0], p[1], p[2], K1R=p[3],
                                 K2R=p[4], zmR=p[5],
                                 Q=float(json.load(open(os.path.join(
                                     OUTD, "baseline.json")))[str(rk)]
                                     ["P_max_N"]))
                prof = rig.set_user(f, NPTS)
                del Q
            m, off = rig.solve(f"fin{rk}_{len(rec)}")
            half = m["L_we_mm"] / 2.0
            mm = margins(off, half)
            rec.append(dict(tag=tag, params=(par(r) if r else None),
                            metrics=m, margins=mm, dist=off,
                            profile=[(round(y * 1e3, 3), round(z * 1e6, 3))
                                     for y, z in prof] if prof else None))
            print(f"  #{rk:<4} {tag:<18} σ {m['sigma_MPa']:7.1f} · edge_L "
                  f"{m['edge_L_MPa']:7.1f} · y* "
                  f"{str(m['y_star_mm']):>8s} · 유효여유 10% "
                  f"{str(mm.get('m10_L')):>7s} · 25% "
                  f"{str(mm.get('m25_L')):>7s} · 50% "
                  f"{str(mm.get('m50_L')):>7s}", flush=True)
        out[rk] = rec
    json.dump(out, open(os.path.join(OUTD, "final.json"), "w"), indent=1)
    print("\n[저장]", os.path.join(OUTD, "final.json"))


if __name__ == "__main__":
    main()
