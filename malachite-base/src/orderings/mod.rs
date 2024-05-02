// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering;

pub(crate) const ORDERINGS: [Ordering; 3] = [Ordering::Equal, Ordering::Less, Ordering::Greater];

/// Converts a string to an [`Ordering`].
///
/// If the string does not represent a valid [`Ordering`], `None` is returned.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::orderings::ordering_from_str;
/// use std::cmp::Ordering;
///
/// assert_eq!(ordering_from_str("Equal"), Some(Ordering::Equal));
/// assert_eq!(ordering_from_str("Less"), Some(Ordering::Less));
/// assert_eq!(ordering_from_str("Greater"), Some(Ordering::Greater));
/// assert_eq!(ordering_from_str("abc"), None);
/// ```
#[inline]
pub fn ordering_from_str(src: &str) -> Option<Ordering> {
    match src {
        "Equal" => Some(Ordering::Equal),
        "Less" => Some(Ordering::Less),
        "Greater" => Some(Ordering::Greater),
        _ => None,
    }
}

/// Iterators that generate [`Ordering`]s without repetition.
pub mod exhaustive;
#[cfg(feature = "random")]
/// Iterators that generate [`Ordering`]s randomly.
pub mod random;
