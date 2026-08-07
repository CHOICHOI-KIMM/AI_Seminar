"""
부록 6 S1 — NSGA-II 평가기 (P1 배치 재사용)
=============================================
MASTA 세션을 한 번 열어두고, NSGA 가 만든 설계점 묶음을 세대 단위로 받아
σ_max · 베어링 질량 · 샤프트 질량을 돌려준다.

해석 절차는 `run_p1_stress_grid.py` 와 **동일**하다 —
  ① 두 베어링 분리 → ② 샤프트 재구성 → ③ 베어링 제원 주입 → ④ 재장착
  ⑤ 지배 LC(Myz_max) 복제 + 듀티사이클 → SYSTEM_DEFLECTION
듀티사이클로 감싸는 것은 기하 변경 시 해석 캐시를 회피하기 위해 필수다.

정수화(§6-4)는 `INTEGERIZE` 로 켜고 끈다.
  S2 재현시험 = 끔(전수 격자와 동일 기준) · S3 본최적화 = 켬

평가 결과는 `eval_cache.csv` 에 누적된다. 같은 설계가 다시 오면
MASTA 를 부르지 않고 캐시에서 돌려준다 — NSGA 는 중복을 실제로 만든다.
"""
import csv
import math
import os
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.dirname(HERE)
ROOT = os.path.dirname(RES)
sys.path.insert(0, HERE)
sys.path.insert(0, ROOT)

import sizing_geom as sg      # noqa: E402

MODEL = (r"D:\AI\AI_Seminar\Main bearing\02. 자료\MASTA"
         r"\26MW_메인베어링_기본설계_v1.4_샤프트 두께,형상 3안"
         r"_베어링 크기 확대_롤러 확대_온도_50도_260726.Masta")
GOV = {"Myz_max": 22673.0}      # 지배 극한 LC (§8-4.2)
LIMIT = 2100.0                  # MPa
FIELDS = ["key", "z1", "z2", "D_pw_mm", "alpha", "D_we_mm", "L_we_mm",
          "slenderness", "Z", "bore_mm", "D_mm", "T_mm", "B_mm", "C_mm",
          "L_eff_m", "mass_brg_kg", "mass_shaft_kg", "mass_total_kg",
          "sigma_max_MPa", "feasible", "warn", "t_s"]


def sc(o, n):
    """스칼라 안전 추출 — run_p1_stress_grid 와 동일"""
    try:
        v = getattr(o, n)
    except Exception:
        return None
    if isinstance(v, (int, float)) and not isinstance(v, bool):
        return float(v)
    for a in ("value", "wrapped"):
        try:
            w = getattr(v, a)
        except Exception:
            continue
        if isinstance(w, (int, float)) and not isinstance(w, bool):
            return float(w)
    return None


# ── 종속변수 (§6-4) ──────────────────────────────────────────────────
def geom(D_pw, alpha_deg, D_we, L_we, integerize):
    """§4-3 사슬 + §6-4 정수화. integerize=False 면 sizing_geom 과 동일."""
    if not integerize:
        return sg.bearing(D_pw, alpha_deg, D_we, L_we)
    ra = math.radians(alpha_deg)
    ca, sa = math.cos(ra), math.sin(ra)
    r1 = lambda x: round(x * 1000) / 1000            # 1 mm 반올림  # noqa: E731
    T, B, C = (r1(sg.T_OVER_LWE * L_we), r1(sg.B_OVER_LWE * L_we),
               r1(sg.C_OVER_LWE * L_we))
    t_i, t_o = sg.TI_OVER_B * B, sg.TO_OVER_C * C
    d = r1(D_pw - D_we * ca - L_we * sa - 2 * t_i)
    D = r1(D_pw + D_we * ca + L_we * sa + 2 * t_o)
    Z = int(sg.ETA * math.pi * D_pw / D_we)
    return dict(D_pw=D_pw, alpha_deg=alpha_deg, D_we=D_we, L_we=L_we,
                t_i=t_i, t_o=t_o, bore=d, outer_diameter=D,
                width=T, inner_ring_width=B, outer_ring_width=C,
                number_of_elements=Z)


def shaft_of(bore, z2, integerize):
    s = sg.shaft(bore, z2)
    if integerize:                                   # 내경만 floor (§6-4.2)
        s["inner_diameter"] = math.floor(bore * sg.ID_OVER_OD * 1000) / 1000
    return s


def repair(D_we, L_we):
    """세장비 수리 (§6-8.1.4 ⒜) — L_we 를 [1.5·D_we, 2.5·D_we] 로 클립"""
    return min(max(L_we, 1.5 * D_we), 2.5 * D_we)


def key_of(z1, z2, D_pw, alpha, D_we, L_we):
    return (f"{z1:.2f}_{z2:.2f}_{D_pw*1e3:.1f}_{alpha:.0f}_"
            f"{D_we*1e3:.2f}_{L_we*1e3:.2f}")


class Evaluator:
    """MASTA 세션 1개를 유지하며 설계점을 평가한다."""

    # ── 확장 지점 (부록 8 이 갈아끼운다) ────────────────────────────
    FIELDS = FIELDS                  # 하위 클래스가 열을 더할 수 있게

    def shaft_of(self, bore, z2, integerize):
        """샤프트 종속제원 — 규칙을 바꾸려면 여기만 재정의한다"""
        return shaft_of(bore, z2, integerize)

    def tweak(self, detail):
        """제원 주입 직후·마운트 직전에 detail 을 더 손볼 자리. 기본은 없음"""

    def finish(self, row, detail):
        """기록 직전 행을 손볼 자리 (열 개명·추가). 기본은 그대로"""
        return row

    def __init__(self, outdir, integerize=True, verbose=True):
        self.outdir = outdir
        self.integerize = integerize
        self.verbose = verbose
        os.makedirs(outdir, exist_ok=True)
        self.cachef = os.path.join(outdir, "eval_cache.csv")
        self.cache = {}
        if os.path.isfile(self.cachef):
            with open(self.cachef, encoding="utf-8-sig") as f:
                for r in csv.DictReader(f):
                    self.cache[r["key"]] = r
        self.n_masta = 0
        self.n_hit = 0
        self._masta = False
        self._fh = None

    # ── MASTA 지연 초기화 ────────────────────────────────────────────
    def _boot(self):
        if self._masta:
            return
        import masta_clr_legacy  # noqa: F401
        import mastapy
        mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
        from mastapy.system_model import Design
        from mastapy.system_model.analyses_and_results.static_loads import (
            AnalysisType)
        from mastapy.bearings import RollerBearingProfileTypes as RP
        self.AnalysisType = AnalysisType

        d = Design.load(MODEL)
        asm = d.all_parts_of_type_root_assembly()[0]
        self.sh = list(asm.all_parts_of_type_shaft())[0]
        self.bs = list(asm.all_parts_of_type_bearing())
        self.uw = [b for b in self.bs if "UW" in str(b)][0]
        self.dw = [b for b in self.bs if "DW" in str(b)][0]
        ipl = next(p for p in asm.all_parts_of_type_power_load()
                   if "input" in str(p).lower())
        for b in self.bs:
            b.detail.roller_profile_set.active_profile_type = RP.DIN_LUNDBERG
        self.dp = asm.design_properties
        self.lc_tpl = {}
        for nm, tq in GOV.items():
            lc = next(c for c in self.dp.static_loads if c.name == nm)
            q = lc.inputs_for_power_load(ipl)
            for a_, v_ in (("speed", 0.0), ("torque", tq * 1e3)):
                try:
                    setattr(q, a_, v_)
                except Exception:
                    pass
            self.lc_tpl[nm] = lc
        self.ds = self.lc_tpl["Myz_max"].design_state_load_case_group
        self._masta = True
        if self.verbose:
            print("[평가기] MASTA 모델 로드 완료", flush=True)

    def _writer(self):
        if self._fh is None:
            new = not os.path.isfile(self.cachef)
            self._fh = open(self.cachef, "a", newline="", encoding="utf-8-sig")
            self._w = csv.DictWriter(self._fh, fieldnames=self.FIELDS)
            if new:
                self._w.writeheader()
        return self._w

    # ── 1점 평가 ────────────────────────────────────────────────────
    def _one(self, n, z1, z2, D_pw, alpha, D_we, L_we):
        t1 = time.perf_counter()
        g = geom(D_pw, alpha, D_we, L_we, self.integerize)
        warn = []
        for b in self.bs:
            try:
                if b.inner_connection is not None:
                    b.inner_connection.delete()
            except Exception as e:
                warn.append(f"unmount:{str(e).splitlines()[0][:28]}")
        s = self.shaft_of(g["bore"], z2, self.integerize)
        try:
            self.sh.remove_all_sections()
            self.sh.add_section(0.0, s["length"], s["outer_diameter"],
                                s["inner_diameter"], s["outer_diameter"],
                                s["inner_diameter"])
        except Exception as e:
            warn.append(f"shaft:{str(e).splitlines()[0][:32]}")
        for b in self.bs:
            bad = sg.apply_to_masta(b.detail, g)
            if bad:
                warn.append("spec:" + bad[0])
            self.tweak(b.detail)
        for b, z in ((self.uw, z1), (self.dw, z2)):
            try:
                b.try_mount_on(self.sh, z)
            except Exception as e:
                warn.append(f"mount{z}:{str(e).splitlines()[0][:26]}")

        a_m = sc(self.uw.detail, "effective_centre_from_front_face")
        T_m = sc(self.uw.detail, "width")
        L_eff = ((z2 - z1) + 2 * (a_m - T_m / 2)
                 if (a_m is not None and T_m is not None) else None)

        names, dups = [], []
        for nm in GOV:
            dups.append(self.lc_tpl[nm].duplicate(self.ds, f"ns_{n}_{nm}"))
            names.append(nm)
        duty = self.dp.add_duty_cycle(f"nsdc_{n}")
        for lc in dups:
            duty.add_static_load(lc)
        csd = duty.analysis_of(self.AnalysisType.SYSTEM_DEFLECTION)
        csd.perform_analysis()
        best = 0.0
        for b in (self.uw, self.dw):
            subs = list(list(csd.results_for(b))[0].component_analysis_cases)
            for sub in subs:
                v = sc(sub.component_detailed_analysis, "maximum_normal_stress")
                if v and v / 1e6 > best:
                    best = v / 1e6
        for x in dups + [duty]:
            try:
                x.delete()
            except Exception:
                pass
        if best <= 0.0:
            warn.append("sigma=0")
        mb = sc(self.uw.detail, "mass") or 0.0
        ms = sc(self.sh, "mass_of_shaft_body") or 0.0
        return self.finish(dict(
            key=key_of(z1, z2, D_pw, alpha, D_we, L_we),
            z1=z1, z2=z2, D_pw_mm=round(D_pw * 1e3, 1), alpha=alpha,
            D_we_mm=round(D_we * 1e3, 2), L_we_mm=round(L_we * 1e3, 2),
            slenderness=round(L_we / D_we, 4), Z=g["number_of_elements"],
            bore_mm=round(g["bore"] * 1e3, 1), D_mm=round(g["outer_diameter"]*1e3, 1),
            T_mm=round(g["width"] * 1e3, 1),
            B_mm=round(g["inner_ring_width"] * 1e3, 1),
            C_mm=round(g["outer_ring_width"] * 1e3, 1),
            L_eff_m=None if L_eff is None else round(L_eff, 5),
            mass_brg_kg=round(mb, 1), mass_shaft_kg=round(ms, 1),
            mass_total_kg=round(2 * mb + ms, 1),
            sigma_max_MPa=round(best, 1),
            feasible=1 if (0 < best < LIMIT) else 0,
            warn="|".join(warn) if warn else "",
            t_s=round(time.perf_counter() - t1, 2)), self.uw.detail)

    # ── 세대 단위 평가 ──────────────────────────────────────────────
    def evaluate(self, pts):
        """pts: [(z1, z2, D_pw, alpha, D_we, L_we)] (SI · m·deg) → [dict]"""
        out, todo = [None] * len(pts), []
        for i, p in enumerate(pts):
            k = key_of(*p)
            r = self.cache.get(k)
            if r is not None:
                self.n_hit += 1
                out[i] = r
            else:
                todo.append((i, p))
        if todo:
            self._boot()
            w = self._writer()
            for i, p in todo:
                self.n_masta += 1
                r = self._one(self.n_masta, *p)
                self.cache[r["key"]] = r
                out[i] = r
                w.writerow(r)
            self._fh.flush()
        return out

    def close(self):
        if self._fh:
            self._fh.close()
            self._fh = None


if __name__ == "__main__":       # 자기검증 — v1.3 기준선 1점
    ev = Evaluator(os.path.join(HERE, "부록6_NSGA", "_selftest"),
                   integerize=False)
    r = ev.evaluate([(0.5, 3.0, 3.3309, 19.0, 0.11051, 0.238048)])[0]
    ev.close()
    print(f"  bore {r['bore_mm']} (기대 3055) · D {r['D_mm']} (기대 3600) · "
          f"T {r['T_mm']} (기대 310) · Z {r['Z']} (기대 87)")
    print(f"  σ {r['sigma_max_MPa']} MPa · 베어링 {float(r['mass_brg_kg'])/1000:.3f} t "
          f"(기대 5.600) · 샤프트 {float(r['mass_shaft_kg'])/1000:.3f} t (기대 43.226)")
    print(f"  L_eff {r['L_eff_m']} (기대 3.61666) · {r['t_s']} s")
