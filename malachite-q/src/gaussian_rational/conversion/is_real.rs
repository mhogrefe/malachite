// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_base::num::conversion::traits::IsReal;

impl IsReal for &GaussianRational {
    /// Determines whether a [`GaussianRational`] is a real number: that is, whether its imaginary
    /// part is zero.
    ///
    /// $f(x) = x \in \R$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{I, One, OneHalf, Zero};
    /// use malachite_base::num::conversion::traits::IsReal;
    /// use malachite_q::gaussian_rational::GaussianRational;
    ///
    /// assert_eq!(GaussianRational::ZERO.is_real(), true);
    /// assert_eq!(GaussianRational::ONE.is_real(), true);
    /// assert_eq!(GaussianRational::ONE_HALF.is_real(), true);
    /// assert_eq!(GaussianRational::I.is_real(), false);
    /// ```
    #[inline]
    fn is_real(self) -> bool {
        self.imaginary == 0u32
    }
}
