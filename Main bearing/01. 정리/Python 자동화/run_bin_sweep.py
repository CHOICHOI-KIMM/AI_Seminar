"""
부록 8: 빈 평균 vs 점 추출 dt 수렴 스윕 (무인) — 50도 모델
==========================================================
대상: ..._유연체_FE_온도_50도_260721.Masta
- 두 방식 모두 **MASTA 실해석**(파생·서브샘플링 없음)
    bin : 빈 내 하중 6분력·rpm 산술평균 → 1해석/빈,  N_b = (rpm_avg/60)·dt
    pt  : 시간축 균등 점추출          → 1해석/점,  N_i = (rpm_i/60)·w_i (사다리꼴)
- dt=0.1 은 빈당 1점이라 두 방식이 동일 → 1회만 해석하여 공용 참값으로 사용
- dt 하나 끝날 때마다 프로세스_v1.md 부록 8-5 표 + 그래프 갱신
실행: PYTHONUTF8=1 python -X utf8 -u run_bin_sweep.py
"""
import csv
import os
import time
import traceback

import masta_fatigue as mf          # 헬퍼 재사용 (mastapy.init 포함)
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
from make_xlsx import build_xlsx

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker

# ── 설정 ────────────────────────────────────────────────
MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.3_샤프트 두께,형상 2안"
         r"_피로하중 반영_유연체_FE_온도_50도_260721.Masta")
DTS = [0.1, 60, 20, 10, 6, 4, 2, 1, 0.6, 0.4, 0.3, 0.2]   # 0.1 = 참값 먼저
REF_DT = 0.1
DT0 = 0.1
PRIMARY = ("Main Bearing_UW", "ISO16281", "modified")
OUT_DIR = os.path.join(mf.HERE, "Results")
PREFIX = "T50"
DOC = os.path.join(mf.HERE, "DLC기반_피로해석_프로세스_v1.md")
PNG = f"{PREFIX}_bin_vs_point.png"
MARK_S, MARK_E = "<!-- APPENDIX_8_START -->", "<!-- APPENDIX_8_END -->"
# ────────────────────────────────────────────────────────

os.makedirs(OUT_DIR, exist_ok=True)
DLC_NAME = os.path.splitext(os.path.basename(mf.DLC_FILE))[0]


def tag(dt):
    return "%g" % dt


def paths(mode, dt):
    b = os.path.join(OUT_DIR, f"{PREFIX}_{mode}_{DLC_NAME}_dt_{tag(dt)}")
    return b + ".csv", b + "_summary.csv"


# ───────── 하중 케이스 생성 ─────────
def bin_cases(data, dt):
    """→ [(idx, t, rpm, rev, loads6)] 빈 평균."""
    k = int(round(dt / DT0))
    n = len(data)
    nb = max(1, n // k)
    edges = [(b * k, (b + 1) * k) for b in range(nb)]
    if edges[-1][1] < n:
        edges[-1] = (edges[-1][0], n)
    out = []
    for bi, (i0, i1) in enumerate(edges):
        m = i1 - i0
        span = m * DT0
        rec = {key: sum(data[i][key] for i in range(i0, i1)) / m
               for key in ("rpm", "Mx", "My", "Mz", "Fx", "Fy", "Fz")}
        rec["t"] = round(i0 * DT0, 4)
        out.append((bi, rec["t"], rec["rpm"], rec["rpm"] / 60.0 * span, rec))
    return out


def point_cases(data, dt):
    """→ [(idx, t, rpm, rev, rec)] 균등 점추출 + 사다리꼴 가중."""
    last = len(data) - 1
    N = int(round(last * DT0 / dt)) + 1
    idxs = [int(round(j * last / (N - 1))) for j in range(N)]
    w = mf.quad_weights(idxs, DT0)
    return [(i, data[i]["t"], data[i]["rpm"], data[i]["rpm"] / 60.0 * wi, data[i])
            for i, wi in zip(idxs, w)]


# ───────── 해석 ─────────
def run_cases(cases, out_csv, ctx):
    lc, pl, bearings, shaft, ipl = ctx
    done = set()
    if os.path.exists(out_csv):
        try:
            for r in csv.DictReader(open(out_csv, encoding="utf-8-sig")):
                done.add(int(r["index"]))
        except Exception:
            done = set()
    if len(done) >= len(cases):
        print(f"    이미 완료({len(cases)}) → 스킵")
        return
    new = not os.path.exists(out_csv)
    f = open(out_csv, "a", newline="", encoding="utf-8-sig")
    w = csv.writer(f)
    if new:
        w.writerow(mf.DATA_HEADER)
    t0 = time.perf_counter()
    n_ok = 0
    for idx, t_s, rpm, rev, rec in cases:
        if idx in done:
            continue
        try:
            loads = mf.set_loads(lc, pl, ipl, rec)
            sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            sd.perform_analysis()
            lv = [loads[c] for c in mf.LOAD_COLS]
            ssf, soff = mf.shaft_din743_sf(sd, shaft)
            sv = [mf.num(ssf), mf.num(soff)]
            for b in bearings:
                d = mf.g(sd.results_for(b), "component_detailed_analysis")
                sin = mf.fnum(mf.g(d, "maximum_normal_stress_inner"))
                sout = mf.fnum(mf.g(d, "maximum_normal_stress_outer"))
                dm = [mf.damage(rev, mf.g(d, p)) for _, p, _, _ in mf.DAMAGE_DEFS]
                w.writerow([idx, t_s, rpm, rev] + lv + [mf.bname(b),
                           mf.num(sin / 1e6 if sin is not None else None),
                           mf.num(sout / 1e6 if sout is not None else None),
                           mf.num(mf.g(d, "iso762006.safety_factor")),
                           mf.num(mf.g(d, "iso2812007.basic_rating_life_cycles")),
                           mf.num(mf.g(d, "iso2812007.modified_rating_life_cycles")),
                           mf.num(mf.g(d, "isots162812008.basic_reference_rating_life_cycles")),
                           mf.num(mf.g(d, "isots162812008.modified_reference_rating_life_cycles"))]
                           + [mf.num(x) for x in dm] + sv)
            n_ok += 1
            if n_ok % 300 == 0:
                el = time.perf_counter() - t0
                print(f"      … {n_ok}/{len(cases)}  ({el:.0f}s, {el/n_ok*1000:.0f} ms/건)")
                f.flush()
        except Exception:
            print(f"      [idx={idx}] 실패")
            traceback.print_exc()
    f.close()
    el = time.perf_counter() - t0
    print(f"    완료 {n_ok}건 / {el:.0f}s ({el/max(n_ok,1)*1000:.0f} ms/건)")


# ───────── 결과 수집 ─────────
def read_summary(mode, dt):
    _, s = paths(mode, dt)
    if not os.path.exists(s):
        return None
    out = {}
    for r in csv.reader(open(s, encoding="utf-8-sig")):
        if len(r) > 8 and r[1] in ("ISO281", "ISO16281") and r[2] in ("basic", "modified"):
            try:
                out[(r[0], r[1], r[2])] = dict(sample=float(r[3]), d30=float(r[5]),
                                               life=float(r[7]))
            except ValueError:
                pass
    return out or None


def collect():
    d = {}
    for dt in DTS:
        for mode in ("bin", "pt"):
            key = "bin" if dt == REF_DT else mode
            d[(mode, dt)] = read_summary(key if dt == REF_DT else mode, dt)
    return d


def update_doc(note=""):
    data = collect()
    ref = data.get(("bin", REF_DT))
    rv = ref.get(PRIMARY, {}).get("sample") if ref else None
    L = [MARK_S, ""]
    L.append(f"> 대상 모델: `{os.path.basename(MODEL)}` (**윤활유 50 °C**) · `{DLC_NAME}` · "
             f"ScaleFactor 45,040 · 주지표 **{PRIMARY[0]} ISO16281 수정수명**")
    L.append(f"> 두 방식 모두 **MASTA 실해석**. dt=0.1은 빈당 1점이라 두 방식이 동일하며 **참값**으로 사용. {note}")
    L.append("")
    L.append("| dt (s) | 케이스수 | 빈평균 UW(yr) | 점추출 UW(yr) | 빈평균 DW(yr) | 점추출 DW(yr) | "
             "**빈 편향 ε** | **점 편향 ε** |")
    L.append("|-------:|--------:|-------------:|-------------:|-------------:|-------------:|"
             "-------------:|-------------:|")
    for dt in sorted(DTS, reverse=True):
        n = 6001 if dt == REF_DT else int(round(600.0 / dt))
        cells = []
        for mode in ("bin", "pt"):
            for bn in ("Main Bearing_UW", "Main Bearing_DW"):
                d = data.get((mode, dt))
                v = d.get((bn, "ISO16281", "modified"), {}).get("life") if d else None
                cells.append(f"{v:,.1f}" if isinstance(v, float) else "–")
        eps = []
        for mode in ("bin", "pt"):
            d = data.get((mode, dt))
            cur = d.get(PRIMARY, {}).get("sample") if d else None
            eps.append(f"**{(cur-rv)/rv*100:+.2f}%**" if (isinstance(cur, float) and rv) else "–")
        mark = " ★참값" if dt == REF_DT else ""
        L.append(f"| {tag(dt)}{mark} | {n:,} | {cells[0]} | {cells[2]} | {cells[1]} | "
                 f"{cells[3]} | {eps[0]} | {eps[1]} |")
    L += ["", "#### 8-5.1 수렴 그래프", "",
          f"![빈평균 vs 점추출 수렴](Results/{PNG})", "",
          "> **(상)** dt별 등가수명 — 두 방식 × 두 베어링. **(하)** 손상 편향 ε (기준 dt=0.1).",
          "> 빈 평균은 저역통과 특성으로 **단조·음(비보수)**, 점 추출은 3P 에일리어싱으로 **비단조**인지 확인.",
          "", MARK_E]
    block = "\n".join(L)
    txt = open(DOC, encoding="utf-8").read()
    if MARK_S in txt and MARK_E in txt:
        txt = txt.split(MARK_S)[0] + block + txt.split(MARK_E, 1)[1]
        open(DOC, "w", encoding="utf-8").write(txt)


def _plainlog(ax, which="x"):
    f = mticker.FuncFormatter(lambda v, _: ("%g" % v))
    for a in ((ax.xaxis,) if which == "x" else (ax.xaxis, ax.yaxis)):
        a.set_major_formatter(f); a.set_minor_formatter(mticker.NullFormatter())


def make_plot():
    """부록 6-3 형식: (상) dt별 등가수명  (하) 손상 편향 ε."""
    data = collect()
    ref = data.get(("bin", REF_DT))
    rv = ref.get(PRIMARY, {}).get("sample") if ref else None
    SER = [("bin", "Main Bearing_UW", "빈평균 UW", "o-", "#c0392b"),
           ("pt", "Main Bearing_UW", "점추출 UW", "s--", "#e67e22"),
           ("bin", "Main Bearing_DW", "빈평균 DW", "^-", "#2471a3"),
           ("pt", "Main Bearing_DW", "점추출 DW", "v--", "#16a085")]
    fig, ax = plt.subplots(2, 1, figsize=(9, 8), sharex=True)

    for mode, bn, lab, st, c in SER:                      # (상) 등가수명
        xs, ys = [], []
        for dt in sorted(DTS):
            d = data.get((mode, dt))
            v = d.get((bn, "ISO16281", "modified"), {}).get("life") if d else None
            if isinstance(v, float):
                xs.append(dt); ys.append(v)
        if xs:
            ax[0].plot(xs, ys, st, color=c, label=lab, ms=5)
    if ref:
        for bn, c in (("Main Bearing_UW", "#c0392b"), ("Main Bearing_DW", "#2471a3")):
            v = ref.get((bn, "ISO16281", "modified"), {}).get("life")
            if isinstance(v, float):
                ax[0].axhline(v, color=c, ls=":", lw=1, alpha=0.6)
    ax[0].set_xscale("log"); ax[0].set_yscale("log")
    ax[0].set_ylabel("등가수명 [years]")
    ax[0].set_title(f"빈 평균 vs 점 추출 수렴 — {DLC_NAME} (윤활유 50 °C)\n"
                    "ISO 16281 수정수명, 점선 = dt 0.1 참값")
    _plainlog(ax[0], "both")
    ax[0].grid(True, which="both", alpha=0.3); ax[0].legend(fontsize=8)

    for mode, bn, lab, st, c in SER:                      # (하) 편향
        if not rv:
            break
        rb = ref.get((bn, "ISO16281", "modified"), {}).get("sample")
        if not isinstance(rb, float) or not rb:
            continue
        xs, ys = [], []
        for dt in sorted(DTS):
            if dt == REF_DT:
                continue
            d = data.get((mode, dt))
            cur = d.get((bn, "ISO16281", "modified"), {}).get("sample") if d else None
            if isinstance(cur, float):
                xs.append(dt); ys.append((cur - rb) / rb * 100)
        if xs:
            ax[1].plot(xs, ys, st, color=c, label=lab, ms=5)
    ax[1].axhline(0, color="k", lw=1)
    ax[1].axhspan(-1, 1, color="#2ecc71", alpha=0.12, label="|ε| ≤ 1%")
    ax[1].set_xscale("log")
    ax[1].set_xlabel("dt [s]")
    ax[1].set_ylabel("손상 편향 ε [%]   (기준 dt=0.1)")
    _plainlog(ax[1])
    ax[1].grid(True, which="both", alpha=0.3); ax[1].legend(fontsize=8)
    fig.tight_layout()
    fig.savefig(os.path.join(OUT_DIR, PNG), dpi=130)
    plt.close(fig)


def main():
    try:
        plt.rcParams["font.family"] = "Malgun Gothic"
        plt.rcParams["axes.unicode_minus"] = False
    except Exception:
        pass
    print("=" * 66)
    print(" 부록 8 스윕 — 빈 평균 vs 점 추출 (50도 모델)")
    print("=" * 66)
    print(" 모델:", os.path.basename(MODEL))

    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    pl = list(asm.all_parts_of_type_point_load())[0]
    bearings = list(asm.all_parts_of_type_bearing())
    shaft = list(asm.all_parts_of_type_shaft())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
    lc = next(c for c in asm.design_properties.static_loads
              if getattr(c, "name", "") == mf.DRIVER_LC)
    ctx = (lc, pl, bearings, shaft, ipl)
    sf, h30 = mf.read_scale_factor(mf.FATIGUE_HRS, DLC_NAME)
    data = mf.parse_dlc(mf.DLC_FILE)
    print(f" 시계열 {len(data)}점 · ScaleFactor {sf:,.0f}")

    total = 6001 + sum(int(round(600.0/d)) * 2 for d in DTS if d != REF_DT)
    print(f" 총 해석 {total:,}건 (예상 {total*0.3/60:.0f}분)\n")

    for k, dt in enumerate(DTS, 1):
        modes = ["bin"] if dt == REF_DT else ["bin", "pt"]
        for mode in modes:
            cases = bin_cases(data, dt) if mode == "bin" else point_cases(data, dt)
            oc, sc = paths(mode, dt)
            print(f"[{k}/{len(DTS)}] dt={tag(dt)} {mode}  ({len(cases):,}건)")
            try:
                run_cases(cases, oc, ctx)
                mf.write_summary(oc, sc, sf, h30, len(cases))
                try:
                    build_xlsx(oc)
                except Exception:
                    pass
            except Exception:
                traceback.print_exc()
        note = "전체 완료" if k == len(DTS) else f"진행 {k}/{len(DTS)}"
        try:
            update_doc(note); make_plot()
            print("  → 부록 8-5·그래프 갱신")
        except Exception:
            traceback.print_exc()

    print("\n스윕 완료.")


if __name__ == "__main__":
    main()
