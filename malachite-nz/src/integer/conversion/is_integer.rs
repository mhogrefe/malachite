// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::conversion::traits::IsInteger;

impl IsInteger for &Integer {
    /// Determines whether an [`Integer`] is an integer. It always returns `true`.
    ///
    /// $f(x) = \textrm{true}$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_base::num::conversion::traits::IsInteger;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.is_integer(), true);
    /// assert_eq!(Integer::ONE.is_integer(), true);
    /// assert_eq!(Integer::from(100).is_integer(), true);
    /// assert_eq!(Integer::NEGATIVE_ONE.is_integer(), true);
    /// assert_eq!(Integer::from(-100).is_integer(), true);
    /// ```
    #[inline]
    fn is_integer(self) -> bool {
        true
    }
}
