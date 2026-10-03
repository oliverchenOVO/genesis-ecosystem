//! Measures real paused worker response construction plus JSON encoding, excluding WebView transport.
use sim_app::{Action, App};
use sim_core::Config;
use std::time::Instant;

fn main() -> Result<(), String> {
    let mut reports = Vec::new();
    for population in [200, 1000, 5000] {
        let directory = tempfile::tempdir().map_err(|e| e.to_string())?;
        let app = App::start(directory.path().into())?;
        app.execute(Action::New {
            config: Config {
                starting_population: population,
                population_limit: population,
                size: 1024,
                ..Config::default()
            },
            temperature: Some(2000),
            regeneration: Some(100),
        })?;
        app.execute(Action::Control {
            running: false,
            speed: 1,
        })?;
        let before = app.execute(Action::Replay)?;
        let mut timings = Vec::new();
        let mut bytes = 0;
        for _ in 0..100 {
            let start = Instant::now();
            let snapshot = app.execute(Action::Snapshot)?;
            if snapshot["running"] != false {
                return Err("Worker must be paused".into());
            }
            let encoded = serde_json::to_vec(&snapshot).map_err(|e| e.to_string())?;
            bytes = encoded.len();
            std::hint::black_box(encoded);
            timings.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        let after = app.execute(Action::Replay)?;
        if before["hash"] != after["hash"] {
            return Err("Snapshots altered world/RNG".into());
        }
        timings.sort_by(f64::total_cmp);
        reports.push(serde_json::json!({"population":population,"samples":100,"bytes":bytes,"median_ms":timings[50],"p95_ms":timings[94],"maximum_ms":timings[99],"unchanged_world_hash":before["hash"]}));
    }
    println!(
        "{}",
        serde_json::json!({"measurement":"real paused simulation worker snapshot construction + JSON encoding; excludes WebView transport","context":"concurrent calibration; absolute latency, not an isolated before/after comparison","results":reports})
    );
    Ok(())
}
