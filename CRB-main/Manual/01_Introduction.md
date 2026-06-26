# Chapter 1: Introduction

## 1.1 System Overview

TRB Contact Analysis System은 원추 롤러 베어링(Tapered Roller Bearing)의 내부 접촉 해석을 수행하는 도구로, 두 가지 슬라이싱 모드를 제공한다:

- **Gen1 (Independent Slice)**: 각 슬라이스를 독립 비선형 스프링으로 모델링. O(n) 연산으로 빠른 초기 설계 검토에 적합.
- **Gen3 (Beam-Coupled Slice)**: Timoshenko 빔 FE와 비선형 Hertz 접촉 스프링을 커플링. O(n²) 연산으로 에지 응력 예측에 정밀.

두 모드는 동일한 입/출력 인터페이스를 공유하여 직접 비교가 가능하다.

## 1.2 Analysis Pipeline

전체 해석은 3개 레벨의 중첩 반복으로 구성된다:

```
Level 1: Bearing Equilibrium (5-DOF) ─── [공통]
 │
 ├─ 외부 하중 (F_r, F_a, M) → 내링 변위 (δx, δy, δz, γx, γy)
 │
 └─ 각 롤러 j에 대해:
      │
      Level 2: Roller Load Distribution ─── [Gen1/Gen3 분기점]
       │
       ├─ Gen1: δ_k = δ_rigid - Δz_k (독립 스프링)
       │
       └─ Gen3: [K_beam]{w} + f_contact(δ) = F_ext (빔 커플링)
            │
            Level 3: Slice Hertz Contact ─── [공통]
             │
             └─ b_k, p_max_k, h_bulk_k (단일 슬라이스 Hertz)
```

## 1.3 Dual-Mode Comparison

| Aspect | Gen1 | Gen3 |
|--------|------|------|
| Slice interaction | 없음 (독립) | Timoshenko beam + inter-slice spring |
| Roller bending | 미고려 | 완전 고려 (가변 I_k) |
| Edge stress prediction | 부정확 (truncated contact) | 정확 (pressure concentration) |
| Computation | O(n) per roller | O(n²) matrix solve |
| Use case | 초기 설계, parametric sweep | 최종 검증, profile optimization |
| ISO 16281 | Basic (Annex method) | Full (advanced method) |

## 1.4 Solver Architecture

```
BearingInput
 ├── MacroGeometry        (d, D, T, α, Z, D_we, L_we, d_pw, G_r, ...)
 ├── RacewayGeometry      (α_i, α_o, R_i, R_o, r_rib, d_uc, L_uc)
 ├── RollerProfile        (crown_type, δ_c, dub-off, R_sph)
 ├── RacewayProfile ×2    (δ_rw, W_a, Ra, custom)
 ├── Material             (E, ν, HRC)
 ├── OperatingConditions  (F_r, F_a, M, n, γ, T_op, ν_40, ν_100)
 └── SolverParams         (mode, n_slices, beam_type, tol, ...)

         │
         ▼
    ┌──────────────────────┐
    │  geometry.rs         │ ← Slicing, profile interpolation
    │  hertz.rs            │ ← Hertz contact, Weber deformation
    │  gen1.rs / gen3.rs   │ ← Roller-level solvers
    │  beam.rs             │ ← Timoshenko beam FE
    │  rib_contact.rs      │ ← Large-end rib contact
    │  bearing.rs          │ ← 5-DOF equilibrium
    │  life.rs             │ ← ISO 16281 fatigue life
    └──────────────────────┘
         │
         ▼
BearingResult
 ├── BearingEquilibrium   (displacement, roller_loads, roller_results)
 ├── FatigueLifeResult    (L_10, L_nm, a_ISO, κ, lamina_lives)
 └── Alerts               (load zone, rib stress, ...)
```

## 1.5 Validation Strategy

| Level | Description | Target Error |
|-------|-------------|-------------|
| A | Single-slice Hertz vs analytical | < 0.1% |
| B | Single-roller Gen3 vs FEA (ANSYS/ABAQUS) | < 3% |
| C | Gen1 ↔ Gen3 cross-validation (flat profile) | convergence |
| D | Full bearing vs Bearinx/MESYS/MASTA | < 5% |
| E | Experimental (strain gauge/displacement sensor) | TBD |
