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
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};

impl EqAbs<Float> for BigExponentFloat {
    /// Determines whether the absolute values of a [`BigExponentFloat`] and a [`Float`] are equal.
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NegativeInfinity, NegativeOne, One, OneHalf,
    /// };
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert!(BigExponentFloat::NEGATIVE_ONE.eq_abs(&Float::one_prec(100)));
    /// assert!(BigExponentFloat::INFINITY.eq_abs(&Float::NEGATIVE_INFINITY));
    /// assert!(BigExponentFloat::ONE_HALF.ne_abs(&Float::ONE));
    /// ```
    #[inline]
    fn eq_abs(&self, other: &Float) -> bool {
        self.partial_cmp_abs(other) == Some(Equal)
    }
}

impl EqAbs<BigExponentFloat> for Float {
    /// Determines whether the absolute values of a [`Float`] and a [`BigExponentFloat`] are equal.
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NegativeInfinity, NegativeOne, One, OneHalf,
    /// };
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert!(Float::one_prec(100).eq_abs(&BigExponentFloat::NEGATIVE_ONE));
    /// assert!(Float::NEGATIVE_INFINITY.eq_abs(&BigExponentFloat::INFINITY));
    /// assert!(Float::ONE.ne_abs(&BigExponentFloat::ONE_HALF));
    /// ```
    #[inline]
    fn eq_abs(&self, other: &BigExponentFloat) -> bool {
        other.eq_abs(self)
    }
}
