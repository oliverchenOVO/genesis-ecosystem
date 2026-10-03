//! Bounded regulatory expression. All authoritative quantities use integers.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MorphologyGenome {
    pub segmentation: u16,
    pub aspect: u16,
    pub appendages: u16,
    pub mouth: u16,
    pub armor: u16,
    pub sensory: u16,
    pub locomotion: u16,
    pub storage: u16,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Mouth {
    Grazer,
    Crusher,
    Piercer,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BodyMorphology {
    pub segment_count: i32,
    pub aspect_ratio: i32,
    pub appendage_count: i32,
    pub mouth: Mouth,
    pub mass: i32,
    pub armor: i32,
    pub bite_capacity: i32,
    pub sensory_investment: i32,
    pub locomotion_efficiency: i32,
    pub storage: i32,
    pub complexity: i32,
    pub maintenance_cost: i32,
    pub movement_cost: i32,
    pub reproduction_cost: i32,
}

impl MorphologyGenome {
    pub fn from_loci(g: &[u16]) -> Self {
        Self {
            segmentation: g[0],
            aspect: g[1],
            appendages: g[2],
            mouth: g[3],
            armor: g[4],
            sensory: g[5],
            locomotion: g[6],
            storage: g[7],
        }
    }

    pub fn express(&self, body_size: i32) -> BodyMorphology {
        let segment_count = 1 + i32::from(self.segmentation) / 250;
        let aspect_ratio = 500 + i32::from(self.aspect);
        let appendage_count = i32::from(self.appendages) / 125;
        let armor = i32::from(self.armor) * 600 / 1000;
        let sensory_investment = i32::from(self.sensory);
        let locomotion_efficiency = 500 + i32::from(self.locomotion);
        let storage = i32::from(self.storage);
        let mass = body_size * 100 + (segment_count - 1) * 150 + armor / 2;
        let complexity = (segment_count - 1) * 80
            + appendage_count * 12
            + armor / 6
            + sensory_investment / 12
            + i32::from(self.locomotion) / 16
            + storage / 16;
        let mouth = match self.mouth {
            0..=332 => Mouth::Grazer,
            333..=665 => Mouth::Crusher,
            _ => Mouth::Piercer,
        };
        let bite_capacity = mass / 50
            + match mouth {
                Mouth::Grazer => 4,
                Mouth::Crusher => 12,
                Mouth::Piercer => 22,
            };
        BodyMorphology {
            segment_count,
            aspect_ratio,
            appendage_count,
            mouth,
            mass,
            armor,
            bite_capacity,
            sensory_investment,
            locomotion_efficiency,
            storage,
            complexity,
            maintenance_cost: 1 + complexity / 90,
            movement_cost: (100 + mass / 10 + armor / 2 + (aspect_ratio - 1000).abs() / 5
                - i32::from(self.locomotion) / 8
                - appendage_count * 4)
                .max(50),
            reproduction_cost: 400 + mass + complexity * 2,
        }
    }
}

impl BodyMorphology {
    pub fn resource_efficiency(&self, hardness: i32) -> i32 {
        match self.mouth {
            Mouth::Grazer => 125 - hardness / 12,
            Mouth::Crusher => {
                65 + hardness / 12
                    + hardness * (self.bite_capacity - hardness / 20).clamp(0, 20) / 50
            }
            Mouth::Piercer => 35,
        }
    }
    pub fn terrain_speed(&self, speed: i32, roughness: i32) -> i32 {
        (speed * 1000 / (1000 + roughness * (self.mass / 100 + self.armor / 100) / 20)).max(1)
    }
    pub fn can_attack(&self, prey: &Self) -> bool {
        self.mouth != Mouth::Grazer && self.bite_capacity * 40 >= prey.mass
    }
    pub fn damage(&self, prey: &Self, aggression: i32, speed: i32, prey_speed: i32) -> i32 {
        let force = (self.bite_capacity + aggression / 50 + (speed - prey_speed) / 2).max(1);
        (force * (1000 - prey.armor) / 1000).max(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn genes() -> MorphologyGenome {
        MorphologyGenome::from_loci(&[0, 500, 0, 100, 0, 0, 0, 0])
    }
    #[test]
    fn complexity_has_storage_benefit_and_real_costs() {
        let simple = genes().express(8);
        let complex = MorphologyGenome::from_loci(&[750, 500, 750, 100, 0, 0, 750, 750]).express(8);
        assert!(complex.segment_count > simple.segment_count);
        assert!(complex.mass > simple.mass && complex.storage > simple.storage);
        assert!(complex.maintenance_cost > simple.maintenance_cost);
        assert!(complex.reproduction_cost > simple.reproduction_cost);
    }
    #[test]
    fn defense_and_terrain_have_causal_tradeoffs() {
        let light = genes().express(8);
        let mut armored = genes();
        armored.armor = 1000;
        let armored = armored.express(8);
        let mut hunter = genes();
        hunter.mouth = 900;
        let hunter = hunter.express(8);
        assert!(hunter.damage(&armored, 500, 8, 8) < hunter.damage(&light, 500, 8, 8));
        assert!(
            armored.movement_cost > light.movement_cost
                && armored.reproduction_cost > light.reproduction_cost
        );
        assert!(armored.terrain_speed(12, 1000) < light.terrain_speed(12, 1000));
    }
    #[test]
    fn feeding_specialization_and_prey_size_constraints() {
        let grazer = genes().express(4);
        let mut crusher = genes();
        crusher.mouth = 500;
        let crusher = crusher.express(4);
        let mut hunter = genes();
        hunter.mouth = 900;
        let hunter = hunter.express(4);
        assert!(grazer.resource_efficiency(0) > crusher.resource_efficiency(0));
        assert!(crusher.resource_efficiency(1000) > grazer.resource_efficiency(1000));
        assert!(!grazer.can_attack(&hunter));
        assert!(!hunter.can_attack(&MorphologyGenome::from_loci(&[1000; 8]).express(12)));
    }
    #[test]
    fn hard_resource_processing_uses_bite_capacity_not_complexity_labels() {
        let mut g = genes();
        g.mouth = 500;
        let small = g.express(8);
        g.segmentation = 750;
        let large = g.express(8);
        assert_eq!(small.resource_efficiency(0), large.resource_efficiency(0));
        assert!(large.resource_efficiency(700) > small.resource_efficiency(700));
        assert_eq!(
            small.resource_efficiency(1000),
            large.resource_efficiency(1000)
        );
        assert!(large.maintenance_cost > small.maintenance_cost);
    }
}
