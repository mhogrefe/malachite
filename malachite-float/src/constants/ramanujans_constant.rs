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
    /// Returns an approximation of Ramanujan's constant, $e^{\pi\sqrt{163}}$, with the given
    /// precision and rounded using the given [`RoundingMode`]. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than or greater than the exact value of the
    /// constant. (Since the constant is irrational, the rounded value is never equal to the exact
    /// value.)
    ///
    /// $$
    /// x = e^{\pi\sqrt{163}}+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p+58}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p+57}$.
    ///
    /// The constant is irrational and transcendental. It is famously close to an integer: $e^{\pi
    /// \sqrt{163}} \approx 262{,}537{,}412{,}640{,}768{,}744 - 7.5 \times 10^{-13}$.
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
    /// let (ramanujans_constant, o) = Float::ramanujans_constant_prec_round(100, Floor);
    /// assert_eq!(
    ///     ramanujans_constant.to_string(),
    ///     "262537412640768743.99999999999909"
    /// );
    /// assert_eq!(o, Less);
    ///
    /// let (ramanujans_constant, o) = Float::ramanujans_constant_prec_round(100, Ceiling);
    /// assert_eq!(
    ///     ramanujans_constant.to_string(),
    ///     "262537412640768743.99999999999932"
    /// );
    /// assert_eq!(o, Greater);
    /// ```
    pub fn ramanujans_constant_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let mut working_prec = prec + 10;
        let mut increment = Limb::WIDTH;
        loop {
            let (pi_lo, pi_hi) = floor_and_ceiling(Self::pi_prec_round(working_prec, Floor));
            let (sqrt_163_lo, sqrt_163_hi) = floor_and_ceiling(
                const { Self::const_from_unsigned(163) }.sqrt_prec_round(working_prec, Floor),
            );
            // pi and sqrt(163) are positive, so pi * sqrt(163) is bracketed by the products of the
            // corresponding bounds.
            //
            // exp is increasing, so exp(arg_lo) <= exp(pi * sqrt(163)) <= exp(arg_hi).
            let (ramanujans_constant_lo, mut o_lo) = Self::from_float_prec_round(
                pi_lo.mul_round(sqrt_163_lo, Floor).0.exp_round(Floor).0,
                prec,
                rm,
            );
            let (ramanujans_constant_hi, mut o_hi) = Self::from_float_prec_round(
                pi_hi.mul_round(sqrt_163_hi, Ceiling).0.exp_round(Ceiling).0,
                prec,
                rm,
            );
            if o_lo == Equal {
                o_lo = o_hi;
            }
            if o_hi == Equal {
                o_hi = o_lo;
            }
            if o_lo == o_hi && ramanujans_constant_lo == ramanujans_constant_hi {
                return (ramanujans_constant_lo, o_lo);
            }
            working_prec += increment;
            increment = working_prec >> 1;
        }
    }

    /// Returns an approximation of Ramanujan's constant, $e^{\pi\sqrt{163}}$, with the given
    /// precision and rounded to the nearest [`Float`] of that precision. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than or greater than the exact value
    /// of the constant. (Since the constant is irrational, the rounded value is never equal to the
    /// exact value.)
    ///
    /// $$
    /// x = e^{\pi\sqrt{163}}+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p+57}$.
    ///
    /// The constant is irrational and transcendental. It is famously close to an integer: $e^{\pi
    /// \sqrt{163}} \approx 262{,}537{,}412{,}640{,}768{,}744 - 7.5 \times 10^{-13}$.
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
    /// let (ramanujans_constant, o) = Float::ramanujans_constant_prec(1);
    /// assert_eq!(ramanujans_constant.to_string(), "2.9e17");
    /// assert_eq!(o, Greater);
    ///
    /// let (ramanujans_constant, o) = Float::ramanujans_constant_prec(10);
    /// assert_eq!(ramanujans_constant.to_string(), "2.6262e17");
    /// assert_eq!(o, Greater);
    ///
    /// let (ramanujans_constant, o) = Float::ramanujans_constant_prec(97);
    /// assert_eq!(ramanujans_constant.to_string(), "262537412640768744.0000000000000");
    /// assert_eq!(o, Greater);
    ///
    /// let (ramanujans_constant, o) = Float::ramanujans_constant_prec(100);
    /// assert_eq!(
    ///     ramanujans_constant.to_string(),
    ///     "262537412640768743.99999999999932"
    /// );
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ramanujans_constant_prec(prec: u64) -> (Self, Ordering) {
        Self::ramanujans_constant_prec_round(prec, Nearest)
    }
}
