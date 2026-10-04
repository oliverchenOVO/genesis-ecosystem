//! Headless observations only. No authority, RNG, save fields or per-event archive.
use crate::{morphology::Mouth, Cell, Organism, World};
use serde::Serialize;
use std::{cell::RefCell, collections::BTreeMap};

#[derive(Default, Serialize)]
struct Exposure {
    samples: u64,
    spatial: Vec<u64>,
    habitat: [u64; 8],
    x: u64,
    y: u64,
    x2: u64,
    y2: u64,
    temperature: i64,
    productivity: u64,
    elevation: u64,
    moisture: u64,
    displacement: BTreeMap<u32, u64>,
    centroid_distance: BTreeMap<u32, u64>,
    habitat_switches: u64,
    parent_habitat_distance: u64,
    current_streak: u64,
    longest_streak: u64,
}
#[derive(Default, Serialize)]
struct Feeding {
    food_events: u64,
    prey_events: u64,
    food_energy: u64,
    prey_energy: u64,
    units: u64,
    hardness: u64,
    productivity: u64,
    x: u64,
    y: u64,
    habitat: [u64; 8],
    hardness_bins: [u64; 10],
    productivity_bins: [u64; 10],
    prey_mass_bins: [u64; 10],
    channels: [u64; 3],
}
#[derive(Default, Serialize)]
struct Mating {
    pairs: u64,
    cross_lineage: u64,
    cross_species: u64,
    cross_habitat: u64,
    cross_mouth: u64,
    genetic_distance_sum: u64,
    distance: BTreeMap<u32, u64>,
    lineage_edges: BTreeMap<(u64, u64), u64>,
    child_edges: BTreeMap<(u64, u64), u64>,
}
struct Origin {
    x: i32,
    y: i32,
    habitat: usize,
    previous_habitat: usize,
}
#[derive(Default)]
struct Collector {
    requested_hunting_movement_by_mouth: [u64; 3],
    requested_hunting_movement_by_lineage: BTreeMap<u64, u64>,
    tick: u64,
    windows: BTreeMap<u64, Window>,
    mouth_budgets: BTreeMap<usize, crate::viability::Cohort>,
    lineage_budgets: BTreeMap<u64, crate::viability::Cohort>,
    origins: BTreeMap<u64, Origin>,
    lineages: BTreeMap<u64, Exposure>,
    species: BTreeMap<u64, Exposure>,
    feeding: BTreeMap<(u64, u64, usize), Feeding>,
    mating: Mating,
    samples: u64,
}
#[derive(Default, Serialize)]
struct Window {
    mating_pairs: u64,
    cross_lineage: u64,
    cross_habitat: u64,
    mating_distance: BTreeMap<u32, u64>,
    gene_flow: BTreeMap<u64, BTreeMap<u64, u64>>,
    feeding_energy_by_mouth: [[u64; 3]; 3],
    lineage_occupancy: BTreeMap<u64, WindowOccupancy>,
}
#[derive(Default, Serialize)]
struct WindowOccupancy {
    samples: u64,
    x: u64,
    y: u64,
    habitat: [u64; 8],
}
thread_local! { static ACTIVE: RefCell<Option<Collector>> = const { RefCell::new(None) }; }
pub fn tick(tick: u64) {
    crate::temporal_ecology::tick(tick);
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            c.tick = tick;
        }
    });
}
pub fn budget(o: &Organism, f: &mut impl FnMut(&mut crate::viability::Cohort)) {
    crate::temporal_ecology::budget(o, f);
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            let budget = c.mouth_budgets.entry(mouth(o)).or_default();
            let before = budget.requested_movement_energy;
            f(budget);
            if o.behavior == crate::Behavior::Hunt {
                let requested = budget.requested_movement_energy - before;
                c.requested_hunting_movement_by_mouth[mouth(o)] += requested;
                if requested > 0 {
                    *c.requested_hunting_movement_by_lineage
                        .entry(o.lineage_id.0)
                        .or_default() += requested;
                }
            }
            f(c.lineage_budgets.entry(o.lineage_id.0).or_default());
        }
    });
}
fn mouth(o: &Organism) -> usize {
    match o.phenotype.morphology.mouth {
        Mouth::Grazer => 0,
        Mouth::Crusher => 1,
        Mouth::Piercer => 2,
    }
}
fn habitat(c: &Cell) -> usize {
    usize::from(c.elevation >= 500) * 4
        + usize::from(c.moisture >= 500) * 2
        + usize::from(c.fertility >= 100)
}
fn cell<'a>(world: &'a World, o: &Organism) -> &'a Cell {
    let side = world.state.config.size / crate::spatial::CELL_SIZE;
    &world.state.environment.cells
        [(o.y / crate::spatial::CELL_SIZE * side + o.x / crate::spatial::CELL_SIZE) as usize]
}
fn distance(ax: i32, ay: i32, bx: i32, by: i32) -> u32 {
    ((i64::from(ax - bx).pow(2) + i64::from(ay - by).pow(2)) as u64).isqrt() as u32
}
pub fn start(world: &World) {
    let mut c = Collector::default();
    for o in &world.state.organisms {
        let h = habitat(cell(world, o));
        c.origins.insert(
            o.id.0,
            Origin {
                x: o.x,
                y: o.y,
                habitat: h,
                previous_habitat: h,
            },
        );
    }
    ACTIVE.with(|a| *a.borrow_mut() = Some(c));
    crate::temporal_ecology::start(world);
    sample(world);
}
pub fn birth(o: &Organism, terrain: &Cell, a: &Organism, b: &Organism) {
    ACTIVE.with(|active| {
        if let Some(c) = active.borrow_mut().as_mut() {
            let h = habitat(terrain);
            c.origins.insert(
                o.id.0,
                Origin {
                    x: o.x,
                    y: o.y,
                    habitat: h,
                    previous_habitat: h,
                },
            );
            for parent in [a, b] {
                *c.mating
                    .child_edges
                    .entry((parent.lineage_id.0, o.lineage_id.0))
                    .or_default() += 1;
            }
        }
    });
}
pub fn death(o: &Organism) {
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            c.origins.remove(&o.id.0);
        }
    });
}
pub fn feeding(
    o: &Organism,
    terrain: &Cell,
    channel: usize,
    units: i32,
    energy: i32,
    prey_mass: i32,
    hardness: i32,
) {
    if energy <= 0 {
        return;
    }
    crate::temporal_ecology::feeding(o, channel, energy);
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            c.windows
                .entry((c.tick.saturating_sub(1)) / 5000)
                .or_default()
                .feeding_energy_by_mouth[mouth(o)][channel] += energy as u64;
            let f = c
                .feeding
                .entry((o.lineage_id.0, o.species_id.0, mouth(o)))
                .or_default();
            f.channels[channel] += energy as u64;
            f.habitat[habitat(terrain)] += 1;
            f.x += o.x as u64;
            f.y += o.y as u64;
            let p = terrain.fertility * (200 + terrain.moisture) / 1000;
            f.productivity += p as u64;
            f.productivity_bins[(p / 20).clamp(0, 9) as usize] += 1;
            if channel == 2 {
                f.prey_events += 1;
                f.prey_energy += energy as u64;
                f.prey_mass_bins[(prey_mass / 100).clamp(0, 9) as usize] += 1;
            } else {
                f.food_events += 1;
                f.food_energy += energy as u64;
                f.units += units as u64;
                f.hardness += hardness as u64;
                f.hardness_bins[(hardness / 100).clamp(0, 9) as usize] += 1;
            }
        }
    });
}
pub fn mating(a: &Organism, b: &Organism, ca: &Cell, cb: &Cell) {
    crate::temporal_ecology::successful_pair(a);
    ACTIVE.with(|active| {
        if let Some(c) = active.borrow_mut().as_mut() {
            let w = c
                .windows
                .entry((c.tick.saturating_sub(1)) / 5000)
                .or_default();
            w.mating_pairs += 1;
            w.cross_lineage += u64::from(a.lineage_id != b.lineage_id);
            w.cross_habitat += u64::from(habitat(ca) != habitat(cb));
            *w.mating_distance
                .entry(distance(a.x, a.y, b.x, b.y))
                .or_default() += 1;
            for (a, b) in [(a, b), (b, a)] {
                *w.gene_flow
                    .entry(a.lineage_id.0)
                    .or_default()
                    .entry(b.lineage_id.0)
                    .or_default() += 1;
            }
            let m = &mut c.mating;
            m.pairs += 1;
            m.cross_lineage += u64::from(a.lineage_id != b.lineage_id);
            m.cross_species += u64::from(a.species_id != b.species_id);
            m.cross_habitat += u64::from(habitat(ca) != habitat(cb));
            m.cross_mouth += u64::from(mouth(a) != mouth(b));
            m.genetic_distance_sum += u64::from(a.genome.distance(&b.genome));
            *m.distance.entry(distance(a.x, a.y, b.x, b.y)).or_default() += 1;
            for (a, b) in [(a, b), (b, a)] {
                *m.lineage_edges
                    .entry((a.lineage_id.0, b.lineage_id.0))
                    .or_default() += 1;
            }
        }
    });
}
pub fn sample(world: &World) {
    if world.state.tick > 0 {
        crate::temporal_ecology::sample(world);
    }
    ACTIVE.with(|a| {
        let mut active = a.borrow_mut();
        let Some(c) = active.as_mut() else {
            return;
        };
        c.samples += 1;
        let mut centers: BTreeMap<u64, (i64, i64, u64)> = BTreeMap::new();
        for o in &world.state.organisms {
            let v = centers.entry(o.lineage_id.0).or_default();
            v.0 += i64::from(o.x);
            v.1 += i64::from(o.y);
            v.2 += 1;
        }
        let mut populations: BTreeMap<u64, u64> = BTreeMap::new();
        let mut species_pop: BTreeMap<u64, u64> = BTreeMap::new();
        for o in &world.state.organisms {
            *populations.entry(o.lineage_id.0).or_default() += 1;
            *species_pop.entry(o.species_id.0).or_default() += 1;
            let terrain = cell(world, o);
            let h = habitat(terrain);
            let row = c
                .windows
                .entry(world.state.tick.saturating_sub(1) / 5000)
                .or_default()
                .lineage_occupancy
                .entry(o.lineage_id.0)
                .or_default();
            row.samples += 1;
            row.x += o.x as u64;
            row.y += o.y as u64;
            row.habitat[h] += 1;
            let origin = c
                .origins
                .get_mut(&o.id.0)
                .expect("observed birth or founder");
            let moved = u64::from(origin.previous_habitat != h);
            origin.previous_habitat = h;
            let displacement = distance(o.x, o.y, origin.x, origin.y);
            let parent_mismatch = u64::from(origin.habitat != h);
            let center = centers[&o.lineage_id.0];
            let centroid = distance(
                o.x,
                o.y,
                (center.0 / center.2 as i64) as i32,
                (center.1 / center.2 as i64) as i32,
            );
            let region = (o.y * 8 / world.state.config.size * 8 + o.x * 8 / world.state.config.size)
                as usize;
            for e in [
                c.lineages.entry(o.lineage_id.0).or_default(),
                c.species.entry(o.species_id.0).or_default(),
            ] {
                e.spatial.resize(64, 0);
                e.samples += 1;
                e.spatial[region] += 1;
                e.habitat[h] += 1;
                e.x += o.x as u64;
                e.y += o.y as u64;
                e.x2 += (o.x as u64).pow(2);
                e.y2 += (o.y as u64).pow(2);
                e.temperature +=
                    i64::from(world.state.environment.temperature + terrain.temperature_offset);
                e.productivity += (terrain.fertility * (200 + terrain.moisture) / 1000) as u64;
                e.elevation += terrain.elevation as u64;
                e.moisture += terrain.moisture as u64;
                *e.displacement.entry(displacement).or_default() += 1;
                *e.centroid_distance.entry(centroid).or_default() += 1;
                e.habitat_switches += moved;
                e.parent_habitat_distance += parent_mismatch;
            }
        }
        for (groups, pop) in [
            (&mut c.lineages, populations),
            (&mut c.species, species_pop),
        ] {
            for (id, e) in groups {
                e.current_streak = if pop.get(id).copied().unwrap_or(0) >= 8 && world.state.tick > 0
                {
                    e.current_streak + 100
                } else {
                    0
                };
                e.longest_streak = e.longest_streak.max(e.current_streak);
            }
        }
    });
}
fn overlap(a: &[u64], b: &[u64]) -> Option<f64> {
    let na = a.iter().sum::<u64>();
    let nb = b.iter().sum::<u64>();
    (na > 0 && nb > 0).then(|| {
        a.iter()
            .zip(b)
            .map(|(a, b)| (*a as f64 / na as f64).min(*b as f64 / nb as f64))
            .sum()
    })
}
fn distances(h: &BTreeMap<u32, u64>) -> serde_json::Value {
    let n = h.values().sum::<u64>();
    if n == 0 {
        return serde_json::Value::Null;
    }
    let target = (n * 9).div_ceil(10);
    let mut cumulative = 0;
    let p90 = h
        .iter()
        .find_map(|(d, k)| {
            cumulative += k;
            (cumulative >= target.min(n)).then_some(*d)
        })
        .unwrap();
    serde_json::json!({"samples":n,"mean":h.iter().map(|(d,k)| u64::from(*d)*k).sum::<u64>() as f64/n as f64,"p90":p90})
}
pub fn finish() -> serde_json::Value {
    let temporal = crate::temporal_ecology::finish();
    let c = ACTIVE
        .with(|a| a.borrow_mut().take())
        .expect("ecological observer started");
    let exposure = |groups: &BTreeMap<u64, Exposure>| {
        groups.iter().map(|(id,e)| {
        let n=e.samples.max(1) as f64; let x=e.x as f64/n; let y=e.y as f64/n;
        serde_json::json!({"id":id,"totals":e,"mean_position":[x,y],"spatial_variance":[e.x2 as f64/n-x*x,e.y2 as f64/n-y*y],"dispersal":distances(&e.displacement),"centroid_distance":distances(&e.centroid_distance)})
    }).collect::<Vec<_>>()
    };
    let mut major = c
        .lineages
        .iter()
        .filter(|(_, e)| e.longest_streak >= 1000)
        .collect::<Vec<_>>();
    major.sort_by_key(|(id, e)| (std::cmp::Reverse(e.samples), **id));
    major.truncate(16);
    major.sort_by_key(|(id, _)| **id);
    let mut overlaps = Vec::new();
    let feeding_profile = |id: u64| {
        let mut bins = [[0_u64; 10]; 3];
        let mut channels = [0_u64; 3];
        for ((lineage, _, _), f) in &c.feeding {
            if *lineage == id {
                for (target, source) in
                    bins.iter_mut()
                        .zip([f.hardness_bins, f.productivity_bins, f.prey_mass_bins])
                {
                    for (a, b) in target.iter_mut().zip(source) {
                        *a += b;
                    }
                }
                for (a, b) in channels.iter_mut().zip(f.channels) {
                    *a += b;
                }
            }
        }
        (bins, channels)
    };
    for (i, (a, ea)) in major.iter().enumerate() {
        for (b, eb) in major.iter().skip(i + 1) {
            let (fa, ca) = feeding_profile(**a);
            let (fb, cb) = feeding_profile(**b);
            overlaps.push(serde_json::json!({"a":a,"b":b,"spatial":overlap(&ea.spatial,&eb.spatial),"habitat":overlap(&ea.habitat,&eb.habitat),"feeding_hardness":overlap(&fa[0],&fb[0]),"feeding_productivity":overlap(&fa[1],&fb[1]),"prey_size":overlap(&fa[2],&fb[2]),"realized_energy_sources":overlap(&ca,&cb)}));
        }
    }
    let matrix=major.iter().map(|(id,_)| {
        let total=c.mating.lineage_edges.iter().filter(|((a,_),_)| a==*id).map(|(_,n)| n).sum::<u64>();
        serde_json::json!({"lineage":id,"parent_participations":total,"partners":major.iter().map(|(partner,_)| serde_json::json!({"lineage":partner,"count":c.mating.lineage_edges.get(&(**id,**partner)).copied().unwrap_or(0),"fraction":if total>0 {Some(c.mating.lineage_edges.get(&(**id,**partner)).copied().unwrap_or(0) as f64/total as f64)} else {None}})).collect::<Vec<_>>()})
    }).collect::<Vec<_>>();
    let edges = |m: &BTreeMap<(u64, u64), u64>| {
        m.iter()
            .map(|((a, b), n)| serde_json::json!({"a":a,"b":b,"count":n}))
            .collect::<Vec<_>>()
    };
    serde_json::json!({"version":4,"r4_temporal":temporal,"channel_names":["soft","hard","prey"],"requested_hunting_movement_by_mouth":c.requested_hunting_movement_by_mouth,"requested_hunting_movement_by_lineage":c.requested_hunting_movement_by_lineage,"windows_5000_ticks":c.windows,"samples":c.samples,"lineages":exposure(&c.lineages),"species":exposure(&c.species),"mouth_budgets":c.mouth_budgets,"lineage_budgets":c.lineage_budgets,"gene_flow_matrix":matrix,
        "feeding":c.feeding.iter().map(|((lineage,species,mouth),f)| serde_json::json!({"lineage":lineage,"species":species,"mouth":mouth,"totals":f})).collect::<Vec<_>>(),
        "mating":{"pairs":c.mating.pairs,"cross_lineage":c.mating.cross_lineage,"cross_species":c.mating.cross_species,"cross_habitat":c.mating.cross_habitat,"cross_mouth":c.mating.cross_mouth,"genetic_distance_sum":c.mating.genetic_distance_sum,"distance":distances(&c.mating.distance),"lineage_edges":edges(&c.mating.lineage_edges),"child_edges":edges(&c.mating.child_edges)},
        "major_persistent_lineages":major.iter().map(|(id,_)| **id).collect::<Vec<_>>(),"overlap":overlaps,
        "definitions":"100-tick occupancy samples plus tick zero; 8x8 spatial regions; habitat bits elevation>=500/moisture>=500/fertility>=100 (fertility range60..140). Productivity bins width20; hardness and prey mass bins width100, clipped to bin9. Physical resource processing hardness replaces the R2 elevation proxy: soft40..90, hard300..633. Persistence >=8 sampled residents for >=1000 ticks. Top16 persistent lineages by exposure then ID. Histogram intersection sum(min(normalized bins)); integer floor Euclidean distance in world units; nearest-rank p90. Origins retained only while alive; habitat switches sampled (lower bound); parent habitat means birth parent A cell. Feeding events require positive credited energy; channels 0=soft, 1=hard, 2=prey. Matings counted once per successful pair; symmetric parent edges total twice pairs; child edges twice births. Temporal windows ticks1..5000 etc (tick-zero occupancy in first). No event archive or world mutations."})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overlap_and_distance_quantiles_have_exact_boundaries() {
        assert_eq!(overlap(&[0, 0], &[1, 0]), None);
        assert_eq!(overlap(&[1, 0], &[0, 1]), Some(0.0));
        assert_eq!(overlap(&[1, 3], &[2, 6]), Some(1.0));
        assert_eq!(distance(0, 0, 3, 4), 5);
        assert_eq!(distances(&BTreeMap::from([(1, 9), (20, 1)]))["p90"], 1);
    }
    #[test]
    fn actual_event_ledgers_close_and_occupancy_preserves_authority() {
        let mut w = World::new(crate::Config {
            starting_population: 50,
            population_limit: 200,
            ..crate::Config::default()
        })
        .unwrap();
        let mut plain = w.clone();
        plain.advance(1200);
        crate::viability::start(&w);
        w.advance(1200);
        let report = crate::viability::finish(&w);
        let e = &report["ecology"];
        assert_eq!(w.hash(), plain.hash());
        assert_eq!(e["samples"], 13);
        let sum_edges = |key: &str| {
            e["mating"][key]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| r["count"].as_u64().unwrap())
                .sum::<u64>()
        };
        assert_eq!(
            sum_edges("lineage_edges"),
            e["mating"]["pairs"].as_u64().unwrap() * 2
        );
        assert_eq!(sum_edges("child_edges"), w.state.counters.births * 2);
        let food = e["feeding"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["totals"]["food_energy"].as_u64().unwrap())
            .sum::<u64>();
        let cohorts = report["cohorts"].as_array().unwrap();
        assert_eq!(
            food,
            cohorts[..4]
                .iter()
                .map(|r| r["totals"]["food_energy"].as_u64().unwrap())
                .sum::<u64>()
        );
        for row in e["lineages"].as_array().unwrap() {
            assert_eq!(
                row["totals"]["samples"].as_u64().unwrap(),
                row["totals"]["spatial"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_u64().unwrap())
                    .sum::<u64>()
            );
        }
        for row in e["mouth_budgets"].as_object().unwrap().values() {
            assert_eq!(
                row["initial_population"].as_u64().unwrap() + row["births"].as_u64().unwrap(),
                row["deaths"].as_u64().unwrap() + row["final_population"].as_u64().unwrap()
            );
        }
    }
}
