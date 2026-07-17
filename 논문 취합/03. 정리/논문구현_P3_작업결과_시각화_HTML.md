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
