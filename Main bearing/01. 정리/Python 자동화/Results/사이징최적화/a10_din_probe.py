# -*- coding: utf-8 -*-
"""§10-12.7 사전조사 — MASTA DIN Lundberg 의 정확한 정의를 201점 전수로 확정

확인할 것
  ① 형상이 순수 로그식  z = A·ln[1/(1−(2y/L_we)²)]  인가 (199개 내점 전수)
  ② 계수 A 가 전 점에서 일정한가 (표준편차)
  ③ 물리적 끝단 낙차 z_end 가 부록 2 의 0.00221·D_we 와 일치하는가
  ④ 절단 배율 K = z_end / A 가 설계와 무관하게 일정한가
  ⑤ 응력 44점 배열의 라미나 구조 (폭·개수) — margin 을 라미나 수로도 재려면 필요
"""
import csv
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import a10_profile_lib as L      # noqa: E402

OUT = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "din")
RANKS = (1, 103, 210)
END_OVER_DWE = 0.00221           # 부록 2 §2-6 실측


def main():
    os.makedirs(OUT, exist_ok=True)
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    rig = L.Rig()
    rig.load_case()
    res = {}

    for rk in RANKS:
        rig.build(P[rk])
        rig.set_din(0.0)
        d = rig.uw.detail
        half = d.effective_roller_length * 1e3 / 2.0
        D_we = d.element_diameter * 1e3
        pr = [(float(p.offset_from_roller_centre) * 1e3,
               float(p.roller_deviation) * 1e6)
              for p in d.outer_race_and_roller_profiles]

        # ── ①② 내점 전수에서 A 역산 ──────────────────────────
        As = []
        for y, z in pr:
            if abs(y) < 1e-9 or abs(abs(y) - half) < 1e-9:
                continue                     # 중앙(0/0) · 끝단(발산) 제외
            u = (y / half) ** 2
            if u >= 1.0:
                continue
            As.append(z / math.log(1.0 / (1.0 - u)))
        n = len(As)
        A = sum(As) / n
        sd = (sum((x - A) ** 2 for x in As) / n) ** 0.5
        rel = 100.0 * sd / A

        # ── ③ 끝단 낙차 ──────────────────────────────────────
        z_end = pr[-1][1]
        z_pred = END_OVER_DWE * D_we * 1e3     # [um]  (D_we 는 mm)
        # ── ④ 절단 배율 ──────────────────────────────────────
        K = z_end / A
        # K 를 등가 평가위치로 환산 :  1-(1-eps)^2 = exp(-K)
        eps = 1.0 - math.sqrt(1.0 - math.exp(-K))

        # ── ⑤ 라미나 구조 ────────────────────────────────────
        m, off = rig.solve(f"din_{rk}")
        xs = [p for p, _ in off]
        # 중복 좌표(접촉 경계 표시)를 접어 고유 좌표만
        uniq = []
        for x in xs:
            if not uniq or abs(x - uniq[-1]) > 1e-9:
                uniq.append(x)
        w = [round(uniq[i + 1] - uniq[i], 4) for i in range(len(uniq) - 1)]
        live = [(p, s) for p, s in off if s > 1e-9]

        res[rk] = dict(
            L_we=round(2 * half, 2), half=round(half, 3), D_we=round(D_we, 2),
            n_fit=n, A_um=round(A, 5), A_sd_um=round(sd, 8),
            A_rel_pct=round(rel, 8),
            z_end_um=round(z_end, 4), z_end_pred_um=round(z_pred, 4),
            z_end_ratio=round(z_end / (D_we * 1e3), 8),
            K=round(K, 6), eps=round(eps, 8),
            n_pts_stress=len(off), n_uniq=len(uniq),
            w_end_L=w[0] if w else None, w_end_R=w[-1] if w else None,
            w_mid=round(sorted(w)[len(w) // 2], 4) if w else None,
            sigma_MPa=m["sigma_MPa"],
            first_live=round(live[0][0], 2) if live else None,
            last_live=round(live[-1][0], 2) if live else None)

        print("=" * 76)
        print(f"#{rk} · L_we {2*half:.1f} · D_we {D_we:.1f}")
        print(f"  ① 로그식 적합 {n}점 · A = {A:.5f} um · 표준편차 "
              f"{sd:.2e} um ({rel:.2e} %)")
        print(f"  ③ 끝단 z_end = {z_end:.4f} um · 부록 2 예측 "
              f"0.00221·D_we = {z_pred:.4f} um · 비 "
              f"{z_end/(D_we*1e3):.8f}")
        print(f"  ④ 절단 배율 K = z_end/A = {K:.6f} "
              f"(등가 평가위치 |y|/반길이 = {1-eps:.8f})")
        print(f"  ⑤ 응력 배열 {len(off)}점 · 고유좌표 {len(uniq)}개 · "
              f"끝 라미나 폭 {w[0]:.3f} / {w[-1]:.3f} · 중앙 폭 "
              f"{sorted(w)[len(w)//2]:.3f} mm")
        print(f"     접촉 {live[0][0]:+.2f} ~ {live[-1][0]:+.2f} · σ_max "
              f"{m['sigma_MPa']:.1f}")

    print("\n" + "=" * 76)
    print("교차검증 요약")
    print("=" * 76)
    print(f"{'설계':>6s} {'A [um]':>12s} {'A 편차':>12s} {'z_end':>11s} "
          f"{'z_end/D_we':>12s} {'K':>10s}")
    for rk in RANKS:
        g = res[rk]
        print(f"{rk:6d} {g['A_um']:12.5f} {g['A_rel_pct']:11.2e}% "
              f"{g['z_end_um']:11.4f} {g['z_end_ratio']:12.8f} "
              f"{g['K']:10.6f}")
    Ks = [res[r]["K"] for r in RANKS]
    rr = [res[r]["z_end_ratio"] for r in RANKS]
    print(f"\n  K   범위 {min(Ks):.6f} ~ {max(Ks):.6f} · 폭 "
          f"{max(Ks)-min(Ks):.2e}")
    print(f"  비  범위 {min(rr):.8f} ~ {max(rr):.8f} "
          f"(부록 2 = {END_OVER_DWE})")
    json.dump({str(k): v for k, v in res.items()},
              open(os.path.join(OUT, "din_definition.json"), "w"), indent=1)
    print("\n[저장]", os.path.join(OUT, "din_definition.json"))


if __name__ == "__main__":
    main()
