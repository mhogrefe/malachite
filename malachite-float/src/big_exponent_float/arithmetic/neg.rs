// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity};
use core::ops::Neg;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::logic::traits::NotAssign;

impl Neg for BigExponentFloat {
    type Output = Self;

    /// Negates a [`BigExponentFloat`], taking it by value.
    ///
    /// NaN and zero are unchanged; a [`BigExponentFloat`] has no negative zero.
    ///
    /// $$
    /// f(x) = -x.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// This is FLINT's `arf_neg`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert!((-BigExponentFloat::NAN).is_nan());
    /// assert_eq!(
    ///     -BigExponentFloat::INFINITY,
    ///     BigExponentFloat::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(-BigExponentFloat::ZERO, BigExponentFloat::ZERO);
    /// assert_eq!(format!("{:?}", -BigExponentFloat::ONE), "-0x1.0");
    /// assert_eq!(format!("{:?}", -BigExponentFloat::from(-1.5)), "0x1.8");
    /// ```
    #[inline]
    fn neg(mut self) -> Self {
        self.neg_assign();
        self
    }
}

impl Neg for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Negates a [`BigExponentFloat`], taking it by reference.
    ///
    /// NaN and zero are unchanged; a [`BigExponentFloat`] has no negative zero.
    ///
    /// $$
    /// f(x) = -x.
    /// $$
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert!((-&BigExponentFloat::NAN).is_nan());
    /// assert_eq!(
    ///     -&BigExponentFloat::INFINITY,
    ///     BigExponentFloat::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(-&BigExponentFloat::ZERO, BigExponentFloat::ZERO);
    /// assert_eq!(format!("{:?}", -&BigExponentFloat::ONE), "-0x1.0");
    /// assert_eq!(format!("{:?}", -&BigExponentFloat::from(-1.5)), "0x1.8");
    /// ```
    #[inline]
    fn neg(self) -> BigExponentFloat {
        -self.clone()
    }
}

impl NegAssign for BigExponentFloat {
    /// Negates a [`BigExponentFloat`] in place.
    ///
    /// NaN and zero are unchanged; a [`BigExponentFloat`] has no negative zero.
    ///
    /// $$
    /// x \gets -x.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::NegAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity, One, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::INFINITY;
    /// x.neg_assign();
    /// assert_eq!(x, BigExponentFloat::NEGATIVE_INFINITY);
    ///
    /// let mut x = BigExponentFloat::ZERO;
    /// x.neg_assign();
    /// assert_eq!(x, BigExponentFloat::ZERO);
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// x.neg_assign();
    /// assert_eq!(format!("{x:?}"), "-0x1.0");
    /// ```
    fn neg_assign(&mut self) {
        if let Self(Finite { sign, .. } | Infinity { sign }) = self {
            sign.not_assign();
        }
    }
}
