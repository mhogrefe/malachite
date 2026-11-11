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
    /// Returns an approximation of a real number, given the number's bits. To avoid troublesome
    /// edge cases, the number should not be a dyadic rational (and the iterator of bits should
    /// therefore be infinite, and not eventually 0 or 1). Given this assumption, the rounding mode
    /// `Exact` should never be passed.
    ///
    /// The bits should be those of a real number in $[0, 1)$, most significant first: the first bit
    /// is the coefficient of $2^{-1}$. This is [`Float::non_dyadic_from_bits_prec_round`] with a
    /// [`BigExponentFloat`] result; the result lies in $[0, 1]$, where [`Float`] and
    /// [`BigExponentFloat`] agree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`, provided that the iterator
    /// produces a 1 within a bounded number of steps.
    ///
    /// # Panics
    /// Panics if `prec` is zero or `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// // 0.010101..., which is 1/3
    /// let bits = [false, true].into_iter().cycle();
    /// let (x, o) = BigExponentFloat::non_dyadic_from_bits_prec_round(bits, 10, Floor);
    /// assert_eq!(format!("{x:?}"), "0x0.554");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn non_dyadic_from_bits_prec_round<I: Iterator<Item = bool>>(
        bits: I,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (x, o) = Float::non_dyadic_from_bits_prec_round(bits, prec, rm);
        (Self::from(x), o)
    }

    /// Returns an approximation of a real number, given the number's bits, rounding to the nearest
    /// value of the given precision. To avoid troublesome edge cases, the number should not be a
    /// dyadic rational (and the iterator of bits should therefore be infinite, and not eventually 0
    /// or 1).
    ///
    /// See [`BigExponentFloat::non_dyadic_from_bits_prec_round`] for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`, provided that the iterator
    /// produces a 1 within a bounded number of steps.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let bits = [false, true].into_iter().cycle();
    /// let (x, o) = BigExponentFloat::non_dyadic_from_bits_prec(bits, 10);
    /// assert_eq!(format!("{x:?}"), "0x0.556");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn non_dyadic_from_bits_prec<I: Iterator<Item = bool>>(
        bits: I,
        prec: u64,
    ) -> (Self, Ordering) {
        Self::non_dyadic_from_bits_prec_round(bits, prec, Nearest)
    }
}
