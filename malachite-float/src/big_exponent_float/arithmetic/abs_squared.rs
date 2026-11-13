// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::arithmetic::square::square_helper;
use alloc::borrow::Cow;
use malachite_base::num::arithmetic::traits::{AbsSquared, AbsSquaredAssign};

impl AbsSquared for BigExponentFloat {
    type Output = Self;

    /// Computes the squared absolute value of a [`BigExponentFloat`], taking it by value. For real
    /// types this is the same as squaring, and, like
    /// [`Square`](malachite_base::num::arithmetic::traits::Square), the result is exact.
    ///
    /// $$
    /// f(x) = |x|^2 = x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsSquared;
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::from(-1.5).abs_squared()),
    ///     "0x2.4"
    /// );
    /// assert_eq!(
    ///     BigExponentFloat::NEGATIVE_INFINITY.abs_squared(),
    ///     BigExponentFloat::INFINITY
    /// );
    /// ```
    #[inline]
    fn abs_squared(self) -> Self {
        square_helper(Cow::Owned(self), None).0
    }
}

impl AbsSquared for &BigExponentFloat {
    type Output = BigExponentFloat;

    /// Computes the squared absolute value of a [`BigExponentFloat`], taking it by reference. For
    /// real types this is the same as squaring, and, like
    /// [`Square`](malachite_base::num::arithmetic::traits::Square), the result is exact.
    ///
    /// $$
    /// f(x) = |x|^2 = x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsSquared;
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!("{:?}", (&BigExponentFloat::from(-1.5)).abs_squared()),
    ///     "0x2.4"
    /// );
    /// assert_eq!(
    ///     (&BigExponentFloat::NEGATIVE_INFINITY).abs_squared(),
    ///     BigExponentFloat::INFINITY
    /// );
    /// ```
    #[inline]
    fn abs_squared(self) -> BigExponentFloat {
        square_helper(Cow::Borrowed(self), None).0
    }
}

impl AbsSquaredAssign for BigExponentFloat {
    /// Replaces a [`BigExponentFloat`] with its squared absolute value. For real types this is the
    /// same as squaring, and the result is exact.
    ///
    /// $$
    /// x \gets |x|^2 = x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsSquaredAssign;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let mut x = BigExponentFloat::from(-1.5);
    /// x.abs_squared_assign();
    /// assert_eq!(format!("{x:?}"), "0x2.4");
    /// ```
    #[inline]
    fn abs_squared_assign(&mut self) {
        let x = core::mem::take(self);
        *self = square_helper(Cow::Owned(x), None).0;
    }
}
