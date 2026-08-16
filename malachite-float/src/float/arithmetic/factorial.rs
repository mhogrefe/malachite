// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 1999-2025 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use core::cmp::Ordering::{self, Equal, Greater, Less};
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, Factorial, FloorLogBase2, ShlRound,
};
use malachite_base::num::basic::traits::Infinity;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, Down, Exact, Floor, Nearest, Up};
use malachite_nz::natural::Natural;

impl Float {
    /// This is mpfr_fac_ui from factorial.c, MPFR 4.2.2, with the result's precision passed
    /// explicitly. The factorial is accumulated at a working precision a little above the target,
    /// with a directed rounding, and a Ziv loop retries at higher precision until the approximation
    /// rounds unambiguously. Where MPFR runs the loop under an extended exponent range and resolves
    /// overflow in a final mpfr_check_range, here the working value is kept scaled to a small
    /// exponent with the accumulated power of 2 tracked separately, and the final exact shift
    /// resolves overflow instead. The scale also gives an exact running lower bound on the result's
    /// exponent, so a factorial too large for any `Float` is detected mid-loop without unbounded
    /// growth.
    ///
    /// Computes the factorial of a `u64`, rounding the result to the specified precision and with
    /// the specified rounding mode. An [`Ordering`] is also returned, indicating whether the
    /// rounded factorial is less than, equal to, or greater than the exact factorial.
    ///
    /// The result is identical to `Float::from_natural_prec_round(Natural::factorial(n), prec,
    /// rm)`, but the computation works at a precision a little above `prec` throughout, which is
    /// far cheaper than computing every bit of the exact factorial when `n` is large and `prec` is
    /// small. A factorial too large for the exponent range yields the usual overflow values:
    /// infinity under `Nearest`, `Up`, and `Ceiling`, and the largest representable value under
    /// `Down` and `Floor`.
    ///
    /// $$
    /// f(n,p) = n!+\varepsilon.
    /// $$
    /// - If $n!$ is representable with $p$ bits, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 n!\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n, p) = O(n (p + \log n) \log (p + \log n) \log\log (p + \log n))$
    ///
    /// $M(p) = O(p + \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `n`, and $p$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the factorial is not exactly
    /// representable with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (f, o) = Float::factorial_prec_round(5, 4, Floor);
    /// assert_eq!(f.to_string(), "120.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (f, o) = Float::factorial_prec_round(100, 10, Floor);
    /// assert_eq!(f.to_string(), "9.3318e157");
    /// assert_eq!(o, Less);
    ///
    /// let (f, o) = Float::factorial_prec_round(100, 10, Ceiling);
    /// assert_eq!(f.to_string(), "9.3426e157");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn factorial_prec_round(n: u64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        // 0! = 1! = 1
        if n <= 1 {
            return (Self::one_prec(prec), Equal);
        }
        if rm == Exact {
            // with an inexact working value the loop cannot certify exactness, and Exact needs the
            // full value anyway
            return Self::from_natural_prec_round(Natural::factorial(n), prec, Exact);
        }
        // A cheap exact lower bound on log2(n!): the upper half of the factors alone is at least
        // (n/2)^(n/2), so log2(n!) >= (n/2)*floor(log2(n/2)). When even this bound exceeds the
        // exponent range, the factorial overflows with no computation at all; the in-loop exponent
        // bound below catches the remaining overflow window.
        let half = n >> 1;
        if u128::from(half) * u128::from(half.floor_log_base_2())
            > const { (Self::MAX_EXPONENT_I64 + 1) as u128 }
        {
            return match rm {
                Floor | Down => (Self::max_finite_value_with_prec(prec), Less),
                _ => (Self::INFINITY, Greater),
            };
        }
        let mut wprec = prec + (n.ceiling_log_base_2() << 1) + 7;
        // the working directed rounding; restarted with the symmetric direction if the two rounding
        // stages disagree in sign
        let mut rnd = Down;
        loop {
            // the value accumulated so far is t*2^k, with t's exponent held at 0
            let mut t = Self::one_prec(wprec);
            let mut k = 0i64;
            let mut o1 = Equal;
            let mut overflow = false;
            for i in 2..=n {
                let (t2, o) = t.mul_prec_round(Self::from(i), wprec, rnd);
                t = t2;
                // assume the first inexact product gives the sign of the difference
                if o1 == Equal {
                    o1 = o;
                }
                let e = i64::from(t.get_exponent().unwrap());
                if e != 0 {
                    k += e;
                    t >>= e;
                }
                // the remaining factors only increase the value, so k + 1 is a lower bound on the
                // result's exponent; once it exceeds the representable range the result is a
                // definite overflow
                if k > const { Self::MAX_EXPONENT_I64 + 1 } {
                    overflow = true;
                    break;
                }
            }
            if overflow {
                // as in mpfr_overflow: toward-zero modes give the largest finite value, and the
                // other modes give infinity
                return match rm {
                    Floor | Down => (Self::max_finite_value_with_prec(prec), Less),
                    _ => (Self::INFINITY, Greater),
                };
            }
            // t is exact, or within one ulp of its (err)th bit in the direction of rnd; this is
            // MPFR_CAN_ROUND's round_p test, whose first rounding mode is Nearest
            let err = i64::exact_from(wprec - 1 - wprec.ceiling_log_base_2());
            if o1 == Equal || t.can_round(err, Nearest, rm, prec) {
                let (y, o2) = Self::from_float_prec_round(t, prec, rm);
                let o = if o1 == Equal {
                    // t is exactly n!/2^k, so the second rounding's comparison is the answer
                    o2
                } else if o2 == Equal || o2 == o1 {
                    // y is on the same side of n!/2^k as t
                    o1
                } else {
                    // the two stages have opposite signs, so y's relation to n!/2^k is unknown:
                    // restart with the symmetric working rounding
                    rnd = if rnd == Down { Up } else { Down };
                    wprec += wprec >> 1;
                    continue;
                };
                // scale back; the exact shift saturates per rm at the exponent limit, standing in
                // for mpfr_check_range
                let (result, o_shift) = y.shl_round(k, rm);
                return if o_shift == Equal {
                    (result, o)
                } else {
                    (result, o_shift)
                };
            }
            wprec += wprec >> 1;
        }
    }

    #[inline]
    /// Computes the factorial of a `u64`, rounding the result to the nearest value of the specified
    /// precision. An [`Ordering`] is also returned, indicating whether the rounded factorial is
    /// less than, equal to, or greater than the exact factorial.
    ///
    /// If the factorial is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(n,p) = n!+\varepsilon.
    /// $$
    /// - If $n!$ is representable with $p$ bits, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 n!\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::factorial_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, p) = O(n (p + \log n) \log (p + \log n) \log\log (p + \log n))$
    ///
    /// $M(p) = O(p + \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `n`, and $p$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (f, o) = Float::factorial_prec(10, 20);
    /// assert_eq!(f.to_string(), "3628800.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (f, o) = Float::factorial_prec(20, 30);
    /// assert_eq!(f.to_string(), "2.4329020103e18");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn factorial_prec(n: u64, prec: u64) -> (Self, Ordering) {
        Self::factorial_prec_round(n, prec, Nearest)
    }
}
