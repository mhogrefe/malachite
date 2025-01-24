// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::random::Seed;
use crate::rounding_modes::{RoundingMode, ROUNDING_MODES};
use crate::slices::{random_values_from_slice, RandomValuesFromSlice};
use std::iter::Copied;

/// Uniformly generates random [`RoundingMode`]s.
pub type RandomRoundingModes = Copied<RandomValuesFromSlice<'static, RoundingMode>>;

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
/// use malachite_base::rounding_modes::RoundingMode::*;
///
/// assert_eq!(
///     random_rounding_modes(EXAMPLE_SEED).take(10).collect_vec(),
///     &[Up, Exact, Ceiling, Up, Floor, Nearest, Exact, Up, Floor, Exact]
/// )
/// ```
#[inline]
pub fn random_rounding_modes(seed: Seed) -> RandomRoundingModes {
    random_values_from_slice(seed, &ROUNDING_MODES).copied()
}
