pub mod types;

// ─── BB Phase 1-S2 (2026-08-20): types.rs 를 ACBB 로 전면 재작성 ──────
// 아래 3개는 CRB(선접촉) 데이터 모델을 소비하므로 지금은 컴파일 불가.
// 각 모듈을 ACBB 기준으로 재작성하는 시점에 순차 재활성화한다.
// pub mod geometry;         // P1-S3: A·α₀·R_i·Σρ·F(ρ)   (Theory §2)
// pub mod hertz;            // P2   : 점접촉 타원 Hertz    (Theory §3, §6)
// pub mod bearing;          // P3   : 5-DOF 평형           (Theory §4)
//
// ─── BB Phase 4~5 에서 ACBB 기준 재작성 후 활성화 ─────────────────────
// pub mod life;             // P4: ISO 16281 §5.2 + ISO 281 볼 C_r
// pub mod static_rating;    // P4: ISO 76 볼 C_0r
// pub mod lubrication;      // P5: κ + Hamrock-Dowson 타원접촉 유막
//
// ─── P1-S1 에서 영구 삭제 ─────────────────────────────────────────────
// gen1 / gen3 / beam / rib_contact / hmehl / transient / transient_io / wec_risk
