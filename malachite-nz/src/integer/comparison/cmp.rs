// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::cmp::Ordering::{self, *};

impl PartialOrd for Integer {
    /// Compares two [`Integer`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Integer {
    /// Compares two [`Integer`]s.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::from(-123) < Integer::from(-122));
    /// assert!(Integer::from(-123) <= Integer::from(-122));
    /// assert!(Integer::from(-123) > Integer::from(-124));
    /// assert!(Integer::from(-123) >= Integer::from(-124));
    /// ```
    fn cmp(&self, other: &Integer) -> Ordering {
        if core::ptr::eq(self, other) {
            Equal
        } else {
            match (self.sign, other.sign) {
                (true, false) => Greater,
                (false, true) => Less,
                (true, true) => self.abs.cmp(&other.abs),
                (false, false) => other.abs.cmp(&self.abs),
            }
        }
    }
}
