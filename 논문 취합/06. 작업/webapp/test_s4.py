"""S4 검증 — 웹 API 결과로 작업내역서 표(V0 · §4.7.4 · §8.4.4–8.4.6 · §8.6.2)를 다시 만들어 코드로 대조 (작업내역서 §9.6.5).

    python test_s4.py
모델 값은 모두 HTTP API(TestClient)에서 받고, 지표 정의(점 선택 · 보간 · RMS · 최대차)는 원 스크립트와 같게 계산한다.
대조 기준 두 가지:
  ① 원 스크립트 산출 JSON(V1b_hooke_li · HL06_compare · HL06_fig6_compare) — 허용 1e-9
  ② 작업내역서 표 인쇄값 — 인쇄 자릿수의 반 단위(반올림 허용)
"""
import json
import subprocess
import sys

import numpy as np
import pandas as pd
from fastapi.testclient import TestClient

import server  # noqa: E402  (skf2010 경로 · 작업 디렉터리 설정)
import fig3_compare as F  # noqa: E402
from hl06_fig6 import c_of, fit_curve  # noqa: E402  (원문 맞춤식 (19)–(25), §8.6 기준)
from hl06_sine import LAM_B, TABLE1, make_case  # noqa: E402
from s0_hooke_li import score  # noqa: E402  (§4.7.4 지표 정의)

cli = TestClient(server.app)
PRE = cli.get("/api/presets").json()
RES = []


def check(name, got, ref, tol):
    d = abs(float(got) - float(ref))
    RES.append(d <= tol)
    print(f"{'PASS' if d <= tol else 'FAIL'}  {name:52s} 웹 {got:.6g} · 기준 {ref:.6g} (차 {d:.1e}, 허용 {tol:.0e})")


def table(name, got, printed: str):
    """표 인쇄값 대조 — 인쇄 자릿수 반 단위까지 허용."""
    dec = len(printed.split(".")[1]) if "." in printed else 0
    check(f"{name} [표 {printed}]", got, float(printed), 0.5 * 10.0**-dec + 1e-12)


def post(req):
    r = cli.post("/api/m2", json=req)
    assert r.status_code == 200, r.text
    return r.json()


def with_(base, **kw):
    out = json.loads(json.dumps(base))
    for k, v in kw.items():
        out[k] = v if not isinstance(v, dict) else {**out.get(k, {}), **v}
    return out


# ---------------------------------------------------------------- V0 (웹이 호출하는 엔진 그대로)
p = subprocess.run([sys.executable, "run_cases.py"], capture_output=True, text=True, encoding="utf-8", errors="replace")
ok = p.returncode == 0 and "22/22 PASS" in p.stdout
RES.append(ok)
print(f"{'PASS' if ok else 'FAIL'}  V0 run_cases.py 종료코드 {p.returncode} · '22/22 PASS' {'있음' if '22/22 PASS' in p.stdout else '없음'}")

# ---------------------------------------------------------------- §4.7.4 2. Leeds S = 0 nRMS · 상관 (Leeds 유효점에서 평가)
xl, hl, pl = F.leeds("a")
mi = np.abs(xl) <= F.CASE.a_x
mh, mp = mi & np.isfinite(hl), mi & np.isfinite(pl)
J474 = json.load(open("ref_data/V1b_hooke_li.json", encoding="utf-8"))
T474 = {"hooke_li": ("0.059", "0.980", "0.152", "0.707"), "skf": ("0.110", "0.940", "0.177", "0.631")}
base = PRE["leeds_S+0"]["request"]
for mode in ("hooke_li", "skf"):
    dh = post(with_(base, model={"s0_mode": mode}, analysis={"x": xl[mh].tolist()}))["result"]["dh"]
    dp = post(with_(base, model={"s0_mode": mode}, analysis={"x": xl[mp].tolist()}))["result"]["dp"]
    sc = score(np.array(dh), np.array(dp), xl, hl, pl, mh, mp)
    for k, t in zip(("nrms_h", "corr_h", "nrms_p", "corr_p"), T474[mode]):
        check(f"§4.7.4 {mode} {k} vs JSON", sc[k], J474[mode]["score"][k], 1e-9)
        table(f"§4.7.4 {mode} {k}", sc[k], t)

# ---------------------------------------------------------------- §8.4.4 중심 hR/b² (HD)
J84 = json.load(open("ref_data/HL06_compare.json", encoding="utf-8"))
T844 = {"D": ("0.223", "0.216"), "E": ("0.031", "0.030"), "F": ("0.0042", "0.0040")}
for c, (t226, t117) in T844.items():
    for E, t in ((226e9, t226), (117e9, t117)):
        req = with_(PRE[f"hl06_{c}"]["request"], case=server._case_dict(make_case(c, 25.0, E)))
        v = post(req)["derived"]["hR_b2"]
        check(f"§8.4.4 {c} E′ {E / 1e9:.0f} hR/b² vs JSON", v, J84["fig4"][f"{c}_hcen"][f"model_E{E / 1e9:.0f}"], 1e-12)
        table(f"§8.4.4 {c} E′ {E / 1e9:.0f} hR/b²", v, t)

# ---------------------------------------------------------------- §8.4.5 중심 복소비 원문 대비 최대 절대차
d4 = pd.read_csv("ref_data/HL06_fig4.csv", comment="#")
T845 = {  # (항목 키, 케이스) → (H/A 안, 식 (24) 안)
    ("t0_re_H", "D"): ("0.09", "0.38"), ("t0_re_H", "E"): ("0.05", "0.36"), ("t0_re_H", "F"): ("0.08", "0.40"),
    ("t0_im_H", "D"): ("0.14", "0.35"), ("t0_im_H", "E"): ("0.10", "0.32"), ("t0_im_H", "F"): ("0.23", "0.44"),
    ("t90_re_H", "D"): ("0.05", "0.19"), ("t90_re_H", "E"): ("0.02", "0.14"), ("t90_re_H", "F"): ("0.04", "0.11"),
    ("t0_re_P", "D"): ("0.25", "1.12"), ("t0_re_P", "E"): ("1.24", "5.08"), ("t0_re_P", "F"): ("22.7", "21.3"),
    ("t90_re_P", "D"): ("0.21", "0.92"), ("t90_re_P", "E"): ("0.76", "5.07"), ("t90_re_P", "F"): ("22.2", "21.3")}
for c in "DEF":
    for mode in ("hooke_li", "skf"):
        for th in (0, 90):
            o = post(with_(PRE[f"hl06_{c}"]["request"], model={"s0_mode": mode},
                           analysis={"type": "sine_centre", "theta": float(th)}))["result"]
            lam = np.log10(o["lam_b"])
            for part in ("re", "im"):
                for q in "HP":
                    key = f"{c}_t{th}_{part}_{q}_{mode}"
                    if key not in J84["fig4"]:
                        continue
                    m = d4[(d4.case == c) & (d4.theta == th) & (d4.part == part) & (d4.quantity == q) & d4.value.notna()]
                    y = np.interp(np.log10(m.lam_b), lam, o[f"{q}_{part}"])
                    v = float(np.abs(y - m.value).max())
                    check(f"§8.4.5 {key} vs JSON", v, J84["fig4"][key]["max_abs_diff"], 1e-9)
                    t = T845.get((f"t{th}_{part}_{q}", c))
                    if t:
                        table(f"§8.4.5 {key}", v, t[0 if mode == "hooke_li" else 1])

# ---------------------------------------------------------------- §8.4.6 공간 분포 RMS 차 (−0.9 < x/b < 0.8)
d3 = pd.read_csv("ref_data/HL06_fig3.csv", comment="#")
T846 = {("dh", 0.7): ("0.446", "0.084", "0.291"), ("dh", 3.0): ("0.089", "0.013", "0.070"),
        ("dp", 0.7): ("0.908", "0.184", "0.651"), ("dp", 3.0): ("0.313", "0.109", "0.132")}
for lb in (0.7, 3.0):
    for mode in ("hooke_li", "skf"):
        o = post(with_(PRE["hl06_H"]["request"], model={"s0_mode": mode}, analysis={"type": "sine_line", "lam_b": lb}))["result"]
        for q in ("dh", "dp"):
            m = d3[(d3.quantity == q) & (np.abs(d3.lam_b - lb) < 1e-9) & (d3.kind == "curve")].sort_values("x_b")
            x, v = m.x_b.to_numpy(), m.value.to_numpy()
            sel = (x > -0.9) & (x < 0.8)
            y = np.interp(x[sel], o["x_b"], o[q])
            rms = float(np.sqrt(np.mean((y - v[sel]) ** 2)))
            j = J84["fig3"][f"{q}_{lb}_{mode}"]
            check(f"§8.4.6 {q} λ/b {lb} {mode} RMS 차 vs JSON", rms, j["rms_diff"], 1e-9)
            t = T846[(q, lb)]
            table(f"§8.4.6 {q} λ/b {lb} {mode} RMS 차", rms, t[1 if mode == "hooke_li" else 2])
            if mode == "hooke_li":
                table(f"§8.4.6 {q} λ/b {lb} 원문 RMS", float(np.sqrt(np.mean(v[sel] ** 2))), t[0])

# ---------------------------------------------------------------- §8.6.2 Fig 6 — 본 모델 vs 원문 맞춤식 · 디지타이징
J86 = json.load(open("ref_data/HL06_fig6_compare.json", encoding="utf-8"))
lam_f = 10.0 ** np.linspace(-5, 3, 801)                       # 원 스크립트의 해석 곡선 격자
web = {}
for name in TABLE1:
    for th in (0, 45, 90):
        o = post(with_(PRE[f"hl06_{name}"]["request"], analysis={"type": "sine_centre", "theta": float(th)}))["result"]
        assert np.allclose(o["lam_b"], LAM_B, rtol=0, atol=0)
        web[(name, th)] = o
short_max = {}
for name in TABLE1:
    for th in (0, 45, 90):
        fa, fp = fit_curve(make_case(name), lam_f, th)
        o, cc = web[(name, th)], c_of(name)
        inside = (LAM_B * cc >= 1e-2) & (LAM_B * cc <= 1e2)
        short = inside & (LAM_B < 2.0)
        da = np.where(inside, np.abs(np.array(o["amp"]) - np.interp(np.log10(LAM_B), np.log10(lam_f), fa)), 0.0)
        dp = np.where(inside, np.abs(np.array(o["pres"]) - np.interp(np.log10(LAM_B), np.log10(lam_f), fp)), 0.0)
        am, pm = float(np.max(np.where(short, da, 0))), float(np.max(np.where(short, dp, 0)))
        j = J86["model_vs_fit_eq_short"][f"{name}_t{th}"]
        check(f"§8.6.2 맞춤식 λ/b<2 {name} θ{th} a/A vs JSON", am, j["amp_max"], 1e-9)
        check(f"§8.6.2 맞춤식 λ/b<2 {name} θ{th} Pλ vs JSON", pm, j["pres_max"], 1e-9)
        short_max[th] = (max(short_max.get(th, (0, 0))[0], am), max(short_max.get(th, (0, 0))[1], pm))
for th, (ta, tp) in zip((0, 45, 90), (("0.006", "0.009"), ("0.012", "0.018"), ("0.018", "0.032"))):
    table(f"§8.6.2 맞춤식 λ/b<2 θ{th} a/A 최대(9 케이스)", short_max[th][0], ta)
    table(f"§8.6.2 맞춤식 λ/b<2 θ{th} Pλ 최대(9 케이스)", short_max[th][1], tp)

d6 = pd.read_csv("ref_data/HL06_fig6_fit_digitized.csv", comment="#")
dig = {}
for th in (0, 45, 90):
    for q, col in (("amp", "value_left_axis"), ("pres", "value_right_axis")):
        m = d6[(d6.theta == th) & (d6.curve == q)]
        if m.empty:
            continue
        L, v = m.Lambda.to_numpy(), m[col].to_numpy()
        mod = [np.interp(np.log10(L), np.log10(LAM_B * c_of(n)), web[(n, th)][q]) for n in TABLE1]
        dm = np.min(np.abs(np.array(mod) - v[None, :]), axis=0)   # 가장 가까운 케이스 점열과의 차
        for tag, sel in (("all", np.ones_like(L, bool)), ("L<=1", L <= 1.0), ("L>1", L > 1.0)):
            if sel.any():
                j = J86["model_vs_digitized"][f"t{th}_{q}_{tag}"]
                dig[(th, q, tag)] = (float(np.median(dm[sel])), float(dm[sel].max()))
                check(f"§8.6.2 디지타이징 θ{th} {q} {tag} 최대 vs JSON", dig[(th, q, tag)][1], j["max"], 1e-9)
                check(f"§8.6.2 디지타이징 θ{th} {q} {tag} 중앙 vs JSON", dig[(th, q, tag)][0], j["median"], 1e-9)
table("§8.6.2 디지타이징 θ0 a/A Λ≤1 최대", dig[(0, "amp", "L<=1")][1], "0.008")
for th, t in zip((0, 45, 90), ("0.058", "0.058", "0.037")):
    table(f"§8.6.2 디지타이징 θ{th} Pλ Λ≤1 최대", dig[(th, "pres", "L<=1")][1], t)
for th, tmax, tmed in ((45, "1.40", "0.38"), (90, "1.45", "0.43")):
    table(f"§8.6.2 디지타이징 θ{th} Pλ Λ>1 최대", dig[(th, "pres", "L>1")][1], tmax)
    table(f"§8.6.2 디지타이징 θ{th} Pλ Λ>1 중앙", dig[(th, "pres", "L>1")][0], tmed)

print(f"{sum(RES)}/{len(RES)} PASS")
raise SystemExit(0 if all(RES) else 1)
