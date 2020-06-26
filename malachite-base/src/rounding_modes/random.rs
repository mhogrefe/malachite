use rand::seq::SliceRandom;
use rand_chacha::ChaCha20Rng;

use random::StandardRand;
use rounding_modes::{RoundingMode, ROUNDING_MODES};

impl StandardRand for RoundingMode {
    /// Generates a random `RoundingMode` that has an equal probability of being any of the 6
    /// `RoundingMode`s.
    ///
    /// Length is infinite.
    ///
    /// Time per iteration: O(1)
    ///
    /// Additional memory per iteration: O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::random::{standard_random_values, EXAMPLE_SEED};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_base::rounding_modes::RoundingMode;
    ///
    /// assert_eq!(
    ///     standard_random_values::<RoundingMode>(EXAMPLE_SEED)
    ///         .take(10).collect::<Vec<RoundingMode>>(),
    ///     &[Floor, Exact, Floor, Nearest, Up, Ceiling, Ceiling, Exact, Nearest, Up]
    /// )
    /// ```
    #[inline]
    fn standard_gen(rng: &mut ChaCha20Rng) -> RoundingMode {
        *ROUNDING_MODES.choose(rng).unwrap()
    }
}
