// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Zero};
use crate::big_exponent_float::comparison::partial_eq_rational::finite_eq_abs_nonzero_rational;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_q::Rational;

impl EqAbs<Rational> for BigExponentFloat {
    /// Determines whether the absolute values of a [`BigExponentFloat`] and a [`Rational`] are
    /// equal.
    ///
    /// $\infty$, $-\infty$, and NaN are not equal to any [`Rational`] in absolute value. The
    /// [`BigExponentFloat`] zero is equal to the [`Rational`] zero.
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
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    ///
    /// assert!(BigExponentFloat::from(-2).eq_abs(&Rational::from(2)));
    /// assert!(BigExponentFloat::from(1.5).eq_abs(&Rational::from_signeds(-3, 2)));
    /// assert!(BigExponentFloat::ONE_HALF.ne_abs(&Rational::from_signeds(1, 3)));
    /// ```
    fn eq_abs(&self, other: &Rational) -> bool {
        match self {
            Self(Zero) => *other == 0u32,
            Self(Finite {
                exponent,
                significand,
                ..
            }) => *other != 0u32 && finite_eq_abs_nonzero_rational(exponent, significand, other),
            _ => false,
        }
    }
}

impl EqAbs<BigExponentFloat> for Rational {
    /// Determines whether the absolute values of a [`Rational`] and a [`BigExponentFloat`] are
    /// equal.
    ///
    /// No [`Rational`] is equal to $\infty$, $-\infty$, or NaN in absolute value. The [`Rational`]
    /// zero is equal to the [`BigExponentFloat`] zero.
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
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    ///
    /// assert!(Rational::from(2).eq_abs(&BigExponentFloat::from(-2)));
    /// assert!(Rational::from_signeds(-3, 2).eq_abs(&BigExponentFloat::from(1.5)));
    /// assert!(Rational::from_signeds(1, 3).ne_abs(&BigExponentFloat::ONE_HALF));
    /// ```
    #[inline]
    fn eq_abs(&self, other: &BigExponentFloat) -> bool {
        other.eq_abs(self)
    }
}
