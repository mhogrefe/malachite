// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Zero};
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::CheckedLogBase2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;

impl PartialEq<Rational> for Float {
    /// Determines whether a [`Float`] is equal to a [`Rational`].
    ///
    /// $\infty$, $-\infty$, and NaN are not equal to any [`Rational`]. Both the [`Float`] zero and
    /// the [`Float`] negative zero are equal to the [`Rational`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!(Float::from(123) == Rational::from(123));
    /// assert!(Float::from(-123) == Rational::from(-123));
    /// assert!(Float::ONE_HALF == Rational::ONE_HALF);
    /// assert!(Float::from(1.0f64 / 3.0) != Rational::from_unsigneds(1u8, 3));
    /// ```
    fn eq(&self, other: &Rational) -> bool {
        match self {
            float_either_zero!() => *other == 0u32,
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                *other != 0
                    && *sign == (*other > 0)
                    && if let Some(log_d) = other.denominator_ref().checked_log_base_2() {
                        let n = other.numerator_ref();
                        i64::from(*exponent)
                            == i64::exact_from(n.significant_bits()) - i64::exact_from(log_d)
                            && significand.cmp_normalized(n) == Equal
                    } else {
                        false
                    }
            }
            _ => false,
        }
    }
}

impl PartialEq<Float> for Rational {
    /// Determines whether a [`Rational`] is equal to a [`Float`].
    ///
    /// No [`Rational`] is equal to $\infty$, $-\infty$, or NaN. The [`Rational`] zero is equal to
    /// both the [`Float`] zero and the [`Float`] negative zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!(Rational::from(123) == Float::from(123));
    /// assert!(Rational::from(-123) == Float::from(-123));
    /// assert!(Rational::ONE_HALF == Float::ONE_HALF);
    /// assert!(Rational::from_unsigneds(1u8, 3) != Float::from(1.0f64 / 3.0));
    /// ```
    #[inline]
    fn eq(&self, other: &Float) -> bool {
        other == self
    }
}
