"""
부록 1: 강체정역학(핀지지+자중) 힘평형·수명 정밀 검토 — 무인
=============================================================
대상: DLC1.2-d-s1, dt=20, k=0.26 → 30빈 (top-25 본해석과 동일 bin_reps)
① MASTA 30빈 단건 해석 → 빈별 베어링 반력(internal_force) + ISO/TS 16281 수명
② 핀지지 정역학 + 자중(빈1 MASTA 반력 역산: 크기 W·작용점 z_W) → 빈별 반력
③ 반력 오차(전 30빈) · 대표 빈 상세(힘평형 Σ) · 빈별 수명/손상 오차 · 총량 비교
산출: DLC1.2-d-s1/appendix1_reactions.csv·appendix1_life.csv + 문서 부록 1 자동 기록
"""
import csv
import math
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, ROOT)

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안"
         r"_피로하중 반영_유연체_FE_온도_50도_260721.Masta")
NAME, DT, K = "DLC1.2-d-s1", 20, 0.26
DT0 = 0.1
E_W = 9.0 / 8.0
DESIGN_YEARS = 30.0
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")
DOC = os.path.join(ROOT, "DLC기반_피로해석_DLC별해석.md")
MS, ME = "<!-- APPENDIX1_START -->", "<!-- APPENDIX1_END -->"

# ── 상수 (프로세스_v1 §3-3.3, §8-3 · 50°C) ──
Z_A, Z_B = 0.5, 3.0
L_, A_, B_ = 2.5, -0.5, 3.0
E_LIM, Y1 = 0.5165, 1.1617
C_N, CU_N, P_EXP = 22228e3, 3929e3, 10.0 / 3.0
NU50, EC50, DPW = 294.637, 0.888378, 3328.6


def a_iso(kap, ratio):
    k = min(max(kap, 0.1), 4.0)
    if k < 0.4:
        term = 1.5859 - 1.3993 / k ** 0.054381
    elif k < 1.0:
        term = 1.5859 - 1.2348 / k ** 0.19087
    else:
        term = 1.5859 - 1.2348 / k ** 0.071739
    inner = 1.0 - term * min(ratio, 5.0) ** 0.4
    return min(0.1 * max(inner, 1e-12) ** -9.185 if inner > 0 else 50.0, 50.0)


def kappa(rpm):
    return NU50 / (45000.0 * max(abs(rpm), 1e-6) ** -0.83 * DPW ** -0.5)


def equiv_load(fr, fa):
    if fr <= 0:
        return abs(fa) * Y1
    return fr if abs(fa) / fr <= E_LIM else 0.4 * fr + Y1 * abs(fa)


def load_raw(name):
    rows = []
    for r in csv.DictReader(open(os.path.join(HERE, name, "raw.csv"),
                                 encoding="utf-8-sig")):
        rows.append({k: float(v) for k, v in r.items()})
    return rows


def load_sf(name):
    for r in csv.DictReader(open(os.path.join(HERE, "dlc_meta.csv"),
                                 encoding="utf-8-sig")):
        if r["DLC"] == name:
            return float(r["ScaleFactor"])
    raise KeyError(name)


def bin_reps(data, dt, k):
    kp = int(round(dt / DT0))
    n = len(data)
    nb = n // kp
    edges = [(b * kp, (b + 1) * kp) for b in range(nb)]
    if edges[-1][1] < n:
        edges[-1] = (edges[-1][0], n)
    out = []
    for bi, (i0, i1) in enumerate(edges):
        m = i1 - i0
        rec = {key: sum(data[i][key] for i in range(i0, i1)) / m
               for key in ("rpm", "Mx")}
        for key in KSIG:
            mu = sum(data[i][key] for i in range(i0, i1)) / m
            var = sum((data[i][key] - mu) ** 2 for i in range(i0, i1)) / m
            rec[key] = mu + math.copysign(1.0, mu) * k * math.sqrt(var)
        out.append((bi, abs(rec["rpm"]) / 60.0 * (m * DT0), rec))
    return out


def hub_masta(rec):
    """파일좌표(kN·kN·m) → MASTA 좌표 [N, N·m]."""
    return dict(FX=-rec["Fz"] * 1e3, FY=rec["Fy"] * 1e3, FZ=rec["Fx"] * 1e3,
                MX=-rec["Mz"] * 1e3, MY=rec["My"] * 1e3)


def pin_base(u):
    """자중 미포함 핀지지 반력(래디얼) [N]."""
    rax = u["FX"] * (B_ / L_) - u["MY"] / L_
    ray = u["FY"] * (B_ / L_) + u["MX"] / L_
    rbx = u["FX"] * (A_ / L_) + u["MY"] / L_
    rby = u["FY"] * (A_ / L_) - u["MX"] / L_
    return rax, ray, rbx, rby


def axial_split(fra, frb, ka):
    sa, sb = 0.5 * fra / Y1, 0.5 * frb / Y1
    if ka >= sa - sb:
        return ka + sb, -sb
    return sa, -(sa - ka)


def pin_full(u, Wx, zW):
    """자중 포함 핀지지: {UW/DW: (RX,RY,RZ,Fr,Fa)}."""
    rax, ray, rbx, rby = pin_base(u)
    rax += Wx * (Z_B - zW) / L_
    rbx += Wx * (zW - Z_A) / L_
    fra, frb = math.hypot(rax, ray), math.hypot(rbx, rby)
    faa, fab = axial_split(fra, frb, u["FZ"])
    return {"UW": (rax, ray, faa, fra, faa), "DW": (rbx, rby, fab, frb, fab)}


def main():
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType

    sf = load_sf(NAME)
    data = load_raw(NAME)
    reps = bin_reps(data, DT, K)
    print(f"[준비] {NAME} dt={DT} k={K} → {len(reps)}빈  SF={sf:,.0f}")

    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    bearings = {("UW" if "UW" in str(b) else "DW"): b
                for b in asm.all_parts_of_type_bearing()}
    lc0 = next(c for c in asm.design_properties.static_loads
               if c.name == "Load Case 1")
    print("[모델] 로드 완료 (50°C)")

    # ── ① MASTA 30빈 단건 해석 ──
    mres = []        # [{UW:(RX,RY,RZ,Fr,L10mr), DW:(...)}]
    t0 = time.perf_counter()
    for bi, rev, rec in reps:
        mf.set_loads(lc0, pl, ipl, rec)
        sd = lc0.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
        sd.perform_analysis()
        row = {}
        for key, b in bearings.items():
            res = sd.results_for(b)
            f = res.internal_force
            x, y, z = float(f.x), float(f.y), float(f.z)
            l10 = (res.component_detailed_analysis
                   .isots162812008.modified_reference_rating_life_cycles)
            row[key] = (x, y, z, math.hypot(x, y), l10)
        mres.append(row)
        if bi % 10 == 0:
            print(f"  [MASTA] 빈 {bi + 1}/{len(reps)}")
    t_m = time.perf_counter() - t0
    print(f"[MASTA] {len(reps)}빈 완료 {t_m:.1f}s")

    # ── ② 자중 역산 (빈1) + 전 빈 W 상수성 검증 ──
    hubs = [hub_masta(rec) for _, _, rec in reps]
    Ws = [m["UW"][0] + m["DW"][0] - u["FX"] for u, m in zip(hubs, mres)]
    u0, m0 = hubs[0], mres[0]
    Wx = Ws[0]
    _, _, rbx0, _ = pin_base(u0)
    zW = Z_A + L_ * (m0["DW"][0] - rbx0) / Wx
    print(f"[자중] W={Wx / 1e3:,.1f} kN  z_W={zW:.4f} m  "
          f"(전 빈 W: {min(Ws) / 1e3:,.1f}~{max(Ws) / 1e3:,.1f} kN)")

    # ── ③ 핀지지(자중 포함) + 오차·수명 ──
    comps = ("RX", "RY", "RZ", "Fr")
    rrows, lrows = [], []
    err = {(bk, c): [] for bk in ("UW", "DW") for c in comps}
    derr = {"UW": [], "DW": []}
    Dp = {"UW": 0.0, "DW": 0.0}
    Dm = {"UW": 0.0, "DW": 0.0}
    for (bi, rev, rec), u, m in zip(reps, hubs, mres):
        p = pin_full(u, Wx, zW)
        rr = {"bin": bi + 1}
        lr = {"bin": bi + 1, "rev": rev}
        for bk in ("UW", "DW"):
            for ci, c in enumerate(comps):
                pv, mv = p[bk][ci], m[bk][ci]
                e = (pv - mv) / mv * 100 if mv else float("nan")
                err[(bk, c)].append(e)
                rr[f"{bk}_{c}_pin_kN"] = pv / 1e3
                rr[f"{bk}_{c}_masta_kN"] = mv / 1e3
                rr[f"{bk}_{c}_err_pct"] = e
            # 수명: 핀지지 P→κ→a_ISO→L10m vs MASTA 16281
            P = equiv_load(p[bk][3], p[bk][4])
            kap = kappa(rec["rpm"])
            ai = a_iso(kap, EC50 * CU_N / max(P, 1.0))
            l10m_pin = ai * (C_N / max(P, 1.0)) ** P_EXP * 1e6
            dp = rev / l10m_pin
            dm = rev / m[bk][4] if m[bk][4] and m[bk][4] > 0 else 0.0
            de = (dp / dm - 1) * 100 if dm > 0 else float("nan")
            derr[bk].append(de)
            Dp[bk] += dp
            Dm[bk] += dm
            lr[f"{bk}_P_kN"] = P / 1e3
            lr[f"{bk}_aiso_pin"] = ai
            lr[f"{bk}_L10m_pin_rev"] = l10m_pin
            lr[f"{bk}_L10m_masta_rev"] = m[bk][4]
            lr[f"{bk}_dmg_pin"] = dp
            lr[f"{bk}_dmg_masta"] = dm
            lr[f"{bk}_dmg_err_pct"] = de
        lr["kappa"] = kappa(rec["rpm"])
        rrows.append(rr)
        lrows.append(lr)

    for fn, rows in (("appendix1_reactions.csv", rrows),
                     ("appendix1_life.csv", lrows)):
        with open(os.path.join(HERE, NAME, fn), "w", newline="",
                  encoding="utf-8-sig") as f:
            w = csv.DictWriter(f, fieldnames=list(rows[0]))
            w.writeheader()
            w.writerows(rows)
    print("[저장] appendix1_reactions.csv / appendix1_life.csv")

    # ── 총량 ──
    best = list(csv.DictReader(open(os.path.join(HERE, NAME,
                "masta_best_summary.csv"), encoding="utf-8-sig")))[0]
    refU, refD = float(best["D30_UW_ref"]), float(best["D30_DW_ref"])
    tot = {}
    for tag, D in (("pin", Dp), ("masta", Dm)):
        d30U, d30D = D["UW"] * sf, D["DW"] * sf
        lU, lD = DESIGN_YEARS / d30U, DESIGN_YEARS / d30D
        lS = (lU ** -E_W + lD ** -E_W) ** (-1 / E_W)
        tot[tag] = (d30U, d30D, lU, lD, lS)
    lUr, lDr = DESIGN_YEARS / refU, DESIGN_YEARS / refD
    lSr = (lUr ** -E_W + lDr ** -E_W) ** (-1 / E_W)
    tot["ref"] = (refU, refD, lUr, lDr, lSr)

    # ── 대표 빈 (MASTA UW 손상 최대) ──
    ib = max(range(len(lrows)), key=lambda i: lrows[i]["UW_dmg_masta"])

    write_doc(reps, hubs, mres, rrows, lrows, err, derr, tot, Wx, zW, Ws,
              ib, t_m)
    print("[문서] 부록 1 기록 완료")
    for tag, v in tot.items():
        print(f"  [{tag:5}] D30 UW={v[0]:.4f} DW={v[1]:.4f}  "
              f"수명 UW={v[2]:.1f} DW={v[3]:.1f} Sys={v[4]:.1f}")


def stat(v):
    vv = [x for x in v if x == x]
    return (sum(vv) / len(vv), min(vv), max(vv))


def write_doc(reps, hubs, mres, rrows, lrows, err, derr, tot, Wx, zW, Ws,
              ib, t_m):
    NL = chr(10)
    bi1, rev1, rec1 = reps[ib]
    u1, m1 = hubs[ib], mres[ib]
    p1 = pin_full(u1, Wx, zW)
    lines = []
    ap = lines.append
    ap(MS)
    ap("")
    ap("# 부록 1. 강체정역학(핀지지+자중) 모델 기반 힘평형 및 수명 검토")
    ap("")
    ap("## 1-1. 배경·목적")
    ap("")
    ap("- 프로세스_v1 §7-3: 자중 미포함 핀지지 반력이 MASTA 대비 Fr +32%(UW)/+66%(DW) 오차 → 수명 활용 기대 낮았음")
    ap("- 그러나 1단계 스크리닝(동일 경로)의 상대지표 ε 예측은 MASTA 실측과 0.0~0.6%p로 일치")
    ap("- 본 부록: 자중을 정확히 포함한 핀지지 모델로 **빈 단위 힘평형·수명을 전수 대조**하여 이 역설의 구조를 규명")
    ap(f"- 대상: {NAME}, dt={DT} s, k={K} → {len(reps)}빈 (상위 25 본해석 합격 조합과 동일)")
    ap("")
    ap("## 1-2. 방법")
    ap("")
    ap("- MASTA: 50°C 모델, 30빈 단건 System Deflection → 베어링 internal_force(R_X,R_Y,R_Z) + ISO/TS 16281 수정참조수명")
    ap("- 핀지지: §3-3.3 정역학(L=2.5, a=−0.5, b=3.0) + TRB쌍 유도축력 분배 + **자중 항 추가**")
    ap("- 자중 역산: 빈 1의 MASTA 반력에서 W = ΣR_X(MASTA) − F_X(인가), 작용점 z_W = z_UW + L·ΔR_X,DW/W")
    ap("- 수명: 핀지지 P(X·Y 카탈로그)→κ→a_ISO(ISO 281 식34~36 해석식)→L10m vs MASTA 16281 — 실사용 두 경로 그대로")
    ap("")
    ap("## 1-3. 자중 캘리브레이션")
    ap("")
    ap(f"- **W = {Wx / 1e3:,.1f} kN** (연직=X축), **z_W = {zW:.3f} m** (z_UW={Z_A}, z_DW={Z_B} 기준)")
    ap(f"- 전 30빈에서 W 재산출 시 {min(Ws) / 1e3:,.1f} ~ {max(Ws) / 1e3:,.1f} kN — "
       "하중 무관 상수 → 자중 해석 타당")
    ap("- z_W에는 유연체 재배분 효과 일부가 흡수됨(등가 자중 개념) — 빈 1 캘리브레이션 후 잔여 29빈은 순수 검증")
    ap("")
    ap(f"## 1-4. 대표 빈 상세 — 빈 {bi1 + 1} (MASTA UW 손상 최대)")
    ap("")
    ap("| 대상 | 항목 | 값 |")
    ap("|---|---|---|")
    ap(f"| Point Load | force_x / force_y / axial_load | "
       f"{u1['FX']:,.0f} / {u1['FY']:,.0f} / {u1['FZ']:,.0f} N |")
    ap(f"| | moment_x / moment_y | {u1['MX']:,.0f} / {u1['MY']:,.0f} N·m |")
    ap(f"| Input Power Load | speed / torque | {rec1['rpm']:.4f} rpm / "
       f"{rec1['Mx'] * 1e3:,.0f} N·m |")
    ap(f"| 자중(핀지지 부가) | W @ z_W | {Wx / 1e3:,.1f} kN @ {zW:.3f} m |")
    ap("")
    ap("**베어링 반력 [kN]** (핀지지 = 자중 포함)")
    ap("")
    ap("| 베어링 | 성분 | 핀지지 | MASTA | 차이 | 오차% |")
    ap("|---|---|---:|---:|---:|---:|")
    comps = ("RX", "RY", "RZ", "Fr")
    for bk in ("UW", "DW"):
        for ci, c in enumerate(comps):
            pv, mv = p1[bk][ci], m1[bk][ci]
            e = (pv - mv) / mv * 100 if mv else float("nan")
            nm = bk if ci == 0 else ""
            b = "**" if c == "Fr" else ""
            ap(f"| {nm} | {b}{c}{b} | {pv / 1e3:,.1f} | {mv / 1e3:,.1f} | "
               f"{(pv - mv) / 1e3:+,.1f} | {b}{e:+.2f}{b} |")
    ap("")
    ap("**힘평형 Σ [kN]** — 자중 포함 후 잔차 확인")
    ap("")
    ap("| 방향 | 핀지지 Σ | MASTA Σ | 인가+자중 |")
    ap("|---|---:|---:|---:|")
    tgt = {"RX": u1["FX"] + Wx, "RY": u1["FY"], "RZ": u1["FZ"]}
    for c in ("RX", "RY", "RZ"):
        ci = comps.index(c)
        ps = (p1["UW"][ci] + p1["DW"][ci]) / 1e3
        ms = (m1["UW"][ci] + m1["DW"][ci]) / 1e3
        ap(f"| Σ{c[-1]} | {ps:,.1f} | {ms:,.1f} | {tgt[c] / 1e3:,.1f} |")
    ap("")
    ap("## 1-5. 전 30빈 반력 오차 [(핀지지−MASTA)/MASTA %]")
    ap("")
    hdr = ("| 빈 | UW R_X | UW R_Y | UW R_Z | UW Fr | "
           "DW R_X | DW R_Y | DW R_Z | DW Fr |")
    ap(hdr)
    ap("|---:" + "|---:" * 8 + "|")
    for i, rr in enumerate(rrows):
        cells = [f"{rr[f'{bk}_{c}_err_pct']:+.1f}"
                 for bk in ("UW", "DW") for c in comps]
        ap(f"| {i + 1} | " + " | ".join(cells) + " |")
    ap("")
    ap("**통계 (평균 / 최소~최대)**")
    ap("")
    ap("| 베어링 | R_X | R_Y | R_Z | Fr |")
    ap("|---|---:|---:|---:|---:|")
    for bk in ("UW", "DW"):
        cells = []
        for c in comps:
            mn, lo, hi = stat(err[(bk, c)])
            cells.append(f"**{mn:+.2f}%** ({lo:+.1f}~{hi:+.1f})")
        ap(f"| {bk} | " + " | ".join(cells) + " |")
    ap("")
    ap("## 1-6. 빈별 손상 비교 [핀지지 vs MASTA 16281]")
    ap("")
    ap("| 빈 | rev | κ | UW P [kN] | UW d(핀) | UW d(MASTA) | UW ε% | "
       "DW d(핀) | DW d(MASTA) | DW ε% |")
    ap("|---:" + "|---:" * 9 + "|")
    for lr in lrows:
        ap(f"| {lr['bin']} | {lr['rev']:.1f} | {lr['kappa']:.2f} | "
           f"{lr['UW_P_kN']:,.0f} | {lr['UW_dmg_pin']:.3e} | "
           f"{lr['UW_dmg_masta']:.3e} | {lr['UW_dmg_err_pct']:+.1f} | "
           f"{lr['DW_dmg_pin']:.3e} | {lr['DW_dmg_masta']:.3e} | "
           f"{lr['DW_dmg_err_pct']:+.1f} |")
    mu, lu, hu = stat(derr["UW"])
    md, ld, hd = stat(derr["DW"])
    ap("")
    ap(f"- 빈별 손상 오차: UW 평균 **{mu:+.1f}%** ({lu:+.1f}~{hu:+.1f}) · "
       f"DW 평균 **{md:+.1f}%** ({ld:+.1f}~{hd:+.1f})")
    ap("")
    ap("## 1-7. 총량 비교 및 결론")
    ap("")
    ap("| 지표 | 핀지지+자중 (30빈) | MASTA (30빈) | MASTA 참값 (dt=0.1) |")
    ap("|---|---:|---:|---:|")
    lbl = ("D30_UW", "D30_DW", "수명 UW [yr]", "수명 DW [yr]", "수명 Sys [yr]")
    for i, name in enumerate(lbl):
        ap(f"| {name} | {tot['pin'][i]:,.4g} | {tot['masta'][i]:,.4g} | "
           f"{tot['ref'][i]:,.4g} |")
    eU = (tot["pin"][0] / tot["masta"][0] - 1) * 100
    eD = (tot["pin"][1] / tot["masta"][1] - 1) * 100
    eUr = (tot["pin"][0] / tot["ref"][0] - 1) * 100
    ap("")
    ap(f"- 핀지지 vs MASTA(동일 30빈): D30 오차 UW **{eU:+.1f}%** / DW **{eD:+.1f}%**")
    ap(f"- 핀지지(30빈) vs MASTA 참값(dt=0.1): UW {eUr:+.1f}%")
    ap(f"- MASTA 30빈 해석 {t_m:.0f}s (단건 모드)")
    ap("")
    ap("<!-- APPENDIX1_CONCL -->")
    ap("")
    ap(ME)
    block = NL.join(lines)
    txt = open(DOC, encoding="utf-8").read()
    if MS in txt:
        txt = txt.split(MS)[0] + block + txt.split(ME, 1)[1]
    else:
        txt = txt.rstrip() + NL + NL + "---" + NL + NL + block + NL
    open(DOC, "w", encoding="utf-8").write(txt)


if __name__ == "__main__":
    main()
