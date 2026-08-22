// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::{Float, emulate_constant_to_float_fn};
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::SaturatingMulAssign;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, Nearest};

// The digits of Liouville's constant in the given base: 1 at every position that is a factorial, 0
// everywhere else. The positions are 1-indexed, so both 1! and 2! contribute a 1, making the
// constant begin 0.11000100....
//
// The multiplication saturates rather than overflowing, which cannot be observed: 21! already
// exceeds a `u64`, and reaching that position would mean reading more digits than any precision
// could ask for.
fn liouvilles_digits() -> impl Iterator<Item = u64> {
    let mut position = 0u64;
    let mut factorial = 1u64;
    let mut index = 1u64;
    core::iter::from_fn(move || {
        position += 1;
        Some(if position == factorial {
            index += 1;
            factorial.saturating_mul_assign(index);
            1
        } else {
            0
        })
    })
}

impl Float {
    /// Returns an approximation of Liouville's constant in a given base, with the given precision
    /// and rounded using the given [`RoundingMode`]. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than or greater than the exact value of the constant.
    /// (Since the constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// Liouville's constant in base $b$ has a digit of 1 at every position that is a factorial and
    /// a digit of 0 everywhere else. That is,
    /// $$
    /// L_b = \sum_{n=1}^\infty b^{-n!}+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 L_b\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 L_b\rfloor-p}$.
    ///
    /// Base 10 gives the classical constant, $0.110001000000000000000001\ldots$, the first number
    /// proven transcendental. The constant is transcendental in every base, being a Liouville
    /// number: its factorial-spaced digits make it approximable by rationals far too well for an
    /// algebraic number.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2, if `prec` is zero, or if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // In base 2 the constant has a 1 at every factorial position -- 1, 2, 6, 24, ... -- and a
    /// // 0 everywhere else, which the binary representation shows directly.
    /// let (x, o) = Float::liouvilles_constant_base_prec_round(2, 64, Floor);
    /// assert_eq!(x.to_string(), "0.765625059604644775391");
    /// assert_eq!(
    ///     format!("{x:#b}"),
    ///     "0b0.1100010000000000000000010000000000000000000000000000000000000000"
    /// );
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::liouvilles_constant_base_prec_round(2, 64, Ceiling);
    /// assert_eq!(x.to_string(), "0.765625059604644775445");
    /// assert_eq!(
    ///     format!("{x:#b}"),
    ///     "0b0.1100010000000000000000010000000000000000000000000000000000000001"
    /// );
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn liouvilles_constant_base_prec_round(
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::non_dyadic_from_digits_prec_round(liouvilles_digits(), base, prec, rm)
    }

    /// Returns an approximation of Liouville's constant in a given base, with the given precision
    /// and rounded to the nearest [`Float`] of that precision. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than or greater than the exact value.
    ///
    /// See [`liouvilles_constant_base_prec_round`](Float::liouvilles_constant_base_prec_round) for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // In base 2 the 1s sit at the factorial positions 1, 2, 6, 24, ...
    /// let (x, o) = Float::liouvilles_constant_base_prec(2, 64);
    /// assert_eq!(x.to_string(), "0.765625059604644775391");
    /// assert_eq!(
    ///     format!("{x:#b}"),
    ///     "0b0.1100010000000000000000010000000000000000000000000000000000000000"
    /// );
    /// assert_eq!(o, Less);
    ///
    /// // A base that is not a power of 2 takes the general path, where the digits cannot simply
    /// // be read off.
    /// let (x, o) = Float::liouvilles_constant_base_prec(3, 50);
    /// assert_eq!(x.to_string(), "0.44581618656046818");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn liouvilles_constant_base_prec(base: u64, prec: u64) -> (Self, Ordering) {
        Self::liouvilles_constant_base_prec_round(base, prec, Nearest)
    }

    /// Returns an approximation of Liouville's constant in base 10, with the given precision and
    /// rounded using the given [`RoundingMode`]. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than or greater than the exact value of the constant.
    /// (Since the constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// Liouville's constant has a decimal digit of 1 at every position that is a factorial and a
    /// digit of 0 everywhere else, so it begins $0.110001000000000000000001\ldots$.
    ///
    /// $$
    /// x = L = \sum_{n=1}^{\infty} 10^{-n!}+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p-1}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The output has precision `prec`.
    ///
    /// This is the base-10 specialization of
    /// [`liouvilles_constant_base_prec_round`](Float::liouvilles_constant_base_prec_round).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::liouvilles_constant_prec_round(100, Floor);
    /// assert_eq!(x.to_string(), "0.11000100000000000000000099999997");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::liouvilles_constant_prec_round(100, Ceiling);
    /// assert_eq!(x.to_string(), "0.11000100000000000000000100000007");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn liouvilles_constant_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::liouvilles_constant_base_prec_round(10, prec, rm)
    }

    /// Returns an approximation of Liouville's constant in base 10, with the given precision and
    /// rounded to the nearest [`Float`] of that precision. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than or greater than the exact value of the
    /// constant. (Since the constant is irrational, the rounded value is never equal to the exact
    /// value.)
    ///
    /// Liouville's constant has a decimal digit of 1 at every position that is a factorial and a
    /// digit of 0 everywhere else, so it begins $0.110001000000000000000001\ldots$.
    ///
    /// $$
    /// x = L = \sum_{n=1}^{\infty} 10^{-n!}+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p-1}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The output has precision `prec`.
    ///
    /// This is the base-10 specialization of
    /// [`liouvilles_constant_base_prec`](Float::liouvilles_constant_base_prec).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let x = Float::liouvilles_constant_prec(100).0;
    /// assert_eq!(x.to_string(), "0.11000100000000000000000099999997");
    /// ```
    #[inline]
    pub fn liouvilles_constant_prec(prec: u64) -> (Self, Ordering) {
        Self::liouvilles_constant_base_prec(10, prec)
    }
}

/// Computes an approximation of Liouville's constant in a given base, returning a primitive float.
///
/// Liouville's constant in base $b$ has a digit of 1 at every position that is a factorial and a
/// digit of 0 everywhere else.
///
/// $$
/// L_b = \sum_{n=1}^{\infty} b^{-n!}.
/// $$
///
/// The returned value is the one closest to the true constant; ties are broken by the
/// round-half-to-even rule. Computing the constant this way is more accurate than summing its
/// digits in primitive-float arithmetic, where each addition rounds.
///
/// $$
/// f(b) = L_b+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |L_b|\rfloor-p}$ and $p$ is the precision of the output
/// (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// The constant lies in $[1/b,1)$, and $b$ is at most $2^{64}-1$, so this function can neither
/// overflow nor underflow.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Panics
/// Panics if `base` is less than 2.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::constants::liouvilles_constant::*;
///
/// // The classical constant, 0.110001000000000000000001...
/// assert_eq!(
///     NiceFloat(primitive_float_liouvilles_constant_base::<f32>(10)),
///     NiceFloat(0.110001)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_liouvilles_constant_base::<f64>(10)),
///     NiceFloat(0.110001)
/// );
/// // In base 2 the constant is 0.11000100000000000000000100...
/// assert_eq!(
///     NiceFloat(primitive_float_liouvilles_constant_base::<f64>(2)),
///     NiceFloat(0.7656250596046448)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_liouvilles_constant_base<T: PrimitiveFloat>(base: u64) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_constant_to_float_fn(|prec| Float::liouvilles_constant_base_prec(base, prec))
}
