pub mod types;
pub mod util;
pub mod geometry;
pub mod hertz;
pub mod bearing;

// ─── BB Phase 1-S3 (2026-08-20) ──────────────────────────────────────
// util     : 접촉 형상 무관 수치·물성 유틸 (E*, 곡률합성, 스플라인, 구 질량)
// geometry : ACBB 기하 전처리 (A·α₀·R_i·γ·Σρ·F(ρ))  — Theory §2
//
// hertz    : 점접촉 타원 Hertz (χ·c_P·a·b·p_H)  — Theory §3, §6
// bearing  : 5-DOF 평형 (해석 야코비안·active set·위상 스윕) — Theory §4
//
// ─── P4-S0-1 (2026-08-23) 에서 영구 삭제 — 신 P5/P6 에서 신규 작성 ────
// life / static_rating / lubrication 3파일(9 830줄)은 비활성 롤러(TRB) 판이었고
// CRB 저장소에 byte-identical 사본이 있어 삭제했다 (Plan §3.6.1.1 발견 ②).
// BB 는 아래를 **신규 작성**한다 — 기존 파일 재활성화가 아니다.
//   신 P5 : 수명   — ISO 16281 §5.2 볼 식 + ISO 281 볼 C_r · ISO 76 볼 C_0r
//   신 P6 : 윤활   — κ + Hamrock-Dowson 점(타원)접촉 유막
//
// ─── P1-S1 에서 영구 삭제 ─────────────────────────────────────────────
// gen1 / gen3 / beam / rib_contact / hmehl / transient / transient_io / wec_risk
