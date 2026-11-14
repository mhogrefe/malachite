// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::significand_prec;
use crate::big_exponent_float::arithmetic::mul_rational::{
    finite_to_float, float_rm, from_float_result,
};
use crate::big_exponent_float::arithmetic::round_to_integer::{
    finite_is_integer, finite_odd_part_bits,
};
use crate::big_exponent_float::exponential::exp::{power_of_2_integer, round_nudged, ziv_loop};
use alloc::borrow::Cow;
use core::cmp::{
    Ordering::{self, *},
    max,
};
use malachite_base::num::arithmetic::traits::{IsPowerOf2, Pow};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;

// (1 + x)^n, rounded to `prec` bits with `rm`.
//
// FLINT has no compound function, and none of `Float`'s (MPFR's) special cases depends on a signed
// zero, so they are kept: in particular, (1 + x)^n for x = -1 and n < 0 is infinity, the one-sided
// limit from x > -1 (just as `arf_rsqrt` of 0 is infinity), even though 0^n is NaN for odd n < 0,
// where 0 may be approached from either side. For a moderate x, 1 + x is formed exactly and raised
// to the nth power, which also detects exact results. For a tiny x, (1 + x)^n is 1 plus or minus a
// tiny amount, and for a huge x it is x^n (1 + 1/x)^n, where (1 + 1/x)^n differs from 1 by far less
// than an ulp: when x^n is itself exactly representable, every approximation could be x^n, so it is
// nudged instead (see `round_nudged`).
pub(crate) fn compound_helper(
    x: &BigExponentFloat,
    n: i64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    let exponent = match x {
        BigExponentFloat(NaN) => {
            return (
                if n == 0 {
                    BigExponentFloat::ONE
                } else {
                    BigExponentFloat::NAN
                },
                Equal,
            );
        }
        BigExponentFloat(Infinity { sign: false }) => return (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Infinity { sign: true }) => {
            return (
                match n {
                    0 => BigExponentFloat::ONE,
                    n if n > 0 => BigExponentFloat::INFINITY,
                    _ => BigExponentFloat::ZERO,
                },
                Equal,
            );
        }
        BigExponentFloat(Zero) => return (BigExponentFloat::ONE, Equal),
        BigExponentFloat(Finite { exponent, .. }) => exponent,
    };
    match x.partial_cmp(&-1i32).unwrap() {
        Less => return (BigExponentFloat::NAN, Equal),
        Equal => {
            return (
                match n {
                    0 => BigExponentFloat::ONE,
                    n if n > 0 => BigExponentFloat::ZERO,
                    _ => BigExponentFloat::INFINITY,
                },
                Equal,
            );
        }
        Greater => {}
    }
    match n {
        0 => return (BigExponentFloat::ONE, Equal),
        1 => return x.add_prec_round_ref_val(BigExponentFloat::ONE, prec, rm),
        _ => {}
    }
    let n_bits = n.unsigned_abs().significant_bits();
    // If |x| < 2^e with e <= -p - 2 - bits(|n|), then |nx| < 2^(-p - 2) and |(1 + x)^n - 1| <=
    // 2|nx| < 2^(-p - 1).
    if *exponent <= -Integer::from(prec + 2 + n_bits) {
        assert_ne!(rm, Exact, "Inexact BigExponentFloat compound");
        return round_nudged(&BigExponentFloat::ONE, (*x > 0u32) == (n > 0), prec, rm);
    }
    if let Ok(e) = i32::try_from(exponent)
        && (Float::MIN_EXPONENT..=Float::MAX_EXPONENT).contains(&e)
        && let Some((y, o)) = from_float_result(
            finite_to_float(Cow::Borrowed(x), e).compound_prec_round(n, prec, float_rm(rm)),
            None,
        )
    {
        assert!(
            rm != Exact || o == Equal,
            "Inexact BigExponentFloat compound"
        );
        return (y, o);
    }
    let n_integer = Integer::from(n);
    // A moderate x: 1 + x has at most about max(p, 64) + 128 + q bits, where q is x's precision.
    if *exponent <= max(prec, 64) + 128 {
        let z = x + BigExponentFloat::ONE;
        return z.pow_integer_prec_round_val_ref(&n_integer, prec, rm);
    }
    // x > 2^(max(p, 64) + 127). (1 + x)^n is exact only if 1 + x is a power of 2 and n < 0, or (for
    // n > 0) if it has at most p bits; the latter is impossible, since 1 + x has more than p bits.
    // 1 + x is a power of 2 only if x is an odd integer, which has as many bits as its exponent, so
    // 1 + x is then cheap to form.
    if finite_is_integer(x) && *exponent == significand_prec(x.significand_ref().unwrap()) {
        let z = x + BigExponentFloat::ONE;
        if z.is_power_of_2() {
            return z.pow_integer_prec_round_val_ref(&n_integer, prec, rm);
        }
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat compound");
    // (1 + x)^n = x^n (1 + 1/x)^n, and |(1 + 1/x)^n - 1| <= 2|n|/x < 2^(66 - e) for x's exponent e,
    // far below 2^(-max(p, q) - 2) for any q <= p + 1. If x^n is a number with at most p + 1
    // significant bits (representable, or a midpoint), every approximation could be x^n itself, so
    // the Ziv loop below could never decide; then x^n is nudged: up for n > 0, since (1 + x)^n >
    // x^n, and down for n < 0. For n < 0, x^n is dyadic only if x is a power of 2. For n > 0, the
    // odd part of x^n is m^n for x's odd part m, which has a single bit if m = 1 and otherwise more
    // than n (bits(m) - 1) bits, so x^n is only formed when that could be at most p + 1.
    let odd_bits = finite_odd_part_bits(x);
    let n_abs = n.unsigned_abs();
    if n > 0
        && (odd_bits == 1
            || n_abs
                .checked_mul(odd_bits - 1)
                .is_some_and(|bits| bits < prec + 1))
    {
        let v = x.pow(n_abs);
        if finite_odd_part_bits(&v) <= prec + 1 {
            return round_nudged(&v, true, prec, rm);
        }
    }
    if n < 0 && odd_bits == 1 {
        let v = power_of_2_integer((exponent - Integer::ONE) * &n_integer);
        return round_nudged(&v, false, prec, rm);
    }
    // Otherwise x^n is not a breakpoint, and the Ziv loop terminates. x^n is correctly rounded to
    // wp + 2 bits. 1 + 1/x is computed with wp + 70 bits, and its nth power with wp + 2 bits: since
    // |n| < 2^64, the relative error of 1 + 1/x contributes less than 2^(-wp - 5). The total
    // relative error is below 2^(-wp - 2) + 2^(-wp - 2) + 2^(-wp - 5) plus the product's rounding,
    // 2^(-wp), so the error is below 2^(EXP - wp + 1).
    ziv_loop(prec, rm, |wp| {
        let power = x.pow_integer_prec_ref_ref(&n_integer, wp + 2).0;
        let r = x.reciprocal_prec_ref(wp + 70).0;
        let c = BigExponentFloat::ONE
            .add_prec(r, wp + 70)
            .0
            .pow_integer_prec_val_ref(&n_integer, wp + 2)
            .0;
        (power.mul_prec(c, wp).0, wp - 2)
    })
}

impl BigExponentFloat {
    /// Computes $(1+x)^n$, where $x$ is a [`BigExponentFloat`] and $n$ is an [`i64`], rounding the
    /// result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = (1+x)^n+\varepsilon.
    /// $$
    /// - If $(1+x)^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $(1+x)^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (1+x)^n\rfloor-p+1}$.
    /// - If $(1+x)^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (1+x)^n\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},n,p,m)=\text{NaN}$ if $n\neq 0$, and $1$ if $n=0$
    /// - $f(-\infty,n,p,m)=\text{NaN}$, even if $n=0$
    /// - $f(\infty,0,p,m)=1$
    /// - $f(\infty,n,p,m)=\infty$ if $n>0$, and $0$ if $n<0$
    /// - $f(0,n,p,m)=1$
    /// - $f(x,n,p,m)=\text{NaN}$ if $x<-1$, even if $n=0$
    /// - $f(-1,n,p,m)=1$ if $n=0$, $0$ if $n>0$, and $\infty$ if $n<0$
    /// - $f(x,0,p,m)=1$ if $x\geq -1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::compound_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::compound_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent `n`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI).compound_prec_round(3, 5, Floor);
    /// assert_eq!(format!("{y:?}"), "0x44.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).compound_prec_round(3, 5, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x48.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).compound_prec_round(3, 5, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x48.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).compound_prec_round(3, 20, Floor);
    /// assert_eq!(format!("{y:?}"), "0x47.0a3");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).compound_prec_round(3, 20, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x47.0a38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).compound_prec_round(3, 20, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x47.0a38");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn compound_prec_round(self, n: i64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        compound_helper(&self, n, prec, rm)
    }

    /// Computes $(1+x)^n$, where $x$ is a [`BigExponentFloat`] and $n$ is an [`i64`], rounding the
    /// result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = (1+x)^n+\varepsilon.
    /// $$
    /// - If $(1+x)^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $(1+x)^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (1+x)^n\rfloor-p+1}$.
    /// - If $(1+x)^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (1+x)^n\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},n,p,m)=\text{NaN}$ if $n\neq 0$, and $1$ if $n=0$
    /// - $f(-\infty,n,p,m)=\text{NaN}$, even if $n=0$
    /// - $f(\infty,0,p,m)=1$
    /// - $f(\infty,n,p,m)=\infty$ if $n>0$, and $0$ if $n<0$
    /// - $f(0,n,p,m)=1$
    /// - $f(x,n,p,m)=\text{NaN}$ if $x<-1$, even if $n=0$
    /// - $f(-1,n,p,m)=1$ if $n=0$, $0$ if $n>0$, and $\infty$ if $n<0$
    /// - $f(x,0,p,m)=1$ if $x\geq -1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::compound_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent `n`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).compound_prec_round_ref(3, 5, Floor);
    /// assert_eq!(format!("{y:?}"), "0x44.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).compound_prec_round_ref(3, 5, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x48.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).compound_prec_round_ref(3, 5, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x48.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).compound_prec_round_ref(3, 20, Floor);
    /// assert_eq!(format!("{y:?}"), "0x47.0a3");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).compound_prec_round_ref(3, 20, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x47.0a38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).compound_prec_round_ref(3, 20, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x47.0a38");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn compound_prec_round_ref(&self, n: i64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        compound_helper(self, n, prec, rm)
    }

    /// Computes $(1+x)^n$, where $x$ is a [`BigExponentFloat`] and $n$ is an [`i64`], in place,
    /// rounding the result to the specified precision and with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets (1+x)^n+\varepsilon.
    /// $$
    /// - If $(1+x)^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $(1+x)^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (1+x)^n\rfloor-p+1}$.
    /// - If $(1+x)^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (1+x)^n\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::compound_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent `n`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.compound_prec_round_assign(3, 5, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x44.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.compound_prec_round_assign(3, 5, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x48.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.compound_prec_round_assign(3, 5, Nearest), Greater);
    /// assert_eq!(format!("{x:?}"), "0x48.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.compound_prec_round_assign(3, 20, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x47.0a3");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.compound_prec_round_assign(3, 20, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x47.0a38");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.compound_prec_round_assign(3, 20, Nearest), Greater);
    /// assert_eq!(format!("{x:?}"), "0x47.0a38");
    /// ```
    #[inline]
    pub fn compound_prec_round_assign(&mut self, n: i64, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = compound_helper(self, n, prec, rm);
        *self = result;
        o
    }

    /// Computes $(1+x)^n$, where $x$ is a [`BigExponentFloat`] and $n$ is an [`i64`], rounding the
    /// result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded value is less than,
    /// equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,n,p) = (1+x)^n+\varepsilon.
    /// $$
    /// - If $(1+x)^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $(1+x)^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (1+x)^n\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},n,p)=\text{NaN}$ if $n\neq 0$, and $1$ if $n=0$
    /// - $f(-\infty,n,p)=\text{NaN}$, even if $n=0$
    /// - $f(\infty,0,p)=1$
    /// - $f(\infty,n,p)=\infty$ if $n>0$, and $0$ if $n<0$
    /// - $f(0,n,p)=1$
    /// - $f(x,n,p)=\text{NaN}$ if $x<-1$, even if $n=0$
    /// - $f(-1,n,p)=1$ if $n=0$, $0$ if $n>0$, and $\infty$ if $n<0$
    /// - $f(x,0,p)=1$ if $x\geq -1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::compound_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::compound_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent `n`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI).compound_prec(3, 5);
    /// assert_eq!(format!("{y:?}"), "0x48.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).compound_prec(3, 20);
    /// assert_eq!(format!("{y:?}"), "0x47.0a38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::power_of_2(1000000000000i64).compound_prec(3, 10);
    /// assert_eq!(format!("{y:?}"), "0x1.0E+750000000000");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn compound_prec(self, n: i64, prec: u64) -> (Self, Ordering) {
        compound_helper(&self, n, prec, Nearest)
    }

    /// Computes $(1+x)^n$, where $x$ is a [`BigExponentFloat`] and $n$ is an [`i64`], rounding the
    /// result to the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,n,p) = (1+x)^n+\varepsilon.
    /// $$
    /// - If $(1+x)^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $(1+x)^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (1+x)^n\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},n,p)=\text{NaN}$ if $n\neq 0$, and $1$ if $n=0$
    /// - $f(-\infty,n,p)=\text{NaN}$, even if $n=0$
    /// - $f(\infty,0,p)=1$
    /// - $f(\infty,n,p)=\infty$ if $n>0$, and $0$ if $n<0$
    /// - $f(0,n,p)=1$
    /// - $f(x,n,p)=\text{NaN}$ if $x<-1$, even if $n=0$
    /// - $f(-1,n,p)=1$ if $n=0$, $0$ if $n>0$, and $\infty$ if $n<0$
    /// - $f(x,0,p)=1$ if $x\geq -1$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::compound_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent `n`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).compound_prec_ref(3, 5);
    /// assert_eq!(format!("{y:?}"), "0x48.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).compound_prec_ref(3, 20);
    /// assert_eq!(format!("{y:?}"), "0x47.0a38");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn compound_prec_ref(&self, n: i64, prec: u64) -> (Self, Ordering) {
        compound_helper(self, n, prec, Nearest)
    }

    /// Computes $(1+x)^n$, where $x$ is a [`BigExponentFloat`] and $n$ is an [`i64`], in place,
    /// rounding the result to the nearest value of the specified precision. An [`Ordering`] is
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets (1+x)^n+\varepsilon.
    /// $$
    /// - If $(1+x)^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $(1+x)^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (1+x)^n\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::compound_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent `n`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.compound_prec_assign(3, 5), Greater);
    /// assert_eq!(format!("{x:?}"), "0x48.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.compound_prec_assign(3, 20), Greater);
    /// assert_eq!(format!("{x:?}"), "0x47.0a38");
    /// ```
    #[inline]
    pub fn compound_prec_assign(&mut self, n: i64, prec: u64) -> Ordering {
        let (result, o) = compound_helper(self, n, prec, Nearest);
        *self = result;
        o
    }
}
