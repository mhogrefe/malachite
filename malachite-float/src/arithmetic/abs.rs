// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{
    float_either_infinity, float_either_zero, float_infinity, float_nan, float_negative_zero,
    float_zero, Float,
};
use malachite_base::num::arithmetic::traits::{Abs, AbsAssign};

impl Float {
    /// If `self` is negative zero, returns positive zero; otherwise, returns `self`, taking `self`
    /// by value.
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
    /// assert_eq!(
    ///     ComparableFloat(Float::NAN.abs_negative_zero()),
    ///     ComparableFloat(Float::NAN)
    /// );
    /// assert_eq!(Float::INFINITY.abs_negative_zero(), Float::INFINITY);
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY.abs_negative_zero(),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     ComparableFloat(Float::ZERO.abs_negative_zero()),
    ///     ComparableFloat(Float::ZERO)
    /// );
    /// assert_eq!(
    ///     ComparableFloat(Float::NEGATIVE_ZERO.abs_negative_zero()),
    ///     ComparableFloat(Float::ZERO)
    /// );
    /// assert_eq!(Float::ONE.abs_negative_zero(), Float::ONE);
    /// assert_eq!(Float::NEGATIVE_ONE.abs_negative_zero(), Float::NEGATIVE_ONE);
    /// ```
    #[inline]
    pub fn abs_negative_zero(mut self) -> Float {
        self.abs_negative_zero_assign();
        self
    }

    /// If `self` is negative zero, returns positive zero; otherwise, returns `self`, taking `self`
    /// by reference.
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
    /// assert_eq!(
    ///     ComparableFloat(Float::NAN.abs_negative_zero_ref()),
    ///     ComparableFloat(Float::NAN)
    /// );
    /// assert_eq!(Float::INFINITY.abs_negative_zero_ref(), Float::INFINITY);
    /// assert_eq!(
    ///     Float::NEGATIVE_INFINITY.abs_negative_zero_ref(),
    ///     Float::NEGATIVE_INFINITY
    /// );
    /// assert_eq!(
    ///     ComparableFloat(Float::ZERO.abs_negative_zero_ref()),
    ///     ComparableFloat(Float::ZERO)
    /// );
    /// assert_eq!(
    ///     ComparableFloat(Float::NEGATIVE_ZERO.abs_negative_zero_ref()),
    ///     ComparableFloat(Float::ZERO)
    /// );
    /// assert_eq!(Float::ONE.abs_negative_zero_ref(), Float::ONE);
    /// assert_eq!(
    ///     Float::NEGATIVE_ONE.abs_negative_zero_ref(),
    ///     Float::NEGATIVE_ONE
    /// );
    /// ```
    pub fn abs_negative_zero_ref(&self) -> Float {
        match self {
            float_negative_zero!() => float_zero!(),
            x => x.clone(),
        }
    }

    /// If `self` is negative zero, replaces it with positive zero; otherwise, does nothing.
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
    /// let mut x = Float::NAN;
    /// x.abs_negative_zero_assign();
    /// assert_eq!(ComparableFloat(x), ComparableFloat(Float::NAN));
    ///
    /// let mut x = Float::INFINITY;
    /// x.abs_negative_zero_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.abs_negative_zero_assign();
    /// assert_eq!(x, Float::NEGATIVE_INFINITY);
    ///
    /// let mut x = Float::ZERO;
    /// x.abs_negative_zero_assign();
    /// assert_eq!(ComparableFloat(x), ComparableFloat(Float::ZERO));
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.abs_negative_zero_assign();
    /// assert_eq!(ComparableFloat(x), ComparableFloat(Float::ZERO));
    ///
    /// let mut x = Float::ONE;
    /// x.abs_negative_zero_assign();
    /// assert_eq!(x, Float::ONE);
    ///
    /// let mut x = Float::NEGATIVE_ONE;
    /// x.abs_negative_zero_assign();
    /// assert_eq!(x, Float::NEGATIVE_ONE);
    /// ```
    pub fn abs_negative_zero_assign(&mut self) {
        if let Float(Zero { sign }) = self {
            *sign = true;
        }
    }
}

impl Abs for Float {
    type Output = Float;

    /// Takes the absolute value of a [`Float`], taking the [`Float`] by value.
    ///
    /// $$
    /// f(x) = |x|.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN}) = \text{NaN}$
    /// - $f(\infty) = f(-\infty) = \infty$
    /// - $f(0.0) = f(-0.0) = 0.0$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Abs;
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
    /// };
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// assert_eq!(
    ///     ComparableFloat(Float::NAN.abs()),
    ///     ComparableFloat(Float::NAN)
    /// );
    /// assert_eq!(Float::INFINITY.abs(), Float::INFINITY);
    /// assert_eq!(Float::NEGATIVE_INFINITY.abs(), Float::INFINITY);
    /// assert_eq!(
    ///     ComparableFloat(Float::ZERO.abs()),
    ///     ComparableFloat(Float::ZERO)
    /// );
    /// assert_eq!(
    ///     ComparableFloat(Float::NEGATIVE_ZERO.abs()),
    ///     ComparableFloat(Float::ZERO)
    /// );
    /// assert_eq!(Float::ONE.abs(), Float::ONE);
    /// assert_eq!(Float::NEGATIVE_ONE.abs(), Float::ONE);
    /// ```
    #[inline]
    fn abs(mut self) -> Float {
        self.abs_assign();
        self
    }
}

impl<'a> Abs for &'a Float {
    type Output = Float;

    /// Takes the absolute value of a [`Float`], taking the [`Float`] by reference.
    ///
    /// $$
    /// f(x) = |x|.
    /// $$
    ///
    /// Special cases:
    /// - $f(\text{NaN}) = \text{NaN}$
    /// - $f(\infty) = f(-\infty) = \infty$
    /// - $f(0.0) = f(-0.0) = 0.0$
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
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
    /// };
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// assert_eq!(
    ///     ComparableFloat((&Float::NAN).abs()),
    ///     ComparableFloat(Float::NAN)
    /// );
    /// assert_eq!((&Float::INFINITY).abs(), Float::INFINITY);
    /// assert_eq!((&Float::NEGATIVE_INFINITY).abs(), Float::INFINITY);
    /// assert_eq!(
    ///     ComparableFloat((&Float::ZERO).abs()),
    ///     ComparableFloat(Float::ZERO)
    /// );
    /// assert_eq!(
    ///     ComparableFloat((&Float::NEGATIVE_ZERO).abs()),
    ///     ComparableFloat(Float::ZERO)
    /// );
    /// assert_eq!((&Float::ONE).abs(), Float::ONE);
    /// assert_eq!((&Float::NEGATIVE_ONE).abs(), Float::ONE);
    /// ```
    fn abs(self) -> Float {
        match self {
            float_nan!() => float_nan!(),
            float_either_infinity!() => float_infinity!(),
            float_either_zero!() => float_zero!(),
            Float(Finite {
                exponent,
                precision,
                significand,
                ..
            }) => Float(Finite {
                sign: true,
                exponent: *exponent,
                precision: *precision,
                significand: significand.clone(),
            }),
        }
    }
}

impl AbsAssign for Float {
    /// Replaces a [`Float`] with its absolute value.
    ///
    /// $$
    /// x \gets |x|.
    /// $$
    ///
    /// Special cases:
    /// - $\text{NaN} \gets \text{NaN}$
    /// - $\infty \gets \infty$
    /// - $-\infty \gets \infty$
    /// - $0.0 \gets 0.0$
    /// - $-0.0 \gets 0.0$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsAssign;
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
    /// };
    /// use malachite_float::{ComparableFloat, Float};
    ///
    /// let mut x = Float::NAN;
    /// x.abs_assign();
    /// assert_eq!(ComparableFloat(x), ComparableFloat(Float::NAN));
    ///
    /// let mut x = Float::INFINITY;
    /// x.abs_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.abs_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::ZERO;
    /// x.abs_assign();
    /// assert_eq!(ComparableFloat(x), ComparableFloat(Float::ZERO));
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.abs_assign();
    /// assert_eq!(ComparableFloat(x), ComparableFloat(Float::ZERO));
    ///
    /// let mut x = Float::ONE;
    /// x.abs_assign();
    /// assert_eq!(x, Float::ONE);
    ///
    /// let mut x = Float::NEGATIVE_ONE;
    /// x.abs_assign();
    /// assert_eq!(x, Float::ONE);
    /// ```
    fn abs_assign(&mut self) {
        if let Float(Infinity { sign } | Zero { sign } | Finite { sign, .. }) = self {
            *sign = true;
        }
    }
}
