// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::{BigExponentFloat, Float};
use core::cmp::Ordering;
use malachite_base::rounding_modes::RoundingMode;

impl BigExponentFloat {
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
    /// The result is that of [its `Float`
    /// counterpart](crate::Float::champernowne_constant_base_prec_round), converted to a
    /// [`BigExponentFloat`].
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
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::champernowne_constant_base_prec_round(16, 100, Floor);
    /// assert_eq!(x.to_string(), "0.0711111111111111102365063523810");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::champernowne_constant_base_prec_round(16, 100, Ceiling);
    /// assert_eq!(x.to_string(), "0.071111111111111110236506352381062");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn champernowne_constant_base_prec_round(
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (x, o) = Float::champernowne_constant_base_prec_round(base, prec, rm);
        (Self::from(x), o)
    }

    /// Returns an approximation of the Champernowne constant in a given base, with the given
    /// precision and rounded to the nearest value of that precision. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than or greater than the exact value.
    ///
    /// See
    /// [`champernowne_constant_base_prec_round`](BigExponentFloat::champernowne_constant_base_prec_round)
    /// for details.
    ///
    /// The result is that of [its `Float`
    /// counterpart](crate::Float::champernowne_constant_base_prec), converted to a
    /// [`BigExponentFloat`].
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
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::champernowne_constant_base_prec(16, 100);
    /// assert_eq!(x.to_string(), "0.071111111111111110236506352381062");
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = BigExponentFloat::champernowne_constant_base_prec(3, 50);
    /// assert_eq!(x.to_string(), "0.5989581675384343");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn champernowne_constant_base_prec(base: u64, prec: u64) -> (Self, Ordering) {
        let (x, o) = Float::champernowne_constant_base_prec(base, prec);
        (Self::from(x), o)
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
    /// This is the base-10 specialization of
    /// [`champernowne_constant_base_prec_round`](BigExponentFloat::champernowne_constant_base_prec_round).
    ///
    /// The result is that of [its `Float`
    /// counterpart](crate::Float::champernowne_constant_prec_round), converted to a
    /// [`BigExponentFloat`].
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
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::champernowne_constant_prec_round(100, Floor);
    /// assert_eq!(x.to_string(), "0.1234567891011121314151617181920");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::champernowne_constant_prec_round(100, Ceiling);
    /// assert_eq!(x.to_string(), "0.12345678910111213141516171819207");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn champernowne_constant_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (x, o) = Float::champernowne_constant_prec_round(prec, rm);
        (Self::from(x), o)
    }

    /// Returns an approximation of the Champernowne constant in base 10, with the given precision
    /// and rounded to the nearest value of that precision. An [`Ordering`] is also returned,
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
    /// This is the base-10 specialization of
    /// [`champernowne_constant_base_prec`](BigExponentFloat::champernowne_constant_base_prec).
    ///
    /// The result is that of [its `Float` counterpart](crate::Float::champernowne_constant_prec),
    /// converted to a [`BigExponentFloat`].
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
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::champernowne_constant_prec(100);
    /// assert_eq!(x.to_string(), "0.12345678910111213141516171819207");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn champernowne_constant_prec(prec: u64) -> (Self, Ordering) {
        let (x, o) = Float::champernowne_constant_prec(prec);
        (Self::from(x), o)
    }
}
