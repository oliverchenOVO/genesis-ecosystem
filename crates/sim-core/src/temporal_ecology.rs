//! Optional R4 measurements of unchanged R3 authority. No RNG or State fields.
use crate::{
    spatial::{distance_squared, Spatial, CELL_SIZE},
    viability::Cohort,
    Organism, World,
};
use serde::Serialize;
use std::{cell::RefCell, collections::BTreeMap};

#[derive(Default, Serialize)]
struct Sample {
    energy: [u64; 3],
    population: u64,
    energy_stock: u64,
    complexity: u64,
    mass: u64,
    bite_capacity: u64,
    carnivory: u64,
    mouth: [u64; 3],
    x: u64,
    y: u64,
    x2: u64,
    y2: u64,
    habitat: [u64; 8],
    cells: BTreeMap<usize, u64>,
    local_stock: [u64; 2],
    processing_efficiency: [u64; 2],
    visible_prey: u64,
    nearby_mates: u64,
    compatible_mates: u64,
    sampled_displacement: BTreeMap<u32, u64>,
}
#[derive(Default, Serialize)]
struct Local {
    samples: u64,
    neighbours: u64,
    eligible_mates: u64,
    compatible_mates: u64,
    visible_prey: u64,
    attack_range_prey: u64,
    with_prey: u64,
    with_compatible_mate: u64,
    with_both: u64,
    ready_samples: u64,
    ready_eligible_mates: u64,
    ready_compatible_mates: u64,
}
#[derive(Default)]
struct Collector {
    tick: u64,
    samples: BTreeMap<u64, BTreeMap<u64, Sample>>,
    budgets: BTreeMap<u64, BTreeMap<u64, Cohort>>,
    predators: BTreeMap<u64, BTreeMap<u64, Cohort>>,
    hunting: BTreeMap<u64, BTreeMap<u64, [u64; 2]>>,
    local: BTreeMap<(u64, u64, usize, usize, bool), Local>,
    fields: Vec<serde_json::Value>,
    previous_positions: BTreeMap<u64, (i32, i32)>,
    individuals: BTreeMap<u64, Individual>,
    individual_fitness: Vec<serde_json::Value>,
    mate_density: BTreeMap<(u64, u64, usize, bool, usize), [u64; 5]>,
    recent_searches: BTreeMap<u64, (u64, u64, usize, bool, usize)>,
}
#[derive(Default)]
struct Individual {
    lineage: u64,
    energy: [u64; 3],
    budget: Cohort,
    samples: u64,
    efficiency: [u64; 2],
    stock: [u64; 2],
    habitat: [u64; 8],
}
#[derive(Default, Serialize)]
struct Fitness {
    individuals: u64,
    energy: [u64; 3],
    organism_ticks: u64,
    offspring: u64,
    successful_matings: u64,
    mating_attempts: u64,
    deaths: u64,
    starvation_deaths: u64,
    predation_deaths: u64,
    completed_lifespan_sum: u64,
    completed_offspring_sum: u64,
    actual_metabolic_energy: u64,
    attack_energy: u64,
    reproductive_energy: u64,
    samples: u64,
    efficiency: [u64; 2],
    stock: [u64; 2],
    habitat: [u64; 8],
}
fn role(e: [u64; 3]) -> &'static str {
    let total = e.iter().sum::<u64>();
    if total == 0 {
        "NoIncome"
    } else if e[0] * 10 >= total * 7 {
        "SoftSpecialist"
    } else if e[1] * 10 >= total * 7 {
        "HardSpecialist"
    } else if e[2] * 2 >= total {
        "PreySpecialist"
    } else {
        "Mixed"
    }
}
fn flush_individuals(c: &mut Collector) {
    let mut groups: BTreeMap<(u64, &str), Fitness> = BTreeMap::new();
    for i in std::mem::take(&mut c.individuals).into_values() {
        let b = i.budget;
        if b.organism_ticks == 0 && b.births == 0 && b.deaths == 0 {
            continue;
        }
        let g = groups.entry((i.lineage, role(i.energy))).or_default();
        g.individuals += 1;
        for (a, v) in g.energy.iter_mut().zip(i.energy) {
            *a += v;
        }
        g.organism_ticks += b.organism_ticks;
        g.offspring += b.offspring_produced;
        g.successful_matings += b.successful_matings;
        g.mating_attempts += b.mating_attempts;
        g.deaths += b.deaths;
        g.starvation_deaths += b.starvation_deaths;
        g.predation_deaths += b.predation_deaths;
        g.completed_lifespan_sum += b.completed_lifespan_sum;
        g.completed_offspring_sum += b.completed_offspring_sum;
        g.actual_metabolic_energy += b.actual_metabolic_energy;
        g.attack_energy += b.attack_energy;
        g.reproductive_energy += b.reproductive_energy;
        g.samples += i.samples;
        for (a, v) in g.efficiency.iter_mut().zip(i.efficiency) {
            *a += v;
        }
        for (a, v) in g.stock.iter_mut().zip(i.stock) {
            *a += v;
        }
        for (a, v) in g.habitat.iter_mut().zip(i.habitat) {
            *a += v;
        }
    }
    c.individual_fitness.extend(groups.into_iter().map(|((lineage,role),totals)|serde_json::json!({"window":c.tick.saturating_sub(1)/1000,"lineage":lineage,"role":role,"totals":totals})));
}
thread_local! { static ACTIVE: RefCell<Option<Collector>> = const { RefCell::new(None) }; }
pub fn start(world: &World) {
    ACTIVE.with(|a| *a.borrow_mut() = Some(Collector::default()));
    sample(world);
}
pub fn tick(tick: u64) {
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            c.recent_searches.clear();
            c.tick = tick;
        }
    });
}
fn density(count: u64) -> usize {
    match count {
        0 => 0,
        1..=2 => 1,
        3..=7 => 2,
        _ => 3,
    }
}
pub fn mate_search(o: &Organism, candidates: [u64; 9]) {
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            let mouth = match o.phenotype.morphology.mouth {
                crate::morphology::Mouth::Grazer => 0,
                crate::morphology::Mouth::Crusher => 1,
                crate::morphology::Mouth::Piercer => 2,
            };
            let key = (
                c.tick.saturating_sub(1) / 1000,
                o.lineage_id.0,
                mouth,
                predator(o),
                density(candidates[0]),
            );
            let row = c.mate_density.entry(key).or_default();
            row[0] += 1;
            row[1] += candidates[0];
            row[2] += candidates[1];
            row[3] += candidates[2];
            c.recent_searches.insert(o.id.0, key);
        }
    });
}
pub fn successful_pair(initiator: &Organism) {
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            if let Some(key) = c.recent_searches.remove(&initiator.id.0) {
                c.mate_density.get_mut(&key).expect("recorded search")[4] += 1;
            }
        }
    });
}
pub fn budget(o: &Organism, f: &mut impl FnMut(&mut Cohort)) {
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            let window = c.tick.saturating_sub(1) / 1000;
            let individual = c.individuals.entry(o.id.0).or_default();
            individual.lineage = o.lineage_id.0;
            f(&mut individual.budget);
            let row = c
                .budgets
                .entry(window)
                .or_default()
                .entry(o.lineage_id.0)
                .or_default();
            let before = [row.requested_movement_energy, row.actual_metabolic_energy];
            f(row);
            if o.behavior == crate::Behavior::Hunt {
                let h = c
                    .hunting
                    .entry(window)
                    .or_default()
                    .entry(o.lineage_id.0)
                    .or_default();
                h[0] += row.requested_movement_energy - before[0];
                h[1] += row.actual_metabolic_energy - before[1];
            }
            if predator(o) {
                f(c.predators
                    .entry(window)
                    .or_default()
                    .entry(o.lineage_id.0)
                    .or_default());
            }
        }
    });
}
fn predator(o: &Organism) -> bool {
    o.phenotype.carnivory > 650 && o.phenotype.morphology.mouth != crate::morphology::Mouth::Grazer
}
pub fn feeding(o: &Organism, channel: usize, energy: i32) {
    if energy <= 0 {
        return;
    }
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            let individual = c.individuals.entry(o.id.0).or_default();
            individual.lineage = o.lineage_id.0;
            individual.energy[channel] += energy as u64;
            c.samples
                .entry((c.tick.saturating_sub(1) / 100 + 1) * 100)
                .or_default()
                .entry(o.lineage_id.0)
                .or_default()
                .energy[channel] += energy as u64;
        }
    });
}
fn eligible(o: &Organism, tick: u64) -> bool {
    crate::world::eligible(o, tick)
}
pub fn sample(world: &World) {
    ACTIVE.with(|a| {
        let mut active = a.borrow_mut();
        let Some(c) = active.as_mut() else { return; };
        let tick = world.state.tick;
        let side = (world.state.config.size / CELL_SIZE) as usize;
        let spatial = Spatial::new(world.state.config.size, &world.state.organisms);
        let rows = c.samples.entry(tick).or_default();
        let mut positions = BTreeMap::new();
        for (i, o) in world.state.organisms.iter().enumerate() {
            let index = (o.y / CELL_SIZE) as usize * side + (o.x / CELL_SIZE) as usize;
            let cell = &world.state.environment.cells[index];
            let p = &o.phenotype;
            let m = &p.morphology;
            let mouth = match m.mouth { crate::morphology::Mouth::Grazer => 0, crate::morphology::Mouth::Crusher => 1, crate::morphology::Mouth::Piercer => 2 };
            let r = rows.entry(o.lineage_id.0).or_default();
            let individual = c.individuals.entry(o.id.0).or_default();
            individual.lineage = o.lineage_id.0;
            individual.samples += 1;
            individual.stock[0] += cell.food as u64;
            individual.stock[1] += cell.hard_food as u64;
            individual.habitat[usize::from(cell.elevation >= 500)*4 + usize::from(cell.moisture >= 500)*2 + usize::from(cell.fertility >= 100)] += 1;
            r.population += 1;
            r.energy_stock += o.energy as u64;
            r.complexity += m.complexity as u64;
            r.mass += m.mass as u64;
            r.bite_capacity += m.bite_capacity as u64;
            r.carnivory += p.carnivory as u64;
            r.mouth[mouth] += 1;
            r.x += o.x as u64; r.y += o.y as u64;
            r.x2 += (o.x as u64).pow(2); r.y2 += (o.y as u64).pow(2);
            r.habitat[usize::from(cell.elevation >= 500) * 4 + usize::from(cell.moisture >= 500) * 2 + usize::from(cell.fertility >= 100)] += 1;
            *r.cells.entry(index).or_default() += 1;
            r.local_stock[0] += cell.food as u64;
            r.local_stock[1] += cell.hard_food as u64;
            for channel in 0..2 {
                let efficiency = cell.feeding_efficiency(o, channel) as u64;
                r.processing_efficiency[channel] += efficiency;
                individual.efficiency[channel] += efficiency;
            }
            if let Some(&(x, y)) = c.previous_positions.get(&o.id.0) {
                let d = (distance_squared(x, y, o.x, o.y) as u64).isqrt() as u32;
                *r.sampled_displacement.entry(d).or_default() += 1;
            }
            positions.insert(o.id.0, (o.x, o.y));
            let mut neighbours = 0;
            let mut mates = 0;
            let mut compatible = 0;
            let mut prey = 0;
            let mut close_prey = 0;
            for j in spatial.nearby(o.x, o.y, p.vision.max(16)) {
                if i == j { continue; }
                let other = &world.state.organisms[j];
                let d = distance_squared(o.x, o.y, other.x, other.y);
                if d <= 16 * 16 {
                    neighbours += 1;
                    if eligible(other, tick) {
                        mates += 1;
                        compatible += u64::from(o.genome.compatible(&other.genome));
                    }
                }
                if predator(o) && other.phenotype.carnivory <= 650 && m.can_attack(&other.phenotype.morphology) {
                    prey += u64::from(d <= p.vision * p.vision);
                    close_prey += u64::from(d <= 8 * 8);
                }
            }
            // Equal local neighbour-count strata, shared by all mouths and worlds.
            r.visible_prey += prey;
            r.nearby_mates += mates;
            r.compatible_mates += compatible;
            let density = match neighbours { 0 => 0, 1..=2 => 1, 3..=7 => 2, _ => 3 };
            let l = c.local.entry((tick.saturating_sub(1) / 1000, o.lineage_id.0, mouth, density, predator(o))).or_default();
            l.samples += 1; l.neighbours += neighbours; l.eligible_mates += mates;
            l.compatible_mates += compatible; l.visible_prey += prey;
            l.attack_range_prey += close_prey; l.with_prey += u64::from(prey > 0);
            l.with_compatible_mate += u64::from(compatible > 0);
            l.with_both += u64::from(prey > 0 && compatible > 0);
            if eligible(o, tick) {
                l.ready_samples += 1;
                l.ready_eligible_mates += mates;
                l.ready_compatible_mates += compatible;
            }
        }
        c.previous_positions = positions;
        if tick > 0 && tick.is_multiple_of(1000) { flush_individuals(c); }
        if tick.is_multiple_of(1000) {
            c.fields.push(serde_json::json!({"tick":tick,"side":side,"cell_size":CELL_SIZE,
                "soft":world.state.environment.cells.iter().map(|c| c.food).collect::<Vec<_>>(),
                "hard":world.state.environment.cells.iter().map(|c| c.hard_food).collect::<Vec<_>>(),
                "soft_productivity":world.state.environment.cells.iter().map(|c| i64::from(c.fertility)*i64::from(200+c.moisture)*2).collect::<Vec<_>>(),
                "hard_productivity":world.state.environment.cells.iter().map(|c| i64::from(c.fertility)*i64::from(1200-c.moisture)*i64::from(c.elevation)).collect::<Vec<_>>() }));
        }
    });
}
pub fn finish() -> serde_json::Value {
    let mut c = ACTIVE
        .with(|a| a.borrow_mut().take())
        .expect("temporal observer started");
    flush_individuals(&mut c);
    serde_json::json!({"version":1,"sample_interval":100,"budget_interval":1000,
        "samples":c.samples,"lineage_budgets_1000":c.budgets,"predator_lineage_budgets_1000":c.predators,
        "hunting_requested_movement_and_actual_total_metabolism_1000":c.hunting,
        "local_density_samples":c.local.iter().map(|((window,lineage,mouth,density,predator),v)| serde_json::json!({"window":window,"lineage":lineage,"mouth":mouth,"density":density,"predator":predator,"totals":v})).collect::<Vec<_>>(),
        "resource_snapshots":c.fields,
        "individual_fitness_1000":c.individual_fitness,
        "mate_search_density":c.mate_density.iter().map(|((window,lineage,mouth,predator,density),v)|serde_json::json!({"window":window,"lineage":lineage,"mouth":mouth,"predator":predator,"density":density,"searches":v[0],"unpaired_candidates":v[1],"eligible_candidates":v[2],"compatible_candidates":v[3],"successful_pairs":v[4]})).collect::<Vec<_>>(),
        "definitions":"R3 authority unchanged. Credited diet income over disjoint100 ticks; endpoints sampled every100 ticks including zero. Budgets disjoint1000 ticks, founders in first and censored survivors in last; births by child, offspring/matings by parent. Local eligibility uses authoritative age/cooldown/energy at end-of-tick; no paired exclusions, snapshot opportunities not matching outcomes. Density strata count other residents within16:0/1..2/3..7/8+. Visible prey uses current vision and authoritative can_attack, close prey within8 (attack distance). End-to-end displacement over100 ticks is a lower bound on path length. Resource stocks and uncapped productivity numerators every1000 ticks. Integer arrays permit deterministic analysis outside simulation. No juvenile/adult state exists; completed lifespan is not adult lifespan."})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn temporal_lineage_energy_and_population_close_at_each_boundary() {
        let mut w = World::new(crate::Config {
            population_limit: 200,
            ..Default::default()
        })
        .unwrap();
        let mut plain = w.clone();
        plain.advance(3000);
        crate::viability::start(&w);
        w.advance(3000);
        let report = crate::viability::finish(&w);
        assert_eq!(w.hash(), plain.hash());
        let t = &report["ecology"]["r4_temporal"];
        assert_eq!(t["samples"].as_object().unwrap().len(), 31);
        assert_eq!(t["resource_snapshots"].as_array().unwrap().len(), 4);
        let fitness = t["individual_fitness_1000"].as_array().unwrap();
        let get = |row: &serde_json::Value, key: &str| row[key].as_u64().unwrap_or(0) as i128;
        for window in 0..3 {
            let start = (window * 1000).to_string();
            let end = ((window + 1) * 1000).to_string();
            for (id, b) in t["lineage_budgets_1000"][window.to_string()]
                .as_object()
                .unwrap()
            {
                let groups = fitness
                    .iter()
                    .filter(|r| {
                        r["window"] == window && r["lineage"].as_u64().unwrap().to_string() == *id
                    })
                    .collect::<Vec<_>>();
                for (source, target) in [
                    ("organism_ticks", "organism_ticks"),
                    ("offspring_produced", "offspring"),
                    ("successful_matings", "successful_matings"),
                    ("deaths", "deaths"),
                ] {
                    assert_eq!(
                        groups
                            .iter()
                            .map(|r| r["totals"][target].as_u64().unwrap())
                            .sum::<u64>() as i128,
                        get(b, source)
                    );
                }
                let a = &t["samples"][&start][id];
                let z = &t["samples"][&end][id];
                assert_eq!(
                    get(a, "population") + get(b, "births"),
                    get(b, "deaths") + get(z, "population")
                );
                assert_eq!(
                    get(a, "energy_stock")
                        + get(b, "offspring_initial_energy")
                        + get(b, "food_energy")
                        + get(b, "prey_energy"),
                    get(b, "actual_metabolic_energy")
                        + get(b, "attack_energy")
                        + get(b, "reproductive_energy")
                        + get(b, "removed_death_energy")
                        + get(z, "energy_stock")
                );
                let mut energy = [0_u64; 3];
                for tick in (window * 1000 + 100..=(window + 1) * 1000).step_by(100) {
                    if let Some(row) = t["samples"][tick.to_string()].get(id) {
                        for (i, v) in energy.iter_mut().enumerate() {
                            *v += row["energy"][i].as_u64().unwrap();
                        }
                    }
                }
                assert_eq!(i128::from(energy[0] + energy[1]), get(b, "food_energy"));
                assert_eq!(i128::from(energy[2]), get(b, "prey_energy"));
            }
        }
    }
    #[test]
    fn sampled_mate_opportunities_match_brute_force_density_oracle() {
        let w = World::new(crate::Config {
            starting_population: 100,
            ..Default::default()
        })
        .unwrap();
        start(&w);
        let t = finish();
        let mut expected = [0_u64; 3];
        for a in &w.state.organisms {
            for b in &w.state.organisms {
                if a.id == b.id || distance_squared(a.x, a.y, b.x, b.y) > 256 {
                    continue;
                }
                expected[0] += 1;
                if eligible(b, 0) {
                    expected[1] += 1;
                    expected[2] += u64::from(a.genome.compatible(&b.genome));
                }
            }
        }
        let rows = t["local_density_samples"].as_array().unwrap();
        for (i, k) in ["neighbours", "eligible_mates", "compatible_mates"]
            .iter()
            .enumerate()
        {
            assert_eq!(
                rows.iter()
                    .map(|r| r["totals"][k].as_u64().unwrap())
                    .sum::<u64>(),
                expected[i]
            );
        }
    }
}
