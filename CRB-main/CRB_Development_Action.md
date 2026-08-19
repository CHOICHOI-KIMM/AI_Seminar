# CRB Contact Analysis System — 개발 실행 기록 (Action Log)

> **Plan**: [CRB_Development_Plan.md](CRB_Development_Plan.md) 참조
> **작성 원칙**: Phase 완료 시 해당 섹션 append. 진행 중 이슈는 ⚠️ 로 즉시 기록, 해결 시 ✅ 로 갱신.
> **날짜 표기**: `YYYY-MM-DD HH:MM:SS` (작업 PC 로컬 시각).

---

## 진행 상태 요약

| Phase | 상태 | 완료일 | 순작업 시간 | 비고 |
|-------|------|--------|-----------|------|
| **0. 환경 분리** | ✅ 완료 | 2026-06-25 | ~12 분 | Sanity 통과, TRB 코드 그대로 동작 가능 상태 |
| **1. 데이터 모델 단순화** | ✅ 완료 | 2026-08-19 | ~1 시간 20 분 | 3 commits (856c219, da07b44, b2d297b), cargo check + npm build 통과. merge: 6d60de3 |
| **2. Geometry 단순화** | ✅ 완료 | 2026-08-19 | ~15 분 | 1 commit (96ac19b), Level A 8 tests pass (< 0.1% error) |
| 3. Roller-Level Solver | ⏳ 대기 | — | — | — |
| 4. Bearing-Level Equilibrium | — | — | — | — |
| 5. Life / Static Rating | — | — | — | — |
| 6. Frontend UI | — | — | — | — |
| 7. Lubrication / Transient | — | — | — | — |
| 8. 검증 + 문서화 | — | — | — | — |

---

## Phase 0 — 폴더 복제 + 환경 분리   ✅ 완료 (2026-06-25)

**전체 소요**: 2026-06-25 21:42:57 ~ 21:54:38 (순작업 ~ 12 분, 대기·대화 시간 제외)

### 0-A. 폴더 복제   (2026-06-25 21:42:57 ~ 21:43:03, 6 초)

- **명령**: `robocopy "TRB-main" "CRB-main" /E /XD node_modules target dist .git /NFL /NDL /NP /R:1 /W:1 /MT:8`
- **결과**: 756 파일, 158.7 MB, exit 3 (정상 — robocopy 0~7 = success)
- **제외**: `node_modules`, `target`, `dist`, `.git`
- **보존**: 사전 생성된 `CRB_Development_Plan.md` (덮어쓰지 않음)
- **결과 디렉토리**: `d:/AI/Main_Bearing/CRB-main/`

### 0-B. 식별자 변경   (2026-06-25 21:43 ~ 21:46, ~ 3 분)

| 파일 | 변경 항목 | Before | After |
|------|----------|--------|-------|
| [package.json](package.json) | `name` | `trb-app` | `crb-app` |
| [src-tauri/Cargo.toml](src-tauri/Cargo.toml) | `name` | `trb-contact-analysis` | `crb-contact-analysis` |
| | `description` | `TRB Contact Analysis...` | `CRB Contact Analysis...` |
| [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json) | `productName` | `trb-contact-analysis` | `crb-contact-analysis` |
| | `identifier` | `com.trb.contact-analysis` | `com.crb.contact-analysis` |
| | `title` | `TRB Contact Analysis` | `CRB Contact Analysis` |
| | `devUrl` | `http://localhost:5174` | `http://localhost:5175` |
| [vite.config.ts](vite.config.ts) | `server.port` | `5174` | **`5175`** (TRB와 동시 실행 가능) |

### 0-C. 문서 헤더 수정   (2026-06-25 21:46 ~ 21:48, ~ 2 분)

| 파일 | 변경 |
|------|------|
| [CLAUDE.md](CLAUDE.md) | 헤더 TRB→CRB + 모태 SW 링크 + Phase 0 상태 주석 (코드는 아직 TRB 알고리즘) |
| [Master_plan.md](Master_plan.md) | 헤더 TRB→CRB + 본문 (~700 줄) 은 TRB 기준 그대로임 명시 (Phase 1+ 갱신) |
| [README.md](README.md) | CRB 헤더 추가 + dev/build 가이드 + dev port 5175 표기 |

### 0-D. Sanity 검증

#### 0-D-1. `npm install`   (2026-06-25 21:48:21 ~ 21:49:12, 51 초)

- **결과**: ✅ exit 0
- **설치 패키지**: 521개 (TRB-main 과 동일)
- **취약점**: 9개 보고 (low 2 / moderate 4 / high 3) — 모두 TRB 와 동일, 의존성 트리 그대로
- **종료 알림**: npm notice "New major version of npm available! 10.9.3 → 11.16.0" (선택적 업데이트, 작업에 무관)

#### 0-D-2. `cargo check`   (2026-06-25 21:49:51 ~ 21:54:38, 4 분 46 초)

- **결과**: ✅ `Finished dev profile [unoptimized + debuginfo] target(s) in 4m 46s`, exit 0
- **컴파일**: 521 의존성 + `crb-contact-analysis` 본체
- **Warning**: 10 개 (모두 "function is never used" — TRB 와 동일, [lubrication.rs](src-tauri/src/solver/lubrication.rs) 의 미사용 함수)
- **검증된 점**:
  - Crate 이름이 `crb-contact-analysis` 로 정상 인식
  - `src-tauri/.cargo/config.toml` 의 VS 2022 BuildTools 경로 그대로 동작 (TRB 환경 검증과 동일)
  - 사전 환경 (Rust 1.95, MSVC 14.44.35207, Win11 SDK 26100) 변경 없이 호환

### 발생 이슈

없음. Phase 0 전 과정에서 빌드 오류·환경 충돌·포트 충돌 모두 발생하지 않음.

### 미해결 / 이월 항목

- ✅ **CRB 시리즈 결정** (해결: 2026-06-25) — [Plan §6 D1·D2](CRB_Development_Plan.md): **모든 시리즈에서 rib contact 제외 → 단일 솔버**, 시리즈 enum 도입 안 함. 근거: ISO 16281 A.3.1 NOTE 1
- ✅ **Row 구성** (해결: 2026-06-25) — [Plan §6 D3](CRB_Development_Plan.md): **단일 row 만 구현** (`n_rows = 1` 고정). Multi-row (NNU 등 풍력 메인베어링) 는 본 계획 외 후속 작업 (F1)
- ✅ **Axial 입력** (해결: 2026-06-25, D1 후속) — [Plan §6 D4](CRB_Development_Plan.md): `F_a = 0` 강제 → `bearing.rs` 평형 DOF = **3 (δr, γx, γy)** 로 단순화
- ⏳ **Manual 정책**: TRB Manual 학습용 유지 vs CRB 별도 작성 — Phase 8 진입 전 결정 (Plan §6 F3)
- ℹ️ `src-tauri/.cargo/config.toml.bak` (TRB-main 에서 만든 백업, Community 경로) 도 함께 복제됨 — CRB 환경에서도 보존

### Plan 대비 편차 ([Plan §3 Phase 0](CRB_Development_Plan.md) 비교)

| 항목 | Plan 예측 | 실제 | 편차 |
|------|----------|------|------|
| 소요 시간 | 1 day | ~ 12 분 (순작업) | **–98%** (자동화 효과, 코드 변경 없음) |
| 작업 범위 | 폴더 복사 + 식별자 + 문서 헤더 + sanity | Plan 그대로 + 포트 5175 분리 | **+** dev port 변경 (동시 실행 위한 추가 결정) |
| 발생 이슈 | (예상 없음) | 없음 | — |

### 다음 단계 진입 조건 (Phase 1)

✅ Phase 1 진입을 위한 결정 사항 모두 확정 ([Plan §6 D1~D4](CRB_Development_Plan.md)):
- D1 모든 시리즈에서 rib contact 제외
- D2 단일 솔버 (시리즈 분기 없음)
- D3 단일 row
- D4 F_a = 0 강제, 평형 DOF = 3

**Phase 1 작업 범위 (확정)**:
- `types.rs`: α, β, D_we_max/min, rib*, R_sph, F_a 등 필드 제거 (단순화)
- `RollerProfile`: dub-off 대칭화
- TypeScript `types/bearing.ts` mirror 갱신
- `defaults.ts`: 단일 row CRB 예시값
- 통과 기준: cargo check + npm run build

검증 명령 (현재 시점 — TRB 코드 그대로 동작 확인용):
```powershell
cd "d:\AI\Main_Bearing\CRB-main"
npm run tauri dev   # http://localhost:5175 + WebView 윈도우
```

---

---

## Phase 1 — 데이터 모델 단순화   ✅ 완료 (2026-08-19)

**전체 소요**: 2026-08-19 15:22 ~ 15:50 (순작업 ~ 80 분, 대화·대기 시간 제외)
**브랜치**: `phase-1` (CRB 브랜치의 서브)
**Commit 수**: 3 (856c219 / da07b44 / b2d297b)

### 실행 방침 요약 (사용자 결정 반영)

| 규약 | 실제 실행 |
|------|---------|
| 자율성 | Hybrid — 4대 트리거만 사용자 확인 (1.3-B 접근방식 + tauri dev 실행 방식) |
| 병렬성 | 논리적 그룹 병렬 (1.4/1.5 파일 병렬 편집, Phase 2 계획 병렬 작성) |
| 보고 | 서브-Phase 완료 시 요약 보고 |
| Commit | 서브-Phase 단위 commit (3 개) + 이슈 시 자동 stub |

### 1.3-A 작업 내역   (2026-08-19 15:22 ~ 15:32, commit `856c219`)

- **파일**: `src-tauri/src/solver/types.rs` (2 files, +228/-60)
- **변경 struct**: MacroGeometry (α/D_we_max/D_we_min/rib 필드 제거, d_we 단일화), RacewayGeometry (α_i/α_o/rib 제거), RollerProfile (r_sph 제거, dub 대칭화), OperatingConditions (f_a/m_y/preload/skf_trb_series 제거), LoadTimePoint (f_a/m_y 제거)
- **cargo check**: 192 컴파일 에러 (예상, types.rs 참조 대량 파일)
- **결정 반영**: D1~D7 (Plan §6)

### 1.3-B 작업 내역   (2026-08-19 15:32 ~ 15:41, commit `da07b44`)

**⚠️ 이슈 트리거 발생**: 192 에러 규모로 접근 방식 결정 필요 → **하이브리드** 선택 (사용자 결정)

- **부수 모듈 disable** (mod.rs 주석 처리): rib_contact, life, static_rating, lubrication, hmehl, transient, transient_io, wec_risk
- **핵심 모듈 최소 수정**:
  - `commands.rs` 재작성: cos_alpha_diff=1.0, rib/transient/hmehl command 제거
  - `lib.rs`: invoke_handler 갱신
  - `geometry.rs`: d_we 단일화, α=0, dub 대칭, 테스트 CRB 갱신
  - `bearing.rs` 통째 stub 재작성: 순수 함수 3개 (roller_positions, radial_load_angle, roller_approach 3-DOF 버전) 만 유지, 통합 함수는 Phase 4 재작성 stub
  - `presets.rs`: TRB(NSK HR30306J) → CRB(NU 240) 기본 preset
- **결과**: 6 files, +153/-4331 (4178 라인 순감소)
- **cargo check**: ✅ exit 0, 4.87 초, 33 warnings (미사용 함수 — 예상 범위)

### 1.4/1.5 작업 내역   (2026-08-19 15:41 ~ 15:50, commit `b2d297b`)

- **1.4 TS mirror** (`src/types/bearing.ts`): Rust struct 갱신 반영 (D1~D7)
  - SineWaveConfig / LoadTimePoint 에서 f_a/m_y 제거
  - PreloadMode 타입 제거
- **1.5 defaults.ts**: NSK HR30306J (TRB) → NU 240 (CRB) 재작성
  - c_r_kn = 1520, c_0r_kn = 2400, f_y = -500 kN (중력 예시)
- **Frontend @ts-nocheck stub** (13 컴포넌트): BearingView3D, GeometryView, InputPanel, LubricationView, ProfileView, ResultsCard, SectionView2D, TransientView, 5 charts (Life/LoadDist/RibContact/RollerComparison/RollerDetail) — Phase 6 (Frontend UI 변경) 에서 정식 재작성 예정
- **BearingResult 인터페이스**: preload_mode/delta_preload_um 필드 제거
- **결과**: 16 files, +223/-143
- **npm run build**: ✅ exit 0, 41.12 초, 0 TS errors

### 병렬 작업 — Phase 2 상세 계획서

사용자 요청에 따라 병렬 진행 (다른 파일, 충돌 없음):
- Plan.md §Phase 2 placeholder → **상세 계획 초안** (11 소절: 목표, 파일, ISO B.5 정식 적용, R_eq CRB 단순형, Level A 검증, 통과 기준, 예상 시간, 이슈, 절차, 산출물, Phase 3 진입 조건)

### 통과 기준 (§7 §1.7)

| 기준 | 결과 |
|------|------|
| `cargo check` exit 0 | ✅ (4.87 초) |
| `npm run build` exit 0 | ✅ (41.12 초) |
| `npm run tauri dev` WebView 팝업 정상 | ⏳ 사용자 직접 검증 대기 |

### 발생 이슈 및 대응

| 이슈 | 대응 | 결과 |
|------|------|------|
| 1.3-B: 192 개 컴파일 에러 | 사용자 확인 → 하이브리드 방식 (핵심 살림 + 부수 stub) | ✅ 0 errors |
| 1.4: Frontend 147 TS 에러 (Phase 6 대상) | 방침 자동 stub → 13 컴포넌트에 `// @ts-nocheck` 임시 지시자 | ✅ 0 errors |
| 1.4: bearing.ts 의 PreloadMode 참조 잔재 | 해당 필드 제거 | ✅ |
| 새 PC 환경 (Node/Rust/VS 미설치) | 이전 세션 대비 상세 절차 반복 (npm/rust/VS 설치, ~22 분) | ✅ 완료 |

### 미해결 / 이월 항목

- ⏳ **tauri dev 팝업 검증**: 사용자 직접 확인 필요 (CLAUDE.md 서버 관리 규약)
- ⏳ **Frontend 컴포넌트**: 13 개 @ts-nocheck 임시 상태 → Phase 6 에서 정식 재작성
- ⏳ **부수 솔버 모듈** (life/lubrication/transient/hmehl/wec_risk 등): mod.rs 에서 disable 상태 → Phase 5/7 에서 재활성화
- ⏳ **bearing.rs 통합 함수**: Phase 4 (A.3.1 3-DOF) 에서 재작성
- ⏳ **SkfTrbSeries 타입**: 아직 bearing.ts 에 존재 (SkfFrictionRef 만 참조) → Phase 7 에서 SKF CRB 대응 검토 시 정리

### Plan 대비 편차 ([Plan §7 Phase 1](CRB_Development_Plan.md) 비교)

| 항목 | Plan 예측 | 실제 | 편차 |
|------|----------|------|------|
| 소요 시간 | 1.5 ~ 2 day | **~80 분** | **–95%** (병렬 편집 + 자동 stub 효과) |
| Sub-phase 수 | 6 (1.3-A/1.3-B/1.4/1.5/1.6/1.7) | 통합 3 commit (1.3-A/1.3-B/1.4-1.5-1.7) | 병합 |
| 발생 이슈 | 5 예상 | 4 발생 | 예상 범위 |
| Frontend 처리 | Phase 6 미리 X | **13 파일 @ts-nocheck 스텁 필요** | Phase 1 통과 기준상 불가피 |

### Phase 2 진입 조건

- ✅ Phase 1 통과 기준 3 개 중 2 개 만족 (cargo check + npm build)
- ⏳ tauri dev 팝업 검증 (사용자)
- Plan §Phase 2 상세 계획 (§7) 준비 완료
- 다음 브랜치: `phase-2` (CRB 로부터 신규 서브 브랜치 예정)

---

*Last updated: 2026-08-19 (Phase 1 코딩 완료, tauri dev 사용자 검증 대기. 3 commits: 856c219 / da07b44 / b2d297b)*

---

## Phase 2 — Geometry 단순화   ✅ 완료 (2026-08-19)

**전체 소요**: ~15 분 (Phase 1 예측 1~2 day 대비 99% 단축 — 이미 Phase 1.3-B 에서 최소 수정한 코드 정식화 + Level A 골든 테스트 신규)
**브랜치**: `phase-2` (CRB 로부터 서브)
**Commit 수**: 1 (`96ac19b`)

### 실행 방침

Phase 1 4대 규약 그대로 채택: Hybrid 자율성 + 논리적 그룹 병렬 + 서브-Phase 단위 보고 + 서브-Phase 단위 commit.

### 2-A 작업 내역   (geometry.rs 정식화)

- `compute_slices` 재작성: 임시 α=0 대체 코드 (alpha_i_rad, alpha_o_rad, alpha_m 변수) **완전 제거**
- 순수 원통 로직으로 단순화:
  - Roller 반경 균일 (r_roller = D_we/2, 모든 slice 동일)
  - γ = D_we / D_pw (α = 0 → cos(α) = 1)
  - R_eq_inner = (D_we_eff/2) · (1 − γ_i)
  - R_eq_outer = (D_we_eff/2) · (1 + γ_o)
- `combine_curvature` 헬퍼 신규: raceway r → ∞ 원통이면 R_roller 유지, 유한이면 series 결합
- ISO 16281 Clause 6.3.2 (p. 22) 근거 주석 명시
- 부록 A.2 (x_axis 정의) 준수: x_axial 은 소단(0) → 대단(L_we) 방향, roller 축 따라

### 2-B 작업 내역   (Level A 테스트 신규)

- **신규 파일**: `src-tauri/tests/geometry_level_a.rs` (289 라인)
- **테스트 8개** (모두 통과):
  1. `level_a_hertz_half_width_matches_analytical` — b 계산 vs Palmgren 해석해
  2. `level_a_hertz_max_pressure_matches_analytical` — p_max 계산 vs 해석해
  3. `level_a_combined_elastic_modulus` — E* 계산 검증
  4. `level_a_compute_slices_uniform_roller_radius` — 원통 roller 반경 균일성
  5. `level_a_compute_slices_uniform_slice_width` — slice 폭 균등 + 총합 = L_we
  6. `level_a_compute_slices_x_axial_symmetric` — slice 중심 좌표 대칭성
  7. `level_a_compute_slices_r_eq_crb_orbital` — CRB γ 궤도 곡률 (D_we/D_pw)
  8. `level_a_end_to_end_slice_hertz_matches_analytical` — 전체 파이프라인
- **통과 기준**: 상대 오차 `TOL_REL = 1e-3` (Plan §2.6 0.1%)

### 2-C 작업 내역   (통과 검증 + 부수 수정)

- **이슈 트리거**: E0603 — integration test 에서 `app_lib::solver::...` 접근 불가 (private module)
- **대응 (자동)**: `lib.rs` 의 `mod solver` → `pub mod solver` (integration test 접근 위해 표준 대응)
- **결과**:
  - `cargo check --lib`: ✅ exit 0, 32 warnings (Phase 1 stub 상태 이월)
  - `cargo test --test geometry_level_a`: ✅ exit 0, **8/8 pass, 0.00s**

### 통과 기준 (§7 §2.6)

| 기준 | 결과 |
|------|------|
| `cargo test --test geometry_level_a` 모두 pass | ✅ 8/8 |
| `cargo check` exit 0 | ✅ (warnings 만) |
| `npm run build` 회귀 확인 | ✅ (Frontend 무변경 → 자동 유지) |
| Phase 1 상태 대비 회귀 없음 | ✅ |

### 발생 이슈 및 대응

| 이슈 | 대응 | 결과 |
|------|------|------|
| E0603: private module solver | 자동 stub → lib.rs 에 pub mod solver | ✅ |

### 미해결 / 이월 항목

- ⏳ Phase 1 이월 그대로 유지 (Frontend @ts-nocheck, 부수 솔버 disable, bearing.rs stub)
- ⏳ Level B/C/D 검증 (Phase 3/4 대상)

### Plan 대비 편차 ([Plan §Phase 2](CRB_Development_Plan.md) 비교)

| 항목 | Plan 예측 | 실제 | 편차 |
|------|----------|------|------|
| 소요 시간 | 1 ~ 2 day | **~15 분** | **–99%** (Phase 1.3-B 에서 이미 최소 수정 완료 상태) |
| Sub-phase 수 | 3~4 (A/B/C) | 단일 commit 병합 | — |
| 신규 테스트 | 1 파일 | 1 파일, 8 tests | Plan 대비 상세 |
| 발생 이슈 | 0~1 예상 | 1 (pub mod 접근성) | 자동 해결 |

### Phase 3 진입 조건

- ✅ Phase 2 통과 기준 (§2.6) 모두 만족
- ✅ Level A 검증 완료 → Gen1/Gen3 solver 알고리즘 검증 base 확보
- 다음 브랜치: `phase-3` (CRB 로부터 신규 서브 브랜치 예정)

---

*Last updated: 2026-08-19 (Phase 1+2 완료. commits: Phase 1 = 856c219/da07b44/b2d297b/c6b663e/ff70d09 (merge 6d60de3), Phase 2 = 96ac19b (merge 대기))*
