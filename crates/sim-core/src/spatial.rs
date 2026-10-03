use crate::model::Organism;

pub const CELL_SIZE: i32 = 16;

pub struct Spatial {
    pub side: usize,
    pub buckets: Vec<Vec<usize>>,
}
impl Spatial {
    pub fn new(size: i32, organisms: &[Organism]) -> Self {
        let side = (size / CELL_SIZE) as usize;
        let mut buckets = vec![Vec::new(); side * side];
        for (i, o) in organisms.iter().enumerate() {
            buckets[(o.y / CELL_SIZE) as usize * side + (o.x / CELL_SIZE) as usize].push(i);
        }
        Self { side, buckets }
    }
    pub fn nearby(&self, x: i32, y: i32, radius: i32) -> impl Iterator<Item = usize> + '_ {
        let max = self.side as i32 - 1;
        let x0 = ((x - radius) / CELL_SIZE).clamp(0, max);
        let x1 = ((x + radius) / CELL_SIZE).clamp(0, max);
        let y0 = ((y - radius) / CELL_SIZE).clamp(0, max);
        let y1 = ((y + radius) / CELL_SIZE).clamp(0, max);
        (y0..=y1).flat_map(move |cy| {
            (x0..=x1).flat_map(move |cx| {
                self.buckets[cy as usize * self.side + cx as usize]
                    .iter()
                    .copied()
            })
        })
    }
}
pub fn distance_squared(ax: i32, ay: i32, bx: i32, by: i32) -> i32 {
    (ax - bx).pow(2) + (ay - by).pow(2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn local_mating_queries_match_distance_oracle_at_cell_and_world_edges() {
        let w = crate::World::new(crate::Config {
            starting_population: 200,
            ..crate::Config::default()
        })
        .unwrap();
        let grid = Spatial::new(w.state.config.size, &w.state.organisms);
        for (x, y) in [(0, 0), (15, 16), (256, 256), (511, 511)] {
            for radius in [16, 24] {
                let mut indexed = grid
                    .nearby(x, y, radius)
                    .filter(|i| {
                        distance_squared(x, y, w.state.organisms[*i].x, w.state.organisms[*i].y)
                            <= radius * radius
                    })
                    .collect::<Vec<_>>();
                indexed.sort_unstable();
                let oracle = w
                    .state
                    .organisms
                    .iter()
                    .enumerate()
                    .filter(|(_, o)| distance_squared(x, y, o.x, o.y) <= radius * radius)
                    .map(|(i, _)| i)
                    .collect::<Vec<_>>();
                assert_eq!(indexed, oracle);
            }
        }
    }
    #[test]
    fn grid_edges_and_local_lookup() {
        let mut grid = Spatial::new(256, &[]);
        grid.buckets[0].push(1);
        grid.buckets[255].push(2);
        assert_eq!(grid.nearby(0, 0, 16).collect::<Vec<_>>(), vec![1]);
        assert_eq!(grid.nearby(255, 255, 16).collect::<Vec<_>>(), vec![2]);
        assert_eq!(grid.nearby(128, 128, 256).count(), 2);
    }
}
