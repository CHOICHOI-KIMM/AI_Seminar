"""CRB Phase 4 Level D 리포트 — Bearing equilibrium (3-DOF NR) 결과 시각화.

Rust cargo test (tests/bearing_level_d.rs, bearing_smoke.rs) 실측 값을 하드코딩하여
    - Q_j polar plot (roller-by-roller normal load)
    - Sjovall 이론값 vs Solver 비교 bar
    - Load-displacement monotonicity curve
    - Clearance vs loaded rollers
을 시각화.

산출물:
    reports/phase4/fig1_qj_polar.png             — Q_j polar (18 roller)
    reports/phase4/fig2_sjovall_comparison.png    — Sjovall vs Solver
    reports/phase4/fig3_load_displacement.png     — F_r → δ_y 응답
    reports/phase4/fig4_clearance_effect.png      — clearance vs loaded
    reports/phase4/results.json
"""

from __future__ import annotations
import json
import math
from pathlib import Path
import matplotlib.pyplot as plt
import matplotlib as mpl
import numpy as np

mpl.rcParams["font.family"] = "Malgun Gothic"
mpl.rcParams["axes.unicode_minus"] = False

ROOT = Path(__file__).parent.parent
OUT_DIR = ROOT / "reports" / "phase4"
OUT_DIR.mkdir(parents=True, exist_ok=True)

# NU 240 (Phase 2~4)
Z = 18
D_PW = 280.0
D_WE = 44.0
L_WE = 42.0

# ─── Rust 실측 결과 (하드코딩) ────────────────────────────────────

# Smoke test pure_gravity F_y=-1000 kN, g_r=30 μm
SMOKE_DELTA_UM = [-0.000, -248.747, 0.0, 0.0, 0.0]   # [δx, δy, δz, γx, γy]
SMOKE_QMAX_KN = 232.7
# Loaded 9/18 rollers, load angle = -90°

# Level D: Sjovall (zero clearance F_y=-1000 kN)
SJOVALL_QMAX_SOLVER_KN = 228.34
SJOVALL_QMAX_THEORY_KN = 226.48
SJOVALL_REL_ERR = 0.008

# Monotonicity (g_r = 30 μm)
MONOTONICITY = [
    (-100.0, -48.163),
    (-500.0, -143.936),
    (-1000.0, -248.747),
    (-2000.0, -438.094),
]

# Clearance vs loaded (F_y = -1000 kN)
CLEARANCE_LOADED = [
    (0.0, 9),
    (30.0, 9),
    (100.0, 0),   # ⚠️ initial guess 문제로 loaded=0 (실제 물리에선 접촉해야)
    (300.0, 0),
]

# Q_j approximation (Sjovall distribution, zero clearance)
def q_j_sjovall(psi_rad, q_max, epsilon=0.5):
    """Q_j = Q_max · max(0, 1 - (1 − cos(ψ − ψ_load)) / (2·ε))^(10/9)  (line contact)"""
    psi_load = -math.pi / 2  # F_y<0 → load at ψ=-90°
    cos_diff = math.cos(psi_rad - psi_load)
    factor = 1.0 - (1.0 - cos_diff) / (2.0 * epsilon)
    if factor <= 0:
        return 0.0
    return q_max * factor**(10.0 / 9.0)


def plot_qj_polar(out_path):
    """18 roller Q_j polar plot (zero clearance case)."""
    q_max = SJOVALL_QMAX_SOLVER_KN
    load_angle = -math.pi / 2
    psi = [load_angle + 2 * math.pi * j / Z for j in range(Z)]
    q_j = [q_j_sjovall(p, q_max, 0.5) for p in psi]

    fig = plt.figure(figsize=(10, 10))
    ax = fig.add_subplot(111, projection="polar")

    # Q_j bars
    width = 2 * math.pi / Z * 0.8
    colors = plt.cm.viridis([q / q_max if q_max > 0 else 0 for q in q_j])
    ax.bar(psi, q_j, width=width, color=colors, edgecolor="black", linewidth=0.5)

    # Load direction 화살표
    ax.annotate("", xy=(load_angle, q_max * 0.5), xytext=(load_angle, 0),
                arrowprops=dict(arrowstyle="->", color="red", lw=2))
    ax.text(load_angle, q_max * 0.55, f"F_y = -1000 kN", ha="center", color="red", fontsize=10)

    ax.set_theta_zero_location("E")  # ψ=0 = right (X-axis)
    ax.set_theta_direction(1)
    ax.set_title(f"Phase 4 Level D — Q_j Polar Distribution\n"
                 f"NU 240 (Z={Z}), Zero clearance, F_y=-1000 kN, Q_max={q_max:.1f} kN",
                 pad=20)
    ax.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(out_path, dpi=240)
    plt.close()
    print(f"  ✅ {out_path.name}")


def plot_sjovall_comparison(out_path):
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))

    # Bar: solver vs theory
    ax1.bar([0, 1], [SJOVALL_QMAX_SOLVER_KN, SJOVALL_QMAX_THEORY_KN],
            color=["#1f77b4", "#ff7f0e"], edgecolor="black")
    ax1.set_xticks([0, 1])
    ax1.set_xticklabels(["Rust Solver\n(3-DOF NR)", "Sjovall Theory\n(J_r(0.5)=0.2453)"])
    ax1.set_ylabel("Q_max [kN]")
    ax1.set_title(f"Zero clearance NU 240 (F_y=-1000 kN, Z={Z})")
    for i, v in enumerate([SJOVALL_QMAX_SOLVER_KN, SJOVALL_QMAX_THEORY_KN]):
        ax1.text(i, v + 5, f"{v:.2f}", ha="center", fontsize=11, fontweight="bold")
    ax1.grid(True, alpha=0.3, axis="y")

    # Relative error
    ax2.bar([0], [SJOVALL_REL_ERR * 100], color="#2ca02c", edgecolor="black")
    ax2.axhline(5.0, color="red", linestyle="--", label="Level D 통과 기준 5%")
    ax2.set_ylabel("상대 오차 [%]")
    ax2.set_title(f"Level D — 상대오차 {SJOVALL_REL_ERR*100:.2f}% (통과)")
    ax2.set_xticks([0]); ax2.set_xticklabels(["Solver vs Sjovall"])
    ax2.set_ylim(0, 6)
    ax2.text(0, SJOVALL_REL_ERR * 100 + 0.2, f"{SJOVALL_REL_ERR*100:.2f}%",
             ha="center", fontsize=11, fontweight="bold")
    ax2.grid(True, alpha=0.3, axis="y")
    ax2.legend()

    plt.suptitle("Phase 4 Level D — Sjovall (Harris & Kotzalas Ch 7) 비교", y=1.02, fontsize=13)
    plt.tight_layout()
    plt.savefig(out_path, dpi=240, bbox_inches="tight")
    plt.close()
    print(f"  ✅ {out_path.name}")


def plot_load_displacement(out_path):
    fig, ax = plt.subplots(figsize=(11, 5))
    f_y = [abs(f) for f, _ in MONOTONICITY]
    dy = [-d for _, d in MONOTONICITY]

    ax.plot(f_y, dy, "o-", color="#1f77b4", markersize=10, linewidth=2, label="Solver")

    # Palmgren 이론곡선 근사
    f_range = np.linspace(50, 2500, 100)
    # δ ∝ F^(9/10) (Palmgren line contact)
    ref_dy = [dy[2] * (f / 1000.0)**0.9 for f in f_range]  # F_r=1000 를 기준으로 스케일
    ax.plot(f_range, ref_dy, "--", color="gray", alpha=0.7, label="Palmgren δ ∝ F^0.9 (참조)")

    for f, d in zip(f_y, dy):
        ax.annotate(f"F={f}\nδ={d:.1f}", xy=(f, d), xytext=(10, -15),
                    textcoords="offset points", fontsize=9)

    ax.set_xlabel("|F_y| [kN]")
    ax.set_ylabel("|δ_y| [μm]")
    ax.set_title("Phase 4 Level D — 하중-변위 monotonicity (NU 240, g_r=30 μm)")
    ax.grid(True, alpha=0.3)
    ax.legend()
    plt.tight_layout()
    plt.savefig(out_path, dpi=240)
    plt.close()
    print(f"  ✅ {out_path.name}")


def plot_clearance_effect(out_path):
    fig, ax = plt.subplots(figsize=(11, 5))
    g_r = [g for g, _ in CLEARANCE_LOADED]
    loaded = [n for _, n in CLEARANCE_LOADED]

    bars = ax.bar([str(g) for g in g_r], loaded, color=["#2ca02c" if n > 0 else "#d62728" for n in loaded],
                  edgecolor="black")
    ax.axhline(9, color="blue", linestyle="--", alpha=0.5, label="Z/2 = 9 (zero clearance 이론)")
    ax.set_xlabel("Radial clearance g_r [μm]")
    ax.set_ylabel("Loaded rollers")
    ax.set_ylim(0, 18)
    ax.set_title("Phase 4 Level D — Clearance vs Loaded Rollers (F_y=-1000 kN)")
    ax.grid(True, alpha=0.3, axis="y")

    for bar, n in zip(bars, loaded):
        ax.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.3,
                f"{n}/18", ha="center", fontsize=10, fontweight="bold")

    ax.text(0.98, 0.95, "⚠️ g_r ≥ 100μm 에서 loaded=0 은 initial guess 개선 필요\n(Phase 4 후속 개선 대상)",
            transform=ax.transAxes, fontsize=9, va="top", ha="right",
            bbox=dict(facecolor="#fff8dc", edgecolor="orange"))
    ax.legend()
    plt.tight_layout()
    plt.savefig(out_path, dpi=240)
    plt.close()
    print(f"  ✅ {out_path.name}")


def main():
    print("=" * 70)
    print("CRB Phase 4 — Level D 리포트 생성")
    print("=" * 70)

    print("\n[1] Rust cargo test 실측 결과:")
    print(f"  Smoke 5/5 pass")
    print(f"  Level D 5/5 pass")
    print(f"  Zero clearance Q_max: Solver={SJOVALL_QMAX_SOLVER_KN} kN, "
          f"Theory={SJOVALL_QMAX_THEORY_KN} kN, rel_err={SJOVALL_REL_ERR*100:.2f}%")
    print(f"  Monotonicity: F=100→2000 kN 에서 δ_y = -48→-438 μm (단조)")
    print(f"  Load zone: g_r=0 → 9/18 loaded (Z/2)")

    results = {
        "smoke_pure_gravity": {
            "delta": SMOKE_DELTA_UM,
            "q_max_kn": SMOKE_QMAX_KN,
        },
        "level_d_sjovall": {
            "q_max_solver_kn": SJOVALL_QMAX_SOLVER_KN,
            "q_max_theory_kn": SJOVALL_QMAX_THEORY_KN,
            "rel_err": SJOVALL_REL_ERR,
        },
        "monotonicity": [{"f_y_kn": f, "delta_y_um": d} for f, d in MONOTONICITY],
        "clearance_effect": [{"g_r_um": g, "loaded": n} for g, n in CLEARANCE_LOADED],
    }
    with open(OUT_DIR / "results.json", "w", encoding="utf-8") as f:
        json.dump(results, f, ensure_ascii=False, indent=2)
    print("\n[2] JSON 저장: results.json")

    print("\n[3] PNG 시각화 생성:")
    plot_qj_polar(OUT_DIR / "fig1_qj_polar.png")
    plot_sjovall_comparison(OUT_DIR / "fig2_sjovall_comparison.png")
    plot_load_displacement(OUT_DIR / "fig3_load_displacement.png")
    plot_clearance_effect(OUT_DIR / "fig4_clearance_effect.png")

    print(f"\n산출물 위치: {OUT_DIR}")
    print("=" * 70)


if __name__ == "__main__":
    main()
