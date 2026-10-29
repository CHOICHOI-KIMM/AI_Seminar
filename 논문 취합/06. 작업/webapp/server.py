"""웹기반 M2 계산기 — 서버(S1 엔진 API) · 작업내역서 §9.

검증된 skf2010 엔진(conditions · ehl_ripple · hl06_sine 등)을 그대로 호출한다(재구현 없음).
    python server.py            -> http://127.0.0.1:8010  (index.html 은 S2 에서 추가)
    POST /api/m2                 계산 (요청 스키마 M2Request)
    GET  /api/presets            프리셋 목록 · 기본 입력
    GET  /api/preset/{name}      프리셋 저장값(cache/*.json) · stale(코드 · 요청 해시 불일치) 표시
    python server.py --build-cache [프리셋 …]   저장값 생성
단위: 요청 · 응답 모두 SI(m · Pa · Pa s · m/s). 위상 s · 길이 입력은 SI.
"""
from __future__ import annotations

import json
import os
import sys
from typing import List, Literal, Optional

import numpy as np
from fastapi import FastAPI, HTTPException
from fastapi.responses import FileResponse
from pydantic import BaseModel, Field

WEB = os.path.dirname(os.path.abspath(__file__))
SKF = os.path.join(WEB, "..", "skf2010")
sys.path.insert(0, SKF)
os.chdir(SKF)                                        # skf2010 모듈이 ref_data/ 상대경로를 쓴다

import conditions as C  # noqa: E402
import ehl_ripple as M2  # noqa: E402
import fig3_shape_test as T  # noqa: E402
import run_cases as RC  # noqa: E402
from hl06_sine import LAM_B, TABLE1, h_pan_hamrock, make_case  # noqa: E402


# ---------------------------------------------------------------- 스키마

class CaseIn(BaseModel):
    E_eff: float = Field(..., gt=0, description="E′ [Pa]")
    Rx: float = Field(..., gt=0)
    Ry: float = Field(..., gt=0)
    p_h: float = Field(..., gt=0)
    eta0: float = Field(..., gt=0)
    alpha: float = Field(..., gt=0)
    u_bar: float = Field(..., gt=0)
    tau0: Optional[float] = Field(None, gt=0, description="Eyring τ0 [Pa], None = Newtonian")


class FilmIn(BaseModel):
    mode: Literal["hd", "pan_hamrock", "value"] = "hd"
    h: Optional[float] = Field(None, gt=0, description="mode = value 일 때 [m]")


class ModelIn(BaseModel):
    s0_mode: Literal["skf", "hooke_li", "hooke_li_all"] = "hooke_li"
    hc_mode: Literal["complex", "magnitude"] = "complex"


class SineCentre(BaseModel):
    type: Literal["sine_centre"] = "sine_centre"
    theta: float = 0.0                                # 거칠기 방향 [deg], 0 = 횡방향
    lam_b: Optional[list[float]] = None               # None = 원문 41 점 격자


class SineLine(BaseModel):
    type: Literal["sine_line"] = "sine_line"
    lam_b: float
    x_b: Optional[list[float]] = None                 # None = −1..1, 801 점


class Patch(BaseModel):
    type: Literal["patch"] = "patch"
    shape: Literal["trapezoid", "flat_top"] = "trapezoid"
    N: int = Field(255, ge=31, le=1023)
    periods: int = Field(3, ge=1, le=8)
    s: float = 0.0                                    # 스냅숏 위상 [m]
    y0: float = 0.0
    nx: int = Field(1201, ge=51, le=5001)
    x: Optional[List[float]] = Field(None, max_length=20001)    # 단면 평가점 [m] 직접 지정(없으면 ±a_x 등간격 nx 점)


class M2Request(BaseModel):
    case: CaseIn
    S: float = 0.0
    film: FilmIn = FilmIn()
    model: ModelIn = ModelIn()
    analysis: SineCentre | SineLine | Patch = Field(..., discriminator="type")


# ---------------------------------------------------------------- 계산

def to_case(c: CaseIn, name="web") -> C.Case:
    return C.Case(name, c.E_eff, c.eta0, c.alpha, c.u_bar, c.Rx, c.Ry, c.p_h, tau0=c.tau0)


def film_h(case, f: FilmIn) -> float:
    if f.mode == "hd":
        return case.h_cen
    if f.mode == "pan_hamrock":
        return h_pan_hamrock(case)
    if f.h is None:
        raise HTTPException(422, "film.mode = value 이면 film.h 필요")
    return f.h


def component(case, h, S, model, wx, wy):
    """단일 성분 해(ehl_ripple.ripple_component) — 중심값 · 보완파 진폭."""
    k = np.hypot(wx, wy)
    s = M2.ripple_component(np.ones_like(k) + 0j, wx, wy, h, case, S, model.hc_mode, s0_mode=model.s0_mode)
    ph = np.exp(1j * (s["psi"] - wx) * case.a_x)
    H = s["h_a"] + s["h_cf"] * ph
    P = (s["p_a"] + s["p_cf"] * ph) * case.a_x / case.E_eff          # P(0) b/E′A
    return s, H, P


def warnings(case, h, req, lam_b=None):
    w = []
    if lam_b is not None and np.max(lam_b) >= 2.0 and req.model.s0_mode != "skf":
        w.append("λ/b ≥ 2 성분 포함 — Hooke & Li 맞춤식 범위 밖(장파장 압력 π/2 포화, §8.6)")
    if req.S != 0.0 and req.model.s0_mode == "hooke_li_all":
        w.append("S ≠ 0 에 H/A 근사 적용(§5.3 (b)) — 원문 방식 아님")
    if req.S != 0.0 and req.case.tau0 is None:
        w.append("S ≠ 0 인데 τ0 없음(Newtonian)")
    return w


def run(req: M2Request) -> dict:
    case = to_case(req.case)
    h = film_h(case, req.film)
    a = req.analysis
    P_g, G = case.alpha * case.p_h, case.alpha * case.E_eff
    U = case.eta0 * case.u_bar / (case.E_eff * case.Rx)
    S_g = G * U**0.25
    _, _, _, M, L = case.dims
    out = dict(derived=dict(a_x=case.a_x, a_y=case.a_y, F=case.F, h=h, hR_b2=h * case.Rx / case.a_x**2,
                            h_cen_hd=case.h_cen, P=P_g, S_greenwood=S_g, Lambda_per_lam_b=P_g**1.5 / S_g**2,
                            moes_M=M, moes_L=L))
    if a.type == "sine_centre":
        lb = np.asarray(a.lam_b if a.lam_b else LAM_B, float)
        k = 2.0 * np.pi / (lb * case.a_x)
        t = np.radians(a.theta)
        wx = k if a.theta == 0 else (0.0 * k if a.theta == 90 else k * np.cos(t))
        wy = 0.0 * k if a.theta == 0 else (k if a.theta == 90 else k * np.sin(t))
        s, H, P = component(case, h, req.S, req.model, wx, wy)
        out["result"] = dict(lam_b=lb.tolist(), Lambda=(lb * P_g**1.5 / S_g**2).tolist(),
                             H_re=H.real.tolist(), H_im=H.imag.tolist(), P_re=P.real.tolist(), P_im=P.imag.tolist(),
                             amp=np.abs(H).tolist(), pres=(np.abs(P) * lb).tolist(),
                             hc=np.abs(s["h_cf"]).tolist(), ha=np.abs(s["h_a"]).tolist())
        out["warnings"] = warnings(case, h, req, lb)
    elif a.type == "sine_line":
        xb = np.asarray(a.x_b if a.x_b else np.linspace(-1.0, 1.0, 801), float)
        k = np.array([2.0 * np.pi / (a.lam_b * case.a_x)])
        s = M2.ripple_component(np.ones(1) + 0j, k, 0.0 * k, h, case, req.S, req.model.hc_mode,
                                s0_mode=req.model.s0_mode)
        x = xb * case.a_x
        ea = np.exp(1j * k * x)
        ec = np.exp(-1j * k * case.a_x) * np.exp(1j * s["psi"] * (x + case.a_x))
        dh = (s["h_a"] * ea + s["h_cf"] * ec).real
        dp = ((s["p_a"] * ea + s["p_cf"] * ec) * case.a_x / case.E_eff).real
        out["result"] = dict(x_b=xb.tolist(), dh=dh.tolist(), dp=dp.tolist())
        out["warnings"] = warnings(case, h, req, np.array([a.lam_b]))
    else:
        r, Lp = (T.trapezoid_patch if a.shape == "trapezoid" else RC.flat_top_patch)(n=a.N, periods=a.periods)
        xx = np.asarray(a.x, float) if a.x else np.linspace(-case.a_x, case.a_x, a.nx)
        dh, dp, resid, _ = M2.ripple_line(r, Lp, case, req.S, h, xx, req.model.hc_mode, y0=a.y0,
                                          shifts=np.array([a.s]), s0_mode=req.model.s0_mode)
        # 단면 초기 간극 r(x): ripple_line 과 같은 Fourier 합성(R_00 = 0) · 높이 z = z̄ − r (바닥 0 → z̄ = max r)
        R = np.fft.fft2(r) / r.size
        R[0, 0] = 0.0
        w = 2.0 * np.pi * np.fft.fftfreq(a.N, d=Lp / a.N)
        Rx = (R * np.exp(1j * w * a.y0)[:, None]).sum(axis=0)
        rl = (np.exp(1j * np.outer(xx - a.s, w)) @ Rx).real
        out["result"] = dict(x=xx.tolist(), dh=dh[0].tolist(), dp=dp[0].tolist(), L=Lp, dx=Lp / a.N,
                             imag_resid=float(resid), r=rl.tolist(), z=(r.max() - rl).tolist(), zbar=float(r.max()))
        out["warnings"] = warnings(case, h, req)
    return out


# ---------------------------------------------------------------- 프리셋 (§9.4)

def _case_dict(c: C.Case) -> dict:
    return dict(E_eff=c.E_eff, Rx=c.Rx, Ry=c.Ry, p_h=c.p_h, eta0=c.eta0, alpha=c.alpha, u_bar=c.u_bar, tau0=c.tau0)


def presets() -> dict:
    ph = json.load(open("ref_data/V1b_phase.json", encoding="utf-8"))
    leeds = _case_dict(C.LEEDS)
    out = {}
    for key, S in (("a", 0.0), ("b", 1.0), ("c", -1.0)):
        out[f"leeds_S{S:+g}"] = dict(
            label=f"Leeds flat-top S = {S:+g} (사다리꼴, §4.7.4 · §5.4)", ref=f"ref_data/V1b_leeds_fig3_{key}.csv",
            request=dict(case=leeds, S=S, film=dict(mode="value", h=0.302e-6),
                         model=dict(s0_mode="hooke_li" if S == 0 else "hooke_li_all"),
                         analysis=dict(type="patch", shape="trapezoid", N=255, periods=3,
                                       s=ph[key]["fit"]["complex"]["s_um"] * 1e-6)))
    for name in TABLE1:
        out[f"hl06_{name}"] = dict(
            label=f"Hooke & Li 2006 P1 케이스 {name} (Ry/Rx 25 · E′ 226 GPa · HD, §8.4)", ref="ref_data/HL06_fig4.csv",
            request=dict(case=_case_dict(make_case(name, 25.0, 226e9)), S=0.0, film=dict(mode="hd"),
                         model=dict(s0_mode="hooke_li"), analysis=dict(type="sine_centre", theta=0.0)))
    from h07_fig12_check import C_LAM, LAM, case_fig10
    out["h07_fig10"] = dict(
        label="Hooke 2007 P1 Fig 10 조건 v/u 0.5 (E′ 226 GPa 가정, §5.3)", ref="ref_data/H07P1_fig12_digitized.csv",
        request=dict(case=_case_dict(case_fig10(226e9)), S=-1.0, film=dict(mode="hd"),
                     model=dict(s0_mode="hooke_li_all"),
                     analysis=dict(type="sine_centre", theta=0.0, lam_b=(LAM / C_LAM).tolist())))
    return out


# ---------------------------------------------------------------- 프리셋 저장값 (§9.6.2)

CACHE = os.path.join(WEB, "cache")
ENGINE_FILES = ("conditions.py", "ehl_ripple.py", "hl06_sine.py", "fig3_shape_test.py", "run_cases.py")


def engine_hash() -> str:
    """엔진 코드 + 이 파일 계산부의 해시 — 저장값이 현재 코드와 같은지 판정."""
    import hashlib
    h = hashlib.sha256()
    for f in ENGINE_FILES:
        h.update(open(os.path.join(SKF, f), encoding="utf-8").read().encode())   # 텍스트 읽기 — 줄바꿈(LF/CRLF) 무관
    src = open(os.path.join(WEB, "server.py"), encoding="utf-8").read()
    h.update(src[:src.index("# ---------------------------------------------------------------- 프리셋 저장값")].encode())
    return h.hexdigest()[:16]


def preset_views(name: str) -> dict:
    """프리셋의 기본 그래프 탭별 요청(§9.5)."""
    base = presets()[name]["request"]
    if name.startswith("leeds"):
        return {"section": base}
    if name.startswith("hl06"):
        v = {f"centre_t{th}": dict(base, analysis=dict(type="sine_centre", theta=float(th))) for th in (0, 45, 90)}
        v.update({f"line_{lb}": dict(base, analysis=dict(type="sine_line", lam_b=lb)) for lb in (0.1, 0.7, 3.0)})
        return v
    return {"hc_vu0.5": base, "hc_vu0.95": dict(base, S=-0.1)}      # h07_fig10: v/u = 1 + S/2


def request_hash(views: dict) -> str:
    import hashlib
    return hashlib.sha256(json.dumps(views, sort_keys=True).encode()).hexdigest()[:16]


def leeds_harmonics(key: str, res: dict, s: float) -> list:
    """Leeds 고조파(기본 주기 125√2 μm) Δh/r · Δp — §4.7.4 2. 표와 같은 방법(fig3_compare.harm)."""
    import fig3_compare as F
    xx, dh, dp = (np.asarray(res[k]) for k in ("x", "dh", "dp"))
    xl, hl, pl = F.leeds(key)
    mi = np.abs(xl) <= F.CASE.a_x
    mh, mp = mi & np.isfinite(hl), mi & np.isfinite(pl)
    Rv, Ho, Po = F.harm(xx, F.rough(xx, s), s), F.harm(xx, dh, s), F.harm(xx, dp, s)
    Hl, Pl = F.harm(xl[mh], hl[mh], s), F.harm(xl[mp], pl[mp], s)
    deg = lambda z: float(np.degrees(np.angle(z)))  # noqa: E731
    return [dict(n=k + 1, lam_um=176.8 / (k + 1), dh_ratio=abs(Ho[k]) / abs(Rv[k]), dh_ratio_leeds=abs(Hl[k]) / abs(Rv[k]),
                 dh_ph=deg(Ho[k] / Rv[k]), dh_ph_leeds=deg(Hl[k] / Rv[k]), dp_GPa=abs(Po[k]) / 1e9,
                 dp_GPa_leeds=abs(Pl[k]) / 1e9, dp_ph=deg(Po[k] / Rv[k]), dp_ph_leeds=deg(Pl[k] / Rv[k]))
            for k in range(F.N_H)]


def build_cache(names=None):
    import time
    os.makedirs(CACHE, exist_ok=True)
    eh = engine_hash()
    for name in names or presets():
        t = time.time()
        views = preset_views(name)
        out = dict(preset=name, engine_hash=eh, request_hash=request_hash(views), views={})
        for k, req in views.items():
            out["views"][k] = run(M2Request(**req))
        if name.startswith("leeds"):
            key = {"leeds_S+0": "a", "leeds_S+1": "b", "leeds_S-1": "c"}[name]
            out["views"]["section"]["harmonics"] = leeds_harmonics(key, out["views"]["section"]["result"],
                                                                   views["section"]["analysis"]["s"])
        json.dump(out, open(os.path.join(CACHE, f"{name}.json"), "w", encoding="utf-8"), ensure_ascii=False)
        print(f"{name}: {time.time() - t:.1f} s")


def load_cached(name: str) -> dict:
    path = os.path.join(CACHE, f"{name}.json")
    if not os.path.exists(path):
        raise HTTPException(404, f"저장값 없음 — python server.py --build-cache 로 생성 ({name})")
    out = json.load(open(path, encoding="utf-8"))
    out["stale"] = (out["engine_hash"] != engine_hash()) or (out["request_hash"] != request_hash(preset_views(name)))
    return out


# ---------------------------------------------------------------- 원문 디지타이징(§9.4 회색 겹침, S3) — GET /api/ref/{name}

def _csv(name: str) -> list:
    import csv
    with open(os.path.join("ref_data", name), encoding="utf-8") as f:
        return list(csv.DictReader(line for line in f if not line.startswith("#")))


def _num(v):
    x = float(v)
    return x if np.isfinite(x) else None


def reference(name: str) -> dict:
    """프리셋에 대응하는 원문 디지타이징 — 기존 ref_data 파일을 그대로 읽음(값 가공 없음)."""
    out = {}
    if name.startswith("leeds"):
        f = {"leeds_S+0": "a", "leeds_S+1": "b", "leeds_S-1": "c"}[name]
        rows = _csv(f"V1b_leeds_fig3_{f}.csv")
        out["leeds"] = {k: [_num(r[k]) for r in rows] for k in ("x_um", "dh_um", "dp_GPa")}
        out["leeds"]["label"] = f"Leeds NN · SKF 2010 Fig 3({f})"
    elif name.startswith("hl06"):
        case = name[-1]
        out["fig6"] = {str(th): {q: [[_num(r["Lambda"]), _num(r[f"value_{ax}_axis"])] for r in _csv("HL06_fig6_fit_digitized.csv")
                                     if int(r["theta"]) == th and r["curve"] == q]
                                 for q, ax in (("amp", "left"), ("pres", "right"), ("unknown", "right"))}
                       for th in (0, 45, 90)}                               # unknown: 원문 귀속 불명, 오른축 값(§8.6 그림과 같음)
        if case in "DEF":
            cal = {(c["image"], c["axis"]): c["px_per_unit"]
                   for c in json.load(open("ref_data/HL06_calib.json", encoding="utf-8"))["calibration"]}
            rows = _csv("HL06_fig4.csv")
            out["fig4"] = {f"{th}_{part}": {q: [[_num(r["lam_b"]), _num(r["value"])] for r in rows
                                                 if r["case"] == case and int(r["theta"]) == th
                                                 and r["part"] == part and r["quantity"] == q] for q in "HP"}
                           for th, part in ((0, "im"), (0, "re"), (90, "re"))}
            out["fig4_scale"] = {f"{th}_{part}": cal[(img, f"{part} left H/A")] / cal[(img, f"{part} right PbE'A")]
                                 for th, part, img in ((0, "im", f"HL06_fig4_trans_{case}"), (0, "re", f"HL06_fig4_trans_{case}"),
                                                       (90, "re", f"HL06_fig4_inline_{case}"))}
        if case == "H":                                                     # Fig 3 은 케이스 H · θ 0
            rows = _csv("HL06_fig3.csv")
            out["fig3"] = {f"{lb:g}_{q}_{k}": [[_num(r["x_b"]), _num(r["value"])] for r in rows
                                               if float(r["lam_b"]) == lb and r["quantity"] == q and r["kind"] == k]
                           for lb in (0.1, 0.7, 3.0) for q in ("dh", "dp") for k in ("curve", "env_hi", "env_lo")}
    else:
        rows = _csv("H07P1_fig12_digitized.csv")
        out["fig12"] = {k: [[_num(r["Lambda"]), _num(r["hc_A"])] for r in rows if r["kind"] == k]
                        for k in sorted({r["kind"] for r in rows})}
    return out


# ---------------------------------------------------------------- 앱

app = FastAPI(title="SKF 2010 M2 calculator")


@app.post("/api/m2")
def api_m2(req: M2Request):
    return run(req)


@app.get("/api/presets")
def api_presets():
    return presets()


@app.get("/api/preset/{name}")
def api_preset(name: str):
    if name not in presets():
        raise HTTPException(404, name)
    return load_cached(name)


@app.get("/api/ref/{name}")
def api_ref(name: str):
    if name not in presets():
        raise HTTPException(404, name)
    return reference(name)


@app.get("/")
def index():
    path = os.path.join(WEB, "index.html")
    if not os.path.exists(path):
        raise HTTPException(404, "index.html 은 S2 에서 추가")
    return FileResponse(path)


if __name__ == "__main__":
    if "--build-cache" in sys.argv:
        build_cache([a for a in sys.argv[1:] if not a.startswith("--")] or None)
    else:
        import threading
        import urllib.request
        import webbrowser

        import uvicorn
        URL = "http://127.0.0.1:8010"

        def up():
            try:
                urllib.request.urlopen(URL + "/api/presets", timeout=1)
                return True
            except OSError:
                return False

        if "--open" in sys.argv:                      # 웹계산기_실행.bat — 서버 준비 후 브라우저 열기
            if up():                                  # 이미 실행 중이면 브라우저만
                webbrowser.open(URL)
                raise SystemExit(0)

            def wait_open():
                for _ in range(120):
                    if up():
                        webbrowser.open(URL)
                        return
                    threading.Event().wait(0.5)
            threading.Thread(target=wait_open, daemon=True).start()
        uvicorn.run(app, host="127.0.0.1", port=8010)
