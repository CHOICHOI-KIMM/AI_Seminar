"""
P1 Phase 2 후처리 집계 (문서 §8-2.5)
=====================================
p1_grid.csv → 세 가지 산출물

  p1_feasible.csv   가능해(σ_max < 2,100)만, 질량 오름차순
  p1_marginal.csv   변수 수준별 σ 통계·가능률 — 방향성 파악용
  p1_selfcheck.txt  행수·중복키·warn·σ=0·소요시간 요약

사용:  python summarize_p1_phase2.py
"""
import collections
import csv
import os
import statistics as st

HERE = os.path.dirname(os.path.abspath(__file__))
DIR = os.path.join(HERE, "P1_극한응력_Phase2")
SRC = os.path.join(DIR, "p1_grid.csv")
LIMIT = 2100.0
VARS = ["z1", "z2", "D_pw_mm", "alpha", "D_we_mm", "L_we_mm", "slenderness"]
KEEP = ["S_rank", "z1", "z2", "D_pw_mm", "alpha", "D_we_mm", "L_we_mm",
        "slenderness", "Z",
        "bore_mm", "D_mm", "T_mm", "t_i_mm", "t_o_mm", "c12_margin",
        "L_eff_m", "mass_brg_kg", "mass_shaft_kg", "mass_total_kg",
        "Myz_max_UW", "Myz_max_DW", "sigma_max_MPa"]


def f(r, k):
    try:
        return float(r[k])
    except (TypeError, ValueError):
        return None


def main():
    with open(SRC, encoding="utf-8-sig") as fh:
        rows = list(csv.DictReader(fh))
    if not rows:
        raise SystemExit("[중단] p1_grid.csv 가 비어 있다")
    for r in rows:
        r["_sig"] = f(r, "sigma_max_MPa") or 0.0
        r["_m"] = f(r, "mass_total_kg") or 0.0
    ok = [r for r in rows if 0 < r["_sig"] < LIMIT]

    # ── 1. 가능해 ──
    ok.sort(key=lambda r: r["_m"])
    with open(os.path.join(DIR, "p1_feasible.csv"), "w", newline="",
              encoding="utf-8-sig") as fh:
        w = csv.DictWriter(fh, fieldnames=["rank_mass"] + KEEP)
        w.writeheader()
        for i, r in enumerate(ok, 1):
            w.writerow({"rank_mass": i, **{k: r[k] for k in KEEP}})

    # ── 2. 변수 수준별 한계효과 ──
    mrows = []
    for v in VARS:
        lv = collections.defaultdict(list)
        for r in rows:
            lv[f(r, v)].append(r)
        for val in sorted(x for x in lv if x is not None):
            g = lv[val]
            sg = [r["_sig"] for r in g if r["_sig"] > 0]
            gk = [r for r in g if 0 < r["_sig"] < LIMIT]
            mrows.append(dict(
                variable=v, level=val, n=len(g), n_feasible=len(gk),
                feasible_pct=round(100.0 * len(gk) / len(g), 1) if g else 0.0,
                sigma_min=round(min(sg), 1) if sg else "",
                sigma_mean=round(st.fmean(sg), 1) if sg else "",
                sigma_max=round(max(sg), 1) if sg else "",
                mass_min_t=round(min(r["_m"] for r in gk) / 1000, 1) if gk else "",
                mass_mean_t=round(st.fmean([r["_m"] for r in gk]) / 1000, 1) if gk else ""))
    with open(os.path.join(DIR, "p1_marginal.csv"), "w", newline="",
              encoding="utf-8-sig") as fh:
        w = csv.DictWriter(fh, fieldnames=list(mrows[0]))
        w.writeheader()
        w.writerows(mrows)

    # ── 3. 자기점검 ──
    keys = [r["idx"] for r in rows]
    dup = [k for k, c in collections.Counter(keys).items() if c > 1]
    warn = [r for r in rows if r.get("warn")]
    zero = [r for r in rows if r["_sig"] <= 0]
    ts = [f(r, "t_s") or 0.0 for r in rows]
    c12 = [f(r, "c12_margin") for r in rows if f(r, "c12_margin") is not None]
    gov = collections.Counter(r.get("governing", "") for r in rows)
    L = []
    L.append("P1 Phase 2 자기점검")
    L.append("=" * 60)
    L.append(f"행 수                 {len(rows):,}")
    L.append(f"고유 키               {len(set(keys)):,}   중복 {len(dup)}건")
    L.append(f"σ=0 (해석 무효)       {len(zero)}건")
    L.append(f"warn 있는 행          {len(warn)}건")
    L.append(f"지배 케이스 분포      {dict(gov)}")
    L.append("")
    L.append(f"가능해 (σ < {LIMIT:.0f})    {len(ok):,}건 ({100.0*len(ok)/len(rows):.1f}%)")
    if ok:
        L.append(f"  σ 범위              {min(r['_sig'] for r in ok):.1f} ~ "
                 f"{max(r['_sig'] for r in ok):.1f} MPa")
        L.append(f"  질량 범위           {min(r['_m'] for r in ok)/1000:.1f} ~ "
                 f"{max(r['_m'] for r in ok)/1000:.1f} t")
        b = ok[0]
        L.append(f"  최경량 가능해       {b['mass_total_kg']} kg · σ {b['sigma_max_MPa']} MPa")
        L.append(f"                      D_pw {b['D_pw_mm']} · α {b['alpha']} · "
                 f"D_we {b['D_we_mm']} · L_we {b['L_we_mm']} · z {b['z1']}/{b['z2']}")
    sg = [r["_sig"] for r in rows if r["_sig"] > 0]
    L.append("")
    L.append(f"σ 전체 분포           {min(sg):.1f} ~ {max(sg):.1f} MPa "
             f"(중앙 {st.median(sg):.1f})")
    L.append(f"C12 여유 최소         ×{min(c12):.3f}" if c12 else "")
    L.append(f"점당 소요             평균 {st.fmean(ts):.2f} s · 최대 {max(ts):.2f} s")
    L.append(f"총 소요               {sum(ts)/60:.1f} 분")
    if dup:
        L.append("")
        L.append(f"!! 중복 키 {len(dup)}건: {dup[:5]}")
    if warn:
        L.append("")
        L.append("!! warn 상위 5건")
        for r in warn[:5]:
            L.append(f"   {r['idx']}  {r['warn'][:70]}")
    txt = "\n".join(L)
    with open(os.path.join(DIR, "p1_selfcheck.txt"), "w", encoding="utf-8") as fh:
        fh.write(txt + "\n")
    print(txt)
    print(f"\n[저장] p1_feasible.csv ({len(ok):,}행) · p1_marginal.csv "
          f"({len(mrows)}행) · p1_selfcheck.txt")


if __name__ == "__main__":
    main()
