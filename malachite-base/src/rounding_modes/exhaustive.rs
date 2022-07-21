use crate::rounding_modes::{RoundingMode, ROUNDING_MODES};
use std::iter::Cloned;
use std::slice::Iter;

pub type ExhaustiveRoundingModes = Cloned<Iter<'static, RoundingMode>>;

/// Generates all [`RoundingMode`]s.
///
/// The output length is 6.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
/// use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
/// use malachite_base::rounding_modes::RoundingMode;
///
/// assert_eq!(
///     exhaustive_rounding_modes().collect_vec(),
///     &[
///         RoundingMode::Down,
///         RoundingMode::Up,
///         RoundingMode::Floor,
///         RoundingMode::Ceiling,
///         RoundingMode::Nearest,
///         RoundingMode::Exact,
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_rounding_modes() -> ExhaustiveRoundingModes {
    ROUNDING_MODES.iter().cloned()
}
