// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity};
use malachite_base::num::arithmetic::traits::{Abs, AbsAssign};

impl Abs for BigExponentFloat {
    type Output = Self;

    /// Takes the absolute value of a [`BigExponentFloat`], taking it by value.
    ///
    /// NaN is unchanged.
    ///
    /// $$
    /// f(x) = |x|.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// This is FLINT's `arf_abs`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Abs;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert!(BigExponentFloat::NAN.abs().is_nan());
    /// assert_eq!(
    ///     BigExponentFloat::NEGATIVE_INFINITY.abs(),
    ///     BigExponentFloat::INFINITY
    /// );
    /// assert_eq!(BigExponentFloat::ZERO.abs(), BigExponentFloat::ZERO);
    /// assert_eq!(format!("{:?}", BigExponentFloat::from(-1.5).abs()), "0x1.8");
    /// assert_eq!(format!("{:?}", BigExponentFloat::from(1.5).abs()), "0x1.8");
    /// ```
    #[inline]
    fn abs(mut self) -> Self {
        self.abs_assign();
        self
    }
}

impl Abs for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Takes the absolute value of a [`BigExponentFloat`], taking it by reference.
    ///
    /// NaN is unchanged.
    ///
    /// $$
    /// f(x) = |x|.
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
    /// use malachite_base::num::arithmetic::traits::Abs;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert!((&BigExponentFloat::NAN).abs().is_nan());
    /// assert_eq!(
    ///     (&BigExponentFloat::NEGATIVE_INFINITY).abs(),
    ///     BigExponentFloat::INFINITY
    /// );
    /// assert_eq!((&BigExponentFloat::ZERO).abs(), BigExponentFloat::ZERO);
    /// assert_eq!(
    ///     format!("{:?}", (&BigExponentFloat::from(-1.5)).abs()),
    ///     "0x1.8"
    /// );
    /// assert_eq!(
    ///     format!("{:?}", (&BigExponentFloat::from(1.5)).abs()),
    ///     "0x1.8"
    /// );
    /// ```
    #[inline]
    fn abs(self) -> BigExponentFloat {
        self.clone().abs()
    }
}

impl AbsAssign for BigExponentFloat {
    /// Replaces a [`BigExponentFloat`] with its absolute value.
    ///
    /// NaN is unchanged.
    ///
    /// $$
    /// x \gets |x|.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::NEGATIVE_INFINITY;
    /// x.abs_assign();
    /// assert_eq!(x, BigExponentFloat::INFINITY);
    ///
    /// let mut x = BigExponentFloat::ZERO;
    /// x.abs_assign();
    /// assert_eq!(x, BigExponentFloat::ZERO);
    ///
    /// let mut x = BigExponentFloat::from(-1.5);
    /// x.abs_assign();
    /// assert_eq!(format!("{x:?}"), "0x1.8");
    /// ```
    fn abs_assign(&mut self) {
        if let Self(Finite { sign, .. } | Infinity { sign }) = self {
            *sign = true;
        }
    }
}
