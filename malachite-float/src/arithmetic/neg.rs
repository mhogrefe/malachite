// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::float_nan;
use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use core::ops::Neg;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::logic::traits::NotAssign;

impl Neg for Float {
    type Output = Float;

    /// Negates a [`Float`], taking it by value.
    ///
    /// $$
    /// f(x) = -x.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN}) = \text{NaN}$
    /// - $f(\infty) = -\infty$
    /// - $f(-\infty) = \infty$
    /// - $f(0.0) = -0.0$
    /// - $f(-0.0) = 0.0$
    ///
    /// This function does not overflow or underflow.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
    /// };
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// assert_eq!(ComparableFloat(-Float::NAN), ComparableFloat(Float::NAN));
    /// assert_eq!(-Float::INFINITY, Float::NEGATIVE_INFINITY);
    /// assert_eq!(-Float::NEGATIVE_INFINITY, Float::INFINITY);
    /// assert_eq!(
    ///     ComparableFloat(-Float::ZERO),
    ///     ComparableFloat(Float::NEGATIVE_ZERO)
    /// );
    /// assert_eq!(
    ///     ComparableFloat(-Float::NEGATIVE_ZERO),
    ///     ComparableFloat(Float::ZERO)
    /// );
    /// assert_eq!(-Float::ONE, Float::NEGATIVE_ONE);
    /// assert_eq!(-Float::NEGATIVE_ONE, Float::ONE);
    /// ```
    #[inline]
    fn neg(mut self) -> Float {
        self.neg_assign();
        self
    }
}

impl Neg for &Float {
    type Output = Float;

    /// Negates a [`Float`], taking it by reference.
    ///
    /// $$
    /// f(x) = -x.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN}) = \text{NaN}$
    /// - $f(\infty) = -\infty$
    /// - $f(-\infty) = \infty$
    /// - $f(0.0) = -0.0$
    /// - $f(-0.0) = 0.0$
    ///
    /// This function does not overflow or underflow.
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
    /// };
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// assert_eq!(ComparableFloat(-&Float::NAN), ComparableFloat(Float::NAN));
    /// assert_eq!(-&Float::INFINITY, Float::NEGATIVE_INFINITY);
    /// assert_eq!(-&Float::NEGATIVE_INFINITY, Float::INFINITY);
    /// assert_eq!(
    ///     ComparableFloat(-&Float::ZERO),
    ///     ComparableFloat(Float::NEGATIVE_ZERO)
    /// );
    /// assert_eq!(
    ///     ComparableFloat(-&Float::NEGATIVE_ZERO),
    ///     ComparableFloat(Float::ZERO)
    /// );
    /// assert_eq!(-&Float::ONE, Float::NEGATIVE_ONE);
    /// assert_eq!(-&Float::NEGATIVE_ONE, Float::ONE);
    /// ```
    fn neg(self) -> Float {
        match self {
            float_nan!() => float_nan!(),
            Float(Infinity { sign }) => Float(Infinity { sign: !*sign }),
            Float(Zero { sign }) => Float(Zero { sign: !*sign }),
            Float(Finite {
                sign,
                exponent,
                precision,
                significand,
            }) => Float(Finite {
                sign: !*sign,
                exponent: *exponent,
                precision: *precision,
                significand: significand.clone(),
            }),
        }
    }
}

impl NegAssign for Float {
    /// Negates a [`Float`] in place.
    ///
    /// $$
    /// x \gets -x.
    /// $$
    ///
    /// Special cases:
    /// - $\text{NaN} \gets \text{NaN}$
    /// - $\infty \gets -\infty$
    /// - $-\infty \gets \infty$
    /// - $0.0 \gets -0.0$
    /// - $-0.0 \gets 0.0$
    ///
    /// This function does not overflow or underflow.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::NegAssign;
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
    /// };
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// let mut x = Float::NAN;
    /// x.neg_assign();
    /// assert_eq!(ComparableFloat(x), ComparableFloat(Float::NAN));
    ///
    /// let mut x = Float::INFINITY;
    /// x.neg_assign();
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.neg_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::ZERO;
    /// x.neg_assign();
    /// assert_eq!(ComparableFloat(x), ComparableFloat(Float::NEGATIVE_ZERO));
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.neg_assign();
    /// assert_eq!(ComparableFloat(x), ComparableFloat(Float::ZERO));
    ///
    /// let mut x = Float::ONE;
    /// x.neg_assign();
    /// assert_eq!(x, Float::NEGATIVE_ONE);
    ///
    /// let mut x = Float::NEGATIVE_ONE;
    /// x.neg_assign();
    /// assert_eq!(x, Float::ONE);
    /// ```
    fn neg_assign(&mut self) {
        if let Float(Infinity { sign } | Zero { sign } | Finite { sign, .. }) = self {
            sign.not_assign();
        }
    }
}
