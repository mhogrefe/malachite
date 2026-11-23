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
use crate::big_exponent_float::arithmetic::round_to_integer::{finite_is_integer, finite_odd_part};
use crate::big_exponent_float::exponential::exp::{round_nudged, ziv_loop};
use alloc::borrow::Cow;
use core::cmp::{
    Ordering::{self, *},
    max,
};
use malachite_base::num::arithmetic::traits::{IsPowerOf2, Parity, Pow};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, One, Zero as ZeroTrait,
};
use malachite_base::num::exponential::traits::CeilingLogBase2;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// The base b of a logarithm: e, 2, 10, or 2^k for a nonzero k.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LogBase {
    E,
    Two,
    Ten,
    PowerOf2(i64),
}

impl LogBase {
    // Whether log_b is decreasing, which happens when b = 2^k with k < 0. Then the infinities among
    // the special values change sign.
    const fn decreasing(self) -> bool {
        matches!(self, Self::PowerOf2(k) if k < 0)
    }

    // y log_b(2), computed with `wp`-bit arithmetic: a relative error of less than 2^(1 - wp).
    fn times_log_b_2(self, y: BigExponentFloat, wp: u64) -> BigExponentFloat {
        match self {
            Self::Two => y,
            Self::E => {
                y.mul_prec(BigExponentFloat::from(Float::ln_2_prec(wp).0), wp)
                    .0
            }
            Self::Ten => {
                y.mul_prec(BigExponentFloat::from(Float::log_10_2_prec(wp).0), wp)
                    .0
            }
            Self::PowerOf2(k) => y.div_prec(BigExponentFloat::from(k), wp).0,
        }
    }

    // y / ln(b), computed with `wp`-bit arithmetic: a relative error of less than 2^(2 - wp).
    fn divide_by_ln_b(self, y: BigExponentFloat, wp: u64) -> BigExponentFloat {
        let log_2_e = || BigExponentFloat::from(Float::log_2_e_prec(wp).0);
        match self {
            Self::E => y,
            Self::Two => y.mul_prec(log_2_e(), wp).0,
            Self::Ten => {
                y.mul_prec(BigExponentFloat::from(Float::log_10_e_prec(wp).0), wp)
                    .0
            }
            Self::PowerOf2(k) => {
                y.mul_prec(log_2_e(), wp)
                    .0
                    .div_prec(BigExponentFloat::from(k), wp)
                    .0
            }
        }
    }

    // The exact value of log_b(2^n) for an integer n, if it is rational: n for b = 2 and n / k for
    // b = 2^k. For b = e or 10 it is irrational unless n = 0, which callers exclude.
    fn of_power_of_2(self, n: Integer) -> Option<Rational> {
        match self {
            Self::Two => Some(Rational::from(n)),
            Self::PowerOf2(k) => Some(Rational::from(n) / Rational::from(k)),
            _ => None,
        }
    }
}

// The rounding of an exact rational logarithm to `prec` bits with `rm`.
fn round_exact_log(value: &Rational, prec: u64, rm: RoundingMode) -> (BigExponentFloat, Ordering) {
    BigExponentFloat::from_rational_prec_round_ref(value, prec, rm)
}

// The j for which x = 10^j, if x is a positive power of 10 with j >= 0. x = m 2^t with m odd is
// 10^j exactly when t = j and m = 5^j; 5^j is only computed when m has about as many bits (between
// 2j and 3j), so the cost is at most proportional to the size of x.
pub(crate) fn power_of_10_exponent(x: &BigExponentFloat) -> Option<u64> {
    let (m, t) = finite_odd_part(x);
    let j = u64::try_from(&t).ok()?;
    let bits = m.significant_bits();
    if j == 0 {
        return (m == 1u32).then_some(0);
    }
    if bits < j << 1 || bits > j.checked_mul(3)? {
        return None;
    }
    (m == const { Natural::const_from(5) }.pow(j)).then_some(j)
}

crate_test_fn! {
    // An approximation S of ln(1 + t) for a nonzero t with |t| <= 1/4, computed by summing the
    // Taylor series t - t^2/2 + t^3/3 - ... with `wp`-bit arithmetic, and the number of bits lost:
    // the error is at most 2^(EXP(S) - wp + loss).
    //
    // With u = 2^(-wp), the kth power of t is computed with k - 1 roundings and the kth term with
    // one more, so its relative error is at most 1.01ku; summed over all terms this is at most
    // 1.01u|t| / (1 - |t|) <= 1.34u|t|. Each partial sum is at most 1.2|t| in absolute value, so
    // the K - 1 additions contribute at most 1.2(K - 1)u|t|. The summation stops once a power of t
    // is below 2^(EXP(t) - wp) <= 2u|t|, and then the remaining terms add up to less than 0.7u|t|.
    // The total is below (1.2K + 0.9)u|t|, and since |ln(1 + t)| >= 0.85|t| for |t| <= 1/4, |t| <
    // 1.18 * 2^EXP(S); so the error is below (1.5K + 1.1) * 2^(EXP(S) - wp) <= (4K + 8) * 2^(EXP(S)
    // - wp).
    ln_1_plus_x_series(t: &BigExponentFloat, wp: u64) -> (BigExponentFloat, u64) {
        let threshold = t.get_exponent().unwrap() - Integer::from(wp);
        let mut sum = t.clone();
        let mut power = t.clone();
        let mut k = 1u64;
        loop {
            k += 1;
            power.mul_prec_assign_ref(t, wp);
            let term = power.div_prec_ref_val(BigExponentFloat::from(k), wp).0;
            if k.even() {
                sum.sub_prec_assign_ref(&term, wp);
            } else {
                sum.add_prec_assign_ref(&term, wp);
            }
            if *power.get_exponent().unwrap() <= threshold {
                return (sum, ((k << 2) + 8).ceiling_log_base_2());
            }
        }
    }
}

// An approximation of log_b(1 + t) for a nonzero t with |t| <= 1/4, given as t itself or, for a
// `Rational` t, rounded to wp + 4 bits by `t_approx`; and an `err` as in `ziv_loop`. Rounding t
// adds a relative error of at most 2^(-wp - 4), which changes ln(1 + t) by about as much,
// relatively; converting to base b adds a relative error below 2^(2 - wp).
fn log_1_plus_small_approx(
    t: &BigExponentFloat,
    base: LogBase,
    wp: u64,
) -> (BigExponentFloat, u64) {
    let (s, loss) = ln_1_plus_x_series(t, wp);
    let err = wp - loss - 1;
    if base == LogBase::E {
        (s, err)
    } else {
        (base.divide_by_ln_b(s, wp), err - 3)
    }
}

// An approximation of log_2(x) for a finite positive x whose exponent e is outside `Float`'s
// exponent range, with a relative error of less than 2^(-wp - 2). With x = m 2^e and 1/2 <= m < 1,
// log_2(x) = e + log_2(m), where |log_2(m)| <= 1 and |e| > 2^30, so there is no cancellation:
// `Float`'s correctly rounded log_2(m) with wp + 4 bits, added to e with a rounding to wp + 4 bits,
// gives a relative error below 2^(-wp - 3).
fn log_2_huge_approx(x: &BigExponentFloat, wp: u64) -> BigExponentFloat {
    let e = x.get_exponent().unwrap();
    let m = finite_to_float(Cow::Borrowed(x), 0);
    let log_2_m = BigExponentFloat::from(m.log_base_2_prec(wp + 4).0);
    BigExponentFloat::from(e.clone())
        .add_prec(log_2_m, wp + 4)
        .0
}

// An approximation of log_b(x) for a finite positive x whose exponent is outside `Float`'s exponent
// range, and an `err` as in `ziv_loop`: the relative error of log_2(x), below 2^(-wp - 2), plus
// that of the conversion to base b, below 2^(1 - wp), gives an error below 2^(EXP - wp + 2).
fn log_huge_approx(x: &BigExponentFloat, base: LogBase, wp: u64) -> (BigExponentFloat, u64) {
    (base.times_log_b_2(log_2_huge_approx(x, wp), wp), wp - 2)
}

// Whether the exponent of the finite nonzero x is within `Float`'s exponent range, and if so, the
// exponent as an `i32`.
fn float_exponent(x: &BigExponentFloat) -> Option<i32> {
    let e = i32::try_from(x.get_exponent().unwrap()).ok()?;
    (Float::MIN_EXPONENT..=Float::MAX_EXPONENT)
        .contains(&e)
        .then_some(e)
}

// The result of one of `Float`'s logarithms, converted, if it does not underflow; with `Exact`, the
// `Float` function is called with `Floor`, and the result must then be exact.
fn from_float_log(
    result: (Float, Ordering),
    rm: RoundingMode,
) -> Option<(BigExponentFloat, Ordering)> {
    let (y, o) = from_float_result(result, None)?;
    assert!(
        rm != Exact || o == Equal,
        "Inexact BigExponentFloat logarithm"
    );
    Some((y, o))
}

// The special value of log_b(x) at a non-finite or nonpositive x, or `None` for finite positive x.
// log_b(1) = 0 is included.
fn log_special(x: &BigExponentFloat, base: LogBase) -> Option<BigExponentFloat> {
    let decreasing = base.decreasing();
    Some(match x {
        BigExponentFloat(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => {
            BigExponentFloat::NAN
        }
        BigExponentFloat(Infinity { sign: true }) if decreasing => {
            BigExponentFloat::NEGATIVE_INFINITY
        }
        BigExponentFloat(Infinity { sign: true }) => BigExponentFloat::INFINITY,
        BigExponentFloat(Zero) if decreasing => BigExponentFloat::INFINITY,
        BigExponentFloat(Zero) => BigExponentFloat::NEGATIVE_INFINITY,
        x if *x == 1u32 => BigExponentFloat::ZERO,
        _ => return None,
    })
}

// log_b(x), rounded to `prec` bits with `rm`.
pub(crate) fn log_helper(
    x: Cow<'_, BigExponentFloat>,
    base: LogBase,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    assert_ne!(base, LogBase::PowerOf2(0), "Logarithm base must not be 2^0");
    if let Some(y) = log_special(&x, base) {
        return (y, Equal);
    }
    if let Some(e) = float_exponent(&x) {
        let xf = finite_to_float(Cow::Borrowed(&x), e);
        let frm = float_rm(rm);
        let result = match base {
            LogBase::E => xf.ln_prec_round(prec, frm),
            LogBase::Two => xf.log_base_2_prec_round(prec, frm),
            LogBase::Ten => xf.log_base_10_prec_round(prec, frm),
            LogBase::PowerOf2(k) => xf.log_base_power_of_2_prec_round(k, prec, frm),
        };
        if let Some(result) = from_float_log(result, rm) {
            return result;
        }
        // The result underflowed `Float`'s exponent range: x is within a tiny distance of 1 (and
        // has a huge precision). log_b(x) = log_b(1 + t) with t = x - 1, computed exactly.
        let t = x.into_owned() - BigExponentFloat::ONE;
        return log_1_plus_x_helper(&t, base, prec, rm);
    }
    // The exact cases: log_2 and log_(2^k) of a power of 2, and log_10 of a power of 10.
    if significand_prec(x.significand_ref().unwrap()) == 1
        && let Some(value) = base.of_power_of_2(x.get_exponent().unwrap() - Integer::ONE)
    {
        return round_exact_log(&value, prec, rm);
    }
    if base == LogBase::Ten
        && let Some(j) = power_of_10_exponent(&x)
    {
        return round_exact_log(&Rational::from(j), prec, rm);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat logarithm");
    ziv_loop(prec, rm, |wp| log_huge_approx(&x, base, wp))
}

// log_b(1 + x), rounded to `prec` bits with `rm`.
pub(crate) fn log_1_plus_x_helper(
    x: &BigExponentFloat,
    base: LogBase,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    assert_ne!(base, LogBase::PowerOf2(0), "Logarithm base must not be 2^0");
    let decreasing = base.decreasing();
    let (exponent, significand) = match x {
        BigExponentFloat(NaN | Infinity { sign: false }) => {
            return (BigExponentFloat::NAN, Equal);
        }
        BigExponentFloat(Infinity { sign: true }) => {
            return (
                if decreasing {
                    BigExponentFloat::NEGATIVE_INFINITY
                } else {
                    BigExponentFloat::INFINITY
                },
                Equal,
            );
        }
        BigExponentFloat(Zero) => return (BigExponentFloat::ZERO, Equal),
        BigExponentFloat(Finite {
            exponent,
            significand,
            ..
        }) => (exponent, significand),
    };
    if *x <= -1i32 {
        return (
            if *x < -1i32 {
                BigExponentFloat::NAN
            } else if decreasing {
                BigExponentFloat::INFINITY
            } else {
                BigExponentFloat::NEGATIVE_INFINITY
            },
            Equal,
        );
    }
    // For 0 < |x| <= 1/2, ln(1 + x) = x - d with 0 < d < x^2 < 2^(2e), where e is x's exponent. If
    // e <= -max(p, q) - 2, where q is x's precision, then d < 2^(e - max(p, q) - 2).
    if base == LogBase::E
        && *exponent <= -Integer::from(max(prec, significand_prec(significand)) + 2)
    {
        assert_ne!(rm, Exact, "Inexact BigExponentFloat logarithm");
        return round_nudged(x, false, prec, rm);
    }
    if let Some(e) = float_exponent(x) {
        let xf = finite_to_float(Cow::Borrowed(x), e);
        let frm = float_rm(rm);
        let result = match base {
            LogBase::E => xf.ln_1_plus_x_prec_round(prec, frm),
            LogBase::Two => xf.log_base_2_1_plus_x_prec_round(prec, frm),
            LogBase::Ten => xf.log_base_10_1_plus_x_prec_round(prec, frm),
            LogBase::PowerOf2(k) => xf.log_base_power_of_2_1_plus_x_prec_round(k, prec, frm),
        };
        if let Some(result) = from_float_log(result, rm) {
            return result;
        }
        // The result underflowed `Float`'s exponent range, so x is tiny; it is handled below.
    }
    if *exponent < 0u32 {
        // x is tiny: near or below the bottom of `Float`'s exponent range, so log_b(1 + x) is
        // irrational.
        assert_ne!(rm, Exact, "Inexact BigExponentFloat logarithm");
        return ziv_loop(prec, rm, |wp| log_1_plus_small_approx(x, base, wp));
    }
    // x is positive and above `Float`'s exponent range. log_b(1 + x) is rational only if 1 + x is a
    // power of 2 (for b = 2 or 2^k) or of 10 (for b = 10), which requires x to be an odd integer:
    // then x has as many bits as its exponent, so forming 1 + x costs no more than reading x.
    if finite_is_integer(x) && *exponent == significand_prec(significand) {
        let z = x + BigExponentFloat::ONE;
        if z.is_power_of_2()
            && let Some(value) = base.of_power_of_2(z.get_exponent().unwrap() - Integer::ONE)
        {
            return round_exact_log(&value, prec, rm);
        }
        if base == LogBase::Ten
            && let Some(j) = power_of_10_exponent(&z)
        {
            return round_exact_log(&Rational::from(j), prec, rm);
        }
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat logarithm");
    // If x = 2^n and b is 2 or 2^k, log_b(1 + x) = v + d with v = log_b(x) = n or n / k rational
    // and d = log_b(1 + 2^(-n)), where 0 < |d| < 2^(1 - n). If v is dyadic, every approximation of
    // log_b(x) is v itself, so the Ziv loop below could never decide the rounding. Instead, when
    // |d| is small enough for `round_nudged` (n >= max(p, q) - EXP(v) + 3, where q is v's
    // precision), v is nudged toward the sign of d; otherwise (only at precisions near n), d is
    // computed from the series and added.
    if significand_prec(significand) == 1
        && let Some(v) = base.of_power_of_2(exponent - Integer::ONE)
        && let Ok(v) = BigExponentFloat::try_from(&v)
    {
        let n = exponent - Integer::ONE;
        let BigExponentFloat(Finite {
            exponent: v_exponent,
            significand: v_significand,
            ..
        }) = &v
        else {
            unreachable!()
        };
        let q = significand_prec(v_significand);
        if n >= Integer::from(max(prec, q) + 3) - v_exponent {
            return round_nudged(&v, !decreasing, prec, rm);
        }
        let r = BigExponentFloat::from_parts(true, -&n + Integer::ONE, Natural::ONE);
        return ziv_loop(prec, rm, |wp| {
            let (d, err) = log_1_plus_small_approx(&r, base, wp);
            (v.add_prec_ref_val(d, wp).0, err.min(wp) - 2)
        });
    }
    // log_b(1 + x) = log_b(x) + log_b(1 + 1/x), and log_b(1 + 1/x) = (1/x - d) / ln(b) with 0 < d <
    // 1/x^2, which is far below the error of the approximation of log_b(x) (1/x < 2^(-2^30)). 1/x
    // is added (after conversion to base b), and d is accounted for by one more lost bit.
    ziv_loop(prec, rm, |wp| {
        let (y, err) = log_huge_approx(x, base, wp);
        let r = base.divide_by_ln_b(x.reciprocal_prec_ref(wp).0, wp);
        (y.add_prec(r, wp).0, err - 2)
    })
}

// log_b(x) for a `Rational` x, rounded to `prec` bits with `rm`. `Float`'s functions handle every
// x, since log_b(x) never overflows; when the result underflows, x is within a tiny distance of 1,
// and log_b(x) = log_b(1 + t) with t = x - 1 is computed from the series, with t rounded to wp + 4
// bits.
pub(crate) fn log_rational_helper(
    x: &Rational,
    base: LogBase,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    assert_ne!(base, LogBase::PowerOf2(0), "Logarithm base must not be 2^0");
    if *x <= 0u32 {
        return (
            if *x < 0u32 {
                BigExponentFloat::NAN
            } else if base.decreasing() {
                BigExponentFloat::INFINITY
            } else {
                BigExponentFloat::NEGATIVE_INFINITY
            },
            Equal,
        );
    }
    let frm = float_rm(rm);
    let result = match base {
        LogBase::E => Float::ln_rational_prec_round_ref(x, prec, frm),
        LogBase::Two => Float::log_base_2_rational_prec_round_ref(x, prec, frm),
        LogBase::Ten => Float::log_base_10_rational_prec_round_ref(x, prec, frm),
        LogBase::PowerOf2(k) => Float::log_base_power_of_2_rational_prec_round_ref(x, k, prec, frm),
    };
    if let Some(result) = from_float_log(result, rm) {
        return result;
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat logarithm");
    let t = x - Rational::ONE;
    ziv_loop(prec, rm, |wp| {
        let t = BigExponentFloat::from_rational_prec_ref(&t, wp + 4).0;
        let (y, err) = log_1_plus_small_approx(&t, base, wp);
        (y, err - 1)
    })
}

// ln(n) for a `u64` n, rounded to `prec` bits with `rm`. `Float`'s function handles every n.
pub(crate) fn ln_unsigned_helper(
    n: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if n == 0 {
        return (BigExponentFloat::NEGATIVE_INFINITY, Equal);
    }
    from_float_log(Float::ln_unsigned_prec_round(n, prec, float_rm(rm)), rm).unwrap()
}

impl BigExponentFloat {
    /// Computes the natural logarithm of a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded logarithm is less
    /// than, equal to, or greater than the exact logarithm. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln x+\varepsilon.
    /// $$
    /// - If $\ln x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln x|\rfloor-p+1}$.
    /// - If $\ln x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=-\infty$
    /// - $f(1,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_prec_round), this function accepts
    /// arguments with any exponent.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::ln_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every finite positive input other than 1).
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI).ln_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).ln_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).ln_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).ln_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "1.144730");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).ln_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "1.1447315");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).ln_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "1.144730");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        log_helper(Cow::Owned(self), LogBase::E, prec, rm)
    }

    /// Computes the natural logarithm of a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded logarithm is
    /// less than, equal to, or greater than the exact logarithm. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln x+\varepsilon.
    /// $$
    /// - If $\ln x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln x|\rfloor-p+1}$.
    /// - If $\ln x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=-\infty$
    /// - $f(1,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_prec_round), this function accepts
    /// arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every finite positive input other than 1).
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "1.144730");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "1.1447315");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "1.144730");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        log_helper(Cow::Borrowed(self), LogBase::E, prec, rm)
    }

    /// Computes the natural logarithm of a [`BigExponentFloat`], in place, rounding the result to
    /// the specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded logarithm is less than, equal to, or greater than the exact
    /// logarithm. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \ln x+\varepsilon.
    /// $$
    /// - If $\ln x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln x|\rfloor-p+1}$.
    /// - If $\ln x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln x|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_prec_round), this function accepts
    /// arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every finite positive input other than 1).
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.ln_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.12");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.ln_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.ln_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.12");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.ln_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.144730");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.ln_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.1447315");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.ln_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.144730");
    /// ```
    #[inline]
    pub fn ln_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = log_helper(Cow::Owned(x), LogBase::E, prec, rm);
        *self = result;
        o
    }

    /// Computes the natural logarithm of a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded logarithm is less than, equal
    /// to, or greater than the exact logarithm. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \ln x+\varepsilon.
    /// $$
    /// - If $\ln x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0,p)=-\infty$
    /// - $f(1,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_prec_round), this function accepts
    /// arguments with any exponent.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::ln_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
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
    /// let (y, o) = BigExponentFloat::from(PI).ln_prec(5);
    /// assert_eq!(y.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).ln_prec(20);
    /// assert_eq!(y.to_string(), "1.144730");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::power_of_2(1000000000000i64).ln_prec(10);
    /// assert_eq!(y.to_string(), "6.936e11");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_prec(self, prec: u64) -> (Self, Ordering) {
        log_helper(Cow::Owned(self), LogBase::E, prec, Nearest)
    }

    /// Computes the natural logarithm of a [`BigExponentFloat`], rounding the result to the nearest
    /// value of the specified precision. The [`BigExponentFloat`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded logarithm is less than, equal
    /// to, or greater than the exact logarithm. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \ln x+\varepsilon.
    /// $$
    /// - If $\ln x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0,p)=-\infty$
    /// - $f(1,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_prec_round), this function accepts
    /// arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
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
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_prec_ref(5);
    /// assert_eq!(y.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).ln_prec_ref(20);
    /// assert_eq!(y.to_string(), "1.144730");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        log_helper(Cow::Borrowed(self), LogBase::E, prec, Nearest)
    }

    /// Computes the natural logarithm of a [`BigExponentFloat`], in place, rounding the result to
    /// the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded logarithm is less than, equal to, or greater than the exact logarithm.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets \ln x+\varepsilon.
    /// $$
    /// - If $\ln x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   x|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_prec_round), this function accepts
    /// arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
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
    /// assert_eq!(x.ln_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "1.12");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.ln_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.144730");
    /// ```
    #[inline]
    pub fn ln_prec_assign(&mut self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = log_helper(Cow::Owned(x), LogBase::E, prec, Nearest);
        *self = result;
        o
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes the natural logarithm of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded logarithm is less than, equal to, or greater than the exact
    /// logarithm. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln x+\varepsilon.
    /// $$
    /// - If $\ln x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln x|\rfloor-p+1}$.
    /// - If $\ln x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=-\infty$
    /// - $f(1,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_rational_prec_round), this function's
    /// result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::ln_rational_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every positive input other than 1).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::ln_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(y.to_string(), "-0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::ln_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(y.to_string(), "-0.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::ln_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Nearest);
    /// assert_eq!(y.to_string(), "-0.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::ln_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(y.to_string(), "-0.5108261");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::ln_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(y.to_string(), "-0.51082516");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::ln_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Nearest);
    /// assert_eq!(y.to_string(), "-0.51082516");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        log_rational_helper(&x, LogBase::E, prec, rm)
    }

    /// Computes the natural logarithm of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded logarithm is less than, equal to, or greater than
    /// the exact logarithm. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln x+\varepsilon.
    /// $$
    /// - If $\ln x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln x|\rfloor-p+1}$.
    /// - If $\ln x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=-\infty$
    /// - $f(1,p,m)=0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_rational_prec_round), this function's
    /// result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every positive input other than 1).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::ln_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "-0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ln_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "-0.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ln_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "-0.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ln_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "-0.5108261");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ln_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "-0.51082516");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ln_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "-0.51082516");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        log_rational_helper(x, LogBase::E, prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes the natural logarithm of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`BigExponentFloat`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded logarithm is less than, equal to, or greater than the exact logarithm. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \ln x+\varepsilon.
    /// $$
    /// - If $\ln x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=-\infty$
    /// - $f(1,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_rational_prec_round), this function's
    /// result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::ln_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::ln_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(y.to_string(), "-0.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ln_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(y.to_string(), "-0.51082516");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        log_rational_helper(&x, LogBase::E, prec, Nearest)
    }

    /// Computes the natural logarithm of a [`Rational`], rounding the result to the nearest value
    /// of the specified precision and returning the result as a [`BigExponentFloat`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded logarithm is less than, equal to, or greater than the exact logarithm. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \ln x+\varepsilon.
    /// $$
    /// - If $\ln x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=-\infty$
    /// - $f(1,p)=0$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_rational_prec_round), this function's
    /// result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::ln_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(y.to_string(), "-0.50");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ln_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(y.to_string(), "-0.51082516");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        log_rational_helper(x, LogBase::E, prec, Nearest)
    }

    /// Computes the natural logarithm of a [`u64`], rounding the result to the specified precision
    /// and with the specified rounding mode and returning the result as a [`BigExponentFloat`]. An
    /// [`Ordering`] is also returned, indicating whether the rounded logarithm is less than, equal
    /// to, or greater than the exact logarithm.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(n,p,m) = \ln n+\varepsilon.
    /// $$
    /// - If $\ln n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \ln n\rfloor-p+1}$.
    /// - If $\ln n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \ln n\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=-\infty$
    /// - $f(1,p,m)=0$
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::ln_unsigned_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits (which is the case for every $n$ other than 1).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::ln_unsigned_prec_round(10, 5, Floor);
    /// assert_eq!(y.to_string(), "2.25");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ln_unsigned_prec_round(10, 5, Ceiling);
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ln_unsigned_prec_round(10, 5, Nearest);
    /// assert_eq!(y.to_string(), "2.25");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ln_unsigned_prec_round(10, 20, Floor);
    /// assert_eq!(y.to_string(), "2.302582");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ln_unsigned_prec_round(10, 20, Ceiling);
    /// assert_eq!(y.to_string(), "2.3025856");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ln_unsigned_prec_round(10, 20, Nearest);
    /// assert_eq!(y.to_string(), "2.3025856");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_unsigned_prec_round(n: u64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        ln_unsigned_helper(n, prec, rm)
    }

    /// Computes the natural logarithm of a [`u64`], rounding the result to the nearest value of the
    /// specified precision and returning the result as a [`BigExponentFloat`]. An [`Ordering`] is
    /// also returned, indicating whether the rounded logarithm is less than, equal to, or greater
    /// than the exact logarithm.
    ///
    /// $$
    /// f(n,p) = \ln n+\varepsilon.
    /// $$
    /// - If $\ln n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $\ln n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 \ln
    ///   n\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=-\infty$
    /// - $f(1,p)=0$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::ln_unsigned_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::ln_unsigned_prec(10, 5);
    /// assert_eq!(y.to_string(), "2.25");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ln_unsigned_prec(10, 20);
    /// assert_eq!(y.to_string(), "2.3025856");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn ln_unsigned_prec(n: u64, prec: u64) -> (Self, Ordering) {
        ln_unsigned_helper(n, prec, Nearest)
    }
}
