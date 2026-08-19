# -*- coding: utf-8 -*-
"""CSV(P1 Phase 3 · P2 Phase 1/2) → data.js 추출.

발표자료 대시보드(index.html)가 읽는 정적 데이터를 만든다.
문서 `DLC기반_피로해석_사이징_최적화.md` 의 표 값과 대조 검증하고,
불일치가 있으면 [warn] 을 출력한다(중단하지는 않는다).

실행: 이 파일이 있는 폴더에서  python make_slide_data.py
"""
import csv
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))          # Results/발표자료/사이징최적화
SIZ = os.path.normpath(os.path.join(HERE, "..", "..", "사이징최적화"))  # Results/사이징최적화

P1_PH3 = os.path.join(SIZ, "P1_극한응력_Phase3", "p1_grid.csv")
P1_PH2 = os.path.join(SIZ, "P1_극한응력_Phase2", "p1_grid.csv")
P2_TGT = os.path.join(SIZ, "P2_피로수명_Phase2", "p2b_targets.csv")
P2_SUM = os.path.join(SIZ, "P2_피로수명_Phase2", "fatigue_summary.csv")
P1_SUM = os.path.join(SIZ, "P2_피로수명_Phase1", "fatigue_summary.csv")

OUT = os.path.join(HERE, "data.js")

SIGMA_LIMIT = 2100.0
D30_LIMIT = 0.5

# 문서 기재값 — 재현 검증용 (§8-4.3.4 · §8-5.6 · §6-1)
EXPECT = {
    "n_points": 8700,
    "n_feasible": 771,
    "sigma_min": 1722.8,
    "mass_min_feasible_t": 135.3,
    "n_p2": 24,
    "baseline_sigma": 3424.2,
    "baseline_D30_UW": 6.9249,   # P2 Phase 1 파이프라인 재현값
    "baseline_life_sys": 3.88,
}

warns = []


def warn(msg):
    warns.append(msg)
    print("[warn] " + msg)


def read_csv(path):
    with open(path, "r", encoding="utf-8-sig", newline="") as f:
        return list(csv.DictReader(f))


def near(a, b, tol=0.05):
    return abs(a - b) <= tol


# ---------------------------------------------------------------- P1 Phase 3
rows = read_csv(P1_PH3)
print("[in ] P1 Phase 3 : %d행" % len(rows))
if len(rows) != EXPECT["n_points"]:
    warn("P1 Phase 3 행수 %d ≠ 문서 %d" % (len(rows), EXPECT["n_points"]))

D_WE_LEVELS = [110, 140, 170, 200, 230]

scatter = []
feas = 0
sigma_min = 1e9
sigma_max = 0.0
mass_min_feas = 1e9
lightest = None
for r in rows:
    sigma = float(r["sigma_max_MPa"])
    mass_t = float(r["mass_total_kg"]) / 1000.0
    dwe = int(float(r["D_we_mm"]))
    ok = 1 if sigma < SIGMA_LIMIT else 0
    # [질량 t(1자리), σ MPa(1자리), D_we 수준 인덱스, 가능여부]
    scatter.append([round(mass_t, 1), round(sigma, 1), D_WE_LEVELS.index(dwe), ok])
    sigma_min = min(sigma_min, sigma)
    sigma_max = max(sigma_max, sigma)
    if ok:
        feas += 1
        if mass_t < mass_min_feas:
            mass_min_feas = mass_t
            lightest = r

print("[chk] 가능해 %d건 (%.1f%%) · σ %.1f ~ %.1f · 최경량 %.1f t"
      % (feas, 100.0 * feas / len(rows), sigma_min, sigma_max, mass_min_feas))
if feas != EXPECT["n_feasible"]:
    warn("가능해 %d건 ≠ 문서 %d건" % (feas, EXPECT["n_feasible"]))
if not near(sigma_min, EXPECT["sigma_min"], 0.1):
    warn("σ 최소 %.1f ≠ 문서 %.1f" % (sigma_min, EXPECT["sigma_min"]))
if not near(mass_min_feas, EXPECT["mass_min_feasible_t"], 0.1):
    warn("최경량 %.1f t ≠ 문서 %.1f t" % (mass_min_feas, EXPECT["mass_min_feasible_t"]))

# 변수별 한계효과 (가능률·σ 최소·최경량)
VARS = [
    ("z1", "z1_m", "z1 [m]"),
    ("z2", "z2_m", "z2 [m]"),
    ("D_pw", "D_pw_mm", "D_pw [mm]"),
    ("alpha", "alpha", "α [°]"),
    ("D_we", "D_we_mm", "D_we [mm]"),
    ("L_we", "L_we_mm", "L_we [mm]"),
]
COL = {"z1": "z1", "z2": "z2", "D_pw": "D_pw_mm", "alpha": "alpha",
       "D_we": "D_we_mm", "L_we": "L_we_mm"}

marginal = {}
for key, _unit, label in VARS:
    col = COL[key]
    acc = {}
    for r in rows:
        lv = float(r[col])
        sigma = float(r["sigma_max_MPa"])
        mass_t = float(r["mass_total_kg"]) / 1000.0
        a = acc.setdefault(lv, {"n": 0, "f": 0, "smin": 1e9, "mmin": None})
        a["n"] += 1
        a["smin"] = min(a["smin"], sigma)
        if sigma < SIGMA_LIMIT:
            a["f"] += 1
            if a["mmin"] is None or mass_t < a["mmin"]:
                a["mmin"] = mass_t
    levels = []
    for lv in sorted(acc):
        a = acc[lv]
        levels.append({
            "level": lv,
            "n": a["n"],
            "feasible": a["f"],
            "rate": round(100.0 * a["f"] / a["n"], 1),
            "sigma_min": round(a["smin"], 1),
            "mass_min": round(a["mmin"], 1) if a["mmin"] is not None else None,
        })
    marginal[key] = {"label": label, "levels": levels}

# 문서 §8-4.3.4 대조 (L_we · D_we 가능률)
DOC_RATE = {
    "L_we": {175: 0.0, 250: 0.0, 325: 0.7, 400: 8.6, 475: 21.7, 550: 34.8},
    "D_we": {110: 0.0, 140: 0.3, 170: 4.8, 200: 10.4, 230: 21.7},
    "z1": {1.0: 15.4, 1.5: 10.3, 2.0: 7.3, 2.5: 4.7, 3.0: 2.6},
}
for k, doc in DOC_RATE.items():
    for lv in marginal[k]["levels"]:
        want = doc.get(int(lv["level"]) if lv["level"] >= 100 else lv["level"])
        if want is None:
            continue
        if abs(lv["rate"] - want) > 0.15:
            warn("%s=%g 가능률 %.1f%% ≠ 문서 %.1f%%" % (k, lv["level"], lv["rate"], want))

# ---------------------------------------------------------------- P1 Phase 2 (퍼널 검증용)
rows2 = read_csv(P1_PH2)
feas2 = sum(1 for r in rows2 if float(r["sigma_max_MPa"]) < SIGMA_LIMIT)
print("[in ] P1 Phase 2 : %d행 · 가능해 %d건" % (len(rows2), feas2))

# ---------------------------------------------------------------- P2 Phase 2
tgt = {r["rank_mass"]: r for r in read_csv(P2_TGT)}
p2 = []
for r in read_csv(P2_SUM):
    name = r["design"]
    t = tgt.get(name)
    if t is None:
        warn("P2 대상 제원 없음: %s" % name)
        continue
    p2.append({
        "design": name,
        "set": t["set"],
        "D_pw": int(float(t["D_pw_mm"])),
        "alpha": int(float(t["alpha"])),
        "D_we": int(float(t["D_we_mm"])),
        "L_we": int(float(t["L_we_mm"])),
        "z1": float(t["z1"]),
        "z2": float(t["z2"]),
        "Z": int(float(t["Z"])),
        "L_eff": round(float(t["L_eff_m"]), 3),
        "mass_t": round(float(t["mass_total_kg"]) / 1000.0, 1),
        "mass_brg_t": round(float(t["mass_brg_kg"]) / 1000.0, 1),
        "mass_shaft_t": round(float(t["mass_shaft_kg"]) / 1000.0, 1),
        "sigma": round(float(t["sigma_max_MPa"]), 1),
        "D30_UW": round(float(r["D30_UW"]), 4),
        "D30_Sys": round(float(r["D30_Sys"]), 4),
        "life_Sys": round(float(r["life_Sys_yr"]), 1),
        "pass": int(r["pass_UW"]) == 1 and int(r["pass_Sys"]) == 1,
    })
print("[in ] P2 Phase 2 : %d건 · 합격 %d건" % (len(p2), sum(1 for d in p2 if d["pass"])))
if len(p2) != EXPECT["n_p2"]:
    warn("P2 대상 %d건 ≠ 문서 %d건" % (len(p2), EXPECT["n_p2"]))
if any(not d["pass"] for d in p2):
    warn("P2 불합격 설계 존재 — 문서는 24건 전원 합격")

margin_min = min(D30_LIMIT / d["D30_UW"] for d in p2)
margin_max = max(D30_LIMIT / d["D30_UW"] for d in p2)
print("[chk] D30 여유 %.1f ~ %.1f배 (문서 4.7 ~ 11.3)" % (margin_min, margin_max))

# ---------------------------------------------------------------- 기준선
base = None
for r in read_csv(P1_SUM):
    if r["design"] == "base":
        base = r
if base is None:
    warn("기준선(base) 행을 P2 Phase 1 요약에서 찾지 못함")
    baseline = {"sigma": EXPECT["baseline_sigma"], "D30_UW": EXPECT["baseline_D30_UW"],
                "life_Sys": EXPECT["baseline_life_sys"], "mass_t": 54.4}
else:
    baseline = {
        "sigma": EXPECT["baseline_sigma"],          # 극한응력은 P1 실측 (§6-1)
        "D30_UW": round(float(base["D30_UW"]), 4),
        "D30_Sys": round(float(base["D30_Sys"]), 4),
        "life_Sys": round(float(base["life_Sys_yr"]), 2),
        "mass_t": 54.4,
        "mass_brg_t": 5.6,
        "mass_shaft_t": 43.2,
        "D_pw": 3331, "alpha": 19, "D_we": 110, "L_we": 238, "Z": 87,
        "z1": 0.5, "z2": 3.0, "L_eff": 3.617,
    }
    if not near(baseline["D30_UW"], EXPECT["baseline_D30_UW"], 0.01):
        warn("기준선 D30_UW %.4f ≠ 문서 %.4f" % (baseline["D30_UW"], EXPECT["baseline_D30_UW"]))

# ---------------------------------------------------------------- 퍼널
funnel = [
    {"label": "전조합 (6변수 격자)", "value": 5 * 7 * 5 * 5 * 5 * 6, "note": "5×7×5×5×5×6"},
    {"label": "기하 제약 통과 (C2·C4)", "value": len(rows), "note": "세장비·최소 스팬"},
    {"label": "극한 응력 가능해", "value": feas, "note": "σ < 2,100 MPa"},
    {"label": "피로 검증 대상", "value": len(p2), "note": "최경량 2집합"},
    {"label": "대표 설계안", "value": 1, "note": "A1"},
]

# ---------------------------------------------------------------- 쓰기
data = {
    "generated_from": "p1_grid.csv(Phase3) · p2b_targets.csv · fatigue_summary.csv(Phase1·2)",
    "limits": {"sigma": SIGMA_LIMIT, "D30": D30_LIMIT},
    "p1": {
        "n": len(rows),
        "feasible": feas,
        "sigma_min": round(sigma_min, 1),
        "sigma_max": round(sigma_max, 1),
        "mass_min_feasible": round(mass_min_feas, 1),
        "d_we_levels": D_WE_LEVELS,
        "cols": ["mass_t", "sigma", "d_we_idx", "feasible"],
        "points": scatter,
    },
    "p1_phase2": {"n": len(rows2), "feasible": feas2},
    "marginal": marginal,
    "p2": p2,
    "p2_margin": {"min": round(margin_min, 1), "max": round(margin_max, 1)},
    "baseline": baseline,
    "funnel": funnel,
    "checks": {"warnings": warns},
}

with open(OUT, "w", encoding="utf-8") as f:
    f.write("// 자동 생성 — make_slide_data.py 로 재생성한다. 직접 수정하지 말 것.\n")
    f.write("const DATA = ")
    json.dump(data, f, ensure_ascii=False, separators=(",", ":"))
    f.write(";\n")

size_kb = os.path.getsize(OUT) / 1024.0
print("[out] %s  (%.0f KB)" % (OUT, size_kb))
print("[done] 경고 %d건" % len(warns))
sys.exit(0)
