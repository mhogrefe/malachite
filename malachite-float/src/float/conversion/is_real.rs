// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::conversion::traits::IsReal;

impl IsReal for &Float {
    /// Determines whether a [`Float`] is a real number. `NaN` and the infinities are not real
    /// numbers.
    ///
    /// $f(x) = x \in \R$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, OneHalf, Zero};
    /// use malachite_base::num::conversion::traits::IsReal;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::ZERO.is_real(), true);
    /// assert_eq!(Float::ONE.is_real(), true);
    /// assert_eq!(Float::ONE_HALF.is_real(), true);
    /// assert_eq!(Float::NAN.is_real(), false);
    /// assert_eq!(Float::INFINITY.is_real(), false);
    /// ```
    #[inline]
    fn is_real(self) -> bool {
        self.is_finite()
    }
}
