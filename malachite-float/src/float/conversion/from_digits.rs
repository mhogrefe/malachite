// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{CheckedLogBase2, FloorLogBase2, Pow, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{Digits, ExactFrom};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// Rounds the exact quotient of two `Natural`s, the second nonzero, to a `Float` of the given
// precision. The conversions are exact, so the division sees the exact quotient and the returned
// `Ordering` describes the quotient itself.
fn quotient_prec_round(n: Natural, d: Natural, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    // A `Natural` needs an exponent of `significant_bits` to be a `Float`, so past `MAX_EXPONENT`
    // it has no `Float` of its own even when the quotient is perfectly ordinary -- which is the
    // usual case here, since the quotient is a digit expansion less than 1 while the numerator and
    // denominator both grow with the precision. Building the `Rational` first never forms either
    // endpoint as a `Float`. It costs a gcd, so it is worth avoiding until it is needed.
    //
    // Both arguments are taken by value: at these sizes the conversions would otherwise copy them,
    // and one caller-side clone is cheaper than two internal ones.
    if max(n.significant_bits(), d.significant_bits()) > Float::MAX_EXPONENT_U64 {
        Float::from_rational_prec_round(Rational::from_naturals(n, d), prec, rm)
    } else {
        Float::exact_from(n).div_prec_round(Float::exact_from(d), prec, rm)
    }
}

impl Float {
    /// Returns an approximation of a real number, given the number's digits in a base that is a
    /// power of 2.
    ///
    /// Each digit contributes exactly $\log_2 b$ bits, so this is
    /// [`non_dyadic_from_bits_prec_round`](Float::non_dyadic_from_bits_prec_round) with the digits
    /// expanded, and it reads the same number of digits that that function reads bits, rounded up
    /// to a whole digit.
    ///
    /// $$
    /// f((x_k),b,p,m) = C+\varepsilon, \quad C=\sum_{k=0}^\infty x_k b^{-(k+1)}.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 C\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 C\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// # Preconditions
    /// $C$ must not be a dyadic rational: the digits must be infinite and not eventually all zero
    /// or all $b-1$. Given that, the rounded value never equals $C$, so the returned [`Ordering`]
    /// is never `Equal` and `Exact` is never a sensible rounding mode. $C$ must also be less than
    /// 1, which holds whenever the digits are read as lying wholly after the point.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `log_base` is zero or greater than 64, if a digit is greater than or equal to
    /// $2^{\ell}$, if `prec` is zero, or if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // 0.4444... in base 16 is 4/15
    /// let (x, o) = Float::non_dyadic_from_power_of_2_digits_prec_round(
    ///     core::iter::repeat(4),
    ///     4,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(x.to_string(), "0.26666641");
    /// assert_eq!(o, Less);
    /// ```
    pub fn non_dyadic_from_power_of_2_digits_prec_round<I: Iterator<Item = u64>>(
        digits: I,
        log_base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(log_base, 0);
        assert!(log_base <= u64::WIDTH);
        Self::non_dyadic_from_bits_prec_round(
            digits.flat_map(move |d| {
                assert!(
                    log_base == u64::WIDTH || d < u64::power_of_2(log_base),
                    "digit out of range"
                );
                (0..log_base).rev().map(move |i| d.get_bit(i))
            }),
            prec,
            rm,
        )
    }

    /// Returns an approximation of a real number, given the number's digits in a base that is a
    /// power of 2, rounding to nearest.
    ///
    /// See [`non_dyadic_from_power_of_2_digits_prec_round`](
    /// Float::non_dyadic_from_power_of_2_digits_prec_round) for details and preconditions.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `log_base` is zero or greater than 64, if a digit is greater than or equal to
    /// $2^{\ell}$, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::non_dyadic_from_power_of_2_digits_prec(core::iter::repeat(4), 4, 20);
    /// assert_eq!(x.to_string(), "0.26666689");
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

    /// Returns an approximation of a real number, given the number's digits in an arbitrary base.
    ///
    /// A digit in a base that is not a power of 2 does not correspond to a whole number of bits, so
    /// this reads a batch of digits, brackets $C$ between the values those digits allow, and reads
    /// more if the bracket is not yet narrow enough to determine both the rounded value and its
    /// position relative to $C$. When the base is a power of 2 it defers to
    /// [`non_dyadic_from_power_of_2_digits_prec_round`](
    /// Float::non_dyadic_from_power_of_2_digits_prec_round), which needs no such loop.
    ///
    /// $$
    /// f((x_k),b,p,m) = C+\varepsilon, \quad C=\sum_{k=0}^\infty x_k b^{-(k+1)}.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 C\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 C\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// # Preconditions
    /// $C$ must not be a dyadic rational. If it is, the bracket can never separate it from the
    /// [`Float`] that equals it and this function does not terminate. Note that this is a condition
    /// on $C$, not on the digits: in a base that is not a power of 2 a dyadic rational has a
    /// non-terminating expansion, as $1/2$ does in base 3. Given the precondition, the rounded
    /// value never equals $C$, so the returned [`Ordering`] is never `Equal` and `Exact` is never a
    /// sensible rounding mode. $C$ must also be less than 1, and the iterator must be infinite.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2, if a digit is greater than or equal to `base`, if `prec` is
    /// zero, or if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // 0.3333... in base 10 is 1/3
    /// let (x, o) = Float::non_dyadic_from_digits_prec_round(core::iter::repeat(3), 10, 20, Floor);
    /// assert_eq!(x.to_string(), "0.33333302");
    /// assert_eq!(o, Less);
    /// ```
    pub fn non_dyadic_from_digits_prec_round<I: Iterator<Item = u64>>(
        mut digits: I,
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert!(base >= 2, "base out of range");
        assert_ne!(prec, 0);
        assert_ne!(rm, Exact);
        if let Some(log_base) = base.checked_log_base_2() {
            return Self::non_dyadic_from_power_of_2_digits_prec_round(digits, log_base, prec, rm);
        }
        // Each digit carries at least `floor(log2(base))` bits, so this many digits is enough to
        // cover the precision, with a few to spare for the rounding decision.
        let per_digit = base.floor_log_base_2();
        let mut target = (prec + 8).div_ceil(per_digit);
        let base_n = Natural::from(base);
        let mut buf: Vec<u64> = Vec::new();
        loop {
            while (buf.len() as u64) < target {
                let d = digits
                    .next()
                    .expect("the digit iterator must not run out; see the preconditions");
                assert!(d < base, "digit out of range");
                buf.push(d);
            }
            let n = Natural::from_digits_desc(&base, buf.iter().copied()).unwrap();
            let power = (&base_n).pow(target);
            // C lies strictly between n / power and (n + 1) / power. The second call consumes both
            // values, so only the first has to clone.
            let (lo, o_lo) = quotient_prec_round(n.clone(), power.clone(), prec, rm);
            let (hi, o_hi) = quotient_prec_round(n + Natural::ONE, power, prec, rm);
            if lo == hi {
                // Rounding is monotonic, so `lo` is the rounding of everything in the bracket, and
                // hence of C. It only remains to place it relative to C, which the orderings of the
                // endpoints settle whenever `lo` falls outside the bracket.
                if o_lo != Greater {
                    return (lo, Less);
                }
                if o_hi != Less {
                    return (hi, Greater);
                }
            }
            target += max(4, target >> 1);
        }
    }

    /// Returns an approximation of a real number, given the number's digits in an arbitrary base,
    /// rounding to nearest.
    ///
    /// See [`non_dyadic_from_digits_prec_round`](Float::non_dyadic_from_digits_prec_round) for
    /// details and preconditions.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2, if a digit is greater than or equal to `base`, or if `prec`
    /// is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::non_dyadic_from_digits_prec(core::iter::repeat(3), 10, 20);
    /// assert_eq!(x.to_string(), "0.33333349");
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
