//! Read-only R4 diagnostic: unchanged R3 control worlds, predator origin and fate ledger.
//! Each world is checked against its frozen R3 hash before its observations are reported.
use sim_core::{predator_lifecycle, Command, Config, World};
use std::collections::BTreeMap;

const TICKS: u64 = 50_000;
const WARMUP_WINDOWS: u64 = 5;
const FROZEN: [&str; 8] = [
    "741738ae7550d269e59e4f39106557cef283e0150a974cb309bf2f37cb92768f",
    "bdbabeda1113836f07ef9348d439d2051a131e28313d7657624cce48258a0b62",
    "8dde43491e172545dbbe50b25de49731fde1f3a21591f77132b026c27b26f9e0",
    "366f3ea5aa6c2f2be90599ce1d0707edc53d01e6e2a026ce7c216c84a0c3c1ea",
    "e7a4d727f03e84c6481a45b54b7c5fb4fabf9b62f23f13620b6198cd9c6a9ac4",
    "aefe58b7e1a54cf7676c0216e1f9ca73208712f22cbc1a1fe72fd46ef422ccd3",
    "4abc090d21f782578a9d7769e95b3a17fc3226d4f1e8d0572e3e777ae1abbb8a",
    "314ebcc777a23132004f3005f3aae60d748629e12be48b77f7b74a329e2bfca9",
];

fn run(seed: u64) -> Result<serde_json::Value, String> {
    let mut world = World::new(Config {
        seed,
        starting_population: 50,
        population_limit: 2000,
        size: 512,
        mutation_multiplier: 100,
    })?;
    world.command(Command {
        tick: 0,
        temperature: 2000,
        regeneration: 4,
    })?;
    predator_lifecycle::start(&world);
    world.advance(TICKS);
    world.validate()?;
    let report = predator_lifecycle::finish(&world);
    let hash = world.hash();
    // Candidates are cross-checked against their own calibration world hashes instead.
    if !std::env::args().any(|a| a == "--candidate") && hash != FROZEN[seed as usize] {
        return Err(format!("seed {seed} hash {hash} differs from frozen R3"));
    }
    let living = world
        .state
        .organisms
        .iter()
        .filter(|o| predator_lifecycle::predator(o))
        .count();
    Ok(
        serde_json::json!({"seed":seed,"world_hash":hash,"frozen_r3_hash_match":hash == FROZEN[seed as usize],
        "final_living_predators":living,"lifecycle":report}),
    )
}

/// Sums every numeric field (and array element) of rows within `[from, to)` windows;
/// `peak_*` fields take the maximum instead.
fn period(worlds: &[serde_json::Value], table: &str, from: u64, to: u64) -> serde_json::Value {
    let mut target = serde_json::Map::new();
    for w in worlds {
        for row in w["lifecycle"][table].as_array().expect("table") {
            let window = row["window"].as_u64().expect("window");
            if window < from || window >= to {
                continue;
            }
            for (k, v) in row["totals"].as_object().expect("totals") {
                let entry = target.entry(k.clone()).or_insert_with(|| match v {
                    serde_json::Value::Array(a) => serde_json::json!(vec![0u64; a.len()]),
                    _ => serde_json::json!(0u64),
                });
                match (entry, v) {
                    (serde_json::Value::Array(acc), serde_json::Value::Array(add)) => {
                        for (a, b) in acc.iter_mut().zip(add) {
                            *a = (a.as_u64().unwrap() + b.as_u64().unwrap()).into();
                        }
                    }
                    (acc, add) if k.starts_with("peak") => {
                        *acc = acc.as_u64().unwrap().max(add.as_u64().unwrap()).into()
                    }
                    (acc, add) => *acc = (acc.as_u64().unwrap() + add.as_u64().unwrap()).into(),
                }
            }
        }
    }
    serde_json::Value::Object(target)
}

/// Adds every numeric field and array element of `add` into `acc`.
fn merge(acc: &mut serde_json::Value, add: &serde_json::Value) {
    for (k, v) in add.as_object().expect("object") {
        match (&mut acc[k], v) {
            (serde_json::Value::Array(a), serde_json::Value::Array(b)) => {
                for (x, y) in a.iter_mut().zip(b) {
                    *x = (x.as_u64().unwrap() + y.as_u64().unwrap()).into();
                }
            }
            (x, y) => *x = (x.as_u64().unwrap() + y.as_u64().unwrap()).into(),
        }
    }
}

fn main() -> Result<(), String> {
    let handles: Vec<_> = (0..8u64)
        .map(|seed| std::thread::spawn(move || run(seed)))
        .collect();
    let mut worlds = Vec::new();
    for h in handles {
        worlds.push(h.join().map_err(|_| "worker panicked")??);
    }
    let mut summary = BTreeMap::new();
    for (name, from, to) in [
        ("warmup", 0, WARMUP_WINDOWS),
        ("post_warmup", WARMUP_WINDOWS, u64::MAX),
    ] {
        summary.insert(
            name,
            serde_json::json!({"births":period(&worlds,"births",from,to),
                "deaths":period(&worlds,"deaths",from,to),"presence":period(&worlds,"presence",from,to)}),
        );
    }
    let candidate = std::env::args().any(|a| a == "--candidate");
    let mut lifetimes: BTreeMap<String, serde_json::Value> = BTreeMap::new();
    for w in &worlds {
        for row in w["lifecycle"]["lifetime_energy"]
            .as_array()
            .expect("lifetimes")
        {
            let key = format!(
                "{}|{}|{}",
                row["origin"].as_str().unwrap(),
                if row["born_after_warmup"].as_bool().unwrap() {
                    "post"
                } else {
                    "warmup"
                },
                row["outcome"].as_str().unwrap()
            );
            match lifetimes.get_mut(&key) {
                Some(acc) => merge(acc, &row["totals"]),
                None => {
                    lifetimes.insert(key, row["totals"].clone());
                }
            }
        }
    }
    println!(
        "{}",
        serde_json::json!({"scenario":"phase2-r4-predator-lifecycle","simulation_version":5,"rules_revision":sim_core::model::SIMULATION_RULES_REVISION,"candidate":candidate,
            "seeds":8,"ticks_per_seed":TICKS,"population":50,"limit":2000,"size":512,"regeneration":4,"mutation":100,"temperature":2000,
            "warmup_ticks":WARMUP_WINDOWS*1000,"all_hashes_match_frozen_r3":worlds.iter().all(|w|w["frozen_r3_hash_match"]==true),
            "predator_founders":worlds.iter().map(|w|w["lifecycle"]["predator_founders"].as_u64().unwrap()).sum::<u64>(),
            "summary":summary,"lifetime_energy":lifetimes,"worlds":worlds})
    );
    Ok(())
}
