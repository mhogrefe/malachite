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
    /// Returns an approximation of the Dottie number, the unique real fixed point of the cosine,
    /// with the given precision and rounded using the given [`RoundingMode`]. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than or greater than the exact
    /// value of the constant. (Since the constant is irrational, the rounded value is never equal
    /// to the exact value.)
    ///
    /// $$
    /// x = d+\varepsilon, \quad \text{where } \cos d = d.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p-1}$.
    ///
    /// The constant is irrational and transcendental (if $d$ were algebraic, $\cos d$ would be
    /// transcendental by the Lindemann-Weierstrass theorem, and could not equal $d$).
    ///
    /// The root of $x - \cos x$ is found by Newton's method with the working precision doubled at
    /// each step, and the final iterate is certified by bounding the residual $x - \cos x$ with a
    /// correctly rounded cosine, so that the result is correctly rounded rather than merely the
    /// fixed point of a rounded cosine.
    ///
    /// The result is that of [its `Float` counterpart](crate::Float::dottie_number_prec_round),
    /// converted to a [`BigExponentFloat`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
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
    /// let (x, o) = BigExponentFloat::dottie_number_prec_round(100, Floor);
    /// assert_eq!(format!("{x:?}"), "0x0.bd34aeec1e716dcad51803874");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::dottie_number_prec_round(100, Ceiling);
    /// assert_eq!(format!("{x:?}"), "0x0.bd34aeec1e716dcad51803875");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn dottie_number_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (x, o) = Float::dottie_number_prec_round(prec, rm);
        (Self::from(x), o)
    }

    /// Returns an approximation of the Dottie number, the unique real fixed point of the cosine,
    /// with the given precision and rounded to the nearest value of that precision. An [`Ordering`]
    /// is also returned, indicating whether the rounded value is less than or greater than the
    /// exact value of the constant. (Since the constant is irrational, the rounded value is never
    /// equal to the exact value.)
    ///
    /// $$
    /// x = d+\varepsilon, \quad \text{where } \cos d = d.
    /// $$
    /// - $|\varepsilon| < 2^{-p-1}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The result is that of [its `Float` counterpart](crate::Float::dottie_number_prec), converted
    /// to a [`BigExponentFloat`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
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
    /// let (x, o) = BigExponentFloat::dottie_number_prec(1);
    /// assert_eq!(format!("{x:?}"), "0x0.8");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::dottie_number_prec(10);
    /// assert_eq!(format!("{x:?}"), "0x0.bd4");
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = BigExponentFloat::dottie_number_prec(100);
    /// assert_eq!(format!("{x:?}"), "0x0.bd34aeec1e716dcad51803875");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn dottie_number_prec(prec: u64) -> (Self, Ordering) {
        let (x, o) = Float::dottie_number_prec(prec);
        (Self::from(x), o)
    }
}
