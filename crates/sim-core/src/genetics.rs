use crate::rng::Rng;
use serde::{Deserialize, Serialize};

pub const LOCI: usize = 14;

/// Haploid bounded regulatory alleles; values are not gameplay stats.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Genome(pub [u16; LOCI]);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Phenotype {
    pub body_size: i32,
    pub speed: i32,
    pub vision: i32,
    pub metabolism: i32,
    pub food_efficiency: i32,
    pub reproduction_threshold: i32,
    pub offspring_count: u32,
    pub mutation_rate: u32,
    pub aggression: i32,
    pub social_affinity: i32,
    pub temperature_optimum: i32,
    pub temperature_tolerance: i32,
    pub energy_capacity: i32,
    pub carnivory: i32,
}

impl Genome {
    pub fn random(rng: &mut Rng) -> Self {
        Self(std::array::from_fn(|_| rng.below(1001) as u16))
    }

    pub fn phenotype(&self) -> Phenotype {
        let g = self.0.map(i32::from);
        let body = 4 + g[0] / 125;
        let capacity = 1200 + g[12] * 2 + body * 40;
        Phenotype {
            body_size: body,
            speed: (2 + g[1] / 100) * 8 / body,
            vision: 16 + g[2] / 16,
            metabolism: 1 + g[3] / 250 + body / 4,
            food_efficiency: 60 + g[4] / 10,
            reproduction_threshold: capacity * (55 + g[5] / 40) / 100,
            offspring_count: 1 + (g[6] / 500) as u32,
            mutation_rate: 5 + (g[7] / 10) as u32,
            aggression: g[8],
            social_affinity: g[9],
            temperature_optimum: 500 + g[10] * 3,
            temperature_tolerance: 400 + g[11],
            energy_capacity: capacity,
            carnivory: g[13],
        }
    }

    pub fn distance(&self, other: &Self) -> u32 {
        self.0
            .iter()
            .zip(other.0)
            .map(|(a, b)| u32::from(a.abs_diff(b)))
            .sum::<u32>()
            / LOCI as u32
    }

    pub fn child(
        a: &Self,
        b: &Self,
        reproduction: &mut Rng,
        mutation: &mut Rng,
        rate: u32,
    ) -> (Self, u32) {
        let mut count = 0;
        let genes = std::array::from_fn(|i| {
            let mut value = if reproduction.below(2) == 0 {
                a.0[i]
            } else {
                b.0[i]
            } as i32;
            if mutation.below(1000) < u64::from(rate) {
                let kind = mutation.below(100);
                value = if kind == 0 {
                    mutation.below(1001) as i32
                } else {
                    let span = if kind < 5 { 300 } else { 60 };
                    value + mutation.below((span * 2 + 1) as u64) as i32 - span
                };
                count += 1;
            }
            value.clamp(0, 1000) as u16
        });
        (Self(genes), count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inheritance_without_mutation() {
        let a = Genome([100; LOCI]);
        let b = Genome([900; LOCI]);
        let (child, count) = Genome::child(&a, &b, &mut Rng(1), &mut Rng(2), 0);
        assert_eq!(count, 0);
        assert!(child.0.iter().all(|g| *g == 100 || *g == 900));
        assert!(child.0.contains(&100) && child.0.contains(&900));
    }

    #[test]
    fn mutation_deterministic_and_bounded_property() {
        for seed in 0..100 {
            let a = Genome::random(&mut Rng(seed));
            let x = Genome::child(&a, &a, &mut Rng(seed), &mut Rng(seed + 1), 1000);
            let y = Genome::child(&a, &a, &mut Rng(seed), &mut Rng(seed + 1), 1000);
            assert_eq!(x, y);
            assert_eq!(x.1, LOCI as u32);
            assert!(x.0 .0.iter().all(|g| *g <= 1000));
        }
    }

    #[test]
    fn phenotype_tradeoff_and_distance() {
        let a = Genome([0; LOCI]);
        let b = Genome([1000; LOCI]);
        assert_eq!(a.distance(&b), 1000);
        assert!(a.phenotype().body_size < b.phenotype().body_size);
        assert!(b.phenotype().reproduction_threshold < b.phenotype().energy_capacity);
    }
}
