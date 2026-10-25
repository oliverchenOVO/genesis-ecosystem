//! Read-only R4 diagnostic: unchanged R3 control worlds, environmental feeding inside each
//! channel's static top-quartile productivity habitat.
//! Each world is checked against its frozen R3 hash before its observations are reported.
use sim_core::{habitat_association, Command, Config, World};

const TICKS: u64 = 50_000;
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
    habitat_association::start(&world);
    world.advance(TICKS);
    world.validate()?;
    let report = habitat_association::finish();
    let hash = world.hash();
    if hash != FROZEN[seed as usize] {
        return Err(format!("seed {seed} hash {hash} differs from frozen R3"));
    }
    Ok(
        serde_json::json!({"seed":seed,"world_hash":hash,"frozen_r3_hash_match":true,"habitat":report}),
    )
}

fn main() -> Result<(), String> {
    let handles: Vec<_> = (0..8u64)
        .map(|seed| std::thread::spawn(move || run(seed)))
        .collect();
    let mut worlds = Vec::new();
    for h in handles {
        worlds.push(h.join().map_err(|_| "worker panicked")??);
    }
    println!(
        "{}",
        serde_json::json!({"scenario":"phase2-r4-habitat-association-unchanged-r3","simulation_version":5,"rules_revision":3,
            "seeds":8,"ticks_per_seed":TICKS,"population":50,"limit":2000,"size":512,"regeneration":4,"mutation":100,"temperature":2000,
            "all_hashes_match_frozen_r3":true,"worlds":worlds})
    );
    Ok(())
}
