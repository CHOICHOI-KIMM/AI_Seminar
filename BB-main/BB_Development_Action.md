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

## 260820 — 좌표계 · 단위 규약 확정 (D-7 ~ D-10)

**배경**: Plan §3.4 데이터 모델을 ISO 규약(`dx` = 축방향)으로 적었는데, 시드 코드(CRB Phase 4)는 `Z` = 샤프트축이라 충돌. 코드와 ISO 원문을 대조해 정리.

**조사 결과**

| | ISO 16281 | CRB 코드 (시드 현재) | Manual/09 (TRB 유래) |
|---|---|---|---|
| 회전축 | **X** | **Z** | Z (`δz` = axial) |
| 반경축 | Y(아래), Z(지면속) | X(수평), Y(수직/중력) | X(F_r 방향), Y |
| 틸트 | ψ about Z → `M_z` | γ_x about X → `m_x` | γ_x, γ_y |
| DOF | 3 (δ_a, δ_r, ψ) | **3** (δx, δy, γx) — D6/D7 로 축소 | 5 (문서만, 코드와 불일치) |
| 틸트 팔 | **R_i** (A.4) | `d_pw/2` | `d_pw/2` |
| φ 원점 | δ_r 방향 고정 | `φ_load = atan2(F_y,F_x)` 하중정렬 | 동일 |

ISO Figure A.1 a) (PDF p.28) 렌더링으로 축 삼각대 육안 확인: `Z ⊗ → X` (오른쪽), `Y` 아래.

**확정 결정**

| ID | 결정 |
|---|---|
| D-7 | **ISO 규약 — X = 회전축.** 미지수 (δ_x, δ_y, δ_z, γ_y, γ_z), 잔차 (F_x, F_y, F_z, M_y, M_z) |
| D-8 | `φ_j = 2π(j−1)/Z` **고정 원점** + 케이지 **위상 스윕** 옵션(기본 36분할) 별도 제공 |
| D-9 | 틸트 모멘트 팔 = **`R_i`** (A.4). CRB 의 `d_pw/2` 는 볼에서 틸트 감도 과소평가 |
| D-10 | 솔버 내부 **mm · N · rad**, UI 경계에서 μm·kN·° 변환 |

**근거 메모**
- D-7: Theory 의 ISO 식을 치환 없이 전사할 수 있고, `δ_z = γ_y = 0` 구속이 ISO 3-DOF 와 항등이라 Level D-2 판정이 명확해진다. CRB 는 이미 3-DOF 로 축소돼 재사용할 5-DOF 구조 자체가 없어 이름 유지의 이득이 없다.
- D-8: 5-DOF 에서는 반경하중 방향과 모멘트 축이 독립이라 단일 정렬각으로 worst-case 를 못 잡는다. Harris Table 7.4 대조 시에는 위상 스윕의 `Q_max` 최악값을 쓴다.
- D-9: 볼의 틸트는 홈 곡률중심을 움직인다. 예) `D_w`=20 mm, `r_i`=0,52`D_w`, α₀=25° → `R_i − D_pw/2` ≈ +0,36 mm.
- D-10: `c_P` [N/mm^1.5], `Σρ` [1/mm], Harris 0,0236 / 2,79×10⁻⁴ 가 모두 mm·N 기준. **현 CRB 코드에는 환산 상수가 전 모듈에 산재**(`lubrication.rs` 155건, `bearing.rs` 19건, `hertz.rs` 14건 …) — 재작성 시 제거 대상.

**Plan 반영**
- §1 결정표에 D-7~D-10 추가
- **§3.4 신설** (좌표계·단위 규약) — 축 정의·5-DOF 일반화 식·CRB↔BB 축 대응·틸트 팔·φ 원점·위상 스윕·단위표
- 기존 데이터 모델은 §3.5 로 이동, `BearingDisplacement`·`DofMask` 갱신
- Phase 1: 단위 경계 계층 신설 작업 추가, Level A 에 `R_i`·단위 청정성(grep 0건)·축 명명 항목 추가
- Phase 3: 위상 스윕 기능 추가, Level C 에 위상 주기성, Level D-1 에 φ 기준 주의, Level D-2 에 모멘트 축 불변 추가
- `CLAUDE.md`: 좌표계·단위 경고 추가, 검증 Level 정의를 BB 기준(A~F)으로 교체 (기존은 CRB 의 Gen1↔Gen3·FEA 정의라 오도 위험)

**다음**: P1 착수 (사용자 승인 대기)

---

## 260820 — Phase 1 착수: 사전 조사 + 빌드 베이스라인

**실행 규약** (사용자 확정): 조사만 병렬·구현은 단일 / 빌드·검증 실패는 자동수정 3회 후 중단(수식·설계 판단 필요 시 즉시 중단) / 스테이지별 커밋, push 없음 / 빌드 환경 선워밍.

### 빌드 베이스라인 (삭제 전)

| 항목 | 결과 |
|---|---|
| `cargo build --lib` | green, 2분 08초 |
| `npm install` | green (`esbuild` win32-x64 정상) |
| `cargo test` | **73 passed, 0 failed** — lib 52 / geometry_level_a 7 / bearing_level_d 8 / bearing_smoke 3 / roller_level_c 3 |

### 조사 결과 — Plan 전제 오류 2건 발견

**① `mod.rs` 실태가 Plan §3.2/§3.3 과 다름.** 삭제 대상 8개 중 5개(`rib_contact`·`hmehl`·`transient`·`transient_io`·`wec_risk`)가 이미 주석 처리돼 컴파일 대상이 아니었고, **남길 예정이던 `life`·`static_rating`·`lubrication` 도 함께 비활성** 상태였다. 실제 빌드되던 솔버 모듈은 `types`·`geometry`·`hertz`·`gen1`·`gen3`·`beam`·`bearing` 7개뿐.

→ **Plan §3.3 의 "그대로 재사용" 자산 2개가 죽은 코드 안에 있다.** `hamrock_dowson_elliptical`(`lubrication.rs:2562`), `κ`·`ν₁`·`e_C`(`life.rs`) 는 컴파일된 적이 없어 재활성화 시 컴파일 여부가 미검증이다. P4·P5 는 "이관"이 아니라 "되살려서 컴파일 통과"부터 시작해야 한다.

**② Phase 순서 의존이 Plan 에 미명시.** `types.rs` 를 ACBB 로 재작성하면 `hertz.rs`·`bearing.rs` 가 즉시 컴파일 불가가 된다(둘 다 `SliceGeometry`·`SliceContactResult`·`RollerProfile` 소비). 두 모듈의 재작성은 각각 P2·P3 일정이므로, **P1 에서 두 모듈을 일시 비활성화**하는 절차가 필요하다 (CRB 가 자기 Phase 1 에서 쓴 하이브리드 stub 과 동일 수법).

→ 두 건 모두 Plan 본문에 반영함 (§3.2 활성/비활성 열, §3.3 단서, Phase 1/2/3 절차).

### 조사 결과 — 구현에 직접 쓰는 사실

**역참조**: 남길 솔버 모듈이 삭제 대상을 참조하는 곳 **0건**. 파손은 `mod.rs`·`commands.rs`·`lib.rs`·`tests/roller_level_c.rs` 4곳에서만 발생. 프론트엔드에서 `solve_roller_gen1/gen3*` 를 `invoke` 하는 코드도 **0건** → 신규 파손 없음. 단 `parse_load_csv`·`solve_transient`·`run_hmehl` 3건은 **삭제 이전부터 이미 런타임 파손** 상태(Rust 쪽 미등록).

**타원적분 부재**: `K(χ)`·`E(χ)` 의 정확한 수치 계산 코드가 저장소 전체에 **0건**. `rib_contact.rs:125` 의 Brewe-Hamrock **회귀 근사**가 유일(주석은 "complete elliptic integral" 이라 오인 소지). → P2 에서 AGM 신규 구현 확정.

**단위 실태** (D-10 근거 보강): 환산 상수 총 400건(활성 모듈 87건). 경계(`commands.rs`)에는 GPa→MPa 2건뿐이고 **kN→N 은 `bearing.rs` 안에 4중 중복**(L81/160/184/320), **GPa→MPa 는 전 모듈 15중 중복**.

같은 물리량 단위 혼용: 힘 `kN`↔`N`(같은 struct 안 공존), 각도 `arcmin`/`deg`/`rad`/`degrees` **4종**, 유막 `μm`↔`nm`(같은 struct), 강성 `[N/mm/μm]`↔`[N/μm]`, 속도 `rpm`↔`rad/s`.

**⚠ grep 에 안 걸리는 μm 스케일 매직넘버 3건** — 놓치면 Jacobian·수렴이 조용히 깨진다:

| 위치 | 값 | mm 전환 시 |
|---|---|---|
| `bearing.rs:58` | `FD_STEP_DISP = 0.01 // [μm]` | `1e-5` 여야 함 |
| `bearing.rs:218` | `.clamp(5.0, 30.0)` (step limit, μm) | `0.005~0.03` |
| `bearing.rs:284` | `.max(1e3) // 1 kN·mm` | 모멘트 잔차 정규화 기준 |

**물리식에 박힌 단위 3대 지점**: `hertz.rs` 시그니처(입력 μm / 출력 μm / 강성 `N/mm/μm`), `gen3.rs` Newton Jacobian `1e-3` 스케일(삭제됨), `bearing.rs:107` tilt 항(`1000.0 * γ * sinψ`).

**`@ts-nocheck` 13개 파일**이 프론트 타입 안전망을 무력화 중 → 단위 변경이 프론트에 **조용히** 전파된다. P6 에서 해제가 리팩터링 안전망의 선행 조건.

**되살릴 것 / 새로 만들 것**: `PreloadMode` enum(`types.rs:343`)은 TRB 잔재로 남아 있으나 입력과 연결이 끊긴 상태 → D-2 로 **재연결**. `f_a`(축하중)와 접촉각 필드는 CRB 가 D4 로 **완전 삭제** → 신규 추가 필요. `BearingEquilibrium.displacement: [f64; 5]`(`types.rs:1077`)는 이미 5칸이고 CRB 가 3개만 채우던 dead slot 구조라, **BB 5-DOF 로 가면 오히려 해소되는 방향**.

**추가 수확**: `lubrication.rs:96` 에 `nvm_central_film`(Nijenbanning-Venner-Moes 1994, 타원접촉 4-regime 통합식, `rx`·`ry` 직접 입력)이 존재 → P5 에서 Hamrock-Dowson 의 교차검증 상대로 쓸 수 있어 **T-8 우회 수단**이 된다.

---

## 260820 — P1-S1: 롤러 전용 모듈 삭제

**커밋** `62dcc4c` — 15 files changed, 69 insertions(+), 7,927 deletions(-)

### 삭제

| 대상 | 행수 | 사유 |
|---|---|---|
| `solver/{gen1,gen3,beam}.rs` | 2,001 | 슬라이스·빔 커플링 — 볼은 단일 점접촉이라 개념 소멸 |
| `solver/rib_contact.rs` | 896 | 볼베어링에 rib 없음 |
| `solver/{hmehl,transient,transient_io,wec_risk}.rs` | 4,481 | 초기 범위 밖 (Plan §2.2) |
| `tests/roller_level_c.rs` | 150 | Gen1↔Gen3 교차검증 — 볼에 대응 개념 없음 |

### 이관 (삭제 전 보존)

`hertz_elliptical_coefficients` → `hertz.rs`. 의존성 0의 순수 함수라 import 추가 불필요. 주석에 **"반환되는 F_e, E_e 는 회귀 근사이지 정확값이 아니다"** 를 명시 — 원본 주석이 "complete elliptic integral" 이라 오인 소지가 있었다. 용도는 P2 의 χ 초기추정값·검산.

### 참조 정리

`solver/mod.rs`(선언 3줄 삭제 + 사유 주석 BB 기준 갱신) / `commands.rs`(use 2줄 + Gen1·Gen3 command 4개 + `Gen1Result`·`Gen3Result` struct) / `lib.rs`(command 등록 4건) / `Cargo.toml`(고아 의존성 `sprs`·`rustfft`·`csv`·`rand`·`rand_distr` 제거, `nalgebra` 는 P3 5×5 Newton 선형해 용도로 존치)

### 검증 — 숫자 소명

```
cargo build --lib   green (58초)
cargo test          73 → 42 passed, 0 failed
```

감소 **31건 전량 소명**:

| 출처 | 개수 |
|---|---|
| `gen1.rs` 내부 유닛 테스트 | 11 |
| `gen3.rs` 내부 유닛 테스트 | 13 |
| `beam.rs` 내부 유닛 테스트 | 4 |
| `tests/roller_level_c.rs` | 3 |
| **합계** | **31** = 73 − 42 ✓ |

나머지 5개 삭제 파일(`rib_contact` 21 / `hmehl` 40 / `transient` 9 / `transient_io` 4 / `wec_risk` 4 = 78개 테스트)은 이미 `mod.rs` 비활성이라 애초에 컴파일·실행 대상이 아니었음. **깨진 것 0건.**

> **기록 규약**: 이후 모든 스테이지도 「증감 전량 소명」 형식을 유지한다. 테스트 수가 줄었을 때 *깨진 것* 과 *지운 것* 을 구분할 수 없으면 검증이 성립하지 않는다.

### 다음

**P1-S2 (types.rs 재작성) 진입 전 사용자 확인 대기.** 위 전제 오류 ② 때문에 `hertz.rs`·`bearing.rs` 를 P1 에서 일시 비활성화해야 하며, 그 결과 P1 종료 시점에 컴파일되는 솔버는 `types`+`geometry` 뿐이고 Tauri command 는 preset 7종만 남는다(해석 기능 일시 0).

---

## 260820 — P1-S2: types.rs ACBB 재작성

**커밋** `0960393` — 8 files changed, 596 insertions(+), 2,610 deletions(-). `types.rs` 1,940행 → 587행.

### 사용자 결정 (착수 전 확인)

| 항목 | 결정 |
|---|---|
| 재작성 방식 | **백지에서 신규 작성** (기존 파일 편집 아님) |
| 파일 구성 | **단일 `types.rs` 유지** (input/result 분할 안 함) |
| 결과 타입 범위 | **P1~P3 에 필요한 것만** — 수명(P4)·윤활(P5) 타입은 미정의 |
| 프리셋 호환 | **폐기 + 기본 프리셋 신규** (마이그레이션 미제공) |

### 새 데이터 모델

| 타입 | 내용 |
|---|---|
| `BallBearingGeometry` | D_w, D_pw, Z, r_i, r_e, α_nom, `ClearanceSpec` + `validate()` + Annex B.2 참조 홈반경 헬퍼 |
| `ClearanceSpec` | `Diametral`(직경 기준 G_r op) / `InitialAngle`(α₀) / `AxialPreload`(F_a0) |
| `Material` | **탄성계수 MPa 통일**. 기본값 ISO 16281 Clause 4 NOTE 1/6 (207 000 MPa, ν = 0,3) |
| `OperatingConditions` | F_x(축)·F_y·F_z·M_y·M_z, ISO 좌표계, [N]·[N·mm] |
| `DofMask` | `FULL`(5-DOF) / `ISO_3DOF`(δ_z = γ_y = 0) 상수 |
| `PhaseSweep` | D-8 케이지 위상 스윕 (기본 36분할, 기본 비활성) |
| `GeometryDerived` | A·α₀·R_i·γ·Σρ_i·Σρ_e·F_i(ρ)·F_e(ρ)·G_r op 캐시 |
| `BallResult` | φ_j·δ_j·α_j·Q_j·loaded + 내/외륜 a·b·p_max (P2 에서 채움) |
| `BearingEquilibrium` | `displacement: [f64; 5]` = [δ_x, δ_y, δ_z, γ_y, γ_z] |
| `PhaseSweepResult`, `GeometrySummary`, `BearingResult`, `Alert` | 결과 계층 |

**CRB 가 삭제했던 것 2건 복원**
- 축방향 예압 (`ClearanceSpec::AxialPreload`) — CRB 는 D4 로 `preload_mode`·`delta_preload_um` 를 제거했었음
- 축하중 `f_x` 와 2축 모멘트 `m_y`·`m_z` — CRB 는 D4(f_a 제거)·D6(m_y 제거) 상태였음

**암묵 계약 제거**: `Material` 의 탄성계수를 GPa → **MPa** 로 변경. CRB 는 GPa 로 보관하고 소비처 15곳이 각자 `* 1000.0` 을 곱하는 구조였다 (D-10 위반 1호).

**단위 (D-10)**: 파일 전체가 mm·N·rad·MPa. 환산 상수 **0건**.

### 모듈 일시 비활성 (Plan Phase 1 작업 5)

`geometry`(P1-S3) / `hertz`(P2) / `bearing`(P3) 을 `mod.rs` 에서 주석 처리. 세 모듈 모두 CRB 데이터 모델(`SliceGeometry`·`SliceContactResult`·`RollerProfile`)을 소비하므로 `types.rs` 교체와 동시에 컴파일 불가가 된다.

연쇄 조치: `commands.rs` 의 `compute_slice_geometry`·`compute_hertz_single_slice`·`solve_bearing`·`solve_bearing_dual` 4개 command 제거, `lib.rs` 등록 해제. `TauriReporter` 는 P1-S3 이후 재사용을 위해 존치.

> **현재 상태**: 컴파일되는 솔버 모듈은 `types` 하나, 등록된 Tauri command 는 preset 7종뿐. 앱은 빌드되나 **해석 기능은 일시적으로 0**. P1-S3 → P2 → P3 순으로 하나씩 재활성화된다.
>
> `geometry.rs`·`hertz.rs` 파일 자체는 **디스크에 보존** (모듈 선언만 주석 처리) — 재작성 시 원본 대조 가능.

### 프리셋

`default_bearing_input()` 을 ACBB 로 교체, 기본 파일명 `NU 240 (CRB Default).json` → `7210 (ACBB Default).json`.

경계치수 d/D/B = 50/90/20 은 ISO 15 치수계열 기준이나 **Z = 16, D_w = 11,5 mm 는 가정값**이며 실 카탈로그 미확인임을 코드 주석과 테스트 픽스처 양쪽에 명시했다 (T-6 과 연결). 구 CRB 프리셋은 스키마가 달라 `load_preset` 에서 역직렬화 오류로 노출되며, 사용자 결정에 따라 마이그레이션은 제공하지 않는다.

### 검증 — 숫자 소명

```
cargo build --lib   green
cargo test          42 → 12 passed, 0 failed
```

| 항목 | 증감 |
|---|---|
| `geometry.rs` 유닛 (모듈 비활성으로 미실행) | −11 |
| `hertz.rs` 유닛 (모듈 비활성으로 미실행) | −13 |
| `tests/geometry_level_a.rs` 삭제 | −3 |
| `tests/bearing_level_d.rs` 삭제 | −7 |
| `tests/bearing_smoke.rs` 삭제 | −8 |
| 신규 `types.rs` 유닛 | **+12** |
| **합계** | 42 − 42 + 12 = **12** ✓ |

통합테스트 3개는 CRB 슬라이스 기하·3-DOF 평형 검증이라 ACBB 에 대응 개념이 없어 삭제. BB 의 Level A 는 P1-S3, Level C·D 는 P3 에서 신규 작성한다.

신규 12개 테스트: Annex B.2 참조 홈반경 / 픽스처 유효성 / 기하 검증 4종(D_pw ≤ D_w, 홈반경 < 볼반경, Z < 3, A > 0) / 재질 기본값·포아송비 / `ISO_3DOF` 마스크 자유도 수 / 반경하중 합성·방향각·상대속도 / 솔버 파라미터 검증 3종 / serde 왕복.

### ⚠ 오케스트레이터 판단 사항 (사용자 확인 필요)

**단위 경계를 별도 타입으로 분리하지 않았다.** D-10 구현 방식이 두 갈래였다:

1. `BearingInputWire`(μm·kN·°) + `BearingInput`(mm·N·rad) 2벌 + 변환 `impl`
2. **단일 구조체를 내부 단위로 두고 UI 가 표시할 때만 환산** ← 채택

채택 근거: D-10 의 실질 목표(솔버 안에 `1000.0` 부재)를 구조체 중복 없이 달성하고, 프론트엔드는 P6 에서 어차피 재작성이라 JSON 계약 변경 비용이 0이다. 프론트 `FieldGroup` 이 이미 `unit` prop 으로 표시 단위를 받는 구조라 표시 환산은 거기서 처리된다.

**대가**: JSON 에 저장되는 값이 mm·N 이 된다 (기존은 μm·kN). 프리셋을 폐기하기로 했으므로 현재 손해는 없으나, 이후 "JSON 은 kN 으로" 로 방침이 바뀌면 되돌리는 비용이 발생한다. **번복 시 가장 싼 시점은 지금이다.**

### 다음

**P1-S3**: `geometry.rs` 재작성 (A·α₀·R_i·γ·Σρ·F(ρ) — Theory §2) + Level A 검증 + 모듈 재활성화.

---

## 260820 — Plan 수정: Phase 3 을 P3-1 / P3-2 로 분할

**배경**: 사용자 지적 — D-1 결정은 「코드는 5-DOF, 검증은 3-DOF 먼저」였는데 Plan Phase 3 은 제목·DoD 가 5-DOF 일괄이라 결정이 문서에 드러나지 않았다.

**사용자 결정**

| 항목 | 결정 |
|---|---|
| 분할 성격 | **구현 1회 + 검증 2단계** — `bearing.rs` 는 5-DOF 로 한 번만 작성, `DofMask` 로 구속/해방 |
| 부수 기능 배치 | **검증에 필요한 것만 P3-1** — 클리핑(T-3, 수렴 전제) · 예압(D-2, Level C 항목) · 위상 스윕(D-8, Level D-1 대조 기준). 고속 경고(D-3)는 P3-2 |
| 게이트 | **P3-1 종료 시 보고·중단** |
| 번호 체계 | **P3 만 분할** (다른 Phase 는 현 체계 유지) |

**Plan 반영**
- Phase 3 제목을 「평형 솔버」로 바꾸고 P3-1 / P3-2 소절 신설
- P3-1: 3-DOF 구속 검증 (Level C + Level D-1). 작업 0~7 재배치, 부수 기능 3건에 *P3-1 필수* 사유 명기
- P3-2: 5-DOF 해방 검증 (Level D-2). 고속 경고·수렴 강건성 튜닝 이동, Level D-2 에 고속 경고 경계 동작 항목 추가
- **🚦 무인 중단 게이트**를 P3-1 DoD 뒤에 명시: Level D-1 의 14개 점 오차를 보고한 뒤 P3-2 진행 여부 확인. 오차 5 % 초과 시 원인(수식 해석 vs 구현)을 자의적으로 판단하지 않는다
- §1 D-1 행: 「코드는 5-DOF 로 1회 구현, 검증만 2단계」로 문구 교체 + `bearing.rs` 를 두 번 쓰지 않음을 명시
- §5 검증 매트릭스: Level C·D-1 → P3-1, D-2 → P3-2
- §6 리스크 / §7 T-list: T-3 → P3-1(수렴의 전제), T-7 → P3-2

**코드 영향 없음** — 문서만 수정. P1-S2 에서 이미 `DofMask::ISO_3DOF` / `FULL` 상수를 `types.rs` 에 정의해 둔 상태라, 분할된 계획이 자료구조와 그대로 대응한다.

---

## 260820 — P1-S3: geometry.rs 재작성 + util.rs 신설 + Level A ✅ **Phase 1 완료**

**커밋** `f52bfeb` — 8 files changed, 1,039 insertions(+), 601 deletions(-)

### 사용자 결정 (착수 전 확인)

| 항목 | 결정 |
|---|---|
| 단위 경계 (P1-S2 미확정분) | **현행 유지 + 필드명에 단위 접미사** — wire 타입 분리 안 함 |
| S3 범위 | **GeometryDerived + GeometrySummary** |
| 재사용 3건 배치 | **신규 `util.rs` 신설** |
| Level A 배치 | **`tests/geometry_level_a.rs` 통합테스트** |

### types.rs — 단위 접미사 일괄 적용

`d_w → d_w_mm`, `f_x → f_x_n`, `m_z → m_z_nmm`, `alpha_nom → alpha_nom_rad`,
`e_ball → e_ball_mpa`, `sum_rho_i → sum_rho_i_per_mm`, `q → q_n` 등 **33개 필드**.
enum variant 도 `DiametralMm` / `InitialAngleRad` / `AxialPreloadN`.
무차원(`nu`·`gamma`·`z`·`f_rho_i`)은 접미사 없음 — 규약을 파일 헤더에 명시.

### util.rs 신설

접촉 형상·베어링 종류와 무관한 순수 함수만 둔다.

| 함수 | 비고 |
|---|---|
| `combined_elastic_modulus_mpa` | CRB 는 GPa 반환 + 소비처 15중 `* 1000.0` 이던 것을 MPa 로 통일 |
| `harris_e_prime_mpa` | `E′ = 2E*` (Hamrock-Dowson·카탈로그 규약) — P5 대비 |
| `combine_curvature_mm` | CRB `geometry.rs` 에서 이관 |
| `sphere_mass_g` | mm³→cm³ 환산을 담는 **유일한** 장소 (D-10 예외 지점) |
| `cubic_spline_interpolate` | CRB `geometry.rs` 에서 이관 |

### geometry.rs — 백지 재작성 (Theory §2)

`compute_geometry_derived` (A.3 A / A.1 α₀ / A.4 R_i / γ / E.4~E.7 Σρ·F(ρ)) ·
`compute_geometry_summary` (오스큘레이션·볼 질량·n·D_pw) ·
`collect_geometry_alerts` (`HIGH_SPEED` D-3, `GROOVE_RADIUS_OVER_REFERENCE` T-9)

### ⚠ 발견 — 식 (A.1) 의 정의역

**`α₀ = arccos(1 − G_r op/(2A))` 는 `G_r op ≥ 0` 에서만 정의된다.** 음수를 넣으면 arccos 인수가 1 을 넘는다.

즉 **ACBB 예압을 "음의 클리어런스"로 표현할 수 없다.** CRB/TRB 감각으로는 자연스러운 표현이지만 ISO 정식화가 허용하지 않는다. 조용히 0 으로 처리하면 예압이 사라진 채 계산되므로, **명시적으로 거부하고 `AxialPreloadN` 사용을 안내**하도록 했다. Plan §3.5 의 `ClearanceSpec` 설계는 유지된다 (`AxialPreloadN` 이 정공법이었음).

### Level A 검증 (16개)

| ID | 항목 |
|---|---|
| A-1 | `A = 0,05 D_w` (Annex B.2 참조기하 항등) |
| A-2 | α₀ ↔ G_r op 왕복(6점) · 단조성 · G=0 → α₀=0 |
| A-3 | `R_i > D_pw/2` (D-9, CRB 의 `d_pw/2` 과소평가 확인) · α₀ 증가 시 단조 감소 |
| **A-4** | **Σρ·F(ρ) 를 개별 주곡률로 독립 조립해 대조** · 부호 · [0,1) · 차원(스케일 불변) |
| A-5 | Annex B.2 참조 홈반경 · 오스큘레이션 |
| A-6 | 정의역 가드 4종 (예압 / 음수 / 과대 클리어런스 / 홈반경) |
| A-7 | 고속 경고 경계(14 000 vs 15 000 rpm) · 홈반경 초과 경고 |
| **A-8** | **D-10 규약 기계 검증** — 환산 연산 부재 + `pub f64` 필드 단위 접미사 강제 |

**A-4 가 동어반복을 피한 방식**: ISO 축약형 (E.4)~(E.7) 을 베끼지 않고, 개별 주곡률에서 조립한다.

```
볼:   ρ_1x = ρ_1y = 2/D_w
내륜: ρ_2x = +2γ/(D_w(1−γ)),  ρ_2y = −1/r_i
외륜: ρ_2x = −2γ/(D_w(1+γ)),  ρ_2y = −1/r_e
Σρ = ρ_1x + ρ_1y + ρ_2x + ρ_2y,   F(ρ) = (ρ_2x − ρ_2y)/Σρ
```

Harris Ch.6 조립 경로와 ISO 축약형이 rel. err < 1e-12 로 일치.

**A-8 은 규약 자체를 기계 강제**한다. `include_str!` 로 솔버 소스를 읽어 ① 환산 연산(`* 1000.0`·`/ 1e-3` 등)이 없는지 ② `pub … : f64` 필드가 전부 단위 접미사를 갖는지 검사한다. `util.rs` 만 예외.

> **자체검증 테스트가 처음에 실제로 실패했다.** 둘 다 판정 heuristic 이 거칠어 난 오탐이었다 —
> `n_inner_rpm: 1000.0`(회전속도 값)을 환산 상수로, `free_x: bool` 을 유차원 필드로 잡았다.
> **연산자 인접**(`* 1000.0`)만 보고 **`: f64`** 만 검사하도록 좁혀 해결. 검증 코드도 검증이 필요하다는 사례.

### commands.rs / lib.rs

`compute_geometry` command 신설·등록 — **P1-S2 이후 처음으로 해석 command 부활**.

### 계획 변경 — Phase 1 작업 6 취소

Plan Phase 1 의 「단위 경계 계층 신설(`commands` 에 변환을 모음)」은 **불필요해져 취소**했다. 단일 구조체 + 필드명 접미사 방식을 택했으므로 JSON 계약 자체가 mm·N·rad 이고, Rust 쪽에 변환 계층이 존재하지 않는다. Plan 본문에 취소선과 사유를 남기고 DoD 문구도 「환산 상수가 `util.rs` 밖에 없음(A-8 로 기계 검증)」으로 교체했다.

→ **P1-S2 의 「오케스트레이터 판단 사항」은 이 결정으로 해소됨.** 사용자가 필드명 접미사 방식을 택하면서 단일 구조체 노선이 확정되었다.

### 검증 — 숫자 소명

```
cargo build --lib   green
cargo clippy        warning 0
cargo test          12 → 43 passed, 0 failed
```

| 항목 | 증감 |
|---|---|
| `types.rs` 유닛 (기존) | 12 |
| `util.rs` 유닛 (신규) | **+9** |
| `geometry.rs` 유닛 (신규) | **+6** |
| `tests/geometry_level_a.rs` (신규) | **+16** |
| **합계** | 12 + 31 = **43** ✓ |

clippy 경고 6건은 전부 테스트 코드였고, 2건(`assertions_on_constants`)은 상수 마스크 회귀 가드라 사유 주석과 함께 `#[allow]` 처리했다.

### Phase 1 DoD 판정

| 항목 | 결과 |
|---|---|
| Level A 전 항목 통과 | ✅ 16/16 |
| 삭제 모듈이 어디에서도 참조되지 않음 | ✅ 빌드 green |
| 환산 상수가 `util.rs` 밖에 없음 | ✅ A-8 기계 검증 |

**Phase 1 완료.** 현재 컴파일되는 솔버: `types` · `util` · `geometry`. Tauri command: `compute_geometry` + preset 7종.

### 다음

**Phase 2** — 점접촉 타원 Hertz (Theory §3, §6). `hertz.rs` 재작성·재활성화, 완전타원적분 `K(χ)`·`E(χ)` AGM 신규 구현, `χ` 비선형식 (E.1) 솔버, `c_P` (40), 접촉타원 `a`·`b`·`p_H,max` (Harris 6.38~6.46).
Level B 는 **Harris Table 6.1 대조 + ISO 식(36) ↔ Harris 식(6.42) 독립 교차**가 핵심.

---
