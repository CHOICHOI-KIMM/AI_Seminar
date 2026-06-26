use std::io::Read;

use crate::error::SolverError;
use super::types::{LoadTimePoint, TransientInput};

/// Parse a CSV load time series.
///
/// Expected format: `t,Fx,Fy,Fa,Mx,My,rpm` (header row required).
/// Units: t[s], F[kN], M[kN·m], rpm[rpm].
pub fn parse_load_csv<R: Read>(reader: R) -> Result<Vec<LoadTimePoint>, SolverError> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(reader);

    let mut points = Vec::new();

    for (i, result) in rdr.records().enumerate() {
        let record = result.map_err(|e| {
            SolverError::InvalidInput(format!("CSV parse error at row {}: {}", i + 2, e))
        })?;

        if record.len() < 7 {
            return Err(SolverError::InvalidInput(format!(
                "CSV row {} has {} columns, expected 7 (t,Fx,Fy,Fa,Mx,My,rpm)",
                i + 2,
                record.len()
            )));
        }

        let parse_field = |idx: usize, name: &str| -> Result<f64, SolverError> {
            record[idx].parse::<f64>().map_err(|e| {
                SolverError::InvalidInput(format!(
                    "CSV row {}, column '{}': {}",
                    i + 2,
                    name,
                    e
                ))
            })
        };

        points.push(LoadTimePoint {
            t_s: parse_field(0, "t")?,
            f_x: parse_field(1, "Fx")?,
            f_y: parse_field(2, "Fy")?,
            f_a: parse_field(3, "Fa")?,
            m_x: parse_field(4, "Mx")?,
            m_y: parse_field(5, "My")?,
            n_rpm: parse_field(6, "rpm")?,
        });
    }

    if points.is_empty() {
        return Err(SolverError::InvalidInput(
            "CSV load series is empty".into(),
        ));
    }

    validate_time_monotonic(&points)?;
    Ok(points)
}

/// Parse a JSON TransientInput directly.
#[allow(dead_code)]
pub fn parse_transient_json(json_str: &str) -> Result<TransientInput, SolverError> {
    let input: TransientInput = serde_json::from_str(json_str).map_err(|e| {
        SolverError::InvalidInput(format!("JSON parse error: {}", e))
    })?;

    if input.load_series.is_empty() {
        return Err(SolverError::InvalidInput(
            "JSON load series is empty".into(),
        ));
    }

    validate_time_monotonic(&input.load_series)?;
    Ok(input)
}

/// Validate that time values are strictly monotonically increasing.
fn validate_time_monotonic(points: &[LoadTimePoint]) -> Result<(), SolverError> {
    for i in 1..points.len() {
        if points[i].t_s <= points[i - 1].t_s {
            return Err(SolverError::InvalidInput(format!(
                "Time series not monotonic at index {}: t[{}]={} <= t[{}]={}",
                i,
                i,
                points[i].t_s,
                i - 1,
                points[i - 1].t_s
            )));
        }
    }
    Ok(())
}

/// Linearly interpolate a load series to uniform time steps.
///
/// Returns a new series with constant dt spacing from t_start to t_end.
#[allow(dead_code)]
pub fn interpolate_uniform(
    points: &[LoadTimePoint],
    dt: f64,
) -> Result<Vec<LoadTimePoint>, SolverError> {
    if points.len() < 2 {
        return Err(SolverError::InvalidInput(
            "Need at least 2 points for interpolation".into(),
        ));
    }
    if dt <= 0.0 {
        return Err(SolverError::InvalidInput("dt must be positive".into()));
    }

    let t_start = points[0].t_s;
    let t_end = points[points.len() - 1].t_s;
    let n_steps = ((t_end - t_start) / dt).ceil() as usize;

    let mut result = Vec::with_capacity(n_steps + 1);
    let mut seg = 0usize; // current segment index

    for i in 0..=n_steps {
        let t = (t_start + i as f64 * dt).min(t_end);

        // Advance segment pointer
        while seg + 1 < points.len() - 1 && t > points[seg + 1].t_s {
            seg += 1;
        }

        let p0 = &points[seg];
        let p1 = &points[seg + 1];
        let dt_seg = p1.t_s - p0.t_s;
        let frac = if dt_seg.abs() > 1e-15 {
            ((t - p0.t_s) / dt_seg).clamp(0.0, 1.0)
        } else {
            0.0
        };

        result.push(LoadTimePoint {
            t_s: t,
            f_x: p0.f_x + frac * (p1.f_x - p0.f_x),
            f_y: p0.f_y + frac * (p1.f_y - p0.f_y),
            f_a: p0.f_a + frac * (p1.f_a - p0.f_a),
            m_x: p0.m_x + frac * (p1.m_x - p0.m_x),
            m_y: p0.m_y + frac * (p1.m_y - p0.m_y),
            n_rpm: p0.n_rpm + frac * (p1.n_rpm - p0.n_rpm),
        });
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_parsing() {
        let csv_data = "\
t,Fx,Fy,Fa,Mx,My,rpm
0.0,  10.0, 0.0, 50.0, 0.0, 0.0, 100.0
0.1,  20.0, 5.0, 55.0, 1.0, 0.5, 100.0
0.2,  15.0, 3.0, 52.0, 0.5, 0.2, 100.0
";
        let points = parse_load_csv(csv_data.as_bytes()).unwrap();
        assert_eq!(points.len(), 3);
        assert!((points[0].t_s - 0.0).abs() < 1e-10);
        assert!((points[1].f_x - 20.0).abs() < 1e-10);
        assert!((points[2].f_a - 52.0).abs() < 1e-10);
    }

    #[test]
    fn test_csv_non_monotonic() {
        let csv_data = "\
t,Fx,Fy,Fa,Mx,My,rpm
0.0, 10.0, 0.0, 50.0, 0.0, 0.0, 100.0
0.2, 20.0, 0.0, 50.0, 0.0, 0.0, 100.0
0.1, 15.0, 0.0, 50.0, 0.0, 0.0, 100.0
";
        let result = parse_load_csv(csv_data.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn test_json_parsing() {
        let json = r#"{
            "load_series": [
                {"t_s": 0.0, "f_x": 10.0, "f_y": 0.0, "f_a": 50.0, "m_x": 0.0, "m_y": 0.0, "n_rpm": 100.0},
                {"t_s": 0.1, "f_x": 20.0, "f_y": 0.0, "f_a": 55.0, "m_x": 0.0, "m_y": 0.0, "n_rpm": 100.0}
            ],
            "dt_max": 0.001,
            "enable_roller_dynamics": true,
            "snapshot_interval": 1
        }"#;
        let input = parse_transient_json(json).unwrap();
        assert_eq!(input.load_series.len(), 2);
        assert!((input.dt_max - 0.001).abs() < 1e-10);
    }

    #[test]
    fn test_interpolation() {
        let points = vec![
            LoadTimePoint { t_s: 0.0, f_x: 0.0, f_y: 0.0, f_a: 100.0, m_x: 0.0, m_y: 0.0, n_rpm: 100.0 },
            LoadTimePoint { t_s: 1.0, f_x: 10.0, f_y: 0.0, f_a: 200.0, m_x: 0.0, m_y: 0.0, n_rpm: 200.0 },
        ];
        let interp = interpolate_uniform(&points, 0.25).unwrap();
        assert_eq!(interp.len(), 5); // 0.0, 0.25, 0.5, 0.75, 1.0
        // Check midpoint interpolation
        assert!((interp[2].t_s - 0.5).abs() < 1e-10);
        assert!((interp[2].f_x - 5.0).abs() < 1e-10);
        assert!((interp[2].f_a - 150.0).abs() < 1e-10);
        assert!((interp[2].n_rpm - 150.0).abs() < 1e-10);
    }
}
