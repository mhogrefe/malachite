use std::iter::Cloned;
use std::slice::Iter;

use rounding_modes::{RoundingMode, ROUNDING_MODES};

/// Generates all `RoundingMode`s.
///
/// Length is 6.
///
/// Time: worst case O(1) per iteration
///
/// Additional memory: worst case O(1) per iteration
///
/// # Examples
/// ```
/// use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
/// use malachite_base::rounding_modes::RoundingMode;
///
/// assert_eq!(
///     exhaustive_rounding_modes().collect::<Vec<RoundingMode>>(),
///     &[
///         RoundingMode::Down, RoundingMode::Up, RoundingMode::Floor, RoundingMode::Ceiling,
///         RoundingMode::Nearest, RoundingMode::Exact,
///     ]
/// );
/// ```
#[inline]
pub fn exhaustive_rounding_modes() -> Cloned<Iter<'static, RoundingMode>> {
    ROUNDING_MODES.iter().cloned()
}
