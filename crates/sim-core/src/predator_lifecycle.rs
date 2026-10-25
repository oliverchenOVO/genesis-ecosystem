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
/// Lifetime energy ledger of one predator, closed exactly at death or at finish.
/// Requested metabolic components are the authoritative cost terms; the actual debit is
/// clamped to stored energy, and the difference is the starvation shortfall.
#[derive(Clone, Copy, Default, Serialize)]
pub struct Budget {
    pub(crate) individuals: u64,
    pub(crate) organism_ticks: u64,
    pub(crate) initial_energy: u64,
    pub(crate) food_income: u64,
    pub(crate) prey_income: u64,
    pub(crate) basal: u64,
    pub(crate) maintenance: u64,
    /// Movement indexed by the tick's committed behavior: SeekFood, SeekMate, Flee, Hunt,
    /// Explore, Rest. Hunt movement is the pursuit cost.
    pub(crate) movement: [u64; 6],
    pub(crate) behavior_ticks: [u64; 6],
    pub(crate) thermal: u64,
    pub(crate) actual_metabolic: u64,
    pub(crate) metabolic_shortfall: u64,
    pub(crate) starving_ticks: u64,
    pub(crate) attacks: u64,
    pub(crate) attack_cost: u64,
    pub(crate) kills: u64,
    pub(crate) reproduction_cost: u64,
    pub(crate) matings: u64,
    pub(crate) offspring: u64,
    pub(crate) final_energy: u64,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Origin {
    Founder,
    FounderDescended,
    DeNovo,
    DeNovoDescended,
}
#[derive(Default)]
struct Collector {
    founders: u64,
    births: BTreeMap<u64, Births>,
    deaths: BTreeMap<u64, Deaths>,
    presence: BTreeMap<u64, Presence>,
    living: BTreeMap<u64, (Origin, u64, Budget)>,
    /// (origin, born after warmup, outcome) -> summed lifetime budgets.
    lifetimes: BTreeMap<(Origin, bool, &'static str), Budget>,
}
const WARMUP: u64 = 5000;
fn origin_name(o: Origin) -> &'static str {
    match o {
        Origin::Founder => "Founder",
        Origin::FounderDescended => "FounderDescended",
        Origin::DeNovo => "DeNovo",
        Origin::DeNovoDescended => "DeNovoDescended",
    }
}
fn add(total: &mut Budget, b: &Budget) {
    macro_rules! sum { ($($f:ident),*) => { $(total.$f += b.$f;)* } }
    sum!(
        individuals,
        organism_ticks,
        initial_energy,
        food_income,
        prey_income,
        basal,
        maintenance,
        thermal,
        actual_metabolic,
        metabolic_shortfall,
        starving_ticks,
        attacks,
        attack_cost,
        kills,
        reproduction_cost,
        matings,
        offspring,
        final_energy
    );
    for i in 0..6 {
        total.movement[i] += b.movement[i];
        total.behavior_ticks[i] += b.behavior_ticks[i];
    }
}
/// Applies `f` to the living predator's budget; non-predators and inactive collectors no-op.
pub fn energy(o: &Organism, f: impl FnOnce(&mut Budget)) {
    if !predator(o) {
        return;
    }
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            f(&mut c.living.get_mut(&o.id.0).expect("tracked predator").2);
        }
    });
}
pub fn metabolism(o: &Organism, components: [i32; 4], actual: i32, starving: bool) {
    energy(o, |b| {
        b.organism_ticks += 1;
        b.basal += components[0] as u64;
        b.maintenance += components[1] as u64;
        b.movement[o.behavior as usize] += components[2] as u64;
        b.behavior_ticks[o.behavior as usize] += 1;
        b.thermal += components[3] as u64;
        b.actual_metabolic += actual as u64;
        b.metabolic_shortfall += (components.iter().sum::<i32>() - actual) as u64;
        b.starving_ticks += u64::from(starving);
    });
}
fn close(c: &mut Collector, o: &Organism, outcome: &'static str) {
    let (origin, born, mut b) = c.living.remove(&o.id.0).expect("tracked predator");
    b.individuals = 1;
    b.final_energy = o.energy.max(0) as u64;
    let input = b.initial_energy + b.food_income + b.prey_income;
    let output = b.actual_metabolic + b.attack_cost + b.reproduction_cost + b.final_energy;
    assert_eq!(
        input, output,
        "predator {} energy ledger must close",
        o.id.0
    );
    add(
        c.lifetimes
            .entry((origin, born >= WARMUP, outcome))
            .or_default(),
        &b,
    );
}
thread_local! { static ACTIVE: RefCell<Option<Collector>> = const { RefCell::new(None) }; }

pub fn predator(o: &Organism) -> bool {
    o.phenotype.carnivory > 650 && o.phenotype.morphology.mouth != Mouth::Grazer
}
fn window(tick: u64) -> u64 {
    tick.saturating_sub(1) / 1000
}

pub fn start(world: &World) {
    let living: BTreeMap<_, _> = world
        .state
        .organisms
        .iter()
        .filter(|o| predator(o))
        .map(|o| {
            let budget = Budget {
                initial_energy: o.energy.max(0) as u64,
                ..Default::default()
            };
            (o.id.0, (Origin::Founder, world.state.tick, budget))
        })
        .collect();
    ACTIVE.with(|a| {
        *a.borrow_mut() = Some(Collector {
            founders: living.len() as u64,
            living,
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
        let parent_origins: Vec<_> = [a, b]
            .iter()
            .filter_map(|p| c.living.get(&p.id.0).map(|l| l.0))
            .collect();
        let origin = if parent_origins
            .iter()
            .any(|o| matches!(o, Origin::Founder | Origin::FounderDescended))
        {
            Origin::FounderDescended
        } else if parent_origins.is_empty() {
            Origin::DeNovo
        } else {
            Origin::DeNovoDescended
        };
        let budget = Budget {
            initial_energy: child.energy.max(0) as u64,
            ..Default::default()
        };
        c.living.insert(child.id.0, (origin, tick, budget));
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
        close(
            c,
            o,
            match cause {
                DeathCause::Starvation => "Starvation",
                DeathCause::Predation => "Predation",
                DeathCause::Environment => "Environment",
                DeathCause::Age => "Age",
            },
        );
    });
}
pub fn finish(world: &World) -> serde_json::Value {
    let mut c = ACTIVE
        .with(|a| a.borrow_mut().take())
        .expect("predator lifecycle started");
    for o in world.state.organisms.iter().filter(|o| predator(o)) {
        close(&mut c, o, "Censored");
    }
    assert!(c.living.is_empty(), "every tracked predator is closed");
    serde_json::json!({"version":2,"window_ticks":1000,"predator_founders":c.founders,
        "lifetime_energy":c.lifetimes.iter().map(|((origin,post,outcome),v)|serde_json::json!({"origin":origin_name(*origin),"born_after_warmup":post,"outcome":outcome,"totals":v})).collect::<Vec<_>>(),
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
        let p = super::finish(&world);
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
        let life = |k: &str| sum(&p["lifetime_energy"], &format!("/{k}"));
        for (mine, theirs) in [
            ("food_income", "food_energy"),
            ("prey_income", "prey_energy"),
            ("attack_cost", "attack_energy"),
            ("reproduction_cost", "reproductive_energy"),
            ("actual_metabolic", "actual_metabolic_energy"),
            ("organism_ticks", "organism_ticks"),
            ("offspring", "offspring_produced"),
            ("matings", "successful_matings"),
            ("attacks", "attacks"),
            ("kills", "kills"),
            ("starving_ticks", "starvation_ticks"),
            ("basal", "requested_basal_energy"),
            ("maintenance", "requested_morphology_energy"),
            ("thermal", "requested_thermal_energy"),
        ] {
            assert_eq!(life(mine), get(theirs), "{mine}");
        }
        let movement: u64 = (0..6)
            .map(|i| sum(&p["lifetime_energy"], &format!("/movement/{i}")))
            .sum();
        assert_eq!(movement, get("requested_movement_energy"));
        assert_eq!(
            sum(&p["lifetime_energy"], "/individuals"),
            p["predator_founders"].as_u64().unwrap() + born
        );
        assert!(life("prey_income") > 0 && life("kills") > 0);
        let all: u64 = (0..3)
            .map(|i| sum(&p["births"], &format!("/children/{i}")))
            .sum();
        assert_eq!(all, world.state.counters.births);
    }
}
