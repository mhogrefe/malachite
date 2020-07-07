use rand::Rng;
use rand_chacha::ChaCha20Rng;

use num::arithmetic::traits::Parity;
use random::seed::Seed;

/// Generates random `bools` uniformly.
#[derive(Clone, Debug)]
pub struct RandomBools {
    rng: ChaCha20Rng,
    x: u32,
    bits_left: u8,
}

impl Iterator for RandomBools {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        if self.bits_left == 0 {
            self.x = self.rng.gen();
            self.bits_left = 31;
        } else {
            self.x >>= 1;
            self.bits_left -= 1;
        }
        Some(self.x.odd())
    }
}

/// Generates a random `bool` that has an equal probability of being `true` or `false`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_base::bools::random::random_bools;
/// use malachite_base::random::EXAMPLE_SEED;
///
/// assert_eq!(
///     random_bools(EXAMPLE_SEED).take(10).collect::<Vec<bool>>(),
///     &[true, false, false, false, true, true, true, false, true, true]
/// )
/// ```
pub fn random_bools(seed: Seed) -> RandomBools {
    RandomBools {
        rng: seed.get_rng(),
        x: 0,
        bits_left: 0,
    }
}
