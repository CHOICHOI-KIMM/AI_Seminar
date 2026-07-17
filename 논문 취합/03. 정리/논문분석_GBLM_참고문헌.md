# GBLM 체계 분석 — 참고문헌·모델 상세 정리 (2015/2019/2023 원문 정독 기반)

> **분석 대상**: SKF **Generalized Bearing Life Model (GBLM)** 3부작
> - **2015** Morales-Espejel, Gabelli & de Vries, *A Model for Rolling Bearing Life with Surface and Subsurface Survival — Tribological Effects*, Tribol. Trans. 58, 894–906. → **GBLM 원형(하중기반·베어링)**
> - **2019** Gabelli & Morales-Espejel, *A model for hybrid bearing life with surface and subsurface survival*, Tribol. Int. → **★중심 논문(하이브리드 세라믹 베어링)**
> - **2023** Morales-Espejel & Félix-Quiñónez, *A stress-based model for general rolling contacts with surface and subsurface survival: Application to gears*, Tribol. Int. → **응력기반 일반화(기어·캠)**
> - **기반**: 2011 Morales-Espejel & Brizmer, *Micropitting Modelling in Rolling–Sliding Contacts* → GBLM의 **표면응력 물리엔진**(별도 P3 구현 대상)
>
> **원문 MD**(모두 `03. 정리/`): `2015. (SKF) A Model for Rolling Bearing Life with Surface and Subsurface_1745a3.md` · `2019. (SKF) A model for hybrid bearing life with surface and subsurface survival.md` · `2023. (SKF) A stress-based model for general rolling contacts with surfac_f4e12a.md`
> **문서 성격**: 모델 **구현**이 아닌 GBLM **이론 체계 해부 + 참고문헌 계보 정리**. 모든 식·상수는 원문 식번호·절로 소급(추적성). 값이 원문 미제공이면 §6 미결로 표기.
> **작성 기준일**: 2026-07-15

---

## 0. GBLM 핵심 수식 체계 (지배식 + 논문별 식번호 대응)

GBLM의 골격은 세 논문에서 **동일**하며, 표기·적용대상·간이화 수준만 다르다. 아래는 지배식과 각 논문 식번호 매핑이다.

| # | 지배 개념 | 수식(대표형) | 2015 | 2019 | 2023 |
|---|---|---|---|---|---|
| G0 | **직렬 신뢰성 곱법칙** (Weibull 최약链) | $S=\prod_i S_i$, $\ln\frac1S=\sum_i \ln\frac1{S_i}$ | [1][2][3] | (1)(2) | — |
| G1 | **손상 적분형 생존확률** ($\Delta V\to0$) | $\ln\frac1{S(N)}=\int_{V}G_v(N)\,dV$ | [4][5] | (3)(4) | — |
| G2 | **표면/부표면 분리** (얇은층 $\hat h$) | $\ln\frac1S=\int_{V_v}G_v dV_v+\hat h\int_A G_s\,dA$ | [6][7] | (5) | — |
| G3 | **부표면 멱법칙** (Ioannides-Harris) | $G_v=\bar A N^e\dfrac{\langle\sigma_v-\sigma_{u,v}\rangle^{c}}{z^{h}}$ | [8][11] | (6) | (1) |
| G4 | **표면 손상함수** | $\hat h\int_A G_s dA=\bar B N^{m}\int_A\langle\sigma_s-\sigma_{u,s}\rangle^{c}dA$ | [9][12] | (7) | (2) |
| G5 | **생존확률 통합식** | $\ln\frac1S=N^e[\bar A\!\int\!\frac{\langle\cdot\rangle^c}{z^h}dV+\bar B\!\int\!\langle\cdot\rangle^c dA]$ | [13] | (8) | (2) |
| G6 | **수명 해(단일접촉)** | $L=\frac{[\ln(1/S)]^{1/e}}{u}[\cdots]^{1/e}$ | [14][16] | (8) | (3) |
| G7 | **표면손상적분** $I_s$ | $I_s=\bar B\int_A\langle\sigma_s-\sigma_{u,s}\rangle^{c}dA$ | [23] | (10) | (4) |
| G8 | **$I_s$ 해석근사 (지수형, 5상수)** | $u^{?}I_s\approx f_1\exp[\frac{f_2}{(P/P_u)^{f_3}}+\frac{f_4}{(P/P_u)^{f_5}}]$ | [31] | (11) | (5) |
| G9 | **표면손상비 / 표면피로지표** | $S_R=I_s/(I_s+I_{ss})$ (2023) | [32] | Fig.6 | (8)(9) |

**세 논문의 근본 차이(한 줄)**:
- **2015 = 하중기반(load-based) 베어링 수명식** — $L_{10}$이 동적정격하중 $C$와 $P/P_u$의 함수(식[29]). 표면항 $I_s$를 지수형 곡선적합(식[31]).
- **2019 = 하중기반 + 하이브리드 특화** — 표면응력지수 $I_s^*=f(P_r,\eta_{env})$를 세라믹-강 계면에 맞춰 별도 데이터블록으로 확보(식(11)(12)).
- **2023 = 응력기반(stress-based) 일반화** — 실제 **시간이동·시간가변 응력장**을 직접 적분(식(3)), 표면항을 **3상수 tanh 간이식**(식(6))으로 대체, 기어·캠 등 임의 접촉으로 확장.

---

## 1. GBLM 구성요소별 상세 + 참고문헌 체인

GBLM은 "확률론 골격 × 부표면 엔진 × 표면 엔진 × 하중/응력 변환 × 교정"의 조립체다. 요소별로 원 유도 참고문헌과 세 논문의 역할을 정리한다.

### C1. 확률론적 생존 골격 (Weibull 최약链 + 곱법칙)
- **원리**: 접촉을 직렬(chain) 요소계로 보고, 전체 생존확률 = 각 요소 생존확률의 곱. 로그 취해 손상적분으로 전환.
- **원 출처**: Weibull 1939 [강도 통계], Lundberg-Palmgren 1947/1952 [베어링 동적용량에 곱법칙 최초 적용].
- **논문 역할**: 2015 §"PROBABILISTIC DAMAGE APPROACH" 식[1]~[5]에서 정본 유도. 2019 §2.1 식(1)~(4) 재서술. 2023은 생략하고 [18][19] 인용으로 대체.
- **핵심 규약**: 표준 Weibull 기울기 채택 시 $m=e$ → $(uL)^{m-e}=1$로 수명해가 **명시적(explicit)**(2015 식[16] 주석). $m\ne e$면 반복 필요·비표준 분포.

### C2. 부표면 피로 엔진 (Ioannides-Harris 멱법칙)
- **원리**: Hertz 응력장에서 유발되는 임계응력진폭 $\sigma_v$(피로유발응력)와 피로한계 $\sigma_{u,v}$로 부표면 체적손상 적분. 깊이가중 $z^h$ 포함.
- **지배식**: $\int_{V_v}G_v dV_v=\bar A N^e\int_{V_v}\dfrac{\langle\sigma_v-\sigma_{u,v}\rangle^{c}}{z^{h}}dV_v$ (2015 식[11], 2019 식(6), 2023 식(1)).
- **원 출처**: **Ioannides-Harris 1985** [피로한계 $\sigma_u$ 도입, LP 확장] — GBLM 부표면항의 정본. 보조: Lundberg-Palmgren 1947/1952, Ioannides et al. 1999 [해석적 정식화], Ioannides 1985(SEECO) [$\sigma_u(N)$ 가변].
- **피로기준**: 직교전단응력진폭 $\tau_{xz}$(또는 임계각 회전 $\tau_{x'z'}$)를 주로 사용. 2023 §3.1은 **비비례(non-proportional) 응력** → 임계각이 시간에 따라 변함(Olver 2005, Kim-Olver 1998).
- **논문 역할**: 세 논문 모두 부표면항은 **동일**(established RCF method, 2019 문장 "can be solved using established Rolling Contact Fatigue methods [35]"). 차이는 응력장 산출 방식(2015/2019=하중 평균근사 식[22], 2023=시간가변 이력 직접적분).

### C3. 표면 손상 엔진 (표면 손상적분 $I_s$)
- **원리**: 표면 미세형상·마찰트랙션이 만드는 얇은층($\hat h$, 조도 깊이수준) 응력 $\sigma_s$로 **면적분** 손상. 부표면 Hertz 응력과 **독립**(2015 Fig.1).
- **지배식**: $I_s=\bar B\int_A\langle\sigma_s-\sigma_{u,s}\rangle^{c}dA$ (2015 식[23], 2019 식(10), 2023 식(4)).
- **표면응력 $\sigma_s$의 출처 = 2011 마이크로피팅 모델**: 혼합윤활(비뉴턴 Reynolds)+반무한체 탄성 FFT로 표면압력·트랙션 산출 → Dang Van 피로기준 + Palmgren-Miner 누적 + 국소 Archard 마모. (2015 §"Advanced Surface Distress Model", 원 ref = Morales-Espejel & Brizmer 2011).
- **원 출처 체인**: Morales-Espejel & Brizmer 2011 [표면 물리모델], Lubrecht et al. 1990 [표면 손상함수 형태], Gabelli et al. 2008 [$\eta$ 환경계수], Morales-Espejel et al. 2010 [micro-geometry 수명정격].
- **간이화 형태(3세대 진화)**:
  - **5상수 지수형** (2015 식[31], 2019 식(11)): $u^{?}I_s\approx f_1\exp[\frac{f_2}{(P/P_u)^{f_3}}+\frac{f_4}{(P/P_u)^{f_5}}]$. 상수는 베어링유형·환경계수별 데이터블록.
  - **3상수 tanh형** (2023 식(6)·부록A): $\widetilde I_s=\frac{a_1}{\Lambda^{1.3}}\tanh([\frac{p_H}{p_{Hu}}]^{1/3}\frac{b_1}{\Lambda^{1.3}})-\frac{c_1}{50}[\frac{p_H}{p_{Hu}}]^{1/3}$, $\widetilde I_s=I_s/\bar B$. 상수 $a_1,b_1,c_1$은 Λ별 Table A1(13점 보간).
- **핵심 규약 주의**: 표면 피로한계는 보수적으로 $\tau_{u,s}=0$ 가정 가능(2015 Table1 주석, 가공·화학흡착 고려).

### C4. 하이브리드(세라믹-강) 표면 특화 — **2019 고유 기여**
- **문제**: 세라믹 볼은 탄성계수↑ → Hertz 접촉타원↓ → **접촉압 약 12%↑**. 전통 RCF는 이를 벌점화하여 하이브리드 수명을 과소평가.
- **해법**: 표면손상적분을 세라믹-강 계면에 맞춰 재교정. 세라믹-강은 **경계윤활 마찰계수가 낮아**(Hager et al. 2011 측정값) 표면피로응력이 낮음 → 12% 높은 압력을 **표면에서 보상**.
- **표면응력지수**: $I_s=f(P_r,\eta_{env})$, $P_r=P/P_u$, $\eta_{env}=\eta_{lub}\cdot\eta_{cont}$ (2019 식(11), Fig.6). 좋은 윤활(η_env→0.85)에선 표면응력 무시가능, 오염·박막(η_env→0.035)에선 표면피로 지배.
- **근거 연구**: Brizmer et al. 2015 [하이브리드 마이크로피팅 실험·이론], Vieillard et al. 2016 [인공덴트 하이브리드 표면 RCF 진전], Morales-Espejel & Gabelli 2011 [인덴테이션 거동].

### C5. 하중기반 → 베어링/기어 수명 변환
- **베어링(2015)**: LP 변환으로 단일접촉식[16]을 베어링하중식으로. 점접촉 $k$(식[18]), 선접촉 $k$(식[19]), 최중하중 요소 합성(식[20][21]). 지수 $w=pe=(c-h+2)/3$. 표면항 포함 반해석식[29]: $L_{10}=\frac{a_u(C/P)^p}{[1+\frac{u^m L_{10}^{(m-e)}I_s}{\ln(1/0.9)}(C/P)^{ep}a_u^e]^{1/e}}$. 피로한계계수 $a_u$(식[25]) — $\Psi_{brg}P_u/P$ 함수, 무피로한계 시 $a_u=\widehat A$.
- **하이브리드(2019)**: 식(12)에 식(11) 표면지수 대입 → 하이브리드 $L_{10}$.
- **기어(2023)**: 톱니 접촉을 시간스텝($n_t$개)으로 분할, 각 스텝 접촉해석 → 응력이력. 완전기어 수명 식(10)~(15): $L=(2n)^{-1/e}L_i$ (동일 gear ratio). n=톱니수.
- **원 출처**: Lundberg-Palmgren 1947 (식[44][46][81][89][95] 인용), ISO 281:2007 [κ, $e_c$, 정격].

### C6. 응력기반 정식화 — **2023 고유 기여**
- **동기**: 하중기반은 실제 응력장이 아닌 하중 이상화(동적정격)에 의존 → 시간가변·응력집중(오정렬, 프로파일, 에지효과)을 표현 못함.
- **시간이동·시간가변 응력이력** (§3.1): $\sigma_v(x,y,z)=\max_t(\tau_{xz})_t-\min_t(\tau_{xz})_t$ (임계각의 시간 진폭). 하중·등가반경이 접촉경로 따라 변함 → 부표면적분이 이력 전체 포괄.
- **표면피로지표 $S_R=I_s/(I_s+I_{ss})$** (식(8)): 1에 가까우면 표면지배, 0이면 부표면지배. 완화전략 트리거용 진단지표.
- **원 출처**: Kim-Olver 1998 [거친표면 응력이력], Olver 2005 [비비례 응력·임계면], Morales-Espejel et al. 2010 [접촉솔버], Morales-Espejel, Rycerz, Kadiric 2018 [기어 마이크로피팅 — 식(6) 곡선적합 원천].

---

## 2. 논문 계보·진화 대조표 (2011 → 2015 → 2018 → 2019 → 2023)

> GBLM 계보의 축. **2011**은 표면엔진(물리), **2015**가 GBLM 정식 출발, **2019**가 중심(하이브리드), **2023**이 최신 일반화. (2018 기어논문 [19]는 2015→2023 사이 하중기반 기어 확장으로, 2023의 직접 선행.)

| 축 | 2011 Micropitting | 2015 GBLM(원형) | 2018 Gear[19] | **2019 Hybrid(중심)** | 2023 Stress-based |
|---|---|---|---|---|---|
| **대상** | 롤링-슬라이딩 표면 | 강-강 롤링베어링 | 스퍼기어(하중기반) | 하이브리드 세라믹볼 베어링 | 일반 롤링/슬라이딩(기어 예시) |
| **모델 성격** | 표면 물리(micro-EHL) | **하중기반** 수명 | 하중기반 | 하중기반+하이브리드특화 | **응력기반** 일반 |
| **새 기여** | 표면손상 응력엔진(DangVan+Miner+Archard) | 표면/부표면 **분리**, $I_s$ 도입 | $I_s$ 기어 곡선적합 | 세라믹-강 표면지수 $f(P_r,\eta_{env})$ | 시간가변 응력이력, 3상수 $\widetilde I_s$, $S_R$ |
| **표면항 형태** | 수치(전체 시뮬) | 5상수 지수(식[31]) | 5상수 지수(기어) | 5상수 지수·데이터블록(식(11)) | **3상수 tanh**(식(6)) |
| **부표면항** | — | IH 식[11] | IH | IH 식(6) | IH 시간이력 식(1)(9) |
| **핵심 검증** | 마이크로피팅 면적률 | 227풀·6650베어링, 6309예제 | 기어 내구 | Rosado-Forster·Chiu, 509 하이브리드(R²98.6%) | Krantz 기어(Λ1.13/6.17) |
| **교정상수** | k_lub 등 | $\bar A,\bar B$ 내재 | $\bar A,\bar B$ | c1~c5 데이터블록 | $\bar A,\bar B$ 2점교정(full-film→A, poor→B) |
| **표면 간이변수** | — | $P/P_u$, κ | $W/W_u$, Λ | $P/P_u$, $\eta_{env}$ | $p_H/p_{Hu}$, Λ |

**진화 논리 요약**:
1. **2011**이 표면응력을 물리적으로 계산할 수 있게 함(단, 비용 큼) →
2. **2015**가 그 결과를 곡선적합(식[31])하여 베어링 수명식에 표면항 $I_s$로 삽입, 표면/부표면 분리 확률식 확립 →
3. **2018/2019**가 각각 기어·하이브리드로 대상 확장(여전히 하중기반) →
4. **2023**이 하중 이상화를 버리고 실제 시간가변 응력장 직접 적분 + 표면항을 3상수로 간소화하여 임의 기계요소로 일반화.

---

## 3. 상수·지수·물성 파라미터 (원문 추출값, 출처 병기)

| 파라미터 | 값 | 출처 | 비고 |
|---|---|---|---|
| 부표면 지수 $c$ | (예) 31/3 | 2023 Table3 (as [4]) | 응력멱지수 |
| 깊이 지수 $h$ | (예) 7/3 | 2023 Table3 (as [4]) | $z^h$ |
| Weibull 기울기 $e$ | 10/9 (ISO), 실측 1.2 | 2015(ISO 281), 2023 Table3 | 표준정격 vs 시험 |
| 표면 Weibull $m$ | =$e$(표준가정) | 2015 식 주석, 2019 §2.2 | 다르면 비표준분포 |
| 지수관계 | $w=pe=(c-h+2)/3$ | 2015 식[17] 주석 | 하중지수 |
| 피로한계 $\tau_u$(응력기반) | 360 MPa (베어링강 가정) | 2023 Table3 (as [26]) | 기어강 상한 가정 |
| 표면 피로한계 $\tau_{u,s}$ | 0 (보수적) | 2015 Table1 주석 | 가공·흡착 고려 |
| $p_{Hu}$ (피로한계 초과 Hertz압) | 1.5 GPa (베어링강) | 2023 부록A (as [26]) | 기어강도 유사 가정 |
| 교정상수 $\bar A$ | 0.10295e11 (기어예제) | 2023 Table3 | full-film 시험 교정 |
| 교정상수 $\bar B$ | 0.1169 (기어예제) | 2023 Table3 | poor-lub 시험 교정 |
| 표면함수 상수 $a_1,b_1,c_1$ | Λ별 13점표 | 2023 Table A1 | tanh 간이식 보간 |
| 환경계수 $\eta_{env}$ | Rosado-F 0.85 / Chiu 0.035 | 2019 Table1 | $=\eta_{lub}\eta_{cont}$ |
| 하이브리드 접촉압 상승 | ~12% | 2019 Abstract | 세라믹 E↑ |
| κ (점도비) 범위 | 0.1 ~ 4 | 2015 Table2 | ISO 281 |
| 표면손상 판정 | 면적 1.5% 손상 | 2015 식[30] $N_{1.5\%}$ | 크랙개시 |

> ⚠️ 하이브리드 표면지수 c1~c5(2019 식(11))는 "계산프로그램 내 데이터블록"으로 **원문 수치 미공개**(§6-Q1). 교정상수 $\bar A,\bar B$는 재료·윤활·표면마감 변경 시 **재교정 필요**(2023 §6).

---

## 4. 통합 참고문헌 — 보유·MD변환 현황

> 세 논문 인용을 GBLM 체계 기여도로 재분류(중복 병합). **보유**=리포 PDF 존재, **MD변환**=`03. 정리/` MD 존재. 미보유·미변환은 **없음** 표기. 역할·인용매핑은 §1~3 참조.

### 1차 출처 — 분석 대상 (계보 축)

| Ref | 서지(약칭) | 보유 | MD변환 | Google Scholar |
|---|---|:---:|:---:|:---:|
| P1 | Morales-Espejel & Brizmer 2011, Tribol. Trans. 54 (Micropitting, 표면엔진) | ✅ | ✅ | [🔍](https://scholar.google.com/scholar?q=Micropitting+Modelling+Rolling-Sliding+Contacts+Morales-Espejel+Brizmer) |
| P2 | Morales-Espejel, Gabelli & de Vries 2015, Tribol. Trans. 58 (GBLM 원형) | ✅ | ✅ | [🔍](https://scholar.google.com/scholar?q=A+Model+for+Rolling+Bearing+Life+with+Surface+and+Subsurface+Survival+Morales-Espejel) |
| P3 | Gabelli & Morales-Espejel 2019, Tribol. Int. (하이브리드, ★중심) | ✅ | ✅ | [🔍](https://scholar.google.com/scholar?q=A+model+for+hybrid+bearing+life+with+surface+and+subsurface+survival+Gabelli) |
| P4 | Morales-Espejel & Félix-Quiñónez 2023, Tribol. Int. (응력기반, 기어) | ✅ | ✅ | [🔍](https://scholar.google.com/scholar?q=A+stress-based+model+for+general+rolling+contacts+surface+subsurface+survival+gears) |

### Tier A — GBLM 골격 정본 — 보유 1/5 · MD변환 0/5

| Ref | 서지(약칭) | 보유 | MD변환 | Google Scholar |
|---|---|:---:|:---:|:---:|
| [1] | Ioannides & Harris 1985, J. Tribol. 107, 367–378 (부표면 멱법칙) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=A+new+life+model+for+rolling+bearings+Ioannides+Harris+1985) |
| [2] | Lundberg & Palmgren 1947, Acta Polytech. Mech. 1(3) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=Dynamic+Capacity+of+Rolling+Bearings+Lundberg+Palmgren+1947) |
| [3] | Lundberg & Palmgren 1952, Acta Polytech. Mech. 2(4) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=Dynamic+Capacity+of+Roller+Bearings+Lundberg+Palmgren+1952) |
| [4] | Weibull 1939, Proc. R. Swed. Acad. Eng. Sci. 151 (최약链) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=A+Statistical+Theory+of+Strength+of+Materials+Weibull+1939) |
| [5] | Morales-Espejel & Gabelli 2018, Wear 404–405, 133–142 (GBLM 기어확장) | ✅ | 없음 | [🔍](https://scholar.google.com/scholar?q=A+model+for+gear+life+with+surface+and+subsurface+survival+Morales-Espejel+2018) |

### Tier B — 구성요소 지원 — 보유 1/8 · MD변환 0/8

| Ref | 서지(약칭) | 보유 | MD변환 | Google Scholar |
|---|---|:---:|:---:|:---:|
| [6] | Ioannides, Bergling & Gabelli 1999, Acta Polytech. Scand. 137 (해석정식화) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=An+Analytical+Formulation+for+the+Life+of+Rolling+Bearings+Ioannides+Bergling+Gabelli) |
| [7] | Gabelli et al. 2012, Int. J. Fatigue 37, 155–168 (Fatigue Limit Part II) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=The+Fatigue+Limit+of+Bearing+Steels+Part+II+Gabelli) |
| [8] | Lai et al. 2012, Int. J. Fatigue 37 (Fatigue Limit Part I) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=The+Fatigue+Limit+of+Bearing+Steels+Part+I+Lai) |
| [9] | Gabelli, Morales-Espejel & Ioannides 2008, Tribol. Trans. 51, 428–445 (η 환경계수) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=Particle+Damage+in+Hertzian+Contacts+and+Life+Ratings+of+Rolling+Bearings+Gabelli) |
| [10] | Morales-Espejel, Gabelli & Ioannides 2010, IMechE Part C 224 (micro-geo 정격) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=Micro-Geometry+Lubrication+and+Life+Ratings+of+Rolling+Bearings+Morales-Espejel) |
| [11] | Morales-Espejel, Wemekamp & Félix-Quiñónez 2010, IMechE Part J 224, 621–637 (접촉/마찰솔버) | ✅ | 없음 | [🔍](https://scholar.google.com/scholar?q=Micro-geometry+effects+on+the+sliding+friction+transition+in+elastohydrodynamic+lubrication) |
| [12] | Lubrecht, Jacobson & Ioannides 1990, Japan Int. Tribol. Conf. (표면손상함수형) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=Lundberg+Palmgren+Revisited+Lubrecht+Jacobson+Ioannides) |
| [13] | ISO 281:2007, Rolling Bearings — Dynamic Load Ratings and Rating Life | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=ISO+281+2007+Rolling+bearings+dynamic+load+ratings+rating+life) |

### Tier C — 하이브리드·표면 특화 (P3 중심) — 보유 0/6 · MD변환 0/6

| Ref | 서지(약칭) | 보유 | MD변환 | Google Scholar |
|---|---|:---:|:---:|:---:|
| [14] | Brizmer, Gabelli, Vieillard, Morales-Espejel 2015, Tribol. Trans. 58, 829–835 | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=experimental+theoretical+study+hybrid+bearing+micropitting+reduced+lubrication+Brizmer) |
| [15] | Vieillard, Kadin, Morales-Espejel, Gabelli 2016, Wear 364–365, 211–223 (인공덴트) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=surface+rolling+contact+fatigue+damage+progression+hybrid+bearings+artificial+dents+Vieillard) |
| [16] | Morales-Espejel & Gabelli 2011, Tribol. Trans. 54, 589–606 (인덴테이션 거동) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=The+behaviour+of+indentation+marks+in+rolling-sliding+EHL+contacts+Morales-Espejel+Gabelli) |
| [17] | Morales-Espejel & Gabelli 2016, Tribol. Int. 96, 279–288 (sporadic 표면손상) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=A+model+for+rolling+bearing+life+sporadic+surface+damage+deterministic+indentations) |
| [18] | Hager, Doll, Evans, Shiller 2011, Wear 271, 1761–1771 (경계마찰 측정) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=Minimum+Quantity+Lubrication+M50+M50+Si3N4+Hager) |
| [19] | Brizmer, Pasaribu, Morales-Espejel 2013, Tribol. Trans. 56, 739–748 (첨가제) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=Micropitting+Performance+of+Oil+Additives+in+Lubricated+Rolling+Contacts+Brizmer) |

### Tier D — 응력이력·비비례·기어 (P4 중심) — 보유 4/8 · MD변환 1/8

| Ref | 서지(약칭) | 보유 | MD변환 | Google Scholar |
|---|---|:---:|:---:|:---:|
| [20] | Kim & Olver 1998, Tribol. Int. 31(12), 727–736 (응력이력) | ✅ | ✅ | [🔍](https://scholar.google.com/scholar?q=Stress+history+in+rolling-sliding+contact+of+rough+surfaces+Kim+Olver) |
| [21] | Olver 2005, IMechE Part J 219, 313–330 (비비례 응력·임계면) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=The+mechanisms+of+rolling+contact+fatigue+an+update+Olver+2005) |
| [22] | Morales-Espejel, Rycerz, Kadiric 2018, Wear 398–399, 99–115 (기어 마이크로피팅, 식6 원천) | ✅ | 없음 | [🔍](https://scholar.google.com/scholar?q=Prediction+of+micropitting+damage+in+gear+teeth+contacts+Morales-Espejel+Rycerz+Kadiric) |
| [23] | Krantz, Alanou, Evans, Snidle 2000, NASA TM-2000-210044 (기어 검증데이터) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=Surface+Fatigue+Lives+of+Case-Carburized+Gears+Krantz) |
| [24] | Coy, Townsend, Zaretsky 1983, NASA CP-2210 (기어형상) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=An+Update+on+the+Life+Analysis+of+Spur+Gears+Coy+Townsend+Zaretsky) |
| [25] | Brandão, Seabra, Castro 2010 Part I, Wear 268, 1–12 (기어 표면손상 수치) | ✅ | 없음 | [🔍](https://scholar.google.com/scholar?q=Surface+initiated+tooth+flank+damage+Part+I+numerical+model+Brandao+Seabra) |
| [26] | Brandão, Seabra, Castro 2010 Part II, Wear 268, 13–22 (마이크로피팅 개시·질량손실) | ✅ | 없음 | [🔍](https://scholar.google.com/scholar?q=Surface+initiated+tooth+flank+damage+Part+II+prediction+micropitting+Brandao) |
| [27] | Li & Kahraman 2014, Int. J. Fatigue 59, 224–233 (기어 마이크로피팅) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=A+micro-pitting+model+for+spur+gear+contacts+Li+Kahraman) |

### Tier E — 배경·통계·경쟁모드 — 보유 3/8 · MD변환 0/8

| Ref | 서지(약칭) | 보유 | MD변환 | Google Scholar |
|---|---|:---:|:---:|:---:|
| [28] | Tallian 1967, ASLE Trans. 10(4), 418–439 (경쟁 파손모드) | ✅ | 없음 | [🔍](https://scholar.google.com/scholar?q=On+Competing+Failure+Modes+in+Rolling+Contact+Tallian+1967) |
| [29] | Tallian & McCool 1971, Wear 17, 447–461 (Spalling 표면모델) | ✅ | 없음 | [🔍](https://scholar.google.com/scholar?q=An+Engineering+Model+of+Spalling+Fatigue+Failure+in+Rolling+Contact+Surface+Model+Tallian) |
| [30] | Chiu, Tallian, McCool 1969, ASLE Trans. 12 (Spalling 수학모델) | ✅ | 없음 | [🔍](https://scholar.google.com/scholar?q=A+Mathematical+Model+of+Spalling+Fatigue+Failure+in+Rolling+Contact+Chiu+Tallian+McCool) |
| [31] | McCool 1978, Tribol. Trans. 21, 271–284 (경쟁위험·통계) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=Competing+Risk+and+Multiple+Comparison+Analysis+for+Bearing+Fatigue+Tests+McCool) |
| [32] | Sadeghi et al. 2009, J. Tribol. 131, 041403 (RCF 총설) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=A+Review+of+Rolling+Contact+Fatigue+Sadeghi) |
| [33] | Basquin 1910, Proc. ASTM 10, 625–630 (멱법칙 물리성) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=The+Exponential+Law+of+Endurance+Tests+Basquin+1910) |
| [34] | Kun et al. 2008, Phys. Rev. Lett. 100, 094301 (Basquin 보편성) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=Universality+behind+Basquin+Law+of+Fatigue+Kun) |
| [35] | Miner 1945, J. Appl. Mech. 12, A159–A164 (Palmgren-Miner 누적) | 없음 | 없음 | [🔍](https://scholar.google.com/scholar?q=Cumulative+Damage+in+Fatigue+Miner+1945) |

> **보유 종합**: 1차출처 4/4 · Tier A~E 9/35(보유) · 3/35(MD변환). GBLM **골격 정본(IH1985·LP·Weibull) 및 하이브리드 실험문헌(Tier C)이 리포 미보유** — 확보 필요분(§6 연계). Tallian 계보(1967/1969/1971)·기어 표면손상(Brandão)·응력이력(Kim-Olver)은 마이크로피팅 수집분과 공유.

---

## 5. 적용·검증 사례 (게이트별 재현검증 후보)

| 사례 | 논문 | 조건 | 결과·오라클 | 비고 |
|---|---|---|---|---|
| 6309 심구볼 예제 | 2015 Table4 | κ=0.1/0.4/2.9, 10kN | $L_{10}$=17/81/1656 Mrev, 표면기여 100/96/23% | ISO 281 정합확인 |
| 227풀 내구시험 | 2015 Fig.10 | 볼·롤러 6650베어링 | $R_s$ back-calc ≤ 모델한계곡선(보수적 안전) | 표면모델 검증 |
| Rosado-Forster | 2019 Table1,Fig7 | 3.5/3.1 GPa, η=0.85 | 부표면지배, GBLM=L10,5(고유의) | 고하중·양윤활 |
| Chiu | 2019 Table1,Fig8 | 2.6/2.3 GPa, η=0.035 | 표면지배, 하이브리드 우위 | 오염·박막 |
| 509 하이브리드 상관 | 2019 Fig.11 | 20시리즈, 192파손 | Pearson 0.91, R²=98.6% | 종합 상관 |
| Krantz 기어(superfinish) | 2023 §5.1 | Λ=6.17, AISI9310 | $L_{10}$=25.56 Mrev, $S_R$=0 | A 교정 |
| Krantz 기어(ground) | 2023 §5.3 | Λ=1.13 | $L_{10}$=5.16 Mrev, $S_R$=0.854 | B 교정, 표면지배 |
| 오정렬 효과 | 2023 §5.4 | α=0.25rad | $L_{10}$ 25.58→16.5 Mrev | 응력기반 강점 |
| worst-case 대비 | 2023 §6 | 최대하중 고정 | 7.82 vs 25.56 Mrev | 시간가변 필요성 입증 |

---

## 6. 미해결 이슈·미지 상수 (분석 큐)

> GBLM을 정량 재현·이식하려 할 때 원문이 **미제공**하거나 **가정**한 항목. (구현이 목표면 P3에서 해소 대상.)

| ID | 미제공/가정 내용 | 관련식 | 조사경로 | 상태 |
|---|---|---|---|---|
| Q1 | 하이브리드 표면지수 상수 $c_1$~$c_5$ (프로그램 데이터블록, 수치 비공개) | 2019 식(11) | 2011 표면엔진 재구현→역적합 | ☐ |
| Q2 | 교정상수 $\bar A,\bar B$의 재료의존성(2점 내구시험 필요) | 2023 §5.1 | full-film/poor-lub 시험쌍 | ☐ |
| Q3 | 표면층 두께 $\hat h$의 실제값(상수B에 흡수) | G2·[7] | 조도 깊이수준 가정 | ☐ |
| Q4 | 표면 Weibull 기울기 $m$의 실제값(표준 $m=e$ 가정) | G4·[13] | 내구시험 분포 | ☐ |
| Q5 | 부표면 피로한계 $\sigma_{u,v}$, $z'$(응력가중평균깊이) 정의 | 2023 식(1) | IH1985·Gabelli2012 | ☐ |
| Q6 | 2011 표면엔진(micro-EHL) 상세 = 별도 P3 구현대상(Dang Van·Archard·상보파) | C3 전반 | `micropitting-p3-project` 연계 | ☐ |
| Q7 | 5상수 지수형↔3상수 tanh형 정합성(2015/2019 vs 2023 표면함수) | 식[31]/(11) vs (6) | 두 곡선 수치대조 | ☐ |
| Q8 | $\eta_{env}$↔κ↔Λ 변환관계(ISO 281 lub factor, contamination) | 2019 §3 | Gabelli2008·ISO281 | ☐ |

---

## 부록. 문서·이미지 자산

- 통합 MD(본 분석 입력): `2015. (SKF) A Model for Rolling Bearing Life with Surface and Subsurface_1745a3.md`(569줄, img18) · `2019. (SKF) A model for hybrid bearing life with surface and subsurface survival.md`(528줄, img16) · `2023. (SKF) A stress-based model for general rolling contacts with surfac_f4e12a.md`(560줄, img25)
- 이미지 폴더: `2015_SKF_A_Model_for_Rolling_Bearing_Lif/` · `2019_SKF_A_model_for_hybrid_bearing_life/` · `2023_SKF_A_stress_based_model_for_genera/`
- 표면엔진 기반논문(별도): `Test_pipeline. 2011. (SKF) Micropitting.md` — GBLM 표면항 $\sigma_s$의 원 물리모델. P3 마이크로피팅 구현 프로젝트가 이를 Rust로 구현 중.

---

**끝**
