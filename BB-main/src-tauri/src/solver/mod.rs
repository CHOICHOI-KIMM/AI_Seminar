pub mod types;
pub mod geometry;
pub mod hertz;
pub mod bearing;

// ─── BB Phase 1 (2026-08-20): 롤러 전용 모듈 영구 삭제 ─────────────────
// gen1 / gen3 / beam        : 슬라이스·빔 커플링 — 볼은 단일 점접촉이라 개념 소멸
// rib_contact               : 볼베어링에 rib 없음 (hertz_elliptical_coefficients 는 hertz.rs 로 이관)
// hmehl / transient
// transient_io / wec_risk   : 초기 범위 밖 (Plan §2.2)
//
// ─── 비활성 (BB Phase 4~5 에서 ACBB 기준 재작성 후 활성화) ─────────────
// pub mod life;             // P4: ISO 16281 §5.2 + ISO 281 볼 C_r
// pub mod static_rating;    // P4: ISO 76 볼 C_0r
// pub mod lubrication;      // P5: κ + Hamrock-Dowson 타원접촉 유막
