# Mixed/Partial EHL 출력의 공학적 활용 — 분석 레포트

> **주제:** 수치해석으로 정밀하게 푼 Mixed/Partial EHL의 출력(유막두께 $h$, 유체압 $p_h$, 돌기압 $p_a$, 돌기 하중분담비 $\zeta=W_a/W_t$)을 베어링 수명계수($a_{\text{ISO}}$) 재보정이 아니라 **어떤 공학적 목적에 어떻게 활용**해야 하는가.
> **대상:** 20MW+ 초대형 풍력 메인베어링(TRB), 저속·고하중·그리스 윤활.
> **연계:** 계획서 `분석계획_MixedEHL_공학적활용.md`, 본문 `분석_EHL 윤활해석.md`(부록1 가정·한계, 부록2 점압모델).
> **작성 규약:** 핵심 주장·수식·수치는 본문 문장 안에 출처(저자·연도·학술지)를 직접 명기. 미검증 서지는 `[VERIFY]` 표기. 로컬 2차자료는 `(로컬)` 표기.
> **작성 상태:** 집필 중 — 장별로 실시간 갱신.

---

## 1. 서론 — 문제 정의와 중심 논제

### 1.1 조도 이중 반영(double counting) 문제

MK(Maierhofer–Koch 계열, 본 보고서 M2)형 유막식처럼 **유막두께 공식에 조도 보정 $r_c(\sigma)$ 가 내장된 모델**을 쓰면서, 그 출력 유막으로 다시 막비 $\Lambda=h/\sigma$ 를 만들어 점도비 $\kappa$ 를 거쳐 ISO 281 수명보정계수 $a_{\text{ISO}}$ 에 입력하면, 표면조도가 **두 번** 반영된다.

$$
\underbrace{h_{\text{smooth}} \times r_c(\sigma)}_{\text{조도 1회: } h \text{ 감소}} \;\longrightarrow\; \Lambda=\frac{h_{MK}}{\sigma}\;\;\underbrace{(\div\,\sigma)}_{\text{조도 2회}} \;\longrightarrow\; \kappa \;\longrightarrow\; a_{\text{ISO}}
$$

이는 ISO 281의 $a_{\text{ISO}}$ 곡선이 **매끈 표면 유막 기준의 $\Lambda$** 로 calibration 되었기 때문이다. 즉 조도 효과는 이미 $\Lambda=h_{\text{smooth}}/\sigma$ 단계에서 반영되므로, $h$ 자체에 조도를 또 포함하면 이중 계산이 되어 $\kappa$ 와 수명을 과소예측(과도하게 보수적)한다. **이 판단은 타당하다.**

### 1.2 ISO 281 $a_{\text{ISO}}$의 역할 경계

ISO 281:2007의 $a_{\text{ISO}}=f(\kappa,\ C_u/P,\ \eta_c)$ 는 Lundberg & Palmgren(1947)의 아표면 개시 피로 이론과 Ioannides & Harris(1985, *ASME J. Tribology* 107(3):367–378)의 응력기반 일반화에 뿌리를 둔다. 점도비 $\kappa=\nu/\nu_1$ 의 기준점도 $\nu_1$ 은 "표면을 분리하기에 충분한 최소유막"을 기준으로 정의되므로, $\kappa$ 는 사실상 **막비 $\lambda=h_{\min}/\sigma$ 의 대리변수**다(NASA/TP, Zaretsky 2016 정리). 따라서 $a_{\text{ISO}}$ 에는 **박막·혼합윤활(돌기 상호작용)의 수명 페널티가 이미 스칼라 형태로 내장**되어 있다.

> **귀결:** 수명 계산용으로는 $a_{\text{ISO}}$(조도 통계 내장)로 충분하며, Mixed EHL의 미시 출력을 $a_{\text{ISO}}$ 재보정에 다시 쓰면 안 된다.

### 1.3 중심 논제 — 관심사의 분리(Separation of Concerns)

> $a_{\text{ISO}}$(거시 수명, 조도 통계 내장)와 Mixed EHL 미시 출력(국부 손상 물리)은 **서로 다른 질문에 답하는 도구**다. Mixed EHL 출력의 정당한 용도는 $a_{\text{ISO}}$ 재보정이 아니라:
> 1. **$a_{\text{ISO}}$ 가 원리적으로 다루지 않는 표면개시 손상 모드** — 마이크로피팅·스커핑·마모 — 의 예측,
> 2. **마찰·동력손실·발열** 예측,
> 3. **표면/아표면 피로의 명시적 분리**(SKF GBLM)에 surface 항의 입력으로 공급.

근거: Lundberg–Palmgren 기반 $a_{\text{ISO}}$ 는 아표면 개시 스폴링에 calibration 되어 **표면개시 손상을 구조적으로 포함하지 않는다**. 현대 베어링 수명모델 SKF GBLM(Morales‑Espejel, Gabelli & de Vries 2015, *Tribology Transactions* 58(5):894–906)은 바로 이 한계를 surface/subsurface 생존확률 분리로 해결하며, surface 항은 Mixed EHL 응력으로 구동된다(8·9장).

---

## 2. 이론 배경 — Mixed/Partial EHL의 지배식과 출력

### 2.1 평균유동 모델 (통계적 혼합윤활) — Patir & Cheng

유막이 조도와 비슷해지면 매끈 영역에서의 국부 Reynolds 식을 그대로 풀 수 없다. Patir & Cheng(1978, *ASME J. Lubr. Technol.* 100(1):12–17; 1979, 101(2):220–230)은 조도를 통계적으로 평균화한 **평균 Reynolds 방정식**을 도입하고, 그 계수인 **유동계수(flow factor)** 로 조도 효과를 포착했다:

$$
\frac{\partial}{\partial x}\!\left(\phi_x\frac{h^3}{12\eta}\frac{\partial \bar p}{\partial x}\right)
+\frac{\partial}{\partial y}\!\left(\phi_y\frac{h^3}{12\eta}\frac{\partial \bar p}{\partial y}\right)
=\frac{U}{2}\frac{\partial \bar h_T}{\partial x}+\frac{U}{2}\sigma\frac{\partial \phi_s}{\partial x}+\frac{\partial \bar h_T}{\partial t}
$$

- $\phi_x,\phi_y$ : **압력 유동계수**(Poiseuille 흐름 보정, 매끈 시 $\to 1$)
- $\phi_s$ : **전단 유동계수**(Couette 흐름 보정, 동일 매끈면 시 $\to 0$)
- $\bar h_T$ : 조도 포함 평균 간극, $\sigma=\sqrt{\sigma_1^2+\sigma_2^2}$, 표면패턴 파라미터 $\gamma$(=1 등방, >1 종방향, <1 횡방향).

> 한계: 평균유동 모델은 **국부 돌기압의 첨두(spike)** 를 직접 주지 않는다. 그 부분은 통계 접촉모델(2.2) 또는 결정론 모델(2.3)이 보완한다.

### 2.2 돌기접촉 통계 — Greenwood–Williamson / Greenwood–Tripp

Greenwood & Williamson(1966, *Proc. R. Soc. A* 295(1442):300–319)은 조도면을 동일 곡률반경 $\beta$ 의 구형 돌기군(높이 분포 $\phi(z)$, 밀도 $\eta_s$)으로 모형화하고, 각 돌기에 Hertz 접촉을 적용해 접촉수·실접촉면적·탄성하중을 높이분포 적분으로 표현했다. 탄·소성 판정은 **소성지수**

$$
\psi=\frac{E'}{H}\sqrt{\frac{\sigma_s}{\beta}}
\quad(\psi<0.6:\text{탄성},\ \psi>1.0:\text{소성})
$$

로 한다($H$=경도, $\sigma_s$=summit 높이 표준편차).

Greenwood & Tripp(1970–71, *Proc. IMechE* 185(1):625–633)은 이를 두 조도면으로 확장했고, 혼합 EHL/베어링 하중분담에 쓰이는 **평균 돌기접촉압** 형태를 준다:

$$
p_a(\lambda)=K'\,E'\,F_{5/2}(\lambda),\qquad
F_n(\lambda)=\frac{1}{\sqrt{2\pi}}\int_\lambda^\infty (s-\lambda)^n e^{-s^2/2}\,ds,\qquad \lambda=\frac{h}{\sigma}
$$

$F_{5/2}$ 의 차수가 $\lambda\to 0$ 에서 돌기압이 급상승하는 정도를 결정한다. `[VERIFY]` 선행계수($K'$ 의 $8\sqrt2/15$ vs $16\sqrt2/15$)는 "평균압 vs 총하중", sum‑surface $\sqrt2$ 관례에 따라 달라지므로 인용 시 기준 문헌에 고정할 것.

### 2.3 결정론적 Mixed EHL — Hu–Zhu 통합 Reynolds

Hu & Zhu(2000, *ASME J. Tribology* 122(1):1–9)는 **측정된 3D 조도면** 위에서 완전유막·혼합·경계 영역을 하나의 격자·하나의 방정식계로 푸는 **통합(unified) Reynolds 방정식**을 제시했다. 유막이 있는 절점($h>0$)에서는 통상 Reynolds 식이, 접촉 절점($h\to0$)에서는 $h^3$ Poiseuille 항이 소멸하여 **건접촉 조건으로 자동 축퇴**한다 — 영역 전환 없이 단일 압력장 $p$ 를 얻는 것이 핵심이다. 유막·변형식은 전 영역 공통:

$$
h(x,y)=h_0+\frac{x^2}{2R_x}+\frac{y^2}{2R_y}+\delta_1(x,y)+\delta_2(x,y)+V(x,y)
$$

($\delta_1,\delta_2$=측정 조도, $V$=전체 압력에 의한 탄성변형, DC‑FFT 합성). Zhu & Wang(2012, *Proc. IMechE Part J* 226(11):1010–1022; *Interfacial Mechanics*, CRC, 2019)은 이를 표준 실용기법으로 정립했다.

### 2.4 네 가지 핵심 출력과 하중분담

결정론 Mixed EHL 1회 수렴해에서 **동시에** 얻는 네 출력:

| 출력 | 정의/유래 | 의미 |
|---|---|---|
| 유막 $h,\ h_{\min}$ | 유막·변형식 | $\lambda$ 산정의 지배량 |
| 유체압 $p_h$ | 유막절점 Reynolds 해 | 유막이 지지하는 하중 |
| 돌기압 $p_a$ | 접촉절점($h\approx0$) 압력(결정론) 또는 $K'E'F_{5/2}$(통계) | 국부 응력집중 |
| 하중분담비 $\zeta$ | $\zeta=W_a/W_t,\ W_t=\!\int p_h\,dA+\!\int p_a\,dA$ | 돌기/유막 하중분배 |

$\zeta=0$ 완전유막, $\zeta=1$ 건·경계접촉, $0<\zeta<1$ 혼합. Zhu & Wang(2012)은 막비가 ~0.6–1.2 이상이면 사실상 완전유막, 반대로 ~0.05–0.1 의 매우 얇은 조건에서도 미시 유체역학이 하중의 10–15%를 지지할 수 있어 **혼합윤활 $\lambda$ 영역이 고전적 1–3 띠보다 훨씬 넓다**고 보고했다.

### 2.5 막비 $\Lambda$ 와 윤활 레짐

$$
\Lambda=\frac{h_{\min}}{\sigma},\qquad \sigma=\sqrt{\sigma_1^2+\sigma_2^2}
$$

Hamrock, Schmid & Jacobson(*Fundamentals of Fluid Film Lubrication*, 2nd ed., 2004)의 통상 경계는 $\Lambda<1$ 경계윤활, $1\le\Lambda\le3$ 혼합윤활, $\Lambda>3$ 완전유막이다(문헌별로 경계/완전유막 컷오프는 다소 상이 — `[VERIFY]`). Zhu & Wang(2012)은 이 고정 경계가 과단순임을 지적한다(2.4).

---

## 3. 출력 → 공학적 물리량 변환

Mixed EHL 네 출력은 그 자체가 목적이 아니라, 아래 **공학적 판정 물리량**으로 변환되어야 활용된다(4~9장의 입력).

| Mixed EHL 출력 | 유도 물리량 | 변환 관계 | 활용 장 |
|---|---|---|---|
| $h,\ h_{\min}$ | 막비 $\lambda=h/\sigma$ | 윤활 레짐·표면손상 위험지표 | 4·5·10 |
| $\zeta=W_a/W_t$ | 마찰계수 $\mu$ | $\mu=\dfrac{\mu_b W_a+\mu_h W_h}{W_t}$ (경계·유막 가중) | 4·6 |
| $\tau=\mu_b\,p_a$ | 마찰열류 $q=\mu_b p_a v_s$ | Blok flash 온도 입력 | 6 |
| $p_a$ | 돌기 응력이력 → $\tau_a,\sigma_h$ | Dang Van 피로 파라미터 | 5·8 |
| $p_a,\ v_s$ | 마모깊이 $\Delta h$ | 국부 Archard $\Delta h=k\,p_a\,\Delta s/H$ | 7 |
| $p_h+p_a$ | 아표면 응력장 | von Mises/직교전단 vs 깊이 | 8 |

핵심 통찰: **마이크로피팅·스커핑·마모는 Lundberg–Palmgren 기반 $a_{\text{ISO}}$(아표면 피로)가 원리적으로 다루지 않는 표면개시 모드**다. Mixed EHL 출력의 진짜 가치는 이 변환을 통해 그 손상 모드들을 정량화하는 데 있다.

---

## 4. 활용 ① 마찰·동력손실·발열

### 4.1 하중분담비로부터의 마찰계수

Mixed EHL의 하중분담비 $\zeta=W_a/W_t$ 는 마찰을 **유막 전단 성분과 경계(돌기) 성분의 가중합**으로 분해하는 직접 입력이다:

$$
\mu=\frac{\mu_b\,W_a+\mu_h\,W_h}{W_t}=\mu_b\,\zeta+\mu_h\,(1-\zeta)
$$

경계마찰 $\mu_b\approx0.1$–$0.15$ 가 유막마찰 $\mu_h\approx0.01$–$0.04$ 보다 훨씬 크므로, $\zeta$ 가 조금만 커져도 총마찰이 급증한다. 유막 성분 $\mu_h$ 는 윤활유 유변학(전단응력 $\tau$)에서, 그리고 점압거동에서 오며, 이는 본문 부록2(점압모델)와 직결된다. 그리스·저속에서는 한계전단·비뉴턴 효과가 트랙션에 관여한다(Liu et al. 2023, 로컬).

### 4.2 베어링 마찰토크·동력손실로의 집계

국부 마찰을 베어링 전체 토크로 적분·집계하는 실용 모델은 다음과 같다.

- **유체역학 구름저항력(rolling resistance):** Biboulet & Houpert(2010, *Proc. IMechE Part J*, line/point contacts)는 등점도-강체(IVR)와 EHL 레짐의 구름력을 닫힌형으로 주고, 전이 파라미터 $M$ 로 혼합한다(로컬 정리):
$$
F_R^{\text{Trans}}=\frac{1}{1+M/6.6}\,F_R^{\text{IVR}}+\frac{M/6.6}{1+M/6.6}\,F_R^{\text{EHL}}
$$
- **리브–롤러단 마찰(TRB 고유):** Wang, Wong & Zhang(1996, *Tribology International* 29(4):313–321, 로컬)은 리브 마찰을 **돌기 성분 + 유막전단 성분**으로 분해한다:
$$
M_{r}=M_{r,a}+M_{r,\text{EHL}}=\frac{\varepsilon}{D_s}\Big(F_{r,a}\,r_i+abE'\!\!\iint \bar\tau_x\big|_{Y=0}\,dX\,dZ\Big)
$$
$F_{r,a}=\mu_a W_a$ 의 $W_a$ 가 바로 Mixed EHL의 돌기하중이다.

- **다물체 시뮬레이션(MBS) 검증:** Wingertszahn et al.(2023, *Lubricants* 11:369, 로컬)은 슬라이스(주접촉)+셀(리브) 모델로 마찰토크를 60–5000 rpm, 0.1–100 kN에서 실측 대비 **±10% 이내**로 예측했다 — Mixed EHL 국부마찰을 베어링 토크로 집계하는 검증된 경로다.

### 4.3 발열 → 벌크온도 → 물성 피드백

총마찰 동력손실 $P_f=\sum \mu\,W\,v_s$ 는 발열원이 되어 접촉부 벌크온도를 올리고, 이는 $\eta_0(T),\ \alpha(T)$ 와 그리스 bleed를 바꿔 다시 유막·$\zeta$ 에 되먹임된다(본문 부록1 Step 0·4와 연결). 저속 메인베어링에서는 입구 전단발열($L<0.1$)은 작지만, **누적 발열에 의한 정상상태 벌크온도 상승**이 윤활성능의 지배 입력이 된다.

> **4장 요지:** $\zeta$ 와 국부 전단은 **수명계수가 아니라 마찰·동력손실·열모델의 입력**으로 쓴다. 이는 $a_{\text{ISO}}$ 와 독립적이며 이중반영 문제가 없다.

---

## 5. 활용 ② 마이크로피팅 (표면개시 피로)

### 5.1 왜 $a_{\text{ISO}}$ 로 부족한가

마이크로피팅은 돌기 스케일의 표면개시 RCF로, **아표면 개시에 calibration 된 Lundberg–Palmgren/$a_{\text{ISO}}$ 가 구조적으로 포착하지 못하는** 손상이다. Mixed EHL의 돌기압 이력이 이 모드의 직접 구동원이다.

### 5.2 Morales‑Espejel & Brizmer 경쟁 모델

Morales‑Espejel & Brizmer(2011, *Tribology Transactions* 54(4):625–643)는 마이크로피팅을 **표면피로와 마일드 마모의 경쟁**으로 모형화했다:
1. **표면피로** — 측정 조도를 부분 EHL에 넣어 돌기압(·전단) 이력을 구하고, 오버롤링마다 각 돌기가 받는 미시 응력사이클을 **Dang Van 다축피로 기준**으로 평가. 기준 초과 시 마이크로피트 개시.
2. **마일드 마모** — 수정 Archard형 국부마모가 돌기 정점을 깎아 표면을 평활화(런인).

마모가 피로 손상을 앞서 제거하면 마이크로피팅이 억제되고, 피로가 마모를 앞서면 발달한다. 위상은 사이클마다 갱신되어 다음 Mixed EHL 해에 되먹임된다. Morales‑Espejel(2021, *Proc. IMechE Part J* 235(8):1680–1691)은 여기에 **마찰발열·flash 온도**에 의한 표면손상(creep 기반)을 추가했다.

### 5.3 Dang Van 다축 피로 기준

Dang Van et al.(1989, *Biaxial and Multiaxial Fatigue*, EGF 3, pp. 459–478; 원전 1973)의 기준은 임의 순간 $t$ 에서

$$
\tau_a(t)+a\,\sigma_h(t)\le b,\qquad
a=\frac{\tau_{-1}-\tfrac12\sigma_{-1}}{\sigma_{-1}/3},\quad b=\tau_{-1}
$$

$\tau_a$=중시적 전단응력 진폭(Tresca), $\sigma_h=(\sigma_1+\sigma_2+\sigma_3)/3$ 순간 정수압, $\tau_{-1}$=완전반전 비틀림 피로한도, $\sigma_{-1}$=완전반전 인장 피로한도. 돌기압 사이클이 $\tau_a$ 를, 트랙션/SRR이 키운 근표면 인장 정수압이 $\sigma_h$ 를 만든다 — 따라서 Mixed EHL 출력으로 직접 구동된다.

### 5.4 $\lambda$·SRR·하중·조도 영향 (실험·수치)

- **SRR:** Rycerz & Kadiric(2019, *Tribology Letters* 67(2):63)는 유막을 고정한 채 SRR만 변화시켜, $|SRR|$ 증가가 마이크로피팅을 **증가**시키고(돌기 응력사이클 수 증가가 원인, 유막 감소 아님), 음(–)의 SRR이 같은 크기 양(+)보다 손상이 큼을 보였다.
- **속도·마모 경쟁:** Zhou, Zhu & Liu(2019, *Coatings* 9(10):639)는 mixed EHL→레인플로 계수→수정 Goodman/Miner 누적+국부 Archard로, 저속(얇은 막)일수록 손상면적비가 커지고(예 ~3.05% vs ~0.26%), $10^7$ 사이클 마모가 첨두 돌기압을 ~3→1.8 GPa로 낮춤을 보였다.
- **조도·초정밀가공:** Liu et al.(2019, *Coatings* 9(1):42 리뷰)에 따르면 조도가 가장 지배적 인자이며, $R_a\!\approx\!0.1\,\mu m$ 이하 초정밀가공(ISF)은 연삭 대비 피로수명 ~4배, 시험에서 마이크로피팅 0%를 보고. 런인 마일드마모가 $\lambda$ 를 키워 위험을 낮춘다.

### 5.5 ISO/TR 15144 (기어 마이크로피팅) 유추

ISO/TR 15144‑1:2014(`[VERIFY]` 판년)는 작용선 따라 국부 막비 $\lambda_{GF}=h/R_a$ 를 Dowson–Higginson형 식으로 점별 계산하고, 안전계수 $S_\lambda=\lambda_{GF,\min}/\lambda_{GFP}$ 로 위험을 등급화한다($S_\lambda<1$ 높음, $1\le S_\lambda<2$ 중간, $\ge2$ 낮음). 베어링용 표준은 아니지만, **국부 $\lambda$ 기반 표면손상 판정**의 표준화된 유추로 본 레포트의 베어링 마이크로피팅(5.2)과 논리가 동일하다.

> **5장 요지:** 돌기압 이력 → Dang Van 피로 + 마일드 마모 경쟁이 마이크로피팅의 정량 도구다. 이는 $a_{\text{ISO}}$ 와 **별개 손상모드**이므로 이중반영이 아니라 보완이다.

---

## 6. 활용 ③ 스커핑/스미어링 (접착 손상)

### 6.1 Blok flash 온도 기준

Blok(1937; 회고 "The flash temperature concept," *Wear* 6(6):483–494, 1963)은 스커핑(접착 유막붕괴)이 접촉부 **총 순간온도**가 윤활유 고유의 임계온도에 도달할 때 개시한다고 보았다:

$$
T_c=T_B+T_{fl}\ \ge\ T_{\text{crit}}
$$

$T_B$=벌크온도, $T_{fl}$=마찰 flash 온도상승. 띠형 Hertz 접촉의 flash 온도(AGMA 925/ISO/TR 13989‑1 코드화):

$$
T_{fl}=0.62\,\mu\,w_n\,\frac{|\sqrt{v_1}-\sqrt{v_2}|}{B_{M1}\sqrt{v_1}+B_{M2}\sqrt{v_2}}\cdot\frac{1}{\sqrt{b_H}}
$$

$B_M=\sqrt{\lambda\rho c}$=열침투계수. 핵심은 $T_{fl}\propto \mu\,w_n\,|\sqrt{v_1}-\sqrt{v_2}|$ — **국부 마찰계수 × 국부 하중 × 미끄럼속도**(마찰동력형 묶음)로, 이는 모두 Mixed EHL 출력($\mu$ from $\zeta$, $w_n$ from $p_a$, $v_s$ from 운동학)에서 온다. 직유(무첨가) 광유–강 조합의 임계온도는 ~150 °C, 점도·첨가제에 따라 ~150–300 °C(EP유 고온)로 인용된다.

### 6.2 혼합윤활 스커핑 기준 (트랙션·마찰동력)

- **최대 트랙션 > 수정 전단강도:** Zhang, Cheng & Wang(2004, *Tribology Transactions* 47(1):149–156)과 이를 정밀화한 Lyu, Meng, Zhang & Wen(2021, *Engineering Failure Analysis* 123:105276)은 스커핑을 $\tau_{\max}=\mu_b\,p_{\text{contact}}\ge\tau_{\lim}$ 로 본다. 온도상승→점도저하→유막박화→돌기하중↑→트랙션↑·전단강도↓ 의 되먹임으로 국부 붕괴. (브리프의 "Tribology International" 표기는 오류 — *Engineering Failure Analysis* 가 정확.)
- **마찰동력 강도(seizure):** Matveevsky(1992, *Wear* 155(1):1–5)는 단위면적 마찰동력 $q_f=\mu\,p\,v_s\ge q_{\text{crit}}$ 를 시저 기준으로 제시($PV$ 인자의 일반화). $q_{\text{crit}}$ 는 트라이보쌍별 상수로, 보편값이 아니라 측정·피팅값으로 취급.

### 6.3 돌기 첨두압 → 경계마찰 → flash → 국부 붕괴 (메커니즘 사슬)

혼합 EHL에서 $\Lambda$ 가 ~1 이하로 떨어지면 (1) 돌기 첨두압이 매끈 Hertz 평균을 초과하고, (2) 그 지점은 경계마찰($\mu_b$)로 작동, (3) Blok 식에 의해 극한 flash 온도가 발생, (4) 점도붕괴·EP막 파손→금속접착→스커핑. 이는 Lyu et al.(2021)의 "온도상승 윤활열화" 모델과 정확히 일치한다.

### 6.4 베어링 스미어링/스키딩

Evans, Barr, Houpert & Boyd(2013, *Tribology Transactions* 56(5):703–716)에 따르면 **스미어링은 베어링의 접착마모/스커핑 발현**으로, 무·저하중 구간(부하영역 밖)·고가속·케이지 슬립에서 롤러가 구르지 않고 미끄러지는 **스키딩**으로 발생한다. 부하영역 진입 시 급가속 슬립이 마찰열·접착 줄무늬를 만든다. 정량 기준으로 슬립률 ~5%, 트랙션곡선 정점 미만의 미끄럼/구름비에서는 미끄럼 없음, 슬립속도는 반경하중 증가에 지수적으로 감소 — 신품 초기·하중방향 반전에서 위험 최대.

### 6.5 풍력 메인베어링 적합성

Hart, de Mello & Dwyer‑Joyce(2022, *Wind Energy Science* 7:1533–1550)는 기아(현실 그리스) 조건에서 35 °C 운전점의 **~90%가 혼합윤활 레짐**, 5 °C 상승으로 EHL→혼합 전이, $\Lambda$ 는 하중에 둔감(50% 하중감소에 ~6%)하고 온도·기아·점압계수에 민감함을 보였다. 저속·저하중·하중반전이라는 스키딩·스미어링 호발 조건과 결합되어, **본 베어링에서 스커핑/스미어링은 예측 대상으로 타당한 손상모드**다.

> **6장 요지:** $\mu,p_a,v_s$ → flash 온도/트랙션 기준이 스커핑·스미어링의 정량 도구다. 다시 $a_{\text{ISO}}$ 와 독립.

---

## 7. 활용 ④ 마모 (마일드/접착, 런인)

### 7.1 국부 Archard

Archard(1953, *J. Applied Physics* 24(8):981–988)의 $V=K\,W\,s/H$ 를 Mixed EHL 격자에 **국부 적용**한다. 돌기압 $p_a(x,y)$ 가 있는 절점마다 패스당 마모깊이는

$$
\Delta h(x,y)=k\,\frac{p_a(x,y)\,\Delta s(x,y)}{H}
$$

$\Delta s$=국부 미끄럼거리(SRR·운동학), $H$=경도. **돌기하중만 마모에 기여**하며 유막지지분은 무마모 — 하중분담 $W_t=W_a+W_h$ 의 $W_a$ 가 입력이다. 윤활강 마찰쌍의 무차원 $K\approx10^{-8}$–$10^{-9}$(문헌 전형값, 인용 시 출처 PDF 확인).

### 7.2 에너지 기반 마모

마모를 소산 마찰에너지에 비례시키는 모델: $V=\alpha\sum E_d,\ E_d=\int \mu W\,ds$ (Ramalho & Miranda 2006, *Wear* 260:361–367; Fouvry et al. 2003, *Wear* 255:287–298, 2007, *Tribology International* 40:1428–1442). $\mu$ 를 내포해 마찰레짐 변화를 포착하며, 국부 누적에너지밀도로 마모 **깊이 분포**까지 예측 가능. 강·코팅의 $\alpha\sim10^{-6}$–$10^{-8}\ \mathrm{mm^3/J}$(조건의존, 인용 시 출처 확인).

### 7.3 마모–조도 진화 연성 (결정론)

Zhu, Martini, Wang, Hu, Lisowsky & Wang(2007, *ASME J. Tribology* 129(3):544–552)은 돌기접촉점에서만 국부 Archard로 마모시키고 위상을 갱신해 다음 Mixed EHL 해에 되먹이는 **마모–조도 진화 연성**을 제시했다(마모에 의한 위상변화가 원 조도·탄성변형과 동급일 수 있음). Feng et al.(2025, *Surface and Coatings Technology* 502:131939; 브리프의 Tribology International 표기 오류)은 3D 점접촉 Mixed EHL+Archard 양방향 연성으로 마무리면별 마모깊이차(CBN 대 honed +38.2% 등)를 보고.

### 7.4 런인과 마이크로피팅 경쟁

Akbarzadeh & Khonsari(2010, *ASME J. Tribology* 132(3):032102; 2009, 131(2):024503)는 하중분담 개념으로 런인 거동을 예측 — 마일드 마모가 돌기 정점을 깎아 $R_q\downarrow$, $\lambda=h_{\min}/\sigma\uparrow$ 로 혼합→완전유막 쪽으로 이동시켜 마찰·마이크로피팅을 함께 낮춘다. 이는 5장 Morales‑Espejel 경쟁모델의 "마모가 피로를 억제"하는 가지와 동일하며, **마모는 손상이자 동시에 보호(런인)** 라는 양면성을 갖는다(과도 평활화는 장기적으로 아표면 피팅으로 손상 이전 가능 — Liu et al. 2019).

> **7장 요지:** $p_a,v_s$ → 국부 Archard/에너지 마모가 마모깊이·런인·형상열화의 정량 도구다. 수명은 형상변화→$\lambda$→(5·8장)로 간접 연결되지, $a_{\text{ISO}}$ 직접 보정이 아니다.

---

## 8. 활용 ⑤ 수명통합 — 표면/아표면 분리

### 8.1 아표면 응력장과 임계깊이의 표면 이동

완전유막 EHL에서는 매끈 Hertz 압력이 작용해 최대 전단(직교/von Mises)이 표면 아래 $z\approx0.5a$–$0.78a$ 에 위치(Lundberg–Palmgren의 아표면 개시점)한다. 그러나 혼합 EHL에서는 돌기 첨두압이 매끈 Hertz 최대를 넘는 단파장 응력집중을 중첩시켜, **근표면 von Mises 응력의 첨두가 표면 쪽으로 이동**한다 — 즉 $\lambda$ 가 낮아질수록 균열개시점이 아표면→근표면으로 옮겨간다. Zhu, Ren & Wang(2009, *ASME J. Tribology* 131(4):041501)은 피팅이 Hertz 압력이 아니라 **아표면 von Mises 응력**과 상관됨을, Epstein, Keer, Wang, Cheng & Zhu(2003, *Tribology Transactions* 46(4):506–513)는 혼합윤활 표면형상이 접촉피로에 미치는 영향을 보였다. 따라서 $(p_h+p_a)$ 조합 압력장 → 근표면 응력장 계산이 표면개시 수명의 물리적 입력이다.

### 8.2 응력기반 수명 — Ioannides–Harris

Ioannides & Harris(1985, *ASME J. Tribology* 107(3):367–378)는 Lundberg–Palmgren(1947)을 일반화해, **피로한도 응력 $\tau_u$** 도입과 응력적분의 국소화로 임의 응력장(근표면 포함)에 적용 가능하게 했다:

$$
\ln\frac{1}{S}\ \propto\ N^e\int_V \frac{(\tau-\tau_u)^c}{z^h}\,dV\quad(\tau>\tau_u)
$$

$\tau$=임계응력(I–H는 von Mises, L–P는 직교전단 $\tau_0$), $V$=응력체적, $e,c,h$=Weibull·응력·깊이 지수. **$\tau_u=0$ 이면 L–P로 환원**된다. 이것이 ISO 281의 피로하중한 $C_u$ 와 $a_{\text{ISO}}$ 의 이론적 기반(Ioannides, Bergling & Gabelli 1999)이다. 다만 Zaretsky(NASA/TP‑2013‑215305)는 관통경화 베어링강의 진짜 피로한도 존재는 개방문헌 데이터로 뒷받침되지 않으며, 피로한도 사용이 수명을 무한대로 끌어 **과대예측**할 수 있다고 경고한다 — 비판적 인용 필요.

### 8.3 Tallian의 이원(표면/아표면) 모델

Tallian(1971, *Wear* 17(5–6), Part I–III)은 최초로 **표면/아표면을 구분한 공학 RCF 모델**을 제시해, 스폴링과 표면 distress(마이크로피팅)를 분리하고 표면수명을 결함 수·막비 $\lambda$·돌기 경사각의 함수로 두었다. GBLM은 이 Tallian형 표면항을 Ioannides–Harris 응력적분 + Mixed‑EHL/Dang Van 마이크로피팅으로 현대화한 것으로 위치지을 수 있다.

### 8.4 SKF GBLM — 표면/아표면 생존확률 분리

Morales‑Espejel, Gabelli & de Vries(2015, *Tribology Transactions* 58(5):894–906)의 일반화 베어링 수명모델(GBLM)은 생존확률을 **아표면항(고전 Hertz/L–P)과 표면항(트라이볼로지: 막비·돌기응력·덴트·표면 distress)** 으로 분리하고, 둘을 독립위험(직렬신뢰도)으로 결합한다. 수명수준 형태:

$$
L_{10}^{-1/\beta}=L_{10,\text{surf}}^{-1/\beta}+L_{10,\text{sub}}^{-1/\beta}
$$

($\beta$=Weibull 기울기). **표면항은 부분 EHL 압력장**(돌기 첨두압·미시 피로사이클)으로 구동되며, 그 마이크로피팅 엔진이 5장의 Morales‑Espejel–Brizmer(2011, Dang Van) 모델이다. 표면 distress 적분은 접촉압·윤활품질 $\kappa$·(덴트 시)압흔형상의 함수다. 핵심 이점은 **표면·아표면에 서로 다른 Weibull 기울기**를 부여할 수 있다는 점으로, 단일 스칼라 $a_{\text{ISO}}$ 로는 불가능하다. 관련: 인공덴트 손상 진행(Morales‑Espejel & Gabelli 2015, *Tribology Transactions* 58(3):418–431), 산발 덴트손상(2016, *Tribology International* 96:279–288), 하이브리드 베어링(2019, *Proc. IMechE Part C* 233(15):5491–5498; *Wear* 422–423:223–234 `[VERIFY]` 페이지).

> **8장 요지:** Mixed EHL 출력은 **GBLM의 표면항 입력**으로 들어가 표면개시 수명을 정량화한다. 아표면항은 매끈 Hertz 응력으로 별도 계산. 이것이 "수명에 쓰되 이중반영 않는" 정당한 경로다.

---

## 9. ISO 281과의 비중복(no-double-counting) 원칙

### 9.1 왜 $a_{\text{ISO}}$ 가 이미 조도를 담는가

ISO 281:2007의 수정수명 $L_{nm}=a_1\,a_{\text{ISO}}\,L_{10}$ 에서 $a_{\text{ISO}}=f(\kappa,C_u/P,\eta_c)$ 이고, $\kappa=\nu/\nu_1$ 의 기준점도 $\nu_1$ 은 표면분리에 필요한 최소유막을 기준으로 정의된다. 따라서 $\kappa$ 는 **막비 $\lambda=h_{\min}/\sigma$ 의 대리변수**이며, $a_{\text{ISO}}$ 는 박막·돌기상호작용(혼합윤활)의 수명 페널티를 **이미 스칼라로 내장**한다(Zaretsky 2016 정리). 여기에 Mixed EHL의 조도/돌기 결과를 추가로 곱해 $a_{\text{ISO}}$ 를 또 깎으면 동일 물리를 **이중 계산**한다(1.1).

### 9.2 동적정격하중 $C$ 인위보정의 비물리성

Morales‑Espejel & Gabelli(2015)는 표면효과를 맞추려 동적정격하중 $C$ 에 상수배율 $\chi$ 를 곱하는 방식이 비물리적임을 보였다 — $C$ 는 **아표면에만** 영향하는데 표면손상은 아표면 현상이 아니며, 필요한 $\chi$ 가 운전영역별로 <0.8에서 >1.7까지 변해 단일 상수로는 표면/아표면 경쟁을 표현할 수 없다.

### 9.3 실무 규칙: "하나의 물리효과, 하나의 모델항"

| Mixed EHL 출력 | 정당한 사용처 | 금지 사용처 |
|---|---|---|
| 막비 $\lambda$, 돌기응력이력, 근표면 응력, Dang Van 손상 | **GBLM 표면항** $L_{10,\text{surf}}$ | $a_{\text{ISO}}$ 추가 디레이팅 |
| 매끈 Hertz 아표면 von Mises | **GBLM 아표면항** $L_{10,\text{sub}}$ (I–H/L–P) | — |
| $\zeta$, 국부 전단, $p_a,v_s$ | 마찰·발열·마이크로피팅·스커핑·마모(4–7장) | 수명계수 재보정 |

- 조도/돌기/$\lambda$ 는 **(a) 고전 ISO 281 계산이라면 $\kappa$ 통해 $a_{\text{ISO}}$ 안에서, 또는 (b) GBLM의 명시적 표면항 안에서** 다루되 **둘 다는 금지**.
- GBLM의 명시적 표면모델을 채택하면, 아표면 수명 $L_{10,\text{sub}}$ 는 $\kappa$ 로 디레이팅하지 않은(=조도는 표면항이 담당) 값을 써야 한다.

> **9장 요지:** Mixed EHL 출력은 "수명을 다시 깎는" 보정이 아니라, **표면개시 손상의 명시적 정량화(GBLM 표면항·4~7장 손상모델)** 로 쓴다. $a_{\text{ISO}}$ 와 명시적 표면모델의 **택일**이 비중복의 핵심.

---

## 10. 20MW+ 풍력 메인베어링 적용

### 10.1 운전 레짐과 지배 손상모드

본 베어링은 저속·고하중·그리스 윤활 대구경 TRB로, 본문 부록1·2에서 정리했듯 **혼합/경계윤활이 운전수명의 상당비율을 차지**한다. Hart et al.(2022, *Wind Energy Science* 7:1533–1550)은 기아 조건에서 35 °C 운전점의 ~90%가 혼합윤활, $\Lambda$ 가 하중보다 온도·기아·점압계수에 민감함을 보였다. Stirling(2023, Univ. of Strathclyde Diss., 로컬)은 모멘트하중에 의한 롤러 에지로딩·2열 부등분담을 보고했다. 이 레짐에서 Mixed EHL 출력이 직접 답하는 지배 손상모드는:

| 손상모드 | 본 레짐에서의 발생 인자 | 1차 입력(Mixed EHL) | 해당 장 |
|---|---|---|---|
| 마이크로피팅 | 박막·혼합윤활·SRR(스큐) | 돌기압 이력 → Dang Van | 5 |
| 스미어링/스커핑 | 저·무하중 구간 스키딩, 하중반전 | $\mu(\zeta),p_a,v_s$ → flash/트랙션 | 6 |
| 마모·런인 | 그리스 기아·미보충·초기조도 | $p_a,v_s$ → 국부 Archard | 7 |
| 표면개시 RCF | 근표면 응력집중 | $(p_h+p_a)$ → GBLM 표면항 | 8 |
| 아표면 RCF | 고하중 Hertz | 매끈 Hertz → $a_{\text{ISO}}$/GBLM 아표면항 | 8·9 |
| (별도) 정지·미소진동 | false brinelling·WEC | EHL 밖, 별도 기준 | 10.3 |

### 10.2 에지로딩·크라우닝과 국부 출력의 가치

모멘트하중·미스얼라인먼트에 의한 **에지로딩**은 롤러 단부에서 응력·돌기접촉을 국부적으로 집중시킨다. 매끈 Hertz·$a_{\text{ISO}}$ 만으로는 이 국부집중을 보지 못하므로, **수정 프로파일(크라우닝) 설계 검증**에 Mixed EHL의 국부 $p_a$·$\lambda$ 분포가 직접 쓰인다(4·5·8장 연계). 이는 "수명계수"가 아니라 **형상설계 판정** 용도다.

### 10.3 정상상태 EHL 밖의 손상 (경계 명시)

정지·미소진동(요/피치 과도)에서의 **false brinelling·프레팅 부식·WEC(백색에칭균열)** 는 고전·혼합 EHL의 정상상태 틀로 예측되지 않는다(본문 부록1 Step 7). Mixed EHL 출력은 이 모드들의 **직접 수명예측이 아니라 위험 스크리닝**(미끄럼량·접촉응력·국부 마찰열의 입력)으로만 활용해야 하며, 별도의 트라이보화학·접촉역학 기준이 필요함을 명시한다.

### 10.4 실측·상태감시(CM) 연계

마찰토크(4장, Wingertszahn 2023 ±10% 검증)·온도·진동 신호는 Mixed EHL 예측의 검증·보정 입력이 된다. 그리스 기아·막비 변화는 직접 계측이 어려우므로(본문 부록1), Mixed EHL 모델은 **CM 신호 해석의 물리기반 프록시**로 기능한다.

> **10장 요지:** 본 레짐에서 Mixed EHL 출력의 최우선 가치는 **마이크로피팅·스미어링·에지로딩 설계검증**이며, 수명은 GBLM 표면/아표면 분리(8·9장)로 다룬다.

---

## 11. 결론 — 실무 의사결정 플로우

### 11.1 핵심 결론

1. **조도 이중반영 문제는 타당**하며, MK형 조도내장 유막으로 $\kappa\to a_{\text{ISO}}$ 를 재보정하면 안 된다(1.1, 9.1).
2. **수명계수는 $a_{\text{ISO}}$ 로 충분**하되, 그것은 아표면 개시 피로에 한정된다(1.2).
3. **Mixed EHL 출력의 정당한 용도**는 ($a_{\text{ISO}}$ 가 못 잡는) **표면개시 손상(마이크로피팅·스커핑·마모)·마찰/발열**의 정량화와 **GBLM 표면항** 입력이다(3~9장).
4. **비중복 원칙**: 조도/$\lambda$ 는 $a_{\text{ISO}}$(고전) **또는** GBLM 표면항(명시) 중 한 곳에서만 다룬다(9.3).

### 11.2 목적별 도구 선택 플로우

```
[목적이 무엇인가?]
   │
   ├─ 표준 수명(인증/카탈로그)  ─────────► ISO 281 a_ISO (κ=ν/ν1)  [Mixed EHL 미시출력 사용 금지]
   │
   ├─ 정밀 수명(표면+아표면 분리) ────────► SKF GBLM
   │        ├─ 아표면항: 매끈 Hertz von Mises → I–H/L–P
   │        └─ 표면항 : Mixed EHL 돌기응력 → Dang Van(마이크로피팅)
   │                    (이때 a_ISO의 κ 디레이팅 중복 금지)
   │
   ├─ 마찰·동력손실·발열 ────────────────► ζ=Wa/Wt → μ 가중합 → 토크 집계(Wingertszahn/Biboulet–Houpert)
   │
   ├─ 마이크로피팅 위험 ─────────────────► 돌기압 이력 → Dang Van + 마일드 Archard 경쟁(Morales–Espejel)
   │
   ├─ 스커핑/스미어링 위험 ──────────────► μ·p_a·v_s → Blok flash(T_crit) / 트랙션>τ_lim / 스키딩 슬립률
   │
   ├─ 마모·런인·형상열화 ───────────────► p_a·Δs → 국부 Archard/에너지 마모 → 조도진화 → (λ 재평가)
   │
   └─ 형상설계(크라우닝·에지로딩) ───────► Mixed EHL 국부 p_a·λ 분포 → 프로파일 검증
```

### 11.3 권고

- 본 20MW+ 메인베어링은 **혼합윤활 비중이 크므로**, 카탈로그 $a_{\text{ISO}}$ 수명 외에 **GBLM 표면항 + 마이크로피팅/스미어링 스크리닝**을 병행할 것을 권고.
- Mixed EHL 해석은 **수명 재보정 도구가 아니라 손상모드·설계검증 도구**로 포지셔닝.
- `[VERIFY]` 표기 서지(아래)는 본문 인용 전 1차 원문 대조 필요.

---

## 12. 통합 참고문헌

> 표기: 대부분 DOI·권·페이지까지 1차 검증. `[VERIFY]`=서지 일부 미확정(원문 대조 권장), `(로컬)`=Reference 폴더 보유 2차자료.

**이론·출력 (2·3장)**
1. Patir, N., Cheng, H. S. (1978). An Average Flow Model… *ASME J. Lubr. Technol.* 100(1):12–17.
2. Patir, N., Cheng, H. S. (1979). Application of Average Flow Model… *ASME J. Lubr. Technol.* 101(2):220–230. `[VERIFY]` 페이지.
3. Greenwood, J. A., Williamson, J. B. P. (1966). Contact of Nominally Flat Surfaces. *Proc. R. Soc. A* 295(1442):300–319.
4. Greenwood, J. A., Tripp, J. H. (1970–71). The Contact of Two Nominally Flat Rough Surfaces. *Proc. IMechE* 185(1):625–633.
5. Hu, Y. Z., Zhu, D. (2000). A Full Numerical Solution to the Mixed Lubrication in Point Contacts. *ASME J. Tribology* 122(1):1–9.
6. Zhu, D., Wang, Q. J. (2012). On the λ Ratio Range of Mixed Lubrication. *Proc. IMechE Part J* 226(11):1010–1022.
7. Wang, Q. J., Zhu, D. (2019). *Interfacial Mechanics.* CRC Press.
8. Hamrock, B. J., Schmid, S. R., Jacobson, B. O. (2004). *Fundamentals of Fluid Film Lubrication*, 2nd ed. `[VERIFY]` Λ 경계 페이지.

**마찰·동력손실 (4장)**
9. Biboulet, N., Houpert, L. (2010). Hydrodynamic force/moment in pure rolling lubricated contacts, Parts 1–2. *Proc. IMechE Part J.* (로컬)
10. Wang, Wong, Zhang (1996). Partial EHL analysis of rib–roller end contact in TRB. *Tribology International* 29(4):313–321. (로컬)
11. Wingertszahn et al. (2023). Predicting Friction of TRB with Detailed MBS Models. *Lubricants* 11:369. (로컬)
12. Liu et al. (2023). Thermal EHL of inner-ring rib and roller end in TRB with Carreau model. (로컬)

**마이크로피팅 (5장)**
13. Morales‑Espejel, G. E., Brizmer, V. (2011). Micropitting Modelling in Rolling–Sliding Contacts… *Tribology Transactions* 54(4):625–643.
14. Morales‑Espejel, G. E. (2021). Thermal damage and fatigue estimation… *Proc. IMechE Part J* 235(8):1680–1691.
15. Dang Van, K., Griveau, B., Message, O. (1989). On a new multiaxial fatigue limit criterion. In *Biaxial and Multiaxial Fatigue*, EGF 3, 459–478. (원전 1973)
16. Rycerz, P., Kadiric, A. (2019). Influence of Slide–Roll Ratio on Micropitting… *Tribology Letters* 67(2):63.
17. Zhou, Y., Zhu, C., Liu, H. (2019). A Micropitting Study Considering Rough Sliding and Mild Wear. *Coatings* 9(10):639.
18. Liu, H. et al. (2019). A Review on Micropitting Studies of Steel Gears. *Coatings* 9(1):42.
19. ISO/TR 15144‑1·2:2014. Micropitting load capacity of cylindrical gears. `[VERIFY]` 판년(원판 2010).

**스커핑·스미어링 (6장)**
20. Blok, H. (1963). The flash temperature concept. *Wear* 6(6):483–494. (원 1937 IMechE/WPC `[VERIFY]` 페이지)
21. Zhang, C., Cheng, H. S., Wang, Q. J. (2004). Scuffing Behavior… Part II. *Tribology Transactions* 47(1):149–156.
22. Lyu, B., Meng, X., Zhang, R., Wen, C. (2021). A deterministic contact evolution and scuffing failure analysis… *Engineering Failure Analysis* 123:105276.
23. Matveevsky, R. M. (1992). Friction power as a criterion of seizure… *Wear* 155(1):1–5.
24. Evans, R. D., Barr, T. A., Houpert, L., Boyd, S. V. (2013). Prevention of Smearing Damage in CRB. *Tribology Transactions* 56(5):703–716.
25. ISO/TR 13989‑1:2000; AGMA 925‑A03/B22 (Blok flash 온도법 표준화).

**마모·런인 (7장)**
26. Archard, J. F. (1953). Contact and Rubbing of Flat Surfaces. *J. Appl. Phys.* 24(8):981–988.
27. Zhu, D., Martini, A., Wang, W., Hu, Y., Lisowsky, B., Wang, Q. J. (2007). Simulation of Sliding Wear in Mixed Lubrication. *ASME J. Tribology* 129(3):544–552.
28. Akbarzadeh, S., Khonsari, M. M. (2010). On the Prediction of Running‑In Behavior… *ASME J. Tribology* 132(3):032102.
29. Akbarzadeh, S., Khonsari, M. M. (2009). Prediction of Steady State Adhesive Wear in Spur Gears… *ASME J. Tribology* 131(2):024503.
30. Ramalho, A., Miranda, J. C. (2006). Relationship between wear and dissipated energy. *Wear* 260(4–5):361–367.
31. Fouvry, S. et al. (2003). Energy description of wear mechanisms. *Wear* 255:287–298.
32. Brandão, J. A., Martins, R., Seabra, J. H. O., Castro, M. J. D. (2014). Gear tooth flank surface wear during FZG micropitting test. *Wear* 311(1–2):31–39.
33. Feng, Y. et al. (2025). Predictions of friction and wear in ball bearings based on 3D point contact mixed EHL. *Surface and Coatings Technology* 502:131939.

**수명통합·비중복 (8·9장)**
34. Lundberg, G., Palmgren, A. (1947). Dynamic Capacity of Rolling Bearings. *Acta Polytechnica*, Mech. Eng. Ser. 1(3). (롤러: IVA Handlingar 210, 1952)
35. Ioannides, E., Harris, T. A. (1985). A New Fatigue Life Model for Rolling Bearings. *ASME J. Tribology* 107(3):367–378.
36. Ioannides, E., Bergling, G., Gabelli, A. (1999). An Analytical Formulation for the Life of Rolling Bearings. *Acta Polytechnica Scandinavica*, ME 137.
37. ISO 281:2007; ISO/TR 1281‑2:2008. (로컬)
38. Tallian, T. E. (1971). An engineering model of spalling fatigue failure…, Parts I–III. *Wear* 17(5–6). `[VERIFY]` 페이지.
39. Morales‑Espejel, G. E., Gabelli, A., de Vries, A. J. C. (2015). A Model for Rolling Bearing Life with Surface and Subsurface Survival. *Tribology Transactions* 58(5):894–906.
40. Morales‑Espejel, G. E., Gabelli, A. (2015). The Progression of Surface RCF Damage… with Artificial Dents. *Tribology Transactions* 58(3):418–431.
41. Morales‑Espejel, G. E., Gabelli, A. (2016). …Sporadic Surface Damage from Deterministic Indentations. *Tribology International* 96:279–288.
42. Morales‑Espejel, G. E., Gabelli, A. (2019). …Hybrid bearing cases. *Proc. IMechE Part C* 233(15):5491–5498; *Wear* 422–423:223–234 `[VERIFY]`.
43. Epstein, D., Keer, L. M., Wang, Q. J., Cheng, H. S., Zhu, D. (2003). Effect of Surface Topography on Contact Fatigue in Mixed Lubrication. *Tribology Transactions* 46(4):506–513.
44. Zhu, D., Ren, N., Wang, Q. J. (2009). Pitting Life Prediction Based on 3‑D Line‑Contact Mixed EHL and Subsurface von Mises Stress. *ASME J. Tribology* 131(4):041501.
45. Zaretsky, E. V. (2016). *Rolling Bearing Life Prediction, Theory, and Application.* NASA/TP‑2013‑215305/REV1.

**풍력 적용 (10장)**
46. Hart, E., de Mello, E., Dwyer‑Joyce, R. (2022). Wind turbine main‑bearing lubrication Part 1·2. *Wind Energy Science* 7:1021–1042; 7:1533–1550.
47. Stirling, A. (2023). Internal load modelling of tapered‑roller main bearings in wind turbines. Diss., Univ. of Strathclyde. (로컬)

> 본 레포트는 본문 `분석_EHL 윤활해석.md`의 **부록1(가정·한계)·부록2(점압모델)** 와 상호 보완 관계다.

---

# 부록 1. Mixed EHL 출력의 고속 준해석(semi-analytical) 계산 — 표면피로용 응력장까지

> **동기(사용자 문제제기):** 표면피로 평가에는 결국 **6개 표면/아표면 응력성분**이 필요하고, 이를 위해 상세 유막·돌기 압력분포가 있어야 한다. 이를 **풀 수치해석(deterministic mixed EHL) 없이 괜찮은 정확도로 빠르게** 얻을 수 있다면 큰 이득이다. 결론부터: **가능하며, 이미 확립된 준해석 파이프라인이 존재한다.** 사용자의 직관(가우시안 조도 → 푸리에 분해 → 압력 리플)은 *진폭감쇠(amplitude reduction) 이론*과 정확히 일치한다.

## A1.0 4단계 고속 파이프라인 개요

```
[운전조건+조도]
  └①스칼라 회귀식 ──► h_c, h_min, 돌기하중비 L_a, 최대압력      (Masjedi–Khonsari)
  └②상세 압력분포 ──► p_h(x), p_a(x) 리플                      (진폭감쇠 / 하중분담)
        └③해석적 응력 ──► σ_xx,σ_zz,σ_xz,σ_yy (6성분)           (Smith–Liu + DC-FFT)
              └④다축 피로 ──► 마이크로피팅/RCF 수명·FoS          (Dang Van / Crossland)
```

각 단계는 수치 EHL solver를 회귀식·해석해·FFT 합성으로 대체한다.

> **A1.0-보론. 통계 모델이 "매끈한" 압력분포를 주는 이유와 그 함의 (스파이크가 없는 배경)**
>
> Masjedi & Khonsari(2012, [B1])의 수치 결과(원논문 Fig. 2: $W=10^{-4},U=10^{-11},G=4500,V=0.01$, $\bar\sigma=5\times10^{-6}\to2\times10^{-5}\to5\times10^{-5}$)는 거칠기가 커져도 돌기압 $P_a$·총압 $P$ 가 **매끈하게(스파이크 없이)** 나온다. 이는 모델의 근본 성격 때문이다.
>
> 1. **통계 평균(stochastic-averaged) 모델 — 결정론 아님.** 거칠기를 3D 위상으로 해상하지 않고 두 통계모형으로 평균화한다: 유체압 $P_h$ 는 **Patir–Cheng(1978) 평균유동**(유동계수 $\phi_x,\phi_s$ 로 거칠기 효과 평균), 돌기압 $P_a$ 는 **Greenwood–Williamson/Zhao–Maietta–Chang 통계 탄소성 접촉**(국부 간극의 함수).
> 2. **수학적 이유.** $P_a(X)\propto E'\,F_{5/2}\!\big(h(X)/\sigma\big)$ 에서 평균 간극 $h(X)$ 와 통계적분 $F_{5/2}$ 가 모두 매끈하므로 $P_a$ 도 매끈하다. 가우시안 높이분포에 대한 **기대값 적분을 취하는 순간 개별 돌기 첨두가 평균 속으로 사라진다.**
> 3. **거칠기는 스칼라 $\bar\sigma=\sigma/R$ 로만 개입** — 지배식은 매끈한 거시영역에서 풀린다. 해상할 개별 돌기접촉이 없어 스파이크가 생길 여지가 없다(이 덕분에 회귀식 압축이 가능).
> 4. **세 패널의 메시지 = 하중분담 이동.** $\bar\sigma\!\uparrow$ → $P_a$ plateau 상승(~0→0.15→0.35), $P_h$ 첨두 하강(~1.0→0.82→0.58), 총압 첨두는 낮아지며 넓게 퍼짐. 즉 막비 저하 시 하중이 **유막→돌기로 이전**됨을 보일 뿐, 응력집중을 보이는 게 아니다.
> 5. **⚠️ 출구쪽 첨두는 조도 스파이크가 아니라 EHL 출구 압력 스파이크(Petrusevich spike).** 유막 수축부의 정상적 EHL 특징, 거칠기와 무관.
>
> **함의(표면피로와 직결):** 이 매끈한 $P_a$ 는 **스칼라 하중분담비 $L_a$·평균 마찰**에는 완벽하지만, **표면피로용 6응력성분(근표면 von Mises 첨두)을 계산하기엔 부족**하다 — 응력 스파이크가 평균화로 지워졌기 때문. 따라서 ①(회귀식)은 **설계 스크리닝**, ②(b)/③(진폭감쇠·결정론)은 **국부 손상해석**을 담당하는 **상보 관계**다. 빠른 6응력성분이 목표라면 ①의 평균압 위에 **A1.2(b) 진폭감쇠(Hooke–Li 2006)로 결정론 리플을 재구성**하거나 결정론 Mixed EHL(Hu–Zhu 2000)을 거쳐야 한다.

## A1.1 ① 스칼라 회귀식 (Masjedi–Khonsari)

Masjedi & Khonsari(2012, *ASME J. Tribology* 134(1):011503)는 결정론 Mixed EHL(수정 Reynolds + 탄소성 돌기변형) 대량해를 회귀하여 **중심유막 $H_c$, 최소유막 $H_{\min}$, 돌기하중비 $L_a=W_{asp}/W_{total}$** 를 무차원 $W,U,G$ + 무차원조도 $\bar\sigma=\sigma/R$ + 경도 $V$ 의 거듭제곱곱 닫힌형으로 제공한다. 점접촉판은 Masjedi & Khonsari(2015, *Tribology International* 82(A):228–244), 표면패턴 확장은 (2014, *Proc. IMechE Part J* 1350650114534228). 이는 **즉시 계산되는 스칼라 대리모델**이나, 압력 profile은 주지 않는다 — 분포는 ②가 담당.

## A1.2 ② 상세 압력분포의 고속 계산

두 계열이 서로 **다른 질문**에 답하므로 구분이 핵심이다.

### (a) 하중분담(load-sharing) — 평균압·하중분배 (리플 없음)
Johnson, Greenwood & Poon(1972, *Wear* 19(1):91–108)이 창시했고, Gelinck & Schipper(2000, *Tribology International* 33(3–4):175–181)가 평활 EHL 유막 + Greenwood–Tripp(1970) 통계 돌기접촉을 **병렬 하중분담**($W_t=W_h(h)+W_a(h)$, 공통 간극 $h$)으로 풀어 Stribeck 곡선을 준다. Greenwood–Tripp 평균 돌기압을 **국부 간극에 점별 적용**하면 *공간변화 평균 돌기압장*까지 얻지만(사용자 아이디어의 통계판), 이는 **통계평균이지 결정론 리플이 아니다.** Akchurin et al.(2015, *Tribology Letters* 59:19)은 측정조도 반무한체 BEM(FFT)으로 결정론 돌기압장 + 평균유막을 빠르게 결합한다.

### (b) 진폭감쇠/조화감쇠 — 상세 리플 압력 (사용자 직관의 정확한 구현)
정현 조도 1성분(파장 $\lambda$, 진폭 $A_i$)이 EHL 고압부를 지나며 탄성 평탄화되어 변형진폭 $A_d$ 로 감쇠된다. **측정/가우시안 조도를 푸리에 조화성분으로 분해 → 각 성분을 전달함수로 감쇠 → 중첩 → 변형 간극·압력 리플 재구성**:
- Greenwood & Morales‑Espejel(1994, *Proc. IMechE Part J* 208(2):121–132): 정현 조도의 압력 리플 해석식(섭동해, 입구 생성 complementary wave 포함).
- Venner & Lubrecht(1999)·Hooke & Venner(2000, *Proc. IMechE Part J* 214(5):439–444): 진폭감쇠가 단일 무차원 파라미터의 **마스터커브**로 붕괴(장파장 강감쇠~10%, 단파장 거의 무감쇠). `[VERIFY]` 파라미터 $\nabla$ 정의식.
- **Hooke & Li(2006, *Proc. IMechE Part C* 220(6):901–914): FFT로 압력·간극장을 거의 실시간 재구성** ★사용자 목표의 직접 구현체.
- 건접촉 빌딩블록: Westergaard(1939, *J. Appl. Mech.* 6:A49–A53), Johnson, Greenwood & Higginson(1985, *Int. J. Mech. Sci.* 27(6):383–396), 완전접촉압 $p^*=\pi E^*\Delta/\lambda$.
- 종합 리뷰: Morales‑Espejel(2014, *Proc. IMechE Part J* 228(11):1217–1242).
- **제약:** 선형 섭동영역(진폭 $\ll$ 유막)에서 정확, 고진폭은 근사. 순수구름 기준(rolling-sliding은 후속연구).

## A1.3 ③ 압력분포 → 6개 응력성분 (해석적·고속)

표면압력 $p(x)$ + 트랙션 $q(x)=\mu p(x)$ 만 있으면 반무한체 응력장은 **선형 중첩(convolution)** 으로 닫힌형:

### (a) 닫힌형 해석해
- Flamant(2D 선하중)·Boussinesq–Cerruti(3D 법선·접선 점하중) 그린함수. 분포하중은 이를 적분:
$$
\sigma_{ij}(x,z)=\int_{-a}^{a}\!\big[p(s)\,G^{N}_{ij}(x-s,z)+q(s)\,G^{T}_{ij}(x-s,z)\big]\,ds
$$
- **McEwen(1949, *Phil. Mag.* Ser.7 40):** Hertz 법선하중 아표면 응력 닫힌형(보조변수 $m,n$).
- **Smith & Liu(1953, *J. Appl. Mech.* 20(2):157–166):** 법선+접선 Hertz 조합 아표면 응력 닫힌형 ★표준 인용. Johnson(*Contact Mechanics*, 1985) Ch.2–4가 이를 정리.

### (b) 임의 분포 → DC-FFT 고속 합성
회귀/진폭감쇠로 얻은 임의 압력분포는 영향계수(IC) 합성으로:
- **Liu, Wang & Liu(2000, *Wear* 243:101–111): DC-FFT** — $g=\mathrm{IFFT}(\mathrm{FFT}(K)\!\cdot\!\mathrm{FFT}(p))$, $O(N\log N)$. 영패딩+wrap-around로 **비주기(aperiodic)** 정확.
- **Liu & Wang(2002, *ASME J. Tribology* 124(1):36–45): 표면 트랙션 → 아표면 6응력성분** DC-FFT.
- 비-FFT 대안: Polonsky & Keer(1999, *Wear* 231:206–219, MLMS+CG), Brandt & Lubrecht(1990, *J. Comput. Phys.* 90:348–370, MLMI). 가속 IC: Nikas(2006, *Proc. IMechE Part J* 220(1):19–28, 2–20× 속도향상).

### (c) 응력집중의 표면 이동 (왜 표면개시인가)
매끈 Hertz는 최대전단 $\approx0.30p_0$ @ $z\approx0.78a$, 교번 직교전단 $\approx0.25p_0$ @ $z\approx0.5a$ 로 **아표면**. 두 효과가 임계응력을 **표면으로 이동**시킨다:
- **마찰:** Hamilton & Goodman(1966, *J. Appl. Mech.* 33(2):371–376) — $\mu\gtrsim0.3$ 에서 최대 von Mises가 표면(후연부)으로.
- **조도:** 단파장 압력 하모닉의 응력은 깊이에 따라 $\sim e^{-2\pi z/\lambda}$ 로 감쇠(Westergaard/JGH) → 표면 근처 집중. Webster & Sayles(1986, *ASME J. Tribology* 108(3):314–320) 수치확인, 근표면 von Mises 최대 ~+30%.

> **A1.3-보론. 조화성분 응력 닫힌형(식 12·13)의 배경과 계산법 (Morales‑Espejel & Brizmer 2011)**
>
> Morales‑Espejel & Brizmer(2011, [13]) 마이크로피팅 모델은 6응력성분을 **단일 푸리에 조화 표면하중에 대한 반무한체 닫힌형**으로 계산한다. 법선압력 $p(x,y,0)=p_0\cos(\alpha x)\cos(\beta y)$ 와 트랙션 $q(x,y,0)=q_0\cos(\alpha x)\cos(\beta y)$ 에 대해 깊이 $z$ 의 $\sigma_x,\sigma_y,\sigma_z,\tau_{xy},\tau_{yz},\tau_{xz}$ 를 준다(원논문 식 12: 법선부, 식 13: 접선부).
>
> **(1) 무엇인가 — 주파수영역 그린함수(FRF).** 이 식들은 A1.3의 공간영역 Boussinesq–Cerruti 그린함수의 **주파수영역 등가물(frequency response function)** 이다. 반무한체가 선형탄성이므로, 조화 표면하중에 대한 응력 응답도 **같은 면내 파수 $\alpha,\beta$ 의 조화함수**이고, 깊이방향만 별도 함수로 분리된다. 여기서 핵심 파라미터는 **2D 파수 크기**
> $$\zeta=\sqrt{\alpha^2+\beta^2}\quad(=2\pi/\lambda),$$
> 모든 성분에 공통으로 곱해지는 **보편 깊이감쇠 $e^{-\zeta z}$**, 그리고 무차원 조합 $(\alpha/\zeta),(\beta/\zeta)$(파수벡터 방향코사인)·$\zeta z$(무차원 깊이)·$\nu$ 이다.
>
> **(2) 정당성 — 표면 경계조건 만족.** $z=0$ 에서 식이 작용하중으로 환원된다:
> - 법선부: $\sigma_z\big|_{z=0}=p_0\cos\alpha x\cos\beta y$ (=작용압력 ✓), $\tau_{xz}\big|_{z=0}=p_0(\alpha z)e^{-\zeta z}\sin\alpha x\cos\beta y\to0$ (법선하중→표면전단 0 ✓).
> - 접선부: $\tau_{xz}\big|_{z=0}=q_0[1-(\alpha/\zeta)(\alpha z)]e^{-\zeta z}\cos\alpha x\cos\beta y\to q_0\cos\alpha x\cos\beta y$ (=작용트랙션 ✓), $\sigma_z\big|_{z=0}\to0$ ✓.
>
> **(3) 유도 배경.** 반무한체 Navier 방정식(또는 Papkovich–Neuber 포텐셜)에 조화 ansatz를 넣으면 해가 **(z의 다항식)$\times e^{-\zeta z}\times$(면내 조화함수)** 형태가 된다. biharmonic 연산자의 이중근 때문에 $(1+\zeta z)e^{-\zeta z}$, $(\alpha z)e^{-\zeta z}$ 같은 항이 나타난다(예: $\sigma_z^{(p)}=p_0(1+\zeta z)e^{-\zeta z}\cos\alpha x\cos\beta y$).
>
> **(4) 계산법 — FFT 중첩 워크플로(원논문의 "FFT approach"):**
> 1. 매 순간 $t$(오버롤링 위치)마다 변형 조도 기반 고속 EHL해(A1.2)에서 $p(x,y,0),\ q(x,y,0)=\mu p$ 획득.
> 2. 이를 **2D FFT로 조화 분해**: $p=\sum_{m,n}p_{mn}\cos(\alpha_m x)\cos(\beta_n y)$, $\alpha_m=2\pi m/L_x,\ \beta_n=2\pi n/L_y,\ \zeta_{mn}=\sqrt{\alpha_m^2+\beta_n^2}$.
> 3. 각 하모닉 $(m,n)$ 에 식 12·13 적용 → 그 하모닉의 6성분 깊이분포.
> 4. 전 하모닉 **중첩(합)** → 임의 깊이 $z$ 의 $\sigma_{ij}(x,y,z)$. FFT로 $O(N\log N)$, **FEM·서브표면 메시 불필요**.
> 5. $m$ 개 시간스텝(오버롤링) 반복 → 각 물질점의 **응력이력** 구성.
> 6. 이력을 Dang Van 기준(A1.4)에 투입 → 마이크로피팅 개시.
>
> **(5) 핵심 물리 — 왜 표면개시인가(스펙트럼 근거).** 깊이감쇠율이 $\zeta=2\pi/\lambda$ 이므로 각 조도 파장은 깊이 $\sim\lambda/2\pi$ 까지만 침투한다. **단파장(돌기) 하모닉 → 얕은 근표면 응력집중**, 장파장(거시 Hertz) → 심부. 이것이 A1.3(c)에서 정성적으로 말한 "조도가 임계응력을 표면으로 이동"의 **정량적 스펙트럼 증명**이며, Smith–Liu 닫힌형([B14])·Liu–Wang FRF/DC‑FFT([B17])와 동일 골격이다.
>
> **(6) 빠른 이유와 한계.** A1.2(b) 진폭감쇠가 이미 조도를 푸리에 하모닉으로 분해하므로, **같은 하모닉 분해 위에 식 12·13을 곧바로 얹어** 압력→응력을 한 번에 처리한다(분해→감쇠→응력→중첩이 모두 푸리에 골격). 한계: 선형탄성 반무한체·소진폭·FFT 주기성(영패딩 필요)·$q=\mu p$(Coulomb) 가정, 잔류응력·경도구배는 별도 중첩. 식 [12]·[13]의 인용표시 "(16)"은 원논문의 FFT 응력법 선행문헌(Morales‑Espejel 등)을 가리킨다.

## A1.4 ④ 응력장 → 다축 피로 기준

| 기준 | 식 | RCF 적용성 |
|---|---|---|
| **Dang Van** | $\tau_a(t)+a\,\sigma_h(t)\le b$ | 표면/마이크로피팅 적합(비례경로↔SKF GBLM 표면항). 단 **아표면 RCF엔 비보수적** |
| **Crossland** | $\sqrt{J_{2,a}}+a_C\,\sigma_{H,\max}\le\tau_{-1}$ | 아표면 RCF에 더 현실적 |
| **Findley** | $\max_{\text{plane}}(\tau_a+k\sigma_{n,\max})=f$ | 임계면·균열방향 |
| **L–P 직교전단** | $\ln(1/S)\propto \tau_0^c N^e V/z_0^h$ | 고전 베어링 수명 |
| **Ioannides–Harris** | $\ln(1/S)\propto N^e\!\int_V \frac{(\sigma_i-\sigma_u)^c}{z'^h}dV$ | 체적적분·피로한도(ISO 281 기반) |

**중요 경고:** Ciavarella & Monno(2010, *Tribology International* 43(11):2139–2144)는 RCF에서 **Dang Van이 정수압 압축항 때문에 비보수적(과대 허용)**, Crossland·Papadopoulos가 보수적임을 보였다 — 아표면 RCF 한계 산정 시 기준 선택이 결과를 좌우. 하중순서 효과는 Beheshti & Khonsari(2011, *Tribology International* 44(12):1620–1628)의 CDM(연속체손상)으로 포착. Desimone, Bernasconi & Beretta(2006, *Wear* 260:567–572)는 Hertz→아표면→Dang Van 레시피를 확립.

## A1.5 통합 워크플로와 정확도·한계

$$
\underbrace{\text{Masjedi–Khonsari}}_{\text{스칼라 } H,L_a}\!\rightarrow\!
\underbrace{\text{Hooke–Li / 진폭감쇠}}_{p_h,p_a\text{ 리플}}\!\rightarrow\!
\underbrace{\text{Smith–Liu + DC-FFT}}_{6\ \sigma_{ij}(x,z)}\!\rightarrow\!
\underbrace{\text{Dang Van / Crossland}}_{\text{micropitting/RCF}}
$$

이는 Morales‑Espejel & Brizmer(2011) 마이크로피팅 모델과 Beheshti & Khonsari가 실제로 쓰는 "전 수치해석 회피" 경로와 동일하다.

**한계(반드시 명시):**
1. 진폭감쇠는 **진폭 ≪ 유막**의 선형영역 — 고하중 깊은 기아·대진폭에선 정확도 저하(풀 수치해석 필요).
2. 반무한체 해석해는 **선형탄성·균질** 가정 — 잔류응력·침탄경화 구배는 별도 중첩.
3. **피로기준 선택 민감도**(Dang Van 비보수 vs Crossland 보수) — 단일값 일반화 금지, 영역별 해석.
4. 통계(가우시안) 압력은 평균, 결정론 리플은 측정조도 필요 — 목적에 맞게 선택.

## A1.6 풍력 메인베어링 적용

Cerullo(2014, *Proc. IMechE Part C* 228(12):2079–2089)는 **풍력 롤러베어링에 EHL 압력 + Dang Van**을 적용해 잔류응력·경도구배를 반영, 아표면 개시를 예측했다 — 본 레짐에 직접 부합하는 선례. 본문 10장(저속·고하중·그리스)과 결합하면, 고속 파이프라인은 **운전조건 스윕(수천 케이스)에서 마이크로피팅·표면응력 위험을 빠르게 스크리닝**하는 실용 도구가 된다(풀 수치 EHL은 검증용 소수 케이스에 한정).

## A1.7 부록 1 참고문헌

**회귀식·스칼라**
- [B1] Masjedi, M., Khonsari, M. M. (2012). Film Thickness and Asperity Load Formulas for Line-Contact EHL With Provision for Surface Roughness. *ASME J. Tribology* 134(1):011503. DOI 10.1115/1.4005514.
- [B2] Masjedi, M., Khonsari, M. M. (2015). On the effect of surface roughness in point-contact EHL: Formulas for film thickness and asperity load. *Tribology International* 82(A):228–244.
- [B3] Masjedi, M., Khonsari, M. M. (2015). On the prediction of steady-state wear rate in spur gears. *Wear* 342–343:234–243.

**상세 압력분포(하중분담/진폭감쇠)**
- [B4] Johnson, K. L., Greenwood, J. A., Poon, S. Y. (1972). A simple theory of asperity contact in EHL. *Wear* 19(1):91–108.
- [B5] Gelinck, E. R. M., Schipper, D. J. (2000). Calculation of Stribeck curves for line contacts. *Tribology International* 33(3–4):175–181.
- [B6] Akchurin, A., Bosman, R., Lugt, P. M., van Drogen, M. (2015). …friction coefficient in mixed lubrication based on load-sharing… *Tribology Letters* 59:19.
- [B7] Greenwood, J. A., Morales‑Espejel, G. E. (1994). The behaviour of transverse roughness in EHL contacts. *Proc. IMechE Part J* 208(2):121–132.
- [B8] Venner, C. H., Lubrecht, A. A. (1999). Amplitude reduction of non-isotropic harmonic patterns… *Tribology Series* 36:151–162. / Hooke, C. J., Venner, C. H. (2000). Surface roughness attenuation… *Proc. IMechE Part J* 214(5):439–444.
- [B9] Hooke, C. J., Li, K. Y. (2006). Rapid calculation of the pressures and clearances in rough EHL contacts… Part 1. *Proc. IMechE Part C* 220(6):901–914.
- [B10] Westergaard, H. M. (1939). Bearing Pressures and Cracks. *J. Appl. Mech.* 6:A49–A53. `[VERIFY]` 페이지.
- [B11] Johnson, K. L., Greenwood, J. A., Higginson, J. G. (1985). The contact of elastic regular wavy surfaces. *Int. J. Mech. Sci.* 27(6):383–396. `[VERIFY]` 페이지.
- [B12] Morales‑Espejel, G. E. (2014). Surface roughness effects in EHL: A review… *Proc. IMechE Part J* 228(11):1217–1242.

**압력→응력(해석·DC-FFT)**
- [B13] McEwen, E. (1949). Stresses in Elastic Cylinders in Contact Along a Generatrix. *Phil. Mag.* Ser.7, 40.
- [B14] Smith, J. O., Liu, C. K. (1953). Stresses Due to Tangential and Normal Loads on an Elastic Solid… *J. Appl. Mech.* 20(2):157–166.
- [B15] Johnson, K. L. (1985). *Contact Mechanics.* Cambridge Univ. Press. (Ch.2–4)
- [B16] Liu, S., Wang, Q., Liu, G. (2000). A versatile method of discrete convolution and FFT (DC-FFT)… *Wear* 243(1–2):101–111.
- [B17] Liu, S., Wang, Q. (2002). Studying Contact Stress Fields Caused by Surface Tractions With DC-FFT. *ASME J. Tribology* 124(1):36–45.
- [B18] Polonsky, I. A., Keer, L. M. (1999). …rough contact problems based on MLMS and CG. *Wear* 231(2):206–219.
- [B19] Brandt, A., Lubrecht, A. A. (1990). Multilevel matrix multiplication and fast solution of integral equations. *J. Comput. Phys.* 90(2):348–370.
- [B20] Nikas, G. K. (2006). Boussinesq–Cerruti functions and… acceleration of subsurface stress computations. *Proc. IMechE Part J* 220(1):19–28.

**응력집중·피로기준**
- [B21] Hamilton, G. M., Goodman, L. E. (1966). The Stress Field Created by a Circular Sliding Contact. *J. Appl. Mech.* 33(2):371–376.
- [B22] Webster, M. N., Sayles, R. S. (1986). A Numerical Model for the Elastic Frictionless Contact of Real Rough Surfaces. *ASME J. Tribology* 108(3):314–320.
- [B23] Dang Van, K. (1993). Macro-Micro Approach in High-Cycle Multiaxial Fatigue. *ASTM STP 1191*:120–130.
- [B24] Crossland, B. (1956). Effect of large hydrostatic pressures on the torsional fatigue strength… *Proc. IMechE Int. Conf. Fatigue of Metals*:138–149.
- [B25] Findley, W. N. (1959). A theory for the effect of mean stress on fatigue… *J. Eng. Ind. (Trans. ASME)* 81:301–306.
- [B26] Ciavarella, M., Monno, F. (2010). A comparison of multiaxial fatigue criteria as applied to RCF. *Tribology International* 43(11):2139–2144.
- [B27] Beheshti, A., Khonsari, M. M. (2011). On the prediction of fatigue crack initiation in rolling/sliding contacts… *Tribology International* 44(12):1620–1628.
- [B28] Desimone, H., Bernasconi, A., Beretta, S. (2006). On the application of Dang Van criterion to RCF. *Wear* 260(4–5):567–572.
- [B29] Cerullo, M. (2014). Application of Dang Van criterion to RCF in wind turbine roller bearings under EHL. *Proc. IMechE Part C* 228(12):2079–2089.

> `[VERIFY]` 다수는 publisher 본문 직접 열람이 차단되어 abstract·메타데이터로 교차확인된 항목(서지 신뢰도 높음, 수식 내부값은 원문 대조 권장). 특히 Venner–Lubrecht $\nabla$ 정의식, Westergaard/JGH 페이지, Dang Van $a,b$ 보정상수는 원문 확인 필요.

---

# 부록 2. 돌기하중비 $L_a$ 기반 혼합윤활 직접설계 — 가능성·한계와 방향성 (실증 검토)

> **검토 대상 가설:** "MK(2012) 회귀식으로 계산되는 돌기하중비 $L_a$ 는, 점도비 $\kappa$ 로 윤활 레짐을 *간접* 판단하는 ISO 281보다 혼합윤활을 더 **직접** 분석할 수 있다. 단 $L_a$–수명 관계·$\kappa$ 관계·threshold 설정에 충분한 공학적 근거·백데이터가 필요하며, 이는 메인베어링 설계연구에서 미개척으로 보인다."
> **검토 방식:** 1차 문헌 실증 조사(3축 병렬) 기반 객관 판정 + 비판 + 방향성 제안.

## A2.1 가설별 실증 판정

| 가설 | 판정 | 핵심 근거 |
|---|---|---|
| ① $L_a$ 가 $\kappa$ 보다 혼합윤활을 더 **직접** 진단 | 🟧 대체로 맞음(조건부) | Spies, Parab & Fatemi(2025, *Tribology International* 211:110812): **동일 $\kappa/\Lambda$ 라도 조도 분포·lay에 따라 하중분담·마찰이 크게 다름** → 단일 스칼라($\kappa/\Lambda$) 부적절, $L_a$ 가 더 물리적 |
| ② $L_a$–수명·$\kappa$·threshold에 **백데이터 필요** | ✅ 강하게 확인(핵심약점 정확) | 문헌에 **$L_a$ 임계값("$L_a<X\%$ 안전") 전무**. $L_a$–수명은 항상 *간접*(wear/micropitting 모델 경유) |
| ③ 메인베어링 설계에서 **미개척** | ✅ 확인 | ISO 281/16281은 **$\kappa$ 만** 사용. Hart et al.(2022 Part2)은 MK식을 **유막두께 용도로만** 쓰고 $L_a$ 는 계산조차 안 함(λ에서 멈춤). GBLM도 λ/표면distress이지 $L_a$ 회귀 아님 |

## A2.2 반드시 짚을 비판 포인트 (객관성)

**(A) $L_a$ 와 $\kappa$ 는 독립이 아니라 강상관 → 이중계산 위험.** $\kappa\approx\Lambda^{1.3}$ (ISO/TR 1281-2 통상 인용, `[VERIFY]`)이고 $L_a$ 도 같은 유막/조도 물리의 단조함수다. 따라서 **$\kappa\!\to\!a_{\text{ISO}}$(수명) + $L_a$(threshold)를 함께 쓰면 동일 물리를 이중 반영**한다(본문 9장 비중복 원칙과 동일). $L_a$ 는 "$\kappa$ 를 대체하는 더 나은 수명지표"가 아니다 — 미검증이며 중복.

**(B) MK의 $L_a$ 도 σ̄ 기반 "평균 스칼라" — 완전한 직접측정 아님.** MK(2012) $L_a$ 는 측정 3D 위상이 아니라 무차원조도 $\bar\sigma$ 하나로 산출한 통계평균(부록1 A1.0-보론). Spies(2025)가 **동일 $\sigma$ 라도 분포·lay에 따라 달라짐**을 보였으므로, MK의 $L_a$ 는 "$\kappa$ 보다는 직접적이나 결정론(Akchurin 2015; Zhu–Wang)보다는 간접적"인 **중간 위치**다.

**(C) $L_a$→수명 threshold는 실증 부재 — 유일한 검증 임계는 여전히 $\Lambda$.** $\Lambda\gtrsim3$ 완전유막·$\Lambda\lesssim1$ 표면개시 손상이라는 **$\Lambda$ 임계만 ISO 281 $a_{\text{ISO}}$ 에 실증 내장**돼 있다. $L_a$ 임계는 누구도 보정한 바 없다.

## A2.3 핵심 재프레이밍 — $L_a$ 의 "진짜 자리"

> **$L_a$ 를 아표면 수명($a_{\text{ISO}}$) 대체로 쓰지 말고, $a_{\text{ISO}}$ 가 구조적으로 못 잡는 표면개시 손상(마이크로피팅·마모·스미어링)의 입력·threshold로 써라.**

$a_{\text{ISO}}$ 는 아표면 피로를 이미 잘 다룬다($\kappa$ 로). $L_a$ 의 고유가치는 **표면손상축**에 있고, 실증 선례가 존재한다:
- **Moallem, Akbarzadeh & Ariaei(2016, *Proc. IMechE Part J* 230(5):591–599):** 하중분담($L_a$ 계열)→마이크로피팅 수명, **FZG 시험 대비 ~97% 일치**. → $L_a$→표면수명의 실증 앵커.

즉 본 접근은 본문의 "관심사의 분리"(1·9장)와 정확히 합치하며, **$L_a$ 는 표면손상 정량화의 게이트웨이**로 자리매김할 때 가장 강력하다.

## A2.4 방향성 제안 (구체적·신규 기여)

**(1) $\Lambda$-임계 → $L_a$-임계 변환 (백데이터 부재의 우회로) ★즉시 실행 가능한 신규 기여**
검증된 $\Lambda=1,\ \Lambda=3$ 경계를 MK식에 대입해 **본 메인베어링 운전영역의 등가 $L_a$ 임계를 역산**한다(예: 주어진 $W,U,G,\bar\sigma,V$ 에서 $\Lambda=3\leftrightarrow L_a\approx?\%$, $\Lambda=1\leftrightarrow L_a\approx?\%$). → 기존 $\Lambda$ 의 실증 백데이터를 $L_a$ 언어로 번역하므로 방어 가능하면서 $L_a$ 의 직접 진단성을 활용.

**(2) MK 회귀식 적용범위 검증 (필수 선결).** MK식은 특정 $W,U,G,\bar\sigma,V$ 범위에서 피팅. 메인베어링은 **초저속($U$ 매우 작음)·대형($\bar\sigma$ 큼)** 이라 외삽 위험 → 운전영역이 피팅범위 내인지 먼저 확인(미확인 시 결과 신뢰불가).

**(3) $L_a$ → 표면손상 모델 연계 (수명은 간접).** $L_a$ 를 $a_{\text{ISO}}$ 재보정이 아니라 **micropitting(Moallem 2016)·마모(Masjedi–Khonsari 2015 *Wear* 342–343:234–243)·스미어링 입력**으로 연결. 본문 5~8장 + 부록1 파이프라인과 직결.

**(4) 이중계산 회피 규약 명시.** "$\kappa\!\to\!a_{\text{ISO}}$(아표면) **또는** $L_a\!\to\!$표면손상" 택일. 동일 물리 중복 금지(9장 원칙 승계).

**(5) 백데이터 전략.** 직접 $L_a$–수명 데이터셋 없음 → (a) $\Lambda$-기반 ISO 281 $a_{\text{ISO}}$ 를 백데이터로 차용(=(1)), (b) 부록1 결정론 Mixed EHL+Dang Van으로 자체 생성하여 MK의 $L_a$ 와 교차검증.

## A2.5 종합 판단

사용자 통찰은 **실증적으로 지지되며 진짜 연구공백을 짚었다**(특히 ②·③). 다만 가장 강한 형태는 "$L_a$ 로 $\kappa$ 를 대체"가 아니라:

> **"$\kappa$(아표면 수명)는 유지하되, $L_a$ 를 표면손상축의 직접 진단·임계 지표로 신규 도입하고, 그 임계는 검증된 $\Lambda$-경계를 MK식으로 번역해 정초한다."**

이 프레이밍이면 (a) 이중계산 회피, (b) 실증 백데이터($\Lambda$·$a_{\text{ISO}}$·Moallem) 활용, (c) 메인베어링에서 실제로 미개척인 **표면손상-직접설계** 공백을 메우는 명확한 신규 기여가 된다.

## A2.6 부록 2 참고문헌

- [C1] Spies, C., Parab, Y., Fatemi, A. (2025). Mixed friction: Critical assessment of engineering load sharing equations based on the Lambda-parameter. *Tribology International* 211:110812. — 동일 $\kappa/\Lambda$ 라도 분포·lay 따라 하중분담 상이(κ/λ 단일스칼라 한계 실증).
- [C2] Zhu, D., Wang, Q. J. (2012). On the λ ratio range of mixed lubrication. *Proc. IMechE Part J* 226(11):1010–1022. DOI 10.1177/1350650112461867. — 고전 λ 경계의 정량적 부정확성.
- [C3] Moallem, H., Akbarzadeh, S., Ariaei, A. (2016). Prediction of micropitting life in spur gears operating under mixed-lubrication regime using load-sharing concept. *Proc. IMechE Part J* 230(5):591–599. DOI 10.1177/1350650115607896. — 하중분담→마이크로피팅 수명, FZG 대비 ~97% 일치(L_a→표면수명 실증 앵커).
- [C4] Masjedi, M., Khonsari, M. M. (2015). On the prediction of steady-state wear rate in spur gears. *Wear* 342–343:234–243. DOI 10.1016/j.wear.2015.08.010. — $L_a$ → 마모율(임계값은 제시 안 함).
- [C5] Hart, E., de Mello, E., Dwyer-Joyce, R. (2022). Wind turbine main-bearing lubrication Part 2. *Wind Energy Science* 7:1533–1550. — MK식을 유막두께 용도로만 사용, $L_a$ 미산출(미개척 근거).
- [C6] Kenworthy, J. et al. (2024). Wind turbine main bearing rating lives… ISO 281: A critical review. *Wind Energy* 27(2):179–197. DOI 10.1002/we.2883. — 메인베어링 수명은 κ 기반 $a_{\text{ISO}}$ 만 사용.
- [C7] Morales‑Espejel, G. E., Gabelli, A., de Vries, A. J. C. (2015). A Model for Rolling Bearing Life with Surface and Subsurface Survival. *Tribology Transactions* 58(5):894–906. — GBLM은 표면항을 λ/표면distress로 다룸($L_a$ 회귀 아님).
- [C8] ISO 281:2007; ISO/TR 1281‑2:2008; ISO 16281:2025 — κ만 수용, 직접 돌기·하중분담 지표 부재.
- (본문 참조) Masjedi & Khonsari 2012 [B1]; Akchurin et al. 2015 [B6]; Hu–Zhu 2000.

> **검증 메모:** $\kappa\approx\Lambda^{1.3}$ 지수는 secondary 다수 인용이나 ISO 1차 원문 미추출(`[VERIFY]`). Moallem 2016의 ~97% 일치·Spies 2025의 분포 의존성은 abstract/open-text로 확인. $L_a$ 임계 부재는 조사범위 내 일관 확인(부재 입증의 한계상 "조사범위 내 미발견").

