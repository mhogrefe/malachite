// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use malachite_base::num::conversion::traits::IsReal;

impl IsReal for &BigExponentFloat {
    /// Determines whether a [`BigExponentFloat`] is a real number: whether it is finite.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::num::conversion::traits::IsReal;
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert!(BigExponentFloat::from(Float::from(2.5)).is_real());
    /// assert!(!BigExponentFloat::NAN.is_real());
    /// ```
    #[inline]
    fn is_real(self) -> bool {
        self.is_finite()
    }
}
