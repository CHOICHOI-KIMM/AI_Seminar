//! SSOT 상수 + 모듈 간 인터페이스 구조체(순수 struct, serde 불필요).
//!
//! 여기의 struct 는 M1(dry)/M2(lub)/M6(share) 등 모든 하위 모듈이 공유하는 **동결된
//! 데이터 계약(frozen contract)** 이다. 모든 필드는 SI 단위([`crate::units`] 참조)이며
//! 각 필드에 단위 주석을 명시한다.
//!
//! 좌표 규약: 접촉 패치 로컬. x=구름방향, y=횡방향, z=깊이(+). 인장 응력 +.

// ─────────────────────────────────────────────────────────────────────────
//  SSOT 상수
// ─────────────────────────────────────────────────────────────────────────

/// 베어링강(52100/AISI 강) 대표 영률 [Pa].
pub const E_STEEL_PA: f64 = 210.0e9;
/// 베어링강 대표 푸아송비 [-].
pub const NU_STEEL: f64 = 0.30;
/// 강-강 접촉 환산탄성계수 `E_red` [Pa].
/// `1/E_red = (1-nu^2)/E1 + (1-nu^2)/E2` → `E_red = E/(2(1-nu^2))`.
/// 210e9 / (2*(1-0.09)) = 115.384…e9 Pa.
pub const E_RED_STEEL_PA: f64 = E_STEEL_PA / (2.0 * (1.0 - NU_STEEL * NU_STEEL));
/// 논문 `E'` 와 코드 `E_red` 의 관계: `E' = E_PRIME_FACTOR * E_red` (1회 치환).
pub const E_PRIME_FACTOR: f64 = 2.0;

/// 수치 0 판정 임계값 (일반).
pub const EPS: f64 = 1e-12;

// ─────────────────────────────────────────────────────────────────────────
//  격자 & 2D 필드
// ─────────────────────────────────────────────────────────────────────────

/// 접촉 패치 로컬 계산 격자.
/// x=구름방향, y=횡방향. 물리 도메인 크기는 `lx × ly` [m].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Grid {
    /// x(구름방향) 격자점 수 [-]
    pub nx: usize,
    /// y(횡방향) 격자점 수 [-]
    pub ny: usize,
    /// x 방향 물리 길이 [m]
    pub lx: f64,
    /// y 방향 물리 길이 [m]
    pub ly: f64,
}

impl Grid {
    /// 새 격자 생성.
    pub fn new(nx: usize, ny: usize, lx: f64, ly: f64) -> Self {
        Grid { nx, ny, lx, ly }
    }

    /// x 방향 격자 간격 dx [m] (nx>1 가정, 주기적/구간 등분).
    #[inline]
    pub fn dx(&self) -> f64 {
        if self.nx > 0 {
            self.lx / self.nx as f64
        } else {
            0.0
        }
    }

    /// y 방향 격자 간격 dy [m].
    #[inline]
    pub fn dy(&self) -> f64 {
        if self.ny > 0 {
            self.ly / self.ny as f64
        } else {
            0.0
        }
    }

    /// 총 격자점 수 nx*ny [-].
    #[inline]
    pub fn len(&self) -> usize {
        self.nx * self.ny
    }

    /// 격자가 비었는지 여부.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.nx == 0 || self.ny == 0
    }
}

/// 격자 위 2D 스칼라 필드 (row-major: index = i + j*nx, i는 x, j는 y).
/// 단위는 저장하는 물리량에 따름(압력 Pa, 유막/거칠기 m 등) — 사용처에서 명시.
#[derive(Debug, Clone, PartialEq)]
pub struct Field2 {
    /// x(구름방향) 격자점 수 [-]
    pub nx: usize,
    /// y(횡방향) 격자점 수 [-]
    pub ny: usize,
    /// row-major 데이터, 길이 = nx*ny.
    pub data: Vec<f64>,
}

impl Field2 {
    /// 0 으로 채운 필드 생성.
    pub fn zeros(nx: usize, ny: usize) -> Self {
        Field2 {
            nx,
            ny,
            data: vec![0.0; nx * ny],
        }
    }

    /// 상수 값으로 채운 필드 생성.
    pub fn filled(nx: usize, ny: usize, value: f64) -> Self {
        Field2 {
            nx,
            ny,
            data: vec![value; nx * ny],
        }
    }

    /// 기존 데이터로부터 생성. `data.len() == nx*ny` 를 요구(불일치 시 panic).
    pub fn from_vec(nx: usize, ny: usize, data: Vec<f64>) -> Self {
        assert_eq!(
            data.len(),
            nx * ny,
            "Field2::from_vec: data.len()={} != nx*ny={}",
            data.len(),
            nx * ny
        );
        Field2 { nx, ny, data }
    }

    /// 격자 형상과 동일한 0 필드 생성.
    pub fn zeros_like_grid(grid: &Grid) -> Self {
        Field2::zeros(grid.nx, grid.ny)
    }

    /// 1D 인덱스 (i=x, j=y) → row-major offset.
    #[inline]
    pub fn idx(&self, i: usize, j: usize) -> usize {
        i + j * self.nx
    }

    /// (i,j) 값 읽기 (경계 검사 포함).
    #[inline]
    pub fn at(&self, i: usize, j: usize) -> f64 {
        debug_assert!(i < self.nx && j < self.ny, "Field2::at out of bounds");
        self.data[self.idx(i, j)]
    }

    /// (i,j) 가변 참조.
    #[inline]
    pub fn at_mut(&mut self, i: usize, j: usize) -> &mut f64 {
        debug_assert!(i < self.nx && j < self.ny, "Field2::at_mut out of bounds");
        let k = self.idx(i, j);
        &mut self.data[k]
    }

    /// (i,j) 값 쓰기.
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, value: f64) {
        let k = self.idx(i, j);
        self.data[k] = value;
    }

    /// 총 요소 수 [-].
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 비었는지 여부.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 최댓값 (빈 필드는 None).
    pub fn max(&self) -> Option<f64> {
        self.data.iter().copied().reduce(f64::max)
    }

    /// 최솟값 (빈 필드는 None).
    pub fn min(&self) -> Option<f64> {
        self.data.iter().copied().reduce(f64::min)
    }
}

// ─────────────────────────────────────────────────────────────────────────
//  재료 & 운전 조건
// ─────────────────────────────────────────────────────────────────────────

/// 접촉쌍 재료 물성 (환산값 기준).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialProps {
    /// 환산탄성계수 E_red [Pa]. `1/E_red=(1-nu1^2)/E1+(1-nu2^2)/E2`.
    /// (논문 E' 사용 식에서는 `E' = 2*E_red` 로 치환.)
    pub e_red: f64,
    /// 대표 푸아송비 [-] (동종재 가정 시 단일값).
    pub nu: f64,
    /// 표면 경도 [Pa] (마이크로피팅 임계응력 정규화용).
    pub hardness: f64,
    /// 재료 압력 한계 p_lim [Pa] (소성/항복 클램프용).
    pub p_lim: f64,
}

/// 접촉 운전 조건 (접촉 패치 기준, SI).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OperatingConditions {
    /// 최대 Hertz 압력 p_H [Pa].
    pub p_h: f64,
    /// 평균 구름속도(entrainment) u_mean = (u1+u2)/2 [m/s].
    pub u_mean: f64,
    /// 표면 2 속도 u2 [m/s] (표면 1은 u_mean·slide_roll 로 유도).
    pub u2: f64,
    /// 미끄럼-구름비 SRR = (u1-u2)/u_mean [-].
    pub slide_roll: f64,
    /// 기준온도 점도 eta0 [Pa·s].
    pub eta0: f64,
    /// 압점도계수 alpha_visc [Pa^-1].
    pub alpha_visc: f64,
    /// Eyring 전단응력 tau0 [Pa].
    pub tau0: f64,
    /// 접촉 온도 [K].
    pub temp: f64,
}

// ─────────────────────────────────────────────────────────────────────────
//  모듈 입력 / 결과 인터페이스
// ─────────────────────────────────────────────────────────────────────────

/// 부분윤활(mixed/partial EHL) 해석 입력.
#[derive(Debug, Clone)]
pub struct PartialLubInput {
    /// 계산 격자.
    pub grid: Grid,
    /// 표면 1 거칠기 높이장 [m] (평균 0, 양방향 주기).
    pub rough1: Field2,
    /// 표면 2 거칠기 높이장 [m] (평균 0, 양방향 주기).
    pub rough2: Field2,
    /// 재료 물성.
    pub mat: MaterialProps,
    /// 운전 조건.
    pub op: OperatingConditions,
    /// 공칭(smooth) 중심 유막두께 h_bar [m] (Dowson-Toyoda h_c).
    pub h_bar: f64,
}

/// 건식(dry) 접촉 해석 결과.
#[derive(Debug, Clone)]
pub struct DryResult {
    /// 건식 접촉 압력장 p_dry [Pa] (압축 하중, 부호는 인장 + 규약상 접촉면은 음압이나
    /// 관례상 압력 크기를 양수로 저장 — 사용처 주석 준수).
    pub p_dry: Field2,
    /// 건식 간극/침투장 h_dry [m].
    pub h_dry: Field2,
}

/// 완전윤활(full-film EHL) 해석 결과.
#[derive(Debug, Clone)]
pub struct LubResult {
    /// EHL 압력장 p_lub [Pa].
    pub p_lub: Field2,
    /// EHL 유막두께장 h_lub [m].
    pub h_lub: Field2,
}

/// 부분윤활(partial/mixed) 해석 결과.
#[derive(Debug, Clone)]
pub struct PartialLubResult {
    /// 전이(transition) 압력장 p_tran [Pa] (아스페리티+유막 합).
    pub p_tran: Field2,
    /// 전이 유막/간극장 h_tran [m].
    pub h_tran: Field2,
    /// 경계윤활 분율 phi_bl [-] (아스페리티 접촉 하중분율, 0~1).
    pub phi_bl: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn e_red_steel_constant() {
        // E_red = 210e9 / (2*0.91) = 115.384...e9
        assert_relative_eq!(E_RED_STEEL_PA, 115.384_615e9, max_relative = 1e-5);
    }

    #[test]
    fn field2_indexing_roundtrip() {
        let mut f = Field2::zeros(4, 3);
        f.set(2, 1, 7.5);
        assert_eq!(f.idx(2, 1), 2 + 1 * 4);
        assert_relative_eq!(f.at(2, 1), 7.5, max_relative = 1e-15);
        *f.at_mut(0, 0) = -3.0;
        assert_relative_eq!(f.at(0, 0), -3.0, max_relative = 1e-15);
        assert_eq!(f.len(), 12);
    }

    #[test]
    fn field2_min_max() {
        let f = Field2::from_vec(2, 2, vec![1.0, -2.0, 3.5, 0.0]);
        assert_relative_eq!(f.max().unwrap(), 3.5, max_relative = 1e-15);
        assert_relative_eq!(f.min().unwrap(), -2.0, max_relative = 1e-15);
    }

    #[test]
    fn grid_spacing() {
        let g = Grid::new(10, 5, 1e-3, 5e-4);
        assert_relative_eq!(g.dx(), 1e-4, max_relative = 1e-12);
        assert_relative_eq!(g.dy(), 1e-4, max_relative = 1e-12);
        assert_eq!(g.len(), 50);
    }
}
