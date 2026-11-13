// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use malachite_base::num::arithmetic::traits::{
    Abs, AbsAssign, CanonicalizeUnit, CanonicalizeUnitAssign, IsUnit,
};
use malachite_base::num::basic::traits::One;

impl CanonicalizeUnit for BigExponentFloat {
    type Output = Self;

    /// Brings a [`BigExponentFloat`] into canonical unit form, taking it by value.
    ///
    /// A finite nonzero [`BigExponentFloat`] is a unit, since it has a multiplicative inverse, so
    /// its canonical form is 1. The other values are not units, and their canonical form is their
    /// absolute value: zero stays zero, both infinities become $\infty$, and NaN stays NaN.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity, One};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     BigExponentFloat::from(-1.5).canonicalize_unit(),
    ///     BigExponentFloat::ONE
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::NEGATIVE_INFINITY.canonicalize_unit(),
    ///     BigExponentFloat::INFINITY
    /// );
    /// ```
    #[inline]
    fn canonicalize_unit(mut self) -> Self {
        self.canonicalize_unit_assign();
        self
    }
}

impl CanonicalizeUnit for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Brings a [`BigExponentFloat`] into canonical unit form, taking it by reference.
    ///
    /// A finite nonzero [`BigExponentFloat`] is a unit, since it has a multiplicative inverse, so
    /// its canonical form is 1. The other values are not units, and their canonical form is their
    /// absolute value: zero stays zero, both infinities become $\infty$, and NaN stays NaN.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity, One};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     (&BigExponentFloat::from(-1.5)).canonicalize_unit(),
    ///     BigExponentFloat::ONE
    /// );
    /// assert_eq!(
    ///     (&BigExponentFloat::NEGATIVE_INFINITY).canonicalize_unit(),
    ///     BigExponentFloat::INFINITY
    /// );
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> BigExponentFloat {
        if self.is_unit() {
            BigExponentFloat::ONE
        } else {
            self.abs()
        }
    }
}

impl CanonicalizeUnitAssign for BigExponentFloat {
    /// Brings a [`BigExponentFloat`] into canonical unit form, in place.
    ///
    /// A finite nonzero [`BigExponentFloat`] is a unit, since it has a multiplicative inverse, so
    /// its canonical form is 1. The other values are not units, and their canonical form is their
    /// absolute value: zero stays zero, both infinities become $\infty$, and NaN stays NaN.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(-1.5);
    /// x.canonicalize_unit_assign();
    /// assert_eq!(x, BigExponentFloat::ONE);
    /// ```
    #[inline]
    fn canonicalize_unit_assign(&mut self) {
        if self.is_unit() {
            *self = Self::ONE;
        } else {
            self.abs_assign();
        }
    }
}
