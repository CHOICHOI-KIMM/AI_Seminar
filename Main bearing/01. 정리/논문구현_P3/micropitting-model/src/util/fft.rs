//! 2D FFT 순/역 변환 래퍼 (rustfft 기반).
//!
//! 데이터는 row-major `Vec<Complex<f64>>`, 길이 = nx*ny, index = i + j*nx
//! ([`crate::types::Field2`] 와 동일한 레이아웃). x(구름방향)=행 내부, y(횡방향)=행.
//!
//! 규약:
//! - [`fft2_forward`] 는 정규화하지 않는다(rustfft 관례).
//! - [`fft2_inverse`] 는 `1/(nx*ny)` 로 정규화하여 왕복(round-trip)이 항등이 되도록 한다.
//!
//! EHL 압력↔변형 커널 곱(순환 컨볼루션) 및 거칠기 스펙트럼 계산의 공용 기반.

use rustfft::num_complex::Complex;
use rustfft::FftPlanner;

/// 1D FFT 를 nx-크기 행들에 대해 순차 적용(각 행 in-place).
fn fft_rows(data: &mut [Complex<f64>], nx: usize, ny: usize, inverse: bool) {
    let mut planner = FftPlanner::<f64>::new();
    let fft = if inverse {
        planner.plan_fft_inverse(nx)
    } else {
        planner.plan_fft_forward(nx)
    };
    for j in 0..ny {
        let row = &mut data[j * nx..(j + 1) * nx];
        fft.process(row);
    }
}

/// 열(y 방향, stride=nx) FFT. 임시 버퍼로 gather/scatter.
fn fft_cols(data: &mut [Complex<f64>], nx: usize, ny: usize, inverse: bool) {
    let mut planner = FftPlanner::<f64>::new();
    let fft = if inverse {
        planner.plan_fft_inverse(ny)
    } else {
        planner.plan_fft_forward(ny)
    };
    let mut col = vec![Complex::new(0.0, 0.0); ny];
    for i in 0..nx {
        for j in 0..ny {
            col[j] = data[i + j * nx];
        }
        fft.process(&mut col);
        for j in 0..ny {
            data[i + j * nx] = col[j];
        }
    }
}

/// 2D 순방향 FFT (비정규화). 입력 nx,ny 는 row-major data 형상.
pub fn fft2_forward(data: &mut [Complex<f64>], nx: usize, ny: usize) {
    assert_eq!(data.len(), nx * ny, "fft2_forward: len != nx*ny");
    if nx == 0 || ny == 0 {
        return;
    }
    fft_rows(data, nx, ny, false);
    fft_cols(data, nx, ny, false);
}

/// 2D 역방향 FFT (`1/(nx*ny)` 정규화). `fft2_forward` 와 왕복 시 항등.
pub fn fft2_inverse(data: &mut [Complex<f64>], nx: usize, ny: usize) {
    assert_eq!(data.len(), nx * ny, "fft2_inverse: len != nx*ny");
    if nx == 0 || ny == 0 {
        return;
    }
    fft_cols(data, nx, ny, true);
    fft_rows(data, nx, ny, true);
    let scale = 1.0 / (nx * ny) as f64;
    for v in data.iter_mut() {
        *v *= scale;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn roundtrip_identity() {
        let nx = 8;
        let ny = 4;
        let orig: Vec<Complex<f64>> = (0..nx * ny)
            .map(|k| Complex::new((k as f64).sin(), (0.3 * k as f64).cos()))
            .collect();
        let mut buf = orig.clone();
        fft2_forward(&mut buf, nx, ny);
        fft2_inverse(&mut buf, nx, ny);
        for (a, b) in orig.iter().zip(buf.iter()) {
            assert_relative_eq!(a.re, b.re, epsilon = 1e-10);
            assert_relative_eq!(a.im, b.im, epsilon = 1e-10);
        }
    }

    #[test]
    fn dc_component_is_sum() {
        // 상수 필드 → DC(0,0) = sum, 나머지 ~0
        let nx = 4;
        let ny = 4;
        let mut buf = vec![Complex::new(2.0, 0.0); nx * ny];
        fft2_forward(&mut buf, nx, ny);
        assert_relative_eq!(buf[0].re, 2.0 * (nx * ny) as f64, max_relative = 1e-10);
        assert_relative_eq!(buf[1].re, 0.0, epsilon = 1e-10);
    }
}
