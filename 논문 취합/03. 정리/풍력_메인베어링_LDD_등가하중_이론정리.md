# 풍력 메인베어링 피로수명 해석: LDD → 등가하중 변환 이론 정리

> 수백 개 로드케이스의 변동하중을 LDD(Load Duration Distribution)로 축약하고 등가하중(equivalent load)으로 변환하는 방법의 이론적 근거, 등가 성립 기준, 가정과 한계, 그리고 검증된 1차 문헌 정리.

---

## 0. 한눈에 보는 파이프라인

```
[로드케이스 시계열]  →  [사이클 카운팅]  →  [손상 누적]  →  [등가하중]
 수백 개 하중 이력      회전당 1 사이클       Miner 선형합      P_eq 도출

── 등가 성립 기준: 동일 누적 손상 ──────────────────────────
   D(변동)  =  Σ ( nᵢ / Nᵢ )  =  D(등가)
   ⟹  P_eq  =  [ Σ ( Pᵢ^p · nᵢ ) / Σ nᵢ ]^(1/p)
────────────────────────────────────────────────────────
```

등가하중이 성립하는 유일한 근거는 **"Palmgren–Miner 누적손상이 같으면 등가로 본다"**는 정의이며, 하중의 *p*승 가중평균이 그 수학적 형태다.

---

## 1. 물리적 기반 — 왜 하중을 지수승으로 다루는가

메인베어링의 지배적 손상모드는 **구름접촉피로(RCF, rolling contact fatigue)**이며, 수명 예측의 뿌리는 Lundberg & Palmgren(1947)의 구름베어링 동적용량 이론이다. 이것이 이후 ISO 281의 기반이 되었다.

핵심 관계식:

$$
L_{10} = \left(\frac{C}{P}\right)^{p}
$$

| 기호 | 의미 |
|---|---|
| $L_{10}$ | 90% 신뢰도 정격수명 (10⁶ 회전 단위) |
| $C$ | 기본 동정격하중 ($L_{10}=1$백만 회전이 되는 하중) |
| $P$ | 등가베어링하중 |
| $p$ | 하중–수명 지수: **볼 3, 롤러 10/3** |
| $e$ | Weibull 기울기: 볼 10/9, 롤러 9/8 |

**결정적 물리 사실:** 수명은 하중의 *p*승에 반비례한다. 하중이 2배면 수명은 $2^3 = 8$배(볼) 또는 $2^{10/3} \approx 10$배(롤러) 감소한다. 이 강한 비선형성 때문에 "평균하중"을 산술평균으로 잡으면 안 되고, 반드시 **_p_승 가중평균**을 써야 한다 — 이것이 등가하중의 물리적 출발점이다.

---

## 2. 등가가 성립하는 "기준" — Palmgren–Miner 선형 손상 누적

시계열을 스칼라 하나로 바꿀 수 있는 근거는 오직 하나: **손상의 총량이 같으면 등가**. 그 손상을 더하는 규칙이 Palmgren–Miner 가설이다.

- 각 하중수준 $i$의 단기 손상: $D_i^{ST} = n_i / N_i$ (겪은 사이클 ÷ 허용 사이클)
- 파손 조건: $D = \sum \dfrac{n_i}{N_i} = 1$

### 등가하중 유도 (직접 전개)

1. 하중수준 $i$의 허용수명: $N_i = (C/P_i)^p$ (×10⁶ 회전)
2. 총손상: $D = \sum \dfrac{n_i}{N_i} = \dfrac{1}{C^p}\sum n_i P_i^{\,p}$
3. 동일 손상을 총 회전수 $N = \sum n_i$에서 단일하중 $P_{eq}$로 내면:
   $$\frac{N \cdot P_{eq}^{\,p}}{C^p} = \frac{1}{C^p}\sum n_i P_i^{\,p}$$
4. 정리:

$$
\boxed{\,P_{eq} = \left[\frac{\sum P_i^{\,p}\, n_i}{\sum n_i}\right]^{1/p}\,}
$$

볼베어링($p=3$)에서는 **하중의 3승 가중평균(cubic mean load)**이 된다.

> 실무 구현에서는 작업 사이클을 조건이 근사적으로 일정한 여러 구간으로 나눠 구간별 수명을 계산한 뒤 결합하며, 이 구간별 손상합이 위 식과 동치다.

---

## 3. 알고리즘 — LDD / LRD vs Rainflow

풍력 업계의 관행적 이분법: **구조부재는 Rainflow, 모든 베어링·기어는 LRD/LDD**. 손상을 세는 물리량이 다르기 때문이다.

| 구분 | 대상 | 세는 물리량 | 지수 | 잃는 정보 |
|---|---|---|---|---|
| **Rainflow + S-N** | 타워·블레이드·용접부 등 구조부재 | 응력 range·mean (히스테리시스 루프) | Wöhler(S-N) 기울기 $m$ | 시간 순서, 주파수 |
| **LDD** (Load Duration Distribution) | 기어·베어링 (시간 기반) | 하중 수준별 **체류시간** | 하중–수명 지수 $p$ | 사이클 순서, range–mean 결합 |
| **LRD** (Load Revolution Distribution) | 베어링·기어 (회전 기반) | 하중 수준별 **회전수** | 하중–수명 지수 $p$ | 순서, 하중존 상세 분포 |

### LDD의 물리적 정의 (핵심)

LDD는 전동체가 **하중존을 들어가고 나오는 사건**을 세어 **축 1회전당 1 사이클**로 계산한다. 10분 평균 축속도 $\bar{\omega}_i$를 쓰면:

$$
n_i = \bar{\omega}_i \, \Delta t
$$

즉 베어링의 "1 사이클"은 응력 최대–최소 진동이 아니라 **전동체가 하중존을 1회 통과하는 사건**이다. 그래서 range/mean이 아니라 **하중 크기 × 회전수**가 손상을 지배하며, 이것이 구조부재용 Rainflow와 근본적으로 다른 이유다.

> IEC 61400-4에 기어박스 부품 설계코드가 제시되며, 전역 시뮬레이션 토크 시간이력을 통상 **64개 하중 bin의 LDD**로 축약해 기어·베어링·축 설계에 사용한다.

---

## 4. 직관 — "가장 아픈 순간이 대부분의 손상을 낸다"

산술평균은 모든 순간을 동등하게 취급하지만, 베어링 손상은 **고하중 구간에 극단적으로 편중**된다.

- NREL 메인베어링 연구: 등가 radial 하중 $P_{eq}$는 운전점에 강하게 의존하고 **정격풍속 부근에서 정점**.
- 정격풍속 부근 소수의 로드케이스가 전체 수명 손상 대부분을 결정.
- LDD를 그리면 대부분의 시간은 저하중에 있지만, **손상 기여는 고하중 꼬리에 집중**.
- 등가하중식은 이 편중을 $p$승으로 자동 반영한다.

---

## 5. 세우는 가정과, 등가변환으로 "잃는" 효과 ⚠️

> 등가하중은 강력하지만, 시계열을 스칼라 하나로 압축하는 순간 **되돌릴 수 없는 정보 손실**이 발생한다.

| 가정 / 단순화 | 무엇을 버리는가 | 근거·주의점 |
|---|---|---|
| Miner 선형 누적 | 하중 순서(sequence)·과부하 지연 효과 | 선행 하중 순서 영향 무시. 실측 $\sum n/N$은 0.001~10까지 벗어날 수 있음 |
| 등가하중 = 순서 무관 | 시간 이력·과도 이벤트의 위치 | 순서를 지우면 어떤 로드케이스가 언제 왔는지 복원 불가 |
| ISO 281 = 등회전·등하중 | 하중존 내부 응력분포의 비대칭 | 임의 하중은 ISO 16281이 상세 시뮬레이션 요구 |
| $P = X F_r + Y F_a$ 축약 | 반경·축·모멘트 하중의 다축 결합 상세 | 등가동하중은 동시 작용 하중을 단일 성분으로 표현한 가상하중 |
| 회전 베어링 가정 | 요동(oscillation)·정지중 하중 | 소진폭 반복 요동은 ISO 281 가정 밖 → NREL DG03 별도 방법 |
| RCF만 고려 | 마모·프레팅·false brinelling·전식 | ISO 281은 마모·부식·전기침식 영향 미포함 |
| $a_{ISO}$ 분리 처리 | 윤활·오염·온도의 시변 결합 | 별도 계수로 곱해질 뿐 시변 결합 반영 못 함 |
| range 계수 필요 | 같은 max/min·다른 평균의 구분 | 피크만 보는 방법은 평균이 다른 두 변동을 구별 못 함 |

### 특히 주목할 두 가지

1. **정지중·요동중 하중은 회전기반 등가하중이 원리적으로 담지 못한다.**
   RCF는 회전을 전제하지만, 풍력 메인베어링은 아이들링·정지·저속에서 false brinelling·프레팅이 실제 고장의 큰 부분을 차지한다.

2. **LDD/등가하중은 ISO 16281의 라인/전동체별 내부 하중분포를 뭉갠다.**
   최신 연구가 등가하중 대신 세그먼트별 하중 사이클수·최대 접촉압력을 구해 Palmgren–Miner로 세그먼트 피로확률을 추정하는 방향으로 가는 이유. 계산부하가 가장 적은 NREL DG03 방식은 내부 하중분포 기반 방식보다 **훨씬 긴 수명**을 산출 → 등가하중 단순화가 결과를 **낙관적으로 편향**시킬 수 있음.

---

## 6. 문헌·출처 표 (1차 자료 우선)

| # | 문헌 / 표준 | 이 논의에서의 역할 | 출처 |
|---|---|---|---|
| 1 | Lundberg & Palmgren, *Dynamic Capacity of Rolling Bearings* (1947) | $L_{10}=(C/P)^p$, RCF 수명이론 원전 | Acta Polytechnica |
| 2 | ISO 281:2007 | 동정격하중·정격수명·등가동하중 국제표준 (등회전·등하중 가정) | iso.org/standard/38102.html |
| 3 | ISO 16281 | 임의 하중하 내부 하중분포 기반 상세 수명 | wes.copernicus.org/articles/5/1743/2020 |
| 4 | NREL DG03 (Yaw & Pitch Bearing Life) | 요동베어링 등가하중·듀티사이클 적용 가이드 | github.com/frasere/ISO-281 |
| 5 | IEC 61400-1 / 61400-4 | DLC 로드케이스, 기어박스 LDD(64 bin) 설계코드 | IEC |
| 6 | Palmgren(1924) & Miner 선형 손상 규칙 | 등가 성립 기준(누적손상=1), 순서 무시 가정 | Risø-R-1063(EN), osti.gov |
| 7 | NREL, *WT Main Bearing Rating Lives* (2024, TP-86901) | 메인베어링 $P_{eq}$의 운전점 의존성·정격풍속 정점 실증 | docs.nrel.gov/docs/fy24osti/86901.pdf |
| 8 | Stammler / Schwack / Menck 계열 (WES) | 내부하중분포 기반 vs DG03 등가법 결과 비교 | wes.copernicus.org/articles/5/1743/2020 |
| 9 | Digital Twin drivetrain 논문 (Springer, 2023) | LDD 물리 정의(회전당 1사이클), $n_i=\bar{\omega}\Delta t$ | doi.org/10.1007/s10010-023-00627-0 |
| 10 | Menck (2023) RCF review (WES preprint) | LRD·Harris factor·요동 등가 이론 정리 | wes.copernicus.org/preprints/wes-2023-102 |

---

## 7. 핵심 요약

- **등가 근거:** Palmgren–Miner 누적손상이 같으면 등가. 수학적 형태는 하중의 $p$승 가중평균(cubic mean).
- **알고리즘:** 구조부재 = Rainflow(응력 range/mean, S-N 기울기 $m$), 베어링 = LRD/LDD(회전당 1 사이클, 하중–수명 지수 $p$).
- **잃는 것:** 하중 순서, 정지·요동중 손상, 내부 하중분포, 비피로 고장모드(마모·프레팅·전식).
- **보완 경로:** 정밀 검증이 필요하면 ISO 16281 기반 세그먼트 해석으로 교차확인.

---

*본 문서는 검증된 표준(ISO 281, ISO 16281, IEC 61400 시리즈, NREL DG03/TP-86901)과 1차 문헌을 근거로 작성됨.*

---

# 추가 조사 1 — 세그먼트별 국소 손상 접근 & ISO 16281 연구문헌

> 등가하중(전역 스칼라) 대신 **세그먼트별 하중 사이클수·최대 접촉압력**을 구해 Palmgren–Miner로 세그먼트 피로확률을 추정하는 방향으로 연구가 이동하는 이유와, ISO 16281을 이용하는 최신 연구문헌 정리.

## A1-1. 왜 "등가하중"에서 "세그먼트별 국소 손상"으로 이동하는가

핵심 이유는 등가하중이 **손상을 위치와 무관한 전역 스칼라로 합산**한다는 근본 한계다. Menck(2023)에 따르면, 시변 운전조건에서 가장 정확한 방법은 유한세그먼트법(Finite Segment Method)이며, 이는 시간에 따라 전역·위치무관 손상을 합산하는 대신 **국소 하중 변화를 고려**하기 때문이다. 구체적 동인은 네 가지다.

1. **하중존의 원주방향 비대칭.** 메인·피치 베어링은 굽힘모멘트로 하중이 원주를 따라 크게 다르게 분포한다. ISO 281은 이 경우 ISO 16281을 참조하도록 하며, 충분한 정밀도를 위해 상세 베어링 시뮬레이션을 요구한다. 등가하중 하나로는 어느 세그먼트가 실제로 많이 굴려지는지 표현 불가.

2. **비피로 고장모드와의 괴리.** ISO 15243에 기술된 프레팅·false brinelling 등 마찰부식 고장은 ISO 281·ISO/TS 16281 어디에도 미반영. 세그먼트별 접촉압력·미끄럼을 직접 구하는 접근이 이런 국소 현상을 다룰 여지를 남긴다.

3. **현장 고장률과 예측의 불일치.** 메인베어링은 20년차 **22–25% 고장률**이 보고되며 ISO 281 정격수명 예측을 크게 벗어난다. Kenworthy 등(2024)은 이를 (1) RCF가 주원인이 아니거나 (2) RCF가 기여하나 현행 정격수명법이 이를 충분히 포착하지 못함으로 정리하고, **ISO/TS 16281 기반 분석 확장을 명시적으로 권고**한다.

4. **물리적 원점으로의 복귀.** 세그먼트법은 각 응력사이클마다 최대 직교전단응력 $\tau_0$, 그 깊이 $z_0$, 응력체적 $V$를 Lundberg–Palmgren 식에 직접 대입해 Miner로 누적하고, 개별 세그먼트 생존확률을 **Weibull 최약연쇄(weakest-link)**로 결합해 라인수명 → 베어링수명으로 합친다. 등가하중이라는 중간 축약을 건너뛰고 접촉응력 수준에서 직접 계산.

### Lundberg–Palmgren 생존확률식 (세그먼트 접근의 물리적 기초)

$$
\ln\frac{1}{S} \propto \frac{N^{e}\,\tau_0^{\,c}\,V}{z_0^{\,h}}
$$

| 기호 | 의미 |
|---|---|
| $S$ | 생존확률 |
| $\tau_0$ | 최대 직교전단응력 |
| $z_0$ | $\tau_0$ 발생 깊이 |
| $V$ | 응력을 받는 체적(loaded volume) |
| $N$ | 하중 사이클수(rollovers) |
| $e,\,c,\,h$ | Weibull 지수 및 응력–수명 지수 |

세그먼트법은 이 식을 **베어링 전체가 아니라 각 세그먼트에** 적용한다는 점이 등가하중법과의 결정적 차이다.

## A1-2. ISO 16281 / 세그먼트 접근 연구문헌

> Google Scholar 링크는 제목 기반 검색 URL이며, 옆의 DOI가 원문 식별자다. Menck의 논문은 IWES 방침상 DOI로 오픈액세스 공개.

| 문헌 | 핵심 기여 | DOI / 출처 | Google Scholar |
|---|---|---|---|
| **Menck, O. (2023)** — *The Finite Segment Method: A Numerical RCF Life Model for Bearings Subjected to Stochastic Operating Conditions*, J. Tribol. 145(3):031201 | L–P을 세그먼트 이산모델로 일반화. 세그먼트별 $\tau_0\!\cdot\! z_0\!\cdot\! V$ 직접 평가 후 Miner 누적. 블레이드 베어링 적용 | `10.1115/1.4055916` | [🔗](https://scholar.google.com/scholar?q=Finite+Segment+Method+Menck+rolling+contact+fatigue+bearings) |
| **Menck, O. (2024)** — *Review of RCF life calculation for oscillating bearings and application-dependent recommendations for use*, WES 9:777–800 | 5개 접근을 "Miner를 얼마나 정확히 적용하나" 축으로 정렬. FSM·Escalero·Hai 등 세그먼트법 비교 리뷰 | `10.5194/wes-9-777-2024` | [🔗](https://scholar.google.com/scholar?q=Review+rolling+contact+fatigue+oscillating+bearings+recommendations+Menck) |
| **Stammler, Menck et al. (2020)** — *Fatigue lifetime calculation of wind turbine blade bearings considering blade-dependent load distribution*, WES 5:1743 | 하중분포를 시뮬레이션·회귀로 구해 **ISO 16281로 각 궤도 개별수명** 계산. NREL DG03이 ISO 16281 축약판임을 정량 비교 | `10.5194/wes-5-1743-2020` | [🔗](https://scholar.google.com/scholar?q=Fatigue+lifetime+wind+turbine+blade+bearings+blade-dependent+load+distribution) |
| **Menck, O. et al. (2025)** — *RCF calculation of a three-row roller pitch bearing in a wind turbine*, WES 10:2771 | 롤러를 라미나로 이산화한 **ISO/TS 16281 기반** FE 압력분포 수명계산. IWES BEAT6.1 실측 변형률로 FE 검증 | `10.5194/wes-10-2771-2025` | [🔗](https://scholar.google.com/scholar?q=rolling+contact+fatigue+three-row+roller+pitch+bearing+wind+turbine+Menck) |
| **Kenworthy, J. et al. (2024)** — *Wind turbine main bearing rating lives as determined by IEC 61400-1 and ISO 281: A critical review and exploratory case study*, Wind Energy 27 | **메인베어링 특정.** ISO 281/16281·IEC 61400-1 정격수명 비판적 검토. 현장 22–25% 고장률과 불일치, ISO/TS 16281 확장 권고 | `10.1002/we.2883` | [🔗](https://scholar.google.com/scholar?q=Wind+turbine+main+bearing+rating+lives+IEC+61400-1+ISO+281+Kenworthy) |
| **Escalero et al. (2023/2024)** — probabilistic RCF prediction, multiple-row ball bearings | **3D 이산화 궤도** 모델. 각 유한요소 응력사이클 이력을 rainflow로 세고, $\tau_0$ 기준·S-N·Weibull 최약연쇄로 요소별 파손확률 결합 | (Menck 2024 §2.2 인용) | [🔗](https://scholar.google.com/scholar?q=Escalero+probabilistic+rolling+contact+fatigue+multiple-row+ball+bearings+wind+turbine) |
| **Hai et al. (2012)** — generalization of ISO 281 for slewing bearings | 베어링을 세그먼트로 분할(진폭 의존 폭). Menck과 유사하나 추가 단순화 포함 | (Menck 2024 인용) | [🔗](https://scholar.google.com/scholar?q=Hai+slewing+bearing+life+ISO+281+generalization+segments) |
| **NREL DG03 / Stammler et al. (2024)** — *Yaw and Pitch Bearing Life* 설계 가이드라인 | ISO/TS 16281 기반. 롤러를 라미나로 분할한 **상세 압력분포** 요구. 등가하중법보다 정밀 | NREL Design Guideline DG03 | [🔗](https://scholar.google.com/scholar?q=NREL+DG03+yaw+pitch+bearing+life+design+guideline) |
| **NREL (2024) TP-5000-86901** — *Wind Turbine Main Bearing Rating Lives* | 메인베어링 $P_{eq}$의 운전점 의존성·정격풍속 정점 실증. $C_u$(피로하중한계) 대비 위치 논의 | docs.nrel.gov/docs/fy24osti/86901.pdf | [🔗](https://scholar.google.com/scholar?q=Wind+Turbine+Main+Bearing+Rating+Lives+NREL) |

### 보조 참고자료

- **NREL TP-82462** — 1.5MW 피치베어링 rating(ISO 16281 적용, macropitting·contact truncation 사례)
- **NREL TP-80195** — 드라이브트레인 신뢰성. ISO 15243 고장모드가 ISO 281/16281에 미반영임을 명시
- **NREL 86299 (Keller·Guo·Brasseur·Evans)** — 메인베어링 정격수명 포스터(ISO 281 + ISO/TS 16281)

## A1-3. 유의점

- Escalero 연구는 본문에서 "2023"(preprint)과 "2024"(최종본)로 혼재 인용됨 → 위 Scholar 검색으로 최종 DOI 확인 권장.
- ISO/TS 16281은 최신판에서 **ISO 16281**로 대체됨(TS 지위 해제). 인용 시 판본 확인 필요.
- 세그먼트법은 정밀하지만 FE 압력분포·상세 시뮬레이션을 요구해 계산부하가 크다. 대부분의 실무는 여전히 $C$–$P$ 기반 등가하중법 + oscillation factor(Harris/Houpert/Rumbarger)를 근사로 사용하며, 세그먼트법은 검증·연구용으로 병행되는 단계다.

---

*추가 조사 1 작성: 세그먼트 국소 손상 접근의 근거와 ISO 16281 계열 1차 문헌(Menck 2023/2024/2025, Stammler 2020, Kenworthy 2024, Escalero, Hai, NREL DG03/TP-86901) 기준.*

---

# 추가 조사 2 — 등가하중 대신 "bin별 ISO 16281 + a_ISO" 방법의 물리적 타당성·한계 및 유사 선행연구

> **제안 아이디어:** 등가하중($P_{eq}$)으로 축약하지 않고, 모든 로드케이스의 LDD(하중·회전 조건의 결합 분포)를 유지한 채 **각 조건(bin)에 대해 ISO 16281 수명과 $a_{ISO}$를 개별 평가**하고 Miner로 손상을 누적하는 방법. 이것이 더 정확한가에 대한 물리적 검토와 문헌 기반 조사.

## A2-1. 제안 방법의 형식화

각 LDD bin $i$ 를 $(F_{r,i}, F_{a,i}, M_i;\ n_i;\ \kappa_i,\ e_{C,i})$ 로 두면:

$$
L_{10mr,i} = a_1 \cdot a_{ISO,i} \cdot L_{10r,i}, \qquad a_{ISO,i}=f\!\left(\frac{e_{C,i}\,C_u}{P_i},\ \kappa_i\right)
$$

- $L_{10r,i}$ : bin $i$의 내부 하중분포로부터 얻은 **ISO 16281 기준정격수명**
- Miner 누적: $\displaystyle D=\sum_i \frac{u_i}{L_{10mr,i}}$, 총수명 $=1/D$

**등가하중 방식과의 차이:** 후자는 $P_{eq}=(\sum P_i^p n_i/\sum n_i)^{1/p}$ 를 먼저 구해 **단 한 번의** ISO 16281 + 단 하나의 $a_{ISO}(P_{eq})$ 만 적용한다.

## A2-2. 왜 bin별 방식이 더 정확한가 — "평균 후 적용 ≠ 적용 후 평균"

세 곳에서 교환법칙이 깨진다.

1. **$a_{ISO}$의 강한 비선형성 (핵심).** $a_{ISO}$는 $e_C C_u/P$와 점도비 $\kappa$의 함수로, 하중이 피로하중한계 $C_u$에 가까워지면 급격히 커지고 **최대 50에서 캡**된다. 등가하중 $P_{eq}$는 순수 멱법칙($a_{ISO}$ 상수) 가정에서 유도된 값이므로, $a_{ISO}(P)$가 변하는 순간 $\sum n_i P_i^p/(a_{ISO,i}C^p) \ne N P_{eq}^p/(a_{ISO}(P_{eq})C^p)$. 비선형 함수를 가중평균에 적용한 값 ≠ 함수값의 가중평균(**Jensen 부등식**). $a_{ISO}$가 저하중에서 치솟으므로 등가하중은 **저하중 bin의 수명 기여를 체계적으로 오추정**한다.

2. **$\kappa$·오염이 bin마다 다르다.** $\kappa=\nu/\nu_1$은 속도·온도 의존 → 운전점마다 다른 $a_{ISO}$. 등가하중 하나에는 "올바른 $\kappa$"가 없다. 롤러베어링 $a_{ISO}$는 실제로 **라미나(section)별** 계산되며, ISO/TS 16281이 클리어런스·미스얼라인먼트를 고려해 ISO 281보다 정확.

3. **ISO 16281 내부분포 자체가 외부하중에 비선형.** 클리어런스·미스얼라인먼트·복합하중이 있으면 전동체별 하중 $Q_j$·$L_{10r}$은 외부하중의 멱법칙이 아님. 하중존 폭·활성 전동체 수가 하중수준마다 다르므로 $P_{eq}$ 한 번의 실행 ≠ bin별 실행의 손상 결합.

## A2-3. 표준 근거 (제안 방법 = 정공법)

- **ISO 281:** 변동 duty cycle은 각 하중 구간마다 별도 L10 계산 후 시간가중 결합. 동하중은 대표 duty cycle/스펙트럼·피크하중 포함.
- 따라서 **bin별 계산이 표준이 정한 정공법**이고, 등가하중 $P_m$은 $a_{ISO}$·내부분포가 bin 간 일정할 때만 정확한 근사(특수해).
- **IEC 61400-4:** ISO 281 + ISO/TS 16281 병용.

## A2-4. "더 정확"의 한계 — 4가지 함정

1. **결합은 여전히 Miner 선형이고, 바로 그 부분이 의심받는다.** Kenworthy et al.(2024)은 변동조건에서 선형 손상 누적의 타당성 자체가 불확실하다고 명시. bin별 수명을 정밀화해도 결합의 선형중첩 가정은 그대로 → 하중 순서·상호작용 효과 미포착.
2. **피로한계 캡이 저하중 사각지대를 만든다.** $a_{ISO}$가 (오염보정) 피로하중한계 이하에서 사실상 무한대(캡 50)로 감 → 그 이하 bin은 손상 기여 ≈ 0. 그러나 변동진폭 피로 연구는 과부하가 피로한계를 소거해 **한계 이하 하중도 손상**을 낼 수 있음을 보임. Kenworthy는 하중이 대부분 $C_u$ 아래인 베어링에서 **수명 과대예측 위험**을 직접 지적.
3. **평균은 사라진 게 아니라 bin 내부로 옮겨갔다.** LDD bin은 보통 10분 평균 하중·속도 기반 → bin 안에서 난류 변동·하중 방향 변화는 여전히 평균. 정확도는 (하중, 속도, $\kappa$, 방향) 이산화 해상도에 수렴적으로 의존.
4. **원주방향 위치 이력 미해결.** bin별 ISO 16281은 각 bin을 고유 내부분포의 정상회전 조건으로 취급할 뿐, 실제 시간이력에서 모멘트 방향이 돌면서 **어느 궤도 세그먼트가 언제 큰 하중을 받는지**는 추적 못 함. 이건 FSM/Escalero(추가 조사 1)의 영역.

## A2-5. 정확도 사다리에서의 위치

```
① 등가하중          →   ② bin별 ISO 16281       →   ③ 유한세그먼트법(FSM)
단일 P_eq·ISO281        bin마다 aISO·Miner            세그먼트 τ₀·z₀·V
aISO 상수 가정          (← 제안 방법)                 위치이력 추적
──────────────── 정확도·계산비용·입력요구량 증가 → ────────────────
공통 한계: Miner 선형 결합 · 피로한계 이하 저하중 사각지대
         (② 도 이 두 가정을 상속 — ③ 만이 위치이력을 해결)
```

제안 방법은 **등가하중과 FSM 사이의 합리적 중간**: 표준 정합적이고 등가하중보다 명확히 정확하나, FE 비용 없이 Miner·피로한계 한계는 상속.

## A2-6. 유사 선행연구 조사 (유사도별)

**핵심 발견:** Kenworthy et al.(2024)은 메인베어링에 대해 **"일련의 정상조건 운전 케이스 + 케이스별 $a_{ISO}$ + 가중결합"**을 명시적으로 수행하며 동시에 비판한다(저하중 과대예측·선형누적 타당성). 그리고 **동일 분석을 ISO/TS 16281로 확장할 것을 권고하되 아직 미수행**임을 밝힌다. 즉 "메인베어링 + 전체 LDD + bin별 ISO 16281 + $a_{ISO}$"라는 정확한 조합은 최신 리뷰가 지목한 **미해결 프론티어**다.

| 유사도 | 문헌 | 무엇을 했나 (제안 방법과의 관계) | DOI/출처 · Scholar |
|---|---|---|---|
| ★★★ 거의 동일 (ISO 281 수준) | **Kenworthy et al. (2024)**, *WT main bearing rating lives as determined by IEC 61400-1 and ISO 281*, Wind Energy 27 | **메인베어링.** 운전점별 정상조건 + $a_{ISO}$ + 가중결합. 저하중 과대예측·선형누적 비판. **ISO 16281 확장 권고(미수행)** | `10.1002/we.2883` · [🔗](https://scholar.google.com/scholar?q=Kenworthy+wind+turbine+main+bearing+rating+lives+IEC+61400-1+ISO+281) |
| ★★★ 동일 접근 (동반 보고서) | **NREL TP-5000-86901 (2024)**, *WT Main Bearing Rating Lives* | 풍속 bin(운전점)별 $P_{eq}$·$C_u$ 비교, Weibull 풍속가중 수명 | docs.nrel.gov/docs/fy24osti/86901.pdf · [🔗](https://scholar.google.com/scholar?q=NREL+wind+turbine+main+bearing+rating+lives) |
| ★★★ bin별 ISO 16281 + $a_{ISO}$ (블레이드) | **Stammler, Menck & Schleich (2020)**, WES 5:1743 | 하중을 (M, β, θ) 다차원 bin으로 카운팅 → **각 bin에 ISO 16281 수명** + 윤활 $a_{ISO}$ → 결합. 제안 방법의 블레이드 판 | `10.5194/wes-5-1743-2020` · [🔗](https://scholar.google.com/scholar?q=Fatigue+lifetime+wind+turbine+blade+bearings+blade-dependent+load+distribution+Stammler) |
| ★★ LDD 기반 운전점별 손상누적 (메인/드라이브트레인) | **Nejad et al.** 계열 · **디지털트윈 (Springer 2023)** | LRD/LDD로 1회전=1사이클, $m=10/3$, $D=\Sigma n_i/N_i$ 운전점별 누적. 메인·HSS 베어링 손상 | `10.1007/s10010-023-00627-0` · [🔗](https://scholar.google.com/scholar?q=Nejad+wind+turbine+drivetrain+bearing+fatigue+load+revolution+distribution) |
| ★★ 메인베어링 LDD 손상기여 | **Liverud Krathe et al. (2025)**, Wind Energy | 메인베어링 반경하중 vs 누적회전수(LDD)로 bin별 손상기여 평가(Nejad 방식) | `10.1002/we.70005` · [🔗](https://scholar.google.com/scholar?q=Liverud+Krathe+main+bearing+fatigue+drivetrain+OpenFAST) |
| ★★ 유성기어 베어링 세그먼트+접촉압력+Miner | **Long-Term Probability Distribution of WT Planetary Bearing Loads** | 궤도 세그먼트별 하중사이클수·최대 접촉압력을 aeroelastic MBS 시계열에서 구해 Palmgren-Miner 피로확률 추정 | academia.edu/136871218 · [🔗](https://scholar.google.com/scholar?q=Long-Term+Probability+Distribution+Wind+Turbine+Planetary+Bearing+Loads) |
| ★★ 블레이드 베어링 국소 하중사이클 (세그먼트) | **Graßmann et al. (2021)**, *Method to determine the local load cycles of a blade bearing using flexible MBS*, Forsch. Ingenieurwes. | 각 궤도 하중분포 시계열 → 세그먼트별 하중사이클·최대 접촉압력 → Miner 피로확률 | `10.1007/s10010-021-00457-y` · [🔗](https://scholar.google.com/scholar?q=Method+local+load+cycles+blade+bearing+flexible+multi-body+simulation+Gra%C3%9Fmann) |
| ★ 세그먼트 응력 기반 (FE-MBS, 추가조사1 연계) | **Leupold et al. (2021)** · **Lopez et al. (2019)** | 세그먼트별 축소 FE 응력·다축 피로기준. 등가하중 미사용, 국소 응력이력 직접 사용 | (Menck 2024 §2 인용) · [🔗](https://scholar.google.com/scholar?q=Leupold+Lopez+blade+bearing+segment+multiaxial+fatigue+wind+turbine) |

## A2-7. 종합 판정

- 제안 방법(등가하중 축약을 건너뛰고 하중·회전 분포 유지 → 조건별 ISO 16281 + $a_{ISO}$ → Miner)은 **물리적으로 타당하고 IEC 61400-1/ISO 281 duty-cycle 정공법**이다.
- **ISO 281 수정수명 수준**의 운전점별 + $a_{ISO}$ + 결합은 Kenworthy(2024)·NREL 86901이 메인베어링에 대해 이미 수행·비판.
- **ISO 16281 수준**의 bin별 수명 + $a_{ISO}$는 Stammler/Menck(2020)이 블레이드 베어링에 대해 수행.
- **메인베어링에 대한 bin별 ISO 16281 + $a_{ISO}$의 명시적 조합**은 Kenworthy가 권고했으나 공개문헌 미수행 상태 → **제안이 겨냥하는 정확한 빈틈**.
- 단, 이 방법도 Miner 선형성·피로한계 사각지대·원주 위치이력 미해결을 상속하므로 "정답"이 아니라 **"더 나은 근사"**로 위치시켜야 한다.

---

*추가 조사 2 작성: bin별 ISO 16281 + a_ISO 방법의 물리적 타당성($a_{ISO}$ 비선형성·Jensen 부등식), 표준 근거(ISO 281 변동하중 절차·IEC 61400-4), 한계(Miner·피로한계·위치이력), 유사 선행연구(Kenworthy 2024, NREL 86901, Stammler 2020, Nejad, Liverud Krathe 2025, Graßmann 2021 등) 기준.*

---

# 추가 조사 3 — 표면피로(Surface Fatigue)의 풍력 적용 현황 및 LDD 직접적분 연구 빈틈

> **문제 제기:** [[2015_SKF_표면표면하수명모델_정독번역]]이 다루는 **표면피로**는 ISO 281이 물리적으로 모델링하지 않는다(표면하 헤르츠 피로만 모델링, 표면 효과는 $a_{ISO}$ 전역 디레이팅에 *평균*으로만 반영). 따라서 표면 손상항에는 **등가하중($P_{eq}$) 방법을 적용할 수 없고**, 각 LDD bin에 표면 응력을 직접 적분해 수명을 계산해야 한다. 이 접근이 풍력 분야에서 연구된 사례가 있는지 광범위 조사(조사일: 2026-07, 웹 1차·2차 자료 교차조사).

## A3-1. 왜 표면항에 등가하중을 쓸 수 없는가 — 전제의 검증

$P_{eq}=(\sum P_i^p n_i/\sum n_i)^{1/p}$ 는 $L_{10}=(C/P)^p$ 의 **순수 멱법칙**(지수 $p$ 고정, $a_{ISO}$ 상수)에서만 손상을 보존한다. 표면 손상함수(2015 논문 식 [9]) $G_s \propto N^m\langle\sigma_s-\sigma_{u,s}\rangle^c$ 는 이 가정을 두 방향에서 깬다.

1. **입력이 하중 크기가 아니다.** 표면 응력은 점도비 $\kappa$·하중비 $P/P_u$·거칠기·미끄럼비 $S$의 복합 함수다. $P_{eq}$ 하나로는 bin별 $\kappa_i$가 사라진다([[풍력_메인베어링_LDD_등가하중_이론정리#A2-2]]의 $a_{ISO}$ 논리와 동형).
2. **비단조 거동.** 정규화 표면 피로함수 $R_s$(2015 논문 Fig. 7)는 **저하중·저$\kappa$에서 급증**하다 $P/P_u>10$에서 포화한다. 하중의 $p$승 가중평균은 이 형상을 재현할 수 없다.

➡ 표면항 $I_s$는 **각 운전점(bin)에서 개별 평가 후 손상 누적(Palmgren–Miner)** 해야 한다. 이는 추가 조사 2의 "bin별 ISO 16281 + $a_{ISO}$" 논리를 **표면피로 축으로 확장**한 것이다.

> **뉘앙스:** 표면피로가 ISO 281에서 *완전히* 빠진 것은 아니다. $a_{ISO}$(윤활·오염 디레이팅)에 **뭉뚱그려 평균 반영**되어 있다([[2015_SKF_표면표면하수명모델_정독번역#📎 부록 B]]). 문제는 이것이 *전역·평균* 처리라 특정 표면 고장모드를 분리·정량화하지 못한다는 점 — 2015 논문의 출발점.

## A3-2. 표면/표면하 분리 수명모델을 풍력에 적용한 직접 사례

### (a) SKF GBLM — 2015 논문의 상품화, 풍력 기어박스 베어링에 실적용
- 2015 논문(표면·표면하 분리)이 **Generalized Bearing Life Model(GBLM)**로 제품화되어 SKF가 명시적으로 **풍력 기어박스 베어링**에 적용. 표면항=면적적분(surface fatigue), 표면하=헤르츠 RCF 체적적분으로 분리 계산.
- **DuraPro**(맞춤 열처리 풍력 기어박스 베어링)를 GBLM으로 "표면·표면하 피로에 더 강함"을 정량화. **DNV 감사** 결과 GBLM이 "해당 ISO 표준을 대체할 수 있다"고 인정.
- **변동조건 처리 = 사용자 제안과 동일**: GBLM은 variable conditions를 "**linear damage accumulation(Palmgren–Miner)**"으로 결합 — 단일 $P_{eq}$를 만들지 않고 **조건별 L10 계산 후 손상 누적**. 즉 "LDD별 직접 계산" 접근을 SKF가 이미 사용. 단, 표면 상수·$I_s$ 식은 **사내 독점**이라 공개논문에 미상세.

### (b) 모델 엔진 — Morales-Espejel & Brizmer(2011) 표면 디스트레스 모델
- $I_s$ 계산 엔진(혼합윤활 EHL + **Dang Van** 피로 + **Archard** 마일드 마모)이 마이크로피팅↔마모 **경쟁 메커니즘**을 시뮬. 후속으로 하이브리드 베어링(2019, 세라믹 볼)까지 동일 골격 확장.

## A3-3. 기어 쪽 — 사용자 제안 방법의 확립된 선례 (가장 강력)

풍력 **기어 마이크로피팅**은 표면피로를 변동 스펙트럼에 직접 적분하는 방법론이 **표준·논문 수준에서 이미 성숙**. 베어링보다 앞서 있다.

- **ISO/TR 15144-1** — 마이크로피팅 위험을 안전계수 $S_\lambda=\lambda_{GFP}/\lambda_{GFmin}$(국소 유막두께비)로 평가하는 최초의 국제 계산법. **GL/DNV가 풍력 기어박스 인증에서 요구**.
- **Al-Tubi & Long (2013)** — *Prediction of wind turbine gear micropitting under **variable load and speed conditions** using ISO/TR 15144-1* (Proc. IMechE Part C, DOI `10.1177/0954406212469593`). **사용자 제안의 기어 버전**: 변동 하중·속도 스펙트럼을 따라 접촉응력·미끄럼·국소온도·유막을 각 조건에서 계산해 마이크로피팅 위험 평가. "높은 접촉응력·하중변동·반복 사이클이 결정 인자".
- **경쟁 메커니즘 수치해석** — 풍력 기어 pitting vs micropitting 경쟁을 거칠기 고려해 시뮬(Zhang et al. 2019, *Eng. Failure Analysis* 104, `10.1016/j.engfailanal.2019.05.016`).
- **SCADA 기반 확률적 마이크로피팅** — 실측 운전데이터로 flank 마이크로피팅 위험 확률평가(Al-Tubi, Long et al. 2015, *IET Renewable Power Generation*, `10.1049/iet-rpg.2014.0277`).

➡ **"표면피로 + 변동 스펙트럼 + 손상누적" 조합은 기어에서 검증된 방법론.** 베어링(특히 메인베어링)으로 **이식**하는 것이 본 연구의 위치.

## A3-4. 풍력 메인베어링·드라이브트레인의 표면피로

- **메인샤프트 베어링(구면/테이퍼 롤러)**이 마이크로피팅·표면기인 피로로 **6–10년 내 조기고장** 다수 보고. 저속 베어링·기어에서 특히 문제.
- **표면공학 대응** — 코팅(DLC)·흑산화(black oxide) 기반 개선 연구(Doll 2022, *Surface & Coatings Technology* 442:128545, `10.1016/j.surfcoat.2022.128545`).
- **트라이볼로지 종합 리뷰**(Dhanola & Garg 2020, *Eng. Failure Analysis* 118:104885, `10.1016/j.engfailanal.2020.104885`)는 마이크로피팅을 WEC와 구분되는 별도 모드로 다루나, **GBLM류 분리 수명모델도 LDD 기반 표면피로 적분도 다루지 않음**(현상 기술 위주) → 리뷰조차 비어 있다는 것이 빈틈의 방증.

### (보조) WEC(백색조직균열) — "표면이냐 표면하냐" 논쟁 자체
- WEC는 풍력 기어박스 베어링 조기고장의 대표 모드. 주로 **표면하 개시**(MnS 개재물)로 분류되나 **수소·미끄럼·표면 트라이보화학**이 구동인자라 표면/표면하 경계가 흐린 대표 사례 → 2015 논문 분리모델의 문제의식과 직결. (López-Uruñuela et al. 2021, *Int. J. Fatigue* 145:106091, `10.1016/j.ijfatigue.2020.106091`)

## A3-5. 피치·요 베어링(요동) — 표면피로보다 마모·프레팅 지배
- 5m급 실규모 시험(Stammler·Schwack·Menck)은 **마모·false brinelling·프레팅**이 지배적이라 결론 → 표면피로가 주역이 아니며, 오히려 [[풍력_메인베어링_LDD_등가하중_이론정리#5. 세우는 가정과, 등가변환으로 "잃는" 효과 ⚠️]]의 "회전기반 등가하중이 못 담는 영역".
- Menck **유한세그먼트법(FSM)**은 표면하 RCF를 세그먼트 위치이력으로 정밀화하나 **표면피로 항은 미포함** → 표면피로는 여전히 별도 과제.

## A3-6. 정확도·성숙도 지도

| 축 | 확립도 | 대표 문헌 |
|---|:---:|---|
| 표면피로 분리 수명모델(이론) | ★★★ | 2015 SKF, GBLM |
| GBLM의 풍력 기어박스 베어링 적용 | ★★★ (상세 독점) | SKF DuraPro, DNV 감사 |
| 기어 마이크로피팅 + 변동 스펙트럼 적분 | ★★★ | Al-Tubi & Long 2013, ISO/TR 15144-1 |
| **메인베어링 표면피로 + LDD 직접적분** | **☆ (공개문헌 거의 공백)** | — |
| 표면피로 + Palmgren-Miner 타당성 비판 | ★★ | Kenworthy et al. 2024, M. Lewis |

## A3-7. 종합 판정

1. 표면피로의 풍력 적용 연구는 **광범위하게 존재하나, 무게중심이 "기어 마이크로피팅"과 "실패 현상 기술(WEC)"에 쏠려** 있다.
2. **표면/표면하 분리 수명모델(GBLM)을 풍력 메인베어링에 LDD 스펙트럼으로 직접 적분**한 공개 연구는 거의 공백이다 — SKF 사내엔 방법이 있으나 미공개, 학계는 ISO 281/16281(표면하 중심)에 머물고 Kenworthy(2024)가 "16281 확장"을 권고했으나 미수행. **본 연구가 겨냥하는 표면피로 축의 미개척 프론티어.**
3. 방법론적 정당성은 **기어 선례(Al-Tubi)와 GBLM의 linear damage accumulation**이 이미 뒷받침 → "베어링으로의 이식"이 합리적 연구 설계.
4. 단, 이 방법도 [[풍력_메인베어링_LDD_등가하중_이론정리#A2-4]]의 3대 한계((a) Miner 선형성 논란, (b) $I_s$ 상수의 자사 베어링 보정 필요([[2015_SKF_표면표면하수명모델_정독번역#📎 부록 D]]), (c) κ·거칠기의 bin 내 평균화)를 상속하므로 "정답"이 아니라 **"더 나은 근사"**로 위치시킬 것.

## A3-8. 주요 출처 (서지정보·DOI 검증 완료, CrossRef 대조 2026-07)

### 학술 논문 (DOI)

| # | 저자·연도 · 논문명 | 저널 (권·호·페이지) | DOI |
|---|---|---|---|
| 1 | **Morales-Espejel, G.E. & Gabelli, A. (2015)** — *A Model for Rolling Bearing Life with Surface and Subsurface Survival — Tribological Effects* | *Tribology Transactions* **58**(5): 894–906 | [10.1080/10402004.2015.1025932](https://doi.org/10.1080/10402004.2015.1025932) |
| 2 | **Morales-Espejel, G.E. & Gabelli, A. (2019)** — *Application of a rolling bearing life model with surface and subsurface survival to hybrid bearing cases* | *Proc. IMechE Part C: J. Mech. Eng. Sci.* **233**(15): 5491–5498 | [10.1177/0954406219848470](https://doi.org/10.1177/0954406219848470) |
| 3 | **Al-Tubi, I.S. & Long, H. (2013)** — *Prediction of wind turbine gear micropitting under variable load and speed conditions using ISO/TR 15144-1: 2010* | *Proc. IMechE Part C: J. Mech. Eng. Sci.* **227**(9): 1898–1914 | [10.1177/0954406212469593](https://doi.org/10.1177/0954406212469593) |
| 4 | **Kenworthy, J., Hart, E., Stirling, J., Stock, A., Keller, J., Guo, Y., Brasseur, J. & Evans, R. (2024)** — *Wind turbine main bearing rating lives as determined by IEC 61400-1 and ISO 281: A critical review and exploratory case study* | *Wind Energy* **27**(2) | [10.1002/we.2883](https://doi.org/10.1002/we.2883) |
| 5 | **López-Uruñuela, F.J., Fernández-Díaz, B., Pagano, F., López-Ortega, A., Pinedo, B., Bayón, R. & Aguirrebeitia, J. (2021)** — *Broad review of "White Etching Crack" failure in wind turbine gearbox bearings: Main factors and experimental investigations* | *International Journal of Fatigue* **145**: 106091 | [10.1016/j.ijfatigue.2020.106091](https://doi.org/10.1016/j.ijfatigue.2020.106091) |
| 6 | **Zhang, B., Liu, H., Zhu, C. & Li, Z. (2019)** — *Numerical simulation of competing mechanism between pitting and micro-pitting of a wind turbine gear considering surface roughness* | *Engineering Failure Analysis* **104**: 1–14 | [10.1016/j.engfailanal.2019.05.016](https://doi.org/10.1016/j.engfailanal.2019.05.016) |
| 7 | **Zhou, Y., Zhu, C. & Liu, H. (2019)** — *A Micropitting Study Considering Rough Sliding and Mild Wear* | *Coatings* **9**(10): 639 | [10.3390/coatings9100639](https://doi.org/10.3390/coatings9100639) |
| 8 | **Doll, G.L. (2022)** — *Surface engineering in wind turbine tribology* | *Surface and Coatings Technology* **442**: 128545 | [10.1016/j.surfcoat.2022.128545](https://doi.org/10.1016/j.surfcoat.2022.128545) |
| 9 | **Dhanola, A. & Garg, H.C. (2020)** — *Tribological challenges and advancements in wind turbine bearings: A review* | *Engineering Failure Analysis* **118**: 104885 | [10.1016/j.engfailanal.2020.104885](https://doi.org/10.1016/j.engfailanal.2020.104885) |
| 10 | **Al-Tubi, I.S., Long, H. et al. (2015)** — *Probabilistic analysis of gear flank micro-pitting risk in wind turbine gearbox using SCADA data* | *IET Renewable Power Generation* **9**(6) | [10.1049/iet-rpg.2014.0277](https://doi.org/10.1049/iet-rpg.2014.0277) |

### 표준 (DOI 없음)

| # | 표준 | 역할 |
|---|---|---|
| 11 | **ISO/TR 15144-1** (:2010 / :2014), *Calculation of micropitting load capacity of cylindrical spur and helical gears* | 마이크로피팅 안전계수 $S_\lambda$ 계산법. GL/DNV 풍력 기어박스 인증 요구 |
| 12 | **ISO 281:2007**, *Rolling bearings — Dynamic load ratings and rating life* | 표면하 RCF 기반 정격수명(표면 효과는 $a_{ISO}$ 전역 디레이팅) |

### 산업·기술 자료 (DOI 없음)

| # | 자료 | 역할 · 링크 |
|---|---|---|
| 13 | SKF, *The SKF Generalized Bearing Life Model* (Evolution #4, 2015) | GBLM 상품화·풍력 적용 — [PDF](https://evolution.skf.com/wp-content/uploads/2015/09/SKF_Generalized_Bearing_Life_Model_EN_evo415.pdf) |
| 14 | SKF, *A new bearing design (DuraPro) for wind turbine gearboxes* (Evolution) | 풍력 기어박스 베어링 GBLM 적용·DNV 감사 — [link](https://evolution.skf.com/new-bearing-design-for-wind-turbine-gearboxes/) |
| 15 | Lewis, M., *Micro-pitting of wind turbine bearings — a review* | 메인/기어박스 마이크로피팅 종설 — [LinkedIn](https://www.linkedin.com/pulse/micro-pitting-wind-turbine-bearings-review-mike-lewis) |
| 16 | Lewis, M., *The validity of the Palmgren-Miner law for rolling bearings* | Miner 선형누적 비판(기어 pitting 교훈) — [LinkedIn](https://www.linkedin.com/pulse/validity-palmgren-miner-law-rolling-bearings-lessons-mike-lewis) |
| 17 | *Bearing and gearbox failures: challenge to wind turbines*, STLE/TLT (2020) | 현장 고장·마이크로피팅 개관 — [STLE](https://www.stle.org/files/TLTArchives/2020/08_August/Feature.aspx) |
| 18 | *Calculating Micropitting for Wind Energy*, Wind Systems Magazine | ISO/TR 15144 풍력 적용 실무 해설 — [link](https://www.windsystemsmag.com/calculating-micropitting-for-wind-energy/) |

> ⚠️ 인용 신뢰수준 및 검증 이력:
> - 모든 학술 논문의 제목·저자·저널·권호·DOI는 **CrossRef API로 교차검증**(2026-07). ScienceDirect/MDPI 원문은 403 차단되어 CrossRef 서지 레코드를 1차 근거로 사용.
> - **정정 이력**: 초안에서 문헌 5(WEC 리뷰)를 *Engineering Failure Analysis* PII `S014211232030623X`로 표기했으나, CrossRef 확인 결과 실제 게재지는 **International Journal of Fatigue 145(2021)**임 — 정정 반영.
> - 문헌 10(SCADA 마이크로피팅)의 저자·권호는 IET 레코드 기반이며 정밀 인용 시 원문 재확인 권장.
> - GBLM의 풍력 메인베어링 **LDD별 표면항 적분** 구체 절차는 SKF 사내 독점으로 공개 미상세(자료 13·14는 제품·개념 설명 수준). Al-Tubi & Long(2013)은 **기어** 대상이며, 베어링 직접 이식은 본 연구가 메울 빈틈.
> - WEC의 표면/표면하 개시 비중은 문헌 간 이견 존재.

---

*추가 조사 3 작성: 표면피로의 풍력 적용 현황 광범위 조사 — 등가하중 부적용 전제 검증(표면 손상함수 비단조성), 분리 수명모델의 풍력 적용(GBLM/DuraPro·DNV), 기어 마이크로피팅 변동스펙트럼 선례(Al-Tubi 2013·ISO/TR 15144), 메인베어링·WEC·피치베어링 현황, 미개척 빈틈(메인베어링 표면피로 LDD 직접적분) 식별. 기준.*

---

# 추가 조사 4 — 표면 거칠기의 공학적 모사: 측정치 의존 문제와 처리·생성 방법론 기반 연구방향

> **문제 제기:** 표면피로/마이크로피팅([[Test. 2011. (SKF) Micropitting Modelling in Rolling-Sliding Contacts]], [[2015_SKF_표면표면하수명모델_정독번역]])와 Mixed EHL([[분석_MixedEHL_공학적활용]]) 모델은 모두 **실측(measured) 3D 거칠기**를 입력으로 쓴다. 이는 *특정 모델의 정확성 검증*엔 적합하나, 모델을 **일반 설계도구로 실용화**하려면 단일 측정 realization이 아니라 **정합성 있는(통계적으로 대표·재현 가능한) 거칠기 특성화·생성**을 통해 성능을 검토해야 한다. 핵심 이론적 논점: 높이분포를 가우시안으로 두더라도 **위치(공간)분포를 알려면 PSD 또는 ACF가 규정되어야** 한다. 본 조사는 기존 **통계적/결정론적 거칠기 처리 방법**과 **거칠기 생성 방법(Gaussian·Non-Gaussian·Fractal·Hybrid)**을 포괄해 정리하고 연구방향을 검토한다(조사일: 2026-07).

## A4-1. 왜 "높이분포만으로 부족"한가 — 표면의 완전 통계기술은 2축

거칠기 표면 $z(x,y)$의 통계기술은 **독립적인 두 축**을 요구한다.

| 축 | 규정 대상 | 파라미터 | 물리적 의미 |
|---|---|---|---|
| **진폭(높이)** | 높이 확률밀도 PDF | $R_q(\sigma)$, 왜도 $S_k$, 첨도 $K_u$ | 돌기가 "얼마나 높은가" |
| **공간(위치)** | 자기상관 ACF / 파워스펙트럼 PSD | 상관길이 $\beta^*_x,\beta^*_y$, 스펙트럼 모멘트 $m_0,m_2,m_4$, 이방성(lay) | 돌기가 "어떻게 배열·연결되는가" |

**핵심 정리(사용자 논점의 근거):** 두 축은 **독립**이다 — 같은 $\sigma$(높이분포)라도 ACF가 다르면 완전히 다른 표면이다. Gaussian 랜덤필드는 **ACF(=PSD)만으로 완전히 규정**되므로(Whitehouse & Archard 1970), "가우시안 높이분포 가정"은 진폭축만 고정할 뿐 **공간축(ACF/PSD)을 별도로 규정해야 표면이 결정**된다. 실제 궤도면은 연삭·호닝·런인으로 **음의 왜도($S_k<0$)** 를 갖는 비가우시안이라 진폭축도 고차 통계가 필요하다.

> **왜 공간축이 표면피로를 지배하나:** 조도의 각 파장 하모닉은 깊이 $\sim\lambda/2\pi$ 까지만 침투한다([[분석_MixedEHL_공학적활용#A1.3]] 식 12·13, 깊이감쇠 $e^{-\zeta z}$, $\zeta=2\pi/\lambda$). **단파장(고주파) 성분이 근표면 응력집중을 만들어 마이크로피팅을 개시**시키므로, 표면피로 결과는 사실상 거칠기의 **PSD(파장별 에너지 분포)**가 지배한다. 따라서 "정합적 거칠기"란 곧 **대표 PSD·height PDF의 정합적 규정**이다.

## A4-2. 기존 거칠기 처리(treatment) 방법 — 각 방법이 요구하는 거칠기 표현

| 계열 | 대표 방법 | 거칠기 입력 형태 | 국부 응력 스파이크 | 표면피로 6응력성분 |
|---|---|---|---|---|
| **통계적(평균)** | Patir–Cheng 평균유동(flow factor) / Greenwood–Williamson·Tripp 돌기접촉 | 저차 통계 $\sigma$·pattern $\gamma$(=상관길이비)·summit 통계($\beta,\eta_s$) | ✗ 평균화로 소멸 | ✗ (스칼라 하중분담·마찰까지만) |
| **결정론(spectral)** | Hu–Zhu 통합 Reynolds / Morales–Espejel 진폭감쇠(amplitude reduction) | **전체 스펙트럼**(측정맵 또는 규정 PSD) → 푸리에 성분 | ✓ 해상됨 | ✓ (근표면 von Mises 첨두) |

- **통계적 방법**은 거칠기를 **저차 파라미터로 축약**하므로 개별 realization이 불필요하다는 장점이 있으나, 기대값 적분에서 개별 돌기 첨두가 평균 속으로 사라져 **표면피로용 응력집중을 주지 못한다**([[분석_MixedEHL_공학적활용#A1.0]] 보론).
- **결정론 방법**(2011 SKF 모델의 코어)은 거칠기를 **푸리에 하모닉으로 분해→각 성분 감쇠→중첩**하므로 국부 응력이력을 준다. 즉 **입력 스펙트럼(PSD)이 결과를 직접 지배** → 정합적 PSD 규정이 결정적. 단 측정맵 사용 시 **realization 의존**이 문제.

➡ **treatment 방법은 이미 확립**되어 있고, 미결 과제는 그 입력인 **거칠기를 어떻게 정합적으로 규정·생성하는가**로 환원된다.

## A4-3. 거칠기 생성(generation) 방법 — 측정 realization을 대체·앙상블화

측정맵 의존을 벗어나 **지정 통계량을 만족하는 거칠기를 생성**하는 4계열. 모두 **앙상블(다수 realization)** 생성이 가능해 확률적 수명분산의 입력이 된다.

| # | 계열 | 원리 | 규정 통계량 | 대표 문헌 |
|---|---|---|---|---|
| 1 | **선형변환/스펙트럴 (Gaussian)** | 목표 PSD(또는 ACF)에 랜덤위상 부여 → IFFT | PSD/ACF (높이=가우시안) | Patir (1978); Wu (2000); spectral representation |
| 2 | **Non-Gaussian** | 디지털필터 계수를 목표 ACF+moment에 맞춤 / Johnson 변환으로 $S_k,K_u$ 부여 | PSD/ACF **+** 비가우시안 높이 PDF | Hu & Tonder (1992); Bakolas (2003) |
| 3 | **Fractal (self-affine)** | Weierstrass–Mandelbrot 함수, 다중스케일 자기유사 | 프랙탈 차원 $D$·스케일계수 $G$ (단일 파라미터쌍) | Majumdar & Bhushan (1990); Yan & Komvopoulos (1998); Wang et al. (2021) |
| 4 | **Hybrid** | PSD와 지정 높이 PDF를 **동시** 만족(반복 알고리즘) | PSD **AND** height PDF 정확 동시 | Pérez-Ràfols & Almqvist (2019); Chen et al. (2025) |

- **1(스펙트럴)**: 가장 단순·표준. 가우시안 한정.
- **2(Non-Gaussian)**: 궤도면의 음의 왜도를 반영 — 실제 베어링 표면에 필수. 단 ACF와 height PDF를 동시에 정확히 맞추기 어려움(순차 부여 시 상호 왜곡).
- **3(Fractal)**: 소수 파라미터($D,G$)로 다중스케일 표현. 단 **단일 $D$가 전 파장에서 자기유사**를 가정 → 파장별 스펙트럼이 다른 실제 가공표면(연삭+호닝 중첩)과 불일치 가능.
- **4(Hybrid)**: 2·1의 한계(PSD↔PDF 동시 규정)를 반복법으로 해소 — **가장 정합적**. 최신 방향.

## A4-4. 연구 프레이밍 — 측정 realization → 통계적 대표성

- 측정 거칠기 1샘플은 **특정 realization**이다. 표면피로는 **극치 돌기(worst asperity)**에 민감하므로 realization마다 손상·수명 분산이 크다 → 표면수명 Weibull 분산의 일부는 **거칠기 realization 분산**에서 기원.
- 따라서 "정합적 거칠기"란 **궤도면 가공등급(연삭/호닝/슈퍼피니싱)별 대표 통계량**($\sigma, S_k, K_u$, PSD/ACF, lay)을 규정하고, 그로부터 **생성 앙상블**을 만들어 모델을 구동하는 것이다.
- SKF GBLM도 여러 measured roughness에 대한 **파라메트릭 스터디로 표면항을 curve-fit**([[2015_SKF_표면표면하수명모델_정독번역#📎 부록 D]]) → 사실상 통계적 대표화를 수행. 다만 그 결과가 **상수로 블랙박스화**되어 조건·표면 변화 시 재현이 어렵다. 생성 기반 접근은 이 과정을 **투명·재현 가능**하게 만든다.

## A4-5. 연구방향성 (구체)

1. **거칠기 특성화 프로토콜 확립** — 메인베어링 궤도면 실측 → PSD·ACF·height PDF($S_k,K_u$)·스펙트럼 모멘트($m_0,m_2,m_4$, Nayak 1971)·이방성(lay)을 **ISO 25178 areal 파라미터**로 정량화. 가공등급별 대표값 라이브러리.
2. **생성기 선택·검증** — 4계열 중 궤도면 통계(비가우시안·이방성·다중스케일)에 부합하는 방법 채택(유력: Hybrid Pérez-Ràfols 2019 / Non-Gaussian Bakolas 2003). **측정 거칠기 해석 결과 ↔ 동일 통계량 생성 거칠기 해석 결과의 통계적 동치성** 검증이 핵심 관문.
3. **처리–생성 연계 파이프라인** — 통계적(Patir–Cheng/GW–GT)으로 $\lambda$·하중분담·마찰 **스크리닝** → 결정론(진폭감쇠, Hooke–Li 2006)으로 국부 표면응력·마이크로피팅 **정밀해석**([[분석_MixedEHL_공학적활용#A1.5]] 파이프라인 재사용).
4. **거칠기 파라미터 → 표면피로 민감도 맵** — $\sigma, S_k, K_u, \gamma$(lay), 프랙탈 $D$가 마이크로피팅·표면수명에 미치는 영향 정량화(Rycerz–Kadiric의 SRR·[[분석_MixedEHL_공학적활용#5.4]] 조도지배 실증과 연계).
5. **LDD bin별 표면피로 적분(추가 조사 3)의 입력 공급** — 각 bin의 $\kappa_i$·운전조건에 정합적 거칠기 **앙상블**을 공급 → 표면항 손상 누적 + **확률적 표면수명 분산** 산출. 추가 조사 3의 미개척 빈틈(메인베어링 표면피로 LDD 직접적분)에 **정합적 입력**을 제공하는 것이 본 조사의 위치.
6. **마모–거칠기 진화 결합** — 생성 거칠기 → 런인/마일드 마모(2011 SKF Archard, [[분석_MixedEHL_공학적활용#7]]) → 재특성화 → 위험 재평가.

## A4-6. 한계·주의 (연구 설계 시 명시)

1. **통계적 방법의 평균화**: 표면피로 6응력성분을 못 주므로, 표면피로 정량화는 반드시 결정론(생성 거칠기) 경로 필요.
2. **PSD↔height PDF 동시 규정의 난점**: 순차 부여(스펙트럴 후 비가우시안 변환)는 한쪽을 왜곡 → Hybrid 반복법도 근사. 검증 필수.
3. **프랙탈의 단일 $D$ 가정**: 실제 가공표면은 파장대별 스펙트럼 기울기가 상이(bi-fractal) → 단일 $D$ 부적합 가능. PSD 직접 규정이 더 안전.
4. **이방성(lay)**: 궤도면은 강한 방향성(원주 방향 연삭결). 등방 생성기는 부적합 → 이방 ACF/PSD 필수(Bakolas 2003 arbitrarily oriented).
5. **거칠기 진화**: 초기 거칠기 특성화만으로 장기 수명 예측 불가 — 마모/런인 진화 반영 필요.
6. **realization 앙상블 크기**: 극치 민감 손상이므로 소수 realization은 통계 불안정 — 앙상블 수렴 확인 필요.

## A4-7. 선행연구 — "거칠기 모델 + 기어·베어링 피로" 적용 사례 (광범위 조사)

> **핵심 질문:** 측정 거칠기가 아니라 **통계적으로 생성·규정한 거칠기 모델**(비가우시안 $S_k/K_u$, ACF/PSD, 프랙탈)을 **기어·베어링의 접촉피로·마이크로피팅 수명 계산**에 실제로 적용한 연구가 있는가?
> **답:** **있다 — 특히 기어 쪽에 직접 선례가 존재하며, 사용자 구상의 핵심(생성 거칠기 → Mixed EHL → 다축피로 수명)을 그대로 수행한 논문이 있다.** 즉 본 접근은 "신규 방법론"이 아니라 **이미 검증된 방법론**이며, 신규성은 그 이식 대상(풍력 메인베어링 LDD 표면피로)에 있다.

### (a) ★ 가장 직접적 — 생성 비가우시안 거칠기 → Mixed EHL → 피로수명

- **Yan, Wang & Zhang (2014, *ASME J. Tribology*):** 지정 **ACF·왜도 $S_k$·첨도 $K_u$ 로 비가우시안 거칠기를 수치 생성** → Mixed EHL 점접촉 → 아표면 응력 → **피로수명** 계산. 결과: **$S_k\uparrow$ 일수록 최대압력·von Mises·수명 저하**, $K_u$ 는 수명에 **비단조 변동**, $S_k\!\leftrightarrow\!K_u$ 교호작용이 강함. → **사용자 구상의 거의 완전한 선례**(단 일반 점접촉, 풍력·LDD·표면하 분리 미적용). DOI [10.1115/1.4027480](https://doi.org/10.1115/1.4027480)
- **Zhao et al. (2022, *Lubrication Science*):** Johnson 변환으로 **비가우시안 생성 거칠기** → Mixed lubrication **마모** 예측(피로 아닌 마모축이나 *동일 입력 철학* — 생성 거칠기의 통계량이 트라이볼로지 결과를 지배). DOI [10.1002/ls.1580](https://doi.org/10.1002/ls.1580)
- **Gao et al. (2024, *Mechanics of Solids*):** **$S_k$·$K_u$·ACF** 가 비-Hertz 압력분포와 **von Mises 깊이분포**에 미치는 영향(접촉→피로의 전단계 응력장). DOI [10.1134/S0025654424604245](https://doi.org/10.1134/S0025654424604245)

### (b) 프랙탈 거칠기 → 기어 접촉·윤활

- **Zhao et al. (2022, *Machines*):** **3D 프랙탈(Weierstrass–Mandelbrot) 거칠기**로 스퍼기어 EHL 물림특성 해석(수명 직접계산은 아니나 프랙탈 생성 거칠기의 기어 접촉 적용). DOI [10.3390/machines10080705](https://doi.org/10.3390/machines10080705)
- 생성법 계보(A4-3 표): Majumdar–Bhushan (1990), Yan–Komvopoulos (1998), Wang et al. (2021).

### (c) 거칠기(스펙트럼/측정) → 기어 RCF 수명 — 풍력 포함

- **Liu et al. (2020, *Friction*):** **풍력 기어쌍** 접촉피로를 거칠기 고려해 계산 — 풍력 표면피로에 직접 근접. DOI [10.1007/s40544-019-0277-3](https://doi.org/10.1007/s40544-019-0277-3)
- **Everitt & Alfredsson (2020, *Tribology International*):** 거칠기(shot-peened/ground/worn) → 열-EHL+slip → RCF. **거칠기 스펙트럼이 근표면 응력·수명을 지배**함을 정량 입증. DOI [10.1016/j.triboint.2020.106394](https://doi.org/10.1016/j.triboint.2020.106394)

### (d) 거칠기 파라미터 ↔ 마이크로피팅 상관 (생성 파라미터 선정의 실증 근거)

- **Roy et al. (2018, *Surface & Coatings Technology*):** 마이크로피팅 진행 시 **$S_k$ 감소·$K_u$ 증가** — $S_k/K_u$ 가 표면피로 진행의 진단지표임을 실측(생성 거칠기에서 어떤 통계량을 규정해야 하는지의 근거). DOI [10.1016/j.surfcoat.2018.05.083](https://doi.org/10.1016/j.surfcoat.2018.05.083)
- **Bergstedt et al. (2020, *Proc. IMechE Part C*):** 매끈면이 마이크로피팅↓이나 **pitting 수명↓** 트레이드오프(거칠기 최적화의 양면성). DOI [10.1177/0954406220931541](https://doi.org/10.1177/0954406220931541)

### 종합 판정

1. **비가우시안($S_k/K_u$)·ACF 생성 거칠기 → 피로수명**은 **Yan et al.(2014)가 이미 확립** → 사용자 구상은 방법론적으로 **검증됨**(신규 위험 낮음).
2. **프랙탈·PSD 생성 거칠기의 기어 접촉·윤활 적용**도 존재(Zhao 2022, Everitt 2020).
3. 그러나 이들은 **일반 점/선접촉·기어** 대상이며, **① 풍력 메인베어링 ② LDD 변동스펙트럼(bin별) ③ 표면/표면하 분리(GBLM)** 로 결합한 사례는 **없음** → [[풍력_메인베어링_LDD_등가하중_이론정리#추가 조사 3]]의 빈틈과 정확히 연결.
4. **본 연구의 위치 = "확립된 생성 거칠기–피로 방법론(Yan 2014류)"을 "풍력 메인베어링 LDD 표면피로 적분(추가 조사 3)"에 이식** — 방법은 검증되어 있고, 적용 대상이 미개척인 형태의 명확한 신규 기여.

## A4-8. 참고문헌 (서지·DOI 검증 완료, CrossRef 대조 2026-07)

### 거칠기 통계 특성화 (characterization)

| # | 저자·연도 · 논문명 | 저널 (권·호·페이지) | DOI |
|---|---|---|---|
| 1 | **Nayak, P.R. (1971)** — *Random Process Model of Rough Surfaces* | *J. Lubrication Technology* **93**(3):398–407 | [10.1115/1.3451608](https://doi.org/10.1115/1.3451608) |
| 2 | **Whitehouse, D.J. & Archard, J.F. (1970)** — *The properties of random surfaces of significance in their contact* | *Proc. R. Soc. Lond. A* **316**(1524):97–121 | [10.1098/rspa.1970.0068](https://doi.org/10.1098/rspa.1970.0068) |
| 3 | **Persson, B.N.J., Albohr, O., Tartaglino, U., Volokitin, A.I. & Tosatti, E. (2004)** — *On the nature of surface roughness with application to contact mechanics, sealing, rubber friction and adhesion* | *J. Phys.: Condens. Matter* **17**(1):R1–R62 | [10.1088/0953-8984/17/1/R01](https://doi.org/10.1088/0953-8984/17/1/R01) |

### 거칠기 생성 (generation) — 계열별

| # | 계열 | 저자·연도 · 논문명 | 저널 (권·호·페이지) | DOI |
|---|---|---|---|---|
| 4 | 스펙트럴(Gaussian) | **Patir, N. (1978)** — *A numerical procedure for random generation of rough surfaces* | *Wear* **47**(2):263–277 | [10.1016/0043-1648(78)90157-6](https://doi.org/10.1016/0043-1648(78)90157-6) |
| 5 | 스펙트럴(FFT) | **Wu, J-J. (2000)** — *Simulation of rough surfaces with FFT* | *Tribology International* **33**(1):47–58 | [10.1016/S0301-679X(00)00016-5](https://doi.org/10.1016/S0301-679X(00)00016-5) |
| 6 | Non-Gaussian | **Hu, Y.Z. & Tonder, K. (1992)** — *Simulation of 3-D random rough surface by 2-D digital filter and Fourier analysis* | *Int. J. Machine Tools & Manufacture* **32**(1–2):83–90 | [10.1016/0890-6955(92)90064-N](https://doi.org/10.1016/0890-6955(92)90064-N) |
| 7 | Non-Gaussian | **Bakolas, V. (2003)** — *Numerical generation of arbitrarily oriented non-Gaussian three-dimensional rough surfaces* | *Wear* **254**(5–6):546–554 | [10.1016/S0043-1648(03)00133-9](https://doi.org/10.1016/S0043-1648(03)00133-9) |
| 8 | Fractal | **Majumdar, A. & Bhushan, B. (1990)** — *Role of Fractal Geometry in Roughness Characterization and Contact Mechanics of Surfaces* | *J. Tribology* **112**(2):205–216 | [10.1115/1.2920243](https://doi.org/10.1115/1.2920243) |
| 9 | Fractal(3D W–M) | **Yan, W. & Komvopoulos, K. (1998)** — *Contact analysis of elastic-plastic fractal surfaces* | *J. Applied Physics* **84**(7):3617–3624 | [10.1063/1.368536](https://doi.org/10.1063/1.368536) |
| 10 | Fractal(spectral) | **Wang, Y., Azam, A., Wilson, M.C.T., Neville, A. & Morina, A. (2021)** — *Generating fractal rough surfaces with the spectral representation method* | *Proc. IMechE Part J* **235**(12):2640–2653 | [10.1177/13506501211049624](https://doi.org/10.1177/13506501211049624) |
| 11 | Hybrid(PSD+PDF) | **Pérez-Ràfols, F. & Almqvist, A. (2019)** — *Generating randomly rough surfaces with given height probability distribution and power spectrum* | *Tribology International* **131**:591–604 | [10.1016/j.triboint.2018.11.020](https://doi.org/10.1016/j.triboint.2018.11.020) |
| 12 | Hybrid(spectral+iter) | **Chen, J., Zang, F., Zhao, X. et al. (2025)** — *Generating non-Gaussian rough surfaces using analytical functions and spectral representation method with an iterative algorithm* | *Applied Mathematical Modelling* **137**:115665 | [10.1016/j.apm.2024.115665](https://doi.org/10.1016/j.apm.2024.115665) |

### 거칠기 모델의 기어·베어링 피로 적용 — 선행연구 (A4-7)

| # | 저자·연도 · 논문명 | 저널 (권·호·페이지) | DOI | 거칠기 모델 |
|---|---|---|---|---|
| 13 | **Yan, X-L., Wang, X-L. & Zhang, Y-Y. (2014)** — *Influence of Roughness Parameters Skewness and Kurtosis on Fatigue Life Under Mixed EHL Point Contacts* | *J. Tribology* **136**(3):031503 | [10.1115/1.4027480](https://doi.org/10.1115/1.4027480) | 생성 비가우시안(ACF+$S_k$+$K_u$) → 피로수명 ★ |
| 14 | **Zhao, J., Li, Z., Zhang, H. & Zhu, R. (2022)** — *Prediction of the tribological characteristics of non-Gaussian rough surfaces during sliding wear in mixed lubrication* | *Lubrication Science* **34**(2):63–79 | [10.1002/ls.1580](https://doi.org/10.1002/ls.1580) | 생성 비가우시안(Johnson) → 마모 |
| 15 | **Gao, Z., Liu, M., Dong, H., Wang, W. & Fu, W. (2024)** — *Investigation on Contact Behaviors Exhibited by Non-Gaussian Rough Surfaces* | *Mechanics of Solids* **59**(4) | [10.1134/S0025654424604245](https://doi.org/10.1134/S0025654424604245) | 생성 비가우시안 → 압력·von Mises |
| 16 | **Zhao, Z., Yang, Y., Han, H., Ma, H., Wang, H. & Li, Z. (2022)** — *Meshing Characteristics of Spur Gears Considering Three-Dimensional Fractal Rough Surface under EHL* | *Machines* **10**(8):705 | [10.3390/machines10080705](https://doi.org/10.3390/machines10080705) | 프랙탈(W–M) → 기어 EHL |
| 17 | **Liu, H., Liu, H., Zhu, C., Sun, Z. & Bai, H. (2020)** — *Study on contact fatigue of a wind turbine gear pair considering surface roughness* | *Friction* **8**(3):553–567 | [10.1007/s40544-019-0277-3](https://doi.org/10.1007/s40544-019-0277-3) | 거칠기 고려 → 풍력 기어 접촉피로 |
| 18 | **Everitt, C-M. & Alfredsson, B. (2020)** — *The influence of gear surface roughness on rolling contact fatigue under thermal EHL with slip* | *Tribology International* **151**:106394 | [10.1016/j.triboint.2020.106394](https://doi.org/10.1016/j.triboint.2020.106394) | 거칠기 스펙트럼 → 기어 RCF |
| 19 | **Roy, S., White, D. & Sundararajan, S. (2018)** — *Correlation between evolution of surface roughness parameters and micropitting of carburized steel under boundary lubrication condition* | *Surface & Coatings Technology* **350**:445–452 | [10.1016/j.surfcoat.2018.05.083](https://doi.org/10.1016/j.surfcoat.2018.05.083) | $S_k/K_u$ ↔ 마이크로피팅(실증) |
| 20 | **Bergstedt, E., Lin, J. & Olofsson, U. (2020)** — *Influence of gear surface roughness on the pitting and micropitting life* | *Proc. IMechE Part C* **234**(24):4953–4961 | [10.1177/0954406220931541](https://doi.org/10.1177/0954406220931541) | 측정 거칠기 → pitting↔micropitting |

### 거칠기 처리(treatment)·표면피로 — 관련 노트에서 검증됨(재인용)

| 문헌 | 역할 | 소재 노트 |
|---|---|---|
| Patir & Cheng (1978/1979) 평균유동; Greenwood–Williamson (1966); Greenwood–Tripp (1970–71) | 통계적 처리 | [[분석_MixedEHL_공학적활용#2.1]]–2.2 |
| Hu & Zhu (2000) 통합 Reynolds; Greenwood–Morales-Espejel (1994); Morales-Espejel (2014) 진폭감쇠 리뷰; Hooke–Li (2006) | 결정론 처리(스펙트럴) | [[분석_MixedEHL_공학적활용#2.3]], A1.2 |
| Morales-Espejel & Brizmer (2011), *Tribology Transactions* 54(4):625–643 | 마이크로피팅(측정 거칠기+Dang Van+Archard) | [[Test. 2011. (SKF) Micropitting Modelling in Rolling-Sliding Contacts]] |
| Morales-Espejel, Gabelli & de Vries (2015), *Tribology Transactions* 58(5):894–906 | GBLM 표면항(측정 거칠기 파라메트릭) | [[2015_SKF_표면표면하수명모델_정독번역]] |

> ⚠️ 인용 신뢰수준: 생성·특성화 문헌 1–12의 서지는 **CrossRef API 교차검증**(2026-07). 처리(treatment) 문헌은 [[분석_MixedEHL_공학적활용]] 2·A1장에서 이미 1차 검증된 것을 재인용. Persson et al.(2004)의 게재연은 J. Phys. Condens. Matter 17권(2005 표기 문헌 다수)이며 CrossRef 등록연도는 2004 — 인용 시 17(1):R1 확인. Chen et al.은 2025 게재(온라인 2024). ISO 25178(areal)·ISO 4287(profile)은 표준(DOI 없음).

---

*추가 조사 4 작성: 표면 거칠기의 공학적 모사 — 측정치 realization 의존 문제 정식화(진폭축 height PDF × 공간축 PSD/ACF의 독립성, 가우시안 가정만으론 미결정), 기존 통계적/결정론적 처리 방법의 거칠기 요구 정리, 생성 방법 4계열(스펙트럴·Non-Gaussian·Fractal·Hybrid) 포괄, 측정→통계적 대표성 프레이밍, 추가 조사 3(LDD 표면피로 적분) 입력 연계 연구방향 6항 및 한계 6항. 기준.*
*(A4-7 선행연구 추가: "거칠기 모델 + 기어·베어링 피로" 적용 사례 광범위 조사 — 생성 비가우시안(ACF·Sk·Ku)→Mixed EHL→피로수명 직접 선례 Yan et al.2014 식별, 프랙탈·PSD 기어 접촉피로(Zhao 2022·Everitt 2020·Liu 2020 풍력기어), Sk/Ku↔마이크로피팅 실증(Roy 2018), 문헌 13–20 DOI 검증. 본 연구=검증된 생성거칠기-피로 방법론의 풍력 메인베어링 LDD 표면피로 이식으로 위치. 기준.)*
