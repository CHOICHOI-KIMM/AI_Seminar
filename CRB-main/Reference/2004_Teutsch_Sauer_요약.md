# Teutsch & Sauer (2004) — Alternative Slicing Technique 핵심 요약

**원제:** An Alternative Slicing Technique to Consider Pressure Concentrations in Non-Hertzian Line Contacts
**저자:** Roman Teutsch, Bernd Sauer (Kaiserslautern 대학)
**출처:** ASME Journal of Tribology, Vol. 126, July 2004

---

## 1. 논문의 목적

롤러 베어링의 **롤러-레이스웨이 접촉 해석**을 위한 빠르고 정확한 방법 제안.
기존 슬라이싱 기법의 한계(슬라이스 간 상호작용 무시 → 압력 집중 표현 불가)를 극복.

---

## 2. 기존 하중-변위 관계식 비교

논문에서 검토한 주요 관계식:

| 저자 | 특징 | 명시적 풀이 가능 |
|------|------|:---:|
| **Lundberg (1939)** | 최적 프로파일 가정, 축방향 균일 압력 | X |
| **Dinnik** | 포물선 압력분포, 반무한체 | X |
| **Kowalsky (1940)** | 타원 압력분포 | X |
| **Palmgren (1959)** | 실험식, 롤러 직경 무관 | O |
| **Kunert (1961)** | 최적 프로파일 가정 | O |
| **Tripp (1985)** | Houpert가 최선으로 평가 | X |
| **Houpert (2001)** | Tripp 기반 curve-fit, 기하 보정항 포함 | O |

**핵심 문제:** Tripp/Dinnik/Kowalsky 식이 가장 정확하나, $Q$를 $\delta$로부터 직접 구할 수 없음 (반복 계산 필요).

---

## 3. 제안된 방법 (AST: Alternative Slicing Technique)

### 3.1 새로운 하중-변위 curve-fit

Dinnik 식을 기반으로 **명시적으로 풀 수 있는** 새 관계식 도출:

$$\delta_i = 3.17 \cdot \left(\frac{d_m}{2}\right)^{0.08} \cdot \left(\frac{Q(1-\nu^2)}{EL}\right)^{0.92} \quad \text{(내륜)}$$

$$\delta_o = 2.66 \cdot \left(\frac{t}{1+D/d_m}\right)^{0.09} \cdot \left(\frac{Q(1-\nu^2)}{EL}\right)^{0.91} \quad \text{(외륜)}$$

- FEM 대비 **최대 오차 1.5% 미만**
- 롤러-평판 접촉 시 롤러 직경에 무관 (Palmgren 결과 확인), 단 **판 두께 $t$에 의존**

### 3.2 영향계수 행렬 (핵심 아이디어)

기존 슬라이싱: 각 슬라이스가 **독립적** → 압력 집중 표현 불가

**AST의 핵심:** 반무한체 이론과 유사하게, 슬라이스 $j$의 힘이 **인접 슬라이스 $k$의 변형에도 기여**

$$[S_w] \cdot \{q\} = \{\Delta\}$$

- $[S_w]$: 가중 영향계수 행렬 (대칭, $n \times n$)
- $\{q\}$: 각 슬라이스 하중 벡터
- $\{\Delta\}$: 변위 벡터 ($\delta_j^{1/ex}$)

### 3.3 가중함수

Singh & Paul (1974)의 아이디어 차용 — 영향은 거리에 반비례:

$$w_{j,k} = \left(\frac{1}{r_{j,k}}\right)^{1/ex} \quad (j \neq k)$$

$$w_{j,k} = \left(\frac{4}{l}\right)^{1/ex} \quad (j = k)$$

### 3.4 풀이 방법

- 가우스 소거법 등 표준 선형대수 방법으로 풀이
- **반복 계산은 접촉 영역 판정(슬라이스가 접촉에서 이탈하는 경우)에만 필요**
- 미정렬/프로파일링은 각 슬라이스별 초기 관입량 차이로 반영

---

## 4. 검증 결과

| 검증 항목 | 결과 |
|-----------|------|
| FEM 대비 총 접촉력 | 매우 좋은 일치 (슬라이스 수 30~100개로 충분) |
| FEM 대비 압력분포 | 좋은 일치, 약간 "barrel" 형태 (de Mul 등도 동일 관찰) |
| 프로파일 롤러 | Lundberg 프로파일 적용 시 단부 압력 집중 현저히 감소 |
| 미정렬 롤러 | de Mul 등의 반무한체 모델 및 실험 결과와 잘 일치 |
| 계산 속도 | Pentium 200MHz에서 단일 접촉 수 분의 1초 |

---

## 5. 주요 결론

1. 슬라이스 간 **상호 영향을 고려**하여 압력 집중을 표현할 수 있는 개선된 슬라이싱 기법 제안
2. Dinnik 식 기반 **명시적 curve-fit**으로 반복 계산 없이 하중 ↔ 변위 변환 가능
3. 30~100개 슬라이스로 FEM 수준의 정확도 달성
4. 프로파일링, 미정렬 모두 적용 가능
5. **빠른 계산 속도** → 베어링 정적 하중분포 계산, 동적 시뮬레이션에 적합

---

## 6. TRB 프로젝트와의 관련성

| AST 개념 | TRB 프로젝트 매핑 |
|----------|-------------------|
| 독립 슬라이스 (기존 방식) | **Gen1 솔버** — 슬라이스 간 독립, $O(n)$ |
| 영향계수 행렬 (AST) | **Gen3 솔버와 유사한 접근** — 슬라이스 간 커플링 |
| Dinnik 기반 curve-fit | `hertz.rs`의 하중-변위 관계에 활용 가능 |
| 가중함수 $w_{j,k}$ | Gen3의 Timoshenko 빔 강성 행렬과 역할 유사 (변형 전파) |
| 접촉 지수 0.91~0.92 | Palmgren의 10/9 ≈ 1.111 지수와 비교 검토 필요 |

**핵심 차이점:** AST는 영향계수 행렬로 커플링을 표현하고, Gen3는 Timoshenko 빔 FE로 커플링을 표현. AST가 더 단순하지만, Gen3가 물리적으로 더 엄밀.
