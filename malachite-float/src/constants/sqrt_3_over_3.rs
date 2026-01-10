// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use core::cmp::Ordering;
use malachite_base::rounding_modes::RoundingMode::{self, *};

impl Float {
    /// Returns an approximation to one third of the square root of 3, with the given precision and
    /// rounded using the given [`RoundingMode`]. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than or greater than the exact value of the constant.
    /// (Since the constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// $$
    /// x = \sqrt{3}/3=\sqrt{1/3}=1/\sqrt{3}.
    /// $$
    ///
    /// The constant is irrational and algebraic.
    ///
    /// The output has precision `prec`.
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
    /// let (sqrt_3_over_3, o) = Float::sqrt_3_over_3_prec_round(100, Floor);
    /// assert_eq!(
    ///     sqrt_3_over_3.to_string(),
    ///     "0.577350269189625764509148780501"
    /// );
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt_3_over_3, o) = Float::sqrt_3_over_3_prec_round(100, Ceiling);
    /// assert_eq!(
    ///     sqrt_3_over_3.to_string(),
    ///     "0.577350269189625764509148780502"
    /// );
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sqrt_3_over_3_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::reciprocal_sqrt_prec_round(const { Self::const_from_unsigned(3) }, prec, rm)
    }

    /// Returns an approximation to one third of the square root of 3, with the given precision and
    /// rounded to the nearest [`Float`] of that precision. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than or greater than the exact value of the
    /// constant. (Since the constant is irrational, the rounded value is never equal to the exact
    /// value.)
    ///
    /// $$
    /// x = \sqrt{3}/3=\sqrt{1/3}=1/\sqrt{3}.
    /// $$
    ///
    /// The constant is irrational and algebraic.
    ///
    /// The output has precision `prec`.
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
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt_3_over_3, o) = Float::sqrt_3_over_3_prec(1);
    /// assert_eq!(sqrt_3_over_3.to_string(), "0.5");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt_3_over_3, o) = Float::sqrt_3_over_3_prec(10);
    /// assert_eq!(sqrt_3_over_3.to_string(), "0.577");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt_3_over_3, o) = Float::sqrt_3_over_3_prec(100);
    /// assert_eq!(
    ///     sqrt_3_over_3.to_string(),
    ///     "0.577350269189625764509148780502"
    /// );
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sqrt_3_over_3_prec(prec: u64) -> (Self, Ordering) {
        Self::sqrt_3_over_3_prec_round(prec, Nearest)
    }
}
