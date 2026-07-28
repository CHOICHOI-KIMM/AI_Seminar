"""
사이징 최적화 부록 3-7 — 핀지지 힘평형 대조 (현행 기하 vs L_eff)
================================================================
DLC별해석.md 부록 1 과 동일 양식·동일 빈(DLC1.2-d-s1, dt=20, k=0.26)으로
핀지지 정역학 두 변형의 베어링 반력을 MASTA 와 대조한다.

  변형 1 : 현행 기하 + 자중   L=2.500  A=-0.500  B=3.000   (= 부록 1 재현)
  변형 2 : L_eff  + 자중      L, A, B 를 MASTA 실측 a 로부터 산출

MASTA 기준 = 빔 샤프트 3안 + v1.3 제원 + DIN Lundberg (본 연구 기준)
α = 19°, 30° 두 케이스.

가설: ray ≈ MX/L 이므로 모멘트 계수가 0.4000 → 0.2765 (배율 0.69125) 로 바뀌면
      부록 1 의 R_Y 오차 +45.1%(UW) / +44.5%(DW) 가 각각 +0.30% / -0.11% 로 줄어야 한다.
"""
import csv
import math
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
DLCDIR = os.path.join(RES, "DLC별해석")
OUTDIR = os.path.join(HERE, "부록3_S1기하검증")
sys.path.insert(0, ROOT)

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
V13_SEQ = [("element_diameter", 0.11051), ("roller_length", 0.238048),
           ("outer_diameter", 3.6), ("inner_ring_width", 0.3),
           ("outer_ring_width", 0.253), ("width", 0.31),
           ("number_of_elements", 87)]
NAME, DT, K = "DLC1.2-d-s1", 20, 0.26      # 부록 1 과 동일
Z1, Z2 = 0.5, 3.0
ALPHAS = [19.0, 30.0]
KSIG = ("Fz", "Fy", "Fx", "Mz", "My")


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
    return [{k: float(v) for k, v in r.items()} for r in csv.DictReader(
        open(os.path.join(DLCDIR, name, "raw.csv"), encoding="utf-8-sig"))]


def bin_reps(data, dt, k):
    kp = int(round(dt / 0.1)); n = len(data); nb = max(n // kp, 1)
    edges = [(b * kp, (b + 1) * kp) for b in range(nb)]
    if edges and edges[-1][1] < n:
        edges[-1] = (edges[-1][0], n)
    out = []
    for bi, (i0, i1) in enumerate(edges):
        m = i1 - i0
        rec = {key: sum(data[i][key] for i in range(i0, i1)) / m for key in ("rpm", "Mx")}
        for key in KSIG:
            mu = sum(data[i][key] for i in range(i0, i1)) / m
            var = sum((data[i][key] - mu) ** 2 for i in range(i0, i1)) / m
            rec[key] = mu + math.copysign(1.0, mu) * k * math.sqrt(var)
        out.append((bi, abs(rec["rpm"]) / 60.0 * (m * 0.1), rec))
    return out


def hub_masta(rec):
    return dict(FX=-rec["Fz"] * 1e3, FY=rec["Fy"] * 1e3, FZ=rec["Fx"] * 1e3,
                MX=-rec["Mz"] * 1e3, MY=rec["My"] * 1e3)


class Pin:
    """핀지지 정역학 (자중 포함) — 기하 변형별 인스턴스"""

    def __init__(self, tag, L_, A_, B_, zA, zB, Y1):
        self.tag, self.L, self.A, self.B = tag, L_, A_, B_
        self.zA, self.zB, self.Y1 = zA, zB, Y1
        self.W = self.zW = None

    def base(self, u):
        rax = u["FX"] * (self.B / self.L) - u["MY"] / self.L
        ray = u["FY"] * (self.B / self.L) + u["MX"] / self.L
        rbx = u["FX"] * (self.A / self.L) + u["MY"] / self.L
        rby = u["FY"] * (self.A / self.L) - u["MX"] / self.L
        return rax, ray, rbx, rby

    def calibrate(self, u0, m0):
        """빈 1 의 MASTA 반력에서 자중 W·작용점 z_W 역산"""
        self.W = m0["UW"][0] + m0["DW"][0] - u0["FX"]
        _, _, rbx0, _ = self.base(u0)
        self.zW = self.zA + self.L * (m0["DW"][0] - rbx0) / self.W
        return self.W, self.zW

    def full(self, u):
        rax, ray, rbx, rby = self.base(u)
        rax += self.W * (self.zB - self.zW) / self.L
        rbx += self.W * (self.zW - self.zA) / self.L
        fra, frb = math.hypot(rax, ray), math.hypot(rbx, rby)
        sa, sb = 0.5 * fra / self.Y1, 0.5 * frb / self.Y1
        ka = u["FZ"]
        if ka >= sa - sb:
            faa, fab = ka + sb, -sb
        else:
            faa, fab = sa, -(sa - ka)
        return {"UW": (rax, ray, faa, fra), "DW": (rbx, rby, fab, frb)}


def stat(v):
    return (sum(v) / len(v), min(v), max(v))


def main():
    import masta_clr_legacy  # noqa: F401
    import mastapy
    mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
    import masta_fatigue as mf
    from mastapy.system_model import Design
    from mastapy.system_model.analyses_and_results.static_loads import AnalysisType
    from mastapy.bearings import RollerBearingProfileTypes as RP

    os.makedirs(OUTDIR, exist_ok=True)
    reps = bin_reps(load_raw(NAME), DT, K)
    hubs = [hub_masta(rec) for _, _, rec in reps]
    print(f"[준비] {NAME} dt={DT} k={K} → {len(reps)}빈 (부록 1 과 동일)")

    design = Design.load(MODEL)
    asm = design.all_parts_of_type_root_assembly()[0]
    pl = list(asm.all_parts_of_type_point_load())[0]
    ipl = next(p for p in asm.all_parts_of_type_power_load() if "input" in str(p).lower())
    bmap = {("UW" if "UW" in str(b) else "DW"): b
            for b in asm.all_parts_of_type_bearing()}
    for b in bmap.values():
        det = b.detail
        for k_, v_ in V13_SEQ:
            try:
                setattr(det, k_, v_)
            except Exception as e:
                print(f"  !! {k_}: {str(e).splitlines()[0][:50]}")
        det.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG
    lc0 = next(c for c in asm.design_properties.static_loads if c.name == "Load Case 1")
    print("[모델] 빔 샤프트 3안 + v1.3 제원 + DIN")

    allrows = []
    for alpha in ALPHAS:
        for b in bmap.values():
            b.detail.contact_angle = math.radians(alpha)
        det = bmap["UW"].detail
        a_ = sc(det, "effective_centre_from_front_face")
        T_ = sc(det, "width")
        Y1 = sc(det, "dynamic_axial_load_factor_for_high_axial_radial_load_ratios")
        e_ = sc(det, "limiting_value_for_axial_load_ratio")
        c_ = a_ - T_ / 2
        print(f"\n{'='*70}\n[α = {alpha:.0f}°]  a={a_*1e3:.1f} mm  T/2={T_*1e3/2:.1f} mm"
              f"  c={c_*1e3:.1f} mm  e={e_:.6f}  Y1={Y1:.6f}")

        pins = [
            Pin("현행기하", 2.500000, -0.500000, 3.000000, Z1, Z2, Y1),
            Pin("L_eff", (Z2 - Z1) + 2 * c_, c_ - Z1, Z2 + c_, Z1 - c_, Z2 + c_, Y1),
        ]
        for p in pins:
            print(f"   {p.tag:8} L={p.L:.6f} A={p.A:+.6f} B={p.B:.6f} 1/L={1/p.L:.4f}")

        # ── MASTA 30빈 단건 ──
        mres, t0 = [], time.perf_counter()
        for bi, rev, rec in reps:
            mf.set_loads(lc0, pl, ipl, rec)
            sd = lc0.analysis_of(AnalysisType.SYSTEM_DEFLECTION)
            sd.perform_analysis()
            row = {}
            for key, b in bmap.items():
                r = sd.results_for(b)
                f = r.internal_force
                x, y, z = float(f.x), float(f.y), float(f.z)
                row[key] = (x, y, z, math.hypot(x, y))
            mres.append(row)
        print(f"   [MASTA] {len(reps)}빈 완료 {time.perf_counter()-t0:.1f}s")

        Ws_chk = [m["UW"][0] + m["DW"][0] - u["FX"] for u, m in zip(hubs, mres)]
        for p in pins:
            W, zW = p.calibrate(hubs[0], mres[0])
            print(f"   [자중] {p.tag:8} W={W/1e3:,.1f} kN  z_W={zW:.4f} m"
                  f"   (전 빈 W {min(Ws_chk)/1e3:,.1f}~{max(Ws_chk)/1e3:,.1f} kN)")

        # ── 빈별 오차 ──
        err = {p.tag: {f"{brg}_{comp}": [] for brg in ("UW", "DW")
                       for comp in ("RX", "RY", "RZ", "Fr")} for p in pins}
        for i, (u, m) in enumerate(zip(hubs, mres)):
            row = dict(alpha=alpha, bin=i + 1)
            for p in pins:
                pr = p.full(u)
                for brg in ("UW", "DW"):
                    for j, comp in enumerate(("RX", "RY", "RZ", "Fr")):
                        pv = pr[brg][j] if comp != "Fr" else pr[brg][3]
                        mv = m[brg][j] if comp != "Fr" else m[brg][3]
                        ep = (pv / mv - 1) * 100 if abs(mv) > 1 else float("nan")
                        err[p.tag][f"{brg}_{comp}"].append(ep)
                        row[f"{p.tag}_{brg}_{comp}_pin"] = pv / 1e3
                        row[f"{brg}_{comp}_masta"] = mv / 1e3
                        row[f"{p.tag}_{brg}_{comp}_err"] = ep
            allrows.append(row)

        print(f"\n   {'성분':10} {'현행기하 평균(범위)':>28} {'L_eff 평균(범위)':>28}")
        for brg in ("UW", "DW"):
            for comp in ("RX", "RY", "RZ", "Fr"):
                a1 = stat([x for x in err["현행기하"][f"{brg}_{comp}"] if x == x])
                a2 = stat([x for x in err["L_eff"][f"{brg}_{comp}"] if x == x])
                print(f"   {brg} {comp:6} {a1[0]:+9.2f}% ({a1[1]:+8.1f}~{a1[2]:+8.1f})"
                      f"   {a2[0]:+9.2f}% ({a2[1]:+8.1f}~{a2[2]:+8.1f})")

    with open(os.path.join(OUTDIR, "forcebal_per_bin.csv"), "w", newline="",
              encoding="utf-8-sig") as f:
        keys = []
        for r in allrows:
            for k_ in r:
                if k_ not in keys:
                    keys.append(k_)
        w = csv.DictWriter(f, fieldnames=keys)
        w.writeheader(); w.writerows(allrows)
    print(f"\n[저장] {os.path.join(OUTDIR, 'forcebal_per_bin.csv')}")


if __name__ == "__main__":
    main()
