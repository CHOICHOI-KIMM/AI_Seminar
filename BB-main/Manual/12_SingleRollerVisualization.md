# Chapter 12: Single Roller Contact Visualization

## 12.1 Overview

Stress Contour 탭에서 개별 롤러를 선택하면 해당 롤러의 접촉 해석 결과를 3가지 뷰로 상세 확인할 수 있다. 베어링 전체 히트맵에서는 볼 수 없는 슬라이스 레벨의 상세 정보를 시각화한다.

## 12.2 Roller Selector

Stress Contour 탭 상단의 드롭다운에서 롤러를 선택한다.

- **Bearing Overview**: 기존 베어링 전체 히트맵 (기본값)
- **Roller #N**: 하중이 걸린(Q > 0) 롤러만 선택 가능
- 각 항목에 ψ(각도 위치)와 Q(법선 하중) 표시

## 12.3 Distributions View (방안 1)

선택 롤러의 슬라이스별 분포를 2×2 그리드로 표시한다.

### 12.3.1 Contact Stress p_max
- **X축**: 축방향 위치 [mm]
- **Y축**: 최대 접촉 응력 p_max [MPa]
- Inner(파란선)/Outer(노란선) 레이스웨이 동시 표시
- 엣지 응력 상승(edge stress rise) 패턴 확인에 유용

### 12.3.2 Contact Half-Width b_k
- **X축**: 축방향 위치 [mm]
- **Y축**: 접촉 반폭 b [mm]
- Inner/Outer 동시 표시
- R_eq 차이에 의한 내/외륜 접촉폭 차이 확인

### 12.3.3 Line Load q_k
- **X축**: 축방향 위치 [mm]
- **Y축**: 선압 q [N/mm]
- 바 차트, 응력 수준별 색상 그라데이션
- Gen1: 크라운 프로파일에 의한 포물선형, Gen3: 빔 커플링 효과로 재분배

### 12.3.4 Approach δ_k
- **X축**: 축방향 위치 [mm]
- **Y축**: 접근량 δ [μm]
- 영역 채움(fill) 차트
- Gen1: δ_rigid − Δz_total(k), Gen3: 빔 변형 반영

## 12.4 Contact Patch View (방안 2)

롤러 표면의 2D 접촉 압력 분포를 히트맵으로 시각화한다.

### 12.4.1 Hertz Pressure Distribution

각 슬라이스의 접촉 폭 방향으로 Hertz 반타원 압력 분포를 재구성:

$$
p(y) = p_{max,k} \sqrt{1 - \left(\frac{y}{b_k}\right)^2}
$$

- **X축**: 축방향 슬라이스 위치 [mm]
- **Y축**: 접촉 폭 방향 [mm] (−b_max ~ +b_max)
- **색상**: 접촉 압력 [MPa] (viridis 컬러 스케일)

### 12.4.2 Contact Boundary

접촉 경계(b_k 윤곽선)를 흰색 점선으로 오버레이 표시한다. 크라운 프로파일에 따라 접촉 패치의 형상이 달라진다:

- **Flat profile**: 사각형 접촉 (b_k ≈ 균일)
- **Parabolic/Logarithmic**: 타원형 접촉 (중앙 b_k 최대, 양단 감소)
- **Partial contact**: 비접촉 슬라이스에서 접촉 경계가 끊어짐

### 12.4.3 Inner/Outer Toggle

Raceway 토글(Inner/Outer)로 전환 가능. 외륜은 R_eq_outer > R_eq_inner이므로 동일 하중에서 접촉폭이 더 넓고 p_max가 낮다.

## 12.5 Gen1 vs Gen3 Comparison View (방안 3)

Dual 모드로 해석한 경우에만 활성화. 동일 롤러에 대해 Gen1(독립 슬라이스)과 Gen3(빔 커플링) 결과를 오버레이한다.

### 12.5.1 Summary Bar

상단에 주요 비교 지표를 한 줄로 표시:
- Q [N]: Gen1 → Gen3 (변화율 %)
- p_max,i [MPa]: Gen1 → Gen3 (변화율 %)
- p_max,o [MPa]: Gen1 → Gen3 (변화율 %)

### 12.5.2 Overlay Charts (2×2)

| 차트 | 의미 |
|------|------|
| p_max Inner | Gen1(파선) vs Gen3(실선) 내륜 접촉 응력 |
| p_max Outer | Gen1(파선) vs Gen3(실선) 외륜 접촉 응력 |
| q_k | Gen1 vs Gen3 선압 분포 — 빔 커플링 효과 확인 |
| δ_k | Gen1 vs Gen3 접근량 — 빔 변형에 의한 재분배 확인 |

### 12.5.3 Interpretation

- **Gen3 효과가 큰 경우**: 엣지 응력이 Gen1 대비 크게 변함 → 크라운 최적화 필요
- **Gen3 효과가 작은 경우**: 독립 슬라이스 가정이 유효 → Gen1으로 충분
- **일반적 경향**: Gen3은 엣지 슬라이스의 하중을 중앙으로 재분배하여 p_max를 낮추는 경향

## 12.6 구현 파일

| 파일 | 역할 |
|------|------|
| `src/components/charts/StressContourChart.tsx` | 롤러 선택 UI 통합, 뷰 전환 |
| `src/components/charts/RollerDetailChart.tsx` | Distributions 뷰 (2×2 차트) |
| `src/components/charts/ContactPatchChart.tsx` | Contact Patch 히트맵 |
| `src/components/charts/RollerComparisonChart.tsx` | Gen1 vs Gen3 비교 오버레이 |
