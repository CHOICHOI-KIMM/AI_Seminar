# -*- coding: utf-8 -*-
"""§10-12.7.8 그림 — 세 해법 비교 (형상 위 · 응력분포 아래)

현행 DIN Lundberg · Fujiwara 비대칭(§10-12.6) · 비대칭 DIN(§10-12.7)
§10-12.6 의 그림과 달리 **현행 DIN 형상도 그린다** — outer_race_and_roller_
profiles 로 읽을 수 있음을 §10-12.7.1 에서 확인했다.
"""
import csv
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import a10_profile_lib as L      # noqa: E402
import a10_asymdin as AD         # noqa: E402
import a10_asymdin2 as A2        # noqa: E402

OUT = A2.OUT
FUJI = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "fujiwara")
FIG = os.path.join(HERE, "figures", "asymdin_a10.png")
FIG2 = os.path.join(HERE, "figures", "asymdin_vs_din.png")
SKIP2 = "Fujiwara 비대칭 (§10-12.6)"    # 2해법 그림에서 제외
RANKS = A2.RANKS


def collect():
    P = {int(x["rank"]): x for x in csv.DictReader(
        open(L.PARETO, encoding="utf-8-sig"))}
    QB = json.load(open(os.path.join(FUJI, "baseline.json"), encoding="utf-8"))
    FR = list(csv.DictReader(open(os.path.join(FUJI, "fujiwara_all.csv"),
                                  encoding="utf-8-sig")))
    b2 = json.load(open(os.path.join(OUT, "best2.json"), encoding="utf-8"))
    p3 = os.path.join(OUT, "best3.json")
    b3 = json.load(open(p3, encoding="utf-8")) if os.path.exists(p3) else {}
    rig = L.Rig()
    rig.load_case()
    out = {}
    for rk in RANKS:
        rig.build(P[rk])
        d = rig.uw.detail
        Lwe, Dwe = d.effective_roller_length, d.element_diameter
        rec = []

        # ① 현행 DIN — 형상은 MASTA 가 준다
        rig.set_din(0.0)
        prof = [(float(p.offset_from_roller_centre) * 1e3,
                 float(p.roller_deviation) * 1e6)
                for p in d.outer_race_and_roller_profiles]
        m, off = rig.solve(f"f_{rk}_din")
        rec.append(dict(tag="현행 DIN Lundberg", prof=prof, dist=off,
                        met=AD.metrics(m, off, Lwe)))

        # ② Fujiwara 비대칭 (§10-12.6)
        S = [r for r in FR if int(r["rank"]) == rk and r["stage"] == "S3"
             and r["feasible"] == "1"]
        if S:
            t = max(S, key=lambda r: float(r["score"]))
            fn = L.profile_fn(Lwe, float(t["K1L"]), float(t["K2L"]),
                              float(t["zmL_um"]) / 1e6, K1R=float(t["K1R"]),
                              K2R=float(t["K2R"]),
                              zmR=float(t["zmR_um"]) / 1e6,
                              Q=QB[str(rk)]["P_max_N"])
            pts = rig.set_user(fn, A2.NPTS)
            m, off = rig.solve(f"f_{rk}_fuji")
            rec.append(dict(tag="Fujiwara 비대칭 (§10-12.6)",
                            prof=[(y * 1e3, z * 1e6) for y, z in pts],
                            dist=off, met=AD.metrics(m, off, Lwe)))

        # ③ 비대칭 DIN (§10-12.7) — 확장 결과가 있으면 좋은 쪽
        cand = [b2.get(str(rk))]
        if str(rk) in b3:
            cand.append(b3[str(rk)])
        cand = [c for c in cand if c]
        w = max(cand, key=lambda c: c["s"][4])
        fn = A2.asym_din2(Lwe, Dwe, *w["p"])
        pts = rig.set_user(fn, A2.NPTS)
        m, off = rig.solve(f"f_{rk}_adin")
        rec.append(dict(tag="비대칭 DIN (§10-12.7)",
                        prof=[(y * 1e3, z * 1e6) for y, z in pts],
                        dist=off, met=AD.metrics(m, off, Lwe), p=w["p"]))
        out[rk] = rec
        print(f"  #{rk} 수집 완료 · {len(rec)}건", flush=True)
    json.dump(out, open(os.path.join(OUT, "final_fig.json"), "w"), indent=1)
    return out


def draw(D, only=None, path=None, title=None):
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    from matplotlib import font_manager
    for f in ("Malgun Gothic", "NanumGothic", "Gulim"):
        if any(f == x.name for x in font_manager.fontManager.ttflist):
            plt.rcParams["font.family"] = f
            break
    plt.rcParams["axes.unicode_minus"] = False
    STY = {"현행 DIN Lundberg": ("0.35", "-", 2.0),
           "Fujiwara 비대칭 (§10-12.6)": ("tab:orange", "--", 1.9),
           "비대칭 DIN (§10-12.7)": ("tab:red", "-", 2.3)}
    ranks = sorted(D, key=lambda k: int(k))
    fig, ax = plt.subplots(2, len(ranks), figsize=(15.5, 8.2),
                           gridspec_kw=dict(height_ratios=[1, 1.35]))
    for j, rk in enumerate(ranks):
        a0, a1 = ax[0][j], ax[1][j]
        half = None
        for r in D[rk]:
            if only and r["tag"] not in only:
                continue
            c, ls, lw = STY.get(r["tag"], ("tab:blue", "-", 1.5))
            a0.plot([p[0] for p in r["prof"]], [p[1] for p in r["prof"]],
                    ls, color=c, lw=lw, label=r["tag"])
            a1.plot([p[0] for p in r["dist"]], [p[1] for p in r["dist"]],
                    ls, color=c, lw=lw, label=r["tag"])
            mt = r["met"]
            half = mt["sigma_MPa"] and (r["dist"][-1][0])
            if mt.get("y_star_mm") is not None:
                a1.plot([mt["y_star_mm"]], [mt["sigma_MPa"]], "o", color=c,
                        ms=7, mec="white", mew=1.2, zorder=5)
        a0.invert_yaxis()
        a0.set_title(f"#{rk}", fontsize=12)
        a0.set_ylabel("낙차 $z$ [μm]" if j == 0 else "")
        a0.grid(alpha=0.3)
        a0.axvline(0, color="0.85", lw=0.8)
        a1.axhline(2100, color="tab:red", ls=":", lw=1.1)
        a1.text(0.02, 2100, " σ ≤ 2,100", fontsize=8, color="tab:red",
                va="bottom", transform=a1.get_yaxis_transform())
        a1.set_xlabel("롤러 축방향 위치 $y$ [mm]  (음수 = 하중 치우침 쪽)")
        a1.set_ylabel("내륜 접촉응력 [MPa]" if j == 0 else "")
        a1.grid(alpha=0.3)
        a1.set_ylim(0, 2350)
        for a in (a0, a1):
            a.set_xlim(-half * 1.03, half * 1.03)
    ax[0][0].legend(fontsize=8, loc="lower center", framealpha=0.9)
    fig.suptitle(title or ("세 프로파일 비교 — 형상(위)과 길이방향 "
                          "응력분포(아래) · ● = 최대응력 위치"), fontsize=13)
    fig.tight_layout(rect=(0, 0, 1, 0.95))
    os.makedirs(os.path.dirname(FIG), exist_ok=True)
    out = path or FIG
    fig.savefig(out, dpi=150)
    print("[저장]", out)


if __name__ == "__main__":
    p = os.path.join(OUT, "final_fig.json")
    D = (json.load(open(p, encoding="utf-8"))
         if ("--redraw" in sys.argv and os.path.exists(p)) else collect())
    draw(D)
    keep = ("현행 DIN Lundberg", "비대칭 DIN (§10-12.7)")
    draw(D, only=keep, path=FIG2,
         title="현행 DIN 대 비대칭 DIN — 형상(위)과 길이방향 응력분포(아래) "
               "· ● = 최대응력 위치")
