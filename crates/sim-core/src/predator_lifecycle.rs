//! Optional R4 predator origin and fate ledger: which parents produce predator-phenotype
//! children, whether predator parents transmit the phenotype, and how predators die.
//! Separate collector: no State, RNG, save or frontend fields.
use crate::{morphology::Mouth, DeathCause, Organism, World};
use serde::Serialize;
use std::{cell::RefCell, collections::BTreeMap};

#[derive(Default, Serialize)]
struct Births {
    /// Children indexed by the number of predator parents (0, 1, 2).
    children: [u64; 3],
    predator_children: [u64; 3],
    /// Predator children whose parents are both non-predators, split by why each parent
    /// was not a predator: Grazer mouth, carnivory <= 650, or both.
    de_novo_parent_grazer: u64,
    de_novo_parent_low_carnivory: u64,
    de_novo_parent_both: u64,
    predator_child_carnivory_sum: u64,
    predator_child_energy_sum: u64,
    predator_child_threshold_sum: u64,
}
#[derive(Default, Serialize)]
struct Deaths {
    deaths: u64,
    starvation: u64,
    predation: u64,
    environment: u64,
    age: u64,
    lifespan_sum: u64,
    offspring_sum: u64,
    /// Died before ever being old enough to mate (age clause of `world::eligible`).
    before_maturity: u64,
    without_offspring: u64,
}
#[derive(Default, Serialize)]
struct Presence {
    predator_organism_ticks: u64,
    ticks_with_predator: u64,
    ticks_with_two_or_more: u64,
    peak_living: u64,
}
#[derive(Default)]
struct Collector {
    founders: u64,
    births: BTreeMap<u64, Births>,
    deaths: BTreeMap<u64, Deaths>,
    presence: BTreeMap<u64, Presence>,
}
thread_local! { static ACTIVE: RefCell<Option<Collector>> = const { RefCell::new(None) }; }

pub fn predator(o: &Organism) -> bool {
    o.phenotype.carnivory > 650 && o.phenotype.morphology.mouth != Mouth::Grazer
}
fn window(tick: u64) -> u64 {
    tick.saturating_sub(1) / 1000
}

pub fn start(world: &World) {
    let founders = world.state.organisms.iter().filter(|o| predator(o)).count() as u64;
    ACTIVE.with(|a| {
        *a.borrow_mut() = Some(Collector {
            founders,
            ..Default::default()
        })
    });
}
/// Living predators at the end of a completed tick.
pub fn census(world: &World) {
    ACTIVE.with(|a| {
        let mut active = a.borrow_mut();
        let Some(c) = active.as_mut() else { return };
        let living = world.state.organisms.iter().filter(|o| predator(o)).count() as u64;
        let p = c.presence.entry(window(world.state.tick)).or_default();
        p.predator_organism_ticks += living;
        p.ticks_with_predator += u64::from(living > 0);
        p.ticks_with_two_or_more += u64::from(living > 1);
        p.peak_living = p.peak_living.max(living);
    });
}
pub fn birth(child: &Organism, a: &Organism, b: &Organism, tick: u64) {
    ACTIVE.with(|active| {
        let mut active = active.borrow_mut();
        let Some(c) = active.as_mut() else { return };
        let parents = usize::from(predator(a)) + usize::from(predator(b));
        let r = c.births.entry(window(tick)).or_default();
        r.children[parents] += 1;
        if !predator(child) {
            return;
        }
        r.predator_children[parents] += 1;
        r.predator_child_carnivory_sum += child.phenotype.carnivory as u64;
        r.predator_child_energy_sum += child.energy.max(0) as u64;
        r.predator_child_threshold_sum += child.phenotype.reproduction_threshold as u64;
        if parents == 0 {
            for p in [a, b] {
                let grazer = p.phenotype.morphology.mouth == Mouth::Grazer;
                let low = p.phenotype.carnivory <= 650;
                match (grazer, low) {
                    (true, true) => r.de_novo_parent_both += 1,
                    (true, false) => r.de_novo_parent_grazer += 1,
                    _ => r.de_novo_parent_low_carnivory += 1,
                }
            }
        }
    });
}
pub fn death(o: &Organism, tick: u64, cause: DeathCause) {
    if !predator(o) {
        return;
    }
    ACTIVE.with(|a| {
        let mut active = a.borrow_mut();
        let Some(c) = active.as_mut() else { return };
        let d = c.deaths.entry(window(tick)).or_default();
        let age = tick - o.birth_tick;
        d.deaths += 1;
        match cause {
            DeathCause::Starvation => d.starvation += 1,
            DeathCause::Predation => d.predation += 1,
            DeathCause::Environment => d.environment += 1,
            DeathCause::Age => d.age += 1,
        }
        d.lifespan_sum += age;
        d.offspring_sum += u64::from(o.offspring);
        d.before_maturity += u64::from(age < 60 + o.phenotype.morphology.complexity as u64 / 4);
        d.without_offspring += u64::from(o.offspring == 0);
    });
}
pub fn finish() -> serde_json::Value {
    let c = ACTIVE
        .with(|a| a.borrow_mut().take())
        .expect("predator lifecycle started");
    serde_json::json!({"version":1,"window_ticks":1000,"predator_founders":c.founders,
        "births":c.births.iter().map(|(w,v)|serde_json::json!({"window":w,"totals":v})).collect::<Vec<_>>(),
        "deaths":c.deaths.iter().map(|(w,v)|serde_json::json!({"window":w,"totals":v})).collect::<Vec<_>>(),
        "presence":c.presence.iter().map(|(w,v)|serde_json::json!({"window":w,"totals":v})).collect::<Vec<_>>(),
        "definitions":"Predator = carnivory>650 and non-Grazer mouth (inherited phenotype), same as other R4 observers. Births by child, indexed by number of predator parents; de novo parent reasons count each non-predator parent once. Deaths of predators only; before_maturity uses the age clause of authoritative eligibility. Presence is the end-of-tick census of living predators. Descriptive counts, not causal estimates."})
}

#[cfg(test)]
mod tests {
    use crate::{Command, Config, World};
    fn sum(rows: &serde_json::Value, path: &str) -> u64 {
        rows.as_array()
            .unwrap()
            .iter()
            .map(|r| r["totals"].pointer(path).unwrap().as_u64().unwrap())
            .sum()
    }
    #[test]
    fn predator_ledger_closes_against_viability_cohort_without_hash_effect() {
        let mut world = World::new(Config {
            starting_population: 80,
            population_limit: 300,
            ..Config::default()
        })
        .unwrap();
        world
            .command(Command {
                tick: 0,
                temperature: 2000,
                regeneration: 4,
            })
            .unwrap();
        let mut plain = world.clone();
        plain.advance(3000);
        crate::viability::start(&world);
        super::start(&world);
        world.advance(3000);
        let p = super::finish();
        let v = crate::viability::finish(&world);
        assert_eq!(world.hash(), plain.hash());
        let cohort = v["cohorts"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["cohort"] == "predator-like")
            .unwrap();
        let get = |k: &str| cohort["totals"][k].as_u64().unwrap();
        let born: u64 = (0..3)
            .map(|i| sum(&p["births"], &format!("/predator_children/{i}")))
            .sum();
        assert_eq!(born, get("births"));
        assert_eq!(sum(&p["deaths"], "/deaths"), get("deaths"));
        assert_eq!(sum(&p["deaths"], "/starvation"), get("starvation_deaths"));
        assert_eq!(
            sum(&p["deaths"], "/lifespan_sum"),
            get("completed_lifespan_sum")
        );
        // Founders + births = deaths + survivors; census counts every predator once per tick.
        assert_eq!(
            p["predator_founders"].as_u64().unwrap() + born,
            get("deaths") + get("final_population")
        );
        assert_eq!(
            sum(&p["presence"], "/predator_organism_ticks"),
            get("organism_ticks") - get("initial_population") + get("final_population")
        );
        let all: u64 = (0..3)
            .map(|i| sum(&p["births"], &format!("/children/{i}")))
            .sum();
        assert_eq!(all, world.state.counters.births);
    }
}
