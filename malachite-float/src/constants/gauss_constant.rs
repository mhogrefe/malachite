// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 1999, 2001-2024 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use core::cmp::Ordering;
use core::cmp::Ordering::*;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::platform::Limb;

impl Float {
    /// Returns an approximation of Gauss's constant, with the given precision and rounded using the
    /// given [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the rounded
    /// value is less than or greater than the exact value of the constant. (Since the constant is
    /// irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = G+\varepsilon=1/\mathrm{AGM}(1,\sqrt{2})+\varepsilon,
    /// $$
    /// where AGM is the arithmetic-geometric mean.
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p-1}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
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
    /// let (gauss_constant, o) = Float::gauss_constant_prec_round(100, Floor);
    /// assert_eq!(gauss_constant.to_string(), "0.834626841674073186281429732799");
    /// assert_eq!(o, Less);
    ///
    /// let (gauss_constant, o) = Float::gauss_constant_prec_round(100, Ceiling);
    /// assert_eq!(gauss_constant.to_string(), "0.8346268416740731862814297328");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn gauss_constant_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let mut working_prec = prec + 10;
        let mut increment = Limb::WIDTH;
        loop {
            let sqrt_2_lo = Self::sqrt_2_prec_round(working_prec, Floor).0;
            let mut sqrt_2_hi = sqrt_2_lo.clone();
            sqrt_2_hi.increment();
            let lo = Self::ONE
                .agm_round(sqrt_2_hi, Ceiling)
                .0
                .reciprocal_round(Floor)
                .0;
            let hi = Self::ONE
                .agm_round(sqrt_2_lo, Floor)
                .0
                .reciprocal_round(Ceiling)
                .0;
            let (gauss_constant_lo, mut o_lo) = Self::from_float_prec_round(lo, prec, rm);
            let (gauss_constant_hi, mut o_hi) = Self::from_float_prec_round(hi, prec, rm);
            if o_lo == Equal {
                o_lo = o_hi;
            }
            if o_hi == Equal {
                o_hi = o_lo;
            }
            if o_lo == o_hi && gauss_constant_lo == gauss_constant_hi {
                return (gauss_constant_lo, o_lo);
            }
            working_prec += increment;
            increment = working_prec >> 1;
        }
    }

    /// Returns an approximation of Gauss's constant, $G=1/\mathrm{AGM}(1,\sqrt{2})$, with the given
    /// precision and rounded to the nearest [`Float`] of that precision. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than or greater than the exact value
    /// of the constant. (Since the constant is irrational, the rounded value is never equal to the
    /// exact value.)
    ///
    /// $$
    /// x=G+\varepsilon=1/\mathrm{AGM}(1,\sqrt{2})+\varepsilon,
    /// $$
    /// where AGM is the arithmetic-geometric mean.
    /// - $|\varepsilon| < 2^{-p-1}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
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
    /// let (gauss_constant, o) = Float::gauss_constant_prec(1);
    /// assert_eq!(gauss_constant.to_string(), "1.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (gauss_constant, o) = Float::gauss_constant_prec(10);
    /// assert_eq!(gauss_constant.to_string(), "0.835");
    /// assert_eq!(o, Greater);
    ///
    /// let (gauss_constant, o) = Float::gauss_constant_prec(100);
    /// assert_eq!(gauss_constant.to_string(), "0.834626841674073186281429732799");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gauss_constant_prec(prec: u64) -> (Self, Ordering) {
        Self::gauss_constant_prec_round(prec, Nearest)
    }
}
