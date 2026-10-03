//! Read-only evidence inspector; never used by the desktop or authoritative tick loop.
use std::collections::BTreeMap;
fn main() -> Result<(), String> {
    let path = std::env::args().nth(1).ok_or("save path required")?;
    let world = sim_core::persistence::load(std::path::Path::new(&path))?;
    let before = world.hash();
    let mut cohorts = BTreeMap::<u64, (usize, i64, usize, usize, u64, u64, u64)>::new();
    for o in &world.state.organisms {
        let c = cohorts
            .entry(o.lineage_id.0)
            .or_insert((0, 0, 0, 0, 0, u64::MAX, 0));
        c.0 += 1;
        c.1 += i64::from(o.phenotype.morphology.complexity);
        c.2 += usize::from(o.phenotype.morphology.segment_count > 1);
        c.3 += usize::from(o.parents.is_some());
        c.4 += u64::from(o.offspring);
        c.5 = c.5.min(o.generation);
        c.6 = c.6.max(o.generation);
    }
    let rows: Vec<_> = cohorts.into_iter().map(|(id,c)| {
        let lineage = &world.state.lineages[&sim_core::model::LineageId(id)];
        serde_json::json!({"lineage_id":id,"parent_lineage":lineage.parent,"origin_tick":lineage.origin_tick,"population":c.0,"mean_complexity":c.1/c.0 as i64,"multiunit_population":c.2,"living_members_with_real_parents":c.3,"offspring_already_produced_by_living_members":c.4,"generation_min":c.5,"generation_max":c.6,"current_high_complexity_cohort":c.0>=8&&c.1/c.0 as i64>=200})
    }).collect();
    let mut continued = world.clone();
    let mut streaks = BTreeMap::<u64, u64>::new();
    for row in &rows {
        if row["current_high_complexity_cohort"] == true {
            streaks.insert(row["lineage_id"].as_u64().unwrap(), 0);
        }
    }
    let mut samples = Vec::new();
    for _ in 0..10 {
        continued.advance(100);
        continued.validate()?;
        let mut current = BTreeMap::<u64, (u64, i64)>::new();
        for o in &continued.state.organisms {
            let c = current.entry(o.lineage_id.0).or_default();
            c.0 += 1;
            c.1 += i64::from(o.phenotype.morphology.complexity);
        }
        let qualifying: Vec<_> = current
            .iter()
            .filter(|(_, c)| c.0 >= 8 && c.1 / c.0 as i64 >= 200)
            .map(|(id, _)| *id)
            .collect();
        for (id, streak) in &mut streaks {
            *streak = if qualifying.contains(id) {
                *streak + 100
            } else {
                0
            };
        }
        samples.push(serde_json::json!({"tick":continued.state.tick,"qualifying_high_complexity_lineages":qualifying,"population":continued.state.organisms.len()}));
    }
    if world.hash() != before {
        return Err("Inspection mutated saved authority".into());
    }
    println!(
        "{}",
        serde_json::json!({"path":path,"tick":world.state.tick,"hash":before,"population":world.state.organisms.len(),"living_lineages":rows,"continuation":{"ticks":1000,"hash":continued.hash(),"saved_high_complexity_lineage_sampled_streaks":streaks,"samples":samples},"definitions":"Actual living lineage cohorts and inherited ancestry/offspring. Continued a clone under frozen rules, sampling100 ticks and requiring >=8 members with mean C>=200. Original save/world unchanged. Pair with full-run calibration for historical duration evidence."})
    );
    Ok(())
}
