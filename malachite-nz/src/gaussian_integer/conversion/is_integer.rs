// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use malachite_base::num::conversion::traits::IsInteger;

impl IsInteger for &GaussianInteger {
    /// Determines whether a [`GaussianInteger`] is an integer: that is, whether its imaginary part
    /// is zero.
    ///
    /// $f(x) = x \in \Z$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Zero, I};
    /// use malachite_base::num::conversion::traits::IsInteger;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert_eq!(GaussianInteger::ZERO.is_integer(), true);
    /// assert_eq!(GaussianInteger::ONE.is_integer(), true);
    /// assert_eq!(GaussianInteger::I.is_integer(), false);
    /// assert_eq!(GaussianInteger::from(-100).is_integer(), true);
    /// ```
    #[inline]
    fn is_integer(self) -> bool {
        self.imaginary == 0u32
    }
}
