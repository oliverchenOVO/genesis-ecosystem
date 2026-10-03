//! Read-only comparison: retain the existing nine-trait normalization and 150 cutoff.
use sim_core::{analysis::morphology_profile, morphology::Mouth, persistence};
use std::collections::BTreeMap;
fn main() -> Result<(), String> {
    let path = std::env::args().nth(1).ok_or("save path required")?;
    let world = persistence::load(std::path::Path::new(&path))?;
    let original_hash = world.hash();
    let mut groups = BTreeMap::<u64, (u64, [i64; 9])>::new();
    for o in &world.state.organisms {
        let m = &o.phenotype.morphology;
        let g = groups.entry(o.lineage_id.0).or_default();
        g.0 += 1;
        for (sum, value) in g.1.iter_mut().zip([
            m.segment_count * 1000,
            m.mass,
            m.armor,
            m.bite_capacity,
            m.locomotion_efficiency,
            m.sensory_investment,
            m.complexity,
            o.phenotype.speed,
            if m.mouth == Mouth::Piercer { 1000 } else { 0 },
        ]) {
            *sum += i64::from(value);
        }
    }
    let profiles = groups
        .iter()
        .filter(|(_, g)| g.0 >= 8)
        .map(|(id, (n, sums))| {
            (
                *id,
                *n,
                morphology_profile(sums.map(|v| (v / *n as i64) as i32)),
            )
        })
        .collect::<Vec<_>>();
    let mut pairs = Vec::new();
    for (i, (a, _, pa)) in profiles.iter().enumerate() {
        for (b, _, pb) in profiles.iter().skip(i + 1) {
            let distance = pa.iter().zip(pb).map(|(a, b)| (a - b).abs()).sum::<i32>() / 9;
            pairs.push(
                serde_json::json!({"a":a,"b":b,"distance":distance,"at_least_150":distance>=150}),
            );
        }
    }
    assert_eq!(original_hash, world.hash());
    println!(
        "{}",
        serde_json::json!({"path":path,"tick":world.state.tick,"hash":original_hash,"species_metrics":sim_core::analysis::ecology_metrics(&world),"lineage_profiles":profiles,"pair_distances":pairs,"definitions":"Same nine normalized morphology traits and integer mean L1 distance as acceptance. Living lineages >=8; snapshot observation only, not a persistence claim; authority and RNG unchanged."})
    );
    Ok(())
}
