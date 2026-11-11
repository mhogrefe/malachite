// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering::*;

impl PartialEq<Float> for BigExponentFloat {
    /// Determines whether a [`BigExponentFloat`] is equal to a [`Float`].
    ///
    /// This implementation follows the IEEE 754 standard: NaN is not equal to anything. A
    /// [`BigExponentFloat`] has no stored precision and a [`Float`] does; values with different
    /// precisions are equal if they represent the same number. Both [`Float`] zeros are equal to
    /// the [`BigExponentFloat`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, One, OneHalf, Zero};
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert!(BigExponentFloat::ONE == Float::one_prec(100));
    /// assert!(BigExponentFloat::ZERO == Float::NEGATIVE_ZERO);
    /// assert!(BigExponentFloat::ONE_HALF != Float::ONE);
    /// assert!(BigExponentFloat::NAN != Float::NAN);
    /// ```
    #[inline]
    fn eq(&self, other: &Float) -> bool {
        self.partial_cmp(other) == Some(Equal)
    }
}

impl PartialEq<BigExponentFloat> for Float {
    /// Determines whether a [`Float`] is equal to a [`BigExponentFloat`].
    ///
    /// This implementation follows the IEEE 754 standard: NaN is not equal to anything. A
    /// [`BigExponentFloat`] has no stored precision and a [`Float`] does; values with different
    /// precisions are equal if they represent the same number. Both [`Float`] zeros are equal to
    /// the [`BigExponentFloat`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, One, OneHalf, Zero};
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert!(Float::one_prec(100) == BigExponentFloat::ONE);
    /// assert!(Float::NEGATIVE_ZERO == BigExponentFloat::ZERO);
    /// assert!(Float::ONE != BigExponentFloat::ONE_HALF);
    /// assert!(Float::NAN != BigExponentFloat::NAN);
    /// ```
    #[inline]
    fn eq(&self, other: &BigExponentFloat) -> bool {
        other == self
    }
}
