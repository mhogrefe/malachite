// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use malachite_base::num::arithmetic::traits::IsUnit;

impl IsUnit for BigExponentFloat {
    /// Determines whether a [`BigExponentFloat`] is a unit: whether it is finite and nonzero, so
    /// that it has a multiplicative inverse; NaN and the infinities are not units.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::IsUnit;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::from(-1.5).is_unit(), true);
    /// assert_eq!(BigExponentFloat::ZERO.is_unit(), false);
    /// assert_eq!(BigExponentFloat::INFINITY.is_unit(), false);
    /// assert_eq!(BigExponentFloat::NAN.is_unit(), false);
    /// ```
    #[inline]
    fn is_unit(&self) -> bool {
        self.is_finite() && !self.is_zero()
    }
}
