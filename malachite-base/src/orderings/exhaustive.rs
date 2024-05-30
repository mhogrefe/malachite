// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::orderings::ORDERINGS;
use core::cmp::Ordering::{self, *};
use core::iter::Copied;
use core::slice::Iter;

pub type ExhaustiveOrderings = Copied<Iter<'static, Ordering>>;

/// Generates all [`Ordering`]s, in increasing order.
///
/// The output length is 3.
///
/// # Worst-case complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::orderings::exhaustive::orderings_increasing;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(
///     orderings_increasing().collect_vec(),
///     &[Less, Equal, Greater]
/// );
/// ```
#[inline]
pub fn orderings_increasing() -> ExhaustiveOrderings {
    [Less, Equal, Greater].iter().copied()
}

/// Generates all [`Ordering`]s, with `Equal` coming first.
///
/// The output length is 3.
///
/// # Worst-case complexity per iteration
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use itertools::Itertools;
/// use malachite_base::orderings::exhaustive::exhaustive_orderings;
/// use std::cmp::Ordering::*;
///
/// assert_eq!(
///     exhaustive_orderings().collect_vec(),
///     &[Equal, Less, Greater]
/// );
/// ```
#[inline]
pub fn exhaustive_orderings() -> Copied<Iter<'static, Ordering>> {
    ORDERINGS.iter().copied()
}
