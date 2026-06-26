# 13. Static Load Rating (정정격하중 평가)

## 13.1 개요

정정격하중(Static Load Rating) 평가는 베어링이 정적 또는 저속 하중 조건에서 허용 가능한 소성 변형 이내로 작동하는지 확인합니다.

본 시스템은 두 가지 표준을 구현합니다:
- **ISO 76:2006** — 기본 정정격하중 C₀ᵣ, 정등가하중 P₀ᵣ, 정안전계수 S₀
- **ISO 17956:2025** — 라미나 기반 유효 정안전계수 S₀,eff (실제 내부 하중 분포 반영)

## 13.2 ISO 76 — 기본 정정격하중

### 13.2.1 C₀ᵣ (Basic Static Radial Load Rating)

단열 레이디얼 롤러 베어링의 기본 정정격하중 (ISO 76 Eq.7):

```
C₀ᵣ = 44 × (1 - γ) × i × Z × L_we × D_we × cos(α)  [N]
```

여기서:
- `γ = D_we × cos(α) / D_pw` (피치원 직경 비)
- `i = 1` (단열)
- `Z` = 롤러 수
- `L_we` = 유효 접촉 길이 [mm]
- `D_we` = 평균 롤러 직경 [mm]
- `α` = 접촉각 [rad]

계수 44는 4,000 MPa 접촉 응력 기준 (롤러 베어링)에서 유도됩니다.

### 13.2.2 P₀ᵣ (Static Equivalent Radial Load)

ISO 76 Eq.(8)-(9):
```
P₀ᵣ = max(X₀·F_r + Y₀·F_a,  F_r)
```

단열 롤러 베어링 (ISO 76 Table 3):
- `X₀ = 0.5`
- `Y₀ = 0.22 × cot(α)`

### 13.2.3 S₀ (Static Safety Factor)

ISO 76 Eq.(14):
```
S₀ = C₀ᵣ / P₀ᵣ
```

**권장 가이드라인 (ISO 76 Table 5, 롤러 베어링)**:
| 운전 조건 | S₀ min |
|-----------|--------|
| 저소음/정밀 | 3 |
| 일반 운전 | 1.5 |
| 충격 하중 | 3 |

## 13.3 ISO 17956 — 유효 정안전계수

ISO 17956:2025는 ISO 76의 단순화된 가정(고정 하중 분포)을 실제 내부 하중 분포 해석(ISO 16281 기반)으로 대체합니다.

### 13.3.1 q₀ (Reference Lamina Load)

ISO 17956 Eq.(7):
```
q₀ = (1/n_s) × (5 / (i × Z × cos(α))) × C₀ᵣ  [N]
```

여기서:
- `n_s` = 라미나(슬라이스) 수 (≥ 30)
- `C₀ᵣ` = ISO 76 기본 정정격하중 [N]

### 13.3.2 q_max (Maximum Lamina Load)

ISO 17956 Eq.(6):
```
q_max = max{q_{j,k}} over all rollers j, all laminae k
```

실제 베어링 내부 하중 분포에서 가장 큰 라미나 하중입니다.
라미나 하중 = q_k(슬라이스 선하중 [N/mm]) × slice_width [mm].

### 13.3.3 S₀,eff (Effective Static Safety Factor)

ISO 17956 Eq.(5):
```
S₀,eff = q₀ / q_max
```

### 13.3.4 ISO 76 대비 장점

| 구분 | ISO 76 | ISO 17956 |
|------|--------|-----------|
| 하중 분포 | 고정 가정 | 실제 계산 (ISO 16281) |
| 틸트/미스얼라인먼트 | 고려 불가 | 반영됨 |
| 롤러 프로파일 | 고려 불가 | 라미나 모델 반영 |
| 에지 응력 | 고려 불가 | 라미나 분포로 간접 반영 |

## 13.4 구현 파일

- `src-tauri/src/solver/static_rating.rs` — Rust 솔버 모듈
- `src/types/bearing.ts` — `StaticRatingResult` 인터페이스
- `src/components/ResultsCard/index.tsx` — Summary 패널 표시
- `src/components/DetailView/index.tsx` — Static 탭 상세 뷰

## 13.5 UI 입력 필드

| 필드 | 위치 | 설명 |
|------|------|------|
| C₀ᵣ (static) | Solver > Load Ratings | Auto/Manual 토글, Manual 시 직접 입력 [kN] |
| S₀ min | Solver > Load Ratings | 최소 정안전계수 요구치 (기본값 1.0) |

## 13.6 수치 예제 (NSK HR30306J)

입력:
- D_we = 10.53 mm, L_we = 11.65 mm, Z = 14, α = 11.859°, D_pw = 51.0 mm
- F_r = 5.0 kN, F_a = 2.0 kN

ISO 76:
- γ = 10.53 × cos(11.859°) / 51.0 = 0.202
- C₀ᵣ = 44 × (1 - 0.202) × 14 × 11.65 × 10.53 × cos(11.859°) / 1000 ≈ 59.6 kN
- Y₀ = 0.22 × cot(11.859°) = 1.048
- P₀ᵣ = max(0.5×5.0 + 1.048×2.0, 5.0) = max(4.596, 5.0) = 5.0 kN
- S₀ = 59.6 / 5.0 = 11.9

NSK 카탈로그: C₀ᵣ = 59.8 kN → 자동계산값과 거의 일치.
