# 그리스 윤활 롤링 베어링의 윤활막 두께 이론

## 저자 정보

**P.M. Lugt**, **M.T. van Zoelen**, **C.H. Venner**

**출처**: Grease Lubrication in Rolling Bearings
**출판사**: John Wiley & Sons, 2013
**챕터**: Chapter 9 – Film Thickness Theory for Single Contacts / Chapter 10 – Film Thickness in Grease Lubricated Rolling Bearings

---

## 초록

롤링 베어링의 그리스 윤활에서 충분한 윤활막 두께 형성은 베어링 수명을 결정하는 핵심 요소입니다. 본 챕터들은 탄성유체윤활(EHL) 이론의 기초부터 시작하여, Reynolds 방정식 및 박막 방정식의 유도, 접촉 기하학과 탄성 변형, 오일 및 그리스의 막두께 공식, 스타베이션(starvation) 모델, 그리고 이 이론들의 실제 베어링 적용까지를 체계적으로 다룹니다. 특히 그리스 윤활 베어링이 대부분의 운전 시간 동안 스타베이션 조건에서 작동한다는 점을 강조하며, 경미한 스타베이션(Damiens 모델)과 심각한 스타베이션(Van Zoelen 모델)을 결합한 실용적 막두께 예측 방법을 제시합니다.

---

## 목차

1. [개요](#1-개요)
2. [종합 요약](#2-종합-요약)
   - 2.1 [연구 핵심 정보](#▪-21-연구-핵심-정보)
   - 2.2 [핵심 이론/모델 비교](#▪-22-핵심-이론모델-비교)
   - 2.3 [방법론 개요](#▪-23-방법론-개요)
   - 2.4 [주요 실험/검증 결과](#▪-24-주요-실험검증-결과)
   - 2.5 [주요 발견사항](#▪-25-주요-발견사항)
   - 2.6 [적용 사례](#▪-26-적용-사례)
   - 2.7 [실무적 함의](#▪-27-실무적-함의)
3. [탄성유체윤활(EHL) 기초](#3-탄성유체윤활ehl-기초)
4. [접촉 기하학 및 변형](#4-접촉-기하학-및-변형)
5. [EHL 막두께 - 오일](#5-ehl-막두께---오일)
6. [EHL 막두께 - 그리스](#6-ehl-막두께---그리스)
7. [스타베이션](#7-스타베이션)
8. [스핀](#8-스핀)
9. [베어링 표면의 박막 유동](#9-베어링-표면의-박막-유동)
10. [롤링 베어링의 스타베이션 EHL](#10-롤링-베어링의-스타베이션-ehl)
11. [케이지 간극과 막두께](#11-케이지-간극과-막두께)
12. [전체 베어링 막두께](#12-전체-베어링-막두께)
13. [기호 설명](#13-기호-설명)
14. [참고문헌](#14-참고문헌)

---

## 1. 개요

롤링 베어링의 윤활은 전동체와 레이스웨이 사이에 충분한 윤활막을 형성하여 표면 접촉을 방지하는 것이 핵심입니다. 그리스 윤활의 경우, 초기에는 완전 충만(fully flooded) 조건에서 유체역학적 막이 형성되지만, 시간이 경과하면 접촉부에 대한 윤활유 공급이 제한되면서 **스타베이션(starvation)**이 발생합니다.

### 두 챕터의 관계

- **Chapter 9**: 단일 접촉의 막두께 이론 – EHL의 기초 방정식, 접촉 기하학, 오일/그리스 막두께 공식, 스타베이션 모델
- **Chapter 10**: 실제 베어링에의 적용 – 베어링 표면에서의 박막 유동, 원심력 효과, 다중 접촉 환경에서의 스타베이션 모델 적용

### 핵심 메시지

- 완전 충만 조건의 막두께는 주로 **점도 × 속도**에 의해 결정됨
- 그리스 윤활 베어링은 대부분의 운전 시간 동안 **스타베이션 상태**에서 작동
- 스타베이션 시 막두께는 시간에 따라 \(h \propto t^{-1/\gamma}\)로 감소
- 막두께 감소율은 하중이 클수록 오히려 **낮아짐** (점도-압력 관계에 의함)

---

## 2. 종합 요약

### ▪ 2.1 연구 핵심 정보

| 항목        | 내용                                                                                                                            |
| --------- | ----------------------------------------------------------------------------------------------------------------------------- |
| **연구 대상** | • 롤링 베어링의 전동체-레이스웨이 간 윤활막 두께<br>• 그리스 및 오일 윤활 조건 모두 포함                                                                        |
| **주요 목적** | • EHL 막두께 이론의 체계적 정리<br>• 스타베이션 조건에서의 막두께 예측 모델 제시<br>• 단일 접촉 이론의 실제 베어링 적용 방법 제공                                             |
| **핵심 이론** | • Reynolds 방정식 및 박막 방정식<br>• Hertz 접촉 이론<br>• EHL 막두께 공식 (Hamrock-Dowson, Nijenbanning 등)<br>• 스타베이션 모델 (Damiens, Van Zoelen) |
| **적용 분야** | • 그리스 윤활 롤링 베어링 설계<br>• 베어링 수명 예측<br>• 윤활 조건 최적화                                                                              |

-------------

### ▪ 2.2 핵심 이론/모델 비교

| 연구자/모델                     | 연도        | 주요 특징                           | 적용 범위                             |
| -------------------------- | --------- | ------------------------------- | --------------------------------- |
| **Reynolds**               | 1886      | 저널 베어링 윤활막 압력 분포 유도             | HD 윤활 기초                          |
| **Ertel/Grubin**           | 1949      | 탄성 변형 + 압력-점도 효과 포함 → EHL 개념 확립 | EHL 기초                            |
| **Dowson & Higginson**     | 1977      | 선접촉 EHL 공식, 수치해의 곡선 피팅          | 선접촉, 완전 충만                        |
| **Hamrock & Dowson**       | 1978      | 점접촉 EHL 막두께 공식 (가장 널리 사용)       | 점접촉, 완전 충만, piezo-viscous elastic |
| **Nijenbanning et al.**    | 1994      | 전체 파라미터 영역에 유효한 고정밀 막두께 공식      | 모든 EHL 영역                         |
| **Kauzlarich & Greenwood** | 1972      | Herschel-Bulkley 모델 기반 그리스 막두께  | 선접촉, 그리스                          |
| **Dong & Qiang**           | 1988      | Bauer 유변학 모델 기반 그리스 막두께 보정      | 선접촉, 그리스                          |
| **Yang & Qian**            | 1987      | Bingham 모델 기반 타원 접촉 그리스 막두께     | 타원접촉, 그리스                         |
| **Chevalier/Damiens**      | 1996/2004 | 경미한 스타베이션 모델 (입구 메니스커스 기반)      | 스타베이션 초기                          |
| **Van Zoelen et al.**      | 2008      | 심각한 스타베이션 모델 (박막 유동 기반)         | 스타베이션 후기, 실제 베어링                  |

-------------

### ▪ 2.3 방법론 개요

| 구성요소         | 세부 내용                                                                                                                                                          |
| ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **지배 방정식**   | • Navier-Stokes → 박막 근사 → Reynolds 방정식<br>• 박막 방정식 (자유 표면 유동)                                                                                                  |
| **접촉 기하학**   | • Hertz 이론: 접촉 타원 크기, 최대 압력, 상호 접근량<br>• 탄성 변형: 반무한체 가정, 적분 방정식                                                                                                |
| **막두께 공식**   | • Hamrock-Dowson 공식 (Eq. 9.48): \(h_m/R_x = 3.63 U^{0.68}G^{0.49}/W^{0.073}\)<br>• Nijenbanning 공식 (Eq. 9.51): 점근해 기반 고정밀                                      |
| **그리스 유변학**  | • Herschel-Bulkley: \(\tau = \tau_y + K\dot{\gamma}^n\)<br>• Bingham: \(\tau = \tau_y + K\dot{\gamma}\)<br>• 항복응력의 막두께 영향은 무시 가능 → 기유 점도가 지배적                  |
| **스타베이션 모델** | • Damiens (경미): \(h_{cs} = h_{cff}(r_0^{-\gamma} + n)^{-1/\gamma}\)<br>• Van Zoelen (심각): \(h_c(t) = (C \cdot t + h_{c,0}^{-2})^{-1/2}\)<br>• 결합 모델: Eq. 10.23 |
| **박막 유동**    | • 원심력에 의한 펌핑 효과 (SRB, TRB, ACBB)<br>• 특성 시간: \(\tau_c = \eta/(\rho\omega^2\tilde{h}_i^2)\)                                                                     |

-------------

### ▪ 2.4 주요 실험/검증 결과

| 실험/검증 항목       | 방법                            | 핵심 결과                                                                           |
| -------------- | ----------------------------- | ------------------------------------------------------------------------------- |
| **그리스 막두께 측정** | 2-디스크 장치 (Poon, 1972)         | • 초기 막두께 > 기유 기반 예측<br>• 시간에 따라 감소, 에이징 그리스는 기유 수준                              |
| **간섭계 측정**     | Ball-on-disc (Kaneta 등, 2000) | • 증점제 덩어리가 접촉부 통과<br>• 완전 충만 시 기유보다 두꺼운 막                                       |
| **박막 유동 검증**   | WYKO 간섭계 (Van Zoelen, 2008)   | • TRB 내륜 레이스웨이의 층 두께 감소 정확히 예측<br>• 모델과 실험 우수한 일치                               |
| **베어링 막두께**    | 정전용량법 (Wilson, 1979)          | • 그리스 막두께 ≈ 기유 막두께 × 1.1~1.4<br>• 스타베이션은 거의 즉시 발생<br>• 200시간 후에도 완전 충만의 ~50% 유지 |
| **스핀들 베어링**    | 정전용량법 (Barz, 1996)            | • 고속에서 막두께 ≈ EHL 이론의 16-20%<br>• 고속 영역에서 막두께 거의 일정 (경계층 형성)                     |
| **볼 수 영향**     | Hannover rig (Baly 등, 2006)   | • 볼 수에 대해 막두께 거의 독립적<br>• 주요 재윤활 메커니즘이 접촉부 근처에 존재                               |

-------------

### ▪ 2.5 주요 발견사항

| 항목                    | 내용                                                                                                                                                      |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **막두께 결정 인자**         | • 완전 충만: 점도 × 속도가 지배적<br>• 스타베이션: 시간, 하중(압력), 점도가 지배적<br>• 속도는 스타베이션 막두께에 영향 없음 (심각 스타베이션 시)                                                            |
| **그리스 vs 오일**         | • 완전 충만: 그리스 막두께 ≈ 기유 막두께 × \((K/\eta_{oil})^{0.74}\)<br>• 항복응력은 막두께에 거의 무영향 (<3%)<br>• 실용적으로 기유 점도로 막두께 계산 가능                                          |
| **스타베이션 특성**          | • 선접촉: 측면 유동 작아 스타베이션 느림<br>• 점접촉: 스타베이션 빠르게 발생<br>• 막두께 감소율: \(h \propto t^{-1/\gamma}\), \(\gamma=2\) (심각), \(\gamma=3\sim15\) (경미)                   |
| **역설적 현상**            | • 하중 증가 → 초기 막두께 약간 감소, 그러나 감소율 낮아짐<br>• 속도 증가 → 완전충만 막두께 증가, 스타베이션 막두께 변화 없음                                                                           |
| **보충(replenishment)** | • 원심력/표면장력에 의한 보충은 너무 느림 (실용적 무의미)<br>• 그리스 전단 열화에 의한 유동성 회복이 더 중요<br>• 잔류 경계층 (6~80 nm) 형성으로 막 안정화 가능                                                  |
| **원심력 펌핑**            | • SRB, TRB, ACBB에서 원심력에 의한 윤활유 펌핑 효과 존재<br>• 특성 시간 \(\tau_c = \eta/(\rho\omega^2\tilde{h}_i^2)\)로 정의<br>• 5 cSt, 10000 rpm, 1 μm 초기층 → 90% 손실까지 약 1.4시간 |

-------------

### ▪ 2.6 적용 사례

| 사례 | 조건 | 결과 |
|------|------|------|
| **6204 베어링** | • 10000 rpm, 100°C<br>• 순수 레이디얼 하중 208 N<br>• 기유 점도 30 cSt | • M=696, L=18.3<br>• Nijenbanning: \(h_c\) = 0.665 μm<br>• Hamrock-Dowson: \(h_{min}\) = 0.51 μm |
| **22205 SRB** | • 5600 rpm, 120°C<br>• 순수 레이디얼 900 N (C/P=50)<br>• 기유 점도 5.2 mPa·s | • M=5000, L=13<br>• \(h_{cff}\) = 0.14 μm<br>• 경미→심각 스타베이션 전이 모델 적용 |
| **NJ 312 CRB** | • 외륜 회전<br>• 기유 점도 80 cSt | • 원심력에 의한 보충 시간 >> 과전동 간격<br>• 보충 효과 실질적으로 없음 |
| **스핀들 베어링 (ACBB)** | • 고속 운전<br>• Li-hydroxy-stearate 그리스 | • 막두께 16-20% (EHL 이론 대비)<br>• 볼 수에 무관 (접촉부 근처 재윤활) |

-------------

### ▪ 2.7 실무적 함의

| 영역         | 권장사항                                                                                                                   |
| ---------- | ---------------------------------------------------------------------------------------------------------------------- |
| **막두께 계산** | • 완전 충만: Nijenbanning 공식 사용 (전체 파라미터 영역)<br>• 그리스: 기유 점도로 계산 (실용적으로 충분)<br>• 스타베이션: Damiens + Van Zoelen 결합 모델 적용      |
| **베어링 설계** | • 스타베이션은 필연적 → 설계 시 반드시 고려<br>• 선접촉(CRB, TRB)은 점접촉(DGBB)보다 스타베이션에 유리<br>• 원심력에 의한 펌핑 효과 고려 필요 (SRB, TRB, ACBB)         |
| **그리스 선정** | • 기유 점도가 가장 중요한 파라미터<br>• 증점제 종류에 따라 경계층 두께 및 거동 상이<br>• 전단 안정성이 장기 윤활 성능에 직접 영향                                       |
| **운전 조건**  | • 고속: 완전 충만 막두께 증가, 그러나 스타베이션 빠르게 발생<br>• 고하중: 완전 충만 막두께 약간 감소, 그러나 스타베이션 감소율 낮음<br>• 스핀: 스타베이션 감소/제거 효과 (볼 베어링 축하중 시) |

---

## 3. 탄성유체윤활(EHL) 기초

### ▪ 3.1 역사

1886년 Reynolds의 유체윤활 방정식 발표 이후, Martin/Gumbel(1916)이 기어에 적용했으나 예측 막두께가 조도보다 얇아 실제 작동을 설명할 수 없었습니다. **1949년 Ertel/Grubin**이 탄성 변형과 압력-점도 효과를 포함하여 충분한 막두께를 예측하는 데 성공하면서 **탄성유체윤활(EHL)** 개념이 확립되었습니다.

주요 이정표:
- **1977**: Dowson & Higginson – 선접촉 EHL 공식
- **1978**: Hamrock & Dowson – 점접촉 EHL 막두께 공식 (가장 널리 사용)
- **1987**: Lubrecht et al. – 다중격자 기법 도입으로 수치 정확도 향상
- **1994**: Nijenbanning et al. – 전체 파라미터 영역 고정밀 막두께 공식

### ▪ 3.2 Reynolds 방정식

Navier-Stokes 방정식에 박막 근사(thin film approximation)를 적용하면 Reynolds 방정식이 유도됩니다:

$$
\frac{\partial}{\partial x}\left[\frac{\rho h^3}{\eta}\frac{\partial p}{\partial x}\right] + \frac{\partial}{\partial y}\left[\frac{\rho h^3}{\eta}\frac{\partial p}{\partial y}\right] = 6(u_1+u_2)\frac{\partial(\rho h)}{\partial x} + 12\frac{\partial(\rho h)}{\partial t} \tag{9.24}
$$

**압력 발생의 세 가지 메커니즘:**
- **쐐기 효과(Wedge)**: 입구의 쐐기 형상에 의한 압력 생성
- **신장 효과(Stretch)**: 표면 접선 속도 변화에 의한 효과 (롤링 베어링에서 무시 가능)
- **압착 효과(Squeeze)**: 표면 거칠기, 진동 해석에서 중요

### ▪ 3.3 박막 방정식 (Thin Layer Equation)

자유 표면에서의 박막 유동을 기술하는 방정식:

$$
\frac{\partial \tilde{h}}{\partial t} + \frac{1}{3\eta}\frac{\partial}{\partial x}\left[\tilde{h}^3\left(f_z\frac{\partial \tilde{h}}{\partial x} + \sigma\frac{\partial^3 \tilde{h}}{\partial x^3} + f_x\right)\right] = 0 \tag{9.16}
$$

이 방정식은 원심력이나 표면장력에 의해 구동되는 레이스웨이 위의 윤활유 유동을 계산하는 데 사용됩니다.

---

## 4. 접촉 기하학 및 변형

### ▪ 4.1 강체 기하학

전동체-레이스웨이 접촉의 간극은 포물선으로 근사화됩니다:

$$
h(x, y) = h_0 + \frac{x^2}{2R_x} + \frac{y^2}{2R_y} \tag{9.27, 9.30}
$$

**등가 곡률반경:**

$$
\frac{1}{R_x} = \frac{1}{R_{x_1}} + \frac{1}{R_{x_2}}, \quad \frac{1}{R_y} = \frac{1}{R_{y_1}} \pm \frac{1}{R_{y_2}} \tag{9.28, 9.31}
$$

![](./parsed_result_Grease%20Lubrication%20in%20Rolling%20Bearings_Ch%209,%2010.pdf/images/7227630f4e3e2ff7763bf31087568aa5f36c08482b34a0e7d128373c97b1b0ec.jpg)
*그림 9.3: 내륜 레이스웨이와 전동체 사이의 간극 (주행 방향)*

### ▪ 4.2 탄성 변형 (Hertz 이론)

**등가 탄성 계수:**

$$
\frac{2}{E'} = \frac{1-v_1^2}{E_1} + \frac{1-v_2^2}{E_2} \tag{9.34}
$$

**접촉 타원 반폭:**

$$
a_x = \left(\frac{6R_xF\kappa E_c}{E'\pi(1+\lambda)}\right)^{1/3} \tag{9.35}
$$

**최대 Hertz 압력:**

$$
p_{max} = \frac{3F}{2\pi a_x a_y} \tag{9.40}
$$

![](./parsed_result_Grease%20Lubrication%20in%20Rolling%20Bearings_Ch%209,%2010.pdf/images/275a3a413c78a1b2e3436746968b15ece99924101c5793e5518e7fced270d0d8.jpg)
*그림 9.4: 접촉 타원*

---

## 5. EHL 막두께 - 오일

### ▪ 5.1 Hamrock-Dowson 공식 (1978)

가장 널리 사용되는 EHL 막두께 공식:

$$
\frac{h_m}{R_x} = 3.63\frac{U^{0.68}G^{0.49}}{W^{0.073}}\left(1-e^{-0.68\kappa_d}\right) \tag{9.48}
$$

$$
\frac{h_c}{R_x} = 2.69\frac{U^{0.67}G^{0.53}}{W^{0.067}}\left(1-e^{-0.73\kappa_d}\right)
$$

**무차원 파라미터:**

$$
U = \frac{\eta_0 u_s}{2E'R_x}, \quad G = \alpha E', \quad W = \frac{F}{E'R_x^2} \tag{9.49}
$$

**핵심**: 막두께는 주로 **점도 × 속도** (U 파라미터)에 의해 결정됩니다.

### ▪ 5.2 Nijenbanning 공식 (1994)

전체 파라미터 영역에 유효한 고정밀 공식. Moes 무차원 수 사용:

$$
M = \frac{F}{E'R_x^2}\left(\frac{E'R_x}{\eta_0 u_s}\right)^{3/4}, \quad L = \alpha E'\left(\frac{E'R_x}{\eta_0 u_s}\right)^{-1/4}, \quad \lambda = \frac{R_x}{R_y} \tag{9.50}
$$

Nijenbanning 공식은 4개의 점근해(\(H_{RI}, H_{RP}, H_{EI}, H_{EP}\))를 조합한 구조로, 복잡해 보이지만 명시적이며 계산기로도 계산 가능합니다 (Eq. 9.51~9.53).

![](./parsed_result_Grease%20Lubrication%20in%20Rolling%20Bearings_Ch%209,%2010.pdf/images/b5cbfb016ddf81aaaffcad65ad283b7810709b01cdb7248c90bdaf40096f426c.jpg)

![](./parsed_result_Grease%20Lubrication%20in%20Rolling%20Bearings_Ch%209,%2010.pdf/images/04a3323d29777c52c3668e2f5fc4cbf9c386a531834d0ea8dda44e7936b226ed.jpg)
*그림 9.5: 다양한 하중 조건에서의 EHL 압력 및 막두께 분포 (Wijnant)*

---

## 6. EHL 막두께 - 그리스

### ▪ 6.1 측정 결과 요약

- **Poon (1972)**: 초기 막두께가 기유 기반 예측보다 두꺼움 → 시간에 따라 감소 → 에이징된 그리스는 기유 수준
- **Kaneta et al. (2000)**: 간섭계로 증점제 덩어리가 접촉부를 통과하는 것을 시각화

![](./parsed_result_Grease%20Lubrication%20in%20Rolling%20Bearings_Ch%209,%2010.pdf/images/297144fb57197c4bc95f80a7417fedd0be52f4c6b5d23fd6e6103901afd15ab5.jpg)
*그림 9.6: 2-디스크 장치의 막두께 측정 (Poon, 1972)*

### ▪ 6.2 막두께 모델

**핵심 결론**: 그리스의 항복응력은 막두께에 거의 영향을 미치지 않으며 (<3%), 막두께는 주로 **기유 점도**에 의해 결정됩니다.

**Yang & Qian (1987)** – Bingham 모델 기반 보정:

$$
\frac{h}{h_{oil}} = \left(\frac{K}{\eta_{oil}}\right)^{0.74} \tag{9.72}
$$

여기서 \(K\)는 Bingham 그리스 점도입니다. 실용적으로 **완전 충만 막두께는 기유 점도로 계산**할 수 있습니다.

---

## 7. 스타베이션

### ▪ 7.1 스타베이션의 개념

접촉 입구에 충분한 윤활유가 공급되지 않으면, 메니스커스가 Hertz 접촉 영역에 가까워지며 막두께가 감소합니다. 윤활유 공급이 더 줄어들면 막두께는 공급된 윤활유 층의 두께에 의해 결정됩니다.

![](./parsed_result_Grease%20Lubrication%20in%20Rolling%20Bearings_Ch%209,%2010.pdf/images/34e48a69c71ae6126f363719f4fd8956df771012529eb4ada829a6c19284a399.jpg)

![](./parsed_result_Grease%20Lubrication%20in%20Rolling%20Bearings_Ch%209,%2010.pdf/images/f8788789ac1e3eb3217af60555383ae30c03a5c07cae287ab202c72d62a66cc4.jpg)
*그림 9.11: 완전 충만 및 스타베이션 EHL 접촉의 막두께/압력 비교*

### ▪ 7.2 경미한 스타베이션 (Damiens 모델)

입구 윤활유 층 두께의 함수로 막두께를 표현:

$$
\frac{h_c}{h_{cff}} = \frac{r}{\sqrt[\gamma]{1 + r^\gamma}} \tag{9.73}
$$

연속 과전동 시 막두께 감소:

$$
h_c(t) \propto t^{-1/\gamma} \tag{9.76}
$$

여기서 \(\gamma\)는 측면 유동 저항 파라미터로, 원형 접촉에서 약 3, 넓은 타원 접촉에서 최대 15까지입니다.

### ▪ 7.3 심각한 스타베이션 (Van Zoelen 모델)

Hertz 건조 접촉 압력 분포를 가정하고, 측면 유동에 의한 층 두께 감소를 계산:

$$
h_c(t) = \frac{1}{\sqrt{\frac{1}{6}\bar{\rho}_c^2 \mathcal{F}_k(0)t + h_{c,0}^{-2}}} \tag{9.86}
$$

**핵심 특성:**
- 모든 접촉 형태에서 \(\gamma = 2\)
- **속도가 방정식에 포함되지 않음** → 스타베이션율은 시간에만 의존
- **하중 증가 → \(\mathcal{F}_k\) 감소 → 막두께 감소율 낮아짐** (점도의 지수적 압력 의존성)

**스타베이션 시 물리적 파라미터의 영향 (표 9.1):**

| 입력 | 스타베이션 막두께 | 완전 충만 막두께 |
|------|:---:|:---:|
| 압력 ↑ | h ↑ | h ↓ |
| 점도 ↑ | h ↑ | h ↑ |
| 속도 ↑ | h = | h ↑ |

### ▪ 7.4 기유 보충 (Replenishment)

접촉부 뒤편에서 형성된 **리지(ridge)**로부터의 보충은 매우 느리며, 특히 타원/선접촉(실제 베어링)에서는 실질적으로 무의미합니다. 표면장력과 원심력에 의한 보충도 과전동 간격에 비해 너무 느립니다.

### ▪ 7.5 그리스 윤활 접촉의 스타베이션

**선접촉**: Herschel-Bulkley 유변학에 의해 입구에서의 측면 유동이 억제되므로 스타베이션이 느리게 진행됩니다. 초기에는 완전 충만 상태를 유지하다가, 그리스가 에이징되면 기유와 유사한 거동을 보이며 스타베이션이 시작됩니다.

**점접촉**: 스타베이션이 빠르게 발생합니다. Mérieux et al.은 4가지 거동 유형을 분류:
1. 완전 충만
2. 스타베이션
3. 스타베이션 후 안정화 (경계층 형성)
4. 스타베이션 후 회복 (그리스 전단 열화에 의한 유동성 회복)

**잔류 경계층**: Cann(2001)에 따르면 그리스 막은 두 부분으로 구성됩니다:

$$
h_T = h_R + h_{EHL} \tag{9.93}
$$

잔류막 두께: \(6~\mathrm{nm} < h_R < 80~\mathrm{nm}\)

---

## 8. 스핀

볼 베어링에 축하중이 작용하면 볼의 스핀 운동이 발생합니다. Cann과 Lubrecht(2007)의 ball-on-disc 측정에 따르면, **스핀은 스타베이션을 감소/제거**하는 효과가 있습니다. 스핀 운동이 트랙 옆의 윤활유 리지를 트랙 안으로 끌어들이기 때문입니다.

---

## 9. 베어링 표면의 박막 유동

### ▪ 9.1 접촉 보충 (Ch 10.1.1)

**결론**: 원심력과 표면장력에 의한 리지의 보충은 과전동 간격(밀리초 단위)에 비해 너무 느림 → **실질적으로 무의미**.

- 내륜: 원심력이 리지를 얇게 만들어 궁극적으로 윤활유를 튕겨냄
- 외륜 회전 시: 원심력이 보충을 촉진하나, 여전히 너무 느림

### ▪ 9.2 원심력에 의한 박막 유동 (Ch 10.1.2)

SRB, TRB, ACBB 등 접촉각이 있는 베어링에서 원심력의 접선 성분이 윤활유를 **펌핑**하여 윤활유 손실을 가속합니다.

**지배 방정식:**

$$
\frac{1}{3\eta}\frac{1}{r}\frac{\partial}{\partial s}\left(r\tilde{h}^3 f_s\right) + \frac{\partial \tilde{h}}{\partial t} = 0 \tag{10.1}
$$

**TRB 내륜의 해석해:**

$$
\tilde{h}(s,t) = \sqrt{\frac{3\eta}{4\sin^2(\alpha)\rho\omega^2 t}\left(1-\frac{1}{\left(\frac{\sin\alpha}{R}s+1\right)^{4/3}}\right)} \tag{10.3}
$$

![](./parsed_result_Grease%20Lubrication%20in%20Rolling%20Bearings_Ch%209,%2010.pdf/images/3540fa7698b5496cdd9e07643826b1be521586e453cba890aaf681310ceb47a1.jpg)
*그림 10.5: TRB 내륜 레이스웨이의 층 두께 감소 – 모델과 실험 비교*

### ▪ 9.3 베어링 구성요소의 박막 유동 결합 (Ch 10.1.3)

**특성 시간** (펌핑 효과의 심각성 평가):

$$
\tau_c = \frac{\eta}{\rho\omega^2\tilde{h}_i^2} \tag{10.4}
$$

**예시**: 5 cSt 기유, 10000 rpm, 1 μm 초기층 → \(\tau_c\) = 5초 → 90% 손실까지 약 **1.4시간**

![](./parsed_result_Grease%20Lubrication%20in%20Rolling%20Bearings_Ch%209,%2010.pdf/images/321a8a86d6558eadb0f4200e78ce2c4f65d282c70389732c8aa16678f3eec1c4.jpg)
*그림 10.6: SRB 및 TRB의 스케일링된 층 두께 감소 (무차원 시간)*

---

## 10. 롤링 베어링의 스타베이션 EHL

### ▪ 10.1 경미한 스타베이션 (Damiens 모델의 베어링 적용)

모든 접촉이 최고 하중 접촉과 동일하다고 가정하고 과전동 횟수를 시간으로 변환:

$$
h_{cs} = \frac{1}{\sqrt[\gamma]{C_D t + h_{cff}^{-\gamma}}} \tag{10.9}
$$

$$
C_D = \frac{u_s}{2\pi h_{cff}^\gamma}\left(\frac{z}{d_m} + \frac{1}{d_r}\right)
$$

### ▪ 10.2 심각한 스타베이션 (Van Zoelen 모델의 베어링 적용)

베어링의 모든 접촉부에서의 측면 유동 합산:

$$
\mathcal{F}(y) = \sum_{k=1}^{n_c} \mathcal{F}_k(y), \quad n_c = 2n_r \tag{10.14}
$$

중심 막두께의 시간 변화:

$$
h_c(t) = \frac{1}{\sqrt{\frac{1}{6}\bar{\rho}_c^2 \mathcal{F}(0)t + h_{c,0}^{-2}}} \tag{10.18}
$$

### ▪ 10.3 경미/심각 스타베이션 결합

두 모델의 부드러운 전이를 위해:

$$
h = h_Z + \frac{h_D - h_Z}{1 + (t/t_{tr})^m}, \quad m = 3 \tag{10.23}
$$

![](./parsed_result_Grease%20Lubrication%20in%20Rolling%20Bearings_Ch%209,%2010.pdf/images/5e80dffc5a7ddd619df8d736f98b9c8da21a7c7db9dca334cfabab1a15d15c96.jpg)
*그림 10.8: Damiens 모델(경미)과 Van Zoelen 모델(심각)의 결합 (22205 베어링)*

**핵심**: Van Zoelen 모델의 초기 층 두께 선택(\(c_t h_{cff}\))은 장시간 후의 막두께 예측에 영향 없음 → 심각한 스타베이션 예측의 강건성 확보

---

## 11. 케이지 간극과 막두께

Damiens et al.(2004)의 광학 ball-on-disc 실험 결과:

| 윤활 조건 | 케이지 효과 |
|----------|-----------|
| **오일 윤활** | 케이지가 항상 막두께를 감소시킴 (스크래핑 효과) |
| **그리스 윤활** | 케이지가 막두께를 유지/증가시킴 (그리스를 트랙으로 밀어넣음) |

- 간극이 작을수록 막두께 증가 → 케이지가 그리스를 트랙으로 재분배
- 너무 작은 간극은 스크래핑 효과 주의

![](./parsed_result_Grease%20Lubrication%20in%20Rolling%20Bearings_Ch%209,%2010.pdf/images/149ba6f9cafce15543cfad1f431ff4529e442353b1da22ed139ccd435a4edaac.jpg)
*그림 10.10: 그리스 윤활 시 케이지 간극에 따른 막두께*

---

## 12. 전체 베어링 막두께

### 주요 실험적 발견사항

- **Wilson (1979)**: 그리스 막두께 ≈ 기유 기반 × 1.1~1.4 (완전 충만 시). "겉보기 점도"가 기유보다 30-35% 높음.
- **Muennich & Gloeckner**: Li 그리스는 50-80% 높은 겉보기 점도, Na/Ca/Ba 그리스는 200-260% 높은 겉보기 점도
- **Barz (1996)**: 고속 스핀들 베어링에서 막두께 = EHL 이론의 16-20%. 고속에서 막두께 거의 일정 (경계층 형성)
- **Baly et al. (2006)**: **볼 수에 대해 막두께 거의 독립적** → 주요 재윤활 메커니즘이 접촉부 근처에 존재하며, 접촉 간 간섭 없음

![](./parsed_result_Grease%20Lubrication%20in%20Rolling%20Bearings_Ch%209,%2010.pdf/images/513ce85a767eeff377d8790aede6c7ede4cf67eb241c7154e9c70da57cb1ddf8.jpg)
*그림 10.11: 스핀들 베어링의 상대 막두께 vs 속도 (Barz, 1996)*

![](./parsed_result_Grease%20Lubrication%20in%20Rolling%20Bearings_Ch%209,%2010.pdf/images/5d27a035dad832e4e74cd69cc2b0368954e19d199f9c9fc45c67b61d42cdb934.jpg)
*그림 10.12: 볼 수가 스핀들 베어링의 막두께에 미치는 영향 (Baly et al., 2006)*

---

## 13. 기호 설명

| 기호 | 설명 | 단위 |
|------|------|------|
| \(h\) | 윤활막 두께 (film thickness) | m |
| \(\tilde{h}\) | 윤활유 층 두께 (layer thickness) | m |
| \(h_c\) | 중심 막두께 (central film thickness) | m |
| \(h_{cff}\) | 완전 충만 중심 막두께 | m |
| \(h_m\) | 최소 막두께 (minimum film thickness) | m |
| \(p\) | 압력 | Pa |
| \(p_h\) | 최대 Hertz 접촉 압력 | Pa |
| \(\eta, \eta_0\) | 동점도, 대기압 동점도 | Pa·s |
| \(\alpha\) | 압력-점도 계수 | Pa⁻¹ |
| \(\rho, \rho_0\) | 밀도, 대기압 밀도 | kg/m³ |
| \(u_s\) | 표면 합속도 (sum velocity) | m/s |
| \(R_x, R_y\) | 등가 곡률반경 | m |
| \(E'\) | 등가 탄성 계수 | Pa |
| \(F\) | 접촉 하중 | N |
| \(a_x, a_y\) | 접촉 타원 반폭 | m |
| \(\kappa\) | 접촉 타원 편심률 (\(a_x/a_y\)) | - |
| \(\lambda\) | 곡률반경 비 (\(R_x/R_y\)) | - |
| \(M, L\) | Moes 무차원 수 (하중수, 윤활수) | - |
| \(U, G, W\) | Hamrock-Dowson 무차원 파라미터 | - |
| \(\gamma\) | 스타베이션 파라미터 | - |
| \(\mathcal{F}_k\) | 측면 유동 함수 | - |
| \(\tau_y\) | 항복 전단응력 | Pa |
| \(K\) | 컨시스턴시 지수 / Bingham 점도 | Pa·s^n |
| \(n\) | 유동 거동 지수 | - |
| \(\tau_c\) | 특성 시간 (펌핑) | s |
| \(\omega\) | 각속도 | rad/s |
| \(\sigma\) | 표면장력 | J/m² |
| \(z\) | 전동체 수 | - |
| \(d_m\) | 피치 직경 | m |
| \(d_r\) | 전동체 직경 | m |
| \(n_r\) | 전동체 수 | - |
| \(l_t\) | 총 트랙 길이 | m |

---

## 14. 참고문헌

본 문서에서 인용된 주요 참고문헌:

- [110] Cann, P.M., 2001, Film thickness measurements
- [115] Cann, P.M., Lubrecht, A.A., 2007, Impact of spin on film thickness
- [124] Chevalier, F., et al., 1996, Starved EHL calculations
- [156,158] Damiens, B., et al., 2004, Starved EHL curve fits
- [177,178] Dowson, D., Higginson, G.R., EHL solutions and engineering formulae
- [218] Gershuni, L., et al., 2008, Thin layer flow models
- [233] Grubin, A.N., 1949, EHL concept
- [243,244,245] Hamrock, B.J., Dowson, D., 1977-1978, Film thickness formulae
- [249] Harris, T.A., Rolling Bearing Analysis
- [309] Kaneta, M., et al., 2000, Interferometry measurements
- [315] Kauzlarich, J.J., Greenwood, J.A., 1972, Grease EHL for line contacts
- [401] Martin, H.M., 1916, Lubrication of gear teeth
- [417] Mérieux, J., et al., Grease film thickness classification
- [447] Nijenbanning, G., et al., 1994, High-accuracy film thickness formula
- [493] Reynolds, O., 1886, Hydrodynamic lubrication
- [582,583,584,585,586] Van Zoelen, M.T., et al., 2008, Thin film flow and starvation models
- [587,589] Venner, C.H., Lubrecht, A.A., Multigrid methods in EHL
- [610] Wijnant, Y.H., Contact dynamics in EHL
- [625] Yang, P., Qian, S., 1987, Grease EHL for elliptical contacts
- [638] Zhu, W.S., Neng, Y.T., Grease film thickness measurements

---

**문서 작성일**: 2026-03-10
**원본 출처**: Lugt, P.M., "Grease Lubrication in Rolling Bearings", Chapter 9 & 10, John Wiley & Sons, 2013
**정리**: MinerU AI Document Parser 기반 + Claude 정리

---

**끝**
