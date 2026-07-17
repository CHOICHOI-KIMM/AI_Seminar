# GBLM 비판적 검토 — 의문사항 해소 로그

> **목적**: [[논문분석_GBLM_참고문헌]]에 정리한 GBLM 체계에 대해 **비판적·교차적 검토**로 의문을 하나씩 해소한다.
> **방법 규약**: 모든 답변은 **P1~P4 원문 본문을 직접 확인·verbatim 인용**하여 근거화한다(추측 금지). 인용은 `(논문 §/식, L=MD줄번호)` 형식.
> **1차 출처(P1~P4)**:
> - **P1** = 2011 Micropitting (`Test_pipeline. 2011. (SKF) Micropitting.md`)
> - **P2** = 2015 GBLM 원형 (`2015. (SKF) A Model for Rolling Bearing Life…_1745a3.md`)
> - **P3** = 2019 하이브리드 (`2019. (SKF) A model for hybrid bearing life…survival.md`)
> - **P4** = 2023 응력기반·기어 (`2023. (SKF) A stress-based model…surfac_f4e12a.md`)
> **작성 기준일**: 2026-07-15 · **상태**: 진행 중(Q1·Q2 해소)

---

## Q1. 아표면(부표면) 적분은 어떤 "잘 확립된 방법"으로 푸는가? — 그리고 "정확한 부표면 적분에 Micro-EHL 거칠기 결과가 필요하다"는 통념의 교차검토

### Q1-A. 논문이 말하는 부표면항 풀이법 (직접 인용)

부표면 손상적분(§0 G3):
$$\int_{V_v}G_v\,dV_v=\bar A\,N^e\int_{V_v}\frac{\langle\sigma_v-\sigma_{u,v}\rangle^{c}}{z^{h}}dV_v$$

**P3(2019) §2.2, L203** — 부표면항은 확립된 RCF법으로 해결된다고 명시:
> "The subsurface term of Eq. (8), (represented by the volume integral), **can be solved using established Rolling Contact Fatigue methods, see reference [35]**. However, the surface term, given by the area integral of Eq. (8), **must be quantified in a radical diferent manner**."

여기서 **[35] = Ioannides & Harris 1985**(참고문헌 [1], §C2 정본). 즉 부표면항은 **Lundberg-Palmgren → Ioannides-Harris 계열의 고전 RCF 적분**으로 푼다. 입력 응력 $\sigma_v$는 **매끈(smooth) Hertz 응력장**에서 나온다.

**P2(2015) "Surface and Subsurface Survival", L121** — 부표면 응력이 거칠기와 **무관**하다고 규정:
> "surface phenomena such as surface distress, wear, indentations, frictional heating, etc., in general affect the fatigue of a **very thin material layer** … For instance, surface traction and **roughness-induced surface stresses will not, in general, affect the subsurface smooth Hertzian stress** or the amplitude of the fatigue stress criterion of the rolling contact (Ioannides, et al. (12))."

**P4(2023) §4 Solution scheme, L191** — 부표면 응력이력은 매끈 접촉솔버로 산출:
> "for each time step the contact problem is solved using a contact solver. Here for the sake of simplicity elastic semiinfinite bodies **either a dry contact solver or a Hertzian calculation is proposed**, but an EHL solver can equally be used to calculate the pressures."

**P4(2023) §5.2, L373** — 거칠기는 부표면이 아니라 **표면적분에서 별도 처리**:
> "for this case the pressure fields are "smooth", which is normal for subsurface stresses in a homogeneous material, since the surface stresses (e.g. roughness, friction, etc) are **treated separately in the surface integral**."

> **소결(Q1-A)**: 부표면항은 세 논문 모두 **매끈 Hertz(또는 매끈 접촉) 응력장 + 고전 IH적분**으로 푼다. 2015/2019는 평균근사(P2 식[22]), 2023은 시간가변 응력이력 직접적분(P4 식(1)(9))이라는 차이만 있을 뿐, **거칠기·Micro-EHL은 부표면 입력에 들어가지 않는다.**

### Q1-B. "정확한 부표면 적분에 Micro-EHL 결과가 필요하다" — 비판적/교차적 검토

**결론: 통념은 GBLM의 핵심 가정과 정면 배치되며, 부분적으로만 옳다.** 오해의 근원은 **표면항과 부표면항의 혼동**이다.

1. **모델은 표면/부표면을 의도적으로 분리(decoupling)한다.** Micro-EHL(P1 2011 모델)이 필요한 것은 **표면 손상적분 $I_s$의 $\sigma_s$**(P3 §2.3, L223: "advanced surface distress modelling … i.e. micro-EHL")이지, **부표면 $\sigma_v$가 아니다.** 질문자가 "부표면에 micro-EHL이 필요"라 본 것은 **표면항의 요구조건을 부표면항에 투영한 카테고리 혼동**이다.

2. **분리의 근거는 "깊이 스케일" 논증(가정)이다.** 거칠기 유발 응력은 조도 깊이수준의 **얇은층 $\hat h$**(P2 Fig.1)에 갇히고, 부표면 최대전단 구역(z≈0.5b, Hertz)에는 도달하지 않는다고 **가정**한다(P2 L121, ref (12) Ioannides et al.). → 이는 **유도된 사실이 아니라 모델링 가정**이다.

3. **가정의 타당성은 λ(막두께/조도)에 의존한다 — 여기가 취약점.**
   - 양윤활(λ↑, full-film)에서는 거칠기 압력리플이 작고 얕게 갇혀 분리가 잘 성립.
   - **경계윤활/박막(λ↓)** 에서는 micro-EHL 압력리플·asperity 접촉이 국부 응력집중을 **더 깊이** 침투시켜, "표면층에만 갇힌다"는 전제가 약화됨. 실제로 이 영역이 **표면피로가 지배**하는 조건(P3 Chiu 시험 η_env=0.035, 표면지배)이므로, 분리 가정이 가장 의심스러운 구간과 표면항이 중요한 구간이 겹친다.
   - 단, GBLM은 이 침투분을 **부표면이 아니라 표면적분 $I_s$가 흡수**하도록 설계했다(2011 micro-EHL이 표면 근방 전응력텐서를 Dang Van으로 평가). 따라서 "부정확"이라기보다 **회계 분담을 표면항에 몰아준 것**이며, 경계는 $\hat h$의 실제값(§6-Q3 미결)에 달려 있다.

4. **효율성 관점의 교차논증**: 만약 부표면조차 micro-EHL 거칠기 해가 필요하다면, GBLM의 실용성(부표면=값싼 고전 RCF, 표면=비싼 micro-EHL을 사전 곡선적합 $I_s$로 축약)이라는 **설계 근거 자체가 붕괴**한다. 세 논문이 부표면을 매끈장으로 유지하는 것은 **의도된 계산전략**이다.

> **판정(Q1-B)**: 질문자의 직관(거칠기가 근표면 응력을 교란한다)은 **물리적으로 옳으나**, GBLM에서 그 효과는 **부표면항이 아니라 표면항 $I_s$의 몫**이다. 부표면 적분은 매끈 Hertz장 + IH법으로 풀며 **micro-EHL 불요**. 다만 표면/부표면 분리는 **λ 의존적 가정**이고, 경계윤활에서 분리 경계($\hat h$)의 물리적 타당성은 논문이 정량 입증하지 않은 **잔여 취약점**이다(→ §의문큐 Q1-r1).

---

## Q2. A·B 상수가 시험으로 교정된다면 "시험 예측"은 당연 — 독립 검증 부재 문제 + 간이식 상수 조사

### Q2-A. 교정→검증 순환성 비판 (직접 인용)

**P4(2023) §5.1, L268** — A, B를 시험 목표수명에 맞춰 반복 조정:
> "the subsurface life integral constant A is varied starting with an initial value to try to predict the gear population life … **The aim is to predict $L_{10}\approx25.0$ Mrevs** in order to be between 50% and 10% confidence level … **This process requires several iterations until the target life is achieved.** Once this is done, the constant A is fixed and calibrated. After this, **similar procedure is followed for the constant B.**"

**P4(2023) §6, L441** — 저자 스스로 인정한 단점:
> "A shortcoming of the current model … is the fact that **the calibration process needs two endurance tests per material, one at full-film and another at poor lubrication conditions**, so that the constants A and B can be calibrated."

**P4(2023) §6, L443**:
> "the calibration of the model begins with use of the full-film test to set the value of the constant A, then the test result of the poor lubrication conditions is used to set the value of the constant B."

→ **P4의 기어 "예측"(Fig.6)은 A(초정밀 시험)·B(연삭 시험) 두 점에 맞춘 교정이므로, 그 두 조건의 재현은 정의상 순환(circular)이다.** P4 스스로 "**concept model … for illustration purposes**"(§7 Conclusion)로 한정한다.

### Q2-A(보강). P2·P3는 $\bar A,\bar B$를 어떻게 처리하는가 (P4 사례와 동일 수준의 본문 인용)

**■ P2(2015) — 부표면 $\bar A$: 이 시험으로 교정하지 않고 기존 정격상수 채용.**
- **P2 "RESULTS AND DISCUSSION", L413**:
> "The **subsurface model used here has been compared with endurance tests in the past** (e.g., Ioannides and Harris (11); Ioannides, et al. (12)) and **will not be repeated here**."
- **P2 6309 예제, L452**:
> "the surface–subsurface life rating life model **can be set using similar constants and parameters as applied in the ISO 281 model** (ISO 281:2007 (15))."
- **P2 6309 예제, L454**:
> "Others exponents and **constants used in Eqs. [29] and [25] can be taken from Ioannides, et al. (12)**, leading to the results of Table 4."

→ 즉 $\bar A$(및 지수 $c,h,e$)는 **ISO 281 / Ioannides et al.(12)의 기존 정격상수**에서 가져오며, **P4처럼 목표수명에 반복적합하지 않는다.**

**■ P2(2015) — 표면 $\bar B$: 단일 적합이 아니라 $I_s$를 시험에서 역산하여 독립 물리곡선과 대조.**
- **P2 식[23] 정의, L308**:
> "$I_s$ represents the **unknown surface damage integral**, which includes a constant layer thickness $\hat h$ **in the constant B**."
- **P2 방법론, L415**:
> "This equation can be used to **back-calculate the surface damage parameter $R_s=u^e I_s/[Kln(1/0.9)]$** corresponding to 90% reliability of a bearing populations that are endurance tested."
- **P2 방법론 1단계, L419**:
> "1. **Solve Eq. [29] for $I_s$** with known operating conditions and $L_{10}$ lives from endurance tests."
- **P2 결과, L444**:
> "almost all test results are positioned **below the limit curves obtained from Eq. [31]**. From Fig. 10 it can be concluded that the model theory provides a **safe estimation** of the raceway survival."

→ $\bar B$ 자체를 특정 시험에 맞추는 것이 아니라, **각 시험풀에서 $I_s(\to R_s)$를 역산**하고 이를 **2011 micro-EHL 물리모델에서 나온 독립 한계곡선(식[31])과 대조**한다. 이 대조가 P2의 (준독립) 검증 논리이다.

**■ P3(2019) — $\bar A,\bar B$의 수치화·교정에 대한 명시적 서술 "없음".**
- **P3 상수 정의, L177 / L185**: (정의만 제시)
> L177: "A is the **bearing subsurface constant** for Rolling Contact Fatigue." · L185: "B is the **surface RCF constant**, and m is the characteristic slope of the Weibull statistics."
- **P3 부표면 풀이, L203**:
> "The subsurface term of Eq. (8) … **can be solved using established Rolling Contact Fatigue methods, see reference [35]**." (→ 표준 정격, $\bar A$ 암묵)
- **P3 표면지수 상수, L270**:
> "the parameters $c_1;c_2;c_3;c_4;c_5$ are determined numerically as data sets … **Their values are stored in form of data blocks in the calculation program.**" (→ $\bar B$ 스케일·$c$상수 비공개)
- **P3 적용, L303**:
> "The running conditions of Chiu's tests … were introduced into an **ad hoc bearing life code** with the implementation of the Generalized Bearing Life Model according to Eq. (12)."

→ **P3 본문에는 $\bar A,\bar B$를 어떻게 수치화/교정하는지에 대한 서술이 없다.** 부표면은 기존 RCF법(표준정격)으로, 표면은 사전계산 데이터블록($c_1$~$c_5$)으로 대체한다. **시험별로 조정되는 것은 $\bar A,\bar B$가 아니라 환경계수 $\eta_{env}$**이다:
> **P3 L299**: "The resulting GBLM environmental factor $\eta_{env}$ … is **0.035** This was obtained with an estimated film thickness to composite roughness ratio of the test 0.675 corresponding to a **lubrication factor … 0.175 and ISO 281 factor for contamination, 0.2**."

> **소결(Q2-A 보강)**: **A·B의 명시적 "시험 교정" 서술은 P4에만 존재**한다. **P2**는 $\bar A$를 기존 정격상수로 고정하고 $\bar B$(표면)를 시험 역산 $R_s$ ↔ 물리모델 식[31] **대조**로 검증한다(적합 아님). **P3**는 $\bar A,\bar B$의 수치화·교정을 **아예 서술하지 않고**, 부표면=established 법·표면=data block으로 처리하며 시험별 조정 변수는 $\eta_{env}$이다. → 따라서 "A,B를 시험에 맞췄으니 예측은 당연"이라는 순환성 비판은 **P4에 정확히 적용**되나, **P2/P3에는 그대로 적용되지 않는다**(A,B가 시험적합 산물이 아니므로).

**교차 비교(상수 출처·검증 독립성):**

| 논문 | 상수 출처 | 검증 독립성 | 판정 |
|---|---|---|---|
| **P4 2023** | A,B를 동일 Krantz 2시험에 반복적합 | **가장 약함**. 2점은 순환. 단 신뢰도분포(S=0.1~0.9)·오정렬·worst-case는 외삽 | 개념실증 |
| **P2 2015** | $\bar A,\bar B$는 ISO 281 설정에 정렬; 표면함수는 **2011 micro-EHL(식[31])에서 독립 산출** | **중간**. 227풀 6650베어링의 back-calc $R_s$를 식[31]과 대조 → 준독립 교차 | 부분검증 |
| **P3 2019** | c1~c5는 **micro-EHL 파라미터 스터디**(수명시험과 무관); 부표면은 established 강 RCF[35]. **$\bar A,\bar B$는 식(12)에 명시(하중기반 C형 아님)** | **가장 강함**. 509베어링·20시리즈 상관(Pearson 0.91, R²98.6%). 단 η_env는 시험별 추정 | 준독립 검증 |

**P2(2015) 검증논리(L444)** — 물리모델 곡선 vs 시험 back-calc의 독립 대조:
> "almost all test results are positioned **below the limit curves obtained from Eq. [31]**. From Fig. 10 it can be concluded that the model theory provides a **safe estimation** of the raceway survival."
식[31]은 시험수명이 아니라 **2011 micro-EHL 물리모델**에서 나오므로, 이 대조는 (A,B 스케일과 무관한) **함수 형상의 준독립 검증**이다.

**P3(2019) 상관, L368**:
> "the Pearson correlation coeficient between the GBLM predicted life and the experimental L10;50 observations is **0.91** … the coeficient of determination (R-square) of the GBLM model … is **98.6%**."

> **판정(Q2-A)**: 질문자의 지적은 **원칙적으로 타당**하고 **P4가 명시적으로 인정**한다. 단, 순환성은 **P4(기어)=거의 전면적**, **P2/P3=제한적**이다. 왜냐하면 **A,B(또는 C)는 "스케일"만 정하고**, 하중·윤활(κ/λ)·Weibull 기울기의 **"형상(shape)"은 micro-EHL 물리와 표준정격이 구속**하므로, 다수 시험군의 형상을 재현하는 것은 자명하지 않은 검증부담이기 때문이다. **엄밀 검증에는 교정에 쓰지 않은 독립 시험셋이 필요**하며, 그에 가장 근접한 것이 **P3의 509베어링/20시리즈 상관**이다. 반면 **P4 기어는 독립 검증이 사실상 부재**(개념실증). **공통 약점**: A,B가 재료·윤활·표면마감 변경 시 **재교정 필요**(P4 §6, L439: "the need to recalibrate these constants could become apparent"). (→ §의문큐 Q2-r1: P3 η_env 시험별 추정의 자유도 점검.)

### Q2-B. 간이식 2형태의 상수 — 논문 제시 여부 조사

표면손상적분 $I_s$의 간이 곡선적합은 **2세대**로 진화(§C3):

#### (i) 5상수 지수형 — **수치 비공개**
- **P2(2015) 식[31]**: $u^{m}I_s\approx f_1\exp\!\big[\frac{f_2}{(P/P_u)^{f_3}}+\frac{f_4}{(P/P_u)^{f_5}}\big]$
  - **P2 L389**: "$f_1,f_2,f_3,$ and $f_4$ are **curve-fitted constants that depend on the surface stress conditions** (e.g., lubrication, contamination, etc.)." → **수치표 없음**. Fig.7의 $R_s$ 곡선만 제시.
  - **P2 L391**: "Eq. [31] can be solved for $f_s$ using some calculated points … with the use of a **collocation algorithm**" — 즉 사용자가 micro-EHL 점들로 직접 적합해야 함.
- **P3(2019) 식(11)**: $I_s^*\approx c_1\exp\!\big[\frac{c_2}{(P/P_u)^{c_3}}+\frac{c_4}{(P/P_u)^{c_5}}\big]$
  - **P3 L270**: "the parameters $c_1,c_2,c_3,c_4,c_5$ are determined numerically as data sets … **Their values are stored in form of data blocks in the calculation program.**" → **명시적으로 비공개**(프로그램 내부 데이터블록).
- (참고) **P2 식[30]** $N_{1.5\%}$의 $c_1$~$c_4$도 "depend on … k and … $d_m$"로만 서술, **수치 없음**(P2 L370).

#### (ii) 3상수 tanh형 — **P4 Table A1에 수치 공개(유일하게 재현 가능)**
- **P4(2023) 식(6)/부록A 식(A2)**: $\widetilde I_s=\dfrac{a_1}{\Lambda^{1.3}}\tanh\!\Big(\big[\frac{p_H}{p_{Hu}}\big]^{1/3}\frac{b_1}{\Lambda^{1.3}}\Big)-\dfrac{c_1}{50}\big[\frac{p_H}{p_{Hu}}\big]^{1/3}$, 여기서 $\widetilde I_s=I_s/\bar B$, $W/W_u=(p_H/p_{Hu})^{1/3}$ (식(A3)).
- **P4 Table A1 (L361–367) — 13개 Λ점 verbatim**:

| Λ | 0.17 | 0.23 | 0.3 | 0.4 | 0.5 | 0.59 | 0.67 | 0.84 | 1 | 1.37 | 1.7 | 2.33 | 2.9 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **a₁** | 1.3 | 1.7 | 1.9 | 2 | 2 | 2 | 2 | 1.9 | 1.9 | 1.7 | 1.5 | 0.7 | 0.2 |
| **b₁** | 0.04 | 0.1 | 0.2 | 0.5 | 0.8 | 1.5 | 2.5 | 3.5 | 5 | 10 | 25 | 30 | 40 |
| **c₁** | 4.7 | 3.2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

  - 중간값은 보간(P4 L487: "For intermediate values, interpolation can be applied").
  - 피로한계 기준압 **$p_{Hu}=1.5$ GPa**(베어링강, 기어강 유사 가정; P4 부록A L497).

#### (iii) 함께 필요한 스케일·지수 상수 (P4 기어 예제, Table 3, L285–291)
| 상수 | 값 | 근거 |
|---|---|---|
| $\tau_u$ | 360 MPa | 베어링강 [26] 가정 |
| $c$ | 31/3 | as [4] |
| $h$ | 7/3 | as [4] |
| $e$ | 1.2 | 시험 $L_{10,50}$ (Table 2) |
| $\bar A$ | 0.10295×10¹¹ | **교정값**(full-film) |
| $\bar B$ | 0.1169 | **교정값**(poor-lub) |

> **소결(Q2-B)**: **5상수형(P2 식[31]·P3 식(11))의 상수는 어느 논문도 수치 미공개**(P3은 "data blocks in the calculation program"으로 명시 비공개). **3상수형(P4 식(6))만 Table A1로 a₁,b₁,c₁ 13점을 공개** → **재현 가능한 유일한 표면 간이식**. 단 $\bar A,\bar B$ 스케일과 $p_{Hu}$·$\tau_u$는 여전히 재료별 교정/가정이 필요하다(Q2-A와 연결).

---

## Q3. P2의 정규화 표면손상함수 $R_s$ — 상수 $\bar B$는 어디로 갔나 & $R_s$는 시험이 필요한가, 수치해석만으로 되는가

### Q3-A. $\bar B$는 어떻게 처리되는가 — "$I_s$ 안으로 흡수"

**$\bar B$는 사라진 것이 아니라 $I_s$ 정의 안에 접혀 들어가 있다.** P2 식[23]:
$$\hat h\int_A G_s\,dA=\bar B\,N^{m}\int_A\langle\sigma_s-\sigma_u\rangle^{c}dA=N^{m}I_s$$

- **P2 식[23] 직후, L308** — $\bar B$가 $I_s$에 포함됨을 명시:
> "where $I_s$ represents the **unknown surface damage integral, which includes a constant layer thickness $\hat h$ in the constant B**."

즉 정의상 $I_s=\bar B\int_A\langle\sigma_s-\sigma_u\rangle^{c}dA$ 이며, **표면층 두께 $\hat h$까지 $\bar B$에 흡수**된다. 따라서 5상수 지수식(식[31])이 **$u^m I_s$**를 적합하므로, $\bar B$는 이미 $I_s$ 안에 있어 **전지수 상수 $f_1$에 흡수**된다:
$$u^{m}I_s\approx f_1\exp\!\Big[\tfrac{f_2}{(P/P_u)^{f_3}}+\tfrac{f_4}{(P/P_u)^{f_5}}\Big]\quad(\text{식[31]})$$
→ **$\bar B$가 식[31]에 따로 안 보이는 이유**: 좌변 $u^m I_s$의 $I_s$가 이미 $\bar B$를 품고 있어, 적합상수 $f_1$이 $\bar B$(및 적분 스케일)를 대신 담기 때문이다. **별도 항으로 등장할 필요가 없다.**

**$R_s$는 다시 상수로 정규화**되어 절대 스케일이 제거된다.
- **P2 Nomenclature, L37 / Fig.7 caption, L406**:
> "$R_s$ Normalized surface damage function, $R_s=I_s\,u^{e}L^{e}_{10.BR}/[K\,Ln(1/0.9)]$"
- **P2 L391** — Fig.7의 $R_s$는 "상수로 정규화":
> "An example of the obtained surface fatigue function … is shown in Fig. 7 ($R_s$) **normalized with respect to a constant**."

$K$는 "Constant for the surface damage function"(P2 Nomenclature, L54). $I_s$가 $\bar B$를 품고, $R_s$는 그 $I_s$를 상수 $K$로 나눠 **무차원 정규화**한 양이다. → **절대 스케일($\bar B$/$K$)은 정규화로 상쇄**되고, $R_s$는 하중($P/P_u$)·윤활(κ) 의존 "형상"만 남는다. (이 덕분에 모델과 시험을 같은 정규화 축에서 대조 가능.)

> ⚠️ **표기 불일치(원문 내부)**: $R_s$ 정의가 두 형태로 나온다 — Nomenclature·Fig.7(L37/L406)은 $R_s=I_s u^e L^e_{10.BR}/[KLn(1/0.9)]$, back-calc 본문(L415)은 $R_s=u^e I_s/[Kln(1/0.9)]$ (즉 $L^e_{10.BR}$ 인자 유무). OCR 오독 가능성 포함해 §의문큐 Q3-r1로 이월.

### Q3-B. $R_s$를 구하는 데 시험이 필요한가? — **두 경로 존재, 예측용은 수치해석만으로 충분**

P2는 $R_s$를 **서로 독립인 두 경로**로 얻고, 이 둘을 Fig.10에서 대조한다.

**① 수치해석(모델) 경로 — 시험 불요.** 식[31]을 2011 micro-EHL 표면손상 모델의 계산점으로 적합:
- **P2 L391**:
> "Eq. [31] can be solved for $f_s$ using some **calculated points of $u^m I_s$ obtained with the advanced surface distress model** with the use of a collocation algorithm by fixing a number of locations in the abscissa $P/P_u$."
- **P2 L442** — Fig.10의 **실선(solid lines)** 이 이 경로:
> "plot the results of the **surface distress model (solid lines) from Eq. [31]**. The dependence on η of this equation is **implicit in the surface topographies and operating conditions** used to obtain this equation."

→ 즉 **예측용 $R_s$ 곡선은 표면형상(조도)+운전조건을 micro-EHL에 넣어 계산한 수치결과만으로 얻는다. 시험 결과 불필요.**

**② 시험 back-calc 경로 — 시험 필요.** 측정 $L_{10}$으로 식[29]를 $I_s$에 대해 역산:
- **P2 L415**:
> "This equation can be used to **back-calculate the surface damage parameter $R_s=u^e I_s/[Kln(1/0.9)]$** corresponding to 90% reliability of a **bearing populations that are endurance tested**."
- **P2 방법론, L419**:
> "1. **Solve Eq. [29] for $I_s$** with known operating conditions and **$L_{10}$ lives from endurance tests**."
- **P2 L444** — Fig.10의 **점(dots)** 이 이 경로:
> "the surface damage parameter $R$ **back-calculated from a large set of endurance tests results (dots)** is displayed **alongside** the surface damage parameter obtained from the surface distress model presented in Eq. [31] **(lines)**."

→ 이 경로는 **엔듀런스 시험의 $L_{10}$이 있어야** 한다.

> **소결(Q3)**: **(B의 처리)** $\bar B$는 정의상 $I_s$ 안에 흡수(식[23] L308)되고, $\hat h$까지 포함한다. 식[31]은 $u^m I_s$를 적합하므로 $\bar B$는 적합상수 $f_1$에 들어가 **별도로 나타나지 않으며**, $R_s$는 상수 $K$로 정규화되어 절대 스케일이 상쇄된다. **($R_s$ 산출)** **예측용 $R_s$는 micro-EHL 수치해석만으로 충분**(식[31], Fig.10 실선; 시험 불요). 시험 $L_{10}$은 **검증용 back-calc $R_s$**(식[29], Fig.10 점)에만 필요하다. → **이는 Q2-A 결론과 정합**: P2의 표면 예측함수는 시험적합 산물이 아니므로(수치해석 기반), 표면항에 대한 순환성 비판은 성립하지 않는다.

### Q3-C. 종합: L10 계산에 수치해석·시험·상수 A·B가 어떻게 필요한가 — P2·P3·P4 엄격 대조

**전제 교정 1 — P2 식[29](하중기반)는 $u^m I_s$만으로 풀리지 않는다.**
- **P2 식[29], L343**: $L_{10}=\dfrac{a_u\,(C/P)^{p}}{\big[1+\frac{u^m L_{10}^{(m-e)}I_s}{\ln(1/0.9)}(C/P)^{ep}a_u^{e}\big]^{1/e}}$
- **P2 L334**: 대괄호(=$I_s$ 없는 항)는 "**the dynamic capacity of the bearing to the power pe**" → **$C$에 부표면 $\bar A$가 내재**(식[17]~[21]).
→ 이 하중기반 형태에선 부표면($C,a_u\!\leftarrow\!\bar A$) + 표면($I_s\!\leftarrow\!\bar B$) 둘 다 필요. $u^m I_s$만으로는 부족.

**전제 교정 2 (핵심 — 사용자 지적 반영) — P3는 하중기반 형태가 아니라, $\bar A,\bar B$가 명시적으로 살아있는 응력적분 형태를 쓴다.**
- **P3 식(9), L212**: $L_{10}=\Big[\dfrac{\bar A u^e}{\ln(1/0.9)}\!\int_{V_v}\!\dfrac{\langle\sigma_v-\sigma_{u,v}\rangle^c}{z^h}dV_v+\dfrac{\bar B u^e}{\ln(1/0.9)}\!\int_A\!\langle\sigma_s-\sigma_{u,s}\rangle^c dA\Big]^{-1/e}$ — $\bar A,\bar B$가 **계수로 명시**.
- **P3 식(10), L218** + 도입문 **L215 "excluding constant terms"**: $I_s^*=\int_A\langle\sigma_s-\sigma_{u,s}\rangle^c dA$ — **$\bar B$를 뺀** 순수 표면적분.
- **P3 식(11)**: $I_s^*\approx c_1\exp[\cdots]$ — **$\bar B$-free**. → 그러므로 **L270의 "data blocks"($c_1$~$c_5$)는 $\bar B$ 없는 $I_s^*$를 근사**한 것이며, $\bar B$를 포함하지 않는다.
- **P3 식(12), L283**: $L_{10}=\dfrac{[\ln(1/0.9)]^{1/e}}{u}\Big[\bar A\!\int_{V_v}\!\dfrac{\langle\sigma_v-\sigma_{u,v}\rangle^c}{z^h}dV_v+\bar B\,c_1\exp\!\big[\tfrac{c_2}{(P/P_u)^{c3}}+\tfrac{c_4}{(P/P_u)^{c5}}\big]\Big]^{-1/e}$ — **$\bar A,\bar B$가 그대로 노출**.

→ 즉 **P3는 P2의 [17]~[29](동적정격 $C$ 하중기반 변환)를 재현하지 않았고**, 구조적으로 **P4(식(7))와 동형**($\bar A,\bar B$ 명시)이다. (⚠️ 본 Q3-C 초판이 "P3=P2형($\bar A$를 $C$에 은닉)·표준 물려받음"이라 한 것은 **오류였고 여기서 교정**한다.)

**사용자 이분법의 엄격 판정:**

| 가설 | 판정 | 근거 |
|---|---|---|
| **(i)** P3가 [17]~[29]를 단순 생략(→ $C$ 암묵 사용) | **부정확** | P3는 변환을 안 했을 뿐 아니라 **$C$ 형태로 대체한 게 아니라 응력적분 형태(식12)로 $\bar A,\bar B$를 명시 유지**. "$C$ 은닉"은 P2에만 해당. |
| **(ii)** 세라믹은 표준규격 불가 → P4처럼 A·B 살림(상세 생략) | **구조는 맞음(2 유보)** | 식(12)에 $\bar A,\bar B$ 명시 = P4형 ✅. ⓐ 그러나 **부표면은 표준 강 RCF를 실제 사용**(P3 L203 "established RCF methods [35]"; 궤도가 강이라 압력만 12%↑) → "세라믹이라 부표면 표준 불가"는 아님. ⓑ **P3는 P4와 달리 $\bar A,\bar B$를 시험 교정한다고 서술하지 않음**(교정 절차 부재). |

**세 논문의 실제 위치(엄격):**
- **하중기반 형태($\bar A\to C$)**: **P2만** 사용(6309 예제, 표준 강 베어링, ISO 281/Ioannides 상수).
- **응력적분 형태($\bar A,\bar B$ 명시)**: **P3·P4 공통**. — 차이는 $\bar A,\bar B$ **확정 방법**:
  - **P3(하이브리드)**: 부표면 $\bar A$ = established 강 RCF [35](표준 강 궤도, 압력만↑). 표면 = micro-EHL로 $I_s^*$(=$c_1$~$c_5$, **$\bar B$ 제외**) 산출, $\bar B$는 식(12)에 명시. **$\bar A,\bar B$ 절대값은 프로그램 내**(L270 data blocks·L303 "ad hoc bearing life code") — **시험 교정 서술 없음**.
  - **P4(기어)**: $\bar A,\bar B$를 **2회 내구시험으로 교정**(L268), "constants already consider this material"(L270).

**P3가 $\bar B$를 명시 유지한 "이유"(가설 ii의 핵심 부분 지지):** 하이브리드 세라믹-강 표면 tribology(낮은 경계마찰→낮은 $\sigma_s$)를 **표준의 단일 de-rating으로 표현할 수 없다**(P3 서론: 전통 RCF가 12%↑압력으로 하이브리드를 부당 벌점) → 표면항을 **명시(식12에 $\bar B$ 살림)** 하여 하이브리드 전용 $I_s^*$를 삽입. 단 이는 **표면항**에 국한되고, 부표면은 강 궤도라 표준 RCF를 그대로 쓴다.

**■ 종합표 (재작성) — 형태·상수확정·필요입력**

| 항목 | P2 (2015, 강 베어링) | P3 (2019, 하이브리드) | P4 (2023, 기어) |
|---|---|---|---|
| 수명식 형태 | 하중기반 식[29]($\bar A\to C$)＋응력적분 식[16] | **응력적분 식(12)($\bar A,\bar B$ 명시)** | 응력기반 식(7)($\bar A,\bar B$ 명시) |
| [17]~[29] 변환 | 수행 | **미수행** | 미수행 |
| 부표면 $\bar A$ | 표준 $C$에 내재 | established 강 RCF [35](표준강) | **full-film 시험 교정** |
| 표면 $\bar B$ | $I_s$에 흡수(식[31]) | **식(12)에 명시**, 값은 프로그램 내 | **poor-lub 시험 교정** |
| 표면형상 산출 | micro-EHL 식[31] | micro-EHL $I_s^*$ 식(11) $c_1$~$c_5$ | micro-EHL tanh 식(6) $a_1,b_1,c_1$ |
| L10에 새 시험 | 불요(표준) | **서술 없음**(교정 절차 부재) | 필요(2시험) |

> **판정(Q3-C 재작성)**: **P3는 P2의 하중기반 형태를 쓰지 않는다.** 사용자 지적대로 P3의 "data blocks"($c_1$~$c_5$)는 **$\bar B$를 제외한 $I_s^*$**(식(10) "excluding constant terms")만 담고, **$\bar B$는 식(12)에 명시적으로 살아있다** — 구조상 **P4와 동형**이다. 따라서 이분법 중 **(i) "단순 [17]~[29] 생략(→$C$)" 은 부정확**(생략했으나 $C$ 대체가 아니라 응력적분형 유지), **(ii) "P4처럼 A·B 살림"이 구조적으로 옳다** — 단 P3는 P4와 달리 **$\bar A,\bar B$ 시험 교정을 서술하지 않고** 부표면은 표준 강 RCF([35])를 쓴다. 정리: **P3는 제3의 위치** — *구조=P4($\bar A,\bar B$ 명시), 상수확정=미서술(established 표준·프로그램 내, 시험교정 부재)*. $\bar B$를 명시 유지한 것은 하이브리드 **표면** tribology를 표준 de-rating으로 담을 수 없기 때문(가설 ii 부분 지지). **미해소**: P3가 하이브리드용 $\bar A,\bar B$ 절대값을 어디서(2015 강 프레임 상속? 별도 교정?) 얻는지는 **본문에 명시 없음** → 의문큐 Q3-r2.
>
> **수치해석 vs 시험 역할(공통 결론)**: **수치해석(micro-EHL)** 은 세 논문 모두 **응력장과 표면함수 "형상"($I_s^*$/$u^m I_s$)** 을 제공한다. **절대 스케일 $\bar A,\bar B$** 는 — P2=표준정격($C$)에서 상속, P3=established 표준+프로그램 내(출처 미서술), P4=2회 시험 교정 — 으로 확정된다. 질문자의 "$u^m I_s$만으로 계산 가능"은 **표면함수 형상 산출에 한해** 옳고, **완전한 $L_{10}$**(부표면 $\bar A$ 포함)에는 성립하지 않는다.

### Q3-C(추가). P2 표면항의 정당성 3검토 — $\bar B$ 흡수·수치 전적 의존·$K$ 미제시

**① $\bar B$를 $u^m I_s$에 흡수하는 것은 타당한가? — 수학적으로 OK, 그러나 $\bar B$가 독립성을 잃는다.**
- 흡수 자체는 상수를 적분 밖으로 묶는 정당한 재정의($I_s\equiv\bar B\int_A\langle\sigma_s-\sigma_u\rangle^c dA$, $\hat h$도 $\bar B$에 포함; 식[23] L308).
- 문제: $u^m I_s$를 통째로 micro-EHL로 산출하면 **$\bar B$(표면 RCF 재료상수)가 독립적으로 관측·교정되지 않고 micro-EHL 손상모델의 내부기준이 대신 정한다.** 구체적으로 $u^m I_s$는 식[30]의 $N_{1.5\%}$(=micro-EHL이 예측한 1.5% 표면손상 도달 사이클)에서 유도되고, $N_{1.5\%}$는 **Dang Van 피로기준 + "1.5% 손상면적" 정의**에 의존한다(P2 §Advanced Surface Distress Model). → **$\bar B$ 절대 스케일 = micro-EHL 손상모델 캘리브레이션에 종속**. 편리하나 표면 정확도가 전적으로 2011 모델 충실도에 묶이는 **잠재적 순환**.

**② 표면항을 전적으로 수치해석에 의존하고 교정상수를 두지 않는 것은 충분한가? — 2011 모델의 기존검증을 "상속".**
- P2는 별도 표면수명 교정 없이 식[30]→[31]로 표면항 산출. 정당성 근거 = **2011 모델이 이미 독립 검증되었다는 전제**:
  > **P2 §Advanced Surface Distress Model, L352**: "this model **has been validated with experiments** and has been used with success in the description of indentation failures in rolling contacts (Morales-Espejel and Gabelli (19))."
- 즉 P2는 표면모델 검증을 **상속**하며 재수행하지 않는다. 이는 그 검증이 대상 영역(하중·윤활 범위)을 실제로 커버할 때만 충분. 절대 스케일이 베어링 표면수명과 맞는지는 별도 확인 필요 → Fig 10.

**③ Fig 10의 곡선-점 일치로 충분히 논증되는가? — $K$ 미제시가 결정적 한계.**
- **정밀 일치가 아니라 보수적 상한**임에 유의:
  > **P2 L444**: "The curves of the surface distress model **represent the limit conditions** … almost all test results are **positioned below the limit curves** … the model theory provides a **safe estimation** of the raceway survival."
  → 모델은 **보수적 상한 포락선**, 시험점은 그 아래. "형상 추종 + 안전측 bounding"이지 정확한 오버레이가 아니다.
- **$K$ 문제(사용자 지적 정확)**: 두 $R_s$는 모두 **상수 $K$로 정규화**된다 — 모델(Fig.7 "normalized with respect to a constant", L391)·시험($R_s=u^e I_s/[K\ln(1/0.9)]$, L415). 그런데 **$K$("Constant for the surface damage function", L54)의 값·결정법은 논문 어디에도 없다**(전수확인: $K$는 정의 L54·Fig.7식 L406·back-calc식 L415에만 등장, 값 부재).
  - **함의(양면)**: (a) 모델·시험이 **동일 $K$로 정규화**되면 비교에서 $K$가 상쇄되어 Fig 10은 $I_s^{model}\!\leftrightarrow\!I_s^{test}$의 **상대 크기+형상**을 유효 대조한다(→ "형상 추종"은 유효). (b) 그러나 **$K$ 값 미제시**이므로 **절대 $R_s$ 축·절대 표면수명 스케일을 재현할 수 없고**, 두 $R_s$가 정말 같은 $K$인지도 명시 확인 불가. 즉 Fig 10은 **정규화된 보수적 형상검증**이며 **절대 앵커($K\!\leftrightarrow\!\bar B$)는 문서화되지 않았다.**
  - 게다가 시험 $I_s^{test}$는 식[29]를 **표준 $C,a_u$로 역산**한 값이라, 이 검증은 "표준 부표면 상수 하에서 수치 표면함수가 측정수명을 보수적으로 bounding한다"는 **조건부** 논증이다.

> **소결(Q3-C 추가)**: **①** $\bar B$ 흡수는 수학적으로 정당하나 $\bar B$를 micro-EHL 손상기준에 종속시켜 독립성을 잃게 한다. **②** 교정상수 없는 수치 전적 의존은 **2011 모델 기존검증 상속**에 기대며, 그 자체는 P2 L352로 정당화되나 절대 스케일 일치는 미보장. **③** **Fig 10은 표면함수의 하중·윤활 "형상"을 준독립·보수적으로 검증하는 강한 증거이지만 절대 스케일 논증은 아니다** — 비교가 **미제시 상수 $K$로 정규화**된 보수적 상한 대조이고 $K(\!\leftrightarrow\!\bar B)$의 값·결정이 논문에 **없다**. 따라서 "수치=시험이니 충분"은 **형상·안전측 bounding 수준에서 참**이고 **절대 표면수명의 재현·검증은 $K$ 미제시로 닫히지 않는다**(사용자 지적 타당). → 의문큐 Q3-r3.

---

## 의문 큐 (미해소 / 후속)

| ID | 의문 | 근거·연계 | 상태 |
|---|---|---|---|
| Q1-r1 | 표면/부표면 분리 경계 $\hat h$의 물리적 타당성(경계윤활 λ↓에서 거칠기 응력의 부표면 침투) — 논문 정량 미입증 | Q1-B·참고문헌 §6-Q3 | ☐ |
| Q2-r1 | P3 η_env(0.85/0.035) 시험별 추정의 자유도 — 상관 0.91의 실제 예측력 | Q2-A·P3 §3 | ☐ |
| Q2-r2 | 5상수형↔3상수형 정합성: 동일 조건에서 식[31]/(11)과 식(6)의 $I_s$가 일치하는가(P4가 [18][19] 곡선을 tanh로 근사했다고 주장, L131) | Q2-B·참고문헌 §6-Q7 | ☐ |
| Q3-r1 | P2 $R_s$ 정의 표기 불일치: Nomenclature/Fig.7(L37/L406)의 $L^e_{10.BR}$ 인자가 back-calc식(L415)엔 부재 — OCR 오독 여부 원 PDF 대조 | Q3-A | ☐ |
| Q3-r2 | P3가 하이브리드용 $\bar A,\bar B$ **절대값**을 어디서 얻는지 본문 미서술 — 2015 강 프레임 상속인지 별도 교정인지(식(12) 명시하나 값 출처 불명) | Q3-C | ☐ |
| Q3-r3 | P2 정규화 상수 $K$의 값·결정법이 본문 부재 — Fig 10 검증은 미제시 $K$로 정규화된 보수적 상한 대조라 절대 스케일 미확정 | Q3-C(추가) | ☐ |
| Q4 | $p_{Hu}=1.5$ GPa·$\tau_u=360$ MPa의 근거([26] Gabelli 2012, 리포 **미보유**) 확인 필요 | 참고문헌 Tier B [7] | ☐ |

---

**끝 (진행 중 — 다음 의문 추가 시 갱신)**
