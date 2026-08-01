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
use malachite_base::num::factorization::traits::Primes;
use malachite_base::rounding_modes::RoundingMode::{self, Nearest};

// The digits of the Copeland–Erdős constant in the given base: the base-`base` representations
// of the primes run together.
fn copeland_erdos_digits(base: u64) -> impl Iterator<Item = u64> {
    u64::primes().flat_map(move |p| p.to_digits_desc(&base))
}

impl Float {
    /// Returns an approximation of the Copeland–Erdős constant in a given base, with the given
    /// precision and rounded using the given [`RoundingMode`]. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than or greater than the exact value of the
    /// constant. (Since the constant is irrational, the rounded value is never equal to the exact
    /// value.)
    ///
    /// The Copeland–Erdős constant in base $b$ is formed by concatenating the base-$b$
    /// representations of the primes after the point:
    /// $$
    /// CE_b = 0.\overline{p_1 p_2 p_3 \ldots}_b+\varepsilon,
    /// $$
    /// where $p_i$ is the $i$th prime written in base $b$.
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 CE_b\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 CE_b\rfloor-p}$.
    ///
    /// Base 10 gives the classical constant, $0.235711131719\ldots$. The Copeland–Erdős theorem
    /// says that it is normal in its base, which in particular makes it irrational.
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
    /// // In base 16 the digits are the primes in hexadecimal: 2, 3, 5, 7, b, d, 11, 13, 17,
    /// // ..., which the hexadecimal representation spells out.
    /// let (x, o) = Float::copeland_erdos_constant_base_prec_round(16, 100, Floor);
    /// assert_eq!(x.to_string(), "0.13805753390178350683643564212329");
    /// assert_eq!(format!("{x:#x}"), "0x0.2357bd1113171d1f25292b2f34");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::copeland_erdos_constant_base_prec_round(16, 100, Ceiling);
    /// assert_eq!(x.to_string(), "0.13805753390178350683643564212348");
    /// assert_eq!(format!("{x:#x}"), "0x0.2357bd1113171d1f25292b2f38");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn copeland_erdos_constant_base_prec_round(
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::non_dyadic_from_digits_prec_round(copeland_erdos_digits(base), base, prec, rm)
    }

    /// Returns an approximation of the Copeland–Erdős constant in a given base, with the given
    /// precision and rounded to the nearest [`Float`] of that precision. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than or greater than the exact value.
    ///
    /// See
    /// [`copeland_erdos_constant_base_prec_round`](Float::copeland_erdos_constant_base_prec_round)
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
    /// use std::cmp::Ordering::*;
    ///
    /// // In base 16 the digits are the primes in hexadecimal: 2, 3, 5, 7, b, d, 11, 13, ...
    /// let (x, o) = Float::copeland_erdos_constant_base_prec(16, 100);
    /// assert_eq!(x.to_string(), "0.13805753390178350683643564212329");
    /// assert_eq!(format!("{x:#x}"), "0x0.2357bd1113171d1f25292b2f34");
    /// assert_eq!(o, Less);
    ///
    /// // Base 3 concatenates 2, 12, 21, 111, 122, 200, ...
    /// let (x, o) = Float::copeland_erdos_constant_base_prec(3, 50);
    /// assert_eq!(x.to_string(), "0.80174949296954523");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn copeland_erdos_constant_base_prec(base: u64, prec: u64) -> (Self, Ordering) {
        Self::copeland_erdos_constant_base_prec_round(base, prec, Nearest)
    }

    /// Returns an approximation of the Copeland–Erdős constant in base 10, with the given
    /// precision and rounded using the given [`RoundingMode`]. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than or greater than the exact value of the
    /// constant. (Since the constant is irrational, the rounded value is never equal to the exact
    /// value.)
    ///
    /// The Copeland–Erdős constant is formed by concatenating the decimal representations of the
    /// primes after the radix point.
    ///
    /// $$
    /// x = CE = 0.235711131719\ldots+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p-1}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The output has precision `prec`.
    ///
    /// This is the base-10 specialization of
    /// [`copeland_erdos_constant_base_prec_round`](Float::copeland_erdos_constant_base_prec_round).
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
    /// let (x, o) = Float::copeland_erdos_constant_prec_round(100, Floor);
    /// assert_eq!(x.to_string(), "0.23571113171923293137414347535946");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::copeland_erdos_constant_prec_round(100, Ceiling);
    /// assert_eq!(x.to_string(), "0.23571113171923293137414347535966");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn copeland_erdos_constant_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::copeland_erdos_constant_base_prec_round(10, prec, rm)
    }

    /// Returns an approximation of the Copeland–Erdős constant in base 10, with the given
    /// precision and rounded to the nearest [`Float`] of that precision. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than or greater than the exact value
    /// of the constant. (Since the constant is irrational, the rounded value is never equal to the
    /// exact value.)
    ///
    /// The Copeland–Erdős constant is formed by concatenating the decimal representations of the
    /// primes after the radix point.
    ///
    /// $$
    /// x = CE = 0.235711131719\ldots+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p-1}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The output has precision `prec`.
    ///
    /// This is the base-10 specialization of
    /// [`copeland_erdos_constant_base_prec`](Float::copeland_erdos_constant_base_prec).
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
    /// let (x, _) = Float::copeland_erdos_constant_prec(100);
    /// assert_eq!(x.to_string(), "0.23571113171923293137414347535966");
    /// ```
    #[inline]
    pub fn copeland_erdos_constant_prec(prec: u64) -> (Self, Ordering) {
        Self::copeland_erdos_constant_base_prec(10, prec)
    }
}

/// Computes an approximation of the Copeland–Erdős constant in a given base, returning a
/// primitive float.
///
/// The Copeland–Erdős constant in base $b$ is formed by concatenating the base-$b$
/// representations of the primes after the radix point.
///
/// $$
/// CE_b = 0.\overline{2\,3\,5\,7\,11\,\ldots}_b.
/// $$
///
/// The returned value is the one closest to the true constant; ties are broken by the
/// round-half-to-even rule. Computing the constant this way is more accurate than summing its
/// digits in primitive-float arithmetic, where each addition rounds.
///
/// $$
/// f(b) = CE_b+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |CE_b|\rfloor-p}$ and $p$ is the precision of the output
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
/// use malachite_float::float::constants::copeland_erdos_constant::*;
///
/// // The classical constant, 0.23571113171923...
/// assert_eq!(
///     NiceFloat(primitive_float_copeland_erdos_constant_base::<f32>(10)),
///     NiceFloat(0.23571113)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_copeland_erdos_constant_base::<f64>(10)),
///     NiceFloat(0.23571113171923294)
/// );
/// // Base 1000 gives each prime its own three-digit block: 002, 003, 005, 007, 011, 013
/// assert_eq!(
///     NiceFloat(primitive_float_copeland_erdos_constant_base::<f64>(1000)),
///     NiceFloat(0.002003005007011013)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_copeland_erdos_constant_base<T: PrimitiveFloat>(base: u64) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_constant_to_float_fn(|prec| Float::copeland_erdos_constant_base_prec(base, prec))
}
