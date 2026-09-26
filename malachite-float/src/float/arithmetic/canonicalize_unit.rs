// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::arithmetic::traits::{
    AbsAssign, CanonicalizeUnit, CanonicalizeUnitAssign, IsUnit,
};

impl CanonicalizeUnit for Float {
    type Output = Self;

    /// Brings a [`Float`] into canonical unit form, taking it by value.
    ///
    /// A finite nonzero [`Float`] is a unit, since it has a multiplicative inverse, so its
    /// canonical form is 1, with the same precision. The other values are not units, and their
    /// canonical form is their absolute value: both zeros become $0.0$, both infinities become
    /// $\infty$, and NaN stays NaN.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(-1.5).canonicalize_unit().to_string(), "1.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.canonicalize_unit().to_string(), "0.0");
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY.canonicalize_unit().to_string(),
    ///     "Infinity"
    /// );
    /// assert_eq!(Float::NAN.canonicalize_unit().to_string(), "NaN");
    /// ```
    #[inline]
    fn canonicalize_unit(mut self) -> Self {
        self.canonicalize_unit_assign();
        self
    }
}

impl CanonicalizeUnit for &Float {
    type Output = Float;

    /// Brings a [`Float`] into canonical unit form, taking it by reference.
    ///
    /// A finite nonzero [`Float`] is a unit, since it has a multiplicative inverse, so its
    /// canonical form is 1, with the same precision. The other values are not units, and their
    /// canonical form is their absolute value: both zeros become $0.0$, both infinities become
    /// $\infty$, and NaN stays NaN.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::from(-1.5)).canonicalize_unit().to_string(), "1.0");
    /// assert_eq!(
    ///     (&Float::NEGATIVE_ZERO).canonicalize_unit().to_string(),
    ///     "0.0"
    /// );
    /// assert_eq!(
    ///     (&Float::NEGATIVE_INFINITY).canonicalize_unit().to_string(),
    ///     "Infinity"
    /// );
    /// assert_eq!((&Float::NAN).canonicalize_unit().to_string(), "NaN");
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> Float {
        if self.is_unit() {
            Float::one_prec(self.get_prec().unwrap())
        } else {
            self.clone().canonicalize_unit()
        }
    }
}

impl CanonicalizeUnitAssign for Float {
    /// Replaces a [`Float`] with its canonical unit form.
    ///
    /// A finite nonzero [`Float`] is a unit, since it has a multiplicative inverse, so its
    /// canonical form is 1, with the same precision. The other values are not units, and their
    /// canonical form is their absolute value: both zeros become $0.0$, both infinities become
    /// $\infty$, and NaN stays NaN.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(-1.5);
    /// x.canonicalize_unit_assign();
    /// assert_eq!(x.to_string(), "1.0");
    /// ```
    #[inline]
    fn canonicalize_unit_assign(&mut self) {
        if self.is_unit() {
            *self = Self::one_prec(self.get_prec().unwrap());
        } else {
            self.abs_assign();
        }
    }
}
