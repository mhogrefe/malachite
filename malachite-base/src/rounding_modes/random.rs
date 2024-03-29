use crate::random::Seed;
use crate::rounding_modes::{RoundingMode, ROUNDING_MODES};
use crate::slices::{random_values_from_slice, RandomValuesFromSlice};
use std::iter::Cloned;

/// Uniformly generates random [`RoundingMode`]s.
pub type RandomRoundingModes = Cloned<RandomValuesFromSlice<'static, RoundingMode>>;

/// Uniformly generates random [`RoundingMode`]s.
///
/// The output length is infinite.
///
/// # Expected complexity
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::rounding_modes::random::random_rounding_modes;
/// use malachite_base::rounding_modes::RoundingMode;
/// use malachite_base::rounding_modes::RoundingMode::*;
///
/// assert_eq!(
///     random_rounding_modes(EXAMPLE_SEED).take(10).collect_vec(),
///     &[Up, Exact, Ceiling, Up, Floor, Nearest, Exact, Up, Floor, Exact]
/// )
/// ```
#[inline]
pub fn random_rounding_modes(seed: Seed) -> RandomRoundingModes {
    random_values_from_slice(seed, &ROUNDING_MODES).cloned()
}
