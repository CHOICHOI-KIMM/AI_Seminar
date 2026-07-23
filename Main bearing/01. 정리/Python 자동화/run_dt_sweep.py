"""
dt 수렴 스윕 (무인) — 부록 F 실시간 갱신 + 시각화
=================================================
- fatigue_config.xlsx 의 MODEL/DLC/OUT_DIR 사용 (DT_S/OUT_CSV 는 스윕이 덮어씀)
- 모델 1회 로드 후 dt 목록을 순차 해석
- dt 하나 끝날 때마다: CSV/요약/xlsx 생성 → 프로세스.md 부록 F 갱신 → PNG 갱신
- 이미 완료된 dt 는 건너뜀(중단 후 재실행 안전)
실행: PYTHONUTF8=1 python -X utf8 run_dt_sweep.py
"""
import csv
import math
import os
import time
import traceback

import masta_fatigue as mf          # 설정 로드 + mastapy.init + 헬퍼 재사용
from mastapy.system_model import Design
from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
from make_xlsx import build_xlsx

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

# ── 스윕 설정 ────────────────────────────────────────────
DTS = [10, 6, 4, 2, 1, 0.6, 0.4, 0.3, 0.2, 0.1]   # 새로 돌릴 dt (60,20 은 기존 재사용)
ALL_DTS = [60, 20] + DTS                           # 부록 F 표에 넣을 전체
REF_DT = 0.1                                       # 수렴 기준
PRIMARY = ("Main Bearing_UW", "ISO16281", "modified")   # 주지표
DOC = os.path.join(mf.HERE, "DLC기반_피로해석_프로세스.md")
PNG = "convergence_dt.png"
MARK_S, MARK_E = "<!-- APPENDIX_F_START -->", "<!-- APPENDIX_F_END -->"
# ─────────────────────────────────────────────────────────

OUT_DIR = str(mf.OUT_DIR) if mf.OUT_DIR else mf.HERE
if not os.path.isabs(OUT_DIR):
    OUT_DIR = os.path.join(mf.HERE, OUT_DIR)
os.makedirs(OUT_DIR, exist_ok=True)
DLC_NAME = os.path.splitext(os.path.basename(mf.DLC_FILE))[0]


def tag(dt):
    return "%g" % dt


def paths(dt):
    base = os.path.join(OUT_DIR, f"Fatigue_{DLC_NAME}_dt_{tag(dt)}")
    return base + ".csv", base + "_summary.csv"


# ───────── 부록 F 갱신 ─────────
def read_summary(dt):
    """요약 CSV → {(bearing,std,basis): dict}. 없으면 None."""
    _, s = paths(dt)
    if not os.path.exists(s):
        return None
    out = {}
    for r in csv.reader(open(s, encoding="utf-8-sig")):
        if len(r) > 8 and r[1] in ("ISO281", "ISO16281") and r[2] in ("basic", "modified"):
            try:
                out[(r[0], r[1], r[2])] = dict(sample=float(r[3]), d30=float(r[5]),
                                               sf=float(r[6]), life=float(r[7]))
            except Exception:
                pass
    return out or None


def collect():
    return {dt: read_summary(dt) for dt in ALL_DTS}


def update_appendix(status_note=""):
    data = collect()
    ref = data.get(REF_DT)
    refv = ref.get(PRIMARY, {}).get("sample") if ref else None

    L = []
    L.append(MARK_S)
    L.append("")
    L.append("## 부록 F. dt 수렴 스윕 결과 (실시간)")
    L.append("")
    L.append(f"> 대상: `{DLC_NAME}` · 모델 `{os.path.basename(mf.MODEL_PATH)}` · "
             f"ScaleFactor 45,040(30년) · 수정수명 기준")
    L.append(f"> 주지표: **{PRIMARY[0]} × {PRIMARY[1]} {PRIMARY[2]}** (부록 E 권장). "
             f"N = 600/dt + 1. {status_note}")
    L.append("")
    L.append("### F-1. dt별 등가수명 / 30년손상")
    L.append("")
    L.append("| dt (s) | N | 상태 | UW ISO281m 수명(yr) | **UW ISO16281m 수명(yr)** | DW ISO281m 수명(yr) | DW ISO16281m 수명(yr) | UW ISO16281m 30년손상 |")
    L.append("|-------:|--:|:----:|------------------:|-------------------------:|------------------:|--------------------:|---------------------:|")
    for dt in ALL_DTS:
        N = int(round(600.0 / dt)) + 1
        d = data.get(dt)
        if not d:
            L.append(f"| {tag(dt)} | {N} | 대기 | – | – | – | – | – |")
            continue

        def gv(b, s, k="life", fmt="{:,.1f}"):
            v = d.get((b, s, "modified"), {}).get(k)
            return fmt.format(v) if isinstance(v, float) else "–"
        L.append(f"| {tag(dt)} | {N} | ✅ | {gv('Main Bearing_UW','ISO281')} | "
                 f"**{gv('Main Bearing_UW','ISO16281')}** | {gv('Main Bearing_DW','ISO281')} | "
                 f"{gv('Main Bearing_DW','ISO16281')} | "
                 f"{gv(PRIMARY[0], PRIMARY[1], 'd30', '{:.4f}')} |")
    L.append("")

    # 수렴오차
    L.append("### F-2. 수렴오차 ε = (D(dt) − D_ref)/D_ref   (기준 dt=0.1, 음수=손상 과소=비보수)")
    L.append("")
    if refv:
        L.append("| dt (s) | N | UW ISO16281m ε | UW ISO281m ε | DW ISO16281m ε |")
        L.append("|-------:|--:|---------------:|-------------:|---------------:|")
        for dt in ALL_DTS:
            d = data.get(dt)
            N = int(round(600.0 / dt)) + 1
            if not d:
                L.append(f"| {tag(dt)} | {N} | – | – | – |")
                continue

            def eps(b, s):
                cur = d.get((b, s, "modified"), {}).get("sample")
                rv = ref.get((b, s, "modified"), {}).get("sample")
                if isinstance(cur, float) and isinstance(rv, float) and rv:
                    return f"{(cur - rv) / rv * 100:+.2f}%"
                return "–"
            L.append(f"| {tag(dt)} | {N} | **{eps('Main Bearing_UW','ISO16281')}** | "
                     f"{eps('Main Bearing_UW','ISO281')} | {eps('Main Bearing_DW','ISO16281')} |")
    else:
        L.append("_(기준 dt=0.1 해석 완료 후 자동 계산됩니다)_")
    L.append("")
    L.append("### F-3. 수렴 그래프")
    L.append("")
    L.append(f"![dt 수렴](Results/{PNG})")
    L.append("")
    L.append(MARK_E)
    block = "\n".join(L)

    txt = open(DOC, encoding="utf-8").read() if os.path.exists(DOC) else ""
    if MARK_S in txt and MARK_E in txt:
        pre = txt.split(MARK_S)[0]
        post = txt.split(MARK_E, 1)[1]
        txt = pre + block + post
    else:
        txt = txt.rstrip() + "\n\n---\n\n" + block + "\n"
    open(DOC, "w", encoding="utf-8").write(txt)


def make_plot():
    data = collect()
    series = [("Main Bearing_UW", "ISO16281", "UW ISO16281 mod", "o-", "#c0392b"),
              ("Main Bearing_UW", "ISO281", "UW ISO281 mod", "s--", "#e67e22"),
              ("Main Bearing_DW", "ISO16281", "DW ISO16281 mod", "^-", "#2980b9"),
              ("Main Bearing_DW", "ISO281", "DW ISO281 mod", "v--", "#16a085")]
    ref = data.get(REF_DT)
    fig, ax = plt.subplots(2, 1, figsize=(9, 8), sharex=True)

    for b, s, lab, st, col in series:
        xs, ys = [], []
        for dt in sorted(ALL_DTS):
            d = data.get(dt)
            v = d.get((b, s, "modified"), {}).get("life") if d else None
            if isinstance(v, float):
                xs.append(dt); ys.append(v)
        if xs:
            ax[0].plot(xs, ys, st, color=col, label=lab, ms=5)
    ax[0].set_xscale("log"); ax[0].set_yscale("log")
    ax[0].set_ylabel("Equivalent life [years]")
    ax[0].set_title(f"dt convergence — {DLC_NAME}")
    ax[0].grid(True, which="both", alpha=0.3); ax[0].legend(fontsize=8)

    if ref:
        for b, s, lab, st, col in series:
            xs, ys = [], []
            rv = ref.get((b, s, "modified"), {}).get("sample")
            if not isinstance(rv, float) or not rv:
                continue
            for dt in sorted(ALL_DTS):
                d = data.get(dt)
                cur = d.get((b, s, "modified"), {}).get("sample") if d else None
                if isinstance(cur, float):
                    xs.append(dt); ys.append((cur - rv) / rv * 100)
            if xs:
                ax[1].plot(xs, ys, st, color=col, label=lab, ms=5)
        for lv, c in ((2, "#888"), (-2, "#888")):
            ax[1].axhline(lv, ls=":", color=c, lw=1)
        ax[1].axhline(0, color="k", lw=1)
    ax[1].set_xscale("log")
    ax[1].set_xlabel("dt [s]   (N = 600/dt + 1)")
    ax[1].set_ylabel("Damage error vs dt=0.1 [%]")
    ax[1].grid(True, which="both", alpha=0.3); ax[1].legend(fontsize=8)
    fig.tight_layout()
    fig.savefig(os.path.join(OUT_DIR, PNG), dpi=110)
    plt.close(fig)


# ───────── 해석 ─────────
def run_dt(dt, ctx):
    data, lc, pl, bearings, shaft, ipl, sf, h30, dt0 = ctx
    out_csv, summary_csv = paths(dt)
    idxs, N = mf.sample_indices(len(data), dt, dt0)
    wts = mf.quad_weights(idxs, dt0)
    w_of = dict(zip(idxs, wts))

    done = set()
    if os.path.exists(out_csv):
        try:
            for r in csv.DictReader(open(out_csv, encoding="utf-8-sig")):
                done.add(int(r["index"]))
        except Exception:
            done = set()
    if len(done) >= N:
        print(f"  [dt={tag(dt)}] 이미 완료({N}점) → 스킵")
        return True

    new = not os.path.exists(out_csv)
    f = open(out_csv, "a", newline="", encoding="utf-8-sig")
    w = csv.writer(f)
    if new:
        w.writerow(mf.DATA_HEADER)
    t0 = time.perf_counter()
    n_ok = 0
    for i in idxs:
        if i in done:
            continue
        rec = data[i]
        rev = (rec["rpm"] / 60.0) * w_of[i]
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
                w.writerow([i, rec["t"], rec["rpm"], rev] + lv + [mf.bname(b),
                           mf.num(sin / 1e6 if sin is not None else None),
                           mf.num(sout / 1e6 if sout is not None else None),
                           mf.num(mf.g(d, "iso762006.safety_factor")),
                           mf.num(mf.g(d, "iso2812007.basic_rating_life_cycles")),
                           mf.num(mf.g(d, "iso2812007.modified_rating_life_cycles")),
                           mf.num(mf.g(d, "isots162812008.basic_reference_rating_life_cycles")),
                           mf.num(mf.g(d, "isots162812008.modified_reference_rating_life_cycles"))]
                           + [mf.num(x) for x in dm] + sv)
            n_ok += 1
            if n_ok % 200 == 0:
                el = time.perf_counter() - t0
                print(f"    … {n_ok}/{N}  ({el:.0f}s, {el/max(n_ok,1)*1000:.0f} ms/pt)")
                f.flush()
        except Exception:
            print(f"    [dt={tag(dt)} idx={i}] 실패")
            traceback.print_exc()
    f.close()
    el = time.perf_counter() - t0
    print(f"  [dt={tag(dt)}] 완료 {n_ok}점 / {el:.0f}s ({el/max(n_ok,1)*1000:.0f} ms/pt)")
    mf.write_summary(out_csv, summary_csv, sf, h30, N)
    try:
        build_xlsx(out_csv)
    except Exception as e:
        print("    [warn] xlsx 실패:", e)
    return True


def preflight(ctx):
    """1점 해석으로 하중 전달률·점당시간 확인 (게이트)."""
    data, lc, pl, bearings, shaft, ipl, sf, h30, dt0 = ctx
    rec = data[0]
    t0 = time.perf_counter()
    loads = mf.set_loads(lc, pl, ipl, rec)
    sd = lc.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
    sd.perform_analysis()
    el = time.perf_counter() - t0
    fx = loads["force_x_N"]; fa = loads["axial_load_N"]
    sx = sz = 0.0
    for b in bearings:
        v = mf.g(sd.results_for(b), "internal_force")
        try:
            sx += float(v.x); sz += float(v.z)
        except Exception:
            pass
    rx = abs(sx / fx) * 100 if fx else 0
    rz = abs(sz / fa) * 100 if fa else 0
    print(f"[사전검증] 반경 전달률={rx:.1f}%  축 전달률={rz:.1f}%  (1점 {el:.2f}s)")
    ok = rx > 50 and rz > 50
    print("  → " + ("정상 (스윕 진행)" if ok else "★비정상★ 하중이 베어링에 전달되지 않음 → 중단"))
    return ok, el


def main():
    print("=" * 62)
    print(" dt 수렴 스윕 (무인)")
    print("=" * 62)
    print(" 모델:", os.path.basename(mf.MODEL_PATH))
    print(" 출력:", OUT_DIR)
    print(" dt 목록:", DTS, " (기존 재사용: 60, 20)")

    model_path = mf.MODEL_PATH
    if not os.path.exists(model_path) and os.path.exists(model_path + ".Masta"):
        model_path += ".Masta"
    design = Design.load(model_path)
    asm = design.all_parts_of_type_root_assembly()[0]
    pl = list(asm.all_parts_of_type_point_load())[0]
    bearings = list(asm.all_parts_of_type_bearing())
    shaft = list(asm.all_parts_of_type_shaft())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
    lc = next(c for c in asm.design_properties.static_loads
              if getattr(c, "name", "") == mf.DRIVER_LC)
    sf, h30 = mf.read_scale_factor(mf.FATIGUE_HRS, DLC_NAME)
    data = mf.parse_dlc(mf.DLC_FILE)
    dt0 = round(data[1]["t"] - data[0]["t"], 6)
    ctx = (data, lc, pl, bearings, shaft, ipl, sf, h30, dt0)

    ok, per_pt = preflight(ctx)
    total = sum(int(round(600.0 / d)) + 1 for d in DTS)
    print(f"[예상] 총 {total:,}점 × {per_pt:.2f}s ≈ {total*per_pt/60:.0f}분")
    if not ok:
        print("사전검증 실패 → 스윕 중단")
        return

    update_appendix("진행중…")
    make_plot()
    for k, dt in enumerate(DTS, 1):
        print(f"\n[{k}/{len(DTS)}] dt={tag(dt)}s  (N={int(round(600.0/dt))+1})")
        try:
            run_dt(dt, ctx)
        except Exception:
            print(f"  [dt={tag(dt)}] 예외 발생 → 다음 dt 계속")
            traceback.print_exc()
        note = f"진행 {k}/{len(DTS)} (dt={tag(dt)} 완료)" if k < len(DTS) else "전체 완료"
        update_appendix(note)
        make_plot()
        print(f"  → 부록 F·그래프 갱신됨")

    print("\n" + "=" * 62)
    print(" 스윕 완료. 부록 F 및", PNG, "확인")


if __name__ == "__main__":
    main()
