// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_base::num::conversion::traits::IsInteger;

impl IsInteger for &GaussianRational {
    /// Determines whether a [`GaussianRational`] is an integer: that is, whether its imaginary part
    /// is zero and its real part is an integer.
    ///
    /// $f(x) = x \in \Z$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{I, One, OneHalf, Zero};
    /// use malachite_base::num::conversion::traits::IsInteger;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(GaussianRational::ZERO.is_integer(), true);
    /// assert_eq!(GaussianRational::ONE.is_integer(), true);
    /// assert_eq!(GaussianRational::I.is_integer(), false);
    /// assert_eq!(GaussianRational::ONE_HALF.is_integer(), false);
    /// ```
    #[inline]
    fn is_integer(self) -> bool {
        self.imaginary == 0u32 && self.real.is_integer()
    }
}
