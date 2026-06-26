# 분석 레포트 작성 계획서 — Mixed/Partial EHL 출력의 공학적 활용

> **상태:** 작성 계획(검토용). 본 계획 확정 후 본문 집필 착수.
> **연계 문서:** `분석_EHL 윤활해석.md`(본문 §1–9, 부록 1·2)
> **작성일 기준:** 2026-06-19
> **무게중심(확정):** 손상 메커니즘 균형 — 마찰·마이크로피팅·스커핑·마모·수명통합을 균등 비중으로 다룸.
> **산출물(확정):** 신규 독립 파일(예: `분석_MixedEHL_공학적활용.md`).

---

## 0. 문제 정의 (이 레포트가 답하는 질문)

1. **조도 이중 반영 문제(확인됨):** MK(M2) 유막식은 조도 보정 $r_c(\sigma)$ 가 내장되어 있어, 그 결과 유막으로 $\Lambda=h/\sigma$ 를 만들어 $\kappa\to a_{\text{ISO}}$ 에 넣으면 조도가 **두 번** 반영된다(① $r_c$ 로 $h$ 감소, ② $\Lambda$ 에서 $\sigma$ 로 또 나눔). ISO 281 $a_{\text{ISO}}$ 곡선이 **매끈 표면 $h$ 기준 $\Lambda$** 로 calibration 되었기 때문이다.
2. **따라서 수명 계산용으로는 $a_{\text{ISO}}$(이미 조도 통계 반영) 사용으로 충분**하며, Mixed EHL 미시 출력을 수명계수 재보정에 쓰면 안 된다.
3. **핵심 질문:** 그렇다면 수치해석으로 정밀하게 푼 Mixed EHL의 출력 — **유막두께 $h$, 유체압 $p_h$, 돌기압 $p_a$, 돌기 하중분담비 $W_a/W_t$** — 를 **공학적/실용적으로 어떻게 활용**해야 하는가?

---

## 1. 중심 논제 (Thesis)

> **관심사의 분리(Separation of Concerns):** $a_{\text{ISO}}$ (거시 수명, 조도 통계 내장)와 Mixed EHL 미시 출력(국부 손상 물리)은 **서로 다른 질문에 답하는 도구**다. Mixed EHL 출력의 정당한 용도는 $a_{\text{ISO}}$ 재보정이 아니라, **(i) $a_{\text{ISO}}$ 가 원리적으로 다루지 않는 표면개시 손상 모드(마이크로피팅·스커핑·마모)의 예측**과 **(ii) 마찰·발열 예측**, **(iii) surface/subsurface 피로 분리(SKF GBLM)** 이다.

근거: Lundberg–Palmgren 기반 $a_{\text{ISO}}$ 는 **아표면 개시 피로**를 다루며, 표면개시 손상(마이크로피팅 등)은 구조적으로 포함하지 않는다. 현대 베어링 수명모델(SKF GBLM)은 바로 이 한계를 **surface/subsurface 분리**로 해결하며, surface 항은 Mixed EHL 응력으로 구동된다.

---

## 2. Mixed EHL 출력 → 공학적 활용 매핑 (레포트 핵심 표)

| Mixed EHL 출력 | 유도 물리량 | 공학 모델/판정 | 활용 목적 | 핵심 문헌(잠정) |
|---|---|---|---|---|
| 유막 $h,\ h_{\min}$ | $\lambda=h/\sigma$ | 윤활 레짐, 손상 위험지표 | 설계검증·윤활선정 | Patir–Cheng; Hamrock |
| 유체압 $p_h$ | 부분 하중지지 | (돌기압 합산) 아표면 응력 | RCF 평가 | Hu–Zhu; Zhu–Wang |
| **돌기압 $p_a$** | 국부 응력집중·실접촉면적 | **Dang Van 피로 + 소성판정** | **마이크로피팅·표면피로** | Morales‑Espejel; Greenwood–Tripp |
| **하중분담비 $W_a/W_t$** | 경계마찰 기여·마찰계수 | 트랙션 → 발열 | **마찰토크·동력손실** | Wang 1996(로컬); Wingertszahn 2023(로컬) |
| 마찰전단 $\tau$ | 마찰열류 $q$ | **Blok flash temperature** | **스커핑/스미어링 한계** | Blok; 스커핑 모델 |
| 돌기 미끄럼량 | 마모깊이 | **수정 Archard(mild wear)** | **마모·런인** | Archard; Morales‑Espejel |
| 조합 응력장 $(p_h+p_a)$ | von Mises/직교전단 vs 깊이 | **surface↔subsurface 분리** | **GBLM·응력기반 수명** | SKF GBLM; Ioannides–Harris |

---

## 3. 제안 목차 (11장) — 장별 내용·매핑 문헌·예상 비중

> 손상 메커니즘 균형: 4·5·6·7·8장을 **동등 비중**으로 집필.

| 장 | 제목 | 핵심 내용 | 매핑 문헌(잠정) | 비중 |
|---|---|---|---|---|
| 1 | 서론·문제정의 | 조도 이중반영 정식화, $a_{\text{ISO}}$ 역할 경계, 논제 제시 | ISO 281, ISO/TR 1281; 본문 §7.2 | 소 |
| 2 | 이론배경: Mixed/Partial EHL | 평균 Reynolds + Patir–Cheng flow factor, Greenwood–Tripp, 결정론적(deterministic) Zhu–Wang/Hu–Zhu unified Reynolds | Patir–Cheng 1978/79; G–T 1970; Hu–Zhu 2000; Zhu–Wang | 중 |
| 3 | 출력→물리량 변환 | $\lambda$, 하중분담 $W_a/W_t$, 조합 응력장, 마찰계수, flash 열류 정의 | 로컬 Wang 1996; Liu 2023 | 중 |
| **4** | **활용① 마찰·동력손실·발열** | 유체+돌기 하중분담 기반 마찰계수, 트랙션곡선, 발열 → 벌크온도 연계; MBS 검증 | Wingertszahn 2023(로컬, ±10%); Biboulet–Houpert; Wang 1996 | **균등** |
| **5** | **활용② 마이크로피팅(표면개시 피로)** | 돌기응력 반복 → Dang Van 고주기피로 + 수정 Archard mild wear **경쟁모델**; $\lambda$·SRR·하중 영향 | Morales‑Espejel 2011/2021; ISO/TR 15144(기어 유추); Brandão | **균등** |
| **6** | **활용③ 스커핑/스미어링** | Blok flash temperature 임계, 한계전단(traction>수정 전단강도) 기준, 국부 유막붕괴 | Blok 1937; 스커핑 모델(2012/2021); 본문 부록2(점도) | **균등** |
| **7** | **활용④ 마모(mild/adhesive)** | Archard·에너지 기반 마모깊이, 형상열화 피드백, 런인 | Archard; deterministic wear(2009/2025) | **균등** |
| **8** | **활용⑤ 수명통합: surface vs subsurface** | 조합 응력장 → von Mises/직교전단 깊이분포; **SKF GBLM**(표면·아표면 생존확률 분리), Ioannides–Harris 응력기반 | Morales‑Espejel & Gabelli(GBLM); I–H 1985; Tallian | **균등** |
| 9 | ISO 281과의 비중복 원칙 | $a_{\text{ISO}}$ ↔ GBLM 관계, 이중반영 회피 가이드(어떤 출력을 어디에 쓰고 쓰지 말아야 하는지) | ISO 281; GBLM | 중 |
| 10 | 20MW+ 풍력 메인베어링 적용 | 저속·고하중·그리스 맥락에서 어떤 손상모드가 지배적인지, 부록1·2 연결, 실측·CM 연계 | Hart 2022; 본문 부록1·2; Stirling 2023 | 중 |
| 11 | 결론·실무 의사결정 플로우 | "목적별 도구 선택" 플로우차트, 권고 | — | 소 |

---

## 4. 문헌 조사 전략 (방대한 조사)

### 4.1 이미 확보(로컬, 1차)
- Wang et al. 1996 — 부분 EHL, Greenwood–Tripp 하중분담, 리브 마찰 분해
- Liu et al. 2023 — 열 EHL + Carreau, 유막·온도식
- Wingertszahn et al. 2023 — MBS 마찰 예측(실측 ±10% 검증)
- ISO 281 / ISO/TR 1281‑1·2 — 수명·$a_{\text{ISO}}$·$\kappa$
- 본문 부록1·2 — 가정·한계, 점압모델

### 4.2 추가 수집 대상(우선순위)
1. **SKF GBLM** — Morales‑Espejel & Gabelli (surface/subsurface 분리 수명) ★최우선
2. **마이크로피팅** — Morales‑Espejel 2011(Tribology Transactions), 2021(thermal); ISO/TR 15144
3. **결정론적 Mixed EHL** — Hu & Zhu 2000(unified Reynolds), Zhu & Wang 리뷰
4. **Flow factor** — Patir & Cheng 1978/1979
5. **돌기접촉 통계** — Greenwood–Williamson 1966, Greenwood–Tripp 1970–71
6. **스커핑/flash temp** — Blok 1937/1963; 최신 스커핑 기준(2012/2021)
7. **응력기반 수명** — Ioannides–Harris 1985; Tallian
8. **마모** — Archard 1953; deterministic wear(2009 gear, 2025 turbine bearing)
9. **풍력 적용** — Hart 2022 Part1·2(이미 확보)

### 4.3 방법
- **deep‑research 하네스**로 활용축(마찰/마이크로피팅/스커핑/마모/수명)별 **병렬 검색 → 출처 교차·반증검증 → 종합**.
- 각 주장에 출처 `[n]` 표기, 로컬은 `(로컬)`·원문 대조 권장 표기(본문 규약 승계).
- 1차 문헌 우선, 리뷰/계산기 사이트는 보조.

---

## 5. 산출물·작성 규약
- **파일:** `분석_MixedEHL_공학적활용.md` (신규 독립).
- **수식:** LaTeX(`$$`/`$`) — 본문 파일과 동일 스타일.
- **출처 표기:** 번호 `[n]` + 문헌목록, 로컬은 `(로컬)` 및 대조권장, 저자견해/일반경향 명시(본문 규약 승계).
- **★본문 직접 출처 명시(추가 지침):** 모든 핵심 주장·수식·수치는 **본문 문장 안에 출처 문헌을 직접 명기**한다(예: "Morales‑Espejel & Gabelli(2015)에 따르면 …", "Blok(1937)의 임계온도 기준 …"). 번호 `[n]`만 다는 데 그치지 말고 **저자·연도(가능하면 학술지)를 문장에 노출**하여, 독자가 문장만 읽어도 근거 문헌을 알 수 있게 한다. 추정·저자견해는 그 취지를 문장에 명시한다.
- **교차참조:** 본문 부록1(가정·한계)·부록2(점압)과 상호 링크.

---

## 6. 검토 포인트 (확정 요청 사항)

본문 집필 착수 전 아래를 확인 바랍니다:

1. **목차 11장 구성**이 적절한가? (가감할 장이 있는지 — 예: WEC/백색에칭균열을 별도 장으로 둘지)
2. **2장(이론배경) 깊이** — 지배방정식을 어느 수준까지 전개할지(개념 수준 vs 수식 유도 수준).
3. **10장(풍력 적용)** — 20MW+ 메인베어링 특정 수치예제를 포함할지(가용 설계값 필요).
4. **GBLM 비중** — "균등"이지만 9장(비중복 원칙)에서 GBLM을 핵심 축으로 더 끌어올릴지.
5. **분량 목표** — 개략(각 장 1–2p) vs 상세(각 장 3–5p).

> 위 5개 확정 후 deep‑research로 본문 집필을 시작하겠습니다.
