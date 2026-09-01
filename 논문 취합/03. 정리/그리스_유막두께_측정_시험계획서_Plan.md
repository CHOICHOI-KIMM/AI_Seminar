# 그리스 유막두께 측정 시험계획서 — 수립 계획 (Plan)

> 작성 260820 · 대상 장비: PCS Instruments **EHD / EHD-HS** (광간섭 유막두께 측정)
> 목적: 초대형 TRB 메인베어링의 **그리스 유막두께 예측 모델**을 확보하고, 자체 시험으로 검증한다.
> 본 문서는 시험계획서 본편을 쓰기 위한 **틀 문서**다 — 지표 선정 기준·모델 후보·시험 구성·조건 검토 항목을 정리하고, 확정된 것과 미결인 것을 구분한다.

---

# 1. 배경과 최종 목표

| 항목                | 내용                                                                                                                     |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| **최종 목표** | 초대형 TRB의 그리스 유막두께 예측 모델 확보                                                                              |
| **활용처**    | ① 메인베어링 설계 ② 그리스 선정 ③**그리스 재윤활 주기 결정**                                                    |
| **접근**      | 출판된 이론으로 기본 신뢰성 확보 → 자체 시험으로 검증                                                                   |
| **제약**      | 베어링 단위 시험은 한계 →**전동체 단위(롤러-디스크) 시험**으로 대체하고, 그에 따른 그리스 거동 차이를 고려해야 함 |

### 1-1. 이론적 쟁점 (문헌 검토 결과)

그리스 유막두께를 정교하게 예측하려면 **두 기구를 동시에** 다뤄야 한다.

| 기구                                                       | 방향      | 대표 모델                                          |
| ---------------------------------------------------------- | --------- | -------------------------------------------------- |
| **압력에 의한 오일 손실** (측방유동, side flow)      | 유막 감소 | Cann (2004) · Van Zoelen (2009~2012) · Damiens   |
| **블리딩에 의한 오일 보충** (bleeding replenishment) | 유막 회복 | 2026 CRB 모델 (지수함수형, 시험으로 파라미터 확보) |

- **Van Zoelen 모델**은 심각한 기아윤활(severe starvation)을 전제하고 **오일 보충을 고려하지 않는다.** 따라서 태생적으로 **보수적** 예측이다.
- 문헌의 Ball/Roller-Disc 시험과는 잘 일치한다 — 그 시험 조건에서는 보충이 미미하기 때문이다.
- 그러나 **실제 베어링에서는 블리딩 보충을 무시할 수 없다.**
- 보충을 고려한 2026 모델이 정확도가 더 높을 것으로 기대되나 **시험 검증이 부재**하다. 압력 손실 항은 검증된 Cann/Van Zoelen을 쓰고, 전체는 Van Zoelen과 정성적으로 비교해 유사성만 확인한 상태다.

**→ 본 시험의 위치**: 이 "검증 부재" 구간을 우리 시험으로 메운다. 롤러-디스크에서 **보충이 억제된 조건**과 **보충이 작동하는 조건**을 분리해 측정하면, 두 모델의 적용 한계를 실측으로 그을 수 있다.

---

## 2. 확정 사항 (260820)

| # | 항목           | 결정                                                                            |
| -: | -------------- | ------------------------------------------------------------------------------- |
| 1 | 시험 구성      | **2단계 — 선 오일, 후 그리스. 단, 둘 다 롤러(선접촉) 시험**              |
| 2 | 유막 지표      | **정상상태 중앙유막두께 h_c** + **과도 감쇠곡선 h(t)·감쇠시간**    |
| 3 | 해석모델       | **전 모델 검토** — 그리스 특화 모델 + **일반 비뉴턴 EHL 모델까지** |
| 4 | 상사 기준      | **무차원수 정합 + 속도·온도 중심**                                       |
| 5 | 오일 시험 목적 | **장비·절차 검증 + 비뉴턴 거동 확인** (전단박화·열효과 포함)            |
| 6 | 그리스 시료    | **기유 점도 대비 2종**                                                    |
| 7 | 감쇠 시험 운용 | **장시간 연속 감쇠** (충전 후 수~수십 시간 통짜 기록)                     |
| 8 | 저장 위치      | `논문 취합/03. 정리/`                                                         |

---

## 3. 유막두께 지표 — 선정 기준과 결론

문헌에는 정상상태·과도·기아도·마찰 등 다양한 지표가 등장한다. 무엇을 측정할지 정하려면 **기준을 먼저 세워야** 한다.

### 3-1. 선정 기준 (4개)

|            # | 기준                  | 뜻                                                                               |
| -----------: | --------------------- | -------------------------------------------------------------------------------- |
| **C1** | **측정 가능성** | 우리 장비(광간섭·1 nm 분해능·1~1000 nm 범위)로 직접·안정적으로 얻을 수 있는가 |
| **C2** | **모델 대응성** | 검토 대상 해석모델이 그 지표를 직접 출력하는가 (환산·가정 없이 대조 가능한가)   |
| **C3** | **설계 연결성** | 메인베어링 설계·그리스 선정·**재윤활 주기**로 이어지는가                 |
| **C4** | **재현성**      | 그리스 충전·러닝인 이력에 과도하게 좌우되지 않는가                              |

### 3-2. 후보 지표 평가

| 지표                                  |        C1 측정        |         C2 모델         |        C3 설계        | C4 재현 | 판정                            |
| ------------------------------------- | :-------------------: | :---------------------: | :-------------------: | :-----: | ------------------------------- |
| **중앙유막두께 h_c (정상상태)** |          ◎          |           ◎           |          ◎          |   ○   | **주지표 채택**           |
| **감쇠곡선 h(t) · 감쇠시간**   |          ◎          |           ◎           | ◎ (재윤활 주기 직결) |   △   | **주지표 채택**           |
| 최소유막두께 h_min                    | △ (선접촉 에지 영향) |           ○           |          ○          |   △   | 부지표 — h_c 에서 환산         |
| **기아도 h_c,0 / h_c,ff**       |  ◎ (계산으로 산출)  | ◎ (모델 적용범위 판정) |          △          |   ○   | **부지표 채택**           |
| **유막비 Λ = h/σ**            |  ◎ (조도 실측 필요)  |           ○           |          ◎          |   ○   | **부지표 채택**           |
| 마찰(트랙션) 계수                     |          ◎          |           △           |          △          |   ○   | 참고 — 비뉴턴 검증에 활용      |
| 오일 분리율·블리딩률                 |    ✕ (별도 시험)    |           ◎           |          ○          |   ○   | 범위 밖 (문헌값·물성시험 인용) |

### 3-3. 채택 결론

```
주지표   h_c (정상상태, 속도·하중·온도 스윕)
         h(t) · 감쇠시간 (장시간 연속)
부지표   기아도 h_c,0/h_c,ff   — 모델 적용범위 판정용
         Λ = h/σ              — 설계 판정 연결용
참고     트랙션 계수           — 비뉴턴 거동 확인용
```

> **부지표 두 개를 반드시 함께 기록하는 이유** — 문헌에서 Van Zoelen 모델과 실측이 어긋나는 원인이 **기아 정도의 차이**로 설명되었다. 같은 그리스라도 속도에 따라 기아도가 달라지고(문헌 사례: 속도가 오를수록 기아가 심해져 오히려 모델 예측에 가까워짐), 이를 기록하지 않으면 모델 불일치의 원인을 사후에 분리할 수 없다.

---

## 4. 해석모델 — 검토 대상 (전 모델)

### 4-1. 그리스 특화 모델

| 모델 | 참고문헌 | 다루는 것 | 전제·한계 | 본 시험에서의 역할 |
| --- | --- | --- | --- | --- |
| **Van Zoelen (감쇠)** | van Zoelen, Venner & Lugt (2009), *Prediction of film thickness decay in starved elasto-hydrodynamically lubricated contacts using a thin layer flow model*, Proc. IMechE Part J **223**(3):541–552<br>· van Zoelen (2009), *Thin layer flow in rolling element bearings*, PhD thesis, Univ. of Twente | 측방유동에 의한 h(t) 감쇠 | **심각한 기아** 전제, 보충 미고려 → 보수측 | **기준선(baseline)** |
| **Cann (2004)** | Cann, Damiens & Lubrecht (2004), *The transition between fully flooded and starved regimes in EHL*, Tribology International **37**:859–864 | 압력에 의한 오일 손실 | 검증 이력 풍부 | 압력 손실 항의 대조 |
| **Damiens** | Damiens, Venner, Cann & Lubrecht (2004), *Starved lubrication of elliptical EHD contacts*, J. Tribology **126**(1):105–111 | 측방유동 | — | Van Zoelen과 결합·확장 대상 |
| **2026 CRB 블리딩 보충** | ★ Puthumana, van Zoelen & Lugt (2026), *Transient Analysis of Film Thickness During Bleed Phase in Grease-Lubricated Cylindrical Roller Bearings* (Taylor & Francis, OA · 저널·권호 확인 필요)<br>`2026. (Lugt) Transient Analysis…Bleed Phase in CRB.md` | 지수함수형 오일 보충 | **시험 검증 부재**, 파라미터 필요 | **검증 목표 모델** |
| **Lugt thin layer flow** | ★ Venner, van Zoelen & Lugt (2012), *Thin layer flow and film decay modeling for grease lubricated rolling bearings*, Tribology International **47**:175–187<br>`2012. (Lugt) Thin layer flow and ~.md` | 궤도 잔류 그리스층의 재유동 | — | 보충 기구의 물리 해석 |
| **타원비 효과** | ★ Gao, van Zoelen, Osara, Meeuwenoord, Pasaribu & Lugt (2025), *Film thickness decay in grease lubricated wide elliptical contacts*, Tribology International, art. 111137<br>`2026. (Lugt) Film thickness decay…wide elliptical contacts.md` | 감쇠시간 ∝ k² (k = b/a) | — | **롤러 시험 설계에 직결** (§6-3) |

> ★ = 변환본 보유 (`논문 취합/03. 정리/`). 파일명을 함께 적었다.

### 4-2. 일반 EHL 모델 (비그리스)

| 모델 | 참고문헌 | 용도 |
| --- | --- | --- |
| **Hamrock–Dowson** | Hamrock & Dowson (1977), *Isothermal Elastohydrodynamic Lubrication of Point Contacts: Part III — Fully Flooded Results*, J. Lubrication Technology **99**(2):264–276<br>· 기아: 같은 저자 (1977), *Part IV — Starvation Results*, J. Lubrication Technology **98**:15–24 | 완전유막 h_c,ff 산출 → 기아도 분모, 오일 시험 검증 기준 |
| **Moes–Venner** | Moes (1992), *Optimum similarity analysis with applications to elastohydrodynamic lubrication*, Wear **159**:57–66 | 무차원 M·L 기반 h_c — **상사 설계의 기준**(§6-1) |
| **비뉴턴 (Eyring · Carreau)** | Eyring (1936), *Viscosity, Plasticity, and Diffusion as Examples of Absolute Reaction Rates*, J. Chemical Physics **4**(4):283–291<br>· Carreau (1972), *Rheological Equations from Molecular Network Theories*, Trans. Society of Rheology **16**(1):99–127 | 고전단율 전단박화 — 오일 시험에서 확인 (확정 #5) |
| **열보정 (inlet shear heating)** | ⚠ **출처 확인 필요** — 보유 자료에서 특정 모델의 원전을 확정하지 못함 | 고속·고점도에서 h_c 저하 보정 |
| **기아 보정 (Wolveridge·Chevalier 등)** | Wolveridge, Baglin & Archard (1970), *The starved lubrication of cylinders in line contact*, Proc. IMechE **185**(1):1159–1169<br>· Chevalier, Lubrecht, Cann, Colin & Dalmaz (1998), *Film thickness in starved EHL point contacts*, J. Tribology **120**(1):126–133 | 유입부 오일량 부족 시 h_c 저하 |
| **압축성 (Dowson–Higginson)** | Dowson & Higginson (1966), *Elasto-hydrodynamic Lubrication: The Fundamentals of Roller and Gear Lubrication*, Pergamon Press, Oxford | 밀도-압력 관계 — 감쇠 예측에 영향(과소평가 시 감쇠를 빠르게 예측) |

### 4-3. 모델 ↔ 지표 ↔ 시험 대응

| 시험                            | 지표           | 대조 모델                                                          |
| ------------------------------- | -------------- | ------------------------------------------------------------------ |
| Phase A 오일 · 속도 스윕       | h_c            | Hamrock–Dowson · Moes–Venner · 비뉴턴 · 열보정                |
| Phase B 그리스 · 정상상태 스윕 | h_c, 기아도    | 기아 보정 EHL · 블리딩 보충 모델                                  |
| Phase B 그리스 · 장시간 감쇠   | h(t), 감쇠시간 | **Van Zoelen** · Cann/Damiens · **블리딩 보충 모델** |

---

## 5. 시험 계획 (골격)

### 5-1. 2단계 구성

```
Phase A — 오일 (롤러-디스크)          목적: 장비·절차 검증 + 비뉴턴 거동 확인
   └ 그리스의 기유 단독 사용 → 이론 EHL 식과 대조
   └ 통과 기준: h_c 실측 vs 예측 편차 ≤ ±10% (완전유막 영역)

Phase B — 그리스 (롤러-디스크)        목적: 유막 예측 모델 검증
   ├ B-1 정상상태 스윕   (속도 × 하중 × 온도)   → h_c
   └ B-2 장시간 연속 감쇠 (대표 조건)          → h(t) · 감쇠시간
```

### 5-2. 시험 매트릭스 (초안 — 조건값은 §6 검토 후 확정)

| 인자                        | 수준 (안)                  | 근거                                                                                          |
| --------------------------- | -------------------------- | --------------------------------------------------------------------------------------------- |
| **그리스**            | 2종 (기유 저점도 / 고점도) | 확정#6 · 문헌의 Li/저점도 vs Li/고점도 대비 구성과 동형                                      |
| **엔트레인먼트 속도** | 4~6 수준 (저속 중심)       | 문헌 감쇠시험은 0.1~0.2 m/s 수준. 4 m/s 초과 시 기아로 사실상 기유 성능만 측정됨(장비사 의견) |
| **하중(접촉압력)**    | 3 수준                     | 장비 상한 내(§6-2)                                                                           |
| **온도**              | 2~3 수준                   | 실기 운전온도 포함                                                                            |
| **감쇠 시험**         | 대표 조건 2~4건 × 장시간  | 확정#7                                                                                        |

### 5-3. 절차 골격

1. 시편·그리스 충전 (스쿠프 사용 시 완전유막 공급 / 미사용 시 자연 기아)
2. **소킹** — 그리스는 오일보다 온도 도달이 느리므로 대기시간 확보
3. 러닝인(처닝) — 유막·온도 안정화까지. 문헌 사례는 베어링에서 25~30 h
4. B-1 정상상태 스윕 — 각 스텝에서 안정 구간의 대표값 기록
5. B-2 장시간 감쇠 — 충전 직후부터 연속 기록
6. 시험 후 궤도 그리스 분포 관찰(사진), 시편 조도 재측정

---

## 6. 시험 조건 구성을 위한 검토 사항

여기가 이 문서의 핵심이다. **실기와 시험을 어떻게 대응시킬지**가 결과의 해석 가능 여부를 결정한다.

### 6-1. 상사 기준 — 무차원수 + 속도·온도 (확정 #4)

| 무차원수 | 정의 | 정합 목표 |
| --- | --- | --- |
| **M** (하중) | Moes 무차원 하중 | 실기 값에 근접 |
| **L** (재료) | Moes 무차원 재료수 (α·E′ 관련) | 실기 값에 근접 |
| **Λ** | h/σ | 실기 운전 영역과 동일 체제(경계/혼합/완전유막) |
| 엔트레인먼트 속도 u_e | — | 실기와 **동일 오더** |
| 온도 T | — | 실기 운전온도 범위 |

### 6-1.1 실기 대표 운전점 (260820 산출 · 미결 1 해소 진행)

**기준 설계안** — 사이징 최적화 부록 10-11.S3-c(3목적 NSGA-II) 프론트에서 뽑은 3안. 모두 α = 15° 이고 σ_max 2,093 ~ 2,100 MPa 로 응력 한계에 붙어 있다.

| # | `D` [mm] | `D_pw` [mm] | α [°] | `D_we` [mm] | `L_we` [mm] | 세장비 | Z | 베어링 [t] | 총질량 [t] | 비고 |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 1 | 4,039 | 3,481 | 15 | 227.6 | 533.5 | 2.382 | 44 | 27.54 | 173.00 | 최소 외경 |
| **103** | **4,498** | **4,028** | **15** | **204.2** | **427.8** | **2.137** | **57** | **21.43** | **128.83** | **대표 채택 (중간안)** |
| 210 | 4,983 | 4,596 | 15 | 170.0 | 347.6 | 2.095 | 78 | 16.44 | 103.88 | 최소 질량 |

> **#103 을 대표로, #1·#210 은 조건 범위로 쓴다** (확정). 세 안의 롤러 크기가 227.6 → 170.0 mm 로 25% 차이 나므로 접촉 기하도 함께 변한다. 시험 조건 창을 잡을 때는 세 안이 만드는 범위를 모두 덮는다.

**접촉 기하와 엔트레인먼트 속도 — 산출식과 근거**

내륜이 축과 함께 회전하고 외륜이 정지한 조건이다. 케이지 각속도와 전동체 표면속도는 Harris & Kotzalas §10.2 · §10.3, 궤도 곡률 정의는 §2.5.3 을 따랐다.

```
γ      = D_we·cos α / D_pw                     (전동체 지름비)
내륜 궤도 반경  R_i = (D_pw − D_we·cos α) / (2 cos α)     (볼록)
외륜 궤도 반경  R_o = (D_pw + D_we·cos α) / (2 cos α)     (오목)
등가 반경     1/R_x = 1/(D_we/2) ± 1/R_race            (내륜 +, 외륜 −)
엔트레인먼트   u_e = ω_i·D_pw·(1 − γ²) / 4 = (π·n·D_pw/60)·(1 − γ²)/2
```

**u_e 는 내륜·외륜 접촉에서 값이 같다.** 케이지 각속도 `ω_c = ω_i(1−γ)/2` 를 대입하면 내륜 접촉의 `(ω_i−ω_c)·R_i` 와 외륜 접촉의 `ω_c·R_o` 가 모두 `ω_i·D_pw(1−γ²)/4` 로 정리되기 때문이다. 반면 **등가 반경 R_x 는 두 접촉이 다르다** — 외륜은 오목면이라 R_x 가 더 크고, 따라서 같은 조건에서 유막이 더 두껍다. 유막이 얇은 쪽인 **내륜 접촉이 임계**이므로 대표값으로 삼는다.

| # | γ | **내륜** R_x [mm] | 외륜 R_x [mm] | 외륜/내륜 |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 0.0632 | **106.6** | 119.4 | +12.0% |
| **103** | **0.0490** | **97.1** | 107.1 | +10.3% |
| 210 | 0.0357 | **82.0** | 89.4 | +9.0% |

**식 검증 — 문헌 실측값 대조.** 2026 CRB 논문(★ 보유)은 시험 베어링(SKF NU206 계열 조립체 · 내륜 회전)의 엔트레인먼트 속도를 명시했다. 같은 식에 그 베어링 제원을 넣어 재현되는지 확인했다.

| 회전수 | 논문 기재 u_e | 본 식 산출 (D_pw 46.5 · D_we 9.0 mm) | 편차 |
| ---: | ---: | ---: | ---: |
| 4,000 rpm | 4.69 m/s | 4.687 m/s | **−0.1%** |
| 6,000 rpm | 7.03 m/s | 7.031 m/s | **+0.0%** |

→ 식과 회전 조건 가정(내륜 회전·외륜 정지·순수 구름)이 모두 타당함을 확인했다.

**TRB 의 축방향 속도 구배 — 소단과 대단이 다르다.** TRB 는 원뿔 정점이 축 위 한 점에 모이도록 설계되어 접촉선 전체에서 순수 구름이 성립하지만, **국부 궤도반경이 접촉선을 따라 변하므로 u_e 도 선형으로 변한다.** 반경 변화폭은 `±(L_we/2)·sin α` 다.

| # | D_pw/2 [mm] | ±(L_we/2)·sin α [mm] | 소단 → 대단 | 평균 대비 | h_c 변동 (∝ u_e^0.67) |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1,740.5 | ±69.0 | **+8.3%** | ±4.0% | ±2.6% |
| **103** | **2,014.0** | **±55.4** | **+5.7%** | **±2.8%** | **±1.8%** |
| 210 | 2,298.0 | ±45.0 | **+4.0%** | ±2.0% | ±1.3% |

**본 계획서의 u_e 는 평균 지름 기준값이다** (확정). 편차가 #103 기준 ±2.8%, 유막두께로는 ±1.8% 로 시험 반복오차 안에 들 가능성이 크므로 조건을 나누지 않는다. 다만 **시험 리그(원통 롤러-디스크)는 접촉선을 따라 u_e 가 균일하므로 이 구배 자체는 재현되지 않는다** — 실기 대비 차이로 기록한다(§9 미결 9).

**회전속도 — DLC 종류별 작동 특성** (전 111 DLC · 30년 가동시간 263,014 h)

본 111 DLC 세트는 IEC 61400-1 Table 2 의 **피로 해석 대상(Type of analysis = F)** DLC 에서 추출된 것이다. 아래 표의 IEC 열은 그 규격 정의를 옮긴 것이다.

| 계열 | IEC 설계상황 | 풍모델 | 해석유형 | 건수 | 가동시간 | 시간 비중 | 손상 비중 | rpm 범위 |
| --- | --- | :-: | :-: | ---: | ---: | ---: | ---: | --- |
| **DLC1.2** | Power production | NTM | **F** | 66 | **236,513 h** | **89.9%** | **98.7%** | 3.50 ~ 6.19 |
| DLC6.4 | Parked (standing still or idling) | NTM | F | 12 | 24,632 h | 9.4% | 0.9% | 0.03 ~ 0.30 |
| DLC4.1 | Normal shut down | NWP | F | 9 | 1,833 h | 0.7% | 0.5% | 0.85 ~ 2.26 |
| DLC2.4.2 | Power production + fault | NTM | F | 12 | 25 h | ≈ 0% | ≈ 0% | 2.37 ~ 4.21 |
| DLC2.4.1 | Power production + fault | NTM | F | 12 | 10 h | ≈ 0% | ≈ 0% | 2.54 ~ 3.81 |

> **DLC 3.1 (Start up · NWP · F) 은 본 세트에 포함되어 있지 않다** (사실 기록).
>
> NTM = Normal turbulence model · NWP = Normal wind profile model · F = 피로(Fatigue) 해석 대상.

**정상운전 DLC1.2 의 풍속 빈별 분포** — 시간과 손상의 무게중심이 서로 다르다.

| 빈 | rpm | 가동시간 [h] | 시간 % | 손상 % | 손상/시간 |
| :-: | ---: | ---: | ---: | ---: | ---: |
| a | 3.50 | 37,800 | 14.4% | 14.0% | 0.97 |
| b | 3.70 | 45,700 | 17.4% | 13.7% | 0.79 |
| c | 4.76 | 45,040 | 17.1% | 16.4% | 0.95 |
| **d** | **5.79** | 38,180 | 14.5% | **23.2%** | **1.59** |
| **e** | **6.12** | 28,490 | 10.8% | **16.9%** | **1.56** |
| f | 6.17 | 18,970 | 7.2% | 7.9% | 1.10 |
| g | 6.18 | 11,340 | 4.3% | 3.8% | 0.89 |
| h ~ k | 6.18 ~ 6.19 | 10,993 | 4.1% | 2.9% | 0.52 ~ 0.78 |

> **표의 rpm 산출 방법**
>
> 1. **원시값** — 각 DLC 의 GH Bladed 시계열(`.$150`) 2번째 채널이 로터 회전속도 [rpm] 이다. 이 채널의 **산술평균**이 그 DLC 의 `rpm_mean` 이다 (`build_dlc_rawdata.py`, 시계열 전 점 대상 · 가중 없음).
> 2. **빈 대표값** — 각 풍속 빈(a ~ k)은 난류 시드가 다른 6건으로 구성된다. 표의 rpm 은 그 **6건 `rpm_mean` 의 단순평균**이다(시드 간 가동시간이 같으므로 가중 불필요).
> 3. **가동시간** — `ScaleFactor` 는 시간이 아니라 **30년간 해당 시계열의 발생 횟수**다. 따라서 `가동시간 = ScaleFactor × (n_pts − 1) × dt₀ / 3600` 으로 환산했다. 시계열 길이가 계열마다 달라(DLC1.2·6.4 = 600 s · 2.4.2 = 300 s · 4.1 = 200 s · 2.4.1 = 60 s) 이 환산을 거치지 않으면 오차가 난다. 전 111 DLC 합계는 **263,014 h** 로 원자료와 일치한다.
> 4. **가중 평균의 정의** — 가동시간 가중 `n̄_t = Σ(hᵢ·nᵢ) / Σhᵢ`. 표의 손상 비중은 스크리닝 단계의 `D30_UW_scr` 기준이다.
> 5. **손상/시간** = (손상 비중) ÷ (시간 비중). 1보다 크면 그 빈이 체류시간에 비해 손상을 많이 만든다는 뜻이다.

**속도 구간별 누적** (전 111 DLC)

| 구간 [rpm] | 성격 | 가동시간 | 시간 % | 손상 % |
| --- | --- | ---: | ---: | ---: |
| 0.0 ~ 0.5 | 정지·유휴 | 24,632 h | 9.4% | 0.9% |
| 0.5 ~ 3.0 | 저속 | 1,845 h | 0.7% | 0.5% |
| 3.0 ~ 4.5 | 중속 | 83,524 h | 31.8% | 27.7% |
| 4.5 ~ 5.5 | 정격 근방 | 45,040 h | 17.1% | 16.4% |
| **5.5 ~ 7.0** | **정격 상단** | **107,973 h** | **41.1%** | **54.7%** |

**대표 속도 후보와 엔트레인먼트 속도** (#103 기준 · `u_e = (π·n·D_pw/60)·(1−γ²)/2`)

| 후보 | n [rpm] | u_e [m/s] | 근거 |
| --- | ---: | ---: | --- |
| DLC1.2 하한 | 3.50 | 0.368 | 저풍속 |
| 가동시간 가중 평균 | 4.93 | 0.519 | 운전 시간 기준 |
| DLC1.2 상한 | 6.19 | 0.651 | 정격 |
| (참고) DLC4.1 | 0.98 | 0.103 | 정지 과정 |
| (참고) DLC6.4 | 0.04 | 0.004 | 유휴 — 유막 형성 거의 없음 |

**#1 · #210 기준 범위** — 같은 회전수에서 설계안에 따라 u_e 가 달라진다.

| n [rpm] | #1 | #103 | #210 |
| ---: | ---: | ---: | ---: |
| 3.50 | 0.318 | 0.368 | 0.421 |
| 4.93 | 0.447 | 0.519 | 0.592 |
| 6.19 | 0.562 | 0.651 | 0.744 |

**→ 시험 속도 창: u_e = 0.32 ~ 0.74 m/s** (세 설계안 × 정상운전 rpm 전 범위). 장비 상한 4 m/s 안이고, 문헌 감쇠시험(0.1 ~ 0.2 m/s)보다 2 ~ 4배 빠르다.

> **대표 회전속도는 아직 확정하지 않는다** (사용자 지시, 260820). 정상운전 DLC1.2 의 회전속도는 3.50 ~ 6.19 rpm 에 분포하고, 가동시간으로 보면 중속(3.0 ~ 4.5 rpm)이 31.8% 로 가장 길다. 하한·가동시간 가중·상한 세 후보 중 무엇을 대표로 삼을지는 **시험 목적의 우선순위**에 따라 정한다. 결정 시 §9 미결 1 을 갱신한다.

### 6-1.2 대표 온도 (확정)

**30 / 50 / 70 °C 3수준.** 50 °C 는 기존 DLC 피로해석·사이징 최적화가 쓴 값이라 해석 체계와 입력이 일치하고, 30·70 °C 는 계절·부하 변동 폭을 감싼다. 온도-점도 의존성을 시험에서 직접 확인한다.

### 6-1.3 대표 접촉압력 — 장비 기준 역설정 (확정)

실기 접촉압력(σ_max 2,093 ~ 2,100 MPa 수준)은 장비로 재현할 수 없다. 따라서 **압력은 상사 대상에서 제외**하고, 장비 가능범위(사파이어-강 롤러 최대 759 MPa)를 3수준으로 나눠 **압력 의존성의 경향**만 확보한다. 실기 압력으로의 확장은 모델을 통한 외삽으로 처리하며, 그 타당성 논증은 §9 미결 2 로 남는다.

### 6-2. 장비 능력 vs 실기 요구 — 압력 갭

RFQ 회신 기준 시편 조합별 접촉압력이다.

| 디스크             | 전동체                | 평균 압력 | **최대 압력** | 측정         |
| ------------------ | --------------------- | --------: | ------------------: | ------------ |
| 유리               | 강 (roller)           |   275 MPa |   **412 MPa** | 유막·트랙션 |
| **사파이어** | **강 (roller)** |   506 MPa |   **759 MPa** | 유막·트랙션 |
| 강                 | 강 (roller)           |   421 MPa |             631 MPa | 트랙션 전용  |

- 실기 TRB의 운전 접촉압력(≈1 GPa 이상)과 **직접 일치시킬 수 없다.** 유막 측정이 가능한 최대는 **사파이어 디스크 759 MPa**다.
- **대응**: 압력은 상사 대상에서 제외하고 **무차원수(M·L)로 대응**한다(확정 #4). 압력 의존성은 장비 가능범위 내 3수준으로 **경향**을 확보하고, 실기 압력으로의 외삽은 모델을 통해 수행한다.
- **리스크**: 압력 손실 항(Cann/Van Zoelen)은 접촉압력에 강하게 의존한다. 저압 영역에서 맞춘 모델을 고압으로 외삽하는 것의 타당성은 **별도 논증이 필요**하다 → §9 미결 2

### 6-3. 접촉 형상 — 타원비 k 와 감쇠시간

문헌에서 확인된 관계: **감쇠시간은 타원비 k = b/a 의 제곱에 대략 비례**한다. k를 10배 키우면 같은 유막 감소에 걸리는 시간이 약 100배 길어진다.

**롤러-디스크는 넓은 타원접촉(큰 k)이므로 감쇠가 매우 느리다.** 실제로 문헌 사례에서 원형접촉은 17분 만에 기아도가 0.18→0.04로 떨어진 반면, 넓은 타원접촉은 117분에 걸쳐 0.61→0.38 변화에 그쳤다.

| 함의                                                 | 대응                                                                                              |
| ---------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| 감쇠 시험 1건이**수~수십 시간** 소요될 수 있음 | 확정#7(장시간 연속)과 정합. 무인 연속기록 설정 필요                                               |
| 시험 기간이 롤러 길이(k)에 좌우됨                    | **롤러 시편 제원(직경·유효길이·크라우닝)을 조기에 확정**해야 일정 산출 가능 → §9 미결 3 |
| 실기 TRB의 k 와 시편 k 를 맞출수록 감쇠 거동이 유사  | 시편 선정 시 k 정합을 고려                                                                        |

### 6-4. 기아 정도의 제어 — 가장 중요한 설계 변수

Van Zoelen 모델은 **심각한 기아**에서만 엄밀히 성립한다. 문헌 시험조차 "중간 정도 기아"라 모델을 참고용으로만 사용했다. 우리 시험에서는 이를 **의도적으로 나누어** 설계한다.

| 조건                    | 공급 방식                        | 기대 기아도       | 검증 대상                  |
| ----------------------- | -------------------------------- | ----------------- | -------------------------- |
| **완전유막 근접** | 그리스 스쿠프 사용               | h_c,0/h_c,ff → 1 | 기유 EHL·보충 모델        |
| **자연 기아**     | 스쿠프 미사용, 초기 충전 후 방치 | 점차 감소         | **Van Zoelen 감쇠**  |
| **보충 작동**     | 궤도 측면 그리스 저장부 형성     | 중간값에서 정체   | **블리딩 보충 모델** |

**→ 이 세 조건의 h(t)를 비교하면 "압력 손실"과 "블리딩 보충"을 실험적으로 분리할 수 있다.** 이것이 문헌의 미검증 구간을 메우는 핵심 설계다.

### 6-5. 온도 제어

- 그리스는 오일과 달리 포트에서 직접 열전달이 되지 않아 **소킹 시간이 길다**(장비사 의견). 온도 프로브를 접촉부 가까이 두되 접촉하지 않도록 배치한다.
- 감쇠 시험은 장시간이므로 **실온 변동이 결과에 개입**할 수 있다(문헌에서도 야간 실온 변화가 베어링 온도에 영향). 항온 관리·실온 동시 기록이 필요하다.

### 6-6. 속도 상한

장비사 의견에 따르면 **4 m/s 초과에서는 그리스가 기아에 빠져 사실상 기유의 유막 형성 성능만 측정**된다. 우리 목적(그리스 거동)에는 저속~중속 영역이 적합하며, 이는 메인베어링 저속 조건과도 부합한다.

---

### 6-7. 그리스 후보 검토 — ISO 점도비 κ (260820)

**대상 4종** — `grease_final_한건연 송부.xlsx` 「기본 스펙 비교」 시트에서 선정.

| 그리스 | 제조사 | 증주제 | 기유 | 기유 VG | ν@40 °C | ν@100 °C | NLGI | 사용온도 [°C] |
| --- | --- | --- | --- | --- | ---: | ---: | :-: | --- |
| **Mobil SHC Grease 681 WT** | ExxonMobil | 리튬 복합 | 합성 | VG 680 | 680 | 74 | 1.5 | −40 ~ 150 |
| **Gadus S5 V460KP 1.5** | Shell | 리튬-칼슘 복합 | 완전 합성 | VG 460 | 460 | — | 1.5 | −40 ~ 150 |
| **Tribol GR SW 460-1** | Castrol | 리튬 복합 | 합성 | VG 460 | 460 | 53 | 1 | — |
| **STABYL EOS E 2** | FUCHS | 리튬 | 합성(PAO) | VG 320 | 320 | — | — | −45 ~ 130 |

#### 6-7.1 산출 방법

```
요구 점도   ν₁ = 45000 · n^(−0.83) · dm^(−0.5)      [ISO 281 · n < 1000 rpm]
평균 지름   dm = 0.5 (d + D)                        [ISO 281 정의]
점도비     κ = ν(70 °C) / ν₁
```

- **운전 온도 70 °C** 기준. `ν(70 °C)` 는 원자료 시트가 40 · 100 °C 값을 Walther(ASTM D341)로 보간한 값을 그대로 사용했다.
- **회전수 n** 은 §6-1.1 의 정상운전 대표 속도 3수준(3.50 / 4.93 / 6.19 rpm)이다.
- 판정 관문은 원자료 시트와 동일하게 **κ ≥ 1**.

**설계안별 dm 과 요구 점도 ν₁**

| # | d [mm] | D [mm] | **dm** [mm] | ν₁ @3.50 rpm | ν₁ @4.93 rpm | ν₁ @6.19 rpm |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 2,907 | 4,039 | 3,473 | 270.0 | 203.1 | 168.2 |
| **103** | 3,546 | 4,498 | **4,022** | **250.9** | **188.8** | **156.3** |
| 210 | 4,199 | 4,983 | 4,591 | 234.8 | 176.7 | 146.3 |

#### 6-7.2 κ 산출 결과

**대표 설계안 #103** (dm 4,022 mm)

| 그리스 | 기유 VG | ν(70 °C) | κ @3.50 | κ @4.93 | κ @6.19 | κ ≥ 1 |
| --- | --- | ---: | ---: | ---: | ---: | :-: |
| Mobil SHC Grease 681 WT | VG 680 | 191 | 0.76 | **1.01** | **1.22** | 상한·가중에서만 |
| Gadus S5 V460KP 1.5 | VG 460 | 137 * | 0.55 | 0.73 | 0.88 | 전 구간 미달 |
| Tribol GR SW 460-1 | VG 460 | 133 | 0.53 | 0.70 | 0.85 | 전 구간 미달 |
| STABYL EOS E 2 | VG 320 | 95 * | 0.38 | 0.50 | 0.61 | 전 구간 미달 |

**#1 · #210 비교** (κ @4.93 rpm 기준)

| 그리스 | #1 (dm 3,473) | **#103** (dm 4,022) | #210 (dm 4,591) |
| --- | ---: | ---: | ---: |
| Mobil SHC Grease 681 WT | 0.94 | **1.01** | 1.08 |
| Gadus S5 V460KP 1.5 | 0.67 | 0.73 | 0.78 |
| Tribol GR SW 460-1 | 0.65 | 0.70 | 0.75 |
| STABYL EOS E 2 | 0.47 | 0.50 | 0.54 |

\* `ν(70 °C)` 가 추정치인 항목 — 100 °C 실측값이 없어 원자료 시트가 VG 등급에서 추정했다.

#### 6-7.3 결과

- **κ ≥ 1 을 만족하는 조합은 `Mobil SHC Grease 681 WT` 가 4.93 · 6.19 rpm 에서 통과하는 경우뿐**이다. 나머지 3종은 세 설계안 · 세 속도 전 조합에서 1 미만이다.
- 저풍속(3.50 rpm)에서는 **4종 모두 κ < 1** 이다.
- 설계안 간 차이는 작다 — dm 이 3,473 → 4,591 mm 로 32% 커져도 κ 는 약 15% 오르는 데 그친다(`ν₁ ∝ dm^(−0.5)`).
- 속도 의존성이 지배적이다 — 3.50 → 6.19 rpm 에서 요구 점도가 250.9 → 156.3 mm²/s 로 38% 낮아진다(`ν₁ ∝ n^(−0.83)`).

> **원자료 시트와 결과가 다른 이유** — 시트는 `dm = 3,000 mm · n = 10 rpm` 을 전제해 `ν₁ = 122 mm²/s` 를 썼고, 그 기준에서는 위 4종 중 3종이 통과였다. 본 검토는 §6-1.1 에서 확정한 실기 대표 운전점(dm 3,473 ~ 4,591 mm · n 3.50 ~ 6.19 rpm)을 적용한 것이며, 회전수가 10 → 4.93 rpm 으로 낮아지면서 요구 점도가 1.55배로 올라간 것이 차이의 주된 원인이다. 같은 식(ISO 281)에 시트 전제를 넣으면 121.5 mm²/s 로 시트 값이 재현되므로, 산출식 자체는 일치한다.

---

## 7. 기타 검토 사항

| # | 항목                     | 내용                                                                                               |
| -: | ------------------------ | -------------------------------------------------------------------------------------------------- |
| 1 | **시편 표면조도**  | Λ 산출에 필요. 시험 전후 실측하고 마모 여부 확인                                                  |
| 2 | **그리스 충전량**  | 감쇠 거동에 직접 영향. 정량 충전 절차와 재현성 확보 방법 필요                                      |
| 3 | **측정 불확도**    | 유막 1~1000 nm·분해능 1 nm. 저속·박막 영역에서의 불확도 별도 평가                                |
| 4 | **반복성**         | 그리스 윤활은 이력 의존성이 크다. 동일 조건 반복 횟수(최소 2회) 규정                               |
| 5 | **시험 순서 효과** | 하중·속도 스윕 순서가 결과에 영향(문헌은 감소→증가→감소 순으로 이력 확인). 순서를 고정하고 기록 |
| 6 | **기유 물성**      | 점도(2점 이상)·압력점도계수 α·밀도·열전도도. 모델 입력값이므로 출처 명시                       |
| 7 | **데이터 처리**    | 샘플링율·대표값 산출 방식(중앙값/구간평균) 사전 규정                                              |
| 8 | **안전·운영**     | 장시간 무인 운전 시 안전 조건, 시료 취급                                                           |

---

## 8. 산출물

| 산출물                    | 내용                                                                 |
| ------------------------- | -------------------------------------------------------------------- |
| **시험계획서 본편** | 본 문서의 결정을 반영한 정식 계획서 (조건표·절차서·판정기준·일정) |
| 조건 산출 워크시트        | 실기 대표점 → 무차원수 → 장비 조건 매핑 계산                       |
| 시험 데이터셋             | h_c 스윕 · h(t) 감쇠 원자료                                         |
| 모델 대조 보고서          | 모델별 예측 vs 실측, 적용범위 판정                                   |

---

## 9. 미결 사항 — 본편 작성 전 확정 필요

| # | 항목                           | 필요한 결정·작업                                                                                          |
| -: | ------------------------------ | ---------------------------------------------------------------------------------------------------------- |
| 1 | **실기 대표 운전점** | 🔶 **부분 해소 (260820, §6-1.1)** — 기준 설계안 #103(부록 10-11.S3-c) · 접촉 기하 · 속도 창 u_e 0.32 ~ 0.74 m/s · 온도 30/50/70 °C · 압력 장비 역설정까지 확정. **남은 것: 대표 회전속도 1점 선정** — 하한 3.50 / 가동시간 가중 4.93 / 상한 6.19 rpm 중 시험 목적 우선순위에 따라 결정 |
| 2 | **저압→고압 외삽 논거** | 장비 상한(759 MPa)과 실기(≈1 GPa 이상)의 격차를 어떻게 정당화할지. 무차원수 정합만으로 충분한지 검토      |
| 3 | **롤러 시편 제원**       | 직경·유효길이·크라우닝·재질. 감쇠시간(∝k²)과 시험 일정에 직결                                         |
| 4 | **디스크 재질**          | 사파이어(759 MPa) 채택 여부 — 비용 대비 압력 확보                                                         |
| 5 | **그리스 2종 확정**      | 기유 점도 대비 구성. 실기 후보 그리스 포함 여부                                                            |
| 6 | **스쿠프 사용 여부**     | §6-4 세 조건을 모두 볼지, 일부만 볼지                                                                     |
| 7 | **시험 기간·대수**      | 감쇠 1건 수~수십 시간 × 조건 수 → 총 소요 산출 후 범위 조정                                              |
| 8 | **장비 보유 상태**       | EHD/EHD-HS 도입 여부·시점 (RFQ 진행 중). 미도입 시 외부 시험 대안 검토                                    |
| 9 | **TRB 축방향 속도 구배 미재현** | 실기 TRB 는 접촉선을 따라 u_e 가 소단→대단으로 #103 기준 +5.7% 변하지만, 원통 롤러-디스크 리그는 균일하다. 유막두께 환산 시 ±1.8% 수준이라 조건 분리는 하지 않되, 모델 대조에서 편차 요인으로 남겨둔다 |

---

## 10. 참고 자료 (변환본)

| 문헌                                                               | 역할                                                                 |
| ------------------------------------------------------------------ | -------------------------------------------------------------------- |
| `2012. (Lugt) Thin layer flow and ~.md`                          | 궤도 잔류층 재유동·감쇠 이론                                        |
| `2014. (SKF) On the Film Thickness…Low Speeds.md`               | 저속 영역 유막 거동                                                  |
| `2024. (SKF) Film Thickness…Deep Groove Ball.md`                | 볼베어링 유막·기유 물성 상관                                        |
| `2026. (Lugt) Effect of speed, load, grease type…CRB.md`        | 정상상태 h_c 의 속도·하중·그리스 의존성, 처닝·러닝인 절차         |
| `2026. (Lugt) Film thickness decay…wide elliptical contacts.md` | **감쇠 h(t)·타원비 효과·기아도** — 본 시험 설계의 직접 근거 |
| `2026. (Lugt) Transient Analysis…Bleed Phase in CRB.md`         | **블리딩 보충 모델** — 검증 목표                              |
| `2026. (SKF) Electrical capacitance method…CRB.md`              | 대안 측정법(전기용량) — 베어링 단위 측정 시 참고                    |
| `EHD2 fully automated manual.md`                                 | 장비 절차·사양                                                      |
| `Request for Quotation…EHD and EHS-HS Systems.md`               | **시편 조합별 접촉압력·측정범위** — §6-2 근거               |
| Harris & Kotzalas (2006), *Rolling Bearing Analysis, 5th ed. — Essential Concepts of Bearing Technology*, CRC Press | **베어링 키네매틱스·접촉 기하** — §2.5.3 Curvature · §10.2 Cage Speed · §10.3 Rolling Element Speed (`AI_Seminar_BB/BB-main/Reference/`) |

---

# 부록 1. 그리스 유막두께 정의 검토 — "속도-유막 곡선"은 정상상태인가

## A1-1. 문제 제기

문헌에는 성격이 다른 두 종류의 그래프가 섞여 있다.

| 유형 | 가로축 | 대표 문헌 | 우리 지표 |
|---|---|---|---|
| **(가) 속도-유막 곡선** | 엔트레인먼트 속도 | 2014 Cen · 2024 Shetty · 2026 Gao(CRB) | 정상상태 `h_c` |
| **(나) 감쇠 곡선** | 시간 | 2012 Venner · 2026 Gao(elliptical) · 2026 Puthumana | `h(t)` · 감쇠시간 |

**(가)의 각 점은 어떤 조건에서 측정된 값이며, 정상상태로 볼 수 있는가?** 이것이 정해지지 않으면 우리 시험의 B-1(정상상태)과 B-2(감쇠)를 어떻게 구분해 설계할지 정할 수 없다.

## A1-2. 문헌별 실제 측정 조건 — 원문 확인

### ① 2014 Cen, Lugt & Morales-Espejel — 저속 영역 (WAM5, ball/roller-on-disc)

출처: `2014. (SKF) On the Film Thickness of Grease-Lubricated Contacts at Low Speeds.md` §WAM5

> "The tests were run at surface speeds ranging from 10⁻⁴ to 0.2 m/s. **The surface speed was ramped up and down within 15 min. A scoop was used to ensure that the tests were done under fully flooded conditions.**"

→ 속도를 **15분 안에 올렸다 내리는 램프(ramp)** 방식이며, **스쿠프로 완전유막을 강제 공급**했다. 각 점에서 정상상태를 기다린 것이 아니다. 시험 조건은 하중 20 N, 최대압력 0.46 GPa(볼) / **0.77 GPa(롤러)**, 조도 Rq < 10 nm 이다(Table 1).

### ② 2024 Shetty et al. — 심구 볼베어링 마스터 커브 (전기용량법)

출처: `2024. (SKF) Film Thickness in Grease-Lubricated Deep Groove Ball.md` §Radial Load

> "**Instantaneous film thickness is measured for about 10–15 min for each speed. The film thickness becomes stable within 1–2 min after setting new speed/load conditions**... The time average value of the measured capacitance during this period is considered for further analysis. **All measurements are repeated at least twice.**"

→ 조건 변경 후 **1~2분이면 안정**되고, 그 뒤 **10~15분 평균**을 대표값으로 쓴다. 측정 전 처닝(churning)을 거친다 — 자유공간의 30%를 그리스로 채우고 4,000 rpm(u = 6.6 m/s)에서 운전. 같은 논문은 이 상태를 이렇게 규정한다.

> "It was observed that **the film thickness after the churning phase is almost steady.**"

### ③ 2026 Gao et al. — CRB 속도·하중 스윕 (전기용량법)

출처: `2026. (Lugt) Effect of speed, load, grease type on film thickness in CRB.md` §Steady-state speed and load sweep

- 처닝·러닝인을 **90~150시간** 수행하고, 유막·온도가 안정된 뒤 스윕을 시작한다. 안정까지 걸린 시간은 **25~30시간**으로 관측됐다.
- 온도를 35 ± 1 °C로 제어하고, 각 스텝에서 **1시간 중앙값**을 대표값으로 쓴다(샘플링 10 kHz).
- 논문은 이 값을 "정상상태"라 부르면서도 동시에 **"early-life film thickness ... after the grease churning phase"** 로 한정한다.

### ④ 2026 Gao et al. — 넓은 타원접촉 감쇠 (WAM, **테이퍼 롤러 + 유리 디스크**)

출처: `2026. (Lugt) Film thickness decay in grease lubricated wide elliptical contacts.md` §2

- **테이퍼 롤러와 반사 코팅 유리 디스크** 조합으로 측정했다. 우리가 계획한 롤러-디스크 구성과 같은 계열이다.
- 최대 접촉압력이 0.22 GPa(WAM) / 0.48 GPa(PCS EHD)로 낮은 이유를 논문이 직접 밝힌다.

> "The relatively low-pressure conditions in this study with **maximum contact pressure below 0.5 GPa are primarily due to the limitation of the experimental setup, particularly, the coated glass disc.**"

- 감쇠는 **시간축**으로 기록되며, 속도 100 / 150 / 200 mm/s 각각에서 별도의 감쇠 곡선을 얻는다.

### ⑤ 2026 Puthumana, van Zoelen & Lugt — 블리드 단계 과도해석

출처: `2026. (Lugt) Transient Analysis of Film Thickness During Bleed Phase in CRB.md` §Abstract

> "...film thickness plateau, resulting from a nearly constant early-stage oil supply and self-regulating nonlinear EHL losses. **At longer times, the film thickness gradually decreases, governed by the characteristic time scale of grease bleed**, indicating that oil availability and rate of supply control the long-term film behavior."

→ 처닝 이후의 "안정 구간"은 **평탄부(plateau)**일 뿐이고, 더 긴 시간 척도에서는 **블리드 속도에 지배되어 서서히 감소**한다.

### ⑥ 2012 Venner, van Zoelen & Lugt — 박층 유동(thin layer flow) 모델

출처: `2012. (Lugt) Thin layer flow and ~.md` §Abstract · §1 Introduction

**정의 — 이 논문은 "층 두께"와 "유막두께"를 같은 것으로 쓴다.** 초록이 예측 대상을 `layer thickness (film thickness) decay` 로 병기한다. 즉 기아 영역에서 말하는 유막두께는 **궤도 위에 남아 접촉부로 들어가는 공급층의 두께**다.

> "A model is presented to predict **lubricant supply layer changes on tracks** in rolling bearings due to centrifugal forces and elastohydrodynamic contact pressure. Experimental validation is shown for centrifugal force driven free surface flow, and **layer thickness (film thickness) decay** in single elastohydrodynamically lubricated contacts."

**감쇠를 일으키는 두 기구**

| 기구 | 내용 |
|---|---|
| **원심력** | 회전에 의한 자유표면 유동으로 궤도 위 층이 이동 |
| **압력 배출(pressure ejection)** | EHD 접촉압력이 층을 접촉 밖으로 밀어냄 |

논문은 기아 영역의 발생 과정을 이렇게 서술한다.

> "The rolling contact pushes aside the grease in the early overrollings creating grease levees to the side. **The film thickness initially decays with time as grease pushed to the side does not flow back onto the track for relubrication.**"

**재윤활 간격과의 연결** — 보충을 모두 무시하면 이 모델은 **최악(worst case) 예측**이 되고, 그 결과는 재윤활 간격의 상한으로 읽을 수 있다.

> "In its simplest form, **all replenishment ignored, the model yields a worst case film thickness decay prediction.** The results can be interpreted as a **maximum allowable interval between replenishment events** in a bearing, ... or be used to determine a required rate of replenishment by e.g. bleeding, to sustain a certain film thickness level."

**계산상의 이점** — 수백만 회의 과임(overrolling)마다 기아 EHL 문제를 푸는 방식을 피한다. 논문은 선행 방식(과임 단위 접근)의 한계를 이렇게 지적한다.

> "...this approach cannot easily be extended to the case of rolling bearings where **the number of overrollings is not well defined** as the lubricant layers on the inner and outer raceway and the r[oller]..."

검증은 **단일 원형·타원 접촉**에 대해 광간섭으로 수행되었고 "excellent agreement" 로 보고된다.

### ⑦ 2018 Fischer, Jacobs, Stratmann & Burghardt — 기유 종류가 유막 형성에 미치는 영향 (RWTH)

출처: `2018. (RWTH) Effect of Base Oil Type in Grease on Film Formation in EHD Contacts.md` (Lubricants 2018, 6, 32) · §2 Materials and Methods

**이 논문은 지금까지의 문헌 중 유일하게 공급 조건의 양극단을 한 연구에서 모두 구현했다.** 볼-온-디스크 광간섭 리그, 순수 구름(SRR = 0%), 속도를 단계적으로 올리며 측정한다.

**(1) 완전유막 조건 — 스쿠프**

> "When measuring under fully flooded lubrication, an additional element, the so-called **scoop**, is placed directly in front of the EHD contact... This trapezoidal grease reservoir is pressed against the disc, so that **the EHD contact is continuously supplied with grease. In this state, the contact is always fully flooded and the onset of starvation can be prevented.**"

**(2) 기아 조건 — 정량 도포 후 차단**

> "For film thickness measurements under starved lubrication, **a grease film with a definite height of approx. 0.1 mm is applied to the disc initially** before the measurement. This leads to a **total mass of approx. 1.0 g grease on the disc** for each test."

도포 균일화를 위해 측정 전 **30 mm/s 정속으로 15분 러닝인**을 거친다. → 우리 §4-5 의 "스쿠프 장착 / 미장착" 구분에 대한 **직접적인 선례이며, 기아 조건을 "정량 도포 + 러닝인" 으로 규격화**한 점이 참고가 된다.

**(3) 스타베이션의 정의 — 속도를 올려도 유막이 늘지 않는다**

> "With increasing rolling speed, the film thickness of oil lubricated contacts usually grows. However, in case of grease lubricated contacts, which are not fully flooded, **the film thickness remains constant or even decreases with further increasing rotational speed. This effect is referred to as starvation. Since the onset of starvation depends on the grease composition,** the film formation of two different grease compositions is investigated"

→ 부록 1 주 2 의 기아 논의와 이어진다. 여기서는 기아를 **"속도-유막 곡선의 기울기가 꺾이는 지점"** 으로 관측 가능하게 정의했고, 그 지점이 **그리스 조성에 따라 달라진다**는 것이 연구의 출발점이다.

**(4) 그리스 · 기유 · 블리드유를 함께 측정**

> "The film thickness measurements are performed on a ball-on-disc tribometer **for each grease, as well as the corresponding bleed and pure base oils.**"

- 시료: NLGI 2 · 무첨가 · 리튬 복합 증주제 2종 — **PAO-Li-100**(기유 98.01 mm²/s @40 °C · 22.14 @80 °C) · **PAG-Li-140**(141.71 @40 °C · 40.64 @80 °C)
- 블리드유는 **DIN 51817** 절차로 추출
- 표면조도 Ra ≈ 0.02 µm 로 혼합윤활을 배제

→ 우리 Phase A(기유 선행 시험)의 선례다. 다만 이 논문은 **블리드유까지 세 번째 시료로 두었다** — 그리스에서 실제로 나오는 오일이 순수 기유와 다를 수 있기 때문이다.

**(5) 반복 5회**

> "All measurements, under fully flooded and starved lubrication, **were performed five times** with increasing rolling speed. The mean values of the results were taken and are presented in the diagrams **with standard deviation error bars** to indicate the scattering of the results."

논문은 그 이유를 그리스의 거동이 재현되기 어렵다는 데서 찾는다.

> "Due to the known **chaotic behaviour of grease**, the reproducibility of measurement results is difficult to achieve. Therefore, the test setup and conditions for each measurement should be hold as similar as possible."

## A1-2.1 문헌 분류 — 네 축

| 문헌 | ⓐ 감쇠 여부 | ⓑ **재보충 조건** | ⓒ **스타베이션 상태** | ⓓ 시험 형태 |
|---|---|---|---|---|
| ① 2014 Cen et al. | **비감쇠** (스쿠프로 완전유막 유지) | **강제 공급** — 스쿠프로 접촉부에 지속 공급 | **완전유막 의도** — 기아를 피하려 스쿠프·실드 사용 | **디스크**(WAM5) + 베어링(Tractor) 병행 |
| ② 2024 Shetty et al. | **비감쇠** (준정상, 10~15분 평균) | **자연 보충** — 처닝 후 궤도 잔류층·측면 저장부 | **기아** — 기아도를 `h_g/h_ff` 로 정량. 속도 증가가 기아를 개시 | **베어링** (DGBB · 전기용량) |
| ③ 2026 Gao et al. (CRB) | **비감쇠** (준정상, 1시간 중앙값) | **자연 보충** — 케이지·궤도 잔류 그리스의 블리드 | **기아** — 블리드로 공급되는 기아 접촉 | **베어링** (CRB · 전기용량) |
| ④ 2026 Gao et al. (타원접촉) | **감쇠** (시간축 곡선) | **능동 억제** — 제방을 트랙 밖으로 밀어내 되돌아옴 차단 (주 1) | **약한 기아(mild)** — 저자들이 직접 그렇게 규정. `h_c,0/h_cff` 0.61 ~ 0.91 | **디스크** (WAM 테이퍼 롤러 + 유리 디스크 · PCS EHD) |
| ⑤ 2026 Puthumana et al. | **감쇠** (블리드 단계 과도) | *(모델 가정)* **보충 포함** — 블리드를 지수함수로 모델링 | *(모델 가정)* **기아 전제** — 선행 모델의 심한 기아 가정을 보충 포함으로 확장 | **베어링** (모델 중심) |
| ⑥ 2012 Venner et al. | **감쇠** (층 흐름 모델) | *(모델 가정)* **보충 무시** — 최단순형은 최악 예측 | *(모델 가정)* **기아 영역 전제** — "starved regime" 을 대상으로 명시 | **디스크** 검증 → 베어링 확장 |
| ⑦ 2018 Fischer et al. (RWTH) | **비감쇠** (속도 단계 스윕) | **두 조건 모두 수행** — 강제 공급(스쿠프) ↔ 초기 정량 도포(0.1 mm · 1.0 g) 후 차단 | **두 상태 모두 의도적 구현** — 완전유막에서는 기아 개시를 막고, 기아 조건에서는 개시 속도를 관측 | **디스크** (볼-온-디스크 광간섭) |

> **주 1 — ④ 의 능동 억제 기법.** 접촉 초기에 궤도 양옆으로 밀려난 그리스 제방(levee)은 층 두께에 비해 커서 트랙으로 되돌아와 보충원이 된다. 이를 줄이기 위해 논문은 **디스크의 트랙 반경을 굴리는 동안 조금씩 옮겨 제방을 양쪽으로 약 2.5 mm 씩 밖으로 밀어낸 뒤**, 롤러를 원래 트랙 반경으로 되돌려 측정했다.
>
> > "These levees are relatively large compared to the lubricant layer thickness in the track, **resulting in track replenishment. To minimize this replenishment,** the radius of the track on the disc R_disc ... **is gradually adjusted during rolling, pushing the levees about 2.5 mm out on each side of the track.**"
>
> 출처: `2026. (Lugt) Film thickness decay in grease lubricated wide elliptical contacts.md` §3 · 롤러 R_disc = 45 mm · 볼 R_disc = 40 mm

### 주 2 — 스타베이션과 감쇠는 다른 것인가

**질문**: "재보충 부족에 의한 유막 감쇠" 와 "인렛부 유체 공급 부족에 의한 스타베이션" 은 서로 다른 현상인가.

원문에서 starvation 이 정의·서술된 대목을 모으면 답이 나온다.

**① 스타베이션의 정의 — 원인은 두 가지다**

> "**Starvation occurs when the bearing is not sufficiently packed with grease and/or when there is not enough time for replenishment of the running track.**"
> — `2014. (SKF) On the Film Thickness…Low Speeds.md` §Introduction

즉 ⓐ 그리스 양 부족 ⓑ **궤도 재보충 시간 부족** 이다. 두 번째가 곧 "재보충 부족" 이다.

**② 기아 상태의 정량 — 완전유막 대비 비율**

> "**The degree of starvation is measured as the ratio of the measured grease film thickness (h_g) to the fully flooded base oil film thickness (h_ff)** calculated using Hamrock and Downson's equation"
> — `2024. (SKF) Film Thickness in Grease-Lubricated Deep Groove Ball.md` §Summary of previous findings

④ 도 같은 지표를 쓰며, 값이 1 이상이면 완전유막으로 본다.

**③ 감쇠는 기아 영역 안에서 일어나는 시간 변화다**

> "The second case is referred to as **the starved regime**. The rolling contact pushes aside the grease in the early overrollings creating grease levees to the side. **The film thickness initially decays with time as grease pushed to the side does not flow back onto the track for relubrication.**"
> — `2012. (Lugt) Thin layer flow and ~.md` §1

> "As the amount of oil available for lubrication is often very small **the contact is starved and operates at a film thickness level much below the fully flooded limit.** In between overrollings **very little replenishment** from the side of the track can take place"
> — 같은 문헌 §1

**④ 기아의 정도에 따라 적용 모델이 달라진다**

> "the Van Zoelen model is **characteristic of heavily starved contacts**. Under this condition, the influence of speed has been shown to be minimal. According to Van Zoelen, **in case of mild starvation, side flow is more pronounced** due to a longer pressure build-up in the inlet"
> — `2026. (Lugt) Film thickness decay…wide elliptical contacts.md` §4.3

> "this model ... only considers side flow in elliptical contacts without accounting for reflow when the contacts are severely starved. Hence, **while this model is not strictly applicable due to the mild level of starvation in our tests,** we have used it as a reference."
> — 같은 문헌 §4.3

**⑤ 같은 층 두께라도 속도가 오르면 기아가 심해진다**

> "**For a given film thickness, higher speeds lead to more severe starvation.** The measurement results at higher speeds are therefore closer to the predictions of the Van Zoelen model, which is only applicable to severely starved conditions."
> — 같은 문헌 §5 결론

**정리 — 다른 현상이 아니라 "상태"와 "과정"의 차이다**

| 구분 | 스타베이션 | 감쇠 |
|---|---|---|
| 성격 | **상태(state)** — 인렛이 채워지지 않은 정도 | **과정(process)** — 그 상태가 시간에 따라 나빠지는 것 |
| 지표 | `h/h_ff` (기아도) — 한 시점의 값 | `h(t)` · 감쇠시간 — 시간 이력 |
| 원인 | 그리스 양 부족 **또는** 재보충 시간 부족 | 측방유동 손실 > 재보충 |

- **감쇠는 기아를 만드는 기구이고, 기아는 감쇠의 결과 상태다.** 인과가 `재보충 부족 → 궤도 층 두께 감소(감쇠) → 인렛 충전 부족(기아) → 유막 저하` 로 이어지므로 별개 현상이 아니다.
- 다만 **기아가 감쇠 없이도 생길 수 있다.** ①의 정의에서 "그리스 양 부족" 이 그 경우이고, ⑤ 처럼 **같은 층 두께에서 속도만 올려도 기아도가 나빠진다.** 감쇠는 기아의 충분조건이 아니라 원인 중 하나다.
- 실무적으로 중요한 것은 **기아의 정도가 적용 가능한 모델을 가른다**는 점이다(④). Van Zoelen 은 심한 기아용이며, 약한 기아에서는 엄밀히 적용되지 않는다 — ④ 의 저자들도 자기 시험이 약한 기아임을 인정하고 참고용으로만 썼다.

**네 축이 말해주는 것**

- **ⓐ 감쇠 여부**가 곧 A1-3 의 "공급 조건" 축이다. 비감쇠 계열은 공급을 유지(①의 스쿠프)하거나 평탄부에서 측정(②·③)했고, 감쇠 계열은 공급이 끊긴 상태의 시간 변화를 다룬다.
- **ⓑ 재보충 조건이 ⓐ 를 결정한다.** 감쇠 여부는 그리스나 속도가 아니라 **오일이 다시 들어오는가**로 갈린다. 같은 리그·같은 그리스라도 ① 처럼 스쿠프를 달면 비감쇠, ④ 처럼 제방을 제거하면 감쇠가 된다. 특히 ④ 는 **가만히 두면 제방에서 보충이 일어난다**는 사실을 역으로 보여준다 — 디스크 시험에서 "보충 없음"은 저절로 얻어지지 않고 **의도적으로 만들어야 하는 조건**이다.
- 보충의 강도는 네 단계로 늘어선다: **강제 공급(①) > 자연 보충(②·③) > 능동 억제(④) > 보충 무시(⑥, 모델)**. ⑤ 는 자연 보충을 정량 모델로 옮긴 경우다.
- **ⓓ 시험 형태**가 다르면 같은 "유막두께"라도 측정 대상이 다르다. 디스크 시험은 **단일 접촉**을 광학적으로 직접 보고, 베어링 시험은 **최대부하 위치의 접촉**을 전기용량으로 환산한다. ②·③ 이 처닝을 요구한 것은 베어링 시험이기 때문이며, 단일 접촉 시험에는 그 개념이 그대로 적용되지 않는다.
- 모델 문헌(⑤·⑥)의 ⓑ·ⓒ 는 **시험 조건이 아니라 모델 가정**이다. 표에서 *(모델 가정)* 으로 구분했다.

## A1-3. 답 — 세 개의 시간 척도로 갈린다

| 척도 | 현상 | 문헌 근거 | 정상상태인가 |
|---|---|---|---|
| **분** (1~15 min) | 속도·하중 변경 후 재평형 | ②: 1~2분 내 안정, 10~15분 평균 | **국소적으로 예 ** — 측정 가능한 준정상 |
| **시간** (25~150 h) | 처닝 종료, 궤도 그리스 재분포 | ③: 25~30시간 후 안정 · ②: "almost steady" | **조건부 예** — 평탄부 |
| **수백 시간** | 블리드에 의한 오일 고갈 | ⑤: 장시간에서 점진 감소 | **아니오** |

**결론 3가지**

1. **(가) 속도-유막 곡선의 각 점은 "그 시점의 공급 상태에서 측정한 준정상값"이다.** 열적·유동적으로 평형에 이른 값이지만, 그리스 공급 이력(처닝 완료 여부, 잔류 오일량)에 의존한다. ②·③처럼 처닝 후 측정한 값은 **평탄부의 값**이며, 무한히 지속되는 값이 아니다.

2. **(가)와 (나)를 가르는 것은 시간축이 아니라 "공급 조건"이다.** ①은 스쿠프로 오일을 계속 공급해 감쇠 자체를 막았고, ④는 공급 없이 감쇠를 관찰했다. 같은 리그·같은 그리스라도 **스쿠프 유무가 곡선의 성격을 바꾼다.**

3. **따라서 "정상상태 유막두께"는 절대값이 아니라 조건부 정의다.** 보고할 때 ⓐ 처닝/러닝인 이력 ⓑ 공급 방식 ⓒ 측정 구간 길이와 대표값 산출법을 함께 적지 않으면 다른 문헌과 비교할 수 없다.

## A1-4. 시험계획 구성에 대한 의견

### (1) B-1 을 "준정상 (quasi-steady)" 으로 명확히 규정한다

"정상상태"라는 말 대신 **측정 프로토콜로 정의**한다. 문헌 ②·③의 방식을 그대로 따르는 것이 비교 가능성 면에서 유리하다.

| 항목 | 권장 규정 | 근거 |
|---|---|---|
| 조건 변경 후 대기 | 신호 안정까지 대기 후 **추가 10분** | ②의 "1~2분 내 안정" 관측 |
| 측정 구간 | **10~15분** 연속 기록 | ② |
| 대표값 | 구간 **중앙값** (이상치 영향 배제) | ③의 1시간 중앙값 방식 |
| 반복 | **최소 2회** | ② "repeated at least twice" |

### (2) 러닝인(처닝) 완료를 B-1 착수의 전제조건으로 못 박는다

③이 25~30시간을 요했다는 점은 **B-1 이전에 상당한 시간이 필요**함을 뜻한다. 다만 우리 리그는 베어링이 아니라 단일 접촉이므로 처닝 개념이 그대로 적용되지 않는다 — **"유막·온도 신호가 안정될 때까지"** 로 판정 기준을 두고 실제 소요를 기록해 문헌과 비교하는 편이 안전하다.

### (3) 공급 방식을 시험 인자로 명시적으로 나눈다 — 가장 중요

A1-3 의 결론 2에 따라 **스쿠프 유무를 B-1 과 B-2 를 가르는 축**으로 삼는다.

| 시험 | 공급 방식 | 얻는 것 | 대조 모델 |
|---|---|---|---|
| **B-1** | **스쿠프 장착** (완전유막 공급) | 속도·하중·온도 의존성의 준정상 `h_c` | 완전유막 EHL · 기아 보정 |
| **B-2** | **스쿠프 미장착** (공급 차단) | 감쇠 `h(t)` · 감쇠시간 | Van Zoelen |
| **S** | 궤도 측면 저장부 | 보충이 작동하는 중간 거동 | 블리딩 보충 모델 |

이렇게 하면 ①(스쿠프·완전유막)과 ④(공급 없음·감쇠)의 **두 문헌 계열을 각각 재현**하게 되어, 우리 데이터를 양쪽 모두와 직접 비교할 수 있다.

### (4) 보고 서식에 이력 정보를 필수 항목으로 넣는다

각 데이터점마다 다음을 함께 기록한다 — 이것이 없으면 문헌 대조가 성립하지 않는다.

```
공급 방식 (스쿠프 유무) · 충전량 · 러닝인 소요시간 · 조건 변경 후 대기시간
측정 구간 길이 · 대표값 산출법 · 반복 횟수 · 기아도 h_c,0/h_c,ff
```

### (5) 문헌 ④가 우리 구성과 가장 가깝다는 점을 활용한다

④는 **테이퍼 롤러 + 유리 디스크**로 넓은 타원접촉을 만들었고, 압력이 0.5 GPa 미만인 이유도 **코팅 유리 디스크의 한계**라고 명시했다. 우리가 사파이어를 택해 759 MPa 까지 올리는 것은 이 제약을 한 단계 넘는 셈이므로, **④와 같은 압력대(0.2~0.5 GPa)의 조건을 최소 1점 포함**하면 문헌과의 직접 대조점이 생긴다.

> **제안** — B-1 하중 3수준 중 **저압 수준을 0.22 ~ 0.48 GPa 대역**에 맞춘다. ④의 결과와 겹치는 조건이 생겨 우리 리그·절차의 타당성을 문헌으로 교차 검증할 수 있다.

---

# 부록 2. 그리스 유막두께 감쇠 검토의 필요성

## A2-1. 왜 감쇠를 보아야 하는가

### (1) 베어링 수명을 결정하는 것은 윤활제다

> "when a bearing is correctly installed and there are no external factors such as debris, or material defects, **it is often the lubricant that determines the service life.**"
> — `2012. (Lugt) Thin layer flow and ~.md` §1

### (2) 그리스 수명의 정의 자체가 "유막을 유지할 수 있는 기간" 이다

> "**Grease life can loosely be defined as the time during which the grease can ensure the (re)formation of a low shear separation layer** between the rolling element and the raceway surfaces."
> — 같은 문헌 §1

**분리층(separation layer) = 유막**이므로, 그리스 수명은 곧 **유막을 유지할 수 있는 시간**이다. 정상상태 유막두께 `h_c` 는 "지금 얼마나 두꺼운가" 만 말해줄 뿐 **"언제까지 유지되는가" 를 말해주지 않는다.** 그 시간 정보를 주는 것이 감쇠 곡선 `h(t)` 다.

### (3) 개방형 베어링에서는 그리스 수명이 곧 재윤활 간격이다

> "In open bearings the grease may relatively easily be replaced at regular **"re-lubrication" intervals determined by the grease life.** ... For both cases **accurate prediction of grease life is paramount to bearing service life prediction and control.**"
> — 같은 문헌 §1

메인베어링은 재윤활을 전제로 운용하는 개방형이므로 이 문장이 그대로 적용된다. **재윤활 주기 결정은 감쇠 거동을 모르면 근거를 세울 수 없다.**

### (4) 그리스 수명 모델은 아직 경험식 수준이다

> "**Models to predict grease life are much less well developed than fatigue life models and mostly empirical.** The first step toward improved models is sufficient understanding of film formation in grease lubricated contacts."
> — 같은 문헌 §1

피로수명(ISO/TS 16281)은 체계가 서 있지만 **그리스 수명 쪽은 그렇지 않다.** 우리가 사이징 최적화에서 피로수명을 정밀하게 다뤘음에도 윤활 수명이 미검증으로 남아 있는 이유가 여기에 있다.

---

## A2-2. 문헌이 밝힌 감쇠 특성

### (1) 시간 구조 — 평탄부 뒤에 서서히 내려간다

> "...film thickness plateau, resulting from a nearly constant early-stage oil supply and self-regulating nonlinear EHL losses. **At longer times, the film thickness gradually decreases, governed by the characteristic time scale of grease bleed**, indicating that **oil availability and rate of supply control the long-term film behavior.**"
> — `2026. (Lugt) Transient Analysis…Bleed Phase in CRB.md` §Abstract

→ 감쇠는 단조 감소가 아니라 **초기 급감 → 평탄부 → 장기 완만 감소**의 구조를 갖는다. 어느 구간을 측정했느냐에 따라 전혀 다른 결론이 나온다.

### (2) 접촉 형상 — 넓은 접촉일수록 훨씬 느리다

`2026. (Lugt) Film thickness decay…wide elliptical contacts.md` §4.2 는 감쇠시간이 **타원비 `k = b/a` 의 제곱에 대략 비례**함을 보인다. k 를 10배 키우면 같은 유막 감소에 걸리는 시간이 약 100배가 된다.

같은 논문의 실측 대비: 원형접촉은 17분 만에 기아도가 0.18 → 0.04 로 떨어진 반면, 넓은 타원접촉은 117분에 걸쳐 0.61 → 0.38 변화에 그쳤다.

→ **TRB 처럼 접촉선이 긴 형상은 감쇠가 매우 느리다.** 이는 유리한 특성이지만, 동시에 **시험으로 관측하려면 긴 시간이 필요**함을 뜻한다.

### (3) 속도와 기아 정도 — 심한 기아에서는 하중이 지배한다

> "Van Zoelen et al. developed a model for film thickness decay under severe starvation ... and showed that **film thickness decay eventually becomes nearly independent of speed and is primarily governed by load.**"
> — `2026. (Lugt) Effect of speed, load, grease type…CRB.md` §Introduction

> "**For a given film thickness, higher speeds lead to more severe starvation.**"
> — `2026. (Lugt) Film thickness decay…wide elliptical contacts.md` §5

→ 감쇠의 지배 인자가 **기아 정도에 따라 바뀐다.** 약한 기아에서는 속도가 영향을 주지만, 심한 기아로 가면 하중이 지배한다.

### (4) 정량 스케일 — 감쇠율은 매우 작다

같은 문헌 Table 4 의 평균 감쇠율 `ḣ_ave` 는 **0.01 ~ 0.03 nm/s** 수준이다(Li/M · Li/SS, 100 ~ 200 mm/s). 그리스 종류에 따라 **2 ~ 3배 차이**가 난다.

→ 절대값이 작다는 것은 **측정에 높은 분해능과 긴 시간이 필요**하다는 뜻이다. 장비의 1 nm 분해능이 이 때문에 중요하다.

---

## A2-3. 활용 방안

### (1) 재윤활 간격의 상한 산정 — 가장 직접적인 활용

> "In its simplest form, **all replenishment ignored, the model yields a worst case film thickness decay prediction.** The results can be interpreted as a **maximum allowable interval between replenishment events** in a bearing"
> — `2012. (Lugt) Thin layer flow and ~.md` §1

**개념 흐름** (수치 시산은 파라미터 확보 후)

```
감쇠곡선 h(t)  ─┐
                ├→  h(t) = h_허용 이 되는 시각 t*  →  재윤활 간격 상한 = t*
허용 최소 유막 ─┘        (보충 무시 = 최악 가정)

허용 최소 유막은 Λ = h/σ 기준으로 정한다 (예: 혼합윤활 하한)
```

### (2) 필요 보충률 역산 — 설계 요구조건으로 전환

> "...or be used to **determine a required rate of replenishment by e.g. bleeding, to sustain a certain film thickness level.**"
> — 같은 문헌 §1

목표 유막을 정하면 **그리스가 최소 얼마나 오일을 내놓아야 하는지**가 나온다. 이는 그리스 선정의 정량 기준(블리딩 특성)으로 쓸 수 있다.

### (3) 그리스 선정 — 감쇠율로 순위를 매긴다

A2-2 (4) 처럼 그리스별 감쇠율이 2~3배 차이 난다. 정상상태 `h_c` 만 비교하면 초기값이 큰 그리스가 유리해 보이지만, **감쇠가 빠르면 재윤활 주기가 짧아진다.** 두 지표를 함께 보아야 순위가 뒤집히지 않는다.

### (4) 설계 판정 — 형상이 감쇠에 미치는 영향 반영

A2-2 (2) 의 `k²` 관계는 **설계 변수(롤러 길이·접촉 형상)가 윤활 지속성에 직접 영향**함을 뜻한다. 사이징 최적화가 응력·피로만으로 후보를 골랐다면, 감쇠 관점의 판정을 추가할 근거가 된다.

---

## A2-4. 26 MW 메인베어링에서 특히 필요한 이유

| 조건 | 값 | 감쇠 관점의 함의 |
|---|---|---|
| **저속** | 3.50 ~ 6.19 rpm | κ 0.38 ~ 1.22 (§6-7) — 이미 혼합윤활 영역. 유막이 조금만 더 줄어도 경계윤활로 넘어간다 |
| **대형** | dm 3,473 ~ 4,591 mm | 궤도 면적이 크고 보충 경로가 길다. 블리드된 오일이 접촉부까지 도달하는 데 불리 |
| **개방형·재윤활** | 주기적 재윤활 전제 | A2-1 (3) 이 그대로 적용 — **주기 결정이 실무 과제** |
| **장수명** | 30년 설계수명 | 단기 정상상태보다 **장기 거동**이 지배 |
| **긴 접촉선** | L_we 348 ~ 534 mm | 감쇠는 느리지만(유리), 그만큼 **관측에 긴 시간**이 필요 |

**요약** — 우리 대상은 감쇠가 느린 형상이면서도 이미 윤활 여유가 얇은 조건이다. 감쇠가 느리다는 것이 곧 안전을 뜻하지 않는다. **출발점(κ ≈ 1 내외)이 낮기 때문에 작은 감쇠도 체제 전환을 일으킬 수 있다.**

---

## A2-5. 감쇠를 검토하지 않을 경우의 한계

| # | 한계 | 결과 |
|--:|---|---|
| 1 | 정상상태 `h_c` 만으로는 **시간 정보가 없다** | 재윤활 주기를 정할 근거가 없어 관행값에 의존하게 된다 |
| 2 | Van Zoelen 의 **보수 예측만 쓰면 과도하게 안전측** | 실제보다 짧은 주기를 강제해 유지보수 비용이 커진다 |
| 3 | 보충(블리딩) 모델이 **미검증 상태로 남는다** | 정확도를 알 수 없어 설계 판정에 쓸 수 없다 |
| 4 | 그리스 선정이 **초기 유막두께로만** 이뤄진다 | 감쇠가 빠른 제품을 고를 위험 (A2-3 (3)) |
| 5 | 형상의 윤활 지속성 효과를 **설계에 반영하지 못한다** | 사이징 결과가 응력·피로에만 근거하게 된다 |
---

## A2-6. 상세 문헌 분석 및 검토

감쇠를 정면으로 다룬 세 문헌을 대상으로 ① 감쇠를 일으키기 위한 시험 조건 ② 감쇠의 시간 스케일 ③ 2012 논문의 베어링 적용 모델을 정리한다.

| 약칭 | 문헌 | 성격 |
|---|---|---|
| **[V12]** | `2012. (Lugt) Thin layer flow and ~.md` — Venner, van Zoelen & Lugt | 모델 + 단일접촉 실측 + 베어링 적용 |
| **[G26]** | `2026. (Lugt) Film thickness decay…wide elliptical contacts.md` — Gao et al. | 실측 중심 (WAM · PCS EHD) |
| **[P26]** | `2026. (Lugt) Transient Analysis…Bleed Phase in CRB.md` — Puthumana et al. | **모델 중심** (베어링 대상) |

---

### A2-6.1 감쇠를 일으키기 위한 시험 조건

감쇠를 관측하려면 **오일이 다시 들어오지 못하게 막아야** 한다. 세 문헌이 그 조건을 어떻게 만들었는지 정리한다.

| 항목 | **[V12]** 단일접촉 | **[G26]** 넓은 타원접촉 | **[P26]** 베어링 |
|---|---|---|---|
| 장치 | 볼-온-디스크 (광간섭) | **WAM** 테이퍼 롤러-디스크 · **PCS EHD** 볼-디스크 | CRB 베어링 (모델 적용 조건) |
| 접촉 형상 | 원형 · 타원 | **넓은 타원** (a 39.2 µm · b 2,710 µm) · 원형(141 µm) | 롤러-궤도 선접촉 |
| 디스크 | 유리 | **코팅 유리** | — |
| 속도 `u_m` | 60 ~ 186 mm/s | **100 · 150 · 200 mm/s** (WAM) · 200 · 300 mm/s (PCS) | 모델 입력 |
| 하중 | 20 N | **40 N** (WAM) · 20 N (PCS) | 모델 입력 |
| 최대 압력 `p_h` | 0.51 GPa | **0.22 GPa** (WAM) · 0.48 GPa (PCS) | 모델 입력 |
| 온도 | 24.5 °C | 실온 | 모델 입력 |
| 그리스 | — | **Li/M** (η₀ 0.38 Pa·s · α 30 GPa⁻¹) · **Li/SS** (0.135 · 21.5) | Li 계열 (문헌값) |
| **보충 차단 방법** | 초기 도포 후 방치 | **능동 억제** — 트랙 반경을 옮겨 제방을 양쪽 2.5 mm 밖으로 밀어냄 | *(모델 가정)* 보충을 **지수함수**로 명시 모델링 |
| 부가 변수 | 속도·하중 변화 | **롤러 스큐** 0 · ±1 · ±2° | 블리드율 파라미터 |

**압력 조건이 낮은 이유** — [G26]은 그 제약을 직접 밝힌다.

> "The relatively low-pressure conditions in this study with **maximum contact pressure below 0.5 GPa are primarily due to the limitation of the experimental setup, particularly, the coated glass disc.**"

#### A2-6.1.1 보충 차단 방법 — 상세

감쇠를 관측하려면 궤도 밖으로 밀려난 오일이 **다시 트랙으로 돌아오지 못하게** 해야 한다. 문헌이 쓴 방법은 세 갈래다.

##### ① 소량·고점도 시료로 기아 상태를 조성 — [V12]

시료를 적게 넣고 점도가 높은 것을 써서 애초에 재유동(reflow)이 일어나기 어렵게 만든다.

> "To minimize reflow effects, which are not accounted for in the model, **only a small amount of oil of high viscosity was used, i.e. the contact runs under starved conditions.**"

- **목적**: 모델이 다루지 않는 재유동을 실험에서 제거해, 측정과 모델의 전제를 맞춘다
- **주의**: 시료량이 결과를 좌우하므로 **정량 충전과 기록**이 필수다

##### ② 트랙 반경을 옮겨 제방(levee)을 밀어냄 — [V12] · [G26]

초기 과임에서 궤도 양옆에 쌓인 그리스 제방이 보충원이 되므로, 디스크의 트랙 반경을 조금씩 바꿔 제방을 트랙에서 멀리 떨어뜨린 뒤 원래 반경으로 돌아와 측정한다.

**[V12] — 원형접촉에 적용**

> "for the circular contact measurements **the levees formed to the side of the track during the initial overrollings were pushed aside so as to minimize their role in replenishment. This was done by varying the track radius on the disc during the first few rotations.**"

**[G26] — 넓은 타원접촉에도 적용**

> "These levees are relatively large compared to the lubricant layer thickness in the track, **resulting in track replenishment. To minimize this replenishment,** the radius of the track on the disc R_disc ... **is gradually adjusted during rolling, pushing the levees about 2.5 mm out on each side of the track.** Afterward, the roller is repositioned to the initial track radius"

- **적용 시점**: [V12]는 "처음 몇 회전", [G26]은 "굴리는 동안 점진적으로"
- **이동량**: [G26]은 **양쪽 각 2.5 mm** 로 명시. [V12]는 수치 미제시
- **디스크 트랙 반경**: [G26] 롤러 45 mm · 볼 40 mm / [V12] 36 mm

##### ③ 모델에서 보충을 명시적으로 다룸 — [P26]

시험으로 막는 대신, 보충이 있는 상태를 **모델에 넣는다.** 블리드로 공급되는 오일량을 시간에 따른 지수 감소로 기술한다.

> "Measurements reported by Hogenberk et al. suggest that, in cylindrical roller bearings, **the remaining oil volume decays exponentially over time.**"

- 이 경우 "차단"이 아니라 **정량화**가 목적이다. 보충률 파라미터를 확보해야 쓸 수 있다

---

##### 문헌 간 판단이 갈리는 지점 — 넓은 접촉에서 제방 제거가 필요한가

**[V12] 는 필요 없다고 본다.**

> "**For the elliptic contact measurements this procedure was not needed as reflow was not an issue due to the larger track width.**"

**[G26] 은 넓은 타원접촉에도 적용했다.** 위 ② 의 인용이 그것이며, 그 이유를 "제방이 트랙 내 층 두께에 비해 상대적으로 크기 때문"으로 밝힌다.

| | [V12] (2012) | [G26] (2026) |
|---|---|---|
| 원형접촉 | 제방 제거 **수행** | 제방 제거 수행 (볼 R_disc 40 mm) |
| **넓은 타원접촉** | **불필요하다고 판단** — 트랙 폭이 넓어 재유동이 문제되지 않음 | **수행** — 제방이 층 두께 대비 크다 |

> **미해결.** 두 문헌은 같은 연구 그룹에서 나왔으나 넓은 접촉에서의 제방 제거 필요성에 대해 서로 다른 판단을 적었다. 어느 쪽이 옳은지는 문헌만으로 판정할 수 없다. **우리 시험은 롤러(넓은 타원)를 쓰므로 이 쟁점이 설계에 직접 걸린다** — 감쇠를 다루는 계획에서는 제방 제거 여부를 **시험 변수로 두어 자체 확인**하는 방안을 검토해야 한다.

> **v1 · v2 와의 관계** — 위 방법들은 모두 보충을 **막는** 방향이지만, v1 · v2 는 스쿠프로 보충을 **더하는** 반대 방향의 구성이다. 따라서 이 절의 내용은 v1 · v2 에 직접 적용되지 않고, 감쇠를 다루는 계획(초판 · 본 Plan)에 해당한다.

---

#### starvation 관련 원문 인용

**[V12] — 기아 영역의 정의와 발생 과정**

> "The second case is referred to as **the starved regime**. The rolling contact pushes aside the grease in the early overrollings creating grease levees to the side. **The film thickness initially decays with time as grease pushed to the side does not flow back onto the track for relubrication.**"

> "As the amount of oil available for lubrication is often very small **the contact is starved and operates at a film thickness level much below the fully flooded limit.** In between overrollings **very little replenishment** from the side of the track can take place"

**[G26] — 기아 정도가 모델 적용범위를 가른다**

> "the Van Zoelen model is **characteristic of heavily starved contacts**. Under this condition, the influence of speed has been shown to be minimal. According to Van Zoelen, **in case of mild starvation, side flow is more pronounced** due to a longer pressure build-up in the inlet"

> "this model ... only considers side flow in elliptical contacts without accounting for reflow when the contacts are severely starved. Hence, **while this model is not strictly applicable due to the mild level of starvation in our tests,** we have used it as a reference."

> "**For a given film thickness, higher speeds lead to more severe starvation.** The measurement results at higher speeds are therefore closer to the predictions of the Van Zoelen model, which is only applicable to severely starved conditions."

기아도는 `h_c,0 / h_c,ff` 로 정량되며, [G26] 실측에서 **0.91 (100 mm/s) · 0.65 (150 mm/s) · 0.61 (200 mm/s)** 이었다.

**[P26] — 심한 기아 가정의 한계와 확장**

> "the model is developed for **severely starved lubrication in elliptical contacts**, and in that case, the pressure distribution can be approximated as a Hertzian pressure distribution."

> "...cross flow of lubricant plays a critical role in determining the film thickness **under severely starved conditions.**"

**정리** — 세 문헌 모두 감쇠를 **기아 영역 안의 현상**으로 다룬다. 다만 기아의 **정도**가 다르다. [V12]·[P26]의 모델은 **심한 기아**를 전제하고, [G26]의 실측은 **약한 기아**여서 저자들이 모델을 참고용으로만 썼다.

---

### A2-6.2 감쇠의 시간 스케일 — 전동체 시험 vs 베어링

#### (1) 문헌별 실측·예측 시간

| 문헌 | 대상 | 조건 | 감쇠 관측 | 시간 스케일 |
|---|---|---|---|---|
| **[V12]** | 단일접촉 (디스크) | u_m 60~186 mm/s · 0.51 GPa | 유막 감쇠 곡선 기록 | **분 ~ 수십 분** |
| **[V12]** | **베어링 22317** | F_r 2.5~10 kN · 750~3000 rpm | **50 nm → 10 nm** | **1.8 ~ 4.1 시간** |
| **[G26]** | 넓은 타원접촉 (WAM) | 100~200 mm/s · 0.22 GPa | 기아도 0.61 → 0.38 | **117분** |
| **[G26]** | 원형접촉 (PCS) | 200 mm/s · 0.48 GPa | 기아도 0.18 → 0.04 | **17분** |
| **[G26]** | 평균 감쇠율 | Li/M · Li/SS | `ḣ_ave` | **0.01 ~ 0.03 nm/s** |
| **[P26]** | 베어링 (모델) | 블리드 지배 | 평탄부 후 완만 감소 | **수백 시간 규모** |

**[V12] 베어링 22317 의 감쇠시간** (초기 50 nm → 임계 10 nm)

| Case | 반경하중 [kN] | 내륜 회전 [rpm] | 엔트레인먼트 [m/s] | 최대압력 [GPa] | **감쇠시간 [h]** |
|:-:|---:|---:|---:|---:|---:|
| 1 | 10 | 750 | 2.56 | 1.23 | **1.8** |
| 2 | 10 | 1,500 | 5.11 | 1.23 | **2.7** |
| 3 | 10 | 3,000 | 10.2 | 1.23 | **4.1** |
| 4 | 5 | 3,000 | 10.2 | 1.02 | **4.0** |
| 5 | 2.5 | 3,000 | 10.2 | 0.85 | **3.8** |

#### (2) 과임(overrolling) 횟수로 환산

절대시간만 비교하면 회전수 차이가 가려진다. **같은 감쇠에 몇 번의 과임이 필요했는지**를 함께 본다.

> 산출식 (본 문서 계산) — 디스크 리그: `f = u / (2πR_track)` · 베어링 내륜 궤도점: `f = n_r(Ω − Ω_c)/60`, `Ω_c = Ω(1−γ)/2`

| 대상 | 조건 | 과임 주파수 | 감쇠 시간 | **과임 횟수** |
|---|---|---:|---:|---:|
| [G26] WAM 타원접촉 | 100 mm/s · R_track 45 mm | 0.35 Hz | 117분 | **약 2,500회** |
| [G26] WAM 타원접촉 | 200 mm/s | 0.71 Hz | 117분 | 약 5,000회 |
| [V12] 단일접촉 | 100 mm/s · R_disc 36 mm | 0.44 Hz | (분 단위) | **10³ 오더** |
| **[V12] 베어링 22317** | 750 rpm · γ 0.189 · n_r 14 | **104 Hz** | 1.8 h | **약 67만 회** |
| **[V12] 베어링 22317** | 3,000 rpm | **416 Hz** | 4.1 h | **약 614만 회** |

#### (3) 읽히는 것 — 전동체 시험과 베어링은 과임 횟수가 세 자릿수 다르다

- **절대시간은 비슷하다** — 디스크 시험 약 2시간, 베어링 약 2~4시간. 겉으로는 같은 규모로 보인다.
- **그러나 과임 횟수는 10³ 대 10⁶ 으로 세 자릿수 차이가 난다.** 디스크 리그는 저속·큰 트랙 반경이라 과임 주파수가 1 Hz 미만인 반면, 베어링은 수백 Hz다.
- **즉 디스크 시험은 "과임 1회당 손실이 큰" 조건에서 감쇠를 본다.** 접촉압력이 0.22 ~ 0.51 GPa 로 낮은데도 적은 과임으로 감쇠가 진행된다는 것은, 절대시간이 아니라 **접촉을 지나는 횟수 기준으로는 리그가 훨씬 가혹**함을 뜻한다.
- **[P26] 의 수백 시간 스케일은 성격이 다르다.** 이는 과임에 의한 손실이 아니라 **블리드로 공급되는 오일이 고갈되는 시간**이므로, 위 두 스케일과 직접 비교할 대상이 아니다.

> **시험 설계에 대한 함의** — 우리 리그(롤러-디스크, 저속)에서 얻은 감쇠 곡선을 실기 베어링 시간축으로 옮기려면 **절대시간이 아니라 과임 횟수(또는 `F(0)·t`)로 환산**해야 한다. 절대시간으로 옮기면 세 자릿수의 오차가 생긴다.

---

### A2-6.3 [V12] 의 베어링 적용 모델 — 이론 정리

#### (1) 단일 접촉에서의 유도

**측방 유량** — 접촉을 지나는 동안 압력 구배로 층이 옆으로 밀려나는 양이다.

$$
\hat{q}_{y,k}(y,t) = -\left( \int_{a^{-}}^{a^{+}} \left( \frac{\rho h^{3}}{12\eta} \frac{\partial p}{\partial y} \right) dx \right)_{k} \tag{15}
$$

**물성 모델** — 밀도는 Dowson–Higginson, 점도는 Roelands 형을 쓴다.

$$
\rho = \rho_{0}\, \frac{5.9\times10^{8} + 1.34\,p}{5.9\times10^{8} + p} \tag{16}
$$

$$
\eta(p) = \eta_{0} \exp\left( (\ln\eta_{0} + 9.67)\left( -1 + \left(1 + \frac{p}{p_{r}}\right)^{2} \right) \right) \tag{17}
$$

**다중 접촉 합산** — 궤도 위 한 점이 여러 접촉을 지나므로 유량을 더한다.

$$
\hat{q}_{y}(y,t) = \sum_{k=1}^{n_{c}} \hat{q}_{y,k}(y,t) \tag{18}
$$

**감쇠 계수 F** — 층 감쇠 속도를 지배하는 항으로 정리된다.

$$
\mathcal{F}(y) = \sum_{k=1}^{n_{c}} \mathcal{F}_{k}(y) \tag{23}
$$

$$
\mathcal{F}_{k}(y) = \frac{2\rho_{0}^{2} p_{h}^{2}}{l_{t} b^{2}} \int_{a^{-}}^{a^{+}} \left( \eta(p)^{-1} \rho(p)^{-2} p^{-1} \right) dx \tag{24}
$$

식 (24)는 근사식 (25)로 대체할 수 있으며, 여기에는 압력-점도계수 α 와 헤르츠 압력 `p_h`, 접촉 반폭 `a`·`b`, 궤도 총길이 `l_t` 가 들어간다.

**해석해** — 궤도 중앙(`y = 0`)의 층 두께와 중앙유막두께는 닫힌 형태로 주어진다.

$$
\tilde{h}_{\infty}(0,t) = \left( \tfrac{2}{3}\mathcal{F}(0)\,t + \tilde{h}_{0,\infty}^{-2} \right)^{-1/2} \tag{26}
$$

$$
h_{c}(t) = \left( \tfrac{1}{6}\overline{\rho}_{c}^{\,2}\,\mathcal{F}(0)\,t + h_{c,0}^{-2} \right)^{-1/2} \tag{27}
$$

→ **감쇠는 `t^(−1/2)` 형태**이며, 초기값 `h_c,0` 과 감쇠 계수 `F(0)` 두 개로 결정된다.

#### (2) 베어링으로의 확장

베어링에서는 **하중이 원주 방향으로 분포**하므로 접촉마다 `a`·`b`·`p_h` 가 다르다. 이를 원주 평균으로 처리한다.

$$
\mathcal{F}_{k}(y) = \frac{1}{2\pi} \int_{0}^{2\pi} \overline{\mathcal{F}}_{k}(y,\varPsi)\, d\varPsi \tag{28}
$$

여기서 `F̄_k(y,Ψ)` 는 식 (24)를 하중분포 `F = F(Ψ)` 에 대응하는 `a(F)`·`b(F)`·`p_h(F)` 로 계산한 값이다.

**임계 감쇠시간** — 층 두께가 초기값에서 임계값까지 줄어드는 시간이 닫힌 형태로 나온다.

$$
t_{cr}(\tilde{h}_{cr}, \tilde{h}_{\infty,0}) = \frac{3}{2\,\mathcal{F}(0)} \left( \frac{1}{\tilde{h}_{cr}^{2}} - \frac{1}{\tilde{h}_{\infty,0}^{2}} \right) \tag{29}
$$

**이 식이 재윤활 간격 산정의 출발점이다** — 허용 최소 유막을 `h̃_cr` 로 두면 `t_cr` 이 곧 보충 없이 버틸 수 있는 시간이 된다.

#### (3) 적용 가정 · 조건 · 범위

| 구분 | 내용 |
|---|---|
| **핵심 가정** | **보충이 전혀 없다** — 그래서 결과는 **최악(worst case) 예측**이다 |
| | 압력 배출(pressure ejection)만 고려. 원심력 항은 별도로 다룸 |
| | 헤르츠 압력 분포 근사 |
| | 초기 층이 궤도를 가로질러 **대칭**이라고 가정 (해석해 (26) 적용 조건) |
| **베어링 적용 조건** | 축하중 스러스트 베어링은 모든 접촉이 동일 하중 → 접촉 수만 맞추면 된다 |
| | 반경하중 베어링은 하중분포가 필요 → 식 (28)의 **원주 평균** |
| | **내륜·외륜 각각의 하중분포를 따로** 다뤄야 한다 (형상·하중이 다르면 `a`·`b`·`p_h` 가 달라짐) |
| **적용 범위·한계** | 궤도를 가로지르는 층 두께의 **급격한 변화는 실제로 일어나지 않는다** — 표면장력과 유입부 경계 부근의 국부 압력유동으로 완화되나 **점근 관계식에 반영되어 있지 않다** |
| | 과임 횟수가 잘 정의되지 않는 베어링에서도 쓸 수 있도록, **과임 단위로 EHL 을 반복해서 푸는 방식을 피한 것**이 이 모델의 설계 의도다 |
| **검증 상태** | 단일 원형·타원 접촉에 대해 광간섭으로 검증 — 논문은 "excellent agreement" 로 보고 |
| | 베어링 적용은 **예측 제시**이며, 베어링 실측과의 직접 대조는 이 논문 범위 밖이다 |

> **우리 계획과의 연결** — 식 (27)은 우리가 얻을 `h_c(t)` 와 **직접 대조 가능한 형태**다. `F(0)` 을 시험 조건에서 계산하고 측정 곡선을 `h_c^(−2)` 대 `t` 로 그리면 **직선**이 되어야 하며, 그 기울기가 `F(0)·ρ̄_c²/6` 와 일치하는지로 모델을 검증할 수 있다. 다만 이는 **감쇠를 다루는 계획(초판·Plan)** 에 해당하며, v1·v2 는 스쿠프로 완전유막을 유지하므로 이 검증은 범위 밖이다.
