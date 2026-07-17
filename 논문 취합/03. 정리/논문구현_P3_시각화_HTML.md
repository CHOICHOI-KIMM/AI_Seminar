# 논문구현 P3 시각화 HTML — 파라미터 입력 + 인터랙티브 그래프 재현 계획

> **목적**: 구현 완료된 **M1~M5 정적 체인**을 브라우저에서 **파라미터 입력 → 실시간 재계산 → 그래프**로 다룰 수 있게 하고, **P2-2 재검토판·원논문·각 참고문헌의 그래프를 그대로 모방**한다.
> **선행**: [[논문구현_P3_총괄계획]] §8.C(M3/M4/M5+시간루프 계획) · [[논문 구현_P2-2_검증데이터검토서_통합_M1-M6_재검토]](검증 SSOT, 코드 동결) · crate `논문구현_P3/micropitting-model/`(87단위+2통합 green).
> **작성일**: 2026-07-17 · **브랜치**: `P3_HTML`(main 기준 신규, `P3_M5`→main 병합 후 분기)
> **게이트 G-VIS**: 참조곡선 정량 오라클 전건 일치 · reference.rs leaf 불변식 변이증명 · 네이티브 `cargo test` 무회귀 · 시간루프 의존 그래프(Fig10/11/12) **미표시 유지**.
> **확정(2026-07-17, 연구자 3문 사인오프)**: ① 착수순서 = **HTML 먼저(정적 M1~M5)**, 시간루프는 후속. ② 계산엔진 = **Rust→WASM(SSOT 단일 유지)**, JS 물리 재구현 금지. ③ 문헌 폐형식 = **`src/reference.rs` 신설(leaf 규칙)**.

---

## 0. 한눈에 보기 (Executive Summary)

- **무엇을 만드는가**: 정적 HTML + WASM 뷰어 4탭 — **참조곡선 / 검증 / 단일스텝 필드 / 시간진화(비활성)**. 사용자가 하중·속도·점도·거칠기 등을 입력하면 **실제 Rust 솔버**가 브라우저에서 돌아 그래프를 갱신한다.
- **핵심 제약 ★**: 이 뷰어는 **CRB-main 통합 경로가 아니다**. 총괄계획 §1.1(L30)이 CRB-main을 **"인프로세스 네이티브 Rust — 사이드카/외부바이너리/WASM 없음"** 으로 못박고 있다. 따라서 **최종 통합 경로(native Tauri)는 불변**이고, 뷰어는 **세미나·연구자 검토용 병렬 산출물**이다. → WASM 도입이 §1.2 통합 절차를 바꾸지 않도록, 모든 변경은 **`micropitting-model` 크레이트에 비파괴적**이어야 한다(§3 불변식).
- **왜 WASM인가**: 물리식이 Rust/JS 두 곳에 존재하면 **SSOT가 이중화**되어 [[micropitting-conventions]] 규약과 [[micropitting-work-method]] 대원칙("자의적 작업 금지 · 모든 계수는 P2 식[n]으로 소급")을 정면 위반한다. 드리프트 시 어느 쪽이 정본인지 판별 불가. → 물리는 Rust 하나, JS는 **플로팅·UI만**.
- **왜 지금 가능한가**: 크레이트가 예상보다 WASM에 유리하다 — 파일 I/O·시계·스레드 **0건**, rayon은 **단 한 줄**(`m4_fatigue.rs:381`), `types.rs`는 **전 struct serde 파생 완료**.
- **왜 Fig10/11/12는 못 하는가**: A_p(피팅면적률)는 **Δn 시간루프의 산물**이고, 시간루프는 미구현. 총괄계획 §8.C.4(L435)가 피로↔마모 경쟁을 **"판정식이 아니라 창발구조"** 로 규정 → 폐형식 없음 → **역피팅 없이는 재현 불가**. §6 anti-fudge 게이트 참조.

---

## 1. 현황 확인 (2026-07-17 실측)

### 1.1 코드 상태

| 항목 | 확인값 | 출처 |
|---|---|---|
| 브랜치 | `P3_M5` (M5 G3 통과, main 미병합) | `git branch` |
| 태그 | `P3_M1,2,6` · `P3_phase1b_G3` · `P3_M3_G3` · `P3_M4_G3` | `git tag` |
| 테스트 | **87단위 + 2통합 green** | 메모리·작업결과 |
| 공개 모듈 | `units`·`types`·`util`·`m1_dry`·`m2_lub`·`m3_stress`·`m4_fatigue`·`m5_wear`·`m6_share`·`partial_lub` (9개, **crate-root 재수출 없음**) | `lib.rs:19-30` |
| 오라클 테스트 | **27건** (`vc_*` 20 · `rp_*` 4 · `cv_*` 2 · `dc_*` 3) | `grep` 실측 |

### 1.2 WASM 적합성 — 실측 결과

| 항목 | 결과 | 함의 |
|---|---|---|
| 파일 I/O·`std::time`·`std::thread`·`println!` | **전 소스 0건** | 순수 계산 라이브러리, 이식 장애 없음 |
| `rayon` | **`m4_fatigue.rs:51`(use) + `m4_fatigue.rs:381`(`into_par_iter`) 단 2줄** | `ny`(보통 4~64) 루프라 병렬 이득 미미 → feature 게이팅으로 무해하게 제거 |
| serde | `types.rs:13` — **`types.rs` 전 struct 파생 완료** | WASM 경계 통과 **작업 불필요** |
| 미파생 struct | `m3_stress::StressInput`·`m4_fatigue::FatigueParams`·`m5_wear::WearParams`·`WearInput<'a>`·`m6_share::SharePolicy` | 셸에서 Rust측 조립 필요(§4.2) |
| 툴체인 | `wasm32-unknown-unknown` **미설치**, `wasm-pack` **부재**, cargo 1.95.0 | Phase 0에서 설치 |

### 1.3 ★ 함정 — `solve_partial` 이름 충돌

| 경로 | 정체 | 반환 |
|---|---|---|
| `partial_lub::solve_partial` (`partial_lub.rs:227`) | **진짜 오케스트레이터** (M1+M2+M6 + μ_eff 외부루프) | `phi_bl` 실값, `q_tran = μ_eff·p_tran` |
| `m2_lub::solve_partial` (`m2_lub.rs:494`) | **패스스루 스텁** (하중분담 범위 외 명시) | **`phi_bl = 0.0`, `q_tran = zeros`** |

**시그니처가 동일**하여 잘못 `use` 해도 조용히 컴파일된다. `phi_bl=0` → M5 `wear_coefficient(phi_bl,·)` 건마모 기여 **0** → 마모가 사라진 결과가 정상처럼 보인다. → **셸에서 `partial_lub::solve_partial`만 노출**하여 구조적으로 차단(§4.2).

### 1.4 ★ 메모리 정정 — Fig 10/11/12 축

| Fig | 실제 x축 | 실제 y축 | 출처 |
|---|---|---|---|
| **Fig 10** | **Λ** (Lubrication Quality Parameter) | A_p (%) | 원논문 L448·L471 |
| **Fig 11a** | **S** (slide–roll ratio) | A_p (%) | 원논문 L461·L475 |
| **Fig 12** | **하중 사이클수** | A_p (%) | 원논문 L465 |

기존 메모리 [[micropitting-p3-project]] 의 *"Fig10/11 A_p·피팅율 정성 G5"* 서술은 **"vs time"으로 오인될 소지**가 있었다. "vs time"인 것은 **Fig 12**다. → 메모리 갱신 대상(§8).

---

## 2. 산출물 구조

```
논문구현_P3/
├── micropitting-model/          # 기존 crate — 비파괴 변경만
│   ├── src/reference.rs         # ★신규: 문헌 폐형식 (leaf)
│   └── Cargo.toml               # rayon optional + feature "parallel"
├── micropitting-wasm/           # ★신규: wasm-bindgen 셸
│   ├── src/lib.rs               # 경계 어댑터 (Rust측 struct 조립)
│   └── Cargo.toml
└── viewer/                      # ★신규: HTML/JS 프런트
    ├── index.html
    ├── app.js                   # UI·플로팅만 (물리 금지)
    ├── worker.js                # WASM 호출 (UI 블로킹 방지)
    └── pkg/                     # wasm-pack 산출물
```

**층 분리 원칙**: `viewer/` 는 **물리식을 한 줄도 갖지 않는다**. 숫자는 전부 WASM 경계를 넘어온 것이거나 `reference.rs` 가 계산한 것이다. 이 규칙이 SSOT 단일화의 실체다.

---

## 3. Phase 0 — WASM 스파이크 (derisk, 최선행)

> **목적**: 나머지 전 계획의 전제(= 이 크레이트가 브라우저에서 돈다)를 **최소 비용으로 반증 시도**한다. 여기서 막히면 §4 이후가 무의미하므로 제일 먼저 친다.

### 3.1 작업

1. `rustup target add wasm32-unknown-unknown` · `cargo install wasm-pack`.
2. `Cargo.toml`: `rayon = { version = "1", optional = true }` + `[features] default = ["parallel"] · parallel = ["dep:rayon"]`.
3. `m4_fatigue.rs:51` → `#[cfg(feature = "parallel")] use rayon::prelude::*;`
   `m4_fatigue.rs:381` → `parallel` 시 `.into_par_iter()`, 아니면 `.into_iter()` (`cfg_if` 또는 함수 분기).
4. **최소 셸**: `solve_wear` **하나만** 노출 → 브라우저 콘솔에서 값 확인.

### 3.2 통과 기준 (전건 필수)

| # | 기준 | 사유 |
|---|---|---|
| 1 | `wasm-pack build --no-default-features` 성공 | `rustfft`·`nalgebra` 의 wasm32 빌드 가능성이 **미검증 가정**. 실측 필요 |
| 2 | 브라우저에서 `solve_wear` 가 네이티브와 **동일 값** 반환 | 경계 왕복 무결성 |
| 3 | **네이티브 `cargo test` 87단위+2통합 green 유지** | feature 게이팅이 기존 검증을 깨지 않음 (default=parallel) |
| 4 | `--no-default-features` 로도 네이티브 `cargo test` green | 직렬 경로가 병렬 경로와 동일 물리임을 증명 |

### 3.3 실측할 것 — M4 계산량

`solve_stress` 는 `2 + 6·nz` FFT (`nz` 기본 15, `NZ_DEFAULT`/`DEPTH_FRAC` @ `m3_stress.rs:110-112`) → 가볍다.
**병목은 `solve_fatigue`** — `nz·ny` 컬럼 각각에 `nx` 길이 이력의 **MCE 최소외접초구 최소화**를 돌린다. 이것이 **바로 rayon을 쓰던 그 자리**이므로, WASM 단일스레드에서 비용이 그대로 드러난다.

- 측정: (nx,ny,nz) = (256,32,15) / (512,64,15) 벽시계.
- 대응 순서: ① 그리드 축소 → ② Web Worker(UI 블로킹 방지) → ③ 그래도 부족하면 `wasm-bindgen-rayon`(SharedArrayBuffer + COOP/COEP 헤더 필요 = `file://` 불가 → **로컬 서버 강제**, 배포 단순성 상실) 또는 해당 스윕만 사전계산 JSON.
- **③은 Phase 0 실측 후에만 판단** — 선제 도입 금지.

> **주의(총괄계획 L385 교훈)**: `nx=8192·ny=4·nz=31` 은 **기본값이 아니다**. `vc_m3_hertz_line_von_mises`(`m3_stress.rs:737-766`) **한 테스트의 수렴 선택**(256b 창 → +0.49%)이다. 뷰어 기본값으로 옮기면 브라우저가 죽는다.

---

## 4. Phase 1~2 — 크레이트 측 작업

### 4.1 Phase 1 — `src/reference.rs` 신설 (leaf)

**문제**: 문헌 폐형식이 전부 `#[cfg(test)]` 안에 있어 뷰어에서 참조곡선으로 그릴 수 없다.

**대상 (실측 8곳)**

| 문헌 폐형식 | 현재 위치 | 재검토판 등급 |
|---|---|---|
| GW1994 eq(15)(16)(17) 진폭감소 | `m2_lub.rs:1089` `vc_m2_ar_gw1994_closed_form` | A |
| GW1994 Table 1 스팟값 | `m2_lub.rs:1130` `vc_m2_spot_gw1994_table1` | A |
| Venner 2000 eq(29) 점접촉 마스터 | `m2_lub.rs:1158` `vc_m2_master_venner2000` | A |
| Venner 1997 eq(5) + Table 1 | `m2_lub.rs:1222` `vc_m2_comp_amplitude_venner` | A |
| Tripp/ME 2003 eq[10]/[16] 6성분 | `m3_stress.rs:469` `vc_m3_sin_normal` · `:526` `vc_m3_sin_tangential` | A |
| McEwen 선접촉 축상 vM | `m3_stress.rs:730` `vc_m3_hertz_line_von_mises` | A |
| Milano 2006 App A 폐형식 | `m4_fatigue.rs:528` `vc_m4_milano_closed_form` | A |
| Archard 1953 §III.2 | `m5_wear.rs:266` `vc_m5_archard_closed_form` | A |

**방식**: **이관(move)**하고 기존 테스트가 `use crate::reference::*` 로 **import**. 복제가 아니다.

**★ leaf 불변식 (위반 시 오라클 붕괴)**

```
reference.rs 는 types·units 외 import 금지.
m1~m6·partial_lub 는 reference 를 절대 import 금지.
```

**사유**: 모델이 reference를 쓰기 시작하면 **오라클이 모델 자신을 검증**하게 되어 순환한다. 이는 Phase 1에서 실제로 발생해 G3 fail을 냈던 **M2 Q oracle tautology와 동형**이며, M4에서도 *"기존 오라클 전수 대칭 → centroid 스텁이 전 오라클 통과"* 라는 사각지대로 재발했던 실패 모드다([[micropitting-p3-project]]).

**증명 방법 (2중)**
1. **변이 게이트**: `reference.rs` 상수/식 변조 → **해당 VC만 FAIL** 확인 → 즉시 원복. (전건 FAIL이면 결합 과다, 무FAIL이면 오라클 사문화.)
2. **구조 가드 테스트**: `m1..m6`·`partial_lub` 소스에 `reference` 참조가 없음을 grep으로 강제하는 테스트 1건.

**동결 저촉 여부**: 재검토판이 선언한 **"코드 동결"** 은 *물리 거동*의 동결이다. 본 이관은 **비거동 변경**(테스트 통과값 불변, `cargo test` 무회귀)이므로 저촉하지 않는다. 단 Phase 0 기준 ③④와 함께 **무회귀를 기계 증명**한 뒤에만 커밋한다.

### 4.2 Phase 2 — `micropitting-wasm` 셸 크레이트

**역할**: 얇은 경계 어댑터. **물리 로직 금지.**

| 처리 | 사유 |
|---|---|
| `partial_lub::solve_partial` **만** 노출 | §1.3 스텁 함정 구조적 차단 |
| `StressInput`(serde 미파생)·`WearInput<'a>`(lifetime) **Rust측 조립** | lifetime은 wasm-bindgen 경계 통과 불가 |
| `FatigueParams`·`WearParams`·`SharePolicy` 는 `Default` 보유 → 부분 override JSON | `m4_fatigue.rs:98`·`m5_wear.rs:101`·`m6_share.rs:87` |
| **`*_traced` 변종 노출** (`solve_partial_traced`·`combine_share_traced`) | ★ 크레이트에 **로깅이 전무** → `PartialTrace`/`ShareTrace`(`phi_history`·`converged`·`iters`·`load_residual`·`flow_balance_residual`)가 **유일한 진단 채널**. 미노출 시 **미수렴 해와 정상 해가 UI에서 구분 불가** |
| `contact_half_width`(`m3_stress.rs:381`, pub) 노출 | `b` 도출 → 깊이 슬라이더·Venner ∇ |
| `solve_stress_at_depths` 노출 | `solve_stress` 는 `nz` 고정(상수) → 깊이 해상도 슬라이더는 이쪽으로만 가능 |

**UI 필수 표시**: `converged == false` 이면 그래프에 **경고 배지**. 조용한 미수렴이 최악의 실패 모드다.

---

## 5. Phase 3 — 뷰어 4탭

### 5.1 탭 1 — 참조곡선 (최우선, 가치/비용 최고)

**전부 순수 대수 + 정량 오라클 보유 → 지금 100% 가능.**

| 곡선 | x / y | 오라클 |
|---|---|---|
| **Venner 1997 Fig 1 + eq(5)** | λ/b / (A_d/A_i) | **Table 1 = 18점 verbatim** ★ 최대 자산 |
| Venner 1997 Fig 4 마스터 붕괴 | ∇ = (λ/b)M^{3/4}/L^{1/2} | eq(5) 오버레이 |
| Venner 2000 eq(29) | ∇₂ = (λ/a)M^{1/2}/L^{1/2} | 스팟 **0.739** (코드 0.7036, <6%) |
| GW 1994 eq(15)(16)(17) | λ / 진폭비 | **Table 1 그룹바** |
| Milano App A | ωt / τ_DV·σ_H | ≤1% |
| McEwen | z/b / (σ_vM/p_h) | **0.557 @ z≈0.70b** |
| Archard Fig 7 | log P / log 마모율 | **기울기 1.00 / 0.98** |

**Venner 1997 Table 1 (A_i/H_c = 0.1 열 = anti-fudge 앵커, 총괄계획 L476: 2~5% 재현 요구, 튜닝 금지)**

| λ/b | 4.0 | 2.0 | **1.0** | **0.5** | **0.25** | 0.125 |
|---|---|---|---|---|---|---|
| A_d/A_i | 0.030 | 0.073 | **0.183** | **0.394** | **0.660** | 0.839 |

**UI 정직성 요구**
- eq(5) verbatim 단서: *"A_d/A_i ≤ 0.5 에서 잘 근사하나, 0.5 < A_d/A_i < 1 구간은 **너무 작은 값을 예측하는 것으로 보인다**"* → 해당 ∇ 구간 **음영 + "fit degrades" 표기**.
- **선접촉/점접촉 혼동 차단**: Venner 1997 = 선(`M=W(2U)^{-1/2}`), Venner 2000 = 점(`M=W(2U)^{-3/4}`). 총괄계획 L476이 **line-M을 (31)로 검증하는 것을 카테고리 오류로 명시 금지**. → UI에서 두 축을 **절대 같은 플롯에 겹치지 않는다**.
- **RQ-M2-comp-curve = P4 이월**: 상보파 전곡선은 **단일점(g=0.5, λ/b≈0.377)만 결선**됨. 탭에 **"단일점 앵커 · 전곡선 P4"** 명시. 재시도 금지.

### 5.2 탭 2 — 검증 (VC-* 27건)

**★ 등급 색분리가 이 탭의 존재 이유다.** 재검토판 L9-11이 등급을 정의하고, **C등급은 "회귀가드일 뿐 검증이 아니다"** 라고 못박는다.

| 등급 | 의미 | 표기 |
|---|---|---|
| **A** | 해석해·문헌 정량 일치 = **진짜 검증** | 녹색 |
| **B** | 구조·보존·차원·극한·정성 | 황색 |
| **C** | 자기일관성·항등식·가정+민감도 — **검증 아님** | 회색 + 명시 캡션 |

27건을 **균일한 목록으로 나열하면 문서가 애써 만든 구분이 UI에서 무너진다.** A(문헌정합)와 C(자기일관성)가 같은 무게로 보이는 순간, 뷰어는 검증 상태를 **실제보다 강하게** 표현하게 된다.

재검토판 판정 요약도 함께 표시: **P2-2정정 6 · 상태갱신 2 · 신규등재 3 · 허구 verbatim 삭제 1 · 코드이월 2** (L161).

**정직 표기 필수 항목**
- `VC-M4-DesiTab`: 코드 단일선 예측 **0.859/0.801** vs 실험 **0.76/0.68** → **X 『상태갱신』**. *"단일선 예측 ≠ 실험(정상거동)"* — Desimone가 2-slope를 제안한 바로 그 이유. **실패로 표시하지 말 것.**
- `RP-M1-plim`: 문헌 **4.3 GPa** vs 코드 fixture **4.0 GPa** → 코드이월(우선순위 low) 명시.
- `N_ref = 1e6` = **C등급 가정**(SKF 미제공).
- `RP-klub`: k_lub ∈ [1e-11, 5e-10] = **가정+민감도**. Archard L308 verbatim은 **"10² or 10³"**(=100~1000)이며 f_w≈10의 근거는 **MAIN L371 마찰비**(과거 문서 오기 정정분).

### 5.3 탭 3 — 단일스텝 필드

**근거 = x-스냅샷 트릭** (총괄계획 L389): *정상상태 단일형상에서 물질점의 응력 시간이력 = 접촉이 그 점을 지나가는 공간변화(T = x/ū) = **M3 한 스냅샷의 x축 변화***. → **1사이클 Dang Van 손상맵은 시간루프 없이 정적으로 산출 가능**.

| 출력 | 모듈 |
|---|---|
| p_tran(x,y) · h_tran(x,y) · φ_bl | `partial_lub::solve_partial_traced` |
| σ_vM(x,z) @ y=0 (Fig 6b·15b 대응) | `m3_stress::solve_stress_at_depths` |
| 1사이클 d(x,y,z) · N(x,y,z) | `m4_fatigue::solve_fatigue` |
| Δh_w | `m5_wear::solve_wear` |

**표기 필수**: 원논문 Fig 6/15는 **"in the last wear step"**(L363·L524) = **마모된 형상**이다. 정적 실행은 **미마모 형상**이므로 **"미마모 형상 · 정성 비교 전용(RP-Field)"** 캡션 필수. 재검토판 L112가 *"이미지, 수치추출 불가"* 로 RP-Field를 **정성 전용**으로 못박으므로 정량 대조 금지.

### 5.4 탭 4 — 시간진화 (Fig 10/11/12) — **비활성**

**표시하지 않는다. 곡선을 그리지 않는다.** 안내문만 둔다.

**사유 (3중)**
1. **물리**: A_p는 Δn 루프의 산물. 총괄계획 L435 — 피로↔마모 경쟁은 *"판정식이 아니라 시간루프 UPD에서 M4(d>1 피트)·M5(Δh_w)가 동일 형상을 함께 갱신하는 **창발구조**"*. **폐형식 없음** → 계산 불가.
2. **금지**: 총괄계획 L440 — 이미지 대조라 **정량 임계 금지**. 재검토판 RP-Λmax/RP-Slip 모두 원문이 *"qualitatively"* 명시 → **정성 유효**, G5 게이트.
3. **전례**: 총괄계획 L476 anti-fudge 게이트는 **이전 시도(`wf_2638d46e`)가 곡선을 역피팅했다가 적대 크리틱 fail** 을 낸 결과로 생겼다.

> **그럴듯한 곡선을 미리 그려두면, 나중에 시간루프가 다른 답을 낼 때 어느 쪽이 정본인지 판별할 수 없다.** 빈 탭이 틀린 탭보다 낫다.

**안내문 문안**: *"Fig 10(A_p vs Λ) · Fig 11(A_p vs S) · Fig 12(A_p vs 사이클수)는 Δn 시간루프(총괄계획 §8.C.5)를 요구합니다. Λ≈1.1 peak는 M4/M5 경쟁의 창발 결과로 폐형식이 없으며, 역피팅은 anti-fudge 게이트(L476)로 금지됩니다. 시간루프 G5 통과 후 활성화됩니다."*

---

## 6. 폼 기본값 (원논문 Table 1 = roller–disc)

| 파라미터 | 기호 | 값 | 단위 | 출처 |
|---|---|---|---|---|
| 환산탄성계수 | E′ | **231** | GPa | 원논문 L335 (코드 230.77, 0.1%) |
| Eyring 응력 | τ₀ | **3×10⁶** | Pa | L335 |
| 경계마찰계수 | μ_bl | **0.12** | – | L335 |
| EHL 마찰계수 | μ_ehl | **0.05** | – | L335 |
| 기유점도 VG32@75°C | η₀ | **0.0094** | Pa·s | L335 |
| 기유점도 VG10@75°C | η₀ | **0.0034** | Pa·s | L335 |
| 점도-압력계수 | α | **20.78** | GPa⁻¹ | L335 |
| 슬라이드-롤비 | S | **0.02** | – | L335 |
| 평균속도 | ū | **1.0** | m/s | L335 |
| 최대 Hertz압 | p_h | **1.5** | GPa | L416 등 |
| 경도 | H | **7** | GPa | L371 |
| 항복압 | p_lim | **4.3** | GPa | L143 (코드 4.0 = 이월) |
| Wöhler | A / B | **−43.0** / **1220** | MPa | 부호 확정 |
| Dang Van | α_dv | **0.232** | – | 3(τ_W/σ_W−½) |
| 마모계수 | k_lub | **5×10⁻¹⁰** (0 = 무마모) | – | 가정+민감도 |

**프리셋**: Fig 10(디스크 Rq 0.230 / 롤러 0.060 µm, η₀ 스윕) · Fig 11(Table 2, VG32) · Fig 12(0.7 / 0.05 µm, VG32, 4770 cyc/min) · Fig 5(0.5 / 0.05 µm, VG10).

**사용자 입력 필수(문서 미제공)**: `r_x`(등가반경) · `Grid.lx/ly` · ν(0.3, 가정 표기). `r_x`는 총괄계획 L386의 **★공유 enabler** — `b = 2·r_x·p_h/E_red`(`m3_stress.rs:381`)로 M3 깊이·McEwen 앵커·M5 접촉폭·Venner ∇ 를 동시에 연다.

### 6.1 ★ 폼에 넣으면 안 되는 값 (허구 verbatim)

| 값 | 판정 |
|---|---|
| **"0.244 × 1.016 mm @ 1.5 GPa"** | 재검토판 **R-M1-1b: 삭제(원문 부재)** — OCR 허구 |
| **Fig 14 A_p = 3.16 / 4.33 / 0.91 %** | 재검토판 주1: 원PDF 직접판독 결과 **본문 부재 → 판정 근거 제외**. 정성 경향만 |
| **Fig 12 baseline A_p = 2.4 %** | 그래프 판독값. verbatim은 *"yielding Ap = 4.56%"*(μ_bl 0.15) 뿐 |
| **2011 식[12]/[13]** | OCR 손상(σ_y). **SSOT = Tripp/ME 2003 식[10]/[16]** |

---

## 7. 리스크 · 게이트

| # | 리스크 | 심각도 | 대응 |
|---|---|---|---|
| R1 | `rustfft`/`nalgebra` wasm32 빌드 실패 | **높음(계획 전제)** | **Phase 0 최선행 스파이크**. 실패 시 → 사전계산 JSON 폴백(인터랙티브성 상실, 재협의) |
| R2 | M4 브라우저 계산량 초과 | 중 | 그리드 축소 → Worker → (실측 후에만) rayon-wasm/사전계산 |
| R3 | reference.rs 순환 → 오라클 tautology | **높음(전례 2회)** | leaf 불변식 + 변이증명 + 구조 가드 테스트 |
| R4 | `m2_lub::solve_partial` 스텁 오사용 | 중(**무증상**) | 셸에서 `partial_lub::` 만 노출 |
| R5 | 미수렴 해가 정상처럼 표시 | 중(**무증상**) | `*_traced` 노출 + `converged` 배지 |
| R6 | 시간진화 탭 역피팅 유혹 | **높음(전례 1회)** | §5.4 비활성 유지. G-VIS 게이트 항목 |
| R7 | WASM이 CRB 통합 경로를 오염 | 중 | 셸은 **별도 크레이트**. `micropitting-model`은 feature 게이팅 외 무변경. §0 핵심제약 |
| R8 | JS에 물리식 유입 → SSOT 이중화 | 중 | `viewer/` 물리 금지 규칙. 코드리뷰 항목 |

**게이트 G-VIS 통과 기준**
1. 참조곡선 정량 오라클 **전건 일치** (Venner Table1 A_i/H_c=0.1 열 2~5%, GW Table1 ±5/15/18/20%, McEwen <1%, Milano ≤1%, Archard 기울기).
2. reference.rs **leaf 불변식 변이증명** + 구조 가드 green.
3. 네이티브 `cargo test` **87단위+2통합 무회귀** (default 및 `--no-default-features` 양쪽).
4. 검증 탭 **A/B/C 등급 분리 표시** 확인.
5. 시간진화 탭 **곡선 0개** 확인.
6. `viewer/` 에 물리식 **0건** 확인.

---

## 8. 착수 순서 · 후속

**브랜치 운용 (2026-07-17 실행 완료)**: M5는 이미 G3 통과(변이 5/5 CAUGHT · 크리틱 3/3 · fabrication_found=false)이므로 기존 패턴대로 **`P3_M5` → main `--no-ff` 병합**(병합커밋 **`a26c860`**) **+ 태그 `P3_M5_G3`** → main 기준 **`P3_HTML`** 분기. (태그명 ≠ 브랜치명 — ref 충돌 교훈.)

> **★ M1~M6 전 모듈 main 병합 완료** (`a26c860`): `lib.rs` 에 `m1_dry`·`m2_lub`·`m3_stress`·`m4_fatigue`·`m5_wear`·`m6_share`·`partial_lub` 전건 등재 확인. 태그 `P3_M1,2,6`·`P3_M3_G3`·`P3_M4_G3`·`P3_M5_G3`. **단, "M1~M6 병합 = 논문 재현 완료"가 아니다** — 미구현분은 **시간루프(E)**(§5.4·총괄계획 §8.C.5)이며, 이것이 Fig 10/11/12(A_p) 부재의 원인이다.

| 순서 | 작업 | 산출 |
|---|---|---|
| 0 | **P3_M5 → main 병합 + 태그** | 기준선 정리 |
| 1 | **Phase 0 WASM 스파이크** ← **최선행** | 계획 전제 검증/반증 |
| 2 | Phase 1 `reference.rs` + 변이증명 | SSOT 유지 참조곡선 |
| 3 | Phase 2 `micropitting-wasm` 셸 | 경계 |
| 4 | Phase 3 탭1(참조곡선) → 탭2(검증) → 탭3(필드) | 가치순 |
| 5 | G-VIS | 완료 |

**후속 (본 계획 범위 밖)**
- **시간루프**(총괄계획 §8.C.5) → 완료 시 **탭4 활성화**가 자연스러운 결선점. 뷰어가 시간루프의 **디버깅 도구**로도 기능.
- **RQ-M2-comp-curve** → P4(§8.D). 재시도 금지.
- 잔여: RQ-M6-tol · 평균압 p̄=p_h 규약 · RP-M1-plim(4.0→4.3).

**문서 반영 (연구자 지시 = 작업결과 + 작업내역 **항상 함께**)**
- 본 문서 = 계획 정본. 실행 결과는 `논문구현_P3_작업결과_시각화_HTML.md`(신설) + `논문 구현_workflow_작업내역.md`(이력).
- 메모리 갱신: [[micropitting-p3-project]] 에 §1.4 Fig10/11/12 축 정정 + 본 계획 포인터.
