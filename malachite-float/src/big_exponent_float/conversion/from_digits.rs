// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::Float;
use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering;
use malachite_base::rounding_modes::RoundingMode::{self, *};

impl BigExponentFloat {
    /// Returns an approximation of a real number, given the number's base-$2^k$ digits. To avoid
    /// troublesome edge cases, the number should not be a dyadic rational (and the iterator of
    /// digits should therefore be infinite, and not eventually 0 or $2^k - 1$). Given this
    /// assumption, the rounding mode `Exact` should never be passed.
    ///
    /// This is [`Float::non_dyadic_from_power_of_2_digits_prec_round`] with a [`BigExponentFloat`]
    /// result; the result lies in $[0, 1]$, where [`Float`] and [`BigExponentFloat`] agree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`, provided that the iterator
    /// produces a nonzero digit within a bounded number of steps.
    ///
    /// # Panics
    /// Panics if `prec` is zero, `rm` is `Exact`, or `log_base` is zero or greater than 64.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// // 0.111... in base 4, which is 1/3
    /// let digits = std::iter::repeat(1);
    /// let (x, o) =
    ///     BigExponentFloat::non_dyadic_from_power_of_2_digits_prec_round(digits, 2, 10, Floor);
    /// assert_eq!(format!("{x:?}"), "0x0.554");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn non_dyadic_from_power_of_2_digits_prec_round<I: Iterator<Item = u64>>(
        digits: I,
        log_base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (x, o) =
            Float::non_dyadic_from_power_of_2_digits_prec_round(digits, log_base, prec, rm);
        (Self::from(x), o)
    }

    /// Returns an approximation of a real number, given the number's base-$2^k$ digits, rounding to
    /// the nearest value of the given precision.
    ///
    /// See [`BigExponentFloat::non_dyadic_from_power_of_2_digits_prec_round`] for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`, provided that the iterator
    /// produces a nonzero digit within a bounded number of steps.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or `log_base` is zero or greater than 64.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let digits = std::iter::repeat(1);
    /// let (x, o) = BigExponentFloat::non_dyadic_from_power_of_2_digits_prec(digits, 2, 10);
    /// assert_eq!(format!("{x:?}"), "0x0.556");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn non_dyadic_from_power_of_2_digits_prec<I: Iterator<Item = u64>>(
        digits: I,
        log_base: u64,
        prec: u64,
    ) -> (Self, Ordering) {
        Self::non_dyadic_from_power_of_2_digits_prec_round(digits, log_base, prec, Nearest)
    }

    /// Returns an approximation of a real number, given the number's digits in a given base. To
    /// avoid troublesome edge cases, the number should not be a dyadic rational. Given this
    /// assumption, the rounding mode `Exact` should never be passed.
    ///
    /// This is [`Float::non_dyadic_from_digits_prec_round`] with a [`BigExponentFloat`] result; the
    /// result lies in $[0, 1]$, where [`Float`] and [`BigExponentFloat`] agree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`, provided that the iterator
    /// produces digits that determine the rounding within a bounded number of further steps.
    ///
    /// # Panics
    /// Panics if `prec` is zero, `rm` is `Exact`, or `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// // 0.333... in base 10, which is 1/3
    /// let digits = std::iter::repeat(3);
    /// let (x, o) = BigExponentFloat::non_dyadic_from_digits_prec_round(digits, 10, 10, Floor);
    /// assert_eq!(format!("{x:?}"), "0x0.554");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn non_dyadic_from_digits_prec_round<I: Iterator<Item = u64>>(
        digits: I,
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (x, o) = Float::non_dyadic_from_digits_prec_round(digits, base, prec, rm);
        (Self::from(x), o)
    }

    /// Returns an approximation of a real number, given the number's digits in a given base,
    /// rounding to the nearest value of the given precision.
    ///
    /// See [`BigExponentFloat::non_dyadic_from_digits_prec_round`] for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`, provided that the iterator
    /// produces digits that determine the rounding within a bounded number of further steps.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let digits = std::iter::repeat(3);
    /// let (x, o) = BigExponentFloat::non_dyadic_from_digits_prec(digits, 10, 10);
    /// assert_eq!(format!("{x:?}"), "0x0.556");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn non_dyadic_from_digits_prec<I: Iterator<Item = u64>>(
        digits: I,
        base: u64,
        prec: u64,
    ) -> (Self, Ordering) {
        Self::non_dyadic_from_digits_prec_round(digits, base, prec, Nearest)
    }
}
