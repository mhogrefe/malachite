// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// Compares the absolute value of a finite nonzero `BigExponentFloat`, given by its exponent and
// significand, with a nonzero `Rational`.
//
// The exponents are compared first; only when they agree, which bounds the exponent by the size of
// the `Rational`, is a cross-multiplication formed. The exact exponent comparison decides every
// case the screens of `cmp_product` could.
#[cfg_attr(dylint_lib = "malachite_lints", allow(use_cmp_product))]
pub(crate) fn finite_cmp_abs_nonzero_rational(
    exponent: &Integer,
    significand: &Natural,
    other: &Rational,
) -> Ordering {
    let exp_cmp = exponent
        .partial_cmp(&(other.floor_log_base_2_abs() + 1))
        .unwrap();
    if exp_cmp != Equal {
        return exp_cmp;
    }
    let shift = i64::exact_from(exponent) - i64::exact_from(significand.significant_bits());
    let abs_shift = shift.unsigned_abs();
    match shift.sign() {
        Equal => (significand * other.denominator_ref()).cmp(other.numerator_ref()),
        Greater => {
            ((significand * other.denominator_ref()) << abs_shift).cmp(other.numerator_ref())
        }
        Less => {
            let n_trailing_zeros = significand.trailing_zeros().unwrap();
            if abs_shift <= n_trailing_zeros {
                ((significand >> abs_shift) * other.denominator_ref()).cmp(other.numerator_ref())
            } else {
                ((significand >> n_trailing_zeros) * other.denominator_ref())
                    .cmp(&(other.numerator_ref() << (abs_shift - n_trailing_zeros)))
            }
        }
    }
}

impl PartialOrd<Rational> for BigExponentFloat {
    /// Compares a [`BigExponentFloat`] to a [`Rational`].
    ///
    /// NaN is not comparable to any [`Rational`]. $\infty$ is greater than any [`Rational`], and
    /// $-\infty$ is less. The [`BigExponentFloat`] zero is equal to the [`Rational`] zero. The
    /// comparison is exact, whatever the size of the [`BigExponentFloat`]'s exponent.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity, OneHalf};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    ///
    /// assert!(BigExponentFloat::from(2) > Rational::from_signeds(3, 2));
    /// assert!(BigExponentFloat::ONE_HALF < Rational::from_signeds(2, 3));
    /// assert!(BigExponentFloat::from(-2) < Rational::from_signeds(-3, 2));
    /// assert!(BigExponentFloat::INFINITY > Rational::from(100));
    /// assert!(BigExponentFloat::NEGATIVE_INFINITY < Rational::from(-100));
    /// ```
    fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
        match self {
            Self(NaN) => None,
            Self(Infinity { sign }) => Some(if *sign { Greater } else { Less }),
            Self(Zero) => Some(other.sign().reverse()),
            Self(Finite {
                sign,
                exponent,
                significand,
            }) => {
                let s_x = if *sign { Greater } else { Less };
                let s_cmp = s_x.cmp(&other.sign());
                if s_cmp != Equal {
                    return Some(s_cmp);
                }
                let abs_cmp = finite_cmp_abs_nonzero_rational(exponent, significand, other);
                Some(if *sign { abs_cmp } else { abs_cmp.reverse() })
            }
        }
    }
}

impl PartialOrd<BigExponentFloat> for Rational {
    /// Compares a [`Rational`] to a [`BigExponentFloat`].
    ///
    /// No [`Rational`] is comparable to NaN. Every [`Rational`] is smaller than $\infty$ and
    /// greater than $-\infty$. The [`Rational`] zero is equal to the [`BigExponentFloat`] zero. The
    /// comparison is exact, whatever the size of the [`BigExponentFloat`]'s exponent.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity, OneHalf};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    ///
    /// assert!(Rational::from_signeds(3, 2) < BigExponentFloat::from(2));
    /// assert!(Rational::from_signeds(2, 3) > BigExponentFloat::ONE_HALF);
    /// assert!(Rational::from_signeds(-3, 2) > BigExponentFloat::from(-2));
    /// assert!(Rational::from(100) < BigExponentFloat::INFINITY);
    /// assert!(Rational::from(-100) > BigExponentFloat::NEGATIVE_INFINITY);
    /// ```
    #[inline]
    fn partial_cmp(&self, other: &BigExponentFloat) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}
