"""
부록 6 S3-c — 프론트 분석
============================
본최적화 결과를 §8-4.4.4 전수 파레토와 대조하고, 미결 항목(격자 상한 부착·
`Z` 계단·세장비 경계)을 실측으로 답한다.

산출: 부록6_NSGA/S3_본최적화/s3_analysis.md (표 원문) + 화면 요약
"""
import csv
import os
import sys

import numpy as np
from pymoo.indicators.hv import HV

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import nsga_s3_run as S3        # noqa: E402

OUT = os.path.join(HERE, "부록6_NSGA", "S3_본최적화")
FRONT = os.path.join(OUT, "s3_front.csv")
GENLOG = os.path.join(OUT, "s3_genlog.csv")
EXH = os.path.join(HERE, "P1_극한응력_Phase3", "p1_pareto.csv")


def rd(p):
    with open(p, encoding="utf-8-sig") as f:
        return list(csv.DictReader(f))


def f(r, k):
    return float(r[k])


def dedupe(rows):
    """설계변수가 같은 행을 하나로 접는다.

    `eliminate_duplicates=None`(§6-8 ▸) 이라 여러 `u` 가 같은 `L_we` 로
    환산된 개체가 프론트에 그대로 남는다. 목적값도 당연히 같으므로
    **보고 단계에서 접는다** — 탐색 자체에는 영향이 없다.
    """
    seen, out = set(), []
    for r in rows:
        k = (r["z1"], r["z2"], r["D_pw_mm"], r["alpha"], r["D_we_mm"],
             r["L_we_mm"])
        if k not in seen:
            seen.add(k)
            out.append(r)
    return out


def main():
    F_raw = rd(FRONT)
    F = dedupe(F_raw)
    G = rd(GENLOG)
    E = [r for r in rd(EXH) if r.get("subset") == "z1>=1.0"]

    nf = np.array([[f(r, "mass_brg_t"), f(r, "mass_total_t")] for r in F])
    ex = np.array([[f(r, "mass_brg_kg") / 1e3, f(r, "mass_total_kg") / 1e3]
                   for r in E])
    hv = HV(ref_point=S3.HV_REF)
    out = []

    def say(s=""):
        print(s)
        out.append(s)

    say(f"# S3-c 프론트 분석 — 고유 설계 {len(F)}건 "
        f"(프론트 행 {len(F_raw)} · 중복 {len(F_raw)-len(F)} 제거)")
    say()
    say("## 전수(§8-4.4.4) 대비")
    say()
    say("| 지표 | 전수 (8,700점) | NSGA (33,600 평가) | 개선 |")
    say("|---|--:|--:|--:|")
    for i, nm in enumerate(("베어링 1개 질량 [t]", "총질량 [t]")):
        a, b = ex[:, i].min(), nf[:, i].min()
        say(f"| 최소 {nm} | {a:.3f} | **{b:.3f}** | {100*(b-a)/a:+.1f}% |")
    say(f"| 프론트 점수 | {len(ex)} | **{len(nf)}** | ×{len(nf)/len(ex):.1f} |")
    say(f"| 하이퍼볼륨 | {hv(ex):,.1f} | **{hv(nf):,.1f}** | "
        f"{100*(hv(nf)-hv(ex))/hv(ex):+.2f}% |")
    say()

    # 전수 프론트 각 점이 NSGA 프론트에 지배되는가
    dom = sum(1 for e in ex if np.any((nf[:, 0] <= e[0]) & (nf[:, 1] <= e[1])
                                      & ((nf[:, 0] < e[0]) | (nf[:, 1] < e[1]))))
    say(f"**전수 파레토 {len(ex)}건 중 {dom}건이 NSGA 프론트에 지배된다.** "
        f"나머지 {len(ex)-dom}건은 NSGA 가 재현했거나 그 구간을 비워 둔 것이다.")
    say()

    say("## 설계변수 분포")
    say()
    say("| 변수 | 최소 | 최대 | 최빈/비고 |")
    say("|---|--:|--:|---|")
    for k, lab, lo, hi in (("D_pw_mm", "D_pw [mm]", 3300, 4500),
                           ("D_we_mm", "D_we [mm]", 110, 230),
                           ("L_we_mm", "L_we [mm]", 175, 550),
                           ("alpha", "α [°]", 15, 30),
                           ("z1", "z1 [m]", 1.0, 1.5),
                           ("z2", "z2 [m]", 3.0, 6.0),
                           ("slenderness", "세장비", 1.5, 2.5)):
        v = np.array([f(r, k) for r in F])
        at_lo = 100 * np.mean(np.isclose(v, lo, atol=1e-6))
        at_hi = 100 * np.mean(np.isclose(v, hi, atol=1e-6))
        note = f"하한부착 {at_lo:.0f}% · 상한부착 {at_hi:.0f}%"
        say(f"| {lab} | {v.min():,.4g} | {v.max():,.4g} | {note} |")
    say()

    z = np.array([int(r["Z"]) for r in F])
    sg_ = np.array([f(r, "sigma_max_MPa") for r in F])
    say(f"- **롤러 수 `Z`** {z.min()} ~ {z.max()} · 서로 다른 값 {len(set(z))}종")
    say(f"- **σ_max** {sg_.min():,.1f} ~ {sg_.max():,.1f} MPa "
        f"(한도 2,100 · 2,090 초과 {100*np.mean(sg_ > 2090):.0f}%)")
    say()

    say("## 수렴 (세대별 하이퍼볼륨)")
    say()
    say("| 세대 | HV | 프론트 | 최소 베어링 [t] | 최소 총질량 [t] |")
    say("|--:|--:|--:|--:|--:|")
    for g in G:
        if int(g["gen"]) in (1, 5, 25, 50, 75, 100, 104, 125, 150):
            say(f"| {g['gen']} | {float(g['hv']):,.1f} | {g['n_front']} | "
                f"{float(g['f1_min']):.3f} | {float(g['f2_min']):.3f} |")
    hvs = np.array([float(g["hv"]) for g in G])
    last_gain = next((int(G[i]["gen"]) for i in range(len(hvs) - 1, 0, -1)
                      if hvs[i] - hvs[i - 1] > 0.5), None)
    say()
    say(f"**HV 가 0.5 이상 오른 마지막 세대는 {last_gain} 이다** "
        f"(최종 {hvs[-1]:,.2f}).")
    say()

    say("## 최경량 5건 (베어링 질량 기준)")
    say()
    cols = ["mass_brg_t", "mass_total_t", "D_pw_mm", "alpha", "D_we_mm",
            "L_we_mm", "z1", "z2", "Z", "slenderness", "sigma_max_MPa"]
    say("| " + " | ".join(cols) + " |")
    say("|" + "--:|" * len(cols))
    for r in F[:5]:
        say("| " + " | ".join(str(r[c]) for c in cols) + " |")

    with open(os.path.join(OUT, "s3_analysis.md"), "w",
              encoding="utf-8") as fh:
        fh.write("\n".join(out) + "\n")
    print(f"\n[저장] {os.path.join(OUT, 's3_analysis.md')}")


if __name__ == "__main__":
    main()
