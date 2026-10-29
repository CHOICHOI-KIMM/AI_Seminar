"""§6.6.4 M3 재검토 검증 — 실측값을 독립 기준과 코드로 대조.

    python test_m3.py        (v2_direct.py 산출 JSON 이 있으면 V2 회귀까지)
① 보완파 통일: S = 0 에서 hooke_li_all = hooke_li (성분 해 전체 동일).
② M3 안 M2(ripple_field) 단면 = 웹 계산기 API(ripple_line 경로) — S = 0 · ±1, hooke_li_all.
③ §7.4.1 항등: 특수적분 p_a = E′κ(h_a − r_a)/4 (식 (10)(11)) · 건접촉 P_dry = w⁻¹ FFT(u_dry) (식 (3)).
④ V2 회귀(S = 0 · paper): v2_direct 결과 = 기존 V2_results.json(hooke_li 열).
"""
import json
import os
import sys

import numpy as np

import conditions as C
import dry_contact as DC
import ehl_ripple as ER
import fig3_shape_test as T
import kernels as K

RES = []


def check(name, got, ref, tol, rel=False):
    g, r = np.asarray(got), np.asarray(ref)
    d = float(np.max(np.abs(g - r)) / (np.max(np.abs(r)) if rel and np.max(np.abs(r)) > 0 else 1.0))   # 기준이 0 이면 절대차
    RES.append(d <= tol)
    print(f"{'PASS' if d <= tol else 'FAIL'}  {name:58s} {'상대' if rel else '최대'}차 {d:.2e} (허용 {tol:.0e})")


# ① S = 0 보완파: hooke_li_all = hooke_li
case, h = C.LEEDS, 0.302e-6
rng = np.random.default_rng(1)
n, L = 31, 400e-6
w = 2 * np.pi * np.fft.fftfreq(n, d=L / n)
wy, wx = np.meshgrid(w, w, indexing="ij")
R = np.fft.fft2(rng.normal(size=(n, n))) / n**2 * 1e-7
R[0, 0] = 0
a = ER.ripple_component(R, wx, wy, h, case, 0.0, "complex", s0_mode="hooke_li")
b = ER.ripple_component(R, wx, wy, h, case, 0.0, "complex", s0_mode="hooke_li_all")
for k in ("h_a", "p_a", "h_cf", "p_cf", "psi"):
    check(f"① S=0 hooke_li_all = hooke_li : {k}", b[k], a[k], 0.0)

# ③-1 특수적분 탄성 항등 p_a = E′κ(h_a − r_a)/4 (S = 0 · ±1)
kap = np.hypot(wx, wy)
nz = kap > 0
for S in (0.0, 1.0, -1.0):
    s = ER.ripple_component(R, wx, wy, h, case, S, "complex", s0_mode="hooke_li_all")
    check(f"③ p_a = E′κ(h_a − r_a)/4  (S = {S:+g})", s["p_a"][nz], (case.E_eff * kap * (s["h_a"] - R) / 4)[nz], 1e-12, rel=True)

# ③-2 건접촉 탄성 항등 P = w⁻¹ FFT(u) (DC 제외)
dc = C.DEOLALIKAR
xg = (np.arange(64) - 32) * (2 * dc.a_x / 64)
X, Y = np.meshgrid(xg, xg)
g = 0.15e-6 + 1.2e-6 * (np.hypot(X, Y) / 18e-6) ** 2 - 1.2e-6 * (np.hypot(X, Y) < 18e-6)
d = DC.solve(g, 2 * dc.a_x, 2 * dc.a_x, dc.E_eff, 0.01 * dc.p_h, p_lim=dc.p_lim)
wf = K.w_frf(g.shape, 2 * dc.a_x, 2 * dc.a_x, dc.E_eff)
m = wf > 0
check("③ 건접촉 FFT(p) = w⁻¹ FFT(u)  (DC 제외)", np.fft.fft2(d["p"])[m], (np.fft.fft2(d["u"]) / np.where(m, wf, 1))[m], 1e-10, rel=True)

# ② M3 안 M2(ripple_field) 단면 = 웹 계산기 API
sys.path.insert(0, os.path.join("..", "webapp"))
import server  # noqa: E402
from fastapi.testclient import TestClient  # noqa: E402

os.chdir(os.path.dirname(os.path.abspath(ER.__file__)))
cli = TestClient(server.app)
req0 = cli.get("/api/presets").json()["leeds_S+0"]["request"]
r2, Lp = T.trapezoid_patch(n=255)
xf = (np.arange(255) - 127) * Lp / 255
for S in (0.0, 1.0, -1.0):
    dh, dp, x = ER.ripple_field(r2, Lp, case, S, h, "complex", "hooke_li_all")
    req = dict(req0, S=S, model=dict(s0_mode="hooke_li_all", hc_mode="complex"),
               analysis=dict(req0["analysis"], s=0.0, y0=0.0, x=xf.tolist()))
    o = cli.post("/api/m2", json=req).json()["result"]
    check(f"② ripple_field y=0 Δh = 웹 API  (S = {S:+g})", dh[127], o["dh"], 1e-12, rel=True)
    check(f"② ripple_field y=0 Δp = 웹 API  (S = {S:+g})", dp[127], o["dp"], 1e-12, rel=True)

# ④ V2 회귀 (v2_direct 결과가 있을 때)
if os.path.exists("ref_data/V2_direct.json"):
    new = json.load(open("ref_data/V2_direct.json", encoding="utf-8"))
    old = json.load(open("ref_data/V2_results.json", encoding="utf-8"))
    for shape in ("ellipsoid", "cap"):
        a_, b_ = new[f"S0_{shape}"], old[f"hooke_li_{shape}"]
        for k_new, k_old in (("phi_bl", "phi_bl"), ("paper_p_max_hertz", "p_max_hertz"), ("paper_p_max_mean", "p_max_mean"),
                             ("h_max_lead", "h_max_lead"), ("h_max_trail", "h_max_trail"), ("h_far", "h_far")):
            check(f"④ V2 {shape} {k_new} = 기존 V2_results", a_[k_new], b_[k_old], 1e-12)
else:
    print("④ 건너뜀 — v2_direct.py 실행 후 다시")

print(f"{sum(RES)}/{len(RES)} PASS")
raise SystemExit(0 if all(RES) else 1)
