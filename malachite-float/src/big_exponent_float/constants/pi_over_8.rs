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
    /// Returns an approximation of $\pi/8$, with the given precision and rounded using the given
    /// [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than or greater than the exact value of the constant. (Since the constant is
    /// irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = \pi/8+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p-1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p-2}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The result is that of [its `Float` counterpart](crate::Float::pi_over_8_prec_round),
    /// converted to a [`BigExponentFloat`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// let (x, o) = BigExponentFloat::pi_over_8_prec_round(100, Floor);
    /// assert_eq!(x.to_string(), "0.3926990816987241548078304229099");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::pi_over_8_prec_round(100, Ceiling);
    /// assert_eq!(x.to_string(), "0.39269908169872415480783042291031");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pi_over_8_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (x, o) = Float::pi_over_8_prec_round(prec, rm);
        (Self::from(x), o)
    }

    /// Returns an approximation of $\pi/8$, with the given precision and rounded to the nearest
    /// value of that precision. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than or greater than the exact value of the constant. (Since the constant is
    /// irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = \pi/8+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p-2}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The result is that of [its `Float` counterpart](crate::Float::pi_over_8_prec), converted to
    /// a [`BigExponentFloat`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// let (x, o) = BigExponentFloat::pi_over_8_prec(1);
    /// assert_eq!(x.to_string(), "0.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = BigExponentFloat::pi_over_8_prec(10);
    /// assert_eq!(x.to_string(), "0.3926");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::pi_over_8_prec(100);
    /// assert_eq!(x.to_string(), "0.3926990816987241548078304229099");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn pi_over_8_prec(prec: u64) -> (Self, Ordering) {
        let (x, o) = Float::pi_over_8_prec(prec);
        (Self::from(x), o)
    }
}
