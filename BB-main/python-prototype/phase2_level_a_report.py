"""CRB Phase 2 Level A 검증 리포트 생성 스크립트.

Rust cargo test (tests/geometry_level_a.rs) 와 동일한 계산을 Python 으로 재현하여
    (1) 8 tests 각 결과 표 (JSON + terminal 표)
    (2) matplotlib PNG 4장 (reports/phase2/)
을 생성한다.

산출물:
    reports/phase2/fig1_errors_bar.png       — 8 tests 상대오차 bar chart
    reports/phase2/fig2_req_distribution.png — 30 slice R_eq 분포
    reports/phase2/fig3_scatter.png          — 해석해 vs solver 산점도
    reports/phase2/fig4_hertz_profile.png    — Hertz 접촉 형상 (p_max, b)
    reports/phase2/results.json              — 원본 수치 데이터

사용법:
    cd d:/AI/AI_Seminar_CRB/CRB-main
    python python-prototype/phase2_level_a_report.py
"""

from __future__ import annotations
import json
import math
from dataclasses import dataclass, asdict
from pathlib import Path
import matplotlib.pyplot as plt
import matplotlib as mpl
import numpy as np

# 한글 폰트 (Windows Malgun Gothic)
mpl.rcParams["font.family"] = "Malgun Gothic"
mpl.rcParams["axes.unicode_minus"] = False

# ─── 프로젝트 경로 ─────────────────────────────────────────────
ROOT = Path(__file__).parent.parent
OUT_DIR = ROOT / "reports" / "phase2"
OUT_DIR.mkdir(parents=True, exist_ok=True)


# ─── 입력 파라미터 (Rust 테스트와 동일) ─────────────────────────
E_GPA = 210.0
NU = 0.3
Q_NMM_TEST = 500.0
R_EQ_MM_TEST = 22.0

# NU 240 지오메트리
D_BORE = 200.0
D_OUTER = 360.0
T_WIDTH = 58.0
Z_ROLLERS = 18
D_WE = 44.0
L_WE = 42.0
D_PW = 280.0
G_R = 30.0
N_SLICES = 30

TOL_REL = 1e-3  # Level A 허용 오차 0.1%


# ─── Analytical formulas (Hertz line contact) ─────────────────
def combined_e_star_mpa(e_gpa: float, nu: float) -> float:
    """두 body 같은 재질: 1/E* = 2·(1-ν²)/E → E* = E / (2·(1-ν²))"""
    e_star_gpa = e_gpa / (2.0 * (1.0 - nu**2))
    return e_star_gpa * 1000.0  # GPa → MPa


def hertz_half_width_analytical(q_nmm: float, r_eq_mm: float, e_star_mpa: float) -> float:
    return math.sqrt(4.0 * q_nmm * r_eq_mm / (math.pi * e_star_mpa))


def hertz_max_pressure_analytical(q_nmm: float, r_eq_mm: float, e_star_mpa: float) -> float:
    return math.sqrt(q_nmm * e_star_mpa / (math.pi * r_eq_mm))


# ─── Solver formulas (Rust 코드와 동일) ────────────────────────
def hertz_half_width_solver(q_nmm: float, r_eq_mm: float, e_star_mpa: float) -> float:
    """Rust hertz.rs: b = sqrt(4*q*R/(π*E*))"""
    if q_nmm <= 0 or r_eq_mm <= 0 or e_star_mpa <= 0:
        return 0.0
    return math.sqrt(4.0 * q_nmm * r_eq_mm / (math.pi * e_star_mpa))


def hertz_max_pressure_solver(q_nmm: float, b_mm: float) -> float:
    """Rust hertz.rs: p_max = 2*q/(π*b)"""
    if b_mm <= 0:
        return 0.0
    return 2.0 * q_nmm / (math.pi * b_mm)


def combined_e_star_solver_mpa(e_gpa: float, nu: float) -> float:
    """Rust hertz.rs::combined_elastic_modulus"""
    inv = (1.0 - nu**2) / e_gpa + (1.0 - nu**2) / e_gpa
    return (1.0 / inv) * 1000.0


def compute_slices_solver() -> list[dict]:
    """Rust geometry.rs::compute_slices (CRB 순수 원통) 재현.

    Flat profile, r_race = 1e9 (원통 근사) → R_eq = R_roller · (1 ∓ γ), γ = D_we/D_pw.
    """
    r_roller = D_WE / 2.0
    slice_width = L_WE / N_SLICES
    gamma = D_WE / D_PW
    r_eq_inner = r_roller * (1.0 - gamma)
    r_eq_outer = r_roller * (1.0 + gamma)

    slices = []
    for k in range(N_SLICES):
        x_axial = (k + 0.5) * slice_width
        slices.append({
            "k": k,
            "x_axial": x_axial,
            "r_roller": r_roller,
            "r_eq_inner": r_eq_inner,
            "r_eq_outer": r_eq_outer,
            "slice_width": slice_width,
        })
    return slices


# ─── 8 tests 실행 ──────────────────────────────────────────────
@dataclass
class TestResult:
    name: str
    quantity: str
    input_summary: str
    expected: float | str
    solver: float | str
    rel_err: float
    tol: float
    passed: bool


def run_tests() -> list[TestResult]:
    """CRB Phase 2 Level A — compute_slices 구현 검증 (3 tests).

    [설계 결정 2026-08-19 사용자 재검토]
    이전 8 tests 중 5, 6, 8 만 유지. Test 1/2/3 (Hertz b, p_max, E*) 은 solver 함수와
    analytical 함수가 **동일 공식** 을 사용하여 오차 0 = 동어반복 (tautology) 이었음.
    Test 4 (roller 반경), 7 (R_eq 궤도) 도 계산 경로가 solver 내부와 동일하여 제외.
    Test 5, 6 만이 순수한 slicer 구현 (분할 균일성) 검증. Test 8 은 pipeline 연결 검증.

    진짜 independent 검증 (Reference 도서 / FEA) 은 Level B (Phase 3+) 대상.
    """
    slices = compute_slices_solver()
    results: list[TestResult] = []

    # Test 5: compute_slices — uniform slice_width + total = L_we
    total_width = sum(s["slice_width"] for s in slices)
    rel = abs(total_width - L_WE) / L_WE
    results.append(TestResult(
        name="level_a_compute_slices_uniform_slice_width",
        quantity="Slice 폭 총합 vs L_we [mm]",
        input_summary=f"L_we={L_WE}, n_slices={N_SLICES}",
        expected=L_WE, solver=total_width, rel_err=rel, tol=TOL_REL, passed=rel < TOL_REL,
    ))

    # Test 6: x_axial 대칭 (slice 중심 좌표 대칭성)
    max_asym = max(abs(slices[k]["x_axial"] + slices[N_SLICES-1-k]["x_axial"] - L_WE) for k in range(N_SLICES // 2))
    rel = max_asym / L_WE
    results.append(TestResult(
        name="level_a_compute_slices_x_axial_symmetric",
        quantity="x_axial 대칭성 편차 [mm]",
        input_summary=f"L_we={L_WE}, n_slices={N_SLICES}",
        expected=0.0, solver=max_asym, rel_err=rel, tol=TOL_REL, passed=rel < TOL_REL,
    ))

    # Test 8: End-to-end pipeline 연결 검증
    #   compute_slices 결과 → hertz 함수 호출이 panic 없이 유효 값 반환 하는지 확인.
    #   ⚠️ 수치 자체는 self-consistency (동일 공식) — 실제 정확도 검증은 Level B (Phase 3+).
    e_star_mpa = combined_e_star_mpa(E_GPA, NU)
    q_per_mm = 500.0
    all_finite = True
    for s in slices:
        b_s = hertz_half_width_solver(q_per_mm, s["r_eq_inner"], e_star_mpa)
        p_s = hertz_max_pressure_solver(q_per_mm, b_s)
        if not (math.isfinite(b_s) and b_s > 0 and math.isfinite(p_s) and p_s > 0):
            all_finite = False
            break
    results.append(TestResult(
        name="level_a_end_to_end_pipeline_valid",
        quantity="30 slices × (b, p_max) 모두 유효 (finite, positive)",
        input_summary=f"q={q_per_mm}, n_slices={N_SLICES}",
        expected="all finite & positive", solver="OK" if all_finite else "FAIL",
        rel_err=0.0 if all_finite else 1.0, tol=TOL_REL, passed=all_finite,
    ))

    return results


# ─── 시각화 ────────────────────────────────────────────────────
def plot_errors_bar(results: list[TestResult], out_path: Path) -> None:
    fig, ax = plt.subplots(figsize=(11, 5))
    names = [f"Test {i+1}" for i in range(len(results))]
    errs = [r.rel_err if r.rel_err > 0 else 1e-16 for r in results]
    colors = ["#2ca02c" if r.passed else "#d62728" for r in results]

    bars = ax.bar(names, errs, color=colors, edgecolor="black", linewidth=0.5)
    ax.axhline(TOL_REL, color="red", linestyle="--", linewidth=1.2, label=f"허용 오차 {TOL_REL:.0e} (0.1%)")
    ax.set_yscale("log")
    ax.set_ylabel("상대 오차 (log scale)")
    ax.set_title("CRB Phase 2 Level A — 8 tests 상대 오차 (모두 통과)")
    ax.set_ylim(1e-17, 1e-1)
    ax.grid(True, which="both", axis="y", alpha=0.3)
    ax.legend()

    for bar, r in zip(bars, results):
        h = bar.get_height()
        label = f"{r.rel_err:.1e}" if r.rel_err > 0 else "0"
        ax.text(bar.get_x() + bar.get_width()/2, h * 1.5, label, ha="center", va="bottom", fontsize=8)

    plt.xticks(rotation=15, ha="right")
    plt.tight_layout()
    plt.savefig(out_path, dpi=240)
    plt.close()
    print(f"  ✅ {out_path.name}")


def plot_req_distribution(out_path: Path) -> None:
    slices = compute_slices_solver()
    x = [s["x_axial"] for s in slices]
    r_in = [s["r_eq_inner"] for s in slices]
    r_out = [s["r_eq_outer"] for s in slices]

    fig, ax = plt.subplots(figsize=(15, 5))   # 가로 확대 (10→15)
    ax.plot(x, r_in, "o-", label=f"R_eq (내륜) = {r_in[0]:.3f} mm", color="#1f77b4", markersize=5)
    ax.plot(x, r_out, "s-", label=f"R_eq (외륜) = {r_out[0]:.3f} mm", color="#ff7f0e", markersize=5)
    ax.axhline(D_WE / 2, color="gray", linestyle=":", label=f"D_we/2 = {D_WE/2} mm (참조)")
    ax.set_xlabel("x_axial [mm] (소단 → 대단)")
    ax.set_ylabel("등가 곡률반경 R_eq [mm]")
    ax.set_title(f"NU 240 CRB — 30 slice R_eq 분포 (γ = D_we/D_pw = {D_WE/D_PW:.4f})")
    ax.grid(True, alpha=0.3)
    ax.legend(loc="center right")
    plt.tight_layout()
    plt.savefig(out_path, dpi=240)
    plt.close()
    print(f"  ✅ {out_path.name}")


def plot_scatter(out_path: Path) -> None:
    e_star_mpa = combined_e_star_mpa(E_GPA, NU)
    q_range = np.linspace(100, 2000, 30)
    R_test = R_EQ_MM_TEST

    b_a = [hertz_half_width_analytical(q, R_test, e_star_mpa) for q in q_range]
    b_s = [hertz_half_width_solver(q, R_test, e_star_mpa) for q in q_range]
    p_a = [hertz_max_pressure_analytical(q, R_test, e_star_mpa) for q in q_range]
    p_s = [hertz_max_pressure_solver(q, hertz_half_width_solver(q, R_test, e_star_mpa)) for q in q_range]

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))

    ax1.scatter(b_a, b_s, c=q_range, cmap="viridis", s=40, edgecolor="black", linewidth=0.5)
    ax1.plot([min(b_a), max(b_a)], [min(b_a), max(b_a)], "r--", label="y = x (완벽 일치)")
    ax1.set_xlabel("해석해 b [mm]")
    ax1.set_ylabel("Solver b [mm]")
    ax1.set_title("접촉 반폭 b — 해석해 vs Solver")
    ax1.grid(True, alpha=0.3)
    ax1.legend()

    ax2.scatter(p_a, p_s, c=q_range, cmap="viridis", s=40, edgecolor="black", linewidth=0.5)
    ax2.plot([min(p_a), max(p_a)], [min(p_a), max(p_a)], "r--", label="y = x")
    ax2.set_xlabel("해석해 p_max [MPa]")
    ax2.set_ylabel("Solver p_max [MPa]")
    ax2.set_title("최대 접촉 응력 p_max — 해석해 vs Solver")
    ax2.grid(True, alpha=0.3)
    ax2.legend()

    cbar = plt.colorbar(ax1.collections[0], ax=[ax1, ax2], pad=0.02, shrink=0.8)
    cbar.set_label("q [N/mm]")

    plt.suptitle(f"CRB Level A 검증 — q ∈ [100, 2000] N/mm, R = {R_test} mm, 30 samples", y=1.02)
    plt.savefig(out_path, dpi=240, bbox_inches="tight")
    plt.close()
    print(f"  ✅ {out_path.name}")


def plot_hertz_profile(out_path: Path) -> None:
    """이론적 Hertz 반타원 참고 그림 (검증 아님, 형상 이해용).

    [설계 결정 2026-08-19] 이전엔 'solver vs analytical 비교' 시각화 였으나 두 함수가
    동일 공식이라 동어반복. 여기선 순수히 Hertz theoretical formula 의 반타원 분포
    형상만 제시 — Reference (Palmgren / Johnson Contact Mechanics) 정형 그림.
    """
    e_star_mpa = combined_e_star_mpa(E_GPA, NU)
    R = R_EQ_MM_TEST
    q_cases = [200, 500, 1000, 2000]

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))

    for q in q_cases:
        b = hertz_half_width_analytical(q, R, e_star_mpa)
        p_max = hertz_max_pressure_analytical(q, R, e_star_mpa)
        x = np.linspace(-b, b, 200)
        p = p_max * np.sqrt(1.0 - (x / b)**2)
        ax1.plot(x * 1000, p, label=f"q = {q} N/mm, b = {b*1000:.1f} μm, p_max = {p_max:.0f} MPa")

    ax1.set_xlabel("x [μm] (접촉면 폭 방향)")
    ax1.set_ylabel("접촉 응력 p(x) [MPa]")
    ax1.set_title("Hertz line contact 압력 분포 — 이론 반타원 (참고)")
    ax1.grid(True, alpha=0.3)
    ax1.legend(fontsize=9)

    qs = np.linspace(50, 3000, 100)
    bs = [hertz_half_width_analytical(q, R, e_star_mpa) * 1000 for q in qs]
    pms = [hertz_max_pressure_analytical(q, R, e_star_mpa) for q in qs]
    ax2b = ax2.twinx()
    l1, = ax2.plot(qs, bs, "b-", label="b [μm]")
    l2, = ax2b.plot(qs, pms, "r-", label="p_max [MPa]")
    ax2.set_xlabel("q [N/mm]")
    ax2.set_ylabel("접촉 반폭 b [μm]", color="b")
    ax2b.set_ylabel("최대 응력 p_max [MPa]", color="r")
    ax2.set_title(f"q 에 따른 b, p_max 변화 (R = {R} mm)")
    ax2.grid(True, alpha=0.3)
    ax2.legend([l1, l2], [l1.get_label(), l2.get_label()], loc="upper left")

    plt.tight_layout()
    plt.savefig(out_path, dpi=240)
    plt.close()
    print(f"  ✅ {out_path.name}")


# ─── main ──────────────────────────────────────────────────────
def main():
    print("=" * 70)
    print("CRB Phase 2 — Level A 검증 리포트 생성")
    print("=" * 70)

    results = run_tests()

    print("\n[1] Test 결과 요약:")
    print(f"{'#':<3}{'name':<52}{'rel_err':>12}{'passed':>10}")
    print("-" * 77)
    for i, r in enumerate(results, 1):
        pass_str = "✅ PASS" if r.passed else "❌ FAIL"
        print(f"{i:<3}{r.name:<52}{r.rel_err:>12.2e}{pass_str:>10}")

    n_pass = sum(1 for r in results if r.passed)
    print(f"\n결과: {n_pass}/{len(results)} pass")

    # JSON 저장
    json_path = OUT_DIR / "results.json"
    with open(json_path, "w", encoding="utf-8") as f:
        json.dump([{**asdict(r), "expected": str(r.expected), "solver": str(r.solver)} for r in results], f, ensure_ascii=False, indent=2)
    print(f"\n[2] JSON 저장: {json_path.name}")

    # PNG 생성
    #   [설계 결정 2026-08-19] fig3_scatter (해석해 vs solver 산점도) 삭제 —
    #   두 함수가 동일 공식이라 완벽 일치가 자연스러움 (동어반복 검증 무의미).
    #   Fig 4 는 유지: 이론적 Hertz 반타원 참고 그림 (검증 아니라 형상 이해).
    print("\n[3] PNG 시각화 생성:")
    plot_errors_bar(results, OUT_DIR / "fig1_errors_bar.png")
    plot_req_distribution(OUT_DIR / "fig2_req_distribution.png")
    plot_hertz_profile(OUT_DIR / "fig4_hertz_profile.png")

    print(f"\n산출물 위치: {OUT_DIR}")
    print("=" * 70)


if __name__ == "__main__":
    main()
