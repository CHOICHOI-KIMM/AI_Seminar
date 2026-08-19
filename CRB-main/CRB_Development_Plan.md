# CRB Contact Analysis System — 개발 계획서

> 기존 TRB-main 의 SW 체계를 기반으로 원통롤러베어링(CRB, Cylindrical Roller Bearing) 해석 SW 를 신규 개발하기 위한 종합 계획.
>
> - **작성 기준**: ISO 16281:2025 (Rolling bearings — Methods for calculating the modified reference rating life)
> - **모태 SW**: `d:/AI/Main_Bearing/TRB-main/` (Tauri 2.0 + Rust + React 데스크톱 앱)
> - **목표 SW**: `d:/AI/Main_Bearing/CRB-main/` (본 폴더)

---

## 1. ISO 16281 기반 CRB ↔ TRB 핵심 차이 요약

> ISO 16281:2025 (en) 참조 위치를 **CRB/TRB 각각 분리** + **정확한 ISO 원문 페이지** 표기.
> 페이지는 `TRB-main/Reference/ISO_16281_2025(en).pdf` 의 ISO 원문 페이지 번호 (PDF 페이지 = ISO 페이지 + 6) 기준으로 직접 확인.
> ⚠️ Clause 구조: **3** = Terms and definitions, **4** = Symbols and abbreviated terms, **5** = Calculation of rating life, **6** = Calculation of contact deflection.
> `—` 는 해당 표준에 별도 절이 없거나 공통 처리.

| 구분 | 항목 | CRB (Cylindrical Roller Bearing) | TRB (Tapered Roller Bearing) | CRB 참조 (ISO 16281:2025) | TRB 참조 (ISO 16281:2025) |
|------|------|----------------------------------|------------------------------|---------------------------|---------------------------|
| **Geometry** | Roller 형상 | 원통 (β = 0°) | 원추 (β = half cone angle) | Clause 4 Symbols (**p. 2**) | Clause 4 Symbols: β (**p. 5**) |
| | Contact angle | α = 0° (순수 radial) | α ≠ 0° (10~30°) | A.3.1 radial bearing (**pp. 22–23**) | A.3.2 + `Figure A.4` (**pp. 24–25**) |
| | Roller diameter | D_we (균일) | D_we_max / D_we_min | Clause 4 D_we (**pp. 3, 5**) | Clause 4 Symbols (**pp. 3, 5**) |
| | L_we 기준축 | along **roller axis** | along **roller contact line** (β 보정) | Clause 4 NOTE 3 (**p. 4**) | Clause 4 NOTE 3 (**p. 4**) |
| | x_k 좌표 | roller axis 따라 | lateral surface 따라 | Clause 4 NOTE 5 (**p. 5**) | Clause 4 NOTE 5 (**p. 5**) |
| **Raceway** | Taper angle (α_i, α_o) | 0° (cylindrical bore) | ≠ 0° | A.3.1 본문 (**pp. 22–23**) | A.3.2 본문 (**pp. 24–25**) |
| | Profile (radial) | B.5 (단순 대칭) | B.6 (β 보정 `0.00045/cos β`) | **B.5** (**p. 29**) | **B.6** (**p. 29**) |
| | Profile (thrust) | B.8 (대칭) | B.9 (β 보정) | **B.8** (**p. 29**) | **B.9** (**p. 30**) |
| **Rib contact** | 필요 여부 | **일반적으로 무시** (보조적) | **필수** (axial 지지) | A.3.1 **NOTE 1** "not necessary" (**p. 22**) | A.3.2 본문 `axial force on the rib` (**p. 24**) |
| | 입력 (h_rib, α_rib, R_sph) | 옵션/생략 | 필수 | — | A.3.2 + B.6 (**pp. 24, 29**) |
| **Load** | Axial 지지 | 약하거나 없음 (시리즈에 따라) | 구조적으로 지지 | A.3.1 (**pp. 22–23**) | A.3.2 `Figure A.4` (**pp. 24–25**) |
| | 평형 DOF | 3~4-DOF (radial+tilt 중심) | 5-DOF (δx, δy, δz, γx, γy) | A.3.1 Formula (A.11)/(A.12) (**p. 22**) | A.3.2 Formula 본문 (**pp. 24–25**) |
| | 접촉력 방향 | radial direction | perpendicular to taper raceway | A.3.1 `Figure A.2` (**p. 23**) | A.3.2 `Figure A.4` (**p. 25**) |
| **Elastic deflection** | 공식 번호 | Formula **(42)** | Formula **(47)** | **6.3.2** Cylindrical rollers (**p. 17**) | **6.3.3** Tapered rollers (**p. 17**) |
| | Spring constant 기호 | c_R | c_T | Clause 4 Symbols (**pp. 2–5**) | Clause 4 Symbols (**pp. 2–5**) |
| **Profile (Roller)** | Dub-off | 대칭 (단일 δ_dub, L_dub) | 비대칭 (large/small 분리) | B.5 (**p. 29**) | B.6 (**p. 29**) |
| | End sphere R_sph | 옵션 (rib 없으면 불필요) | 필수 | — | B.6 + A.3.2 (**pp. 24, 29**) |
| **Lamina model** | Figure 참조 | `Figure A.2` (cylindrical) | `Figure 2` (tapered lamina) / `Figure A.4` (FBD) | 6.3.2 + A.3.1 (**pp. 17, 23**) | 6.3.3 + A.3.2 (**pp. 17, 25**) |
| **Stress concentration** | 적용 식 / 함수 | f[j,k] 정의는 동일 (C.1/C.2) | 동일 (C.1/C.2) | 5.3.5 + C.1/C.2 (**pp. 13, 31–32**) | 5.3.5 + C.1/C.2 (**pp. 13, 31–32**) |
| **Roller bending (Gen3)** | EI(x) | **균일** (단면 일정) | **위치 종속** (가변 I_k) | — (구현 영역, ISO 외) | — (구현 영역, ISO 외) |
| **Fatigue life** | Lamina life 식 | 동일 (Lundberg-Palmgren) | 동일 | 5.3 (**pp. 10–14**) | 5.3 (**pp. 10–14**) |
| | C_R 상수 | ISO 281 CRB 식 | ISO 281 TRB 식 | ISO 281 (외부 참조) | ISO 281 (외부 참조) |
| **Pre-/Post-processing** | Hertz parameter 계산 | Annex E 참조 | Annex E 참조 | E (Hertzian parameters) | E (Hertzian parameters) |

### 1.1 페이지 매핑 빠른 참조표 (ISO 원문 페이지 기준)

| ISO 16281 절 | ISO 페이지 | PDF 페이지 | 주요 내용 |
|--------------|-----------|-----------|----------|
| 1 Scope / 2 Normative references | 1 | 7 | 적용 범위 |
| 3 Terms and definitions | 1~2 | 7~8 | basic / modified reference rating life 등 |
| **4 Symbols and abbreviated terms** | **2~5** | 8~11 | D_we, β, α, c_R, c_T 등 기호 정의 (NOTE 3 @ p.4, NOTE 5 @ p.5) |
| 5.1 Ball bearings | 6 | 12 | 볼 베어링 수명 |
| 5.2 Roller bearings (intro) | 7~9 | 13~15 | (skipped — 5.3 가 본 표 핵심) |
| **5.3 Roller bearings (lamina-level)** | **10~14** | 16~20 | Lamina life. **5.3.5 stress concentration @ p. 13** |
| 6.1~6.2 Line contact intro | 15~16 | 21~22 | 양면 접촉 → 단면 분리 |
| **6.3.2 Cylindrical rollers** | **17** | 23 | CRB 탄성변형 Formula **(42)~(46)** |
| **6.3.3 Tapered rollers** | **17** | 23 | TRB 탄성변형 Formula **(47)~(50)**, `Figure 2` |
| Annex A 시작 / A.1 | 20 | 26 | Internal load distribution intro |
| **A.3.1 Cylindrical roller bearings** | **22~23** | 28~29 | CRB 평형, `Figure A.2` @ p.23, **NOTE 1** "rib 무시 가능" @ p.22 |
| **A.3.2 Tapered roller bearings** | **24~25** | 30~31 | TRB 평형, `Figure A.4` @ p.25 (rib 포함 FBD) |
| **B.5 Cylindrical & needle (radial)** | **29** | 35 | CRB profile Formula |
| **B.6 Tapered (radial)** | **29** | 35 | TRB profile Formula, β 보정 |
| B.8 Thrust cylindrical & needle | 29 | 35 | Thrust CRB profile |
| B.9 Thrust tapered | 30 | 36 | Thrust TRB profile, β 보정 |
| C.1 Stress concentration (detailed) | 31 | 37 | Non-Hertzian 상세 |
| C.2 Stress concentration (approx.) | 32 | 38 | 근사 함수 f[j,k] |
| Annex E Hertzian parameters | (참조) | (별도) | 본 표 사용 식의 파라미터 정의 |

### 1.1 핵심 결론

CRB 는 ISO 16281 관점에서 **TRB 의 특수 케이스(β = 0, α = 0)** 로 볼 수 있어, TRB-main 의 아키텍처와 알고리즘 골격을 그대로 재활용 가능. 단, 일부 모듈은 단순화 또는 분기 처리, 일부는 입력 인터페이스 자체 재설계가 필요.

---

## 2. TRB-main 모듈별 CRB 전환 영향도

### 2.1 Rust solver (`src-tauri/src/solver/`)

| 모듈 | 변경 강도 | 주요 변경 사항 |
|------|---------|--------------|
| `types.rs` | 🔴 **대규모** | `MacroGeometry`: α 제거(0 고정), D_we 단일화, **rib 필드 제거**(D1). `RacewayGeometry`: α_i / α_o 제거. `RollerProfile`: dub-off 대칭화, **R_sph 제거**(D1). `OperatingConditions`: **F_a 제거 또는 0 강제**(D4) |
| `geometry.rs` | 🟡 중간 | 슬라이스의 `r_roller_k = D_we/2` 균일 (테이퍼 보정 제거). 등가 곡률반경 계산 단순화 |
| `hertz.rs` | 🟢 거의 불변 | Hertz line contact + Weber bulk 공식은 동일 |
| `gen1.rs` | 🟢 거의 불변 | 독립 슬라이스 방식 동일. δ_rigid 계산만 단순화 (α = 0) |
| `gen3.rs` | 🟡 중간 | Newton-Raphson 동일, active set 동일. EI_k 균일에 따라 행렬 구조 단순화 |
| `beam.rs` | 🟡 중간 | Timoshenko beam 유효, I_k = const 로 단순화 (행렬 banded 구조 유지) |
| `rib_contact.rs` | 🟢 **비활성/제거** (D1) | 모든 시리즈에서 미사용 — 모듈 자체 삭제 또는 `mod.rs` 등록 해제 |
| `bearing.rs` | 🔴 **대규모** | 5-DOF → **3-DOF: `(δx, δy, γx)`** (D4+D6+D7). 접촉력 방향 = 순수 radial. ISO 16281 A.3.1 알고리즘. **단일 row** (D3). 좌표계: Y=수직(중력), X=수평, Z=shaft (D5). misalignment 는 X축 about (M_x) 만, γ_y = 0 강제 (D6) |
| `life.rs` | 🟡 중간 | C_R 상수 사용 (ISO 281 CRB 식), lamina life 합성식은 동일 |
| `static_rating.rs` | 🟡 중간 | ISO 76 CRB 식 (C_0r 계수 다름) |
| `transient.rs` / `transient_io.rs` | 🟢 불변 | 시간 영역 통합 framework 동일 |
| `hmehl.rs`, `lubrication.rs` | 🟢 불변 | EHL 식 그대로 적용 (cylindrical line contact 가 base) |
| `wec_risk.rs` | 🟢 불변 | 동일 |

### 2.2 Frontend (`src/components/`)

| 컴포넌트 | 변경 강도 | 변경 내용 |
|----------|---------|-----------|
| `InputPanel/` | 🔴 **대규모** | α, β, D_we_max/min, rib 입력 제거/통합. dub-off 단일 필드. preset UI 변경 |
| `BearingView3D/` | 🔴 **대규모** | 원추 → 원통 형상 렌더링 (Three.js `CylinderGeometry`) |
| `SectionView2D/` | 🟡 중간 | 2D 단면도 — 원통 단면으로 |
| `ProfileView/` | 🟢 거의 불변 | profile 시각화는 동일 (대칭만 다름) |
| `ResultCharts/`, `ContourMap/`, `ComparisonView/` | 🟢 불변 | 차트/등고선/비교 동일 |
| `LubricationView/`, `TransientView/`, `ThermalSpeedView/` | 🟢 불변 | 동일 |
| `GeometryView/` | 🟡 중간 | 형상 미리보기 — 원통화 |

---

## 3. 권장 작업 프로세스 (Phase 0 ~ 8)

```
┌─────────────────────────────────────────────────────────────────┐
│ Phase 0: 폴더 복제 + 환경 분리                          [1 day] │
│  • TRB-main → CRB-main 전체 복사                                │
│  • package.json name 변경 (trb-app → crb-app)                   │
│  • Cargo.toml name 변경 (trb-contact-analysis → crb-)           │
│  • src-tauri/tauri.conf.json: app identifier 변경                │
│  • src-tauri/.cargo/config.toml 은 사용자 환경 그대로 유지       │
│  • README/CLAUDE.md 헤더 변경 (TRB → CRB)                       │
│  • Sanity check: npm install && npm run tauri dev                │
└─────────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ Phase 1: 데이터 모델 단순화                          [2~3 days] │
│  • types.rs:                                                     │
│    - MacroGeometry: α 제거, D_we 단일화, rib → Option            │
│    - RacewayGeometry: α_i/α_o 제거                              │
│    - RollerProfile: dub-off 대칭, R_sph Option                  │
│  • TypeScript types/bearing.ts mirror 업데이트                   │
│  • defaults.ts: CRB 표준 예시 (예: NU2240, N2240)                │
│  • presets.rs: TRB preset 제거, CRB preset 추가                  │
│  • 통과 기준: cargo check + npm run build                        │
└─────────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ Phase 2: Geometry 단순화                              [2 days]  │
│  • geometry.rs: compute_slices 를 cylindrical 로                 │
│    - r_roller_k = D_we/2 (균일)                                  │
│    - 등가 곡률반경: R_eq = D_we/2 × r_race/(D_we/2 + r_race)     │
│    - profile superposition (ISO B.5 적용)                        │
│  • Golden test: 단일 slice Hertz 해석해와 비교 (Level A < 0.1%) │
└─────────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ Phase 3: Roller-Level Solver (Gen1/Gen3)             [3~4 days] │
│  • gen1.rs: δ_rigid 단순화 (α = 0), 알고리즘 골격 유지            │
│  • beam.rs: I_k = const, 행렬 구조 단순화 (banded 유지)          │
│  • gen3.rs: Newton-Raphson 동일, active set 동일                  │
│  • 검증: Gen1 ↔ Gen3 교차 검증 (flat profile 시 수렴, Level C)  │
└─────────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ Phase 4: Bearing-Level Equilibrium (3-DOF)            [3 days]  │
│  • bearing.rs: ISO 16281 A.3.1 알고리즘 (CRB)                    │
│    - 평형 DOF: {δr, γx, γy} (axial 무시 또는 시리즈별 분기)      │
│    - 접촉력 방향 = pure radial                                   │
│    - Q_j 계산: 각 roller 위치 ψ_j 에서 radial 침투                │
│  • Tauri command 시그니처 그대로 (solve_bearing 등)              │
│  • rib_contact.rs: Option 처리, 입력 없으면 skip                 │
│  • 검증: MASTA/Bearinx CRB 결과와 비교 (Level D < 5%)            │
└─────────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ Phase 5: Life / Static Rating                         [2 days]  │
│  • life.rs: ISO 16281 5.3 식 그대로 (lamina-level)              │
│    - 단, basic dynamic load rating C_R = ISO 281 CRB 식          │
│  • static_rating.rs: ISO 76 CRB 식 (p_0 = 4000 MPa 기준)         │
└─────────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ Phase 6: Frontend UI 변경                            [4~5 days] │
│  • InputPanel/: α/β/D_we_max/min 필드 제거, rib 섹션 옵션화      │
│  • BearingView3D/: 원통 roller 렌더링 (CylinderGeometry)         │
│  • SectionView2D/: 원통 단면도                                    │
│  • Defaults / preset UI 업데이트                                 │
│  • npm run tauri dev 로 end-to-end 동작 확인                     │
└─────────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ Phase 7: 추가 모듈 적용 (Lubrication/Transient/HMEHL) [2 days]  │
│  • hmehl.rs / lubrication.rs / transient.rs: 거의 그대로         │
│    - kinematic 식만 CRB 형태로 (cage speed, slip velocity 등)    │
│  • View 컴포넌트 (LubricationView, TransientView) 동일 유지      │
└─────────────────────────────────────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ Phase 8: 검증 + 문서화                               [3~4 days] │
│  • Manual/ 폴더: CRB 기준으로 17 챕터 재작성                     │
│    (특히 02_Geometry, 06/07 Solver, 09 Equilibrium, 16 LoadFlow) │
│  • CLAUDE.md, Master_plan.md → CRB 버전                          │
│  • ISO 16281 검증 예제 (Annex) 실행                              │
│  • MESYS/MASTA CRB 결과와 비교 보고서 (reports/)                 │
└─────────────────────────────────────────────────────────────────┘
```

**총 예상 기간**: **약 22 ~ 26 일** (3.5 ~ 4 주, 단일 개발자 풀타임 기준)

---

## 4. 우선순위 / 리스크 매트릭스

> §6 결정 반영 (D1: rib 제거, D3: 단일 row, D4: F_a 제거).

| 작업 | 영향도 | 난이도 | 리스크 |
|------|-------|-------|-------|
| `bearing.rs` **3-DOF (δr, γx, γy)** 평형 재설계 | 🔴 매우 큼 | 🔴 높음 | 좌표계 / 부호 실수 → 결과 전체 오염 |
| `types.rs` 필드 제거 (α, β, D_we_max/min, rib*, R_sph, F_a) | 🔴 큼 | 🟡 중 | Frontend 동기화 누락 시 빌드 깨짐 |
| `BearingView3D` 원통 렌더링 | 🟡 중 | 🟡 중 | Three.js 기하 변환 — 시각만 영향 |
| `geometry.rs` slice 변환 | 🟡 중 | 🟢 낮음 | 균일화로 단순화 |
| Gen3 `beam.rs` EI 균일화 | 🟢 작음 | 🟢 낮음 | 검증 용이 |
| `rib_contact.rs` **비활성/삭제** (D1) | 🟢 작음 | 🟢 낮음 | 단순 제거 — `mod.rs` + Tauri command 등록부에서 해제 |
| Lubrication / Transient 이식 | 🟢 작음 | 🟢 낮음 | 그대로 |

---

## 5. 권장 첫 작업 순서

오늘 / 이번 주에 시작한다면:

1. **Phase 0 — 폴더 복제**: 가장 단순, 즉시 가능. CRB-main 환경에서 `npm run tauri dev` 까지 한 번 띄워보기 (TRB 그대로 동작하는지 sanity check).
2. **Phase 1 — types.rs 단순화**: 데이터 구조부터 정리해야 이후 모듈 변경이 깨끗함. Rust 컴파일러가 강제 가이드 역할.
3. **Phase 4 — bearing.rs 재설계 선제 검토**: 가장 어려운 부분이라 일찍 paper-design 해두기 권장.

---

## 6. 결정 사항 (Decisions)

> 사용자 결정 (2026-06-25). Phase 1 진입을 위한 scope 확정.

### 6.1 결정 완료 항목

| # | 결정 항목 | 결정 내용 | 근거 / 영향 |
|---|----------|----------|------------|
| **D1** | **Rib contact 처리** | **모든 시리즈에서 고려 제외** (N/NU/NJ/NUP/NN/NNU 무관) | ISO 16281 A.3.1 NOTE 1 (ISO p. 23): *"for typical load cases, the consideration of axial rib loads for cylindrical roller bearings is not necessary"*. NJ/NUP 의 small axial 도 본 SW 범위에서는 무시. |
| **D2** | **시리즈별 분기** | **단일 솔버** — 시리즈 enum 도입 안 함, 평형/접촉/수명 알고리즘 동일 | D1 결정 결과 시리즈별 차이가 사라짐. 시리즈는 명세 외 정보(사용자 메모)로만 관리. |
| **D3** | **Row 구성** | **단일 row 만 구현** (`n_rows = 1` 고정) | Phase 1~8 본체는 단일 row 만. 풍력 메인베어링용 multi-row (NNU 등) 는 본 계획 외 후속 작업으로 분리. |
| **D4** | **Axial 입력 처리** | D1·D2 결과 — `OperatingConditions.F_a` 는 **항상 0** 으로 강제 (또는 입력 자체 제거) | `bearing.rs` 평형 DOF 가 명확히 radial+tilt 중심으로 단순화 |

### 6.2 결정에 따른 Plan 영향 (Section 2~4 일관성 갱신)

| 영향 위치 | 변경 |
|----------|------|
| §2.1 `rib_contact.rs` | "선택화 (Option 분기)" → **"미사용 / 모듈 비활성"** (D1) |
| §2.1 `bearing.rs` | "5-DOF → 3~4-DOF" → **"3-DOF (δr, γx, γy) — axial 제외"** (D1+D4) |
| §2.1 `types.rs` | "rib 필드 Option<>" → **"rib 필드 제거"** (D1) |
| §4 우선순위 매트릭스 | "`rib_contact.rs` Option 처리" 행 → **"`rib_contact.rs` 비활성/제거" (난이도·리스크 모두 🟢 낮음)** |
| Phase 1 작업량 | 시리즈 enum/validation 분기 제거로 **−0.5 day** |
| Phase 4 작업량 | DOF 3 으로 확정 → 평형식 단순 → **−0.5 day** |
| **총 예상 기간** | 22~26 일 → **약 21~25 일** (변동 작음) |

### 6.3 좌표계 & 모멘트 축 결정 (2026-06-25 추가)

| ID | 결정 항목 | 결정 내용 | 근거 |
|----|----------|----------|------|
| **D5** | **좌표계** (TRB-main 그대로 승계) | X = horizontal radial, **Y = vertical radial (= 중력 방향)**, Z = bearing axis (shaft) | [bearing.rs:78](src-tauri/src/solver/bearing.rs#L78), BearingView3D 의 `[px,py,0]` 배치 + Three.js Y-up |
| **D6** | **Single-plane misalignment 축** | **M_x (γ_x) 만 사용**, M_y = γ_y = 0 강제 | ISO 16281 A.3.1 Formula (A.10) (p. 23) 가 한 평면 misalignment 만 다룸. **중력 방향이 아닌 X축** 채택 (풍력 메인베어링의 자중·풍하중 모멘트가 X축 about 임) |
| **D7** | **평형 DOF (정확 명세)** | **3-DOF: (δx, δy, γx)** | D4 (δz 제거) + D6 (γy 제거). 기존 §2.1 "3-DOF (δr, γx, γy)" 표기는 잘못 — D7 로 정정 |

### 6.4 미결정 / 후속 항목 (본 계획 외)

| # | 항목 | 처리 시점 |
|---|------|---------|
| F1 | Multi-row CRB (NNU 49 등 풍력 메인베어링 표준) | 본 계획 (Phase 0~8) 종료 후 별도 sub-project |
| F2 | NJ/NUP small axial rib 지지 모델 | 본 SW 가 시장 검증 통과 후 옵션 모듈로 |
| F3 | TRB-main Manual 학습용 유지 여부 / CRB 별도 작성 | Phase 8 진입 전 결정 |
| F4 | 풍력 메인베어링용 reference 모델 (예시 geometry) | Phase 1 의 defaults.ts 작성 시 선택 |

---

## 7. Phase별 상세 작업 계획

> 각 Phase 의 진입 시점에 본 섹션을 상세화. 현재는 **Phase 1 만 상세**, Phase 2~8 은 placeholder.
> 실제 작업 내역·검증·이슈는 [CRB_Development_Action.md](CRB_Development_Action.md) 에 누적 기록.

---

### Phase 1 — 데이터 모델 단순화

#### 1.1 목표

- TRB 특화 데이터 구조를 CRB scope (Plan §6 D1~D4) 에 맞게 단순화
- **목적**: 이후 Phase 2~5 의 알고리즘 변경이 깨끗하게 진행되도록 데이터 골격을 먼저 정리
- **빌드 가능 상태 유지**: `cargo check` + `npm run build` 통과 (솔버 결과의 수치 정합성은 Phase 2 이후 검증)

#### 1.2 작업 대상 파일

| 파일 | 변경 강도 | 주요 작업 |
|------|---------|---------|
| [src-tauri/src/solver/types.rs](src-tauri/src/solver/types.rs) | 🔴 대규모 | 5개 struct 필드 제거/통합 |
| [src/types/bearing.ts](src/types/bearing.ts) | 🔴 대규모 | Rust mirror 동기화 |
| [src/defaults.ts](src/defaults.ts) | 🟡 중간 | CRB 표준 기본값 (단일 row, F_a=0) |
| [src-tauri/src/presets.rs](src-tauri/src/presets.rs) | 🟡 중간 | TRB preset 제거 + CRB preset 자리만 마련 |
| [src-tauri/src/solver/mod.rs](src-tauri/src/solver/mod.rs) | 🟢 작음 | `pub mod rib_contact;` 주석 처리 (D1) |
| (이슈 발생 시 임시) 솔버 모듈들 (`bearing.rs`, `gen1.rs`, `gen3.rs`, `life.rs`, `rib_contact.rs` 등) | 🟢 stub 처리 | 제거된 필드 참조부를 임시 stub (`unimplemented!()` / `todo!()`) 으로 → Phase 2~5 에서 본격 수정 |

#### 1.3 `types.rs` 상세 변경 명세

> 컬럼: **유지** = 그대로 / **제거** = 삭제 / **통합** = 두 필드를 하나로 / **추가** = 신규.

**`MacroGeometry`** ([types.rs:29](src-tauri/src/solver/types.rs#L29))

| 필드 | 현재 (TRB) | 변경 후 (CRB) | 근거 |
|------|-----------|--------------|------|
| `d` (bore) | 유지 | 유지 | — |
| `outer_diameter` | 유지 | 유지 | — |
| `t` (width) | 유지 | 유지 | — |
| `alpha` | f64 (deg) | **제거** | CRB α = 0 (Plan §1) |
| `z` (roller 수) | 유지 | 유지 | — |
| `d_we_max`, `d_we_min` | 분리 | **통합 → `d_we: f64`** | CRB 원통 = 균일 D_we (Plan §1) |
| `l_we` | 유지 | 유지 | — |
| `d_pw` | 유지 | 유지 | — |
| `h_rib`, `alpha_rib`, `h_c` | rib 정의 | **모두 제거** | D1 (rib contact 제외) |
| `g_r` (radial clearance) | 유지 | 유지 | — |

**`RacewayGeometry`** ([types.rs:76](src-tauri/src/solver/types.rs#L76))

| 필드 | 변경 |
|------|------|
| `alpha_i`, `alpha_o` | **제거** (CRB raceway 비-원추) |
| `r_i`, `r_o` (transverse curvature) | **유지** (CRB 도 transverse 곡률 있음 — 일반적으로 ∞ 처리 가능하지만 필드는 보유) |
| `r_rib`, `r_rib_circ` | **제거** (D1) |
| `d_uc`, `l_uc` (undercut) | 유지 (CRB 도 raceway undercut 있음) |

**`RollerProfile`** ([types.rs:103](src-tauri/src/solver/types.rs#L103))

| 필드 | 변경 |
|------|------|
| `crown_type` (enum) | **유지** (Logarithmic/Circular/Parabolic/Custom/Polynomial — CRB 도 동일) |
| `delta_c` (crown drop) | 유지 |
| `delta_dub_l`, `delta_dub_s` | **통합 → `delta_dub: f64`** (CRB dub-off 대칭) |
| `l_dub_l`, `l_dub_s` | **통합 → `l_dub: f64`** |
| `r_sph` (large-end sphere) | **제거** (D1, rib 없음) |
| `sigma_roller` | 유지 |

**`RacewayProfile`** ([types.rs:121](src-tauri/src/solver/types.rs#L121))

| 필드 | 변경 |
|------|------|
| 전체 | **변경 없음** (raceway profile 은 CRB/TRB 공통) |

**`OperatingConditions`** ([types.rs:443](src-tauri/src/solver/types.rs#L443))

| 필드 | 변경 | 근거 |
|------|------|------|
| `f_x`, `f_y` | 유지 (radial 2 성분) | D5 좌표계 |
| `f_a` | **제거 또는 0 강제** | D4 |
| `m_x` | **유지** (X축 about, 풍력 자중·풍하중의 주된 tilting moment) | D5+D6 |
| `m_y` | **제거 또는 0 강제** (single-plane misalignment) | D6 |
| `gamma` (external misalignment) | **유지** (X축 about, [bearing.rs:63](src-tauri/src/solver/bearing.rs#L63) 주석과 일치) | D6 |
| `n_inner_rpm`, `n_outer_rpm`, `t_op` | 유지 | — |
| Lubrication 관련 (40여 필드, p. 462~582) | **변경 없음** | Phase 7 에서 검토 |
| `preload_mode`, `delta_preload_um` | **제거** (axial preload — CRB 무관) | D4 |
| `skf_trb_series` (`SkfTrbSeriesEnum`) | **제거** (TRB 전용 SKF 시리즈) | Phase 7 에서 friction model 의 SKF 옵션도 검토 |

#### 1.4 `bearing.ts` (TypeScript mirror) 동기화 원칙

Rust `types.rs` 의 모든 struct 를 TS interface 로 1:1 mirror. Phase 1 의 모든 필드 변경을 그대로 반영. 자동화 도구 없이 수동 동기화 — Rust serde 의 snake_case ↔ TS camelCase 변환 규칙 확인 필요 (`#[serde(rename_all)]` 없음 → snake_case 유지).

#### 1.5 `defaults.ts` 변경

TRB 의 `defaultInput` 을 단일 row CRB 표준값으로 교체. 후보:
- **NU 240** (200 × 360 × 58 mm, Z=18 roller, D_we=44 mm 정도) — 시리즈 무관 (D2), 단일 row, F_a=0
- 풍력 메인베어링 reference 는 multi-row 라 F4 후속 결정 사항 (현 Phase 1 에서는 선택 안 함)

#### 1.6 `presets.rs` 변경

- 기존 TRB preset (e.g. 30206, 30306) 모두 제거
- CRB preset 자리만 마련 (`ensure_default_preset` 의 기본값을 1.5 의 NU 240 으로)
- 사용자 정의 preset 저장 형식 (`.trb.json` → `.crb.json`) 변경 검토 — 단, file extension 변경은 backward-compat 영향 있음, 작업 마지막에 결정

#### 1.7 통과 기준 (Phase 1 종료 조건)

- [ ] `cargo check` exit 0 (warning 만 허용)
- [ ] `npm run build` exit 0 (TypeScript 타입 에러 0)
- [ ] `npm run tauri dev` 로 WebView 윈도우 정상 팝업 (UI 일부 깨짐 허용 — Phase 6 에서 정리)
- [ ] 솔버 호출은 **수치 정합성 무관** — Tauri command 가 panic 없이 호출 가능 + 결과 객체 반환 (값은 stub OK)

#### 1.8 예상 시간

- **1.5 ~ 2 day** (Plan §3 의 2~3 day 에서 D2 단순화로 -0.5 day)
- 세부: types.rs 변경 (4h) + 컴파일 에러 stub 처리 (4~6h) + TS 동기화 (3h) + defaults/presets (2h) + 검증 (1~2h)

#### 1.9 잠재 이슈 / 대응

| 이슈 | 대응 |
|------|------|
| 솔버 모듈 다수 (bearing.rs, gen1.rs, gen3.rs, life.rs, rib_contact.rs 등) 가 제거된 필드를 참조 → **컴파일 에러 다수** | 임시 stub (`todo!()`, `unimplemented!("CRB Phase 2+")`) 로 우선 컴파일 통과. 본격 수정은 Phase 2~5 에서 |
| TypeScript mirror 누락 → `npm run build` 실패 | Rust 컴파일 후 TS 일괄 비교 (diff 도구 사용 권장) |
| Preset 저장/로드 backward-compat 깨짐 | `.trb.json` 파일 로드 시 경고 + 기본값 fallback. 새 저장은 `.crb.json` |
| TRB Manual 코드 인용이 더 이상 일치 안 함 | Phase 1 에서는 Manual 수정 안 함 (Phase 8 에서 일괄 갱신, F3 결정 대기) |
| Frontend 컴포넌트가 제거된 필드를 참조 → UI 일부 깨짐 | Phase 6 까지 허용 — Phase 1 은 컴파일만 우선 |

#### 1.10 검증 절차 (순서)

1. `types.rs` 수정 (struct 단위로 점진적, 각 단계 cargo check)
2. 컴파일 에러 발생 솔버 모듈을 stub 처리 (panic 메시지에 "Phase 2+" 명시)
3. `cargo check` 통과 확인
4. `src/types/bearing.ts` 동기화
5. `src/defaults.ts` + `src-tauri/src/presets.rs` 갱신
6. `npm run build` 통과 확인
7. `npm run tauri dev` 로 WebView 팝업 확인
8. Action.md 에 결과 기록 (6 소절 템플릿)

#### 1.11 산출물

- 수정 파일 5개 (위 §1.2)
- 임시 stub 파일 (Phase 2~5 에서 본격 구현 대상)
- [CRB_Development_Action.md](CRB_Development_Action.md) 의 Phase 1 섹션 (완료 시 append)

#### 1.12 Phase 2 진입 조건

- Phase 1 통과 기준 (§1.7) 모두 만족
- 본 Phase 에서 발견된 추가 결정 사항 (있다면) Plan §6 갱신

---

### Phase 2 — Geometry 단순화 (상세 계획 초안, 2026-08-19)

> Phase 1.3-B 에서 `geometry.rs` 를 CRB 로 최소 수정 완료 (α=0 대체, d_we 단일화, dub 대칭).
> Phase 2 는 이를 **정식 CRB 알고리즘** 으로 재작성 + **Level A 검증** 추가.

#### 2.1 목표
- `geometry.rs` 를 CRB 정식 (임시 α=0 대체 코드 정리, 순수 원통 로직으로)
- ISO 16281 **B.5 (Cylindrical & needle, p. 29)** 정식 profile 적용
- **Level A 검증**: 단일 slice Hertz vs 해석해 (Roark/Palmgren) 상대오차 < 0.1%
- x_axial → x_axis 변수명 정리 (부록 부분 A.2 반영, 선택)

#### 2.2 작업 대상 파일

| 파일 | 변경 강도 | 주요 작업 |
|------|---------|---------|
| `src-tauri/src/solver/geometry.rs` | 🟡 중간 | Phase 1 임시 α=0 대체 제거, ISO B.5 정식 적용 |
| `src-tauri/src/solver/hertz.rs` | 🟢 검토만 | line contact 공식 유지 확인 |
| `src-tauri/src/solver/types.rs` | 🟢 선택적 | `x_axial` → `x_axis` 명명 (부록 A 정합) |
| `src-tauri/tests/geometry_level_a.rs` | 🔴 **신규** | Level A 해석해 비교 골든 테스트 |

#### 2.3 ISO 16281 B.5 formula 정식 적용

원통 roller profile 참조식 (ISO p. 29):
```
Δz(x) = A_prof · ln[ 1 / (1 - (2·(x - L_we/2) / L_we)²) ]
```
- Reusner 로그 프로파일과 동일 형태
- `A_prof` = user 입력 (delta_c 로부터 유도)
- Dub-off: 양 끝 마지막 L_dub 구간에서 추가 drop (부록 A.1)

#### 2.4 등가 곡률반경 (CRB 단순형)
```
R_eq_inner = (D_we/2) · r_i / (D_we/2 + r_i)     // α=0 → γ_i = 0
R_eq_outer = (D_we/2) · r_o / (D_we/2 + r_o)     // 원통 raceway 라 r_i, r_o ≈ ∞ → R_eq ≈ D_we/2
```
Phase 1.3-B 의 `gamma_i = d_k · cos(α_i) / d_pw` 는 α_i = 0 → `gamma_i = d_k / d_pw` (표준 CRB 회전-반경 γ 정의와 일치).

#### 2.5 Level A 검증 (단위 테스트, 신규)

**Setup**:
- Single slice, flat profile (crown=0, dub=0)
- D_we = 20 mm, L_we = 15 mm, δ = 1 μm
- E' = 210 GPa, ν = 0.3

**Expected (Palmgren line contact 해석해)**:
- Q_expected = C · δ^(10/9), where C = f(E', L_we, D_we)
- p_max_expected = f(Q, R_eq, L_we, E')

**허용 오차**: |Q_calc − Q_expected| / Q_expected < **0.1%**

#### 2.6 통과 기준

- [ ] `cargo test --test geometry_level_a` 모두 pass
- [ ] `cargo check` warning 만 (10~33 개, TRB 잔재)
- [ ] `npm run build` 유지 통과
- [ ] Phase 1 종료 시 상태 대비 회귀 없음

#### 2.7 예상 시간

- **1 ~ 2 day** (Plan §3 예측과 동일)
- 세부: geometry.rs 재작성 (4h) + Level A 테스트 작성 (3h) + 해석해 계산 및 튜닝 (2~4h) + 문서 (1h)

#### 2.8 잠재 이슈 / 대응

| 이슈 | 대응 |
|------|------|
| `x_axial` 변수명 변경 시 다른 파일 (bearing, gen1) 도 참조 | Phase 2 에서는 **명명 변경 보류** (Phase 3 로 이관). 지금은 로직만 정리 |
| ISO B.5 formula 의 A_prof 파라미터화 방식 | 원문 재확인 후 결정. Reusner 로그 프로파일 (Phase 1 default) 은 등가 |
| r_i / r_o 무한대 근사 시 수치 오버플로우 | `r_i > 1e6` 이면 원통으로 판정 → R_eq = D_we/2 로 단순화 |
| Level A 해석해와의 미세 오차 (< 0.5% 지만 > 0.1%) | 슬라이스 폭 미세 조정 or Weber bulk 항 재확인 |

#### 2.9 검증 절차

1. `geometry.rs` 재작성 (α=0 대체 코드 → 순수 원통 로직)
2. `cargo check` 통과
3. Level A 테스트 파일 신규 작성 (`tests/geometry_level_a.rs`)
4. `cargo test --test geometry_level_a` 통과
5. `cargo check` + `npm run build` 회귀 확인
6. `Action.md` Phase 2 섹션 append

#### 2.10 산출물

- 정식 CRB `geometry.rs`
- `tests/geometry_level_a.rs` (Level A 골든)
- `Action.md` Phase 2 섹션 (6 소절)
- (선택) 부록 D — Level A~E 검증 방법론 정리

#### 2.11 Phase 3 진입 조건

- Phase 2 통과 기준 (§2.6) 모두 만족
- Level A 검증 결과 문서화
- `gen1.rs` / `gen3.rs` 가 사용하는 SliceGeometry 인터페이스 안정성 확인

### Phase 3 — Roller-Level Solver (Gen1/Gen3) (상세 계획, 2026-08-19)

> Phase 1.3-B 에서 `gen1.rs / gen3.rs / beam.rs` 는 Phase 1 stub 상태 (cos_alpha_diff=1.0 인자만 넘김) 로 유지.
> Phase 3 는 **최소 정식화** (재작성 최소) + **Level C 진짜 독립 검증** (Gen1 O(n) 독립 vs Gen3 beam-coupled O(n²), 서로 다른 알고리즘 → 동어반복 아님).

#### 3.1 목표
- 3 모듈 (gen1/gen3/beam) 을 CRB 맥락으로 최소 정식화 (주석·변수명·CRB 명시)
- `beam.rs` 의 EI 균일화 유효성 검증 (원통 = D_we 균일 → I = π/4·r⁴ = const, 이미 함수 시그니처 지원)
- **Level C 검증**: 사용자 결정 = **진짜 독립 비교** — Gen1 (독립 slice O(n)) vs Gen3 (beam-coupled Newton-Raphson O(n²)) 는 서로 다른 알고리즘이므로 동어반복 회피
- 검증 조건 = **flat profile + zero misalignment** → 이론적으로 두 결과 수렴 예상

#### 3.2 작업 대상 파일

| 파일 | 변경 강도 | 주요 작업 |
|------|---------|---------|
| `src-tauri/src/solver/gen1.rs` | 🟢 최소 | CRB 명시 주석, `_cos_alpha_diff` 파라미터 유효성 (α=0 → 1.0) 문서화. 함수 시그니처 유지 |
| `src-tauri/src/solver/gen3.rs` | 🟢 최소 | 위와 동일 |
| `src-tauri/src/solver/beam.rs` | 🟡 검토 | `beam_section_properties(r_roller)` 는 이미 원통 대응 가능. I = const 유효성 재확인 |
| `src-tauri/tests/roller_level_c.rs` | 🔴 **신규** | Level C: Gen1 ↔ Gen3 flat profile 수렴 검증 |

⚠️ **재작성 최소 방침 (사용자 결정)** — Gen1/Gen3 알고리즘 자체 재작성 X. Phase 1 이래 유지되는 로직이 CRB 에서도 유효 (α, β 인자만 0 처리) 함을 검증만.

#### 3.3 Gen1 vs Gen3 알고리즘 차이 (독립 검증 근거)

| 항목 | Gen1 (Independent Slice) | Gen3 (Beam-Coupled) |
|------|-------------------------|-------------------|
| Slice interaction | **없음** (각 slice 독립 비선형 스프링) | Timoshenko beam + Hertz spring **coupling** |
| Roller bending | **무시** | 완전 고려 (EI, GA_s) |
| 계산 복잡도 | **O(n)** per roller | O(n²) matrix solve (Newton-Raphson + active set) |
| δ_k 결정 | `δ_k = δ_rigid − Δz_k` (직접) | `[K_beam]{w} + f_contact(δ) = F_ext` 풀이 |
| 접촉력 관계 | Palmgren: `q_k = C·δ_k^(10/9)` | 동일 Hertz, 단 δ 는 beam FE 로부터 |

**핵심**: 두 알고리즘이 **서로 다른 수학적 경로** 로 같은 문제 해결 → flat profile + 강체 roller 가정 시 두 결과가 이론적으로 일치해야 함 → **진짜 독립 검증**.

#### 3.4 CRB 단순화 요점

| 항목 | TRB (Phase 1 이전) | CRB (현재) |
|------|-------------------|-----------|
| α (contact angle) | ≠ 0 (10~30°) | **0** — `cos_alpha_diff = 1.0` |
| β (roller taper) | half cone angle | **0** — 원통 |
| I_k (단면 관성) | 위치 종속 (r_k = r_small + (r_large-r_small)·k/n) | **균일** (r_k = D_we/2 = const → I = π·r⁴/4 = const) |
| beam element stiffness | 요소별 다름 (E·I_k 변동) | **모든 요소 동일 E·I** → 계산 대칭성 개선 |

#### 3.5 Level C 검증 설계 (진짜 독립 비교)

**Setup**:
- NU 240 지오메트리 (Phase 2 재사용)
- Flat profile: `crown_type = Parabolic { c2: 0.0 }`, `delta_dub = 0`, `l_dub = 0`
- `n_slices = 30`
- 여러 δ_rigid 값에서 Gen1/Gen3 실행:
  - δ_rigid ∈ {5, 10, 20, 50, 100} μm

**비교 지표**:
1. **Q_total** (roller 전체 하중) — 두 알고리즘 상대오차 < **1%** (Level C 통과 기준)
2. **q_k 분포** (slice 별 line load) — L2 norm 오차 < **2%**
3. **beam deflection w_k** (Gen3 only) — flat profile 이면 매우 작아야 (< 0.1 μm)

**이론적 근거**: Flat profile + 강체 roller 가정 시 Gen3 의 beam 항 `[K_beam]{w}` 는 rigid body mode 만 존재 → Gen1 결과와 수렴. 만약 크게 다르면 알고리즘 구현 실수 발견.

#### 3.6 통과 기준

- [ ] `cargo check --lib` exit 0 (warnings 만)
- [ ] `cargo test --test roller_level_c` 모두 pass
  - `q_total` 상대오차 < 1% (Gen1 ↔ Gen3, 5개 δ 조건)
  - `q_k` 분포 L2 오차 < 2%
- [ ] `cargo test --test geometry_level_a` 회귀 확인 (3 pass 유지)
- [ ] Phase 1/2 통과 상태 대비 회귀 없음

#### 3.7 예상 시간

- **1 ~ 2 day** (사용자 결정 = 재작성 최소이므로 검증 중심)
- 세부: gen1/gen3/beam 검토 + 주석 (2h) + Level C 테스트 작성 (3~4h) + 실행·조정 (2~3h) + Python 리포트 (2h) + Action.md (1h)

#### 3.8 잠재 이슈 / 대응

| 이슈 | 대응 |
|------|------|
| Gen1 ↔ Gen3 결과 오차 > 1% | (a) beam.rs I=const 미적용 확인, (b) rigid body mode 제거 로직 확인, (c) NR convergence tolerance 조정 |
| beam.rs 가 실제로 tapered 로직 남아 있음 | `beam_section_properties` 호출부 확인. `r_roller` 인자를 D_we/2 (const) 로 통일 |
| Level C 테스트가 오래 걸림 | δ 조건 축소 or n_slices 축소. Phase 3 통과만이 목적 |
| Gen3 가 stub 인 하위 모듈 (life 등) 참조 | Phase 5+ 대상이므로 Gen3 자체는 stub 아님. 확인 |

#### 3.9 검증 절차

1. gen1/gen3/beam 코드 검토 + CRB 명시 주석 추가 (변경 최소)
2. `beam_section_properties(D_we/2)` 로 I=const 확인 (단위 테스트 or 인라인 검증)
3. `cargo check` 통과 확인
4. `tests/roller_level_c.rs` 신규 작성 (Level C 5 δ 조건)
5. `cargo test --test roller_level_c` 실행 → 통과 확인
6. `python-prototype/phase3_level_c_report.py` 신규 (Gen1/Gen3 결과 비교 시각화)
7. Action.md Phase 3 섹션 append (6 소절 + 상세 표 + PNG)

#### 3.10 산출물

- (선택적) 갱신된 `gen1.rs / gen3.rs / beam.rs` (주석·CRB 명시)
- `tests/roller_level_c.rs` (Level C 골든 테스트)
- `python-prototype/phase3_level_c_report.py` (Gen1↔Gen3 비교 시각화)
- `reports/phase3/*.png` (Q 비교 bar, q_k 분포, beam deflection 등)
- `Action.md` Phase 3 섹션 (6 소절 + 상세 표 + PNG 임베드)

#### 3.11 Phase 4 진입 조건

- ✅ Phase 3 통과 기준 (§3.6) 모두 만족
- ✅ Level C 검증 결과 문서화 (Q, q_k 비교 표)
- ✅ Phase 4 (`bearing.rs` 3-DOF 재작성) 시 사용할 안정된 gen1/gen3 인터페이스 확보

### Phase 4 — Bearing-Level Equilibrium (3-DOF) (상세 계획, 2026-08-19 병렬 작성)

> Phase 1.3-B 에서 `bearing.rs` 는 **통째 stub** 상태 (`solve_bearing_equilibrium`, `solve_bearing_dual` → `Err("Phase 4 stub")`).
> Phase 4 는 **CRB 3-DOF 완전 재작성** — Phase 중 가장 큰 작업. ISO 16281 A.3.1 알고리즘 구현.

#### 4.1 목표
- `bearing.rs` 를 CRB 정식 알고리즘으로 **완전 재작성** (Phase 1 stub 대체)
- ISO 16281 **A.3.1 (ISO p. 22)** — Cylindrical roller bearing equilibrium
- **3-DOF 평형**: (δx, δy, γx) — D4+D6+D7 반영
- 좌표계: X=수평, Y=수직(중력), Z=shaft (D5)
- Single row (D3)
- Rib contact 미포함 (D1)
- **Level D 검증**: MASTA / Bearinx / MESYS 결과와 비교 (< 5% 오차)

#### 4.2 작업 대상 파일

| 파일 | 변경 강도 | 주요 작업 |
|------|---------|---------|
| `src-tauri/src/solver/bearing.rs` | 🔴 **전면 재작성** | Phase 1 stub → 3-DOF Newton-Raphson 평형 solver, dual mode (Gen1/Gen3) |
| `src-tauri/src/solver/types.rs` | 🟢 검토만 | `BearingEquilibrium.displacement` 필드 = `[f64; 5]` (TRB 잔재) → `[f64; 3]` 변경 필요 시 |
| `src-tauri/src/commands.rs` | 🟢 무변경 | `solve_bearing`, `solve_bearing_dual` command 시그니처 그대로 |
| `src-tauri/tests/bearing_level_d.rs` | 🔴 **신규** | Level D 검증 (Reference 값 하드코딩) |
| `src-tauri/tests/bearing_smoke.rs` | 🔴 **신규** | Smoke test (수렴/부호/기본 특성) |

#### 4.3 ISO 16281 A.3.1 알고리즘 (CRB Bearing Equilibrium)

**입력**: F_x, F_y, M_x, γ_ext (5 성분 중 CRB 는 3개 유효 — F_a=0, M_y=0)
**미지수**: δ_x, δ_y, γ_x (3 DOF)
**평형식 (3개)**:
```
Σⱼ Q_j · cos ψ_j                            = F_x            (radial X)
Σⱼ Q_j · sin ψ_j                            = F_y            (radial Y)
Σⱼ Q_j · (d_pw/2) · sin ψ_j                 = M_x            (tilting about X)
```

여기서 Q_j = roller j 의 총 normal load (slice 합계):
```
Q_j = Σ_k q_{j,k} · l_k    (slice sum)
q_{j,k} = f(δ_{j,k}, R_eq_k, E*, l_k)   (Palmgren line contact, Phase 3 검증)
δ_{j,k} = δ_rigid_j(ψ_j, δx, δy, γx) - Δz_total_k    (Gen1) or Gen3 beam FE
```

**Roller 접근량** (Phase 1.3-B `roller_approach` 재사용):
```
δ_rigid_j = δ_x · cos ψ_j + δ_y · sin ψ_j
            + (d_pw/2) · (γx + γ_ext) · sin ψ_j · 1000
            - g_r / 2
```

**Newton-Raphson**: [J]·Δ{δ} = -residual, 수렴 시 완료

#### 4.4 CRB 3-DOF 명세 (D5+D6+D7)

| DOF | 물리 | 코드 표기 | 범위 (예) |
|-----|------|---------|---------|
| δ_x | 수평 radial 변위 [μm] | `disp[0]` | -50 ~ 50 |
| δ_y | 수직 radial 변위 [μm] (중력 방향) | `disp[1]` | -100 ~ 100 |
| γ_x | X축 about misalignment [rad] | `disp[2]` | ±10 arcmin |

Phase 1 stub 의 `roller_approach(disp: &[f64; 3], ...)` 시그니처 (이미 3-DOF) 재사용.

#### 4.5 Level D 검증 설계

**진짜 독립 검증 (Phase 2 반성 반영)**:

**Option D1 — Reference 도서 값**:
- Harris & Kotzalas 5th ed. Ch 7 (Bearing internal load distribution) 예제
- Palmgren *Ball and Roller Bearing Engineering* (1959)
- Jones (1946) — 원 저작

**Option D2 — Commercial software 비교**:
- MASTA (사용자 접근 가능, KIMM 라이센스 확인 필요)
- MESYS bearing calc
- Bearinx (Schaeffler)
- SKF SimPro

**Option D3 — ISO 16281 Annex 예제** (있다면):
- ISO 문서 자체 검증 예제 (있는지 확인 필요)

**통과 기준**: Q_max, δ_max 등 주요 지표 상대오차 < **5%** (Plan §5.2 Level D)

**Fallback**: 정성 검증 (부호, 대칭성, load zone extent) — Reference 값 확보 어려울 시

#### 4.6 통과 기준

- [ ] `cargo check --lib` exit 0
- [ ] `cargo test --test bearing_smoke` 모두 pass (수렴/부호/기본 특성)
- [ ] `cargo test --test bearing_level_d` 모두 pass (Reference or 정성)
- [ ] `cargo test --test roller_level_c` 회귀 확인
- [ ] `cargo test --test geometry_level_a` 회귀 확인
- [ ] `npm run tauri dev` 로 solve_bearing command 호출 시 결과 반환 (Phase 1 stub → 실제 결과)
- [ ] **각 DOF 별 non-trivial condition test 통과** (2026-08-19 추가):
  - δ_x DOF: F_x ≠ 0 조건 → δ_x ≠ 0 확인
  - δ_y DOF: F_y ≠ 0 조건 → δ_y ≠ 0 확인
  - γ_x DOF: M_x ≠ 0 조건 → γ_x ≠ 0 확인, 부호 반전 대칭, 단조성, self-consistency (rel_err < 5%)
  - **원칙**: 각 DOF 를 침묵실패 없이 실측 검증. 모든 test 가 특정 DOF=0 조건이면 그 DOF path 는 검증 안 된 것.

#### 4.7 예상 시간

- **3 ~ 5 day** (Plan §3 예측 3 day 대비 여유. 재작성 규모 큼)
- 세부:
  - bearing.rs 전면 재작성 (12~15h)
  - Smoke test 작성 (3h)
  - Level D 검증 (Reference 값 확보 시간 별도) (4~6h)
  - Python 리포트 (Q_j polar, δ 등) (3h)
  - Action.md 상세 (2h)

#### 4.8 잠재 이슈 / 대응

| 이슈 | 대응 |
|------|------|
| **BearingResult 타입 필드 (5-DOF 잔재)** | `displacement: [f64; 5]` 유지 (backward-compat) 하되 δz, γy 는 항상 0 |
| Newton-Raphson 수렴 실패 (jacobian singular) | Levenberg-Marquardt fallback, 초기값 조정 |
| Reference 값 확보 어려움 | Fallback: 정성 검증 (부호/대칭성/load zone) + 사용자 확인 |
| solve_bearing 이 부수 모듈 (life, static_rating) 참조 | 그 모듈들이 disable 상태 → Result 에 default 값 넣거나 Option 처리 |
| BearingResult 의 f_a_effective_kn 등 axial 필드 | 항상 0 채워서 반환 (JSON 호환) |
| **가장 큰 리스크**: 좌표계 부호 실수 | Smoke test 로 조기 발견 (F_y=-1000 → δ_y 부호, Q_j 최대 위치 ψ_j 확인) |

#### 4.9 검증 절차

1. `bearing.rs` 재작성 — `solve_bearing_equilibrium` (Gen1 based)
2. Smoke test (`tests/bearing_smoke.rs`) — 부호/수렴/기본
3. `cargo test --test bearing_smoke` 통과
4. `solve_bearing_dual` 구현 (Gen1 + Gen3 비교)
5. Level D reference 값 확보 → `tests/bearing_level_d.rs`
6. `cargo test --test bearing_level_d` 통과
7. 회귀 확인 (Phase 2, 3 tests)
8. Python 리포트 (Q_j polar plot, 3-DOF δ 시각화)
9. Action.md Phase 4 섹션 (11 소절 상세)

#### 4.10 산출물

- 재작성된 `bearing.rs` (~500~800 라인 예상)
- `tests/bearing_smoke.rs` (신규)
- `tests/bearing_level_d.rs` (신규, Reference 값)
- `python-prototype/phase4_level_d_report.py` (신규)
- `reports/phase4/*.png` (Q_j polar, δ 3-DOF, load zone, Gen1↔Gen3 비교)
- `Action.md` Phase 4 섹션 (6 소절 + 상세 표 + PNG)

#### 4.11 Phase 5 진입 조건

- ✅ Phase 4 통과 기준 (§4.6) 모두 만족
- ✅ `solve_bearing` command 가 실제 결과 반환 (Frontend 에서 호출 가능)
- ✅ Level D 검증 문서화
- ✅ Phase 5 (Life/Static Rating) 재활성화 시 사용할 `BearingResult` 인터페이스 안정

---

#### 4.12 다음 세션 재개 계획 — TRB 원본 참고 완전 재작성 (2026-08-19 WIP 후속)

> **배경**: 2026-08-19 세션에서 Phase 4-A (bearing.rs 재작성) + 4-B (Smoke test) 착수했으나 3-DOF 통합 NR 이 수렴 실패 (`4c853a4` WIP commit).
> 사용자 결정: TRB-main 원본 로직 참고하여 완전 재작성.

##### 4.12.1 세션 재개 절차

```powershell
cd d:\AI\AI_Seminar_CRB
git checkout phase-4          # WIP 브랜치로 이동
git log --oneline -5          # 4c853a4 확인

# TRB 원본 bearing.rs 재추출 (git 히스토리에서)
git show 5441446:CRB-main/src-tauri/src/solver/bearing.rs `
  > $env:TEMP\trb_bearing_original.rs
# → 4113 라인, TRB 5-DOF NR 원본

# 현재 CRB bearing.rs (WIP, 430 라인) 검토
Get-Content CRB-main\src-tauri\src\solver\bearing.rs | Measure-Object -Line

# TRB 원본 핵심 라인: 278 (initial_guess), 740 (solve_bearing_equilibrium),
#                    92 (compute_residual), 17 (solve_3x3)
```

##### 4.12.2 현재 (2026-08-19 WIP) 실패 원인 진단

| 관측 | 원인 | 대응 방향 |
|------|------|---------|
| Smoke: `pure_gravity` residual = **892 kN** (F_y=-1000 의 89% 미달) | NR 100 iter 후 stuck. 실제 필요 δ_y ≈ -100 μm 인데 initial guess -50 μm 에서 진행 못 함 | Phase 분리 방식 채택 시 iteration 수 줄어들며 자연스러운 수렴 |
| Smoke: `pure_fx` δ_y = **-95 μm** (F_y=0 인데 non-zero) | 3-DOF 통합 NR 의 **γ_x DOF 가 F_y residual 로 잘못 흘러들어감** (Jacobian coupling) | γ_x 를 outer loop 로 분리 → radial 2-DOF NR 은 γ_x=0 조건에서 수행 |
| Zero load 만 pass | initial guess = 0 이 정답이라 iteration 불필요 | — |

**결정적 통찰**: `γ_x` 는 `M_x = Σ Q_j·(d_pw/2)·sin ψ_j` 만 지배. `F_y = Σ Q_j·sin ψ_j` 와 **sin ψ_j 공통** 이므로 3-DOF Jacobian 에서 δ_y 와 γ_x 열이 거의 linearly dependent → Jacobian 조건수 나쁨 → NR 발산.

##### 4.12.3 TRB 원본 알고리즘 (참고 상세)

**파일**: `git show 5441446:CRB-main/src-tauri/src/solver/bearing.rs` (4113 라인, 5-DOF)

**핵심 구조**:

| 함수 | 라인 | 역할 |
|------|------|------|
| `solve_3x3` | 17~55 | 3×3 linear solve (Cramer's rule) |
| `compute_residual` | 92~240 | 5-DOF residual [Fx, Fy, Fa, Mx, My] |
| `initial_guess` | 278~319 | k_radial = Z/2 · 500 · cos α 기반 (clamp ±50 μm) |
| `compute_induced_thrust` | 332~349 | TRB 특유 axial 반력 (α ≠ 0) |
| `solve_preload_displacement` | 620~730 | dz 초기값 target F_a 로부터 |
| **`solve_bearing_equilibrium`** | **740~1260** | **핵심: Phase A (radial 2×2 또는 3×3) + Phase B (tilting) 분리** |
| `solve_bearing_equilibrium_5dof` | 1263~2250 | 완전 5-DOF 통합 (fallback) |
| `solve_bearing_dual` | 2258~ | Gen1 + Gen3 비교 |

**Phase A 핵심 로직** (라인 892~945, dz fixed case = **CRB 에 직접 대응**):
```rust
// 2×2 NR for (δx, δy), δz 고정
let f_radial = f_r.max(1.0);
for _outer in 0..max_iter {
    let (residual, _) = compute_residual(input, &slices, &disp, 0.0)?;
    let r_radial = (residual[0].powi(2) + residual[1].powi(2)).sqrt();
    if r_radial / f_radial < tol { break; }

    // 2×2 Jacobian (forward FD, h_s = 0.01 μm)
    let mut j2 = [[0.0_f64; 2]; 2];
    for col in 0..2 {
        let mut dp = disp;
        dp[col] += h_s;
        let (rp, _) = compute_residual(input, &slices, &dp, 0.0)?;
        for row in 0..2 { j2[row][col] = (rp[row] - residual[row]) / h_s; }
    }

    // Cramer's rule (2×2)
    let det = j2[0][0] * j2[1][1] - j2[0][1] * j2[1][0];
    let dx_step = (j2[1][1] * (-residual[0]) - j2[0][1] * (-residual[1])) / det;
    let dy_step = (j2[0][0] * (-residual[1]) - j2[1][0] * (-residual[0])) / det;

    // Step clamp (max_step = disp_mag * 0.5, clamp 5~30 μm)
    let step_norm = (dx_step.powi(2) + dy_step.powi(2)).sqrt();
    let scale = if step_norm > max_step { max_step / step_norm } else { 1.0 };
    let dd = [dx_step * scale, dy_step * scale];

    // Line search: 20회 반감, best_alpha 유지
    let mut alpha_ls = 1.0;
    let mut best_alpha = 0.0;
    let mut best_norm = r_radial;
    for _ in 0..20 {
        let mut d_trial = disp;
        d_trial[0] += alpha_ls * dd[0];
        d_trial[1] += alpha_ls * dd[1];
        let (rt, _) = compute_residual(input, &slices, &d_trial, 0.0)?;
        let rt_r = (rt[0].powi(2) + rt[1].powi(2)).sqrt();
        if rt_r < best_norm { best_norm = rt_r; best_alpha = alpha_ls; }
        if rt_r < r_radial { break; }
        alpha_ls *= 0.5;
    }
    if best_alpha < 1e-15 { best_alpha = alpha_ls; }
    disp[0] += best_alpha * dd[0];
    disp[1] += best_alpha * dd[1];
}
```

##### 4.12.4 TRB → CRB 변환 매핑 표

| TRB 5-DOF | CRB 3-DOF | 변환 방식 | 근거 |
|-----------|----------|---------|------|
| `disp[0]` = δx | `disp[0]` = δx | 동일 | D5 좌표계 |
| `disp[1]` = δy | `disp[1]` = δy | 동일 | D5 |
| `disp[2]` = δz | 제거 (=0) | axial 지지 없음 | D4 |
| `disp[3]` = γx | `disp[2]` = γx | 배열 위치 변경 | D6 |
| `disp[4]` = γy | 제거 (=0) | single-plane misalignment | D6 |
| `f_a_input * 1000` | `0.0` 강제 | axial 하중 무시 | D4 |
| `preload_mode / delta_preload_um` | 제거 | axial preload 무관 | D4 |
| `compute_induced_thrust(f_r, α_o)` | 제거 | α=0 → sin α = 0 → thrust 없음 | D1 |
| `alpha_rad = raceway_geom.alpha_o.to_radians()` | `0.0` 상수 | 원통 raceway | D1 |
| `cos α, sin α` | `1.0, 0.0` | α=0 | — |
| `cos_alpha_diff = cos(α_o - α_i)` | `1.0` | 이미 Phase 1.3-B 로 반영 | — |
| Phase A dz_free 분기 | 항상 dz fixed=0 | δz free 케이스 제거 | D4 |
| Phase A **2×2 NR** (δx, δy) | **동일 유지** — 핵심 재활용 | γ_x 는 outer loop | 4.12.5 |
| `solve_preload_displacement` | 제거 | axial 없음 | D4 |
| M_x, M_y residual | **M_x 만** (γ_x 담당) | γ_y=0 (D6) | — |
| rib_contact 참조 | 제거 | D1 | 완료 (Phase 1.3-B) |

##### 4.12.5 재작성 전략 — Phase 분리 방식 (핵심)

**Outer loop**: γ_x 1-DOF NR
```
for outer_iter in 0..max_iter:
    Phase A: (δx, δy) 2-DOF NR  (γ_x 고정, TRB 원본 그대로 이식)
        수렴 시 M_x residual 계산
    if |M_x_residual| / M_x_ref < tol: break
    γ_x update (1-DOF NR):
        dMdγ = numerical FD (M_x with γ_x + dγ)
        Δγ_x = -M_x_residual / dMdγ
        line search on γ_x
```

**이유**:
- Phase A 는 TRB 원본과 동일 (검증된 로직)
- γ_x 는 별도 축이라 coupling 없음
- outer loop 는 보통 3~5 iter 로 수렴 (M_x 는 δ 에 약한 종속)

**대안**: 3×3 통합 NR 로 하되:
- Jacobian pivoting (QR decomposition)
- Levenberg-Marquardt damping
- 그러나 Phase 분리가 더 단순 + 안정

##### 4.12.6 Rust 코드 skeleton (예상)

```rust
pub fn solve_bearing_equilibrium(
    input: &BearingInput,
    progress: &dyn ProgressReporter,
) -> Result<BearingResult, SolverError> {
    let slices = compute_slices(...)?;
    let mut disp = initial_guess_crb(input);   // [δx, δy, γx] 3-DOF

    let m_x_target = input.operating.m_x * 1e6;  // N·mm
    let m_x_ref = m_x_target.abs().max(1e3);

    // Outer loop: γ_x (M_x equilibrium)
    for outer in 0..input.solver.max_iterations {
        // Phase A: 2-DOF NR (δx, δy), γ_x 고정
        phase_a_radial_2dof(&mut disp, input, &slices)?;

        // M_x residual 계산
        let (r_all, _) = compute_residual_3d(input, &slices, &disp)?;
        if (r_all[2] / m_x_ref).abs() < input.solver.convergence_tol { break; }

        // γ_x update (1-DOF NR)
        let h_g = 1e-6_f64;   // rad
        let mut disp_p = disp;
        disp_p[2] += h_g;
        let (r_p, _) = compute_residual_3d(input, &slices, &disp_p)?;
        let dmdg = (r_p[2] - r_all[2]) / h_g;
        if dmdg.abs() > 1e-30 {
            disp[2] += -r_all[2] / dmdg * 0.5;  // damping 0.5
        }
    }

    // 최종 결과 구축 (BearingResult, default 필드 채움)
    build_bearing_result(input, &slices, disp, ...)
}

fn phase_a_radial_2dof(disp: &mut [f64; 3], input: &BearingInput, slices: &[SliceGeometry])
    -> Result<(), SolverError>
{
    // TRB 원본 line 892~945 로직 그대로 이식 (α=0, dz=0 강제)
    // 2×2 NR, max_step clamp 5~30 μm, line search 20회 best_alpha
}
```

##### 4.12.7 Level D 검증 참조값

**Sjovall integral** (Harris & Kotzalas 5th ed. Ch 7, Eq 7.71):
```
Q_max / F_r = 1 / (Z · J_r(ε))
```
- ε = 0.5 (zero clearance, radial only): `J_r(0.5) = 0.2453`
- Zero clearance NU 240 (Z=18), F_y=-1000 kN:
  - **Q_max = 1000 · 1000 / (18 · 0.2453) = 226,455 N ≈ 226.5 kN**
- Load zone extent (2·ψ_lim):
  - ε=0.5 → 정확히 180°
  - ε<0.5 (preload) → > 180°
  - ε>0.5 (clearance) → < 180°

**Clearance 조건** (g_r > 0):
- ε = 0.5 · (1 − g_r / (2·δ_max))
- δ_max ≈ 100 μm, g_r = 30 μm → ε ≈ 0.425 → J_r(0.425) ≈ 0.21 (Harris 표)
- Q_max ≈ 1000·1000 / (18·0.21) ≈ 264.6 kN

**Level D 통과 기준**: 상대오차 < **5%** (Plan §5.2 Level D)

**추가 Reference (정성)**:
- F_x = 0, F_y = -F_r 조건: **δ_x = 0** (대칭성), γ_x = 0 (M_x=0 조건)
- F_x = F_r, F_y = 0: **δ_y = 0** (대칭성)
- Load zone 하부 (F_y < 0 이면 ψ ≈ -90° 부근) 에 Q_max 집중

##### 4.12.8 예상 소요 시간 (다음 세션)

| 단계 | 시간 |
|------|------|
| Session 재개 + TRB 원본 재추출 | 5분 |
| bearing.rs Phase 분리 구조 재작성 | 25분 |
| Smoke test 통과 확인 (4~5 조건) | 10분 |
| Level D test (Harris/Sjovall 참조값) | 15분 |
| Python 리포트 + Action.md | 15분 |
| Commit + merge 사전 질문 | 5분 |
| **합계** | **약 75분** |

이 정도 시간이면 새 세션의 context 여유로 완주 가능. `4c853a4` WIP 위에 이어 작업.

### Phase 5 — Life / Static Rating (상세 계획, 2026-08-19 병렬 작성)

> Phase 1.3-B 에서 `life.rs`, `static_rating.rs` 는 `mod.rs` 에서 disable 상태.
> Phase 5 는 두 모듈 재활성화 + ISO 16281 5.3 (lamina-level life) + ISO 76 (static rating) + ISO 281 CRB 상수 (C_r).

#### 5.1 목표
- `life.rs` 재활성화 — ISO 16281 5.3 (Cylindrical roller lamina-level life)
- `static_rating.rs` 재활성화 — ISO 76:2006 CRB (C_0r, P_0r, S_0)
- ISO 17956:2025 (lamina-level effective static safety S_0,eff) 지원
- **Level E 검증**: 정성 (부호/monotonicity) + 가능 시 Reference

#### 5.2 작업 대상 파일

| 파일 | 변경 강도 | 주요 작업 |
|------|---------|---------|
| `src-tauri/src/solver/life.rs` | 🔴 재작성 | ISO 16281 5.3 lamina-level. CRB C_r 상수 (ISO 281 CRB 식) |
| `src-tauri/src/solver/static_rating.rs` | 🔴 재작성 | ISO 76 CRB C_0r + ISO 17956 S_0,eff |
| `src-tauri/src/solver/mod.rs` | 🟢 minor | 두 모듈 disable 주석 해제 |
| `src-tauri/src/solver/bearing.rs` | 🟡 부분 | Phase 4 의 stub 필드 (life/static_rating) 실제 계산 값으로 대체 |
| `src-tauri/tests/life_level_a.rs` | 🔴 신규 | Life 단위 테스트 (해석해 비교) |
| `src-tauri/tests/static_rating_level_a.rs` | 🔴 신규 | Static rating 단위 테스트 |

#### 5.3 ISO 16281 5.3 (CRB Roller Bearings Lamina-Level Life)

**per-slice equivalent lamina load** (ISO Eq. 24~25):
```
q_ei,k = (Σⱼ (q_j,k · cos ψⱼ)^{10/3})^{3/10}   (inner ring, 회전 side)
q_eo,k =  Σⱼ q_j,k · cos ψⱼ / Z                (outer ring, 정지 side, 근사)
```

**per-lamina life** (Lundberg-Palmgren):
```
L_10,k = (Q_c,k / q_e,k)^{10/3}    [10⁶ rev]
```

**Bearing life 합성** (Weibull, e = 9/8 for roller):
```
1/L_10^e = Σ_k (1/L_10,k)^e   → L_10 = (Σ_k L_10,k^{-e})^{-1/e}
```

**Modified life**:
```
L_nm = a_ISO · L_10   (a_ISO = f(κ, η_c, C_u/P))
```

#### 5.4 ISO 281 CRB C_r 계수

```
C_r = b_m · f_c · (L_we · cos α)^{7/9} · Z^{3/4} · D_we^{29/27}
    = b_m · f_c · L_we^{7/9} · Z^{3/4} · D_we^{29/27}    (CRB: α=0)
```
- `b_m` = 1.1 (CRB 표준, ISO 281)
- `f_c` = 형상 계수 (γ = D_we/D_pw 함수, ISO 281 표)

#### 5.5 ISO 76 CRB Static Rating

```
C_0r = f_0 · Z · L_we · D_we · cos α = f_0 · Z · L_we · D_we    (α=0)
```
- `f_0` = 44 (CRB 표준, ISO 76:2006)

**S_0 = C_0r / P_0r** (P_0r 는 정적 등가 하중, CRB 는 P_0r = F_r)

#### 5.6 ISO 17956 lamina-level S_0,eff

```
q_0 = C_0r × (some factor)  (per-lamina reference)
q_max = actual maximum lamina load (from Phase 4 equilibrium)
S_0,eff = q_0 / q_max
```

#### 5.7 통과 기준

- [ ] `cargo check --lib` exit 0
- [ ] `cargo test --test life_level_a` all pass
- [ ] `cargo test --test static_rating_level_a` all pass
- [ ] `bearing.rs` 의 life/static_rating 필드가 실제 계산 값 (Default 대체)
- [ ] Phase 4 회귀 확인
- [ ] **각 계산 경로 별 non-trivial condition test** (§4.6 침묵실패 방지 원칙 적용): 예. L_10 계산은 실제 하중분포 (Q_j ≠ uniform) 조건 사용, S_0 는 Q_max 지점 (특정 roller 지목) 조건.

#### 5.8 예상 시간

- **2 ~ 3 day**
- 세부: life.rs 재작성 (5h), static_rating.rs 재작성 (3h), tests (4h), 통합 (2h), 리포트 (2h)

#### 5.9 잠재 이슈

| 이슈 | 대응 |
|------|------|
| ISO 281 f_c 계수 표 (γ 함수) 접근 | ISO 원문 확인 or 문헌 (Harris 부록 A) 하드코딩 |
| ISO 16281 Eq. 24~25 의 회전 side 판별 (내륜/외륜 어느 것이 회전) | operating.n_inner_rpm, n_outer_rpm 로 판별 (Phase 4 이미 반영) |
| bearing.rs 재수정 (Phase 4 stub 필드 → 실제 계산) | 인터페이스 최소 변경 |
| Level E (실험 검증) 불가 | 정성 검증 (monotonicity: F_r ↑ → L_10 ↓) 로 대체 |

#### 5.10 산출물

- 재작성된 `life.rs`, `static_rating.rs`
- `tests/life_level_a.rs`, `tests/static_rating_level_a.rs`
- `python-prototype/phase5_life_report.py`
- `reports/phase5/*.png` (L_10 vs F_r, C_r/P 곡선 등)
- `Action.md` Phase 5 섹션

#### 5.11 Phase 6 진입 조건

- ✅ Phase 5 통과 기준 (§5.7) 모두 만족
- ✅ `solve_bearing` 이 life/static_rating 실제 값 반환
- ✅ Frontend (Phase 6) 에서 이 값 표시 가능한 인터페이스 확정

진입 시점에 상세화. 핵심: ISO 16281 5.3 + ISO 281 C_R (CRB) + ISO 76 C_0r.

### Phase 6 — Frontend UI 변경 (상세 계획, 2026-08-19 병렬 작성)

> Phase 1.4 에서 13 개 Frontend 컴포넌트에 `// @ts-nocheck` 임시 지시자 추가.
> Phase 6 는 이 stub 을 실제 CRB UI 로 정식 재작성 + 원통 3D 렌더링.

#### 6.1 목표
- Phase 1.4 의 `@ts-nocheck` 13 파일을 CRB 데이터 모델 (types/bearing.ts, defaults.ts) 에 맞게 정식 재작성
- BearingView3D: TRB 원추 → CRB 원통 렌더링 (Three.js `CylinderGeometry`)
- InputPanel: α/β/D_we_max/min/rib 필드 UI 제거, CRB 필드만 노출
- GeometryView, SectionView2D: 원통 단면도
- ResultsCard, LifeChart, LoadDistChart: Phase 4~5 결과 표시
- End-to-end 동작 확인 (`npm run tauri dev` → 완전 렌더링 + solve → 결과 표시)

#### 6.2 작업 대상 파일 (13 개 @ts-nocheck + 신규)

| 컴포넌트 | 변경 강도 | 주요 작업 |
|----------|---------|---------|
| `BearingView3D/index.tsx` | 🔴 대규모 | TRB `LatheGeometry` → CRB `CylinderGeometry`, α=0 → roller z축 정렬, 원통 raceway |
| `GeometryView/index.tsx` | 🔴 대규모 | α_i/α_o 표기 제거, D_we 단일, dub 대칭 |
| `SectionView2D/index.tsx` | 🔴 대규모 | 원통 단면 (TRB 사다리꼴 → 직사각형) |
| `InputPanel/index.tsx` | 🔴 대규모 | α/D_we_max/min/rib UI 제거, CRB 필드 group 재구성 |
| `ProfileView/index.tsx` | 🟡 중간 | dub-off 대칭 표기 |
| `ResultsCard/index.tsx` | 🟡 중간 | preload_mode/f_a 필드 UI 제거 |
| `LubricationView/index.tsx` | 🟡 중간 | (Phase 7 재활성화 대상, 지금은 minor) |
| `TransientView/TransientTimeChart.tsx` | 🟢 minor | LoadTimePoint f_a/m_y 컬럼 제거 |
| `LifeChart.tsx`, `LoadDistChart.tsx` | 🟡 중간 | f_a 참조 제거 |
| `RibContactDetailChart.tsx` | 🟢 삭제 or 감춤 | D1: rib 없음 → 컴포넌트 자체 제거 or feature flag |
| `RollerComparisonChart.tsx`, `RollerDetailChart.tsx` | 🟢 minor | TRB 잔재 unused var 정리 |

#### 6.3 BearingView3D 원통 렌더링 상세

**TRB 방식** (Phase 0 원본):
```typescript
// LatheGeometry (2D profile revolve → 3D 원추/사다리꼴)
const innerPts = [new THREE.Vector2(rBore, -halfT), ...];
const innerRingGeo = new THREE.LatheGeometry(innerPts, 64, 0, Math.PI * 2);
```

**CRB 방식** (Phase 6 재작성):
```typescript
// 원통 = 단순 CylinderGeometry (radius uniform)
const rollerGeo = new THREE.CylinderGeometry(
  D_we / 2,   // radiusTop
  D_we / 2,   // radiusBottom (uniform = 원통)
  L_we,       // height
  32          // radialSegments
);
// 회전 축을 shaft 축 (Z) 으로 맞춤: RotateX(-π/2)
```

#### 6.4 InputPanel 구조

**제거**: α, β, D_we_max, D_we_min, h_rib, α_rib, R_sph, dub_l/s, f_a, m_y, preload_mode

**유지 그룹**:
- Macro Geometry: d, D, T, Z, **D_we (단일)**, L_we, D_pw, G_r
- Raceway Geometry: r_i, r_o, d_uc, l_uc
- Roller Profile: crown_type, δ_c, **δ_dub (대칭)**, **L_dub (대칭)**, sigma_roller
- Operating: F_x, F_y, **M_x (만)**, γ, n_rpm, T_op, ν_40, ν_100 (F_a, M_y 제거)
- Material, Lubrication, Solver 는 그대로

#### 6.5 통과 기준

- [ ] `npm run build` 0 TS errors (모든 @ts-nocheck 제거)
- [ ] `npm run tauri dev` WebView 창 정상 팝업 + **UI 완전 렌더링** (Phase 1 의 blank 문제 해결)
- [ ] Solve 실행 시 결과 (Q_j polar, load distribution) 정상 표시
- [ ] Phase 4~5 결과 (life, static rating) 표시 정확

#### 6.6 예상 시간

- **4 ~ 5 day** (13 파일 대규모 재작성 + Three.js 원통 지오메트리)
- 세부: BearingView3D (8h) + GeometryView/SectionView2D (6h) + InputPanel (5h) + 나머지 8개 (6h) + 통합 테스트 (4h) + Action.md (2h)

#### 6.7 잠재 이슈 / 대응

| 이슈 | 대응 |
|------|------|
| Three.js LatheGeometry vs CylinderGeometry 스케일링 차이 | 원통은 훨씬 단순 — 오히려 코드 축소 |
| InputPanel 필드 제거 시 preset JSON 하위호환 | serde default 로 이미 처리, TS 는 optional 필드 |
| Rib chart 완전 삭제 vs feature flag | 완전 삭제 (D1: 영구 제거) |
| Frontend 컴포넌트 간 shared type 참조 오류 | bearing.ts 단일 source of truth |
| 3D 렌더링 성능 (100+ roller × 30+ slice) | Three.js instancing 검토 (Phase 6 후반) |

#### 6.8 검증 절차

1. `@ts-nocheck` 제거 (파일별) → tsc 에러 확인 → 필드 참조 수정
2. `npm run build` 점진적 통과
3. `npm run tauri dev` → WebView 팝업 + UI 렌더링 육안 확인
4. NU 240 default 로 `Solve Bearing` 클릭 → 결과 표시 확인
5. `Save Project` / `Load Project` (.crb.json) 동작 확인
6. `Action.md` Phase 6 섹션 (스크린샷 포함)

#### 6.9 산출물

- 13 개 컴포넌트 재작성 (TS 정식)
- (선택) 신규 컴포넌트: `CylinderView` etc.
- `reports/phase6/*.png` (Before/After UI 스크린샷)
- `Action.md` Phase 6 섹션

#### 6.10 Phase 7 진입 조건

- ✅ Phase 6 통과 기준 (§6.5) 모두 만족
- ✅ End-to-end UI 정상 동작
- ✅ Frontend 가 Phase 7 (Lubrication/Transient) 재활성화 시 준비 완료

#### 6.11 우선순위 로드맵

1. **Priority A** (핵심): BearingView3D + InputPanel — 이 둘만으로도 사용자 경험 크게 개선
2. **Priority B** (중요): GeometryView + SectionView2D + ResultsCard
3. **Priority C** (부수): 5개 chart 컴포넌트 minor 수정

진입 시점에 상세화. 핵심: InputPanel 필드 정리, BearingView3D 원통 렌더링.

### Phase 7 — Lubrication / Transient / HMEHL (placeholder)

진입 시점에 상세화. 핵심: kinematic 식만 CRB 형태 (cage speed, slip velocity).

### Phase 8 — 검증 + 문서화 (placeholder)

진입 시점에 상세화. 핵심: Manual 재작성 (F3 결정 후), ISO 검증 예제, MESYS/MASTA 비교 보고서.

---

## 8. 참고 문헌

| 문헌 | 위치 | 주요 활용 |
|------|------|----------|
| ISO 16281:2025 | `TRB-main/Reference/ISO_16281_2025(en).pdf` (md: `ISO_16281_2025.md`) | CRB 알고리즘 본체 (5.3, 6.3.2, A.3.1, B.5) |
| ISO 281:2007 | `TRB-main/Reference/ISO_281_2007.md` | C_R (basic dynamic load rating) |
| ISO 76:2006 | `TRB-main/Reference/ISO_76_2006.md` | Static load rating C_0r |
| ISO TR 1281-1/2 | `TRB-main/Reference/ISO_TR_1281-*.md` | Calculation background |
| Yan 2025 (CRB Dynamic Model) | `TRB-main/Reference/2025_Yan_et_al_Dynamic_Model_CRB.md` | CRB flexible roller dynamics 참고 |
| TRB-main `CLAUDE.md`, `Master_plan.md` | `TRB-main/` | 아키텍처 / 인터페이스 청사진 |

---

## 부록 A — Roller Profile 용어 해설 (Dub-off, End sphere R_sph)

### A.1 Dub-off (덥-오프) — 양 끝 추가 drop

**정의**: Roller profile 양 끝의 마지막 구간 L_dub 에서 crown profile 위에 **추가로 부여하는 큰 폭 drop δ_dub**. Crown 만으로 완화되지 않는 끝부분의 edge stress spike 를 누르기 위해 사용.

**기하 도식**:
```
        ←──────── L_we (effective length) ────────→
        ┌──────────────────────────────────────┐
        │ \                                  /  │
crown   │  \____ (crown profile, 가운데) ___/   │
        │ ↘                                 ↙  │  ← dub-off (마지막 L_dub)
        ↑     ↑                          ↑    ↑
       δ_dub  L_dub                    L_dub δ_dub
```

**TRB 의 비대칭**:
- 대단(large end) ↔ 소단(small end) 직경 다름 → 응력 분포 본질적 비대칭
- 대단은 rib 접촉 + 큰 axial 분력 → **더 큰** dub-off 필요
- 소단은 sliding 위주 → 작은 dub-off
- TRB-main 의 4개 필드: `delta_dub_l`, `l_dub_l`, `delta_dub_s`, `l_dub_s`
- ISO 16281 **B.6 (p. 29)** 와 일치

**CRB 의 대칭**:
- 양 끝 직경 동일 (D_we 균일) → 응력 분포 좌우 대칭
- Rib 접촉 없음 (D1) → 한쪽에 추가 부담 없음
- **단일 (δ_dub, L_dub)** 로 양쪽 동일 적용
- ISO 16281 **B.5 (p. 29)** 와 일치

### A.2 End sphere R_sph (대단 구면 반경)

**정의**: TRB roller 의 대단면 끝 단면 형상이 **평면이 아닌 구면(sphere)** 으로 가공된 것. 그 구면이 내륜 rib 면과 만나서 **Hertzian point contact (타원 접촉)** 을 형성.

**TRB 에서 필수인 이유**:
- TRB 의 raceway 접촉력 sin α 분력 = axial 방향 → roller end 에서 rib 으로 전달 → rib 이 받쳐줘야 평형
- Roller end 가 평면이라면 rib 과 line contact → 응력 무한대 (실용 불가)
- 구면 R_sph → rib 평면과 점접촉 (타원) → 유한한 Hertz stress
- R_sph 가 클수록 접촉 면적 ↑ 응력 ↓, 그러나 rib 마찰 ↑ — 절충

**CRB 에서 무관 (제거)**:
- ISO 16281 **A.3.1 NOTE 1 (p. 23)**: *"for typical load cases, the consideration of axial rib loads for cylindrical roller bearings is **not necessary**"*
- N/NU: rib axial 지지 자체 없음
- NJ/NUP: small axial 가능하지만 본 SW 범위에서 무시 (D1)
- → R_sph 필드 의미 없음 → **제거**

### A.3 CRB vs TRB 비교 요약

| 항목 | TRB | CRB (본 SW) | ISO 16281 |
|------|------|------------|-----------|
| Dub-off 필드 수 | 4 (`delta_dub_l/s`, `l_dub_l/s`) | **2** (`delta_dub`, `l_dub`, 양쪽 동일) | B.6 → **B.5** |
| End sphere R_sph | 필수 | **제거** | A.3.2 → **A.3.1 NOTE 1** |
| 양 끝 대칭성 | 비대칭 | **완전 대칭** | — |
| Roller end 평면 가공 | 불가 (구면 필수) | **가능** (rib 미접촉) | — |

---

## 부록 B — 좌표계 & 모멘트 축 결정 분석

### B.1 TRB-main 좌표계 (코드 기준)

| 축 | 의미 | 코드 근거 |
|----|------|----------|
| **X** | Horizontal radial | `f_x` (radial X). Roller `ψ=0` → +X |
| **Y** | **Vertical radial = 중력 방향** | `f_y` (radial Y). [BearingView3D](src/components/BearingView3D/index.tsx) 의 `[px, py, 0]` 배치 + Three.js Y-up 카메라 컨벤션 |
| **Z** | Bearing axis (shaft 회전축) | `f_a` (axial). Roller cylinder 가 Z 따라 회전 |

**Roller angular position ψ_j** ([bearing.rs:78](src-tauri/src/solver/bearing.rs#L78)):
- `δ_r = δx · cos(ψ) + δy · sin(ψ)`
- ψ = 0 → +X (수평 우)
- ψ = π/2 → +Y (위)
- ψ = -π/2 → -Y (아래 = **중력 방향**)

### B.2 ISO 16281 A.3.1 의 single-plane misalignment 가정

**Formula (A.10)** (ISO p. 23):
```
θ_j = arctan(tan θ · cos ψ_j)
```
- 한 평면 안에서의 misalignment θ 만 다룸
- ISO 의 `cos ψ_j` 항 ↔ TRB-main 의 `γx · sin ψ - γy · cos ψ` 항에서 **X축 about γ_x** 가 standard form

[bearing.rs:63](src-tauri/src/solver/bearing.rs#L63) 주석: *"gamma_ext: external misalignment [rad] (imposed **about x-axis**)"* — 코드가 이미 X축 about 으로 설계됨.

### B.3 풍력 메인베어링의 물리적 하중 시나리오

- Shaft: Z 축 수평
- 자중 + 풍하중: bearing 중심에서 떨어진 거리에서 **-Y 방향** force
- 이 force × shaft span = **X축에 대한 tilting moment M_x** (γ_x)
- 좌우 비대칭 하중 거의 없음 → **M_y ≈ 0**

### B.4 결정 (Plan §6.3 D5~D7 반영)

| 결정 ID | 결정 |
|---------|------|
| D5 | 좌표계 TRB-main 그대로 — Y = 중력 |
| D6 | **M_x (γ_x) 만 사용**, M_y = γ_y = 0 강제 — 풍력 메인베어링의 자연스러운 single-plane |
| D7 | **평형 DOF = 3: (δx, δy, γx)** — δz=0 (D4), γy=0 (D6) |

### B.5 Future flag

- 향후 풍력 외 응용 (수직 shaft, 좌우 비대칭 로터 등) 추가 시 **D6 해제** 필요
- 그때는 (δx, δy, γx, γy) 4-DOF 로 확장 가능 — bearing.rs 의 residual 식 [183~188](src-tauri/src/solver/bearing.rs#L183-L188) 은 이미 일반화되어 있어 큰 변경 없음

---

## 부록 C — SW 개발 워크플로우 안내 (변경 관리 · Git · 롤백)

> **목적**: Phase 1 이후 본격 코드 변경 시, 실수 방지·이전 상태 복구·여러 시도 병행을 위한 기본 SW 개발 테크닉 안내.
> **독자 가정**: Git 경험이 적은 사용자.
> **범위**: CRB-main 프로젝트 맥락에서 즉시 적용 가능한 핵심만.

### C.1 왜 이런 기법이 필요한가

코드를 직접 수정하기 시작하면 다음 위험들이 발생합니다:

| 위험 | 예시 |
|------|------|
| **실수로 동작하던 코드 망가뜨림** | Phase 2 작업 중 Phase 1 의 잘된 부분까지 깨뜨림 |
| **이전 상태로 돌아갈 수 없음** | "어제 잘 됐는데..." 라며 1주일 작업 폐기 |
| **여러 시도를 병행 불가** | 알고리즘 A vs B 둘 다 시험하고 싶은데 한 번에 하나만 가능 |
| **누가 무엇을 왜 바꿨는지 추적 불가** | 한 달 뒤 본인이 봐도 의도를 모름 |
| **두 PC 간 동기화 불가** | 사무실 PC ↔ 노트북 작업 분리 어려움 |

이 5가지 모두를 한 번에 해결하는 표준 도구가 **Git** + 선택적으로 **GitHub** 입니다.

### C.2 Git 기본 개념 (3분 요약)

```
┌────────────────────┐    git add     ┌────────────────────┐    git commit    ┌────────────────────┐
│  Working Directory │ ──────────────▶│   Staging Area     │ ────────────────▶│   Local Repository │
│  (실제 편집 중인    │                │   (다음 커밋에 포함  │                  │   (영구 보존 + 이력) │
│   파일들)          │                │    할 변경 후보)    │                  │                    │
└────────────────────┘                └────────────────────┘                  └──────────┬─────────┘
                                                                                          │
                                                                                  git push │ git pull
                                                                                          ▼
                                                                              ┌────────────────────┐
                                                                              │ Remote (GitHub)    │
                                                                              │ (백업 + 협업)      │
                                                                              └────────────────────┘
```

핵심 단어 5개:
- **commit**: "지금 이 상태를 영구 보존" 명령. 메시지 함께 기록.
- **branch**: 코드의 평행 우주. main 에서 분기해 자유롭게 시험, 검증 후 합치거나 버림.
- **diff**: 어제 ↔ 오늘 변경된 내용 비교.
- **revert / reset**: 이전 commit 상태로 되돌리기 (두 방식 차이는 §C.5 참조).
- **stash**: 작업 중 임시 보관 (commit 만들지 않고 잠시 치워두기).

### C.3 본 프로젝트 Git 초기화 (1회만)

CRB-main 은 현재 Git 저장소가 아닙니다 (Phase 0 복제 시 `.git` 제외). 다음 명령으로 초기화:

```powershell
cd "d:\AI\Main_Bearing\CRB-main"

# 1) Git 저장소 초기화
git init

# 2) .gitignore 작성 — 추적 제외 대상 (C.4 참조)
# (별도 파일로 작성)

# 3) 사용자 정보 설정 (1회, 전역)
git config --global user.name "Your Name"
git config --global user.email "drivetrain001@gmail.com"

# 4) 현재 상태 (Phase 0 완료 상태) 를 첫 commit
git add .
git commit -m "Phase 0: TRB-main 복제 + 환경 분리 완료

- TRB-main → CRB-main 전체 구조 복제 (node_modules/target 제외)
- 식별자 변경 (crb-app, crb-contact-analysis, port 5175)
- 문서 헤더 TRB→CRB
- npm install + cargo check 통과
"
```

### C.4 `.gitignore` 권장 내용 (CRB-main 용)

```gitignore
# 빌드 산출물
node_modules/
src-tauri/target/
dist/

# IDE 설정
.vscode/
.idea/
*.swp

# OS 파일
.DS_Store
Thumbs.db

# 로컬 환경 (PC별로 다른 경로)
src-tauri/.cargo/config.toml
src-tauri/.cargo/config.toml.bak
# → 사용자 PC 별 VS 경로가 다름. config.toml 은 git 추적 제외 권장.
#   대신 config.toml.template 를 만들어 추적

# 임시 / 백업 파일
*.bak
*.tmp
*~

# 사용자 프로젝트 파일 (선택)
*.crb.json
```

### C.5 변경 관리 4가지 핵심 기법 비교

| 기법 | 명령 예시 | 언제 쓰나 | 위험도 |
|------|----------|----------|------|
| **백업 파일 (.bak)** | `cp foo.rs foo.rs.bak` | 1~2분짜리 즉시 비교 (Git 도 안 쓰고 싶을 때) | 🟢 낮음 (단 Git이 더 우월) |
| **별도 파일 새로 만들기** | `gen1.rs` 옆에 `gen1_v2.rs` 생성 | 알고리즘 A/B 병행 시험 (둘 다 컴파일하고 호출부에서 선택) | 🟢 낮음 |
| **Branch** | `git checkout -b try-newton` | "큰 변경" 시험 (실패 시 통째로 버리기 좋음) | 🟢 낮음 (강추) |
| **Stash** | `git stash` / `git stash pop` | 작업 중 다른 일 잠깐 처리 (커밋 만들기 애매한 변경 임시 보관) | 🟡 중간 (`stash drop` 시 영구 손실) |

**롤백 3가지** (작업 되돌리기):

| 명령 | 효과 | 비고 |
|------|------|------|
| `git checkout -- foo.rs` | foo.rs 만 직전 commit 상태로 복구 (working dir 변경 폐기) | **로컬 미커밋 변경 폐기** |
| `git revert <commit>` | 그 commit 의 변경을 **취소하는 새 commit 추가** (이력 보존) | **공유된 이력 안전 롤백** (권장) |
| `git reset --hard <commit>` | 그 commit 이후 모든 commit 삭제 (이력 자체 제거) | ⚠️ **위험** — push 한 후엔 쓰지 마세요 |

### C.6 GitHub 연동 (선택, 강력 권장)

GitHub 에 백업하면: (a) PC 고장 대비, (b) 두 PC 간 동기화, (c) 협업 가능.

```powershell
# 1) GitHub 에 빈 repo 생성 (웹에서 또는 gh CLI 로)
gh repo create CRB-main --private --source=. --remote=origin

# 2) 첫 push
git push -u origin main

# 이후 push
git push
```

> 본 시스템에는 GitHub CLI (`gh`) 가 이미 설치되어 있음 (`C:\Program Files\GitHub CLI`). `gh auth login` 으로 1회 인증.

### C.7 본 프로젝트 권장 워크플로우 (Phase 1 진입 시 적용)

#### C.7.1 Phase 단위 브랜치 전략

```
main ────●─────────────────────●─────────────●─────────────●──── ...
         │                       ▲             ▲             ▲
         │ (Phase 0 완료)         │ (merge)      │ (merge)      │
         │                       │             │             │
         └──┬── phase-1 ─────────●             │             │
            └── phase-2 ─────────────────────●               │
            └── phase-3 ───────────────────────────────────●
```

- `main`: 통과 검증된 안정 상태 (각 Phase 완료 후 merge)
- `phase-1`, `phase-2`, ...: 진행 중 작업 브랜치 (자유롭게 시험·실패·재시도)

명령 예시:
```powershell
# Phase 1 시작
git checkout -b phase-1

# 작업 → commit (자주, 작게)
git add src-tauri/src/solver/types.rs
git commit -m "Phase 1.1: MacroGeometry — α/D_we_max/D_we_min/rib 필드 제거"

# ... (여러 commit 누적)

# Phase 1 완료 → 검증 통과 → main 으로 merge
git checkout main
git merge phase-1
git push                  # GitHub 사용 시
```

#### C.7.2 Commit message 컨벤션 (제안)

```
<Phase 번호>: <한 줄 요약 — 무엇을 왜 변경했는지>

<선택: 본문 — 상세 변경 항목, 검증 결과, 관련 결정 ID>
```

예시:
```
Phase 1.3: OperatingConditions f_a/m_y/preload 필드 제거

- 결정 D4 (F_a=0) + D6 (γ_y=0) 반영
- skf_trb_series 도 제거 (TRB 전용)
- cargo check 통과, npm run build 통과
```

#### C.7.3 시험적 변경 시 별도 파일 vs 브랜치

| 상황 | 권장 방식 |
|------|----------|
| "혹시 모르니 이전 버전을 한 줄 옆에 두고 싶다" | `foo.rs.bak` (또는 commented-out 코드) — 단 일주일 내 정리 |
| "알고리즘 A 와 B 를 둘 다 호출 가능하게 두고 토글로 시험" | `foo_a.rs` + `foo_b.rs` 별도 파일, 호출부에서 분기 |
| "큰 구조 변경을 안전하게 시험" | `git checkout -b try-XXX` 브랜치, 실패 시 `git checkout main && git branch -D try-XXX` |
| "이 부분만 시간 거꾸로 돌리고 싶다" | `git revert <commit>` (해당 변경만 취소하는 새 commit) |
| "작업 중인데 갑자기 다른 일이 들어옴" | `git stash` 로 잠시 치워두기, 다른 일 끝나면 `git stash pop` |

### C.8 즉시 적용 권장 절차 (Phase 1 진입 직전)

다음 5분 작업으로 안전망 구축:

```powershell
cd "d:\AI\Main_Bearing\CRB-main"

# 1. Git 초기화
git init
git config --global user.name "Your Name"      # 1회만
git config --global user.email "your@email"    # 1회만

# 2. .gitignore 작성 (C.4 내용을 .gitignore 파일로 저장)

# 3. Phase 0 상태를 첫 commit
git add .
git commit -m "Phase 0: 환경 분리 완료"

# 4. (선택) GitHub 백업
gh auth login           # 1회만
gh repo create CRB-main --private --source=. --push

# 5. Phase 1 작업 브랜치
git checkout -b phase-1

# 이제 안전하게 작업 가능. 망쳐도 git checkout main 으로 즉시 복구.
```

### C.9 자주 쓰는 명령 한 페이지 요약

```powershell
# 상태 확인
git status                          # 현재 변경 사항 (어느 파일이 수정/추가됐는지)
git diff                            # 구체적 변경 내용 (Working ↔ Staging)
git diff --staged                   # Staging ↔ Last commit
git log --oneline -10               # 최근 10개 commit 한 줄 요약

# 변경 보존
git add <file>                      # 특정 파일을 staging 에 추가
git add .                           # 전체 변경 staging
git commit -m "message"             # commit 생성

# 브랜치
git branch                          # 현재 브랜치 목록
git checkout -b <new-branch>        # 새 브랜치 만들고 이동
git checkout <branch>               # 기존 브랜치로 이동
git merge <branch>                  # 현 브랜치에 다른 브랜치 합치기

# 되돌리기
git checkout -- <file>              # 그 파일만 직전 commit 상태로 (Working dir 변경 폐기)
git reset HEAD <file>               # staging 만 취소 (working dir 보존)
git revert <commit-hash>            # 그 commit 의 변경을 취소하는 새 commit
git stash                           # 임시 보관
git stash pop                       # 보관한 것 꺼내기

# GitHub
git push                            # remote 로 보내기
git pull                            # remote 에서 받아오기
gh repo view --web                  # 브라우저에서 repo 열기
```

### C.10 학습 자료 (자료원)

- 공식 Git Book (한글): https://git-scm.com/book/ko/v2 — Chapter 1~3 만 읽어도 충분
- Atlassian Git Tutorial: https://www.atlassian.com/git/tutorials — 시각적 설명 우수
- GitHub Docs (한글): https://docs.github.com/ko
- 책 추천: 『*프로 Git*』 (Scott Chacon, 무료 공개)

### C.11 본 SW 개발 맥락의 추가 팁

| 상황 | 권장 행동 |
|------|----------|
| Phase N 작업 중 절반쯤 진행됐는데 막힘 | commit 일단 만들기 (`WIP: ...` message). 막힌 부분만 별도 issue 로 기록 후 다른 Phase 로 넘어갈 수도 |
| 솔버 결과 수치가 갑자기 이상함 | `git log` 로 최근 commit 확인 → `git diff <과거_commit>` 으로 원인 추적 → `git bisect` (이진 탐색) 로 어느 commit이 깨뜨렸는지 자동 검색 |
| 사무실 PC ↔ 노트북 작업 분리 | GitHub repo 만들고 양 PC 에서 `git pull` / `git push` 로 동기화. 절대 USB 로 폴더 복사 X |
| AI(Claude) 가 코드 수정한 후 마음에 안 듦 | `git diff` 로 어디를 바꿨는지 확인 → `git checkout -- <file>` 로 즉시 롤백 |
| Phase 8 (문서화) 진입 전에 전 과정 회고하고 싶음 | `git log --oneline --graph --all` 로 전 Phase 의 commit 트리 시각화 |

---

### C.12 기존 GitHub 레포지터리에 CRB-main 올리기 + VS Code 에서 Git 작업

> **상황 가정**: 사용자가 이미 본인의 GitHub 계정에 레포지터리를 가지고 있고, 거기에 CRB-main 을 추가하려는 경우. 그리고 일상 작업은 Visual Studio Code 의 Git 통합 UI 로 진행하고 싶음.

#### C.12.1 사전 확인 — 어떤 시나리오인가 (4가지 중 선택)

| 시나리오 | 설명 | 선택 기준 |
|---------|------|----------|
| **A. 기존 레포가 비어 있거나 CRB 전용** | CRB-main 통째로 main 브랜치에 push | 그 레포가 CRB 프로젝트만을 위한 것 |
| **B. 기존 레포에 다른 프로젝트도 있음 (monorepo)** | CRB-main 을 기존 레포의 **하위 폴더**로 추가 (예: `repo-root/CRB-main/`) | 한 레포에 TRB-main, CRB-main 등 여러 프로젝트 공존 |
| **C. 별도 브랜치로 분리** | 기존 레포의 새 브랜치 (예: `crb-main` 브랜치) 로 CRB-main push | 다른 프로젝트와 완전 분리하되 같은 레포에서 관리 |
| **D. Git worktree + Sparse-checkout (상급자 방식, ⭐ 본 프로젝트 실제 채택)** | 별도 브랜치 (`CRB`) 를 별도 물리 폴더 (`AI_Seminar_CRB/`) 로 worktree, 그 worktree 는 **CRB-main 폴더만** sparse-checkout | 한 monorepo 에서 여러 브랜치를 동시에 편집 + 각 브랜치별 필요 파일만 노출 |

→ 본 안내는 **A** (가장 단순), **B** (가장 실용적), **D** (본 프로젝트 채택) 세 시나리오를 다룸. C 는 §C.7 의 브랜치 전략으로 응용 가능.

#### C.12.2 시나리오 A — 기존 빈 레포에 CRB-main 통째로 push

```powershell
cd "d:\AI\Main_Bearing\CRB-main"

# 1) Git 초기화 (이미 했다면 생략)
git init
git add .
git commit -m "Phase 0: CRB-main 초기 상태"

# 2) 기존 레포 URL 확인 (GitHub 웹에서 "Code" 버튼 → HTTPS 또는 SSH URL 복사)
#    예: https://github.com/<USERNAME>/<REPONAME>.git

# 3) remote 등록
git remote add origin https://github.com/<USERNAME>/<REPONAME>.git

# 4) 기본 브랜치 이름을 main 으로 (GitHub 기본)
git branch -M main

# 5) 첫 push
git push -u origin main
```

⚠️ **기존 레포에 이미 파일이 있다면** `git push` 가 거부됩니다. 그 경우:
```powershell
# 먼저 기존 내용 pull (충돌 가능)
git pull origin main --allow-unrelated-histories
# 충돌 해결 후
git push -u origin main
```
충돌이 복잡하면 시나리오 B 가 더 안전.

#### C.12.3 시나리오 B — 기존 레포의 하위 폴더로 CRB-main 추가 (권장)

가장 안전하고 직관적인 방법. 기존 레포를 로컬에 clone → CRB-main 폴더를 그 안으로 복사 → push.

```powershell
# 1) 작업 임시 위치로 이동 (예: d:\Work)
cd "d:\Work"

# 2) 기존 레포 clone
git clone https://github.com/<USERNAME>/<REPONAME>.git
cd <REPONAME>

# 3) CRB-main 폴더 통째로 복사 (node_modules, target, .git 제외)
robocopy "d:\AI\Main_Bearing\CRB-main" ".\CRB-main" /E /XD node_modules target dist .git

# 4) 상태 확인 → 새 파일 추가 → commit
git status
git add CRB-main/
git commit -m "Add CRB-main project (Phase 0 complete)

- TRB-main 의 SW 체계 복제 기반
- 환경 분리 (식별자, port 5175) 완료
- Phase 1 진입 대기 상태
"

# 5) push
git push
```

이후 작업 위치는 **`d:\Work\<REPONAME>\CRB-main\`** 이 됩니다 (기존 `d:\AI\Main_Bearing\CRB-main\` 은 백업 또는 삭제 결정).

> **권장**: 원본 폴더 `d:\AI\Main_Bearing\CRB-main\` 은 즉시 삭제하지 말고 1~2일 정도 보존 → Git 작업이 안정되면 정리.

#### C.12.4 VS Code Git 통합 — UI 구성

VS Code 는 Git 을 **별도 확장 없이 기본 지원**합니다. 주요 UI:

```
┌────────────────────────────────────────────────────────────────────────┐
│ [≡] 메뉴바                                                              │
├──┬─────────────────────────────────────────────────────────────────────┤
│📁│  Explorer        ┌─────────────────────────────────┐                │
│🔍│  Search          │ 파일 트리                        │   에디터        │
│🔀│  Source Control  │                                 │   영역          │
│🐞│  Run/Debug       │                                 │                │
│⌘ │  Extensions      └─────────────────────────────────┘                │
│  │                                                                     │
├──┴─────────────────────────────────────────────────────────────────────┤
│ [main↑3↓0]   하단 상태바: 현재 브랜치 + 푸시/풀 대기 표시                  │
└────────────────────────────────────────────────────────────────────────┘
```

핵심 UI 위치:
- **좌측 사이드바 🔀 아이콘 (Source Control)** — Ctrl+Shift+G 단축키
- **하단 상태바 좌측** — 현재 브랜치명 + 동기화 상태 (`↑` = push 대기, `↓` = pull 대기)
- **에디터 좌측 게터(gutter)** — 줄별 변경 표시 (▎녹색 = 추가, ▎파랑 = 수정, ▎빨강 = 삭제)

#### C.12.5 VS Code 일상 워크플로우 (UI 클릭만)

##### 1) 폴더 열기
- `File → Open Folder` → CRB-main 폴더 선택 (또는 monorepo 의 경우 레포 루트)
- Git 저장소가 자동 인식되어 하단 상태바에 브랜치명 표시

##### 2) 변경 확인
- 좌측 🔀 (Source Control) 클릭
- "CHANGES" 섹션에 수정된 파일 목록
- 파일 클릭 → **diff viewer** 자동 열림 (좌: 이전, 우: 현재)

##### 3) Staging (commit 후보 선택)
- 각 파일 옆 `+` 버튼 → "STAGED CHANGES" 로 이동
- 전체 staging: 섹션 제목 옆 `+` 버튼

##### 4) Commit
- 상단 메시지 박스에 commit 메시지 입력
- Ctrl+Enter (또는 `✓ Commit` 버튼) 으로 commit 생성
- ⚠️ 메시지 첫 줄은 50자 이내 요약, 본문은 한 줄 비우고 작성 (§C.7.2 컨벤션 참조)

##### 5) Push / Pull (Sync)
- 하단 상태바의 동기화 아이콘 클릭 (↻ 모양) — push + pull 자동 수행
- 또는 `... → Push` / `Pull` 메뉴

##### 6) 브랜치 만들기 / 전환
- 하단 상태바의 브랜치명 클릭
- 상단 명령 팔레트에 옵션 표시:
  - `Create new branch...` — 새 브랜치 생성
  - `Checkout to...` — 기존 브랜치 전환
- 또는 Ctrl+Shift+P → "Git: Checkout to..."

##### 7) 충돌 해결
- Pull 시 충돌 발생하면 파일이 `<<<<<<<` `=======` `>>>>>>>` 마커로 표시됨
- VS Code 가 각 충돌 블록 위에 4개 버튼 제공:
  - **Accept Current Change** (내 것 유지)
  - **Accept Incoming Change** (상대 것 채택)
  - **Accept Both Changes** (둘 다)
  - **Compare Changes** (비교 보기)
- 모든 충돌 해결 후 staging + commit

#### C.12.6 추천 VS Code 확장 (선택, 강력 권장)

| 확장 | 기능 | 설치 명령 |
|------|------|----------|
| **GitLens** | 줄별 git blame, 풍부한 commit 히스토리, 비교 도구 | `code --install-extension eamodio.gitlens` |
| **Git Graph** | commit 트리 시각화 (`git log --graph` 의 GUI 버전) | `code --install-extension mhutchie.git-graph` |
| **Git History** | 파일별 commit 히스토리 우클릭 메뉴 | `code --install-extension donjayamanne.githistory` |
| **GitHub Pull Requests** | VS Code 안에서 PR 열기/리뷰 (GitHub 사용 시) | `code --install-extension GitHub.vscode-pull-request-github` |

설치 방법:
- 좌측 사이드바 ⌘ (Extensions) 클릭
- 검색창에 확장 이름 입력 → "Install" 버튼

#### C.12.7 일상 사이클 예시 (Phase 1 작업 중)

```
[월요일 오전]
1. VS Code 폴더 열기
2. 🔀 사이드바 확인 — 새 변경 없음 (깨끗한 상태)
3. 하단 상태바: "main" → 클릭 → "Create new branch..." → "phase-1" 입력
4. types.rs 편집 (MacroGeometry 의 alpha 필드 제거)
5. cargo check (터미널에서) → 통과
6. 🔀 사이드바 → types.rs 옆 + 버튼 (staging)
7. 메시지 입력: "Phase 1.1: MacroGeometry alpha 필드 제거 (D5)"
8. Ctrl+Enter → commit
9. ↻ 클릭 → push (phase-1 브랜치가 GitHub 에 올라감)

[월요일 오후 — 추가 작업]
10. d_we_max/min → d_we 통합
11. cargo check → 통과
12. 🔀 → staging → commit "Phase 1.1: D_we 통합" → push

[화요일 — Phase 1 완료]
13. cargo check + npm run build 모두 통과
14. 하단 브랜치명 클릭 → "Checkout to..." → "main"
15. 명령 팔레트 (Ctrl+Shift+P) → "Git: Merge Branch..." → "phase-1" 선택
16. push → main 에 Phase 1 통합 완료
17. (선택) phase-1 브랜치 삭제: 명령 팔레트 → "Git: Delete Branch..." → "phase-1"
```

#### C.12.8 자주 발생하는 문제 / 트러블슈팅

| 증상 | 원인 | 해결 |
|------|------|------|
| `Source Control 패널이 비어 있음 / "no source control providers"` | Git 미설치 또는 폴더가 git 저장소 아님 | 터미널에서 `git init` 후 VS Code 재시작 |
| `Push 시 "Updates were rejected"` | Remote 가 로컬보다 앞서 있음 (다른 PC 에서 작업) | 먼저 Pull (↻) → 충돌 해결 → Push |
| `사용자 이름 / 이메일 묻는 메시지` | `git config` 미설정 | `git config --global user.name "..."` + `user.email "..."` |
| `Push 시 인증 실패 (HTTPS)` | Personal Access Token 필요 (2021 이후 GitHub 패스워드 인증 폐지) | GitHub Settings → Developer settings → Personal access tokens 생성 → 첫 push 시 token 을 password 로 입력. 또는 `gh auth login` 으로 자동 처리 |
| `Commit 메시지 입력창이 안 보임` | Source Control 패널이 좁아짐 | 패널 폭 늘리기 또는 Ctrl+Shift+G 로 패널 토글 |
| `대용량 파일 (>100MB) push 실패` | GitHub 의 파일 크기 제한 | `.gitignore` 에 추가 + `git rm --cached <file>` + Git LFS 검토 |
| `한글 commit 메시지 깨짐 (Windows)` | 터미널 인코딩 문제 | `git config --global core.quotepath false` + VS Code 내부 git 사용 (UI 입력은 안전) |
| `Phase 0 의 .cargo/config.toml 이 push 됨 — 다른 PC 에서 동작 안 함` | PC 별 경로 차이 | `.gitignore` 에 `src-tauri/.cargo/config.toml` 추가, `git rm --cached` 로 추적 해제 |

#### C.12.9 GitHub CLI (gh) 와 VS Code 병행 활용

본 시스템에 `gh` CLI 가 설치되어 있어 두 도구를 함께 활용 가능:

```powershell
# 인증 (1회만)
gh auth login

# 브라우저에서 현재 레포 열기
gh repo view --web

# 새 issue 생성 (VS Code 에 없는 기능)
gh issue create --title "Phase 4 평형 알고리즘 수치 발산 이슈" --body "..."

# PR 생성 (VS Code 의 GitHub PR 확장으로도 가능하지만 CLI 가 빠름)
gh pr create --title "Phase 1 완료" --body "D1~D7 반영"
```

#### C.12.10 시나리오 D — Git Worktree + Sparse-checkout (⭐ 본 프로젝트 실제 채택)

##### C.12.10.1 개념

**Git Worktree**: 하나의 저장소를 **여러 물리 폴더에 각기 다른 브랜치로 체크아웃**하는 기능. 브랜치 전환 시 파일 갈아 끼우는 대신 폴더별로 브랜치 격리.

**Sparse-checkout**: 저장소 전체가 아닌 **특정 폴더/파일만** 워킹 디렉토리에 노출. 대형 monorepo 에서 필요한 부분만 다루기 좋음.

**두 기능 조합의 위력**:
```
GitHub Repo (CHOICHOI-KIMM/AI_Seminar)
├── main 브랜치       ─┐
├── CRB 브랜치        ─┤ 세 브랜치 모두 존재
└── P3_HTML 브랜치    ─┘

로컬 파일시스템
├── d:/AI/AI_Seminar/           ← main worktree (전체 파일)
│   ├── CRB-main/               ← main 에도 CRB-main 있음 (초기 상태 기록)
│   ├── Main bearing/
│   └── 논문 취합/
│
└── d:/AI/AI_Seminar_CRB/       ← CRB worktree (sparse-checkout: CRB-main 만)
    └── CRB-main/               ← CRB 브랜치 작업 전용, 다른 폴더 미노출
```

**장점**:
- 두 브랜치를 **동시에 편집** 가능 (컨텍스트 스위칭 없음)
- CRB 작업 폴더에는 **CRB-main 만 보임** → 다른 프로젝트 파일에 방해 안 받음
- 각 worktree 는 독립 IDE 세션 (VS Code 창) 열기 가능

##### C.12.10.2 초기 세팅 (1회)

```powershell
# 1) monorepo 를 어딘가에 이미 clone 했다고 가정
#    예: d:\AI\AI_Seminar (main 브랜치 체크아웃 상태)

cd "d:\AI\AI_Seminar"

# 2) CRB 브랜치가 없다면 먼저 생성
git branch CRB main            # main 에서 CRB 분기
git push -u origin CRB         # 원격에 push

# 3) 별도 물리 폴더로 worktree 생성 (핵심)
git worktree add "d:\AI\AI_Seminar_CRB" CRB

# 4) 새 worktree 로 이동 후 sparse-checkout 설정
cd "d:\AI\AI_Seminar_CRB"
git sparse-checkout init --cone
git sparse-checkout set CRB-main

# → 이제 d:\AI\AI_Seminar_CRB\ 에는 CRB-main 폴더만 보임
```

##### C.12.10.3 일상 작업

```powershell
# CRB 작업 → CRB worktree 에서만
cd "d:\AI\AI_Seminar_CRB\CRB-main"
code .                          # VS Code 로 열기 — CRB-main 만 workspace 로 보임

# 수정 → commit → push (일반 git 명령 그대로)
git add .
git commit -m "Phase 1.1: ..."
git push                        # origin/CRB 로 push
```

##### C.12.10.4 Phase 1 서브 브랜치 (본 프로젝트 채택 형태)

```powershell
cd "d:\AI\AI_Seminar_CRB"       # CRB worktree
git checkout -b phase-1         # CRB 로부터 phase-1 서브 브랜치 생성
# ... 작업 ...
git commit -m "..."
# Phase 1 완료 시:
git checkout CRB
git merge phase-1
git push
```

##### C.12.10.5 다른 브랜치도 worktree 로 (선택)

```powershell
# 각 Phase 나 목적별로 별도 물리 폴더
git worktree add "d:\AI\AI_Seminar_P3" P3_HTML
git worktree add "d:\AI\AI_Seminar_experiment" experiment-branch

# 목록 확인
git worktree list
```

##### C.12.10.6 정리

```powershell
# Worktree 제거 (물리 폴더도 함께 삭제됨)
git worktree remove "d:\AI\AI_Seminar_CRB"

# 오래된 링크 정리
git worktree prune
```

##### C.12.10.7 주의사항

- **같은 브랜치를 두 worktree 에서 동시 체크아웃 불가** — Git 이 잠금 걸어줌. 그러므로 각 worktree 는 서로 다른 브랜치
- **`.git` 은 파일** (worktree 는 폴더가 아니라 파일로 표시) — 지우면 안 됨
- **Sparse-checkout 은 파일 필터**, 브랜치 무관. 원격에는 여전히 모든 파일 있음
- **VS Code 는 각 worktree 를 독립 workspace 로 인식** — 여러 창 동시 편집 가능

#### C.12.11 다중 PC 환경 — 개발도구 재설치 필요성

⚠️ **매우 중요**: Git worktree 는 **파일만 동기화**합니다. **개발도구 (Node.js, Rust, VS 2022 BuildTools)** 는 각 PC 에 **독립 설치** 되어야 합니다.

##### C.12.11.1 시나리오 — 두 PC 병행 작업

| 항목 | PC-A (원 개발 PC) | PC-B (동기화 후) |
|------|-------------------|------------------|
| Git 저장소 | ✅ 완비 | ✅ clone 완료 |
| 소스 파일 | ✅ 최신 | ✅ 최신 (push/pull) |
| **Node.js** | ✅ 설치됨 | ❌ **새로 설치 필요** |
| **Rust toolchain** | ✅ 설치됨 | ❌ **새로 설치 필요** |
| **VS 2022 BuildTools** | ✅ 설치됨 | ❌ **새로 설치 필요** |
| **PATH 환경변수** | ✅ 설정됨 | ❌ **새로 설정 필요** |

##### C.12.11.2 PC-B 에서 필요한 순차 설치

```powershell
# 1) PowerShell 실행정책 (CurrentUser)
Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned -Force

# 2) Node.js LTS (약 3~5분)
winget install --id OpenJS.NodeJS.LTS -e --accept-source-agreements --accept-package-agreements --silent

# 3) Rust toolchain (약 2~3분)
Invoke-WebRequest https://win.rustup.rs/x86_64 -OutFile $env:TEMP\rustup-init.exe
& $env:TEMP\rustup-init.exe -y --default-toolchain stable-msvc --default-host x86_64-pc-windows-msvc --profile default

# 4) VS 2022 BuildTools (약 15~25분, UAC 프롬프트 클릭 필요)
Invoke-WebRequest https://aka.ms/vs/17/release/vs_buildtools.exe -OutFile $env:TEMP\vs_buildtools.exe
Start-Process -FilePath $env:TEMP\vs_buildtools.exe -Wait -ArgumentList @(
  "--passive","--wait","--norestart",
  "--add","Microsoft.VisualStudio.Workload.VCTools",
  "--add","Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
  "--add","Microsoft.VisualStudio.Component.Windows11SDK.22621",
  "--includeRecommended"
)

# 5) 글로벌 cargo config (SSL CRL 우회 — 회사망/연구기관망 필수)
@"
[http]
check-revoke = false

[net]
git-fetch-with-cli = true
"@ | Set-Content "$env:USERPROFILE\.cargo\config.toml" -Encoding utf8

# 6) 검증
cd <프로젝트 폴더>\src-tauri
cargo check    # 첫 실행 시 의존성 컴파일 5~10분
cd ..
npm install    # 약 1분
```

##### C.12.11.3 PC 간 잠재 이슈

| 이슈 | 대응 |
|------|------|
| **MSVC 버전 차이** — 프로젝트 `src-tauri/.cargo/config.toml` 이 하드코딩된 MSVC 버전 (예: `14.44.35207`) | 새 PC 의 실제 설치 버전으로 갱신. `Get-ChildItem "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC"` 로 확인 |
| **Windows SDK 버전 차이** (예: `10.0.26100` vs `10.0.22621`) | 동일하게 config.toml 의 LIB/INCLUDE 경로 갱신 |
| **VS 설치 경로 차이** (`Program Files` vs `Program Files (x86)`, Community/Professional/Enterprise/BuildTools) | config.toml 의 CC/CXX/LIB/INCLUDE/linker 경로 4가지 통째로 갱신 |
| **`.cargo/config.toml` 이 git 추적 중** — PC-A 의 경로가 PC-B 에서 안 맞음 | `.gitignore` 에 `src-tauri/.cargo/config.toml` 추가 + `config.toml.template` 을 대신 git 추적 |
| **User PATH 에 cargo bin 미포함** | `[Environment]::SetEnvironmentVariable("Path", "$([Environment]::GetEnvironmentVariable('Path', 'User'));$env:USERPROFILE\.cargo\bin", "User")` |
| **회사망 SSL 오류 (`CRYPT_E_NO_REVOCATION_CHECK`)** | 5번 단계의 글로벌 config 필수 |

##### C.12.11.4 시간 예산

| 단계 | 소요 시간 |
|------|---------|
| Node.js 설치 | 3~5분 |
| Rust 설치 | 2~3분 |
| VS 2022 BuildTools (다운로드 6GB) | **15~25분** (가장 오래) |
| Cargo config + PATH | 1분 |
| 첫 cargo check (전 의존성 컴파일) | 5~10분 |
| npm install | 1~2분 |
| **총계** | **약 30~50분** |

한 번 세팅하면 이후 소스 push/pull 은 즉시 반영. 재설치는 OS 재설치 시에만.

#### C.12.12 한 페이지 요약 — "VS Code 에서 Git 쓰는 법 5초 안내"

| 키/위치 | 동작 |
|---------|------|
| **Ctrl+Shift+G** | Source Control 패널 열기 |
| **메시지 + Ctrl+Enter** | Commit |
| **하단 ↻ 아이콘** | Push + Pull (Sync) |
| **하단 브랜치명 클릭** | 브랜치 전환/생성 |
| **에디터 좌측 게터** | 줄별 변경 시각화 |
| **파일 우클릭 → "View File History"** | 그 파일의 commit 이력 (GitLens 필요) |

---

*Last updated: 2026-08-19 (Phase 0 완료 + §6 결정 D1~D7 + §7 Phase 1 상세 + 부록 A·B·C(§C.1~C.12) 완성. C.12.10 시나리오 D (git worktree + sparse-checkout) 및 C.12.11 다중 PC 환경 안내 추가. [실행 기록은 CRB_Development_Action.md](CRB_Development_Action.md))*
