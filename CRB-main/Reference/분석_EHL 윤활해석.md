# EHL 최소유막두께 공식 분석 — Line / Point Contact

> 풍력발전기 메인베어링 상세설계 과제 / EHL 윤활해석 근거 정리
> ISO 표준 인용식(식 37, 식 53)의 원본 추적 및 물리적 조건 분석

---

## 1. 두 수식의 정체

표준이 제시한 두 최소유막두께 공식은 EHL(탄성유체윤활, Elastohydrodynamic Lubrication) 이론의 수치해석 결과를 무차원 회귀식으로 정리한 **고전 공식**이다.

### 1.1 Line contact (식 37)

$$
H_{\min} = \frac{2.65\,\bar{U}^{0.7}\,G^{0.54}}{\bar{Q}^{0.13}}
$$

→ **Dowson–Higginson** 선접촉 최소유막 공식 (수정판)

### 1.2 Point contact (식 53)

$$
H_{\min} = \frac{3.63\,\bar{U}^{0.68}\,G^{0.49}\left(1 - e^{-0.68\,k}\right)}{\bar{Q}^{0.073}}
$$

→ **Hamrock–Dowson** 타원(점)접촉 최소유막 공식

> ⚠️ **재인용 주의**: 표준이 인용한 **[19] Dwyer-Joyce, [20] Palmgren은 재인용(2차) 문헌**이다.
> 특히 Palmgren의 저서(*Grundlagen der Wälzlagertechnik*, 1964)는 Hamrock–Dowson 식(1977)보다 **13년 앞서** 출간되어 식 53의 원저자가 될 수 없다.

---

## 2. 무차원 변수 정의 (두 식 공통)

| 기호 | 명칭 | 정의 | 물리적 의미 |
|---|---|---|---|
| $\bar{U}$ | 속도(velocity) 파라미터 | $\bar{U} = \dfrac{\eta_0\,u}{E'\,R}$ | 입구점도 × 이송속도 → 유막을 *형성*하는 힘 |
| $G$ | 재료(materials) 파라미터 | $G = \alpha\,E'$ | 점도-압력계수 × 등가탄성계수 |
| $\bar{Q}$ | 하중(load) 파라미터 | 선접촉 $\dfrac{w'}{E'R}$ / 점접촉 $\dfrac{w}{E'R^2}$ | 접촉을 *짓누르는* 힘 |
| $H$ | 무차원 유막두께 | $H = \dfrac{h}{R}$ | 결과값 |
| $k$ | 타원율(ellipticity) 파라미터 | $k = \dfrac{a}{b} \approx 1.03\left(\dfrac{R_y}{R_x}\right)^{0.64}$ | 접촉타원 장·단반경 비 (점접촉 전용) |

기호 정의:

- $\eta_0$ : 상압 동점도 (ambient dynamic viscosity)
- $u$ : 평균 이송속도 (entrainment velocity), $u = \dfrac{u_1 + u_2}{2}$
- $E'$ : 등가(축소) 탄성계수, $\dfrac{2}{E'} = \dfrac{1-\nu_1^2}{E_1} + \dfrac{1-\nu_2^2}{E_2}$
- $R$ : 등가(축소) 반경
- $\alpha$ : 점도-압력계수 (pressure–viscosity coefficient, Barus)
- $w,\ w'$ : 하중 (점접촉 총하중 / 선접촉 단위길이당 하중)
- $a,\ b$ : 접촉타원 장반경·단반경

---

## 3. 각 식이 고려한 조건·물리적 사항

### 3.1 공통 가정 (두 식 모두)

- **등온 (Isothermal)** — 온도 상승에 의한 점도 저하 무시
- **완전급유 (Fully flooded)** — 입구 윤활유 충분 (starvation 아님)
- **평활면 (Smooth surface)** — 표면거칠기 무시 (막비 $\Lambda$ 는 별도 검토 필요)
- **뉴턴 유체 + Barus 점도식**: $\eta = \eta_0\,e^{\alpha p}$
- **정상상태·층류** — Reynolds 방정식 ↔ Hertz 탄성변형 연성 해석

### 3.2 Line contact (Dowson–Higginson, 식 37) 고유 조건

- 무한길이 원통의 **2D 선접촉 → 측면 누설(side leakage) 없음**
- 하중지수 $-0.13$ : 하중에 비교적 민감
- 속도항 $\bar{U}^{0.7}$ 이 지배적, 하중 영향은 작음 → EHL의 핵심 특성을 정량화
  (하중을 늘려도 Hertz 면압이 넓게 퍼져 유막은 잘 줄지 않음)

### 3.3 Point contact (Hamrock–Dowson, 식 53) 추가 고려사항

- **타원형 접촉 → 측면 누설을 명시적으로 반영** (★ 선접촉 대비 핵심 차이)
- 보정항 $\left(1 - e^{-0.68\,k}\right)$ = 측면 누설 보정
  - $k = 1$ (구-평면, 원형접촉): 누설 최대 → 유막 감소
  - $k \to \infty$ (선접촉 근사): 항 $\to 1$ → 식 37과 연속적으로 연결
- 하중지수 $-0.073$ (선접촉 $-0.13$ 보다 작음)
  → 점접촉은 하중 증가 시 접촉타원이 넓어지며 흡수되어 유막두께가 하중에 **더 둔감**
- 도출 범위: $k = 1 \sim 8$, 속도 약 2자릿수, 하중 약 1자릿수,
  재료 = 청동/강/질화규소, 윤활유 = 파라핀계·나프텐계 광유

---

## 4. 원본(Primary) 레퍼런스

### 4.1 식 37 (Line) → Dowson & Higginson

- **D. Dowson and G. R. Higginson**, *Elasto-hydrodynamic Lubrication: The Fundamentals of Roller and Gear Lubrication*, Pergamon Press, Oxford.
  - 1판 1966 — 원식 $H_{\min} = 1.6\,G^{0.6}\,U^{0.7}\,W^{-0.13}$
  - 2판(SI) 1977 — 재료지수 $0.6 \to 0.54$ 수정되어 **"2.65 형태"** 확립

$$
\frac{h_{\min}}{R} = 2.65\,G^{0.54}\,U^{0.70}\,W^{-0.13}
$$

  - 보조: **ESDU Data Item 78035** (설계 표준화)

### 4.2 식 53 (Point) → Hamrock & Dowson  ★ 진짜 원본 ★

- **B. J. Hamrock and D. Dowson**, *"Isothermal Elastohydrodynamic Lubrication of Point Contacts: Part III — Fully Flooded Results"*, ASME Journal of Lubrication Technology, **Vol. 99, No. 2, 1977, pp. 264–276** (NASA Lewis Research Center)

$$
\bar{H}_{\min} = 3.63\,U^{0.68}\,G^{0.49}\,W^{-0.073}\left(1 - e^{-0.68\,k}\right)
$$

  - 3부작 구성: Part I (1976, 이론·해석) / Part II (1976, 타원율·재료) / Part III (1977, 완전급유 최종식)
- 단행본: **B. J. Hamrock and D. Dowson**, *Ball Bearing Lubrication: The Elastohydrodynamics of Elliptical Contacts*, Wiley, 1981

---

## 5. 핵심 요약

1. 표준의 **[19]·[20]은 재인용 문헌** — 식의 원저자가 아님 (Palmgren 1964는 시간상 불가)
2. 식 37 원본 = **Dowson & Higginson (1966 / 1977)**, 식 53 원본 = **Hamrock & Dowson (1977, Part III)**
3. 본질적 차이 = 점접촉 식이 **측면 누설(타원율 $k$)** 을 추가 고려 → 하중 민감도가 더 낮음 ($-0.073$ vs $-0.13$)

> 학술 인용 시에는 **Hamrock–Dowson (1977) Part III** 원논문을 직접 인용하는 것이 정확하며, NASA NTRS에서 무료 PDF로 입수 가능하다.

---

## 6. 참고문헌 (URL)

- Hamrock & Dowson, *Isothermal EHL of Point Contacts Part III — Fully Flooded Results* (NASA NTRS PDF): https://ntrs.nasa.gov/archive/nasa/casi.ntrs.nasa.gov/19770004457.pdf
- Hamrock & Dowson Part III — ASME J. Tribology 99(2):264: https://asmedigitalcollection.asme.org/tribology/article-abstract/99/2/264/420202/
- Lubrecht, Venner & Colin — *The Dowson, Higginson, Hamrock contribution* (review): https://journals.sagepub.com/doi/10.1243/13506501JET508
- *Non-Dimensional Groups, Film Thickness Equations and Correction Factors for EHL: A Review* (MDPI Lubricants 2020): https://www.mdpi.com/2075-4442/8/10/95
- Dowson & Higginson, *Reflections on early studies of EHL* (2021): https://journals.sagepub.com/doi/10.1177/13506501211037218
- Hamrock & Dowson Part I (NASA NTRS PDF): https://ntrs.nasa.gov/api/citations/19750022492/downloads/19750022492.pdf

---

# 부록 1. 20MW+ 초대형 메인베어링 적용을 위한 가정·한계 정합성 평가 및 물리적 사항 도출

> 적용 대상: 20MW급 이상 풍력 메인베어링(TRB), **저속·고하중·그리스 윤활** 운전조건
> 목적: 고전 EHL 식(Dowson–Higginson / Hamrock–Dowson)의 가정·한계를 **운전 레짐 기준으로 재평가**하고, 신뢰성 있는 윤활성능 예측에 필요한 물리량을 논리적 프로세스로 도출.
> 작성 기조: 학술논문 보충자료 수준 — 정량 임계값·상관식·참고문헌 명시.

---

## 7. 레퍼런스 모델 가정·한계의 정합성 평가

발표자료의 "공통 가정 및 한계" 4개 항목을 **운전 레짐(저속·고하중·그리스) 관점에서** 재평가한다. 핵심 결론: **4개 항목을 동등한 "유막 감소 요인"으로 나열하는 것은 부정확**하며, 항목별 지배도와 작용 방향(증가/감소)이 다르다.

> **출처 표기 규약**
> - 정량값·상관식 뒤의 `[n]` 은 **§9 문헌목록**에 대응한다.
> - 표기 없는 식은 **표준 교과서 수준의 정의식**(원전 특정 불필요)이다.
> - `(로컬)` 은 본 Reference 폴더 내 PDF/MD를 2차로 읽어 정리한 것으로, **인용 시 원문 대조 권장**.
> - 수치 임계값은 출처의 실험·해석 조건에 종속되므로, 본 베어링 조건으로 **재산정 필요**.

### 7.1 등온(Isothermal) 가정 — *이 레짐에서는 영향 과대평가 (재서술 필요)*

| 구분 | 발표자료 서술 | 정합성 평가 |
|---|---|---|
| 원 서술 | "온도변화에 따른 점도변화 및 유막두께 감소 고려 한계" | △ 부분적 타당하나 **고속 레짐에 해당**하는 설명 |

**핵심 물리:** 입구 전단발열(inlet shear heating)에 의한 유막 감소는 **열부하 파라미터** $L$ 로 지배된다(정의식은 Murch–Wilson 열보정 이론 [15]).

$$
L = \frac{\eta_0\,\beta\,u^2}{k_f}
\qquad
\varphi_T = \frac{h_{\text{thermal}}}{h_{\text{isothermal}}}
$$

여기서 $\beta$ = 점도의 온도계수, $k_f$ = 윤활유 열전도도, $u$ = 이송속도.

- 판정 기준 $L < 0.1$ → 열적 영향 무시 가능($\varphi_T \approx 1$), $L > 0.1$ → 유막 유의 감소: Manjunath et al. 2023 문헌리뷰 Table 1 [12](로컬).
- 실측(TRB): 고속(>1400 rpm)에서 $\varphi_T \approx 0.94\text{–}0.96$(유막 약 4–6% 감소), 무보정 시 구름저항토크 6–8% 과대평가: Manjunath et al. 2023 [12](로컬). 입구발열 보정식 원전은 Matsuyama(1998–2001), [12]에 재인용.
- **메인베어링(저속, 통상 10–20 rpm 이하)에서는 $u$가 작아 $L \ll 0.1$ → 입구 전단발열에 의한 유막 감소는 사실상 무시 가능** (저자 추론; $L$ 정의식 [15]에 근거).

> **재서술 제안:** 저속 메인베어링에서 등온 가정의 실질 오차는 "입구발열에 의한 유막 감소"가 아니라 **(i) 기준점도 $\eta_0(T)$ 를 실제 접촉부 벌크온도에서 평가해야 하는 문제**와 **(ii) 온도가 그리스 유출(bleed)·$\alpha$·소모수명에 미치는 영향**이다(그리스 bleed의 온도의존성: [3]). 즉 *등온 가정 자체보다 입력 물성의 온도 평가*가 지배적 불확실성이다(저자 견해).

### 7.2 평활면(Smooth surface) 가정 — *이 레짐의 지배적 한계 (가장 중요)*

| 구분 | 발표자료 서술 | 정합성 평가 |
|---|---|---|
| 원 서술 | "혼합·경계윤활 조건에서 돌기접촉·마찰력 변화·전단열 발생 고려 한계" | ◎ 타당하며 **본 레짐의 1차 지배 한계** |

**핵심 물리:** 저속·고하중에서 유막두께가 작아 **막비(film parameter) $\Lambda$** 가 낮아지고, 표면거칠기 돌기가 하중을 분담하는 **혼합/경계윤활**로 진입한다.

$$
\Lambda = \frac{h_{\min}}{\sigma},\qquad \sigma=\sqrt{\sigma_1^2+\sigma_2^2}
$$

| $\Lambda$ | 윤활 레짐 |
|---|---|
| $\Lambda > 3$ | 완전유막 EHL (돌기접촉 무시) |
| $1 < \Lambda < 3$ | 혼합윤활 (돌기접촉 유의) |
| $\Lambda < 1$ | 경계윤활 (돌기접촉 지배) |

- $\Lambda$ 정의 및 레짐 경계: 표준 교과서(Hamrock, *Fundamentals of Fluid Film Lubrication*) [17] — **경계값은 문헌별로 상이**(예: $\Lambda{<}1$ vs ${<}1.5$)하므로 인용 시 기준 명시 필요.
- 풍력 메인베어링은 **운전수명의 무시할 수 없는 비율에서 혼합윤활**로 작동: Hart et al. 2022 Part 1/2 [1][2].
- 돌기 하중분담: **Greenwood–Tripp** 모델 [6](원전). 아래 $K'$ 구체형은 TRB 리브–롤러단 부분 EHL 해석 [14](로컬)에서 정리, **원전 [6] 및 [14] 대조 권장**:

$$
\bar P_a = K' F_{2.5}(\lambda),\quad
K' = \frac{8}{15}\sqrt{2\pi}\,(N\beta_s\sigma)\sqrt{\sigma/\beta_s},\quad
\lambda=\frac{h}{\sigma}
$$

- 수명 연계: ISO 281 점도비 $\kappa = \nu/\nu_1$ 가 작아질수록 수명보정계수 $a_{\text{ISO}}$ 급감: ISO 281 / ISO‑TR 1281‑2 [10]. (배율 "1/2–1/5"는 일반적 경향 예시 — 본 조건 재산정 필요.)

> **심화 제안:** 본 레짐에서는 고전식의 $h_{\min}$ 을 그대로 쓰지 말고, **(i) $\Lambda$ 또는 $\kappa$ 로 레짐 판정 → (ii) 혼합윤활 시 Greenwood–Tripp 돌기 하중분담 [6] + 경계마찰계수 도입 → (iii) $a_{\text{ISO}}$ 로 수명 환산 [10]**의 연쇄가 필수.

### 7.3 완전급유(Fully flooded) 가정 — *이 레짐의 임계 한계 (단, 양방향 작용)*

| 구분 | 발표자료 서술 | 정합성 평가 |
|---|---|---|
| 원 서술 | "그리스의 부족윤활 특성에 의한 유막두께 감소 고려 한계" | ◎ 타당하나 **단방향(감소)만 서술한 것은 불완전** |

**핵심 물리 (A) — 기아윤활(starvation)에 의한 감소:**

- 그리스 윤활 구름접촉은 **기아윤활이 정상상태**이다(일반적 사실: [3]). 실무적으로 완전급유 예측의 **약 70% 수준으로 유막을 감하여** 추정(질량유량 약 30% 감소에 대응): Hart et al. 2022 Part 2 [2].
- 시간의존 유막감쇠(압력배출 지배, 중기아 레짐): Lugt 2012 thin‑layer 모델 [3](로컬, 식·계수 원문 대조 권장):

$$
h_c(t)=\left(\tfrac{1}{6}\,\bar\rho_c^{\,2}\,\mathcal F(0)\,t + h_{c,0}^{-2}\right)^{-1/2}
$$

  여기서 감쇠는 **속도에 거의 무관**하고 압력배출(side-flow)이 지배 → 저속 운전에서도 감쇠 진행 [3].

**핵심 물리 (B) — 증주제(thickener)에 의한 증가:**

- **초저속에서는 그리스 증주제(thickener)가 기유(base oil) 막에 더해져 유막이 기유 EHL 예측보다 두꺼워진다**: Hart et al. 2022 Part 1 [1]; 그리스 박막 연구 [8].
- 증주제 종류별 국부 증막 기여 순서 **PP > Ca > Li**: 그리스 기아/증주제 박막 연구 [8](웹 검색 출처 — **원논문 특정·대조 필요**).
- 즉 그리스 효과는 **단순 감소가 아니라 레짐 의존적 양방향**: (저속·박막) 증주제 증막 ↔ (과회전·미보충) 기아 감막.

**핵심 물리 (C) — 보충(replenishment) 동역학:**

- 보충은 기유 유출(bleed)·모세관 재유동·롤러 스큐 유도 유동에 의존 [3]. 롤러 스큐각 $\pm2°$ 가 감쇠율을 $\sim\!50\%$ 변동: Gao et al. 2026 [4](로컬). 원심력 재배치는 $\Omega$가 작은 저속에서 미약 → 기아 지속 [3][4].

> **심화 제안:** "완전급유 → 기아 70%" 단일 보정 [2] 으로 끝내지 말고, **기아도(degree of starvation)** 를 입구 유막공급으로 정의하고, **증주제 증막 효과 [1][8]** 와 **보충율(over-rolling 주기 대비 reflow 시간) [3][4]** 을 함께 모델링. 저속·미보충 구간은 *기아*, 초저속·박막 구간은 *증주제 증막*이 지배.

### 7.4 뉴턴유체 + Barus 가정 — *두 개의 독립 이슈로 분리 필요*

| 구분 | 발표자료 서술 | 정합성 평가 |
|---|---|---|
| 원 서술 | "그리스의 비뉴턴유체 특성 및 비선형 점도변화 고려 한계" | △ 타당하나 **고압 점압거동과 전단희박화를 혼재** |

본 항목은 물리적으로 **서로 다른 두 한계**를 포함하므로 분리 평가한다.

**(A) Barus 점압식의 한계 — 고하중에서 유의 (실제 오차 큼):**

$$
\text{Barus: } \eta(p)=\eta_0 e^{\alpha p}
\qquad\text{(고압에서 점도 과대평가)}
$$

- Barus 식 원전: Barus(1893) [16]. 고압 과대예측 경향은 정량 EHL 물성 연구 [7]에서 확립.
- 20MW급 고하중 접촉압(최대 $\sim$2–3 GPa, **본 베어링 설계값으로 재확인 필요**)에서 Barus는 점도를 과대평가 → 유막 과대예측 [7].
- **Roelands**식이 고압 점근거동을 더 정확히 모사: Roelands(1966) [5]; 계수 $p_r$ 는 Liu et al. 2023 [13](로컬)에서 사용:

$$
\eta(p)=\eta_0\exp\!\Big\{(\ln\eta_0+9.67)\big[-1+(1+p/p_r)^{Z}\big]\Big\},\quad p_r=1.96\times10^8\ \text{Pa}
$$

- **권고:** 유막형성에 쓰는 점압계수는 초기 Barus 기울기가 아니라 **Blok의 막형성 점압계수 $\alpha^*$(역점근 등점도압)** 또는 Bair의 정량 점압물성 [7]을 사용 — Barus 기울기는 유막을 체계적으로 과대예측 [7].

**(B) 뉴턴 가정(전단희박화)의 한계 — 유막에는 경미, 마찰·그리스에는 유의:**

- 전단율 $\dot\gamma \sim \Delta u/h$. **저속·저 SRR(순수구름 우세) 메인베어링 구름접촉에서는 입구 전단율이 낮아 전단희박화의 유막두께 영향은 작다**: Liu et al. 2023 TEHL‑Carreau 해석 [13](로컬, 저속·저온에서 전단희박화 영향 최소 보고).
- 단, **트랙션(마찰)·발열**은 전단희박화·한계전단응력에 민감 → Carreau/Carreau–Yasuda 필요 [13]:

$$
\eta^*=\eta\big[1+(\eta\dot\gamma/G)^2\big]^{(n-1)/2}
$$

- **그리스 벌크**는 항복응력을 갖는 **Herschel–Bulkley** 거동 $\tau=\tau_0+K\dot\gamma^{\,n}$ → 보충·유동(7.3 C)에 직접 관여: 그리스 유변학 [3].

> **재서술 제안:** "비뉴턴·비선형 점도"를 하나로 묶지 말고 **(A) 고압 점압거동(Barus→Roelands/$\alpha^*$, 유막에 유의) [5][7][16]** 과 **(B) 전단희박화(유막엔 경미, 마찰·그리스 유동엔 유의) [3][13]** 로 분리 기술.

### 7.5 정합성 평가 종합 (지배도 매트릭스)

| 가정 | 발표자료 분류 | 저속·고하중·그리스 레짐 실제 지배도 | 작용 방향 | 권고 처리 | 출처 |
|---|---|---|---|---|---|
| 등온 | 유막감소 | **낮음** (입구발열 무시 가능) | 감소(미미) | $\eta_0(T)$ 정확 평가로 대체 | [12][15] |
| 평활면 | 유막감소 | **매우 높음 (1차 지배)** | 돌기접촉·마찰↑ | $\Lambda/\kappa$ 판정 + Greenwood–Tripp + $a_{\text{ISO}}$ | [1][2][6][10] |
| 완전급유 | 유막감소 | **높음 (임계)** | **양방향** | 기아 70% + 증주제 증막 + 보충동역학 | [1][2][3][4][8] |
| 뉴턴+Barus | 유막감소 | Barus **높음**/뉴턴 **낮음** | Barus 과대→감소보정 | Roelands/$\alpha^*$ + (마찰)Carreau + (그리스)H–B | [3][5][7][13][16] |

---

## 8. 신뢰성 있는 윤활성능 예측을 위한 물리적 사항 도출 (논리적 프로세스)

저속·고하중·그리스라는 **운전 레짐의 물리로부터** 필요한 모델 요소를 단계적으로 도출한다. 각 단계는 "왜 필요한가(물리) → 무엇을 도입하는가(모델) → 임계 판정값"으로 구성.

### Step 0. 운전 포락선 정의 (입력 조건의 물리적 구획)

- 회전수 레짐 구획: **정지(false brinelling 위험) / 초저속 0.1–1 rpm(기아·미끄럼) / 저속 1–15 rpm(기아–보충 천이) / 정격 15+ rpm(입구발열 발현)**. (구획 경계는 저자 설정 — 정성 근거: 저속 기아 [1][3], 입구발열 발현속도 [12])
- 하중: 모멘트 하중에 의한 **롤러 에지로딩·2열 부등분담** 반영: Stirling 2023 [11](로컬). 비회전·요/피치 과도에 의한 **미소진동(dither)** 식별.
- 온도: 접촉부 벌크온도(자기발열+환경) → $\eta_0(T),\ \alpha(T)$, 그리스 bleed율 결정 [3].

### Step 1. 기저 유막 (고전식의 올바른 입력)

- Hamrock–Dowson(점접촉, 식 53) [§4.2] / Dowson–Higginson(선접촉, 식 37) [§4.1]로 $h_{\min,\,ff}$ 산정.
- **필수 물리:** $\eta_0$ 는 벌크온도 기준, $\alpha$ 는 **막형성 점압계수 $\alpha^*$**(§7.4A, [7]) 사용. 크라우닝 롤러의 **타원율 $k$** 반영(에지 응력집중 → 수정 프로파일).

### Step 2. 윤활 레짐 판정 (완전유막인지부터 확인)

- $\Lambda=h_{\min}/\sigma$ [17], 또는 ISO 점도비 $\kappa=\nu/\nu_1$ [10] 계산.
- $\Lambda>3$($\kappa$ 충분): Step 3로. $\Lambda<3$: **혼합윤활 분기**(Step 5 필수).
- **물리:** 저속·고하중에서 본 베어링은 상당시간 $\Lambda<3$ → 평활면 가정 붕괴: Hart 2022 [1][2].

### Step 3. 기아·그리스 보정 (완전급유 가정 해제)

- 기아도 정의: 입구 유막공급/완전급유 비. 1차 추정 **$h \approx 0.7\,h_{\min,ff}$**: Hart 2022 Part 2 [2].
- 시간의존 감쇠 [3](로컬) + 보충(스큐·bleed·reflow) 동역학 [4](로컬).
- **증주제 증막**(초저속·박막, [1][8])과 **기아 감막**(과회전·미보충, [2][3])의 경쟁을 레짐별로 분기.

### Step 4. 열 보정 (저속에서는 정량적으로 생략 근거 제시)

- 열부하 $L=\eta_0\beta u^2/k_f$ [15] 계산 → **$L<0.1$이면 $\varphi_T\approx1$로 두고 입구발열 보정 생략** [12](생략의 정량 근거 명시).
- 대신 Step 0의 $\eta_0(T)$ 평가 정확도가 지배 → 자기발열·환경온도 모델 연계.

### Step 5. 혼합윤활·돌기접촉 (평활면 가정 해제, 본 레짐 핵심)

- Greenwood–Tripp 돌기 하중분담 [6] → 유체압/돌기압 분리, 실접촉면적·플래시온도(TRB 적용예 [14], 로컬).
- 경계마찰계수·EP/AW 첨가제 보호막 → 미끄럼 구간 마찰·마모 평가.
- **출력:** $a_{\text{ISO}}$(수명보정, [10]) 및 표면손상(미세피칭·smearing) 위험도.

### Step 6. 트랙션·발열 (마찰 예측 시 비뉴턴 도입)

- 유막두께가 아니라 **마찰/발열 예측 목적**일 때 Carreau–Yasuda·한계전단응력 도입 [13].
- 저 SRR 구름접촉이면 유막두께엔 영향 경미(§7.4B, [13]) — 모델 복잡도 절감 근거.

### Step 7. 과도·진동·정지 (정상상태 EHL 밖의 손상 메커니즘)

- **정지·미소진동:** false brinelling/프레팅 부식, **WEC(백색에칭균열)** — 고전 EHL로 예측 불가(정성 근거 [1][11]). 별도 기준(미끄럼량·접촉응력·수소취화·그리스 첨가제) 필요.
- **기동–정지·돌풍 과도:** 유막 붕괴–회복 주기 → 과도 EHL/보충 시간상수 비교 [3][4].

### Step 8. 수명·불확실성 환산

- ISO 281 / ISO‑TR 1281‑2 [10]: $\kappa$, 오염도 $e_C$, $a_{\text{ISO}}$ 로 수정정격수명 산출.
- 기아·증주제·혼합윤활 보정의 **불확실성 전파**(감도분석) → 신뢰구간 제시(방법론: 저자 제안).

### 8.9 도출된 "필수 물리적 사항" 요약 (모델 체크리스트)

| # | 물리적 사항 | 도입 모델/파라미터 | 본 레짐 필요성 | 출처 |
|---|---|---|---|---|
| 1 | 벌크온도 기준 물성 | $\eta_0(T),\ \alpha^*(T)$ | 필수 (지배 입력) | [3][7] |
| 2 | 막형성 점압계수 | Blok $\alpha^*$ / Roelands / Bair | 필수 (Barus 과대예측 보정) | [5][7][16] |
| 3 | 윤활 레짐 판정 | $\Lambda,\ \kappa$ | 필수 (혼합윤활 분기) | [10][17] |
| 4 | 돌기 하중분담 | Greenwood–Tripp, 경계마찰 | **핵심** (저속·고하중 지배) | [6][14] |
| 5 | 기아 보정 | starvation factor($\approx0.7$), Lugt 감쇠식 | **핵심** (그리스) | [2][3] |
| 6 | 증주제 증막 | thickener 기여(PP>Ca>Li) | 초저속 박막 구간 | [1][8] |
| 7 | 보충 동역학 | bleed·스큐($\pm2°\!\to\!\pm50\%$)·reflow | 저속 기아 지속성 | [3][4] |
| 8 | 열보정 생략 근거 | $L<0.1\Rightarrow\varphi_T\approx1$ | 정량적 단순화 근거 | [12][15] |
| 9 | 비뉴턴(마찰) | Carreau–Yasuda, 한계전단응력 | 트랙션·발열 한정 | [13] |
| 10 | 그리스 벌크유동 | Herschel–Bulkley($\tau_0,K,n$) | 보충·채널링 | [3] |
| 11 | 과도·진동·정지 | false brinelling/WEC 기준 | 정상 EHL 밖, 별도 | [1][11] |
| 12 | 수명 환산·감도 | ISO 281 $a_{\text{ISO}}$, $e_C$ | 신뢰성 정량화 | [10] |

> **결론(논리 귀결):** 20MW+ 저속·고하중·그리스 메인베어링에서 신뢰성 있는 윤활예측의 **1차 지배 물리는 (4) 혼합윤활 돌기접촉 [1][2][6]과 (5) 그리스 기아 [2][3][4]**이며, 등온·뉴턴 가정의 해제는 **이 레짐에서 2차적**이다(§7.5). 고전식의 $h_{\min}$ 은 *출발값*일 뿐, 위 체크리스트의 보정 연쇄를 거쳐야 정량적 신뢰도를 확보한다.

---

## 9. 심화분석 참고문헌

> 표기: `(로컬)` = 본 Reference 폴더 내 파일을 2차로 읽어 정리(인용 시 원문 대조 권장). 그 외는 원전/공개 문헌.

**[1]** E. Hart, et al., *Wind turbine main-bearing lubrication – Part 1: An introductory review of EHL theory*, **Wind Energy Science, 7, 1021–1042 (2022)**. https://wes.copernicus.org/articles/7/1021/2022/ — 혼합윤활 비율, 증주제 증막, 저속 기아.

**[2]** E. Hart, et al., *Wind turbine main-bearing lubrication – Part 2: Simulation-based results for a double-row spherical-roller main-bearing in a 1.5 MW turbine*, **Wind Energy Science, 7, 1533–1550 (2022)**. https://wes.copernicus.org/articles/7/1533/2022/ — **기아 70%(질량유량 −30%) 보정**.

**[3]** P. M. Lugt, *Grease Lubrication in Rolling Bearings*, Wiley, 2013; 및 *Thin-layer flow in rolling EHL contacts* — 시간의존 감쇠식 $h_c(t)$, 그리스 유변학·bleed (로컬: `2012. (Lugt) Thin layer flow and ~`).

**[4]** Gao et al. (2026), *Film thickness decay in grease-lubricated contacts* — **롤러 스큐 ±2° → 감쇠율 ±50%**, 타원율 감쇠 의존성 (로컬: `2026. (Lugt) Film thickness decay in grease ~`).

**[5]** C. J. A. Roelands, *Correlational Aspects of the Viscosity–Temperature–Pressure Relationship of Lubricating Oils*, PhD, TU Delft, 1966 — Roelands 점압식.

**[6]** J. A. Greenwood, J. H. Tripp, *The contact of two nominally flat rough surfaces*, **Proc. IMechE, 185, 1970–71, pp. 625–633** — 돌기접촉 하중분담(원전).

**[7]** S. Bair, *High-Pressure Rheology for Quantitative Elastohydrodynamics*, 2nd ed., Elsevier, 2019; H. Blok, $\alpha^*$(역점근 등점도압, 1965) — 막형성 점압계수, Barus 과대예측.

**[8]** 그리스 기아/증주제 박막 연구 (예: P. M. Cann; De Laurentis et al., *Tribology Letters*) — **증주제 증막 순서 PP>Ca>Li**(웹 검색 출처, **원논문 특정·대조 필요**). 참고: MDPI *Lubricants* 그리스 박막 연구 https://www.mdpi.com/2075-4442/3/2/197

**[9]** *Non-Dimensional Groups, Film Thickness Equations and Correction Factors for EHL: A Review*, **MDPI Lubricants 8(10):95 (2020)** — 열·기아·전단희박화 보정계수 종합. https://www.mdpi.com/2075-4442/8/10/95

**[10]** **ISO 281:2007 / ISO‑TR 1281‑2:2008** — 점도비 $\kappa$, 오염계수 $e_C$, 수명보정 $a_{\text{ISO}}$ (로컬 PDF/MD 보유).

**[11]** A. Stirling, *Internal load modelling of tapered-roller main bearings in wind turbines*, Diss., Univ. of Strathclyde, 2023 — 롤러 에지로딩·2열 분담·수명 감도 (로컬).

**[12]** Manjunath et al. (2023), *Experimental Analysis of Rolling Torque and Thermal Inlet Shear Heating in TRB* — 열부하 $L<0.1$ 기준, $\varphi_T\approx0.94$–$0.96$(>1400 rpm) (로컬).

**[13]** Liu et al. (2023), *Thermal EHL analysis of inner-ring rib and roller end in TRB with the Carreau model* — Roelands $p_r$, Carreau 식, 저속 전단희박화 영향 최소 (로컬).

**[14]** Wang et al., *Partial EHL analysis of rib–roller end contact in TRB* — Greenwood–Tripp $K'$ 구체형, 혼합윤활 마찰 (로컬).

**[15]** R. S. Murch, W. R. D. Wilson (1975), 열보정 이론; R. Gohar, *Elastohydrodynamics* — 열부하 파라미터 $L$ 정의식.

**[16]** C. Barus (1893), *American Journal of Science*, 45, 87–96 — Barus 점압식 $\eta=\eta_0 e^{\alpha p}$ 원전.

**[17]** B. J. Hamrock, S. R. Schmid, B. O. Jacobson, *Fundamentals of Fluid Film Lubrication*, 2nd ed., 2004 — 막비 $\Lambda$ 정의·윤활 레짐 경계.

---

# 부록 2. 압력–점도 관계식 조사 — Roelands vs Barus 모델의 고압(GPa) 정확도 비판적 검증

> **검증 대상 주장:** "Roelands 모델은 Barus 모델보다 GPa 수준의 고압(접촉 압력)에서 점도 예측 정확도가 높다."
> **검증 방식:** 1차 자료(이미지)를 그대로 신뢰하지 않고, 고압 점도계(high-pressure viscometry) 측정 기반 문헌으로 상호·비판 검증.
> **부록 1과의 연결:** 본 부록은 부록 1 §7.4(A) "Barus 점압식의 한계"와 Step 1·체크리스트 #2(막형성 점압계수)의 근거를 심화한다.

---

## A2.1 결론 요약 (TL;DR)

- **부분적으로만 맞고, 표현 그대로는 틀렸다.** Roelands가 Barus보다 "덜 지수적(less-than-exponential)"으로 점도가 증가하도록 설계된 것은 사실이며, 중간 압력대(대략 $\le 0.5$ GPa)에서는 다수 오일에 대해 Barus보다 실측에 가깝다.
- **그러나 "GPa 수준 접촉 압력" 조건에서는 주장이 뒤집힌다.** 약 0.5 GPa를 넘으면 실제 점도는 **Barus보다도 더 빠르게(super-exponential / super-Arrhenius)** 증가한다. 이 영역에서 Roelands는 점도를 **가장 심하게 과소예측**한다.
- **결정적 모순:** 주장의 근거 이미지의 영어 원문(Bair 1993 인용)이 스스로 "0.5 GPa 이상에서는 Barus보다도 점도가 더 빠르게 증가한다"고 명시한다. 즉 이미지 좌측(한국어) 요약이 **같은 이미지 우측(영어 원출처)의 결론과 충돌**한다.
- **현대 정량 EHL 문헌의 합의:** Barus도 Roelands도 GPa급 EHL 접촉의 정량 해석에는 부적합하다. Roelands 본인조차 자신의 식을 EHL(접촉) 압력이 아니라 유체역학적(hydrodynamic) 압력 범위($\sim$0.15–0.5 GPa)로만 권장했다. 고압에서는 **자유부피(free-volume) 계열 모델**(Doolittle, Yasutomi-WLF, Bair–Casalini) + **상태방정식(Tait/Murnaghan)** 이 표준으로 권장된다.

> **판정:** 이미지의 한국어 주장은 과잉 단순화이며, 진짜 GPa급 접촉 압력에 대해서는 문헌적으로 반박된다.

---

## A2.2 이미지 내부의 자기모순 (먼저 짚을 점)

| 항목 | 이미지 좌측 (한국어 요약) | 이미지 우측 (영어 원출처) |
|---|---|---|
| 핵심 주장 | "GPa 수준 접촉 압력에서 Barus보다 정확하다" | Roelands가 Barus보다 덜 지수적인 것은 **"오직 $\sim$0.5 GPa까지만" 참** (Bair 1993) |
| 0.5 GPa 이상 | 언급 없음 | **점도가 Barus보다도 더 빠르게 증가** → 두 모델 모두 과소예측, 그래서 Bair et al.(1998) 자유부피 모델 도입 필요 |

좌측 요약은 우측 본문이 명시한 "0.5 GPa"라는 **유효 상한**과 "그 이상에서는 둘 다 틀린다"는 **핵심 단서**를 누락한 채 결론만 "Barus보다 정확"으로 일반화했다. EHL 접촉 압력은 통상 **1–3 GPa**이므로, 누락된 단서가 곧 결론을 좌우한다.

---

## A2.3 실제 액체의 압력–점도 거동 (기준선)

문헌이 일관되게 보여주는 실측 거동:

1. **저~중압($0 \sim 0.5$ GPa):** $\log\eta$ vs $p$ 곡선이 "일반적 오목(general concave)" — 지수보다 **느리게** 증가. → 이 구간에서 단순 지수식(Barus)은 **과대예측** 경향.
2. **고압($\gtrsim 0.5$ GPa, 유리전이 접근):** 거동이 **"지수보다 빠른(greater-than-exponential / super-Arrhenius)"** 으로 전환. → 이 구간에서는 Barus조차 **과소예측**.

즉 실제 곡선은 "처음엔 완만 → 나중엔 급격"(변곡점 존재)인데, 직선(Barus)이나 한쪽으로만 휜 고정형 곡선(Roelands)으로는 양쪽을 동시에 맞추기 어렵다. 90년에 걸친 고압 점도계 측정이 이 super-Arrhenius 거동을 반복 확인했다(Bair, Vergne 등).

---

## A2.4 각 모델의 한계 (문헌 교차 검증)

### A2.4.1 Barus 식

$$
\eta(p) = \eta_0\,e^{\alpha p}
$$

- 가장 단순·직관적이라 EHL 식 유도에 널리 쓰였으나, **실제 점도 변화의 근사로는 부정확**하다는 것이 정설.
- 통상 **1 GPa 이상에서 부정확**하며, 중간 압력대에서는 증가율을 **과대예측**. 반대로 매우 높은 압력에서는 super-Arrhenius 거동 때문에 **과소예측**으로 뒤바뀐다.

### A2.4.2 Roelands 식

$$
\eta(p) = \eta_0 \exp\!\left\{ (\ln\eta_0 + 9.67)\left[ -1 + \left(1 + \frac{p}{p_r}\right)^{Z}\right] \right\},\quad p_r = 1.962\times10^{8}\ \text{Pa}
$$

- Barus의 중간압 과대예측을 완화하려 **덜 지수적(오목)인 고정 형태**로 설계 → 일부 오일·중간 압력대에서 Barus보다 실측에 근접.
- 그러나 **고정된 수학적 형태**가 실제 곡률을 못 맞춘다:
  - 일부 오일은 저압 곡률조차 못 맞춰 회귀 $\mu_0$ 가 실측과 크게 달라짐(예: 실측 20 mPa·s ↔ 회귀 24 mPa·s).
  - 피팅 구간을 좁히면(예: 350 MPa까지) 300 MPa 이상에서 PAO 점도가 POE보다 높게 나오는 등 **물리적으로 틀린 외삽** 발생.
- 여러 연구가 **0.5 GPa 미만에서조차 Roelands 부적절** 사례를 보고. Roelands 권장 유효 상한은 본인 기준 약 **0.15–0.5 GPa**.
- 핵심: **Roelands 본인은 자신의 식을 EHL(접촉) 압력이 아니라 유체역학적 압력용으로만 권장**했다. EHL 접촉(1–3 GPa)에 쓰는 것은 원저자 적용 범위를 벗어난 사용이다.

### A2.4.3 Bair의 비판 (= 이미지 영어 원문의 출처)

- **Bair, S. (1993)**, *"A note on the use of Roelands equation to describe viscosity for EHD Hertzian zone calculations,"* ASME J. Tribology, 115, 333–334 → 이미지가 인용한 바로 그 "Bair 1993". Roelands 식의 Hertzian(접촉) 영역 적용 정확도에 의문 제기.
- 이후 Bair·Vergne 등은 고압 점도계 측정을 근거로 **Barus·Roelands·Eyring을 EHL 정량 도구로 쓰는 것**을 강하게 비판하며, "고전 EHL이 진정한 정량 학문이 되려면 실측 기반 rheology를 반영해야 한다"고 주장(Vergne & Bair 2014; Bair et al. 2016; Bair 2019). 최근에는 이를 "viscosity artifice / catastrophe"라고까지 표현.

---

## A2.5 그렇다면 고압에서 무엇이 정확한가

EHL/GPa 영역에서 실측에 부합한다고 보고되는 모델군:

- **Doolittle 자유부피(free-volume) 모델:** "분자당 자유부피"에 흐름 저항이 의존한다는 물리적 근거 기반. 저압 inlet에서 점도가 볼록(convex)하게 급증하는 거동을 Roelands보다 잘 재현.
- **Yasutomi 상관식 (압력 수정 WLF):** Doolittle 자유부피 이론에서 유도, 상태방정식 없이도 사용 가능. EHL 압력대에서 실측과 양호한 일치(개선판 Yasutomi 권장).
- **Bair–Casalini 열역학 스케일링 모델.**
- 위 모델들은 보통 **Tait 또는 Murnaghan 상태방정식**과 함께 부피(압축성)를 같이 다룬다.

> **요지:** "고압 정확도"를 원한다면 Barus↔Roelands 양자택일이 아니라, **자유부피 계열 + EOS**로 넘어가야 한다는 것이 현대 문헌의 방향. (→ 부록 1 체크리스트 #2 "막형성 점압계수 $\alpha^*$ / 정량 rheology"와 직결)

---

## A2.6 Roelands $Z$ 지수 표에 대한 코멘트

| 윤활유 | 표의 $Z$ 범위 / 전형값 | 비판적 코멘트 |
|---|---|---|
| 광유(Mineral) | 0.60–0.75 / 0.67 | Roelands $Z$ 지수의 통상 범위로 무리 없음 |
| PAO | 0.45–0.55 / 0.50 | 동일 |
| 에스터(Ester) | 0.50–0.60 / 0.55 | 동일 |

- 수치 범위는 합리적이나, **"종류별 전형값 하나(상수 $Z$)를 GPa까지 그대로 적용"하는 사용법 자체가 Bair류 비판의 핵심 대상**이다. Roelands의 $Z$(및 점압계수 $\alpha$)는 특정 압력 구간에서만 유효한 **유효(effective) 값**이며, 단일 상수로 EHL 전 영역을 커버하면 큰 오차가 생긴다(저온·고압일수록 결과가 $Z$에 매우 민감).
- 따라서 표를 "정확한 고압 입력값"이 아니라 **"제한된 압력 범위용 근사 파라미터"** 로 이해해야 한다.

---

## A2.7 수식 일관성 체크 (참고)

- 한국어식: $\eta(p) = \eta_0 \exp\!\left\{ (\ln\eta_0 + 9.67)\left[ (1 + p/p_r)^{Z_r} - 1 \right] \right\}$, $p_r = 196.2$ MPa
- 영어식(원문 식 36): $f(p) = (\ln\eta_0 + 9.67)\left[ -1 + (1 + 5.1\times10^{-9}\,p)^{Z} \right]$
- $5.1\times10^{-9}\ \text{Pa}^{-1} \approx 1/(196\ \text{MPa}) \approx 1/p_r$ 이고 $f(p) = \ln\eta(p)$ 형태이므로 **두 식은 동일**하다. 또한 $p=0$ 이면 $(1)^Z - 1 = 0 \Rightarrow \eta(0) = \eta_0$ 로 정확히 환원된다.
- 즉 **식 자체의 수학적 일관성은 문제없다.** 문제는 식이 아니라 **"GPa 고압에서 정확하다"는 적용 주장**이다.

---

## A2.8 종합 판정

| 질문 | 판정 |
|---|---|
| Roelands가 Barus보다 "덜 지수적"인가? | ✅ 사실 (설계상) |
| 중간 압력대($\le \sim0.5$ GPa)에서 다수 오일에 대해 Barus보다 실측에 가까운가? | 🟧 대체로 사실 (단, 오일에 따라 0.5 GPa 미만에서도 부정확 사례 존재) |
| **"GPa 수준 접촉 압력"에서 Roelands가 Barus보다 정확한가?** | ❌ **반박됨.** 0.5 GPa 이상에서 실제 점도는 Barus보다도 빠르게 증가, Roelands는 가장 크게 과소예측 |
| GPa 영역의 정확한 모델은? | ➡️ Roelands/Barus가 아니라 **Doolittle·Yasutomi·Bair–Casalini + Tait/Murnaghan EOS** |

> **한 줄 결론:** "Roelands = 고압 정확 모델"이라는 프레이밍은 출처(Bair 1993)와 현대 정량 EHL 문헌 양쪽 모두에 의해 부정된다. Roelands는 *중간 압력대에서 Barus를 개선한 근사*일 뿐, *GPa급 접촉 압력의 정확한 모델은 아니다.*

---

## A2.9 참고문헌 (교차검증 출처)

**[A1]** Bair, S. (1993). *A note on the use of Roelands equation to describe viscosity for EHD Hertzian zone calculations.* ASME J. Tribology, 115, 333–334. — (이미지의 "Bair 1993" 원출처)

**[A2]** Bair, S. (2004). *Roelands' missing data.* Proc. IMechE Part J: J. Engineering Tribology. — Roelands 식 정확도 의문 제기.

**[A3]** Bair, S. et al. (2016). *Classical EHL Versus Quantitative EHL: A Perspective Part II — Super-Arrhenius Piezoviscosity.* Tribology Letters. — super-Arrhenius 거동, Roelands 곡률 적합 실패, "Roelands 본인은 EHL 압력용으로 권장 안 함".

**[A4]** Vergne, P. & Bair, S. (2014); Bair, S. (2019). — 정량 EHL을 위한 실측 기반 rheology 필요성.

**[A5]** Bair, S. (2026). *The Barus–Roelands–Eyring Viscosity Artifice: A Catastrophe for Elastohydrodynamic Lubrication.* ASME J. Tribology, 148(8). — Barus/Roelands/Eyring의 EHL 정량 적용 비판.

**[A6]** Petrone et al. (2013). *Effect of an Improved Yasutomi Pressure-Viscosity Relationship on the EHL Line Contact Problem.* ISRN. — "두 식 모두 전형적 EHL 압력의 piezoviscous 거동을 제대로 모델링하지 못함", 자유부피/Yasutomi 도입 근거.

**[A7]** *Pressure-Viscosity Coefficient* (ScienceDirect Topics, Bair 발췌). — Roelands 유효 상한 0.15–0.5 GPa, 예외적 고압 적합 액체(TMPO) 사례 등.

**[A8]** *Viscosity Calculator: Barus and Roelands* (Tribonet). — Barus가 >1 GPa에서 증가율 과대예측한다는 일반 설명.

**[A9]** ASME J. Tribology 145(5), 2023. — "Roelands는 Barus보다 우수하나 350 MPa 이상에서 일반 오목 거동 과대평가" 등 정량 비교.

> **주의:** 위 출처들은 압력 구간·오일 종류에 따라 "과대/과소" 방향이 달라지므로, 단일 문장으로 일반화하기보다 **압력 영역별로 구분해 해석**하는 것이 정확하다.
