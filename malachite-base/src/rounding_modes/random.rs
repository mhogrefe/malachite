use std::iter::Cloned;

use random::Seed;
use rounding_modes::{RoundingMode, ROUNDING_MODES};
use slices::{random_values_from_slice, RandomValuesFromSlice};

/// Uniformly generates a random `RoundingMode`.
///
/// The output length is infinite.
///
/// # Expected worst-case complexity
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::rounding_modes::RoundingMode;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_base::rounding_modes::random::random_rounding_modes;
///
/// assert_eq!(
///     random_rounding_modes(EXAMPLE_SEED).take(10).collect_vec(),
///     &[Up, Exact, Ceiling, Up, Floor, Nearest, Exact, Up, Floor, Exact]
/// )
/// ```
#[inline]
pub fn random_rounding_modes(seed: Seed) -> Cloned<RandomValuesFromSlice<'static, RoundingMode>> {
    random_values_from_slice(seed, &ROUNDING_MODES).cloned()
}
