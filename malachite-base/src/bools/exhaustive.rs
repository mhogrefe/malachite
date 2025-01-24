// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::iter::Copied;
use core::slice::Iter;

/// An iterator that generates both [`bool`]s.
///
/// This `struct` is created by [`exhaustive_bools`]; see its documentation for more.
pub type ExhaustiveBools = Copied<Iter<'static, bool>>;

/// Generates both [`bool`]s.
///
/// The output length is 2.
///
/// # Worst-case complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::bools::exhaustive::exhaustive_bools;
///
/// assert_eq!(exhaustive_bools().collect_vec(), &[false, true]);
/// ```
#[inline]
pub fn exhaustive_bools() -> ExhaustiveBools {
    [false, true].iter().copied()
}
