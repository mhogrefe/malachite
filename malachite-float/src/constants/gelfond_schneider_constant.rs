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
    /// Returns an approximation of the Gelfond–Schneider constant, $2^{\sqrt 2}$, with the given
    /// precision and rounded using the given [`RoundingMode`]. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than or greater than the exact value of the
    /// constant. (Since the constant is irrational, the rounded value is never equal to the exact
    /// value.)
    ///
    /// $$
    /// x = 2^{\sqrt 2}+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p+2}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p+1}$.
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
    /// let (gelfond_schneider_constant, o) =
    ///     Float::gelfond_schneider_constant_prec_round(100, Floor);
    /// assert_eq!(
    ///     gelfond_schneider_constant.to_string(),
    ///     "2.6651441426902251886502972498731"
    /// );
    /// assert_eq!(o, Less);
    ///
    /// let (gelfond_schneider_constant, o) =
    ///     Float::gelfond_schneider_constant_prec_round(100, Ceiling);
    /// assert_eq!(
    ///     gelfond_schneider_constant.to_string(),
    ///     "2.6651441426902251886502972498762"
    /// );
    /// assert_eq!(o, Greater);
    /// ```
    pub fn gelfond_schneider_constant_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let mut working_prec = prec + 10;
        let mut increment = Limb::WIDTH;
        loop {
            let (sqrt_2_lo, sqrt_2_hi) =
                floor_and_ceiling(Self::sqrt_2_prec_round(working_prec, Floor));
            // 2^x is increasing, so 2^sqrt_2_lo <= 2^sqrt(2) <= 2^sqrt_2_hi.
            let (gelfond_schneider_constant_lo, mut o_lo) = Self::from_float_prec_round(
                Self::power_of_2_of_float_round(sqrt_2_lo, Floor).0,
                prec,
                rm,
            );
            let (gelfond_schneider_constant_hi, mut o_hi) = Self::from_float_prec_round(
                Self::power_of_2_of_float_round(sqrt_2_hi, Ceiling).0,
                prec,
                rm,
            );
            if o_lo == Equal {
                o_lo = o_hi;
            }
            if o_hi == Equal {
                o_hi = o_lo;
            }
            if o_lo == o_hi && gelfond_schneider_constant_lo == gelfond_schneider_constant_hi {
                return (gelfond_schneider_constant_lo, o_lo);
            }
            working_prec += increment;
            increment = working_prec >> 1;
        }
    }

    /// Returns an approximation of the Gelfond–Schneider constant, $2^{\sqrt 2}$, with the given
    /// precision and rounded to the nearest [`Float`] of that precision. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than or greater than the exact value
    /// of the constant. (Since the constant is irrational, the rounded value is never equal to the
    /// exact value.)
    ///
    /// $$
    /// x = 2^{\sqrt 2}+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p+1}$.
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
    /// let (gelfond_schneider_constant, o) = Float::gelfond_schneider_constant_prec(1);
    /// assert_eq!(gelfond_schneider_constant.to_string(), "2.0");
    /// assert_eq!(o, Less);
    ///
    /// let (gelfond_schneider_constant, o) = Float::gelfond_schneider_constant_prec(10);
    /// assert_eq!(gelfond_schneider_constant.to_string(), "2.6641");
    /// assert_eq!(o, Less);
    ///
    /// let (gelfond_schneider_constant, o) = Float::gelfond_schneider_constant_prec(100);
    /// assert_eq!(
    ///     gelfond_schneider_constant.to_string(),
    ///     "2.6651441426902251886502972498731"
    /// );
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gelfond_schneider_constant_prec(prec: u64) -> (Self, Ordering) {
        Self::gelfond_schneider_constant_prec_round(prec, Nearest)
    }
}
