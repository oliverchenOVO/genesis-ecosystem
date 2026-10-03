#![recursion_limit = "256"]
use sim_core::{
    model::{SIMULATION_RULES_REVISION, SIMULATION_VERSION},
    persistence,
    replay::{Checkpoint, Replay},
    rng::RNG_VERSION,
    Command,
};
use sim_core::{Config, World};
use std::time::Instant;
mod calibration;

fn argument(name: &str, default: u64) -> u64 {
    let args: Vec<_> = std::env::args().collect();
    args.windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].parse().expect("numeric argument"))
        .unwrap_or(default)
}
fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if let Some(pair) = args.windows(2).find(|pair| pair[0] == "--verify-save") {
        let mut saved = persistence::load(std::path::Path::new(&pair[1]))?;
        let hash = saved.hash();
        let mut replayed = Replay::from_world(&saved).verify()?;
        if replayed.hash() != hash {
            return Err("Saved world replay mismatch".into());
        }
        let restored = serde_json::json!({"seed":saved.state.config.seed,"tick":saved.state.tick,"population":saved.state.organisms.len(),"species":saved.state.species.len(),"lineages":saved.state.lineages.len(),"telemetry_samples":saved.state.telemetry.len(),"commands":saved.state.commands,"rng":saved.state.rng,"hash":hash,"phase2_metrics":sim_core::analysis::ecology_metrics(&saved),"species_records":saved.state.species.values().collect::<Vec<_>>(),"multicellular_example":saved.state.organisms.iter().find(|o|o.phenotype.morphology.segment_count>1)});
        let continuation = argument("--continue", 1000);
        saved.advance(continuation);
        replayed.advance(continuation);
        saved.validate()?;
        replayed.validate()?;
        if saved.hash() != replayed.hash() {
            return Err("Saved world continuation mismatch".into());
        }
        println!(
            "{}",
            serde_json::json!({"path":pair[1],"verified":true,"restored":restored,"continuation_ticks":continuation,"continuation_hash":saved.hash()})
        );
        return Ok(());
    }
    if args.iter().any(|a| a == "--calibrate") {
        return calibration::run();
    }
    if args.iter().any(|a| a == "--golden") {
        return golden();
    }
    if args.iter().any(|a| a == "--stress") {
        return stress();
    }
    let seed = argument("--seed", 42);
    let ticks = argument("--ticks", 10_000);
    let population = argument("--population", 500) as usize;
    let limit = argument("--limit", population.max(2000) as u64) as usize;
    let mut world = World::new(Config {
        seed,
        size: argument("--size", 512) as i32,
        mutation_multiplier: argument("--mutation", 100) as u32,
        starting_population: population,
        population_limit: limit,
    })?;
    let temperature = args
        .windows(2)
        .find(|p| p[0] == "--temperature")
        .map(|p| p[1].parse::<i32>().map_err(|e| e.to_string()))
        .transpose()?
        .unwrap_or(2000);
    let regeneration = argument("--regeneration", 12) as i32;
    if temperature != 2000 || regeneration != 12 {
        world.command(Command {
            tick: 0,
            temperature,
            regeneration,
        })?;
    }
    let start = Instant::now();
    let mut organism_ticks = 0u128;
    let mut occupied_ticks = 0u64;
    for tick in 0..ticks {
        let population = world.state.organisms.len();
        organism_ticks += population as u128;
        occupied_ticks += u64::from(population > 0);
        world.step();
        if tick % 100 == 99 {
            world.validate()?;
        }
    }
    world.validate()?;
    let elapsed = start.elapsed().as_secs_f64();
    if let Some(pair) = args.windows(2).find(|p| p[0] == "--save-world") {
        persistence::save_atomic(&world, std::path::Path::new(&pair[1]))?;
    }
    #[cfg(feature = "sim-profile")]
    eprintln!(
        "{}",
        serde_json::to_string(&sim_core::profile::report()).map_err(|e| e.to_string())?
    );
    println!(
        "{}",
        serde_json::json!({"seed":seed,"ticks":ticks,"initial_population":population,"elapsed_seconds":elapsed,"ticks_per_second":ticks as f64/elapsed,"occupied_ticks":occupied_ticks,"organism_ticks":organism_ticks.to_string(),"mean_population":organism_ticks as f64/ticks.max(1) as f64,"peak_population":world.state.counters.peak_population,"final_population":world.state.organisms.len(),"species_count":world.state.species.len(),"births":world.state.counters.births,"deaths":world.state.counters.deaths,"mutations":world.state.counters.mutations,"predations":world.state.counters.predations,"world_hash":world.hash(),"phase2_metrics":sim_core::analysis::ecology_metrics(&world)})
    );
    Ok(())
}

fn golden() -> Result<(), String> {
    let config = Config {
        seed: 42,
        starting_population: 50,
        population_limit: 300,
        ..Config::default()
    };
    let mut world = World::new(config.clone())?;
    let commands = vec![
        Command {
            tick: 300,
            temperature: 1200,
            regeneration: 8,
        },
        Command {
            tick: 1200,
            temperature: 2600,
            regeneration: 16,
        },
    ];
    let mut checkpoints = Vec::new();
    for target in [0, 300, 500, 1000, 1200, 2000] {
        world.advance(target - world.state.tick);
        for command in commands.iter().filter(|c| c.tick == target) {
            world.command(command.clone())?;
        }
        checkpoints.push(Checkpoint {
            tick: target,
            hash: world.hash(),
        });
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&Replay {
            simulation_version: SIMULATION_VERSION,
            rules_revision: SIMULATION_RULES_REVISION,
            rng_version: RNG_VERSION,
            config,
            commands,
            final_tick: 2000,
            checkpoints
        })
        .map_err(|e| e.to_string())?
    );
    Ok(())
}

fn stress() -> Result<(), String> {
    let seeds = argument("--seeds", 100);
    let ticks = argument("--ticks", 50_000);
    let population = argument("--population", 100) as usize;
    let limit = argument("--limit", 500) as usize;
    let start = Instant::now();
    let mut handles = Vec::new();
    for worker in 0..4 {
        handles.push(std::thread::spawn(move||{
            let mut results=Vec::new();
            for seed in (worker..seeds).step_by(4){
                let checked=std::panic::catch_unwind(||->Result<serde_json::Value,String>{
                    let mut world=World::new(Config{seed,starting_population:population,population_limit:limit,..Config::default()})?;
                    world.command(Command{tick:0,temperature:1800+(seed%5) as i32*100,regeneration:12})?;
                    let mid=ticks/2;
                    for _ in 0..mid/1000{world.advance(1000);world.validate()?;}world.advance(mid%1000);world.validate()?;
                    let bytes=persistence::encode(&world)?;let mut resumed=persistence::decode(&bytes)?;
                    if world.hash()!=resumed.hash(){return Err("save corruption: roundtrip mismatch".into());}
                    let remaining=ticks-mid;
                    for _ in 0..remaining/1000{world.advance(1000);world.validate()?;resumed.advance(1000);}
                    world.advance(remaining%1000);resumed.advance(remaining%1000);world.validate()?;resumed.validate()?;
                    if world.hash()!=resumed.hash(){return Err("save continuation mismatch".into());}
                    let replay=Replay::from_world(&world).verify()?;
                    if replay.hash()!=world.hash(){return Err("replay mismatch".into());}
                    Ok(serde_json::json!({"seed":seed,"ticks":ticks,"population":world.state.organisms.len(),"peak":world.state.counters.peak_population,"births":world.state.counters.births,"deaths":world.state.counters.deaths,"species":world.state.species.len(),"hash":world.hash(),"save_bytes":bytes.len()}))
                });
                results.push(match checked{Ok(Ok(value))=>value,Ok(Err(error))=>serde_json::json!({"seed":seed,"error":error}),Err(_)=>serde_json::json!({"seed":seed,"error":"panic"})});
                eprintln!("seed {seed} complete");
            }
            results
        }));
    }
    let mut results = Vec::new();
    for handle in handles {
        results.extend(handle.join().map_err(|_| "Stress worker panicked")?);
    }
    results.sort_by_key(|r| r["seed"].as_u64());
    let failures = results.iter().filter(|r| r.get("error").is_some()).count();
    println!(
        "{}",
        serde_json::json!({"scenario":"multi-seed","seeds":seeds,"ticks_per_seed":ticks,"initial_population":population,"population_limit":limit,"workers":4,"elapsed_seconds":start.elapsed().as_secs_f64(),"failures":failures,"results":results})
    );
    if failures > 0 {
        Err(format!("{failures} seed validation failures"))
    } else {
        Ok(())
    }
}
