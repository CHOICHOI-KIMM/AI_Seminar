"""
부록 5 — DLC별 윤활조건(κ · λ · a_ISO) 검토
=============================================
기준선 · A1 · B1 세 모델에 대해 111 DLC 를 dt=20 으로 나눈 **각 빈에서**
MASTA 가 산출하는 윤활 지표를 추출하고, DLC별 회전수 가중평균·단순평균을 낸다.

빈 구성은 각 설계의 스크리닝 k 를 그대로 쓴다(§8-3.2) — P2 해석과 동일한 빈이라
결과가 §8-3·§8-5 와 직결된다.

추출 (UW·DW 각각)
  κ        iso2812007.viscosity_ratio
  λ 내/외  lambda_ratio_inner / _outer
  h_min    minimum_lubricating_film_thickness_inner / _outer   [m]
  ν · ν₁   kinematic_viscosity / iso2812007.reference_kinematic_viscosity
  a_ISO    isots162812008.life_modification_factor_for_systems_approach
           (MASTA 상한 50 — 포화 여부를 함께 기록)

산출: 부록5_윤활조건/lub_per_bin.csv · lub_per_dlc.csv · 윤활조건_검토.xlsx
"""
import csv
import math
import os
import sys
import time

import psutil

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
DLCDIR = os.path.join(RES, "DLC별해석")
OUTDIR = os.path.join(HERE, "부록5_윤활조건")
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg   # noqa: E402

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
NBATCH = 20
DT0, DT = 0.1, 20.0
AISO_CAP = 50.0
MEM_LIMIT = 95.0
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")

# 대상 3모델 — (태그, D_pw, α, D_we, L_we, z1, z2, k 출처)
K_PH1 = os.path.join(HERE, "P2_피로수명_Phase1", "screen_k.csv")
K_PH2 = os.path.join(HERE, "P2_피로수명_Phase2", "screen_k.csv")
TARGETS = [
    ("기준선", 3.3309, 19.0, 0.11051, 0.238048, 0.5, 3.0, K_PH1, "base"),
    ("A1",     3.900,  23.0, 0.230,   0.475,    1.0, 5.0, K_PH2, "A1"),
    ("B1",     3.900,  27.0, 0.230,   0.475,    1.5, 5.5, K_PH2, "B1"),
]
FIELDS = ["model", "DLC", "bin", "rpm", "rev", "k", "ScaleFactor", "brg",
          "kappa", "lambda_in", "lambda_out", "hmin_in_um", "hmin_out_um",
          "nu_mm2s", "nu1_mm2s", "a_iso", "a_iso_capped"]


def safe(o, n):
    try:
        return getattr(o, n)
    except Exception:
        return None


def sc(o, n):
    v = safe(o, n)
    if isinstance(v, (int, float)) and not isinstance(v, bool):
        return float(v)
    for a in ("value", "wrapped"):
        w = safe(v, a)
        if isinstance(w, (int, float)) and not isinstance(w, bool):
            return float(w)
    return None


def load_raw(name):
    with open(os.path.join(DLCDIR, name, "raw.csv"), encoding="utf-8-sig") as f:
        return [{k: float(v) for k, v in r.items()} for r in csv.DictReader(f)]


def bin_reps(data, k):
    kp = int(round(DT / DT0))
    n = len(data)
    nb = max(n // kp, 1)
    edges = [(b * kp, (b + 1) * kp) for b in range(nb)]
    if edges and edges[-1][1] < n:
        edges[-1] = (edges[-1][0], n)
    out = []
    for bi, (i0, i1) in enumerate(edges):
        m = i1 - i0
        rec = {key: sum(data[i][key] for i in range(i0, i1)) / m
               for key in ("rpm", "Mx")}
        for key in KSIG:
            mu = sum(data[i][key] for i in range(i0, i1)) / m
            var = sum((data[i][key] - mu) ** 2 for i in range(i0, i1)) / m
            rec[key] = mu + math.copysign(1.0, mu) * k * math.sqrt(var)
        out.append((bi, abs(rec["rpm"]) / 60.0 * (m * DT0), rec))
    return out


def load_k(path, design):
    with open(path, encoding="utf-8-sig") as f:
        return {r["DLC"]: r for r in csv.DictReader(f) if r["design"] == design}


def extract(cda):
    """빈 1개 · 베어링 1개의 윤활 지표"""
    iso = safe(cda, "iso2812007")
    ts = safe(cda, "isots162812008")
    a = sc(ts, "life_modification_factor_for_systems_approach")
    if a is None:
        a = sc(iso, "life_modification_factor_for_systems_approach")
    nu, nu1 = sc(cda, "kinematic_viscosity"), sc(iso, "reference_kinematic_viscosity")
    hi, ho = (sc(cda, "minimum_lubricating_film_thickness_inner"),
              sc(cda, "minimum_lubricating_film_thickness_outer"))
    return dict(
        kappa=sc(iso, "viscosity_ratio"),
        lambda_in=sc(cda, "lambda_ratio_inner"),
        lambda_out=sc(cda, "lambda_ratio_outer"),
        hmin_in_um=None if hi is None else hi * 1e6,
        hmin_out_um=None if ho is None else ho * 1e6,
        nu_mm2s=None if nu is None else nu * 1e6,
        nu1_mm2s=None if nu1 is None else nu1 * 1e6,
        a_iso=a,
        a_iso_capped=int(a is not None and a >= AISO_CAP - 1e-6))


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
    from mastapy.bearings import RollerBearingProfileTypes as RP

    os.makedirs(OUTDIR, exist_ok=True)
    per_bin = os.path.join(OUTDIR, "lub_per_bin.csv")
    done = set()
    if os.path.isfile(per_bin):
        with open(per_bin, encoding="utf-8-sig") as f:
            done = {(r["model"], r["DLC"]) for r in csv.DictReader(f)}
    print(f"[부록 5] 모델 {len(TARGETS)} × DLC 111 · 기존 완료 {len(done):,}")

    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    sh = list(asm.all_parts_of_type_shaft())[0]
    bs = list(asm.all_parts_of_type_bearing())
    uw = [b for b in bs if "UW" in str(b)][0]
    dw = [b for b in bs if "DW" in str(b)][0]
    bmap = {"UW": uw, "DW": dw}
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load()
               if "input" in str(p).lower())
    dp = asm.design_properties
    lc0 = next(c for c in dp.static_loads if c.name == "Load Case 1")
    ds = lc0.design_state_load_case_group
    for b in bs:
        b.detail.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG

    new = not os.path.isfile(per_bin)
    fh = open(per_bin, "a", newline="", encoding="utf-8-sig")
    wr = csv.DictWriter(fh, fieldnames=FIELDS)
    if new:
        wr.writeheader()

    t_all = time.time()
    for tag, dpw, al, dwe, lwe, z1, z2, kpath, kdes in TARGETS:
        g = sg.bearing(dpw, al, dwe, lwe)
        for b in bs:
            try:
                if b.inner_connection is not None:
                    b.inner_connection.delete()
            except Exception:
                pass
        s = sg.shaft(g["bore"], z2)
        sh.remove_all_sections()
        sh.add_section(0.0, s["length"], s["outer_diameter"], s["inner_diameter"],
                       s["outer_diameter"], s["inner_diameter"])
        for b in bs:
            sg.apply_to_masta(b.detail, g)
        for b, z in ((uw, z1), (dw, z2)):
            b.try_mount_on(sh, z)
        det = uw.detail
        print(f"\n[{tag}] D_pw {dpw*1e3:,.0f} α {al:.0f} D_we {dwe*1e3:.0f} "
              f"L_we {lwe*1e3:.0f} z {z1}/{z2}  ·  합성거칠기 "
              f"{sc(det,'combined_surface_roughness_inner')*1e6:.4f} µm", flush=True)

        kmap = load_k(kpath, kdes)
        todo = [n for n in sorted(kmap) if (tag, n) not in done]
        t_m = time.time()
        for i, name in enumerate(todo, 1):
            kr = kmap[name]
            k, sf = float(kr["k"]), float(kr["ScaleFactor"])
            reps = bin_reps(load_raw(name), k)
            for b0 in range(0, len(reps), NBATCH):
                chunk = reps[b0:b0 + NBATCH]
                lcs = []
                for cid, rev, rec in chunk:
                    lc = lc0.duplicate(ds, f"lub_{tag}_{i}_{cid}")
                    mf.set_loads(lc, pl, ipl, rec)
                    lcs.append(lc)
                duty = dp.add_duty_cycle(f"lubdc_{tag}_{i}_{b0}")
                for lc in lcs:
                    duty.add_static_load(lc)
                csd = duty.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
                csd.perform_analysis()
                for key, b in bmap.items():
                    subs = list(list(csd.results_for(b))[0].component_analysis_cases)
                    for (cid, rev, rec), sub in zip(chunk, subs):
                        wr.writerow(dict(
                            model=tag, DLC=name, bin=cid,
                            rpm=round(rec["rpm"], 5), rev=round(rev, 5),
                            k=k, ScaleFactor=sf, brg=key,
                            **{kk: (None if vv is None else
                                    (vv if isinstance(vv, int) else round(vv, 6)))
                               for kk, vv in extract(sub.component_detailed_analysis).items()}))
                for x in lcs + [duty]:
                    try:
                        x.delete()
                    except Exception:
                        pass
                if psutil.virtual_memory().percent > MEM_LIMIT:
                    raise MemoryError("메모리 95% 초과")
            fh.flush()
            if i % 30 == 0 or i == len(todo):
                print(f"    [{i}/{len(todo)}] {name}  ({time.time()-t_m:.0f}s)",
                      flush=True)
        print(f"  → {tag} 완료 {(time.time()-t_m)/60:.1f}분", flush=True)

    fh.close()
    print(f"\n[완료] {(time.time()-t_all)/60:.1f}분 · {per_bin}")


if __name__ == "__main__":
    main()
