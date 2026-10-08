# CLAUDE.md - BB Contact Analysis System

> **본 프로젝트는 CRB-main (`d:/AI/AI_Seminar_CRB/CRB-main`) 의 SW 체계를 모태로 개발 중인 BB(볼베어링 계열) 해석 SW 임.**
> 현재 검증 완료 범위는 **단열 ACBB**(각접촉 볼베어링)이며, DGBB·4PCBB 는 같은 코어를 쓰는 변종으로 확장 예정이다 (Plan §3.6.9).
>
> - 이론 정리: [BB_Development_Theory.md](BB_Development_Theory.md) — **식·계수의 유일한 출처**
> - 작업 계획서: [BB_Development_Plan.md](BB_Development_Plan.md)
> - 작업 로그: [BB_Development_Action.md](BB_Development_Action.md)
> - TRB → CRB 전환 선례: [_crb_archive/](_crb_archive/)
>
> 코딩·UI 일반 원칙(Simple is Best · 정공법 · 공용 컴포넌트 · 타이포그래피)은 **글로벌 CLAUDE.md** 를 따른다.
> 이 문서는 그것을 **BB 에 어떻게 적용하는지**와 BB 고유 규약만 적는다 (아래 「글로벌 규칙의 BB 적용」).

---

> ⚠️ **좌표계가 CRB 와 다르다 (Plan D-7)**: BB 는 **ISO 규약 — X = 회전축**, Y·Z = 반경방향, `φ_j = 0` 이 **+Y** (D-8).
> CRB 는 Z = 샤프트축이다. CRB/TRB 코드를 참고할 때 축 이름을 그대로 옮기면 안 된다 (`CRB X→BB Y`, `CRB Y→BB Z`, `CRB Z→BB X`).
>
> ⚠️ **단위 (Plan D-10)**: 솔버 내부는 **mm · N · rad**. μm·kN·° 변환은 `commands` 경계(또는 UI)에서만. 솔버 안에 `1000.0` 이 등장하면 안 된다.
> 필드명은 단위 접미사를 단다(`dx_mm`, `q_n`, `ry_rad`) — Level A-8 이 기계적으로 검사한다.
>
> ⚠️ **화면 각도 표시 (Plan §3.6.4.10)**: 0° = +Y = **화면 아래**, 각도는 **우측(+Z)으로 증가** — ISO 16281 A.2.2 NOTE「Y 축은 지면 아래쪽」.

---

## Project Overview

BB Contact Analysis System — 볼베어링 내부 접촉 해석 도구. 볼은 점접촉이므로 **슬라이싱이 없다**.

- **5-DOF 베어링 평형** (`δx, δy, δz, γy, γz`) → 볼별 하중 `Q_j`·운전 접촉각 `α_j`
- **타원 Hertz 점접촉** (Harris Ch.6) → 접촉타원 `a`·`b`, `p_max`
- **수명** ISO 281 / 16281 · **정정격** ISO 76
- 윤활(`κ` + Hamrock-Dowson 점접촉 유막)은 Phase 6 예정

## Technology Stack

- **Solver core**: Rust (nalgebra, serde, ts-rs)
- **Desktop shell**: Tauri 2
- **Frontend**: React 19 + TypeScript (Vite 7)
- **3D**: Three.js (@react-three/fiber · drei)
- **Charts**: Plotly.js

## Development Commands

```bash
npm run dev            # 프론트엔드 개발 서버 (Vite)
npm run build          # tsc -b + Vite 프로덕션 빌드
npm run lint           # ESLint (baseline 44 = TRB 잔존 파일 몫. src/bb 는 0 이어야 한다)
npm run tauri dev      # Tauri 개발 모드 (프론트+백엔드)
cargo test             # Rust 솔버 테스트 (src-tauri/)
```

> ⚠ 타입 검사는 **`npx tsc -b --noEmit`** 으로 한다. 루트 `tsconfig.json` 은 `references` 만 있어
> `npx tsc --noEmit` 은 앱을 검사하지 않는다 (Plan §3.6.5.3).

---

## Project Structure

```
src-tauri/src/
  commands.rs              — Tauri 커맨드 (전부 `bb_` 접두 — 전역 네임스페이스라 필수)
  presets.rs               — 프리셋 로드
  solver/common/           — 계열 공통: util.rs (보간·수치) · types.rs (Alert 등)
  solver/bb/               — 볼 계열 전용
    types.rs                — BbInput · BbResult · BbLifeResult … (ts-rs 로 TS 생성)
    geometry.rs             — A, α₀, R_i, Σρ, F(ρ)            (Theory §2)
    hertz.rs                — 타원 점접촉                      (Theory §6)
    bearing.rs              — 5-DOF 평형 (DofMask 로 3-DOF 구속 가능)
    life.rs                 — ISO 281 / 16281 수명            (Theory §7.1~7.10)
    static_rating.rs        — ISO 76 정정격                   (Theory §7.11~7.13)
src-tauri/tests/           — <대상>_level_<등급>.rs (Level A~E)

src/bb/                    — BB 전용 프론트 (CanvasArea 에 **prop 으로 주입**)
  generated/               — ts-rs 생성 타입 (손으로 고치지 않는다)
  shared.ts                — num · toDeg · 데이터 색 팔레트 · useGeometryResponse  ← 공용 로직
  Bb*View.tsx              — Geometry · Section · 3D · LoadDist · StressContour · Life
  BbInputPanel · BbResultsCard
  healthcheck.ts           — VITE_BB_HEALTHCHECK=1 런타임 스모크 (커맨드 왕복 + 6탭 마운트)
  errorBridge.ts           — 웹뷰 오류 → Rust 로그
src/components/            — TRB 잔존 (legacy 탭). components/ → bb/ import 는 ESLint 로 금지
```

> `Manual/` · `python-prototype/` 는 아직 **CRB 시절 내용**이다. BB 기준 재작성 전까지 근거로 쓰지 않는다.

---

## 글로벌 규칙의 BB 적용 〔2026-10-08, Plan Phase 4-4〕

| 글로벌 규칙 | BB 에서의 구체 |
|---|---|
| **상수 SSOT** (§0.2) | `num`·`toDeg`·**데이터 색**(`C_LOADED` 등)은 **`src/bb/shared.ts` 한 곳**. 뷰에 다시 정의하지 않는다 |
| **Rule of Three** | 3번째 등장 시 `shared.ts`(로직) 또는 `ui.tsx`(UI)로 추출. 2곳까지는 인라인 허용 |
| **공용 컴포넌트 우선 · 단일 진입점** | BB UI 공용 모듈은 **`src/bb/ui.tsx`**(알림 박스·카드·알림 목록·로딩 + `DetailTable` 재노출). **첫 UI 수정 때 신설**하고, 그 뒤 BB 뷰는 공용 UI 를 이 파일로만 import 한다 |
| **토큰 우회 금지** | Tailwind 는 `index.css` 의 `@theme` 토큰 클래스(`text-text-canvas`, `bg-canvas-subtle`, `text-warning` …). SVG·Plotly 처럼 클래스를 못 쓰는 곳은 **`shared.ts` 의 색 상수**를 쓴다 |
| ⚠ 의미가 다른 색 | 데이터 계열색과 상태 토큰은 **값이 같아도 합치지 않는다** (`C_LOADED` ≡ `--color-warning` 값이지만 별개). 경고색을 바꿨는데 하중 그래프 색이 바뀌는 결합을 막는다 |
| **폰트 단계 3~4개** | HTML: **11 / 12 / 13 / 14 px** 안에서. SVG 도면 글자도 **실제 px 로 11/12/13 고정**(viewBox 비례 금지 — `clientWidth` 로 환산) |
| **굵게 자제** | 화면당 1~2곳(핵심 수치). 설명문의 강조는 굵게 대신 색·위치로 |
| **소급 리팩터링 금지** (§0.3) | 위 UI 규칙은 **새로 만들거나 고치는 뷰에만** 적용. 손대지 않는 뷰를 일괄 수정하지 않는다 |
| **검증은 테스트 코드 실측** | Rust 는 Level A~E 테스트. 프론트는 `healthcheck.ts` 가 실제 앱에서 측정·비교한다 |

---

## Key Domain Concepts

- **Displacement** — `{dx_mm, dy_mm, dz_mm, ry_rad, rz_rad}` named struct (배열 금지 — 인덱스 의미가 CRB 와 다르다)
- **`BallBearingKind`** — `Acbb` · `Dgbb` · `FourPoint`. 변종은 폴더가 아니라 enum 으로 구분 (Plan §3.6.9). 결과에도 `kind` 판별자가 있다
- **`A = r_i + r_e − D_w`** — 곡률중심 거리. ACBB 해석의 기준 길이
- **`R_i`** — 틸트 팔 (D-9a/b). `D_pw/2` 가 아니다
- **`resultInput`** — Solve 시점 입력 스냅샷. 결과와 시점을 맞춰야 하는 화면 값은 `bbInput`(편집 중)이 아니라 이것을 읽는다

---

## 필수 준수 사항

### 1. 서버 관리 〔개정 2026-08-24 — Plan §3.6.5.3〕

- **장기 구동은 사용자 몫**. Claude 는 개발 서버를 띄워 놓은 채 두지 않는다.
- **예외 — 런타임 헬스체크**: Claude 는 **검증 목적에 한해** `npm run tauri dev` 를
  **백그라운드로 기동 → 로그 확인 → 반드시 종료** 할 수 있다.
  - 목적: 웹뷰 JS 런타임 오류·Rust 패닉·커맨드 왕복 실패를 **사용자보다 먼저** 걸러내기 위함
  - 조건: ① 백그라운드 실행 ② 로그 파일로 출력 수집 ③ 확인 후 **프로세스 종료**
  - **포트를 점유한 채 방치하거나 사용자 세션과 겹치게 두지 않는다**
- 육안 확인(레이아웃·숫자의 물리적 타당성·축/부호 방향)은 **사용자가** `npm run tauri dev` 로 수행한다.
- 오류 발견 시 — **명백한 것**(오타·import 누락·필드명/타입 불일치)만 자동 수정하고 **목록을 전량 보고**한다.
  **판단이 필요한 것**(표시 물리량 선택·레이아웃·**솔버가 틀렸을 가능성**)은 **멈추고 질문**한다.
  화면을 고쳐 숫자를 맞추면 검증이 무의미해지기 때문이다.

### 2. 매뉴얼 관리
- 기능 추가/변경 시 `./Manual/` 폴더의 해당 매뉴얼 업데이트
- 기존 매뉴얼이 있으면 업데이트, 없으면 새로 작성

### 3. 커밋
- **`git add -A` · `git add .` 금지 — 명시 경로만**. 하위단계마다 즉시 커밋
- 커밋 제목 앞에 `YYMMDD` 접두. push 는 사용자 지시가 있을 때만

---

## 공통 실수 방지 체크리스트

### Rust Solver 수정 시
1. `types.rs` 변경 → `cargo test` 로 **ts-rs 생성물(`src/bb/generated/`) 재생성** 후 함께 커밋
2. 새 solver 모듈 → `solver/bb/mod.rs` 에 등록 + **A-8 자기검사 대상에 편입**
3. 식·계수는 **Theory 에 있는 것만**. Theory 에 없으면 먼저 Theory 에 근거와 함께 전사한다 (OCR 은 2경로 검증)

### Frontend 수정 시
1. 새 뷰는 `src/bb/` 에 두고 `App.tsx` → `CanvasArea` **prop 주입**으로 배선 (`components/` 가 `bb/` 를 import 하지 않는다)
2. `num`·`toDeg`·색은 `shared.ts` 에서 import — 뷰 안에 다시 정의하지 않는다
3. `null` 결과는 **빈칸**으로 표시 — 0 이나 대시로 채우지 않는다 (「계산 안 됨」과 「0」을 구분)

---

## 검증 체크리스트

### 빌드 검증 (5단 사다리 — Plan §3.6.5.3)
- [ ] `npx tsc -b --noEmit` 0
- [ ] `npm run build` 성공
- [ ] `npm run lint` baseline 증감 0
- [ ] `cargo test` 통과 · `cargo clippy --all-targets -- -D warnings` 0
- [ ] 런타임 헬스체크 ALL PASS (커맨드 왕복 + 6탭 마운트)

### 수치 검증 레벨 (Plan §5)
- **Level A**: 기하 (A, α₀, R_i, Σρ, F(ρ)) — 해석적 항등 + A-8 단위 접미사 자기검사
- **Level B**: 점접촉 — **Harris Table 6.1 (양방향)** + AGM↔Gauss-Legendre 교차 + ISO 내부 일관성
- **Level C**: 평형 해석해 (순수 축하중 대칭성, 예압 균등)
- **Level D-1**: 통합 — **Harris Table 7.4** (3-DOF 구속 모드, 오차 ≤ 5 %)
- **Level D-2/D-3**: 5-DOF 해방 — 축퇴 항등성 · 모멘트 부호 판별
- **Level E**: 수명·정정격 — ISO 표 재현(E-1 ISO 76 Table 1 · E-9/E-10 ISO 281 Table 3)
- **Level F**: 윤활 — h_min 범위·κ≈Λ^1.3·운동학 (Phase 6)
