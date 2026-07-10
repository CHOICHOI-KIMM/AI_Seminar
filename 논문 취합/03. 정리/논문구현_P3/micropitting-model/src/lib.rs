//! # micropitting_model — P3 Phase 0 공유 기반 crate
//!
//! 표면기인 마이크로피팅(surface-initiated micropitting) 모델의 공용 규약·인터페이스·
//! 공용 유틸을 **동결(freeze)** 하는 crate. 이후 단계(M1 dry, M2 lub, M6 share …)의
//! 모든 모듈은 여기서 정의한 [`types`] 인터페이스와 [`util`] 유틸을 SSOT로 사용한다.
//!
//! ## 참조-재구현 원칙
//! - CRB(`CRB-main`) 코드는 **알고리즘·계수 참조**만 한다 (직접 의존 금지).
//! - 직접 crate 의존은 `rustfft` / `nalgebra` / `rayon` 뿐.
//!
//! ## 코드 SSOT (전략 B — 표준/CRB 정렬)
//! - 환산탄성계수 `E_red`(표준): `1/E_red = (1-nu1^2)/E1 + (1-nu2^2)/E2`.
//!   논문 `E'`는 `E' = 2*E_red` 로 1회 치환한다(각 식 주석에 명기).
//! - 단위: SI (m, Pa, s). 유막·간극도 m.
//! - alpha 명명 분리: `alpha_wave[1/m]`, `alpha_dv[무차원]`, `alpha_visc[Pa^-1]`.
//! - 응력 부호: 인장 +.
//! - 좌표: 접촉 패치 로컬. x=구름방향, y=횡방향, z=깊이(+). 거칠기는 양방향 주기.

pub mod units;
pub mod types;
pub mod util;

pub mod m1_dry;
pub mod m2_lub;
pub mod m6_share;
