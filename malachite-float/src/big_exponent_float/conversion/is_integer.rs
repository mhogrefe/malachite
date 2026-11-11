// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::big_exponent_float::BigExponentFloat;
use malachite_base::num::conversion::traits::{ConvertibleFrom, IsInteger};
use malachite_nz::integer::Integer;

impl IsInteger for &BigExponentFloat {
    /// Determines whether a [`BigExponentFloat`] is an integer. NaN and $\pm\infty$ are not.
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
    /// use malachite_base::num::conversion::traits::IsInteger;
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert!(BigExponentFloat::from(Float::from(4.0)).is_integer());
    /// assert!(!BigExponentFloat::from(Float::from(2.5)).is_integer());
    /// ```
    #[inline]
    fn is_integer(self) -> bool {
        Integer::convertible_from(self)
    }
}
