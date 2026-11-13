// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use malachite_base::num::arithmetic::traits::CanonicalUnitIPow;

impl CanonicalUnitIPow for BigExponentFloat {
    /// Finds the power of $i$ that brings a [`BigExponentFloat`] into canonical Gaussian-unit form.
    /// The canonical Gaussian-unit form of a [`BigExponentFloat`] is its absolute value, so this is
    /// 2 for negative values, negative infinity included, since $x i^2 = -x$, and 0 otherwise, NaN
    /// included.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalUnitIPow;
    /// use malachite_base::num::basic::traits::{NaN, NegativeInfinity, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::from(-1.5).canonical_unit_i_pow(), 2);
    /// assert_eq!(
    ///     BigExponentFloat::NEGATIVE_INFINITY.canonical_unit_i_pow(),
    ///     2
    /// );
    /// assert_eq!(BigExponentFloat::from(1.5).canonical_unit_i_pow(), 0);
    /// assert_eq!(BigExponentFloat::ZERO.canonical_unit_i_pow(), 0);
    /// assert_eq!(BigExponentFloat::NAN.canonical_unit_i_pow(), 0);
    /// ```
    #[inline]
    fn canonical_unit_i_pow(&self) -> u64 {
        if self.is_sign_negative() { 2 } else { 0 }
    }
}
