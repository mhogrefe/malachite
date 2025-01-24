// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{PowerOf2, RoundToMultipleOfPowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity, Zero};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

impl Float {
    /// Raises 2 to an integer power, returning a [`Float`] with the specified precision and with
    /// the specified rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// returned power is less than, equal to, or greater than the exact power. The ordering is
    /// usually `Equal`, but is `Less` or `Greater` if overflow or underflow occurs.
    ///
    /// $f(k) = 2^k$, and the result has precision `prec`.
    ///
    /// - If `pow` is greater than $2^{30}-2$ and `rm` is `Floor` or `Down`, the largest
    ///   representable `Float` with the given precision is returned.
    /// - If `pow` is greater than $2^{30}-2$ and `rm` is `Ceiling` or `Up`, or `Nearest`, $\infty$
    ///   is returned.
    /// - If `pow` is less than $-2^{30}$ and `rm` is `Floor`, `Down`, or `Nearest`, positive zero
    ///   is returned.
    /// - If `pow` is less than $-2^{30}$ and `rm` is `Ceiling` or `Up`, the smallest positive
    ///   `Float` is returned.
    ///
    /// If you want the behavior of `Nearest` (that is, returning $\infty$ on overflow and positive
    /// zero on underflow), you can use `Float::power_of_2_prec` instead.
    ///
    /// If you need a [`Float`] with precision 1, then the [`PowerOf2`] implementation may be used
    /// instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is exact and `pow` is greater than $2^{30}-2$ or less
    /// than $-2^{30}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::power_of_2_prec_round(0, 1, Nearest);
    /// assert_eq!(p.to_string(), "1.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::power_of_2_prec_round(0, 100, Nearest);
    /// assert_eq!(p.to_string(), "1.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::power_of_2_prec_round(100, 1, Nearest);
    /// assert_eq!(p.to_string(), "1.0e30");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::power_of_2_prec_round(100, 100, Nearest);
    /// assert_eq!(p.to_string(), "1267650600228229401496703205376.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::power_of_2_prec_round(-100, 1, Nearest);
    /// assert_eq!(p.to_string(), "8.0e-31");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::power_of_2_prec_round(-100, 100, Nearest);
    /// assert_eq!(p.to_string(), "7.88860905221011805411728565283e-31");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::power_of_2_prec_round(i64::power_of_2(30) - 1, 10, Floor);
    /// assert_eq!(p.to_string(), "too_big");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_prec_round(i64::power_of_2(30) - 1, 10, Ceiling);
    /// assert_eq!(p.to_string(), "Infinity");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_prec_round(-i64::power_of_2(30) - 1, 10, Floor);
    /// assert_eq!(p.to_string(), "0.0");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::power_of_2_prec_round(-i64::power_of_2(30) - 1, 10, Ceiling);
    /// assert_eq!(p.to_string(), "too_small");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn power_of_2_prec_round(pow: i64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        if let Ok(exponent) = i32::try_from(pow) {
            if let Some(exponent) = exponent.checked_add(1) {
                if (Float::MIN_EXPONENT..=Float::MAX_EXPONENT).contains(&exponent) {
                    return (
                        Float(Finite {
                            sign: true,
                            exponent,
                            precision: prec,
                            significand: Natural::power_of_2(
                                prec.round_to_multiple_of_power_of_2(Limb::LOG_WIDTH, Ceiling)
                                    .0
                                    - 1,
                            ),
                        }),
                        Equal,
                    );
                }
            }
        }
        match (pow > 0, rm) {
            (_, Exact) => panic!("Inexact power_of_2"),
            (true, Ceiling | Up | Nearest) => (Float::INFINITY, Greater),
            (true, _) => (Float::max_finite_value_with_prec(prec), Less),
            (false, Floor | Down | Nearest) => (Float::ZERO, Less),
            (false, Ceiling | Up) => (Float::min_positive_value_prec(prec), Greater),
        }
    }

    /// Raises 2 to an integer power, returning a [`Float`] with the specified precision. An
    /// [`Ordering`] is also returned, indicating whether the returned power is less than, equal to,
    /// or greater than the exact power. The ordering is usually `Equal`, but is `Greater` in the
    /// case of overflow and `Less` in the case of underflow.
    ///
    /// $f(k) = 2^k$, and the result has precision `prec`.
    ///
    /// If `pow` is greater than $2^{30}-2$, $\infty$ is returned. If `pow` is less than $-2^{30}$,
    /// positive zero is returned. If you want different overflow and underflow behavior, try using
    /// `Float::power_of_2_prec_round` instead.
    ///
    /// If you need a [`Float`] with precision 1, then the [`PowerOf2`] implementation may be used
    /// instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::power_of_2_prec(0, 1);
    /// assert_eq!(p.to_string(), "1.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::power_of_2_prec(0, 100);
    /// assert_eq!(p.to_string(), "1.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::power_of_2_prec(100, 1);
    /// assert_eq!(p.to_string(), "1.0e30");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::power_of_2_prec(100, 100);
    /// assert_eq!(p.to_string(), "1267650600228229401496703205376.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::power_of_2_prec(-100, 1);
    /// assert_eq!(p.to_string(), "8.0e-31");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::power_of_2_prec(-100, 100);
    /// assert_eq!(p.to_string(), "7.88860905221011805411728565283e-31");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::power_of_2_prec(i64::power_of_2(30) - 1, 10);
    /// assert_eq!(p.to_string(), "Infinity");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::power_of_2_prec(-i64::power_of_2(30) - 1, 10);
    /// assert_eq!(p.to_string(), "0.0");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn power_of_2_prec(pow: i64, prec: u64) -> (Float, Ordering) {
        Float::power_of_2_prec_round(pow, prec, Nearest)
    }
}

impl PowerOf2<u64> for Float {
    /// Raises 2 to an integer power, returning a [`Float`] with precision 1.
    ///
    /// To get a [`Float`] with a higher precision, try [`Float::power_of_2_prec`].
    ///
    /// $f(k) = 2^k$.
    ///
    /// If `pow` is greater than $2^{30}-2$, $\infty$ is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::power_of_2(0u64).to_string(), "1.0");
    /// assert_eq!(Float::power_of_2(3u64).to_string(), "8.0");
    /// assert_eq!(Float::power_of_2(100u64).to_string(), "1.0e30");
    /// assert_eq!(
    ///     Float::power_of_2(u64::power_of_2(30) - 1).to_string(),
    ///     "Infinity"
    /// );
    /// ```
    fn power_of_2(pow: u64) -> Float {
        if let Ok(exponent) = i32::try_from(pow) {
            if let Some(exponent) = exponent.checked_add(1) {
                if exponent <= Float::MAX_EXPONENT {
                    return Float(Finite {
                        sign: true,
                        exponent,
                        precision: 1,
                        significand: Natural::HIGH_BIT,
                    });
                }
            }
        }
        Float::INFINITY
    }
}

impl PowerOf2<i64> for Float {
    /// Raises 2 to an integer power, returning a [`Float`] with precision 1.
    ///
    /// To get a [`Float`] with a higher precision, try [`Float::power_of_2_prec`].
    ///
    /// $f(k) = 2^k$.
    ///
    /// If `pow` is greater than $2^{30}-2$, $\infty$ is returned. If `pow` is less than $-2^{30}$,
    /// positive zero is returned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::power_of_2(0i64).to_string(), "1.0");
    /// assert_eq!(Float::power_of_2(3i64).to_string(), "8.0");
    /// assert_eq!(Float::power_of_2(100i64).to_string(), "1.0e30");
    /// assert_eq!(Float::power_of_2(-3i64).to_string(), "0.1");
    /// assert_eq!(Float::power_of_2(-100i64).to_string(), "8.0e-31");
    /// assert_eq!(
    ///     Float::power_of_2(i64::power_of_2(30) - 1).to_string(),
    ///     "Infinity"
    /// );
    /// assert_eq!(
    ///     Float::power_of_2(-i64::power_of_2(30) - 1).to_string(),
    ///     "0.0"
    /// );
    /// ```
    #[inline]
    fn power_of_2(pow: i64) -> Float {
        if let Ok(exponent) = i32::try_from(pow) {
            if let Some(exponent) = exponent.checked_add(1) {
                if (Float::MIN_EXPONENT..=Float::MAX_EXPONENT).contains(&exponent) {
                    return Float(Finite {
                        sign: true,
                        exponent,
                        precision: 1,
                        significand: Natural::HIGH_BIT,
                    });
                }
            }
        }
        if pow > 0 {
            Float::INFINITY
        } else {
            Float::ZERO
        }
    }
}
