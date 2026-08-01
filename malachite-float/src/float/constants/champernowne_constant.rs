// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::{Float, emulate_constant_to_float_fn};
use core::cmp::Ordering;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{Digits, ExactFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, Nearest};

// The digits of the Champernowne constant in the given base: the base-`base` representations of 1,
// 2, 3, ... run together. A `u64` counter is inexhaustible here, since the digits contributed by
// the first n integers grow faster than n.
fn champernowne_digits(base: u64) -> impl Iterator<Item = u64> {
    (1u64..).flat_map(move |n| n.to_digits_desc(&base))
}

impl Float {
    /// Returns an approximation of the Champernowne constant in a given base, with the given
    /// precision and rounded using the given [`RoundingMode`]. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than or greater than the exact value of the
    /// constant. (Since the constant is irrational, the rounded value is never equal to the exact
    /// value.)
    ///
    /// The Champernowne constant in base $b$ is formed by concatenating the base-$b$
    /// representations of the positive integers after the point. That is,
    /// $$
    /// C_b = \sum_{n=1}^\infty n b^{-(n + \sum_{k=1}^n \lfloor\log_b k\rfloor)}+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 C_b\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 C_b\rfloor-p}$.
    ///
    /// Base 10 gives the classical constant, $0.123456789101112\ldots$. The constant is normal in
    /// its base, by construction, and transcendental in every base, by Mahler's theorem.
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
    /// let (_, o) = Float::champernowne_constant_base_prec_round(10, 100, Floor);
    /// assert_eq!(o, Less);
    ///
    /// let (_, o) = Float::champernowne_constant_base_prec_round(10, 100, Ceiling);
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn champernowne_constant_base_prec_round(
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::non_dyadic_from_digits_prec_round(champernowne_digits(base), base, prec, rm)
    }

    /// Returns an approximation of the Champernowne constant in a given base, with the given
    /// precision and rounded to the nearest [`Float`] of that precision. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than or greater than the exact value.
    ///
    /// See [`champernowne_constant_base_prec_round`](Float::champernowne_constant_base_prec_round)
    /// for details.
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
    ///
    /// let (c, _) = Float::champernowne_constant_base_prec(10, 100);
    /// assert_eq!(c.to_string(), "0.12345678910111213141516171819207");
    /// ```
    #[inline]
    pub fn champernowne_constant_base_prec(base: u64, prec: u64) -> (Self, Ordering) {
        Self::champernowne_constant_base_prec_round(base, prec, Nearest)
    }

    /// Returns an approximation of the Champernowne constant in base 10, with the given precision
    /// and rounded using the given [`RoundingMode`]. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than or greater than the exact value of the constant.
    /// (Since the constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// The Champernowne constant is formed by concatenating the decimal representations of the
    /// positive integers after the radix point.
    ///
    /// $$
    /// x = C = 0.123456789101112\ldots+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p-1}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The output has precision `prec`.
    ///
    /// This is the base-10 specialization of
    /// [`champernowne_constant_base_prec_round`](Float::champernowne_constant_base_prec_round).
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
    /// let (x, o) = Float::champernowne_constant_prec_round(100, Floor);
    /// assert_eq!(x.to_string(), "0.12345678910111213141516171819197");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::champernowne_constant_prec_round(100, Ceiling);
    /// assert_eq!(x.to_string(), "0.12345678910111213141516171819207");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn champernowne_constant_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::champernowne_constant_base_prec_round(10, prec, rm)
    }

    /// Returns an approximation of the Champernowne constant in base 10, with the given precision
    /// and rounded to the nearest [`Float`] of that precision. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than or greater than the exact value of the
    /// constant. (Since the constant is irrational, the rounded value is never equal to the exact
    /// value.)
    ///
    /// The Champernowne constant is formed by concatenating the decimal representations of the
    /// positive integers after the radix point.
    ///
    /// $$
    /// x = C = 0.123456789101112\ldots+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p-1}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The output has precision `prec`.
    ///
    /// This is the base-10 specialization of
    /// [`champernowne_constant_base_prec`](Float::champernowne_constant_base_prec).
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
    /// let (x, _) = Float::champernowne_constant_prec(100);
    /// assert_eq!(x.to_string(), "0.12345678910111213141516171819207");
    /// ```
    #[inline]
    pub fn champernowne_constant_prec(prec: u64) -> (Self, Ordering) {
        Self::champernowne_constant_base_prec(10, prec)
    }
}

/// Computes an approximation of the Champernowne constant in a given base, returning a primitive
/// float.
///
/// The Champernowne constant in base $b$ is formed by concatenating the base-$b$ representations of
/// the positive integers after the radix point.
///
/// $$
/// C_b = 0.\overline{1\,2\,3\,4\,5\,\ldots}_b.
/// $$
///
/// The returned value is the one closest to the true constant; ties are broken by the
/// round-half-to-even rule. Computing the constant this way is more accurate than summing its
/// digits in primitive-float arithmetic, where each addition rounds.
///
/// $$
/// f(b) = C_b+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |C_b|\rfloor-p}$ and $p$ is the precision of the output
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
/// use malachite_float::float::constants::champernowne_constant::*;
///
/// // The classical constant, 0.123456789101112...
/// assert_eq!(
///     NiceFloat(primitive_float_champernowne_constant_base::<f32>(10)),
///     NiceFloat(0.12345679)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_champernowne_constant_base::<f64>(10)),
///     NiceFloat(0.12345678910111213)
/// );
/// // Base 1000 groups the integers into three-digit blocks: 001, 002, 003, ...
/// assert_eq!(
///     NiceFloat(primitive_float_champernowne_constant_base::<f64>(1000)),
///     NiceFloat(0.001002003004005006)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_champernowne_constant_base<T: PrimitiveFloat>(base: u64) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_constant_to_float_fn(|prec| Float::champernowne_constant_base_prec(base, prec))
}
