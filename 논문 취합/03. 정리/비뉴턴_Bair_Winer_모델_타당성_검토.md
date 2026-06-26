# 비뉴턴 Bair-Winer 모델의 그리스 윤활 해석 적용 타당성 검토

## 1. Bair-Winer 모델 개요

### 1.1 모델 정의

Bair-Winer(B-W) 모델은 1979년 Georgia Institute of Technology의 Scott Bair와 Ward O. Winer가 제안한 **수정 Maxwell 점탄성 구성 방정식**으로, **한계 전단응력(limiting shear stress)** 개념을 도입한 것이 핵심이다.

모델이 요구하는 3개 물성 파라미터:

| 파라미터 | 기호 | 의미 |
|---------|------|------|
| 저전단 뉴턴 점도 | $\eta_0$ | 저전단율에서의 점도 (Barus/Roelands 식으로 압력 의존) |
| 한계 탄성 전단계수 | $G_\infty$ | 유체의 탄성 한계 |
| 한계 전단응력 | $\tau_L$ | 유체가 견딜 수 있는 최대 전단응력 |

구성 방정식:

$$
\dot{\gamma} = \frac{\dot{\tau}}{G_\infty} + \frac{\tau_L}{\eta} \ln\left(\frac{1}{1 - \tau/\tau_L}\right)
$$

**물리적 의미**: 저전단율에서는 뉴턴 유체처럼 거동하고, 전단율 증가 시 점탄성 천이를 거쳐, **한계 전단응력 $\tau_L$에 도달하면 응력이 더 이상 증가하지 않고 소성 항복**하는 거동을 보인다.

한계 전단응력은 압력에 선형 비례:

$$
\tau_L = \Lambda \cdot p
$$

여기서 $\Lambda$는 한계 전단응력 계수로, 대부분의 윤활유에서 $\Lambda \approx 0.05$–$0.10$ 범위이다.

### 1.2 다른 비뉴턴 모델과의 비교

| 모델 | 유형 | 핵심 특징 | 고전단율 거동 | 그리스 적용 |
|------|------|----------|------------|-----------|
| **Bair-Winer** | 점탄성 + 한계응력 | 응력 상한 존재 | 소성 항복 — 응력 포화 | 기유 위주 |
| **Ree-Eyring** | 응력 활성화 유동 | sinh 법칙 | 대수적 증가 — 응력 포화 없음 | 기유 위주 |
| **Carreau(-Yasuda)** | 일반화 뉴턴 유체 | 두 뉴턴 평탄역 사이 멱법칙 전이 | 점도 감소, 응력은 계속 증가 | 기유 위주 |
| **Herschel-Bulkley** | 항복응력 + 멱법칙 | 항복응력 이하에서 유동 없음 | 멱법칙 유동 | **그리스 표준** |
| **수정 Carreau-Yasuda + LSS** | Bair의 현대적 접근 | 전단 박화 + 한계응력 결합 | 전단 박화 후 응력 포화 | 기유 (최신) |

**핵심 차이점**:

- **B-W vs Ree-Eyring**: B-W는 전단응력에 **경성 상한(hard cap)**을 부여하여 트랙션 계수가 고미끄럼비에서 포화되는 실험 결과를 정확히 재현한다. Eyring 모델은 응력이 대수적으로 계속 증가하여 **마찰을 과대 예측**하는 경향이 있다.
- **B-W vs H-B**: H-B는 그리스 고유의 **항복응력**을 포착하지만, 고압 접촉부 내부의 **한계 전단응력 거동**을 기술하지 못한다. B-W는 그 반대이다.

---

## 2. 그리스 윤활 해석 적용 타당성

### 2.1 적용 가능한 영역 (FOR)

**(1) 한계 전단응력은 그리스에서도 물리적으로 실재한다**

연구에 따르면 "the behaviour of grease does not differ from that of liquid lubricants regarding limiting shear stress behavior." 접촉부 내부의 극고압(0.5–3 GPa) 조건에서 그리스 기유는 일반 윤활유와 동일한 한계 전단응력 거동을 보인다.

**(2) EHL 접촉부 내부에서는 기유 거동이 지배적**

접촉부 고압 영역(Hertz 접촉 영역)에서는 그리스가 주로 **기유 성분**을 통해 거동하며, 증주제 섬유는 입구 영역에서의 막 형성에 주로 기여한다. 따라서 접촉부 내부의 트랙션/마찰 해석에는 기유에 대한 B-W 모델 적용이 합리적이다.

**(3) 트랙션 예측 우수성**

B-W 모델은 EHL 접촉의 트랙션 예측에서 "more realistic, particularly when evaluating the traction in the EHD contact"으로 평가된다. Bair & Habchi(2016, 2024)는 점도계 측정 유변학 물성만으로 트랙션 계수와 막두께를 정량적으로 예측하여 실험과 "remarkable agreement"를 달성하였다.

**(4) 20MW+ TRB 조건에서의 의미**

본 시스템의 극고하중(~1000 kN) 조건에서 Hertz 접촉압은 1–2 GPa에 달한다. 이 영역에서:

$$
\tau_L = \Lambda \cdot p \approx 0.07 \times 1.5 \text{ GPa} = 105 \text{ MPa}
$$

이는 마찰 계수의 상한을 $\mu_{max} \approx \Lambda \approx 0.07$로 제한하며, Eyring 모델이 예측하는 무한 증가보다 실험적으로 관찰되는 거동에 부합한다.

### 2.2 적용 한계 (AGAINST)

**(1) 그리스는 단순 유체가 아니다**

그리스는 항복응력, 틱소트로피(thixotropy), 벽면 미끄럼(wall slip), 점탄성을 동시에 보이는 복합 유체이다. B-W 모델은 이러한 복합 거동을 포착하도록 설계되지 않았다.

**(2) 증주제 효과 미반영**

B-W는 단상(single-phase) 유체 모델이다. 실제 그리스에서는 증주제 입자가 접촉부에 진입하여 기유와 분리되고, 경계층을 형성한다. 증주제 유형(Li, Ca, polyurea 등)에 따라 접촉부 투과율이 다르며, 이는 B-W 모델에 반영되지 않는다.

**(3) 입구 영역의 유변학 한계**

막 형성이 시작되는 입구 영역에서 그리스 유변학은 **항복응력에 의해 지배**되며, 이는 H-B 모델이 더 적합하다. B-W의 강점은 고압 영역에 국한된다.

**(4) 스타베이션 미반영**

그리스 윤활 접촉은 대부분 스타베이션 상태이다. B-W 모델은 벌크 유변학만 기술하며, 그리스 윤활의 지배적 현상인 스타베이션 메커니즘을 내재적으로 포함하지 않는다.

**(5) 파라미터 측정 난이도**

B-W의 3개 파라미터는 고압 점도계에서 측정해야 하며, 소수의 기유에 대해서만 측정이 완료되어 있다. 그리스 전체 시스템에 대한 측정은 더욱 어렵다.

### 2.3 타당성 종합 평가

| 적용 영역 | 타당성 | 비고 |
|----------|--------|------|
| 접촉부 내부 트랙션/마찰 예측 | **높음** | 기유의 한계 전단응력 거동이 지배 |
| 마찰 계수 상한 추정 | **높음** | $\mu_{max} \approx \Lambda$ 으로 실용적 |
| 접촉부 내부 막두께 | **중간** | 전단 박화 효과 반영 가능하나, 입구 조건 별도 필요 |
| 입구 영역 막 형성 | **낮음** | H-B 모델이 더 적합 |
| 스타베이션 해석 | **없음** | 별도 스타베이션 모델 필요 |
| 열적 효과 | **중간** | 열-B-W 결합 해석 연구 존재 (Khonsari 등) |

---

## 3. 주요 연구 사례

### 3.1 Bair-Winer 모델 기반 연구 (기유/윤활유)

| 연구 | 내용 | 핵심 결과 |
|------|------|----------|
| Bair & Winer (1979), ASME J. Lubr. Technol. | 원 모델 제안. HVI 650, 5P4E, Santotrac 50에 대해 0.6–2.5 GPa 범위 검증 | 디스크 머신 실험과 일치 |
| Bair & Winer (1992), ASME J. Tribol. | 고압 고전단 유변학 체계적 측정 (2–200 MPa) | 한계 전단응력의 압력 선형 의존성 확인 |
| Khonsari 등 (1994), ASME J. Tribol. | 열-EHL 해석에 B-W 구성방정식 적용 | 열효과 포함 시 트랙션 예측 정확도 향상 |
| Bair & Habchi (2016), Proc. IMechE J | 기어유의 복합 전단 박화에 대한 정량적 EHL 막 형성 | 점도계 측정치만으로 막두께·트랙션 정량 예측 성공 |
| Habchi 등 (2014), ASME J. Tribol. | 한계 전단응력 + 전단 박화 결합 EHL 모델 | 수정 Carreau + LSS의 수치적 구현 |
| Bair & Habchi (2024), Proc. IMechE J | 전단 의존 점도의 해석적 EHL 막두께 공식 영향 | 기존 Hamrock-Dowson 공식의 유의미한 오차 지적 |
| Bair (2019), *High Pressure Rheology for Quantitative EHL*, 2nd Ed. | 포괄적 단행본 | 정량적 EHL을 위한 고압 유변학의 표준 참고서 |

### 3.2 그리스 EHL 비뉴턴 모델 연구

| 연구 | 모델 | 내용 | 핵심 결과 |
|------|------|------|----------|
| Kauzlarich & Greenwood (1972) | H-B | 그리스 EHL에 H-B 모델 최초 적용 | 항복응력이 입구 막 형성에 미치는 영향 분석 |
| Yoo 등 (1997), Tribology Trans. | H-B | 그리스 열-EHL 수치 해석 | H-B 파라미터의 온도 의존성 중요 |
| Cann (1999), Lubrication Science | 실험 | 그리스 EHL 접촉부의 증주제 층 관찰 | 잔류 증주제 경계층이 막두께에 기여 확인 |
| Lugt (2013), *Grease Lubrication in Rolling Bearings* | H-B 중심 | 그리스 윤활 종합 교과서 | 항복응력 영향 <3% (중-고속), 극저속에서는 재부상 가능 |
| 혼합 EHL 그리스 해석 (2021), Tribology Int. | 수정 H-B + 표면 조도 | 그리스 윤활 혼합 EHL | 표면 조도와 비뉴턴 효과의 결합 영향 |
| 그리스 혼합 EHL 해석법 (2022), Tribology Letters | H-B 기반 | 그리스 혼합 EHL 방법론 | 저속 고하중 조건에서의 혼합윤활 체계적 분석 |

### 3.3 풍력 터빈 메인베어링 윤활 연구

| 연구 | 내용 | 핵심 결과 |
|------|------|----------|
| Hart 등 (2022), Wind Energy Science, Part 1 | 풍력 메인베어링 EHL 이론 리뷰 | 비뉴턴 전단 박화가 롤러 미끄럼 영역에서 중요 |
| Hart 등 (2022), Wind Energy Science, Part 2 | 1.5 MW 터빈 SRB 메인베어링 EHL 시뮬레이션 | 35°C에서 혼합윤활 전이, 30% 스타베이션 시 ~90% 혼합윤활 |
| Lubrication reliability in random wind (2023) | 불규칙 풍하중 하의 윤활 신뢰성 | SCADA 기반 실운전 조건에서 윤활 상태 평가 |
| Tribological failure analysis (2022), PMC | 풍력 베어링 트라이볼로지 고장 분석 리뷰 | 윤활 부족이 주요 고장 원인 |

**중요 발견**: 풍력 터빈 메인베어링에 Bair-Winer 또는 수정 Carreau + LSS 접근을 그리스 윤활에 체계적으로 적용한 공개 연구는 **현재까지 부재**하다. 이는 명확한 **연구 공백(research gap)**이다.

---

## 4. 20MW+ 풍력 TRB 메인베어링에 대한 적용 전략 제안

### 4.1 하이브리드 모델 접근법

기존 문서([그리스_윤활_분석_1.md](그리스_윤활_분석_1.md))에서 분석한 본 시스템의 조건(극저속 $u_s \leq 1$ m/s, 극고하중 ~1000 kN, TRB 선접촉)을 고려할 때, **단일 모델로는 불충분**하며 다음과 같은 하이브리드 접근이 필요하다:

```
┌─────────────────────────────────────────────────────────┐
│                    그리스 윤활 EHL 해석                     │
├────────────────┬────────────────┬───────────────────────┤
│   입구 영역       │   접촉부 내부      │   출구/측면 유동          │
│  (Film Formation)│  (Traction)     │  (Starvation)        │
├────────────────┼────────────────┼───────────────────────┤
│ Herschel-Bulkley │ Bair-Winer     │ Van Zoelen /         │
│ (항복응력 + 전단박화) │ 또는 수정          │ Damiens 모델          │
│                │ Carreau + LSS  │                       │
├────────────────┼────────────────┼───────────────────────┤
│ τ_y, k, n      │ η₀(p,T), G∞,  │ F_k(y), ρ_c          │
│ (그리스 파라미터)    │ Λ (기유 파라미터)   │ (막두께 감소율)           │
└────────────────┴────────────────┴───────────────────────┘
```

### 4.2 구현 단계

**1단계: 기유 고압 유변학 특성화**

- 그리스 기유(예: PAO 또는 광유, ISO VG 220–460)에 대해 고압 점도계 측정 필요
- 측정 항목: $\eta(p, T, \dot{\gamma})$, $\tau_L(p)$, $G_\infty(p)$
- Bair의 고압 점도계(Georgia Tech) 또는 유사 장비 활용

**2단계: 입구 영역 H-B 모델 구축**

- 그리스 전체에 대한 H-B 파라미터 ($\tau_y$, $k$, $n$) 온도 의존 측정
- 입구 막 형성에 대한 비뉴턴 보정 계수 계산

**3단계: 접촉부 B-W/수정 Carreau + LSS 적용**

- 1단계 측정 기유 물성으로 접촉부 내부 트랙션 해석
- 마찰 계수 및 발열량 예측

**4단계: 스타베이션 및 혼합윤활 결합**

- Van Zoelen 측면유동 모델과 결합
- 혼합윤활 영역($\Lambda \approx 1$–$3$)에서의 표면 조도 효과 포함

### 4.3 본 시스템에서 B-W 모델이 주는 실질적 가치

| 기여 항목 | 설명 |
|----------|------|
| **마찰 손실 상한 추정** | $\mu_{max} \approx \Lambda \approx 0.05$–$0.10$ → 최대 마찰 토크 예측 가능 |
| **발열량 예측** | 트랙션 정확 예측 → 열 발생량 정량화 → 온도 분포 해석 입력 |
| **윤활유 선정 기준** | 기유별 $\Lambda$ 비교를 통한 최적 그리스 기유 선정 |
| **Eyring 모델 오차 보정** | 기존 Eyring 기반 해석의 마찰 과대 예측 수정 |
| **혼합윤활 전이 판단** | 정확한 EHL 막두께 → $\Lambda$ 비 산정 정확도 향상 |

---

## 5. 결론

### 5.1 타당성 판단

Bair-Winer 모델의 그리스 윤활 해석 적용은 **조건부로 타당**하다:

- **적합**: EHL 접촉부 내부의 기유 거동(트랙션, 마찰, 한계 전단응력) 해석
- **부적합**: 그리스 전체 시스템(증주제 포함)의 입구 유동, 스타베이션, 에이징 해석
- **권장 접근**: H-B(입구) + B-W 또는 수정 Carreau+LSS(접촉부) + Van Zoelen(스타베이션)의 **하이브리드 모델**

### 5.2 연구 공백 및 기여 가능성

- 풍력 터빈 메인베어링 그리스 윤활에 한계 전단응력 기반 모델을 체계적으로 적용한 연구는 **공개 문헌에 부재**
- 20MW+ 급의 극저속/극고하중 조건은 기존 EHL 검증 범위를 벗어나며, B-W 모델의 이 영역 적용은 **학술적 기여 가치가 높음**
- 다만 파라미터 측정(고압 유변학)의 실험적 뒷받침이 필수적

### 5.3 기존 분석과의 연계

기존 문서([그리스_윤활_분석_1.md](그리스_윤활_분석_1.md))에서 지적한 핵심 결론 — "EHL 이론만으로는 충분한 설계 근거를 제공하지 못하며, EP/AW 첨가제 경계 윤활막이 핵심" — 은 B-W 모델을 적용하더라도 변하지 않는다. B-W 모델은 **EHL 해석의 정확도를 향상**시키지만, 혼합/경계 윤활이 불가피한 본 시스템에서 EHL 막두께 자체의 한계를 극복하지는 못한다.

B-W 모델의 실질적 가치는 **마찰/트랙션의 정량적 예측**에 있으며, 이는 발열량 계산, 윤활유 선정, 혼합윤활 전이 판단에 직접적으로 기여한다.

---

## 참고문헌

### Bair-Winer 모델 원저

1. Bair, S. & Winer, W.O. (1979), "A Rheological Model for Elastohydrodynamic Contacts Based on Primary Laboratory Data," ASME J. Lubr. Technol., 101, pp. 258-265.
2. Bair, S. & Winer, W.O. (1979), "Shear Strength Measurements of Lubricants at High Pressure," J. Lubr. Technol., 101, pp. 251-257.
3. Bair, S. & Winer, W.O. (1992), "The High Pressure High Shear Stress Rheology of Liquid Lubricants," ASME J. Tribol., 114(1), pp. 1-9.
4. Bair, S. (2019), *High Pressure Rheology for Quantitative Elastohydrodynamics*, 2nd Ed., Elsevier.

### B-W 적용 EHL 해석

5. Khonsari, M.M. 등 (1994), "Thermal EHL Analysis Using a Bair-Winer Constitutive Equation," ASME J. Tribol., 116(1), pp. 37-46.
6. Habchi, W. 등 (2014), "EHL Model With Limiting Shear Stress and Realistic Shear-Thinning," ASME J. Tribol., 136(2), 021503.
7. Bair, S. & Habchi, W. (2016), "Quantitative EHL Film Forming for a Gear Oil with Complex Shear-Thinning," Proc. IMechE Part J, 230(5).
8. Bair, S. & Habchi, W. (2024), "Shear Dependent Viscosity Effects on Analytical EHL Film Thickness," Proc. IMechE Part J.
9. Bair, S. (2025), "The High-Pressure Rheology of Gear Oils," Proc. IMechE Part J.

### 그리스 EHL

10. Kauzlarich, J.J. & Greenwood, J.A. (1972), "EHL with Herschel-Bulkley Model Greases," ASLE Trans., 15(4), pp. 269-277.
11. Yoo, J.G. 등 (1997), "Numerical Analysis of Grease Thermal EHL Problems Using the Herschel-Bulkley Model," Tribology Trans., 40(2).
12. Cann, P.M. (1999), "Starved Grease Lubrication of Rolling Contacts," Lubrication Science, 11(3).
13. Lugt, P.M. (2013), *Grease Lubrication in Rolling Bearings*, John Wiley & Sons.

### 풍력 터빈 메인베어링

14. Hart, E. 등 (2022), "Wind Turbine Main-Bearing Lubrication – Part 1: An Introductory Review of EHL Theory," Wind Energy Science, 7, pp. 1021-1042.
15. Hart, E. 등 (2022), "Wind Turbine Main-Bearing Lubrication – Part 2: Simulation-Based Results for a 1.5 MW Turbine," Wind Energy Science, 7, pp. 1533-1550.

### 한계 전단응력 관련

16. Lohner, T. 등 (2016), "On the Limiting Shear Stress Concept," Tribology Letters, 64(3).
17. Lohner, T. 등 (2024), "Approach to Determine Limiting Shear Stress at High Pressures," Lubricants, 12(4), 128.

---

**문서 작성일**: 2026-03-10
**관련 문서**: [그리스_윤활_분석_1.md](그리스_윤활_분석_1.md) (20MW+ TRB EHL 해석 기초 분석)
