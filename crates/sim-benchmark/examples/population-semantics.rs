//! Read-only R4 diagnostic: unchanged R3 control worlds (seeds 0..23), birth flow under four
//! species-grouping semantics (lineage, genetic cluster, genetic x mouth, gene-flow community).
//! Each world is checked against its frozen R3 hash before its observations are reported.
use sim_core::{population_semantics, Command, Config, World};

const TICKS: u64 = 50_000;
const FROZEN: [&str; 24] = [
    "741738ae7550d269e59e4f39106557cef283e0150a974cb309bf2f37cb92768f",
    "bdbabeda1113836f07ef9348d439d2051a131e28313d7657624cce48258a0b62",
    "8dde43491e172545dbbe50b25de49731fde1f3a21591f77132b026c27b26f9e0",
    "366f3ea5aa6c2f2be90599ce1d0707edc53d01e6e2a026ce7c216c84a0c3c1ea",
    "e7a4d727f03e84c6481a45b54b7c5fb4fabf9b62f23f13620b6198cd9c6a9ac4",
    "aefe58b7e1a54cf7676c0216e1f9ca73208712f22cbc1a1fe72fd46ef422ccd3",
    "4abc090d21f782578a9d7769e95b3a17fc3226d4f1e8d0572e3e777ae1abbb8a",
    "314ebcc777a23132004f3005f3aae60d748629e12be48b77f7b74a329e2bfca9",
    "50a11b5861374fc2e626f71e84a00cee313b625e8765e7f47ec7889296efc3b5",
    "09d7fec1c76f3566b79630a10cbc4f1c607baa2f679e3b2eafcc814388242ec4",
    "95494bfc64c4bc431710956858fc10feea44ae949da179f5f1209a3496902e7d",
    "d70e6cc36a3b5e2ea6427a5592e5da4cd5ec29274d521314f2612e217f4029d5",
    "00a153b7b61884f9c7b7e51c91d4ab9215b27e9ded05873228cc7d7f26109384",
    "9f873e565bfdbe7e73a6f0595138eb06b760efa1c2e54a7efb9934f591f9ba8b",
    "9ba360295aaa03fb1e95899146a41aba33f1a2f1832a39026424f3e076ff2716",
    "d8dfd663994eeca601b5ec7912ce26a521e0b8c0da6fa51b7a25f6606a86e27f",
    "470c4d2123d91a5965b38225d19239d8a90a77072ae43b47d577be021b2dbb1c",
    "c1431597cc80250e279b7e9c0c8dba09776bcdbfa71090f0c7bf1d1a2f7470a4",
    "c278cc074e1056ce0a9858546ecf486511ccd5c6db26176b0d1ef9740fb4d5fa",
    "f452dc00af6d0710fa668cb4170a7dfda17b08324c49ec93b82f143c8f9138ac",
    "756138596fc92142d27dcfe1a37c61b5fb6c221c25119715a8193c1c4a5772d6",
    "3fa113d5a564dec808b51cd8e5dd5877cc9b739f650aeb5847f46f419646a601",
    "687a5f293303a69dc4c73913952b7ad10cb8ffc6a88a61703ab1b7e55809516f",
    "9c6bc3e604fa374cf5cc274e85b9b7c8fb937adfab3aae13f47753bfb27f58cd",
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
    population_semantics::start(&world);
    world.advance(TICKS);
    world.validate()?;
    let report = population_semantics::finish();
    let hash = world.hash();
    // Candidates are cross-checked against their own calibration world hashes instead.
    if !std::env::args().any(|a| a == "--candidate") && hash != FROZEN[seed as usize] {
        return Err(format!("seed {seed} hash {hash} differs from frozen R3"));
    }
    Ok(
        serde_json::json!({"seed":seed,"world_hash":hash,"frozen_r3_hash_match":hash == FROZEN[seed as usize],"population_semantics":report}),
    )
}

fn main() -> Result<(), String> {
    let handles: Vec<_> = (0..24u64)
        .map(|seed| std::thread::spawn(move || run(seed)))
        .collect();
    let mut worlds = Vec::new();
    for h in handles {
        worlds.push(h.join().map_err(|_| "worker panicked")??);
    }
    println!(
        "{}",
        serde_json::json!({"scenario":"phase2-r4-population-semantics","simulation_version":5,"rules_revision":sim_core::model::SIMULATION_RULES_REVISION,"candidate":std::env::args().any(|a| a == "--candidate"),
            "seeds":24,"ticks_per_seed":TICKS,"population":50,"limit":2000,"size":512,"regeneration":4,"mutation":100,"temperature":2000,
            "all_hashes_match_frozen_r3":worlds.iter().all(|w|w["frozen_r3_hash_match"]==true),"worlds":worlds})
    );
    Ok(())
}
