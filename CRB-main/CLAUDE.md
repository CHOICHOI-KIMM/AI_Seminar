# CLAUDE.md - CRB Contact Analysis System

> Simple is Best 원칙은 글로벌 CLAUDE.md 참조
>
> **본 프로젝트는 [TRB-main](../TRB-main/) 의 SW 체계를 모태로 신규 개발 중인 CRB(Cylindrical Roller Bearing) 해석 SW 임.**
> 작업 계획서: [CRB_Development_Plan.md](CRB_Development_Plan.md)
>
> 현재 본 CLAUDE.md / Master_plan.md / Manual 내용은 **TRB-main 의 복제본** 상태이며, Phase 1 이후 CRB 기준으로 점진 갱신 예정.

---

## Project Overview

CRB Contact Analysis System — Cylindrical Roller Bearing 내부 접촉 해석 도구. Gen1 (independent slice, 고속)과 Gen3 (Timoshenko beam-coupled, 고정밀) 이중 모드 슬라이싱을 지원하며, 공통 입출력 인터페이스로 결과를 직접 비교 가능.

> ⚠️ **현재 상태(Phase 0 완료)**: 코드 본체는 아직 TRB 알고리즘. Phase 1 (types.rs 단순화) 부터 본격 CRB 화 진행.

## Technology Stack

- **Solver core**: Rust (nalgebra, sprs, rayon, serde)
- **Desktop shell**: Tauri 2.0
- **Frontend**: React + TypeScript (Vite)
- **3D visualization**: Three.js (@react-three/fiber)
- **Charts**: Plotly.js
- **Prototyping**: Python (NumPy/SciPy) — 알고리즘 검증 후 Rust 포팅
- **MASTA bridge**: C# sidecar via Tauri (geometry import, JSON/CSV)

## Development Commands

```bash
npm run dev           # 프론트엔드 개발 서버 (Vite)
npm run build         # TypeScript 체크 + Vite 프로덕션 빌드
npm run lint          # ESLint 실행
npm run tauri dev     # Tauri 개발 모드 (프론트+백엔드)
npm run tauri build   # Tauri 프로덕션 빌드
cargo test            # Rust 솔버 테스트 (src-tauri/ 에서)
```

---

## Architecture

Three-level nested iteration solver:

- **Level 1 (Bearing)**: 5-DOF equilibrium (δx, δy, δz, γx, γy) → per-roller normal force Q_j. 양 모드 공통.
- **Level 2 (Roller)**: Per-roller axial load distribution q_k. **Gen1/Gen3 분기점.**
  - Gen1: `δ_k = δ_rigid(k) - Δz_total_k`, independent nonlinear springs, O(n)
  - Gen3: Timoshenko beam FE `[K_beam]{w} + f_contact(δ) = F_ext`, Newton-Raphson with active set, O(n²)
- **Level 3 (Slice)**: Hertz line contact per slice (b_k, p_max_k, Weber bulk h_k). 양 모드 공통.

추가 공통 모듈: rib contact (elliptical Hertz), ISO 16281 fatigue life, HMEHL 윤활, dual-mode comparison.

## Project Structure

```
src-tauri/src/solver/    — Rust solver modules
  types.rs                — 전체 struct/enum 정의
  geometry.rs             — SliceGeometry, profile interpolation (cubic spline)
  hertz.rs                — Hertz contact + Weber bulk deformation
  gen1.rs                 — Independent slice solver
  gen3.rs                 — Beam-coupled solver (Newton-Raphson)
  beam.rs                 — Timoshenko beam FE element/assembly
  rib_contact.rs          — Large-end rib elliptical contact
  bearing.rs              — 5-DOF bearing equilibrium
  life.rs                 — ISO 16281 fatigue life
  hmehl.rs                — Homogenized Mixed EHL 윤활 해석
  lubrication.rs          — 윤활 관련 유틸리티
  static_rating.rs        — 정적 하중 정격
  transient.rs            — Transient dynamics solver
  transient_io.rs         — Transient I/O 처리
  wec_risk.rs             — White Etching Crack 위험도
src/components/           — React UI
  InputPanel/              — 입력 패널
  BearingView3D/           — 3D 베어링 시각화
  ResultCharts/            — 결과 차트
  ContourMap/              — 접촉 압력 등고선
  ComparisonView/          — Gen1↔Gen3 비교
  ProfileView/             — 프로파일 시각화
  LubricationView/         — 윤활 해석 결과
  TransientView/           — 과도 해석 결과
  AlertPanel/              — 경고/알림
  GeometryView/            — 형상 뷰
  SectionView2D/           — 2D 단면도
  ThermalSpeedView/        — 열속도 뷰
python-prototype/          — 검증용 Python 프로토타입
Manual/                    — 기능별 매뉴얼 (마크다운)
```

## Key Domain Concepts

- **Slice**: Roller effective length L_we를 n등분. 각 슬라이스는 위치별 반경, 곡률, profile correction Δz 보유.
- **Profile superposition**: `Δz_total_k = Δz_roller + Δz_raceway_inner + Δz_raceway_outer`. Custom profile은 cubic spline 보간.
- **Crown types**: Logarithmic (Reusner), Circular, Parabolic, Custom (측정 데이터).
- **Contact exponent**: Palmgren line contact `q_k = C_k × δ_k^(10/9)` (δ_k > 0).
- **Gen3 beam**: 2n×2n sparse banded stiffness matrix (n nodes, 2 DOF: w_k, θ_k). Tapered roller → non-uniform EI_k.
- **Dual mode**: Gen1 결과가 Gen3 초기 추정값으로 사용 → 수렴 가속.

---

## 필수 준수 사항

### 1. 서버 관리
- **Claude는 서버를 직접 시작하지 않음** (사용자가 직접 관리)
- 코드 수정 후 → "서버 재시작 필요" 안내
- 테스트 필요 시 → 사용자에게 확인 요청

### 2. 매뉴얼 관리
- 기능 추가/변경 시 `./Manual/` 폴더의 해당 매뉴얼 업데이트
- 기존 매뉴얼이 있으면 업데이트, 없으면 새로 작성

---

## 공통 실수 방지 체크리스트

### Rust Solver 수정 시
1. `types.rs`의 struct 변경 → **Tauri command 반환 타입**과 **프론트엔드 TypeScript 타입** 동기화
2. 새 solver 모듈 추가 → `mod.rs`에 등록
3. 수치 알고리즘 변경 → Python prototype의 golden test와 비교 검증
4. `serde` 직렬화 필드 변경 → 기존 저장 파일과의 호환성 확인

### Frontend 수정 시
1. Rust 타입 변경 시 → `invoke()` 호출부와 TypeScript 인터페이스 동기화
2. 차트/3D 컴포넌트 → 대용량 데이터(100+ rollers × 50+ slices) 성능 확인
3. Tauri command 시그니처 변경 → 프론트엔드 호출부 일괄 업데이트

### 코드 수정 시 점검
> 상세 규칙은 글로벌 CLAUDE.md의 "수정 시 점검" 참조

---

## 검증 체크리스트

### 빌드 검증
- [ ] `npm run build` 성공 (TypeScript + Vite)
- [ ] `cargo test` 통과 (src-tauri/)
- [ ] `cargo clippy` 경고 없음

### 수치 검증 레벨
- **Level A**: Single-slice Hertz vs analytical (<0.1% error)
- **Level B**: Single-roller Gen3 vs FEA (ANSYS/ABAQUS, <3%)
- **Level C**: Gen1↔Gen3 cross-validation (zero misalignment + flat profile에서 수렴)
- **Level D**: Full bearing vs Bearinx/MESYS/MASTA (<5%)
- **Level E**: Experimental (strain gauge/displacement sensor)

---

## Development Strategy

Python prototype first (Phase 1-2), then Rust port (Phase 3+). Python golden test 결과는 포팅 시 bit-level 검증을 위해 보존.
