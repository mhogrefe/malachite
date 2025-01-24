// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{significand_bits, Float};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_q::Rational;

impl PartialOrd<Rational> for Float {
    /// Compares a [`Float`] to a [`Rational`].
    ///
    /// NaN is not comparable to any [`Rational`]. $\infty$ is greater than any [`Rational`], and
    /// $-\infty$ is less. Both the [`Float`] zero and the [`Float`] negative zero are equal to the
    /// [`Rational`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!(Float::from(80) < Rational::from(100));
    /// assert!(Float::from(-80) > Rational::from(-100));
    /// assert!(Float::INFINITY > Rational::from(100));
    /// assert!(Float::NEGATIVE_INFINITY < Rational::from(-100));
    /// assert!(Float::from(1.0f64 / 3.0) < Rational::from_unsigneds(1u8, 3));
    /// ```
    fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
        match (self, other) {
            (float_nan!(), _) => None,
            (float_infinity!(), _) => Some(Greater),
            (float_negative_infinity!(), _) => Some(Less),
            (float_either_zero!(), y) => 0u32.partial_cmp(y),
            (
                Float(Finite {
                    sign: s_x,
                    exponent: e_x,
                    significand: significand_x,
                    ..
                }),
                y,
            ) => Some(if *y == 0u32 {
                if *s_x {
                    Greater
                } else {
                    Less
                }
            } else {
                let s_cmp = s_x.cmp(&(*y > 0));
                if s_cmp != Equal {
                    return Some(s_cmp);
                }
                let e_x = i64::from(*e_x);
                let exp_cmp = (e_x - 1).cmp(&other.floor_log_base_2_abs());
                if exp_cmp != Equal {
                    return Some(if *s_x { exp_cmp } else { exp_cmp.reverse() });
                }
                let shift = e_x - i64::exact_from(significand_bits(significand_x));
                let abs_shift = shift.unsigned_abs();
                let prod_cmp = match shift.sign() {
                    Equal => (significand_x * other.denominator_ref()).cmp(other.numerator_ref()),
                    Greater => ((significand_x * other.denominator_ref()) << abs_shift)
                        .cmp(other.numerator_ref()),
                    Less => {
                        let n_trailing_zeros = significand_x.trailing_zeros().unwrap();
                        if abs_shift <= n_trailing_zeros {
                            ((significand_x >> abs_shift) * other.denominator_ref())
                                .cmp(other.numerator_ref())
                        } else {
                            ((significand_x >> n_trailing_zeros) * other.denominator_ref())
                                .cmp(&(other.numerator_ref() << (abs_shift - n_trailing_zeros)))
                        }
                    }
                };
                if *s_x {
                    prod_cmp
                } else {
                    prod_cmp.reverse()
                }
            }),
        }
    }
}

impl PartialOrd<Float> for Rational {
    /// Compares an [`Rational`] to a [`Float`].
    ///
    /// No [`Rational`] is comparable to NaN. Every [`Rational`] is smaller than $\infty$ and
    /// greater than $-\infty$. The [`Rational`] zero is equal to both the [`Float`] zero and the
    /// [`Float`] negative zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert!(Rational::from(100) > Float::from(80));
    /// assert!(Rational::from(-100) < Float::from(-80));
    /// assert!(Rational::from(100) < Float::INFINITY);
    /// assert!(Rational::from(-100) > Float::NEGATIVE_INFINITY);
    /// assert!(Rational::from_unsigneds(1u8, 3) > Float::from(1.0f64 / 3.0));
    /// ```
    #[inline]
    fn partial_cmp(&self, other: &Float) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}
