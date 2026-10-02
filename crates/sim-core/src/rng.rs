use serde::{Deserialize, Serialize};

pub const RNG_VERSION: u32 = 1;

/// SplitMix64, with explicit wrapping arithmetic and saved stream state.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rng(pub u64);

impl Rng {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    pub fn below(&mut self, n: u64) -> u64 {
        assert!(n > 0);
        let threshold = n.wrapping_neg() % n;
        loop {
            let value = self.next_u64();
            if value >= threshold {
                return value % n;
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Streams {
    pub world: Rng,
    pub behavior: Rng,
    pub reproduction: Rng,
    pub mutation: Rng,
}

impl Streams {
    pub fn new(seed: u64) -> Self {
        let mut source = Rng(seed);
        Self {
            world: Rng(source.next_u64()),
            behavior: Rng(source.next_u64()),
            reproduction: Rng(source.next_u64()),
            mutation: Rng(source.next_u64()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn published_splitmix_vector() {
        assert_eq!(Rng(0).next_u64(), 0xe220a8397b1dcdaf);
    }

    #[test]
    fn independent_streams_and_bounds() {
        let mut a = Streams::new(42);
        let mut b = a.clone();
        a.world.next_u64();
        assert_eq!(a.mutation.next_u64(), b.mutation.next_u64());
        for bound in [1, 7, 256, u64::MAX] {
            for _ in 0..1000 {
                assert!(a.world.below(bound) < bound);
            }
        }
    }
}
