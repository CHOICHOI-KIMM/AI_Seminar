# 15. Transient Roller Dynamics & Sliding Analysis

## 15.1 Overview

The transient solver tracks roller rotational dynamics under time-varying loads, computing slip ratio (SRR), sliding velocity, friction power, and cumulative damage for each roller. It adds per-slice SRR analysis to capture profile-induced geometric micro-slip.

**Pipeline per time step:**
1. Interpolate load → solve bearing equilibrium (fast Gen1)
2. Compute target roller speed from cone apex kinematics
3. Integrate roller rotational dynamics (Forward Euler + traction clamping)
4. Compute per-slice SRR (dynamic + geometric)
5. Accumulate damage metrics

## 15.2 Cone Apex Kinematics

For a TRB with inner/outer contact angles α_i, α_o and roller half-taper angle φ = (α_o − α_i)/2:

$$\omega_{roller,target} = \omega_i \cdot \frac{\sin\alpha_i \cdot \sin\alpha_o}{\sin\phi \cdot (\sin\alpha_i + \sin\alpha_o)}$$

This is the pure-rolling angular velocity — when all cones share a common apex, every slice has identical SRR.

## 15.3 Roller Dynamics Integration

Torque balance on each roller:

$$I \cdot \frac{d\omega}{dt} = \tau_{traction} + \tau_{cage} + \tau_{rib} - \tau_{viscous}$$

| Torque | Model | Formula |
|--------|-------|---------|
| τ_viscous | Couette flow in cage pocket gap | C_viscous × ω_roller |
| τ_cage | Centrifugal cage pocket friction | k_cage × ω_cage² |
| τ_traction | EHL contact (Eyring or simplified) | μ(SRR) × Q × r_mean |
| τ_rib | Rib face sliding friction (Houpert 2002) | μ_rib × F_rib × r_contact |

**Rib friction torque (v2):** The rib-roller large-end contact is pure sliding. The rib friction coefficient μ_rib = 0.05 (mixed/boundary lubrication, Houpert 2002: 0.03–0.08). F_rib is the rib contact force from the bearing equilibrium solution, and r_contact is the radial position of the rib contact point.

**RollerDragParams** — computed once from bearing geometry + oil viscosity:
- `C_viscous = η₀ × r² × A_wetted / gap` where A_wetted = f_wet × 2π × r × L
- `k_cage = μ_pocket × m_roller × R_pitch × (2/3 × r_roller)`
- `cage_speed_ratio = (1 − d_we·cos(α) / d_pw) / 2`

**Integration method: 4th-order Runge-Kutta (RK4)**

The traction coefficient depends on SRR which depends on ω, so the equation of motion is nonlinear. RK4 provides 4th-order accuracy (Creju et al. 1994) vs 1st-order Forward Euler:

```
k₁ = f(ω_n)
k₂ = f(ω_n + 0.5·dt·k₁)
k₃ = f(ω_n + 0.5·dt·k₂)
k₄ = f(ω_n + dt·k₃)
ω_{n+1} = ω_n + dt·(k₁ + 2k₂ + 2k₃ + k₄)/6
```

where f(ω) = (τ_applied(ω) − τ_viscous(ω)) / I, with τ_applied clamped to ±τ_available.

## 15.4 Slip Ratio (SRR)

**Roller-average (dynamic) SRR:**

$$SRR_{dynamic} = \frac{\omega_{actual} - \omega_{target}}{\omega_{target}}$$

**Slip threshold:** |SRR| > 0.05% → roller classified as "in slip".

## 15.5 Per-Slice Geometric SRR

Profile modifications (crown, dub-off) break the ideal cone geometry, creating position-dependent micro-slip along the roller length.

**Model:**

$$SRR_{total,k} = SRR_{dynamic} + SRR_{geometric,k}$$

The profile correction Δz [μm] changes the effective rolling radius at each slice:

$$r_{eff,k} = r_{cone,k} - \Delta z_k \cdot \cos(\phi) \cdot 10^{-3} \quad [mm]$$

Since the roller rotates as a rigid body (same ω for all slices), slices with reduced radius have lower surface velocity. The geometric SRR is the deviation from the roller mean:

$$SRR_{geometric,k} = -\frac{(\Delta z_k - \overline{\Delta z}) \cdot \cos(\phi) \cdot 10^{-3}}{r_k}$$

Where:
- Δz_k = (Δz_total_inner + Δz_total_outer) / 2 — averaged profile correction [μm]
- r_k — roller radius at slice k [mm]
- cos(φ) — projection factor (≈ 1 for small taper angles)
- 10⁻³ — converts μm → mm for dimensionless ratio
- Mean subtraction: the dynamic SRR already captures the roller-average effect

**Typical magnitudes:** 0.01–0.1% for standard crown (δ_c = 2–10 μm) and dub-off profiles.

**Key property:** For flat profiles (no crown/dub-off), Δz = 0 at all slices → SRR_geometric = 0. Only profile modifications create differential sliding.

### 15.5.1 Heathcote Slip

Within each slice's contact zone, elastic deformation creates a curved contact surface with varying rolling radius. The second-order approximation:

$$SRR_{Heathcote,k} = \frac{b_k^2}{8 \cdot R_{eq} \cdot r_k}$$

where b_k is the Hertz contact half-width estimated from nominal load:

$$b_k^2 = \frac{4 \cdot q_k \cdot R_{eq}}{\pi \cdot E^*}$$

The Heathcote contribution is added to the profile SRR before mean subtraction. Typical magnitudes: 1e-5 to 1e-4 (much smaller than profile SRR but physically always present).

**Pre-computation:** Both profile and Heathcote SRR vectors are computed once before the time loop (geometry and nominal load are constant).

## 15.6 Viscosity Estimation

ASTM D341 (Walther) equation for kinematic viscosity interpolation:

$$\ln(\ln(\nu + 0.7)) = A - B \cdot \ln(T)$$

where T [K] is absolute temperature. Given ν₄₀ and ν₁₀₀:

$$B = \frac{z_1 - z_2}{\ln(T_2) - \ln(T_1)}, \quad z(T) = z_1 - B \cdot (\ln T - \ln T_1)$$

Dynamic viscosity: η = ν × 10⁻⁶ × ρ_oil [Pa·s]

## 15.7 WEC Risk: Directional SRR

The WEC risk assessment now distinguishes SRR direction based on literature (Wear, 2022):

- **Negative SRR** (ω_actual < ω_target, roller slower): WEC-prone direction. Traction force drives subsurface crack propagation.
- **Positive SRR** (ω_actual > ω_target, roller faster): Does not produce WEC under otherwise identical conditions.

**Directional thresholds:** Negative SRR uses 60% of standard thresholds (more conservative):

| Risk Level | Standard threshold | Negative SRR threshold |
|------------|-------------------|----------------------|
| Medium | SRR > 2% | SRR_neg > 1.2% |
| High | SRR > 5% | SRR_neg > 3% |
| Critical | SRR > 10% | SRR_neg > 6% |

The `SmearingRiskAssessment` now reports `max_negative_srr` and `max_positive_srr` separately.

## 15.8 Contour Visualization

The Slice SRR tab shows:

1. **Heatmap** (roller position ψ × slice position %L → SRR [%])
   - X axis: roller circumferential position [°]
   - Y axis: slice axial position [% of effective length]
   - Color: total SRR = dynamic + geometric [%]
   - Time slider to select snapshot

2. **Time history** of maximum per-slice SRR across all rollers

## 15.8 Optimizations

| Optimization | Description |
|-------------|-------------|
| Slice caching | compute_slices() called once |
| Warm-start | Previous displacement seeds NR |
| Load-change skip | Reuse equilibrium when load barely changes |
| Lightweight solver | solve_equilibrium_fast() — no validation/life/film |
| Rayon parallelization | Roller dynamics integrated in parallel |
| Adaptive solve frequency | Equilibrium solve interval adapts to load rate |
