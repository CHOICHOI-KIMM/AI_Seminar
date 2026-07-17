//! 네이티브 기준값 러너 (Phase 0 기준 ②).
//!
//! `fixture_wear.json` 을 **WASM 과 동일한 `run_wear`** 에 통과시켜 결과 JSON 을 표준출력한다.
//! node 로 돌린 WASM 출력과 문자열 대조 → 경계 왕복 무결성 판정.
//!
//! 실행: `cargo run -p micropitting_wasm --example native_ref`

fn main() {
    let fixture = include_str!("../fixture_wear.json");
    println!("{}", micropitting_wasm::run_wear(fixture));
}
