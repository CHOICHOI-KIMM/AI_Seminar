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

---

## Phase 1 — `src/reference.rs` (leaf) · **통과, R3 해소**

### 1.1 목적

문헌 폐형식이 전부 `#[cfg(test)]` 에 갇혀 뷰어가 참조곡선으로 그릴 수 없다 → **pub leaf 모듈로 이관**.
계획 §4.1. 최대 리스크 = **R3(오라클 tautology)** — 본 프로젝트에서 **이미 두 번 발생**한 실패 모드.

### 1.2 ★ 계획 정정 — 이관 대상 8곳 중 **1곳은 이관 불가**

계획 §4.1 은 `m5_wear.rs:266 vc_m5_archard_closed_form` 을 이관 대상으로 적었으나, **실제 코드를 읽어보니 독립 문헌 폐형식이 아니다** — 손유도 리터럴(1.428e-15) + 대수 항등이다.

**Archard 식[14] `Δh_w/Δn = k·p·u_s/H` 는 `m5_wear::wear_depth_rate` 자체**다. 이를 reference 로 복제하면 *같은 식의 복제본끼리 비교* → **R3 을 새로 만드는 꼴**(M2 Q oracle tautology 와 동형). → **Archard 는 실험 데이터만 등재**(`ARCHARD1953_FIG7_SLOPE_*`), 폐형식 미등재. M5 의 물리 앵커는 테스트 안의 손유도 리터럴·H 위치·k·p 선형성으로 **유지**.

> **판별 기준(향후 재사용)**: "우리 모델이 곧 그 식인가?" → 그렇다면 reference 에 두지 않는다. reference 는 **모델과 독립인** 문헌 폐형식·출판 데이터만.

**Archard Fig 7 기울기 원문 확인**(L306 verbatim): *"The slopes of these wear rate/load graphs are **1.00 for brass** and **0.98 for stellite** (standard error **0.015** in each case)."* → 실측 등재.

### 1.3 이관 결과 — **이관(move), 복제 아님**

기존 VC 를 전부 `crate::reference` 호출로 **결선**했다. 복제로 남기면 reference 는 이관이 아니라 **SSOT 이중화**이고 Phase 1 의 목적 자체가 사라진다.

| 문헌 | reference 항목 | 결선된 기존 VC |
|---|---|---|
| GW1994 식(15)(16)(17) | `gw1994_a`·`gw1994_h1_over_z1`·`gw1994_p1_over_z1` | `vc_m2_ar_gw1994_closed_form` |
| GW1994 Table1/2 | `GW1994_TABLE1_PRESENT`·`GW1994_TABLE2_KWEH`·`GW1994_E_PRIME_PA`·`GW1994_LAMBDA_M`·`GW1994_Z1_M` | `vc_m2_spot_gw1994_table1` |
| Venner1997 식(5)+Table1 | `venner1997_nabla`·`_amplitude_reduction`·`_nabla_from_ratio`·`VENNER1997_TABLE1`(18점)·`_ANCHOR_SPOTS`·`_HALF_CROSSING_BRACKET`·`_M`·`_L` | `vc_m2_comp_amplitude_venner` |
| Venner2000 식(29) | `venner2000_f_bar`·`_nabla2`·`_amplitude_reduction`·`VENNER2000_EXAMPLE`·`_EXAMPLE_NUMERICS` | `vc_m2_master_venner2000` |
| Tripp2003 식[10]/[16] | `tripp2003_normal_bisin`·`_tangential_bisin`·`_trace_normal`·`_trace_tangential` | `vc_m3_sin_normal`·`vc_m3_sin_tangential` |
| McEwen | `mcewen_axial_stresses`·`_von_mises_over_p0`·`_von_mises_peak`·`hertz_line_pressure` | `vc_m3_hertz_line_von_mises` |
| Milano2006 A.8/A.10/A.6 | `milano2006_tau_dv`·`_sigma_h`·`_z_star_uniaxial` | `vc_m4_milano_closed_form` |
| Desimone2006 식(7)·Table | `desimone2006_alpha_dv`·`DESIMONE2006_R_RATIO_TABLE` | (신규 등재) |
| Archard1953 **데이터만** | `ARCHARD1953_FIG7_SLOPE_BRASS/STELLITE/STD_ERR`·`GW1994_HALF_PUMPING_G` | (§1.2 — 폐형식 미등재) |

**효과**: 뷰어 참조곡선과 오라클 기대값이 **같은 코드**를 쓴다 → 화면에 그려지는 곡선 = 오라클이 지키는 곡선.

### 1.4 ★ leaf 불변식 — 구조 가드 + **가드 자체의 변이증명**

```text
(1) reference 는 types·units 외 크레이트 내부를 import 하지 않는다.
(2) m1~m6·partial_lub 의 생산 코드는 reference 를 참조하지 않는다.
    (#[cfg(test)] 안에서 기대값으로 쓰는 것은 허용 = 이 모듈의 용도)
```

`include_str!` + `#[cfg(test)]` 이전 절단으로 **생산 코드만** 검사(테스트의 정당한 import 는 false-positive 회피). 파일 I/O 없음 → wasm/CI 무관 동작.

**가드 자체를 변이증명**했다 — 테스트 안 된 가드는 가드가 아니다. `m5_wear.rs` 생산코드에 `use crate::reference::…` 주입 →

```
test reference::tests::reference_is_leaf_not_used_by_model_production_code ... FAILED
leaf 불변식 위반: m5_wear.rs 의 생산코드가 reference 를 참조 → 오라클 순환(tautology).
위반 줄: use crate::reference::ARCHARD1953_FIG7_SLOPE_BRASS;
```
→ **CAUGHT**(파일명·위반 줄 지목). 원복 후 green.

### 1.5 변이 게이트 — **6/6 CAUGHT · 범위 정확**

계획 §4.1 기준: *전건 FAIL 이면 결합 과다, 무FAIL 이면 오라클 사문화* — **둘 다 아니어야** 한다.

| 변이 | FAIL 한 테스트 | 건수 |
|---|---|---|
| Venner1997 eq(5) `0.17 → 0.34` | `vc_m2_comp_amplitude_venner` · `venner1997_eq5_reproduces_table1_anchor_spots` · `venner1997_inversion_roundtrips` | **3** |
| McEwen `sz` 부호 반전 | `vc_m3_hertz_line_von_mises` · `mcewen_peak_is_classical_value` | **2** |
| Tripp 식[10] `szz (1+ζz) → (1−ζz)` | `vc_m3_sin_normal` · `tripp2003_components_satisfy_trace_identity` | **2** |
| Milano A.8 `4τ_a² → 2τ_a²` | `vc_m4_milano_closed_form` · `milano2006_closed_form_spot_values` | **2** |
| GW1994 eq(15) `2λ → 3λ` | `vc_m2_spot_gw1994_table1` · `gw1994_table1_present_theory_spots` | **2** |
| Venner1997 Table1 `0.183 → 0.283` | `venner1997_eq5_reproduces_table1_anchor_spots` | **1** |

**판정**: 6/6 CAUGHT, 각 변이가 **관련 테스트만** 1~3건 떨어뜨림 = 결합 과다 아님 · 사문화 아님. **원복 후 106단위+2통합 green**(변이잔재 0).

주목: Tripp 변이를 **trace 항등**(`tripp2003_trace_normal`)이 포착 — 6성분식과 **독립 유도**된 정본 판별자(P2-1 L116)가 설계대로 전사오류를 가른다.

### 1.6 신규 오라클 (reference 자체 sanity — 전사 오류 검출)

19건 추가. 특기할 것:
- `line_and_point_nabla_are_not_interchangeable` — **선접촉(1997) ↔ 점접촉(2000) 카테고리 오류 차단**(총괄계획 L476)을 기계 고정. 같은 (λ/b,M,L) 에 두 ∇ 가 50 % 이상 달라야 통과 → 뷰어에서 두 축을 겹쳐 그리면 안 되는 이유가 코드에 박힘.
- `venner1997_half_crossing_bracket_matches_table1` — 상수 (0.25,0.5) 가 Table1 에서 **실제로 0.5 를 교차**하는지 = 상수의 근거 자체를 검증.
- `tripp2003_surface_boundary_conditions` — `z=0` 에서 `σzz=p`·`σxz=q`.
- `desimone2006_alpha_dv_matches_skf_value` — `3(1/√3−½) ≈ 0.2320508` = SKF 채택값 문헌 교차검증.

### 1.7 검증 — 전건 green

| 항목 | 결과 |
|---|---|
| 네이티브 `cargo test` (default) | **106단위 + 2통합 green** (87 + reference 19) |
| 네이티브 `cargo test --no-default-features` | **106단위 + 2통합 green** |
| WASM 빌드 (`wasm-pack --release`) | 무회귀 |
| 파리티 (기준 ②) | **문자열 동일 PASS** |

### 1.8 포착·조치

| # | 포착 | 조치 |
|---|---|---|
| 1 | 계획의 이관 대상 8곳 중 Archard 는 **독립 폐형식이 아님**(모델식 자체) | 데이터만 등재. §1.2 판별기준 수립 |
| 2 | `invert_lob` 이 민감도 대역·변이게이트에서도 사용 중 → 단순 치환 시 컴파일 실패 | reference 호출 클로저로 복원. 하드코딩 구간 (0.25,0.5) 도 `VENNER1997_HALF_CROSSING_BRACKET` 로 통일 |
| 3 | **`mv` 백업 원복 후 가드가 계속 FAIL** — 파일 내용은 원복(`git diff` 공백)이나 **백업의 옛 mtime 때문에 cargo 가 재빌드 생략**(stale 아티팩트) | `touch` 후 재빌드 → green. **교훈: `include_str!` 가드는 mtime 역행에 취약** — 변이 실험 시 원복 후 `touch` 필수 |

### 1.9 판정

**Phase 1 통과. R3(tautology) 해소** — leaf 불변식이 구조 가드로 강제되고, 가드 자체와 문헌값 양쪽이 변이증명됨.

**미해소 이월**: R4(`solve_partial` 스텁, 무증상) · R5(조용한 미수렴, 무증상) · R6(시간진화 역피팅) · R8(JS 물리 유입 — Phase 3).

**재현 절차**
```bash
# 전체 (default / 직렬 양쪽)
cargo test --manifest-path "논문 취합/03. 정리/논문구현_P3/micropitting-model/Cargo.toml"
cargo test --manifest-path "논문 취합/03. 정리/논문구현_P3/micropitting-model/Cargo.toml" --no-default-features
# leaf 가드만
cargo test --manifest-path "…/Cargo.toml" reference_is_leaf
```

---

## Phase 2 — 숙제 3건(무증상 실패 봉쇄) · **통과, R4·R5 해소**

### 2.1 목적 — "진입점 추가"가 아니다

Phase 0 이 남긴 숙제 3건은 **셋 다 무증상**(틀린 답이 정상처럼 보임)이다. Phase 0 을 `solve_wear` 하나로 좁게 자른 덕에 조용히 잠들어 있었을 뿐 사라진 게 아니다. → Phase 2 의 실질은 **틀린 상태를 도달 불가로 만드는 것**.

| # | 무증상 실패 | 봉쇄 방식 |
|---|---|---|
| 1 | `grid`↔`Field2` 차원 불일치 → 조용한 오독 | `check_dims` 를 조립 **직전** 관문으로 — 통과 못하면 `*Input` 이 존재하지 않음 |
| 2 | R4 `m2_lub::solve_partial` 스텁(`phi_bl=0`) → 건마모 소멸 | `partial_lub::` 만 import + 구조 가드 + 모델측 `#[deprecated]` |
| 3 | R5 미수렴 해가 정상 해와 구분 불가 | `_traced` 만 사용 + `Diagnostics` **비-`Option`** |

### 2.2 ★ 숙제 1 — 심각도 상향(실측)

Phase 0 §0.8 은 이를 "WASM abort"로 기록했으나, **실측 결과 그건 안전한 쪽이었다**.

`Field2::at` 은 `idx = i + j*self.nx` 로 **Field2 자신의 nx** 를 쓰고 호출측은 `grid.nx` 로 순회한다.

| 경우 | 결과 |
|---|---|
| Field2 **작음** (Phase 0 fixture) | 인덱스가 `data.len()` 초과 → **패닉**(시끄러운 실패) |
| Field2 **큼** | 인덱스가 범위 안 → **조용히 다른 원소를 읽음** ← 진짜 위험 |

실측: Field2 4×4, grid 2×2 에서 `at(1,1)` → idx **5**(기대 3). 게다가 `debug_assert!(i < self.nx)` 는 `i < grid.nx ≤ self.nx` 라 **구조적으로 통과**해 못 잡고, release(=`wasm-pack --release`)에선 **컴파일 아웃**된다. → 경계 검사만이 유일한 방어.

**패닉이 아니라 오류 반환인 이유**: WASM 에서 패닉은 abort = 모듈 인스턴스 오염 → 이후 호출까지 죽는다. 폼 입력 실수로 페이지를 새로고침하게 할 수는 없다.

**모델 `at()` 은 무변경**(동결 유지). 조용한 오독은 네이티브에도 있는 잠재 버그지만, 고치면 **동결 코드의 거동 변경**(오독→패닉)이라 연구자 판단 필요 → **별건 상정**(§2.7).

### 2.3 숙제 2 — `#[deprecated]` (비거동)

스텁에 `#[deprecated]` 부착. **거동 무변경**(속성만) — rayon 게이팅과 같은 성격. 효과는 **크레이트 전역**에서 오사용이 컴파일 경고로 드러나는 것 = 함정을 문서가 아니라 **컴파일러가 지킨다**.

실효성 확인(프로브 주입):
```
warning: use of deprecated function `m2_lub::solve_partial`:
  패스스루 스텁(phi_bl=0·q_tran=0). 실제 부분윤활은 partial_lub::solve_partial 사용.
  phi_bl=0 이 M5 로 가면 건마모가 조용히 사라진다.
```
스텁 자신의 테스트(`partial_passthrough`)에만 `#[allow(deprecated)]` — 그 테스트의 목적이 **스텁이 스텁임을 고정**(`phi_bl=0` assert)하는 것이라 정당.

경고는 무시될 수 있으므로 셸에는 **구조 가드**를 별도로 둔다(§2.5).

### 2.4 숙제 3 — 타입으로 진단 강제

```rust
pub struct PartialResponse { ok, result, diagnostics: Option<Diagnostics>, error }
//                                        ^ 성공 시 항상 Some — 만드는 경로가 _traced 뿐
```
핵심은 "trace 를 노출한다"가 아니라 **trace 없는 경로를 셸에서 없앤다**는 것. `partial_lub::solve_partial`(trace 버림)을 쓰지 않으므로 진단을 숨기는 코드를 **쓸 수가 없다**.

`Diagnostics`: `outer_converged`·`share_converged`·`outer_iters`·`share_iters`·`load_residual`·`flow_balance_residual`·`mu_eff`·`p_bar`·`asperity_degenerate`·`contact_count`.

### 2.5 구조 가드 + 변이 3/3 CAUGHT

Phase 1 의 `include_str!` 패턴 재사용(검증된 것). 가드 자체를 변이증명:

| 변이 | FAIL |
|---|---|
| 셸에 `use m2_lub::solve_partial as stub_solve` 주입 | `shell_never_reaches_m2_lub_stub` — 위반 줄 지목 |
| 셸에 `partial_lub::solve_partial(` (비-traced) 주입 | `shell_uses_only_traced_solver` — 위반 줄 지목 |
| `check_dims` 무력화(`if false`) | `check_dims_rejects_smaller_field` · `_larger_field_silent_misread` · `run_wear_rejects_dim_mismatch_without_panic` (3건) |

**범위 정확**(관련 테스트만), 원복 후 green.

### 2.6 검증

| 항목 | 결과 |
|---|---|
| 셸 `cargo test` | **7 green** |
| 모델 `cargo test` (default / 직렬) | **106+2 green 양쪽**, `deprecated` 경고 0 |
| WASM 실경로 `verify_phase2.js` | **8/8 PASS** |

WASM 실측: `dh_w_mean=1.168e-15`(**Phase 0 과 동일** = 파리티 무회귀) · `phi_bl=0.0664 ≠ 0`(**스텁 아닌 진짜 오케스트레이터**) · `outer=true share=true` · `load_residual=1.54e-15` · `outerIters=2 shareIters=28 muEff=0.0546`.

### 2.7 ★ 포착·조치

| # | 포착 | 조치 |
|---|---|---|
| 1 | **`wasm-pack.exe` OS 레벨 실행 차단**(`Access is denied`, bash·PowerShell 양쪽). 17:58 까지 정상 → 이후 차단. ACL FullControl 정상·Zone.Identifier 없음·파일 무손상 → **백신/EDR 휴리스틱 추정**(Defender 서비스는 0x800106ba 로 미응답 = 서드파티 AV 정황) | **wasm-pack 우회** — `cargo build --target wasm32` + `wasm-bindgen` CLI **2단계 직접 빌드**. wasm-pack 은 이 둘의 래퍼일 뿐이라 산출물 동등. CLI 버전은 Cargo.lock 의 wasm-bindgen 과 **정확히 일치** 필요(0.2.126). 절차를 `micropitting-wasm/Cargo.toml` 주석에 고정 |
| 2 | **stale pkg 로 옛 코드를 테스트해 "WASM 패닉"을 오진**. 원인: wasm-pack 이 조용히 실패했는데 내 grep 필터(`^error\|Done`)가 `Permission denied` 를 걸러냄 → 17:58 산출물이 그대로 남아 `check_dims` 없는 코드가 돌았다 | `verify_phase2.js` 에 **신선도 가드** 신설 — 진입점 3종 존재 + `wasm >= src` mtime 을 **테스트 전에** 확인, 미달 시 `exit 2`. **Phase 1 의 mv/mtime stale 과 동형 사고 2회째** → 가드를 스크립트에 영구 내장 |
| 3 | 미사용 import `StressResult` | 제거 |

> **교훈(누적 2회)**: stale 아티팩트는 **거짓 실패**(Phase 2, 패닉 오진)와 **거짓 성공**(Phase 1, 가드 통과 착각) 양방향으로 속인다. 빌드 산출물을 테스트하는 스크립트는 **신선도를 스스로 검증**해야 한다. 또한 **로그 필터가 실패를 숨길 수 있다** — `grep` 으로 빌드 출력을 좁히면 예상 밖 오류를 놓친다.

### 2.8 판정

**Phase 2 통과. R4·R5 해소 + Phase 0 숙제 3건 종결.** 세 무증상 실패가 전부 구조적으로 봉쇄되고, 봉쇄 자체가 변이증명됨.

**모델 크레이트 변경 누계 = 비거동 3건**(rayon 게이팅 · reference 결선 · `#[deprecated]`) → 계획 §0 "CRB 통합 경로 비파괴"·R7 유지.

**미해소 이월**: R6(시간진화 역피팅 — Phase 3) · R8(JS 물리 유입 — Phase 3) · **모델 `at()` 하드닝(별건, 연구자 판단 필요)** · wasm-pack 실행차단(환경).

**재현 절차**
```bash
# 셸 + 모델
cargo test                      # micropitting-wasm/ 에서 (7 green)
cargo test --manifest-path "…/micropitting-model/Cargo.toml"                        # 106+2
cargo test --manifest-path "…/micropitting-model/Cargo.toml" --no-default-features  # 106+2

# WASM (wasm-pack 차단 → 2단계 직접 빌드)
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target nodejs --out-dir pkg-node \
    target/wasm32-unknown-unknown/release/micropitting_wasm.wasm
node verify_phase2.js           # 신선도 가드 + 8/8
```
**환경 추가**: wasm-bindgen-cli 0.2.126 (wasm-pack 0.15.0 은 실행 차단 상태).

---

# 스프린트 (P3_HTML_spike, 2026-07-23 착수)

> **전략(연구자 승인)**: 빠른 결과 우선 — **스크래치 브랜치 `P3_HTML_spike` + 로컬 커밋(푸시 없음)**.
> 성공 → `P3_HTML` 병합 · 실패 → 브랜치 삭제(무흔적 롤백). 작업 위치 = worktree `AI_Seminar_P3`.
> **순서** = 죽을 것부터: **① web 빌드 스파이크 → ② reference 노출 → ③ 필드 진입점 → ④ 4탭**.
> **중단 기준**: ①에서 web 타깃 WASM 이 브라우저에서 근본적으로 구동 불가일 때 **만** 중단.
> 성능·레이아웃·개별 탭 문제는 수리 가능 → 중단 사유 아님.
>
> **정직성 불가침 5 (스프린트여도 유지)**:
> 1. 탭4(시간진화) **빈 채 유지** — 그럴듯한 곡선 금지 (R6, 역피팅 전례 `wf_2638d46e`)
> 2. JS 물리식 **0건** — 숫자는 전부 WASM/reference 경유 (R8)
> 3. 검증 탭 **A/B/C 등급 분리** — C(회귀가드)를 검증처럼 보이게 하지 않기
> 4. **미수렴 경고 배지** — `converged=false` 숨기지 않기
> 5. 탭3 **"미마모 형상·정성 전용(RP-Field)" 캡션**
>
> 미결이던 **탭2 VC 데이터 출처 = JSON 데이터 파일안으로 잠정 확정**(스프린트 판단, 연구자 재검토 대상).

## S① — web 빌드 스파이크 (계획)

**목적**: 유일한 전체 킬러 리스크 해소 — `--target web` WASM 이 **실제 브라우저 환경**에서
로드·실행되는가. 지금까지 검증된 것은 nodejs 타깃뿐(Phase 0~2). Phase 0 R1 과 동성격의
미검증 전제이므로 최선행.

**작업**:
1. `cargo build --target wasm32-unknown-unknown --release` (기존 산출물 재사용 가능)
2. `wasm-bindgen --target web --out-dir viewer/pkg` — **web 타깃 glue** 신규 생성
   (wasm-pack 실행차단 지속 → Phase 2 확립한 2단계 직접 빌드 그대로)
3. 최소 테스트 페이지 `viewer/spike.html`: 모듈 로드 → `solve_wear_json`(Phase 0 fixture)
   + `solve_partial_json` 호출 → 결과를 DOM 에 기록
4. **로컬 서버 필수**: web 타깃 glue 는 `.wasm` 을 fetch 로 로드 → `file://` 불가.
   `python -m http.server` 사용. (SharedArrayBuffer 미사용 → COOP/COEP 불요 = 계획 §3.3 결정 유지)
5. **헤드리스 브라우저로 기계 판정**: Edge(`msedge --headless`) 로 페이지 열어 DOM 덤프
   → `[SPIKE-PASS]` 문자열 확인. 수동 클릭 불요·재현 가능.

**통과 기준**:
- (a) web 타깃 빌드 성공 + 페이지 로드 시 JS 예외 없음
- (b) `solve_wear_json` 출력 == **nodejs/네이티브 기지값**(`dh_w_mean=1.168142857142861e-15`,
  Phase 0 파리티 앵커) — 문자열 동일
- (c) `solve_partial_json` 이 `phi_bl≠0` + `diagnostics.outerConverged=true` 반환
  (진짜 오케스트레이터·진단 채널이 브라우저에서도 생존)
- (d) 신선도 가드(진입점 존재·mtime) 페이지에 내장 — stale 3회차 방지

**리스크**: 헤드리스 Edge 부재/차단 시 → 대안: 결과를 fetch 로 서버에 회신하는 self-report 페이지
또는 연구자 수동 1회 확인(최후). wasm-bindgen CLI 는 이미 확보(0.2.126, Phase 2).

### S① 결과 (2026-07-23) — **통과, 중단 기준 미발동**

| 기준 | 결과 |
|---|---|
| (a) web 타깃 빌드·로드 | ✅ `wasm-bindgen --target web` 성공(446KB) · 실브라우저(Edge)서 glue+wasm fetch·초기화 확인(서버 액세스 로그) |
| (b) 파리티 앵커 | ✅ `dh_w_mean == 1.168142857142861e-15` 문자열 동일 (네이티브=nodejs=**web 3자 일치**) |
| (c) 오케스트레이터·진단 | ✅ `phi_bl≠0` + `outerConverged=true` 브라우저 생존 |
| (d) 신선도 가드 | ✅ 페이지 내장(진입점 3종 검사) |
| 차원검사 | ✅ 브라우저에서도 `{ok:false}` (abort 아님) |

**기계 판정**: 헤드리스 Edge → 페이지 **self-report** `GET /__spike__/SPIKE-PASS/fails=0`
(Edge `--dump-dom` 이 Windows 서 stdout 0바이트 quirk → 계획된 폴백 사용. 서버 로그 = 판정 채널).
**빌드 절차**: Phase 2 확립 2단계 그대로 + `--target web`. 로컬 서버 `python -m http.server` 필수(`file://` 불가) 확정.
→ **S② 진행.**

## S② — reference 노출 (결과) — **통과**

**설계**: 셸에 2 진입점 — `reference_curve_json(kind, params)`(곡선 샘플러: venner1997·venner2000·gw1994·mcewen·milano·tripp2003) + `reference_tables_json()`(정적 표·상수 일괄). 수식 평가는 전부 `reference.rs` 호출, 셸은 **샘플링 루프만**(좌표 생성은 물리 아님) → JS 에는 완성 배열만 = R8 유지. 곡선 단위 반환으로 경계 호출 수천회 회피(Phase 3 숙제 1 해소).

**정직성 장치 내장**: venner1997 meta 에 `lineContactOnly`·venner2000 에 `pointContactOnly` 플래그(축 겹침 금지, 총괄계획 L476) · `fitDegradesMask` 시리즈(원문 자인 과소예측역 음영용) · Table1 18점·앵커 스팟·브래킷 동봉.

**검증**: 셸 **11 green**(기존 7 + s2 4: pass-through bit-exact 등가·McEwen peak·tables 완비성·오류경로). 로그축 min≤0 등 구조적 오류 반환.
→ **S③ 진행.**

## S③ — 전체 체인 진입점 (결과) — **통과**

**설계**: `solve_chain_json` 1개 — partial_lub(traced)→M3(`solve_stress_at_depths`)→M4→M5 배선. **크기 대책**: 6성분 전체 필드(수십 MB) 대신 뷰어가 그릴 것만 — y₀ 슬라이스 `vm_xz`(nz×nx)·x-프로파일(p/h/q/Δh_w)·**(y,z) Dang Van·수명 맵**(M4 의 D 는 x=시간이력 broadcast 라 (y,z)가 정보 전부). **정직성**: `unwornGeometry:true` 플래그 동봉(RP-Field 캡션 강제용) + `diagnostics` 비-Option 유지.

**검증**: 셸 **13 green**(+s3 2). 포착 1건: 스모크 fixture 의 `phi_bl=0` — 스텁 아닌 **fixture 물리**(rough2=0 → 유막이 접촉 삼킴). 검증된 verify_phase2 fixture(0.23+0.06µm)로 정렬해 해소. **교훈: phi_bl=0 은 "스텁" 과 "정당한 무접촉" 두 원인이 있다 — 뷰어도 이 구분을 표시해야**(asperity_degenerate·contact_count 노출이 근거).
→ **S④ 진행.**

## S④ — 뷰어 4탭 (결과) — **통과 · 스프린트 완주**

**구성** (`viewer/`): `index.html`(4탭) + `plot.js`(캔버스 렌더 전용, 물리 0건) + `worker.js`(WASM 호출 전담 module worker — R2 대응 UI 비블로킹) + `vc_data.json`(재검토판 전사 29건, **잠정 — 연구자 재검토 대상**) + `spike.html`(S①).

| 탭 | 내용 | 정직성 장치 |
|---|---|---|
| ① 참조곡선 | 6종(venner1997/2000·gw1994·mcewen·milano·tripp2003) + Table1 18점 오버레이 | 선/점접촉 배지(겹침 금지)·fit degrades 음영·앵커=anti-fudge 문구 |
| ② 검증 | VC 29건 색분리 표 | A/B/C/X/OPEN 등급 정의 상단 고정·X(DesiTab)=정상거동 표기·출처·잠정 명시 |
| ③ 단일스텝 | 폼(원논문 Table1 기본값)→Worker→체인: p/q·h/Δh_w 프로파일 + σ_vM(x,z)·D(y,z) 히트맵 | **미마모·정성 전용 경고 고정**·미수렴 경고배지·φ_bl=0 두 원인 구분 표시(S③ 교훈) |
| ④ 시간진화 | **곡선 0개** — 안내문만(창발구조 L435·정량임계 금지 L440·anti-fudge L476·전례 wf_2638d46e) | 불가침 1 그대로 |

**기계 판정(헤드리스 Edge, self-report)**: `initDone → gotTables → drewCurve → loadedVC → 스모크 체인 → `**`VIEWER-PASS`**.

**포착·조치**: ⚠️ **`--virtual-time-budget` 이 module worker 의 wasm 인스턴스화를 얼림** — `init()` 이 resolve/reject 모두 안 됨(단계 리포터로 특정: `msg/refTables` 수신 후 `initDone` 부재). 실시간 실행(budget 제거+kill)으로 전환 → 전 시퀀스 완주. **stale(2회)과 동류의 '하네스가 속이는' 3번째 사례 — 이번엔 하네스가 거짓 실패를 만듦.** 뷰어 결함 아님. 단계 리포터는 진단용으로 존치(무해·catch 처리).

**최종 검증**: 모델 **106+2 green**(default·직렬) · 셸 **13 green** · 브라우저 **VIEWER-PASS**.

## 스프린트 판정 — **성공 → P3_HTML 병합**

S①~S④ 전건 통과·중단 기준 미발동·정직성 불가침 5 전건 유지. 커밋 4개(S①~S④)를 `P3_HTML` 로 병합(로컬, **푸시는 별도 결정**). Phase 3 잔여를 스프린트가 흡수했으므로 G-VIS 게이트 항목 중 미완은: 참조곡선 **정량 오라클 전건 일치의 문서화 대조**(§7 G-VIS 1 — 코드 오라클은 green이나 뷰어 표시값 대조 절차 미실시)·R8 구조가드(뷰어 JS 검사 자동화) — **후속 마감 대상**.

### S④ 후속 — file:// 무증상 정지 수정 (연구자 실사용 보고, 2026-07-23)

**증상**: 탐색기 더블클릭(= `file://`)으로 열면 "WASM 로딩 중…"에서 **조용히 정지**.
**원인**: web 타깃 glue 의 fetch·module worker 는 `file://` 에서 브라우저가 차단(S① 계획 명시 제약). 더 나쁜 건 **module script 는 `plot.js` import 자체가 죽어 모듈 안의 어떤 감지 코드도 실행 불가** → 무증상.
**수정**: (a) **classic script** file:// 가드 — module 밖에서 감지해 실행방법 안내 표시(무증상→유증상, 이 저장소 원칙대로) (b) `뷰어실행.bat` — 더블클릭으로 서버+브라우저 자동. (c) http 경로 회귀 = **VIEWER-PASS 재판정**.
**교훈**: 배포물의 첫 실사용 실패는 기능이 아니라 **실행 경로**에서 났다 — 가드는 반드시 "죽는 지점 바깥"(classic script)에 둬야 한다.

## S⑤ — 탭3 알고리즘 도식 (연구자 추가 지시, 2026-07-23) — **통과**

**지시**: 원논문 Fig 1(PDF)·P2 재구성 그림 참고, 탭3 좌측에 계산 알고리즘 도식.
**원본 판독**: MinerU 추출 `2011_SKF_Micropitting_pipeline_images/img_0001.jpg` = Fig 1 — d=0 → **Mixed-EHL in t** → n+Δn → Dang Van ln(N)=F → Miner d+=n/N → **d>1?** → Removal(pit) / Wear Model 형상갱신 → 루프. **Fig 1 은 시간루프가 뼈대** → 그대로 그리면 뷰어가 미구현부까지 하는 것처럼 보임.

**연구자 4문 사인오프**: ① 기준 = **구현 기준 재구성 + 미구현 회색·잠금** ② 배치 = 탭3 좌측 고정 ③ 인터랙션 = **박스 클릭 → 해당 그래프 스크롤+플래시**(잠금 박스 → 탭④ 이동; 실시간 단계표시는 체인이 단일 WASM 호출이라 불가 — 가짜 진행바는 부정직이라 배제) ④ 라벨 = 한글+기호.

**구현**: 인라인 SVG(의존성 0·물리 0). 컬러 실선 = 정적 체인(입력→[M1·M2→M6 partial_lub 그룹]→M3→M4·M5, 식번호 표기) / **회색 점선+🔒 = 시간루프 3단**(Δn·Miner 누적 → d>1 피트제거·형상갱신 → A_p 출력) + Fig1 대응 주석·루프 화살표. 하단 캡션: *"원논문 Fig 1 의 구현 기준 재구성 — 1:1 아님(Fig1 은 시간루프가 뼈대)"*. 부수: **URL 해시 탭 딥링크**(#tab1~4, 검수·공유용).

**검증**: http 회귀 **VIEWER-PASS**(실시간) + **스크린샷 시각 검수**(#tab3 딥링크로 캡처 — 도식·잠금부·캡션·레이아웃 정상).

## S⑥ — 입력 기준형상 플롯 + h_tran 음수 주석 (연구자 지시, 2026-07-23) — **통과**

**배경(연구자 Fig 6 대조 질의)**: 원논문 Fig 6(a)는 (x,y) 3D — 뷰어는 y₀ 슬라이스. 차이 3층 분석 — ① 표시(S③ 크기 대책, 데이터는 존재) ② **입력**(실측 광대역 2D 거칠기 vs 합성 사인 y-불변 → 우리 필드는 사실상 1D) ③ 상태(last wear step vs 미마모 정적). 후속으로 기준형상 표시·음수 주석 지시.

**연구자 4문 사인오프**: 두 표면+합성+h̄ 기준선 / **worker echo**(표시=투입 동일 배열 — JS 재계산 금지, 드리프트 원천 차단) / 음영+note / 전폭·프로파일 위.

**구현**: ① `roughCanvas` — z₁·z₂·합성(점선)+h̄ 수평선, 수식 캡션 동적 `z_i(x)=√2·Rq_i·sin(2πNx/l_x)`(√2=RMS→진폭 환산, **입력 규격이지 물리 아님** 명시) ② 유막 플롯 h<0 **붉은 음영**(contactMask) + 범례 "(음수=접촉간섭)" + note("클램프 없는 선형 중첩 간극함수·하중은 M1 dry 가 φ_bl 분담·Fig 6(a) 음수 표시와 동계열") ③ 검수용 `?autorun`(부팅 후 체인 자동 실행 → `autorunPlotted` self-report) + 렌더 오류 self-report.

**★포착·조치 — serde_json 의 ∞→null**: autorun 이 `chainError: fmt null.toExponential` 적발 → node 스캔으로 **`dvYz` 32/480 null** 특정. 원인 = **Dang Van D 분모 `τ_f−a_DV·p̂`≤0 인 표면층에서 D=∞**(물리적으로 의미: 어떤 수명에서도 기준 위배)를 serde_json 이 null 로 직렬화. **모델 무변경**(D=∞ 는 정직한 출력) — **표시 계층**을 null-안전화: 히트맵 ∞ 전용색(마젠타-레드)+컬러바 범례 "= ∞ (D 기준 위배)", fmt·linePlot 비유한 가드. **교훈: WASM→JSON 경계에서 f64 비유한값은 소리 없이 null 이 된다 — 표시 계층은 null 을 '숨길 수 없는 표시'로 변환할 것.**

**검증**: `chainError` 재발 없음 + **`autorunPlotted`**(렌더 경로 무예외 기계 확인) + VIEWER-PASS 유지.

## S⑦·S⑧ — 표시 정련 (연구자 피드백 라운드, 2026-07-23) — **통과**

**S⑦ (4dcc79b·580f245)**: 범례 **플롯 프레임 밖 상단 밴드**(곡선과 구조적 비겹침) · y눈금 우측정렬(회전 단위라벨과 분리) · 단위 **µm/GPa**(원논문 Fig 6 표기 정합, 표시 계층 환산) · **Δh_w 우측 보조축**(8자릿수 차로 불가시했던 것을 배율 조작 없이 제 크기로) · 진폭 명시 A₁·A₂·**A_합성=√2(Rq₁+Rq₂)**(연구자 선택=수식 계산) · **Δh_w 단위 nm/Mcyc**(연구자 µm/cyc 검토→1e-9 대라 지수 불가피로 기각, ×1e15 순수 환산으로 O(1) 숫자 — 1.4e-15 m/cyc = 1.4 nm/Mcyc ≈ "72만 사이클 시험 전구간 ~1 nm").

**S⑧**: 히트맵 **실단위 µm 눈금**(x·z 축, niceTicks) · **`depth_frac` 폼 입력**(z_max/b, 기본 0.25 유지 — 현 65µm 는 논문 Fig 6b 50µm 보다 이미 깊음; 셸 경계 knob, `solve_stress_at_depths` 임의 깊이라 **모델 무변경**, 상한 4b 검증) · **제목 "미마모·정성" 삭제 + 상단 배너 유지**(연구자 사인오프 — 중복 제거이지 RP-Field 표기 포기가 아님).

**물리 노트(연구자 z범위 질의 답)**: 뷰어 σ_vM 이 세로 줄무늬인 것은 범위 문제가 아니라 **입력 차이** — 균일 p̄ + 단일주파수 리플이라 리플응력은 ~λ/2π≈14µm 감쇠 후 DC 균일값. 논문의 깊이방향 bulb 는 Hertz 매크로 분포의 산물. z 확대는 감쇠 관찰용이지 bulb 재현이 아님.

**검증**: 셸 13 green · web 재빌드(신선도 확인 wasm>src) · autorunPlotted. **하네스 실수 1건**: 체인 끝 `&` 가 전체를 백그라운드화 → 옛 로그 판정할 뻔 — 신선도 가드·타임스탬프 대조로 적발(stale 계보 인식이 방어로 작동).

## S⑨ — 히트맵 contourf 스타일 (연구자 Fig 6b 대조 요청, 2026-07-23) — **통과**

**진단**: 블록·줄무늬 원인 = 보간 없음 + 연속 컬러맵 + 성긴 nz(셋의 중첩; 데이터 문제 아님).
**연구자 3문 사인오프**: 보간+이산 레벨(contourf) / viridis 유지+레벨화(jet 은 지각왜곡으로 비권장 수용) / nz 기본 15→31.
**구현**: heatmap 을 **픽셀 쌍선형 보간(ImageData) + 10단 레벨 양자화**로 재작성 — 저응력 하단이 논문처럼 한 색의 연속 띠. 컬러바도 이산 밴드. **∞ 셀 처리 유지**(이웃에 ∞ 있으면 최근접 코너 폴백 — D 기준위배 띠를 보간으로 뭉개지 않음). 표시 계층만 변경(데이터 불변·표준 시각화).
**검증**: autorunPlotted(신규 타임스탬프) · 셸/모델 무변경.
