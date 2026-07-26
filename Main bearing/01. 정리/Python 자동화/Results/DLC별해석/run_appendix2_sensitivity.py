"""
부록 2 Phase A — 배치 변수(Z1, L) 해석적 민감도 DOE (MASTA 불필요)
==================================================================
설계점 56개(Z1 7 × L 8)마다 전 111 DLC 스크리닝을 재수행하여:
 ① 기존(현행 설계) 확정 (dt,k)* 고정 시 새 기하에서의 ε_UW/ε_Sys → 합격 유지 여부
 ② 새 기하 기준 재선정 k*(중앙 타겟팅) → Δk*
산출: 부록2_sensitivity/design_grid_summary.csv(설계점별 집계)
      + design_grid_detail.csv(설계점×DLC 상세) + 문서 §2-8 결과 기록
기하: z_P=0, z_UW=Z1, z_DW=Z1+L → a=−Z1, b=Z1+L, L=L (run_dlc_screening 전역 패치)
"""
import csv
import os
import sys
import time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import run_dlc_screening as scr  # noqa: E402

Z1S = [0.3, 0.4, 0.45, 0.5, 0.55, 0.6, 0.8]
LS = [2.0, 2.25, 2.375, 2.5, 2.625, 2.75, 3.0, 3.5]
BASE = (0.5, 2.5)
OUTDIR = os.path.join(HERE, "부록2_sensitivity")
DOC = os.path.join(ROOT, "DLC기반_피로해석_DLC별해석.md")
MS, ME = "<!-- APPENDIX2_RESULT_START -->", "<!-- APPENDIX2_RESULT_END -->"


def eval_fixed(rows, dt0, k0):
    """screen_one 산출 rows에서 dt=dt0 그리드의 ε를 k0에 선형보간 → (eU, eS)."""
    sub = sorted((r for r in rows if r[1] == dt0), key=lambda r: r[0])
    if not sub:
        return None, None
    ks = np.array([r[0] for r in sub])
    eU = np.array([r[2] for r in sub])
    eS = np.array([r[4] for r in sub])
    return float(np.interp(k0, ks, eU)), float(np.interp(k0, ks, eS))


def main():
    os.makedirs(OUTDIR, exist_ok=True)
    meta = {r["DLC"]: r for r in csv.DictReader(open(
        os.path.join(HERE, "dlc_meta.csv"), encoding="utf-8-sig"))}
    master = {r["DLC"]: r for r in csv.DictReader(open(
        os.path.join(HERE, "dlc_master_summary.csv"), encoding="utf-8-sig"))}
    dlcs = sorted(meta)

    # 시계열 캐시 (1회 로드)
    raw = {}
    for name in dlcs:
        raw[name] = np.genfromtxt(os.path.join(HERE, name, "raw.csv"),
                                  delimiter=",", skip_header=1,
                                  encoding="utf-8-sig")
    print(f"[캐시] {len(raw)} DLC 시계열 로드")

    # 기존 확정 조합 (현행 설계 스크리닝 선정값)
    fixed = {}
    for name in dlcs:
        m = master[name]
        if m.get("valid") == "1" and m.get("best_dt_s") not in ("", None, "None"):
            fixed[name] = (float(m["best_dt_s"]), float(m["best_k"]))
    print(f"[기존 조합] {len(fixed)} DLC")

    det_f = open(os.path.join(OUTDIR, "design_grid_detail.csv"), "w",
                 newline="", encoding="utf-8-sig")
    dw = csv.writer(det_f)
    dw.writerow(["Z1_m", "L_m", "DLC", "dt_fix", "k_fix",
                 "eps_UW_fix", "eps_Sys_fix", "pass_fix",
                 "dt_new", "k_new", "dk"])
    summary = []
    t0 = time.time()
    for Z1 in Z1S:
        for L in LS:
            scr.L_, scr.A_, scr.B_ = L, -Z1, Z1 + L
            nfail = ndtsw = 0
            dks, deUs, deSs = [], [], []
            for name in dlcs:
                sf = float(meta[name]["ScaleFactor"])
                rows, sm = scr.screen_one(raw[name], sf)
                if name not in fixed or not sm["valid"]:
                    continue
                dt0, k0 = fixed[name]
                eU, eS = eval_fixed(rows, dt0, k0)
                ok = eU is not None and 0 <= eU <= 3 and 0 <= eS <= 3
                if not ok:
                    nfail += 1
                b = sm["best"]
                dt_n, k_n = (b[0], b[1]) if b else (None, None)
                dk = (k_n - k0) if (k_n is not None and dt_n == dt0) else None
                if dt_n is not None and dt_n != dt0:
                    ndtsw += 1
                if dk is not None:
                    dks.append(dk)
                deUs.append(eU)
                deSs.append(eS)
                dw.writerow([Z1, L, name, dt0, k0,
                             f"{eU:.3f}" if eU is not None else "",
                             f"{eS:.3f}" if eS is not None else "",
                             int(ok), dt_n, k_n,
                             f"{dk:.3f}" if dk is not None else ""])
            summary.append(dict(
                Z1_m=Z1, L_m=L, n_eval=len(deUs), n_fail=nfail,
                n_dt_switch=ndtsw,
                mean_eps_UW=float(np.mean(deUs)),
                mean_eps_Sys=float(np.mean(deSs)),
                min_eps_UW=float(np.min(deUs)), max_eps_UW=float(np.max(deUs)),
                mean_dk=float(np.mean(dks)) if dks else float("nan"),
                max_abs_dk=float(np.max(np.abs(dks))) if dks else float("nan")))
            print(f"  (Z1={Z1}, L={L})  불합격 {nfail:3d}  dt전환 {ndtsw:2d}  "
                  f"Δk 평균 {summary[-1]['mean_dk']:+.3f} 최대 "
                  f"{summary[-1]['max_abs_dk']:.3f}  ({time.time()-t0:.0f}s)")
    det_f.close()
    with open(os.path.join(OUTDIR, "design_grid_summary.csv"), "w",
              newline="", encoding="utf-8-sig") as f:
        w = csv.DictWriter(f, fieldnames=list(summary[0]))
        w.writeheader()
        w.writerows(summary)
    print(f"[저장] design_grid_summary.csv ({len(summary)} 설계점)")
    write_doc(summary)
    print("[문서] §2-8 기록 완료")


def write_doc(summary):
    NL = chr(10)
    S = {(s["Z1_m"], s["L_m"]): s for s in summary}
    base = S[BASE]
    lines = [MS, "", "## 2-8. Phase A 결과 — Z1·L 민감도 맵 (해석적, 260723)", ""]
    ap = lines.append
    ap(f"- 설계점 {len(summary)}개 × 유효 DLC {base['n_eval']}개 스크리닝 재수행 "
       f"(기준점 (0.5, 2.5) 불합격 {base['n_fail']}개 — 자기일관성 확인)")
    ap("")
    ap("**허용역 맵 — 기존 (dt,k)\\* 고정 시 [0,3] 창 이탈 DLC 수** (행=Z1 [m], 열=L [m])")
    ap("")
    hdr = "| Z1\\\\L | " + " | ".join(f"{L:g}" for L in LS) + " |"
    ap(hdr)
    ap("|---:" + "|---:" * len(LS) + "|")
    for Z1 in Z1S:
        cells = []
        for L in LS:
            s = S[(Z1, L)]
            v = s["n_fail"]
            mark = f"**{v}**" if (Z1, L) == BASE else str(v)
            cells.append(mark)
        ap(f"| {Z1:g} | " + " | ".join(cells) + " |")
    ap("")
    ap("**재선정 k\\*의 이동량 |Δk| 최대** (동일 dt 기준, 행=Z1, 열=L)")
    ap("")
    ap(hdr)
    ap("|---:" + "|---:" * len(LS) + "|")
    for Z1 in Z1S:
        cells = []
        for L in LS:
            s = S[(Z1, L)]
            v = s["max_abs_dk"]
            cells.append(f"{v:.2f}" if v == v else "–")
        ap(f"| {Z1:g} | " + " | ".join(cells) + " |")
    ap("")
    ap("<!-- APPENDIX2_CONCL -->")
    ap("")
    ap(ME)
    txt = open(DOC, encoding="utf-8").read()
    block = NL.join(lines)
    if MS in txt:
        txt = txt.split(MS)[0] + block + txt.split(ME, 1)[1]
    else:
        anchor = "<!-- APPENDIX2_OPEN -->"
        txt = txt.replace(anchor, block + NL + NL + anchor)
    open(DOC, "w", encoding="utf-8").write(txt)


if __name__ == "__main__":
    main()
