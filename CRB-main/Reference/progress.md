# 진행 상태 — Mixed EHL 공학적 활용 분석 레포트

> 갱신 규약: 각 단계 종료 시 `[완료]/[진행중]/[대기]`로 상태 갱신.
> 산출물: `분석_MixedEHL_공학적활용.md` (신규 독립), 계획서: `분석계획_MixedEHL_공학적활용.md`

## 0. 준비
- [완료] 분석 방향·논제 확정 (관심사의 분리)
- [완료] 작성 계획서 수립 + 지침(본문 직접 출처 명시) 반영

## 1. 문헌 조사 (11장 기준)
- [완료] 2~3장: Mixed EHL 이론·출력 (Patir–Cheng, GW/GT, Hu–Zhu, Zhu–Wang, λ)
- [완료] 5장: 마이크로피팅 (Morales‑Espejel & Brizmer 2011, Dang Van, ISO/TR 15144)
- [완료] 6장: 스커핑/스미어링 (Blok, Lyu 2021, Zhang–Cheng–Wang, Matveevsky, Evans)
- [완료] 7장: 마모 (Archard, 에너지기반, Zhu 2007, Akbarzadeh–Khonsari)
- [완료] 8~9장: 수명·GBLM·이중반영 (Morales‑Espejel & Gabelli 2015, Ioannides–Harris)
- [완료] 4장: 마찰·동력손실 (로컬 Wingertszahn 2023, Biboulet–Houpert, Wang 1996)
- [완료] 10장: 풍력 적용 (Hart 2022 Part1/2, Stirling 2023, 부록1·2)

## 2. 본문 집필
- [완료] 1~3장 (서론·이론배경·출력변환)
- [완료] 4~5장 (마찰 / 마이크로피팅)
- [완료] 6~7장 (스커핑 / 마모)
- [완료] 8~9장 (수명통합 GBLM / 비중복 원칙)
- [완료] 10~11장 (풍력 적용 / 결론·의사결정 플로우)
- [완료] 통합 참고문헌(47건) + [VERIFY]·(로컬) 표기

## 3. 마무리
- [완료] 본문 내 부록1·2 상호 교차참조 명시 (서두·4·6·8·10장)
- [완료] 발표 슬라이드용 압축본 작성 (`슬라이드_MixedEHL_핵심.md`, 2장: 매핑표+플로우차트)

## 4. 부록 1 — 고속 준해석 계산 (압력분포→응력→피로)
- [완료] 조사: 회귀식(Masjedi–Khonsari) — 5개 배경에이전트 완료
- [완료] 조사: 상세 압력분포 고속법 (하중분담 / 진폭감쇠 Hooke–Li·Venner–Lubrecht·Greenwood–Morales-Espejel)
- [완료] 조사: 압력→6응력 해석/DC-FFT (Smith–Liu, McEwen, Boussinesq–Cerruti, Liu–Wang DC-FFT, Polonsky–Keer, Brandt–Lubrecht)
- [완료] 조사: 응력집중 표면이동 (Hamilton–Goodman μ>0.3, Webster–Sayles, exp(-2πz/λ))
- [완료] 조사: 피로기준 (Dang Van, Crossland/Findley, CDM, L–P/I–H; Ciavarella–Monno; Cerullo 2014 풍력)
- [완료] 본문 통합: `분석_MixedEHL_공학적활용.md` 부록 1 작성 + 참고문헌 [B1~]

## 6. 신규 연구방향 — Regression(L_a) 기반 설계 활용
- [완료] 조사 A: 돌기하중비 L_a — threshold·수명연계·설계활용 사례 (Moallem 2016 등)
- [완료] 조사 B: κ–λ–L_a 관계·κ 지표 한계 (Spies 2025, Zhu–Wang 2012)
- [완료] 조사 C: 메인베어링(풍력) 설계 지표 현황·신규성 (Hart 2022, Kenworthy 2024)
- [완료] 비판적 판단 + 방향성 제안 → 본문 **부록 2** 통합 (A2.1~A2.6, 참고문헌 [C1~C8])

## 7. 확률과정/스펙트럼 접근 — 통계적 압력·6응력 계산 가능성
- [완료] 조사: 랜덤과정·압력분포 통계 (Nayak, Persson 2001 확산방정식, GW/GT)
- [완료] 조사: 표면하 응력통계·PSD 필터 (Müser 2018, Persson 2008/2023, 극값통계)
- [완료] 수리·물리 분석(3명제 판정) + 본문 **부록 3** 통합 (A3.1~A3.6, 참고문헌 [D1~D13])
- 핵심: 가설 타당(스펙트럼 접촉역학), 단 비가우시안·극값성 관문 / "통계 응력→다축피로" 모델은 미발표 공백
- [완료] 추가조사: 윤활(Mixed EHL) 통계화 — dry 한계 정정 → **부록 3 A3.7** (Christensen·Persson–Scaraggi·진폭감쇠, [D14~D23])
- 핵심2: 윤활은 h³ 비선형 → 평균/스펙트럼까지만 / 결합분포→응력통계 윤활판은 미발표 공백

## 5. 남은(선택) 작업
- [대기] `[VERIFY]` 서지 원문 대조: Patir-Cheng'79, Hamrock Λ, ISO/TR 15144 판년, Blok 1937, Tallian, GBLM Wear 422-423, Venner–Lubrecht ∇ 정의식, Westergaard/JGH 페이지
- [대기] (선택) 슬라이드 압축본에 부록1 파이프라인 1장 추가
