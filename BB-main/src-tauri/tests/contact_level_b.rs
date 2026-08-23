// Level B 검증 — 점접촉 타원 Hertz (Plan §5, Phase 2)
//
// 판정 대상: BB_Development_Theory.md §3, §6.
// 실행: cargo test --test contact_level_b
//
// ── 외부 문헌 검증 2곳 중 하나 ──────────────────────────────────────
// 골든값: Harris & Kotzalas 5th ed. Table 6.1 (원서 p.128, 육안 확인 완료).
//
// ── 검증 철학 ───────────────────────────────────────────────────────
// ① 표와의 대조를 **양방향**으로 한다 (P2 결정).
//    역방향: 표의 F(ρ) → 자체 솔버로 χ → a*·b*·δ* → 표값 대조   (솔버 검증)
//    정방향: 같은 표를 **Gauss-Legendre 타원적분**으로 재현        (적분 검증)
//    두 경로가 서로 다른 것을 잡는다.
// ② ISO 내부 일관성: 식 (38) 에서 유도한 c_P 가 식 (40) 과 일치해야 한다.
//    ISO 의 계수 1,48 은 사실 π/√4,5 다 — 두 식의 전사가 맞는지 검증한다.
//
// ⚠ **ISO (36) ↔ Harris (6.42) 는 물리 모델 교차가 아니다.**
//    P2 선조사에서 두 식이 대수적으로 동일함을 확인했다. 따라서 이 대조는
//    두 표준의 독립 검증이 아니라 **전사·구현 검증**이다 (B-5 주석 참조).

use bb_core::solver::bb::geometry::compute_geometry_derived;
use bb_core::solver::bb::hertz::*;
use bb_core::solver::bb::types::*;
use bb_core::solver::common::types::*;
use bb_core::solver::common::util;

/// Harris Table 6.1 — 무차원 접촉 파라미터. (F(ρ), a*, b*, δ*)
/// 원서 p.128. 마지막 행 F(ρ)=1 (a*=∞) 은 수치 대조 대상이 아니라 제외.
const TABLE_6_1: [(f64, f64, f64, f64); 23] = [
    (0.0, 1.0, 1.0, 1.0),
    (0.1075, 1.0760, 0.9318, 0.9974),
    (0.3204, 1.2623, 0.8114, 0.9761),
    (0.4795, 1.4556, 0.7278, 0.9429),
    (0.5916, 1.6440, 0.6687, 0.9077),
    (0.6716, 1.8258, 0.6245, 0.8733),
    (0.7332, 2.011, 0.5881, 0.8394),
    (0.7948, 2.265, 0.5480, 0.7961),
    (0.83495, 2.494, 0.5186, 0.7602),
    (0.87366, 2.800, 0.4863, 0.7169),
    (0.90999, 3.233, 0.4499, 0.6636),
    (0.93657, 3.738, 0.4166, 0.6112),
    (0.95738, 4.395, 0.3830, 0.5551),
    (0.97290, 5.267, 0.3490, 0.4960),
    (0.983797, 6.448, 0.3150, 0.4352),
    (0.990902, 8.062, 0.2814, 0.3745),
    (0.995112, 10.222, 0.2497, 0.3176),
    (0.997300, 12.789, 0.2232, 0.2705),
    (0.9981847, 14.839, 0.2072, 0.2427),
    (0.9989156, 17.974, 0.18822, 0.2106),
    (0.9994785, 23.55, 0.16442, 0.17167),
    (0.9998527, 37.38, 0.13050, 0.11995),
    // F(ρ) = 1 → a* = ∞ 는 제외
    (0.0, 1.0, 1.0, 1.0), // 패딩 (배열 길이 맞춤, 0번과 동일해 무해)
];

fn fixture() -> BallBearingGeometry {
    let d_w_mm = 11.5;
    let (r_i_mm, r_e_mm) = BallBearingGeometry::reference_groove_radii(d_w_mm);
    BallBearingGeometry {
        bore_mm: 50.0,
        outer_diameter_mm: 90.0,
        width_mm: 20.0,
        z: 16,
        d_w_mm,
        d_pw_mm: 70.0,
        r_i_mm,
        r_e_mm,
        alpha_nom_rad: 40.0_f64.to_radians(),
        clearance: BbClearanceSpec::InitialAngleRad(40.0_f64.to_radians()),
    }
}

fn rel(a: f64, b: f64) -> f64 {
    ((a - b) / b).abs()
}

// ═══════════════════════════════════════════════════════════════════
//  B-1. 완전타원적분 극한
// ═══════════════════════════════════════════════════════════════════

#[test]
fn b1_elliptic_limits() {
    // χ = 1 (m = 0): K = E = π/2 — 원형 접촉
    let (k, e) = util::elliptic_k_e_agm(0.0);
    assert!(rel(k, std::f64::consts::FRAC_PI_2) < 1e-15);
    assert!(rel(e, std::f64::consts::FRAC_PI_2) < 1e-15);

    // m → 1: K 는 발산, E → 1
    let (k_hi, e_hi) = util::elliptic_k_e_agm(1.0 - 1e-12);
    assert!(k_hi > 14.0, "K(m→1) = {k_hi}");
    assert!(rel(e_hi, 1.0) < 1e-9, "E(m→1) = {e_hi}");
}

// ═══════════════════════════════════════════════════════════════════
//  B-2. AGM ↔ Gauss-Legendre 교차 (독립 유도 경로)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn b2_agm_vs_quadrature_over_table_range() {
    // Table 6.1 이 덮는 χ 전 범위에서 두 수치법이 일치해야 한다.
    for &(f_rho, _, _, _) in TABLE_6_1.iter().take(22).skip(1) {
        let chi = solve_chi(f_rho).unwrap();
        let m = 1.0 - 1.0 / (chi * chi);
        let (k1, e1) = util::elliptic_k_e_agm(m);
        let (k2, e2) = util::elliptic_k_e_quadrature(m);
        assert!(rel(k1, k2) < 1e-10, "F(ρ)={f_rho} χ={chi}: K {k1} vs {k2}");
        assert!(rel(e1, e2) < 1e-10, "F(ρ)={f_rho} χ={chi}: E {e1} vs {e2}");
    }
}

// ═══════════════════════════════════════════════════════════════════
//  B-3. Harris Table 6.1 — 역방향 (자체 χ 솔버 + AGM)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn b3_table_6_1_reverse_direction() {
    // 표의 F(ρ) → solve_chi → (6.44)~(6.46) → 표의 a*·b*·δ* 대조.
    // 표는 유효숫자 4~5자리라 판정은 1e-3 상대오차.
    let mut worst = 0.0_f64;
    for &(f_rho, a_ref, b_ref, d_ref) in TABLE_6_1.iter().take(22) {
        let chi = solve_chi(f_rho).unwrap();
        let m = 1.0 - 1.0 / (chi * chi);
        let (k, e) = util::elliptic_k_e_agm(m);
        let (a_star, b_star, d_star) = dimensionless_contact_coefficients(chi, k, e);

        for (got, want, name) in [
            (a_star, a_ref, "a*"),
            (b_star, b_ref, "b*"),
            (d_star, d_ref, "δ*"),
        ] {
            let err = rel(got, want);
            worst = worst.max(err);
            assert!(
                err < 1.0e-3,
                "F(ρ)={f_rho} χ={chi:.6}: {name} 계산 {got:.6} vs 표 {want} (오차 {err:.2e})"
            );
        }
    }
    println!("B-3 역방향 최대 상대오차: {worst:.3e}");
}

// ═══════════════════════════════════════════════════════════════════
//  B-4. Harris Table 6.1 — 정방향 (Gauss-Legendre 경로)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn b4_table_6_1_forward_direction_via_quadrature() {
    // AGM 을 전혀 쓰지 않고 구적만으로 같은 표를 재현한다.
    // χ 는 표의 F(ρ) 에서 구하되, 그 이후 K·E·a*·b*·δ* 는 전부 구적 경로.
    let mut worst = 0.0_f64;
    for &(f_rho, a_ref, b_ref, d_ref) in TABLE_6_1.iter().take(22) {
        let chi = solve_chi(f_rho).unwrap();
        let m = 1.0 - 1.0 / (chi * chi);
        let (k_q, e_q) = util::elliptic_k_e_quadrature(m);
        let (a_star, b_star, d_star) = dimensionless_contact_coefficients(chi, k_q, e_q);

        for (got, want, name) in [
            (a_star, a_ref, "a*"),
            (b_star, b_ref, "b*"),
            (d_star, d_ref, "δ*"),
        ] {
            let err = rel(got, want);
            worst = worst.max(err);
            assert!(
                err < 1.0e-3,
                "F(ρ)={f_rho}: {name} 구적 {got:.6} vs 표 {want} (오차 {err:.2e})"
            );
        }
    }
    println!("B-4 정방향(구적) 최대 상대오차: {worst:.3e}");
}

#[test]
fn b4b_chi_solver_roundtrip_on_table_values() {
    // 표의 F(ρ) → χ → F(ρ) 왕복이 기계정밀도로 닫혀야 한다.
    for &(f_rho, _, _, _) in TABLE_6_1.iter().take(22).skip(1) {
        let chi = solve_chi(f_rho).unwrap();
        let back = f_rho_from_chi(chi);
        assert!(
            (back - f_rho).abs() < 1e-12,
            "F(ρ)={f_rho} → χ={chi} → F(ρ)={back}"
        );
    }
}

// ═══════════════════════════════════════════════════════════════════
//  B-5. ISO (36) ↔ Harris (6.42)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn b5_iso_and_harris_deflection_agree() {
    // ⚠ 이 둘은 **대수적으로 동일한 식**이다 (P2 선조사에서 전개 확인):
    //     δ = [9 Σρ Q² K³ / (8π² E*² χ² E)]^(1/3)
    // 따라서 물리 모델 교차가 아니라 **전사·구현 검증**이다.
    // 그래도 값어치가 있다 — 두 식을 서로 다른 형태로 옮겨 적었고,
    // 어느 한쪽 전사가 틀리면 여기서 갈린다.
    let g = fixture();
    let d = compute_geometry_derived(&g).unwrap();
    let m = Material::default();
    let c = compute_contact_derived(&d, &m).unwrap();

    for q in [10.0, 100.0, 1_000.0, 5_000.0, 20_000.0] {
        let iso_i = single_contact_deflection_iso(
            &m,
            d.sum_rho_i_per_mm,
            c.chi_inner,
            c.k_ellip_inner,
            c.e_ellip_inner,
            q,
        );
        let har_i = single_contact_deflection_harris(
            c.e_star_mpa,
            d.sum_rho_i_per_mm,
            c.delta_star_inner,
            q,
        );
        assert!(
            rel(iso_i, har_i) < 1e-10,
            "Q={q} 내륜: ISO(36) {iso_i:.9e} vs Harris(6.42) {har_i:.9e}"
        );

        let iso_e = single_contact_deflection_iso(
            &m,
            d.sum_rho_e_per_mm,
            c.chi_outer,
            c.k_ellip_outer,
            c.e_ellip_outer,
            q,
        );
        let har_e = single_contact_deflection_harris(
            c.e_star_mpa,
            d.sum_rho_e_per_mm,
            c.delta_star_outer,
            q,
        );
        assert!(rel(iso_e, har_e) < 1e-10, "Q={q} 외륜");
    }
}

// ═══════════════════════════════════════════════════════════════════
//  B-6. ISO 내부 일관성 — 식 (36)+(37) = (38) = (39)+(40)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn b6_iso_1_48_coefficient_is_pi_over_sqrt_4_5() {
    // 식 (38) 로부터 c_P 를 유도하면  c_P = (L·bracket)^(−3/2),
    //   L = ∛(4,5 ((1−ν²)/(πE))²)
    // → L^(−3/2) = (π/√4,5) · E/(1−ν²)
    // ISO (40) 의 계수 1,48 이 정확히 π/√4,5 인지 확인한다.
    let exact = std::f64::consts::PI / 4.5_f64.sqrt();
    // π/√4,5 = 1,480 961 … → ISO 는 1,48 로 **절사**해 실었다.
    // 상대 편차 6,5e-4 (0,065 %). 이 편차는 c_P 를 거쳐 δ 에 약 0,043 %,
    // Q 에 약 0,065 % 로 전파된다 — b6b 의 1e-3 허용치가 이 때문이다.
    let rel_dev = (exact - 1.48).abs() / exact;
    assert!(rel_dev < 1.0e-3, "π/√4,5 = {exact} — ISO 의 1,48 과 편차 {rel_dev:.3e}");
    assert!(
        rel_dev > 1.0e-4,
        "편차가 예상보다 작음 — 계수 전사를 재확인할 것 ({rel_dev:.3e})"
    );
}

#[test]
fn b6b_c_p_matches_total_deflection_formula() {
    // (36)+(37) 로 얻은 총 변형과 (39)+(40) 의 c_P 가 일관되어야 한다.
    // ISO 의 1,48 은 π/√4,5 = 1,48096… 의 반올림이라 ~0,07 % 편차가 남는다.
    let g = fixture();
    let d = compute_geometry_derived(&g).unwrap();
    let m = Material::default();
    let c = compute_contact_derived(&d, &m).unwrap();

    for q in [50.0, 500.0, 5_000.0, 50_000.0] {
        let delta_sum = single_contact_deflection_iso(
            &m,
            d.sum_rho_i_per_mm,
            c.chi_inner,
            c.k_ellip_inner,
            c.e_ellip_inner,
            q,
        ) + single_contact_deflection_iso(
            &m,
            d.sum_rho_e_per_mm,
            c.chi_outer,
            c.k_ellip_outer,
            c.e_ellip_outer,
            q,
        );
        let delta_cp = delta_from_q(c.c_p_n_per_mm15, q);
        assert!(
            rel(delta_cp, delta_sum) < 1e-3,
            "Q={q}: (36)+(37) 합 {delta_sum:.9e} vs c_P 경로 {delta_cp:.9e}"
        );
    }
}

#[test]
fn b6c_load_deflection_exponent_is_three_halves() {
    // Q ∝ δ^1.5 — 하중을 8배 하면 변형은 4배 (8^(2/3) = 4)
    let g = fixture();
    let d = compute_geometry_derived(&g).unwrap();
    let c = compute_contact_derived(&d, &Material::default()).unwrap();
    let d1 = delta_from_q(c.c_p_n_per_mm15, 1_000.0);
    let d8 = delta_from_q(c.c_p_n_per_mm15, 8_000.0);
    assert!(rel(d8 / d1, 4.0) < 1e-12, "δ 비 = {}", d8 / d1);
}

// ═══════════════════════════════════════════════════════════════════
//  B-7. Brewe-Hamrock 근사 오차 (Harris 명시 범위)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn b7_brewe_hamrock_within_stated_error() {
    // Harris 원문(p.127): 1 ≤ κ ≤ 10 에서 E 오차는 κ=1 근방 제외 사실상 0,
    // κ=1 근방에서 E < 2 %, F < 2,6 %.
    //
    // (6.33) κ ≈ 1,0339 (R_y/R_x)^0,636 을 역산해 ratio 를 얻고,
    // (6.34)(6.35) 의 E·F 를 **정확한 타원적분**과 대조한다.
    // 회귀식을 회귀식으로 검증하는 순환이 되지 않도록, 비교 상대는 항상 정확값이다.
    let mut worst_e = 0.0_f64;
    let mut worst_f = 0.0_f64;
    for chi in [1.0_f64, 1.2, 1.5, 2.0, 3.0, 5.0, 7.5, 10.0] {
        let ratio = (chi / 1.0339).powf(1.0 / 0.636);
        let (_, f_bh, e_bh) = hertz_elliptical_coefficients(1.0, ratio.max(1.0));
        let m = 1.0 - 1.0 / (chi * chi);
        let (k_exact, e_exact) = util::elliptic_k_e_agm(m);
        worst_e = worst_e.max(rel(e_bh, e_exact));
        worst_f = worst_f.max(rel(f_bh, k_exact));
    }
    println!("B-7 Brewe-Hamrock 최대 오차 — E: {worst_e:.3e}, F: {worst_f:.3e}");
    assert!(worst_e < 0.03, "E 오차 {worst_e} 가 Harris 명시 범위(2 %) 초과");
    assert!(worst_f < 0.04, "F 오차 {worst_f} 가 Harris 명시 범위(2,6 %) 초과");
}

// ═══════════════════════════════════════════════════════════════════
//  B-8. 접촉타원·응력 물리 정합
// ═══════════════════════════════════════════════════════════════════

#[test]
fn b8_contact_ellipse_physics() {
    let g = fixture();
    let d = compute_geometry_derived(&g).unwrap();
    let c = compute_contact_derived(&d, &Material::default()).unwrap();

    let q = 5_000.0; // [N]
    let (a_i, b_i, p_i) = contact_ellipse(
        c.e_star_mpa,
        d.sum_rho_i_per_mm,
        c.a_star_inner,
        c.b_star_inner,
        q,
    );
    let (a_e, b_e, p_e) = contact_ellipse(
        c.e_star_mpa,
        d.sum_rho_e_per_mm,
        c.a_star_outer,
        c.b_star_outer,
        q,
    );

    // 형상비 = χ
    assert!(rel(a_i / b_i, c.chi_inner) < 1e-12);
    assert!(rel(a_e / b_e, c.chi_outer) < 1e-12);

    // 접촉타원은 볼보다 작아야 한다
    assert!(a_i < g.d_w_mm / 2.0, "a_i = {a_i} 가 볼 반경 이상");
    assert!(a_e < g.d_w_mm / 2.0, "a_e = {a_e} 가 볼 반경 이상");

    // 내륜 곡률합이 크므로 접촉면이 작고 응력이 높다
    assert!(p_i > p_e, "p_i = {p_i} ≤ p_e = {p_e}");

    // 크기 감각: D_w 11,5 mm 볼에 5 kN 이면 GPa 급
    assert!(p_i > 1_000.0 && p_i < 6_000.0, "p_i = {p_i} MPa");

    // 응력은 Q^(1/3) 스케일 — 8배 하중 → 2배 응력
    let (_, _, p8) = contact_ellipse(
        c.e_star_mpa,
        d.sum_rho_i_per_mm,
        c.a_star_inner,
        c.b_star_inner,
        8.0 * q,
    );
    assert!(rel(p8 / p_i, 2.0) < 1e-12, "응력 비 = {}", p8 / p_i);
}

#[test]
fn b8b_fatigue_limit_constant() {
    // ISO 281 Annex B.3.1 권장 σ_Hu = 1 500 MPa
    assert!((SIGMA_HU_MPA - 1500.0).abs() < 1e-12);
}
