//! Optional, deterministic realized-fitness accounting. Never part of State, RNG,
//! persistence, snapshots or the normal desktop build. One collector per worker.
use crate::{morphology::Mouth, DeathCause, Organism, World};
use serde::Serialize;
use std::cell::RefCell;

const NAMES: [&str; 9] = [
    "C<80",
    "80<=C<140",
    "140<=C<200",
    "C>=200",
    "single-unit",
    "multi-unit",
    "high-complexity",
    "Piercer",
    "predator-like",
];

#[derive(Clone, Default, Serialize)]
pub struct Cohort {
    pub initial_population: u64,
    pub births: u64,
    pub deaths: u64,
    pub final_population: u64,
    pub organism_ticks: u64,
    pub initial_energy: u64,
    pub offspring_initial_energy: u64,
    pub food_energy: u64,
    pub prey_energy: u64,
    pub requested_basal_energy: u64,
    pub requested_morphology_energy: u64,
    pub requested_movement_energy: u64,
    pub requested_thermal_energy: u64,
    pub actual_metabolic_energy: u64,
    pub attack_energy: u64,
    pub reproductive_energy: u64,
    pub removed_death_energy: u64,
    pub final_energy: u64,
    pub food_attempt_ticks: u64,
    pub successful_feeding_ticks: u64,
    pub resource_units: u64,
    pub available_resource_sum: u64,
    pub feeding_efficiency_sum: u64,
    pub feeding_quota_sum: u64,
    pub effective_speed_sum: u64,
    pub construction_cost_sum: u64,
    pub capacity_sum: u64,
    pub energy_sum: u64,
    pub reproduction_ready_ticks: u64,
    pub perceptible_mate_ticks: u64,
    pub perceptible_prey_ticks: u64,
    pub hunt_ticks: u64,
    pub mating_attempts: u64,
    pub available_mate_attempts: u64,
    pub initiating_mate_searches: u64,
    pub nearby_unpaired_candidates: u64,
    pub nearby_eligible_candidates: u64,
    pub nearby_compatible_candidates: u64,
    pub nearby_mean_distance_compatible_candidates: u64,
    pub nearby_original_loci_compatible_candidates: u64,
    pub nearby_structural_loci_compatible_candidates: u64,
    pub nearby_segmentation_compatible_candidates: u64,
    pub nearby_mouth_compatible_candidates: u64,
    pub nearby_carnivory_compatible_candidates: u64,
    pub successful_matings: u64,
    pub offspring_produced: u64,
    pub multiunit_offspring_produced: u64,
    pub high_complexity_offspring_produced: u64,
    pub same_lineage_offspring_produced: u64,
    pub offspring_complexity_sum: u64,
    pub attacks: u64,
    pub kills: u64,
    pub starvation_ticks: u64,
    pub starvation_deaths: u64,
    pub predation_deaths: u64,
    pub environment_deaths: u64,
    pub age_deaths: u64,
    pub completed_lifespan_sum: u64,
    pub completed_offspring_sum: u64,
    pub censored_age_sum: u64,
    pub censored_offspring_sum: u64,
}

#[derive(Default)]
struct Collector {
    cohorts: [Cohort; 9],
    energy_window: [[u64; 2]; 2],
}
thread_local! { static ACTIVE: RefCell<Option<Collector>> = const { RefCell::new(None) }; }

fn indices(o: &Organism) -> impl Iterator<Item = usize> {
    let m = &o.phenotype.morphology;
    let bucket = match m.complexity {
        0..=79 => 0,
        80..=139 => 1,
        140..=199 => 2,
        _ => 3,
    };
    [
        Some(bucket),
        Some(if m.segment_count > 1 { 5 } else { 4 }),
        (m.complexity >= 200).then_some(6),
        (m.mouth == Mouth::Piercer).then_some(7),
        (o.phenotype.carnivory > 650 && m.mouth != Mouth::Grazer).then_some(8),
    ]
    .into_iter()
    .flatten()
}
fn update(o: &Organism, mut f: impl FnMut(&mut Cohort)) {
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            for i in indices(o) {
                f(&mut c.cohorts[i]);
            }
            crate::ecological_diagnostics::budget(o, &mut f);
        }
    });
}

/// Start only at a world boundary; replay and restored continuations stay unobserved.
pub fn start(world: &World) {
    crate::ecological_diagnostics::start(world);
    ACTIVE.with(|a| *a.borrow_mut() = Some(Collector::default()));
    for o in &world.state.organisms {
        update(o, |c| {
            c.initial_population += 1;
            c.initial_energy += o.energy as u64;
        });
    }
}
pub fn tick(o: &Organism, ready: bool) {
    update(o, |c| {
        c.organism_ticks += 1;
        c.energy_sum += o.energy as u64;
        c.capacity_sum += o.phenotype.energy_capacity as u64;
        c.construction_cost_sum += o.phenotype.morphology.reproduction_cost as u64;
        c.reproduction_ready_ticks += u64::from(ready);
    });
}
pub fn perception(o: &Organism, mate: bool, prey: bool) {
    update(o, |c| {
        c.perceptible_mate_ticks += u64::from(mate);
        c.perceptible_prey_ticks += u64::from(prey);
    });
}
pub fn movement(o: &Organism, speed: i32, hunt: bool) {
    update(o, |c| {
        c.effective_speed_sum += speed as u64;
        c.hunt_ticks += u64::from(hunt);
    });
}
pub fn food(o: &Organism, available: i32, efficiency: i32, quota: i32, amount: i32, gained: i32) {
    income(o, gained as u64, 0);
    update(o, |c| {
        c.food_attempt_ticks += 1;
        c.available_resource_sum += available as u64;
        c.feeding_efficiency_sum += efficiency.max(0) as u64;
        c.feeding_quota_sum += quota as u64;
        c.resource_units += amount as u64;
        c.food_energy += gained as u64;
        c.successful_feeding_ticks += u64::from(gained > 0);
    });
}
pub fn attack(o: &Organism, spent: i32) {
    update(o, |c| {
        c.attacks += 1;
        c.attack_energy += spent as u64;
    });
}
pub fn kill(o: &Organism, gained: i32) {
    income(o, 0, gained as u64);
    update(o, |c| {
        c.kills += 1;
        c.prey_energy += gained as u64;
    });
}
fn income(o: &Organism, food: u64, prey: u64) {
    let predator = o.phenotype.carnivory > 650 && o.phenotype.morphology.mouth != Mouth::Grazer;
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            c.energy_window[usize::from(predator)][0] += food;
            c.energy_window[usize::from(predator)][1] += prey;
        }
    });
}
/// Observed realized food/prey income since the preceding sample, not trait labels.
pub fn take_energy_window() -> [[u64; 2]; 2] {
    ACTIVE.with(|a| {
        a.borrow_mut()
            .as_mut()
            .map(|c| std::mem::take(&mut c.energy_window))
            .unwrap_or_default()
    })
}
pub fn metabolism(o: &Organism, components: [i32; 4], actual: i32, starving: bool) {
    update(o, |c| {
        c.requested_basal_energy += components[0] as u64;
        c.requested_morphology_energy += components[1] as u64;
        c.requested_movement_energy += components[2] as u64;
        c.requested_thermal_energy += components[3] as u64;
        c.actual_metabolic_energy += actual as u64;
        c.starvation_ticks += u64::from(starving);
    });
}
pub fn death(o: &Organism, tick: u64, cause: DeathCause) {
    crate::ecological_diagnostics::death(o);
    update(o, |c| {
        c.deaths += 1;
        c.removed_death_energy += o.energy as u64;
        c.completed_lifespan_sum += tick - o.birth_tick;
        c.completed_offspring_sum += u64::from(o.offspring);
        match cause {
            DeathCause::Starvation => c.starvation_deaths += 1,
            DeathCause::Predation => c.predation_deaths += 1,
            DeathCause::Environment => c.environment_deaths += 1,
            DeathCause::Age => c.age_deaths += 1,
        }
    });
}
pub fn mating_attempt(o: &Organism, available: bool) {
    update(o, |c| {
        c.mating_attempts += 1;
        c.available_mate_attempts += u64::from(available);
    });
}
pub fn mate_search(o: &Organism, candidates: [u64; 9]) {
    update(o, |c| {
        c.initiating_mate_searches += 1;
        c.nearby_unpaired_candidates += candidates[0];
        c.nearby_eligible_candidates += candidates[1];
        c.nearby_compatible_candidates += candidates[2];
        c.nearby_mean_distance_compatible_candidates += candidates[3];
        c.nearby_original_loci_compatible_candidates += candidates[4];
        c.nearby_structural_loci_compatible_candidates += candidates[5];
        c.nearby_segmentation_compatible_candidates += candidates[6];
        c.nearby_mouth_compatible_candidates += candidates[7];
        c.nearby_carnivory_compatible_candidates += candidates[8];
    });
}
pub fn reproduction(o: &Organism, spent: i32, children: u32) {
    update(o, |c| {
        c.successful_matings += u64::from(children > 0);
        c.offspring_produced += u64::from(children);
        c.reproductive_energy += spent as u64;
    });
}
pub fn parent_child(parent: &Organism, child: &Organism) {
    update(parent, |c| {
        c.multiunit_offspring_produced += u64::from(child.phenotype.morphology.segment_count > 1);
        c.high_complexity_offspring_produced +=
            u64::from(child.phenotype.morphology.complexity >= 200);
        c.same_lineage_offspring_produced += u64::from(child.lineage_id == parent.lineage_id);
        c.offspring_complexity_sum += child.phenotype.morphology.complexity as u64;
    });
}
pub fn birth(o: &Organism) {
    update(o, |c| {
        c.births += 1;
        c.offspring_initial_energy += o.energy as u64;
    });
}

/// Exact integral energy closure per overlapping cohort; zero-sized rates are null.
pub fn finish(world: &World) -> serde_json::Value {
    for o in &world.state.organisms {
        update(o, |c| {
            c.final_population += 1;
            c.final_energy += o.energy as u64;
            c.censored_age_sum += world.state.tick - o.birth_tick;
            c.censored_offspring_sum += u64::from(o.offspring);
        });
    }
    let collector = ACTIVE
        .with(|a| a.borrow_mut().take())
        .expect("viability start required");
    let ratio = |n: u64, d: u64| (d > 0).then(|| n as f64 / d as f64);
    let rows: Vec<_> = collector.cohorts.iter().enumerate().map(|(i,c)| {
        let input = c.initial_energy + c.offspring_initial_energy + c.food_energy + c.prey_energy;
        let output = c.actual_metabolic_energy + c.attack_energy + c.reproductive_energy + c.removed_death_energy + c.final_energy;
        serde_json::json!({"cohort":NAMES[i],"totals":c,"energy_closure_error":i128::from(input)-i128::from(output),
            "population_closure_error":i128::from(c.initial_population+c.births)-i128::from(c.deaths+c.final_population),
            "mean_intake_per_organism_tick":ratio(c.food_energy+c.prey_energy,c.organism_ticks),
            "mean_expenditure_per_organism_tick":ratio(c.actual_metabolic_energy+c.attack_energy+c.reproductive_energy,c.organism_ticks),
            "mean_completed_lifespan":ratio(c.completed_lifespan_sum,c.deaths),
            "mean_completed_offspring":ratio(c.completed_offspring_sum,c.deaths),
            "offspring_per_organism_tick":ratio(c.offspring_produced,c.organism_ticks),
            "mating_success_per_attempt":ratio(c.successful_matings,c.mating_attempts),
            "mean_effective_speed":ratio(c.effective_speed_sum,c.organism_ticks),
            "mean_construction_cost":ratio(c.construction_cost_sum,c.organism_ticks),
            "mean_resource_available_per_attempt":ratio(c.available_resource_sum,c.food_attempt_ticks),
            "mean_food_energy_per_feeding_attempt":ratio(c.food_energy,c.food_attempt_ticks),
            "starvation_death_fraction":ratio(c.starvation_deaths,c.deaths),
            "predation_death_fraction":ratio(c.predation_deaths,c.deaths)})
    }).collect();
    serde_json::json!({"diagnostic_version":4,"ecology":crate::ecological_diagnostics::finish(),"tick":world.state.tick,"cohorts":rows,
        "definitions":"Overlapping inherited phenotype cohorts, whole-run exposure; parent offspring counted per parent, births by child phenotype. Requested metabolic components may exceed actual energy when depleted; actual total is clamped. Completed lifetimes exclude right-censored survivors, recorded separately. No counters in authoritative State or compact UI."})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Command, Config};
    #[test]
    fn accounting_closes_with_births_deaths_replay_and_no_observer_hash_effect() {
        let config = Config {
            starting_population: 50,
            population_limit: 200,
            ..Config::default()
        };
        let mut world = World::new(config).unwrap();
        world
            .command(Command {
                tick: 0,
                temperature: 2000,
                regeneration: 4,
            })
            .unwrap();
        let mut unobserved = world.clone();
        unobserved.advance(3000);
        start(&world);
        world.advance(3000);
        let report = finish(&world);
        assert_eq!(world.hash(), unobserved.hash());
        assert!(world.state.counters.births > 0 && world.state.counters.deaths > 0);
        for row in report["cohorts"].as_array().unwrap() {
            assert_eq!(row["energy_closure_error"], 0);
            assert_eq!(row["population_closure_error"], 0);
            assert!(
                row["totals"]["high_complexity_offspring_produced"]
                    .as_u64()
                    .unwrap()
                    <= row["totals"]["offspring_produced"].as_u64().unwrap()
            );
        }
        let rows = report["cohorts"].as_array().unwrap();
        assert_eq!(
            rows[..4]
                .iter()
                .map(|r| r["totals"]["offspring_produced"].as_u64().unwrap())
                .sum::<u64>(),
            world.state.counters.births * 2
        );
        assert_eq!(
            rows[..4]
                .iter()
                .map(|r| r["totals"]["high_complexity_offspring_produced"]
                    .as_u64()
                    .unwrap())
                .sum::<u64>(),
            rows[3]["totals"]["births"].as_u64().unwrap() * 2
        );
        assert_eq!(
            report["cohorts"][0]["totals"]["deaths"].as_u64().unwrap()
                + report["cohorts"][1]["totals"]["deaths"].as_u64().unwrap()
                + report["cohorts"][2]["totals"]["deaths"].as_u64().unwrap()
                + report["cohorts"][3]["totals"]["deaths"].as_u64().unwrap(),
            world.state.counters.deaths
        );
        let replay = crate::replay::Replay::from_world(&world).verify().unwrap();
        assert_eq!(replay.hash(), world.hash());
    }
    #[test]
    fn feeding_windows_drain_without_losing_whole_run_accounting() {
        let mut world = World::new(Config::default()).unwrap();
        start(&world);
        world.advance(100);
        let first = take_energy_window();
        assert!(first.iter().map(|r| r[0]).sum::<u64>() > 0);
        assert_eq!(take_energy_window(), [[0; 2]; 2]);
        world.advance(100);
        let second = take_energy_window();
        let report = finish(&world);
        for (role, cohort) in [(0, "single-unit"), (1, "predator-like")] {
            let row = report["cohorts"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["cohort"] == cohort)
                .unwrap();
            if role == 1 {
                assert_eq!(
                    first[role][0] + second[role][0],
                    row["totals"]["food_energy"].as_u64().unwrap()
                );
                assert_eq!(
                    first[role][1] + second[role][1],
                    row["totals"]["prey_energy"].as_u64().unwrap()
                );
            }
            assert_eq!(row["energy_closure_error"], 0);
        }
        let total_window_food = first.iter().chain(second.iter()).map(|r| r[0]).sum::<u64>();
        let total_food = report["cohorts"].as_array().unwrap()[..4]
            .iter()
            .map(|r| r["totals"]["food_energy"].as_u64().unwrap())
            .sum::<u64>();
        assert_eq!(total_window_food, total_food);
        assert_eq!(take_energy_window(), [[0; 2]; 2]);
    }
    #[test]
    fn reset_and_threads_do_not_mix_independent_worlds() {
        let run = || {
            let mut world = World::new(Config {
                starting_population: 10,
                population_limit: 50,
                ..Config::default()
            })
            .unwrap();
            start(&world);
            world.advance(100);
            finish(&world)
        };
        let first = run();
        assert_eq!(first, run());
        assert_eq!(first, std::thread::spawn(run).join().unwrap());
    }
}
