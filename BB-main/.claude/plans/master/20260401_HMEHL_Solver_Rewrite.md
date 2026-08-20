# HMEHL Solver Rewrite — Venner Multigrid + FBNS

## TL;DR

### Quick Summary
Habchi full-system Newton → **Venner FAS multigrid + distributive relaxation** 전면 교체 완료.
M=62228 프리셋 조건에서 flat film + EHL pressure spike 확인.

### Status: Phase 1-2 완료 ✅, Phase 3-4 진행 예정

---

## Phase 1: 정리 ✅ 완료
- [x] hmehl.rs 데드 코드 1,478줄 삭제
- [x] ehl2d.rs 882줄 삭제
- [x] 빌드+테스트 36/36 pass

## Phase 2: Venner Multigrid 코어 ✅ 완료
- [x] `v_cycle()` — FAS V-cycle (pre/post smoothing, restriction, correction)
- [x] `compute_residual()` — Reynolds 잔차
- [x] `restrict_fas()`, `prolongate_correction()` — multigrid transfer
- [x] `line_relax_nd()` — Thomas 알고리즘 라인 이완
- [x] `update_film_nd()` — 무차원 유막 업데이트
- [x] `solve()` — FMG 진입점 (coarsest→finest + V-cycle)
- [x] Newton 삭제: `newton_habchi`, `habchi_*` 5개 함수, `ReynoldsCoeffs`
- [x] V-cycle 안전장치: residual 증가 시 auto-revert
- [x] 프리셋 검증: M=62228에서 flat film (0% floor), spike p/p_h=1.87
- [x] 36/36 테스트 통과

### Phase 2 결과
| 지표 | 이전 (Habchi Newton) | V-cycle |
|------|---------------------|---------|
| 코드량 | 4,321줄 | **2,396줄** (-45%) |
| Film at contact | 68% at floor | **0% at floor** |
| Film shape | 압력 추종 | **flat + outlet dip** |
| p_max/p_h (TRB) | 0.98 | **1.87** |
| h_min/h_c | 의미없음 | **0.809** |

---

## Phase 3: FBNS 캐비테이션 — 다음 단계

### 목적
현재 `P ≥ 0` 클램핑은 질량비보존. FBNS(Fischer-Burmeister Newton Strategy)로 교체하면:
- 출구 영역 캐비테이션 물리 정확도 향상
- 질량 보존 자동 만족
- 출구 film constriction (h_min dip) 정밀도 향상

### 구현 항목
- [ ] FBNS complementarity function: Φ(P, θ) = √(P²+θ²) - P - θ = 0
- [ ] θ 보조 변수 (cavitation fraction) + active set 관리
- [ ] distributive_relax에 FBNS 통합 (P≥0 클램핑 대체)
- [ ] 테스트: 출구 캐비테이션 영역 질량보존 검증

### 우선순위: 중
현재 P≥0 클램핑으로도 물리적으로 합리적인 결과가 나오므로 급하지 않음.
FBNS는 micropitting 해석이나 과도(transient) EHL에서 더 중요.

---

## Phase 4: 검증 + 최적화

### 검증 항목
- [ ] Venner Level A: 단일 슬라이스 Hertz vs 해석 (<0.1%)
- [ ] Venner Level B: Smooth surface h_c/DH ratio (0.3~3.0)
- [ ] Venner Level C: TRB pressure spike (p/p_h > 1.5) ✅ 이미 통과
- [ ] Venner Level D: Moes sweep (M=100~100,000)
- [ ] Kaneta 유막두께 검증 (SNU 논문 Fig 3.6)
- [ ] Coulon 압력분포 검증 (SNU 논문 Fig 3.4)

### 최적화 항목
- [ ] ω 동적 조절 (잔차 감소율 모니터링)
- [ ] V-cycle 횟수 자동 결정 (수렴 기반)
- [ ] 격자 수 자동 선택 (M 기반: 저M→128, 고M→512)

---

## 참고 문헌

1. Venner, C.H. (1991) — *Multilevel Solution of the EHL Line and Point Contact Problems*
2. Lubrecht, A.A. (1987) — *The Numerical Solution of the EHL Problem*
3. Hansen, E. et al. (2011) — HMEHL with roughness homogenization
4. Woloszynski, T. et al. (2015) — *Efficient FBNS Algorithm for EHL*
5. Bartel, D. (2010) — *Simulation von Tribosystemen*
6. SNU Thesis (`Reference/EHL/SNU.md`) — HMEHL-FBNS MATLAB 구현
