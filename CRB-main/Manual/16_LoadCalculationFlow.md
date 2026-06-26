# Chapter 16: 하중 계산 전체 플로우 (Dual-Raceway + Rib Contact)

## 16.1 사용자 입력

| 카테고리 | 입력 항목 |
|---------|----------|
| **기하** | d_pw, z, l_we, d_we_max/min, α_i, α_o, α_rib, g_r, h_rib |
| **프로파일** | 롤러 크라운(Log/Circ/Para/Custom, δ_c), 덥오프(δ_dub, l_dub), 레이스웨이 크라우닝(inner/outer 별도) |
| **리브** | r_sph(롤러 구면), r_rib(필렛), r_rib_circ(원주, 자동/수동), h_c(접촉높이, 자동/수동) |
| **재료** | E_roller, E_ring, ν |
| **하중** | F_x, F_y, F_a, M_x, M_y, γ_ext |
| **프리로드** | 모드(Force/Displacement), 값 |
| **솔버** | n_slices, tol, max_iter, rib_contact_mode(Coupled/PostProcess) |

## 16.2 Step 0: 전처리 — 슬라이스 생성

> 참조: geometry.rs — `compute_slices()`

```
L_we → n 등분

FOR each slice k = 0..n-1:
  x_k = (k + 0.5) × L_we/n              ← 슬라이스 중심 축방향 위치
  r_roller_k = r_small + (r_large − r_small) × x_k/L_we   ← 테이퍼

  // 등가 반경 (Harris/Nguyen-Schäfer orbital curvature)
  γ_i = D_k·cos(α_i) / D_pw_k
  γ_o = D_k·cos(α_o) / D_pw_k
  R_eq_inner_k = (D_k/2)·(1 − γ_i)     ← inner: conforming → 작음
  R_eq_outer_k = (D_k/2)·(1 + γ_o)     ← outer: non-conforming → 큼

  // 프로파일 보정 (3개 표면)
  Δz_roller_k = crown(x_k) + dub_off(x_k)     ← 롤러 자체
  Δz_inner_k  = raceway_profile_inner(x_k)     ← 내륜 레이스웨이
  Δz_outer_k  = raceway_profile_outer(x_k)     ← 외륜 레이스웨이

  // 합산 (롤러는 양쪽 공통)
  Δz_total_inner_k = Δz_roller_k + Δz_inner_k
  Δz_total_outer_k = Δz_roller_k + Δz_outer_k
```

### 크라운 타입별 수식

| 타입 | 수식 | 마스터 파라미터 |
|------|------|---------------|
| Logarithmic | `A·ln(1/(1−(x/half_l)²))` | A는 δ_c에서 역산 |
| Circular | `R − √(R²−x²)` [μm] | R은 δ_c에서 역산 |
| Parabolic | `c₂·x²` | c₂ = δ_c / half_l² |
| Custom | 큐빅 스플라인 보간 | 실측 데이터 |
| Polynomial | `−(p₁x⁴+p₂x³+p₃x²+p₄x+p₅)` | 부호 반전(실측→솔버) |

### 덥오프 수식

```
소단: x < l_dub_s → δ_dub_s × (1 − x/l_dub_s)²
대단: (l_we−x) < l_dub_l → δ_dub_l × (1 − (l_we−x)/l_dub_l)²
```

## 16.3 Step 1: 프리로드 변위 결정

> 참조: bearing.rs — `solve_preload_displacement()`

```
cos_α_diff = cos(α_o − α_i)

SWITCH preload_mode:
  DisplacementFromForce / DisplacementFromForceIterative:
    // 순수 축하중 상태에서 1D NR로 F_a → δz 변환
    목표: Z × Q(δz·sinα) × sinα = F_a

    FOR iter = 0..max_iter:
      δ_rigid = δz × sin(α_o)

      IF rib_coupled:
        δ_rib = 0
        FOR rib_iter = 0..30:                     ← 리브 내부 반복
          δ_eff = max(δ_rigid − δ_rib·sinα, 0)
          Q = Gen1(δ_eff, cos_α_diff)             ← dual-raceway
          Q_axial = Q·sin(α_o−α_i)/cos(α_i)
          δ_rib_new = rib_hertz(Q_axial)          ← §16.6 참조
          if |δ_rib_new − δ_rib| < 0.001: BREAK
          δ_rib = δ_rib_new
      ELSE:
        Q = Gen1(δ_rigid, cos_α_diff)

      F_a_calc = Z × Q × sin(α_o)
      err = F_a_calc − F_a_target
      // NR 업데이트 δz

    → δz_preload [μm]

  Displacement:
    δz_preload = 사용자 입력값 직접 사용
```

## 16.4 Step 2: Phase A — 힘 평형 (Newton-Raphson)

> 참조: bearing.rs — `compute_residual()`, `solve_bearing_equilibrium()`

미지수: `[δx, δy, (δz)]` — 3×3 NR(δz 자유) 또는 2×2 NR(δz 고정)

```
cos_α_diff = cos(α_o − α_i)
sin_α_diff = sin(α_o − α_i)

FOR NR_iter = 0..max_iter:

  residual = [0; 5]

  FOR roller j = 0..Z-1:

    ── Step 2A: 강체 접근량 ──────────────────────────────────────

      δ_r = δx·cosψ + δy·sinψ                    (반경 성분)
      δ_a = δz + (d_pw/2)·1000·(γx·sinψ − γy·cosψ) (축 성분)
      δ_rigid = δ_r·cosα_o + δ_a·sinα_o − g_r/2  (α_o 방향)

    IF δ_rigid ≤ 0: Q_j = 0, SKIP

    ── Step 2B: 리브 결합 반복 (Coupled 모드) ────────────────────

      δ_rib = 0
      FOR rib_iter = 0..30:

        Step 2B-1: 유효 접근량
          δ_eff = max(δ_rigid − δ_rib·sinα_o, 0)

        Step 2B-2: Gen1 슬라이스 해석 (dual-raceway)
          FOR slice k = 0..n-1:
            δ_available_k = δ_eff
                           − Δz_total_outer_k
                           − Δz_total_inner_k × cos_α_diff

            IF δ_available_k > 0:

              Step 2B-3: Dual-raceway Hertz NR
                solve: δ_hertz_outer(q, R_eq_outer, E*)
                     + δ_hertz_inner(q, R_eq_inner, E*) × cos_α_diff
                     = δ_available_k / 1000  [mm]

                여기서 δ_hertz(q, R, E*) =
                  (2q/πE*)·[ln(4R/b) − 0.5]
                  b = √(4qR/πE*)

                → q_k [N/mm]

            Q_k = q_k × slice_width
          Q_normal = Σ Q_k

        Step 2B-4: 축방향 분력 → 리브 접촉
          Q_axial = Q_normal × sin(α_o−α_i) / cos(α_i)

        Step 2B-5: 리브 Hertz 타원 접촉 (§16.6 참조)
          → δ_rib_new [μm]

        IF |δ_rib_new − δ_rib| < 0.001 μm: BREAK
        δ_rib = δ_rib_new

      → Q_normal_j (리브 컴플라이언스 반영 완료)

    ── Step 2C: 5-DOF 잔차 기여 (Harris 정식) ───────────────────

      residual[0] += Q_j·cosα_o·cosψ_j          (Fx)
      residual[1] += Q_j·cosα_o·sinψ_j          (Fy)
      residual[2] += Q_j·sinα_o                  (Fz)
      residual[3] += Q_j·(d_pw/2)·sinα_o·sinψ_j (Mx)
      residual[4] += −Q_j·(d_pw/2)·sinα_o·cosψ_j (My)

  // 외력 차감
  residual[0] −= Fx;  residual[1] −= Fy;  residual[2] −= Fa
  residual[3] −= Mx;  residual[4] −= My

  // 수렴 판정
  IF ‖R‖ / F_total < tol: BREAK

  // NR 업데이트 (수치 Jacobian + line search)
  J = numerical_jacobian(h = 0.01 μm)
  Δδ = J⁻¹·(−R), step limiting, backtracking
  δ += Δδ
```

### 리브 접촉의 평형식 반영 방식

리브 힘은 평형식(residual)에 **별도 항으로 직접 나타나지 않는다.** 대신 Coupled 모드에서 리브 변형 δ_rib이 유효 접근량 δ_eff를 줄여, Q_normal 자체가 리브 컴플라이언스를 내포하게 된다.

```
물리적 모델:

  [Roller]──spring(α_o방향)──[Raceway]
      │
      └──rib spring(축방향)──[Flange]

  리브 = 축방향 직렬 스프링 → 접근량 감소 효과
```

PostProcess 모드에서는 리브 변형을 무시하고 δ_rigid 그대로 사용한다. 리브 결과는 사후 계산만 하고 평형에 영향 없음.

## 16.5 Step 3: Phase B — 모멘트 평형

> 참조: bearing.rs — moment equilibrium section

M_x, M_y 또는 γ_ext ≠ 0 일 때만 실행.

```
미지수: [γx, γy]

FOR iter = 0..max_iter:
  R_moment = [residual[3], residual[4]]

  IF ‖R_moment‖ / M_norm < tol: BREAK

  // 2×2 중심차분 Jacobian (h = 5e-4 rad)
  J_m[i,j] = (R[3+i](γ+h) − R[3+i](γ−h)) / (2h)

  Δγ = J_m⁻¹·(−R_moment), step limiting, line search
  γ += Δγ
```

## 16.6 리브 접촉 상세 (Elliptical Hertz Point Contact)

> 참조: rib_contact.rs — `compute_rib_contact()`

롤러 구면 끝단이 리브 토로이달 면에 눌리는 **타원형 점접촉** 모델이다.

```
[롤러 구면 끝단]  ← r_sph (볼록, 양방향 동일)
        ⊙
   ──────────── ← 리브면 (r_rib: 자오선 오목, r_rib_circ: 원주 오목)
```

### 계산 체인

```
Q_axial [N]  (입력)
    │
    ├─ f_rib = Q_axial                        ← 리브 법선력
    │
    ├─ E* = combined_elastic_modulus(...)      ← 결합 탄성 계수
    │
    ├─ 접촉점 위치:
    │   r_base = d_pw/2 + (l_we/2)·sinγ − (d_we_max/2)·cosγ
    │   h_c = 사용자 입력 or h_rib/2 (기본값)
    │   r_contact = r_base + h_c
    │
    ├─ 원주 곡률:
    │   r_rib_circ = r_contact / sin(α_rib)   ← 자동 계산 (or 사용자 override)
    │
    ├─ 등가 반경 (convex-concave, conforming):
    │   R_x = 1/(1/r_sph − 1/r_rib)          (자오선 방향)
    │   R_y = 1/(1/r_sph − 1/r_rib_circ)     (원주 방향)
    │
    ├─ Hamrock-Brewe 타원 계수:
    │   ratio = R_y / R_x
    │   κ_e = 1.0339 × ratio^0.6360           (ellipticity a/b)
    │   F_e = 1.5277 + 0.6023 × ln(ratio)     (1st kind elliptic integral)
    │   E_e = 1.0003 + 0.5968 / ratio         (2nd kind elliptic integral)
    │
    ├─ 곡률 합:
    │   Σρ = 1/R_x + 1/R_y
    │
    ├─ 접촉 타원:
    │   a = (3·κ²·E_e·F_rib / π·E*·Σρ)^(1/3)     [mm] 장축
    │   b = a / κ_e                                 [mm] 단축
    │
    ├─ 최대 접촉압:
    │   p_max = 3·F_rib / (2π·a·b)                 [MPa]
    │
    ├─ Hertz 접근량:
    │   δ_rib = a²·Σρ / (2·κ_e·F_e) × 1000        [μm]
    │
    ├─ 접선 강성:
    │   K_rib = (3/2)·F_rib / δ_rib                 [N/μm]
    │
    └─ 스핀 모멘트:
        M_spin = (3/8)·μ·F_rib·a                    [N·mm] (μ = 0.002)
```

### 리브 접촉의 물리적 역할

| 항목 | 설명 |
|------|------|
| **축방향 구속** | 롤러가 리브 쪽으로 밀리는 것을 막음 |
| **평형 기여** | Q_normal에 간접 흡수 (δ_eff 감소) |
| **Coupled vs PostProcess** | Coupled: δ_rib이 평형에 영향 / PostProcess: 사후 계산만 |
| **강성** | 3/2 Hertz 비선형 (K ∝ F^(1/3)) |

## 16.7 Step 4: 최종 결과 수집

수렴된 `[δx, δy, δz, γx, γy]`로 마지막 `compute_residual` 실행.

```
출력 구조:
├─ 내륜 평형 변위: [δx, δy, δz, γx, γy]
│
├─ 롤러별 (j = 0..Z-1):
│   ├─ ψ_j [°], Q_normal_j [N]
│   │
│   ├─ 슬라이스별 (k = 0..n-1):
│   │   ├─ δ_k [μm]         — 슬라이스 접근량
│   │   ├─ q_k [N/mm]       — 선하중
│   │   ├─ inner:  b_k, p_max_k, h_bulk_k, K_hertz_k
│   │   ├─ outer:  b_k, p_max_k, h_bulk_k, K_hertz_k
│   │   └─ K_combined_k     — 직렬 4-스프링 합성 강성
│   │
│   └─ 리브 접촉:
│       ├─ F_rib [N]         — 리브 법선력 (= Q_axial)
│       ├─ a, b [mm]         — 접촉 타원 반축
│       ├─ p_max_rib [MPa]   — 최대 접촉압
│       ├─ δ_rib [μm]        — 리브 접근량
│       ├─ K_rib [N/μm]      — 접선 강성 (3F/2δ)
│       └─ spin_moment [N·mm] — 스핀 모멘트 (μ=0.002)
│
├─ 수명 (ISO 16281)
├─ 강성 (K_radial, K_axial)
└─ 경고 (과하중, 엣지 로딩, 리브 응력 등)
```

## 16.8 전체 구조도

```
사용자 입력 (기하, 재료, 하중, 프로파일, 리브)
    │
    ▼
[Step 0] 슬라이스 생성
    │  n개 슬라이스 × (R_eq_inner, R_eq_outer, Δz_inner, Δz_outer)
    ▼
[Step 1] 프리로드: F_a → δz_preload (1D NR, 리브 결합 포함)
    │
    ▼
[Step 2] Phase A ═══ NR 반복 ═══════════════════════════════════════╗
    │                                                                ║
    │  for each roller j:                                            ║
    │    ┌─────────────────────────────────────────────────────┐     ║
    │    │ δ_rigid = project(δ, ψ, α_o)                       │     ║
    │    │                                                     │     ║
    │    │ ┌── Rib Coupled Loop (max 30) ──────────────────┐  │     ║
    │    │ │                                                │  │     ║
    │    │ │  δ_eff = δ_rigid − δ_rib·sinα_o               │  │     ║
    │    │ │          ↓                                     │  │     ║
    │    │ │  Gen1 Dual-Raceway:                            │  │     ║
    │    │ │    δ_avail = δ_eff − Δz_outer − Δz_inner·cosΔα│  │     ║
    │    │ │    solve: δ_h_outer(q) + δ_h_inner(q)·cosΔα   │  │     ║
    │    │ │           = δ_avail                            │  │     ║
    │    │ │    → Q_normal                                  │  │     ║
    │    │ │          ↓                                     │  │     ║
    │    │ │  Q_axial = Q·sinΔα/cosα_i                     │  │     ║
    │    │ │          ↓                                     │  │     ║
    │    │ │  Rib Elliptical Hertz:                         │  │     ║
    │    │ │    (R_x, R_y) → Hamrock-Brewe → a, b, p_max   │  │     ║
    │    │ │    → δ_rib                                     │  │     ║
    │    │ │                                                │  │     ║
    │    │ │  수렴? |Δδ_rib| < 0.001 μm                    │  │     ║
    │    │ └────────────────────────────────────────────────┘  │     ║
    │    │                                                     │     ║
    │    │ residual += Q_j 분력 (cosα, sinα, ψ)               │     ║
    │    └─────────────────────────────────────────────────────┘     ║
    │                                                                ║
    │  R = Σ(롤러 힘) − 외력                                        ║
    │  J → Δδ → line search                                         ║
    │  수렴? ‖R‖/F_total < tol ════════════════════════════ No ═════╝
    │     Yes
    ▼
[Step 3] Phase B ═══ NR 반복 (모멘트) ══════════════════════════════╗
    │  미지수: γx, γy                                                ║
    │  R_moment → J_moment → Δγ → line search                       ║
    │  수렴? ‖R_m‖/M_norm < tol ═══════════════════════════ No ═════╝
    │     Yes
    ▼
[Step 4] 최종 결과 수집 + 수명 + 강성 + 경고
```

## 16.9 방향 요약 (좌표계 참조)

모든 변위와 힘의 방향 관계 정리:

| 변수 | 방향 | 비고 |
|------|------|------|
| `δ_rigid` | outer raceway 법선 (α_o) | 내륜 중심 변위를 롤러 위치에서 α_o 방향으로 사영 |
| `Q_normal` | outer raceway 법선 (α_o) | 평형식에서 cosα/sinα로 반경/축 분해 |
| `Q_axial` | 베어링 축방향 | Q_normal × sin(α_o−α_i)/cos(α_i) |
| `δ_rib` | 축방향 (≈ 리브면 법선) | α_o 방향으로 환원 시 ×sinα_o |
| `δ_hertz_outer` | outer raceway 법선 (α_o) | 기준 방향 — 사영 불필요 |
| `δ_hertz_inner` | inner raceway 법선 (α_i) | α_o 방향으로 사영 시 ×cos(α_o−α_i) |
| `Δz_total_outer` | α_o 방향 | 기준 방향 |
| `Δz_total_inner` | α_i 방향 | α_o 방향으로 사영 시 ×cos(α_o−α_i) |
