# Gen3 Beam-Coupled Slice Solver

## 개요

Gen3 솔버는 롤러를 Timoshenko beam으로 모델링하고 각 슬라이스에 비선형 Hertz 접촉 스프링을 커플링합니다. Gen1이 무시하는 롤러 굽힘 변형을 반영하여 더 정확한 하중 분포를 계산합니다.

## 핵심 방정식

```
[K_beam]{w} + {f_contact(δ)} = 0
δ_k = δ_rigid - w_k - Δz_total_k
```

- `K_beam`: 2n×2n Timoshenko beam 강성 행렬
- `w_k`: 빔 굴곡 변위 [μm] (강체 모드 제외)
- `δ_rigid`: 강체 접근량 [μm]
- `Δz_total_k`: 프로파일 보정 [μm]

## 모듈 구조

### beam.rs

| 함수 | 설명 |
|------|------|
| `beam_section_properties(r)` | I [mm⁴], A [mm²] 계산 |
| `beam_element_stiffness(E, I, A, G, κ, L, type)` | 4×4 Timoshenko/Euler-Bernoulli 요소 강성 |
| `assemble_beam_stiffness(slices, material, type)` | 2n×2n 글로벌 강성 행렬 조립 |

### gen3.rs

| 함수 | 설명 |
|------|------|
| `solve_gen3_roller(slices, δ_rigid, material, params)` | 단일 δ_rigid에 대한 빔-접촉 커플링 해석 |
| `solve_gen3_for_load(slices, Q_target, material, params)` | 목표 하중에 대한 δ_rigid 탐색 |

## 알고리즘

### Newton-Raphson with Active Set

1. K_beam 조립 (상수)
2. w = 0 초기화
3. 외부 루프 (active set 변화 감지, 최대 15회):
   - 내부 N-R 루프:
     a. gap_k = δ_rigid - w[2k] - Δz_total_k
     b. Active set: {k | gap_k > 0}
     c. F_contact, K_contact 계산
     d. Residual: R = K_beam·(w/1000) - F_contact
     e. Jacobian: J = K_beam/1000 + diag(K_c)
     f. Δw = -J⁻¹·R
     g. w += Δw
     h. **강체 모드 제거** (핵심!)
     i. 수렴 판정: ||R|| / max(||F||, 1) < tol

### 강체 모드 제거 (Rigid Body Mode Projection)

자유-자유 빔의 K_beam은 singular (강체 변위, 강체 회전). δ_rigid가 이미 전체 접근량을 담당하므로, w에서 강체 성분을 매 반복마다 제거:

```
w_mean = (1/n) Σ w[2k]           # 평균 변위
slope = Σ(w[2k]·xc) / Σ(xc²)    # 선형 추세 (회전)
w[2k] -= w_mean + slope·xc       # 굴곡 성분만 유지
```

### 목표 하중 탐색

Gen1과 동일한 외부 Newton-Raphson:
- δ_rigid 조정으로 Q_total → Q_target 수렴
- 수치 미분: dQ/dδ (perturbation = 0.01 μm)

## Tauri 커맨드

### `solve_roller_gen3`
- 입력: `BearingInput`, `delta_rigid: f64`
- 출력: `Gen3Result` (slice_results, q_total, delta_rigid, beam_deflection, max_deflection)

### `solve_roller_gen3_for_load`
- 입력: `BearingInput`, `q_target: f64`
- 출력: `Gen3Result`

## 설계 결정

1. **Dense matrix (nalgebra DMatrix)**: n≤200 DOF에서 sparse 불필요, 코드 단순성 우선
2. **Free-free beam + rigid body projection**: 접촉 스프링이 구속 제공, 강체 모드는 수학적으로 제거
3. **Fallback**: 접촉 슬라이스 < 2개면 Gen1 동등 결과 반환
4. **Timoshenko 전단 보정**: κ = 10/(9+10ν) (원형 단면)

## 검증

| 테스트 | 검증 내용 |
|--------|-----------|
| Level C | Flat profile + zero misalignment → Gen3 ≈ Gen1 (< 3% 차이) |
| 빔 요소 | 캔틸레버 해석해 대비 < 1% 오차 |
| Timoshenko | 전단 효과로 Euler-Bernoulli 대비 더 큰 처짐 |
| 하중 분포 | Gen3이 Gen1보다 부드러운 분포 (빔 재분배 효과) |
