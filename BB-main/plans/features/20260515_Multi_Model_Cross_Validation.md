# Multi-Model Cross-Validation for TRB Friction (Software Paper Prep)

## TL;DR

- **Goal**: 학술 publication을 위한 검증 강화. 실측 없이도 가능한 4가지 작업 동시 진행.
- **Deliverables**:
  - 5+ analytical TRB friction formulas 직접 구현 (Karna/Witte/Aihara orig/Matsuyama/Houpert 1995/Zhou-H)
  - 6-model head-to-head cross-comparison diagnostic
  - Schwarz LaMBDA Scheuermann + cage friction 추가 모듈
  - 공개 paper 5+개 추가 발굴 (WebSearch)
  - 상용 도구 cross-check (SKF Bearing Select, Bearinx, MESYS — 사용자 협조 필요)
- **Estimated Effort**: ~4-5주 (Claude 수행 가능: 2-4 / 사용자 수행: 1, 3 부분)

## Critical Path

1. **옵션 2** (Karna/Witte/Aihara orig/Matsuyama/Houpert 1995/Zhou-H 식 구현) — Claude 즉시 수행 가능
2. **옵션 4** (Schwarz LaMBDA Scheuermann + cage friction) — Claude 즉시 수행 가능
3. **옵션 1** (paper 발굴) — Claude WebSearch + 사용자 PDF 확보 협조
4. **옵션 3** (상용 도구) — 사용자 GUI 도구 사용, Claude 결과 코드 통합

## Phase A — Analytical Formulas Library (옵션 2)

### A.1 Aihara 1987 원식

Tewari Table 1:

$$M_{i,o}^\text{Aihara} = \frac{1.76 \times 10^2}{1 + 0.29 L^{0.78}} \cdot \frac{1}{\alpha} \cdot (GU)^{0.658} \cdot W^{0.31} \cdot R_e^2 \cdot l$$

$$M_{sl,rib}^\text{Aihara} = e \cdot \mu_\text{rib} \cdot F_a \cdot \exp(-1.8 \lambda_r^{1.2})$$

- $G = \alpha_{pv} \cdot E'$, $U = \eta_0 u_m / (E' R_e)$, $W = w_l / (E' R_e)$
- 우리 BH+Aihara는 Aihara thermal factor만 차용 — 이 식은 Aihara 전체 원식

### A.2 Zhou-Hoepprich 1991

$$M_{i,o}^\text{Z-H} = \varphi_\text{ish} \varphi_\text{bl} \cdot 58.4 \cdot \frac{R_e^2}{\alpha} (GU)^{0.648} W^{0.246} l$$

### A.3 Matsuyama 1998-2001

$$M_{i,o}^\text{Mats} = \varphi_\text{ish} \cdot 14.2 E' l R_e^2 U^{0.75} G^{-0.04} W^{0.08}$$

$$M_{sl,rib}^\text{Mats} = e \mu_\text{rib} F_a \cos\gamma, \quad \mu_\text{rib} = c_1 \exp(-c_2 \lambda_r^{c_3} + c_4)$$

### A.4 Houpert 2002

$$M_{i,o}^\text{Houpert} = 0.04 E' l R_e^2 U^{0.44} W^{0.37}$$

$$M_{sl,rib}^\text{Houpert} = \mu_\text{rib} F_r e \sqrt{1 + 0.18 (b/e)^2}$$

### A.5 작업 단위

- [x] `aihara_1987_raceway_torque()` — 구현, E' factor 추가 후 over-prediction → Tewari Table 1 OCR 모호성 추정. **dim ambiguous 표시, 향후 원논문 확인 시 fix**
- [ ] `aihara_1987_rib_torque()` — 보류 (원식 확인 후)
- [x] `zhou_hoepprich_1991_raceway_torque()` — 동일 dim 이슈, 동일 보류
- [x] `matsuyama_2001_raceway_torque()` — 구현 + Schwarz cross-comparison 통과 (ratio 1.8-2.7×, oil type/온도 차이 영향)
- [ ] `matsuyama_2001_rib_torque()` — 보류
- [x] `houpert_2002_raceway_torque()` — 구현 + cross-comparison 통과 (ratio 0.53-0.65, ATF calibration)
- [ ] `houpert_2002_rib_torque()` — 보류 (drilling 식은 이미 §2.8에 구현)

각 식별 helper는 [lubrication.rs:2295-](../src-tauri/src/solver/lubrication.rs#L2295) 에 추가.

**Aihara 1987 원식 dimensional ambiguity 처리**:
- Tewari Table 1 transcription: $M = 176/(1+0.29L^{0.78}) \cdot (1/\alpha) \cdot (GU)^{0.658} W^{0.31} R_e^2 l$
- E' 없음 → 결과 단위 [Pa·m³] = [N·m], prefactor 176 [Pa] 가정 시 SI 결과 너무 작음 (10⁻⁹ N·m order)
- E' 추가 시 → 너무 큼 (30000× over)
- → Aihara 원논문 직접 확인 필요. Tier 1 #5 paper 확보 시 fix.

## Phase B — Cross-Comparison Diagnostic (옵션 2 응용)

### B.1 Tewari Fig 13 운전점에서 5+ 모델 비교

진단 테스트 `diag_tewari_32008_5model_comparison`:

| Model | M_T @ 2200 rpm 55°C [N·m] | M_T @ 2200 rpm 65°C [N·m] |
|---|---|---|
| Tewari measured | 1.07 | 0.95 |
| 우리 BH + Aihara thermal | (계산) | (계산) |
| Aihara 1987 원식 | (계산) | (계산) |
| Zhou-Hoepprich 1991 | (계산) | (계산) |
| Matsuyama 2001 | (계산) | (계산) |
| Houpert 2002 | (계산) | (계산) |
| SKF 2018 | (계산) | (계산) |

→ Tewari Figure 13에서 그는 Aihara/Zhou/Matsuyama 비교했으므로, 우리가 같은 운전점에서 동일 비교 + Houpert 2002 + SKF 2018 추가 = **우리 contribution**.

### B.2 Schwarz 32216 운전점에서 동일 비교

- 500/2000/4000 rpm × 50°C, axial 6 kN
- 7+ 모델 cross-comparison
- 측정값과 best-match 식 식별 → Schwarz Fig 5에서 LaMBDA가 했던 작업의 확장

### B.3 작업 단위

- [ ] `diag_tewari_32008_multi_model_comparison` 진단 테스트
- [ ] `diag_schwarz_32216_multi_model_comparison` 진단 테스트
- [ ] 결과 CSV / markdown 표 출력

## Phase C — Schwarz LaMBDA Additional Modules (옵션 4)

### C.1 Scheuermann Solid Rolling Friction

Schwarz Eq. 19:

$$M_{T,Sr} = c_r \cdot F_N^{e_r}$$

- $c_r$, $e_r$: empirical fit per geometry
- 32216 paper 값 추출 필요

### C.2 Cage Friction (Coulomb)

Schwarz Eq. 37:

$$F_T^\text{cage} = F_N \cdot \mu_\text{Coulomb}$$

- $\mu_\text{Coulomb} \approx 0.05$-0.10
- Cage-roller pocket contact (geometry-based F_N)

### C.3 작업 단위

- [ ] `scheuermann_solid_rolling_torque()` + test
- [ ] `cage_friction_coulomb()` + test
- [ ] 두 항을 `OperatingConditions`에 optional toggle 추가
- [ ] `compute_traction` / `compute_traction_advanced`에 합산

## Phase D — Paper Hunt (옵션 1, 2026-05-15 완료)

### D.1 발굴 결과 (11+ paper)

**Tier 1 — 즉시 검증 활용** (raceway-only 분리 / 컴포넌트 분리 / open access):

| # | 저자, 연도 | 저널 | 베어링 | 측정 type | Access | Adequacy |
|---|---|---|---|---|---|---|
| 1 | Liu et al. 2024 | *Tribology Int.* | TRB | raceway-only 분리 (radial heavy load) | paywall | HIGH |
| 2 | Marques et al. 2021 | *Tribology Int.* 155 | tandem TRB (차축) | preload + speed sweep | paywall | HIGH |
| 3 | Liu Z. et al. 2022 | *Lubricants* 10(7):154 | **32008J** | 롤러 편차 영향 | **OA** | HIGH |
| 4 | Zhou & Hoeprich 1991 | *J. Tribol.* 113:590 | TRB | **cup/cone/rib 분리 측정** | paywall (ASME) | HIGH |
| 5 | Aihara 1987 | *J. Tribol.* 109:471 | TRB | running torque + 식 form | paywall (ASME) | HIGH (원식 확인) |
| 6 | Liebrecht 박사학위 | TU Kaiserslautern | **32208** | drag/churning 분리 | OA (KLUEDO) | HIGH |

**Tier 2 — 신규/특화**:

| # | 저자, 연도 | 저널 | 베어링 | 비고 |
|---|---|---|---|---|
| 7 | Lee & An 2024 | SAE 2024-01-3047 | wheel hub TRB | 트럭/상용차 |
| 8 | Wu et al. 2025 | *Proc. IMechE Part J* | TRB | equivalent μ |
| 9 | Liu et al. 2025 | *Lubricants* 13(4):160 | paired TRB | tri-axial + preload, OA |
| 10 | Yan et al. 2025 | *J. Braz. Soc. Mech.* | double-row TRB | roller skewing + tilting |
| 11 | Witte 1973 | *ASLE Trans.* 16 | TRB | Aihara 이전 baseline |

**Aihara 1987 원논문 (E' factor 검증)**:
- ASME Digital Collection paywall ([DOI: 10.1115/1.3261475](https://doi.org/10.1115/1.3261475))
- 무료 PDF: ResearchGate / arXiv / Semantic Scholar 모두 미발견
- 권장: (a) 기관 ILL, (b) ASME 개별 구매 (~$25), (c) NSK reprint 요청, (d) 후속 인용 paper(Wang 2018 *Friction* 6:7-22, Gao 2022 *Lubricants*)에서 dimensional form cross-reference

### D.2 즉시 다운로드 가능 (사용자 작업)

1. **Liu 2022 *Lubricants* 10(7):154** — 32008J 정량 데이터 (Tewari 32008과 cross-check)
2. **Liu 2025 *Lubricants* 13(4):160** — paired TRB tri-axial
3. **Liebrecht 박사학위 (KLUEDO)** — 32208 drag/churning
4. **2025 *Lubricants* TRB review** — 종합 리뷰

### D.3 Paywall paper 확보 시도 (사용자 협조)

1. **Liu 2024 *Trib. Int.*** — raceway-only 분리 (가장 우리와 직접 비교 가능)
2. **Zhou-Hoeprich 1991** — cup/cone/rib 분리 측정 (rib drilling 검증의 결정타)
3. **Marques 2021** — tandem TRB preload 측정
4. **Aihara 1987** — Aihara/Zhou-H 식 dimensional form 확정

### D.4 검증 매트릭스 확장 후

5개 베어링 사이즈 × 4개 독립 그룹:
- 32008: Tewari + Liu 2022
- 32208: Schwarz + Liebrecht
- 32216: Schwarz (single ref, 추가 paper 필요)
- TRB tandem: Marques 2021
- 컴포넌트 분리: Zhou-Hoeprich 1991

→ **publication-ready 검증 범위** (1 paper 단일 vs 4 paper 다양).

## Phase E — Commercial Tool Cross-Check (옵션 3, 사용자 협조)

### E.1 대상 도구

- SKF Bearing Select online (무료, 회원가입)
- Schaeffler Bearinx-online Easy Friction (무료, 회원가입)
- MESYS Bearing Calculator (평가판 30일)

### E.2 운전점 매트릭스

3 도구 × 3-5 베어링 × 3 운전점 = 27-45 cross-check points.

### E.3 작업 단위 (사용자)

- [ ] SKF Bearing Select 30206/30306/32206/32306 sweep
- [ ] Bearinx-online 동일 sweep
- [ ] MESYS trial 동일 sweep
- [ ] 결과 CSV/Excel
- [ ] Claude가 진단 테스트로 통합

## Phase F — Report 통합

- [ ] §3 cross-paper comparison에 5+ analytical formula 추가
- [ ] §4 외부 실험 검증에 추가 paper 데이터 통합
- [ ] §5 기능 설명에 Scheuermann/cage option 추가
- [ ] §6 결론에 multi-model cross-comparison 결과 반영

## 의존성

```
Phase A (Claude) → Phase B (Claude)
Phase C (Claude) → Phase B (Claude) 일부
Phase D (Claude+사용자) → Phase B 추가 점
Phase E (사용자) → Report 통합
Phase F (Claude) ← 모든 Phase 결과
```

병렬 가능: A + C + D 동시 진행.
