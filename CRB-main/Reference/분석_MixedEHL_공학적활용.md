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

### 2.6 [보론] 두 접근의 갈림길 — 통계 평균유동(2.1–2.2) vs 일반 Reynolds+진폭저감

거칠기를 Mixed EHL에 넣는 방법은 크게 둘로 갈린다. **(A) 2.1–2.2의 통계적 평균유동+통계돌기**와, **(B) Morales‑Espejel/SKF 마이크로피팅 모델(2011)이 쓰는 일반 Reynolds+진폭저감(amplitude reduction)**. 둘은 **$h^3$ 비선형을 처리하는 방식**과 **출력의 성격**이 근본적으로 다르다.

**(B)의 지배식 — 소진폭 선형화된 Reynolds (SKF 2011, 식 [2]):**
Hertz 중심부에서 미시형상 진폭이 유막보다 훨씬 작으면, Reynolds 식의 **변동량 곱(fluctuation product: $\delta p\!\cdot\!\delta h$)과 도함수 곱을 무시**할 수 있어 식이 **선형**이 된다:
$$
\frac{h^3}{12}\left(\frac{1}{\eta_x}\frac{\partial^2 p}{\partial x^2}+\frac{1}{\eta_y}\frac{\partial^2 p}{\partial y^2}\right)=\bar u\frac{\partial h}{\partial x}+\frac{\partial h}{\partial t}+\frac{h}{B}\left(\bar u\frac{\partial p}{\partial x}+\frac{\partial p}{\partial t}\right)\tag{[2]}
$$
($B$=윤활유 체적탄성계수, $d\rho/dp=\rho/B$). 정현 조도 1성분에 대해 압력 리플 진폭이 닫힌형으로 나온다(식 [5]):
$$
\frac{p_a}{r_a}=\frac{\kappa E'}{4}\,\frac{iQ}{1-iQ-iCQ}
$$
Greenwood & Morales‑Espejel(1994)에 따르면 이동 조도 문제의 해는 **두 성분**: ① **particular integral**(미시형상이 변형된 채 조도면 속도 $u_2$ 로 이동) + ② **complementary function**(입구에서 생성돼 평균 이송속도 $\bar u$ 로 전파되는 파). 즉 **파동 전파** 그림이다. 이 선형화가 바로 **푸리에 중첩·전달함수·FFT**를 가능케 한다(부록1 A1.2(b)·A1.3-보론).

**핵심 대비:**

| 항목 | (A) 평균유동+통계돌기 (2.1–2.2) | (B) 일반 Reynolds+진폭저감 (식 [2]) |
|---|---|---|
| 거칠기 처리 | 통계적 **평균화**(flow factor $\phi$, 높이분포 $\phi(z)$) — 미해상 | 결정론 **미시형상 해상**(푸리에 하모닉) |
| $h^3$ 비선형 | 완전 비선형 유지 → **앙상블 평균**으로 flow factor에 흡수 | **소진폭 선형화**(변동량 곱 무시) |
| 출력 압력 | 평균 $\bar p,\ p_a$ — **매끈(첨두 없음)** | **결정론 압력 리플(스파이크)** |
| 돌기접촉 | 포함(GW/GT 경계·혼합·완전유막) | **미포함**(완전유막·고 $\Lambda$) → **결합모델(B+)로 확장** |
| 대표 목적 | 하중분담비·마찰·Stribeck·평균유막 | **국부 압력·표면응력·마이크로피팅** |
| 물리 그림 | 앙상블 평균 → 스파이크 소멸 | 파동 전파(이송파+입구파) → 스파이크 보존 |
| 속도/한계 | 전 레짐, 그러나 국부 스파이크 없음 | 빠름(FFT), 그러나 소진폭·완전유막·(기본)Newtonian |

**수리적 요점(부록3 A3.7.1과 연결):** 윤활은 $h^3$ 때문에 강한 비선형이라, **(A)는 평균만 닫히고 (B)는 소진폭 선형화만 닫힌다.** (A)는 평균이라 스파이크가 지워지고, (B)는 결정론 섭동이라 스파이크가 살지만 **소진폭·완전유막**에 갇힌다. **스파이크와 돌기접촉을 동시에** 원하면 어느 쪽도 아니고 **2.3의 결정론 통합 Reynolds(Hu–Zhu)** 가 필요하다(대신 느림).

**직관·용도 선택:**
- "평균적으로 거칠기가 유막·하중분담을 어떻게 바꾸나?" → **(A)** (마찰·레짐·수명평균, 경계접촉 포함).
- "이 거칠기가 굴러 지나갈 때 실제 압력 물결이 어떻게 생기고 전파되나?" → **(B)** (국부 응력·마이크로피팅, 고 $\Lambda$·소진폭). *진폭저감 = 유막압력이 조도 마루를 탄성적으로 눌러 평탄화하는 정도.*
- "저 $\Lambda$·큰 진폭에서 스파이크+접촉 동시" → **2.3 Hu–Zhu 통합 Reynolds** 또는 아래 **(B+) 결합모델**.

**(B+) 부분윤활 결합모델 (Combined Model for Partial Lubrication) — (B)로 돌기접촉을 다루는 법 (SKF 2011):**

**동기(왜 필요한가):** Morales‑Espejel et al.(2011)이 지적하듯, 표면속도가 0이 아니고 벽면 slip이 없으면 **Reynolds 식은 원리적으로 dry contact(film breakdown)를 만들 수 없다.** 진폭저감(B) 단독으로는 파장이 길고 유막이 얇을수록 조도가 **무한정 탄성 평탄화**되어(비물리적) 실제 돌기접촉을 못 만든다. → **매우 얇은 간극 영역은 연속체(윤활) 대신 건접촉(dry contact) 모델로 푸는 것이 오차가 적다.** SKF는 이를 **건접촉 모델(접촉 spot)+윤활 모델(완전유막)의 결합**으로 해결한다.

**① 하중분담 알고리즘(Johnson et al., 질량보존 반복):**
- 매끈면 중심유막 $\bar h$ 를 EHL 식으로, 초기 평균압=최대 Hertz $p_h$ 로 둔다.
- **건(dry) 돌기 분담 하중비 $\phi_{bl}$** 가정 → 나머지는 윤활 spot. 건·윤활 모델로 국부압·간극 계산 → 새 건/윤활 영역 식별 → **하중분담 수렴까지 반복**(유량 balance 유지, 변형 간극을 상하로 이동).
- 압축성: 윤활 분담비 변화에 따라 중심유막을 **압축성 보정계수 $c_p$** 로 조정.

**② 건/윤활 patch 식별 + 천이 간극·압력(주파수영역 max):** 단순히 윤활 간극에서 접촉 spot을 건접촉해로 치환하면 (B)의 "연속 평탄화" 문제가 남는다 → 조도 변형은 **건접촉 문제로 제한**돼야 한다. 장파장 성분이 진폭이 커 접촉하기 쉬우므로, **FFT 후 주파수 성분별로 건/윤활 간극을 비교해 절댓값 최대를 선택:**
$$
h_{tran(i,j)}=\mathrm{IFFT}\{\max(|\tilde h_{dry(i,j)}|,\ |\tilde h_{lub(i,j)}|)\}
$$
대응 압력은 탄성변위로부터 식 [1]의 역과정으로 복원($w$=주파수응답함수 FRF, $\mathbf r-\mathbf h_{tran}=\mathbf u$):
$$
p_{tran}=\mathrm{IFFT}\{\,w^{-1}\cdot\mathrm{FFT}(\mathbf r-\mathbf h_{tran})\,\}
$$
소성 발생 시 $p_{tran}\le p_{lim}$, 캐비테이션(음압) 시 $p_{tran}=0$(고진폭 미시형상 회피로 오차 최소화).

**③ 두 조도면·마찰:** 두 조도면이면 건접촉은 두 면으로, 윤활은 각 면 반복 → 총 유체압 $p_{lub}=p_{lub1}+p_{lub2}$. 표면 트랙션 $q(x,y)=\mu(x,y)\,p(x,y)$, $\mu$=완전유막 $\mu_{ehl}$ 또는 경계 $\mu_{bl}$.

**위치 정리(3-way 통합):**
- (B+)는 **(B)의 결정론 리플을 유지하면서 건접촉·하중분담을 얹어** (B)의 "돌기접촉 미포함" 한계를 메운다 = **결정론 하이브리드**.
- (A)의 하중분담은 **통계**($\phi(z)$·flow factor·GT 평균)인 반면, (B+)는 **결정론**(측정조도 건접촉해 + 진폭저감 리플 + Johnson 질량보존 + 주파수 max).
- 2.3 Hu–Zhu는 **단일 방정식**(monolithic, $h\to0$ 자동 축퇴)으로 같은 목표를 이루지만, (B+)는 **두 모델(건+윤활)을 하중분담·주파수 max로 접합한 모듈형 하이브리드** — FFT 기반이라 빠르나 근사(연속체 대신 건접촉으로 얇은 간극을 근사).

> **연결:** (B)의 진폭저감은 sliding/비뉴턴에서 Hooke(2006) 스킴으로 확장(SKF 2011); (B+) 결합모델이 그 위에 건접촉·하중분담을 얹은 완성형이다. 이 골격이 **부록1 A1.2(b)·A1.3-보론(식 12·13)** 과 **부록3 A3.7(C: 스펙트럼 유막압력 리플)** 의 기반이며, **부록1 A1.5의 결정론 파이프라인**과도 직결된다. (A)는 **부록3 A3.7(A·B)** 및 **부록2($L_a$)** 와 연결된다.

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

---

# 부록 3. 확률과정/스펙트럼 접근 — 가우시안 조도에서 압력·6응력성분을 통계적으로 계산할 수 있는가

> **검토 가설:** "MK(2012)가 거칠기를 가우시안으로 모델링해 돌기하중비를 계산하듯, 거칠기가 가우시안이면 (i) 돌기하중비·(ii) 압력분포를 **통계적으로 명시 표현**할 수 있고, 나아가 (iii) 표면피로에 필요한 **6자유도 표면/표면하 응력**도 통계적으로 계산할 수 있지 않은가?"
> **검토 방식:** 수리·물리 분석 + 병렬 문헌조사(2축). 결론: **타당하며, "확률과정/스펙트럼 접촉역학(stochastic/spectral contact mechanics)"으로 이미 정립.** 단 명제별 성립 조건·한계가 다름.

## A3.1 가설의 3분해와 판정

### 명제 1. 가우시안 → 돌기하중비 통계 명시 — ✅ (단 '평균')

MK의 $L_a$ 는 이미 **통계적 기댓값**이다. Greenwood–Williamson(1966)/Greenwood–Tripp(1970)에서 돌기하중은 가우시안 높이분포 $\phi(z)$ 의 앙상블 기댓값:

$$
W_a=\tfrac{4}{3}E'N\sqrt{\beta}\!\int_d^\infty(z-d)^{3/2}\phi(z)\,dz\propto F_{3/2}(\lambda),\quad
F_n(\lambda)=\tfrac{1}{\sqrt{2\pi}}\!\int_\lambda^\infty(s-\lambda)^n e^{-s^2/2}ds=\mathbb{E}[(s-\lambda)^n;s>\lambda]
$$

즉 $L_a$ 는 분포값이 아니라 분포의 **1차 모멘트(평균)** 가 닫힌형으로 나온 것. 실현치별 $L_a$ 의 분산도 원리상 계산 가능하나 MK는 평균만 제공.

### 명제 2. 가우시안 → 압력분포 통계 표현 — 🟧 부분적 ✅ (레짐 의존)

결정적 갈림길이 있다(Persson 2001; Yastrebov et al. 2015):

- **완전접촉/소거칠기(선형) 레짐:** 압력이 표면높이의 **선형 범함수** → 압력장도 **가우시안 랜덤필드**. $p(q)=\tfrac12E^*q\,h(q)$ (Persson 2008, Eq.4) → 압력 PSD $C_p(q)=\tfrac14E^{*2}q^2C(q)$, 분산 $\langle p^2\rangle\propto\int q^3C(q)dq$ (=rms 기울기).
- **부분접촉(현실) 레짐:** $p\ge0$ 제약(인장 불가)으로 압력은 **비가우시안·단측(one-sided)**. Persson은 **확산(Fokker–Planck) 방정식**으로 압력확률분포 $P(p,\zeta)$ 를 준해석적으로 제공:
$$
\frac{\partial P_0}{\partial\zeta}=f(\zeta)\frac{\partial^2P_0}{\partial p^2},\quad
f(\zeta)=\frac{1}{8\zeta}\!\int q^3C(q)dq,\quad P_0(0,\zeta)=0\ (\text{무인장})
$$

즉 **전체 압력분포는 통계 표현 가능**하나 가우시안이 아니라 Persson 이론으로. MK/GW의 매끈한 $P_a$ 는 이 분포의 **평균**일 뿐(부록1 A1.0-보론과 일치).

### 명제 3. 가우시안 → 6응력성분 통계 계산 — ✅ (가장 강력히 성립)

**핵심.** 응력은 압력의 **선형 범함수**이고, 파수 $q=\sqrt{\alpha^2+\beta^2}$ 의 각 압력 하모닉은 깊이로 $(a+bz)e^{-qz}$ 커널(=부록1 A1.3-보론의 Morales‑Espejel 식 12·13 = Westergaard/Johnson)로 전파된다. 압력이 PSD $C_p(q)$ 인 랜덤필드면 각 응력성분도 랜덤필드이고:

$$
\boxed{\;\mathrm{Var}[\sigma_{ij}(z)]=\int|H_{ij}(q,z)|^2\,C_p(q)\,d^2q,\quad
\mathrm{Cov}[\sigma_{ij},\sigma_{kl}](z)=\int H_{ij}H_{kl}^*\,C_p\,d^2q\;}
$$

→ **6성분 응력장의 2차 통계(평균+분산+공분산)가 깊이 $z$ 마다 닫힌형 스펙트럼 적분**으로. 결정론 실현 불필요. 이미 수행된 연구:
- **Müser(2018, *JMPS* 119:73–82):** 랜덤조도 아래 **von Mises 분산·분포를 깊이의 함수로** 계산. 최댓값이 접촉부 바로 아래(근표면), 감쇠길이는 **높이차 자기상관 포화거리**($q$ 비례 지수감쇠 → 단파장=근표면).
- **Persson(2008, *JPCM* 20:312001):** 계면 응력상관 $\langle\sigma(q)\sigma(-q)\rangle=A_0(E^*/4\pi)^2q^2C(q)P(q)$ — "stress PSD = |필터|²×조도 PSD" 자체.
- **Persson(2023, *Tribology Letters* 71:115):** **전체 분포(가우시안)+극값(extreme-value) 첨두**. $\sigma_{rms}\propto$ rms기울기 $\xi$, $\sigma_{\max}\approx\sqrt{2\ln N}\,\sigma_{rms}$ (전형적으로 작용응력 ~10배).

## A3.2 통합 수리 골격 (가설의 엄밀한 형태)

$$
\underbrace{C(q)}_{\text{조도 PSD}}\xrightarrow[\text{(EHL 시 진폭감쇠 }T(q))]{p(q)=\frac12E^*q\,h(q)}
\underbrace{C_p(q)}_{\text{압력 PSD}}\xrightarrow[H_{ij}(q,z)=(a+bz)e^{-qz}]{\text{식(12,13) 커널}}
\underbrace{\mathrm{Var}[\sigma_{ij}(z)]}_{\text{6성분 응력 통계}}\xrightarrow[\text{Rice/극값}]{}\sigma_{\max}\to\text{피로}
$$

가우시안 조도 → (선형/EHL 전달함수) → 압력 PSD → (반무한체 커널) → 6응력 통계 → 극값 → 피로. **전 과정이 PSD의 닫힌형 적분.**

## A3.3 결정적 한계 (가설이 깨지는 지점)

1. **비가우시안 압력 = 핵심 약점.** $p\ge0$ 로 부분접촉에서 압력 비가우시안. 깔끔한 가우시안-선형 응력통계는 **완전접촉/소거칠기 근처에서만** 정확하고, **$L_a$ 가 커질수록(혼합윤활 심화) 정확도 저하** → Persson 비선형 이론 필요. 하필 메인베어링 중요 영역에서 가장 부정확.
2. **피로엔 분산이 아니라 극값.** 표면피로는 **첨두응력**이 구동 → 극값통계(Rice/peak) 한 겹 더 필요(Persson 2023). 가우시안 꼬리가 실제 꼬리를 과소/과대평가 가능.
3. **다축·비례경로(Dang Van).** 6성분은 상관 랜덤필드이고 굴림 진행 **응력이력(비비례경로)** 필요 → 공분산행렬+시간상관까지 있어야 Dang Van을 닫음.
4. **소성 절단.** 돌기 첨두는 항복으로 잘림 → 선형탄성 극값은 과대.

## A3.4 문헌 지도

| 단계 | 통계량 | 핵심 문헌 |
|---|---|---|
| 조도 랜덤과정 | 스펙트럼 모멘트 $m_0,m_2,m_4$ | Nayak(1971); Longuet‑Higgins(1957) |
| 돌기하중=기댓값 | 평균 | GW(1966), GT(1970) |
| 압력 **분포** | 비가우시안 PDF(확산방정식) | Persson(2001); Manners‑Greenwood(2006) |
| 계면 응력상관 | 분산(PSD 적분) | Persson(2008) |
| 표면하 응력 vs 깊이 | von Mises **분산·분포** | **Müser(2018)** |
| 응력 **분포+극값** | 전체분포+첨두 | **Persson(2023)** |
| 깊이 커널 | 결정론 하모닉 | Westergaard(1939)/Johnson(1985)/M‑E 식12·13 |

## A3.5 종합 판단 + 연구 공백(기회)

- 가설은 수리·물리적으로 타당하며 "스펙트럼 접촉역학"으로 이미 정립. 특히 **명제 3(6응력 통계)은 Müser 2018·Persson 2008/2023으로 직접 입증**.
- 단 **MK의 $L_a$ 는 평균일 뿐**, 완전한 통계압력은 Persson, 응력통계는 PSD+커널 적분이 담당.
- **결정적 발견(연구 공백):** 두 조사 모두 **"스펙트럼-통계 응력 → 다축피로(Dang Van) → 마이크로피팅"을 결합한 모델은 문헌에 없다**고 확인. 기존 마이크로피팅 모델은 전부 **결정론(Morales‑Espejel–Brizmer)**. 즉 **"가우시안 PSD → 통계 6응력 → 통계 표면피로" 파이프라인은 미발표 공백 = 신규 기여 가능 영역**.

> **한 줄 결론:** "가우시안 → 통계 압력·응력"은 **완전접촉/선형 근처에서 닫힌형 성립**(Persson·Müser), 그러나 **혼합윤활의 비가우시안성과 피로의 극값성**이 두 관문. 이를 극값통계+EHL 전달함수로 연결하면 **결정론 수치해석을 대체하는 "통계적 표면피로 스크리닝"** 이 가능 — 현재 **미개척 연구주제**. (부록1 결정론 파이프라인과 상보: 결정론=검증·소수케이스, 통계=고속 스크리닝·다수케이스.)

## A3.6 부록 3 참고문헌

- [D1] Nayak, P. R. (1971). Random Process Model of Rough Surfaces. *J. Lubrication Technology (Trans. ASME)* 93(3):398–407. DOI 10.1115/1.3451608.
- [D2] Longuet‑Higgins, M. S. (1957). Statistical properties of an isotropic random surface. *Phil. Trans. R. Soc. A* 250:157–174. `[VERIFY]`.
- [D3] Greenwood, J. A., Williamson, J. B. P. (1966). Contact of Nominally Flat Surfaces. *Proc. R. Soc. A* 295(1442):300–319.
- [D4] Greenwood, J. A., Tripp, J. H. (1970/71). The Contact of Two Nominally Flat Rough Surfaces. *Proc. IMechE* 185(1):625–633.
- [D5] Persson, B. N. J. (2001). Theory of rubber friction and contact mechanics. *J. Chem. Phys.* 115(8):3840–3861. DOI 10.1063/1.1388626.
- [D6] Persson, B. N. J., Bucher, F., Chiaia, B. (2002). Elastic contact between randomly rough surfaces… *Phys. Rev. B* 65:184106. `[VERIFY]` 페이지.
- [D7] Manners, W., Greenwood, J. A. (2006). Some observations on Persson's diffusion theory of elastic contact. *Wear* 261(5–6):600–610. DOI 10.1016/j.wear.2006.01.007.
- [D8] Yastrebov, V. A., Anciaux, G., Molinari, J.-F. (2015). From infinitesimal to full contact between rough surfaces… *Int. J. Solids Struct.* 52:83–102 (arXiv:1401.3800).
- [D9] Persson, B. N. J. (2008). On the elastic energy and stress correlation in the contact between elastic solids with randomly rough surfaces. *J. Phys.: Condens. Matter* 20:312001 (arXiv:0805.0712).
- [D10] **Müser, M. H. (2018).** Internal, elastic stresses below randomly rough contacts. *J. Mech. Phys. Solids* 119:73–82. DOI 10.1016/j.jmps.2018.06.012. (단독저자 — "Müser & Dapp" 아님)
- [D11] **Persson, B. N. J. (2023).** Surface roughness induced stress concentration. *Tribology Letters* 71:115. DOI 10.1007/s11249-023-01741-4 (arXiv:2304.02159).
- [D12] Pohrt, R., Popov, V. L. (2012). Normal Contact Stiffness of Elastic Solids with Fractal Rough Surfaces. *Phys. Rev. Lett.* 108:104301.
- [D13] Bush, A. W., Gibson, R. D., Thomas, T. R. (1975). The elastic contact of a rough surface. *Wear* 35(1):87–111. `[VERIFY]` 페이지.
- (본문 참조) 깊이 커널: Westergaard 1939 [B10], Johnson 1985 [B15], Morales‑Espejel 식12·13(부록1 A1.3-보론).

> **검증 메모:** Persson 2008·2023, Müser 2018, Persson 2001 확산방정식은 원문(arXiv 포함) 직접 확인. Nayak summit-PDF 정확식, Longuet‑Higgins/Bush 페이지, Persson–Bucher–Chiaia 페이지는 `[VERIFY]`. Persson erf 면적-하중식의 분모 상수는 이론버전별 상이(함수형은 견고, 상수는 soft).

---

## A3.7 [추가조사] 윤활(Mixed EHL) 조건에서의 통계화 — A3.1~A3.6의 한계와 정정

> **비판적 단서(중요):** A3.1~A3.6의 핵심 문헌(Persson·Müser·Nayak·GW/GT)은 **대부분 dry elastic contact(윤활 미고려)** 다. 실제 필요한 것은 **유막압력 $p_h$ 와 돌기압력 $p_a$ 를 함께 다루는 Mixed EHL의 통계화**이며, 이는 별개 계보다. 본 절은 그 계보를 추가조사해 A3의 적용범위를 정정한다. **결론: 윤활에서는 $h^3$ 비선형 때문에 통계화가 '평균/스펙트럼'까지만 닫히고, 완전 결합분포→응력통계는 미발표 공백.**

### A3.7.1 왜 윤활이 dry보다 본질적으로 어려운가 (핵심 물리)

- **Dry 탄성접촉:** 압력이 표면높이의 **선형 범함수** → 가우시안 → 분포가 닫힘(A3.1~A3.6, Persson/Müser).
- **윤활:** Reynolds의 Poiseuille 항이 $h^3$(횡방향 거칠기는 $1/h^3$) → **강한 비선형** → 평균연산이 비선형항과 교환 안 됨:
$$
E(h^3)\neq (E\,h)^3,\qquad E(1/h^3)\neq 1/(E\,h)^3
$$
→ **평균만 닫히고 분포는 닫히지 않는다.** 이것이 dry의 "가우시안→통계압력" 기계가 윤활에서 평균 수준에서 멈추는 근본 이유다(Christensen, Patir–Cheng가 모두 평균만 주는 이유).

### A3.7.2 윤활-통계 계보 (세 갈래, 어느 것도 완전치 않음)

**(A) 유막압력 $p_h$ 통계 — 평균만**
- **Christensen(1969, *Proc. IMechE* 184:1013–1026):** 최초의 확률(stochastic) Reynolds. 기대압력 $E(p)$ 를 거칠기 방향별로 구분 — 횡방향 $\partial_x[\,(1/E(1/h^3))\,\partial_x E(p)\,]$, 종방향 $\partial_x[\,E(h^3)\,\partial_x E(p)\,]$.
- **Patir–Cheng(1978/79):** 평균유동(flow factor)으로 앙상블 평균 유막압력 $\bar p$ (부록1 A1.2(a)와 동일).
- **Elrod(1979), Bayada–Chambat(1988), Almqvist–Dasht(2006):** 다중스케일/호모지나이제이션 → 유효(평균) 압력. ❗ 전부 **1차 모멘트(평균)만**, 분산/PDF 없음.

**(B) 돌기압력 $p_a$ 통계 — 평균(분담비)**
- Johnson–Greenwood–Poon(1972, *Wear* 19:91–108), Gelinck–Schipper(2000), **Masjedi–Khonsari(2012)**: GW/GT 통계돌기 + 평균유동 → 평균 돌기하중/분담비 $L_a$(부록2와 연결).

**(C) 스펙트럼 유막압력 리플 — 분산/PSD(2차 모멘트)**
- **Greenwood–Morales‑Espejel(1994, *Proc. IMechE C* 208:121–132), Chapkov–Venner–Lubrecht(2006, *ASME J. Tribol.* 128(4):753–760), Morales‑Espejel(2014 리뷰):** EHL 진폭감쇠 전달함수 $T(q)$ 를 표면 PSD에 적용 → 유막압력 리플 **스펙트럼**. 단 결정론-하모닉, **돌기접촉은 빠짐**.

**(D) "양쪽을 PSD에서" 최근접 — Persson–Scaraggi ★**
- **Persson & Scaraggi(2009, *J. Phys.: Condens. Matter* 21(18):185002; 2011, *Eur. Phys. J. E* 34:113):** 조도 PSD $C(q)$ 에서 flow factor + 하중분담 + Stribeck. **돌기측은 분포적($P(p,\zeta)$), 유체측은 homogenize된 평균 flow factor** → "양쪽 통계화"에 가장 근접하나 출력은 평균·분담.
- **Prajapati & Björling(2024, *Lubricants* 12(3):71):** PSD/Weibull(비가우시안) 표면을 Mixed Lubrication에 투입 — **입력 통계적·솔버 결정론·출력 평균**.

### A3.7.3 정직한 판정 — 어디까지 되고 무엇이 공백인가

| 목표 | 가능 여부 | 최선의 도구 |
|---|---|---|
| $p_h,p_a$ **평균** 통계화 | ✅ 확립 | Masjedi–Khonsari 2012 / Persson–Scaraggi |
| $p_h$ **리플 스펙트럼**(분산) | ✅ (소진폭·완전유막) | 진폭감쇠 $T(q)$ × 표면 PSD |
| $p_a$ **분포** | 🟧 부분(돌기측) | Persson–Scaraggi $P(p,\zeta)$ |
| $p_h,p_a$ **완전 결합분포** → 표면하 응력통계 | ❌ **공백(윤활판 미발표)** | 없음 — 결정론 수치해석만 |

→ **A3.1~A3.6의 통계 파이프라인은 dry에선 성립하나, 윤활(현실)에선 $h^3$ 비선형 때문에 평균/스펙트럼까지만**이 정직한 현황. dry의 Müser/Persson 응력통계에 **대응하는 윤활판은 존재하지 않음**(두 조사 일관 확인).

### A3.7.4 현실적 최선 + 신규 기여 후보

- **지금 쓸 수 있는 것:** 평균 = Masjedi–Khonsari $L_a$ + Persson–Scaraggi 하중분담; 유막 리플 응력 = **진폭감쇠 $T(q)$ × PSD × 식12·13 커널**(부록1 A1.3 골격의 *윤활판* — dry 커널 앞에 $T(q)$ 만 추가).
- **6응력 표면피로 실값:** 여전히 **부록1 결정론 Mixed EHL + FFT 응력**이 필요(통계는 스크리닝 보조).
- **신규 기여 후보(미개척):** $h^3$ 비선형을 우회하는 통계적 유막압력 분포(예: polynomial chaos, 또는 Persson–Scaraggi + 진폭감쇠 결합)로 **$p_h,p_a$ 결합분포 → 응력통계 → 표면피로**를 닫는 것.

### A3.7.5 추가조사 참고문헌

- [D14] Christensen, H. (1969–70). Stochastic Models for Hydrodynamic Lubrication of Rough Surfaces. *Proc. IMechE* 184(Pt1,No.55):1013–1026. DOI 10.1243/PIME_PROC_1969_184_074_02.
- [D15] Patir, N., Cheng, H. S. (1978). An Average Flow Model… *ASME J. Lubr. Technol.* 100(1):12–17. (1979: 101(2):220–229.) ※부록1 [B-]·본문 §2.1과 동일.
- [D16] Elrod, H. G. (1979). A General Theory for Laminar Lubrication with Reynolds Roughness. *ASME J. Lubr. Technol.* 101(1):8–14.
- [D17] Bayada, G., Chambat, M. (1988). New Models in the Theory of the Hydrodynamic Lubrication of Rough Surfaces. *ASME J. Tribology* 110(3):402–407.
- [D18] Almqvist, A., Dasht, J. (2006). The homogenization process of the Reynolds equation… *Tribology International* 39(9):994–1002.
- [D19] **Persson, B. N. J., Scaraggi, M. (2009).** On the transition from boundary lubrication to hydrodynamic lubrication in soft contacts. *J. Phys.: Condens. Matter* 21(18):185002.
- [D20] **Persson, B. N. J., Scaraggi, M. (2011).** Lubricated sliding dynamics: Flow factors and Stribeck curve. *Eur. Phys. J. E* 34:113. DOI 10.1140/epje/i2011-11113-9.
- [D21] Greenwood, J. A., Morales‑Espejel, G. E. (1994). The behaviour of transverse roughness in EHL contacts. *Proc. IMechE Part C* 208(C2):121–132.
- [D22] Chapkov, A. V., Venner, C. H., Lubrecht, A. A. (2006). Roughness Amplitude Reduction Under Non-Newtonian EHD Lubrication Conditions. *ASME J. Tribology* 128(4):753–760.
- [D23] Prajapati, D. K., Björling, M. (2024). The Influence of Non-Gaussian Roughness and Spectral Properties on Mixed Lubrication… *Lubricants* 12(3):71. DOI 10.3390/lubricants12030071.
- (본문 참조) Masjedi–Khonsari 2012 [B1]; Johnson–Greenwood–Poon, Gelinck–Schipper(부록1 [B4][B5]); Morales‑Espejel 2014 [B12].

> **검증 메모:** Persson–Scaraggi(2009/2011)·Christensen(1969)·Patir–Cheng·Prajapati–Björling(2024) 서지 확인. Christensen의 $35/32c^7$ PDF·브래킷 계수, Patir–Cheng 계수 배치는 교과서 표준형(원문 paywall, `[VERIFY]`). "윤활판 응력통계 부재"는 조사범위 내 일관(부재 입증 한계상 "미발견"). 진폭감쇠 $T(q)$ 닫힌형은 원문 대조 권장.

---

# 부록 4. 거칠기 피크 보존 모델링 → FFT 압력해석 (4축 체계 조사 + 계보도·영향도)

> **목적:** 거칠기 **피크 돌기를 보존·표현**하는 수학적 도구를 4축(A 생성 / B 비가우시안·프랙탈 / C summit·극값 / D FFT 압력)으로 체계 조사하고, **실측 베어링 표면 표현력**을 평가하며, 각 논문의 **계보·저널·인용수 기반 영향도**를 누적. 지속 연구 확장의 토대.
> **인용수 표기:** Semantic Scholar/Crossref, **수집일 2026-06-30** (Google Scholar 대비 1.5~2.5배 낮은 보수적 하한). 미확보는 `[CC?]`.
> **계획서:** `분석계획_거칠기피크모델링.md`.

## A4.1 전제 정정 (피크가 사라지는 진짜 원인)

1. **"가우시안 분포"가 아니라 "평균(기댓값) 연산"이 피크를 제거한다.** 가우시안 표면의 *한 실현(realization)* 은 높은 돌기를 포함하고, 이를 FFT-접촉에 넣으면 피크 압력이 나온다. 매끈해지는 건 앙상블 평균 $E[p]$ 를 취할 때뿐(부록3 A3.1·A3.7).
2. **단, 실제 표면은 비가우시안.** 가공·런인 표면은 음의 skewness·높은 kurtosis → 가우시안은 **피크가 모인 꼬리(tail)를 잘못 표현**. 피크 충실엔 비가우시안 marginal 필요.
3. **PSD는 표면을 유일하게 결정하지 않는다 — 위상(phase) 정보 손실.** $S(k)=|\hat z(k)|^2$ 는 위상을 버린다. 피크는 부분적으로 위상 상관(국소화)에 들어 있어, random-phase 가우시안 복원은 **피크의 공간 응집을 흩뜨린다.** → 실측 피크엔 **측정 위상** 또는 **비가우시안 marginal** 필요.

## A4.2 축 A — 표면 표현·생성 (수식)

**(A-1) 스펙트럼 합성(SRM) — 빠르나 가우시안(피크 손실):**
$$
z_{ij}=\sum_k\sum_l \sqrt{S_{kl}}\,e^{i\phi_{kl}}\,e^{i2\pi(ki/M+lj/N)},\quad \phi_{kl}\sim U[0,2\pi]
$$
random phase → 중심극한정리로 **가우시안** marginal(피크 비국소화). Newland(교과서); **Wu(2000, *Tribology Int.* 33(1):47–58, [CC 253])**.

**(A-2) 디지털 필터/ACF — 비가우시안 가능(피크 보존 트렁크):**
$$
Z=K\otimes R,\quad C=K\otimes K,\quad K=\mathrm{ifft2}\!\big(\sqrt{\mathrm{fft2}(C)}\big)
$$
백색잡음 $R$ 에 FIR 필터 $K$, 목표 ACF=$C$. **★Hu & Tønder(1992, *Int. J. Mach. Tools Manuf.* 32:83–90, [CC 345] — 본 축 keystone).**

**(A-3) Johnson 변환 — 목표 $S_{sk},S_{ku}$ 부여:**
$$
z=\gamma+\delta\,f\!\Big(\tfrac{x-\xi}{\lambda}\Big),\quad z\sim N(0,1)
$$
- $S_U$(무계): $x=\xi+\lambda\sinh\frac{z-\gamma}{\delta}$ / $S_B$(유계): 로지스틱 / $S_L$: 로그정규.
- **Johnson(1949, *Biometrika* 36:149) → Watson–Spedding(1982, *Wear* 83:215, [CC 94]) → Hu–Tønder(1992) → Bakolas(2003, *Wear* 254:546, [CC 156]).**

**(A-4) PSD+비가우시안 동시 — 순서 함정:** filter→translate는 PSD 왜곡, translate→filter는 marginal이 가우시안으로 끌림(moment 감쇠). 정확한 동시해 일반적으로 없음 → 반복/하이브리드. **Manesh(2010, *Wear* 268:1371, [CC 96]), Pawar(2013, *J.Tribol.* 135:011401, [CC 59]), Francisco–Brunetière(2016, *Proc.IMechE J* 230:747, [CC 33], 해석커널로 ACF 엄밀 일치).**

## A4.3 축 B — 비가우시안·프랙탈 (수식)

**(B-1) 프랙탈 W–M:** $z(x)=G^{D-1}\sum_n \cos(2\pi\gamma^n x)/\gamma^{(2-D)n}\ (1{<}D{<}2)$, PSD $S(\omega)\propto G^{2(D-1)}/\omega^{5-2D}$. **Berry–Lewis(1980) → Majumdar–Bhushan(1990/91, *J.Tribol.*; MB 1991 [CC 1,284]) → Yan–Komvopoulos(1998, *JAP*, 3D [CC 686]).** MB 접촉: $P\propto A_r^{(3-D)/2}$.
**(B-2) Self-affine:** $C(q)\propto q^{-2(1+H)}$, $D_f=3-H$. **Palasantzas(1993, *PRB* 48:14472); Persson(2001, *JCP* 115:3840).**
**(B-3) bi-Gaussian/stratified(plateau):** 두 가우시안 중첩, material probability curve. **Leefe(1998); ISO 13565-3:1998; Hu(2019, *Tribol.Int.* 134:427, bi-fractal).** → run-in/honed raceway.
**(B-4) Weibull height + PSD:** **Prajapati–Björling(2024, *Lubricants* 12(3):71)** — shape param이 skew 제어, 음의 skew가 유막형성 유리.
**(B-5) 실측 증거:** **Harvey et al.(2025, *Surf.Topogr.* 13(3))** — 베어링강 run-in 시 $S_{sk}{<}0,\ S_{ku}{\uparrow}$.

## A4.4 축 C — 피크·summit·극값 통계 (수식)

**(C-1) 스펙트럼 모멘트·Nayak:** $m_0$(분산),$m_2$(MS기울기),$m_4$(MS곡률), **대역폭 $\alpha=m_0m_4/m_2^2\ge1.5$**.
$$
D_{sum}=\frac{1}{6\pi\sqrt3}\frac{m_4}{m_2},\quad R=\frac{3}{8}\sqrt{\frac{\pi}{m_4}},\quad \langle\kappa\rangle=\frac{8}{3\sqrt\pi}\sqrt{m_4},\quad \sigma_{summit}=\sqrt{m_0}\sqrt{1-\frac{0.8968}{\alpha}}
$$
**Longuet-Higgins(1957, *Phil.Trans.R.Soc.A* 250:157) → Nayak(1971, *J.Lubr.Technol.* 93:398, [CC≈901]).** ★가우시안 표면이라도 **summit 높이분포는 비가우시안**(α 의존 skew).
**(C-2) 돌기접촉:** **GW(1966, *Proc.R.Soc.A* 295:300, [CC≈5,900])** $\psi=\frac{E^*}{H}\sqrt{\sigma/R}$, $A\propto W$; **BGT(1975, *Wear* 35:87)** 타원포물면, $A\approx\sqrt{2\pi}\,W/(E^*\sqrt{m_2})$; **McCool(1986, *Wear* 107:37)** 표면→$(m_0,m_2,m_4)$→GW, 단순≈완전.
**(C-3) skew/kurtosis 효과:** **McCool(1992, *IJMTM* 32:115, [CC 79])** 동일 RMS서 $S_{sk}{=}{+}1\to$ 평균 돌기압 **~1.4배↑**, ${-}1\to$ **~1.7배↓**; **Kotwal–Bhushan(1996, *Tribol.Trans.*, [CC 118])** 양 skew→면적↓·최조기 최대압→마모취약; **Yu–Polycarpou(2004, *J.Tribol.* 126:225, [CC 59])** 결합 시 가우시안화.
**(C-4) ★해상도 의존성(핵심 — 피크는 자[ruler]의 성질):** 자기-affine PSD $C(q)\propto q^{-2(1+H)}$ 에서 곡률 모멘트가 **발산**:
$$
(h''_{rms})^2=\frac{1}{8\pi}\int q^5 C(q)\,dq=\frac{1}{16\pi}\frac{C_0}{2-H}\,q_s^{\,4-2H}\ \Rightarrow\ h''_{rms}\propto q_s^{\,2-H}\to\infty
$$
($q_s$=단파장 컷오프=해상도). 평균 summit 곡률 $\sqrt{m_4}$·반경·Nayak $\alpha$ 모두 해상도 의존 → "asperity는 길이척도 없이는 ill-defined"; 높이 $m_0$ 만 수렴(robust). **Whitehouse–Archard(1970, *Proc.R.Soc.A* 316:97, [CC 919]) → Greenwood–Wu(2001, *Meccanica* 36:617, [CC 259] "an apology" — GW 저자 본인이 peak 정의 철회) → Greenwood(2006, *Wear* 261:191, [CC 222]); 현대 처방 Jacobs–Junge–Pastewka(2017, *Surf.Topogr.* 5:013001, [CC 430], 대역폭 명시 PSD 모멘트); Persson(2001, [CC≈1,484]) magnification으로 컷오프를 명시변수화.** → A4.8 해상도 규약의 근거.
**(C-5) 극값(피크 = 평균 아닌 최댓값):** 최댓값 조건 $\int_{x_{\max}}^\infty P\,dx\approx 1/N$ → 가우시안서 **$x_{\max}\approx\sqrt{2\ln N}\,x_{rms}$** (Gumbel/Fisher–Tippett), $N\sim(q_1/q_0)^2$(파장비). 피크높이 분포는 대역폭 $\varepsilon^2=1-m_2^2/(m_0 m_4)$ 의존(narrow→Rayleigh, broad→Gaussian). **Rice(1944/45, BSTJ) → Cartwright–Longuet-Higgins(1956, *Proc.R.Soc.A* 237:212, [CC 439]) → Persson(2023, *Tribol.Lett.* 71:74, $\sqrt{2\ln N}$); EVT 적용 Ponthus et al.(2019, *PRE* 99:023004), Malekan–Rouhani(2019, *Friction* 7:327, Gumbel→Amontons).** ⚠️ 첫 접촉·국부 소성·시일 누설은 평균 아닌 **최고 돌기**가 지배.

## A4.5 축 D — FFT 결정론 압력해석 (단일 실현→피크 압력)

**계보:** **Ju–Farris(1996, *J.Tribol.* 118:320, 2D 스펙트럼 시조) → Stanley–Kato(1997, *J.Tribol.* 119:481, [CC 289], 변분+FFT, $C(\mathbf k)=2/(E^*|\mathbf k|)$) → Polonsky–Keer(1999, *Wear* 231:206, [CC 629], CGM+MLMS/FFT, 표준 솔버) → DC-FFT Liu–Wang–Liu(2000, *Wear* 243:101, [CC 735, 본 축 최다], 영패딩·wrap-around로 비주기 정확).**
- **해상도 핵심:** **Müser et al.(2017, *Tribol.Lett.* 65:118, [CC 324]) Contact-Mechanics Challenge** — 결정론 FFT-BEM/GFMD는 정확, 돌기모델은 체계편차; **돌기당 수 격자점 필요**(coarse면 피크 압력 절단).
- **탄소성:** Tian–Bhushan(1996), **Jacq–Nélias(2002, *J.Tribol.* 124:653, [CC 282]) SAM**(베어링강 표준), Wang(2010).
- **윤활 대응:** **Hu–Zhu(2000, *J.Tribol.* 122:1, [CC 546])** unified Reynolds — $h\to0$ 서 dry 접촉으로 축퇴(부록1 A1.2(b)·부록3 연결).
- **물리:** $\tilde p(\mathbf k)$ 가 $\propto|\mathbf k|$ 강성으로 단파장 피크에 **국부 고압(스파이크)** 부여 + 단측제약($p\ge0$)이 최고 피크에 하중 집중 → 통계평균이 지운 sporadic 피크를 **결정론이 복원**.

## A4.6 ★실측 베어링 표면 표현력 랭킹

**근거 문헌: Borodich, Jin & Pepelyshev(2020, *Front. Mech. Eng.* 6:64)** — 단일 프랙탈 $D$·PSD-only 모두 **비유일**(동일 PSD라도 반전복제본은 정반대 접촉거동; "PSD 분석은 본질상 프랙탈 접근의 재공식화"); 물리적 프랙탈은 ~1.5 decade만; **조합 descriptor(Abbott 곡선) 권고**. Hu(2019) bi-fractal: stratified 표면은 층마다 다른 $D$.

| 순위 | 모델 | 실측 베어링 표현력 | 근거 |
|---|---|---|---|
| 1 | **조합/하이브리드** (Abbott + bi-Gaussian + bi/multi-fractal + PSD) | 최상 | Borodich 2020 |
| 2 | **bi-Gaussian/stratified + bi-fractal** (run-in·honed raceway) | 높음 | Leefe 1998; ISO 13565-3; Hu 2019 |
| 3 | **비가우시안 skewed(Weibull/Pearson) + self-affine PSD** | 높음 | Prajapati–Björling 2024; Harvey 2025 |
| 4 | self-affine PSD / Persson | 중 | Persson 2001 (PSD 비유일성 한계) |
| 5 | **단일-D 프랙탈(Majumdar–Bhushan)** | 낮음(run-in서 최약) | Borodich 2020; Hu 2019 비판 |
| 6 | **순수 가우시안(GW)** | superfinish 전($S_{sk}{\approx}0$)에만 적정 | Borodich 2020; Harvey 2025 |

> **핵심 결론(질문 #3):** 실측 베어링 raceway(특히 run-in 후 음의 skew·plateau)는 **단일-D 프랙탈도 순수 가우시안도 부적합**하며, **bi-Gaussian/stratified + 비가우시안 marginal + (측정)PSD 조합**이 표현력 최상. FFT-접촉 전처리엔 **디지털필터(Hu–Tønder)+비가우시안 marginal** 트렁크가 정답(피크 보존).

## A4.7 계보도 + 영향도 트래커

### 4축 통합 계보(요약)
```
Fourier/Wiener–Khinchin ─┬─ Newland/Wu2000(SRM, 가우시안) ─────────────┐
Longuet-Higgins1957 ─► Nayak1971(α, m0m2m4) ─► BGT1975, McCool1986     │ (FFT 전처리)
   └─► GW1966 ─► Whitehouse-Archard1970(해상도) ─► Greenwood-Wu2001     │
Johnson1949 ─► Watson-Spedding1982 ─► ★Hu-Tønder1992 ─► Bakolas/Manesh/Francisco
Mandelbrot ─► Berry-Lewis1980 ─► Majumdar-Bhushan1990/91 ─► Yan-Komvopoulos1998
   └─(self-affine) Palasantzas1993, Persson2001
Whitehouse ─► Leefe1998/ISO13565 ─► Hu2019(bi-fractal) ─► Prajapati2024 ─► Harvey2025
   └─[통합·비판] Borodich-Jin-Pepelyshev2020
Ju-Farris1996 ─► Stanley-Kato1997 ─► Polonsky-Keer1999 ─► DC-FFT(Liu-Wang2000) ─► Müser2017
   └─(E-P) Jacq-Nélias2002  └─(윤활) Hu-Zhu2000
```

### 4축 통합 계보 (개조식 풀이 — 직관 정리)

**① 통계의 뿌리 — "표면을 파동의 합으로 본다" (축 C 근간)**
- Fourier/Wiener–Khinchin: 표면을 여러 파장의 사인파 합으로 보고 PSD로 기술하는 토대.
- Rice(1944) → Longuet-Higgins(1957): 그 파동 합(가우시안 랜덤필드)에서 "봉우리(극대점)가 단위면적당 몇 개·얼마나 높은가"를 처음 수학화.
- **Nayak(1971): 이를 트라이볼로지로 가져와 $m_0,m_2,m_4$·대역폭 $\alpha$ 로 요약** → summit 밀도·곡률 공식 확립(이 줄기의 keystone).

**② 통계로 접촉을 풀다 — 그러나 해상도에서 무너진다 (축 C 응용·위기)**
- Greenwood–Williamson(1966): summit 통계를 Hertz에 넣어 실접촉·하중 계산(가장 많이 인용된 접촉모델).
- Bush–Gibson–Thomas(1975)·McCool(1986): GW를 스펙트럼 모멘트로 엄밀화·실용화.
- ★위기: Whitehouse–Archard(1970) "봉우리 곡률은 측정 간격에 의존" 경고 → **Greenwood–Wu(2001) "asperity 정의는 틀렸다"는 자기비판(apology)**. 즉 평균·summit 모델은 **해상도에 발이 묶임**.

**③ 피크를 살리는 생성법 (축 A)**
- Johnson(1949): 임의 skew·kurtosis를 만드는 변환계(통계학 뿌리).
- Watson–Spedding(1982) → **★Hu–Tønder(1992): 디지털 필터로 목표 PSD+비가우시안을 동시 부여**(피크 보존 트렁크의 핵심).
- Bakolas·Manesh·Francisco: 이방성·3D·하이브리드로 확장.

**④ 프랙탈로 멀티스케일을 담다 (축 B — 프랙탈 분기)**
- Mandelbrot → Berry–Lewis(1980): W–M 함수(자기닮음 거칠기의 수학).
- Majumdar–Bhushan(1990/91): 접촉에 적용(프랙탈 접촉모델) → Yan–Komvopoulos(1998): 3D 확장.
- 평행 분기(self-affine): Palasantzas(1993)·Persson(2001) — PSD 지수 $H$ 로 멀티스케일 기술.

**⑤ 실측 표면의 진실 + 통합 비판 (축 B — 실측 분기, ★핵심 질문)**
- Whitehouse → Leefe(1998)/ISO 13565: 실제 가공면은 **두 층(plateau+valley) = bi-Gaussian**.
- Hu(2019): 층마다 다른 프랙탈 차원(bi-fractal) → Prajapati–Björling(2024): Weibull로 음의 skew 반영 → Harvey(2025): 베어링강 run-in 실측이 음의 skew·높은 kurtosis 확인.
- ★**Borodich–Jin–Pepelyshev(2020): "단일-D도 PSD-only도 표면을 유일하게 못 정한다 → 조합 descriptor를 써라"** — 표현력 랭킹(A4.6)의 근거.

**⑥ 피크를 압력으로 — FFT 해석 (축 D)**
- Ju–Farris(1996, 스펙트럼 접촉 시조) → Stanley–Kato(1997, FFT+변분) → Polonsky–Keer(1999, CGM+FFT 표준 솔버) → **DC-FFT(Liu–Wang 2000, 비주기 오차 해결로 실용화)**.
- Müser(2017) Challenge: "결정론 FFT는 정확하나 **격자 해상도가 관건**" 재확인(②의 해상도 경고와 호응).
- 분기: 탄소성(Jacq–Nélias 2002 SAM), 윤활(Hu–Zhu 2000 — $h\to0$ 서 dry 접촉으로 축퇴).

**⑦ 네 줄기가 어떻게 합류하는가 (4축 → 하나의 파이프라인)**
- A·B(피크 보존 표면 생성) → C(피크·해상도·극값 통계로 "무엇을 봐야 하나" 규정) → D(FFT로 피크 압력) → 부록1(응력·피로).
- **한 줄 요약: "표면을 옳게 만들고(A·B) → 피크를 옳게 세고(C) → 빠르게 눌러본다(D)."**
- 공통 교훈(②·⑥ 공명): **해상도(컷오프)를 명시하지 않으면 피크는 재현 불가** — 4축 전체를 관통하는 단일 제약.

### 영향도 트래커 (인용수 = SS/Crossref, 2026-06-30; GS는 1.5~2.5배)

| 논문 | 저널 | 인용수 | 역할 | 베어링 표현력 |
|---|---|---|---|---|
| Greenwood–Williamson 1966 | Proc.R.Soc.A | ≈5,900 | 창시(통계 돌기) | 가우시안 한정 |
| Nayak 1971 | J.Lubr.Technol. | ≈901 | 창시(스펙트럼 모멘트) | 기반 |
| Majumdar–Bhushan 1991 | J.Tribol. | 1,284 | 창시(프랙탈 접촉) | 낮음(단일 D) |
| DC-FFT Liu–Wang–Liu 2000 | Wear | 735 | 표준 도구(FFT) | 도구 |
| Polonsky–Keer 1999 | Wear | 629 | 표준 솔버(CGM+FFT) | 도구 |
| Hu–Zhu 2000 | J.Tribol. | 546 | 창시(윤활 결정론) | 도구(윤활) |
| Liu–Wang 2002 | J.Tribol. | 349 | 응력 FFT | 도구 |
| Hu–Tønder 1992 | IJMTM | 345 | ★창시(비가우시안 생성) | 높음(피크 보존) |
| Müser et al. 2017 | Tribol.Lett. | 324 | 검증(해상도) | 도구 |
| Stanley–Kato 1997 | J.Tribol. | 289 | 창시(FFT 접촉) | 도구 |
| Jacq–Nélias 2002 | J.Tribol. | 282 | 창시(SAM 탄소성) | 도구 |
| Yan–Komvopoulos 1998 | JAP | 686 | 확장(3D 프랙탈) | 중 |
| Wu 2000 | Tribol.Int. | 253 | 확장(SRM/FFT) | 가우시안 |
| Tian–Bhushan 1996 | J.Tribol. | 217 | 확장(변분 E-P) | 도구 |
| Bakolas 2003 | Wear | 156 | 확장(비가우시안 3D) | 높음 |
| Kotwal–Bhushan 1996 | Tribol.Trans. | 118 | 확장(비가우시안 접촉) | 중 |
| Pohrt–Li 2014 | Phys.Mesomech. | 106 | 확장(FFT-BEM 마찰) | 도구 |
| Manesh 2010 | Wear | 96 | 응용(이방·areal) | 높음 |
| Watson–Spedding 1982 | Wear | 94 | 확장(ARMA+Johnson) | 높음 |
| McCool 1992 | IJMTM | 79 | 확장(Weibull skew) | 중 |
| Yu–Polycarpou 2004 | J.Tribol. | 59 / Pawar 2013 59 | 확장 | 중 |
| Francisco–Brunetière 2016 | Proc.IMechE J | 33 | 응용(하이브리드 생성) | 높음 |

`[CC?]` 미확보: Longuet-Higgins 1957, Johnson 1949(book/old), Whitehouse–Archard 1970, BGT 1975, McCool 1986, Persson 2001, Borodich 2020, Leefe 1998, Hu 2019, Prajapati 2024, Harvey 2025, Greenwood 2006 — Google Scholar 직접 재수집 권장.

## A4.8 종합 + 연구확장 우선순위

1. **피크 보존 정답 경로:** (실측 위상 보유) 측정 토포그래피 **또는** Hu–Tønder 디지털필터 + **비가우시안 marginal(음의 $S_{sk}$, 높은 $S_{ku}$)** → **DC-FFT 결정론 접촉(Polonsky–Keer/Stanley–Kato)** → 피크 압력분포 → 부록1 응력·피로.
2. **해상도 규약 필수:** $m_4$/summit 발산 때문에 **컷오프·격자(돌기당 수 점)·대역폭을 명시**(Müser 2017, Greenwood–Wu 2001) — 안 하면 피크 압력 비재현.
3. **베어링 표현력:** bi-Gaussian/stratified + 비가우시안 + PSD 조합(A4.6) 채택.
4. **연구확장 후보(미개척):** 부록3과 결합 — 비가우시안 PSD 표면의 **윤활(Mixed EHL) 결정론 피크 압력 → 통계 응력 → 표면피로**를 닫는 것(부록3 A3.7.4 공백과 동일 표적).

## A4.9 부록 4 참고문헌 [E#]

**축 A** [E1] Wu 2000, *Tribol.Int.* 33(1):47–58, DOI 10.1016/S0301-679X(00)00016-5. [E2] Hu & Tønder 1992, *IJMTM* 32(1-2):83–90, DOI 10.1016/0890-6955(92)90064-N. [E3] Johnson 1949, *Biometrika* 36:149–176. [E4] Watson & Spedding 1982, *Wear* 83:215–231. [E5] Bakolas 2003, *Wear* 254:546–554. [E6] Manesh et al. 2010, *Wear* 268:1371–1379. [E7] Pawar et al. 2013, *J.Tribol.* 135:011401. [E8] Francisco & Brunetière 2016, *Proc.IMechE J* 230:747–768. [E9] Newland, *Random Vibrations…*, 3rd ed.

**축 B** [E10] Berry & Lewis 1980, *Proc.R.Soc.A* 370:459–484. [E11] Majumdar & Bhushan 1990, *J.Tribol.* 112:205–216; [E12] 1991, *J.Tribol.* 113:1–11. [E13] Yan & Komvopoulos 1998, *JAP* 84:3617–3624. [E14] Palasantzas 1993, *PRB* 48:14472. [E15] Persson 2001, *JCP* 115:3840. [E16] Leefe 1998, *Tribol.Ser.* 34:281–290; ISO 13565-2/3. [E17] Hu et al. 2019, *Tribol.Int.* 134:427–434. [E18] Prajapati & Björling 2024, *Lubricants* 12(3):71. [E19] Harvey et al. 2025, *Surf.Topogr.* 13(3). [E20] **Borodich, Jin & Pepelyshev 2020, *Front.Mech.Eng.* 6:64.** [E21] Bhushan (ed.) 2001, *Modern Tribology Handbook*, CRC.

**축 C** [E22] Longuet-Higgins 1957, *Phil.Trans.R.Soc.A* 250:157–174, DOI 10.1098/rsta.1957.0018. [E23] Nayak 1971, *J.Lubr.Technol.* 93:398–407, DOI 10.1115/1.3451608. [E24] Greenwood & Williamson 1966, *Proc.R.Soc.A* 295:300–319. [E25] Whitehouse & Archard 1970, *Proc.R.Soc.A* 316:97–121. [E26] Bush, Gibson & Thomas 1975, *Wear* 35:87–111. [E27] McCool 1986, *Wear* 107:37–60. [E28] McCool 1992, *IJMTM* 32:115–123. [E29] Kotwal & Bhushan 1996, *Tribol.Trans.* 39:890–898. [E30] Chilamakuri & Bhushan 1998, *Proc.IMechE J* 212:19–32. [E31] Yu & Polycarpou 2004, *J.Tribol.* 126:225–232. [E32] Greenwood & Wu 2001, *Meccanica* 36:617–630; Greenwood 2006, *Wear* 261:191–200. [E33] Jacobs, Junge & Pastewka 2017, *Surf.Topogr.* 5:013001. [E33b] Whitehouse & Phillips 1978, *Phil.Trans.R.Soc.A* 290:267–298. [E33c] Rice 1944/45, *Bell Syst.Tech.J.* 23:282 / 24:46. [E33d] Cartwright & Longuet-Higgins 1956, *Proc.R.Soc.A* 237:212–232. [E33e] Persson 2023, *Tribol.Lett.* 71:74 (arXiv:2304.02159) — $x_{\max}\approx\sqrt{2\ln N}\,x_{rms}$; Persson 2023, *Tribol.Lett.* 71:29 — max-height 파라미터 비신뢰성. [E33f] Ponthus et al. 2019, *Phys.Rev.E* 99:023004; Malekan & Rouhani 2019, *Friction* 7:327–339.

**축 D** [E34] Ju & Farris 1996, *J.Tribol.* 118:320–328. [E35] Stanley & Kato 1997, *J.Tribol.* 119:481–485. [E36] Polonsky & Keer 1999, *Wear* 231:206–219. [E37] Liu, Wang & Liu 2000, *Wear* 243:101–111. [E38] Liu & Wang 2002, *J.Tribol.* 124:36–45. [E39] Tian & Bhushan 1996, *J.Tribol.* 118:33–42. [E40] Jacq, Nélias et al. 2002, *J.Tribol.* 124:653–667. [E41] Pohrt & Li 2014, *Phys.Mesomech.* 17:334–340. [E42] **Müser et al. 2017, *Tribol.Lett.* 65:118.** [E43] Hu & Zhu 2000, *J.Tribol.* 122:1–9.

> **검증 메모:** 인용수는 SS/Crossref(2026-06-30), GS는 1.5~2.5배 — 절대값보다 **상대 영향도·계보 위치**가 목적. DOI 다수 확인, 일부 페이지·`[CC?]`는 원문/Scholar 대조 권장. McCool 1992의 1.4×/1.7×, Nayak 0.8968/α 계수는 2차자료 일관(원문 paywall). "Pawar–Sundararajan" 생성논문은 미발견 → Pawar–Pawlus–Etsion–Raeymaekers로 대체. 핵심 질문 #3 근거는 Borodich 2020[E20](원문 정독 권장).

