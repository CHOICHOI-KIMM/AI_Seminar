pub mod types;
pub mod geometry;
pub mod hertz;
pub mod gen1;
pub mod gen3;
pub mod beam;
pub mod bearing;

// ─── Phase 1 하이브리드 stub: 아래 모듈들은 CRB Phase 2~7 에서 재활성화 예정 ───
// D1 (rib contact 제외) 및 알고리즘 재작성 필요 모듈은 현재 disable.
// pub mod rib_contact;      // D1: CRB 전 시리즈에서 미사용 → 영구 삭제 예정
// pub mod life;             // Phase 5: ISO 16281 5.3 + ISO 281 CRB C_R 로 재작성
// pub mod static_rating;    // Phase 5: ISO 76 CRB C_0r 로 재작성
// pub mod lubrication;      // Phase 7: kinematic 식 CRB 형태 (cage/slip) 재확인
// pub mod hmehl;            // Phase 7: HMEHL CRB line contact 재확인
// pub mod transient;        // Phase 7: 시간 영역 통합 CRB 재활성화
// pub mod transient_io;     // Phase 7: LoadTimePoint CRB 필드 반영 후 재활성화
// pub mod wec_risk;         // Phase 7: WEC 위험도 CRB 재확인
