use std::f64::consts::PI;

use nalgebra::{DMatrix, Matrix4};

use crate::solver::types::*;

/// Compute beam section properties from roller radius.
/// Returns (I [mm⁴], A [mm²]).
///
/// CRB (Phase 3): 원통 roller 이므로 모든 slice 가 동일 r_roller = D_we/2 →
/// 자동으로 I = const, A = const 로 전체 beam 요소가 균일. 별도 CRB 분기 불필요.
/// (TRB 는 slice 별 r 다르므로 elem-wise 재계산 자연스러움 — 동일 코드가 양쪽 지원)
pub fn beam_section_properties(r_roller: f64) -> (f64, f64) {
    let i = PI / 4.0 * r_roller.powi(4);
    let a = PI * r_roller.powi(2);
    (i, a)
}

/// Timoshenko beam element stiffness matrix (4×4).
///
/// 2 nodes × 2 DOF (w: transverse displacement, θ: rotation).
/// Φ = 12·E·I / (κ·G·A·L²) captures shear deformation.
/// When beam_type is EulerBernoulli, Φ = 0.
pub fn beam_element_stiffness(
    e: f64,
    i_k: f64,
    a_k: f64,
    g: f64,
    kappa: f64,
    l_e: f64,
    beam_type: BeamType,
) -> Matrix4<f64> {
    let phi = match beam_type {
        BeamType::Timoshenko => 12.0 * e * i_k / (kappa * g * a_k * l_e * l_e),
        BeamType::EulerBernoulli => 0.0,
    };

    let l = l_e;
    let l2 = l * l;
    let coeff = e * i_k / (l * l2 * (1.0 + phi));

    Matrix4::new(
        12.0 * coeff,
        6.0 * l * coeff,
        -12.0 * coeff,
        6.0 * l * coeff,
        6.0 * l * coeff,
        (4.0 + phi) * l2 * coeff,
        -6.0 * l * coeff,
        (2.0 - phi) * l2 * coeff,
        -12.0 * coeff,
        -6.0 * l * coeff,
        12.0 * coeff,
        -6.0 * l * coeff,
        6.0 * l * coeff,
        (2.0 - phi) * l2 * coeff,
        -6.0 * l * coeff,
        (4.0 + phi) * l2 * coeff,
    )
}

/// Assemble global beam stiffness matrix (2n × 2n dense).
///
/// n slices → n nodes → n-1 beam elements.
/// DOF ordering: [w₀, θ₀, w₁, θ₁, ..., w_{n-1}, θ_{n-1}].
/// Boundary condition: free-free (contact springs provide constraint).
pub fn assemble_beam_stiffness(
    slices: &[SliceGeometry],
    material: &Material,
    beam_type: BeamType,
) -> DMatrix<f64> {
    let n = slices.len();
    let ndof = 2 * n;
    let mut k_global = DMatrix::zeros(ndof, ndof);

    if n < 2 {
        return k_global;
    }

    let e = material.e_roller * 1000.0; // GPa → MPa
    let nu = material.nu;
    let g = e / (2.0 * (1.0 + nu));
    let kappa = 10.0 / (9.0 + 10.0 * nu); // Timoshenko shear correction (circular section)
    let l_e = slices[0].slice_width;

    for elem in 0..n - 1 {
        let r_avg = (slices[elem].r_roller + slices[elem + 1].r_roller) / 2.0;
        let (i_e, a_e) = beam_section_properties(r_avg);

        let k_e = beam_element_stiffness(e, i_e, a_e, g, kappa, l_e, beam_type);

        // Scatter into global matrix: DOFs [2*elem, 2*elem+1, 2*elem+2, 2*elem+3]
        let dofs = [2 * elem, 2 * elem + 1, 2 * elem + 2, 2 * elem + 3];
        for (i, &di) in dofs.iter().enumerate() {
            for (j, &dj) in dofs.iter().enumerate() {
                k_global[(di, dj)] += k_e[(i, j)];
            }
        }
    }

    k_global
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_uniform_slices(n: usize) -> (Vec<SliceGeometry>, Material) {
        let l_we = 15.0;
        let slice_width = l_we / n as f64;
        let r = 4.5; // uniform radius [mm]
        let slices: Vec<SliceGeometry> = (0..n)
            .map(|k| SliceGeometry {
                k,
                x_axial: (k as f64 + 0.5) * slice_width,
                r_roller: r,
                r_inner_race: 200.0,
                r_outer_race: 200.0,
                r_eq_inner: (r * 200.0) / (r + 200.0),
                r_eq_outer: (r * 200.0) / (r + 200.0),
                delta_z_total_inner: 0.0, delta_z_total_outer: 0.0,
                slice_width,
            })
            .collect();
        (slices, Material::default())
    }

    #[test]
    fn test_element_symmetry() {
        let e = 210_000.0;
        let r = 4.5;
        let (i, a) = beam_section_properties(r);
        let g = e / (2.0 * 1.3);
        let kappa = 10.0 / (9.0 + 3.0); // nu=0.3
        let l_e = 0.75;

        let k_e = beam_element_stiffness(e, i, a, g, kappa, l_e, BeamType::Timoshenko);

        // Symmetry check: K_e[i,j] == K_e[j,i]
        for i in 0..4 {
            for j in 0..4 {
                assert!(
                    (k_e[(i, j)] - k_e[(j, i)]).abs() < 1e-6 * k_e[(i, j)].abs().max(1.0),
                    "K_e[{i},{j}]={} != K_e[{j},{i}]={}",
                    k_e[(i, j)],
                    k_e[(j, i)]
                );
            }
        }
    }

    #[test]
    fn test_assembly_size_and_symmetry() {
        let (slices, mat) = make_uniform_slices(20);
        let k_g = assemble_beam_stiffness(&slices, &mat, BeamType::Timoshenko);

        assert_eq!(k_g.nrows(), 40);
        assert_eq!(k_g.ncols(), 40);

        // Symmetry
        for i in 0..40 {
            for j in i..40 {
                assert!(
                    (k_g[(i, j)] - k_g[(j, i)]).abs() < 1e-6 * k_g[(i, j)].abs().max(1.0),
                    "K_global[{i},{j}] not symmetric"
                );
            }
        }
    }

    #[test]
    fn test_euler_bernoulli_cantilever() {
        // Cantilever beam: n uniform elements, clamped at node 0, point load P at last node.
        // Analytical tip deflection: w = P·L³ / (3·E·I)
        let n = 20;
        let r = 4.5_f64;
        let l_total = 15.0_f64;
        let l_e = l_total / n as f64;
        let e = 210_000.0_f64; // MPa
        let (i_sec, a_sec) = beam_section_properties(r);

        // Build global stiffness (EB)
        let ndof = 2 * (n + 1); // n+1 nodes for n elements
        let mut k_g = DMatrix::zeros(ndof, ndof);
        let g = e / (2.6);
        let kappa = 0.85;

        for elem in 0..n {
            let k_e = beam_element_stiffness(e, i_sec, a_sec, g, kappa, l_e, BeamType::EulerBernoulli);
            let dofs = [2 * elem, 2 * elem + 1, 2 * elem + 2, 2 * elem + 3];
            for (ii, &di) in dofs.iter().enumerate() {
                for (jj, &dj) in dofs.iter().enumerate() {
                    k_g[(di, dj)] += k_e[(ii, jj)];
                }
            }
        }

        // Apply BC: clamp node 0 (w₀=0, θ₀=0)
        // Reduce system: remove DOFs 0 and 1
        let n_free = ndof - 2;
        let mut k_red = DMatrix::zeros(n_free, n_free);
        for i in 0..n_free {
            for j in 0..n_free {
                k_red[(i, j)] = k_g[(i + 2, j + 2)];
            }
        }

        // Force vector: P at last node displacement DOF
        let p = 1000.0; // N
        let mut f = nalgebra::DVector::zeros(n_free);
        f[n_free - 2] = p; // w DOF of last node

        // Solve
        let decomp = k_red.lu();
        let u = decomp.solve(&f).unwrap();

        let w_tip = u[n_free - 2]; // displacement at tip

        // Analytical: w = P·L³ / (3·E·I)
        let w_analytical = p * l_total.powi(3) / (3.0 * e * i_sec);

        let rel_err = (w_tip - w_analytical).abs() / w_analytical.abs();
        assert!(
            rel_err < 0.01,
            "Cantilever: w_FE={w_tip:.6e}, w_analytical={w_analytical:.6e}, err={:.2}%",
            rel_err * 100.0
        );
    }

    #[test]
    fn test_timoshenko_shear_effect() {
        // Timoshenko deflection should be >= Euler-Bernoulli (shear adds flexibility)
        let n = 10;
        let r = 4.5_f64;
        let l_total = 15.0_f64;
        let l_e = l_total / n as f64;
        let e = 210_000.0_f64;
        let nu = 0.3_f64;
        let (i_sec, a_sec) = beam_section_properties(r);
        let g = e / (2.0 * (1.0 + nu));
        let kappa = 10.0 / (9.0 + 10.0 * nu);

        let mut w_tips = Vec::new();

        for bt in [BeamType::EulerBernoulli, BeamType::Timoshenko] {
            let ndof = 2 * (n + 1);
            let mut k_g = DMatrix::zeros(ndof, ndof);
            for elem in 0..n {
                let k_e = beam_element_stiffness(e, i_sec, a_sec, g, kappa, l_e, bt);
                let dofs = [2 * elem, 2 * elem + 1, 2 * elem + 2, 2 * elem + 3];
                for (ii, &di) in dofs.iter().enumerate() {
                    for (jj, &dj) in dofs.iter().enumerate() {
                        k_g[(di, dj)] += k_e[(ii, jj)];
                    }
                }
            }

            let n_free = ndof - 2;
            let mut k_red = DMatrix::zeros(n_free, n_free);
            for i in 0..n_free {
                for j in 0..n_free {
                    k_red[(i, j)] = k_g[(i + 2, j + 2)];
                }
            }

            let p = 1000.0;
            let mut f = nalgebra::DVector::zeros(n_free);
            f[n_free - 2] = p;

            let u = k_red.lu().solve(&f).unwrap();
            w_tips.push(u[n_free - 2]);
        }

        let w_eb = w_tips[0];
        let w_timo = w_tips[1];

        assert!(
            w_timo >= w_eb * 0.999, // Timoshenko should be >= EB (allow tiny numerical tolerance)
            "Timoshenko w={w_timo:.6e} should be >= EB w={w_eb:.6e}"
        );
        // For a stocky beam (L/d ≈ 15/9 ≈ 1.67), shear effect should be noticeable
        let shear_increase = (w_timo - w_eb) / w_eb * 100.0;
        assert!(
            shear_increase > 0.1,
            "Shear increase only {shear_increase:.4}%, expected > 0.1% for stocky beam"
        );
    }
}
