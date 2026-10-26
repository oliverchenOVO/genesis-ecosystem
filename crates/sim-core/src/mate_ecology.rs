//! Optional R4 mate-choice ecology: is the chosen mate ecologically closer than the pool of
//! eligible, compatible neighbours it was chosen from? Separate collector: no State, RNG,
//! save or frontend fields.
use crate::{analysis::morphology_profile, morphology::Mouth, Cell, Organism};
use serde::Serialize;
use std::{cell::RefCell, collections::BTreeMap};

/// Lifetime realized diet (soft, hard, prey energy) and feeding-habitat class counts.
#[derive(Clone, Copy, Default)]
struct History {
    diet: [u64; 3],
    habitat: [u64; 8],
}
const METRICS: [&str; 9] = [
    "mouth_differs",
    "morphology_distance",
    "diet_distance",
    "habitat_distance",
    "temperature_optimum_difference",
    "mass_difference",
    "genome_distance",
    "different_lineage",
    "different_diet_role",
];
#[derive(Clone, Copy, Default, Serialize)]
struct Sums {
    pairs: u64,
    /// Pairs where both members have any realized diet / habitat history.
    with_history: u64,
    sums: [u64; 9],
}
#[derive(Default, Serialize)]
struct Window {
    searches_with_pool: u64,
    /// Searches whose pool was non-empty but no partner was accepted (candidate rules only).
    searches_without_pair: u64,
    searches_with_choice_among_two_or_more: u64,
    chosen: Sums,
    pool: Sums,
    /// Per search with >= 2 pool members: chosen closer than the pool mean on each metric,
    /// and ties (equal to the pool mean).
    chosen_below_pool_mean: [u64; 9],
    chosen_equal_pool_mean: [u64; 9],
}
/// Perception-stage SeekMate targeting: ready organisms choosing among visible candidates.
#[derive(Default, Serialize)]
struct Targeting {
    ready_with_candidates: u64,
    /// Visible candidate counts: 1, 2..=3, 4..=7, 8+.
    pool_size_bins: [u64; 4],
    pool_members: u64,
    nearest: Sums,
    pool: Sums,
    nearest_below_pool_mean: [u64; 9],
    nearest_equal_pool_mean: [u64; 9],
    choice_searches: u64,
}
#[derive(Default)]
struct Collector {
    tick: u64,
    histories: BTreeMap<u64, History>,
    windows: BTreeMap<u64, Window>,
    targeting: BTreeMap<u64, Targeting>,
}
thread_local! { static ACTIVE: RefCell<Option<Collector>> = const { RefCell::new(None) }; }

fn habitat(c: &Cell) -> usize {
    usize::from(c.elevation >= 500) * 4
        + usize::from(c.moisture >= 500) * 2
        + usize::from(c.fertility >= 100)
}
/// The nine morphology traits used by the species morphology gate, per organism.
pub fn profile(o: &Organism) -> [i32; 9] {
    let m = &o.phenotype.morphology;
    morphology_profile([
        m.segment_count * 1000,
        m.mass,
        m.armor,
        m.bite_capacity,
        m.locomotion_efficiency,
        m.sensory_investment,
        m.complexity,
        o.phenotype.speed,
        if m.mouth == Mouth::Piercer { 1000 } else { 0 },
    ])
}
/// Permille L1 distance between two count distributions (0 identical, 2000 disjoint).
fn distribution_distance(a: &[u64], b: &[u64]) -> Option<u64> {
    let (ta, tb) = (a.iter().sum::<u64>(), b.iter().sum::<u64>());
    if ta == 0 || tb == 0 {
        return None;
    }
    Some(
        a.iter()
            .zip(b)
            .map(|(x, y)| (x * 1000 / ta).abs_diff(y * 1000 / tb))
            .sum(),
    )
}
fn role(diet: &[u64; 3]) -> u64 {
    let t = diet.iter().sum::<u64>();
    match () {
        _ if t == 0 => 0,
        _ if diet[0] * 10 >= t * 7 => 1,
        _ if diet[1] * 10 >= t * 7 => 2,
        _ if diet[2] * 2 >= t => 3,
        _ => 4,
    }
}

pub fn start() {
    ACTIVE.with(|a| *a.borrow_mut() = Some(Collector::default()));
}
pub fn tick(tick: u64) {
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            c.tick = tick;
        }
    });
}
/// Realized feeding: `channel` 0 soft, 1 hard, 2 prey.
pub fn feeding(o: &Organism, cell: &Cell, channel: usize, energy: i32) {
    if energy <= 0 {
        return;
    }
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            let h = c.histories.entry(o.id.0).or_default();
            h.diet[channel] += energy as u64;
            h.habitat[habitat(cell)] += 1;
        }
    });
}
pub fn death(o: &Organism) {
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            c.histories.remove(&o.id.0);
        }
    });
}
fn metrics(c: &Collector, a: &Organism, b: &Organism) -> ([u64; 9], bool) {
    let none = History::default();
    let ha = c.histories.get(&a.id.0).unwrap_or(&none);
    let hb = c.histories.get(&b.id.0).unwrap_or(&none);
    let diet = distribution_distance(&ha.diet, &hb.diet);
    let habitat = distribution_distance(&ha.habitat, &hb.habitat);
    let (pa, pb) = (profile(a), profile(b));
    let values = [
        u64::from(a.phenotype.morphology.mouth != b.phenotype.morphology.mouth) * 1000,
        (pa.iter()
            .zip(pb)
            .map(|(x, y)| (x - y).unsigned_abs())
            .sum::<u32>()
            / 9) as u64,
        diet.unwrap_or(0),
        habitat.unwrap_or(0),
        a.phenotype
            .temperature_optimum
            .abs_diff(b.phenotype.temperature_optimum) as u64,
        a.phenotype
            .morphology
            .mass
            .abs_diff(b.phenotype.morphology.mass) as u64,
        u64::from(a.genome.distance(&b.genome)),
        u64::from(a.lineage_id != b.lineage_id) * 1000,
        u64::from(role(&ha.diet) != role(&hb.diet)) * 1000,
    ];
    (values, diet.is_some() && habitat.is_some())
}
fn add(s: &mut Sums, values: &[u64; 9], history: bool) {
    s.pairs += 1;
    s.with_history += u64::from(history);
    for (t, v) in s.sums.iter_mut().zip(values) {
        *t += v;
    }
}
/// `pool` must be exactly the eligible, compatible neighbours matching chose `chosen` from.
pub fn search<'a>(
    a: &Organism,
    pool: impl Iterator<Item = &'a Organism>,
    chosen: Option<&Organism>,
) {
    ACTIVE.with(|active| {
        let mut active = active.borrow_mut();
        let Some(c) = active.as_mut() else { return };
        let pool: Vec<_> = pool.map(|b| metrics(c, a, b)).collect();
        if pool.is_empty() {
            return;
        }
        let chosen = chosen.map(|b| metrics(c, a, b));
        let w = c
            .windows
            .entry(c.tick.saturating_sub(1) / 5000)
            .or_default();
        w.searches_with_pool += 1;
        let Some(chosen) = chosen else {
            w.searches_without_pair += 1;
            return;
        };
        add(&mut w.chosen, &chosen.0, chosen.1);
        for (values, history) in &pool {
            add(&mut w.pool, values, *history);
        }
        if pool.len() >= 2 {
            w.searches_with_choice_among_two_or_more += 1;
            for m in 0..9 {
                // Compare chosen * n with the pool sum to stay in integers.
                let sum: u64 = pool.iter().map(|(v, _)| v[m]).sum();
                let scaled = chosen.0[m] * pool.len() as u64;
                w.chosen_below_pool_mean[m] += u64::from(scaled < sum);
                w.chosen_equal_pool_mean[m] += u64::from(scaled == sum);
            }
        }
    });
}
/// `pool` is every visible candidate passing the existing SeekMate filter; `nearest` is the
/// target the existing rule selects. Called only for organisms that may choose SeekMate.
pub fn targeting<'a>(
    a: &Organism,
    pool: impl Iterator<Item = &'a Organism>,
    nearest: Option<&Organism>,
) {
    ACTIVE.with(|active| {
        let mut active = active.borrow_mut();
        let Some(c) = active.as_mut() else { return };
        let pool: Vec<_> = pool.map(|b| metrics(c, a, b)).collect();
        if pool.is_empty() {
            return;
        }
        let nearest = metrics(c, a, nearest.expect("non-empty pool has a nearest target"));
        let t = c
            .targeting
            .entry(c.tick.saturating_sub(1) / 5000)
            .or_default();
        t.ready_with_candidates += 1;
        t.pool_members += pool.len() as u64;
        t.pool_size_bins[match pool.len() {
            1 => 0,
            2..=3 => 1,
            4..=7 => 2,
            _ => 3,
        }] += 1;
        add(&mut t.nearest, &nearest.0, nearest.1);
        for (values, history) in &pool {
            add(&mut t.pool, values, *history);
        }
        if pool.len() >= 2 {
            t.choice_searches += 1;
            for m in 0..9 {
                let sum: u64 = pool.iter().map(|(v, _)| v[m]).sum();
                let scaled = nearest.0[m] * pool.len() as u64;
                t.nearest_below_pool_mean[m] += u64::from(scaled < sum);
                t.nearest_equal_pool_mean[m] += u64::from(scaled == sum);
            }
        }
    });
}
pub fn finish() -> serde_json::Value {
    let c = ACTIVE
        .with(|a| a.borrow_mut().take())
        .expect("mate ecology started");
    serde_json::json!({"version":2,"window_ticks":5000,"metrics":METRICS,
        "windows":c.windows.iter().map(|(w,v)|serde_json::json!({"window":w,"totals":v})).collect::<Vec<_>>(),
        "targeting":c.targeting.iter().map(|(w,v)|serde_json::json!({"window":w,"totals":v})).collect::<Vec<_>>(),
        "definitions":"Per actual matching search with a non-empty pool of eligible, compatible, unpaired living neighbours within 16: the nearest-ID-tiebroken chosen mate versus every pool member. Distances: mouth differs x1000; integer mean L1 of the nine-trait gate morphology profile; permille L1 of lifetime realized soft/hard/prey diet fractions; permille L1 of feeding-habitat class fractions (8 elevation/moisture/fertility classes); temperature optimum and mass absolute differences; Genome::distance; different lineage x1000; different lifetime diet role (NoIncome/Soft/Hard/Prey/Mixed) x1000. Missing diet/habitat history contributes 0 and is counted by with_history. chosen_below_pool_mean counts searches with >=2 pool members where the chosen mate is strictly closer than the pool mean. Descriptive, not causal."})
}

#[cfg(test)]
mod tests {
    use crate::{Command, Config, World};
    #[test]
    fn chosen_mates_match_successful_pairs_and_leave_hash_unchanged() {
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
        super::start();
        world.advance(3000);
        let m = super::finish();
        let v = crate::viability::finish(&world);
        assert_eq!(world.hash(), plain.hash());
        let windows = m["windows"].as_array().unwrap();
        let get = |path: &str| -> u64 {
            windows
                .iter()
                .map(|w| w["totals"].pointer(path).unwrap().as_u64().unwrap())
                .sum()
        };
        let cohorts = &v["cohorts"].as_array().unwrap()[..4];
        let available: u64 = cohorts
            .iter()
            .map(|c| c["totals"]["available_mate_attempts"].as_u64().unwrap())
            .sum();
        let compatible: u64 = cohorts
            .iter()
            .map(|c| {
                c["totals"]["nearby_compatible_candidates"]
                    .as_u64()
                    .unwrap()
            })
            .sum();
        // Initiators with a pool; available_mate_attempts also counts the chosen partner.
        assert!(get("/searches_with_pool") > 0);
        assert_eq!(
            get("/searches_without_pair"),
            0,
            "R3 always mates with a non-empty pool"
        );
        assert_eq!(get("/chosen/pairs"), get("/searches_with_pool"));
        assert_eq!(get("/pool/pairs"), compatible);
        assert!(available >= get("/searches_with_pool"));
        assert!(get("/searches_with_choice_among_two_or_more") <= get("/searches_with_pool"));
        let targeting = m["targeting"].as_array().unwrap();
        let tget = |path: &str| -> u64 {
            targeting
                .iter()
                .map(|w| w["totals"].pointer(path).unwrap().as_u64().unwrap())
                .sum()
        };
        assert!(tget("/ready_with_candidates") > 0);
        assert_eq!(tget("/nearest/pairs"), tget("/ready_with_candidates"));
        assert_eq!(tget("/pool/pairs"), tget("/pool_members"));
        let bins: u64 = (0..4).map(|i| tget(&format!("/pool_size_bins/{i}"))).sum();
        assert_eq!(bins, tget("/ready_with_candidates"));
    }
}
