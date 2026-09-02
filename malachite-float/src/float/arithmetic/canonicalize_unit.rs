// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::arithmetic::traits::{
    Abs, AbsAssign, CanonicalizeUnit, CanonicalizeUnitAssign,
};

impl CanonicalizeUnit for Float {
    type Output = Self;

    /// Brings a [`Float`] into canonical unit form, taking it by value. The canonical unit form of
    /// a [`Float`] is its absolute value; negative zero becomes zero and NaN stays NaN.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_base::num::basic::traits::{NegativeZero, Zero};
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// assert_eq!(Float::from(-1.5).canonicalize_unit(), 1.5);
    /// assert_eq!(
    ///     ComparableFloat(Float::NEGATIVE_ZERO.canonicalize_unit()),
    ///     ComparableFloat(Float::ZERO)
    /// );
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> Self {
        self.abs()
    }
}

impl CanonicalizeUnit for &Float {
    type Output = Float;

    /// Brings a [`Float`] into canonical unit form, taking it by reference. The canonical unit form
    /// of a [`Float`] is its absolute value; negative zero becomes zero and NaN stays NaN.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::from(-1.5)).canonicalize_unit(), 1.5);
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> Float {
        self.clone().abs()
    }
}

impl CanonicalizeUnitAssign for Float {
    /// Replaces a [`Float`] with its canonical unit form. The canonical unit form of a [`Float`] is
    /// its absolute value; negative zero becomes zero and NaN stays NaN.
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
    /// assert_eq!(x, 1.5);
    /// ```
    #[inline]
    fn canonicalize_unit_assign(&mut self) {
        self.abs_assign();
    }
}
