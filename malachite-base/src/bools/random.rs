use rand::Rng;
use rand_chacha::ChaCha20Rng;

use random::StandardRand;

impl StandardRand for bool {
    /// Generates a random `bool` that has an equal probability of being `true` or `false`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::random::{standard_random_values, EXAMPLE_SEED};
    ///
    /// assert_eq!(
    ///     standard_random_values::<bool>(EXAMPLE_SEED).take(10).collect::<Vec<bool>>(),
    ///     &[false, true, false, true, false, true, false, false, true, true]
    /// )
    /// ```
    #[inline]
    fn standard_gen(rng: &mut ChaCha20Rng) -> bool {
        rng.gen()
    }
}
