# -*- coding: utf-8 -*-
"""§10-12.6 공용 — 모델 조립 · 길이방향 분포 지표 · Fujiwara 프로파일 생성

지표 정의 (모두 `Myz_max` 의 최대하중 요소 · 내륜 기준)

  sigma       max(normal_stress_inner)                    [MPa]
  y_star      sigma 가 발생하는 축방향 위치               [mm]  (좌단 −L_we/2)
  y_star_pct  |y_star| / (L_we/2)                         [%]   0 = 롤러 중앙
  edge_L      최좌 접촉점의 응력                          [MPa]
  margin_L    롤러 좌단 → 최좌 접촉점 거리                [mm]
  edge_R / margin_R                                       우측 동일
"""
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.dirname(os.path.dirname(HERE)))

import a8_eval                    # noqa: E402
import nsga_eval as ne            # noqa: E402
import sizing_geom as sg          # noqa: E402
import run_appendix7_shaft as a7  # noqa: E402

PARETO = os.path.join(HERE, "부록10_NSGA", "S3_본최적화", "a10_pareto.csv")
GOV, TQ = "Myz_max", 22673.0
E_PRIME = 2.2637e11          # 2 / ((1−ν²)/E ×2) · 강-강 · ν=0.3 E=206 GPa
EPS = 1.0                    # 접촉 판정 하한 [MPa]


def e_prime(E=2.06e11, nu=0.3):
    return 2.0 / (2.0 * (1.0 - nu * nu) / E)


# ── Fujiwara–Kawase 2006 수정 로그 프로파일 ────────────────────────
def fujiwara(y, a, A, K1, K2, zm):
    """편측 낙차 [m]. y 는 롤러 중심 기준 [m]. 대칭이면 |y| 를 넣는다.

    z(y) = K1·A·ln[ 1 / (1 − {1−exp(−zm/(K1·A))}·((|y|−a)/(K2·a) + 1)²) ]
    정의역 |y| > a(1−K2) — 그 안쪽은 직선부(낙차 0).
    """
    t = (abs(y) - a) / (K2 * a) + 1.0
    if t <= 0.0:
        return 0.0
    c = 1.0 - math.exp(-zm / (K1 * A))
    d = 1.0 - c * t * t
    if d <= 1e-12:
        d = 1e-12
    return K1 * A * math.log(1.0 / d)


def amplitude(Q, Lwe, Ep=None):
    """A = 2Q / (π·L_we·E′)"""
    return 2.0 * Q / (math.pi * Lwe * (Ep or e_prime()))


def profile_fn(Lwe, K1L, K2L, zmL, K1R=None, K2R=None, zmR=None,
               Q=None, Ep=None):
    """위치 y[m] → 낙차 z[m] 함수. 좌(−) / 우(+) 파라미터 독립 가능."""
    a = Lwe / 2.0
    A = amplitude(Q, Lwe, Ep)
    k1r = K1L if K1R is None else K1R
    k2r = K2L if K2R is None else K2R
    zmr = zmL if zmR is None else zmR

    def z(y):
        if y < 0.0:
            return fujiwara(y, a, A, K1L, K2L, zmL)
        return fujiwara(y, a, A, k1r, k2r, zmr)
    return z


def profile_points(Lwe, n, K1L, K2L, zmL, K1R=None, K2R=None, zmR=None,
                   Q=None, Ep=None):
    """(position[m], deviation[m]) 목록 — 그림·기록용."""
    a = Lwe / 2.0
    f = profile_fn(Lwe, K1L, K2L, zmL, K1R, K2R, zmR, Q, Ep)
    return [(-a + 2.0 * a * i / (n - 1),
             f(-a + 2.0 * a * i / (n - 1))) for i in range(n)]


# ── MASTA 조립 · 해석 · 지표 ──────────────────────────────────────
def sc(o, n):
    try:
        v = getattr(o, n)
    except Exception:
        return None
    return float(v) if isinstance(v, (int, float)) else None


class Rig(object):
    """모델을 한 번 열고 설계를 바꿔 가며 해석한다."""

    def __init__(self):
        import masta_clr_legacy  # noqa: F401
        import mastapy
        mastapy.init(r"C:\Program Files\SMT\MASTA 14.1.1")
        from mastapy.system_model import Design
        from mastapy.system_model.analyses_and_results.static_loads import (
            AnalysisType)
        from mastapy.bearings import RollerBearingProfileTypes as RP
        from mastapy.bearings.roller_bearing_profiles import ProfileDataToUse
        self.AT, self.RP, self.PD = AnalysisType, RP, ProfileDataToUse

        self.d = Design.load(a7.MODEL)
        asm = self.d.all_parts_of_type_root_assembly()[0]
        self.asm = asm
        self.sh = list(asm.all_parts_of_type_shaft())[0]
        self.bs = list(asm.all_parts_of_type_bearing())
        self.uw = [b for b in self.bs if "UW" in str(b)][0]
        self.dw = [b for b in self.bs if "DW" in str(b)][0]
        self.dp = asm.design_properties
        self._n = 0

    def load_case(self, name=GOV, torque=TQ):
        """토크가 nan 인 LC 에만 지배토크를 넣는다 (§부록 7 주의사항)."""
        asm = self.asm
        ipl = next(p for p in asm.all_parts_of_type_power_load()
                   if "input" in str(p).lower())
        lc = next(c for c in self.dp.static_loads if c.name == name)
        q = lc.inputs_for_power_load(ipl)
        try:
            q.speed = 0.0
        except Exception:
            pass
        try:
            t = float(q.torque)
            if t != t:                       # nan
                q.torque = torque * 1e3
        except Exception:
            try:
                q.torque = torque * 1e3
            except Exception:
                pass
        self.lc = lc
        return lc

    def set_jg(self, design_load):
        """Johns-Gohar = Fujiwara 의 K1 = K2 = 1 특수해. end_drop 을 돌려준다."""
        for b in self.bs:
            ps = b.detail.roller_profile_set
            ps.active_profile_type = self.RP.JOHNS_GOHAR
            ps.active_profile.design_load = design_load
        return float(self.uw.detail.roller_profile_set
                     .active_profile.end_drop)

    def build(self, row):
        """a10_pareto.csv 한 행으로 샤프트·베어링을 세운다."""
        z1, z2 = float(row["z1"]), float(row["z2"])
        g = ne.geom(float(row["D_pw_mm"]) / 1e3, float(row["alpha"]),
                    float(row["D_we_mm"]) / 1e3,
                    float(row["L_w_mm"]) / 1e3, True)
        od = g["bore"]
        idm = a8_eval.shaft_id(od)
        for b in self.bs:
            try:
                if b.inner_connection is not None:
                    b.inner_connection.delete()
            except Exception:
                pass
        self.sh.remove_all_sections()
        self.sh.add_section(0.0, z2 + sg.SHAFT_TAIL, od, idm, od, idm)
        for b in self.bs:
            # 이전 설계의 USERSPECIFIED 좌표가 새 롤러 길이를 벗어나 해석이
            # 깨지는 것을 막는다 — 제원 주입 전에 표준 프로파일로 되돌린다
            ps0 = b.detail.roller_profile_set
            ps0.active_profile_type = self.RP.DIN_LUNDBERG
            sg.apply_to_masta(b.detail, g)
            for a in ("left_element_corner_radius",
                      "right_element_corner_radius"):
                setattr(b.detail, a, a8_eval.R_CORNER)
        for b, z in ((self.uw, z1), (self.dw, z2)):
            b.try_mount_on(self.sh, z)
        self.Lwe = self.uw.detail.effective_roller_length
        return g

    def set_din(self, axial_offset=0.0):
        for b in self.bs:
            ps = b.detail.roller_profile_set
            ps.active_profile_type = self.RP.DIN_LUNDBERG
            ps.active_profile.axial_offset = axial_offset

    def set_user(self, zfun, n=61, targets=None):
        """낙차 함수 z(y[m]) → USERSPECIFIED 주입.

        위치를 직접 쓰면 이전 설계의 좌표와 순서가 엉켜 해석이 깨진다.
        `number_of_points` 를 흔들어 **현 롤러 길이에 맞게 재생성**시킨 뒤,
        MASTA 가 준 위치에서 수식을 평가해 낙차만 넣는다.
        """
        for b in (self.bs if targets is None else targets):
            ps = b.detail.roller_profile_set
            ps.active_profile_type = self.RP.USERSPECIFIED
            up = ps.active_profile
            up.number_of_points = 2
            up.number_of_points = n
            up.data_to_use = self.PD.ACTUAL_DATA
            for p in up.points:
                p.deviation = zfun(float(p.position))
        pts = list(self.uw.detail.roller_profile_set.active_profile.points)
        return [(float(p.position), float(p.deviation)) for p in pts]

    def solve(self, tag=None):
        self._n += 1
        tag = tag or f"p{self._n}"
        dup = self.lc.duplicate(self.lc.design_state_load_case_group,
                                f"pr_{tag}")
        duty = self.dp.add_duty_cycle(f"prd_{tag}")
        duty.add_static_load(dup)
        csd = duty.analysis_of(self.AT.SYSTEM_DEFLECTION)
        csd.perform_analysis()
        det = list(list(csd.results_for(self.uw))[0]
                   .component_analysis_cases)[0].component_detailed_analysis
        if det is None:
            for x in (dup, duty):
                try:
                    x.delete()
                except Exception:
                    pass
            raise RuntimeError("SYSTEM_DEFLECTION 결과 없음 (프로파일 부적합)")
        m = metrics(det, self.Lwe)
        for x in (dup, duty):
            try:
                x.delete()
            except Exception:
                pass
        return m


def metrics(det, Lwe):
    """길이방향 분포에서 지표를 뽑는다."""
    row = det.rows[0]
    els = list(row.elements)
    e = max(els, key=lambda x: sc(x, "normal_load_inner") or 0.0)
    off = [(float(o.offset) * 1e3, float(o.normal_stress_inner) / 1e6)
           for o in e.results_at_roller_offsets]
    off.sort(key=lambda t: t[0])
    half = Lwe * 1e3 / 2.0
    live = [(p, s) for p, s in off if s > EPS]
    out = dict(L_we_mm=round(Lwe * 1e3, 1),
               sigma_MPa=round(sc(det, "maximum_normal_stress_inner") / 1e6, 1),
               edge_row_MPa=round(sc(row, "maximum_normal_edge_stress_inner")
                                  / 1e6, 1),
               n_contact=int(sc(det, "number_of_elements_in_contact") or 0),
               P_max_N=sc(det, "maximum_normal_load_inner"),
               tilt_mrad=round((sc(e, "element_tilt") or 0) * 1e3, 4))
    if not live:
        out.update(dict(y_star_mm=None, y_star_pct=None, edge_L_MPa=0.0,
                        edge_R_MPa=0.0, margin_L_mm=half, margin_R_mm=half,
                        contact_mm=0.0))
        return out, off
    ys = max(live, key=lambda t: t[1])
    out.update(dict(
        y_star_mm=round(ys[0], 2),
        y_star_pct=round(100.0 * abs(ys[0]) / half, 1),
        edge_L_MPa=round(live[0][1], 1),
        edge_R_MPa=round(live[-1][1], 1),
        margin_L_mm=round(live[0][0] + half, 2),
        margin_R_mm=round(half - live[-1][0], 2),
        contact_mm=round(live[-1][0] - live[0][0], 1)))
    return out, off
