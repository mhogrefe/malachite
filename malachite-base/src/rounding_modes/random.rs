use random::random_values_from_slice;
use random::random_values_from_slice::RandomValuesFromSlice;
use random::seed::Seed;
use rounding_modes::{RoundingMode, ROUNDING_MODES};
use std::iter::Cloned;

/// Generates a random `RoundingMode` that has an equal probability of being any of the 6
/// `RoundingMode`s.
///
/// Length is infinite.
///
/// Time per iteration: worst case O(1)
///
/// Additional memory per iteration: wost case O(1)
///
/// # Examples
/// ```
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::rounding_modes::RoundingMode;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_base::rounding_modes::random::random_rounding_modes;
///
/// assert_eq!(
///     random_rounding_modes(EXAMPLE_SEED).take(10).collect::<Vec<RoundingMode>>(),
///     &[Up, Exact, Ceiling, Up, Floor, Nearest, Exact, Up, Floor, Exact]
/// )
/// ```
#[inline]
pub fn random_rounding_modes(seed: Seed) -> Cloned<RandomValuesFromSlice<'static, RoundingMode>> {
    random_values_from_slice(seed, &ROUNDING_MODES).cloned()
}
