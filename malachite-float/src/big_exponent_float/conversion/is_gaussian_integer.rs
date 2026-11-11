// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use malachite_base::num::conversion::traits::{IsGaussianInteger, IsInteger};

impl IsGaussianInteger for &BigExponentFloat {
    /// Determines whether a [`BigExponentFloat`] is a Gaussian integer. A [`BigExponentFloat`] is
    /// real, so this is the same as being an integer.
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
    /// use malachite_base::num::conversion::traits::IsGaussianInteger;
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert!(BigExponentFloat::from(Float::from(4.0)).is_gaussian_integer());
    /// assert!(!BigExponentFloat::from(Float::from(2.5)).is_gaussian_integer());
    /// ```
    #[inline]
    fn is_gaussian_integer(self) -> bool {
        self.is_integer()
    }
}
