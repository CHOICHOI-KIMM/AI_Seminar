# Chapter 3: Profile Modification (Micro Geometry)

## 3.1 Overview

롤러 및 레이스웨이의 미시 형상(profile modification)은 접촉 압력 분포와 에지 응력에 결정적 영향을 미치며, TRB 해석의 핵심 입력이다. 프로파일 보정은 각 슬라이스 위치에서의 간극 변화량 `Δz` [μm]로 표현된다.

### Profile Superposition

각 슬라이스 `k`에서의 총 프로파일 보정량:

$$
\Delta z_{total,k} = \Delta z_{roller,k} + \Delta z_{inner,k} + \Delta z_{outer,k}
$$

- `Δz_roller`: 롤러 크라운 + dub-off
- `Δz_inner`: 내륜 레이스웨이 crowning
- `Δz_outer`: 외륜 레이스웨이 crowning

## 3.2 Roller Crown Types

롤러 크라운은 에지 응력 집중을 방지하기 위해 중앙부 대비 단부를 미세하게 후퇴시키는 형상이다.

### 3.2.1 Logarithmic (Reusner) Profile

Lundberg가 제안하고 Reusner가 실용화한 최적 프로파일. 이론적으로 균일 접촉 응력 분포를 생성한다:

$$
\Delta z(x) = A_{log} \cdot \ln\left(\frac{1}{1 - \left(\frac{2x}{L_{we}}\right)^2}\right)
$$

여기서:
- `x`: 롤러 중심으로부터의 거리 [mm] (`x_centered = x_axial - L_we/2`)
- `A_log`: 로그 프로파일 파라미터 [μm] (일반적으로 0.0001~0.001)
- 가장자리(`|x| → L_we/2`)에서 Δz → ∞이므로, `δ_c`(crown drop)로 클램핑

**물리적 의미**: x=0(중앙)에서 Δz=0, 가장자리로 갈수록 급격히 증가 → 에지 접촉 완화

### 3.2.2 Circular Profile

원형 크라운: 단일 곡률 반경으로 정의되는 가장 단순한 형태.

$$
\Delta z(x) = R_{crown} - \sqrt{R_{crown}^2 - x^2} \approx \frac{x^2}{2 R_{crown}} \text{ (for small x)}
$$

단위 변환: mm → μm (×1000)

여기서:
- `R_crown`: 크라운 곡률 반경 [mm] (일반적으로 1000~10000 mm)

### 3.2.3 Parabolic Profile

포물선 크라운: 제조 공차 관리가 용이하여 가장 널리 사용.

$$
\Delta z(x) = c_2 \cdot x^2
$$

여기서:
- `c_2`: 포물선 계수 [μm/mm²] (일반적으로 0.001~0.1)
- `x`: 롤러 중심으로부터의 거리 [mm]

**Crown drop**과의 관계: `δ_c = c_2 × (L_we/2)²`

### 3.2.4 Custom Profile

실측 데이터 기반 프로파일. (x_mm, Δz_μm) 데이터 포인트로 정의하며, Natural cubic spline으로 보간한다.

## 3.3 Dub-off Correction

롤러 양단의 모따기(chamfer) 또는 릴리프를 모델링한다. 포물선 형태:

$$
\Delta z_{dub}(x) = \begin{cases}
\delta_{dub,S} \cdot \left(1 - \frac{x}{L_{dub,S}}\right)^2 & x < L_{dub,S} \text{ (소단)} \\
\delta_{dub,L} \cdot \left(1 - \frac{L_{we} - x}{L_{dub,L}}\right)^2 & L_{we} - x < L_{dub,L} \text{ (대단)} \\
0 & \text{otherwise}
\end{cases}
$$

| Symbol | Description | Unit |
|--------|-------------|------|
| `δ_dub_S` | Dub-off amount, small end | μm |
| `δ_dub_L` | Dub-off amount, large end | μm |
| `L_dub_S` | Dub-off length, small end | mm |
| `L_dub_L` | Dub-off length, large end | mm |

## 3.4 Raceway Profile Correction

내/외륜 레이스웨이에 대한 crowning:

$$
\Delta z_{rw}(x) = \delta_{rw} \cdot \left(\frac{x - L_{we}/2}{L_{we}/2}\right)^2
$$

여기서 `δ_rw`는 레이스웨이 crowning 량 [μm]. Custom profile이 지정된 경우 이에 추가된다.

| Symbol | Description | Unit |
|--------|-------------|------|
| `δ_rw` | Raceway crowning amount | μm |
| `W_a` | Axial waviness amplitude | μm |
| `Ra` | Surface roughness | μm |

## 3.5 Cubic Spline Interpolation

Custom profile 데이터의 보간에 Natural cubic spline (Thomas algorithm)을 사용한다.

### 알고리즘

정렬된 데이터 포인트 `(x_0, y_0), ..., (x_{n-1}, y_{n-1})`에 대해:

1. **구간 폭**: `h_i = x_{i+1} - x_i`

2. **삼중 대각 시스템** (natural boundary: S''(0) = S''(n-1) = 0):
$$
h_{i-1} c_{i-1} + 2(h_{i-1}+h_i) c_i + h_i c_{i+1} = 3\left(\frac{y_{i+1}-y_i}{h_i} - \frac{y_i-y_{i-1}}{h_{i-1}}\right)
$$

3. **Thomas algorithm**으로 `c_i` 계산 후 `b_i`, `d_i` 역산:
$$
b_i = \frac{y_{i+1}-y_i}{h_i} - \frac{h_i(c_{i+1}+2c_i)}{3}, \quad d_i = \frac{c_{i+1}-c_i}{3h_i}
$$

4. **보간**: 구간 `[x_i, x_{i+1}]`에서 `Δx = x - x_i`:
$$
S(x) = y_i + b_i \Delta x + c_i \Delta x^2 + d_i \Delta x^3
$$

### 경계 처리

- 데이터 범위 밖의 점: 최근접 데이터 값으로 클램핑
- 2개 데이터 포인트: 선형 보간으로 fallback
- 1개 이하: 상수값 반환
