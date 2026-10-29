"""S1 · §9.6.2 검증 — HTTP API(TestClient) 결과를 기존 스크립트 산출물 · 직접 호출과 코드로 대조 (작업내역서 §9.6.1).

    python test_s1.py
기준(독립 산출물): ref_data/HL06_model.json(§8.4) · HL06_fig6_model.json(§8.6) · H07P1_fig12_check.json(§5.3)
                   · V1b_nodes.json(§4.7.7, Leeds S = 0 고조파). S ≠ 0 Leeds 는 저장값이 없어 엔진 직접 호출과 대조.
"""
import json

import numpy as np
from fastapi.testclient import TestClient

import server  # noqa: E402  (skf2010 경로 · 작업 디렉터리 설정)
import ehl_ripple as M2  # noqa: E402
import fig3_compare as F  # noqa: E402
import fig3_shape_test as T  # noqa: E402

cli = TestClient(server.app)
PRE = cli.get("/api/presets").json()
RES = []


def check(name, got, ref, tol):
    d = float(np.max(np.abs(np.asarray(got, float) - np.asarray(ref, float))))
    RES.append((name, d, tol, d <= tol))
    print(f"{'PASS' if d <= tol else 'FAIL'}  {name:58s} 최대차 {d:.2e} (허용 {tol:.0e})")


def post(req):
    r = cli.post("/api/m2", json=req)
    assert r.status_code == 200, r.text
    return r.json()


# 1) §8.4 Hooke & Li 케이스 D · E · F 중심 복소비 (θ 0 · 90)
ref = json.load(open("ref_data/HL06_model.json", encoding="utf-8"))["cases"]
for c in "DEF":
    for th in (0, 90):
        req = dict(PRE[f"hl06_{c}"]["request"])
        req["analysis"] = dict(type="sine_centre", theta=th)
        o = post(req)["result"]
        rr = ref[f"{c}_R25_E226"][f"t{th}_hooke_li"]
        for q in ("H_re", "H_im", "P_re", "P_im"):
            check(f"§8.4 {c} θ{th} {q}", o[q], rr[q], 1e-12)

# 2) §8.6 Fig 6 9 케이스 θ 45 (|a/A| · |P|λ/E′A)
ref6 = json.load(open("ref_data/HL06_fig6_model.json", encoding="utf-8"))["cases"]
for c in "ABCDEFGHI":
    req = dict(PRE[f"hl06_{c}"]["request"])
    req["analysis"] = dict(type="sine_centre", theta=45)
    o = post(req)["result"]
    check(f"§8.6 Fig 6 {c} θ45 amp", o["amp"], ref6[c]["t45"]["amp"], 1e-12)
    check(f"§8.6 Fig 6 {c} θ45 pres", o["pres"], ref6[c]["t45"]["pres"], 1e-12)

# 3) §5.3 Fig 12 근사 |h_c/A| (v/u 0.5, E′ 226)
ref12 = json.load(open("ref_data/H07P1_fig12_check.json", encoding="utf-8"))
o = post(PRE["h07_fig10"]["request"])["result"]
check("§5.3 Fig 12 |h_c/A| v/u 0.5", o["hc"], ref12["vu0.5_E226"]["hc"], 1e-12)
check("§5.3 Fig 12 |h_a/A| v/u 0.5", o["ha"], ref12["vu0.5_E226"]["ha"], 1e-12)

# 4) §4.7.7 Leeds S = 0 (N 255) — 고조파 Δh/r · Δp
refn = json.load(open("ref_data/V1b_nodes.json", encoding="utf-8"))["255"]["harmonics"]
o = post(PRE["leeds_S+0"]["request"])["result"]
xx, dh, dp = (np.array(o[k]) for k in ("x", "dh", "dp"))
s = PRE["leeds_S+0"]["request"]["analysis"]["s"]
Rv = F.harm(xx, F.rough(xx, s), s)
Ho, Po = F.harm(xx, dh, s), F.harm(xx, dp, s)
check("§4.7.7 Leeds S=0 Δh/r (n 1–6)", [abs(Ho[k]) / abs(Rv[k]) for k in range(6)], [h["dh_ratio"] for h in refn], 1e-10)
check("§4.7.7 Leeds S=0 Δp [GPa] (n 1–6)", [abs(Po[k]) / 1e9 for k in range(6)], [h["dp_amp"] for h in refn], 1e-10)

# 5) §5.4 Leeds S = ±1 (H/A 근사) — 엔진 직접 호출과 대조
for key in ("leeds_S+1", "leeds_S-1"):
    req = PRE[key]["request"]
    o = post(req)["result"]
    r, L = T.trapezoid_patch(n=255)
    xx = np.array(o["x"])
    dh, dp, _, _ = M2.ripple_line(r, L, F.CASE, req["S"], 0.302e-6, xx, "complex",
                                  shifts=np.array([req["analysis"]["s"]]), s0_mode="hooke_li_all")
    check(f"§5.4 {key} Δh", o["dh"], dh[0], 1e-18)
    check(f"§5.4 {key} Δp", o["dp"], dp[0], 1e-6)

# 6) 입력 검증 — 잘못된 입력은 422
bad = dict(PRE["hl06_D"]["request"]); bad["case"] = dict(bad["case"], E_eff=-1.0)
r = cli.post("/api/m2", json=bad)
RES.append(("입력 검증 E′ < 0 → 422", 0.0, 0.0, r.status_code == 422))
print(f"{'PASS' if r.status_code == 422 else 'FAIL'}  입력 검증 E′ < 0 → HTTP {r.status_code}")

# 7) §9.6.2 프리셋 저장값 — stale 아님 · 저장값 = 현재 계산(사인파는 전 뷰, Leeds 는 고조파 기준 대조)
for name in PRE:
    c = cli.get(f"/api/preset/{name}").json()
    RES.append((f"저장값 {name} stale 아님", 0.0, 0.0, c["stale"] is False))
    print(f"{'PASS' if c['stale'] is False else 'FAIL'}  저장값 {name} stale = {c['stale']}")
    if not name.startswith("leeds"):
        for k, req in server.preset_views(name).items():
            o = post(req)["result"]
            for q, v in o.items():
                check(f"저장값 {name} {k} {q}", c["views"][k]["result"][q], v, 0.0)
c = cli.get("/api/preset/leeds_S+0").json()["views"]["section"]["harmonics"]
check("저장값 Leeds S=0 Δh/r vs §4.7.7", [h["dh_ratio"] for h in c], [h["dh_ratio"] for h in refn], 1e-10)
check("저장값 Leeds S=0 Δp vs §4.7.7", [h["dp_GPa"] for h in c], [h["dp_amp"] for h in refn], 1e-10)

# 8) S3 원문 데이터 API — /api/ref 값 = ref_data 파일(pandas 로 따로 읽어 대조)
import pandas as pd  # noqa: E402


def same(name, got, ref):
    g, r = np.asarray(got, float).ravel(), np.asarray(ref, float).ravel()
    ok = g.shape == r.shape and np.array_equal(np.isnan(g), np.isnan(r)) and np.array_equal(g[~np.isnan(g)], r[~np.isnan(r)])
    RES.append((name, 0.0, 0.0, ok))
    print(f"{'PASS' if ok else 'FAIL'}  {name} ({len(r)} 값)")


nan = lambda v: [np.nan if x is None else x for x in v]  # noqa: E731
for key, f in (("leeds_S+0", "a"), ("leeds_S+1", "b"), ("leeds_S-1", "c")):
    d, o = pd.read_csv(f"ref_data/V1b_leeds_fig3_{f}.csv", comment="#"), cli.get(f"/api/ref/{key}").json()["leeds"]
    for k in ("x_um", "dh_um", "dp_GPa"):
        same(f"원문 {key} {k}", nan(o[k]), d[k])
d6 = pd.read_csv("ref_data/HL06_fig6_fit_digitized.csv", comment="#")
d4 = pd.read_csv("ref_data/HL06_fig4.csv", comment="#")
for c in "ABCDEFGHI":
    o = cli.get(f"/api/ref/hl06_{c}").json()
    for th in (0, 45, 90):
        for q, col in (("amp", "value_left_axis"), ("pres", "value_right_axis"), ("unknown", "value_right_axis")):
            m = d6[(d6.theta == th) & (d6.curve == q)]
            if c == "A":
                same(f"원문 Fig 6 θ{th} {q}", np.array(nan(sum(o["fig6"][str(th)][q], []))), m[["Lambda", col]].to_numpy())
    if c in "DEF":
        for th, part in ((0, "im"), (0, "re"), (90, "re")):
            for q in "HP":
                m = d4[(d4.case == c) & (d4.theta == th) & (d4.part == part) & (d4.quantity == q)]
                same(f"원문 Fig 4 {c} θ{th} {part} {q}", np.array(nan(sum(o["fig4"][f"{th}_{part}"][q], []))), m[["lam_b", "value"]].to_numpy())
d3, o = pd.read_csv("ref_data/HL06_fig3.csv", comment="#"), cli.get("/api/ref/hl06_H").json()["fig3"]
for lb in (0.1, 0.7, 3.0):
    for q in ("dh", "dp"):
        for k in ("curve", "env_hi", "env_lo"):
            m = d3[(d3.lam_b == lb) & (d3.quantity == q) & (d3.kind == k)]
            same(f"원문 Fig 3 λ/b {lb} {q} {k}", np.array(nan(sum(o[f"{lb:g}_{q}_{k}"], []))).reshape(-1), m[["x_b", "value"]].to_numpy())
d12, o = pd.read_csv("ref_data/H07P1_fig12_digitized.csv", comment="#"), cli.get("/api/ref/h07_fig10").json()["fig12"]
for k in sorted(d12.kind.unique()):
    same(f"원문 Fig 12 {k}", np.array(nan(sum(o[k], []))), d12[d12.kind == k][["Lambda", "hc_A"]].to_numpy())

# 9) 단면 초기 간극 r · 높이 z — 입력 격자와 일치(격자점), §3.10 해석 형상과 평탄부 교차 확인
from fig2_trapezoid import z_edge  # noqa: E402

for key in ("leeds_S+0", "leeds_S-1"):
    req = PRE[key]["request"]
    an = req["analysis"]
    r2, L = T.trapezoid_patch(n=an["N"])
    dx = L / an["N"]
    k = np.arange(-80, 81)
    o = post(dict(req, analysis=dict(an, x=(k * dx + an["s"]).tolist())))["result"]
    check(f"단면 r = 입력 격자 r[0, ix] ({key})", o["r"], r2[0, k % an["N"]], 1e-12 * 1e-6)
    check(f"단면 z = z̄ − r ({key})", o["z"], o["zbar"] - np.array(o["r"]), 0.0)
    check(f"단면 z 범위 0–0.30 μm ({key})", [min(o["z"]), max(o["z"])], [0.0, 0.30e-6], 1e-15)
s310 = 176.0e-6
o = post(dict(PRE["leeds_S+0"]["request"], analysis=dict(PRE["leeds_S+0"]["request"]["analysis"], s=s310)))["result"]
xs = np.array(o["x"])
uc = ((xs - s310) / np.sqrt(2.0) + 62.5e-6) % 125e-6 - 62.5e-6
za = z_edge(np.abs(uc)) * 1e-6
j310 = json.load(open("ref_data/V1_gap_r.json", encoding="utf-8"))
rr = np.array(o["r"])
check("§3.10 윗면 평균 r [μm]", rr[za >= 0.30e-6 - 1e-12].mean() * 1e6, j310["r_top_um"], 0.005)
check("§3.10 홈 평균 r [μm]", rr[za <= 1e-12].mean() * 1e6, j310["r_groove_um"], 0.005)
check("§3.10 z̄ [μm]", o["zbar"] * 1e6, j310["zbar_um"], 1e-9)

n_ok = sum(1 for x in RES if x[3])
print(f"{n_ok}/{len(RES)} PASS")
