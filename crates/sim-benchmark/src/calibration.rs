//! Read-only ecology measurements. Sampling never consumes world RNG or changes state.
use serde_json::{json, Value};
use sim_core::{model::SIMULATION_VERSION, Command, Config, World};
use std::{
    collections::BTreeMap,
    io::Write,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::Instant,
};

#[derive(Default)]
struct Measurements {
    morphology_distance: Vec<f64>,
    niche_distance: Vec<f64>,
    morphology_clusters: Vec<f64>,
    species_complexity: Vec<f64>,
    complex_lineage_streaks: BTreeMap<u64, u64>,
    complex_lineages_observed: std::collections::BTreeSet<u64>,
    maximum_persistent_complex_lineages: usize,
    niche_streaks: BTreeMap<u64, ([i32; 6], u64)>,
    maximum_persistent_niche_clusters: usize,
    morphology_streaks: BTreeMap<u64, ([i32; 9], u64)>,
    maximum_persistent_morphology_clusters: usize,
    #[cfg(feature = "viability")]
    feeding_role_streaks: [u64; 2],
    #[cfg(feature = "viability")]
    longest_feeding_role_streaks: [u64; 2],
    #[cfg(feature = "viability")]
    maximum_persistent_realized_feeding_roles: usize,
    predator_population: Vec<f64>,
    consumer_population: Vec<f64>,
    multicellular_streak: u64,
    longest_multicellular: u64,
    maximum_complexity: i32,
    niche_cluster_streak: u64,
    longest_niche_clusters: u64,
    populations: Vec<f64>,
    food: Vec<f64>,
    diversity: Vec<f64>,
    minimum: usize,
    low_streak: u64,
    longest_low: u64,
    maximum_species: usize,
    rich_streak: u64,
    longest_rich: u64,
    lineage_streaks: BTreeMap<u64, u64>,
    persistent_lineages: usize,
}
fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len().max(1) as f64
}
fn variance(values: &[f64]) -> f64 {
    let m = mean(values);
    mean(&values.iter().map(|v| (v - m).powi(2)).collect::<Vec<_>>())
}
fn summary(values: &[f64]) -> Value {
    let mut ordered = values.to_vec();
    ordered.sort_by(f64::total_cmp);
    if ordered.is_empty() {
        return Value::Null;
    }
    let q = |p: f64| ordered[((ordered.len() - 1) as f64 * p).round() as usize];
    json!({"min":ordered[0],"median":q(0.5),"p90":q(0.9),"p95":q(0.95),"max":ordered[ordered.len()-1],"mean":mean(values)})
}
fn genomic_distance(world: &World) -> f64 {
    let organisms = &world.state.organisms;
    if organisms.is_empty() {
        return 0.0;
    }
    let n = organisms.len() as f64;
    let centroid: Vec<_> = (0..sim_core::genetics::LOCI)
        .map(|i| {
            organisms
                .iter()
                .map(|o| f64::from(o.genome.0[i]))
                .sum::<f64>()
                / n
        })
        .collect();
    organisms
        .iter()
        .map(|o| {
            o.genome
                .0
                .iter()
                .enumerate()
                .map(|(i, g)| (f64::from(*g) - centroid[i]).abs())
                .sum::<f64>()
                / sim_core::genetics::LOCI as f64
        })
        .sum::<f64>()
        / n
}
impl Measurements {
    fn sample(&mut self, world: &World, warmup: u64, interval: u64) {
        let s = &world.state;
        #[cfg(feature = "viability")]
        let feeding_window = sim_core::viability::take_energy_window();
        let n = s.organisms.len();
        self.maximum_species = self
            .maximum_species
            .max(s.species.values().filter(|sp| sp.population > 0).count());
        if s.tick < warmup {
            return;
        }
        self.minimum = self.minimum.min(n);
        self.populations.push(n as f64);
        self.food.push(
            s.environment
                .cells
                .iter()
                .map(|c| f64::from(c.total_food()))
                .sum::<f64>()
                / s.environment.cells.len() as f64,
        );
        self.diversity.push(genomic_distance(world));
        let metrics = sim_core::analysis::ecology_metrics(world);
        #[cfg(feature = "viability")]
        if s.tick > warmup {
            let consumers = metrics.primary_consumer_population;
            let predators = metrics.predator_like_population;
            let resource = feeding_window[0];
            let prey = feeding_window[1];
            let crossings = [
                consumers >= 8
                    && resource[0] > 0
                    && resource[0] * 1000 / (resource[0] + resource[1]).max(1) >= 900,
                predators >= 8 && prey[1] > 0 && prey[1] * 1000 / (prey[0] + prey[1]).max(1) >= 500,
            ];
            for (i, crossing) in crossings.into_iter().enumerate() {
                self.feeding_role_streaks[i] = if crossing {
                    self.feeding_role_streaks[i] + interval
                } else {
                    0
                };
                self.longest_feeding_role_streaks[i] =
                    self.longest_feeding_role_streaks[i].max(self.feeding_role_streaks[i]);
            }
            self.maximum_persistent_realized_feeding_roles =
                self.maximum_persistent_realized_feeding_roles.max(
                    self.feeding_role_streaks
                        .iter()
                        .filter(|t| **t >= 1000)
                        .count(),
                );
        }
        let morph_representatives = sim_core::analysis::morphology_cluster_representatives(world);
        self.morphology_streaks.retain(|id, _| {
            morph_representatives
                .iter()
                .any(|(current, _)| current == id)
        });
        for (id, profile) in morph_representatives {
            let streak = self.morphology_streaks.entry(id).or_insert((profile, 0));
            let distance = streak
                .0
                .iter()
                .zip(profile)
                .map(|(a, b)| (a - b).abs())
                .sum::<i32>()
                / 9;
            if distance >= 150 {
                *streak = (profile, 0);
            }
            streak.1 += interval;
        }
        self.maximum_persistent_morphology_clusters =
            self.maximum_persistent_morphology_clusters.max(
                self.morphology_streaks
                    .values()
                    .filter(|(_, ticks)| *ticks >= 1000)
                    .count(),
            );
        self.morphology_distance
            .push(f64::from(metrics.morphology_mean_distance));
        self.niche_distance
            .push(f64::from(metrics.niche_mean_distance));
        self.morphology_clusters
            .push(metrics.morphology_clusters as f64);
        self.species_complexity
            .push(f64::from(metrics.median_species_complexity));
        let representatives = sim_core::analysis::niche_cluster_representatives(world);
        self.niche_streaks
            .retain(|id, _| representatives.iter().any(|(current, _)| current == id));
        for (id, profile) in representatives {
            let streak = self.niche_streaks.entry(id).or_insert((profile, 0));
            let distance = streak
                .0
                .iter()
                .zip(profile)
                .map(|(a, b)| (a - b).abs())
                .sum::<i32>()
                / 6;
            if distance >= 150 {
                *streak = (profile, 0);
            }
            streak.1 += interval;
        }
        self.maximum_persistent_niche_clusters = self.maximum_persistent_niche_clusters.max(
            self.niche_streaks
                .values()
                .filter(|(_, ticks)| *ticks >= 1000)
                .count(),
        );
        let mut complexity = BTreeMap::<u64, (usize, i64)>::new();
        for o in &s.organisms {
            let entry = complexity.entry(o.lineage_id.0).or_default();
            entry.0 += 1;
            entry.1 += i64::from(o.phenotype.morphology.complexity);
        }
        self.complex_lineage_streaks
            .retain(|id, _| complexity.contains_key(id));
        for (id, (count, sum)) in complexity {
            let streak = self.complex_lineage_streaks.entry(id).or_default();
            if count >= 8 && sum / count as i64 >= 200 {
                *streak += interval;
            } else {
                *streak = 0;
            }
            if *streak >= 1000 {
                self.complex_lineages_observed.insert(id);
            }
        }
        self.maximum_persistent_complex_lineages = self.maximum_persistent_complex_lineages.max(
            self.complex_lineage_streaks
                .values()
                .filter(|ticks| **ticks >= 1000)
                .count(),
        );
        self.predator_population
            .push(metrics.predator_like_population as f64);
        self.consumer_population
            .push(metrics.primary_consumer_population as f64);
        self.maximum_complexity = self.maximum_complexity.max(metrics.maximum_complexity);
        self.multicellular_streak =
            if metrics.multicellular_population >= 8 && metrics.multicellular_population * 4 >= n {
                self.multicellular_streak + interval
            } else {
                0
            };
        self.longest_multicellular = self.longest_multicellular.max(self.multicellular_streak);
        self.niche_cluster_streak = if metrics.niche_clusters >= 2 {
            self.niche_cluster_streak + interval
        } else {
            0
        };
        self.longest_niche_clusters = self.longest_niche_clusters.max(self.niche_cluster_streak);
        if n < (s.config.population_limit / 20).max(2) {
            self.low_streak += interval;
        } else {
            self.low_streak = 0;
        }
        self.longest_low = self.longest_low.max(self.low_streak);
        if s.species.values().filter(|sp| sp.population > 0).count() >= 2 {
            self.rich_streak += interval;
        } else {
            self.rich_streak = 0;
        }
        self.longest_rich = self.longest_rich.max(self.rich_streak);
        let mut counts = BTreeMap::<u64, usize>::new();
        for o in &s.organisms {
            *counts.entry(o.lineage_id.0).or_default() += 1;
        }
        for (id, streak) in &mut self.lineage_streaks {
            if counts.get(id).copied().unwrap_or(0) < 8 {
                *streak = 0;
            }
        }
        for (id, count) in counts {
            if count >= 8 {
                *self.lineage_streaks.entry(id).or_default() += interval;
            }
        }
        self.persistent_lineages = self.persistent_lineages.max(
            self.lineage_streaks
                .values()
                .filter(|v| **v >= 1000)
                .count(),
        );
    }
}
#[derive(Clone, Copy)]
struct OutcomeEvidence {
    final_population: usize,
    limit: usize,
    longest_low: u64,
    longest_rich: u64,
    lineages: usize,
    diversity: f64,
    cv: f64,
    swings: usize,
    births: u64,
    initial: usize,
}
fn classify(e: OutcomeEvidence) -> &'static str {
    let OutcomeEvidence {
        final_population,
        limit,
        longest_low,
        longest_rich,
        lineages,
        diversity,
        cv,
        swings,
        births,
        initial,
    } = e;
    if final_population == 0 || longest_low >= 1000 {
        "Collapse"
    } else if final_population > limit {
        "Explosion"
    } else if cv >= 0.25 && swings >= 4 {
        "Oscillating"
    } else if longest_rich >= 1000 || (lineages >= 3 && diversity >= 30.0) {
        "Rich"
    } else if diversity < 10.0 && cv < 0.02 && births < initial as u64 {
        "Sterile"
    } else {
        "Healthy"
    }
}
fn measure(
    config: Config,
    ticks: u64,
    temperature: i32,
    regeneration: i32,
    sweep: bool,
) -> Result<Value, String> {
    let mut world = World::new(config.clone())?;
    let temperature = if sweep {
        1800 + (config.seed % 5) as i32 * 100
    } else {
        temperature
    };
    if temperature != 2000 || regeneration != 12 {
        world.command(Command {
            tick: 0,
            temperature,
            regeneration,
        })?;
    }
    let warmup = (ticks / 10).min(5000);
    let interval = 100;
    let mut m = Measurements {
        minimum: usize::MAX,
        ..Default::default()
    };
    let mut organism_ticks = 0u128;
    #[cfg(feature = "viability")]
    sim_core::viability::start(&world);
    let start = Instant::now();
    for _ in 0..ticks {
        organism_ticks += world.state.organisms.len() as u128;
        world.step();
        if world.state.tick % interval == 0 {
            m.sample(&world, warmup, interval);
        }
        if world.state.tick % 1000 == 0 {
            world.validate()?;
        }
    }
    world.validate()?;
    let elapsed = start.elapsed().as_secs_f64();
    #[cfg(feature = "viability")]
    let viability = sim_core::viability::finish(&world);
    #[cfg(feature = "viability")]
    if viability["cohorts"]
        .as_array()
        .expect("cohort report")
        .iter()
        .any(|c| c["energy_closure_error"] != 0 || c["population_closure_error"] != 0)
    {
        return Err("Viability accounting closure mismatch".into());
    }
    let verification_start = Instant::now();
    let encoded = sim_core::persistence::encode(&world)?;
    let mut restored = sim_core::persistence::decode(&encoded)?;
    let replayed = sim_core::replay::Replay::from_world(&world).verify()?;
    if restored.hash() != world.hash() || replayed.hash() != world.hash() {
        return Err("Calibration save/load or replay mismatch".into());
    }
    let mut continued = world.clone();
    continued.advance(1000);
    restored.advance(1000);
    continued.validate()?;
    restored.validate()?;
    if continued.hash() != restored.hash() {
        return Err("Calibration RNG continuation mismatch".into());
    }
    let verification_seconds = verification_start.elapsed().as_secs_f64();
    let s = &world.state;
    let pop_mean = mean(&m.populations);
    let cv = if pop_mean > 0.0 {
        variance(&m.populations).sqrt() / pop_mean
    } else {
        0.0
    };
    let swings = m
        .populations
        .windows(2)
        .filter(|p| (p[1] - p[0]).abs() >= config.population_limit as f64 * 0.1)
        .count();
    let trait_names = [
        "body_size",
        "speed",
        "vision",
        "metabolism",
        "temperature_tolerance",
        "aggression",
    ];
    let mut trait_variance = serde_json::Map::new();
    for (i, name) in trait_names.iter().enumerate() {
        let values: Vec<_> = s
            .organisms
            .iter()
            .map(|o| {
                let p = &o.phenotype;
                f64::from(
                    [
                        p.body_size,
                        p.speed,
                        p.vision,
                        p.metabolism,
                        p.temperature_tolerance,
                        p.aggression,
                    ][i],
                )
            })
            .collect();
        trait_variance.insert(
            name.to_string(),
            if values.is_empty() {
                Value::Null
            } else {
                json!(variance(&values))
            },
        );
    }
    let lifespans: Vec<_> = s
        .ancestry
        .values()
        .filter_map(|a| a.death.map(|(tick, _)| (tick - a.birth_tick) as f64))
        .collect();
    let first_spec = s
        .species
        .values()
        .filter(|sp| sp.ancestor.is_some())
        .map(|sp| sp.origin_tick)
        .min();
    let first_ext = s.species.values().filter_map(|sp| sp.extinct_tick).min();
    let final_pop = s.organisms.len();
    let outcome = classify(OutcomeEvidence {
        final_population: final_pop,
        limit: config.population_limit,
        longest_low: m.longest_low,
        longest_rich: m.longest_rich,
        lineages: m.persistent_lineages,
        diversity: mean(&m.diversity),
        cv,
        swings,
        births: s.counters.births,
        initial: config.starting_population,
    });
    let phase2 = json!({"morphology_cluster_distribution":summary(&m.morphology_clusters),"median_species_complexity_distribution":summary(&m.species_complexity),"persistent_high_complexity_lineages":m.complex_lineages_observed.len(),"maximum_simultaneous_persistent_high_complexity_lineages":m.maximum_persistent_complex_lineages,"maximum_persistent_niche_clusters":m.maximum_persistent_niche_clusters,"maximum_persistent_morphology_clusters":m.maximum_persistent_morphology_clusters,"save_load_replay_verified":true,"rng_continuation_ticks":1000,"verification_seconds":verification_seconds,"phase2_metrics":sim_core::analysis::ecology_metrics(&world),"morphology_distance_distribution":summary(&m.morphology_distance),"niche_distance_distribution":summary(&m.niche_distance),"predator_like_population_distribution":summary(&m.predator_population),"consumer_population_distribution":summary(&m.consumer_population),"longest_multicellular_sampled_ticks":m.longest_multicellular,"persistent_multicellular":m.longest_multicellular>=1000,"maximum_sampled_complexity":m.maximum_complexity,"longest_two_niche_cluster_sampled_ticks":m.longest_niche_clusters,"predator_starvation_deaths":s.counters.predator_starvation_deaths,"coexistence_ticks":s.counters.coexistence_ticks,"safety_ceiling_ticks":s.counters.safety_ceiling_ticks,"resource_coefficient_of_variation":if mean(&m.food)>0.0 {variance(&m.food).sqrt()/mean(&m.food)}else{0.0},"innovation_species_counts":(0..5).map(|i|s.species.values().filter(|sp|sp.innovations[i]).count()).collect::<Vec<_>>()});
    #[cfg(feature = "viability")]
    let phase2 = {
        let mut phase2 = phase2;
        phase2["viability"] = viability;
        phase2["realized_trophic_persistence"] = json!({"maximum_simultaneous_persistent_roles":m.maximum_persistent_realized_feeding_roles,
            "longest_resource_consumer_sampled_ticks":m.longest_feeding_role_streaks[0],"longest_prey_consumer_sampled_ticks":m.longest_feeding_role_streaks[1],
            "definitions":"At least8 living members; resource phenotype cohort gets>=90% actual food/prey income from resources, predator-like cohort gets>=50% from prey in each100-tick window;1000 consecutive sampled ticks after warmup. Trait labels alone are insufficient."});
        phase2
    };
    Ok(
        json!({"seed":config.seed,"phase2":phase2,"config":config,"initial_environment":{"temperature":temperature,"regeneration":regeneration},"ticks":ticks,"elapsed_seconds":elapsed,"ticks_per_second":ticks as f64/elapsed,"organism_updates":organism_ticks.to_string(),"organism_updates_per_second":organism_ticks as f64/elapsed,"initial_population":config.starting_population,"final_population":final_pop,"peak_population":s.counters.peak_population,"minimum_population_after_warmup":if m.minimum==usize::MAX{final_pop}else{m.minimum},"population_samples":summary(&m.populations),"species_formed":s.species.len()-1,"maximum_simultaneous_species":m.maximum_species,"final_living_species":s.species.values().filter(|sp|sp.population>0).count(),"extinct_species":s.species.values().filter(|sp|sp.extinct_tick.is_some()).count(),"first_speciation_tick":first_spec,"first_extinction_tick":first_ext,"births":s.counters.births,"deaths":s.counters.deaths,"mutations":s.counters.mutations,"predations":s.counters.predations,"genetic_diversity_approximation":summary(&m.diversity),"final_mean_genome_distance_to_centroid":genomic_distance(&world),"final_trait_variance":trait_variance,"completed_lifespans":lifespans.len(),"mean_completed_lifespan":if lifespans.is_empty(){Value::Null}else{json!(mean(&lifespans))},"food_per_cell":summary(&m.food),"temperature":temperature,"temperature_offsets":summary(&s.environment.cells.iter().map(|c|f64::from(c.temperature_offset)).collect::<Vec<_>>()),"population_coefficient_of_variation":cv,"large_sample_swings":swings,"longest_low_population_sampled_ticks":m.longest_low,"longest_multiple_species_sampled_ticks":m.longest_rich,"maximum_persistent_lineages":m.persistent_lineages,"lineages_formed":s.lineages.len(),"warmup_ticks":warmup,"sample_interval":interval,"classification":outcome,"world_hash":world.hash()}),
    )
}
pub fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    let val = |name: &str| args.windows(2).find(|p| p[0] == name).map(|p| p[1].clone());
    let seeds = super::argument("--seeds", 250);
    let ticks = super::argument("--ticks", 100_000);
    let workers = super::argument("--workers", 8).clamp(1, 32);
    if seeds == 0 || ticks < 100 {
        return Err("Calibration requires seeds > 0 and ticks >= 100".into());
    }
    let config = Config {
        seed: super::argument("--seed", 0),
        starting_population: super::argument("--population", 50) as usize,
        population_limit: super::argument("--limit", 200) as usize,
        size: super::argument("--size", 512) as i32,
        mutation_multiplier: super::argument("--mutation", 100) as u32,
    };
    config.validate()?;
    let temperature = val("--temperature")
        .map(|v| v.parse::<i32>().map_err(|e| e.to_string()))
        .transpose()?
        .unwrap_or(2000);
    let regeneration = super::argument("--regeneration", 12) as i32;
    Command {
        tick: 0,
        temperature,
        regeneration,
    }
    .validate()?;
    let sweep = args.iter().any(|a| a == "--temperature-sweep");
    let progress = val("--output")
        .map(std::fs::File::create)
        .transpose()
        .map_err(|e| e.to_string())?
        .map(|f| Arc::new(Mutex::new(f)));
    let next = Arc::new(AtomicU64::new(0));
    let start = Instant::now();
    let mut handles = Vec::new();
    for _ in 0..workers {
        let next = next.clone();
        let config = config.clone();
        let progress = progress.clone();
        handles.push(std::thread::spawn(move || -> Result<Vec<Value>, String> {
            let mut results = Vec::new();
            loop {
                let index = next.fetch_add(1, Ordering::Relaxed);
                if index >= seeds {
                    break;
                }
                let seed = config
                    .seed
                    .checked_add(index)
                    .ok_or("Seed range overflow")?;
                let checked = std::panic::catch_unwind(|| {
                    measure(
                        Config {
                            seed,
                            ..config.clone()
                        },
                        ticks,
                        temperature,
                        regeneration,
                        sweep,
                    )
                });
                let result = match checked {
                    Ok(Ok(v)) => v,
                    Ok(Err(error)) => json!({"seed":seed,"error":error}),
                    Err(_) => json!({"seed":seed,"error":"panic"}),
                };
                if let Some(file) = &progress {
                    let mut f = file.lock().map_err(|e| e.to_string())?;
                    writeln!(f, "{result}").map_err(|e| e.to_string())?;
                    f.flush().map_err(|e| e.to_string())?;
                }
                eprintln!(
                    "calibration seed {seed} complete: {}",
                    result["classification"]
                );
                results.push(result);
            }
            Ok(results)
        }));
    }
    let mut results = Vec::new();
    for h in handles {
        results.extend(h.join().map_err(|_| "Calibration worker panicked")??);
    }
    results.sort_by_key(|r| r["seed"].as_u64());
    let failures = results.iter().filter(|r| r.get("error").is_some()).count();
    let mut classifications = BTreeMap::new();
    for name in [
        "Collapse",
        "Explosion",
        "Oscillating",
        "Rich",
        "Sterile",
        "Healthy",
    ] {
        classifications.insert(
            name,
            results
                .iter()
                .filter(|r| r["classification"] == name)
                .count(),
        );
    }
    let mut report = json!({"scenario":val("--label").unwrap_or_else(||"Phase1 Baseline".into()),"simulation_version":SIMULATION_VERSION,"rules_revision":sim_core::model::SIMULATION_RULES_REVISION,"analysis_version":sim_core::analysis::ANALYSIS_VERSION,"base_config":config,"temperature_sweep":sweep,"seeds":seeds,"ticks_per_seed":ticks,"workers":workers,"elapsed_seconds":start.elapsed().as_secs_f64(),"failures":failures,"classifications":classifications,"final_population_distribution":summary(&results.iter().filter_map(|r|r["final_population"].as_f64()).collect::<Vec<_>>())});
    // Move large diagnostic ledgers instead of serializing/cloning the whole array.
    report["results"] = Value::Array(results);
    println!("{report}");
    if failures > 0 {
        Err(format!("{failures} calibration failures"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complex_lineage_persistence_requires_cohort_duration_and_resets() {
        let mut world = World::new(Config {
            starting_population: 16,
            ..Config::default()
        })
        .unwrap();
        for o in &mut world.state.organisms {
            o.genome.0[14] = 750;
            o.phenotype = o.genome.phenotype();
        }
        let mut m = Measurements {
            minimum: usize::MAX,
            ..Default::default()
        };
        for _ in 0..9 {
            m.sample(&world, 0, 100);
        }
        assert_eq!(m.complex_lineages_observed.len(), 0);
        m.sample(&world, 0, 100);
        assert_eq!(m.complex_lineages_observed.len(), 1);
        assert_eq!(m.maximum_persistent_complex_lineages, 1);
        world.state.organisms.truncate(7);
        m.sample(&world, 0, 100);
        assert_eq!(m.complex_lineage_streaks[&1], 0);
    }
    #[test]
    fn niche_persistence_tracks_representative_and_bounded_profile_drift() {
        let mut world = World::new(Config::default()).unwrap();
        let mut m = Measurements {
            minimum: usize::MAX,
            ..Default::default()
        };
        for _ in 0..9 {
            m.sample(&world, 0, 100);
        }
        assert_eq!(m.maximum_persistent_niche_clusters, 0);
        m.sample(&world, 0, 100);
        assert_eq!(m.maximum_persistent_niche_clusters, 1);
        world
            .state
            .species
            .get_mut(&sim_core::model::SpeciesId(1))
            .unwrap()
            .niche = [1000; 6];
        m.sample(&world, 0, 100);
        assert_eq!(m.niche_streaks[&1].1, 100);
    }
    #[test]
    fn morphology_persistence_requires_duration_and_resets_on_species_loss() {
        let mut world = World::new(Config::default()).unwrap();
        let mut m = Measurements {
            minimum: usize::MAX,
            ..Default::default()
        };
        for _ in 0..9 {
            m.sample(&world, 0, 100);
        }
        assert_eq!(m.maximum_persistent_morphology_clusters, 0);
        m.sample(&world, 0, 100);
        assert_eq!(m.maximum_persistent_morphology_clusters, 1);
        world
            .state
            .species
            .values_mut()
            .for_each(|s| s.population = 7);
        m.sample(&world, 0, 100);
        assert!(m.morphology_streaks.is_empty());
        world
            .state
            .species
            .values_mut()
            .for_each(|s| s.population = 50);
        m.sample(&world, 0, 100);
        assert!(m.morphology_streaks.values().all(|(_, t)| *t == 100));
    }
    #[test]
    #[cfg(feature = "viability")]
    fn predator_traits_without_observed_prey_energy_do_not_prove_a_trophic_role() {
        let mut world = World::new(Config::default()).unwrap();
        sim_core::viability::start(&world);
        let mut m = Measurements {
            minimum: usize::MAX,
            ..Default::default()
        };
        for _ in 0..10 {
            world.advance(100);
            // Deliberately omit income evidence: living predator traits alone
            // must never fill a missing observation window.
            sim_core::viability::take_energy_window();
            m.sample(&world, 0, 100);
        }
        assert!(m.predator_population.iter().any(|n| *n >= 8.0));
        assert_eq!(m.longest_feeding_role_streaks[1], 0);
        assert_eq!(m.maximum_persistent_realized_feeding_roles, 0);
        sim_core::viability::finish(&world);
    }
    #[test]
    fn classification_requires_evidence_not_species_absence() {
        let e = OutcomeEvidence {
            final_population: 200,
            limit: 200,
            longest_low: 0,
            longest_rich: 0,
            lineages: 1,
            diversity: 40.0,
            cv: 0.01,
            swings: 0,
            births: 5000,
            initial: 50,
        };
        assert_eq!(classify(e), "Healthy");
        assert_eq!(
            classify(OutcomeEvidence {
                final_population: 0,
                ..e
            }),
            "Collapse"
        );
        assert_eq!(
            classify(OutcomeEvidence {
                longest_rich: 1000,
                ..e
            }),
            "Rich"
        );
        assert_eq!(
            classify(OutcomeEvidence {
                final_population: 50,
                diversity: 2.0,
                cv: 0.0,
                births: 0,
                ..e
            }),
            "Sterile"
        );
        assert_eq!(
            classify(OutcomeEvidence {
                final_population: 100,
                cv: 0.3,
                swings: 4,
                ..e
            }),
            "Oscillating"
        );
        assert_eq!(
            classify(OutcomeEvidence {
                final_population: 201,
                ..e
            }),
            "Explosion"
        );
    }
    #[test]
    fn observations_do_not_change_rng_or_world() {
        let world = World::new(Config::default()).unwrap();
        let before = world.hash();
        let mut m = Measurements {
            minimum: usize::MAX,
            ..Default::default()
        };
        m.sample(&world, 0, 100);
        assert_eq!(world.hash(), before);
        assert!(m.diversity[0] > 0.0);
    }
    #[test]
    fn metrics_reproduce_world_without_observer() {
        let config = Config {
            starting_population: 50,
            population_limit: 200,
            ..Config::default()
        };
        let result = measure(config.clone(), 1000, 2000, 12, false).unwrap();
        let mut world = World::new(config).unwrap();
        world.advance(1000);
        assert_eq!(result["world_hash"], world.hash());
        assert_eq!(result["births"], world.state.counters.births);
    }
}
