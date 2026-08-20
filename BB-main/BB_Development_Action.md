# BB Development — Action Log

> [BB_Development_Plan.md](BB_Development_Plan.md) 의 실행 기록. 완료 시마다 **append only**.
> 수식 근거는 [BB_Development_Theory.md](BB_Development_Theory.md) 참조.

---

## 기록 규약

- 날짜는 `YYMMDD` (커밋 접두어와 동일)
- Phase 완료 시 **DoD 체크리스트 결과**와 **검증 Level 통과 여부**를 반드시 남긴다
- 계획과 달라진 결정은 **사유와 함께** 기록한다 (Plan 본문도 함께 갱신)
- 미해결 항목(T-list) 상태 변화는 여기와 Theory §11 양쪽에 반영한다

---

## 260820 — 프로젝트 개설

| 항목 | 내용 |
|---|---|
| 워크트리 | `d:/AI/AI_Seminar_BB`, 브랜치 `BB` (main 에서 분기) |
| sparse-checkout | cone 모드, `BB-main` 만 |
| 시드 | CRB-main Phase 4 스냅샷 전체 복제 (Tauri + React + Rust) |
| 커밋 | `dcb2a1a` |

**결정 사항**
- 분기점: main (CRB 히스토리 미상속)
- Reference: ISO 표준만 선별 유지 (126MB → 23MB)
- 초기 시드: 전체 복제, 코드 BB 화는 미착수

---

## 260820 — Theory 작성 + 리네이밍

**커밋** `99cffa0`

**BB_Development_Theory.md 신규** — ISO 281:2007 + ISO 16281:2025 정독, 수식 전량 전개 + 출처 절·페이지.

주요 발견:
1. **ISO 16281 Annex A.2 의 볼 모델은 3-DOF** (δ_r, δ_a, ψ) — CRB 의 5-DOF 가 아님
2. **ISO 16281:2025 에 수치 예제 없음** (전문 검색 0건) → 외부 검증은 Harris 뿐
3. **MinerU OCR 이 ISO 수식 지수를 손상** — pypdf 원문 텍스트 대조로 5건 정정

리네이밍 (문서/메타만, 코드 무변경):
- `CRB_Development_{Plan,Action}.md` → `_crb_archive/`
- `CLAUDE.md`·`README.md` 재작성, `Master_plan.md`·`History.md` 에 CRB 유래 경고 배너
- `package.json` → `bb-app`, `Cargo.toml`/`tauri.conf.json` → `bb-contact-analysis`
- `concept_image_gen` 삭제 (−30MB)
- `Manual/` 20개는 미변경 (코드 BB 화 시 갱신 예정)

문헌:
- Harris & Kotzalas 5th ed. 확보 (사용자)
- CRB-main 에서 볼 관련 3편 이전

---

## 260820 — poppler 도입 + 원문 육안 검증

**커밋** `7c9f2aa`

**poppler 25.07.0 설치** (`winget install --id oschwartz10612.Poppler --scope user`) → PDF 조판 이미지 직접 확인 경로 확보.

**T-1 해소** — ISO 16281 PDF p.13 육안 확인
- 식 (1)~(4) 대괄호 지수는 **모두 `0,41`**
- 내륜/외륜 구분은 **중괄호 전체 지수의 부호(±10/3)** 로만
- 외곽 지수는 네 식 모두 `3/10`, 스러스트 (3)(4) 에는 `1,044` 계수 없음
- → Theory §7.4 표기가 원문과 일치함을 확인

**T-5 해소** — Harris p.170 육안 확인
- 식 (7.70) 은 실제로 `Q_max = F_r/(J_r·Z·sin α)` 로 인쇄됨
- 식 (7.66) `F_r = Z Q_max J_r cos α` 와 모순 → **원서 오식 확정**
- 구현은 `cos α` 사용

**Harris 표 보완** (텍스트 추출 누락분을 이미지로 복원)
- Table 7.4 에 `ε = ∞` 행 추가
- Table 7.5 에 `ε₁ = 0,9` / `1,0` 두 행 추가
- `ε₁ = 1,0` 행이 단열 Table 7.4 의 `ε = 1` 행과 정확히 일치 → **두 표의 상호 일관성 확인**

---

## 260820 — 설계 결정 확정 + Plan/Action 작성

**결정 사항** (사용자 확정)

| ID | 결정 |
|---|---|
| D-1 | 코드는 5-DOF. P3 에서 3-DOF 구속 검증 → 5-DOF 해방 검증 순 |
| D-2 | 단열 ACBB + 축방향 예압 |
| D-3 | 고속은 경고만 (`n·D_pw > 1e6`) |
| D-4 | 접촉응력 구현 (Harris Ch.6) |
| D-5 | 하이브리드 볼 제외 |
| D-6 | `κ` + 점접촉 유막 (Hamrock-Dowson elliptical) |
| P-1 | 솔버 수직 관통 P1→P6 |
| P-2 | 프론트엔드는 마지막(P6) |

**T-4 해소** — D-4 결정에 따라 Harris Ch.6 (6.38)~(6.46) + Table 6.1 을 Theory §6 에 전개. 원서 p.128 육안 확인 완료.

**Theory §6 확장** — 접촉타원 반경 `a`, `b`, 무차원 계수 `a*`, `b*`, `δ*`, Brewe-Hamrock 근사, Table 6.1 (24행).
→ **Harris (6.42) `δ` 와 ISO (36) `δ_i` 가 같은 물리량**임을 확인. 서로 다른 두 표준의 독립 경로가 되므로 Level B 의 핵심 검증으로 채택.

**CRB 재사용 자산 조사 완료** (코드 직접 확인)

| 자산 | CRB 위치 | 판정 |
|---|---|---|
| `hertz_elliptical_coefficients` | `rib_contact.rs:125` | **그대로 재사용** — 계수가 Harris (6.33)~(6.35) 와 정확히 일치 |
| `hamrock_dowson_elliptical` | `lubrication.rs:2562` | **그대로 재사용** — H&D(1981) 타원 접촉 원식 |
| `κ`, `ν₁`, `e_C`, 점도 모델 | `life.rs`, `lubrication.rs` | 재사용 (접촉 형상 무관) |
| `a_ISO` 롤러 계수 | `life.rs` | **재사용 금지** — 볼은 (31)~(33) |
| `C_u` 롤러 계수 | `life.rs` | **재사용 금지** — 볼은 0.2288 / `C_0/22` |

**신규 미해결 항목**
- **T-8**: Hamrock-Dowson 타원 계수의 원전 미확인 (CRB 코드 주석 인용에 의존) → P5 착수 전 해소
- **T-9**: ISO/TR 8646:1985 미확보 → 홈 반경이 기준(0.52/0.53 `D_w`)을 초과할 때 `f_c` 감소 보정 불가

**산출물**: `BB_Development_Plan.md`, `BB_Development_Action.md` 신규

**다음**: P1 착수 (사용자 승인 대기)

---
