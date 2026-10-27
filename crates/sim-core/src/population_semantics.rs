//! Optional R4 species-gate semantic review: attribute every birth to the parents' groups under
//! four grouping semantics built only from past information. Separate collector: no State,
//! RNG, save or frontend fields, and no species are created.
//!
//! A lineage (current authority) · B genetic population cluster · C genetic cluster × mouth ·
//! D lineage gene-flow community from the previous window.
use crate::{analysis::morphology_profile, genetics::LOCI, morphology::Mouth, Organism, World};
use serde::Serialize;
use std::{cell::RefCell, collections::BTreeMap};

pub const WINDOW: u64 = 1000;
/// Same genetic distance the species detector requires between a lineage and its ancestor.
pub const GENETIC_THRESHOLD: u32 = 100;
/// A lineage pair is linked when flow between them exceeds this share of either one's births,
/// i.e. when the pair would already fail the existing 20% isolation test against each other.
pub const LINK_PERMILLE: u64 = 200;

#[derive(Clone, Copy, Default, Serialize)]
struct Flow {
    within: u64,
    cross: u64,
}
#[derive(Clone, Default, Serialize)]
struct Group {
    members: u64,
    lineages: u64,
    centroid: Vec<u32>,
    /// Births with at least one parent in the group, during the following window.
    births: Flow,
}
#[derive(Default, Serialize)]
struct BirthClasses {
    births: u64,
    cross_lineage: u64,
    /// Cross-lineage births whose parents share mouth and diet role, morphology distance
    /// < 150 and genome distance < GENETIC_THRESHOLD.
    cross_lineage_same_niche: u64,
    /// Cross-lineage births whose parents differ in mouth or diet role.
    cross_lineage_cross_ecotype: u64,
    cross_ecotype: u64,
    cross_mouth: u64,
}
/// Snapshot tally of one group: members, member lineages, genome sums.
type Tally = (u64, std::collections::BTreeSet<u64>, [u64; LOCI]);
#[derive(Default)]
struct Snapshot {
    /// B: stable cluster id -> centroid genome.
    centroids: Vec<(u64, [u32; LOCI])>,
    /// D: lineage -> community id (smallest lineage id in the component).
    communities: BTreeMap<u64, u64>,
    /// D strict: as D, but a pair links only when flow exceeds 20% of both lineages' births.
    strict_communities: BTreeMap<u64, u64>,
}
#[derive(Default)]
struct Collector {
    tick: u64,
    next_cluster: u64,
    snapshot: Snapshot,
    diets: BTreeMap<u64, [u64; 3]>,
    /// Per window: semantic name -> group id -> group.
    windows: BTreeMap<u64, BTreeMap<&'static str, BTreeMap<u64, Group>>>,
    births: BTreeMap<u64, BirthClasses>,
    /// Lineage-pair birth flow of the current window, for the next window's communities and
    /// for the reported gene-flow graph.
    pair_flow: BTreeMap<(u64, u64), u64>,
    lineage_births: BTreeMap<u64, u64>,
    graphs: BTreeMap<u64, serde_json::Value>,
}
thread_local! { static ACTIVE: RefCell<Option<Collector>> = const { RefCell::new(None) }; }

fn genome(o: &Organism) -> [u32; LOCI] {
    o.genome.0.map(u32::from)
}
fn distance(a: &[u32; LOCI], b: &[u32; LOCI]) -> u32 {
    a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).sum::<u32>() / LOCI as u32
}
fn mouth(o: &Organism) -> u64 {
    match o.phenotype.morphology.mouth {
        Mouth::Grazer => 0,
        Mouth::Crusher => 1,
        Mouth::Piercer => 2,
    }
}
fn role(diet: Option<&[u64; 3]>) -> u64 {
    let Some(d) = diet else { return 0 };
    let t = d.iter().sum::<u64>();
    match () {
        _ if t == 0 => 0,
        _ if d[0] * 10 >= t * 7 => 1,
        _ if d[1] * 10 >= t * 7 => 2,
        _ if d[2] * 2 >= t => 3,
        _ => 4,
    }
}
fn profile(o: &Organism) -> [i32; 9] {
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
/// Nearest centroid by genome distance, ties to the lower id; None without centroids.
fn nearest(centroids: &[(u64, [u32; LOCI])], g: &[u32; LOCI]) -> Option<u64> {
    centroids
        .iter()
        .min_by_key(|(id, c)| (distance(c, g), *id))
        .map(|(id, _)| *id)
}
fn groups_of(c: &Collector, o: &Organism) -> [(&'static str, u64); 5] {
    let cluster = nearest(&c.snapshot.centroids, &genome(o)).unwrap_or(u64::MAX);
    let lineage = o.lineage_id.0;
    [
        ("A_lineage", lineage),
        ("B_genetic", cluster),
        ("C_genetic_mouth", cluster.wrapping_mul(4) + mouth(o)),
        (
            "D_community",
            *c.snapshot.communities.get(&lineage).unwrap_or(&lineage),
        ),
        (
            "D_strict_community",
            *c.snapshot
                .strict_communities
                .get(&lineage)
                .unwrap_or(&lineage),
        ),
    ]
}
/// Connected components of lineages linked by births; `strict` requires the pair flow to
/// exceed LINK_PERMILLE of both lineages' births instead of either. Ids are the smallest
/// lineage id in each component.
fn components(
    births: &BTreeMap<u64, u64>,
    flow: &BTreeMap<(u64, u64), u64>,
    strict: bool,
) -> BTreeMap<u64, u64> {
    let mut parent: BTreeMap<u64, u64> = births.keys().map(|l| (*l, *l)).collect();
    fn root(p: &mut BTreeMap<u64, u64>, x: u64) -> u64 {
        let mut r = x;
        while p[&r] != r {
            r = p[&r];
        }
        p.insert(x, r);
        r
    }
    for (&(x, y), &n) in flow {
        if x == y {
            continue;
        }
        let (ax, ay) = (
            n * 1000 > births[&x] * LINK_PERMILLE,
            n * 1000 > births[&y] * LINK_PERMILLE,
        );
        if if strict { ax && ay } else { ax || ay } {
            let (rx, ry) = (root(&mut parent, x), root(&mut parent, y));
            if rx != ry {
                parent.insert(rx.max(ry), rx.min(ry));
            }
        }
    }
    let lineages: Vec<u64> = parent.keys().copied().collect();
    lineages
        .iter()
        .map(|l| (*l, root(&mut parent, *l)))
        .collect()
}

pub fn start(world: &World) {
    ACTIVE.with(|a| *a.borrow_mut() = Some(Collector::default()));
    snapshot(world);
}
pub fn tick(tick: u64) {
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            c.tick = tick;
        }
    });
}
pub fn feeding(o: &Organism, channel: usize, energy: i32) {
    if energy <= 0 {
        return;
    }
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            c.diets.entry(o.id.0).or_default()[channel] += energy as u64;
        }
    });
}
pub fn death(o: &Organism) {
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            c.diets.remove(&o.id.0);
        }
    });
}
pub fn birth(a: &Organism, b: &Organism) {
    ACTIVE.with(|active| {
        let mut active = active.borrow_mut();
        let Some(c) = active.as_mut() else { return };
        let window = c.tick.saturating_sub(1) / WINDOW;
        let (ga, gb) = (groups_of(c, a), groups_of(c, b));
        let (la, lb) = (a.lineage_id.0, b.lineage_id.0);
        let (ra, rb) = (role(c.diets.get(&a.id.0)), role(c.diets.get(&b.id.0)));
        let cross_ecotype = mouth(a) != mouth(b) || ra != rb;
        let morph = profile(a)
            .iter()
            .zip(profile(b))
            .map(|(x, y)| (x - y).abs())
            .sum::<i32>()
            / 9;
        let similar =
            !cross_ecotype && morph < 150 && distance(&genome(a), &genome(b)) < GENETIC_THRESHOLD;
        let k = c.births.entry(window).or_default();
        k.births += 1;
        k.cross_ecotype += u64::from(cross_ecotype);
        k.cross_mouth += u64::from(mouth(a) != mouth(b));
        if la != lb {
            k.cross_lineage += 1;
            k.cross_lineage_same_niche += u64::from(similar);
            k.cross_lineage_cross_ecotype += u64::from(cross_ecotype);
        }
        *c.pair_flow.entry((la.min(lb), la.max(lb))).or_default() += 1;
        *c.lineage_births.entry(la).or_default() += 1;
        if lb != la {
            *c.lineage_births.entry(lb).or_default() += 1;
        }
        let w = c.windows.entry(window).or_default();
        for ((name, x), (_, y)) in ga.iter().zip(gb) {
            let table = w.entry(name).or_default();
            for id in if *x == y { vec![*x] } else { vec![*x, y] } {
                let f = &mut table.entry(id).or_default().births;
                if *x == y {
                    f.within += 1;
                } else {
                    f.cross += 1;
                }
            }
        }
    });
}
/// At each window boundary: close the lineage gene-flow graph, then rebuild B and D groups
/// from the living population and the just-finished window only.
pub fn sample(world: &World) {
    if !world.state.tick.is_multiple_of(WINDOW) {
        return;
    }
    snapshot(world);
}
fn snapshot(world: &World) {
    ACTIVE.with(|active| {
        let mut active = active.borrow_mut();
        let Some(c) = active.as_mut() else { return };
        let tick = world.state.tick;
        let finished = tick / WINDOW;
        // D: lineage gene-flow graph of the window that just ended.
        let communities = components(&c.lineage_births, &c.pair_flow, false);
        let strict_communities = components(&c.lineage_births, &c.pair_flow, true);
        let lineages: Vec<u64> = communities.keys().copied().collect();
        // Edges: [lineage, lineage, births between them, births of each lineage].
        let edges: Vec<_> = c
            .pair_flow
            .iter()
            .filter(|((x, y), _)| x != y)
            .map(|(&(x, y), &n)| serde_json::json!([x, y, n, c.lineage_births[&x], c.lineage_births[&y]]))
            .collect();
        if finished > 0 {
            let within: u64 = c
                .pair_flow
                .iter()
                .filter(|((x, y), _)| x == y)
                .map(|(_, n)| n)
                .sum();
            let total: u64 = c.pair_flow.values().sum();
            c.graphs.insert(
                finished - 1,
                serde_json::json!({"births":total,"within_lineage_births":within,
                    "lineages_with_births":lineages.len(),
                    "communities":communities.values().collect::<std::collections::BTreeSet<_>>().len(),
                    "strict_communities":strict_communities.values().collect::<std::collections::BTreeSet<_>>().len(),
                    "edges":edges}),
            );
        }
        c.pair_flow.clear();
        c.lineage_births.clear();
        // B: leader clustering seeded by previous centroids (stable ids), then recentred.
        let organisms = &world.state.organisms;
        let mut leaders = c.snapshot.centroids.clone();
        let mut assignment = Vec::with_capacity(organisms.len());
        for o in organisms {
            let g = genome(o);
            let id = match leaders
                .iter()
                .filter(|(_, l)| distance(l, &g) < GENETIC_THRESHOLD)
                .min_by_key(|(id, l)| (distance(l, &g), *id))
            {
                Some((id, _)) => *id,
                None => {
                    let id = c.next_cluster;
                    c.next_cluster += 1;
                    leaders.push((id, g));
                    id
                }
            };
            assignment.push(id);
        }
        let mut sums: BTreeMap<u64, (u64, [u64; LOCI])> = BTreeMap::new();
        for (o, id) in organisms.iter().zip(&assignment) {
            let e = sums.entry(*id).or_default();
            e.0 += 1;
            for (s, v) in e.1.iter_mut().zip(genome(o)) {
                *s += u64::from(v);
            }
        }
        c.snapshot.centroids = sums
            .iter()
            .map(|(id, (n, s))| (*id, s.map(|v| (v / n) as u32)))
            .collect();
        c.snapshot.communities = communities;
        c.snapshot.strict_communities = strict_communities;
        // Group records for the window that starts now.
        let starting = tick / WINDOW;
        let mut tables: BTreeMap<&'static str, BTreeMap<u64, Tally>> = BTreeMap::new();
        for o in organisms {
            for (name, id) in groups_of(c, o) {
                let e = tables.entry(name).or_default().entry(id).or_default();
                e.0 += 1;
                e.1.insert(o.lineage_id.0);
                for (s, v) in e.2.iter_mut().zip(genome(o)) {
                    *s += u64::from(v);
                }
            }
        }
        let w = c.windows.entry(starting).or_default();
        for (name, groups) in tables {
            let table = w.entry(name).or_default();
            for (id, (n, lineages, sum)) in groups {
                let g = table.entry(id).or_default();
                g.members = n;
                g.lineages = lineages.len() as u64;
                g.centroid = sum.iter().map(|v| (v / n) as u32).collect();
            }
        }
    });
}
pub fn finish() -> serde_json::Value {
    let c = ACTIVE
        .with(|a| a.borrow_mut().take())
        .expect("population semantics started");
    serde_json::json!({"version":2,"window_ticks":WINDOW,"genetic_threshold":GENETIC_THRESHOLD,"link_permille":LINK_PERMILLE,
        "windows":c.windows.iter().map(|(w,t)|serde_json::json!({"window":w,"groups":t})).collect::<Vec<_>>(),
        "births":c.births.iter().map(|(w,v)|serde_json::json!({"window":w,"totals":v})).collect::<Vec<_>>(),
        "lineage_flow_graphs":c.graphs.iter().map(|(w,v)|serde_json::json!({"window":w,"graph":v})).collect::<Vec<_>>(),
        "definitions":"Groups are rebuilt at each 1000-tick boundary from the living population and the just-finished window only; births in the next window are attributed to both parents' groups (parents' groups use the boundary snapshot: nearest B centroid, own mouth, previous-window community of own lineage, or own lineage when it has no previous births). B: leader clustering on Genome::distance < 100 seeded by previous centroids, then recentred. C: B cluster x mouth. D: connected components of lineages whose pairwise birth flow exceeded 20% of either lineage's births in the previous window; D strict requires 20% of both. Graph edges list [lineage, lineage, births between, births of each]. A group's births count within when both parents share it, cross otherwise. Same-niche cross-lineage births: same mouth and lifetime diet role, morphology profile distance < 150, genome distance < 100. No species are created; no simulation state or RNG is touched."})
}

#[cfg(test)]
mod tests {
    use crate::{Command, Config, World};
    #[test]
    fn birth_attribution_closes_and_leaves_hash_unchanged() {
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
        plain.advance(4000);
        let before = world.state.counters.births;
        crate::viability::start(&world);
        super::start(&world);
        world.advance(4000);
        let r = super::finish();
        crate::viability::finish(&world);
        assert_eq!(world.hash(), plain.hash());
        let births: u64 = r["births"]
            .as_array()
            .unwrap()
            .iter()
            .map(|w| w["totals"]["births"].as_u64().unwrap())
            .sum();
        assert_eq!(births, world.state.counters.births - before);
        for w in r["windows"].as_array().unwrap() {
            let wb = r["births"]
                .as_array()
                .unwrap()
                .iter()
                .find(|b| b["window"] == w["window"])
                .map(|b| b["totals"]["births"].as_u64().unwrap())
                .unwrap_or(0);
            for (name, groups) in w["groups"].as_object().unwrap() {
                let (mut within, mut cross) = (0, 0);
                for g in groups.as_object().unwrap().values() {
                    within += g["births"]["within"].as_u64().unwrap();
                    cross += g["births"]["cross"].as_u64().unwrap();
                }
                // Every birth is within one group or cross between exactly two.
                assert_eq!(within + cross / 2, wb, "{name}");
                assert_eq!(cross % 2, 0, "{name}");
            }
        }
        let k = r["births"].as_array().unwrap();
        let sum = |key: &str| -> u64 { k.iter().map(|w| w["totals"][key].as_u64().unwrap()).sum() };
        assert!(sum("cross_lineage_same_niche") <= sum("cross_lineage"));
        assert!(sum("cross_lineage_cross_ecotype") <= sum("cross_lineage"));
    }
}
