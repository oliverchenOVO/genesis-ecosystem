//! Optional R4 decomposition of actual mate searches into the authoritative eligibility and
//! compatibility clauses. Separate collector: no State, RNG, save or frontend fields.
use crate::{genetics::LOCI, morphology::Mouth, Organism};
use serde::Serialize;
use std::{cell::RefCell, collections::BTreeMap};

/// Loci with the per-locus 400 cap in `Genome::compatible`.
const CAPPED: usize = 14;

#[derive(Default, Serialize)]
struct Readiness {
    organism_ticks: u64,
    ready: u64,
    age_blocked: u64,
    cooldown_blocked: u64,
    energy_blocked: u64,
    energy_only_blocked: u64,
    energy_only_deficit_sum: u64,
}
#[derive(Default, Serialize)]
struct Searches {
    searches: u64,
    with_neighbour: u64,
    with_predator_neighbour: u64,
    with_eligible: u64,
    with_eligible_predator: u64,
    with_compatible: u64,
    successful_pairs: u64,
}
#[derive(Default, Serialize)]
struct Neighbours {
    unpaired: u64,
    age_blocked: u64,
    cooldown_blocked: u64,
    energy_blocked: u64,
    energy_only_blocked: u64,
    energy_only_deficit_sum: u64,
    eligible: u64,
    compatible: u64,
    capped_locus_rejected: u64,
    mean_only_rejected: u64,
    capped_and_mean_rejected: u64,
    capped_locus_failures: [u64; CAPPED],
    single_capped_locus_failures: [u64; CAPPED],
    eligible_mean_distance_sum: u64,
}
#[derive(Default)]
struct Collector {
    tick: u64,
    readiness: BTreeMap<(u64, bool), Readiness>,
    searches: BTreeMap<(u64, bool), Searches>,
    neighbours: BTreeMap<(u64, bool, bool, bool), Neighbours>,
    pending: Option<(u64, (u64, bool))>,
}
thread_local! { static ACTIVE: RefCell<Option<Collector>> = const { RefCell::new(None) }; }

fn predator(o: &Organism) -> bool {
    o.phenotype.carnivory > 650 && o.phenotype.morphology.mouth != Mouth::Grazer
}
/// The three clauses of `world::eligible`, kept separately: age, cooldown, energy.
fn clauses(o: &Organism, tick: u64) -> [bool; 3] {
    let wait = o.phenotype.morphology.complexity as u64 / 4;
    [
        tick - o.birth_tick < 60 + wait,
        tick - o.last_mating < 80 + wait,
        o.energy < o.phenotype.reproduction_threshold,
    ]
}
fn deficit(o: &Organism) -> u64 {
    (o.phenotype.reproduction_threshold - o.energy).max(0) as u64
}

pub fn start() {
    ACTIVE.with(|a| *a.borrow_mut() = Some(Collector::default()));
}
pub fn readiness(o: &Organism, tick: u64) {
    ACTIVE.with(|a| {
        let mut active = a.borrow_mut();
        let Some(c) = active.as_mut() else { return };
        c.tick = tick;
        let [age, cooldown, energy] = clauses(o, tick);
        let r = c
            .readiness
            .entry((tick.saturating_sub(1) / 1000, predator(o)))
            .or_default();
        r.organism_ticks += 1;
        r.ready += u64::from(!age && !cooldown && !energy);
        r.age_blocked += u64::from(age);
        r.cooldown_blocked += u64::from(cooldown);
        r.energy_blocked += u64::from(energy);
        if energy && !age && !cooldown {
            r.energy_only_blocked += 1;
            r.energy_only_deficit_sum += deficit(o);
        }
    });
}
/// `neighbours` must be exactly the unpaired, living residents within the mating radius.
pub fn search<'a>(a: &Organism, neighbours: impl Iterator<Item = &'a Organism>) {
    ACTIVE.with(|active| {
        let mut active = active.borrow_mut();
        let Some(c) = active.as_mut() else { return };
        let tick = c.tick;
        let window = tick.saturating_sub(1) / 1000;
        let key = (window, predator(a));
        let mut seen = [false; 5];
        for b in neighbours {
            let b_predator = predator(b);
            let n = c
                .neighbours
                .entry((window, key.1, b_predator, a.lineage_id == b.lineage_id))
                .or_default();
            n.unpaired += 1;
            seen[0] = true;
            seen[1] |= b_predator;
            let [age, cooldown, energy] = clauses(b, tick);
            n.age_blocked += u64::from(age);
            n.cooldown_blocked += u64::from(cooldown);
            n.energy_blocked += u64::from(energy);
            if energy && !age && !cooldown {
                n.energy_only_blocked += 1;
                n.energy_only_deficit_sum += deficit(b);
            }
            if age || cooldown || energy {
                continue;
            }
            n.eligible += 1;
            seen[2] = true;
            seen[3] |= b_predator;
            let mut total = 0u32;
            let mut failed = [false; CAPPED];
            for (locus, (x, y)) in a.genome.0.iter().zip(b.genome.0).enumerate() {
                let d = x.abs_diff(y);
                if locus < CAPPED && d > 400 {
                    failed[locus] = true;
                }
                total += u32::from(d);
            }
            n.eligible_mean_distance_sum += u64::from(total / LOCI as u32);
            let capped = failed.iter().filter(|f| **f).count();
            let mean = total >= 221 * LOCI as u32;
            for (sum, f) in n.capped_locus_failures.iter_mut().zip(failed) {
                *sum += u64::from(f);
            }
            if capped == 1 {
                let locus = failed.iter().position(|f| *f).expect("one failure");
                n.single_capped_locus_failures[locus] += 1;
            }
            match (capped > 0, mean) {
                (false, false) => {
                    n.compatible += 1;
                    seen[4] = true;
                }
                (true, false) => n.capped_locus_rejected += 1,
                (false, true) => n.mean_only_rejected += 1,
                (true, true) => n.capped_and_mean_rejected += 1,
            }
        }
        let s = c.searches.entry(key).or_default();
        s.searches += 1;
        s.with_neighbour += u64::from(seen[0]);
        s.with_predator_neighbour += u64::from(seen[1]);
        s.with_eligible += u64::from(seen[2]);
        s.with_eligible_predator += u64::from(seen[3]);
        s.with_compatible += u64::from(seen[4]);
        c.pending = Some((a.id.0, key));
    });
}
pub fn successful_pair(initiator: &Organism) {
    ACTIVE.with(|a| {
        let mut active = a.borrow_mut();
        let Some(c) = active.as_mut() else { return };
        if let Some((id, key)) = c.pending.take() {
            assert_eq!(id, initiator.id.0, "pair follows its own search");
            c.searches
                .get_mut(&key)
                .expect("recorded search")
                .successful_pairs += 1;
        }
    });
}
pub fn finish() -> serde_json::Value {
    let c = ACTIVE
        .with(|a| a.borrow_mut().take())
        .expect("mating decomposition started");
    serde_json::json!({"version":1,"window_ticks":1000,
        "readiness":c.readiness.iter().map(|((w,p),v)|serde_json::json!({"window":w,"predator":p,"totals":v})).collect::<Vec<_>>(),
        "searches":c.searches.iter().map(|((w,p),v)|serde_json::json!({"window":w,"initiator_predator":p,"totals":v})).collect::<Vec<_>>(),
        "neighbours":c.neighbours.iter().map(|((w,p,q,l),v)|serde_json::json!({"window":w,"initiator_predator":p,"neighbour_predator":q,"same_lineage":l,"totals":v})).collect::<Vec<_>>(),
        "definitions":"Readiness: start-of-tick authoritative age/cooldown/energy clauses per organism-tick (clauses overlap; energy-only means age and cooldown pass). Searches: every eligible, unpaired, living initiator in matching order, with neighbours exactly as matching sees them (unpaired, living, within16). Eligible neighbours are split by the per-locus400 cap on loci0..13 and the mean-distance threshold (total>=221*22); single-locus failures are the only failed capped locus. Predator = carnivory>650 and non-Grazer mouth, same as other R4 observers. Descriptive counts with repeated observations, not unique organisms or causal estimates."})
}

#[cfg(test)]
mod tests {
    use crate::{Command, Config, World};
    fn sum(rows: &serde_json::Value, key: &str) -> u64 {
        rows.as_array()
            .unwrap()
            .iter()
            .map(|r| r["totals"][key].as_u64().unwrap())
            .sum()
    }
    #[test]
    fn clause_decomposition_closes_against_existing_ledgers_without_hash_effect() {
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
        let d = super::finish();
        let v = crate::viability::finish(&world);
        assert_eq!(world.hash(), plain.hash());
        let cohorts = &v["cohorts"].as_array().unwrap()[..4];
        let total = |key: &str| {
            cohorts
                .iter()
                .map(|c| c["totals"][key].as_u64().unwrap())
                .sum::<u64>()
        };
        assert_eq!(
            sum(&d["readiness"], "organism_ticks"),
            total("organism_ticks")
        );
        assert_eq!(
            sum(&d["readiness"], "ready"),
            total("reproduction_ready_ticks")
        );
        assert_eq!(
            sum(&d["searches"], "searches"),
            total("initiating_mate_searches")
        );
        assert_eq!(
            sum(&d["neighbours"], "unpaired"),
            total("nearby_unpaired_candidates")
        );
        assert_eq!(
            sum(&d["neighbours"], "eligible"),
            total("nearby_eligible_candidates")
        );
        assert_eq!(
            sum(&d["neighbours"], "compatible"),
            total("nearby_compatible_candidates")
        );
        let temporal = v["ecology"]["r4_temporal"]["mate_search_density"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["successful_pairs"].as_u64().unwrap())
            .sum::<u64>();
        assert!(temporal > 0);
        assert_eq!(sum(&d["searches"], "successful_pairs"), temporal);
        for row in d["neighbours"].as_array().unwrap() {
            let t = &row["totals"];
            let get = |k: &str| t[k].as_u64().unwrap();
            assert_eq!(
                get("eligible"),
                get("compatible")
                    + get("capped_locus_rejected")
                    + get("mean_only_rejected")
                    + get("capped_and_mean_rejected")
            );
            assert!(get("unpaired") >= get("eligible") + get("energy_only_blocked"));
            let single = t["single_capped_locus_failures"].as_array().unwrap();
            let any = t["capped_locus_failures"].as_array().unwrap();
            assert!(single
                .iter()
                .zip(any)
                .all(|(s, a)| s.as_u64() <= a.as_u64()));
        }
        for row in d["searches"].as_array().unwrap() {
            let t = &row["totals"];
            let get = |k: &str| t[k].as_u64().unwrap();
            assert!(get("searches") >= get("with_neighbour"));
            assert!(get("with_neighbour") >= get("with_eligible"));
            assert!(get("with_eligible") >= get("with_compatible"));
            assert!(get("with_compatible") >= get("successful_pairs"));
            assert!(get("with_predator_neighbour") >= get("with_eligible_predator"));
        }
    }
    #[test]
    fn inactive_collector_records_nothing_and_replay_matches() {
        let mut world = World::new(Config::default()).unwrap();
        world.advance(300);
        let replay = crate::replay::Replay::from_world(&world).verify().unwrap();
        assert_eq!(replay.hash(), world.hash());
        super::start();
        let empty = super::finish();
        assert!(empty["searches"].as_array().unwrap().is_empty());
        assert!(empty["readiness"].as_array().unwrap().is_empty());
    }
}
