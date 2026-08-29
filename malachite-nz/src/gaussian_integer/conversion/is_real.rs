// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use malachite_base::num::conversion::traits::IsReal;

impl IsReal for &GaussianInteger {
    /// Determines whether a [`GaussianInteger`] is a real number: that is, whether its imaginary
    /// part is zero.
    ///
    /// $f(x) = x \in \R$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Zero, I};
    /// use malachite_base::num::conversion::traits::IsReal;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert_eq!(GaussianInteger::ZERO.is_real(), true);
    /// assert_eq!(GaussianInteger::ONE.is_real(), true);
    /// assert_eq!(GaussianInteger::I.is_real(), false);
    /// assert_eq!(GaussianInteger::from(-100).is_real(), true);
    /// ```
    #[inline]
    fn is_real(self) -> bool {
        self.imaginary == 0u32
    }
}
