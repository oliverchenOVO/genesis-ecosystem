//! Optional R4 habitat association: does realized environmental feeding happen inside each
//! channel's static high-productivity habitat? Separate collector: no State, RNG, save or
//! frontend fields. Productivity masks use the same uncapped numerators as resource growth.
use crate::{morphology::Mouth, Cell, Organism, World};
use serde::Serialize;
use std::{cell::RefCell, collections::BTreeMap};

/// Static per-cell productivity numerators, matching `Cell::regenerate_channels`.
pub fn productivity(c: &Cell) -> [i64; 2] {
    let f = i64::from(c.fertility);
    [
        f * i64::from(200 + c.moisture) * 2,
        f * i64::from(1200 - c.moisture) * i64::from(c.elevation),
    ]
}

#[derive(Clone, Copy, Default, Serialize)]
struct Feeding {
    events: u64,
    energy: u64,
    /// Indexed by mask channel: inside the soft / hard top-quartile productivity mask.
    events_in_mask: [u64; 2],
    energy_in_mask: [u64; 2],
}
#[derive(Default)]
struct Collector {
    tick: u64,
    /// Cell index -> membership in the soft / hard top-quartile masks.
    masks: Vec<[bool; 2]>,
    mask_cells: [u64; 2],
    overlap_cells: u64,
    by_mouth: BTreeMap<(usize, usize), Feeding>,
    by_lineage: BTreeMap<(u64, u64, usize), Feeding>,
}
thread_local! { static ACTIVE: RefCell<Option<Collector>> = const { RefCell::new(None) }; }

fn mouth(o: &Organism) -> usize {
    match o.phenotype.morphology.mouth {
        Mouth::Grazer => 0,
        Mouth::Crusher => 1,
        Mouth::Piercer => 2,
    }
}

/// Masks are the cells at or above the 75th-percentile productivity of each channel.
pub fn start(world: &World) {
    let cells = &world.state.environment.cells;
    let values: Vec<[i64; 2]> = cells.iter().map(productivity).collect();
    let mut thresholds = [0i64; 2];
    for (channel, t) in thresholds.iter_mut().enumerate() {
        let mut sorted: Vec<i64> = values.iter().map(|v| v[channel]).collect();
        sorted.sort_unstable();
        *t = sorted[sorted.len() * 3 / 4];
    }
    let masks: Vec<[bool; 2]> = values
        .iter()
        .map(|v| [v[0] >= thresholds[0], v[1] >= thresholds[1]])
        .collect();
    let mask_cells = [0, 1].map(|ch| masks.iter().filter(|m| m[ch]).count() as u64);
    let overlap_cells = masks.iter().filter(|m| m[0] && m[1]).count() as u64;
    ACTIVE.with(|a| {
        *a.borrow_mut() = Some(Collector {
            masks,
            mask_cells,
            overlap_cells,
            ..Default::default()
        })
    });
}
pub fn tick(tick: u64) {
    ACTIVE.with(|a| {
        if let Some(c) = a.borrow_mut().as_mut() {
            c.tick = tick;
        }
    });
}
/// One successful environmental feeding event in `cell_index` on `channel` (0 soft, 1 hard).
pub fn feeding(o: &Organism, cell_index: usize, channel: usize, energy: i32) {
    if energy <= 0 || channel > 1 {
        return;
    }
    ACTIVE.with(|a| {
        let mut active = a.borrow_mut();
        let Some(c) = active.as_mut() else { return };
        let inside = c.masks[cell_index];
        let window = c.tick.saturating_sub(1) / 1000;
        for f in [
            c.by_mouth.entry((mouth(o), channel)).or_default(),
            c.by_lineage
                .entry((window, o.lineage_id.0, channel))
                .or_default(),
        ] {
            f.events += 1;
            f.energy += energy as u64;
            for (mask, &is_in) in inside.iter().enumerate() {
                f.events_in_mask[mask] += u64::from(is_in);
                f.energy_in_mask[mask] += if is_in { energy as u64 } else { 0 };
            }
        }
    });
}
pub fn finish() -> serde_json::Value {
    let c = ACTIVE
        .with(|a| a.borrow_mut().take())
        .expect("habitat association started");
    serde_json::json!({"version":1,"cells":c.masks.len(),"mask_cells":c.mask_cells,"overlap_cells":c.overlap_cells,
        "by_mouth_channel":c.by_mouth.iter().map(|((m,ch),v)|serde_json::json!({"mouth":m,"channel":ch,"totals":v})).collect::<Vec<_>>(),
        "by_lineage_window_channel":c.by_lineage.iter().map(|((w,l,ch),v)|serde_json::json!({"window":w,"lineage":l,"channel":ch,"totals":v})).collect::<Vec<_>>(),
        "definitions":"Masks: cells at or above the 75th-percentile static productivity numerator per channel (soft fertility*(200+moisture)*2, hard fertility*(1200-moisture)*elevation), fixed at start. Only successful environmental feeding (energy > 0); prey excluded. Mouth 0 Grazer, 1 Crusher, 2 Piercer. Area expectation without association is mask_cells/cells. Descriptive, not causal."})
}

#[cfg(test)]
mod tests {
    use crate::{Command, Config, World};
    #[test]
    fn feeding_events_close_against_viability_and_masks_are_quartiles() {
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
        plain.advance(2000);
        crate::viability::start(&world);
        super::start(&world);
        world.advance(2000);
        let h = super::finish();
        let v = crate::viability::finish(&world);
        assert_eq!(world.hash(), plain.hash());
        let cells = h["cells"].as_u64().unwrap();
        for ch in 0..2 {
            let m = h["mask_cells"][ch].as_u64().unwrap();
            assert!(
                m * 4 >= cells && m * 4 <= cells + cells / 10,
                "quartile mask"
            );
        }
        let food: u64 = v["cohorts"].as_array().unwrap()[..4]
            .iter()
            .map(|c| c["totals"]["food_energy"].as_u64().unwrap())
            .sum();
        let sum = |table: &str| -> u64 {
            h[table]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| r["totals"]["energy"].as_u64().unwrap())
                .sum()
        };
        assert_eq!(sum("by_mouth_channel"), food);
        assert_eq!(sum("by_lineage_window_channel"), food);
        for r in h["by_mouth_channel"].as_array().unwrap() {
            let t = &r["totals"];
            for mask in 0..2 {
                assert!(t["energy_in_mask"][mask].as_u64() <= t["energy"].as_u64());
            }
        }
    }
}
