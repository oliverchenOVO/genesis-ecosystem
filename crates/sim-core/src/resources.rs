//! Independent renewable fields. Integer remainder accumulation avoids rounding
//! slow renewable hard resources permanently to zero. No extra RNG or species rules.
use crate::{Cell, Organism};
pub const DENOMINATORS: [i64; 2] = [300_000, 300_000_000];
impl Cell {
    pub fn total_food(&self) -> i32 {
        self.food + self.hard_food
    }
    pub fn initialize_channels(&mut self) {
        let soft_fraction = (200 + self.moisture * 4 / 5 - self.elevation / 5).clamp(100, 900);
        let soft = self.food * soft_fraction / 1000;
        self.hard_food = ((self.food - soft) / 3).min(300);
        self.food = soft;
    }
    pub fn regenerate_channels(&mut self, regeneration: i32) {
        let rate = i64::from(regeneration) * i64::from(self.fertility);
        let numerators = [
            rate * i64::from(200 + self.moisture) * 2,
            rate * i64::from(1200 - self.moisture) * i64::from(self.elevation),
        ];
        for channel in 0..2 {
            let numerator = numerators[channel] + self.resource_remainders[channel];
            let growth = (numerator / DENOMINATORS[channel]) as i32;
            self.resource_remainders[channel] = numerator % DENOMINATORS[channel];
            if channel == 0 {
                self.food = (self.food + growth).min(1000);
            } else {
                self.hard_food = (self.hard_food + growth).min(300);
            }
        }
    }
    pub fn hardness(&self, channel: usize) -> i32 {
        if channel == 0 {
            40 + self.elevation / 20
        } else {
            300 + self.elevation / 3
        }
    }
    pub fn feeding_efficiency(&self, o: &Organism, channel: usize) -> i32 {
        let hardness = self.hardness(channel);
        let m = &o.phenotype.morphology;
        let processing = if channel == 0 {
            1000
        } else {
            (m.bite_capacity * 20 * 1000 / hardness).min(1000)
        };
        o.phenotype.food_efficiency * (1000 - o.phenotype.carnivory) / 1000
            * m.resource_efficiency(hardness)
            / 100
            * processing
            / 1000
            * if channel == 0 { 1 } else { 3 }
    }
    pub fn preferred_feeding(&self, o: &Organism) -> (usize, i32, i32) {
        let efficiency = [self.feeding_efficiency(o, 0), self.feeding_efficiency(o, 1)];
        let stock = [self.food, self.hard_food];
        let channel = usize::from(stock[1] * efficiency[1] > stock[0] * efficiency[0]);
        (channel, stock[channel], efficiency[channel])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Config, World};
    #[test]
    fn slow_channels_are_renewable_spatially_distinct_and_bounded() {
        let world = World::new(Config::default()).unwrap();
        let mut dry = world.state.environment.cells[0].clone();
        dry.food = 0;
        dry.hard_food = 0;
        dry.resource_remainders = [0; 2];
        dry.elevation = 900;
        dry.moisture = 100;
        dry.fertility = 100;
        let mut wet = dry.clone();
        wet.elevation = 100;
        wet.moisture = 900;
        for _ in 0..100 {
            dry.regenerate_channels(4);
            wet.regenerate_channels(4);
        }
        assert!(dry.hard_food > wet.hard_food && wet.food > dry.food);
        assert!(dry.hard_food > 0 && wet.hard_food > 0);
        assert_eq!(
            dry.resource_remainders[1],
            (100 * 4 * 100 * 1100 * 900) % DENOMINATORS[1]
        );
        let paused = dry.clone();
        dry.regenerate_channels(0);
        assert_eq!(paused, dry);
        for _ in 0..10000 {
            dry.regenerate_channels(100);
        }
        assert!(dry.food <= 1000 && dry.hard_food <= 300);
    }
    #[test]
    fn initial_chemical_budget_and_processing_costs_are_real() {
        let w = World::new(Config::default()).unwrap();
        assert!(w
            .state
            .environment
            .cells
            .iter()
            .all(|c| c.food + 3 * c.hard_food <= 999));
        let mut cell = w.state.environment.cells[0].clone();
        cell.elevation = 900;
        let mut grazer = w.state.organisms[0].clone();
        grazer.genome.0[13] = 0;
        grazer.genome.0[17] = 0;
        grazer.phenotype = grazer.genome.phenotype();
        let mut crusher = grazer.clone();
        crusher.genome.0[17] = 500;
        crusher.phenotype = crusher.genome.phenotype();
        assert!(cell.feeding_efficiency(&grazer, 0) > cell.feeding_efficiency(&crusher, 0));
        assert!(cell.feeding_efficiency(&crusher, 1) > cell.feeding_efficiency(&grazer, 1));
        let mut massive = crusher.clone();
        massive.genome.0[14] = 750;
        massive.phenotype = massive.genome.phenotype();
        assert!(cell.feeding_efficiency(&massive, 1) > cell.feeding_efficiency(&crusher, 1));
        assert!(
            massive.phenotype.morphology.reproduction_cost
                > crusher.phenotype.morphology.reproduction_cost
        );
        assert!(
            massive.phenotype.morphology.maintenance_cost
                > crusher.phenotype.morphology.maintenance_cost
        );
        // Equal zero yields must retain soft-channel tie order and observed supply.
        grazer.genome.0[13] = 1000;
        grazer.phenotype = grazer.genome.phenotype();
        cell.food = 7;
        cell.hard_food = 300;
        assert_eq!(cell.preferred_feeding(&grazer), (0, 7, 0));
    }
}
