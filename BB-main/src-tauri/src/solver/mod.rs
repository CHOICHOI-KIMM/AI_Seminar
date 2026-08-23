pub mod bb;
pub mod common;

// ─── P4-S0-2 (2026-08-23) 폴더 재편 — Plan §3.6.1.6 ───────────────────
// common/ : 베어링 계열 무관 (util · 공통 6타입)
// bb/     : 볼 계열 전용 (types · geometry · hertz · bearing)
//
// 재수출(`pub use`)로 옛 경로를 살려두지 않는다 — **경계가 목적**이므로
// 소비처가 `solver::common::…` / `solver::bb::…` 를 직접 써야 한다.
//
// ⚠ `SolverError` 는 `solver/` 밖(`src/error.rs`)이라 이동하지 않았다.
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
