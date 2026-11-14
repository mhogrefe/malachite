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
    /// Returns an approximation of Euler's constant (also known as the Euler–Mascheroni
    /// constant), $\gamma=\lim_{n\to\infty}\left(\sum_{k=1}^n\frac{1}{k}-\log n\right)$, with the
    /// given precision and rounded using the given [`RoundingMode`]. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than or greater than the exact value
    /// of the constant. (The rounded value is never equal to the exact value of the constant.
    /// Euler's constant has not been proven irrational, but its binary expansion is known to be
    /// aperiodic far beyond any precision reachable by this function.)
    ///
    /// $$
    /// x = \gamma+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p}$.
    ///
    /// The result is that of [its `Float` counterpart](crate::Float::eulers_constant_prec_round),
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
    /// let (x, o) = BigExponentFloat::eulers_constant_prec_round(100, Floor);
    /// assert_eq!(format!("{x:?}"), "0x0.93c467e37db0c7a4d1be3f81");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::eulers_constant_prec_round(100, Ceiling);
    /// assert_eq!(format!("{x:?}"), "0x0.93c467e37db0c7a4d1be3f811");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn eulers_constant_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (x, o) = Float::eulers_constant_prec_round(prec, rm);
        (Self::from(x), o)
    }

    /// Returns an approximation of Euler's constant (also known as the Euler–Mascheroni
    /// constant), $\gamma=\lim_{n\to\infty}\left(\sum_{k=1}^n\frac{1}{k}-\log n\right)$, with the
    /// given precision and rounded to the nearest value of that precision. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than or greater than the exact value
    /// of the constant. (The rounded value is never equal to the exact value of the constant.
    /// Euler's constant has not been proven irrational, but its binary expansion is known to be
    /// aperiodic far beyond any precision reachable by this function.)
    ///
    /// $$
    /// x = \gamma+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p}$.
    ///
    /// The result is that of [its `Float` counterpart](crate::Float::eulers_constant_prec),
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
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::eulers_constant_prec(1);
    /// assert_eq!(format!("{x:?}"), "0x0.8");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::eulers_constant_prec(10);
    /// assert_eq!(format!("{x:?}"), "0x0.93c");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::eulers_constant_prec(100);
    /// assert_eq!(format!("{x:?}"), "0x0.93c467e37db0c7a4d1be3f81");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn eulers_constant_prec(prec: u64) -> (Self, Ordering) {
        let (x, o) = Float::eulers_constant_prec(prec);
        (Self::from(x), o)
    }
}
