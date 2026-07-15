// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::{Float, floor_and_ceiling};
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::platform::Limb;

impl Float {
    /// Returns an approximation of Gelfond's constant, $e^\pi$, with the given precision and
    /// rounded using the given [`RoundingMode`]. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than or greater than the exact value of the constant.
    /// (Since the constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = e^\pi+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p+5}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p+4}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
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
    /// let (gelfonds_constant, o) = Float::gelfonds_constant_prec_round(100, Floor);
    /// assert_eq!(
    ///     gelfonds_constant.to_string(),
    ///     "23.14069263277926900572908636794"
    /// );
    /// assert_eq!(o, Less);
    ///
    /// let (gelfonds_constant, o) = Float::gelfonds_constant_prec_round(100, Ceiling);
    /// assert_eq!(
    ///     gelfonds_constant.to_string(),
    ///     "23.14069263277926900572908636796"
    /// );
    /// assert_eq!(o, Greater);
    /// ```
    pub fn gelfonds_constant_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let mut working_prec = prec + 10;
        let mut increment = Limb::WIDTH;
        loop {
            let (pi_lo, pi_hi) = floor_and_ceiling(Self::pi_prec_round(working_prec, Floor));
            // exp is increasing, so exp(pi_lo) <= exp(pi) <= exp(pi_hi).
            let (gelfonds_constant_lo, mut o_lo) =
                Self::from_float_prec_round(pi_lo.exp_round(Floor).0, prec, rm);
            let (gelfonds_constant_hi, mut o_hi) =
                Self::from_float_prec_round(pi_hi.exp_round(Ceiling).0, prec, rm);
            if o_lo == Equal {
                o_lo = o_hi;
            }
            if o_hi == Equal {
                o_hi = o_lo;
            }
            if o_lo == o_hi && gelfonds_constant_lo == gelfonds_constant_hi {
                return (gelfonds_constant_lo, o_lo);
            }
            working_prec += increment;
            increment = working_prec >> 1;
        }
    }

    /// Returns an approximation of Gelfond's constant, $e^\pi$, with the given precision and
    /// rounded to the nearest [`Float`] of that precision. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than or greater than the exact value of the
    /// constant. (Since the constant is irrational, the rounded value is never equal to the exact
    /// value.)
    ///
    /// $$
    /// x = e^\pi+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p+4}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
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
    /// use std::cmp::Ordering::*;
    ///
    /// let (gelfonds_constant, o) = Float::gelfonds_constant_prec(1);
    /// assert_eq!(gelfonds_constant.to_string(), "2.0e1");
    /// assert_eq!(o, Less);
    ///
    /// let (gelfonds_constant, o) = Float::gelfonds_constant_prec(10);
    /// assert_eq!(gelfonds_constant.to_string(), "23.16");
    /// assert_eq!(o, Greater);
    ///
    /// let (gelfonds_constant, o) = Float::gelfonds_constant_prec(100);
    /// assert_eq!(
    ///     gelfonds_constant.to_string(),
    ///     "23.14069263277926900572908636794"
    /// );
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gelfonds_constant_prec(prec: u64) -> (Self, Ordering) {
        Self::gelfonds_constant_prec_round(prec, Nearest)
    }
}
