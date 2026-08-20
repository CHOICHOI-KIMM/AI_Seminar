pub mod types;
pub mod util;
pub mod geometry;

// ─── BB Phase 1-S3 (2026-08-20) ──────────────────────────────────────
// util     : 접촉 형상 무관 수치·물성 유틸 (E*, 곡률합성, 스플라인, 구 질량)
// geometry : ACBB 기하 전처리 (A·α₀·R_i·γ·Σρ·F(ρ))  — Theory §2
//
// ─── 아직 CRB 데이터 모델을 소비 — 재작성 시 활성화 ────────────────────
// pub mod hertz;            // P2   : 점접촉 타원 Hertz    (Theory §3, §6)
// pub mod bearing;          // P3-1 : 평형 (3-DOF 구속 검증)  (Theory §4)
//
// ─── BB Phase 4~5 에서 ACBB 기준 재작성 후 활성화 ─────────────────────
// pub mod life;             // P4: ISO 16281 §5.2 + ISO 281 볼 C_r
// pub mod static_rating;    // P4: ISO 76 볼 C_0r
// pub mod lubrication;      // P5: κ + Hamrock-Dowson 타원접촉 유막
//
// ─── P1-S1 에서 영구 삭제 ─────────────────────────────────────────────
// gen1 / gen3 / beam / rib_contact / hmehl / transient / transient_io / wec_risk
