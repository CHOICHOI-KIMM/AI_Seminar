# -*- coding: utf-8 -*-
"""DLC 시계열 피로해석 파트 — data.js 생성.

입력
  ../../DLC별해석/dashboard_data.json   (260726 · KPI·scatter·bins·coverage)
  ../../DLC별해석/dlc_master_summary.csv (DLC별 스크리닝 best_dt·best_k·rpm·κ)
  ../../../DLC기반_피로해석_DLC별해석.md  (전 111 DLC 본해석 표 — 최종 (dt,k)·판정·소요시간)

MD 표를 직접 파싱해 JSON 의 KPI 를 **교차검증**한다(합격 95 · 불합격 16 · ΣD30_UW 6.750 ·
수명 UW 4.44 / Sys 3.93 · 조합 편향 +2.49%). 불일치는 [warn] 으로 출력한다.

실행: 이 파일이 있는 폴더에서  python make_slide_data.py
"""
import csv
import json
import os
import re
import sys
from collections import Counter, defaultdict

try:                                   # 콘솔이 cp949 여도 로그가 깨지거나 죽지 않도록
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
except Exception:
    pass

HERE = os.path.dirname(os.path.abspath(__file__))            # Results/발표자료/DLC시계열피로해석
DLCDIR = os.path.normpath(os.path.join(HERE, "..", "..", "DLC별해석"))
PYDIR = os.path.normpath(os.path.join(HERE, "..", "..", ".."))  # Python 자동화

SRC_JSON = os.path.join(DLCDIR, "dashboard_data.json")
SRC_CSV = os.path.join(DLCDIR, "dlc_master_summary.csv")
SRC_MD = os.path.join(PYDIR, "DLC기반_피로해석_DLC별해석.md")
OUT = os.path.join(HERE, "data.js")

# 문서 기재값 (§5-4 · §5-1 · §5-3.0)
EXPECT = {
    "n_pass": 95, "n_fail": 16, "n_corr": 29,
    "sumD30_UW": 6.750, "life_UW": 4.44, "life_Sys": 3.93, "life_DW": 24.10,
    "combo_bias_pct": 2.49,
    "dt_dist": {20: 69, 10: 13, 4: 20, 2: 7, 1: 2},
    "n_bins_total": 9150,
    "hours_true_total": 31.0,
    "cov50_n": 16, "cov90_n": 35,
}

warns = []


def warn(m):
    warns.append(m)
    print("[warn] " + m)


def near(a, b, tol):
    return abs(a - b) <= tol


# ---------------------------------------------------------------- 1. 기존 JSON
with open(SRC_JSON, encoding="utf-8") as f:
    J = json.load(f)
kpi = J["kpi"]
print("[in ] dashboard_data.json (%s) · scatter %d · bins %d · coverage %d"
      % (J.get("generated", "?"), len(J["scatter"]), len(J["bins"]), len(J["coverage"])))

# ---------------------------------------------------------------- 2. 본해석 표 파싱
md = open(SRC_MD, encoding="utf-8").read()
seg = md.split("<!-- DLC_FLEET_RESULTS_START -->")[1].split("<!-- DLC_FLEET_RESULTS_END -->")[0]

num = lambda s: float(re.sub(r"[^0-9.eE+-]", "", s))
fleet = []
for line in seg.splitlines():
    if not re.match(r"^\|\s*\d+\s*\|", line):
        continue
    c = [x.strip() for x in line.strip().strip("|").split("|")]
    # 0:# 1:DLC 2:(dt,k) 3:보정 4:ε_UW 5:ε_Sys 6:ε_DW 7:수명 8:D30 9:시간 10:판정
    dtk = re.findall(r"\(([\d.]+),\s*([\d.]+)\)", c[2])
    if not dtk:
        warn("(dt,k) 파싱 실패: " + c[1])
        continue
    dt, k = float(dtk[0][0]), float(dtk[0][1])
    lives = [num(x) for x in c[7].split("/")]
    tm = re.findall(r"([\d.]+)\s*분\s*/\s*([\d.]+)\s*초", c[9])
    t_true_min, t_combo_s = (float(tm[0][0]), float(tm[0][1])) if tm else (0.0, 0.0)
    fleet.append({
        "rank": int(c[0]),
        "dlc": c[1],
        "family": c[1].split("-")[0].replace("DLC", ""),   # 예: 1.2
        "bin": "-".join(c[1].split("-")[:2]).replace("DLC", ""),  # 예: 1.2-d
        "dt": dt, "k": k,
        "ncorr": int(num(c[3])) if c[3] not in ("", "-") else 0,
        "eps_UW": num(c[4]), "eps_Sys": num(c[5]), "eps_DW": num(c[6]),
        "life_UW": lives[0], "life_DW": lives[1], "life_Sys": lives[2],
        "D30_UW": num(c[8]),
        "t_true_min": t_true_min, "t_combo_s": t_combo_s,
        "pass": 1 if "합격 ✅" in c[10] else 0,
    })

print("[in ] 본해석 표 %d행" % len(fleet))
if len(fleet) != 111:
    warn("본해석 표 %d행 ≠ 111행" % len(fleet))

n_pass = sum(f["pass"] for f in fleet)
n_fail = len(fleet) - n_pass
# `보정` 열은 횟수가 아니라 상태 코드다 — 0(무보정) / 1·2(k 보정) / 4(보정 한도 소진).
n_nocorr = sum(1 for f in fleet if f["ncorr"] == 0)
n_corr_k = sum(1 for f in fleet if f["ncorr"] in (1, 2, 3))
n_corr_max = sum(1 for f in fleet if f["ncorr"] >= 4)
n_fail_in_max = sum(1 for f in fleet if f["ncorr"] >= 4 and not f["pass"])
print("[chk] 합격 %d · 불합격 %d | 무보정 %d · k보정 %d · 한도소진 %d (그중 불합격 %d)"
      % (n_pass, n_fail, n_nocorr, n_corr_k, n_corr_max, n_fail_in_max))
if n_pass != EXPECT["n_pass"]:
    warn("합격 %d ≠ 문서 %d" % (n_pass, EXPECT["n_pass"]))
if n_fail != EXPECT["n_fail"]:
    warn("불합격 %d ≠ 문서 %d" % (n_fail, EXPECT["n_fail"]))
if n_fail_in_max != n_fail:
    warn("불합격 %d건 중 %d건만 보정 한도 소진군 — 서술 재확인 필요" % (n_fail, n_fail_in_max))
for key, val in (("n_pass", n_pass), ("n_fail", n_fail)):
    if kpi[key] != val:
        warn("JSON kpi.%s %s ≠ 표 재집계 %s" % (key, kpi[key], val))

sumD30 = sum(f["D30_UW"] for f in fleet)
print("[chk] ΣD30_UW(표 반올림 합) %.3f · 문서 %.3f" % (sumD30, EXPECT["sumD30_UW"]))
if not near(sumD30, EXPECT["sumD30_UW"], 0.06):     # 표 값이 소수 3자리 반올림이라 여유
    warn("ΣD30_UW %.3f ≠ 문서 %.3f" % (sumD30, EXPECT["sumD30_UW"]))

# ---------------------------------------------------------------- 3. 해석시간 (3장 비용의 벽)
t_true_h = sum(f["t_true_min"] for f in fleet) / 60.0
t_combo_min = sum(f["t_combo_s"] for f in fleet) / 60.0
n_zero = sum(1 for f in fleet if f["t_true_min"] == 0)
print("[chk] 참값 합계 %.1f h (0분 기재 %d건 = 기존 결과 재사용) · 조합 합계 %.1f 분"
      % (t_true_h, n_zero, t_combo_min))
if t_true_h < 20:
    warn("참값 합계 %.1f h — 문서 추정 ≈31 h 와 차이 (0분 기재분 때문일 수 있음)" % t_true_h)

rep = max(fleet, key=lambda f: f["D30_UW"])         # 대표 DLC = 손상 1위
speedup_rep = (rep["t_true_min"] * 60.0 / rep["t_combo_s"]) if rep["t_combo_s"] else None
speedup_all = (t_true_h * 60.0 / t_combo_min) if t_combo_min else None
print("[chk] 대표 %s : %.0f분 → %.0f초 (%.0f배) · 전체 %.0f배"
      % (rep["dlc"], rep["t_true_min"], rep["t_combo_s"], speedup_rep or 0, speedup_all or 0))

# ---------------------------------------------------------------- 4. (dt, k) 분포
dt_dist = Counter(f["dt"] for f in fleet)
print("[chk] 본해석 최종 dt 분포: "
      + " · ".join("%g s:%d" % (d, dt_dist[d]) for d in sorted(dt_dist, reverse=True)))
print("[note] 문서 §5-1 은 **스크리닝 선정** 분포(20:69 / 10:13 / 4:20 / 2:7 / 1:2)로 다르다 — "
      "본해석에서 k·dt 보정이 적용된 결과가 위 분포다. 슬라이드는 **본해석 최종** 분포를 쓴다.")

k_bins = [(0.0, 0.2), (0.2, 0.3), (0.3, 0.4), (0.4, 0.6), (0.6, 1.0), (1.0, 99)]
k_hist = []
for lo, hi in k_bins:
    n = sum(1 for f in fleet if lo <= f["k"] < hi)
    k_hist.append({"lo": lo, "hi": hi, "n": n,
                   "label": ("≥ %.1f" % lo) if hi == 99 else ("%.1f–%.1f" % (lo, hi))})

# ---------------------------------------------------------------- 5. 스크리닝 CSV (κ·rpm)
meta = list(csv.DictReader(open(SRC_CSV, encoding="utf-8-sig")))
n_kappa_clip = sum(1 for r in meta if float(r["n_kappa_clip"] or 0) > 0)
print("[in ] dlc_master_summary.csv %d행 · κ클립 발생 DLC %d개" % (len(meta), n_kappa_clip))

# ---------------------------------------------------------------- 6. 계열별 기여 (표 기준 재집계)
fam = defaultdict(lambda: {"D30": 0.0, "n": 0, "n_pass": 0})
for f in fleet:
    a = fam[f["bin"]]
    a["D30"] += f["D30_UW"]
    a["n"] += 1
    a["n_pass"] += f["pass"]
contrib = sorted(
    [{"bin": b, "D30": round(v["D30"], 3), "n": v["n"], "n_pass": v["n_pass"]}
     for b, v in fam.items()],
    key=lambda d: -d["D30"])

# ---------------------------------------------------------------- 7. 커버리지 (표 기준 재집계)
order = sorted(fleet, key=lambda f: -f["D30_UW"])
tot = sum(f["D30_UW"] for f in order) or 1.0
cov, acc = [], 0.0
for i, f in enumerate(order, 1):
    acc += f["D30_UW"]
    cov.append({"n": i, "dmg_pct": round(100.0 * acc / tot, 2),
                "fail": 0 if f["pass"] else 1})
cov50 = next(c["n"] for c in cov if c["dmg_pct"] >= 50)
cov90 = next(c["n"] for c in cov if c["dmg_pct"] >= 90)
print("[chk] 커버리지(참값 기준) 50%% = 상위 %d개 · 90%% = %d개" % (cov50, cov90))
print("[note] 문서 §5-3.0 의 16 / 35 는 **스크리닝 D30** 기준이다. 슬라이드는 참값 기준 값을 쓴다.")

fail_D30 = sum(f["D30_UW"] for f in fleet if not f["pass"])
print("[chk] 불합격 16건의 손상 기여 %.3f / %.3f = %.2f%%"
      % (fail_D30, tot, 100.0 * fail_D30 / tot))

# ---------------------------------------------------------------- 8. 쓰기
data = {
    "source": "dashboard_data.json(%s) + dlc_master_summary.csv + 본해석 표 111행" % J.get("generated", "?"),
    "kpi": {
        "n_dlc": len(fleet), "n_pass": n_pass, "n_fail": n_fail,
        "n_nocorr": n_nocorr, "n_corr_k": n_corr_k, "n_corr_max": n_corr_max,
        "sumD30_UW": round(sumD30, 3),
        "life_UW": kpi["life_UW"], "life_DW": kpi["life_DW"], "life_Sys": kpi["life_Sys"],
        "combo_bias_pct": kpi["combo_bias_pct"],
        "acc_mean_pp": kpi["acc_mean_pp"], "acc_max_pp": kpi["acc_max_pp"],
        "hours_h": 263014, "n_kappa_clip": n_kappa_clip,
        "n_bins_total": EXPECT["n_bins_total"],
    },
    "time": {
        "true_h": round(t_true_h, 1),
        "true_h_doc": EXPECT["hours_true_total"],
        "combo_min": round(t_combo_min, 1),
        "n_zero_true": n_zero,
        "speedup_all": round(speedup_all, 0) if speedup_all else None,
        "rep": {"dlc": rep["dlc"], "true_min": rep["t_true_min"],
                "combo_s": rep["t_combo_s"],
                "speedup": round(speedup_rep, 0) if speedup_rep else None},
    },
    "dt_dist": [{"dt": d, "n": dt_dist[d]} for d in sorted(dt_dist, reverse=True)],
    "k_hist": k_hist,
    "fleet": [{"dlc": f["dlc"], "bin": f["bin"], "dt": f["dt"], "k": f["k"],
               "eps_UW": f["eps_UW"], "eps_Sys": f["eps_Sys"], "D30": f["D30_UW"],
               "life_Sys": f["life_Sys"], "pass": f["pass"], "ncorr": f["ncorr"]}
              for f in fleet],
    "scatter": J["scatter"],
    "contrib": contrib,
    "coverage": cov,
    "cov_marks": {"50": cov50, "90": cov90},
    "fail_share_pct": round(100.0 * fail_D30 / tot, 2),
    "checks": {"warnings": warns},
}

with open(OUT, "w", encoding="utf-8") as f:
    f.write("// 자동 생성 — make_slide_data.py 로 재생성한다. 직접 수정하지 말 것.\n")
    f.write("const DATA = ")
    json.dump(data, f, ensure_ascii=False, separators=(",", ":"))
    f.write(";\n")

print("[out] %s  (%.0f KB)" % (OUT, os.path.getsize(OUT) / 1024.0))
print("[done] 경고 %d건" % len(warns))
sys.exit(0)
