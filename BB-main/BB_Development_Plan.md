# BB Contact Analysis System — 개발 계획서 (ACBB)

> CRB-main 의 SW 체계를 모태로 **단열 앵귤러콘택트 볼베어링(ACBB)** 해석 SW 를 개발하기 위한 실행 계획.
>
> - **작성일**: 2026-08-20
> - **수식 근거**: [BB_Development_Theory.md](BB_Development_Theory.md) — **본 계획서는 수식을 새로 정의하지 않는다.** 모든 식은 Theory 의 절·식 번호로 참조한다.
> - **모태**: `d:/AI/AI_Seminar_CRB/CRB-main` (Phase 4 스냅샷)
> - **작업 로그**: [BB_Development_Action.md](BB_Development_Action.md)

---

## 1. 결정 사항 (확정)

| ID | 항목 | 결정 | 근거·비고 |
|---|---|---|---|
| **D-1** | 평형 DOF | **코드는 5-DOF 로 1회 구현.** 검증만 2단계로 — **P3-1**(`DofMask::ISO_3DOF` 구속 → Level C·D-1) → **P3-2**(`FULL` 해방 → Level D-2) | ISO 16281 A.1 이 확장 허용. Harris Table 7.4 대조는 3-DOF 상태여야 성립. `bearing.rs` 는 두 번 쓰지 않는다 |
| **D-2** | 베어링 범위 | **단열 ACBB + 축방향 예압** | 복열 조합(DB/DF/DT)·4점접촉은 범위 밖 (§8.2) |
| **D-3** | 고속 효과 | **경고만** — `n·D_pw > 1×10⁶ mm/min` 시 "정적 가정 범위 밖" 표시, 계산은 수행 | ISO 16281 A.4 는 식을 주지 않음 (Theory §5) |
| **D-4** | 접촉응력 | **구현** — Harris Ch.6 (6.38)~(6.46) 로 `a`, `b`, `p_H,max` 산출 | Theory §6. T-4 해소됨 |
| **D-5** | 하이브리드 볼 | **제외** (강제 볼만) | ISO 가 하이브리드 `a_ISO` 를 정의하지 않음 (Theory §7.7) |
| **D-6** | 윤활 계층 | **`κ` + 점접촉 유막(Hamrock-Dowson elliptical)** | HMEHL·조도·열보정 전체 이식은 범위 밖 |
| **D-7** | 좌표계 축 명명 | **ISO 규약 — X = 회전축** | Theory 의 ISO 식을 치환 없이 전사. CRB/TRB 의 `Z = 축방향` 과 다름 (§3.4) |
| **D-8** | `φ_j` 원점 | **고정** `φ_j = 2π(j−1)/Z` + **위상 스윕 옵션** 별도 제공 | CRB 의 하중방향 정렬은 5-DOF 에서 worst-case 를 보장하지 못함 (§3.4.3) |
| **D-9a** | 틸트 **키네마틱** 팔 | **`R_i`** (식 A.4) | CRB 의 `d_pw/2` 는 볼에서 틸트 감도를 과소평가. ISO (A.2)(A.5) = Harris *Adv* (1.13)(1.14) |
| **D-9b** | **모멘트** 팔 (`M_y`·`M_z`) | **`R_i`** — 키네마틱 팔과 통일 | 가상일·기하 분석 결과 `R_i` 가 공액 (Theory §4.5). Harris & Mindel (89)(90) 5-DOF 원전과 일치. ISO (A.8) 의 `D_pw/2` 는 피치원 근사이며 Annex A 는 informative |
| **D-9c** | 틸트 항 | **선형화** `R_i γ` (사인 없음) | 원전 둘 다 선형(Harris *Adv* 1.13 · H&M 81). 5-DOF 성분별 사인은 문헌에도 없고 엄밀하지도 않다. `R_i` 와 함께 가상일 공액을 정확히 만족 |
| **D-10** | 단위 | **솔버 내부 `mm`·`N`·`rad`**, UI 경계에서 `μm`·`kN`·`°` 변환 | `c_P` [N/mm^1.5], `Σρ` [1/mm], Harris 0,0236 계수가 모두 mm·N 기준 |
| **P-1** | Phase 구성 | 솔버 **수직 관통** P1→P6, 각 Phase 말미에 검증 Level 부착 | |
| **P-2** | 프론트엔드 | **솔버 완성 후 마지막(P6)**. 그때까지 `@ts-nocheck` stub 유지 | CRB 가 Phase 1 에서 쓴 방식 |

---

## 2. 범위

### 2.1 포함

- 단열 ACBB (초기 접촉각 α₀ ≠ 0), 강제 볼·강제 레이스웨이
- 축방향 예압 (음의 운전 클리어런스 또는 예압 하중 입력)
- 5-DOF 정적 평형 (δx, δy, δz, γy, γz) — ISO 3-DOF 를 부분집합으로 포함
- 점접촉 타원 Hertz: `χ`, `c_P`, `δ`, `a`, `b`, `p_H,max`
- 볼별 운전 접촉각 `α_j`, 볼 하중 `Q_j`
- ISO 16281 §5.2 기준수명 `L_10r`, 수정 기준수명 `L_nmr`
- ISO 281 `C_r`, `C_u`, `a_ISO`, `κ`
- 점접촉 EHL 유막 `h_c`, `h_min`, `Λ`

### 2.2 제외 (명시적)

| 항목 | 사유 | 재검토 시점 |
|---|---|---|
| 복열 조합 (DB/DF/DT) | D-2 | 단열 완성 후 |
| 4점접촉 (QJ) | D-2. ISO §5.1 이 복열 ACBB 근사를 허용하므로 복열 구현 시 자동 확보 | 복열 이후 |
| DGBB 전용 UI | ACBB 코드의 α₀ = 0 특수 케이스로 자연 포함 (Theory §10.2) | — |
| 스러스트 볼베어링 | 식 (3)(4)(12)(16), `a_ISO` (37)~(39) 별도 분기 필요 | 미정 |
| 원심·자이로 (고속) | D-3. ISO 식 부재 | 필요 시 Jones(1960) 도입 |
| 하이브리드 볼 | D-5 | — |
| HMEHL·혼합윤활·열보정 | D-6 | — |
| 과도해석(transient), WEC | 초기 범위 밖 | — |
| 마찰·토크 | 초기 범위 밖. 확보 문헌 3편은 후속용 | — |

---

## 3. 아키텍처

### 3.1 계층 구조 (CRB 3계층 → BB 2계층)

```
Level 1 (Bearing)  5-DOF 평형 → 볼별 δ_j, α_j, Q_j        ← 유지·개편
Level 2 (Ball)     단일 점접촉 (타원 Hertz)                ← 신규 (CRB Level 2·3 통합 대체)
                   ※ 슬라이스 계층 없음
```

CRB 의 Level 2(롤러 축방향 하중분포) + Level 3(슬라이스 Hertz) 이 **볼에서는 단일 점접촉 하나로 축약**된다. Gen1/Gen3 이중 모드 개념 자체가 소멸한다.

### 3.2 모듈 처분

> **시드 시점 `mod.rs` 실태 (2026-08-20 조사)**: 아래 "시드 상태" 열은 BB 시드(CRB Phase 4 스냅샷) 시점에 **실제로 컴파일되고 있었는지** 를 뜻한다. 비활성 모듈은 CRB 화 과정에서 주석 처리된 상태로, **컴파일된 적이 없다.**

| 파일 | 시드 상태 | 처분 | Phase |
|---|---|---|---|
| `types.rs` | 활성 | 재작성 | P1 |
| `geometry.rs` | 활성 | 재작성 | P1 |
| `hertz.rs` | 활성 | **P1 에서 일시 비활성화** → 재작성 후 재활성화 | P1(비활성)/P2 |
| `bearing.rs` | 활성 | **P1 에서 일시 비활성화** → 재작성 후 재활성화 | P1(비활성)/P3 |
| `life.rs` | **비활성** | 재활성화 + 재작성 (ISO §5.3 → §5.2) | P4 |
| `static_rating.rs` | **비활성** | 재활성화 + 계수 교체 (ISO 76 볼) | P4 |
| `lubrication.rs` | **비활성** | 재활성화 + 부분 재사용 (§3.3) | P5 |
| `gen1.rs`, `gen3.rs`, `beam.rs` | 활성 | **삭제 완료** | P1 ✅ |
| `rib_contact.rs` | 비활성 | **삭제 완료** (`hertz_elliptical_coefficients` 이관 후) | P1 ✅ |
| `hmehl.rs`, `transient.rs`, `transient_io.rs`, `wec_risk.rs` | 비활성 | **삭제 완료** | P1 ✅ |

### 3.3 CRB 에서 그대로 재사용 가능한 자산 (조사 완료)

코드를 직접 확인해 **볼에 바로 쓸 수 있는 것**을 확정했다. 재작성하지 말고 이관한다.

| 자산 | CRB 위치 | BB 이관처 | 근거 |
|---|---|---|---|
| `hertz_elliptical_coefficients(r_x, r_y)` | `rib_contact.rs:125` | `hertz.rs` | Brewe-Hamrock 근사. 계수 `1.0339/0.6360`, `1.5277/0.6023`, `1.0003/0.5968` 가 **Harris (6.33)~(6.35) 와 정확히 일치** (Theory §6.4). `χ` 초기값·검산용 |
| `hamrock_dowson_elliptical(u,g,w,k)` | `lubrication.rs:2562` | `lubrication.rs` 유지 | Hamrock & Dowson (1981) Eqs 7.31/7.33 **타원 접촉 원식**. 볼이 본토라 CRB 보다 오히려 자연스러움 |
| `viscosity_at_temp`, Walther/Roelands 점도 | `lubrication.rs` | 유지 | 접촉 형상 무관 |
| `κ`, `ν₁` (ISO 281 27~29) | `life.rs` | 유지 | 접촉 형상 무관 |
| `e_C` 표 | `life.rs` | 유지 | 접촉 형상 무관 |
| Newton-Raphson 수치 유틸, 수렴 판정 | `bearing.rs` | 참고 재사용 | 구조만 |
| Tauri command 배선, 결과 직렬화 패턴 | `lib.rs`, `commands` | 유지 | |

> ⚠️ **재사용 자산 2개가 비활성 모듈 안에 있다 (2026-08-20 조사)**: `hamrock_dowson_elliptical`(`lubrication.rs`)과 `κ`·`ν₁`·`e_C`(`life.rs`)는 시드 시점에 `mod.rs` 에서 주석 처리돼 **컴파일된 적이 없다.** 따라서 P4·P5 는 "이관"이 아니라 **"재활성화 → 컴파일 통과 → 검증"** 순서로 시작해야 하며, 그 비용을 Phase 산정에 포함해야 한다. (`hertz_elliptical_coefficients` 는 P1 에서 이관 완료 — 순수 함수라 무비용이었다.)
>
> ⚠️ 반대로 **재사용하면 안 되는 것**: `a_ISO` 롤러 계수(1.5859/1.3993/1.2348, 지수 0.4·−9.185) — 볼은 (31)~(33) 의 2.5671/2.2649/1.9987, 지수 0.83·1/3·−9.3 이다 (Theory §7.8). `C_u` 롤러 계수 0.2453·`C_0/8.2` → 볼은 0.2288·`C_0/22`.

### 3.4 좌표계 · 단위 규약 (D-7 ~ D-10)

#### 3.4.1 축 정의 — ISO 16281 규약 채택

ISO 16281:2025 Clause 4 및 Figure A.1 a) (ISO p.22 / PDF p.28, 육안 확인 완료):

```
        Z ⊗ ────→ X          X = 베어링 회전축
          │                  Y = 반경방향 (도면상 아래)
          ↓ Y                Z = 반경방향 (지면 속), 우수좌표계
```

| 미지수 | 물리 | 축 | 잔차 | ISO 대응 |
|---|---|---|---|---|
| `δ_x` | 축방향 변위 | X | `F_x` | `δ_a` — 식 (A.7) |
| `δ_y` | 반경 변위 1 | Y | `F_y` | `δ_r` — 식 (A.6) |
| `δ_z` | 반경 변위 2 | Z | `F_z` | (5-DOF 확장) |
| `γ_y` | 미스얼라인먼트 (about Y) | — | `M_y` | (5-DOF 확장) |
| `γ_z` | 미스얼라인먼트 (about Z) | — | `M_z` | `ψ` — 식 (A.8) |

5-DOF 일반화 (Theory §4.2, §4.3 의 (A.2)(A.5) 확장):

```
반경 성분  R_j = A cos α₀ + δ_y cos φ_j + δ_z sin φ_j
축   성분  X_j = A sin α₀ + δ_x − R_i (γ_z cos φ_j − γ_y sin φ_j)

δ_j = max(0, √(R_j² + X_j²) − A)          α_j = atan2(X_j, R_j)
Q_j = c_P δ_j^(3/2)

잔차   F_x = Σ Q_j sin α_j
       F_y = Σ Q_j cos α_j cos φ_j
       F_z = Σ Q_j cos α_j sin φ_j
       M_y = −R_i Σ Q_j sin α_j sin φ_j
       M_z = +R_i Σ Q_j sin α_j cos φ_j
```

> **팔은 `R_i` 로 통일, 틸트는 선형화** (D-9b·D-9c, 2026-08-20 확정).
> 이 형태는 **Harris & Mindel (1973) 식 (81)(82)(86)~(90) 의 정적 환원형과 동일**하며
> **가상일 공액**이라 야코비안이 대칭이다. 근거는 Theory §4.4 확정 블록 · §4.5 분석.
>
> ISO (A.8) 은 모멘트 팔에 `D_pw/2` 를 쓰므로 본 구현과 다르다. Annex A 가 informative 이고
> A.1 이 확장을 명시 허용하므로 규격 준거는 유지된다. 편차는 약 0,5 % (Theory §4.5 편차표).

**구속 `δ_z = γ_y = 0` → ISO 식 (A.2)(A.5)(A.6)(A.7)(A.8) 과 항등.** 이 항등성이 Level D-2 의 판정 기준이다.

> ⚠️ **CRB/TRB 계보와 축 이름이 다르다.** CRB 코드는 `X = 수평 반경, Y = 수직 반경, Z = 샤프트축` (`types.rs` D5) 이고 `Manual/09` 도 `δz = axial` 로 서술한다. **CRB 코드를 참고할 때 축 이름을 그대로 옮기면 안 된다.** 대응: `CRB X → BB Y`, `CRB Y → BB Z`, `CRB Z → BB X`, `CRB γ_x → BB γ_y`.
>
> 또한 CRB 는 이미 3-DOF `(δx, δy, γx)` 로 축소된 상태(D6/D7)라 **재사용할 5-DOF 구조 자체가 없다.** `bearing.rs` 는 알고리즘(Newton-Raphson·line search·step limiting) 구조만 참고하고 수식은 Theory 에서 직접 옮긴다.

#### 3.4.2 틸트 모멘트 팔 — `R_i` (D-9)

$$R_i = \frac{D_{pw}}{2} + \left(r_i - \frac{D_w}{2}\right)\cos\alpha_0 \qquad \text{(Theory §2.3, 식 A.4)}$$

볼의 틸트는 **홈 곡률중심**을 움직이므로 피치반경이 아니라 `R_i` 가 모멘트 팔이다. CRB 의 `d_pw/2` 를 쓰면 틸트 감도가 과소평가된다 (예: `D_w` = 20 mm, `r_i` = 0,52 `D_w`, α₀ = 25° → 약 +0,36 mm 차이).

#### 3.4.3 볼 각위치 `φ_j` (D-8)

$$\varphi_j = \frac{2\pi(j-1)}{Z}, \qquad j = 1 \ldots Z$$

**고정 원점**을 쓴다 (`φ_1 = 0` 이 `δ_y` = Y축 방향). 검증 재현성을 위해 ISO 정식화를 그대로 따른다.

CRB 의 `φ_load = atan2(F_y, F_x)` 하중방향 정렬은 **채택하지 않는다.** 5-DOF 에서는 반경하중 방향과 모멘트 축이 독립이라 단일 정렬각으로 worst-case 를 보장할 수 없기 때문이다.

대신 **케이지 위상 스윕**을 별도 기능으로 제공한다:

```
for φ₀ in linspace(0, 2π/Z, n_phase):     // 기본 n_phase = 36
    φ_j = φ₀ + 2π(j−1)/Z
    해석 → Q_max, p_H,max, L_10r 기록
출력: 각 지표의 최악값 + 발생 위상
```

볼 해석은 `O(Z)` 라 36회 스윕도 밀리초 단위로 끝난다 (Theory §8). 기본 해석은 `φ₀ = 0` 단일 실행, 스윕은 사용자 토글.

#### 3.4.4 단위 (D-10)

| 위치 | 길이 | 힘 | 모멘트 | 각도 | 응력 |
|---|---|---|---|---|---|
| **솔버 내부** | **mm** | **N** | **N·mm** | **rad** | MPa |
| UI / JSON 경계 | μm (변위), mm (기하) | kN | N·m | ° | MPa |

내부를 ISO 단위로 통일하는 이유: `c_P` [N/mm^(3/2)], `Σρ` [1/mm], Harris (6.39) 의 `0,0236`, (6.43) 의 `2,79×10⁻⁴` 가 모두 mm·N 기준이다. 지수가 `3/2`, `2/3`, `1/3` 로 분수라 중간 단위환산이 끼면 실수가 나기 쉽다.

**변환은 입출력 경계 한 곳에서만** 수행한다 (`commands` 계층). 솔버 내부에는 `1000.0` 같은 환산 상수가 등장하지 않아야 한다 — 이를 Level A 의 점검 항목으로 둔다.

---

### 3.5 데이터 모델 스케치 (P1 에서 확정)

```rust
struct BallBearingGeometry {
    d_w:      f64,   // 볼 직경 D_w [mm]
    d_pw:     f64,   // 피치직경 D_pw [mm]
    z:        u32,   // 볼 수 Z
    r_i:      f64,   // 내륜 홈 곡률반경 [mm]  (기본 0.52 D_w, Theory §2.4)
    r_e:      f64,   // 외륜 홈 곡률반경 [mm]  (기본 0.53 D_w)
    alpha_nom: f64,  // 공칭 접촉각 α [rad]  — 정격하중용 (ISO 281)
    clearance: Clearance,  // 운전 클리어런스 또는 예압
}

enum Clearance {
    Diametral(f64),      // G_r op [mm], 음수면 예압
    InitialAngle(f64),   // α₀ [rad] 직접 입력 → G_r op 역산
    AxialPreload(f64),   // 예압 하중 F_a0 [N] → P3 에서 δ_a0 로 변환
}

/// 하중 무관 전처리 결과 (해석당 1회, Theory §8)
struct ContactPrecomputed {
    a_dist:  f64,   // A = r_i + r_e − D_w
    alpha_0: f64,   // 초기 접촉각
    r_i_c:   f64,   // R_i
    gamma:   f64,   // γ = D_w cos α / D_pw
    sum_rho_i: f64, sum_rho_e: f64,
    chi_i:  f64, chi_e: f64,      // (E.1) 해
    k_i: f64, e_i: f64,           // K(χ_i), E(χ_i)
    k_e: f64, e_e: f64,
    c_p:    f64,                  // 스프링상수 (40)
}

struct BallResult {
    phi:    f64,   // 각위치 ϕ_j
    delta:  f64,   // 탄성변형 δ_j  (A.2)
    alpha:  f64,   // 운전 접촉각 α_j (A.5)
    q:      f64,   // 볼 하중 Q_j
    a_i: f64, b_i: f64, p_h_i: f64,   // 내륜 접촉타원·응력
    a_e: f64, b_e: f64, p_h_e: f64,   // 외륜
    loaded: bool,
}

/// 5-DOF 내륜 변위 — ISO 규약 (D-7). 단위 mm / rad (D-10)
struct BearingDisplacement {
    d_x: f64,   // 축방향 (X = 회전축)   ≡ ISO δ_a
    d_y: f64,   // 반경 (Y)              ≡ ISO δ_r
    d_z: f64,   // 반경 (Z)              — 5-DOF 확장
    g_y: f64,   // 틸트 about Y          — 5-DOF 확장
    g_z: f64,   // 틸트 about Z          ≡ ISO ψ
}

/// DOF 구속 마스크 — ISO 3-DOF 모드 검증용 (Level D-1)
struct DofMask { d_x: bool, d_y: bool, d_z: bool, g_y: bool, g_z: bool }
// ISO 3-DOF 모드 = { d_x: free, d_y: free, d_z: fixed(0), g_y: fixed(0), g_z: free }
```

> **3-DOF ↔ 5-DOF 호환 규약**: §3.4.1 참조. `d_z = g_y = 0` 구속이 ISO 식과 항등해야 하며, 이 항등성이 P3 Level D-2 의 판정 기준이다.

---

### 3.6 프론트엔드 구조 — TRB 현행 ↔ BB 목표

> 조사일 2026-08-21. 신 Phase 4 착수 전 전수 조사 결과.
>
> ⚠️ **전제 정정**: `src/` 는 CRB 가 아니라 **TRB(테이퍼 롤러) 코드**다. 앱 타이틀이
> `TRB Contact Analysis`, 프로젝트 저장 확장자가 `.trb.json`, `alpha_rib`·`r_rib_circ`·
> 리브 접촉 로직이 곳곳에 남아 있다. 모든 파일 헤더에 `// CRB Phase 1.4 stub … Phase 6
> 에서 정식 재작성 예정` 주석이 붙어 있고, 그 "Phase 6" 이 지금의 **신 Phase 4** 다.

#### 3.6.1 왜 바꾸는가 — TRB ↔ ACBB 물리 대비

프론트의 모든 뷰는 **롤러 선접촉**을 전제로 설계되어 있다. 볼로 바뀌면 그 전제가 사라진다.

| # | 개념 | TRB (현행 프론트의 전제) | ACBB (BB 솔버) | 프론트에 미치는 영향 |
|---|---|---|---|---|
| 1 | **접촉 형태** | 선접촉. 롤러 유효길이 `L_we` 를 `n_slices` 로 분할, 슬라이스별 `q_k`·`b_k`·`p_k` | **점접촉 1개.** 타원 `(a, b, p_max)` 가 볼당 내륜·외륜 2쌍 | 축방향 분포 차트가 **전멸**한다. 대신 **타원 형상 뷰**가 필요 |
| 2 | **접촉각** | 롤러 경사 `γ = (α_i+α_o)/2` — 기하로 **고정** | 볼별 `α_j` 가 **하중에 따라 변한다** (`α_j = atan2(X_j, R_j)`) | CRB·TRB 에 없던 자유도. **`α_j(φ)` 곡선**과 단면도의 `α₀ ↔ α_j` 겹쳐그리기가 신설 대상 |
| 3 | **회전축** | **Z** 축 | **X** 축 (ISO 규약, D-7) | 3D 뷰·단면도의 좌표 전면 교체. `CRB X→BB Y`, `CRB Y→BB Z`, `CRB Z→BB X` |
| 4 | **평형 자유도** | 결과 표시는 사실상 반경 위주 | **5-DOF** `(δ_x, δ_y, δ_z, γ_y, γ_z)` | 결과 카드가 **5성분 + 접촉각 범위 + 수렴정보**로 재구성 |
| 5 | **해석 모드** | Gen1(독립 슬라이스) ↔ Gen3(빔 결합) **이중 모드** | **단일 모드** (D-1) | `ComparisonView`·`DualModeToggle`·`RollerComparisonChart` 의 **존재 이유 소멸** |
| 6 | **리브 접촉** | 대단부 리브 = **타원 점접촉** (별도 뷰) | 리브 없음 | 뷰는 죽지만 **코드는 산다** — 이 저장소에서 유일하게 타원 점접촉 히트맵을 그린다 (§3.6.5) |
| 7 | **프로파일** | 크라우닝 `Δz(x)` — 로그/원호/포물선/커스텀 **곡선** | **오스큘레이션 스칼라 2개** `f_i = r_i/D_w`, `f_e = r_e/D_w` | **그릴 자유도 자체가 없다.** `ProfileView` 는 개조 불가 |
| 8 | **수명** | 슬라이스(라미나)별 수명 적산 | 볼별 `Q_ci`/`Q_ce` → ISO 16281 §5.2 | `LifeChart` 는 신 **P5** 에서 신규. 지금은 대상 아님 |
| 9 | **윤활** | HMEHL · 마이크로피팅 · 리브 EHL · 슬라이스 유막 | 점접촉 Hamrock-Dowson (`κ`, `Λ`) | `LubricationView` 2 439줄이 **전부 폐기 모듈 참조**. 신 **P6** 에서 신규 |
| 10 | **과도 / 열속도** | transient 솔버, ISO 15312 | **폐기** (Plan §2) | `TransientView`·`ThermalSpeedView` 삭제 |

> **요약**: 1·2·7 이 이번 Phase 의 핵심이다. 「축방향 분포가 사라지고 타원 형상이 들어오며,
> 접촉각이 상수에서 변수가 된다」 — 이 세 문장이 프론트 변경안 전체를 결정한다.

#### 3.6.2 현행 인벤토리 (전 44파일, 11 067줄)

`invoke` 열의 ~~취소선~~ 은 **등록 해제된 죽은 커맨드**다. 등급은 A 재활용 / B 개조 / C 대체 / D 삭제.

**components/ (32파일 · 9 868줄)**

| 파일 | 줄수 | nocheck | invoke | 등급 | 근거 |
|---|---:|:---:|---|:---:|---|
| `AlertPanel/index.tsx` | 41 | | | **A** | `alerts[{level, category, message}]` → BB 는 `{level, code, message}`. `category`→`code` 한 줄 |
| `ProgressBar.tsx` | 41 | | | **A** | `listen('solver-progress')` 만 사용. 물리 무관 |
| `charts/PlotWithCopy.tsx` | 158 | | | **A** | Plotly 래퍼 + 우클릭 데이터 복사(TSV/CSV/JSON) |
| `charts/plotlyDefaults.ts` | 35 | | | **A** | `darkLayout` / `plotConfig` / viridis |
| `shared/DetailTable.tsx` | 26 | | | **A** | 순수 표시 컴포넌트 |
| `CanvasArea/index.tsx` | 89 | | | **B** | 탭 셸 유지. 11탭 중 4개 제거 + 신설 (§3.6.4) |
| `GeometryView/index.tsx` | 326 | ✓ | | **B** | `DetailTable` 나열 구조 유지. `l_we`·`d_we`·리브·크라우닝 행 → `A`·`α₀`·`f_i/f_e`·`Σρ`·`γ`·`g_r_op`·`n·D_pw` (BB `GeometrySummary` 와 거의 1:1) |
| `ResultsCard/index.tsx` | 492 | ✓ | | **B** | 접이식 사이드바 셸 유지. 표시 물리량 교체 (5-DOF 변위·`α_j` 범위·`Q_max`·`loaded_count`·`p_max`) |
| `BearingView3D/index.tsx` | 262 | ✓ | | **B** | R3F/three 뼈대 유지. 테이퍼 롤러 메쉬 → `sphereGeometry`, 축 Z→X |
| `InputPanel/index.tsx` | **1696** | ✓ | presets 6 · `solve_bearing` · ~~`solve_bearing_dual`~~ ~~`parse_load_csv`~~ ~~`solve_transient`~~ | **B** | 아코디언·프리셋·필드검증 **프레임워크가 자산**. Geometry/Profile/Transient/Dual 섹션 대량 삭제 + BB 4블록 재매핑 → **실질 절반 재작성** |
| `InputPanel/FieldGroup.tsx` | 68 | | | **B** | 필드 그룹 프리미티브. 라벨·단위만 |
| `charts/LoadDistChart.tsx` | 455 | ✓ | | **B** | 원주방향 `Q(ψ)` 극좌표/막대 골격 우수. `rollers[].q_total` → `ball_results[].q_n`, 축방향 서브플롯 제거, **`α_j(φ)` 곡선 추가** |
| `charts/StressContourChart.tsx` | 508 | | | **B** | 히트맵 인프라 유지. (슬라이스 × 접촉폭) 격자 → 단일 타원 내 `p(x,y)`, 내/외륜 탭 유지 |
| `charts/RibContactDetailChart.tsx` | 354 | ✓ | | **B ⭐** | **이 저장소에서 유일하게 이미 타원 점접촉 히트맵을 그린다.** 리브 접촉 → 볼–궤도 접촉 치환이 최단 경로 |
| `ProfileView/index.tsx` | 590 | ✓ | | **C** | 대비표 #7 — 그릴 자유도 자체가 없음 (§3.6.5) |
| `SectionView2D/index.tsx` | 447 | ✓ | | **C** | 형상 계산부(사다리꼴 롤러·리브·γ 경사축) 약 200줄 폐기. **치수선 헬퍼는 살릴 값어치 있음** (§3.6.5) |
| `LubricationView/index.tsx` | **2439** | ✓ | ~~`run_hmehl`~~ | **C** | 최대 파일이자 최대 사망자. 참조 모듈 전부 폐기. 신 P6 에서 요약 카드 수준으로 신규 |
| `charts/LifeChart.tsx` | 349 | ✓ | | **C** | `result.life` 없음. 신 P5 에서 레이아웃만 참고 |
| `charts/ContactPatchChart.tsx` | 145 | | | **C** | 슬라이스를 x축으로 쓰는 패치 컨투어. `RibContactDetailChart` 로 대체됨 |
| `ComparisonView/index.tsx` | 2 | | | **D** | re-export 스텁. Gen1/Gen3 폐기 |
| `ContourMap/index.tsx` | 2 | | | **D** | re-export 스텁 (실체 없음) |
| `ResultCharts/index.tsx` | 2 | | | **D** | re-export 스텁 |
| `DualModeToggle.tsx` | 30 | | | **D** | Gen1/Gen3 토글 그 자체 |
| `ThermalSpeedView/index.tsx` | 133 | | | **D** | `result.thermal_speed` 없음. 대응 모듈 폐기 |
| `TransientView/index.tsx` | 66 | | | **D** | transient 폐기 |
| `TransientView/SliceSlidingContour.tsx` | 168 | | | **D** | 슬라이스 개념 자체 |
| `TransientView/RollerDynamicsChart.tsx` | 157 | | | **D** | 롤러 동역학 |
| `TransientView/DamageRiskView.tsx` | 127 | | | **D** | WEC 위험도 — 모듈 폐기 |
| `TransientView/TransientTimeChart.tsx` | 105 | ✓ | | **D** | 시간이력 |
| `charts/ComparisonChart.tsx` | 239 | | | **D** | Gen1/Gen3 비교 |
| `charts/RollerComparisonChart.tsx` | 160 | ✓ | | **D** | Gen1 vs Gen3 롤러 오버레이 |
| `charts/RollerDetailChart.tsx` | 156 | ✓ | | **D** | 단일 롤러 슬라이스 분포 |

**components/ 밖 (12파일 · 1 199줄)**

| 파일 | 줄수 | invoke | 등급 | 근거 |
|---|---:|---|:---:|---|
| `types/bearing.ts` | **784** | | **C** | **전부 TRB.** Rust `types.rs` 기준 전면 재작성. 살아남는 것은 `Alert`·`AlertLevel`·`Material` 정도. **프론트 전환의 첫 작업** |
| `store.ts` | 80 | | **B** | `useReducer` + Context 1개. **구조가 좋아 유지.** dual/transient 필드·액션 3종만 제거 |
| `App.tsx` | 100 | | **B** | Context 제공 + 레이아웃. 타이틀 `TRB` → `BB` |
| `defaults.ts` | 124 | | **C** | TRB 기본 입력값. BB 스키마로 교체 |
| `project.ts` | 34 | | **B** | `.trb.json` → `.bb.json` |
| `hooks/useActiveResult.ts` | 11 | | **B** | dual 분기 제거 시 `state.result` 반환 한 줄. **호출부 유지 위해 남기는 편이 diff 가 작다** |
| `hooks/useSolver.ts` | 56 | ~~`compute_slice_geometry`~~ ~~`compute_hertz_single_slice`~~ | **D** | 둘 다 죽은 커맨드. 어디서도 import 되지 않음 |
| `main.tsx` | 10 | | **A** | 엔트리 |

**집계**: A 6 · B 12 · C 6 · D 14 (스텁 3 포함) · 기타 6.
`@ts-nocheck` **13파일**. **죽은 `invoke` 6종** — `solve_bearing_dual` · `parse_load_csv` · `solve_transient` · `run_hmehl` · `compute_slice_geometry` · `compute_hertz_single_slice`.

> ⚠️ **등록만 되고 아무도 호출하지 않는 커맨드 2종**: `compute_geometry` · `compute_contact`.
> 신 `GeometryView` 와 접촉타원 뷰의 **연결 지점**이다.

#### 3.6.3 구조 다이어그램

**현행** — 붉은 노드가 죽은 경로다.

```mermaid
graph LR
  subgraph RUST["Rust · commands.rs"]
    CG["compute_geometry<br/>(등록됨 · 미사용)"]
    CC["compute_contact<br/>(등록됨 · 미사용)"]
    SB["solve_bearing"]
    PR["presets 6종"]
    DEAD["solve_bearing_dual · run_hmehl<br/>solve_transient · parse_load_csv<br/>compute_slice_geometry · compute_hertz_single_slice"]
  end
  subgraph FE["React"]
    APP["App.tsx<br/>AppContext"]
    ST["store.ts<br/>useReducer"]
    IP["InputPanel<br/>1696줄"]
    CA["CanvasArea<br/>11 탭"]
    RC["ResultsCard"]
    AP["AlertPanel"]
  end
  subgraph TABS["탭 11개"]
    T1["Geometry"]; T2["Profile"]; T3["Section"]; T4["3D View"]
    T5["Load Dist"]; T6["Stress Contour"]; T7["Lubrication"]; T8["Life"]
    T9["Thermal Speed"]; T10["Comparison"]; T11["Transient"]
  end
  APP --> ST
  APP --> IP
  APP --> CA
  APP --> RC
  APP --> AP
  IP -->|invoke| SB
  IP -->|invoke| PR
  IP -.->|죽음| DEAD
  CA --> T1 & T2 & T3 & T4 & T5 & T6 & T7 & T8 & T9 & T10 & T11
  T7 -.->|죽음| DEAD
  CG -.->|미연결| FE
  CC -.->|미연결| FE

  classDef dead fill:#4a1520,stroke:#c0392b,color:#f5b7b1
  classDef idle fill:#3a3520,stroke:#b7950b,color:#f9e79f
  class DEAD,T2,T7,T9,T10,T11 dead
  class CG,CC idle
```

**목표** — 초록이 신설, 파랑이 개조, 회색이 유지.

```mermaid
graph LR
  subgraph RUST["Rust · commands.rs"]
    CG["compute_geometry"]
    CC["compute_contact"]
    SB["solve_bearing"]
    PR["presets 6종"]
  end
  subgraph FE["React"]
    APP["App.tsx<br/>AppContext"]
    ST["store.ts<br/>dual·transient 제거"]
    TY["types/bb.ts<br/>Rust types.rs 기준 재작성"]
    IP["InputPanel<br/>BB 4블록"]
    CA["CanvasArea"]
    RC["ResultsCard<br/>5-DOF 요약"]
    AP["AlertPanel"]
  end
  subgraph TABS["탭"]
    G["Geometry<br/>compute_geometry 연결"]
    S["Section<br/>축단면 + α₀↔α_j"]
    L["Load<br/>Q_j·α_j 극좌표"]
    E["Contact Ellipse<br/>a·b·p_max"]
    D3["3D View<br/>볼 세트"]
  end
  APP --> ST --> TY
  APP --> IP --> CA
  APP --> RC
  APP --> AP
  IP -->|invoke| SB
  IP -->|invoke| PR
  G -->|invoke| CG
  E -->|invoke| CC
  CA --> G & S & L & E & D3
  P5["신 P5 : Life 탭"]:::later
  P6["신 P6 : Lubrication 탭"]:::later
  CA -.-> P5
  CA -.-> P6

  classDef new fill:#123524,stroke:#27ae60,color:#a9dfbf
  classDef mod fill:#12283a,stroke:#2980b9,color:#aed6f1
  classDef later fill:#2c2c2c,stroke:#7f8c8d,color:#bdc3c7,stroke-dasharray:4 3
  class E,S,TY new
  class G,L,D3,IP,RC,CA,ST mod
```

#### 3.6.4 탭 구성 — 3안 병기

현행 11탭 중 **4탭이 즉시 사망**(`Profile`·`Thermal Speed`·`Comparison`·`Transient`)하고
2탭이 **후속 Phase 로 이월**(`Life`→P5, `Lubrication`→P6)한다. 남는 5탭을 어떻게 구성할지 3안.

| 현행 탭 | 안 A (5탭 · 이월 숨김) | 안 B (7탭 · 자리 확보) | 안 C (4탭 · 초집중) |
|---|---|---|---|
| Geometry | ✅ 개조 | ✅ 개조 | ✅ 개조 |
| Profile | ➡ **Contact Ellipse 로 교체** | ➡ **Contact Ellipse 로 교체** | ➡ **Contact Ellipse 로 교체** |
| Section | 🔄 **BB 축단면 신규** | 🔄 **BB 축단면 신규** | 🔄 **BB 축단면 신규** |
| 3D View | ✅ 개조 | ✅ 개조 | ❌ 이번엔 제외 |
| Load Distribution | ✅ 개조 (+`α_j(φ)`) | ✅ 개조 (+`α_j(φ)`) | ✅ 개조 (+`α_j(φ)`) |
| Stress Contour | 🔀 Contact Ellipse 에 흡수 | 🔀 Contact Ellipse 에 흡수 | 🔀 흡수 |
| Lubrication | ❌ 숨김 (P6 에서 부활) | ⏳ 빈 탭 "P6 예정" | ❌ 숨김 |
| Life | ❌ 숨김 (P5 에서 부활) | ⏳ 빈 탭 "P5 예정" | ❌ 숨김 |
| Thermal Speed / Comparison / Transient | ❌ 삭제 | ❌ 삭제 | ❌ 삭제 |
| **결과 탭 수** | **5** | **7** | **4** |

| | 장점 | 단점 |
|---|---|---|
| **안 A** | 화면에 **동작하는 것만** 남아 검증에 집중된다. 빈 탭이 없어 「이건 왜 비어 있지」가 생기지 않는다 | P5·P6 에서 `CanvasArea` 와 `CanvasTab` 타입을 다시 건드린다 (각 2줄) |
| **안 B** | 최종 형태가 처음부터 보인다. P5·P6 는 컴포넌트만 갈아끼우면 됨 | **빈 탭 2개가 계속 보인다.** 검증용 화면에 노이즈. 「미구현」 표시를 별도로 만들어야 함 |
| **안 C** | 가장 빠르다. 3D 는 미세한 수치 차이를 못 읽으므로 검증 가치가 낮다 | 볼 배치·접촉각의 **공간적 직관**을 잃는다. 3D 개조는 262줄로 크지 않다 |

> **권장: 안 A.** 「접촉/하중을 먼저 눈으로 검증한다」가 이 Phase 의 목적이므로,
> 화면에 **검증 가능한 것만** 두는 편이 목적에 맞다. P5·P6 의 탭 추가 비용은 각 2줄이다.

#### 3.6.5 대체 판정 상세 — `ProfileView` 와 `SectionView2D`

**`ProfileView` (590줄) — 개조 불가, 삭제**

내용은 롤러 유효길이 `L_we` 를 x축으로 하는 `Δz(x)` 프로파일 5종(롤러 크라우닝 / 내륜 궤도 /
외륜 궤도 / 내륜 합성 / 외륜 합성)이고, 코어는 `Δz = δ_c·(2x/L_we)²` 류의 크라우닝 식과
로그 크라운·다항식·커스텀 보간이다. x 격자는 `solver.n_slices` 로 나눈다.

BB 에서 이 뷰의 기반이 **전부 소멸**한다:

| | 이유 |
|---|---|
| (a) | 볼은 점접촉이라 **"축방향 위치 x" 가 없다** |
| (b) | 크라우닝·로그 프로파일·엣지응력 완화는 **선접촉 엣지로딩 대책**이라 대응 개념이 없다 |
| (c) | `n_slices` 가 BB `SolverParams` 에 **존재하지 않는다** |
| (d) | 볼–궤도 형상은 **스칼라 2개(`f_i`, `f_e`)로 완결**된다 — 곡선을 그릴 자유도 자체가 없다 |

살릴 것은 SVG 차트 셸(축 스케일링·주석·우클릭 복사)뿐이고 그것은 `PlotWithCopy` 로 이미 대체된다.
→ **Profile 탭을 Contact Ellipse View 로 교체**한다. 볼 `j` 선택 → 내/외륜 접촉타원 2개를
실제 축척으로 겹쳐 그리고(`a`, `b`, `a/b`, `p_max`) 궤도 홈폭 대비 장축 침범(truncation) 경고를 표시.
**`RibContactDetailChart` 를 개조하는 편이 `ProfileView` 를 고치는 것보다 훨씬 빠르다.**

**`SectionView2D` (447줄) — 형상부는 폐기, 그러나 새 뷰의 값어치가 크다**

현행은 TRB 축방향 반단면 SVG 다. `computeSectionGeometry()` 가 롤러 축 경사 `γ = (α_i+α_o)/2` 를
잡고 사다리꼴 롤러 4점 폴리곤, 내륜(리브 팁·리브 배면 8점)/외륜(6점) 폴리라인, 접촉선, 리브 라인,
리브 접촉점(`h_c`, `α_rib`, `r_rib_circ`)까지 그린다 — **형상 로직 대부분이 테이퍼+리브 전용**이다.

| 살아남는 것 | 죽는 것 |
|---|---|
| `viewBox` + `scale(1,-1)` 좌표 뒤집기 · 치수선/지시선 `Annotations` 유틸 · `d`/`D`/`B`/`D_pw` 스케일 계산 | 사다리꼴 롤러 · `γ` 경사축 · 리브 전체 · `α_i`/`α_o` 이중 접촉각 · `d_we_max/min` · `l_we` |

**BB 축단면 뷰는 오히려 TRB 보다 정보량이 많다:**

- 볼은 **원**(반경 `D_w/2`, 중심 `r = D_pw/2`), 궤도는 홈반경 `r_i`/`r_e` 의 **원호** (폴리곤 아님)
- 곡률중심 `O_i`·`O_e` 를 찍고 그 사이 거리 **`A = r_i + r_e − D_w`** 를 도시
  → **단면도가 곧 BB 해석의 교과서 그림**이 된다 (Theory §2 의 (A.3) 그 자체)
- 무하중 `α₀` 선과 하중 후 `α_j` 선을 **겹쳐** 그려 **대비표 #2 의 새 자유도**를 시각화
- 축을 **X = 회전축**으로 잡고 `δ_x` 를 화살표로

→ **새 파일로 작성하되 `Annotations`/치수선 헬퍼와 `viewBox` 셋업은 복사**한다.
형상 계산부 약 200줄은 통째로 버린다.

#### 3.6.6 상태 · 타입 변경

**`store.ts` (80줄) — 구조 유지, 사망 항목만 제거**

| 항목 | 조치 |
|---|---|
| `AppState.dualResult` · `dualViewMode` · `transientResult` | 삭제 |
| `SET_DUAL_RESULT` · `SET_DUAL_VIEW_MODE` · `SET_TRANSIENT_RESULT` | 삭제 (액션 12종 → 9종) |
| `CanvasTab` 유니온 | 11 → 5 (안 A 기준) |
| `SET_RESULT` 의 `dualResult: null` | 정리 |
| 나머지 (`useReducer` + Context 1개, props drilling 없음) | **그대로 유지** — 구조가 좋다 |

**`types/bearing.ts` (784줄) — 전면 재작성**

Rust `src-tauri/src/solver/types.rs` 가 단일 진리원(SSOT)이다. 프론트가 참조하는
`result.life` · `result.film_thickness` · `result.thermal_speed` · `result.rollers[].slices[]` 는
BB `BearingResult` 에 **하나도 없다**. 현재 `BearingResult` 는

```ts
{ geometry: GeometrySummary, equilibrium: BearingEquilibrium,
  phase_sweep: PhaseSweepResult | null, alerts: Alert[], elapsed_ms: number }
```

뿐이며, `BearingEquilibrium.ball_results[]` 가 볼별 `(φ_j, δ_j, α_j, Q_j, loaded, a/b/p_max × 2)` 를 담는다.

> ⚠️ **스키마 드리프트가 이번 Phase 순서 변경의 유일한 리스크**다 (CLAUDE.md 체크리스트 1번).
> 신 P5(수명)·P6(윤활)에서 `BearingResult` 에 필드가 **추가**되면 TS 타입을 다시 손대야 한다.
> 방지책은 P4-S1 착수 전에 결정한다.

#### 3.6.7 작업 분해 (신 Phase 4)

| 단계 | 내용 | 산출 | DoD |
|---|---|---|---|
| **S1** | `types/bearing.ts` 대체 · 죽은 파일 삭제(D등급 14) · `store.ts` 정리 | 타입 SSOT 확립 | `npm run build` 통과, `@ts-nocheck` 잔여 0 (살아남은 파일 기준) |
| **S2** | `InputPanel` BB 4블록 재매핑 (`geometry`/`material`/`operating`/`solver`) + `ClearanceSpec` 3종 · `PreloadModel` 2종 · `DofMask` · 위상스윕 | 입력 가능 | `solve_bearing` 왕복 성공 |
| **S3** | `GeometryView` 개조 + `compute_geometry` 첫 연결 · `ResultsCard` 5-DOF 요약 · `AlertPanel` | 기하·요약 확인 | Level A 결과가 화면과 일치 |
| **S4** | `LoadDistChart` 개조 (`Q_j` 극좌표 + **`α_j(φ)` 곡선**) · Contact Ellipse View (`RibContactDetailChart` 개조 + `compute_contact` 연결) | **핵심 검증 뷰** | Level C·D 결과가 화면과 일치 |
| **S5** | BB 축단면 뷰 신규 · `BearingView3D` 개조 · 타이틀/확장자 정리 | 공간 직관 | `npm run build` + `npm run lint` 통과 |

**Phase DoD**: `npm run build` + `npm run lint` 통과, 살아남은 전 뷰가 실제 솔버 출력으로 동작,
`@ts-nocheck` 0, 죽은 `invoke` 0.

---

## 4. Phase 계획

### Phase 1 — 데이터 모델 + 기하 (Theory §2)

**작업**
1. 롤러 전용 모듈 삭제: `gen1.rs`, `gen3.rs`, `beam.rs`, `hmehl.rs`, `transient*.rs`, `wec_risk.rs`
2. `rib_contact.rs` 에서 `hertz_elliptical_coefficients` 를 `hertz.rs` 로 이관 후 파일 삭제
3. `types.rs` 재작성 — §3.4 스케치 확정. 슬라이스·프로파일·rib·Gen 모드 필드 전면 제거
4. `geometry.rs` 재작성 — `A`(A.3), `α₀`(A.1), `R_i`(A.4), `γ`, `Σρ_i/Σρ_e`(E.4)(E.5), `F_i(ρ)/F_e(ρ)`(E.6)(E.7)
5. **`hertz.rs`·`bearing.rs` 를 `mod.rs` 에서 일시 비활성화** — `types.rs` 를 ACBB 로 재작성하면 두 모듈이 즉시 컴파일 불가가 된다(둘 다 `SliceGeometry`·`SliceContactResult`·`RollerProfile` 소비). 재작성은 각각 P2·P3 이므로 그때까지 주석 처리. 이에 따라 `commands.rs` 의 `compute_slice_geometry`·`compute_hertz_single_slice`·`solve_bearing`·`solve_bearing_dual` 4개 command 와 `lib.rs` 등록도 함께 해제한다.
   → **P1 종료 시점의 상태**: 컴파일되는 솔버는 `types`+`geometry` 뿐, Tauri command 는 preset 7종만. 앱은 빌드되나 해석 기능은 일시적으로 0. (CRB 가 자기 Phase 1 에서 쓴 하이브리드 stub 과 동일 수법)
6. ~~**단위 경계 계층 신설** — `commands` 에 `μm·kN·° ↔ mm·N·rad` 변환을 모음~~
   → **불필요해져 취소** (P1-S3 결정). 단일 구조체를 내부 단위로 두고 **필드명에 단위 접미사**
   (`_mm`·`_n`·`_nmm`·`_rad`·`_mpa`)를 붙이는 방식을 택했으므로 JSON 계약 자체가 mm·N·rad 다.
   변환은 UI 표시 시점에만 일어나며 Rust 쪽에 변환 계층이 존재하지 않는다.
   **환산 상수의 유일한 허용 장소는 `util.rs` 의 명시적 변환 함수**(현재 `sphere_mass_g` 의
   mm³→cm³ 하나뿐)이며, 이를 Level A 의 A-8 테스트가 기계적으로 강제한다.
7. `mod.rs` 정리, 프론트엔드 `@ts-nocheck` stub 로 빌드 그린 확보
8. TypeScript 타입 미러 갱신 (축 이름은 ISO 규약, D-7)

> ⚠️ **grep 에 안 걸리는 μm 스케일 매직넘버 3건** (2026-08-20 조사). mm 전환 시 놓치면 Jacobian·수렴이 조용히 깨진다 — P3 재작성 시 반드시 확인:
> `bearing.rs:58 FD_STEP_DISP = 0.01 [μm]` → `1e-5` / `bearing.rs:218 .clamp(5.0, 30.0)` → `0.005~0.03` / `bearing.rs:284 .max(1e3) // 1 kN·mm`

**산출물**: `cargo build` + `npm run build` 통과, Level A 통과

**Level A 검증 — 기하**

| 항목 | 방법 | 판정 |
|---|---|---|
| `A`, `α₀`, `R_i` | 손계산 대조 | rel. err < 1e-12 |
| `α₀` 왕복 | `G_r op → α₀ → G_r op` | 항등 |
| `Σρ`, `F(ρ)` | 차원 검사 + `F(ρ) ∈ [0,1)` 범위 | 만족 |
| `r_i = 0.52 D_w`, `r_e = 0.53 D_w` 기본값 | Annex B.1/B.2 | 일치 |
| 예압 표현 | `G_r op < 0` → `α₀ > 0` 단조 증가 | 단조 |
| **`R_i` (D-9)** | (A.4) 로 산출. `R_i > D_pw/2` 이고 α₀ 증가 시 감소 | 부등식·단조 |
| **단위 청정성 (D-10)** | `solver/` 전체에 `1000.0`·`1e3`·`1e-3` 환산 상수가 **0회** 등장 | grep 0건 |
| **축 명명 (D-7)** | 필드명이 `d_x`(축) / `d_y`,`d_z`(반경) / `g_y`,`g_z` 인지 | 일치 |

**DoD**: Level A 전 항목 통과, 삭제 모듈이 어디에서도 참조되지 않음, 환산 상수가 `util.rs` 밖에 없음(A-8 로 기계 검증)

---

### Phase 2 — 점접촉 Hertz (Theory §3, §6)

**작업**
0. `hertz.rs` 재작성 후 `mod.rs` 에서 **재활성화**, `commands.rs` 에 점접촉 command 신설
1. 완전타원적분 `K(χ)`, `E(χ)` — AGM 및 수치적분 2방식 구현 (교차검증용). **저장소에 기존 구현이 없음이 확인됨** — `rib_contact.rs` 의 Brewe-Hamrock 은 회귀 근사이지 정확값이 아니다
2. `χ` 솔버 — (E.1) 비선형 방정식. Brewe-Hamrock (6.33) 을 초기값으로
3. `c_P` (40) — 하중 무관 상수로 캐시
4. `Q = c_P δ^1.5` / 역함수 `δ = (Q/c_P)^(2/3)`
5. `a`, `b` (6.38)(6.40), `p_H,max = 3Q/(2πab)` (6.25)
6. `σ_Hu = 1500 MPa` 대비 판정 플래그

**Level B 검증 — 점접촉** ★ 가장 강력한 검증 단계

| 항목 | 방법 | 판정 |
|---|---|---|
| `K`, `E` 극한 | `χ = 1` 에서 `K = E = π/2` | rel. err < 1e-12 |
| `K`, `E` 2방식 | AGM vs 수치적분 | rel. err < 1e-10 |
| **`a*`, `b*`, `δ*`** | **Harris Table 6.1 (24행) 대조** (Theory §6.5) | rel. err < 1e-3 |
| ISO ↔ Harris 전사 대조 | ISO (36) `δ_i` vs Harris (6.42) `δ`. ⚠ **두 식은 대수적으로 동일**하므로 물리 교차가 아니라 **전사·구현 검증**이다 | rel. err < 1e-10 |
| **ISO 내부 일관성** | (36)+(37) 합 vs (39)+(40) 의 `c_P` 경로. ISO 의 `1,48` 이 `π/√4,5` 의 절사라 **0,065 % 편차가 규격상 존재**한다 | rel. err < 1e-3 |
| `c_P` 차원 | `Q = c_P δ^1.5` 단위 N/mm^1.5 | 일치 |
| Brewe-Hamrock 근사 | 자체 `χ` 솔버 대비 `1 ≤ χ ≤ 10` | 오차 < 3 % (Harris 명시 범위) |

**DoD**: Level B 전 항목 통과. 특히 **Harris Table 6.1 대조**가 통과해야 P3 진행 — 이것이 P2 의 유일한 외부 골든값이다

---

### Phase 3 — 평형 솔버 (Theory §4)

`bearing.rs` 는 **5-DOF 구조로 한 번만 작성**한다. 나뉘는 것은 검증이다 (D-1).
`DofMask` 로 자유도를 구속·해방하며 두 단계를 통과시킨다.

---

#### Phase 3-1 — 3-DOF 구속 검증 ★ 프로젝트 최대 마일스톤

`DofMask::ISO_3DOF` (`δ_z = γ_y = 0`) 상태로 **ISO 16281 A.2.2 와 항등**임을 확인하고,
유일한 외부 문헌 검증인 Harris Table 7.4 를 통과시킨다.

**작업**
0. `bearing.rs` 재작성 후 `mod.rs` 에서 **재활성화**, `commands.rs` 에 평형 command 신설
1. `bearing.rs` — **5-DOF 구조** + `DofMask` 구속 지원 (§3.5). 구속 자유도는 잔차·야코비안에서 행/열 제거
2. `δ_j`·`α_j` 의 5-DOF 일반화 — **§3.4.1 의 식 그대로**. 틸트 팔은 `R_i` (D-9)
3. 잔차 5개: `F_x, F_y, F_z, M_y, M_z` (구속 시 해당 성분 비활성)
4. `φ_j = 2π(j−1)/Z` 고정 원점 (D-8)
5. **비접촉 클리핑** (`δ_j < 0 → 0`) — active set 또는 스무딩 (T-3). *수렴의 전제라 P3-1 필수*
6. **축방향 예압**: `F_a0` → 초기 `δ_x` 로 변환하는 사전 해석 (D-2). *Level C 판정 항목이라 P3-1 필수*
7. **케이지 위상 스윕** (§3.4.3) — `φ₀` 를 `[0, 2π/Z)` 로 `n_phase` 분할, 최악값·발생위상 출력.
   *Level D-1 의 `Q_max` 대조 기준이라 P3-1 필수*

**Level C 검증 — 해석해 (`ISO_3DOF` 구속)**

| 항목 | 조건 | 판정 |
|---|---|---|
| 순수 축하중 | `F_y = F_z = 0, M = 0` → 모든 볼 동일 `δ_j`, `α_j` | 대칭성 완전 일치 |
| 축하중 해석해 | `F_x = Z c_P δ^1.5 sin α_j`, `α_j` 단일 미지수 | 잔차 < 1e-10 |
| 예압 무하중 | `F_ext = 0` 인 예압 상태 → 모든 볼 균등 하중 | 균등 |
| 회전 불변성 | 반경하중을 `Δφ` 회전 + `φ₀` 를 같은 각 회전 → `Q_j` 분포 동일 | rel. err < 1e-10 |
| 위상 스윕 | `φ₀` 스윕 시 `Q_max` 가 주기 `2π/Z` 로 진동, 최악값이 극값 | 주기성 확인 |

**Level D-1 검증 — Harris 대조** ★ 유일한 외부 검증

- 구속: **2-DOF 무정렬 마스크** (`δ_x`·`δ_y` 자유, `δ_z`·`γ_y`·`γ_z` 모두 `Prescribed(0)`).
  ⚠ `ISO_3DOF` 가 아니다 — **Harris Table 7.4 는 미스얼라인먼트를 두지 않은 2-DOF 표**이므로,
  `γ_z` 까지 구속해야 표와 같은 문제를 푼다 (P3-1-S3 에서 확인·정정)
- **`φ` 기준 주의 (D-8)**: Harris Table 7.4 는 `Q_max` 를 최대하중 볼 기준으로 정의한다.
  고정 원점(`φ_1 = 0`)에서는 볼이 정확히 하중 방향에 없을 수 있으므로,
  **위상 스윕으로 얻은 `Q_max` 최악값**을 대조에 쓴다
- 스윕: **`F_r tan α / F_a` 를 진입축으로** 표의 유한 점에 맞춰 변화.
  ⚠ `ε` 를 진입축으로 쓰면 안 된다 — 표의 `ε` 열은 2,5 위가 성겨(2,5 → 5,0 → ∞) 보간이 거칠다.
  Harris Fig. 7.14 도 `F_r tanα/F_a` 를 가로축으로 쓴다
- 대조: `J_r(ε)`, `J_a(ε)`, `Q_max`
- **사용 식**: `Q_max = F_r /(J_r(ε)·Z·cos α)` — 원서 (7.70) 의 `sin α` 는 오식 (Theory §9.1 주의 1)
- **판정**: 오차 ≤ 5 %. Harris 는 모든 볼의 접촉각을 동일하다고 가정하므로 완전 일치는 원리상 불가 (Theory §9.1 주의 2)

**DoD**: Level C 전 항목 + Level D-1 통과

> 🚦 **무인 진행 중단 게이트**. P3-1 통과는 외부 문헌 검증 통과를 뜻하므로, **여기서 멈추고 Level D-1 의 14개 점 오차를 사용자에게 보고**한 뒤 P3-2 진행 여부를 확인받는다. 오차가 5 % 를 넘을 경우 원인이 수식 해석인지 구현인지는 자의적으로 판단하지 않는다.

---

#### Phase 3-2 — 5-DOF 해방 검증

`DofMask::FULL` 로 전 자유도를 풀고, 3-DOF 해가 5-DOF 의 부분집합임을 기계적으로 확인한다.

**작업**

> ⚠ **정정 (P3-2 착수 시점)**: 아래 1~3 은 **P3-1 에서 이미 구현되었다.** D-1 결정이
> 「코드는 5-DOF 로 1회 구현」이었으므로 `bearing.rs` 는 처음부터 `FULL` 경로였고,
> 하중 입력도 `[F_x, F_y, F_z, M_y, M_z]` 로 열려 있었으며, 고속 경고도
> `geometry::collect_geometry_alerts` 에 들어 있었다. **P3-2 의 실질은 4 + Level D-2 다.**

1. ~~`DofMask::FULL` 경로 활성화 — 5×5 야코비안~~ → P3-1 선행 완료 (`bearing.rs`)
2. ~~임의 방향 반경하중·2축 모멘트 입력 경로 정리~~ → P3-1 선행 완료 (`OperatingConditions`)
3. ~~`n·D_pw > 1e6 mm/min` 경고 (D-3)~~ → P3-1 선행 완료 (`HIGH_SPEED`)
4. 수렴 강건성 — 격자 스윕으로 **실패율을 먼저 측정**한다. 실패가 없으면 튜닝하지 않는다
   (실패 없는 솔버의 스텝 제한·line search 파라미터를 손대는 것은 자의적 변경이다)

**Level D-2 검증 — 5-DOF 해방**

| 항목 | 방법 | 판정 |
|---|---|---|
| **축퇴 항등성** | 하중을 `F_x`·`F_y`·`M_z` 로 한정(x–y 평면 대칭 → `F_z`·`M_y` 잔차 항등 0)하고 `ISO_3DOF` 구속 해를 `FULL` 로 자유 계산 → 같은 해에 수렴 (`δ_z → 0`, `γ_y → 0`). **성분별 상대오차 금지** — 0 인 성분끼리 비교하게 되므로 해 벡터 크기를 공통 분모로 | rel. err < 1e-8 |
| 반경하중 방향 불변 | `F_y` 만 / `F_z` 만 / 45° 합성 → 크기 동일 시 위상 스윕 `Q_max` 동일 | rel. err < 1e-8 |
| 모멘트 축 불변 | `M_z` 만 / `M_y` 만 → 크기 동일 시 결과가 90° 회전 대칭 | rel. err < 1e-8 |
| 2축 모멘트 | `M_y`, `M_z` 합성 시 하중분포 비대칭 방향이 합성 모멘트 축과 정합 | 물리적 정합 |
| 수렴 강건성 | **물리적 유효 격자**(모든 점에 `F_x > 0` · 클리어런스 0 → 접촉 보장) 전수 스윕. 접촉 볼 0 개 조합은 오류가 아니라 자명해(Level C-8b)이므로 격자에서 제외하고 사유를 명시 | 실패율 **0 %** |
| 고속 경고 | `n·D_pw` 가 1e6 을 넘을 때만 경고 발생 | 경계 동작 |

**DoD**: Level D-2 전 항목 통과

---

> ### 🔀 Phase 순서 변경 (2026-08-21)
>
> 원안은 `P4 수명 → P5 윤활 → P6 프론트` 였다. **프론트를 맨 앞으로 옮긴다.**
>
> | 근거 | 내용 |
> |---|---|
> | ① 윤활은 **착수 불가** | 구 P5 에 걸린 **T-8**(Hamrock-Dowson 계수 원전 미확보)이 미해소다. 원서 없이 계수를 쓰는 것은 근거기반 원칙 위반 |
> | ② 백엔드가 **이미 준비됨** | `commands.rs` 에 `compute_geometry`·`compute_contact`·`solve_bearing` 등록 완료. 프론트가 붙을 자리가 비어 있다 |
> | ③ 시각화가 **검증 수단** | P3-2 보강에서 드러난 Theory §4.4 모멘트 부호 오기는 `Q_j`·`α_j` 극좌표 분포에서 즉시 보이는 종류다. 수치 시험은 D-3e 라는 전용 시험을 따로 설계해야 잡았다 |

> ⚠ **재작업 리스크와 그 완화**: 구 P4/P5 에서 `BearingResult` 에 필드가 **추가**되면 TS 타입을 다시 손봐야 한다(CLAUDE.md 체크리스트 1번). 그래서 신 P4 의 범위를 **접촉/하중 검증 뷰로 한정**한다 — `equilibrium`·`ball_results`·`phase_sweep`·`alerts` 는 P3 에서 확정되어 이후 Phase 가 **덮어쓰지 않고 추가만** 하므로 재작업 대상이 아니다. 수명 뷰·윤활 뷰는 각각 신 P5·P6 에 붙인다.

### Phase 4 — 프론트엔드 (접촉/하중 검증 뷰) 〔구 Phase 6〕

> 📐 **구조 설계는 §3.6 에 있다** — TRB 현행 ↔ BB 목표 대비, 전 44파일 인벤토리,
> 구조 다이어그램(현행/목표), 탭 구성 3안, `ProfileView`·`SectionView2D` 대체 판정,
> 상태·타입 변경. 아래는 **작업 순서만** 적는다.

**작업 (§3.6.7)**

| 단계 | 내용 | DoD |
|---|---|---|
| **S1** | `types/bearing.ts` 대체 · D등급 14파일 삭제 · `store.ts` 정리 | `npm run build` 통과 |
| **S2** | `InputPanel` BB 4블록 재매핑 | `solve_bearing` 왕복 성공 |
| **S3** | `GeometryView` + `compute_geometry` 연결 · `ResultsCard` 5-DOF · `AlertPanel` | Level A 결과가 화면과 일치 |
| **S4** | `LoadDistChart` 개조(`Q_j` + **`α_j(φ)`**) · Contact Ellipse View(`RibContactDetailChart` 개조 + `compute_contact`) | Level C·D 결과가 화면과 일치 |
| **S5** | BB 축단면 뷰 신규 · `BearingView3D` 개조 · 타이틀/확장자 | build + lint 통과 |

**DoD**: `npm run build` + `npm run lint` 통과, 살아남은 전 뷰가 실제 솔버 출력으로 동작, `@ts-nocheck` 0, 죽은 `invoke` 0

> ⚠️ **미결정 (S1 착수 전)**: ① 타입 동기화 방식(ts-rs 자동생성 ↔ 수작업) ② 탭 구성 3안 중 택1 ③ 삭제 시점(D등급 + `LubricationView` 2 439줄) ④ 시각 확인 절차 — Claude 는 서버를 직접 띄우지 않는다(CLAUDE.md 필수 준수 1)

---

### Phase 5 — 정격하중 및 수명 (Theory §7) 〔구 Phase 4〕

**작업**
1. `C_r` (ISO 281 1/2) + `b_m` Table 1 + **`f_c` Table 2 (40행, 선형보간)**
2. `X`, `Y`, `e` Table 3 — 접촉각·상대축하중 2중 보간
3. `Q_ci`, `Q_ce` (1)(2) — 내륜 0.407 / 외륜 0.389, 중괄호 지수 ±10/3
4. `Q_ei`, `Q_ee` (5)(7) — 내륜 회전·외륜 정지 기본
5. `L_10r` (9), `P_ref r` (11)
6. `C_u` — 정밀법 (B.1)(B.9)(B.10)(B.11) 및 간이법 (B.18)(B.19)
7. `κ` (27)~(29), `a_ISO` **볼 계수** (31)~(33) + 상한 50 / `κ` 클램프 / EP 규칙
8. `L_nmr` (13)
9. `static_rating.rs` — ISO 76 볼 계수로 교체

**Level E 검증 — 수명**

| 항목 | 방법 | 판정 |
|---|---|---|
| `f_c` 보간 | Table 2 격자점 재현 + γ=0.19 최대 60.0 | 격자점 완전 일치 |
| `X/Y/e` 보간 | Table 3 격자점 재현, α ≥ 20° 상수 확인 | 완전 일치 |
| `C_r` 역검증 | 제조사 카탈로그 ACBB 3종 이상의 `C_r` 재현 | 오차 ≤ 5 % (T-6) |
| `C_u` 2경로 | 정밀법(B.1) vs 간이법(B.18) | 동일 오더, 편차 기록 |
| `a_ISO` 경계 | `κ = 0.1/0.4/1/4` 경계 연속성, 상한 50 | 연속·클램프 동작 |
| 수명 단조성 | 하중↑ → `L_10r`↓, 지수 −3 스케일링 | 단조·지수 일치 |
| 롤러 계수 오용 방지 | `a_ISO`·`C_u` 계수가 볼 값인지 단위 테스트로 고정 | 통과 |

**DoD**: Level E 전 항목 통과. 카탈로그 역검증 최소 3종

---

### Phase 6 — 윤활 (D-6) 〔구 Phase 5〕

**작업**
1. `κ`, `ν₁` 는 P4 에서 이미 확보 → 재사용
2. `hamrock_dowson_elliptical` 이관 (§3.3) — U, G, W 무차원수를 **점접촉 볼 기준**으로 재정의
3. 볼 운동학: 케이지 속도, 볼–레이스웨이 인입속도 `u_m` (내륜·외륜 각각)
4. `h_c`, `h_min`, `Λ = h_min/σ` 산출 및 윤활 영역 분류

**Level F 검증 — 윤활**

| 항목 | 방법 | 판정 |
|---|---|---|
| `h_min` 크기 | 통상 ACBB 운전조건에서 0.05 ~ 2 μm | 범위 내 |
| `h_c/h_min` 비 | 1.1 ~ 1.5 | 범위 내 |
| `κ ≈ Λ^1.3` | ISO 281 §9.3.3.3.2 관계 | 정합 |
| 운동학 | 외륜 고정 시 `ω_cage = ω_i(1−γ)/2` | 일치 |
| 문헌 대조 | biboulet-houpert 2010 Part 2 (점접촉) | 정성 정합 |

> 🚦 **착수 게이트 (미해소)**: Hamrock & Dowson (1981) 원서가 저장소에 없다. CRB 구현의 계수(2.69/0.67/0.53/−0.067, 3.63/0.68/0.49/−0.073)는 코드 주석의 인용에 의존한다. P5 착수 전 원전 또는 Harris Ch.12/Hamrock(2004) 로 **계수 육안 확인** 필요 → 신규 항목 **T-8**.

**DoD**: Level F 통과, T-8 해소

---

## 5. 검증 매트릭스 요약

| Level | Phase | 대상 | 외부 근거 |
|---|---|---|---|
| A | P1 | 기하 | 해석적 항등 |
| **B** | P2 | 점접촉 | **Harris Table 6.1 (양방향)** |
| C | **P3-1** | 평형 해석해 (`ISO_3DOF` 구속) | 대칭성·해석해 |
| **D-1** | **P3-1** | 통합 (3-DOF 구속) ★ 중단 게이트 | **Harris Table 7.4** |
| D-2 | **P3-2** | 5-DOF 해방 | 축퇴 항등성 |
| E | **P5** | 수명 | ISO 표 재현 + 카탈로그 |
| F | **P6** | 윤활 | 문헌 정성 대조 |
| **G** | **P4** | **프론트 — 접촉/하중 시각 검증** | 솔버 출력의 육안 정합 |

> **외부 문헌 검증은 B 와 D-1 두 곳**이다. ISO 16281:2025 에는 수치 예제가 없다(전문 검색 0건, Theory §9.2).

---

## 6. 리스크

| 리스크 | 영향 | 대응 |
|---|---|---|
| 비접촉 클리핑의 비평활성 (T-3) | Newton 수렴 실패 | **P3-1** Level C 에서 조기 노출. active set / 스무딩 / line search |
| Harris 접촉각 고정 가정 | Level D-1 이 5 % 이내로 안 들어옴 | 저하중 영역에서 먼저 대조. 편차의 하중 의존성을 기록해 원인 분리 |
| `γ` 를 `α` 로 고정 (T-2) | 고하중에서 `c_P` 편차 | ISO 준거 유지(α 고정). `α_j` 기준 재계산과의 차이를 민감도로 정량화 |
| 카탈로그 `C_r` 역검증 실패 (P5) | `f_c` 보간 또는 `r_i/r_e` 가정 오류 | 제조사 실제 홈 반경 확보. ISO/TR 8646 감소 보정 미확보(T-9) |
| CRB 잔존 코드의 암묵적 롤러 가정 | 조용한 오류 | P1 에서 관련 모듈 **삭제**(주석처리 금지) |
| H-D 계수 원전 미확인 (T-8) | 유막 정확도 | **P6** 착수 전 해소. 미해소가 순서 변경의 직접 사유 |

---

## 7. 미해결 항목 갱신

Theory §11 기준. **T-1, T-4, T-5 는 해소됨**(원문 육안 확인).

| ID | 상태 | Phase |
|---|---|---|
| T-2 `γ` 갱신 여부 | 열림 — 민감도로 판단 | P2/P3 |
| T-3 클리핑 비평활 | 열림 — 구현 과제 | **P3-1** (수렴의 전제) |
| T-6 ISO 수치예제 부재 | 열림 — 카탈로그로 대체 | **P5** |
| T-7 고속 정식화 부재 | 보류 (D-3 = 경고만, **P3-2** 에서 경고만 구현) | P3-2 |
| ~~T-10~~ | ✅ 해소 — Harris & Mindel (1973) 확보·정독 | — |
| ~~T-11~~ | ✅ 해소 — 모멘트 팔 `R_i` 통일 확정 | — |
| **T-8** *(신규)* | Hamrock-Dowson 타원 계수 원전 미확인 | **P6** — 이 항목이 미해소여서 윤활을 맨 뒤로 미뤘다 |
| **T-9** *(신규)* | ISO/TR 8646:1985 미확보 — 홈 반경 초과 시 `f_c` 감소 보정 불가 | **P5** |

---

## 8. 다음 단계

### 8.1 즉시

**Phase 3 완결 (2026-08-21).** P1·P2·P3-1·P3-2(+보강) 완료, 테스트 118개 전량 통과.
외부 문헌 검증 3곳 통과 — Level B(Harris Table 6.1) · **B-3**(Harris & Mindel 1973 Fig. 15 실기 점접촉) · **D-1**(Harris Table 7.4) · **D-3**(Harris & Mindel 1973 식 (81)~(90), 유일한 5-DOF 근거).

다음은 **신 Phase 4 — 프론트엔드**. 범위·기술 결정은 착수 전 별도 확인.

### 8.2 완료 후 확장 후보 (우선순위 순)

1. **복열 조합 (DB/DF/DT)** — ISO 식 (10)(14)(15) 및 Harris Table 7.5 검증값이 이미 Theory 에 전개되어 있어 추가 조사 불필요
2. **4점접촉 (QJ)** — 1번 완료 시 ISO §5.1 근사로 자동 확보
3. 마찰·토크 — 확보 문헌 3편 활용
4. 고속 원심·자이로 (T-7)
5. 스러스트 볼베어링
