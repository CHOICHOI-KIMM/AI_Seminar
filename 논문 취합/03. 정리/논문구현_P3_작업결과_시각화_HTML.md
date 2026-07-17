# 논문구현 P3 작업결과 — 시각화 HTML

> **계획 정본**: [[논문구현_P3_시각화_HTML]] · **이력**: [[논문 구현_workflow_작업내역]]
> **범위**: 파라미터 입력 + 인터랙티브 그래프 재현 HTML (Rust→WASM, SSOT 단일).
> **브랜치**: `P3_HTML`

---

## Phase 0 — WASM 스파이크 (2026-07-17) · **전건 통과, R1·R2 해소**

### 0.1 목적

계획 §3 — **기능 구현이 아니라 전제의 반증 시도**. 계획 전체가 *"이 크레이트가 브라우저에서 돈다"* 는 미검증 가정 위에 서 있었다. 최소 비용(`solve_wear` 1개)으로 이를 확인/반증한다.

### 0.2 통과 기준 4건 — 실측 결과

| # | 기준 | 결과 | 실측값 |
|---|---|---|---|
| ① | `wasm-pack build` (wasm32, 단일스레드) 성공 | **통과** | 산출물 **329 KB** (release). 사전 추정 1~2 MB 대비 대폭 하회 |
| ② | WASM 값 == 네이티브 값 | **통과** | **문자열 동일** (f64 직렬화까지 일치) |
| ③ | 네이티브 `cargo test` (default = parallel) | **통과** | **87단위 + 2통합 green**, 무회귀 (6.79s) |
| ④ | 네이티브 `cargo test --no-default-features` (직렬 = wasm 경로) | **통과** | **87단위 + 2통합 green** (5.62s) |

### 0.3 R1 해소 — 의존성 wasm32 호환성 (**계획 최대 리스크**)

계획 §7 R1 = *"`rustfft`/`nalgebra` wasm32 빌드 실패 → 계획 전제 붕괴"*, 심각도 **높음**.

**결과: 해소.** `cargo build --target wasm32-unknown-unknown --no-default-features` 성공.

| 의존성 | 버전 | wasm32 |
|---|---|---|
| `rustfft` | 6.4.1 | ✅ |
| `nalgebra` | 0.33.3 | ✅ |
| `num-complex`·`simba`·`wide`·`transpose`·`primal-check` 등 전이 | — | ✅ |
| `rayon` | — | **빌드 목록 부재** = feature 게이팅 실작동 확인 |

> 우리 코드 쪽 지표(I/O 0건·rayon 2줄·serde 완비)는 사전에 유리했으나 **의존성 호환성은 별개 문제**였고, 이번에 실측으로 확정했다.

### 0.4 기준 ② 상세 — 경계 왕복 무결성

**설계**: 순수 함수 `run_wear(json) -> json` 을 두고 `#[wasm_bindgen]` 은 wasm32 에서만 얇게 감쌌다. → **동일 코드**가 네이티브·WASM 양쪽에서 돌아 **경계 왕복만 격리 측정**된다.

**fixture** (`micropitting-wasm/fixture_wear.json`): 4×2 격자, p_tran 에 **양압·0·음압(−1e8) 혼재** → `p≤0 → 0` 분기까지 왕복 확인.

```
WASM   : {"ok":true,"result":{"dh_w":{"nx":4,"ny":2,"data":[3.2982857142857253e-15, ... ]},"dh_w_mean":1.168142857142861e-15}}
NATIVE : (동일 문자열)
[PASS] 기준 ②
```

**손계산 교차검증** (자기대조 방지 — 코드가 코드를 검증하지 않도록 독립 유도):

| 항목 | 유도 | 값 |
|---|---|---|
| 접촉 반폭 | `b = 2·r_x·p_h/e_red = 2·0.01·1.5e9/115.38e9` | 2.6e-4 m |
| 접촉폭 | `ℓ_c = 2b` (`CONTACT_WIDTH_FACTOR`) | 5.2e-4 m |
| 사이클당 미끄럼 | `s_cycle = u_s·ℓ_c/ū = 0.02·5.2e-4/1.0` | 1.04e-5 m/cyc |
| 유효 마모계수 | `k = 0.3·(10·4e-10) + 0.7·4e-10` | 1.48e-9 |
| 마모깊이 | `k·p·s_cycle/H = 1.48e-9·1.5e9·1.04e-5/7e9` | **3.298e-15 m/cyc** |

→ 출력 `3.2982857142857253e-15` **일치**. `b` 는 §0.5 벤치 출력(2.600e-4)과도 교차 일치.

### 0.5 R2 해소 — M4 계산량 실측 (계획 §3.3)

계획은 *"병목은 `solve_fatigue`"* (`nz·ny` 컬럼 × `nx` 이력 MCE 최소화 = rayon 쓰던 자리)로 지목했다. **단일스레드 WASM · release · node v22 · 벽시계는 JS 측정**(크레이트에 시계 없음, wasm32 `std::time` 불가):

| 격자 (nx×ny×nz) | 벽시계 | b [m] | max σ_vM [Pa] |
|---|---|---|---|
| 128×16×15 | **123 ms** | 2.600e-4 | 9.659e8 |
| 256×32×15 | **272 ms** | 2.600e-4 | 9.678e8 |
| 512×64×15 | **1284 ms** | 2.600e-4 | 9.679e8 |

**판정**
- 256×32 = 272 ms → **인터랙티브 슬라이더 가능**.
- 512×64 = 1.28 s → **Web Worker 로 충분**.
- **`wasm-bindgen-rayon` 불필요** — 계획 §3.3 의 대응 ③(SharedArrayBuffer + COOP/COEP → `file://` 실행 불가 → 배포 단순성 상실)은 **도입하지 않는다**. "실측 후에만 판단" 조건이 이로써 종결.
- 부수 확인: max σ_vM 이 격자 세분화에 **수렴**(9.659→9.678→9.679e8), `b` 는 손계산값과 일치.

> ⚠️ **벤치 수치의 지위**: 하중장은 **계량용 합성**(정현 리플 + Hertz 포락)이다. **검증된 물리 결과가 아니라 타이밍 부하**이므로 `max D ≈ 3.0` 등에 물리적 의미를 부여하지 말 것.

### 0.6 부수 확인 — 병렬/직렬 등가성

**직렬이 오히려 빨랐다** (5.62s vs 6.79s). `ny`(4~64)가 작아 rayon 이득이 없다는 계획 §1.2 예측이 실측 확인됨. 동시에 기준 ④(직렬 오라클 green)는 **병렬/직렬이 동일 물리를 낸다는 기계 증명**이다 — 열 `j` 는 상호 독립이고 rayon `collect::<Vec<_>>()` 는 순서를 보존하므로 `cols` 가 동일하다.

### 0.7 코드 변경 — **비파괴 2건만**

| 파일 | 변경 | 성격 |
|---|---|---|
| `micropitting-model/Cargo.toml` | `rayon` → `optional = true`; `[features] default=["parallel"]` · `parallel=["dep:rayon"]` | 비거동 (default 유지) |
| `micropitting-model/src/m4_fatigue.rs` | `use rayon` 에 `#[cfg(feature="parallel")]`; `col_iter` cfg 분기 (`into_par_iter` ↔ `0..ny`) | 비거동 (③④ 양쪽 green) |

**CRB-main 통합 경로 영향 없음** — default = parallel 이므로 native Tauri 경로는 종전과 동일. 총괄계획 §1.1(*"사이드카/외부바이너리/WASM 없음"*) 저촉하지 않음. 의존은 **단방향**(`micropitting-wasm` → `micropitting-model`).

**신규** (`micropitting-wasm/`, 계획 §4.2 셸 — **물리 로직 0건**):

| 파일 | 역할 |
|---|---|
| `Cargo.toml` | cdylib+rlib; `micropitting_model` **default-features=false**; wasm-bindgen 은 `cfg(target_arch="wasm32")` 한정 |
| `src/lib.rs` | `run_wear`(순수) + `solve_wear_json`(wasm) · `run_stress_fatigue`(계량) · `WearArgs`/`WearParamsArgs` JSON 미러 |
| `fixture_wear.json` | 기준 ② 고정 입력 |
| `examples/native_ref.rs` | 네이티브 기준값 러너 (동일 `run_wear` 호출) |
| `verify_parity.js` | 기준 ② 대조 (문자열 → 실패 시 수치 하향 진단) |
| `bench_m3m4.js` | §0.5 계량 |
| `.gitignore` | `/target`·`/pkg`·`/pkg-node` |

**셸 설계 결정 (계획 §4.2 이행)**
- `WearInput<'a>` 의 **lifetime** 은 경계를 넘기지 않고 셸에서 조립.
- `WearParams` 는 serde 미파생 → `WearParamsArgs`(전 필드 `Option`) 미러. **기본값(k_lub·f_w)은 모델 크레이트가 소유** — 셸에 상수를 두면 SSOT 이중화.
- `contact_half_width`·`DEPTH_FRAC` 은 **모델 것을 호출**, 셸에서 재유도 금지.
- 응답에 `ok` 필드 강제 → 조용한 실패 차단(R5 동형).
- `deny_unknown_fields` → 오타 입력이 조용히 기본값으로 흐르는 것 차단.

### 0.8 작업 중 포착·조치

| # | 포착 | 조치 |
|---|---|---|
| 1 | fixture 초안이 `grid` 64×16 인데 `p_tran` 2×1 → `Field2::at(i,j)` **범위 초과 → WASM abort** | 격자 일치(4×2)로 수정. **경계 무결성 이전에 입력 정합 검증이 필요**하다는 신호 → Phase 2 정식 진입점에 grid↔Field2 차원 검사 추가 대상 |
| 2 | `Field2::max()` 가 `Option<f64>` 반환(빈 격자 방어) → `fold(f64::max)` 타입 불일치 | `filter_map` + `fold(0.0)` |
| 3 | `micropitting-wasm/target` = **896 MB** — `.gitignore` 부재 시 커밋 사고 | `.gitignore` 신설(모델 크레이트 `/target` 관례 동일 + `/pkg`·`/pkg-node`) |

### 0.9 판정

**Phase 0 통과. R1·R2 해소. 계획 §4(Phase 1 `reference.rs`) 착수 가능.**

계획 대비 갱신:
- **R1 → 종결**(의존성 wasm32 호환 확인).
- **R2 → 하향**(256×32 인터랙티브 / 512×64 Worker; rayon-wasm 불필요 확정).
- 산출물 크기 추정 **1~2 MB → 실측 329 KB**.

**미해소 이월**: R3(reference.rs tautology — Phase 1) · R4(`solve_partial` 스텁, 무증상) · R5(조용한 미수렴, 무증상) · R6(시간진화 역피팅) · R8(JS 물리 유입).

**재현 절차**
```bash
# ③④ 오라클 (프로젝트 루트)
cargo test --manifest-path "논문 취합/03. 정리/논문구현_P3/micropitting-model/Cargo.toml"
cargo test --manifest-path "논문 취합/03. 정리/논문구현_P3/micropitting-model/Cargo.toml" --no-default-features

# ①② 파리티 (micropitting-wasm/)
wasm-pack build --target nodejs --out-dir pkg-node --release
node verify_parity.js "$(cargo run --quiet --release --example native_ref)"

# §0.5 계량
node bench_m3m4.js
```
**환경**: cargo 1.95.0 · wasm-pack 0.15.0 · node v22.18.0 · target `wasm32-unknown-unknown`.
