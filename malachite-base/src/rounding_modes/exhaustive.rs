// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rounding_modes::{RoundingMode, ROUNDING_MODES};
use core::iter::Copied;
use core::slice::Iter;

pub type ExhaustiveRoundingModes = Copied<Iter<'static, RoundingMode>>;

/// Generates all [`RoundingMode`]s.
///
/// The output length is 6.
///
/// # Complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
/// use malachite_base::rounding_modes::RoundingMode::*;
///
/// assert_eq!(
///     exhaustive_rounding_modes().collect_vec(),
///     &[Down, Up, Floor, Ceiling, Nearest, Exact,]
/// );
/// ```
#[inline]
pub fn exhaustive_rounding_modes() -> ExhaustiveRoundingModes {
    ROUNDING_MODES.iter().copied()
}
