# SKF GBLM 표면손상 함수 I_S의 결정 방법론: Morales-Espejel 모델의 미지 상수 문제와 TRB 마이크로피팅 표면손상 수명 계산

---

## 목차

1. [서론 — 문제 정의: $I_S$ 미지 상수 문제](#1-서론--문제-정의-is-미지-상수-문제)
2. [GBLM 이론 구조와 $I_S$의 정의·위치](#2-gblm-이론-구조와-is의-정의위치)
3. [$I_S$ 근본 수식과 미지 상수의 식별](#3-is-근본-수식과-미지-상수의-식별)
4. [마이크로피팅 물리 모델과 표면 적분 손상의 수치 절차](#4-마이크로피팅-물리-모델과-표면-적분-손상의-수치-절차)
5. [미지 상수 결정·교정(Calibration) 방법론](#5-미지-상수-결정교정calibration-방법론)
6. [TRB 적용과 대안 표면피로 수명 모델 비교](#6-trb-적용과-대안-표면피로-수명-모델-비교)
7. [결론 및 실무 권고](#7-결론-및-실무-권고)
8. [참고문헌](#참고문헌)

---

## 1. 서론 — 문제 정의: $I_S$ 미지 상수 문제

---

### 1.1 연구 배경: 표면 기인 손상의 지배성과 GBLM의 등장

#### 현대 베어링 운영 환경의 변화

롤링 베어링의 수명 예측은 20세기 중반 Lundberg–Palmgren(L-P) 이론에서 출발하였다. L-P 모델은 헤르츠 접촉 응력장 내 최대 전단응력이 재료 내부(subsurface)에서 피로 균열을 개시한다는 가정에 기반하며, 이후 Ioannides–Harris(I-H) 모델이 피로한계 응력 $\sigma_0$를 도입하여 ISO 281의 수명 수정계수 $a_{\mathrm{ISO}}$(및 SKF 계열 수명 수정계수)의 이론적 기초가 되었다. 그러나 이들 고전 모델은 공통적으로 **표면이 완전히 매끄럽고 윤활막이 충분히 두꺼운** 이상적 조건을 전제한다.

현대의 베어링 운영 환경은 이 전제를 근본적으로 위협한다. 풍력 발전기 메인 샤프트, 전기차 구동 모터, 고속 공작기계 스핀들 등에서는 다음과 같은 조건이 복합적으로 작용한다:

- **혼합 윤활(mixed lubrication)**: 막두께비 $\lambda = h_{\min}/\sigma_{\text{composite}} < 1$로 asperity 직접 접촉 빈발
- **고슬라이딩 비율**: 롤링-슬라이딩 접촉에서 슬라이딩 속도 성분 증가
- **오염 입자 및 표면 결함**: 압흔(indentation), 스크래치, 마이크로피팅 등 표면 손상 선행

이러한 조건에서는 subsurface 피로보다 **표면 기인 손상(surface-initiated damage)**이 먼저 발생하여 베어링 수명을 지배한다. Morales-Espejel은 인터뷰 및 웨비나에서 이를 명확히 표현하였다:

> *"대부분의 베어링 실패는 재료 내부가 아니라 표면에서 발생한다."* [ref_16]

이 인식은 단순한 경험적 관찰이 아니라, 수십 년간의 실험 데이터와 수치 시뮬레이션이 뒷받침하는 공학적 사실이다 [ref_01, ref_08].

#### GBLM의 등장과 이중 손상 구조

표면 기인 손상을 정량적으로 수명 예측에 통합하기 위해 SKF는 **일반화 베어링 수명 모델(Generalized Bearing Life Model, GBLM)**을 개발하였다 [ref_01]. GBLM의 핵심 혁신은 베어링 수명을 두 개의 독립적 실패 모드로 분리하는 것이다:

| 실패 모드 | 손상 기원 | 지배 응력 | 손상 함수 |
|-----------|-----------|-----------|-----------|
| 모드 1 (표면) | 표면층 asperity 접촉 | 근표면 접촉 응력 $\sigma_{\text{contact}}$ | $I_S$ (surface damage integral) |
| 모드 2 (부표면) | 헤르츠 응력장 내부 | 최대 전단응력 $\tau_{\max}$ | $I_B$ (subsurface stress integral) |

두 모드의 생존 확률 $S_1$, $S_2$는 독립적으로 계산된 후 결합된다:

$$S_{\text{total}} = S_1 \cdot S_2$$

여기서 $S_1$은 표면 손상 함수 $I_S$에, $S_2$는 부표면 응력 적분 $I_B$에 각각 의존한다 [ref_01, ref_11, ref_12]. 이 이중 구조는 GBLM이 고전 L-P/I-H 모델과 근본적으로 구별되는 지점이며, 표면 손상이 지배적인 현대 운영 조건에서의 수명 예측 정확도를 크게 향상시킨다 [ref_08].

Morales-Espejel의 2023년 단행본 [ref_08]은 GBLM의 이론적 토대를 종합하며, 마이크로피팅(micropitting), 압흔 손상, 열 손상 등 다양한 표면 손상 모드를 통합적으로 다룬다. SKF의 공식 기술 자료 [ref_11, ref_12]는 이 모델을 하이브리드 베어링(세라믹 롤링 요소 + 강 레이스) 및 표준 강 베어링 모두에 적용 가능한 형태로 제시한다.

---

### 1.2 핵심 문제의식: $I_S$ 근본 수식·표면 적분 정의식의 미지 상수

#### 문제의 이중 구조

GBLM에서 표면 손상 함수 $I_S$는 수명 예측의 핵심 변수이지만, 그 계산에는 두 가지 층위의 미지 상수 문제가 존재한다. 이 두 난점은 서로 연관되어 있으나 개념적으로 구별되어야 한다.

**난점 (a): $I_S$ 근본 수식의 미지 상수**

GBLM에서 표면 손상 위험도는 정규화된 표면 손상 위험 파라미터 $R_s$로 표현된다 [ref_11]. 이 파라미터는 다음과 같은 **근본 수식(fundamental formula)** 구조를 갖는다:

$$I_S \propto \left(\frac{\sigma_{s}}{\sigma_{s,\text{ref}}}\right)^k \cdot f\!\left(\frac{\sigma_s}{\sigma_{s,\text{limit}}}\right)^n$$

여기서:
- $\sigma_{s,\text{ref}}$: 참조 표면 응력 (reference surface stress) — **미공개 상수**
- $\sigma_{s,\text{limit}}$: 표면 피로한계 응력 (surface fatigue limit stress) — **미공개 상수**
- $k$, $n$: 응력-수명 지수 (stress-life exponents) — **미공개 상수**

이 상수들은 SKF의 내부 교정(calibration) 데이터에 기반하며 공개 문헌에 명시되지 않는다 [ref_01, ref_11]. 따라서 외부 연구자가 $I_S$를 독립적으로 계산하려면 이 상수들을 별도로 결정해야 한다.

**난점 (b): $I_S$ 표면 적분 정의식의 미지 상수**

$I_S$의 물리적 정의는 접촉 영역에 걸친 표면 응력의 적분으로 표현된다 [ref_04]:

$$I_S = \int_A \left(\sigma_{\text{contact}} - \sigma_{\text{limit}}\right)^n \, dA$$

> **[주석]** 이 식은 표면층 두께를 축약한 등가 면적 표현(dA)이다. 기본 정의는 체적 적분 $I_S = \int_V (\sigma_{\text{contact}} - \sigma_{\text{limit}})^n \, dV$ [ref_04][ref_67]이며, dA 형태는 표면층 두께 또는 손상 커널을 적분 방향으로 흡수한 축약 표현이다. 단위 일관성을 위해 dA 표현 사용 시 표면층 두께 또는 정규화 여부(무차원화 등)를 명시해야 한다.

이 식에서도:
- $\sigma_{\text{limit}}$: 표면 접촉 피로한계 — **미공개**
- $n$: 적분 지수 — **미공개**
- 적분 영역 $A$의 깊이 범위 — **부분적으로 미공개**

가 문제가 된다. 특히 $\sigma_{\text{limit}}$는 재료 및 윤활 조건에 따라 달라지는 물리적 임계값이지만, SKF 공개 자료에서는 그 수치가 명시되지 않는다 [ref_11, ref_12].

#### $S_R$ 파라미터와 $I_S$의 위치 해석

최근 연구 [ref_74]는 이 문제에 대한 중요한 시각을 제공한다. 기어 접촉 피로 수명 모델에서 **표면 위험 파라미터** $S_R$을 다음과 같이 정의한다:

$$S_R = \frac{\text{near-surface stress integral}}{\text{total stress integral}} = \frac{I_S}{I_S + I_B}$$

이 정의는 $I_S$가 전체 응력 적분에서 표면층이 차지하는 **상대적 비율**로 해석될 수 있음을 시사한다. $S_R$이 1에 가까울수록 표면 기인 실패가 지배적이며, 0에 가까울수록 부표면 피로가 지배적이다. 이 파라미터는 $I_S$의 절대값 계산 없이도 표면/부표면 실패 모드의 상대적 위험도를 평가할 수 있는 실용적 도구를 제공한다 [ref_74].

#### 정규화 표면 손상 위험도 $R_s$

SKF의 공식 GBLM 기술 자료 [ref_11, ref_12]에서는 $I_S$를 직접 노출하는 대신, **정규화 표면 손상 위험도(normalized surface damage risk)** $R_s$를 사용한다:

$$R_s = \frac{I_S}{I_{S,\text{ref}}}$$

여기서 $I_{S,\text{ref}}$는 기준 조건(reference condition)에서의 표면 손상 적분값이다. 이 정규화는 미지 상수의 일부를 상쇄시키는 효과가 있으나, 기준 조건 자체가 SKF 내부 데이터에 의존하므로 근본적인 미지 상수 문제를 해결하지는 못한다 [ref_11].

#### 문제의 실용적 함의

이 미지 상수 문제는 단순한 학술적 불완전성이 아니라 실질적인 공학 문제를 야기한다:

1. **독립 검증 불가**: 외부 연구자나 엔지니어가 SKF GBLM을 독립적으로 구현하여 결과를 검증할 수 없다.
2. **TRB 마이크로피팅 수명 계산 불가**: 테이퍼 롤러 베어링(Tapered Roller Bearing, TRB)의 마이크로피팅 조건에서 $I_S$를 직접 계산하려면 미지 상수의 결정이 선행되어야 한다.
3. **대안 모델 비교 불가**: Morales-Espejel & Brizmer의 마이크로피팅 물리 모델 [ref_08]과 GBLM $I_S$ 접근법을 정량적으로 비교하려면 동일한 상수 기반이 필요하다.

이러한 문제의식이 본 보고서의 출발점이다.

---

### 1.3 연구 목적, 범위, 보고서 구성

#### 연구 목적

본 보고서의 목적은 다음 세 가지로 요약된다:

1. **$I_S$ 미지 상수의 결정 방법론 체계화**: SKF GBLM의 표면 손상 함수 $I_S$에 포함된 미지 상수($\sigma_{s,\text{ref}}$, $\sigma_{s,\text{limit}}$, $k$, $n$)를 결정하기 위한 이론적·수치적 방법론을 체계적으로 정리한다.

2. **TRB 마이크로피팅 수명 계산 절차 수립**: 테이퍼 롤러 베어링의 마이크로피팅 조건에서 $I_S$를 계산하고 표면 손상 수명을 예측하는 구체적인 수치 절차를 제시한다.

3. **대안 모델과의 비교 및 권고**: Morales-Espejel & Brizmer의 마이크로피팅 물리 모델 [ref_08]과 GBLM $I_S$ 접근법을 비교하고, 실용적 적용을 위한 권고 사항을 도출한다.

#### 연구 범위

본 보고서의 범위는 다음과 같이 한정된다:

- **대상 베어링**: 테이퍼 롤러 베어링(TRB), 단열 및 복열 구성
- **손상 모드**: 마이크로피팅(micropitting) 중심; 압흔 손상 및 열 손상은 비교 목적으로만 언급
- **윤활 조건**: 혼합 윤활 영역($0.5 \leq \lambda \leq 3$)
- **재료**: 표준 베어링강(52100 등); 하이브리드 베어링은 비교 목적으로 참조

#### 보고서 구성

본 보고서는 다음과 같은 흐름으로 구성된다:

| 섹션 | 제목 | 핵심 내용 |
|------|------|-----------|
| **2** | GBLM 이론 구조 | 이중 손상 모델, 생존 확률 결합, $I_S$·$I_B$ 정의 |
| **3** | $I_S$ 근본 수식 분석 | 미지 상수의 물리적 의미, 차원 분석, 교정 전략 |
| **4** | 물리 모델 및 수치 절차 | 마이크로피팅 응력 계산, DC-FFT, asperity 응력 사이클 |
| **5** | 상수 교정 방법론 | 실험 데이터 역해석, 민감도 분석, 불확실도 정량화 |
| **6** | TRB 적용 및 대안 비교 | TRB 기하학, $I_S$ 계산 결과, 대안 모델 비교 |
| **7** | 권고 및 결론 | 실용적 권고, 한계, 향후 연구 방향 |

**섹션 2**에서는 GBLM의 이론 구조를 수식 수준에서 상세히 전개하며, $I_S$와 $I_B$의 정의식 및 이들이 수명 예측에 통합되는 방식을 다룬다. **섹션 3**은 본 보고서의 핵심으로, $I_S$ 근본 수식에 포함된 미지 상수들의 물리적 의미를 분석하고 이를 결정하기 위한 이론적 틀을 제시한다. **섹션 4**는 마이크로피팅 물리 모델과 수치 계산 절차를 상세히 기술하며, **섹션 5**는 실험 데이터를 이용한 상수 교정 방법론을 다룬다. **섹션 6**은 TRB에 대한 구체적 적용 사례와 대안 모델과의 정량적 비교를 제시하고, **섹션 7**에서 실용적 권고와 결론으로 마무리한다.

이 구성은 이론 구조 → 근본 수식 분석 → 물리 모델·수치 절차 → 상수 교정 → TRB 적용·대안 비교 → 권고의 논리적 흐름을 따르며, 트라이볼로지 전문가가 본 보고서를 독립적인 계산 지침서로 활용할 수 있도록 설계되었다.

---

---

## 2. GBLM 이론 구조와 $I_S$의 정의·위치

SKF Generalized Bearing Life Model(GBLM)은 구름 베어링의 피로 수명을 **표면 기원(surface-initiated)** 과 **부표면 기원(subsurface-initiated)** 두 메커니즘으로 분리하여 각각의 생존확률을 독립적으로 산출한 뒤 결합하는 구조를 취한다 [ref_01][ref_04]. 이 절에서는 그 수학적 골격을 전개하고, 표면손상 함수 $I_S$가 모델 내에서 차지하는 위치와 정의를 상세히 기술한다.

---

### 2.1 표면/부표면 생존확률의 분리와 Weibull 결합

#### 2.1.1 전체 생존확률의 곱 분해

구름 베어링의 전체 생존확률 $S$는 표면 손상 메커니즘에 의한 생존확률 $S_\text{surf}$와 부표면 피로에 의한 생존확률 $S_\text{sub}$의 **독립 사건 곱**으로 표현된다 [ref_01][ref_03][ref_04]:

$$
S = S_\text{surf} \cdot S_\text{sub}
$$

이를 로그 변환하면 두 메커니즘의 위험도가 **가산적(additive)** 으로 결합됨이 명확해진다:

$$
\ln\!\left(\frac{1}{S}\right) = \ln\!\left(\frac{1}{S_\text{surf}}\right) + \ln\!\left(\frac{1}{S_\text{sub}}\right)
\tag{2.1}
$$

이 분해는 Weibull 통계의 경쟁 위험(competing risk) 프레임워크에 직접 대응한다. 두 메커니즘이 서로 독립적으로 작동한다는 가정 하에, 어느 한 메커니즘이 먼저 파손을 일으키면 전체 수명이 결정된다 [ref_08].

#### 2.1.2 각 메커니즘의 Weibull 표현

각 생존확률은 Weibull 분포로 기술된다. 부표면 피로에 대해서는 Lundberg-Palmgren(L-P) 형식을 계승하여:

$$
\ln\!\left(\frac{1}{S_\text{sub}}\right) = A_\text{sub} \cdot I_B^{e/c_\text{sub}} \cdot N^{e_\text{sub}}
\tag{2.2}
$$

표면 손상에 대해서는 별도의 Weibull 기울기(Weibull slope) $e_\text{surf}$를 적용한다 [ref_75]:

$$
\ln\!\left(\frac{1}{S_\text{surf}}\right) = A_\text{surf} \cdot I_S^{n/c_\text{surf}} \cdot N^{e_\text{surf}}
\tag{2.3}
$$

여기서 $N$은 응력 사이클 수, $A_\text{surf}$, $A_\text{sub}$는 재료·윤활 조건에 의존하는 계수이다. **표면과 부표면 메커니즘은 서로 다른 Weibull 기울기를 가진다**는 점이 GBLM의 핵심 가정 중 하나이다 [ref_75]. 부표면 피로의 Weibull 기울기는 L-P 모델에서 유래한 $e \approx 10/9$인 반면, 표면 손상의 기울기는 마이크로피팅·마모 메커니즘의 특성을 반영하여 별도로 교정된다 [ref_01][ref_05].

#### 2.1.3 표면층 적분 깊이

표면 손상 메커니즘의 위험 적분은 **접촉 표면으로부터 유한한 깊이**까지의 체적에 대해 수행된다. 이 특성 깊이는 접촉 반경(Hertz 반접촉폭 $a$)의 **0.5–1배** 범위로 설정된다 [ref_75]. 이는 마이크로피팅 균열이 표면 근방의 얕은 층에서 개시·전파된다는 실험적 관찰에 근거한다 [ref_08]. 반면 부표면 피로의 특성 깊이는 최대 전단응력 위치($z \approx 0.47a$, Hertz 이론)에 대응하여 더 깊은 층을 포함한다. **[주석: 여기서 0.5–1.0a는 표면층 적분 영역의 특성 깊이 지표이며, z ≈ 0.47a는 부표면 최대전단 위치의 특성 깊이 지표이다. 두 값 모두 모델 정의 및 접촉 형상에 따라 달라지므로, 특성 깊이 지표와 실제 적분 영역은 모델 정의에 따라 다르다.]**

| 파라미터 | 표면 손상 ($I_S$) | 부표면 피로 ($I_B$) |
|---|---|---|
| 적분 깊이 | $0.5a \sim 1.0a$ | $\sim 0.47a$ 이하 전층 |
| 지배 응력 | 접촉 압력 $\sigma_\text{contact}$ | 최대 전단응력 $\tau_\text{max}$ |
| Weibull 기울기 | $e_\text{surf}$ (별도 교정) | $e \approx 10/9$ (L-P 계승) |
| 한계 응력 | $\sigma_\text{limit}$ | $\tau_0 \approx 276\ \text{MPa}$ |

*표 2.1: 표면 손상과 부표면 피로 메커니즘의 주요 파라미터 비교 [ref_04][ref_67][ref_75]*

#### 2.1.4 소결

식 (2.1)의 로그 가산 구조는 GBLM이 단일 Weibull 분포로 전체 수명을 기술하는 고전 L-P 모델과 근본적으로 다름을 보여준다. 표면 손상이 지배적인 조건(혼합 윤활, 오염 환경)에서는 $\ln(1/S_\text{surf})$ 항이 전체 위험도를 지배하며, 이 항의 크기를 결정하는 것이 바로 $I_S$이다 [ref_01][ref_11].

---

### 2.2 응력 위험 적분(stress risk integral) 체계 내에서 $I_S$의 정의

#### 2.2.1 $I_S$의 수학적 정의

표면손상 함수 $I_S$는 접촉 응력이 재료의 한계 응력을 초과하는 **표면층 체적에 대한 거듭제곱 적분**으로 정의된다 [ref_04][ref_67]:

$$
\boxed{I_S = \iiint_{\Omega_\text{surf}} \bigl(\sigma_\text{contact} - \sigma_\text{limit}\bigr)^n \, dV}
\tag{2.4}
$$

여기서:
- $\Omega_\text{surf}$: 표면층 적분 영역 (깊이 $0.5a \sim 1.0a$까지의 체적)
- $\sigma_\text{contact}$: 접촉 응력 (일반적으로 최대 Hertz 압력 $p_0$ 또는 국소 접촉 압력)
- $\sigma_\text{limit}$: 재료의 표면 피로 한계 응력 (임계값; $\sigma_\text{contact} < \sigma_\text{limit}$인 영역은 적분에 기여하지 않음)
- $n$: 표면 손상 지수 (Weibull 기울기와 연관; 통상 $n = 4 \sim 6$) [ref_67]

피적분 함수 $(\sigma_\text{contact} - \sigma_\text{limit})^n$은 **한계 응력 초과분**에만 비선형적으로 반응하는 구조로, 접촉 압력이 $\sigma_\text{limit}$를 넘지 않으면 $I_S = 0$이 되어 표면 손상 위험이 없음을 의미한다. 이는 피로 한계(fatigue limit) 개념을 표면 손상 모델에 명시적으로 도입한 것이다 [ref_04][ref_08].

#### 2.2.2 부표면 위험 적분 $I_B$와의 병치

부표면 피로에 대응하는 위험 적분 $I_B$는 다음과 같이 정의된다 [ref_04][ref_67]:

$$
I_B = \iiint_{\Omega_\text{sub}} \bigl(\tau_\text{shear} - \tau_0\bigr)^e \, dV
\tag{2.5}
$$

여기서:
- $\Omega_\text{sub}$: 부표면 적분 영역
- $\tau_\text{shear}$: 부표면 최대 전단응력 (Lundberg-Palmgren 모델의 정의에 따름)
- $\tau_0$: 부표면 피로 한계 전단응력 ($\tau_0 \approx 276\ \text{MPa}$, ISO 281 기반) [ref_67]
- $e$: 부표면 피로 지수 ($e \approx 4$, L-P 모델 계승) [ref_67]

$I_S$와 $I_B$를 병치하면 두 적분의 구조적 유사성과 차이가 명확해진다:

| 항목 | $I_S$ (표면) | $I_B$ (부표면) |
|---|---|---|
| 적분 응력 | $\sigma_\text{contact}$ (접촉 압력) | $\tau_\text{shear}$ (전단응력) |
| 한계 응력 | $\sigma_\text{limit}$ | $\tau_0 \approx 276\ \text{MPa}$ |
| 지수 | $n = 4 \sim 6$ | $e \approx 4$ |
| 적분 영역 | 표면층 ($0.5a \sim 1.0a$) | 부표면 전층 |
| 물리적 의미 | 표면 접촉 피로·마이크로피팅 | 부표면 전위 이동·스폴링 |

*표 2.2: 표면 위험 적분 $I_S$와 부표면 위험 적분 $I_B$의 비교 [ref_04][ref_67]*

두 적분 모두 **한계값 초과분의 거듭제곱 적분**이라는 동일한 수학적 틀을 공유하지만, 지배 응력 성분과 적분 영역이 다르다. 이 구조는 Tallian-Chiu(1967)의 원형 모델에서 직접 계승된 것이다 [ref_67].

#### 2.2.3 표면 위험 파라미터 $S_R$과 $R_s$

$I_S$의 물리적 의미를 직관적으로 파악하기 위해 두 가지 정규화 파라미터가 사용된다.

**표면 위험 파라미터 $S_R$** (ref_74): 전체 응력 적분에 대한 표면 근방 응력 적분의 비율로 정의된다 [ref_74]:

$$
S_R = \frac{I_S^\text{near-surface}}{I_S^\text{total}}
\tag{2.6}
$$

$S_R$은 0에서 1 사이의 값을 가지며, $S_R \to 1$이면 피로 손상이 표면 기원으로 지배되고, $S_R \to 0$이면 부표면 기원이 지배적임을 나타낸다 [ref_74]. 이 파라미터는 기어 및 베어링의 표면/부표면 피로 상대 위험도를 정량적으로 비교하는 데 활용된다.

**정규화 표면 손상 위험 $R_s$** (ref_11): SKF GBLM 공식 문서에서 사용되는 표기로, $I_S$를 기준 조건의 위험 적분으로 정규화한 값이다 [ref_11]:

$$
R_s = \frac{I_S}{I_{S,\text{ref}}}
\tag{2.7}
$$

$R_s$는 실제 운전 조건의 표면 손상 위험을 기준 조건 대비 상대적으로 표현하며, GBLM 수명 수정 계수 $a_\text{SKF}$ 계산의 입력값으로 사용된다 [ref_11].

#### 2.2.4 기호 정의 요약

| 기호 | 정의 | 단위 | 출처 |
|---|---|---|---|
| $\sigma_\text{contact}$ | 접촉 압력 (국소 Hertz 압력) | Pa (GPa) | [ref_04] |
| $\sigma_\text{limit}$ | 표면 피로 한계 응력 | Pa (GPa) | [ref_04][ref_67] |
| $n$ | 표면 손상 지수 | 무차원 ($4 \sim 6$) | [ref_67] |
| $\tau_\text{shear}$ | 부표면 최대 전단응력 | Pa (MPa) | [ref_04] |
| $\tau_0$ | 부표면 피로 한계 전단응력 | Pa ($\approx 276\ \text{MPa}$) | [ref_67] |
| $e$ | 부표면 피로 지수 | 무차원 ($\approx 4$) | [ref_67] |
| $S_R$ | 표면 위험 파라미터 (표면/전체 적분 비) | 무차원 ($0 \sim 1$) | [ref_74] |
| $R_s$ | 정규화 표면 손상 위험 | 무차원 | [ref_11] |
| $I_S$ | 표면 손상 위험 적분 | $\text{Pa}^n \cdot \text{m}^3$ | [ref_04][ref_67] |
| $I_B$ | 부표면 피로 위험 적분 | $\text{Pa}^e \cdot \text{m}^3$ | [ref_04][ref_67] |
| $a$ | Hertz 반접촉폭 | m | [ref_75] |

*표 2.3: GBLM 응력 위험 적분 체계의 주요 기호 정의*

#### 2.2.5 소결

$I_S$는 단순한 경험적 수정 계수가 아니라, 접촉 역학과 재료 피로 역학을 연결하는 **물리 기반 체적 적분**이다. 한계 응력 초과분의 거듭제곱 적분이라는 구조는 접촉 압력의 공간 분포 전체를 반영하며, 이것이 GBLM이 고전 L-P 모델 대비 갖는 핵심 장점이다 [ref_01][ref_08]. 그러나 이 구조는 동시에 $\sigma_\text{limit}$와 $n$이라는 두 미지 상수를 도입하며, 이 상수들의 결정 방법론이 본 보고서의 핵심 주제가 된다.

---

### 2.3 Tallian-Chiu에서 GBLM으로: 모델 계보와 수정 계수 구조

#### 2.3.1 Tallian-Chiu(1967) 원형 모델

GBLM의 직접적 선행 모델은 Tallian과 Chiu(1967)가 제안한 **통합 표면-부표면 수명 모델**이다 [ref_67]. 이 모델은 구름 접촉 피로를 표면 기원과 부표면 기원으로 분리하고, 각각에 대해 응력 초과분의 거듭제곱 적분을 위험도 척도로 사용한다는 아이디어를 최초로 체계화하였다. Tallian-Chiu 모델에서 제안된 핵심 수식은 다음과 같다 [ref_67]:

$$
I_S = \int_{\Omega_\text{surf}} (\sigma_\text{contact} - \sigma_\text{limit})^n \, dV, \quad n = 4 \sim 6
\tag{2.8}
$$

$$
I_B = \int_{\Omega_\text{sub}} (\sigma_\text{shear} - \tau_0)^e \, dV, \quad e = 4
\tag{2.9}
$$

이 두 식은 현재 GBLM에서 사용되는 식 (2.4), (2.5)와 수학적으로 동일하다. Tallian-Chiu 모델은 당시 계산 자원의 한계로 인해 실용화되지 못하였으나, 그 수학적 틀은 이후 Morales-Espejel과 Gabelli에 의해 GBLM으로 발전되었다 [ref_01][ref_67].

#### 2.3.2 Lundberg-Palmgren에서 GBLM으로의 발전 계보

구름 베어링 수명 모델의 발전 계보를 정리하면 다음과 같다:

**1단계: Lundberg-Palmgren(1947/1952)** — 부표면 전단응력 기반 단일 Weibull 모델. 표면 손상 메커니즘 미포함. ISO 281의 기초 [ref_67].

**2단계: Tallian-Chiu(1967)** — 표면/부표면 분리 위험 적분 개념 도입. $I_S$와 $I_B$의 원형 정의. 그러나 실용적 교정 방법 미확립 [ref_67].

**3단계: Ioannides-Harris(1985)** — 피로 한계 응력 $\tau_0$를 부표면 적분에 명시적으로 도입. ISO 281:2007의 수정 수명 계수 $a_\text{ISO}$의 기초 [ref_67].

**4단계: Morales-Espejel & Gabelli(2015–2020)** — Tallian-Chiu의 표면 적분 $I_S$를 부활시켜 GBLM에 통합. 트라이볼로지 효과(윤활, 오염, 열)를 $I_S$ 계산에 반영. 내구성 시험을 통한 상수 교정 절차 확립 [ref_01][ref_03][ref_04][ref_05].

**5단계: Hwang & Poll(2022)** — Palmgren-Miner 선형 손상 누적 법칙을 표면/부표면 분리 구조에 적용. 잔류응력 효과를 손상 누적 계산에 반영하여 GBLM의 연속 손상 누적 모델로 확장 [ref_76].

```
Lundberg-Palmgren (1947)
        │
        ▼
Tallian-Chiu (1967) ──── I_S, I_B 원형 정의
        │
        ▼
Ioannides-Harris (1985) ── 피로 한계 τ_0 도입
        │
        ▼
Morales-Espejel & Gabelli (2015–2020) ── GBLM: I_S 부활·통합
        │
        ▼
Hwang & Poll (2022) ──── Palmgren-Miner 손상 누적 연속화
```

*그림 2.1: 구름 베어링 수명 모델의 발전 계보 [ref_01][ref_67][ref_76]*

#### 2.3.3 Palmgren-Miner 기반 손상 누적과 연속화

Hwang & Poll(2022)은 GBLM의 표면/부표면 분리 구조를 유지하면서, 각 메커니즘의 손상을 Palmgren-Miner 법칙에 따라 누적하는 방식을 제안하였다 [ref_76]:

$$
D_\text{total} = D_\text{surf} + D_\text{sub} = \sum_i \frac{n_i}{N_{f,\text{surf},i}} + \sum_j \frac{n_j}{N_{f,\text{sub},j}}
\tag{2.10}
$$

여기서 $n_i$, $n_j$는 각 응력 수준에서의 실제 사이클 수, $N_{f,\text{surf},i}$, $N_{f,\text{sub},j}$는 해당 응력 수준에서의 파손 수명이다. 이 접근법은 가변 하중 조건에서의 수명 예측을 가능하게 하며, 잔류응력이 $\sigma_\text{limit}$ 및 $\tau_0$에 미치는 영향을 명시적으로 반영할 수 있다 [ref_76]. 또한 이 연속화 구조는 $I_S$ 적분을 시간 이력에 따라 누적하는 방식으로 확장되어, 마이크로피팅 손상의 점진적 진행을 모사하는 데 활용된다 [ref_08].

#### 2.3.4 GBLM 수정 계수 구조

GBLM에서 전체 수명 수정 계수 $a_\text{SKF}$는 표면 손상 위험 $R_s$와 부표면 피로 위험을 통합하여 결정된다 [ref_11]. 이 구조에서 $I_S$는 $R_s$를 통해 $a_\text{SKF}$에 직접 영향을 미친다:

$$
L_{nm} = a_1 \cdot a_\text{SKF}(I_S, \kappa, \eta_c) \cdot L_{10}
\tag{2.11}
$$

여기서 $\kappa$는 점도비(viscosity ratio), $\eta_c$는 오염 계수이다. $a_\text{SKF}$는 $I_S$의 함수로서, $I_S$가 증가할수록(표면 손상 위험 증가) $a_\text{SKF}$가 감소하여 수명이 단축된다 [ref_11]. 이 구조는 GBLM이 단순한 수명 수정 계수 모델이 아니라, 접촉 역학과 재료 피로를 연결하는 물리 기반 모델임을 보여준다.

#### 2.3.5 소결

Tallian-Chiu(1967)에서 GBLM(2015–2020)으로의 발전은 단순한 수식 개선이 아니라, 표면 손상 메커니즘을 수명 모델의 **1등 시민(first-class citizen)** 으로 격상시킨 패러다임 전환이다 [ref_01][ref_67]. Hwang & Poll(2022)의 Palmgren-Miner 연속화는 이 틀을 가변 하중 조건으로 확장하였으며 [ref_76], 이 모든 발전의 핵심에 $I_S$ 적분이 위치한다. 그러나 $I_S$의 정확한 계산을 위해서는 $\sigma_\text{limit}$와 $n$이라는 미지 상수의 결정이 선행되어야 하며, 이것이 이후 섹션에서 다룰 핵심 문제이다.

---

---

## 3. I_S 근본 수식과 미지 상수의 식별

본 섹션은 Morales-Espejel GBLM에서 표면손상 함수 $I_S$를 지배하는 근본 수식을 해부하고, 그 안에 내재된 미지 상수들이 왜 단순 해석적 계산으로는 결정될 수 없는지를 수식 수준에서 입증한다. 이를 통해 sec-5에서 다루는 실험 교정(calibration) 절차의 필연성을 논리적으로 선행 정당화한다.

---

### 3.1 표면 적분 정의식과 응력 σ_s의 Hertz 응력으로부터의 유도

#### 3.1.1 I_S 적분 정의식

GBLM에서 표면손상 함수 $I_S$는 접촉 영역 내 표면층에 걸쳐 임계 응력 초과분을 누적하는 체적(면적) 적분으로 정의된다 [ref_01][ref_02]:

$$
I_S = \int_{\text{contact}} \left(\frac{\sigma_s - \sigma_{s,\text{limit}}}{\sigma_{s,\text{ref}}}\right)^k dA, \quad \sigma_s > \sigma_{s,\text{limit}}
\tag{3.1}
$$

> **[주석]** 식 (3.1)의 $dA$는 표면층 두께 또는 손상 커널을 적분 방향으로 축약한 등가 면적 표현이다. I_S의 기본 정의는 체적 적분 $I_S = \int_V (\sigma_{\text{contact}} - \sigma_{\text{limit}})^n \, dV$ [ref_04][ref_67]이며, 이 dA 형태는 표면층 두께(예: 0–10 μm)를 흡수한 단순화 표현이다. 정규화 기준 응력 $\sigma_{s,\text{ref}}$에 의해 무차원화되어 있으므로 $I_S$는 무차원량이다.

여기서:
- $\sigma_s$: 접촉 표면층(깊이 $0$–$10\,\mu\text{m}$)에서의 국소 표면 응력 [ref_01]
- $\sigma_{s,\text{limit}}$: 표면 손상이 개시되는 임계 응력(표면 피로한계)
- $\sigma_{s,\text{ref}}$: 정규화 기준 응력(재료·조도·윤활 조건에 의존)
- $k$: 표면 손상 지수(표면 모드 전용)
- $dA$: 접촉 면적 미소 요소 (표면층 두께 축약 등가 표현)

적분은 $\sigma_s > \sigma_{s,\text{limit}}$인 영역에서만 수행되며, 임계값 이하의 응력은 손상에 기여하지 않는다는 피로한계 개념을 내포한다. 이 정의는 Tallian & Chiu(1967)의 선행 모델 [ref_67]에서 직접 계승된 것으로, 원형은 다음과 같다:

$$
I_S^{\text{Tallian}} = \int (\sigma_{\text{contact}} - \sigma_{\text{limit}})^n \, dV \tag{3.2}
$$

Morales-Espejel 모델은 이를 면적 적분 형태로 정제하고 정규화 상수 $\sigma_{s,\text{ref}}$를 명시적으로 도입함으로써 무차원화 및 교정 가능성을 확보하였다 [ref_04].

부표면 손상 함수 $I_B$와의 대비를 위해 병기하면:

$$
I_B = \int_{\text{subsurface}} \left(\frac{\tau - \tau_0}{\tau_{\text{ref}}}\right)^e \, dV, \quad \tau > \tau_0
\tag{3.3}
$$

여기서 $\tau$는 부표면 전단응력, $\tau_0$는 부표면 피로한계, $e \approx 4$는 부표면 지수이다 [ref_04][ref_67]. 표면 적분 $I_S$와 부표면 적분 $I_B$는 서로 다른 응력 성분, 서로 다른 적분 영역, 서로 다른 지수를 사용한다는 점에서 물리적으로 독립적인 손상 채널을 표현한다.

#### 3.1.2 표면 응력 σ_s의 Hertz 응력으로부터의 유도

$\sigma_s$는 부표면 최대 응력과 명확히 구별되어야 한다. 부표면 최대 전단응력은 깊이 $z \approx 0.5a$ (접촉 반축 $a$)에서 발생하는 직교 전단응력 $\tau_{yz}^{\max}$ 또는 von Mises 등가응력이며, 이는 $I_B$ 계산에 사용된다 [ref_75]. 반면 $\sigma_s$는 접촉 표면 바로 아래 $0$–$10\,\mu\text{m}$ 층에서의 응력 상태를 반영한다 [ref_01][ref_75].

Hertz 접촉 이론에서 최대 접촉 압력 $p_0$와 접촉 반축 $a$, $b$가 주어지면, 표면층 응력 분포는 다음 절차로 유도된다:

**① Hertz 압력 분포:**
타원형 접촉(TRB 등)에서 접촉 압력은:

$$
p(x, y) = p_0 \sqrt{1 - \left(\frac{x}{a}\right)^2 - \left(\frac{y}{b}\right)^2}
\tag{3.4}
$$

**② 표면층 응력 텐서 (단순화된 Hertz 근사):**
표면($z = 0$)에서의 주응력은 단순화된 Hertz 근사로부터 [ref_01][ref_02]:

$$
\sigma_x\big|_{z=0} = -p_0 \cdot \frac{2\nu b}{a+b}, \quad \sigma_y\big|_{z=0} = -p_0 \cdot \frac{2\nu a}{a+b}, \quad \sigma_z\big|_{z=0} = -p_0
\tag{3.5}
$$

여기서 $\nu$는 포아송 비이다. 표면에서 $\sigma_z = -p_0$이며, 이는 법선 방향 압축응력이다. **[주석: 식 (3.5)는 마찰력·잔류응력·표면 조도 응력 집중을 무시한 단순화된 Hertz 근사이며, 엄밀한 표면 응력 텐서는 마이크로 EHL 수치 해석을 통해 산출해야 한다.]**

**③ 표면 응력 σ_s의 정의:**
$\sigma_s$는 표면층에서의 등가 응력(예: von Mises 또는 최대 주응력의 절댓값)으로 정의되며, 접촉 중심부에서 최대값을 가진다 [ref_01]:

$$
\sigma_s^{\max} \approx p_0 \quad \text{(접촉 중심, 표면층)}
\tag{3.6}
$$

실제로는 표면 조도에 의한 응력 집중(stress concentration)이 추가되어:

$$
\sigma_s = K_t \cdot \sigma_s^{\text{Hertz}}
\tag{3.7}
$$

여기서 $K_t$는 표면 조도 형상에 의존하는 응력 집중 계수이다. 이 계수는 윤활막 두께비 $\Lambda = h_{\min}/\sigma_{\text{composite}}$와 연동되며, 마이크로 EHL 해석 없이는 정량화가 어렵다 [ref_01][ref_02].

**④ 부표면 응력과의 깊이별 비교:**

| 응력 성분 | 최대 발생 깊이 | 크기 (p_0 기준) | 적용 함수 |
|---|---|---|---|
| 법선 압력 $\sigma_z$ | $z = 0$ (표면) | $\approx p_0$ | $I_S$ |
| 직교 전단응력 $\tau_{yz}$ | $z \approx 0.5a$ | $\approx 0.25 p_0$ | $I_B$ |
| von Mises 등가응력 | $z \approx 0.5a$ | $\approx 0.56 p_0$ | $I_B$ |
| 표면 von Mises | $z = 0$ | $\approx 0.5 p_0$ (조도 집중 전) | $I_S$ |

출처: [ref_75][ref_01]

이 구분은 $I_S$와 $I_B$가 서로 다른 깊이의 응력 상태를 적분한다는 물리적 근거를 제공하며, 두 함수를 동일한 응력 필드로 계산할 수 없음을 보여준다 [ref_75].

#### 3.1.3 소결

$I_S$의 정의식 (3.1)은 수학적으로 명확하지만, 피적분 함수 내에 $\sigma_{s,\text{limit}}$, $\sigma_{s,\text{ref}}$, $k$라는 세 개의 미지 상수를 포함한다. 또한 $\sigma_s$ 자체가 표면 조도 응력 집중을 통해 윤활 조건에 의존하므로, 이 적분은 순수 해석적 방법으로는 닫힌 형태(closed form)로 계산될 수 없다.

---

### 3.2 미지 상수 3종(σ_s,ref / σ_limit, 지수 n/k)의 물리적 의미와 범위

#### 3.2.1 개요

식 (3.1)에 등장하는 세 미지 상수는 각각 독립적인 물리적 의미를 가지며, 서로 다른 재료·운전 조건 의존성을 보인다. 이들을 체계적으로 정리하면 다음과 같다.

**미지 상수 요약표:**

| 상수 | 기호 | 물리적 의미 | 문헌 범위 | 주요 의존 인자 | 출처 |
|---|---|---|---|---|---|
| 기준 표면응력 | $\sigma_{s,\text{ref}}$ | 정규화 기준; 손상 함수 스케일 결정 | 재료·조도·윤활 의존 (공개값 없음) | 재료 강도, 표면 조도 $R_a$, 윤활막 $\Lambda$ | [ref_01][ref_02] |
| 표면 손상 한계 | $\sigma_{s,\text{limit}}$ | 표면 피로 개시 임계 응력 | $\approx 1.4$–$1.5\,\text{GPa}$ (Hertz 기준) | 재료 경도, 잔류응력, 온도 | [ref_71] |
| 표면 손상 지수 | $k$ (또는 $n$) | 응력-손상 감도; 손상 누적 비선형성 | $n = 4$–$6$ (표면) | 재료 미세구조, 온도 | [ref_67][ref_05] |
| 부표면 지수 (참고) | $e$ | 부표면 응력-수명 지수 | $e = 4$ | 재료 (AISI 52100 기준) | [ref_67][ref_04] |

#### 3.2.2 기준 표면응력 σ_s,ref

$\sigma_{s,\text{ref}}$는 $I_S$ 적분을 무차원화하는 정규화 상수로, 손상 함수의 절대적 크기(scale)를 결정한다. 물리적으로는 "단위 손상률을 유발하는 기준 표면 응력 수준"으로 해석할 수 있다 [ref_01].

이 상수의 핵심 특성은 다음과 같다:

- **재료 의존성**: 베어링강의 경도, 미세구조, 잔류응력 상태에 따라 변화한다. AISI 52100 기준 von Mises 피로한계 $\sigma_{vM} \approx 0.9\,\text{GPa}$가 하한 앵커 역할을 한다 [ref_72].
- **표면 조도 의존성**: 조도가 증가할수록 응력 집중이 커져 유효 $\sigma_s$가 상승하므로, 동일한 Hertz 압력에서도 $\sigma_{s,\text{ref}}$의 상대적 의미가 달라진다 [ref_01][ref_02].
- **윤활 의존성**: 윤활막 두께비 $\Lambda$가 감소하면 금속 접촉 빈도가 증가하여 표면 응력 집중이 심화된다. 이는 $\sigma_{s,\text{ref}}$가 단순 재료 상수가 아니라 트라이볼로지 시스템 상수임을 의미한다 [ref_01][ref_05].
- **공개 불가**: SKF GBLM 문헌 어디에도 $\sigma_{s,\text{ref}}$의 수치가 명시적으로 공개되어 있지 않다. 이는 이 상수가 내구성 시험 데이터로부터 역산(inverse calibration)되어야 함을 의미한다 [ref_01][ref_02].

#### 3.2.3 표면 손상 한계 σ_s,limit

$\sigma_{s,\text{limit}}$은 표면 피로 손상이 개시되는 임계 응력으로, 이 값 이하에서는 이론적으로 무한 수명이 보장된다. 문헌에서 확인된 수치 범위는 다음과 같다:

- **Hertz 압력 기준**: $\sigma_{s,\text{limit}} \approx 1.4$–$1.5\,\text{GPa}$ [ref_71]
  - 롤러 베어링: $\approx 1.4\,\text{GPa}$
  - 볼 베어링: $\approx 1.5\,\text{GPa}$
- **부표면 피로한계와의 관계**: 부표면 직교 전단응력 한계 $\tau_0 = 0.25 \cdot p_0 \approx 200$–$300\,\text{MPa}$ [ref_71]

이 두 값의 관계를 정리하면:

$$
\tau_0 = 0.25 \cdot p_0, \quad p_0 \approx 800\text{–}1200\,\text{MPa} \Rightarrow \tau_0 \approx 200\text{–}300\,\text{MPa}
\tag{3.8}
$$

$$
\sigma_{s,\text{limit}} \approx 1.4\text{–}1.5\,\text{GPa} \gg \tau_0 \approx 200\text{–}300\,\text{MPa}
\tag{3.9}
$$

이 차이는 표면 손상 모드(마이크로피팅)가 부표면 손상 모드(스폴링)보다 훨씬 높은 응력 수준에서 개시됨을 의미하지 않는다. 오히려 $\sigma_{s,\text{limit}}$이 Hertz 최대 압력 $p_0$와 직접 비교되는 반면, $\tau_0$는 전단응력 성분과 비교되기 때문에 단위 기준이 다르다 [ref_71][ref_75].

AISI 52100 베어링강에 대한 von Mises 기준 피로한계 $\sigma_{vM,\text{limit}} \approx 0.9\,\text{GPa}$는 [ref_72]에서 확인되며, 이는 $\sigma_{s,\text{limit}}$의 하한 추정치로 활용될 수 있다. 그러나 이 값은 표준 피로 시험(회전 굽힘, 인장-압축)에서 도출된 것으로, 롤링 접촉 피로(RCF) 조건의 다축 응력 상태와 직접 동일시하기 어렵다 [ref_72].

**온도 의존성**: $\sigma_{s,\text{limit}}$은 온도 상승에 따라 감소하는 경향이 있으며, 이는 고온 운전 조건에서 표면 손상 위험이 증가함을 의미한다 [ref_05].

#### 3.2.4 손상 지수 k (또는 n)

손상 지수 $k$는 응력 초과분 $(\sigma_s - \sigma_{s,\text{limit}})$과 손상 누적률 사이의 비선형 관계를 결정하는 핵심 매개변수이다. 문헌에서 확인된 범위와 근거는 다음과 같다:

**표면 손상 지수 n (= k):**
- 범위: $n = 4$–$6$ [ref_67][ref_05]
- Tallian & Chiu(1967) 원형 모델에서 $n = 4$–$6$으로 제안 [ref_67]
- Morales-Espejel(2020) 열 효과 확장 모델에서 온도 의존성 확인 [ref_05]

**부표면 지수 e (참고):**
- $e = 4$ (AISI 52100 기준) [ref_67][ref_04]
- Zaretsky 모델의 응력-수명 지수 $e \approx 4$와 일치 [ref_67]

**지수의 물리적 의미:**
지수 $n$이 클수록 응력 집중에 대한 손상 민감도가 높아진다. 예를 들어 $n = 4$와 $n = 6$의 경우, 응력이 10% 증가할 때 손상 기여도는 각각 약 46% 및 77% 증가한다:

$$
\frac{(1.1)^6 - 1}{(1.1)^4 - 1} = \frac{0.772}{0.464} \approx 1.66
\tag{3.10}
$$

이는 지수 불확실성이 $I_S$ 계산 결과에 미치는 영향이 매우 크다는 것을 의미한다.

**온도 의존성**: ref_05에 따르면 표면 손상 지수 $n$은 온도 상승에 따라 변화하며, 이는 고온 운전 조건에서 별도의 교정이 필요함을 시사한다 [ref_05].

#### 3.2.5 왜 직접 계산이 불가능한가: 수식적 입증

세 미지 상수가 동시에 미지인 상황에서 $I_S$를 직접 계산할 수 없음을 다음과 같이 수식으로 입증한다.

$I_S$를 단순화된 1차원 형태로 표현하면:

$$
I_S = \int_0^{A_c} \left(\frac{\sigma_s(x) - \sigma_{s,\text{limit}}}{\sigma_{s,\text{ref}}}\right)^k dx
\tag{3.11}
$$

여기서 $A_c$는 접촉 면적이다. 이 식에서:

1. **$\sigma_s(x)$**: Hertz 해석해로부터 계산 가능하나, 표면 조도 응력 집중 $K_t$를 포함하면 마이크로 EHL 수치 해석이 필요하다.
2. **$\sigma_{s,\text{limit}}$**: 문헌값 $1.4$–$1.5\,\text{GPa}$ 범위만 알려져 있으며, 특정 재료·조건에 대한 정확한 값은 미지이다.
3. **$\sigma_{s,\text{ref}}$**: 완전히 미지이며 공개된 수치가 없다.
4. **$k$**: 범위 $4$–$6$만 알려져 있으며, 특정 조건에서의 정확한 값은 미지이다.

세 미지 상수 $(\sigma_{s,\text{limit}}, \sigma_{s,\text{ref}}, k)$에 대해 방정식 (3.11) 하나만으로는 유일해를 결정할 수 없다. 이는 수학적으로 **불충분 결정 시스템(underdetermined system)**이다:

$$
\underbrace{I_S}_{\text{관측 가능}} = f\!\left(\underbrace{\sigma_{s,\text{limit}},\, \sigma_{s,\text{ref}},\, k}_{\text{미지 상수 3개}}\right)
\tag{3.12}
$$

유일해를 얻으려면 최소 3개의 독립적인 실험 조건에서 $I_S$를 측정(또는 수명 데이터로부터 역산)해야 한다. 이것이 sec-5에서 다루는 교정 실험의 수학적 근거이다.

---

### 3.3 Dang Van 다축 기준·Palmgren-Miner와의 정량적 연계

#### 3.3.1 Dang Van 다축 피로 기준과의 연계

Dang Van 기준은 다축 응력 상태에서의 피로 개시를 평가하는 임계면(critical plane) 기반 기준으로, 롤링 접촉 피로에 적용된 형태는 다음과 같다 [ref_77]:

$$
D_{DV} = \frac{\tau_{\max} + \alpha \cdot \sigma_m}{f_{\text{limit}}}
\tag{3.13}
$$

여기서:
- $\tau_{\max}$: 최대 전단응력 (임계면에서)
- $\sigma_m$: 평균 정수압 응력 (hydrostatic stress)
- $\alpha$: 정수압 감도 계수 (재료 상수)
- $f_{\text{limit}}$: 비틀림 피로한계

$D_{DV} \geq 1$이면 피로 개시가 예측된다 [ref_77].

**$I_S$와의 연계:**
$I_S$ 적분의 피적분 함수 $(\sigma_s - \sigma_{s,\text{limit}})^k$는 Dang Van 기준의 $(\tau_{\max} + \alpha \sigma_m - f_{\text{limit}})$와 구조적으로 유사하다. 두 기준 모두:
- 임계 응력 이하에서는 손상 없음 (피로한계 개념)
- 임계 응력 초과분의 거듭제곱으로 손상 누적
- 다축 응력 상태를 단일 등가 응력으로 환원

의 공통 구조를 가진다. 차이점은 Dang Van 기준이 임계면에서의 응력 상태를 사용하는 반면, $I_S$는 접촉 영역 전체에 걸친 적분을 수행한다는 점이다 [ref_77][ref_01].

**표면층 asperity 응력 평가:**
Dang Van 기준은 표면 asperity 접촉에서 발생하는 국소 응력 집중을 평가하는 데 특히 유용하다. Asperity 접촉 시 $\tau_{\max}$와 $\sigma_m$이 모두 증가하므로, $D_{DV}$가 급격히 상승하여 마이크로피팅 개시 위험을 정량화할 수 있다 [ref_77].

이를 $I_S$와 연계하면, $\sigma_{s,\text{limit}}$은 Dang Van 기준에서 $D_{DV} = 1$에 해당하는 응력 수준으로 해석될 수 있다:

$$
\sigma_{s,\text{limit}} \leftrightarrow \sigma_s \text{ such that } D_{DV}(\sigma_s) = 1
\tag{3.14}
$$

이 연계는 $\sigma_{s,\text{limit}}$의 물리적 의미를 Dang Van 기준의 언어로 재해석하는 이론적 근거를 제공한다 [ref_77].

#### 3.3.2 Palmgren-Miner 선형 손상 누적과의 연계

Palmgren-Miner 법칙은 변동 하중 하에서의 피로 손상을 선형으로 누적하는 고전적 방법이다:

$$
D_{PM} = \sum_i \frac{N_i}{N_{f,i}}
\tag{3.15}
$$

여기서 $N_i$는 응력 수준 $i$에서의 실제 사이클 수, $N_{f,i}$는 동일 응력 수준에서의 파괴 사이클 수이다 [ref_67].

**연속 적분화:**
롤링 접촉에서 접촉 압력 분포는 연속적이므로, 이산 합산을 연속 적분으로 변환하면 [ref_67]:

$$
D_{PM} = \int_{\sigma > \sigma_{\text{limit}}} \frac{n(\sigma)}{N_f(\sigma)} \, d\sigma
\tag{3.16}
$$

여기서 $n(\sigma)$는 응력 수준 $\sigma$에서의 사이클 밀도, $N_f(\sigma)$는 S-N 곡선으로부터의 파괴 수명이다.

S-N 곡선을 멱함수 형태 $N_f(\sigma) \propto (\sigma - \sigma_{\text{limit}})^{-k}$로 가정하면:

$$
D_{PM} \propto \int (\sigma - \sigma_{\text{limit}})^k \cdot n(\sigma) \, d\sigma
\tag{3.17}
$$

이는 $I_S$ 적분 (3.1)과 동일한 구조이다. 즉, **$I_S$는 Palmgren-Miner 손상 누적 법칙의 연속 적분 형태**로 해석될 수 있다 [ref_67][ref_01].

이 연계의 중요성은 다음과 같다:
1. $I_S$의 지수 $k$는 S-N 곡선의 기울기와 직접 연결된다.
2. $\sigma_{s,\text{ref}}$는 S-N 곡선의 절편(기준 수명에서의 응력)에 해당한다.
3. $\sigma_{s,\text{limit}}$은 S-N 곡선의 피로한계에 해당한다.

따라서 $I_S$의 세 미지 상수는 표면 RCF의 S-N 곡선을 완전히 결정하는 세 매개변수와 일대일 대응된다 [ref_67].

#### 3.3.3 GBLM 전체 생존 확률과의 통합

GBLM에서 표면 생존 확률 $S_S$와 부표면 생존 확률 $S_B$는 독립적으로 계산된 후 결합된다 [ref_01][ref_04]:

$$
S_{\text{total}} = S_S \cdot S_B
\tag{3.18}
$$

$$
\ln\!\left(\frac{1}{S_S}\right) = A_S \cdot I_S, \quad \ln\!\left(\frac{1}{S_B}\right) = A_B \cdot I_B
\tag{3.19}
$$

여기서 $A_S$, $A_B$는 Weibull 분포 매개변수와 연관된 스케일 계수이다. 이 구조에서 $I_S$의 미지 상수들은 $A_S$와 함께 교정되어야 하므로, 실질적으로 교정해야 할 매개변수의 수는 더 증가한다.

**Palmgren-Miner와의 최종 연계:**
식 (3.19)를 Palmgren-Miner 형태로 재해석하면, $A_S \cdot I_S$는 표면 손상의 누적 손상도 $D_{PM,S}$에 해당하며, $S_S = e^{-D_{PM,S}}$는 Weibull 생존 함수이다. 이 연계는 GBLM이 고전적 피로 이론(Palmgren-Miner)과 현대적 다축 피로 기준(Dang Van)을 통합한 프레임워크임을 보여준다 [ref_01][ref_67][ref_77].

#### 3.3.4 소결: sec-5 교정으로의 연결

본 섹션에서 분석한 바를 종합하면:

1. $I_S$ 정의식 (3.1)은 수학적으로 명확하나, 세 미지 상수 $(\sigma_{s,\text{limit}}, \sigma_{s,\text{ref}}, k)$를 포함한다.
2. 이 상수들은 Hertz 해석해, Dang Van 기준, Palmgren-Miner 법칙으로부터 이론적 범위를 추정할 수 있으나, 특정 재료·조건에 대한 정확한 값은 결정할 수 없다.
3. 수학적으로 3개의 미지 상수를 결정하려면 최소 3개의 독립적 실험 조건이 필요하다.
4. 이것이 sec-5에서 다루는 TRB 마이크로피팅 내구성 시험 기반 교정 절차의 필연적 근거이다.

---

---

## 4. 마이크로피팅 물리 모델과 표면 적분 손상의 수치 절차

본 섹션은 SKF GBLM의 표면손상 함수 $I_S$를 실제로 계산하기 위한 물리 모델과 수치 알고리즘을 전문가 수준으로 기술한다. 크게 세 층위로 구성된다: (1) Morales-Espejel & Brizmer 마이크로피팅 모델의 물리적 기반, (2) 거친표면 접촉압력 및 응력장의 FFT/DC-FFT 수치 절차, (3) 응력 이력의 Rainflow 분해와 Palmgren-Miner 손상 누적을 통한 $I_S$ 산출. 세 단계는 순차적으로 연결되어 하나의 계산 파이프라인을 형성한다.

---

### 4.1 Morales-Espejel & Brizmer 마이크로피팅 모델: λ비·조도·asperity 응력

#### 4.1.1 모델의 물리적 배경

Morales-Espejel & Brizmer(2011)는 롤링-슬라이딩 접촉에서 마이크로피팅을 예측하는 공학 모델을 정립하였다 [ref_17]. 이 모델의 핵심 전제는 마이크로피팅이 **표면 피로(surface fatigue)**와 **mild wear**의 경쟁적 과정으로 결정된다는 것이다 [ref_18]. 표면 피로는 asperity 접촉에서 발생하는 국소 응력 사이클이 누적되어 표면 균열을 생성하는 메커니즘이며, mild wear는 asperity를 점진적으로 제거하여 표면을 평탄화함으로써 피로 손상 누적을 억제하는 역할을 한다. 두 과정의 상대적 속도에 따라 마이크로피팅 발생 여부와 심각도가 결정된다 [ref_18].

재료 박리(material detachment) 관점에서, 표면 위상(surface topography)의 피트(pit) 생성은 열·피로·mild wear가 동시에 작용하는 복합 과정으로 이해된다 [ref_06]. 이 모델은 TRB(테이퍼 롤러 베어링)의 마이크로피팅 수명 계산에 직접 적용 가능하며, $I_S$ 적분의 물리적 근거를 제공한다.

#### 4.1.2 윤활 파라미터 λ비의 정의와 역할

윤활 상태를 정량화하는 핵심 파라미터는 **λ비(lambda ratio)**이다. 이는 최소 유막 두께 $h_\mathrm{min}$을 두 접촉면의 복합 조도(composite roughness)로 나눈 무차원 수로 정의된다 [ref_20]:

$$
\lambda = \frac{h_\mathrm{min}}{\sqrt{R_{a,1}^2 + R_{a,2}^2}}
$$

여기서 $R_{a,1}$, $R_{a,2}$는 각각 롤러와 레이스웨이의 산술평균 조도(arithmetic mean roughness)이다. $\lambda < 1$이면 경계윤활(boundary lubrication), $1 \leq \lambda \leq 3$이면 혼합윤활(mixed lubrication), $\lambda > 3$이면 완전 유체윤활(full film lubrication)로 분류된다 [ref_20].

$h_\mathrm{min}$은 Hamrock-Dowson 또는 유사한 EHL 공식으로부터 접촉 하중 $W$, 등가 탄성계수 $E'$, 등가 곡률반경 $R'$, 속도 파라미터 $U$, 재료 파라미터 $G$의 함수로 계산된다:

$$
h_\mathrm{min} = f\!\left(U,\, G,\, W,\, R'\right)
$$

#### 4.1.3 λ비의 비선형 거동

마이크로피팅 손상과 λ비의 관계는 단조적이지 않다. Brizmer et al.(2019)의 시뮬레이션 및 실험 결과에 따르면, λ비가 증가함에 따라 마이크로피팅 손상은 초기에 증가하여 **λ ≈ 1.5–2 부근에서 최대값**에 도달한 후 감소하는 비선형 거동을 보인다 [ref_19]. 이는 다음과 같이 해석된다:

- **λ < 1 (경계윤활)**: asperity 접촉이 빈번하지만 mild wear 속도도 빨라 피로 손상 누적이 제한됨
- **λ ≈ 1.5–2 (혼합윤활 임계)**: asperity 응력 사이클이 최대이면서 wear에 의한 제거 효과가 상대적으로 약해 마이크로피팅 위험 최대
- **λ > 3 (완전 유체윤활)**: asperity 접촉 빈도 급감으로 손상 누적 소멸

이 비선형성은 $I_S$ 계산 시 λ를 단순 선형 인자로 취급할 수 없음을 의미하며, 모델 내 비선형 가중 함수의 필요성을 정당화한다 [ref_19].

#### 4.1.4 슬라이딩비(SRR)와 asperity 국소 응력

슬라이딩비(Slide-to-Roll Ratio, SRR)는 두 접촉면의 속도 차이를 평균 속도로 나눈 값이다:

$$
\mathrm{SRR} = \frac{|u_1 - u_2|}{(u_1 + u_2)/2}
$$

SRR이 증가하면 asperity 통과 시 접선력(tangential traction)이 증가하여 표면 응력이 상승한다. asperity 국소 응력은 Hertz 접촉 응력에 응력 집중 계수를 곱한 형태로 모델링된다 [ref_17]:

$$
\sigma_\mathrm{local} = \sigma_\mathrm{Hertz} \cdot K_\mathrm{asperity}
$$

여기서 $K_\mathrm{asperity}$는 asperity의 형상(곡률, 높이), 윤활 조건(λ), SRR에 의존하는 무차원 응력 집중 계수이다. 실제 계산에서는 표면 조도 프로파일의 각 asperity에 대해 이 계수를 개별적으로 산정하며, 이를 위해 micro-EHL 해석이 필요하다 [ref_20].

TRB의 경우, 롤러-레이스웨이 선접촉(line contact)에서 trailing edge asperity가 특히 높은 응력 집중을 유발하며, 이는 표면 피로 위험의 주요 원인이 된다 [ref_17].

#### 4.1.5 표면 피로 vs Mild Wear 경쟁 메커니즘

Morales-Espejel et al.(2018)은 마이크로피팅 손상 예측에서 표면 피로와 mild wear의 경쟁을 다음과 같이 정식화하였다 [ref_18]:

- **손상 누적률**: 각 over-rolling 사이클에서 asperity 응력이 피로 한계를 초과하면 손상이 누적됨
- **Wear 제거율**: mild wear에 의해 asperity가 제거되면 해당 asperity의 응력 사이클이 소멸됨
- **순 손상**: 누적 손상에서 wear에 의한 손상 제거를 뺀 값이 마이크로피팅 진행 여부를 결정

이 경쟁 모델은 $I_S$ 적분의 시간 의존성을 설명하며, 표면 위상이 시간에 따라 변화하는 동적 문제로 귀결된다 [ref_06].

---

### 4.2 거친표면 접촉압력과 응력장: DC-FFT / 영향계수 절차

#### 4.2.1 수치 절차 개요

$I_S$ 계산의 핵심 수치 과제는 실제 거친표면(rough surface)에서의 접촉압력 분포와 그로 인한 3차원 응력장을 효율적으로 계산하는 것이다. 이를 위한 표준 절차는 다음 단계로 구성된다:

1. **거친표면 접촉압력 계산**: FFT 기반 탄성 접촉 해석 [ref_29]
2. **응력장 계산**: DC-FFT 기반 Green 함수 합성곱 [ref_27], [ref_28]
3. **표면 traction 응력 포함**: 접선력에 의한 응력 기여 [ref_28]
4. **부표면 von Mises 응력**: 3D 선접촉 혼합 EHL 결합 [ref_32]

#### 4.2.2 FFT 기반 거친표면 접촉압력 계산

Stanley & Kato(1997)의 FFT 기반 방법은 선형 탄성 반무한체(semi-infinite solid)에서 거친표면 접촉압력을 효율적으로 계산한다 [ref_29]. 기본 원리는 표면 변형과 접촉압력의 관계를 합성곱(convolution)으로 표현하고, 이를 FFT로 가속하는 것이다.

탄성 반무한체에서 표면 변형 $u_z(x,y)$와 접촉압력 $p(x,y)$의 관계는 Boussinesq 적분으로 주어진다:

$$
u_z(x,y) = \frac{2}{\pi E'} \iint \frac{p(x',y')}{\sqrt{(x-x')^2+(y-y')^2}}\, dx'\, dy'
$$

이를 이산화하면:

$$
u_z[i,j] = \sum_{k,l} C[i-k,\, j-l]\cdot p[k,l]
$$

여기서 $C[i,j]$는 Boussinesq 영향계수(influence coefficient)이다. 이 이산 합성곱을 FFT로 계산하면 $O(N^2 \log N)$의 계산 복잡도를 달성한다 [ref_29].

접촉 조건(비침투 조건, 압력 비음수 조건)을 만족하는 압력 분포는 반복 수렴 절차(conjugate gradient method 등)로 구한다:

$$
p[i,j] \geq 0, \quad g[i,j] \geq 0, \quad p[i,j] \cdot g[i,j] = 0
$$

여기서 $g[i,j]$는 표면 간극(gap)이다 [ref_29].

#### 4.2.3 DC-FFT 알고리즘: 이산 합성곱과 FFT의 결합

Liu, Wang & Liu(2000)의 DC-FFT(Discrete Convolution and FFT) 방법은 접촉 및 응력 문제에서 이산 합성곱을 FFT로 가속하는 범용 알고리즘이다 [ref_27]. 핵심 아이디어는 **주기적 합성곱(circular convolution)**의 aliasing 오류를 방지하기 위해 영향계수 행렬을 적절히 패딩(zero-padding)하는 것이다.

$N \times M$ 격자에서 합성곱 $h = f * g$를 계산할 때:

1. 영향계수 $C[i,j]$를 $2N \times 2M$ 크기로 확장(zero-padding)
2. 입력 $p[i,j]$도 동일 크기로 확장
3. 2D FFT 적용: $\hat{C} = \mathcal{F}\{C\}$, $\hat{p} = \mathcal{F}\{p\}$
4. 주파수 영역 곱셈: $\hat{u} = \hat{C} \cdot \hat{p}$
5. 역 FFT: $u = \mathcal{F}^{-1}\{\hat{u}\}$
6. 유효 영역 추출

이 절차는 $O(N^2 \log N)$ 복잡도로 $O(N^4)$의 직접 합성곱을 대체하며, 수백만 격자점의 접촉 문제를 실용적 시간 내에 해결한다 [ref_27].

#### 4.2.4 표면 Traction에 의한 응력장: Green 함수 영향계수

Liu & Wang(2002)은 DC-FFT 알고리즘을 표면 traction(접선력)에 의한 응력장 계산으로 확장하였다 [ref_28]. 반무한체 표면에 분포 traction $q_x(x,y)$, $q_y(x,y)$가 작용할 때, 내부 응력 성분 $\sigma_{ij}(x,y,z)$는 Green 함수(영향계수)와의 합성곱으로 표현된다:

$$
\sigma_{ij}(x,y,z) = \iint \left[ G_{ij}^{(p)}(x-x',y-y',z)\cdot p(x',y') + G_{ij}^{(q)}(x-x',y-y',z)\cdot q(x',y') \right] dx'\, dy'
$$

여기서 $G_{ij}^{(p)}$, $G_{ij}^{(q)}$는 각각 법선 하중과 접선 하중에 대한 Mindlin-Cerruti 형태의 Green 함수이다. 이를 주파수 응답 함수(Frequency Response Function, FRF)로 변환하면:

$$
\hat{\sigma}_{ij}(k_x, k_y, z) = \hat{G}_{ij}^{(p)}(k_x, k_y, z)\cdot \hat{p}(k_x, k_y) + \hat{G}_{ij}^{(q)}(k_x, k_y, z)\cdot \hat{q}(k_x, k_y)
$$

역 FFT를 통해 실공간 응력장을 복원한다 [ref_28]. 이 방법은 마찰력(SRR에 비례)에 의한 표면 응력 기여를 정확히 포함하므로, 마이크로피팅 모델에서 SRR의 영향을 정량화하는 데 필수적이다.

#### 4.2.5 반무한체 부표면 응력 계산

ref_31의 FFT 기법은 반무한체에서 접촉압력과 부표면 응력을 동시에 계산하는 절차를 제공한다 [ref_31]. 깊이 $z$에서의 응력 텐서 $\boldsymbol{\sigma}(x,y,z)$는 표면 압력 분포 $p(x,y)$로부터 다음과 같이 계산된다:

$$
\sigma_{ij}(x,y,z) = \sum_{k,l} K_{ij}(x-x_k,\, y-y_l,\, z)\cdot p(x_k, y_l)\, \Delta x\, \Delta y
$$

여기서 $K_{ij}$는 Boussinesq-Cerruti 이론에서 유도된 응력 영향계수이다. DC-FFT 가속을 적용하면 임의 깊이 $z$에서의 전체 응력 텐서를 효율적으로 계산할 수 있다 [ref_27], [ref_31].

#### 4.2.6 3D 선접촉 혼합 EHL과 부표면 von Mises 응력

TRB의 롤러-레이스웨이 접촉은 본질적으로 선접촉(line contact)이다. Zhu et al.의 3D 선접촉 혼합 EHL 모델은 거친표면 형상을 직접 EHL 해석에 포함하여 실제 접촉압력 분포를 계산하고, 이로부터 부표면 von Mises 응력을 산출한다 [ref_32]:

$$
\sigma_\mathrm{vM}(x,y,z) = \sqrt{\frac{1}{2}\left[(\sigma_{xx}-\sigma_{yy})^2 + (\sigma_{yy}-\sigma_{zz})^2 + (\sigma_{zz}-\sigma_{xx})^2 + 6(\tau_{xy}^2+\tau_{yz}^2+\tau_{zx}^2)\right]}
$$

이 모델에서 asperity 응력 사이클이 피팅 수명에 미치는 영향이 정량화되었으며, TRB 선접촉 조건에서의 $I_S$ 계산에 직접 적용된다 [ref_32].

#### 4.2.7 특이 커널 적분 응력장

점접촉(point contact) 조건에서는 특이 커널(singular kernel)을 포함하는 적분 방정식으로 응력장을 계산한다 [ref_22]. 이 방법은 접촉 경계 근방에서의 응력 특이성을 정확히 처리하며, 마이크로피팅 수명 예측의 정확도를 향상시킨다. 특이 커널의 수치 적분은 Gauss-Legendre 구적법 또는 적응형 구적법으로 처리된다 [ref_22].

---

### 4.3 응력 사이클 누적과 손상 적분: Rainflow·다축 피로 기준

#### 4.3.1 Over-rolling 동안의 응력 이력

롤러가 레이스웨이 위의 한 표면점을 통과(over-rolling)하는 동안, 해당 점에서의 응력은 시간에 따라 복잡하게 변화한다. 거친표면의 경우, Hertz 거시 응력에 asperity 통과에 의한 국소 응력 변동이 중첩된다. 하나의 over-rolling 사이클에서 표면점 $(x_0, y_0)$의 응력 이력은 다음과 같이 구성된다:

$$
\sigma_{ij}(t) = \sigma_{ij}^\mathrm{Hertz}(t) + \sigma_{ij}^\mathrm{asperity}(t)
$$

여기서 $\sigma_{ij}^\mathrm{Hertz}(t)$는 Hertz 거시 응력의 시간 변화(롤러 진입-통과-이탈), $\sigma_{ij}^\mathrm{asperity}(t)$는 개별 asperity 통과에 의한 국소 응력 변동이다 [ref_17], [ref_21].

SRR이 존재하는 경우, 접선 응력 성분 $\tau_{xz}(t)$가 추가되며, 이는 응력 이력의 비대칭성과 평균 응력(mean stress)을 유발한다 [ref_21].

#### 4.3.2 다축 피로 기준

표면 응력 상태는 일반적으로 다축(multiaxial) 응력이므로, 단축 S-N 곡선을 직접 적용할 수 없다. 다축 피로 기준으로는 다음이 사용된다 [ref_20], [ref_23]:

**Dang Van 기준** (임계면 기반):

$$
\tau_a + k_\mathrm{DV}\cdot \sigma_H^\mathrm{max} \leq \lambda_\mathrm{DV}
$$

여기서 $\tau_a$는 임계면에서의 전단 응력 진폭, $\sigma_H^\mathrm{max}$는 최대 정수압 응력, $k_\mathrm{DV}$, $\lambda_\mathrm{DV}$는 재료 상수이다.

**von Mises 등가 응력 진폭** (단순화 기준):

$$
\sigma_\mathrm{eq,a} = \sqrt{\frac{3}{2} s_{ij,a} s_{ij,a}}
$$

여기서 $s_{ij,a}$는 편차 응력 텐서의 진폭 성분이다. 이 등가 응력을 단축 S-N 곡선에 적용하여 사이클별 손상을 계산한다 [ref_20].

micro-pitting severity index는 이 다축 피로 기준에 기반하여 정량화되며, 실험 결과와의 비교를 통해 모델이 검증되었다 [ref_23].

#### 4.3.3 Rainflow 사이클 계수

over-rolling 동안의 응력 이력 $\sigma(t)$는 비정규(non-stationary) 변동 진폭 신호이다. 이를 피로 손상 계산에 적용하기 위해 **Rainflow 사이클 계수(Rainflow cycle counting)** 방법을 사용한다 [ref_34].

Rainflow 알고리즘은 응력 이력을 일련의 닫힌 히스테리시스 루프로 분해하여, 각 사이클의 진폭 $\sigma_a^{(k)}$와 평균 응력 $\sigma_m^{(k)}$를 추출한다:

$$
\sigma(t) \xrightarrow{\text{Rainflow}} \left\{(\sigma_a^{(k)},\, \sigma_m^{(k)})\right\}_{k=1}^{N_\mathrm{cyc}}
$$

평균 응력 보정은 Goodman 또는 Morrow 관계식으로 수행한다:

$$
\sigma_{a,\mathrm{eff}}^{(k)} = \frac{\sigma_a^{(k)}}{1 - \sigma_m^{(k)}/\sigma_u}
$$

여기서 $\sigma_u$는 재료의 극한 인장 강도이다 [ref_34].

#### 4.3.4 S-N 곡선과 잔류응력 보정

각 사이클의 손상은 S-N 곡선으로부터 허용 사이클 수 $N_f(\sigma_{a,\mathrm{eff}})$를 구하여 계산한다. Tribology Online(ref_33)의 방법은 잔류응력 측정과 수치 접촉 해석을 결합하여 마이크로피팅에 특화된 S-N 곡선을 구성한다 [ref_33]:

$$
N_f = C_\mathrm{SN} \cdot \left(\sigma_{a,\mathrm{eff}}\right)^{-m_\mathrm{SN}}
$$

여기서 $C_\mathrm{SN}$, $m_\mathrm{SN}$은 잔류응력을 포함한 실험 데이터로부터 결정되는 S-N 상수이다. 잔류응력 $\sigma_\mathrm{res}$는 평균 응력으로 작용하여 유효 응력 진폭을 수정한다 [ref_33].

#### 4.3.5 Palmgren-Miner 손상 누적과 $I_S$ 산출

Rainflow 분해로 얻은 사이클 집합에 대해 **Palmgren-Miner 선형 손상 누적 법칙**을 적용한다 [ref_21], [ref_06]:

$$
D = \sum_{k=1}^{N_\mathrm{cyc}} \frac{n_k}{N_f\!\left(\sigma_{a,\mathrm{eff}}^{(k)}\right)}
$$

여기서 $n_k$는 진폭 $\sigma_{a,\mathrm{eff}}^{(k)}$를 갖는 사이클의 발생 횟수, $N_f$는 해당 응력 진폭에서의 파손 사이클 수이다. 손상 $D = 1$에 도달하면 마이크로피팅 개시로 판정한다.

표면손상 함수 $I_S$는 이 손상 누적을 표면 전체에 걸쳐 적분한 형태로 정의된다. 표면점 $(x,y)$에서의 국소 손상 $D(x,y)$를 접촉 영역 $\Omega$에 걸쳐 적분하면:

$$
I_S = \int_\Omega D(x,y)\, dA = \int_\Omega \left[\sum_{k} \frac{n_k(x,y)}{N_f\!\left(\sigma_{a,\mathrm{eff}}^{(k)}(x,y)\right)}\right] dA
$$

> **[주석]** 이 $dA$ 표현은 표면층 두께 또는 손상 커널을 깊이 방향으로 축약한 등가 면적 표현이다. I_S의 기본 정의는 체적 적분 $I_S = \int_V (\sigma_{\text{contact}} - \sigma_{\text{limit}})^n \, dV$ [ref_04][ref_67]이며, 여기서 $D(x,y)$는 표면층 두께에 걸친 깊이 방향 손상 누적을 이미 흡수한 면적 손상 밀도이다. 수치 구현 시 단위 일관성(면적 손상 밀도의 차원 및 정규화 여부)을 명시해야 한다.

이 적분이 SKF GBLM에서 표면 수명 수정계수 $a_\mathrm{SL}$을 결정하는 핵심 양이다 [ref_06], [ref_21].

---

### 4.4 수치 계산 파이프라인: 의사코드와 플로우

아래는 $I_S$ 계산의 전체 수치 절차를 의사코드 형태로 정리한 것이다.

```
[입력]
  - 표면 조도 프로파일: z_1(x,y), z_2(x,y)  [측정 또는 생성]
  - 접촉 조건: W (하중), R' (등가 곡률반경), E' (등가 탄성계수)
  - 운동 조건: u_1, u_2 (표면 속도) → SRR 계산
  - 윤활 조건: 점도 η, 압력-점도계수 α
  - 재료 특성: σ_u, S-N 상수 (C_SN, m_SN), 잔류응력 σ_res

[Step 1: EHL 해석 및 λ 계산]
  1.1 Hamrock-Dowson 공식으로 h_min 계산
  1.2 λ = h_min / sqrt(Ra_1² + Ra_2²)  [ref_20]
  1.3 λ 범위 확인 (혼합윤활 여부 판단)  [ref_19]
  1.4 SRR에 따른 접선 traction 계수 μ 결정

[Step 2: 거친표면 접촉압력 계산 (FFT 기반)]
  2.1 복합 표면 형상 구성: z(x,y) = z_1(x,y) + z_2(x,y)
  2.2 Boussinesq 영향계수 C[i,j] 계산
  2.3 DC-FFT 패딩: C → 2N×2M 확장  [ref_27]
  2.4 반복 수렴 (conjugate gradient):
      while not converged:
          u_z = IFFT(FFT(C) × FFT(p))  [ref_29]
          gap g = z - u_z - δ (rigid body approach)
          p = max(0, p - ω·g)  (relaxation)
          normalize: ∫p dA = W
  2.5 수렴된 접촉압력 p(x,y) 출력

[Step 3: 응력장 계산 (DC-FFT 영향계수)]
  3.1 법선 하중 응력 영향계수 G_ij^(p)(x,y,z) 계산  [ref_28], [ref_31]
  3.2 접선 하중 응력 영향계수 G_ij^(q)(x,y,z) 계산  [ref_28]
  3.3 접선 traction: q(x,y) = μ · p(x,y)
  3.4 for each depth z_k:
          σ_ij(x,y,z_k) = IFFT(FFT(G_ij^(p)) × FFT(p)
                                + FFT(G_ij^(q)) × FFT(q))  [ref_27]
  3.5 von Mises 응력: σ_vM(x,y,z) 계산  [ref_32]
  3.6 특이 커널 처리 (점접촉 경우)  [ref_22]

[Step 4: Over-rolling 응력 이력 구성]
  4.1 롤러 위치 x_r(t) 시간 이력 설정
  4.2 for each surface point (x_0, y_0):
          for each roller position x_r:
              σ_ij(t) = σ_ij^Hertz(t) + σ_ij^asperity(t)  [ref_17]
  4.3 다축 응력 이력 σ_ij(t) 저장

[Step 5: Rainflow 사이클 계수]
  5.1 다축 응력 → 등가 단축 응력 σ_eq(t) 변환  [ref_20], [ref_23]
  5.2 Rainflow 알고리즘 적용:
          {(σ_a^(k), σ_m^(k))} = Rainflow(σ_eq(t))  [ref_34]
  5.3 Goodman 보정: σ_a,eff^(k) = σ_a^(k) / (1 - σ_m^(k)/σ_u)
  5.4 잔류응력 보정 포함  [ref_33]

[Step 6: Palmgren-Miner 손상 누적]
  6.1 for each cycle k:
          N_f^(k) = C_SN · (σ_a,eff^(k))^(-m_SN)  [ref_33]
          d^(k) = n_k / N_f^(k)
  6.2 국소 손상: D(x_0, y_0) = Σ_k d^(k)  [ref_21], [ref_06]

[Step 7: I_S 적분]
  7.1 for all surface points (x,y) in contact area Ω:
          반복 Step 4–6
  7.2 I_S = ∫∫_Ω D(x,y) dA  (수치 적분)  [ref_06], [ref_21]
  7.3 표면 수명 수정계수 a_SL = f(I_S) 계산

[출력]
  - I_S: 표면손상 적분값
  - D(x,y): 표면 손상 분포 맵
  - a_SL: SKF GBLM 수명 수정계수
```

**그림 4.1** (개념도): 입력 → EHL/λ 계산 → DC-FFT 접촉압력 → 응력장 → Rainflow 분해 → Miner 누적 → $I_S$ 출력의 순차적 파이프라인.

| 단계 | 핵심 알고리즘 | 계산 복잡도 | 주요 참고문헌 |
|------|-------------|------------|-------------|
| 접촉압력 | FFT 기반 탄성 접촉 | $O(N^2 \log N)$ | [ref_29] |
| 응력장 (법선) | DC-FFT + Green 함수 | $O(N^2 \log N)$ | [ref_27], [ref_31] |
| 응력장 (접선) | DC-FFT + Mindlin-Cerruti | $O(N^2 \log N)$ | [ref_28] |
| 응력장 (특이 커널) | 적응형 구적법 | $O(N^2)$–$O(N^3)$ | [ref_22] |
| 사이클 계수 | Rainflow | $O(N_t \log N_t)$ | [ref_34] |
| 손상 누적 | Palmgren-Miner | $O(N_\mathrm{cyc})$ | [ref_21], [ref_06] |

**표 4.1** $I_S$ 계산 파이프라인의 각 단계별 알고리즘과 계산 복잡도 ($N$: 격자 크기, $N_t$: 시간 스텝 수).

---

### 소결

본 섹션에서는 $I_S$ 적분의 실제 수치 계산 절차를 세 층위로 체계화하였다. 물리 모델 측면에서, Morales-Espejel & Brizmer 모델은 λ비와 SRR을 입력으로 하여 asperity 국소 응력을 정량화하고, 표면 피로와 mild wear의 경쟁을 통해 마이크로피팅 진행을 예측한다 [ref_17], [ref_18]. 수치 절차 측면에서, DC-FFT 기반 영향계수 방법은 거친표면 접촉압력과 3차원 응력장을 $O(N^2 \log N)$ 복잡도로 효율적으로 계산한다 [ref_27], [ref_28], [ref_29], [ref_31]. 손상 누적 측면에서, Rainflow 사이클 계수와 Palmgren-Miner 법칙의 결합이 비정규 응력 이력으로부터 $I_S$를 산출하는 표준 절차를 제공한다 [ref_21], [ref_34]. 이 파이프라인은 TRB 마이크로피팅 수명 계산의 핵심 수치 엔진으로 기능한다.

---

---

## 5. 미지 상수 결정·교정(Calibration) 방법론

### 도입

GBLM 표면손상 함수 $I_S$의 핵심 미지 상수—피로한계 응력 $\sigma_\text{limit}$과 손상 지수 $n$—는 순수 이론만으로 결정할 수 없다. Morales-Espejel & Gabelli(2015)는 원문에서 "surface creep-damage model은 베어링 내구성 시험으로 먼저 교정(calibrate)되어야 한다"고 명시하였으며 [ref_01], 이후 연구들도 동일한 입장을 유지한다 [ref_05]. 이 절에서는 미지 상수를 결정하는 세 가지 독립적 경로—**(1) 내구성 시험 기반 곡선 적합(최소제곱) 교정**, **(2) 역해석(Inverse Method / IFORM) 기반 파라미터 식별**, **(3) 표준 시험(FE8·트윈디스크·MPR)과 ISO/TR 15144 검증**—를 체계적으로 정식화하고, 각 경로의 장단점을 비교한다. 세 경로의 공통 핵심 메시지는 **"$\sigma_\text{limit}$과 $n$을 직접 계산하는 대신 교정(calibration)으로 회피한다"**는 것이다.

---

### 5.1 내구성 시험 기반 곡선 적합(최소제곱) 교정

#### 5.1.1 교정의 필요성과 원문 취지

Morales-Espejel & Gabelli(2015)의 GBLM 원문 [ref_01]은 표면손상 함수를

$$
I_S = \int_V \left(\sigma_\text{contact} - \sigma_\text{limit}\right)^n \, dV
$$

로 정의하면서, $\sigma_\text{limit}$과 $n$이 재료·윤활 조건에 따라 달라지는 **경험 상수**임을 명시한다. 이 상수들은 접촉역학 이론만으로 유도되지 않으며, 반드시 **베어링 내구성 시험(endurance test) 데이터**에 대한 적합(fitting) 과정을 통해 결정된다 [ref_01]. 2020년 열 효과 확장 논문 [ref_05]에서도 동일한 교정 절차가 유지되며, 온도 의존적 $n(T)$ 결정을 위해 고온 내구성 시험 데이터가 추가로 요구됨을 보인다.

#### 5.1.2 Lieblein-Zelen 최소제곱 정식화

베어링 피로 수명 파라미터 추정의 통계적 기초는 Lieblein & Zelen(1956) [ref_37]의 최소제곱법에서 비롯된다. 이 방법은 Weibull 분포를 따르는 피로 수명 데이터에 대해 형상 모수(shape parameter)와 척도 모수(scale parameter)를 최소제곱으로 추정하는 절차를 제공한다.

GBLM 교정에 적용하면 다음과 같이 정식화된다. $N_\text{exp}$개의 내구성 시험 결과 $\{(L_{10,i}^\text{exp}, \, \text{조건}_i)\}$가 주어졌을 때, 모델 예측 수명 $L_{10,i}^\text{pred}(\sigma_\text{limit}, n)$과의 잔차 제곱합을 최소화하는 상수 쌍을 구한다:

$$
\min_{\sigma_\text{limit},\, n} \; \mathcal{L}(\sigma_\text{limit}, n) = \sum_{i=1}^{N_\text{exp}} \left[ \ln L_{10,i}^\text{pred}(\sigma_\text{limit}, n) - \ln L_{10,i}^\text{exp} \right]^2
$$

수명의 로그 변환은 Weibull 분포의 대수 선형성을 활용하여 잔차의 정규성을 확보하기 위한 것이다 [ref_37]. 실제 구현에서는 다음 단계를 따른다:

1. **시험 조건 설계**: 다양한 하중($P$), 윤활 조건($\kappa$), 표면 조도($R_a$)를 조합한 내구성 시험 행렬 구성
2. **$I_S$ 수치 계산**: 각 조건에서 DC-FFT 기반 접촉 응력 적분으로 $I_S(\sigma_\text{limit}, n)$ 계산
3. **수명 예측**: $L_{10}^\text{pred} \propto I_S^{-1}$ 관계로 예측 수명 산출
4. **최소제곱 최적화**: 2차원 파라미터 공간 $(\sigma_\text{limit}, n)$에서 $\mathcal{L}$ 최소화 (Nelder-Mead 또는 Levenberg-Marquardt 알고리즘)
5. **신뢰구간 추정**: Lieblein-Zelen 분산 공식으로 $\hat{\sigma}_\text{limit} \pm \delta\sigma$, $\hat{n} \pm \delta n$ 산출 [ref_37]

#### 5.1.3 교정 결과의 물리적 해석

교정을 통해 얻어진 $\sigma_\text{limit}$은 표면 접촉 피로한계 응력으로, 문헌에서는 AISI 52100 베어링강 기준 약 1.4–1.5 GPa 수준이 보고된다 [ref_71]. 손상 지수 $n$은 통상 4–6 범위에서 결정되며 [ref_67], 이는 부표면 피로 지수 $e \approx 4$와 유사하지만 표면 접촉 응력의 고도 비선형성을 반영하여 더 높은 값을 취할 수 있다. 열 효과가 포함된 경우 $n$은 온도의 함수로 확장된다 [ref_05].

#### 5.1.4 소결

최소제곱 교정은 통계적으로 견고하고 구현이 명확하다는 장점이 있으나, 충분한 수의 내구성 시험 데이터(통상 $N_\text{exp} \geq 20$)가 필요하며, 시험 비용과 시간이 크다는 단점이 있다. 또한 교정 결과는 특정 재료·윤활 계열에 한정되므로 외삽(extrapolation) 시 불확실성이 증가한다.

---

### 5.2 역해석(Inverse Method / IFORM) 기반 파라미터 식별

#### 5.2.1 역문제의 정식화

역해석(inverse method)은 관측된 실험 결과로부터 모델 파라미터를 역산하는 방법론이다 [ref_40]. GBLM 상수 결정에서 역문제는 다음과 같이 정의된다: 실험적으로 관측된 마이크로피팅 손상 상태 $\mathbf{d}^\text{obs}$가 주어졌을 때, 이를 재현하는 파라미터 벡터 $\boldsymbol{\theta} = (\sigma_\text{limit}, n)$을 찾는 것이다.

목적함수는 실험-예측 오차의 가중 제곱합으로 정의된다:

$$
\min_{\boldsymbol{\theta}} \; \mathcal{J}(\boldsymbol{\theta}) = \sum_{j} w_j \left[ d_j^\text{pred}(\boldsymbol{\theta}) - d_j^\text{obs} \right]^2
$$

여기서 $d_j$는 $j$번째 관측점(예: 특정 사이클 수에서의 마이크로피팅 면적 비율, 피트 깊이, 또는 손상 지수)이며, $w_j$는 측정 불확실성에 반비례하는 가중치이다 [ref_39].

물리적 제약 조건은 다음과 같이 부과된다 [ref_40]:

$$
\sigma_\text{limit} > 0, \quad n \in [4, 6], \quad \sigma_\text{limit} \leq p_{0,\max}
$$

여기서 $p_{0,\max}$는 시험 조건에서의 최대 Hertz 접촉 압력이다. 이 제약은 물리적으로 의미 없는 해(예: 음의 피로한계, 극단적 지수)를 배제한다.

#### 5.2.2 IFORM(역 1차 신뢰도 방법) 적용

역 1차 신뢰도 방법(Inverse First-Order Reliability Method, IFORM)은 확률론적 피로 수명 예측에서 파라미터를 역산하는 효율적 기법이다 [ref_38]. IFORM의 핵심 아이디어는 목표 신뢰도 수준(예: $L_{10}$ 수명에서 90% 생존 확률)에 대응하는 설계점(design point)을 표준 정규 공간에서 탐색하는 것이다.

GBLM에 적용하면:

1. **표준화 변환**: $\boldsymbol{\theta}$를 표준 정규 변수 $\mathbf{u}$로 변환
2. **한계상태 함수 정의**: $g(\mathbf{u}) = L_{10}^\text{pred}(\boldsymbol{\theta}(\mathbf{u})) - L_{10}^\text{target} = 0$
3. **MPP(Most Probable Point) 탐색**: $\|\mathbf{u}\|$를 최소화하면서 $g(\mathbf{u}) = 0$을 만족하는 점 탐색
4. **파라미터 역산**: MPP에서의 $\boldsymbol{\theta}^*$가 목표 수명을 재현하는 최적 파라미터 [ref_38]

이 방법은 단일 내구성 시험 결과만으로도 파라미터 추정이 가능하다는 장점이 있으며, 특히 데이터가 제한적인 상황에서 유용하다.

#### 5.2.3 수치 시뮬레이션과의 결합

역해석은 수치 시뮬레이션과 결합될 때 더욱 강력해진다 [ref_39]. 구체적으로, 각 파라미터 후보 $\boldsymbol{\theta}$에 대해 DC-FFT 기반 접촉 응력 시뮬레이션을 수행하여 $I_S(\boldsymbol{\theta})$를 계산하고, 이를 실험 관측값과 비교하는 반복 루프를 구성한다:

$$
\boldsymbol{\theta}^{(k+1)} = \boldsymbol{\theta}^{(k)} - \alpha \nabla_{\boldsymbol{\theta}} \mathcal{J}(\boldsymbol{\theta}^{(k)})
$$

여기서 $\alpha$는 스텝 크기, $\nabla_{\boldsymbol{\theta}} \mathcal{J}$는 목적함수의 기울기이다. 기울기는 유한차분(finite difference) 또는 수반 방법(adjoint method)으로 계산된다 [ref_39].

#### 5.2.4 가속수명시험(ALT)과 등가 손상법

실제 베어링 수명 시험은 수천 시간이 소요되므로, 가속수명시험(Accelerated Life Test, ALT)을 통해 시험 시간을 단축하고 역해석에 필요한 데이터를 효율적으로 획득한다 [ref_46]. 등가 손상법(Equivalent Damage Method, EDM)은 가속 조건(고하중, 고온)에서의 손상을 실제 운전 조건의 등가 손상으로 변환하는 방법으로, 다음 관계를 이용한다:

$$
D_\text{actual} = D_\text{accel} \cdot \left(\frac{I_{S,\text{actual}}}{I_{S,\text{accel}}}\right)
$$

Kim et al.(2025) [ref_46]은 기어·베어링 ALT 설계 최적화에 EDM을 적용하여, 시험 시간을 약 60–70% 단축하면서도 파라미터 추정 정확도를 유지할 수 있음을 보였다. 이 접근은 역해석 루프의 입력 데이터 품질을 높이는 데 기여한다.

#### 5.2.5 소결

역해석 방법은 제한된 실험 데이터로도 파라미터 식별이 가능하고, 확률론적 불확실성을 명시적으로 처리할 수 있다는 장점이 있다. 그러나 목적함수의 비볼록성(non-convexity)으로 인해 국소 최솟값(local minimum)에 수렴할 위험이 있으며, 수치 시뮬레이션 비용이 높다. 물리적 제약 조건의 적절한 설정이 해의 유일성 확보에 필수적이다 [ref_40].

---

### 5.3 표준 시험(FE8·트윈디스크·MPR)과 ISO/TR 15144 검증

#### 5.3.1 표준 시험 리그 개관

미지 상수의 교정 및 검증에는 국제적으로 표준화된 시험 리그가 활용된다. 주요 시험 방법은 다음과 같다 [ref_47]:

- **FE8 시험 (DIN 51819)**: 롤러 베어링 내구성 시험 표준. 축방향 하중 하에서 마이크로피팅·스미어링 손상 모드를 평가한다 [ref_42].
- **트윈디스크(Twin-Disc) 시험**: 두 개의 디스크가 구름·미끄럼 접촉을 모사하는 시험 리그. 접촉 압력과 SRR(Slide-to-Roll Ratio)을 독립적으로 제어할 수 있어 파라미터 식별에 유리하다 [ref_44].
- **MPR(Mini Traction Machine / Plint)**: 소형 구름·미끄럼 접촉 시험기. 윤활 조건과 표면 조도의 영향을 신속하게 평가한다 [ref_47].

#### 5.3.2 FE8 시험 기반 상수 결정

FE8 시험(DIN 51819-02/03)은 표준화된 하중 조건과 시험 절차를 제공하여 재현성 높은 마이크로피팅 데이터를 생성한다 [ref_42]. Micro-pitting Severity Index(MSI)는 FE8 시험 후 레이스웨이 표면의 마이크로피팅 면적 비율과 피트 깊이를 정량화하는 지표로 정의된다 [ref_43]:

$$
\text{MSI} = \frac{A_\text{micropitting}}{A_\text{total}} \times \bar{d}_\text{pit}
$$

여기서 $A_\text{micropitting}$은 마이크로피팅 발생 면적, $A_\text{total}$은 전체 접촉 면적, $\bar{d}_\text{pit}$는 평균 피트 깊이이다. MSI를 $I_S(\sigma_\text{limit}, n)$의 함수로 표현하면, FE8 시험 데이터로부터 최소제곱 교정이 가능하다 [ref_43].

#### 5.3.3 트윈디스크 시험 기반 손상 함수 개발

트윈디스크 시험은 접촉 압력을 1.2–1.6 GPa 범위에서 체계적으로 변화시키면서 마이크로피팅 손상을 측정할 수 있어 [ref_44], $\sigma_\text{limit}$ 결정에 특히 유용하다. 접촉 압력이 $\sigma_\text{limit}$ 이하인 조건에서는 손상이 발생하지 않아야 하므로, 손상 발생 임계 압력이 $\sigma_\text{limit}$의 하한을 제공한다.

트윈디스크 시험 데이터로부터 손상 함수를 직접 개발하는 방법은 ref_41에서 정식화되었다. 두 디스크 접촉에서의 RCF 손상 함수는 다음과 같이 구성된다:

$$
D(N) = \int_0^N \left[ \frac{\sigma_\text{contact}(x, z, t) - \sigma_\text{limit}}{\sigma_\text{ref}} \right]^n dt
$$

여기서 $\sigma_\text{ref}$는 참조 응력, $N$은 사이클 수이다. 다양한 압력 조건에서의 시험 데이터에 이 함수를 적합하면 $(\sigma_\text{limit}, n)$을 동시에 결정할 수 있다 [ref_41].

#### 5.3.4 S-N 곡선과 잔류응력 기반 검증

S-N 곡선 기반 마이크로피팅 수명 추정법 [ref_33]은 표준 시험으로 결정된 상수를 독립적으로 검증하는 수단을 제공한다. 이 방법은 잔류응력 측정(X선 회절)과 수치 접촉 분석을 결합하여 유효 응력 진폭을 산출하고, 이를 S-N 곡선에 적용하여 마이크로피팅 수명을 예측한다:

$$
N_f = C \cdot \left(\Delta\sigma_\text{eff}\right)^{-m}
$$

여기서 $\Delta\sigma_\text{eff} = \Delta\sigma_\text{applied} + \sigma_\text{residual}$은 잔류응력이 보정된 유효 응력 진폭, $C$와 $m$은 S-N 곡선 상수이다 [ref_33]. GBLM 예측 수명과 S-N 기반 예측 수명의 일치 여부가 $(\sigma_\text{limit}, n)$ 교정 결과의 타당성을 검증한다.

#### 5.3.5 ISO/TR 15144 절차와의 비교 검증

ISO/TR 15144는 기어 마이크로피팅 예측을 위한 표준 절차를 제공하며, 윤활막 두께 기반 안전계수 $S_\lambda$를 핵심 지표로 사용한다 [ref_45]:

$$
S_\lambda = \frac{\lambda_\text{GF,min}}{\lambda_\text{GF,lim}}
$$

여기서 $\lambda_\text{GF,min}$은 최소 윤활막 두께 비, $\lambda_\text{GF,lim}$은 마이크로피팅 발생 임계 윤활막 두께 비이다. ISO/TR 15144 절차는 GBLM과 독립적인 예측 경로를 제공하므로, 두 방법의 결과 비교가 교정 상수의 타당성 검증에 활용된다 [ref_45].

실험적 평가 결과, ISO/TR 15144는 윤활 조건이 지배적인 경우에는 양호한 예측 성능을 보이나, 표면 응력 집중이 중요한 경우(예: 높은 SRR, 거친 표면)에는 GBLM의 $I_S$ 기반 접근이 더 정확한 것으로 보고된다 [ref_45].

#### 5.3.6 소결

표준 시험 기반 검증은 교정된 상수의 물리적 타당성을 독립적으로 확인하는 필수 단계이다. FE8는 재현성, 트윈디스크는 파라미터 제어성, MPR은 신속성 면에서 각각 장점을 가지며, 세 시험을 상호 보완적으로 활용하는 것이 권장된다 [ref_47].

---

### 5.4 세 경로의 종합 비교

아래 표는 세 가지 교정 경로의 주요 특성을 비교한다.

| 항목 | 경로 1: 최소제곱 교정 | 경로 2: 역해석(IFORM) | 경로 3: 표준 시험 검증 |
|---|---|---|---|
| **주요 입력** | 내구성 시험 수명 데이터 | 손상 상태 관측값 | FE8/트윈디스크/MPR 데이터 |
| **필요 데이터 수** | 많음 ($\geq 20$) | 적음 (수 개) | 중간 |
| **통계적 기반** | Lieblein-Zelen 최소제곱 [ref_37] | IFORM 확률론 [ref_38] | MSI, S-N 곡선 [ref_43, ref_33] |
| **계산 비용** | 중간 | 높음 (반복 시뮬) | 낮음 (표준화) |
| **물리적 제약** | 사후 검토 | 명시적 부과 [ref_40] | 시험 조건으로 내재 |
| **외삽 신뢰도** | 낮음 | 중간 | 높음 (표준 조건) |
| **주요 한계** | 시험 비용·시간 | 국소 최솟값 위험 | 특정 시험 조건 한정 |
| **핵심 참고문헌** | [ref_01, ref_05, ref_37] | [ref_38, ref_39, ref_40, ref_46] | [ref_41, ref_42, ref_43, ref_44, ref_45, ref_47, ref_33] |

**핵심 해법 요약**: 세 경로 모두 $\sigma_\text{limit}$과 $n$을 이론적으로 직접 계산하는 대신, **실험 데이터와 모델 예측의 오차를 최소화하는 교정(calibration)으로 회피**한다는 공통 전략을 취한다 [ref_01]. 이는 GBLM의 실용적 적용에서 가장 중요한 방법론적 특징이다.

실제 적용에서는 세 경로를 순차적으로 활용하는 것이 권장된다: 먼저 표준 시험(경로 3)으로 초기 상수 범위를 설정하고, 역해석(경로 2)으로 정밀 식별한 후, 내구성 시험 최소제곱 교정(경로 1)으로 최종 확정하는 방식이다. 이 과정에서 가속수명시험과 등가 손상법 [ref_46]을 활용하면 전체 교정 비용을 크게 절감할 수 있다.

---

---

## 6. TRB 적용과 대안 표면피로 수명 모델 비교

원추형 롤러 베어링(Tapered Roller Bearing, TRB)은 축방향·반경방향 복합하중을 동시에 지지하는 구조적 특성상, 롤러–레이스웨이 선접촉 압력 분포가 이상적인 Hertz 분포에서 크게 벗어나며 에지 응력 집중, 정렬 불량, rib–롤러 단부 2차 접촉 등 복수의 표면손상 경로가 공존한다. 본 섹션에서는 GBLM의 표면손상 함수 $I_S$를 TRB 기하에 구체화하는 데 필요한 접촉 입력 조건을 정의하고(6.1), TRB 고유의 미끄럼–마이크로피팅 메커니즘을 분석하며(6.2), 마지막으로 Ioannides–Harris(I-H), ISO/TS 16281, Zaretsky, Tallian 모델과 GBLM을 공개 상수 관점에서 비교한다(6.3).

---

### 6.1 TRB 선접촉·에지 응력·로그 크라우닝과 $I_S$ 입력 조건

#### 6.1.1 TRB 선접촉의 비-Hertz 압력 분포

TRB 롤러–레이스웨이 접촉은 명목상 선접촉(line contact)이지만, 롤러 단부 기하, 레이스웨이 곡률 변화, 정렬 불량이 복합되면 접촉 압력 분포는 이상적인 Hertz 반타원 분포에서 현저히 이탈한다. Lim et al.의 준정적 다자유도 TRB 모델[ref_48]은 롤러 길이 방향으로 비균일 슬라이스 하중을 계산하여 에지 부근의 압력 급등(edge loading)을 정량화하였다. 이 모델에서 롤러 $j$의 위치 $x$에서의 접촉 압력 $p(x)$는 슬라이스 탄성 변형 방정식의 반복 수렴으로 구해지며, 에지 근방에서 Hertz 예측 대비 수십 % 이상의 과압이 발생함을 보였다[ref_48].

FEM·실험·해석 비교 연구[ref_49]는 이중열 TRB의 내륜–롤러 및 외륜–롤러 접촉 응력을 세 방법으로 교차 검증하였다. 해석 모델은 Hertz 이론 기반이지만 에지 보정 항을 포함하며, FEM 결과와 ±5% 이내에서 일치하였다[ref_49]. 이는 $I_S$ 적분의 접촉 압력 입력으로 비-Hertz 수치 해를 사용해야 함을 시사한다.

유한 선접촉(finite line contact) 문제에 대한 Boussinesq 반공간 기반 하중 계산 모델[ref_61]은 롤러 단부 에지 보정을 명시적으로 포함하며, 베어링 동역학 시뮬레이션에서 각 롤러의 접촉 하중을 효율적으로 계산하는 방법을 제시하였다. 이 모델은 $I_S$ 수치 적분의 접촉 대역 입력으로 직접 활용 가능하다[ref_61].

Hertz 선접촉 이론의 공학적 정식화[ref_56]에 따르면, 크라우닝이 없는 이상적 선접촉의 최대 Hertz 압력 $p_0$와 반접촉폭 $b$는 다음과 같다:

$$
p_0 = \sqrt{\frac{F E^*}{\pi L R^*}}, \quad b = \sqrt{\frac{4 F R^*}{\pi L E^*}}
$$

여기서 $F$는 롤러 하중, $L$은 유효 접촉 길이, $R^*$는 등가 곡률 반경, $E^*$는 등가 탄성 계수이다[ref_56]. 그러나 TRB에서는 크라우닝 형상과 에지 효과로 인해 실제 압력 분포가 이 식에서 크게 벗어나므로, $I_S$ 적분의 입력 압력 $p(x,y)$는 반드시 비-Hertz 수치 해 또는 에지 보정 해석 모델로부터 취득해야 한다[ref_48][ref_49][ref_61].

#### 6.1.2 에지 응력과 로그 크라우닝에 의한 $I_S$ 입력 조건 정의

에지 응력 집중은 TRB 피로 수명의 주요 저하 요인이다. Schaeffler의 기술 문서[ref_53]는 정렬 불량 TRB에서 레이스웨이 에지 부근의 국소 과하중이 에지 피로(edge fatigue)를 유발하는 메커니즘을 상세히 기술하고 있다. 에지 피로는 $I_S$ 적분 영역 내에서 압력 피크가 집중되는 현상으로, 이를 방치하면 $I_S$ 값이 과대 추정되어 수명이 과소 예측된다[ref_53].

로그 크라우닝(logarithmic crowning)은 에지 응력을 효과적으로 제거하는 표준 설계 수단이다. TRB 롤러 프로파일에 대한 로그 크라우닝 연구[ref_50]는 정렬 불량 조건에서 로그 크라우닝이 접촉 압력 분포를 균일화하고 에지 응력 피크를 제거함을 보였다. 최적 로그 크라우닝 파라미터는 정렬 불량 각도 $\delta$와 하중 $F$의 함수로 결정되며, 크라우닝 적용 후 접촉 압력 분포는 Hertz 분포에 근접하게 된다[ref_50].

원통형 롤러에 대한 유사 연구[ref_51]는 양측 틸트(bilateral tilt) 조건에서 로그 크라우닝이 에지 효과를 제거하고 부표면 응력을 개선함을 확인하였다. 이 결과는 TRB에도 직접 적용 가능하며, 로그 크라우닝 적용 시 $I_S$ 적분의 접촉 압력 입력이 에지 피크 없이 균일한 분포를 가짐을 의미한다[ref_51].

$I_S$ 적분의 접촉 대역 입력 조건을 정의하면 다음과 같다:

$$
I_S = \int_V \left(\sigma_{\mathrm{contact}}(x,y,z) - \sigma_{\mathrm{limit}}\right)^n_+ \, dV
$$

여기서 적분 영역 $V$는 접촉 반폭 $b$의 약 0.5–1배 깊이까지의 표면층이며[ref_09], 접촉 압력 $p(x,y)$는 다음 우선순위로 결정된다:

1. **비-Hertz 수치 해**: 에지 보정 포함 슬라이스 모델[ref_48] 또는 Boussinesq 유한 선접촉 모델[ref_61]
2. **로그 크라우닝 적용 후**: 에지 피크 제거된 균일 분포[ref_50][ref_51]
3. **이상적 Hertz 선접촉**: 크라우닝 완전 적용 시 근사[ref_56]

복합하중·정렬 불량 하의 이중열 TRB[ref_57]에서는 각 롤러의 접촉 하중이 반복 평형 해석으로 결정되며, 이 하중 분포가 $I_S$ 적분의 입력으로 사용된다. 정렬 불량 각도가 증가할수록 하중 분포의 불균일성이 커지고 $I_S$ 값이 증가하여 수명이 단축된다[ref_57].

TRB 피로 수명의 통계적 분포[ref_58]는 Weibull 분포를 따르며, 고신뢰도 영역에서 응력 집중의 영향이 더욱 두드러진다. 이는 $I_S$ 기반 수명 예측에서 에지 응력 처리의 중요성을 통계적으로 뒷받침한다[ref_58].

---

### 6.2 TRB 미끄럼–마이크로피팅과 rib–roller end 2차 접촉

#### 6.2.1 TRB 선접촉에서의 미끄럼 분포와 마이크로피팅

TRB 롤러–레이스웨이 접촉에서는 롤러의 원추 기하로 인해 접촉 길이 방향으로 미끄럼 속도가 불균일하게 분포한다. 롤러의 순수 구름 조건(pure rolling)은 접촉 길이의 특정 위치에서만 성립하며, 그 외 위치에서는 미끄럼이 발생한다. 이 미끄럼 분포는 마이크로피팅 발생 위치와 직접 연관된다.

고속 TRB의 표면 피로 메커니즘 연구[ref_54]는 슬라이딩 접촉에서 trailing edge asperity의 응력 집중이 표면 피로의 주요 개시 메커니즘임을 실험과 이론으로 규명하였다. 고속 조건에서 EHL 필름이 형성되더라도 asperity 접촉이 잔존하면 국소 응력 집중이 발생하며, 이 응력이 $I_S$ 적분의 피적분 함수를 지배한다[ref_54]. 특히 trailing edge에서의 응력 집중은 leading edge 대비 현저히 높아, 마이크로피팅이 trailing edge 방향으로 우선 발생하는 경향을 설명한다[ref_54].

미끄럼–구름 비율(Slide-to-Roll Ratio, SRR)이 마이크로피팅에 미치는 영향[ref_63]은 SRR이 마이크로피팅 손상 범위의 주요 설계 인자임을 보였다. SRR이 증가할수록 마이크로피팅 손상 면적이 증가하며, 이는 $I_S$ 적분값의 증가로 직결된다[ref_63]. TRB에서 SRR은 접촉 위치, 롤러 직경 분포, 회전 속도의 함수로 결정된다.

마이크로피팅에 영향을 미치는 인자들의 종합 평가[ref_62]에 따르면, 접촉 압력은 마이크로피팅 **개시**에, 속도와 SRR은 마이크로피팅 **전파**에 지배적인 역할을 한다. 이 구분은 $I_S$ 적분의 물리적 해석과 일치한다: $I_S$는 접촉 응력이 $\sigma_{\mathrm{limit}}$를 초과하는 영역의 적분이므로 개시 조건을 포착하며, 전파는 별도의 손상 누적 모델이 필요하다[ref_62].

#### 6.2.2 TRB 미끄럼–마이크로피팅에 대한 $I_S$ 적용 절차

sec-4에서 정립된 $I_S$ 계산 절차를 TRB 기하에 적용하면 다음과 같다:

**Step 1: 접촉 압력 분포 계산**
- 비-Hertz 슬라이스 모델[ref_48] 또는 Boussinesq 유한 선접촉 모델[ref_61]로 $p(x,y)$ 계산
- 로그 크라우닝 적용 시 에지 피크 제거[ref_50][ref_51]

**Step 2: EHL 필름 두께 및 $\lambda$ 비 계산**
- 선접촉 EHL 공식으로 중앙 필름 두께 $h_c$ 계산
- $\lambda = h_c / \sqrt{R_{q1}^2 + R_{q2}^2}$ 계산

**Step 3: 미끄럼 속도 분포 계산**
- TRB 원추 기하에서 접촉 길이 방향 미끄럼 속도 $u_s(x)$ 계산
- SRR$(x) = u_s(x) / u_r(x)$ 분포 결정[ref_63]

**Step 4: 표면 응력 계산**
- DC-FFT 기반 표면/부표면 응력 계산
- Asperity 응력 집중 포함[ref_54]

**Step 5: $I_S$ 적분**
$$
I_S = \int_V \left(\sigma_{\mathrm{contact}} - \sigma_{\mathrm{limit}}\right)^n_+ \, dV
$$
- 적분 영역: 접촉 반폭의 0.5–1배 깊이 표면층
- 지수 $n$: 4–6 (미공개 SKF 내부 상수)

#### 6.2.3 rib–roller end 2차 접촉과 $I_S$ 확장

TRB의 고유한 특징 중 하나는 롤러 대단부(large end)와 내륜 플랜지(rib) 사이의 2차 접촉이다. 이 접촉은 축방향 하중을 지지하는 기능을 하지만, 동시에 마이크로피팅과 마모의 추가적인 발생 위치가 된다.

Rib–롤러 단부 접촉의 부분 EHL 해석[ref_55]은 평균 Reynolds 방정식을 이용하여 이 2차 접촉의 필름 두께와 압력 분포를 계산하였다. 이 접촉은 점접촉(elliptical contact)에 가까우며, 접촉 압력은 롤러–레이스웨이 주접촉보다 낮지만 미끄럼 속도가 현저히 높다[ref_55]. 높은 미끄럼 속도는 EHL 필름 형성을 촉진하지만, 동시에 열 발생과 표면 전단 응력을 증가시킨다.

$I_S$ 계산을 rib–roller end 2차 접촉에 확장하면:

$$
I_{S,\mathrm{total}} = I_{S,\mathrm{main}} + \alpha \cdot I_{S,\mathrm{rib}}
$$

여기서 $\alpha$는 2차 접촉의 기여 가중치로, 접촉 면적과 응력 수준의 비율로 결정된다. 그러나 $\alpha$의 정확한 값은 SKF GBLM에서 공개되지 않았으며, 실무에서는 주접촉 $I_{S,\mathrm{main}}$만을 사용하는 경우가 일반적이다[ref_55].

---

### 6.3 대안 모델(I-H, ISO/TS 16281, Zaretsky, Tallian)과 공개 상수 비교

#### 6.3.1 Ioannides–Harris(I-H) 모델

Ioannides와 Harris(1986)[ref_64]는 Lundberg–Palmgren(L-P) 모델의 한계를 극복하기 위해 피로한계 응력(fatigue limit stress) $\sigma_0$를 도입한 새로운 수명 모델을 제안하였다:

$$
\ln\frac{1}{S} \propto \int_V \frac{(\tau - \tau_0)^e}{z^h} \, dV
$$

여기서 $\tau$는 임계 전단 응력, $\tau_0$는 피로한계 전단 응력, $e$는 응력 지수, $z$는 깊이, $h$는 깊이 지수이다. I-H 모델의 핵심 공개 상수는 $\tau_0 \approx 276\,\mathrm{MPa}$, $e \approx 4$이다[ref_64].

L-P 모델과의 직접 비교[ref_65]에서 I-H 모델은 고하중 조건에서 L-P 대비 더 긴 수명을 예측하며, 이는 피로한계 이하의 응력 영역이 수명에 기여하지 않기 때문이다. 그러나 피로한계 $\tau_0$의 정확한 값은 재료와 시험 조건에 따라 변동하며, 이것이 I-H 모델의 주요 불확실성 요인이다[ref_65].

I-H 모델은 ISO 281:2007의 수명 수정계수 $a_{\mathrm{ISO}}$의 이론적 기초가 되었으며, 이후 ISO/TS 16281로 발전하였다.

#### 6.3.2 ISO/TS 16281 모델

ISO/TS 16281은 I-H 이론을 표준화한 수명 계산 방법으로, 수정 기준 정격 수명을 다음과 같이 정의한다[ref_69]:

$$
L_{nm} = \left(\frac{C}{P}\right)^p \cdot a_1 \cdot a_{\mathrm{ISO}}
$$

여기서:
- $p = 10/3$ (롤러 베어링), $p = 3$ (볼 베어링)
- $a_1$: 신뢰도 수정계수 (공개 표)
- $a_{\mathrm{ISO}} = f(\kappa, e_C)$: 윤활·오염 통합 수정계수

$a_{\mathrm{ISO}}$는 점도비 $\kappa = \nu/\nu_1$, 오염 계수 $e_C$, 피로하중한계 $C_u$의 함수로 계산된다[ref_70]:

$$
a_{\mathrm{ISO}} = f\!\left(\kappa, e_C, \frac{C_u}{P}\right)
$$

ISO/TS 16281의 공개 상수 체계는 다음과 같다[ref_69][ref_70]:

| 파라미터 | 공개 여부 | 값/범위 |
|---------|----------|---------|
| $p$ (롤러) | 공개 | $10/3 \approx 3.33$ |
| $a_1$ | 공개 표 | $a_1(90\%) = 1.0$, $a_1(95\%) = 0.64$ |
| $a_{\mathrm{ISO}}$ 계산식 | 공개 | ISO 281:2007 부속서 A |
| $C_u$ (피로하중한계) | 공개 (카탈로그) | 베어링 종류별 |
| $\kappa$ 임계값 | 공개 | $\kappa < 0.4$: 심각 손상 위험 |

ISO/TS 16281은 표면 손상과 부표면 손상을 명시적으로 분리하지 않으며, $a_{\mathrm{ISO}}$가 윤활 조건을 통해 간접적으로 표면 손상 영향을 반영한다. 이것이 GBLM과의 근본적 차이점이다[ref_69].

#### 6.3.3 Zaretsky 모델

Zaretsky의 응력–수명 모델[ref_66]은 L-P 모델의 변형으로, 최대 응력과 수명의 관계를 다음과 같이 표현한다:

$$
L \propto \left(\sigma_{\max}\right)^{-e}, \quad e \approx 4
$$

Zaretsky 모델은 피로한계를 도입하지 않으며, 모든 응력 수준에서 피로 손상이 누적된다고 가정한다. NASA 연구[ref_66]에서 L-P, I-H, Zaretsky 모델의 비교 결과, 피로한계 이하 응력 조건에서 세 모델의 예측 수명 차이가 수 배에 달할 수 있음이 보고되었다[ref_66].

Zaretsky 모델의 주요 특징은 응력 정규화(stress normalization)를 통해 다양한 재료와 접촉 형태에 적용 가능하다는 점이다. 그러나 표면 손상과 부표면 손상을 분리하지 않아 GBLM의 $I_S/I_B$ 분리 개념과 근본적으로 다르다[ref_66].

#### 6.3.4 Tallian 모델

Tallian과 Chiu(1967)[ref_67]의 통합 표면–부표면 수명 모델은 GBLM의 직접적 선행 모델로, 표면 손상 적분 $I_S$와 부표면 손상 적분 $I_B$를 명시적으로 분리하였다:

$$
I_S = \int_V \left(\sigma_{\mathrm{contact}} - \sigma_{\mathrm{limit}}\right)^n \, dV, \quad n = 4\text{–}6
$$

$$
I_B = \int_V \left(\sigma_{\mathrm{shear}} - \tau_0\right)^e \, dV, \quad e = 4
$$

Tallian 모델의 공개 상수는 $n = 4\text{–}6$, $e = 4$이며, $\sigma_{\mathrm{limit}}$와 $\tau_0$의 구체적 값은 재료 의존적이다[ref_67]. GBLM은 Tallian의 $I_S/I_B$ 분리 개념을 계승하면서 SKF 내부 교정 상수를 추가하였다.

#### 6.3.5 피로한계 공개 상수 비교

피로한계 응력의 공개 값은 다음과 같다[ref_71][ref_73]:

| 모델/기준 | 피로한계 | 공개 여부 | 출처 |
|---------|---------|----------|------|
| I-H 모델 | $\tau_0 \approx 276\,\mathrm{MPa}$ | 공개 | [ref_64] |
| ISO 281:2007 (롤러) | $p_{\mathrm{limit}} \approx 1.4\,\mathrm{GPa}$ | 공개 | [ref_73] |
| ISO 281:2007 (볼) | $p_{\mathrm{limit}} \approx 1.5\,\mathrm{GPa}$ | 공개 | [ref_73] |
| 부표면 피로한계 | $\tau_0 \approx 200\text{–}300\,\mathrm{MPa}$ | 공개 | [ref_71] |
| 표면 손상 한계 | $\sigma_{\mathrm{limit}} \approx 1.4\text{–}1.5\,\mathrm{GPa}$ | 공개 | [ref_71] |
| GBLM $\sigma_{\mathrm{limit}}$ | 미공개 | **비공개** | SKF 내부 |
| GBLM 지수 $n$ | 미공개 | **비공개** | SKF 내부 |

피로한계의 물리적 의미에 대한 논쟁[ref_73]은 ISO 281:2007 채택 이후에도 지속되고 있다. 롤러 베어링의 1.4 GPa, 볼 베어링의 1.5 GPa 피로한계는 L-P→I-H→ISO 281 진화 과정에서 경험적으로 결정된 값이며, 이론적 근거에 대한 논란이 있다[ref_73]. 베어링 강재 피로한계의 특성화 연구[ref_71]는 $\tau_0 = 0.25 \cdot p_0$ 관계를 제시하며, 부표면 피로한계 $\tau_0 \approx 200\text{–}300\,\mathrm{MPa}$, 표면 손상 한계 $\sigma_{\mathrm{limit}} \approx 1.4\text{–}1.5\,\mathrm{GPa}$를 공개 상수로 제공한다[ref_71].

#### 6.3.6 모델 종합 비교

| 항목 | L-P | I-H | ISO/TS 16281 | Zaretsky | Tallian | GBLM |
|-----|-----|-----|-------------|---------|---------|------|
| 표면/부표면 분리 | ✗ | ✗ | ✗ | ✗ | ✓ | ✓ |
| 피로한계 도입 | ✗ | ✓ | ✓ | ✗ | ✓ | ✓ |
| 응력 적분 방식 | 체적 | 체적 | 간접 | 최대값 | $I_S/I_B$ | $I_S/I_B$ |
| 공개 상수 | 완전 | 완전 | 완전 | 완전 | 부분 | **미공개** |
| TRB 적용 용이성 | 높음 | 높음 | 높음 | 높음 | 중간 | 낮음 |
| 마이크로피팅 반영 | ✗ | ✗ | 간접 | ✗ | 부분 | ✓ |
| 표준화 여부 | ISO 281 | ISO 281 기초 | ISO/TS 16281 | 비표준 | 비표준 | 비표준 |

GBLM은 표면 손상을 명시적으로 분리하고 마이크로피팅을 직접 반영하는 유일한 모델이지만, $\sigma_{\mathrm{limit}}$와 지수 $n$이 SKF 내부 상수로 비공개되어 있어 독립적 검증이 불가능하다. 반면 ISO/TS 16281은 $a_{\mathrm{ISO}}$ 계산식과 $C_u$ 값이 완전히 공개되어 있어 실무 적용이 용이하다[ref_69][ref_70].

#### 6.3.7 실무 권장 방향

GBLM의 $\sigma_{\mathrm{limit}}$ 미공개 문제로 인해 TRB 마이크로피팅 수명 계산의 실무 접근법은 다음 두 가지로 구분된다:

**접근법 A: ISO/TS 16281 공개 수정계수 활용**
- $a_{\mathrm{ISO}}$를 통해 윤활 조건($\kappa$)과 오염 계수($e_C$)를 반영
- 피로하중한계 $C_u$ (카탈로그 공개값) 활용
- 표면 손상과 부표면 손상의 명시적 분리 불가
- 장점: 완전 공개 상수, 표준화, 소프트웨어 지원[ref_69][ref_70]

**접근법 B: GBLM $I_S$ 개념 + 공개 피로한계 대입**
- $\sigma_{\mathrm{limit}} \approx 1.4\,\mathrm{GPa}$ (롤러, [ref_71][ref_73]) 대입 — **이 값은 SKF GBLM 내부 교정 상수가 아니라 ref_71 기반 공개 피로한계 앵커이며, 독립 교정 전 초기 추정·민감도 분석용으로만 사용한다. GBLM 내부 상수는 비공개이다.**
- 지수 $n = 4\text{–}6$ 범위에서 민감도 분석
- 결과는 SKF 공식 GBLM과 상이할 수 있음
- 장점: 표면 손상 명시적 정량화 가능

두 접근법의 한계와 실무 적용 전략은 sec-7에서 상세히 논의한다.

---

### 소결

본 섹션에서는 GBLM의 $I_S$ 함수를 TRB에 구체화하고 대안 모델과 비교하였다. 주요 결론은 다음과 같다:

1. **TRB 접촉 입력**: $I_S$ 적분의 접촉 압력 입력은 비-Hertz 수치 해[ref_48][ref_61] 또는 로그 크라우닝 적용 후 균일 분포[ref_50][ref_51]를 사용해야 하며, 이상적 Hertz 선접촉 가정은 에지 응력을 과소평가한다[ref_53].

2. **미끄럼–마이크로피팅**: TRB에서 SRR 분포[ref_63]와 trailing edge asperity 응력 집중[ref_54]이 $I_S$ 값을 지배하며, rib–roller end 2차 접촉[ref_55]은 추가적인 $I_S$ 기여 항으로 처리해야 한다.

3. **공개 상수 비교**: I-H($\tau_0 \approx 276\,\mathrm{MPa}$, $e \approx 4$)[ref_64], ISO/TS 16281($p = 10/3$, $a_{\mathrm{ISO}}$ 표)[ref_69][ref_70], Zaretsky($e \approx 4$)[ref_66], Tallian($n = 4\text{–}6$, $e = 4$)[ref_67]은 모두 공개 상수를 제공하는 반면, GBLM의 $\sigma_{\mathrm{limit}}$와 $n$은 비공개이다.

4. **실무 우회 전략**: GBLM의 미공개 상수 문제로 인해 실무에서는 ISO/TS 16281의 공개 수정계수 $a_{\mathrm{ISO}}$와 $C_u$를 활용하는 방식이 권장되며, 이는 sec-7에서 구체적 계산 절차로 연결된다.

---

---

## 7. 결론 및 실무 권고

본 보고서는 SKF GBLM(Generalized Bearing Life Model)의 표면손상 함수 $I_S$에 내재된 미지 상수 문제를 체계적으로 분석하고, 테이퍼 롤러 베어링(TRB) 마이크로피팅 수명 계산에 이를 적용하기 위한 방법론을 제시하였다. 본 장에서는 앞선 논의를 종합하여 해법의 핵심 구조를 정리하고, 실무 엔지니어를 위한 단계별 워크플로를 제공하며, 현재 방법론의 한계와 향후 연구 방향을 논한다.

---

### 7.1 $I_S$ 미지 상수 문제에 대한 해법 종합

#### 7.1.1 문제의 본질 재확인

GBLM의 표면손상 함수는 다음과 같이 정의된다 [ref_04]:

$$I_S = \int_V \left(\sigma_{\mathrm{contact}} - \sigma_{\mathrm{limit}}\right)^n \, dV$$

여기서 $\sigma_{\mathrm{contact}}$는 접촉 표면에서의 응력, $\sigma_{\mathrm{limit}}$는 표면 피로 한계 응력, $n$은 Weibull 형상 지수에 해당하는 멱지수이다. 대응하는 부표면 손상 함수는 [ref_04]:

$$I_B = \int_V \left(\sigma_{\mathrm{shear}} - \tau_0\right)^e \, dV$$

이 두 적분에는 총 네 개의 재료·모델 상수 $\{\sigma_{\mathrm{limit}},\, n,\, \tau_0,\, e\}$가 포함되며, 이 중 $\sigma_{\mathrm{limit}}$와 $n$은 표면 손상 경로에 특화된 미지 상수로서 직접적인 해석적 결정이 불가능하다. **문헌에서 언급되는 $\sigma_{\mathrm{limit}} \approx 1.4$–1.5 GPa는 SKF GBLM 내부 교정 상수가 아니라 ref_71 기반 공개 피로한계 앵커이며, GBLM 내부 상수는 비공개이다.** 이것이 본 보고서 전체를 관통하는 핵심 문제이다 [ref_05].

#### 7.1.2 이중 전략: 교정과 표준 수정계수 우회

$I_S$ 미지 상수 문제는 단일 해석적 해법이 존재하지 않으며, 다음 두 가지 전략의 **병행 적용**으로 실용적으로 해결된다.

**전략 1 — 실험 교정(Calibration, sec-5 연계)**

트윈디스크 시험 또는 FE8 표준 시험 리그에서 얻은 마이크로피팅 수명 데이터를 이용하여 $\sigma_{\mathrm{limit}}$와 $n$을 역산(inverse identification)한다 [ref_41]. 이 경로는 다음 연결고리를 따른다:

1. **근본 수식(sec-3)**: $I_S$ 적분의 물리적 정의 — 접촉 응력이 $\sigma_{\mathrm{limit}}$를 초과하는 체적에 대한 멱함수 적분 [ref_04]
2. **수치 절차(sec-4)**: DC-FFT 기반 응력장 계산 → Rainflow 사이클 계수 → Palmgren-Miner 누적 손상으로 $I_S$ 수치 평가 [ref_05]
3. **교정(sec-5)**: 실험 수명 $L_{10,\mathrm{exp}}$와 모델 예측 $L_{10,\mathrm{model}}$의 잔차를 최소화하는 최적화로 $(\sigma_{\mathrm{limit}}, n)$ 결정 [ref_41]

**전략 2 — 표준 수정계수 우회(sec-6.3 연계)**

ISO/TS 16281 및 ISO/TR 15144 기반의 공개 수정계수 체계를 활용하여 $I_S$를 직접 계산하지 않고 수명 수정계수 $a_{\mathrm{ISO}}$로 표면 손상 효과를 흡수한다 [ref_69][ref_70]. 이 경로에서는 $\sigma_{\mathrm{limit}}$와 $n$의 명시적 값 대신, 피로하중한계 $C_u$와 윤활·오염 계수 $\kappa$, $\eta_c$가 $a_{\mathrm{ISO}}$ 계산에 통합된다.

#### 7.1.3 연결고리 요약

아래 표는 본 보고서의 각 섹션이 해법 구조에서 담당하는 역할을 정리한 것이다.

| 섹션 | 역할 | 핵심 출력 |
|------|------|-----------|
| sec-3 (근본 수식) | $I_S$, $I_B$ 적분의 물리적 정의 | $\sigma_{\mathrm{limit}}$, $n$, $\tau_0$, $e$의 의미 확립 |
| sec-4 (수치 절차) | DC-FFT + Rainflow + Miner로 $I_S$ 수치 계산 | 응력 이력 → 손상 누적값 |
| sec-5 (교정) | 실험 데이터 기반 역산으로 미지 상수 결정 | $(\sigma_{\mathrm{limit}}, n)$ 교정값 |
| sec-6.3 (우회) | ISO 표준 수정계수로 $I_S$ 직접 계산 대체 | $a_{\mathrm{ISO}}$, $a_1$, $C_u$ 기반 $L_{10}$ |

두 전략은 상호 배타적이지 않다. 교정 데이터가 충분한 경우 전략 1이 우선되며, 데이터가 부족하거나 초기 설계 단계에서는 전략 2가 실용적 대안이 된다 [ref_11][ref_70].

---

### 7.2 TRB 마이크로피팅 수명 계산 실무 워크플로 및 권고

#### 7.2.1 단계별 체크리스트

다음은 TRB 마이크로피팅 수명 계산을 위한 6단계 실무 워크플로이다.

---

**① 단계: TRB 기하·하중·윤활 입력**

- 입력 파라미터: 롤러 유효 접촉 길이 $l_{\mathrm{eff}}$, 롤러 반경 $R$, 접촉각 $\alpha$ (통상 10–30°) [ref_69], 내/외륜 기하, 크라우닝 반경
- 하중 조건: 반경 하중 $F_r$, 축 하중 $F_a$, 회전 속도 $n$
- 윤활 조건: 동점도 $\nu$, 점도지수, 첨가제 유형, 오염도 $\eta_c$
- **권고**: TRB의 경우 정렬 불량(misalignment)이 에지 응력을 급격히 증가시키므로, 실제 운전 조건의 정렬 오차를 반드시 입력에 포함할 것 [ref_53]

---

**② 단계: 선접촉 압력 및 에지 응력 계산 (크라우닝 반영)**

- Hertz 선접촉 이론으로 명목 접촉 반폭 $b$와 최대 접촉 압력 $p_0$ 계산
- 크라우닝이 없는 경우 에지 응력 집중이 발생하므로, 로그 크라우닝 프로필 적용 시 에지 응력 제거 효과를 수치적으로 확인할 것 [ref_50]
- 비-Hertz 압력 분포(에지 효과 포함)는 Boussinesq 반공간 기반 유한 선접촉 모델로 보정 [ref_61]
- **권고**: 크라우닝 파라미터 최적화를 통해 $p_0$를 최소화하면 $I_S$ 적분값이 비선형적으로 감소함 ($n$차 멱함수 의존성)

---

**③ 단계: 윤활막 비 $\lambda$ 및 미끄럼률 SRR 산출**

윤활막 비는 다음 식으로 계산한다 [ref_69]:

$$\lambda = \frac{h_{\min}}{\sqrt{R_{a,1}^2 + R_{a,2}^2}}$$

여기서 $h_{\min}$은 EHL 최소 막 두께, $R_{a,1}$, $R_{a,2}$는 각 접촉면의 산술 평균 조도이다.

미끄럼률(Slide-to-Roll Ratio)은:

$$\mathrm{SRR} = \frac{2|u_1 - u_2|}{u_1 + u_2}$$

- **권고**: TRB에서는 롤러 단부와 플랜지 접촉(rib–roller end contact)에서 SRR이 국소적으로 높아지므로 별도 평가 필요 [ref_55]
- $\lambda < 1$: 경계 윤활 → 마이크로피팅 위험 최고
- $1 \leq \lambda < 3$: 혼합 윤활 → 마이크로피팅 주요 발생 영역
- $\lambda \geq 3$: 완전 유체 윤활 → 마이크로피팅 위험 최소

---

**④ 단계: DC-FFT 응력장 + Rainflow + Miner로 $I_S$ 평가**

이 단계가 $I_S$ 수치 계산의 핵심이다.

1. **DC-FFT 응력장 계산**: 접촉 압력 분포 $p(x,y)$를 입력으로 반무한체 내부 응력 텐서 $\sigma_{ij}(x,y,z)$를 이산 합성곱-FFT 알고리즘으로 계산 [ref_27][ref_28]
2. **표면 응력 추출**: 조도 프로필을 중첩하여 asperity 수준의 국소 접촉 응력 $\sigma_{\mathrm{contact}}(x,y)$ 산출
3. **Rainflow 사이클 계수**: 롤러 1회전 동안의 응력 이력을 Rainflow 알고리즘으로 분해하여 응력 진폭 $\Delta\sigma_i$와 평균 응력 $\bar{\sigma}_i$, 사이클 수 $n_i$ 추출 [ref_34]
4. **Palmgren-Miner 누적 손상**: 

$$D = \sum_i \frac{n_i}{N_i(\Delta\sigma_i)}$$

5. **$I_S$ 적분 평가**: $\sigma_{\mathrm{contact}} > \sigma_{\mathrm{limit}}$인 체적 요소에 대해 멱함수 적분 수행

$$I_S = \int_{V: \sigma > \sigma_{\mathrm{limit}}} \left(\sigma_{\mathrm{contact}} - \sigma_{\mathrm{limit}}\right)^n \, dV$$

---

**⑤ 단계: $\sigma_{\mathrm{limit}}$·$n$ 교정값 또는 ISO/TR 15144 공개 상수 사용**

이 단계에서 미지 상수 문제가 실질적으로 해결된다. 아래 표에 권장 상수값을 정리한다.

**표 7.1: $I_S$ 관련 공개 상수 권장값**

| 상수 | 권장 범위 | 출처 | 비고 |
|------|-----------|------|------|
| $n$ (표면 Weibull 지수) | 4–6 | [ref_04][ref_05] | 재료·윤활 조건에 따라 변동 |
| $e$ (부표면 Weibull 지수) | 4 | [ref_04] | ISO 281 기반 표준값 |
| $\tau_0$ (부표면 피로한계) | 200–300 MPa | [ref_71] | $\tau_0 \approx 0.25 \cdot p_0$ |
| $\sigma_{\mathrm{limit}}$ (표면 피로한계) | 1.4–1.5 GPa | [ref_71] | 롤러: 약 1.4 GPa, 볼: 약 1.5 GPa — **ref_71 공개 앵커; SKF GBLM 내부 상수는 비공개. 독립 교정 전 초기 추정·민감도 분석용** |
| $C_u / C$ (피로하중한계 비) | 재료별 상이 | [ref_70] | AISI 52100 기준 |

**표 7.2: ISO 16281 기반 수명 수정계수 $a_{\mathrm{ISO}}$ 계산 입력 파라미터**

| 파라미터 | 정의 | 공개 상수 여부 |
|----------|------|----------------|
| $a_1$ | 신뢰도 수정계수 ($L_{10}$: $a_1=1$) | 공개 (ISO 281 표) |
| $a_{\mathrm{ISO}}$ | 윤활·오염·피로한계 통합 수정계수 | 계산식 공개 [ref_70] |
| $\kappa = \nu/\nu_1$ | 점도비 (실제/기준 점도) | 계산 가능 |
| $\eta_c$ | 오염 계수 | 공개 표 [ref_69] |
| $C_u$ | 피로하중한계 | 카탈로그 공개 |

- **교정 데이터 보유 시**: FE8 또는 트윈디스크 시험에서 역산된 $(\sigma_{\mathrm{limit}}, n)$ 사용 [ref_41]
- **교정 데이터 미보유 시**: ISO/TR 15144는 GBLM의 $I_S$ 상수($\sigma_{\mathrm{limit}}$, $n$)를 직접 산출하는 절차가 아니라, 윤활막 두께 기반 안전계수($S_\lambda$) 중심의 별도 마이크로피팅 위험 평가·검증 경로이다. $\sigma_{\mathrm{limit}} \approx 1.4$ GPa, $n = 4$–6은 독립 교정 전 초기 추정·민감도 분석용 문헌 앵커로만 사용한다 [ref_45][ref_71]

---

**⑥ 단계: 생존확률 결합으로 $L_{10}$ 산출**

GBLM에서 전체 베어링 수명은 표면 손상 생존확률 $S_s$와 부표면 피로 생존확률 $S_b$의 결합으로 계산된다 [ref_11]:

$$S_{\mathrm{total}} = S_s \cdot S_b$$

Weibull 분포 가정 하에:

$$S_s = \exp\left(-\left(\frac{L}{L_{10,s}}\right)^{e_s}\right), \quad S_b = \exp\left(-\left(\frac{L}{L_{10,b}}\right)^{e_b}\right)$$

$L_{10}$ 수명은 $S_{\mathrm{total}} = 0.9$를 만족하는 $L$로 정의된다. 최종 수명식은 [ref_69]:

$$L_{nm} = a_1 \cdot a_{\mathrm{ISO}} \cdot \left(\frac{C}{P}\right)^p$$

여기서 $p = 10/3$ (롤러 베어링), $a_{\mathrm{ISO}}$는 $I_S$ 기반 표면 손상 효과를 흡수한 통합 수정계수이다.

#### 7.2.2 워크플로 요약 다이어그램

```
[① TRB 기하·하중·윤활 입력]
         ↓
[② 선접촉 압력 + 에지 응력 (크라우닝 반영)]
         ↓
[③ λ 및 SRR 산출]
         ↓
[④ DC-FFT 응력장 → Rainflow → Miner → I_S 평가]
         ↓
[⑤ σ_limit·n: FE8/트윈디스크 교정값 우선; 미보유 시 ref_71 문헌 앵커(초기 추정·민감도 분석용) — ISO/TR 15144는 S_λ 기반 별도 검증 경로]
         ↓
[⑥ 생존확률 결합 → L_10 산출]
```

#### 7.2.3 실무 권고 사항

1. **크라우닝 우선 최적화**: TRB에서 에지 응력은 $I_S$ 적분을 지배하는 주요 인자이므로, 로그 크라우닝 파라미터 최적화를 수명 계산 전에 수행할 것 [ref_50]

2. **$\lambda$ 관리**: $\lambda \geq 2$를 목표로 윤활유 점도 및 표면 조도를 관리하면 $I_S$ 값이 급격히 감소함. 슈퍼피니싱(Ra < 0.1 μm)은 $\lambda$ 개선에 효과적 [ref_26]

3. **SRR 최소화**: TRB 플랜지 접촉에서 SRR이 높은 경우 마이크로피팅 위험이 집중되므로, 윤활 공급 경로 최적화 필요 [ref_55]

4. **교정 주기**: $\sigma_{\mathrm{limit}}$와 $n$은 재료 배치(batch), 열처리 조건, 윤활제 유형에 따라 변동하므로 정기적 재교정 권장 [ref_05][ref_41]

5. **초기 설계 민감도 분석**: 교정 데이터 미보유 시 $n = 4$–6 범위에 대해 민감도 분석을 수행해야 하며, 안전측 판단은 정규화 방식과 응력 초과분($\sigma - \sigma_{\mathrm{limit}}$ 또는 정규화 초과분)의 크기 범위에 따라 달라지므로 일률적으로 $n = 4$가 보수적이라 단정할 수 없다 [ref_71]

---

### 7.3 한계와 향후 연구 방향

#### 7.3.1 현재 방법론의 주요 한계

**한계 1: $\sigma_{\mathrm{limit}}$의 재료·윤활쌍별 미공개**

$\sigma_{\mathrm{limit}} \approx 1.4$–1.5 GPa는 AISI 52100 표준 베어링강에 대한 대표값이며 [ref_71], 다음 조건에서의 값은 공개되지 않았다:
- 표면 코팅(DLC, TiN 등) 적용 시
- 질화 처리(nitriding) 또는 침탄 처리 강
- 비표준 윤활제(합성유, 그리스, 첨가제 패키지)와의 상호작용
- 고온 운전 조건 ($T > 120°C$)에서의 열 의존성 [ref_05]

이로 인해 특수 재료·윤활 조합에 대한 $I_S$ 계산의 정확도가 제한된다.

**한계 2: 표면-부표면 Weibull slope 결합의 불확실성**

GBLM은 표면 손상 생존확률 $S_s$와 부표면 피로 생존확률 $S_b$를 독립적으로 가정하고 곱으로 결합한다 [ref_11]. 그러나 실제 베어링 손상에서는 표면 마이크로피팅이 부표면 균열 전파를 가속하는 상호작용이 관찰되며, 이 결합 불확실성은 다음을 초래한다:
- Weibull slope $e_s$와 $e_b$의 독립성 가정 위반 가능성
- 혼합 손상 모드(표면+부표면 동시 진행)에서의 수명 과대 예측 위험
- 생존확률 결합 시 통계적 오차 전파

**한계 3: 교정 데이터의 일반화 한계**

FE8 또는 트윈디스크 시험에서 얻은 교정 상수는 해당 시험 조건(하중, 속도, 윤활, 재료)에 특화된 값이다 [ref_41]. 이를 다른 운전 조건이나 베어링 형식에 직접 적용할 경우:
- 접촉 기하(점접촉 vs 선접촉) 차이에 의한 응력장 형태 변화
- 스케일 효과(시험 리그 소형 vs 실제 대형 TRB)
- 가속 시험 조건과 실제 운전 조건의 손상 메커니즘 차이

가 오차 원인이 된다 [ref_45].

**한계 4: 수치 계산 비용**

DC-FFT 기반 $I_S$ 적분은 조도 프로필의 공간 해상도에 따라 계산 비용이 급증한다. 실제 TRB의 3차원 접촉 영역 전체에 대한 고해상도 계산은 현재 상업 소프트웨어 환경에서 실시간 적용이 어렵다 [ref_27].

#### 7.3.2 향후 연구 방향

**방향 1: 결정 소성 + Cohesive FE 고급 모델 통합**

ref_78에서 제시된 결정 소성 손상 이론(crystal plasticity damage theory)과 cohesive 유한요소법(cohesive FE)의 결합 모델은 마이크로피트 형성의 물리적 메커니즘을 입자 수준에서 모사한다 [ref_78]. 이 접근법은:
- 결정립 방향성(grain orientation)에 따른 균열 개시 위치 예측
- 피로 균열 전파 경로의 명시적 모델링
- $\sigma_{\mathrm{limit}}$의 미시역학적 근거 제공

을 가능하게 하며, 향후 GBLM의 $I_S$ 상수를 물리 기반으로 결정하는 경로를 열 수 있다. 다만 현재는 계산 비용이 매우 높아 단일 접촉 사이클 시뮬레이션에도 상당한 자원이 필요하다 [ref_78].

**방향 2: 재료·윤활쌍별 $\sigma_{\mathrm{limit}}$ 데이터베이스 구축**

다양한 재료(표면 처리 포함)와 윤활제 조합에 대한 체계적 FE8/MPR 시험을 통해 $\sigma_{\mathrm{limit}}$와 $n$의 데이터베이스를 구축하는 것이 시급하다. 특히:
- DLC 코팅 베어링의 표면 피로 한계 (현재 데이터 극히 부족)
- 풍력 발전기용 대형 TRB의 스케일 효과 보정
- 수소 취성(hydrogen embrittlement) 환경에서의 $\sigma_{\mathrm{limit}}$ 저하 정량화

**방향 3: 표면-부표면 손상 결합 모델**

표면 마이크로피팅과 부표면 RCF의 상호작용을 명시적으로 모델링하는 결합 손상 함수 개발이 필요하다. 현재 GBLM의 독립 결합 가정을 넘어서, 표면 균열이 부표면 응력장을 변화시키는 피드백 메커니즘을 포함하는 모델이 요구된다 [ref_79].

**방향 4: 기계학습 기반 상수 추정**

대규모 시험 데이터와 시뮬레이션 결과를 결합한 기계학습(ML) 모델을 통해 운전 조건으로부터 $(\sigma_{\mathrm{limit}}, n)$을 실시간으로 추정하는 접근법이 최근 주목받고 있다. 이는 교정 데이터의 일반화 한계를 극복하는 데 기여할 수 있다.

**방향 5: ISO 표준화 추진**

현재 $\sigma_{\mathrm{limit}}$와 $n$은 SKF 내부 교정값으로 관리되며 완전히 공개되지 않는다. ISO/TS 16281 및 ISO/TR 15144의 개정 과정에서 이 상수들의 표준화된 결정 절차와 기준값을 포함시키는 것이 산업계 전반의 수명 예측 신뢰성 향상에 기여할 것이다 [ref_69][ref_45].

---

---

## 참고문헌 (통합 마스터 목록, ref_01–ref_81)

> 본 보고서에서 인용된 문헌을 ref ID 순서로 정리하였다. 품질 등급: **A** = 학술 논문·공식 보고서·1차 데이터 / **B** = 기술 블로그·리뷰 논문·2차 분석 / **C** = 뉴스·일반 웹·업계 매거진.

[1] [ref_01] **[A]** Morales-Espejel, G.E., Gabelli, A., De Vries, A.J.C. (2015). "A model for rolling bearing life with surface and subsurface survival—tribological effects." *Tribology Transactions*, 58(5), 894–906. DOI: 10.1080/10402004.2015.1025932.

[2] [ref_02] **[A]** Morales-Espejel, G.E., Gabelli, A. (2015). "A model for rolling bearing life with surface and subsurface survival: Sporadic surface damage from deterministic indentations." *Tribology International*, 86, 28–39. DOI: 10.1016/j.triboint.2015.04.040.

[3] [ref_03] **[A]** Morales-Espejel, G.E., Gabelli, A. (2018). "A model for hybrid bearing life with surface and subsurface survival." *Wear*, 412–413, 50–59. DOI: 10.1016/j.wear.2018.10.008.

[4] [ref_04] **[A]** Morales-Espejel, G.E., Gabelli, A. (2019). "Application of a rolling bearing life model with surface and subsurface survival to hybrid bearing cases." *Proc. IMechE Part J*, 233(6). DOI: 10.1177/0954406219848470.

[5] [ref_05] **[A]** Morales-Espejel, G.E., Gabelli, A. (2020). "A model for rolling bearing life with surface and subsurface survival: Surface thermal effects." *Wear/Tribology International*, 456–457, 203054. DOI: 10.1016/j.wear.2020.203054.

[6] [ref_06] **[A]** Morales-Espejel, G.E., Gabelli, A. (2020). "Thermal damage and fatigue estimation in heavily loaded lubricated rolling/sliding contacts with micro-geometry." *Proc. IMechE Part J*, 234(12). DOI: 10.1177/1350650120972591.

[7] [ref_07] **[B]** Morales-Espejel, G.E. et al. "Stress Based Model for General Rolling Contact with Surface and Subsurface Survival: Application to Gears with Material Inhomogeneities." *ResearchGate preprint*.

[8] [ref_08] **[A]** Morales-Espejel, G.E. (2023). *Rolling Bearings: Tribology Damage Modes and Life Modelling.* Taylor & Francis. DOI: 10.1201/9781003559771.

[9] [ref_09] **[B]** Springer (2021). "New life calculation model for hybrid bearings." In *Advanced Bearing Technology* (Ch.). DOI: 10.1007/978-3-030-69799-0_11.

[10] [ref_10] **[B]** "Application of a rolling bearing life model with surface and subsurface damage functions." HAL (2022). hal-03660201.

[11] [ref_11] **[B]** SKF Evolution (2015/2019). "The SKF Generalized Bearing Life Model (GBLM) for hybrid bearings." EN evo415 PDF / 웹.

[12] [ref_12] **[C]** SKF. "SKF Generalized Bearing Life Model" (공식 페이지) / "The SKF formula for rolling bearing life" (*Evolution*).

[13] [ref_13] **[C]** SKF. "Spindle bearing potential damaging mechanisms and mitigation" (Media Hub PDF).

[14] [ref_14] **[C]** SKF / Gear Solutions. "Understanding and Preventing Surface Distress."

[15] [ref_15] **[C]** SKF Evolution. "The progression of surface rolling contact fatigue damage of rolling bearings."

[16] [ref_16] **[C]** Morales-Espejel, G.E. (2019/2025). Interview "A real-world view of hybrid bearing life" (*Bearing News*); Webinar "Surface Life Challenges in Machine Elements" (TriboNet); "From theory to practice" (SKF).

[17] [ref_17] **[A]** Morales-Espejel, G.E., Brizmer, V. (2011). "Micropitting modelling in rolling–sliding contacts: Application to rolling bearings." *Tribology Transactions*, 54(4), 625–643. DOI: 10.1080/10402004.2011.587633.

[18] [ref_18] **[A]** Morales-Espejel, G.E. et al. (2018). "Prediction of micropitting damage in gear teeth contacts considering the concurrent effects of surface fatigue and mild wear." *Wear*, 400–401, 45–55. DOI: 10.1016/j.wear.2017.12.006.

[19] [ref_19] **[A]** Brizmer, V., Morales-Espejel, G.E. et al. (2019). "The effect of contact severity on micropitting: Simulation and experiments." *Tribology International*, 131, 288–298. DOI: 10.1016/j.triboint.2018.10.040.

[20] [ref_20] **[A]** AL-Mayali, M.F., Hutt, S., Sharif, K.J., Clarke, A., Evans, H.P. (2018). "Experimental and Numerical Study of Micropitting Initiation in Real Rough Surfaces in a Micro-EHL Regime." *Tribology Letters*, 66(4):150. DOI: 10.1007/s11249-018-1110-2.

[21] [ref_21] **[A]** "A micropitting study considering rough sliding and mild wear." *Coatings*, 9(10):639 (2019). DOI: 10.3390/coatings9010639.

[22] [ref_22] **[A]** "A physics-based model to predict micro-pitting lives of lubricated point contacts." *Tribology International* (2013). pii S0142112312002666.

[23] [ref_23] **[A]** "Micro-pitting fatigue lives of lubricated point contacts: Experiments and model validation." *Tribology International* (2012). pii S0142112312003507.

[24] [ref_24] **[B]** "A review on micropitting studies of steel gears." *Coatings*, 9(1):42 (2020). DOI: 10.3390/coatings9010042.

[25] [ref_25] **[A]** "Micro-pitting and wear assessment of PAO vs mineral-based engine oil under mixed lubrication." *Lubricants*, 7(5):42 (2019). DOI: 10.3390/lubricants7050042.

[26] [ref_26] **[B]** "The Effect of the Surface Roughness Profile on Micropitting" (*REM/Gear Solutions*).

[27] [ref_27] **[A]** Liu, S., Wang, Q., Liu, G. (2000). "A versatile method of discrete convolution and FFT (DC-FFT) for contact analyses." *Wear*, 243. pii S0043164800004270.

[28] [ref_28] **[A]** Liu, S., Wang, Q. (2002). "Studying contact stress fields caused by surface tractions with a DC-FFT algorithm." *J. Tribol.*, 124(1), 36.

[29] [ref_29] **[A]** Stanley, H.M., Kato, T. (1997). "An FFT-Based Method for Rough Surface Contact." *J. Tribol.*, 119(3), 481.

[30] [ref_30] **[B]** "FFT-Based Methods for Computational Contact Mechanics." *Frontiers in Mech. Eng.* (2020). DOI: 10.3389/fmech.2020.00061.

[31] [ref_31] **[A]** "A new FFT technique for the analysis of contact pressure and subsurface stress in a semi-infinite solid."

[32] [ref_32] **[A]** "Pitting life prediction based on a 3D line contact mixed EHL analysis and subsurface von Mises stress calculation." *J. Tribol.*, 131(4):041501.

[33] [ref_33] **[A]** "Estimation Method of Micropitting Life from S–N Curve Established by Residual Stress Measurements and Numerical Contact Analysis." *Tribology Online*, 14(3):131.

[34] [ref_34] **[B]** "Fatigue life under variable-amplitude loading: cycle-counting and spectral methods" / Rainflow 계수 기반 손상 누적.

[35] [ref_35] **[B]** "Numerical Analysis for the Elastic Contact of Real Rough Surfaces." *Tribology Transactions* (1999). DOI: 10.1080/10402009908982240.

[36] [ref_36] **[B]** Schaeffler. "Fast calculation method for predicting the risk of surface initiated damage in rolling bearings."

[37] [ref_37] **[A]** Lieblein, J., Zelen, M. (1956). "Statistical investigation of the fatigue life of deep-groove ball bearings." *J. Res. NIST*, 57(5):273.

[38] [ref_38] **[A]** "Application of inverse first-order reliability method (IFORM) for probabilistic fatigue life prediction." *Int. J. Fatigue*. pii S0266892010000949.

[39] [ref_39] **[A]** "Inverse method to determine fatigue properties by combining cyclic indentation and numerical simulation." *Materials*, 13(14):3126 (2020).

[40] [ref_40] **[B]** "Damage identification using inverse methods." *Phil. Trans. R. Soc. A*, 365(1851):393.

[41] [ref_41] **[A]** "Rolling contact fatigue: Damage function development from two-disc test data." *Wear*. pii S0043164818313991.

[42] [ref_42] **[B]** Cornel, D. et al. (2021). "Condition monitoring of roller bearings using acoustic emission" (FE8, DIN 51819-02/03). *Wind Energy Science*, 6:367.

[43] [ref_43] **[A]** "An Experimental Evaluation of Micro-pitting Performance of Two Bearing Steels" (OSU 학위논문).

[44] [ref_44] **[B]** "An experimental investigation replicating ground steel gears in mixed lubrication using twin-disk testing, Part 2: Micropitting." *Tribology Transactions* (2024). DOI: 10.1080/10402004.2024.2378811.

[45] [ref_45] **[B]** "An Experimental Evaluation of the Procedures of the ISO/TR 15144 for the Prediction of Micropitting." *Gear Solutions*.

[46] [ref_46] **[B]** Kim et al. (2025). "Optimization of accelerated life test design for gears and bearings using equivalent damage method." *Scientific Reports*.

[47] [ref_47] **[B]** "Micro-pitting: Research progress and prospects" (SciEngine).

[48] [ref_48] **[A]** "A new quasi-static multi-DOF tapered roller bearing model to consider non-Hertzian contact pressures." *Proc. IMechE Part K*. DOI: 10.1177/1464419313513446.

[49] [ref_49] **[A]** "Determination of the contact stresses in double-row tapered roller bearings using FEM, experiment and analytical models." *J. Mech. Sci. Technol.* DOI: 10.1007/s12206-015-1010-4.

[50] [ref_50] **[A]** "Study on Logarithmic Crowning of Tapered Roller Profile Considering Angular Misalignment." *J. Tribol.*, 142(11):111201.

[51] [ref_51] **[A]** "Study on logarithmic crowning of cylindrical roller profile considering angular misalignment." *J. Mech. Sci. Technol.* DOI: 10.1007/s12206-020-0130-7.

[52] [ref_52] **[B]** "Study on Influence of Roller Profile Modification on Wear of Tapered Roller Bearings." *Lubricants*, 14(2):69.

[53] [ref_53] **[B]** Schaeffler. "Rolling Bearing Damage" (WL 82102).

[54] [ref_54] **[A]** "The mechanism of bearing surface fatigue—experiments and theories." *Tribology Transactions* (1997). DOI: 10.1080/10402009708983706.

[55] [ref_55] **[A]** "Partial EHL analysis of rib-roller end contact in tapered roller bearings." *Tribology International*. pii 0301679X9500059D.

[56] [ref_56] **[A]** "An engineering approach to Hertzian contact elasticity—Part I." *J. Tribol.*, 123(3):582.

[57] [ref_57] **[A]** "Mechanical behavior of double-row tapered roller bearing under combined external loads and angular misalignment." *Int. J. Mech. Sci.* pii S0020740318304223.

[58] [ref_58] **[A]** "Statistical distribution of tapered roller bearing fatigue lives at high levels of reliability." *J. Tribol.*, 127(4):865.

[59] [ref_59] **[B]** "Detection method for contact stress distribution of tapered roller bearings." *Sci. Rep.* (PMC11076488).

[60] [ref_60] **[B]** "Tapered Roller Bearings – an overview" (ScienceDirect Topics) / SKF single-row TRB / DTIC high-speed TRB.

[61] [ref_61] **[A]** "Contact Load Calculation Models for Finite Line Contact Rollers in Bearing Dynamic Simulation." *Lubricants*, 13(4):183.

[62] [ref_62] **[B]** "Assessment of the factors influencing micropitting in rolling/sliding contacts." *Wear*. pii S0043164804004065.

[63] [ref_63] **[A]** "The influence of slide–roll ratio on the extent of micropitting damage in rolling–sliding contacts." *Tribology Letters*. DOI: 10.1007/s11249-019-1174-7.

[64] [ref_64] **[A]** Ioannides, E., Harris, T.A. (1986). "A new fatigue life model for rolling bearings." *J. Tribol.*, 107(3):367.

[65] [ref_65] **[A]** Harris, T.A. et al. (1996). "Comparison of life theories for rolling-element bearings." *Tribology Transactions*. DOI: 10.1080/10402009608983525.

[66] [ref_66] **[A]** Zaretsky, E.V. NASA "Rolling bearing life prediction—past, present, and future" (2001) / "...theory and application" (2013).

[67] [ref_67] **[A]** Tallian, T.E., Chiu, Y.P. (1967). Unified/surface-subsurface life model (재검토 2015). [GBLM 직접 선행 모델]

[68] [ref_68] **[B]** "Rolling Bearing Life Modifying Factors for Film Thickness, Surface Topography and Stress." *J. Tribol.*, 103(4):509 (1984).

[69] [ref_69] **[B]** "From innovation to standardization—A century of rolling bearing life formula." *MDPI Machines/Lubricants* (2023). DOI: 10.3390/machines11040444.

[70] [ref_70] **[B]** ISO 16281 수정계수 자료군: "Evolution of rolling bearing life rating..." (TLT 2020) / MESYS / Schaeffler medias.

[71] [ref_71] **[A]** "The fatigue limit of bearing steels – Part II: Characterization for life rating standards." *Int. J. Fatigue* (2011). pii S0142112311003288.

[72] [ref_72] **[B]** "Fatigue Life Assessment of Rolling Bearings Made from AISI 52100 Bearing Steel." *Materials* (2019). PMC6384852.

[73] [ref_73] **[C]** "In search of a fatigue limit: A critique of ISO 281:2007" (STLE TLT 2010) / "ISO 281:2007—and the answer is?".

[74] [ref_74] **[A]** "Application of a contact fatigue life model to assess relative risk of surface vs subsurface initiated fatigue in gears." *Wear* (2025). pii S0043164825004855.

[75] [ref_75] **[A]** "Surface and subsurface rolling contact fatigue characteristic depths and proposal of stress indexes." *Tribology International*. pii S0142112312002137.

[76] [ref_76] **[A]** Hwang, J.I., Poll, G. (2022). "A new approach for fatigue life prediction in rolling bearings based on damage accumulation considering residual stresses." *Front. Manuf. Technol.*, 2:1010759.

[77] [ref_77] **[A]** Dang Van, K. (2005). "On the application of Dang Van criterion to rolling contact fatigue." *Wear*, 258. DOI: 10.1016/j.wear.2005.07.001.

[78] [ref_78] **[B]** "Modelling micropit formation in RCF of bearings with crystal plasticity damage theory coupled with cohesive FE." *Eng. Fract. Mech.* (2024). pii S0013794424000365.

[79] [ref_79] **[B]** "Surface fatigue in lubricated contacts: Mapping failure modes of micropitting versus macropitting." *Tribology International* (2025). pii S0142112325001057.

[80] [ref_80] **[B]** "Contact fatigue life prediction of rolling bearing considering machined surface integrity." *Ind. Lubr. Tribol.*, 74(1):73.

[81] [ref_81] **[B]** "A new damage-mechanics-based model for RCF analysis of cylindrical roller bearing." *Tribology International*. pii S0301679X17305601.
