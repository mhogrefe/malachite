// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::conversion::traits::IsInteger;

impl IsInteger for &Natural {
    /// Determines whether a [`Natural`] is an integer. It always returns `true`.
    ///
    /// $f(x) = \textrm{true}$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::num::conversion::traits::IsInteger;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.is_integer(), true);
    /// assert_eq!(Natural::ONE.is_integer(), true);
    /// assert_eq!(Natural::from(100u32).is_integer(), true);
    /// ```
    #[inline]
    fn is_integer(self) -> bool {
        true
    }
}
