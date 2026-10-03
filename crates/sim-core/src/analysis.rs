//! Read-only, fixed-point species distances. Never consumes RNG or changes a world.
use crate::World;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct EcologyMetrics {
    pub morphology_mean_distance: i32,
    pub morphology_max_distance: i32,
    pub morphology_clusters: usize,
    pub morphology_variance: [i64; 6],
    pub niche_mean_distance: i32,
    pub niche_max_distance: i32,
    pub niche_clusters: usize,
    pub predator_like_population: usize,
    pub primary_consumer_population: usize,
    pub multicellular_population: usize,
    pub maximum_complexity: i32,
    pub median_species_complexity: i32,
    /// (SpeciesId, minimum occupied temperature, maximum occupied temperature), centidegrees.
    pub occupied_temperature_ranges: Vec<(u64, i32, i32)>,
}
pub fn niche_cluster_representatives(world: &World) -> Vec<(u64, [i32; 6])> {
    let mut representatives = Vec::<(u64, [i32; 6])>::new();
    for s in world.state.species.values().filter(|s| s.population >= 8) {
        if !representatives.iter().any(|(_, profile)| {
            profile
                .iter()
                .zip(s.niche)
                .map(|(a, b)| (a - b).abs())
                .sum::<i32>()
                / 6
                < 150
        }) {
            representatives.push((s.id.0, s.niche));
        }
    }
    representatives
}
fn distances<const N: usize>(values: &[[i32; N]]) -> (i32, i32, usize) {
    let mut total = 0i64;
    let mut pairs = 0;
    let mut max = 0;
    let mut representatives = Vec::<usize>::new();
    for (i, a) in values.iter().enumerate() {
        let distance =
            |b: &[i32; N]| a.iter().zip(b).map(|(a, b)| (a - b).abs()).sum::<i32>() / N as i32;
        for b in &values[..i] {
            let d = distance(b);
            total += i64::from(d);
            pairs += 1;
            max = max.max(d);
        }
        if !representatives.iter().any(|j| distance(&values[*j]) < 150) {
            representatives.push(i);
        }
    }
    ((total / pairs.max(1)) as i32, max, representatives.len())
}
pub fn ecology_metrics(world: &World) -> EcologyMetrics {
    let mut morphology = Vec::new();
    let mut niche = Vec::new();
    let mut complexities = Vec::new();
    for s in world.state.species.values().filter(|s| s.population >= 8) {
        let m = s.morphology_summary;
        morphology.push(
            [
                m[0] / 5,
                m[1] / 3,
                m[2],
                m[3] * 10,
                m[4] / 2,
                m[5],
                m[6],
                m[7] * 50,
                m[8],
            ]
            .map(|v| v.clamp(0, 1000)),
        );
        niche.push(s.niche);
        complexities.push(m[6]);
    }
    complexities.sort();
    let (morphology_mean_distance, morphology_max_distance, morphology_clusters) =
        distances(&morphology);
    let (niche_mean_distance, niche_max_distance, niche_clusters) = distances(&niche);
    let mut sums = [0i64; 6];
    let mut squares = [0i64; 6];
    for o in &world.state.organisms {
        let m = &o.phenotype.morphology;
        let values = [
            m.segment_count * 1000,
            m.mass,
            m.armor,
            m.bite_capacity,
            m.sensory_investment,
            m.complexity,
        ];
        for i in 0..6 {
            let v = i64::from(values[i]);
            sums[i] += v;
            squares[i] += v * v;
        }
    }
    let n = world.state.organisms.len().max(1) as i64;
    let morphology_variance =
        std::array::from_fn(|i| ((squares[i] * n - sums[i] * sums[i]) / (n * n)).max(0));
    let predator_like_population = world
        .state
        .organisms
        .iter()
        .filter(|o| {
            o.phenotype.carnivory > 650
                && o.phenotype.morphology.mouth != crate::morphology::Mouth::Grazer
        })
        .count();
    let mut temperatures = std::collections::BTreeMap::<u64, (i32, i32)>::new();
    let side = world.state.config.size / crate::spatial::CELL_SIZE;
    for o in &world.state.organisms {
        let x = o.x / crate::spatial::CELL_SIZE;
        let y = o.y / crate::spatial::CELL_SIZE;
        let temperature = world.state.environment.temperature
            + world.state.environment.cells[(y * side + x) as usize].temperature_offset;
        let range = temperatures
            .entry(o.species_id.0)
            .or_insert((temperature, temperature));
        range.0 = range.0.min(temperature);
        range.1 = range.1.max(temperature);
    }
    EcologyMetrics {
        morphology_mean_distance,
        morphology_max_distance,
        morphology_clusters,
        morphology_variance,
        niche_mean_distance,
        niche_max_distance,
        niche_clusters,
        predator_like_population,
        primary_consumer_population: world.state.organisms.len() - predator_like_population,
        multicellular_population: world
            .state
            .organisms
            .iter()
            .filter(|o| o.phenotype.morphology.segment_count > 1)
            .count(),
        maximum_complexity: world
            .state
            .organisms
            .iter()
            .map(|o| o.phenotype.morphology.complexity)
            .max()
            .unwrap_or(0),
        median_species_complexity: complexities
            .get(complexities.len() / 2)
            .copied()
            .unwrap_or(0),
        occupied_temperature_ranges: temperatures
            .into_iter()
            .map(|(id, (low, high))| (id, low, high))
            .collect(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Config;
    #[test]
    fn occupied_temperature_range_uses_actual_cells_at_world_edges() {
        let mut w = World::new(Config {
            starting_population: 2,
            ..Config::default()
        })
        .unwrap();
        w.state.organisms[0].x = 0;
        w.state.organisms[0].y = 0;
        w.state.organisms[1].x = 511;
        w.state.organisms[1].y = 511;
        w.state.environment.cells[0].temperature_offset = 100;
        w.state
            .environment
            .cells
            .last_mut()
            .unwrap()
            .temperature_offset = 500;
        let before = w.hash();
        assert_eq!(
            ecology_metrics(&w).occupied_temperature_ranges,
            vec![(1, 2100, 2500)]
        );
        assert_eq!(before, w.hash());
        assert_eq!(
            niche_cluster_representatives(&w).len(),
            ecology_metrics(&w).niche_clusters
        );
    }
    #[test]
    fn distances_distinguish_real_species_profiles() {
        assert_eq!(distances(&[[0; 6], [1000; 6]]), (1000, 1000, 2));
        assert_eq!(distances(&[[0; 6], [50; 6]]), (50, 50, 1));
        assert_eq!(distances::<6>(&[]), (0, 0, 0));
    }
    #[test]
    fn observation_is_repeatable_without_changing_rng_or_authority() {
        let w = World::new(Config::default()).unwrap();
        let hash = w.hash();
        let a = ecology_metrics(&w);
        assert_eq!(a, ecology_metrics(&w));
        assert_eq!(hash, w.hash());
        assert_eq!(a.multicellular_population, 0);
        assert_eq!(
            a.predator_like_population + a.primary_consumer_population,
            w.state.organisms.len()
        );
    }
}
