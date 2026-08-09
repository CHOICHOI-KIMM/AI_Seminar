# -*- coding: utf-8 -*-
"""§10-12.7.9 (준비) — 세 프로파일의 ISO/TS 16281 피로수명 비교

§10-12.6 · §10-12.7 은 σ 와 길이방향 분포만 봤다. 「끝단이 닿되 가볍게」와
「끝단을 띄우되 안쪽이 세게」 중 무엇이 나은지 응력만으로는 갈리지 않으므로
(§10-12.7.8 ⑷), **수명이라는 단일 잣대**로 옮겨 본다.

부록 1(`run_appendix1_profile.py`)이 DIN 대 Johns-Gohar 를 111 DLC 로 비교한
그 골격을 그대로 쓴다 — 빈 대표하중 · dt=20 · 부록 4 스크리닝 k 재사용.

  대상   #1 · #103 · #210
  조건   ① 현행 DIN  ② Fujiwara 비대칭(§10-12.6)  ③ 비대칭 DIN(§10-12.7 확장)
  지표   ΣD30 (UW · DW) → 수명 [년]

주의 — 설계 3 x 조건 3 = 9회 완주는 부록 1 기준 설계당 ~8분씩이므로
**1시간 이상**이 걸린다. `--designs` · `--cases` 로 범위를 좁힐 수 있다.
"""
import csv
import io
import json
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import a10_profile_lib as L      # noqa: E402
import a10_asymdin2 as A2        # noqa: E402
import run_appendix1_profile as A1   # noqa: E402

OUT = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "life")
FUJI = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "fujiwara")
ASYM = A2.OUT
RANKS = (1, 103, 210)
CASES = ("din", "fuji", "adin")
LABEL = {"din": "현행 DIN", "fuji": "Fujiwara 비대칭",
         "adin": "비대칭 DIN"}


def apply_profile(rig, case, rk, geo, refs):
    """조건에 맞는 프로파일을 UW·DW 양쪽에 건다."""
    if case == "din":
        rig.set_din(0.0)
        return "k=1 (대칭)"
    if case == "fuji":
        t = refs["fuji"][rk]
        fn = L.profile_fn(geo["Lwe"], t["K1L"], t["K2L"], t["zmL"],
                          K1R=t["K1R"], K2R=t["K2R"], zmR=t["zmR"],
                          Q=t["Q"])
        rig.set_user(fn, A2.NPTS, targets=[rig.uw])
        return (f"K1 {t['K1L']:g}/{t['K1R']:g} · K2 {t['K2L']:g}/"
                f"{t['K2R']:g} · zm {t['zmL']*1e6:.0f}/{t['zmR']*1e6:.0f}um")
    p = refs["adin"][rk]
    fn = A2.asym_din2(geo["Lwe"], geo["D_we"], *p)
    rig.set_user(fn, A2.NPTS, targets=[rig.uw])
    return (f"k {p[0]:g}/{p[1]:g} · δ {p[2]:g} · f {p[3]:g}/{p[4]:g}")


def load_refs():
    FR = list(csv.DictReader(open(os.path.join(FUJI, "fujiwara_all.csv"),
                                  encoding="utf-8-sig")))
    QB = json.load(open(os.path.join(FUJI, "baseline.json"), encoding="utf-8"))
    fuji = {}
    for rk in RANKS:
        S = [r for r in FR if int(r["rank"]) == rk and r["stage"] == "S3"
             and r["feasible"] == "1"]
        if not S:
            continue
        t = max(S, key=lambda r: float(r["score"]))
        fuji[rk] = dict(K1L=float(t["K1L"]), K2L=float(t["K2L"]),
                        zmL=float(t["zmL_um"]) / 1e6, K1R=float(t["K1R"]),
                        K2R=float(t["K2R"]), zmR=float(t["zmR_um"]) / 1e6,
                        Q=QB[str(rk)]["P_max_N"])
    b2 = json.load(open(os.path.join(ASYM, "best2.json"), encoding="utf-8"))
    p3 = os.path.join(ASYM, "best3.json")
    b3 = json.load(open(p3, encoding="utf-8")) if os.path.exists(p3) else {}
    adin = {}
    for rk in RANKS:
        cand = [c for c in (b2.get(str(rk)), b3.get(str(rk))) if c]
        adin[rk] = max(cand, key=lambda c: c["s"][4])["p"]
    return dict(fuji=fuji, adin=adin)


K70 = os.path.join(HERE, "P2_피로수명_S4_70C", "screen_k.csv")


def load_k70():
    """§6-11.7 이 ν(70) = 137.178 로 다시 고른 DLC별 k (a01 기준)."""
    out = {}
    for r in csv.DictReader(io.open(K70, encoding="utf-8-sig")):
        if r.get("design") not in (None, "", "a01"):
            continue
        out[r["DLC"]] = (float(r["k"]), r["ksel"])
    return out


def set_70c(asm):
    """`Load Case 1` 의 베어링 3항만 70 °C 로. 복제가 이를 상속한다(§6-11.7)."""
    lc0 = next(c for c in asm.design_properties.static_loads
               if c.name == "Load Case 1")
    t = lc0.temperatures
    for a in ("rolling_bearing_element", "rolling_bearing_inner_race",
              "rolling_bearing_outer_race"):
        setattr(t, a, 70.0)
    print(f"[온도] 베어링 {t.rolling_bearing_element:.0f} °C · "
          f"샤프트 {t.shaft:.0f} °C · 하우징 {t.housing:.0f} °C "
          f"(use_default={lc0.use_default_temperatures})")


def main(designs, cases):
    os.makedirs(OUT, exist_ok=True)
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    refs = load_refs()
    rig = L.Rig()
    asm = rig.asm
    meta = A1.load_meta()
    kmap = load_k70()
    set_70c(asm)
    print(f"[대상] 설계 {designs} · 조건 {cases} · DLC {len(kmap)}건")
    rows, t0 = [], time.perf_counter()
    for rk in designs:
        rig.build(P[rk])
        d = rig.uw.detail
        geo = dict(Lwe=d.effective_roller_length, D_we=d.element_diameter)
        for case in cases:
            rig.build(P[rk])                     # 프로파일 초기화 포함
            desc = apply_profile(rig, case, rk, geo, refs)
            tag = f"{rk}{case}"
            print(f"\n=== #{rk} · {LABEL[case]} ({desc}) ===", flush=True)
            t1 = time.perf_counter()
            res = A1.run_fatigue(asm, rig.bs, kmap, meta, tag)
            sU = sum(v["D30_UW"] for v in res.values())
            sD = sum(v["D30_DW"] for v in res.values())
            lifeU = 30.0 / sU if sU > 0 else float("inf")
            lifeD = 30.0 / sD if sD > 0 else float("inf")
            e = 9.0 / 8.0
            lifeS = (lifeU ** -e + lifeD ** -e) ** (-1.0 / e)
            rows.append(dict(rank=rk, case=case, label=LABEL[case],
                             desc=desc, sumD30_UW=round(sU, 4),
                             sumD30_DW=round(sD, 4), life_UW=round(lifeU, 3),
                             life_DW=round(lifeD, 3), life_Sys=round(lifeS, 3),
                             minutes=round((time.perf_counter()-t1)/60, 1)))
            print(f"  ΣD30 UW {sU:.3f} · DW {sD:.3f} → 수명 UW "
                  f"{lifeU:.2f} · DW {lifeD:.2f} · Sys **{lifeS:.2f}년** "
                  f"({(time.perf_counter()-t1)/60:.1f}분)", flush=True)
            with open(os.path.join(OUT, "profile_life.csv"), "w", newline="",
                      encoding="utf-8-sig") as f:
                w = csv.DictWriter(f, fieldnames=list(rows[0]))
                w.writeheader()
                w.writerows(rows)
    print(f"\n[완료] {len(rows)}건 · {(time.perf_counter()-t0)/60:.1f}분 · "
          f"{OUT}")


if __name__ == "__main__":
    dz = [int(x) for x in sys.argv[1:] if x.isdigit()] or list(RANKS)
    cs = [x for x in sys.argv[1:] if x in CASES] or list(CASES)
    main(dz, cs)
