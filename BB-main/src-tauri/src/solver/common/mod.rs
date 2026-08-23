// solver/common — 베어링 계열(볼·롤러)에 의존하지 않는 것만 둔다.
//
// P4-S0-2 (2026-08-23) 신설. 경계 근거는 Plan §3.6.1.6 「① 대상별 규칙 · Rust 모듈」.
//   util  : 접촉 형상 무관 수치·물성 유틸 (E*·곡률합성·타원적분·Gauss-Legendre·스플라인)
//   types : 진짜 공통 6타입 (SolverProgress·ProgressReporter·NoopReporter·Material·Alert·AlertLevel)
//
// ⚠ 이 모듈은 `solver::bb` 를 **참조하지 않는다.** 단방향 의존이 경계의 전부다.

pub mod types;
pub mod util;
