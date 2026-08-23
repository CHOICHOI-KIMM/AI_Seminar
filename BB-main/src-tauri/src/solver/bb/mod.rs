// solver/bb — 볼 계열(BB) 전용. 변종(ACBB·DGBB·4PCBB)은 `BallBearingKind` 로만 구분하고
// 변종별 폴더를 파지 않는다 (Plan §3.6.1.3 ④).
//
//   types    : 볼 전용 데이터 모델 (Bb 접두 · BallBearingGeometry · BallResult)
//   geometry : ACBB 기하 전처리 (A·α₀·R_i·γ·Σρ·F(ρ))          — Theory §2
//   hertz    : 점접촉 타원 Hertz (χ·c_P·a·b·p_H)               — Theory §3, §6
//   bearing  : 5-DOF 평형 (해석 야코비안·active set·위상 스윕)  — Theory §4

pub mod bearing;
pub mod geometry;
pub mod hertz;
pub mod types;
