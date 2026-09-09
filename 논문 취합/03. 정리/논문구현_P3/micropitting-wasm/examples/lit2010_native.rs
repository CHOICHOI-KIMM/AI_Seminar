//! ME2010 Fig 7/8 네이티브 러너 — **런타임 계량 + 격자 선택 근거** (WASM 과 동일 `run_lit2010_*`).
//!
//! 실행: `cargo run --release --example lit2010_native [overrides_json]`
//! 예:   `cargo run --release --example lit2010_native '{"nx":512,"ny":256}'`
//!
//! 출력: Fig 7 요약(피크 p/p_h·min h·φ_bl·진단) + Fig 8 표 + 벽시계(그림별).
//! 벽시계는 호출측(이 예제)이 잰다 — 크레이트에 시계 없음(기존 계량 관례).

use std::time::Instant;

fn main() {
    let overrides = std::env::args().nth(1).unwrap_or_else(|| "{}".to_string());

    let t0 = Instant::now();
    let f7 = micropitting_wasm::run_lit2010_fig7(&overrides);
    let t7 = t0.elapsed();
    let v7: serde_json::Value = serde_json::from_str(&f7).expect("fig7 json");
    println!("── Fig 7 ({:.2} s) ──", t7.as_secs_f64());
    if v7["ok"] == true {
        let m = &v7["meta"];
        println!(
            "grid {}x{}  hC={:.3e}  peak p/pH={:.3} (field {:.3})  minH={:.3e} (field {:.3e})  phiBl={:.4}",
            m["grid"]["nx"], m["grid"]["ny"], m["hC"].as_f64().unwrap(),
            v7["peakPOverPh"].as_f64().unwrap(), v7["peakPOverPhField"].as_f64().unwrap(),
            v7["minH"].as_f64().unwrap(), v7["minHField"].as_f64().unwrap(),
            v7["phiBl"].as_f64().unwrap()
        );
        println!("diagnostics: {}", v7["diagnostics"]);
    } else {
        println!("ERROR: {}", v7["error"]);
    }

    let t1 = Instant::now();
    let f8 = micropitting_wasm::run_lit2010_fig8(&overrides);
    let t8 = t1.elapsed();
    let v8: serde_json::Value = serde_json::from_str(&f8).expect("fig8 json");
    println!("── Fig 8 ({:.2} s) ──", t8.as_secs_f64());
    if v8["ok"] == true {
        println!("{:>7} {:>10} {:>9} {:>9} {:>8} {:>5} {:>5} {:>9}", "uMean", "hC", "load%", "area%", "count", "conv", "iters", "loadRes");
        for p in v8["points"].as_array().unwrap() {
            println!(
                "{:>7.3} {:>10.3e} {:>9.3} {:>9.3} {:>8} {:>5} {:>2}/{:>2} {:>9.2e}",
                p["uMean"].as_f64().unwrap(), p["hC"].as_f64().unwrap(),
                p["loadRatioPct"].as_f64().unwrap(), p["areaRatioPct"].as_f64().unwrap(),
                p["contactCount"], p["converged"], p["outerIters"], p["shareIters"],
                p["loadResidual"].as_f64().unwrap()
            );
        }
    } else {
        println!("ERROR: {}", v8["error"]);
    }
}
