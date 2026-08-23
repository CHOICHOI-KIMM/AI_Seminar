// Level D-2 검증 — 5-DOF 해방 (Plan Phase 3-2)
//
// 판정 대상: BB_Development_Theory.md §4.4 확정 형태를 `BbDofMask::FULL` 로 푼 해.
// 실행: cargo test --test equilibrium_level_d2
//
// ── 이 단계가 무엇을 검증하는가 ──────────────────────────────────────
// P3-1 은 자유도를 묶어 두고 **문헌(Harris Table 7.4)** 과 맞춰 보았다.
// 여기서는 묶은 것을 풀고, 5-DOF 해가
//   ① 구속 해를 **부분집합으로 포함**하는지 (축퇴 항등성)
//   ② 베어링의 **축대칭성**을 깨뜨리지 않는지 (반경·모멘트 방향 불변)
//   ③ 2축 모멘트에서 **틸트가 모멘트와 같은 축**으로 서는지
//   ④ 넓은 하중 격자에서 **수렴**하는지
// 를 본다. 외부 문헌 대조는 D-1 이 끝이며, 여기는 구조적 성질 검증이다.

use bb_core::solver::bb::bearing::solve_bearing;
use bb_core::solver::bb::types::*;
use bb_core::solver::common::types::*;

const D_W: f64 = 11.5;
const D_PW: f64 = 70.0;
const ALPHA_DEG: f64 = 40.0;
const Z: u32 = 16;

fn geometry_at(z: u32, alpha_deg: f64) -> BallBearingGeometry {
    let (r_i_mm, r_e_mm) = BallBearingGeometry::reference_groove_radii(D_W);
    BallBearingGeometry {
        bore_mm: 50.0,
        outer_diameter_mm: 90.0,
        width_mm: 20.0,
        z,
        d_w_mm: D_W,
        d_pw_mm: D_PW,
        r_i_mm,
        r_e_mm,
        alpha_nom_rad: alpha_deg.to_radians(),
        // 클리어런스 0 — 초기 접촉각 = 공칭 접촉각
        clearance: BbClearanceSpec::InitialAngleRad(alpha_deg.to_radians()),
    }
}

fn make_z(z: u32, fx: f64, fy: f64, fz: f64, my: f64, mz: f64) -> BbInput {
    BbInput {
        geometry: geometry_at(z, ALPHA_DEG),
        material: Material::default(),
        operating: BbOperatingConditions {
            f_x_n: fx,
            f_y_n: fy,
            f_z_n: fz,
            m_y_nmm: my,
            m_z_nmm: mz,
            n_inner_rpm: 1000.0,
            n_outer_rpm: 0.0,
            temperature_c: 70.0,
        },
        solver: BbSolverParams {
            convergence_tol: 1e-13,
            ..BbSolverParams::default()
        },
    }
}

fn make(fx: f64, fy: f64, fz: f64, my: f64, mz: f64) -> BbInput {
    make_z(Z, fx, fy, fz, my, mz)
}

/// 두 각의 최단 차이 [-π, π)
fn angle_diff(a: f64, b: f64) -> f64 {
    let mut d = a - b;
    while d >= std::f64::consts::PI {
        d -= std::f64::consts::TAU;
    }
    while d < -std::f64::consts::PI {
        d += std::f64::consts::TAU;
    }
    d
}

fn rel(a: f64, b: f64) -> f64 {
    let s = a.abs().max(b.abs()).max(1e-30);
    (a - b).abs() / s
}

// ═══════════════════════════════════════════════════════════════════
//  D-2a. 축퇴 항등성  ★ 이 단계의 핵심
// ═══════════════════════════════════════════════════════════════════

/// `ISO_3DOF`(δ_z = γ_y = 0 구속) 해가 `FULL` 의 해이기도 한가.
///
/// 하중을 `F_x`·`F_y`·`M_z` 로 한정하면 하중계가 x–y 평면에 대해 대칭이므로
/// (φ → −φ 에서 sin φ 만 부호가 바뀐다) `F_z` 와 `M_y` 잔차가 **항등적으로 0** 이다.
/// 따라서 구속을 풀어도 해가 움직이면 안 된다. 움직인다면 5-DOF 확장이
/// 3-DOF 를 부분집합으로 포함하지 않는다는 뜻이다.
#[test]
fn d2a_iso3dof_solution_is_a_full_solution() {
    println!("\n── Level D-2a : 축퇴 항등성 (ISO_3DOF ⊂ FULL) ──");
    println!("   F_x     F_y      M_z      |δ_z|      |γ_y|     max rel.err");
    for &(fx, fy, mz) in &[
        (3000.0, 0.0, 0.0),
        (3000.0, 5000.0, 0.0),
        (3000.0, 5000.0, 2.0e5),
        (500.0, 8000.0, -1.0e5),
        (12000.0, 2000.0, 4.0e5),
    ] {
        let mut a = make(fx, fy, 0.0, 0.0, mz);
        a.solver.dof_mask = BbDofMask::ISO_3DOF;
        let mut b = a.clone();
        b.solver.dof_mask = BbDofMask::FULL;

        let ra = solve_bearing(&a).unwrap();
        let rb = solve_bearing(&b).unwrap();
        assert!(ra.equilibrium.converged && rb.equilibrium.converged, "미수렴");

        // 해방된 자유도가 실제로 0 에 머무는가 (대칭이 요구하는 값)
        let dz = rb.equilibrium.displacement[2];
        let gy = rb.equilibrium.displacement[3];
        assert!(dz.abs() < 1e-12, "δ_z 가 0 이 아님: {dz:.3e} mm");
        assert!(gy.abs() < 1e-12, "γ_y 가 0 이 아님: {gy:.3e} rad");

        // 나머지 3 자유도가 일치하는가.
        // ⚠ **성분별** 상대오차를 쓰면 안 된다 — 순수 축하중에서는 δ_y·γ_z 가
        //   정확히 0 이라 1e-18 대 1e-19 를 비교하는 꼴이 되어 무의미한 O(1) 이 나온다.
        //   해 벡터 전체의 크기를 분모로 삼는다 (병진 [mm], 회전은 R_i 를 곱해 길이 차원).
        let r_i = ra.geometry.r_i_center_mm;
        let scale_u = (0..5)
            .map(|i| {
                let v = ra.equilibrium.displacement[i].abs();
                if i >= 3 { v * r_i } else { v }
            })
            .fold(0.0_f64, f64::max)
            .max(1e-12);
        let mut worst = 0.0_f64;
        for i in [0usize, 1, 4] {
            let d = (ra.equilibrium.displacement[i] - rb.equilibrium.displacement[i]).abs();
            let d = if i >= 3 { d * r_i } else { d };
            worst = worst.max(d / scale_u);
        }
        // 볼 하중까지 일치하는가 — 변위 일치의 결과가 아니라 독립 확인.
        // 비접촉 볼은 Q = 0 이므로 여기서도 Q_max 를 공통 분모로 쓴다.
        let q_scale = ra.equilibrium.q_max_n.max(1e-12);
        for (x, y) in ra
            .equilibrium
            .ball_results
            .iter()
            .zip(&rb.equilibrium.ball_results)
        {
            worst = worst.max((x.q_n - y.q_n).abs() / q_scale);
            worst = worst.max((x.alpha_rad - y.alpha_rad).abs());
        }
        println!(
            "  {fx:7.0} {fy:7.0} {mz:9.2e}  {:.2e}  {:.2e}   {worst:.2e}",
            dz.abs(),
            gy.abs()
        );
        assert!(worst < 1e-8, "ISO_3DOF ↔ FULL 상대오차 {worst:.3e} 가 과대");
    }
    println!("  → 판정 rel. err < 1e-8 통과");
}

// ═══════════════════════════════════════════════════════════════════
//  D-2b. 반경하중 방향 불변
// ═══════════════════════════════════════════════════════════════════

/// 베어링은 회전축에 대해 축대칭이므로, 크기가 같은 반경하중은 **방향에 관계없이**
/// 같은 최악 `Q_max` 를 내야 한다. 볼이 이산 배치라 고정 원점에서는 성립하지 않으므로
/// **위상 스윕의 최악값**으로 비교한다 (D-8).
#[test]
fn d2b_radial_load_direction_invariance() {
    println!("\n── Level D-2b : 반경하중 방향 불변 ──");
    const F_R: f64 = 8000.0;
    let sweep = BbPhaseSweep {
        enabled: true,
        n_phase: 180,
    };
    let mut base: Option<f64> = None;
    let mut worst = 0.0_f64;
    for deg in [0.0_f64, 30.0, 45.0, 90.0, 137.0, 180.0, 271.0] {
        let th = deg.to_radians();
        let mut inp = make(3000.0, F_R * th.cos(), F_R * th.sin(), 0.0, 0.0);
        inp.solver.phase_sweep = sweep;
        let r = solve_bearing(&inp).unwrap();
        let q = r.phase_sweep.as_ref().unwrap().worst_q_max_n;
        let e = match base {
            None => {
                base = Some(q);
                0.0
            }
            Some(b) => rel(q, b),
        };
        worst = worst.max(e);
        println!("  방향 {deg:6.1}°   최악 Q_max = {q:10.3} N   rel {e:.2e}");
    }
    println!("  → 최대 상대편차 {worst:.2e}");
    assert!(worst < 1e-8, "반경하중 방향 불변 위배 {worst:.3e}");
}

// ═══════════════════════════════════════════════════════════════════
//  D-2c. 모멘트 축 불변
// ═══════════════════════════════════════════════════════════════════

/// 모멘트 벡터 `(M_y, M_z)` 도 축대칭성에 따라 **크기만 결과를 지배**해야 한다.
#[test]
fn d2c_moment_axis_invariance() {
    println!("\n── Level D-2c : 모멘트 축 불변 ──");
    const M: f64 = 3.0e5;
    let sweep = BbPhaseSweep {
        enabled: true,
        n_phase: 180,
    };
    let mut base: Option<f64> = None;
    let mut worst = 0.0_f64;
    for deg in [0.0_f64, 45.0, 90.0, 123.0, 180.0, 315.0] {
        let th = deg.to_radians();
        let mut inp = make(4000.0, 0.0, 0.0, M * th.cos(), M * th.sin());
        inp.solver.phase_sweep = sweep;
        let r = solve_bearing(&inp).unwrap();
        let q = r.phase_sweep.as_ref().unwrap().worst_q_max_n;
        let e = match base {
            None => {
                base = Some(q);
                0.0
            }
            Some(b) => rel(q, b),
        };
        worst = worst.max(e);
        println!("  모멘트축 {deg:6.1}°   최악 Q_max = {q:10.3} N   rel {e:.2e}");
    }
    println!("  → 최대 상대편차 {worst:.2e}");
    assert!(worst < 1e-8, "모멘트 축 불변 위배 {worst:.3e}");
}

// ═══════════════════════════════════════════════════════════════════
//  D-2d. 2축 모멘트 — 틸트·하중분포가 합성 모멘트 축과 정합
// ═══════════════════════════════════════════════════════════════════

/// 등방(축대칭) 계에서는 틸트 벡터 `(γ_y, γ_z)` 가 인가 모멘트 벡터와 **같은 축**에
/// 서야 하고, 최대하중 볼도 그 축을 따라 함께 회전해야 한다.
/// 두 축 사이 교차강성이 있으면 여기서 깨진다.
///
/// 볼 이산화가 등방성을 미세하게 깨므로 `Z = 200` 으로 방위 분해능을 올린다.
#[test]
fn d2d_biaxial_moment_aligns_with_tilt_axis() {
    println!("\n── Level D-2d : 2축 모멘트 ↔ 틸트축 정합 (Z = 200) ──");
    const ZF: u32 = 200;
    const M: f64 = 3.0e5;
    let pitch = std::f64::consts::TAU / ZF as f64;

    let mut ref_off: Option<f64> = None;
    let mut ref_peak: Option<f64> = None;
    let mut ref_mag: Option<f64> = None;
    let mut worst_off = 0.0_f64;
    let mut worst_peak = 0.0_f64;
    let mut worst_mag = 0.0_f64;

    println!("   모멘트축   |γ|          틸트축-모멘트축   최대하중볼 회전오차");
    for k in 0..8 {
        let th = std::f64::consts::TAU * k as f64 / 8.0;
        let inp = make_z(ZF, 4000.0, 0.0, 0.0, M * th.cos(), M * th.sin());
        let r = solve_bearing(&inp).unwrap();
        assert!(r.equilibrium.converged, "미수렴");

        let gy = r.equilibrium.displacement[3];
        let gz = r.equilibrium.displacement[4];
        let mag = gy.hypot(gz);
        let off = angle_diff(gz.atan2(gy), th); // 틸트축과 모멘트축의 차이

        // 최대하중 볼의 방위
        let peak = r
            .equilibrium
            .ball_results
            .iter()
            .max_by(|a, b| a.q_n.partial_cmp(&b.q_n).unwrap())
            .unwrap()
            .phi_rad;

        let (e_off, e_peak, e_mag) = match (ref_off, ref_peak, ref_mag) {
            (Some(o), Some(p), Some(m)) => (
                angle_diff(off, o).abs(),
                angle_diff(angle_diff(peak, p), th).abs(),
                rel(mag, m),
            ),
            _ => {
                ref_off = Some(off);
                ref_peak = Some(peak);
                ref_mag = Some(mag);
                (0.0, 0.0, 0.0)
            }
        };
        worst_off = worst_off.max(e_off);
        worst_peak = worst_peak.max(e_peak);
        worst_mag = worst_mag.max(e_mag);
        println!(
            "  {:7.1}°  {mag:.6e}   {:+8.4}°          {:7.4}°",
            th.to_degrees(),
            off.to_degrees(),
            e_peak.to_degrees()
        );
    }
    println!(
        "  → 틸트축 편차 {:.4}° · 크기 편차 {worst_mag:.2e} · 볼 회전오차 {:.4}° (볼 피치 {:.3}°)",
        worst_off.to_degrees(),
        worst_peak.to_degrees(),
        pitch.to_degrees()
    );
    // 틸트축은 이산화와 무관하게 성립해야 한다 (하중계 자체의 회전 대칭)
    assert!(
        worst_off < 1e-6,
        "틸트축이 모멘트축을 따라가지 않음: {worst_off:.3e} rad"
    );
    assert!(worst_mag < 1e-6, "틸트 크기 비등방 {worst_mag:.3e}");
    // 최대하중 볼은 이산 위치이므로 반 피치 이내면 정합
    assert!(
        worst_peak <= pitch * 0.5 + 1e-9,
        "최대하중 볼 방위가 모멘트축을 따라가지 않음: {:.4}° > 반피치 {:.4}°",
        worst_peak.to_degrees(),
        (pitch * 0.5).to_degrees()
    );
}

// ═══════════════════════════════════════════════════════════════════
//  D-2e. 수렴 강건성 — 물리적 유효 격자
// ═══════════════════════════════════════════════════════════════════

/// 하중·접촉각 격자 전수 스윕에서 **수렴 실패율 0 %** 를 요구한다.
///
/// ⚠ 격자를 「해가 존재해야 하는 조합」으로 한정한다 — 모든 점에 **`F_x > 0` 과
///   클리어런스 0** 을 두어 최소 한 개 볼의 접촉을 기하학적으로 보장한다.
///   무하중 + 양의 클리어런스처럼 **접촉 볼이 0 개인 조합은 오류가 아니라 자명해**이며
///   (Level C-8b 에서 확인), 그런 점까지 넣고 「실패 0 %」를 요구하면
///   물리적으로 없는 해를 억지로 만들라는 요구가 된다.
#[test]
fn d2e_convergence_robustness_grid() {
    println!("\n── Level D-2e : 수렴 강건성 격자 스윕 ──");
    let alphas = [15.0, 25.0, 40.0];
    let fxs = [200.0, 1000.0, 5000.0, 20000.0];
    let fys = [0.0, 1000.0, 5000.0, 20000.0];
    let mzs = [0.0, 5.0e4, 3.0e5];

    let mut total = 0u32;
    let mut fail: Vec<String> = Vec::new();
    let mut worst_iter = 0u32;
    let mut worst_res = 0.0_f64;

    for &al in &alphas {
        for &fx in &fxs {
            for &fy in &fys {
                for &mz in &mzs {
                    total += 1;
                    let mut inp = make(fx, fy, 0.0, 0.0, mz);
                    inp.geometry = geometry_at(Z, al);
                    match solve_bearing(&inp) {
                        Ok(r) => {
                            if r.equilibrium.converged {
                                worst_iter = worst_iter.max(r.equilibrium.iterations);
                                worst_res = worst_res.max(r.equilibrium.residual_norm);
                                assert!(
                                    r.equilibrium.loaded_count >= 1,
                                    "α₀={al}° F_x={fx} F_y={fy}: 접촉 볼 0"
                                );
                            } else {
                                fail.push(format!(
                                    "α₀={al}° F_x={fx} F_y={fy} M_z={mz:.1e} → 미수렴 \
                                     (반복 {}, 잔차 {:.2e})",
                                    r.equilibrium.iterations, r.equilibrium.residual_norm
                                ));
                            }
                        }
                        Err(e) => fail.push(format!(
                            "α₀={al}° F_x={fx} F_y={fy} M_z={mz:.1e} → 오류: {e}"
                        )),
                    }
                }
            }
        }
    }
    println!("  격자 {total} 점 · 실패 {}", fail.len());
    println!("  최대 반복 {worst_iter} 회 · 최대 상대잔차 {worst_res:.2e} (tol 1e-13)");
    for f in &fail {
        println!("  ✗ {f}");
    }
    assert!(
        fail.is_empty(),
        "물리적 유효 격자에서 수렴 실패 {}건",
        fail.len()
    );
}

// ═══════════════════════════════════════════════════════════════════
//  D-2f. 고속 경고 경계 동작
// ═══════════════════════════════════════════════════════════════════

/// `n·D_pw` 가 ISO 16281 A.4 의 정적 가정 한계 1e6 mm/min 을 **넘을 때만** 경고가 난다.
/// 경계 양쪽에서 확인해 「항상 켜짐 / 항상 꺼짐」을 배제한다.
#[test]
fn d2f_high_speed_alert_boundary() {
    println!("\n── Level D-2f : 고속 경고 경계 ──");
    // n·D_pw = 1e6 이 되는 회전수
    let n_crit = 1.0e6 / D_PW;
    for (label, n, expect) in [
        ("한계 −10 %", n_crit * 0.9, false),
        ("한계 −0,1 %", n_crit * 0.999, false),
        ("한계 +0,1 %", n_crit * 1.001, true),
        ("한계 ×2", n_crit * 2.0, true),
    ] {
        let mut inp = make(3000.0, 2000.0, 0.0, 0.0, 0.0);
        inp.operating.n_inner_rpm = n;
        let r = solve_bearing(&inp).unwrap();
        let got = r.alerts.iter().any(|a| a.code == "HIGH_SPEED");
        println!(
            "  {label:12}  n = {n:9.1} rpm  → n·D_pw = {:.4e}  경고 {got}",
            n * D_PW
        );
        assert_eq!(
            got, expect,
            "{label}: HIGH_SPEED 경고 기대 {expect}, 실제 {got}"
        );
    }
    println!("  → 경계 동작 정상");
}
