"""
부록 7-6.7.6 — 샤프트 두께 규칙을 적용한 파레토 프론트 재검토
================================================================
§6-11.5a 의 프론트 64건에 **목표 안전율 5.2 의 정확형 규칙**을 적용해 샤프트
내경을 다시 잡고, MASTA 로 다시 돌려 질량과 응력을 실측한다.

  ID = floor( (OD⁴ − 32·W·OD/π)^¼ )      W = 1.393 × 10⁹ mm³

`floor` 를 쓰는 이유는 §6-4.2 와 같다 — 내경을 깎아 내려야 살두께가 두꺼워져
안전측이다. `round` 로 하면 64건 중 32건이 목표 5.2 를 밑돈다.

**MASTA 재해석이 필요한 이유** — 두께가 바뀌면 샤프트 강성이 달라지고, 유연체
FE 모델에서는 처짐이 베어링 하중 분포를 바꿔 **`σ_max` 도 달라진다.** 질량만
해석식으로 고치면 σ ≤ 2,100 제약이 유지되는지 알 수 없다.

총질량이 설계마다 다른 폭으로 줄므로 **지배 관계가 깨질 수 있다.** 64건을 다시
비지배 판정해 새 프론트를 뽑는다.

산출: 부록7_샤프트/front_w52.csv · figures/pareto_w52.{png,svg} + 문서 §7-6.7.6
"""
import csv
import io
import math
import os
import re
import sys
import time

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt                  # noqa: E402
from matplotlib.ticker import MultipleLocator    # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg          # noqa: E402
import run_appendix7_shaft as a7  # noqa: E402

W52 = 1.3930e9                    # mm³ — 목표 SF 5.2 (§7-6.7.5)
LIMIT = 2100.0                    # MPa
GOV, TQ = "Myz_max", 22673.0
SRC = os.path.join(HERE, "부록6_NSGA", "S3_본최적화", "s3_pareto.csv")
OUT = os.path.join(HERE, "부록7_샤프트", "front_w52.csv")
FIG = os.path.join(HERE, "figures")
DOC = a7.DOC
MARK = "<!-- A7:FRONT52 -->"

plt.rcParams.update({"font.family": "Malgun Gothic",
                     "axes.unicode_minus": False, "font.size": 10.5})


def new_id(od_mm):
    """정확형 + floor — §6-4.2 와 같은 안전측 정수화"""
    return math.floor((od_mm ** 4 - 32 * W52 * od_mm / math.pi) ** 0.25)


def pareto(rows, key):
    P = sorted(rows, key=lambda r: (r["mass_brg_t"], r[key]))
    out, best = [], float("inf")
    for r in P:
        if r[key] < best:
            out.append(r)
            best = r[key]
    return out


def sc(o, n):
    try:
        v = getattr(o, n)
    except Exception:
        return None
    if isinstance(v, (int, float)) and not isinstance(v, bool):
        return float(v)
    for a in ("value", "wrapped"):
        try:
            w = getattr(v, a)
        except Exception:
            continue
        if isinstance(w, (int, float)) and not isinstance(w, bool):
            return float(w)
    return None


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import (
        AnalysisType)
    from mastapy.bearings import RollerBearingProfileTypes as RP

    with open(SRC, encoding="utf-8-sig") as f:
        F = [r for r in csv.DictReader(f) if r["subset"] == "z1>=1.0"]
    print(f"[대상] §6-11.5a 프론트 {len(F)}건 · W = {W52/1e9:.4f}e9 mm³")

    d = Design.load(a7.MODEL)
    asm = d.all_parts_of_type_root_assembly()[0]
    sh = list(asm.all_parts_of_type_shaft())[0]
    bs = list(asm.all_parts_of_type_bearing())
    uw = [b for b in bs if "UW" in str(b)][0]
    dw = [b for b in bs if "DW" in str(b)][0]
    for b in bs:
        b.detail.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG
    dp = asm.design_properties
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    lc = next(c for c in dp.static_loads if c.name == GOV)
    q = lc.inputs_for_power_load(ipl)
    for a, v in (("speed", 0.0), ("torque", TQ * 1e3)):
        try:
            setattr(q, a, v)
        except Exception:
            pass
    ds = lc.design_state_load_case_group

    rows, t0 = [], time.perf_counter()
    for i, r in enumerate(F, 1):
        rank = int(r["rank_pareto"])
        z1, z2 = float(r["z1"]), float(r["z2"])
        g = dict(D_pw=float(r["D_pw_mm"]) / 1e3, alpha_deg=float(r["alpha"]),
                 D_we=float(r["D_we_mm"]) / 1e3, L_we=float(r["L_we_mm"]) / 1e3,
                 bore=float(r["bore_mm"]) / 1e3,
                 outer_diameter=float(r["D_mm"]) / 1e3,
                 width=float(r["T_mm"]) / 1e3,
                 inner_ring_width=float(r["B_mm"]) / 1e3,
                 outer_ring_width=float(r["C_mm"]) / 1e3,
                 number_of_elements=int(float(r["Z"])))
        od = float(r["bore_mm"])
        idm = new_id(od)
        for b in bs:
            try:
                if b.inner_connection is not None:
                    b.inner_connection.delete()
            except Exception:
                pass
        sh.remove_all_sections()
        sh.add_section(0.0, z2 + sg.SHAFT_TAIL, od / 1e3, idm / 1e3,
                       od / 1e3, idm / 1e3)
        for b in bs:
            sg.apply_to_masta(b.detail, g)
        for b, z in ((uw, z1), (dw, z2)):
            b.try_mount_on(sh, z)

        def solve(tag):
            """현재 샤프트 상태로 지배 LC 해석 → σ(두 베어링 최대)·UW 미스얼라인먼트"""
            dup = lc.duplicate(ds, tag)
            duty = dp.add_duty_cycle(f"{tag}d")
            duty.add_static_load(dup)
            csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            csd.perform_analysis()
            smax, mis = 0.0, None
            for b in (uw, dw):
                for sub in list(
                        list(csd.results_for(b))[0].component_analysis_cases):
                    det = sub.component_detailed_analysis
                    v = sc(det, "maximum_normal_stress")
                    if v and v / 1e6 > smax:
                        smax = v / 1e6
                    if b is uw:
                        # 상대 미스얼라인먼트 [rad] → mrad. 피로·응력 모두
                        # UW 가 지배적이므로 UW 기준으로 통일한다
                        m = sc(det, "relative_misalignment")
                        if m is not None:
                            mis = abs(m) * 1e3
            ms_ = (sc(sh, "mass_of_shaft_body") or 0.0) / 1e3
            for x in (dup, duty):
                try:
                    x.delete()
                except Exception:
                    pass
            return smax, mis, ms_

        # 현행 내경으로 한 번 — 미스얼라인먼트 기준값을 얻는다(기록이 없다)
        id_old = math.floor(od * sg.ID_OVER_OD)
        sh.remove_all_sections()
        sh.add_section(0.0, z2 + sg.SHAFT_TAIL, od / 1e3, id_old / 1e3,
                       od / 1e3, id_old / 1e3)
        for b in bs:
            try:
                if b.inner_connection is not None:
                    b.inner_connection.delete()
            except Exception:
                pass
        for b in bs:
            sg.apply_to_masta(b.detail, g)
        for b, z in ((uw, z1), (dw, z2)):
            b.try_mount_on(sh, z)
        s_old, mis_old, _ = solve(f"w52o_{i}")

        # 두께 규칙 내경으로 다시
        for b in bs:
            try:
                if b.inner_connection is not None:
                    b.inner_connection.delete()
            except Exception:
                pass
        sh.remove_all_sections()
        sh.add_section(0.0, z2 + sg.SHAFT_TAIL, od / 1e3, idm / 1e3,
                       od / 1e3, idm / 1e3)
        for b in bs:
            sg.apply_to_masta(b.detail, g)
        for b, z in ((uw, z1), (dw, z2)):
            b.try_mount_on(sh, z)
        smax, mis_new, ms = solve(f"w52_{i}")
        mb = (sc(uw.detail, "mass") or 0.0) / 1e3

        rows.append(dict(
            rank=rank, OD_mm=od, ID_old_mm=math.floor(od * sg.ID_OVER_OD),
            ID_new_mm=idm, t_new_mm=(od - idm) / 2,
            mass_brg_t=round(mb, 4), mass_shaft_old_t=float(r["mass_shaft_kg"]) / 1e3,
            mass_shaft_t=round(ms, 4),
            mass_total_old_t=float(r["mass_total_kg"]) / 1e3,
            mass_total_t=round(2 * mb + ms, 4),
            sigma_old=float(r["sigma_max_MPa"]), sigma_new=round(smax, 1),
            sigma_old_rerun=round(s_old, 1),
            misalign_old_mrad=round(mis_old, 4) if mis_old else None,
            misalign_new_mrad=round(mis_new, 4) if mis_new else None,
            feasible=int(0 < smax <= LIMIT)))
        if i % 8 == 0 or i == len(F):
            x = rows[-1]
            print(f"  [{i:2}/{len(F)}] #{rank:2} OD {od:,.0f} · ID "
                  f"{x['ID_old_mm']:,.0f}→{idm:,.0f} · 총 "
                  f"{x['mass_total_old_t']:.1f}→{x['mass_total_t']:.1f} t · σ "
                  f"{x['sigma_old']:,.0f}→{smax:,.0f} "
                  f"({time.perf_counter()-t0:.0f}s)", flush=True)

    with open(OUT, "w", newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)

    ok = [r for r in rows if r["feasible"]]
    # σ 를 통과한 것이 없으면 전체로 프론트를 그린다 — 「얼마나 가벼워지는가」와
    # 「왜 쓸 수 없는가」를 한 그림에서 보여야 한다
    new = pareto(ok or rows, "mass_total_t")
    old = pareto(rows, "mass_total_old_t")
    ds = [r["sigma_new"] - r["sigma_old"] for r in rows]
    print(f"\n[결과] σ 통과 {len(ok)}/{len(rows)} · "
          f"새 프론트 {len(new)}건 (기존 {len(old)})")
    print(f"  총질량 최소 {min(r['mass_total_old_t'] for r in rows):.1f} → "
          f"{min(r['mass_total_t'] for r in rows):.1f} t")
    print(f"  σ {min(r['sigma_new'] for r in rows):,.0f} ~ "
          f"{max(r['sigma_new'] for r in rows):,.0f} MPa "
          f"(증가 {min(ds):+,.0f} ~ {max(ds):+,.0f})")

    # ── 그림 (프론트만) ────────────────────────────────────────
    fig, ax = plt.subplots(figsize=(7.2, 5.4))
    ox = [r["mass_brg_t"] for r in old]
    oy = [r["mass_total_old_t"] for r in old]
    nx = [r["mass_brg_t"] for r in new]
    ny = [r["mass_total_t"] for r in new]
    ax.step(ox, oy, where="post", lw=1.3, c="#95a5a6", alpha=.9, zorder=2)
    ax.scatter(ox, oy, s=22, c="#95a5a6", edgecolors="white", linewidths=.6,
               zorder=3, label=f"현행 규칙 {len(old)}건")
    bad = not ok
    if bad:
        # 프론트가 성립하지 않으므로 **같은 64 설계**의 이동만 보여준다
        q = sorted(rows, key=lambda r: r["mass_brg_t"])
        nx = [r["mass_brg_t"] for r in q]
        ny = [r["mass_total_t"] for r in q]
    ax.plot(nx, ny, lw=1.4, c="#c0392b", alpha=.9, zorder=4,
            ls=(0, (5, 3)) if bad else "-")
    ax.scatter(nx, ny, s=26, c="#c0392b", edgecolors="white", linewidths=.6,
               zorder=5,
               label=(f"두께 규칙 적용 {len(rows)}건 (σ 전량 위반)" if bad
                      else f"두께 규칙 적용 {len(new)}건"))
    if bad:
        j = len(nx) // 2
        ax.annotate("σ > 2,100 MPa — 채택 불가", (nx[j], ny[j]),
                    textcoords="offset points", xytext=(12, -20),
                    fontsize=9.5, color="#c0392b")
    ax.set_xlabel("베어링 1개 질량 [t]")
    ax.set_ylabel("총질량 = 2 × 베어링 + 샤프트 [t]")
    ax.set_title("두께 규칙 적용 전후 파레토 프론트 — `z₁ ≥ 1.0 m`", pad=11)
    ax.xaxis.set_major_locator(MultipleLocator(5))
    ax.yaxis.set_major_locator(MultipleLocator(10))
    ax.grid(True, ls=":", lw=.6, c="#b6bcc6", alpha=.8)
    ax.set_axisbelow(True)
    for s in ("top", "right"):
        ax.spines[s].set_visible(False)
    ax.legend(frameon=False, loc="upper right", fontsize=9.5)
    ax.margins(x=.07, y=.10)
    fig.tight_layout()
    os.makedirs(FIG, exist_ok=True)
    for ext, kw in (("png", dict(dpi=500)), ("svg", {})):
        fig.savefig(os.path.join(FIG, f"pareto_w52.{ext}"),
                    bbox_inches="tight", facecolor="white", **kw)
    plt.close(fig)

    # ── 문서 표 (새 프론트) ────────────────────────────────────
    body = [f"σ 통과 **{len(ok)} / {len(rows)}건** · 한도 2,100 MPa", "",
            "| # | OD [mm] | 내경 [mm] | 두께 [mm] | **베어링** [t] | "
            "샤프트 [t] | **총질량** [t] | Δ총질량 | σ [MPa] | "
            "UW 미스얼라인먼트 [mrad] | 판정 |",
            "|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|"]
    for r in sorted(rows, key=lambda x: x["rank"]):
        body.append(
            f"| {r['rank']} | {r['OD_mm']:,.0f} | "
            f"{r['ID_old_mm']:,.0f} → **{r['ID_new_mm']:,.0f}** | "
            f"{r['t_new_mm']:.1f} | **{r['mass_brg_t']:.2f}** | "
            f"{r['mass_shaft_old_t']:.1f} → {r['mass_shaft_t']:.1f} | "
            f"**{r['mass_total_t']:.1f}** | "
            f"{r['mass_total_t']-r['mass_total_old_t']:+.1f} | "
            f"{r['sigma_old']:,.0f} → **{r['sigma_new']:,.0f}** | "
            + (f"{r['misalign_old_mrad']:.3f} → "
               f"**{r['misalign_new_mrad']:.3f}** | "
               if r["misalign_new_mrad"] else "— | ")
            + f"{'✅' if r['feasible'] else '❌'} |")
    s = io.open(DOC, encoding="utf-8").read()
    close = MARK.replace("<!-- ", "<!-- /")
    pat = re.compile(re.escape(MARK) + r"\n.*?\n" + re.escape(close), re.S)
    if not pat.search(s):
        raise RuntimeError(f"{MARK} 자리표를 찾지 못했다")
    io.open(DOC, "w", encoding="utf-8").write(
        pat.sub(lambda m: f"{MARK}\n" + "\n".join(body) + f"\n{close}",
                s, count=1))
    print(f"[문서] §7-6.7.6 {len(new)}행 · 그림 pareto_w52.png")


if __name__ == "__main__":
    main()
