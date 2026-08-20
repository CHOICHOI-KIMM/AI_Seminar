# Chapter 4: Slice Discretization Model

## 4.1 Slicing Concept

롤러 유효 접촉 길이 `L_we`를 `n` 등분하여 슬라이스로 나눈다. 각 슬라이스는 독립적인 Hertz 선접촉으로 취급된다.

```
   ← L_we →
  ┌──┬──┬──┬──┬──┬──┬──┬──┐
  │k0│k1│k2│  │  │  │  │kn│
  └──┴──┴──┴──┴──┴──┴──┴──┘
  Small end              Large end
  (D_we_min)             (D_we_max)
```

### Slice Parameters

| Symbol | Formula | Description |
|--------|---------|-------------|
| Slice width | `l_k = L_we / n` | 각 슬라이스의 축방향 폭 [mm] |
| Axial position | `x_k = (k + 0.5) × l_k` | 슬라이스 중심 위치 [mm] |
| Roller radius | `R_k = R_small + (R_large - R_small) × x_k / L_we` | 해당 위치의 롤러 반경 [mm] |

## 4.2 Equivalent Radius

두 접촉체의 곡률을 하나의 등가 반경으로 결합한다:

$$
R_{eq} = \frac{R_{roller} \cdot R_{raceway}}{R_{roller} + R_{raceway}}
$$

- **내륜 접촉**: `R_eq_inner = R_k × R_i / (R_k + R_i)` — 볼록(roller)+오목(inner)
- **외륜 접촉**: `R_eq_outer = R_k × R_o / (R_k + R_o)` — 볼록(roller)+오목(outer)

여기서 `R_i`, `R_o`는 레이스웨이 횡단 곡률 반경이다.

## 4.3 Slice Geometry Structure

각 슬라이스의 계산에 필요한 기하학적 정보:

| Field | Unit | Description |
|-------|------|-------------|
| `k` | - | Slice index (0-based) |
| `x_axial` | mm | Slice center axial position |
| `r_roller` | mm | Roller radius at this slice |
| `r_inner_race` | mm | Inner raceway transverse curvature radius |
| `r_outer_race` | mm | Outer raceway transverse curvature radius |
| `r_eq_inner` | mm | Equivalent radius (roller-inner) |
| `r_eq_outer` | mm | Equivalent radius (roller-outer) |
| `delta_z_total_inner` | μm | Total profile correction (roller + inner raceway) |
| `delta_z_total_outer` | μm | Total profile correction (roller + outer raceway) |
| `slice_width` | mm | Slice axial width |

### Profile Correction 구성

각 접촉면의 프로파일 보정량은 롤러 프로파일과 해당 레이스웨이 프로파일의 합이다:

$$
\Delta z_{total,outer,k} = \Delta z_{roller,k} + \Delta z_{raceway,outer,k}
$$

$$
\Delta z_{total,inner,k} = \Delta z_{roller,k} + \Delta z_{raceway,inner,k}
$$

롤러 프로파일(`Δz_roller`)은 양쪽 접촉면에 공통으로 적용된다.

## 4.4 Contact Gap at Each Slice (Dual-Raceway Model)

슬라이스 `k`에서의 접촉 간극(available approach)은 **양쪽 레이스웨이의 프로파일 보정을 모두 반영**한다. 단, inner raceway 접촉 법선 방향(α_i)이 outer raceway 접촉 법선 방향(α_o)과 다르므로, inner 쪽은 `cos(α_o − α_i)` 사영 계수를 적용한다:

$$
\delta_{available,k} = \delta_{rigid} - \Delta z_{total,outer,k} - \Delta z_{total,inner,k} \cdot \cos(\alpha_o - \alpha_i)
$$

- `δ_rigid`: 강체 접근량 [μm] (outer raceway 법선 방향, Level 1/2에서 결정)
- `Δz_total,outer,k`: 외륜 쪽 프로파일 보정량 [μm]
- `Δz_total,inner,k`: 내륜 쪽 프로파일 보정량 [μm]
- `cos(α_o − α_i)`: inner→outer 법선 방향 사영 계수 (TRB에서 ≈ 1.0)

접촉 조건: `δ_available,k > 0` → 접촉 발생, `δ_available,k ≤ 0` → 비접촉

> **참고 (이전 버전과의 차이)**: 이전에는 outer raceway 프로파일만 사용하여 `δ_k = δ_rigid − Δz_total,outer,k`로 계산했다. 현재 dual-raceway 모델에서는 inner raceway 프로파일도 하중 분포(contact path)에 영향을 미친다.

### Profile 효과 시각화

```
δ_rigid ─────────────────────── (constant)
         ╲                ╱
          ╲  Δz_outer_k ╱      ← outer raceway profile correction
           ╲   +        ╱
            ╲Δz_inner_k╱       ← inner raceway profile correction (projected)
             ╲────────╱

δ_available = ─ ─ ─ ─ ─ ─ ─ ─  (actual available approach per slice)
               ↑                ↑
            edge: small δ    center: large δ
            (reduced contact)  (full contact)
```

## 4.5 Slice Count Guidelines

| Application | Recommended n_slices |
|-------------|---------------------|
| Quick parametric sweep | 20~30 |
| Standard analysis | 50~100 |
| Profile optimization | 100~200 |
| Research / validation | 200+ |

Gen3의 경우 빔 강성행렬 크기가 2n×2n이므로, 슬라이스 수가 연산 비용에 직접 영향을 미친다. Gen1은 O(n)이므로 슬라이스 수 증가의 영향이 작다.
