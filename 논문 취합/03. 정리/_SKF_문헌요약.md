# SKF 피로·신뢰성 문헌 요약 (10편)

> 폴더: `02. 분야별/06. 피로, 신뢰성/01. SKF/`
> 정리일: 2026-06-04 | 대상: SKF "Surface & Subsurface Survival" 수명모델 계열 논문 10편
> 핵심 저자: G. E. Morales-Espejel (SKF RTD / INSA-Lyon LaMCoS) 주도 시리즈

---

## 표 ① 서지정보 (출판년도 / 제목 / 저널)

| 연도 | 정식 제목 | 저널 (발행처, vol/page) |
|------|-----------|--------------------------|
| 2011 | Micropitting Modelling in Rolling–Sliding Contacts: Application to Rolling Bearings | *Tribology Transactions* (STLE/Taylor&Francis), Vol. 54(4), 625–643 |
| 2015 | A Model for Rolling Bearing Life with Surface and Subsurface Survival—Tribological Effects | *Tribology Transactions* (Taylor&Francis, OA), Vol. 58(5), 894–906 |
| 2018 | A model for gear life with surface and subsurface survival: Tribological effects | *Wear* (Elsevier), Vol. 404–405, 133–142 |
| 2018 | Prediction of micropitting damage in gear teeth contacts considering the concurrent effects of surface fatigue and mild wear | *Wear* (Elsevier), Vol. 398–399, 99–115 |
| 2020 | A model for rolling bearing life with surface and subsurface survival: Surface thermal effects | *Wear* (Elsevier), Vol. 460–461, Art. 203446 |
| 2021 | Application of a Rolling Bearing Life Model with Surface and Subsurface Survival to Hybrid Bearings in High-Load and High-Speed Applications | *Tribology Online* (JST), Vol. 16(4), 216–222 |
| 2021 | Thermal damage and fatigue estimation in heavily loaded lubricated rolling/sliding contacts with Micro-Geometry | *Proc. IMechE Part J: J. Eng. Tribology* (SAGE/IMechE), Vol. 235(8), 1680–1691 |
| 2023 | A stress-based model for general rolling contacts with surface and subsurface survival: Application to gears | *Tribology International* (Elsevier), Vol. 180, Art. 108209 |
| 2024 | Durability of Aircraft Gas Turbine Mainshaft Ball Bearing Applying Surface and Subsurface Survival | *Tribology Online* (JST), Vol. 19(6), 573–583 |
| 2025 | Predicting resistance of bearing steels to surface-initiated fatigue: Application to hybrid contact | *Tribology International* (Elsevier), Vol. 201, Art. 110225 |

---

## 표 ② 내용 요약 (연구 대상 / 목표 / 방법 / 주요 결과 / 특이점)

### ① 2011 — Micropitting Modelling in Rolling–Sliding Contacts

| 항목 | 내용 |
|------|------|
| 연구 대상 | • 구름–미끄럼 윤활접촉(베어링·기어·캠)의 마이크로피팅(표면기인 피로, ISO 15243 surface distress)<br>• 구름베어링 조건에 적용 |
| 목표 | • 표면 피로와 마일드 마모의 **경쟁 메커니즘**을 함께 다루는 공학적 모델 개발<br>• 실측 거칠기·구름미끄럼 조건 반영한 부분 EHL 영역 예측 |
| 방법 | • 부분윤활 모델 → 응력이력 → **Dang Van** 피로판정 → **Archard** 마모 → 형상갱신 반복루프<br>• FFT 기반 3D 건접촉(Stanley–Kato)+진폭감쇠, 비뉴턴 유체, **Palmgren-Miner** 누적<br>• 데스크톱 케이스당 0.5~1시간 |
| 주요 결과 | • 마이크로피팅 시험기 + 실제 NU211 베어링(66/73/98℃)으로 정성 검증<br>• 슬립 0.5~2%에서 위험 최대; 마모↑ 시 마이크로피팅↓ (경쟁관계)<br>• 횡방향 거칠기가 더 취약, 매끄러운 표면 쪽에 더 빠르게 발생 |
| 특이점 | • 표면피로+마일드마모를 단일 루프로 결합한 "공학적" 모델(고비용 전산해석 회피)<br>• 한계: 정성적 검증 수준, 마모모델 단순, 마찰계수·첨가제 입력 의존성 높음 |

### ② 2015 — Rolling Bearing Life: Surface & Subsurface (Tribological)

| 항목 | 내용 |
|------|------|
| 연구 대상 | • 구름베어링 RCF 수명, **표면 손상(distress·마모·압흔·마이크로피팅)과 표면하 스폴링을 분리**<br>• 6217, 6309 볼베어링 예제 |
| 목표 | • ISO 281/L-P/I-H이 표면손상을 전역 디레이팅으로만 처리하는 한계 극복<br>• 표면 손상을 수명식에 명시적 도입, 표면/표면하 경쟁을 분리·정량화 (이론 정립 단계) |
| 방법 | • Weibull 최약링크 + 신뢰성 곱법칙: ln(1/S)=∫Gv dVv+∫Gs dGs<br>• 독립 손상함수 2개(표면하 Gv, 표면 Gs), 표면=얇은층 bh 근사<br>• 표면응력은 혼합윤활 EHL+Dang Van+Archard 수치해를 curve-fit → 반해석 L10식 |
| 주요 결과 | • 불량(κ=0.1)~양호(κ=4) 윤활 간 수명비 차이 최대 ~3 orders<br>• 사내 6,650개 내구시험으로 표면 파라미터 역산 → 모델이 안전측(보수적) 추정<br>• 6309 예제서 ISO 281과 유사 L10 산출, 저κ서 표면기여 100% |
| 특이점 | • 단일 등가응력 페널티 대신 **표면 손상적분 Is를 수명식에 직접 분리·통합**<br>• 트라이볼로지 수치모델 지식을 수명추정에 결합하는 일반 프레임워크<br>• 한계: 이론단계(응용 설계법은 후속), 공통 Weibull 기울기 가정, m≠e시 반복해 필요 |

### ③ 2018 — Gear Life: Surface & Subsurface (Tribological)

| 항목 | 내용 |
|------|------|
| 연구 대상 | • 기어(스퍼/헬리컬) 치면 RCF 수명, 표면기인 손상 vs 표면하 헤르츠 피로<br>• NASA AISI 9310 스퍼기어 검증 |
| 목표 | • 베어링 동적부하용량 수명모델을 **기어로 이식**<br>• AGMA 조건기반 규칙 대신 통계적 L10(90%) 도입, 표면/표면하 생존 분리 |
| 방법 | • Weibull + L-P + I-H 기반, L10=(Wm/W)^p, 생존확률을 체적·면적 적분 분리<br>• 표면손상적분 Is curve-fit, 환경계수 η=ηb·ηc<br>• **표면피로지표 SR** 정의(1=표면지배), 응력기반·하중기반 2모델, 표면 동적용량 개념 |
| 주요 결과 | • 연삭(Λ1.13): L10=15.9 Mrev, SR=0.96 ↔ 측정 ~12 Mrev<br>• 초정밀(Λ6.17): L10=43.7 Mrev, SR=0.14 ↔ 측정 ~46 Mrev<br>• 윤활 악화 시 SR→1, 하중·피로한계·청정도 영향 정량화 |
| 특이점 | • 기어 최초로 **SR 지표**로 표면/표면하 손상분포 제시, 표면 동적부하용량 개념<br>• 베어링↔기어 모델 cross-fertilisation<br>• 한계: **"개념 모델"**(기어 시험 미보정), 치근 굽힘응력 미포함, 베어링 값 차용 |

### ④ 2018 — Gear Micropitting: Concurrent Fatigue & Mild Wear

| 항목 | 내용 |
|------|------|
| 연구 대상 | • 기어 치면 마이크로피팅(저Λ 혼합윤활, 돌기 수준 RCF)<br>• 표면 피로와 경미 마모의 **동시(concurrent) 작용** |
| 목표 | • 베어링용 검증 모델을 기어 천이 접촉에 적응<br>• slide-roll ratio 영향·메커니즘 규명, 치면 따라가는 국부 손상분포 예측 |
| 방법 | • 혼합 EHL(비뉴턴 Eyring) micro-EHL + Hertz 중첩, 마찰계수 차등(0.11/0.05)<br>• **Dang Van + Palmgren-Miner** 피로, **Archard** 마모를 **동시 병렬** 적용(Brandão 순차모델과 차별)<br>• MPR 삼-디스크 시험기(16MnCr5) 검증, 스퍼기어 11위치 시뮬 |
| 주요 결과 | • 미끄럼↑→마이크로피팅↑ (S=0.3서 Ap≈10%), 모델-실험 우수 일치<br>• 마모계수↑ 시 마이크로피팅↓ (경쟁관계), 손상 최대위치가 마모 시 이동<br>• 마모 존재 시 Λ↓가 오히려 마이크로피팅 감소시킬 수 있음(러닝인 시사) |
| 특이점 | • 피로·마모 **동시 처리**로 거칠기 진화를 현실적으로 포착, 국부 분포 예측<br>• "미끄럼↑→항상 마이크로피팅↑"은 오도 가능<br>• 한계: 균열개시만(전파 미고려)→치근/치선 비대칭 재현 불가, 등온, 마모계수 실험 의존 |

### ⑤ 2020 — Rolling Bearing Life: Surface Thermal Effects

| 항목 | 내용 |
|------|------|
| 연구 대상 | • 구름접촉 **마찰열에 의한 표면 열손상**(스커핑·스미어링·시저)<br>• 고속·고하중·고온 환경, ISO 281 관용범위 밖 |
| 목표 | • 표면/표면하 분리 수명모델에 마찰열 효과 통합<br>• 강재 **열전도율**이 수명에 미치는 영향 정량화 |
| 방법 | • Carslaw&Jaeger 이동열원, FFT 컨볼루션 표면온도, Bos&Moes 열분배<br>• **크리프 손상**(Kachanov–Rabotnov–Lemaitre): dD/dt=[⟨T'⟩/C(1-D)]^m, 임계온도 **Tu**(120~250℃) Macaulay 괄호<br>• 수명식=표면하 피로+표면 피로+**마찰열(크리프) 적분**, 6202 베어링 보정 |
| 주요 결과 | • 그리스1/2 내구차: 순수피로는 동일(230 Mrev), 열손상 포함시 59.4 vs 204.5 Mrev로 실험 추세 일치<br>• 시저근접: 52100강 235,900→539 Mrev, 저열전도 STS는 68.5 Mrev로 더 단축<br>• 고열전도 강이 접촉온도↓→수명↑ |
| 특이점 | • 마찰열 표면손상을 **크리프 누적**으로 정식 수명식에 통합(차별점)<br>• Tu 개념으로 피로↔열손상 전환, 강 열전도율 영향 정량화<br>• 한계: 피로·크리프 독립처리, 벌크가열 제외, 보정 2시험만 |

### ⑥ 2021 — Hybrid Bearings (High-Load/High-Speed)

| 항목 | 내용 |
|------|------|
| 연구 대상 | • 하이브리드 베어링(강 궤도+Si₃N₄ 볼) 수명, ~3GPa·ndm 2.5~3×10⁶<br>• 분말야금 신강 PM1, 표면/표면하 피로 + 시저 |
| 목표 | • 검증된 하이브리드 수명모델을 강인한 PM1에 재조정<br>• 최소수명 시험으로 적용가능 영역(하중-속도 한계) 설정 |
| 방법 | • Ioannides-Harris 기반 표면하 Gv·표면 Gs 분리, m=e, S=0.9 L10식, 표면적분 curve-fit<br>• 상수 e=10/9, c=31/3, h=2.33<br>• R-2 시험기 7209(3.2GPa, 10krpm), 52100(20개) vs PM1(22개) Weibull, 비시저 432h 무고장 시험 |
| 주요 결과 | • PM1 L10,50이 52100 대비 ~15배 향상<br>• 모델 표면파손확률 57% ↔ 실제 표면/표면하 거의 동일비율 발생 일치<br>• 6개 베어링 432h 무고장→최소수명·전이선도 검증 |
| 특이점 | • 표면/표면하 분리로 **하이브리드의 표면 지배조건 유리함** 설명(순수 표면하 모델 한계 보완)<br>• 피로모델+시저 전이선도 결합, 신강 PM1 실증 케이스<br>• 한계: PM1 파손 2건뿐→신뢰구간 넓어 보수적 보정 |

### ⑦ 2021 — Thermal Damage & Fatigue with Micro-Geometry

| 항목 | 내용 |
|------|------|
| 연구 대상 | • 고하중 EHL 롤링/슬라이딩 접촉의 표면손상<br>• **열손상(스미어링·시저) + 마이크로피팅 + 마일드 마모 3현상 경쟁**, 측정 micro-geometry 반영 |
| 목표 | • 미세점식(피로+마모) 모델을 마찰열 flash 온도까지 확장<br>• 고정 "열입력 임계값" 모델 대체, 점진/파국 열손상 설명, 전이선도 계산 |
| 방법 | • Kachanov–Rabotnov 크리프: dD/dt=(ΔT/C(1-D))^m, σ=B·T, Tu Macaulay, 폐형해(저속표면 더 큰 손상)<br>• Carslaw&Jaeger 이동열원 FFT, q=μp<br>• 미세점식(혼합EHL+Dang Van+Miner+Archard)에 열모듈 추가, ball-on-disc 스미어링 검증 |
| 주요 결과 | • 트라이보미터 4조건서 열손상 면적비가 파손여부와 일치(flash 352~390℃)<br>• 저속 볼(14%)>고속 디스크(5%) 손상 (폐형해 일치)<br>• 7218 하이브리드(12.5krpm): 미세점식22%+열손상7% "혼합"모드, 고속화 시 열손상 지배(0.41) |
| 특이점 | • 크리프 열손상을 미세점식에 결합, **3현상 국부 경쟁**을 micro-geometry 단위로 모사<br>• 발열 파손의 점진적 누적 설명, 전이선도 이론 계산<br>• 한계: 1:1 면적비교 곤란, 표면당 1지점만, 용융·접착마모·tribolayer 미모사 |

### ⑧ 2023 — Stress-Based Model for General Rolling Contacts (Gears)

| 항목 | 내용 |
|------|------|
| 연구 대상 | • 중하중 롤링/슬라이딩 일반 기계요소 RCF (베어링·기어·캠)<br>• 적용예: 스퍼기어 피팅, 표면/표면하 분리 |
| 목표 | • 베어링 응력기반·신뢰성 수명모델을 기어 등으로 일반화<br>• **시변(time-varying) 응력장** 반영, 표면적분 단순화(5→3 상수), 시변 무시 시 과소예측 입증 |
| 방법 | • Weibull 최약점 + I-H, 표면하 Iss(τxz 진폭, /z^h)·표면 Is(curve-fit tanh) 분리<br>• **SR=Is/(Is+Iss)** 마이크로피팅 위험지표<br>• 맞물림 1주기 31스텝, 각 스텝 접촉문제(건식/EHL/FEM 플러그인), AISI 9310 기어로 A·B 보정 |
| 주요 결과 | • Superfinish(Λ6.17): L10=25.56 Mrev, SR=0 ↔ 시험 ~25 Mrev<br>• Ground(Λ1.13): L10=5.16 Mrev, SR=0.854 (윤활악화→표면파손 0→85%)<br>• 시변 무시(최악일정)시 L10=7.82 vs 시변고려 25.56 → 과소예측. 정렬불량·메시수렴 분석 |
| 특이점 | • 신뢰성·응력기반 예측을 기어로 일반화, 실제 응력장(불균질·엣지·정렬불량) 직접 반영<br>• 응력이력 계산법 비종속 플러그인 구조, SR 진단<br>• 한계: 재료별 2종(전막·불량윤활) 시험 보정 필요, 치근 굽힘 제외 "개념 모델" |

### ⑨ 2024 — Aircraft Gas Turbine Mainshaft Ball Bearing

| 항목 | 내용 |
|------|------|
| 연구 대상 | • 항공 가스터빈 메인샤프트 볼베어링(~200℃ 고온)<br>• 고온 RCF, 표면/표면하 분리, **온도에 의한 피로한도 저하**, 시저 |
| 목표 | • 표면/표면하 분리 모델을 고온 항공 베어링강에 확장<br>• 고온 피로한도 저하 반영, 기존 SHABERTH(a2·a3)보다 현실적 예측, 시저 한계 규정 |
| 방법 | • Weibull, L10=표면하 체적적분+표면 면적적분, m 동일가정, **RSF** 지표<br>• 직교전단응력 피로한도, 초음파 VHCF 시험으로 **σu(T) 2차식**(52100, M50)<br>• R2 시험기 7209(90/200℃), 무고장 최소수명(99%)→전이선도, 실엔진 4173개 현장데이터 비교 |
| 주요 결과 | • 고온 내구시험: 계산 수명이 측정 L10,50 이하(안전측)<br>• M50이 52100보다 고온 피로강도 유지(200℃서 500 vs 350MPa)<br>• steel-steel(3GPa)·하이브리드(3.5GPa) 무고장→천이선도, 실엔진 L10=29,321h(현장 부합) |
| 특이점 | • 표면/표면하 모델에 **고온 피로한도 저하** 정량 통합→강종별 차별화(52100 vs M50)<br>• κ(점도비) 사용, 하이브리드·압흔 손상에 적합<br>• 한계: m 동일가정, RSF 실측 1:1 곤란(청정도 변동), 실엔진 온도(149℃) 낮아 효과 미흡 |

### ⑩ 2025 — Bearing Steel Resistance to Surface-Initiated Fatigue (Hybrid)

| 항목 | 내용 |
|------|------|
| 연구 대상 | • 하이브리드 접촉(강 vs Si₃N₄) 마이크로피팅, 베어링강 3종<br>• 100Cr6(마르텐사이트), N360(스테인리스), PM-1(분말야금) |
| 목표 | • 하이브리드서 강종별 마이크로피팅 저항성 규명<br>• 세라믹 거칠기 영향 평가, 강 물성(항복강도·가공경화·파괴인성)↔저항성 관계 정량 예측 모델 정립 |
| 방법 | • MPR 시험기(p0=2.5GPa, SRR 2%, λ=0.25~0.8, σcer 80~160nm)<br>• 혼합윤활(FFT 건접촉+rapid 윤활), Ramberg–Osgood 탄소성, truncation pressure, rainflow counting<br>• **신규 LPMC 기준**: Eshelby+Coffin-Manson+Griffith → 핵생성 닫힌형해(식12), G∝KIc |
| 주요 결과 | • 3종 모두 마이크로피팅 면적 <1%, all-steel 대비 개선; 순위 **PM-1>100Cr6>N360**<br>• 거친 세라믹(160nm)도 λ=0.8이면 핵생성 완전 억제<br>• 성능맵(1815회 시뮬): σ0.2↑·Q↑·KIc↑일수록 저항↑; 경도 단독은 ~10배 오차<br>• LPMC를 응착마모로 확장(임계길이 d=6G/2με²) |
| 특이점 | • 강 물성 입력만으로 **자유 피팅 파라미터 없이** 마이크로피팅 예측하는 최초 메커니즘 모델<br>• Φ=σcer/σsteel로 거칠기비 정량화(Φ≈1서 하이브리드 이점 소멸)<br>• 마이크로피팅+응착마모를 에너지 관점 통합<br>• 한계: 마일드마모·온도·tribolayer·수소 효과 미반영 |

---

## 종합 메모

- **계보**: 2015(베어링) → 2018(기어 이식) → 2020(열효과) → 2021(하이브리드/열-micro-geometry) → 2023(응력기반 일반화) → 2024(고온 항공) → 2025(강종 물성 기반 LPMC). 표면/표면하 생존 분리 + 표면피로지표(SR/RSF)가 일관된 축.
- **방법론 공통 도구**: Weibull 최약점 · Ioannides–Harris · Dang Van · Palmgren–Miner · Archard · FFT 기반 EHL/표면하응력 · Kachanov–Rabotnov 크리프(열손상).
- **메인베어링 시사점**: 저윤활·오염·고온 등 가혹조건에서 표면기인 고장이 지배적인 대형 베어링에 표면/표면하 분리 모델이 기존 전역 디레이팅보다 물리적으로 타당. SR/RSF로 마이크로피팅 위험과 표면처리(슈퍼피니싱·코팅) 효과를 정량 진단 가능.
