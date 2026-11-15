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
use crate::big_exponent_float::arithmetic::mul_rational::{
    finite_to_float, from_float_result, shift_finite,
};
use crate::big_exponent_float::exponential::exp::ziv_loop;
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{PowerOf2, Sign};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, Two, Zero as ZeroTrait,
};
use malachite_base::num::exponential::traits::CeilingLogBase2;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

// The special value of AGM(x, y), or `None` if x and y are both finite and positive.
//
// FLINT has no AGM of `arf_t`s (`arb_agm` returns an indeterminate ball for any infinite argument,
// as an enclosure of an unbounded set), so the special values are those of `Float`'s (MPFR's) AGM,
// none of which depends on a signed zero: a NaN, then an infinity (which gives infinity if the
// other argument is positive), then a zero, and only then a negative argument.
fn agm_special(x: &BigExponentFloat, y: &BigExponentFloat) -> Option<BigExponentFloat> {
    Some(match (x, y) {
        (BigExponentFloat(NaN), _) | (_, BigExponentFloat(NaN)) => BigExponentFloat::NAN,
        (BigExponentFloat(Infinity { sign: true }), z)
        | (z, BigExponentFloat(Infinity { sign: true }))
            if *z > 0u32 =>
        {
            BigExponentFloat::INFINITY
        }
        (BigExponentFloat(Infinity { .. }), _) | (_, BigExponentFloat(Infinity { .. })) => {
            BigExponentFloat::NAN
        }
        (BigExponentFloat(Zero), _) | (_, BigExponentFloat(Zero)) => BigExponentFloat::ZERO,
        (BigExponentFloat(Finite { sign: false, .. }), _)
        | (_, BigExponentFloat(Finite { sign: false, .. })) => BigExponentFloat::NAN,
        _ => return None,
    })
}

// `e` as an `i32`, if it is within `Float`'s exponent range.
fn float_range_exponent(e: &Integer) -> Option<i32> {
    let e = i32::try_from(e).ok()?;
    (Float::MIN_EXPONENT..=Float::MAX_EXPONENT)
        .contains(&e)
        .then_some(e)
}

// One AGM step, (a, b) -> (sqrt(ab), (a + b) / 2), with each operation rounded to `wp` bits with
// `Nearest`.
fn agm_step(
    a: &BigExponentFloat,
    b: &BigExponentFloat,
    wp: u64,
) -> (BigExponentFloat, BigExponentFloat) {
    let g = a.mul_prec_ref_ref(b, wp).0.sqrt_prec(wp).0;
    let s = a.add_prec_ref_ref(b, wp).0 >> 1u32;
    (g, s)
}

// The error bound of an AGM computed from k rounded AGM steps, in the form of `ziv_loop`'s `err`.
//
// The AGM is homogeneous, AGM(ca, cb) = c AGM(a, b), and increasing in each argument; so if a' and
// b' are within a relative distance η of a and b, AGM(a', b') lies between AGM((1 - η)a, (1 -
// η)b) and AGM((1 + η)a, (1 + η)b), and is within a relative distance η of AGM(a, b). Relative
// errors in the arguments are therefore not magnified. In an AGM step rounded as in `agm_step`, the
// computed arithmetic mean has a relative error of at most u/2, with u = 2^(1 - wp), and the
// computed geometric mean one of at most (1 + u/2)^(1/2)(1 + u/2) - 1 < u. Hence, after k steps the
// AGM of the computed pair is within a relative distance (1 + u)^k - 1 <= 2ku of the exact AGM,
// provided ku <= 1/2.
//
// If, as in `agm_far_approx`, a correctly rounded AGM of the computed pair is then taken, the total
// relative error is at most (1 + 2ku)(1 + u/2) - 1 <= (2k + 1)u = (4k + 2)2^(-wp). The result y
// lies within a factor of 2 of the exact AGM, so the error is below 2^(EXP(y) + 1 + ceil(log_2(4k +
// 2)) - wp).
fn far_error_bits(k: u64, wp: u64) -> u64 {
    if wp < 2 || k > u64::power_of_2((wp - 2).min(62)) {
        0
    } else {
        wp.saturating_sub(((k << 2) + 2).ceiling_log_base_2() + 1)
    }
}

// An approximation of AGM(a, b), and an `err` as in `ziv_loop`, for finite positive a < b whose
// exponents differ by more than `Float`'s exponent range: both are scaled so that b's exponent is
// 0, and rounded AGM steps are taken until the exponents of the pair differ by an amount within the
// range (each step roughly halves the difference, since the geometric mean's exponent is about the
// mean of the exponents, while the arithmetic mean is about half of the larger argument); then the
// AGM of the pair is computed with `Float`'s `agm_prec`. See `far_error_bits` for the error.
fn agm_far_approx(a: &BigExponentFloat, b: &BigExponentFloat, wp: u64) -> (BigExponentFloat, u64) {
    let e = b.get_exponent().unwrap();
    let mut a = shift_finite(a.clone(), &-e);
    let mut b = shift_finite(b.clone(), &-e);
    let mut k = 0;
    let d = loop {
        let d = a.get_exponent().unwrap() - b.get_exponent().unwrap();
        if let Some(d) = float_range_exponent(&d) {
            break d;
        }
        (a, b) = agm_step(&a, &b, wp);
        k += 1;
    };
    let shift = e + b.get_exponent().unwrap();
    let y = finite_to_float(Cow::Owned(a), d)
        .agm_prec(finite_to_float(Cow::Owned(b), 0), wp)
        .0;
    (
        shift_finite(BigExponentFloat::from(y), &shift),
        far_error_bits(k, wp),
    )
}

// AGM(x, y), rounded to `prec` bits with `rm`.
//
// The special cases are those of `agm_special`. Otherwise, if x = y, the AGM is x itself; and if x
// and y are distinct, the AGM is irrational, so `Exact` is rejected. Both arguments are scaled by
// the same power of 2, so that the larger one has exponent 0; if the smaller one's exponent is then
// within `Float`'s exponent range, the AGM is computed by `Float`'s `agm_prec_round` and scaled
// back. Its result lies between the scaled arguments, and above 2^(-31) (AGM(1, t) is about π / (2
// ln(4/t)) for a small t), so it neither overflows nor underflows. Otherwise, `agm_far_approx` is
// used in a Ziv loop.
pub(crate) fn agm_helper(
    x: Cow<'_, BigExponentFloat>,
    y: Cow<'_, BigExponentFloat>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if let Some(z) = agm_special(&x, &y) {
        return (z, Equal);
    }
    if x == y {
        return x.into_owned().round_to_prec_round(prec, rm);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat AGM");
    let (a, b) = if x < y { (x, y) } else { (y, x) };
    let e = b.get_exponent().unwrap().clone();
    if let Some(d) = float_range_exponent(&(a.get_exponent().unwrap() - &e)) {
        return from_float_result(
            finite_to_float(a, d).agm_prec_round(finite_to_float(b, 0), prec, rm),
            Some(&e),
        )
        .unwrap();
    }
    ziv_loop(prec, rm, |wp| agm_far_approx(&a, &b, wp))
}

crate_test_fn! {
    // AGM(x, y) rounded to `prec` bits with `rm`, without `Float`'s AGM: rounded AGM steps, as in
    // `agm_step`, are taken in a Ziv loop until the pair (a, b) satisfies |b - a| < 2^(EXP(b) - wp
    // + 2), and b is then the approximation. The AGM of the computed pair lies between a and b, and
    // by `far_error_bits` within a relative distance 2ku of the exact AGM, with u = 2^(1 - wp), if
    // ku <= 1/2. With the exact AGM below 2^(EXP(b) + 1), the error is below 2^(EXP(b) - wp)(4 +
    // 8k). Used to check and benchmark `agm_helper`.
    agm_prec_round_generic(
        x: &BigExponentFloat,
        y: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        assert_ne!(prec, 0);
        if let Some(z) = agm_special(x, y) {
            return (z, Equal);
        }
        if x == y {
            return x.clone().round_to_prec_round(prec, rm);
        }
        assert_ne!(rm, Exact, "Inexact BigExponentFloat AGM");
        ziv_loop(prec, rm, |wp| {
            let mut a = x.clone();
            let mut b = y.clone();
            let mut k = 0u64;
            loop {
                let close = match a.sub_prec_round_ref_ref(&b, 1, Up).0 {
                    BigExponentFloat(Zero) => true,
                    d => {
                        *d.get_exponent().unwrap() <= b.get_exponent().unwrap() - Integer::from(wp)
                            + Integer::TWO
                    }
                };
                if close {
                    break;
                }
                (a, b) = agm_step(&a, &b, wp);
                k += 1;
            }
            let err = if wp < 2 || k > u64::power_of_2((wp - 2).min(62)) {
                0
            } else {
                wp.saturating_sub(((k << 3) + 4).ceiling_log_base_2())
            };
            (b, err)
        })
    }
}

// AGM(x, y) for `Rational`s x and y, rounded to `prec` bits with `rm`.
//
// The special cases are those of `Float`'s `agm_rational_prec_round`: a zero argument gives 0, and
// otherwise a negative one gives NaN. If x and y are distinct, `Float`'s `agm_rational_prec_round`
// is used unless its result overflows or underflows (possible only if x and y have billions of
// bits). Otherwise, x and y are rounded to `wp` bits in a Ziv loop, each with a relative error of
// at most 2^(-wp), and their AGM rounded to `wp` bits: by homogeneity and monotonicity (see
// `far_error_bits`), the result is within a relative distance (1 + 2^(-wp))^2 - 1 < 2^(1 - wp) +
// 2^(-2wp) of the exact AGM, so its error is below 2^(EXP + 2 - wp).
pub(crate) fn agm_rational_helper(
    x: &Rational,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match (x.sign(), y.sign()) {
        (Equal, _) | (_, Equal) => return (BigExponentFloat::ZERO, Equal),
        (Less, _) | (_, Less) => return (BigExponentFloat::NAN, Equal),
        _ => {}
    }
    if x == y {
        return BigExponentFloat::from_rational_prec_round_ref(x, prec, rm);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat AGM");
    if let Some(result) =
        from_float_result(Float::agm_rational_prec_round_ref_ref(x, y, prec, rm), None)
    {
        return result;
    }
    ziv_loop(prec, rm, |wp| {
        let a = BigExponentFloat::from_rational_prec_ref(x, wp).0;
        let b = BigExponentFloat::from_rational_prec_ref(y, wp).0;
        (
            agm_helper(Cow::Owned(a), Cow::Owned(b), wp, Nearest).0,
            wp - 2,
        )
    })
}

// AGM(x, q) for a `BigExponentFloat` x and a `Rational` q, rounded to `prec` bits with `rm`.
//
// The special cases are those of `Float`'s `agm_float_rational_prec_round`: a NaN or infinite x
// gives NaN, unless x is infinity and q is positive; then a zero argument gives 0, and otherwise a
// negative one gives NaN. If x and q are distinct, `Float`'s `agm_float_rational_prec_round` is
// used when x's exponent is within `Float`'s exponent range, unless its result overflows or
// underflows; otherwise, q is rounded to `wp` bits in a Ziv loop, as in `agm_rational_helper`.
pub(crate) fn agm_big_exponent_float_rational_helper(
    x: Cow<'_, BigExponentFloat>,
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match &*x {
        BigExponentFloat(Infinity { sign: true }) if *q > 0u32 => {
            return (BigExponentFloat::INFINITY, Equal);
        }
        BigExponentFloat(NaN | Infinity { .. }) => return (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Zero) => return (BigExponentFloat::ZERO, Equal),
        _ if *q == 0u32 => return (BigExponentFloat::ZERO, Equal),
        _ if *x < 0u32 || *q < 0u32 => return (BigExponentFloat::NAN, Equal),
        _ => {}
    }
    if *x == *q {
        return x.into_owned().round_to_prec_round(prec, rm);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat AGM");
    if let Some(e) = float_range_exponent(x.get_exponent().unwrap())
        && let Some(result) = from_float_result(
            finite_to_float(Cow::Borrowed(&x), e)
                .agm_float_rational_prec_round_ref_ref(q, prec, rm),
            None,
        )
    {
        return result;
    }
    ziv_loop(prec, rm, |wp| {
        let b = BigExponentFloat::from_rational_prec_ref(q, wp).0;
        (
            agm_helper(Cow::Borrowed(&x), Cow::Owned(b), wp, Nearest).0,
            wp - 2,
        )
    })
}

impl BigExponentFloat {
    /// Computes the arithmetic-geometric mean (AGM) of two [`BigExponentFloat`]s, rounding the
    /// result to the specified precision and with the specified rounding mode. Both
    /// [`BigExponentFloat`]s are taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded AGM is less than, equal to, or greater than the exact AGM. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(-\infty,y,p,m)=f(x,-\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$; likewise for
    ///   $f(x,\infty,p,m)$
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$ if neither argument is `NaN` or infinite
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero or infinite
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_prec_round). Unlike that function, this function accepts
    /// arguments with any exponent.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::agm_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) =
    ///     BigExponentFloat::from(24).agm_prec_round(BigExponentFloat::from(6), 5, Floor);
    /// assert_eq!(agm.to_string(), "13.0");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) =
    ///     BigExponentFloat::from(24).agm_prec_round(BigExponentFloat::from(6), 5, Ceiling);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) =
    ///     BigExponentFloat::from(24).agm_prec_round(BigExponentFloat::from(6), 5, Nearest);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) =
    ///     BigExponentFloat::from(24).agm_prec_round(BigExponentFloat::from(6), 20, Floor);
    /// assert_eq!(agm.to_string(), "13.45816");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) =
    ///     BigExponentFloat::from(24).agm_prec_round(BigExponentFloat::from(6), 20, Ceiling);
    /// assert_eq!(agm.to_string(), "13.458176");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) =
    ///     BigExponentFloat::from(24).agm_prec_round(BigExponentFloat::from(6), 20, Nearest);
    /// assert_eq!(agm.to_string(), "13.458176");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        agm_helper(Cow::Owned(self), Cow::Owned(other), prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`BigExponentFloat`]s, rounding the
    /// result to the specified precision and with the specified rounding mode. The first
    /// [`BigExponentFloat`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded AGM is less than, equal to, or greater than the
    /// exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(-\infty,y,p,m)=f(x,-\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$; likewise for
    ///   $f(x,\infty,p,m)$
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$ if neither argument is `NaN` or infinite
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero or infinite
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_prec_round). Unlike that function, this function accepts
    /// arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) =
    ///     BigExponentFloat::from(24).agm_prec_round_val_ref(&BigExponentFloat::from(6), 5, Floor);
    /// assert_eq!(agm.to_string(), "13.0");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = BigExponentFloat::from(24).agm_prec_round_val_ref(
    ///     &BigExponentFloat::from(6),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::from(24).agm_prec_round_val_ref(
    ///     &BigExponentFloat::from(6),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::from(24).agm_prec_round_val_ref(
    ///     &BigExponentFloat::from(6),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(agm.to_string(), "13.45816");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = BigExponentFloat::from(24).agm_prec_round_val_ref(
    ///     &BigExponentFloat::from(6),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(agm.to_string(), "13.458176");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::from(24).agm_prec_round_val_ref(
    ///     &BigExponentFloat::from(6),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(agm.to_string(), "13.458176");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        agm_helper(Cow::Owned(self), Cow::Borrowed(other), prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`BigExponentFloat`]s, rounding the
    /// result to the specified precision and with the specified rounding mode. The first
    /// [`BigExponentFloat`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded AGM is less than, equal to, or greater than the
    /// exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(-\infty,y,p,m)=f(x,-\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$; likewise for
    ///   $f(x,\infty,p,m)$
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$ if neither argument is `NaN` or infinite
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero or infinite
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_prec_round). Unlike that function, this function accepts
    /// arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24)).agm_prec_round_ref_val(
    ///     BigExponentFloat::from(6),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(agm.to_string(), "13.0");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24)).agm_prec_round_ref_val(
    ///     BigExponentFloat::from(6),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24)).agm_prec_round_ref_val(
    ///     BigExponentFloat::from(6),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24)).agm_prec_round_ref_val(
    ///     BigExponentFloat::from(6),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(agm.to_string(), "13.45816");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24)).agm_prec_round_ref_val(
    ///     BigExponentFloat::from(6),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(agm.to_string(), "13.458176");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24)).agm_prec_round_ref_val(
    ///     BigExponentFloat::from(6),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(agm.to_string(), "13.458176");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        agm_helper(Cow::Borrowed(self), Cow::Owned(other), prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`BigExponentFloat`]s, rounding the
    /// result to the specified precision and with the specified rounding mode. The first
    /// [`BigExponentFloat`] is taken by reference and the second by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded AGM is less than, equal to, or greater than
    /// the exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(-\infty,y,p,m)=f(x,-\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$; likewise for
    ///   $f(x,\infty,p,m)$
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$ if neither argument is `NaN` or infinite
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero or infinite
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_prec_round). Unlike that function, this function accepts
    /// arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24)).agm_prec_round_ref_ref(
    ///     &BigExponentFloat::from(6),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(agm.to_string(), "13.0");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24)).agm_prec_round_ref_ref(
    ///     &BigExponentFloat::from(6),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24)).agm_prec_round_ref_ref(
    ///     &BigExponentFloat::from(6),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24)).agm_prec_round_ref_ref(
    ///     &BigExponentFloat::from(6),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(agm.to_string(), "13.45816");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24)).agm_prec_round_ref_ref(
    ///     &BigExponentFloat::from(6),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(agm.to_string(), "13.458176");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24)).agm_prec_round_ref_ref(
    ///     &BigExponentFloat::from(6),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(agm.to_string(), "13.458176");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        agm_helper(Cow::Borrowed(self), Cow::Borrowed(other), prec, rm)
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the arithmetic-geometric mean (AGM) of $x$ and a
    /// [`BigExponentFloat`] $y$, rounding the result to the specified precision and with the
    /// specified rounding mode. The [`BigExponentFloat`] on the right-hand side is taken by value.
    /// An [`Ordering`] is returned, indicating whether the rounded AGM is less than, equal to, or
    /// greater than the exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(-\infty,y,p,m)=f(x,-\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$; likewise for
    ///   $f(x,\infty,p,m)$
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$ if neither argument is `NaN` or infinite
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero or infinite
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_prec_round). Unlike that function, this function accepts
    /// arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign(BigExponentFloat::from(6), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "13.0");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign(BigExponentFloat::from(6), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.5");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign(BigExponentFloat::from(6), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.5");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign(BigExponentFloat::from(6), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "13.45816");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign(BigExponentFloat::from(6), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.458176");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign(BigExponentFloat::from(6), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.458176");
    /// ```
    #[inline]
    pub fn agm_prec_round_assign(&mut self, other: Self, prec: u64, rm: RoundingMode) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = agm_helper(Cow::Owned(x), Cow::Owned(other), prec, rm);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the arithmetic-geometric mean (AGM) of $x$ and a
    /// [`BigExponentFloat`] $y$, rounding the result to the specified precision and with the
    /// specified rounding mode. The [`BigExponentFloat`] on the right-hand side is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded AGM is less than,
    /// equal to, or greater than the exact AGM. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=f(-\infty,y,p,m)=f(x,-\infty,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$; likewise for
    ///   $f(x,\infty,p,m)$
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$ if neither argument is `NaN` or infinite
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero or infinite
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_prec_round). Unlike that function, this function accepts
    /// arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign_ref(&BigExponentFloat::from(6), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "13.0");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign_ref(&BigExponentFloat::from(6), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.5");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign_ref(&BigExponentFloat::from(6), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.5");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign_ref(&BigExponentFloat::from(6), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "13.45816");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign_ref(&BigExponentFloat::from(6), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.458176");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign_ref(&BigExponentFloat::from(6), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.458176");
    /// ```
    #[inline]
    pub fn agm_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = agm_helper(Cow::Owned(x), Cow::Borrowed(other), prec, rm);
        *self = result;
        o
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`BigExponentFloat`]s, rounding the
    /// result to the nearest value of the specified precision. Both [`BigExponentFloat`]s are taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded AGM is less than,
    /// equal to, or greater than the exact AGM. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(-\infty,y,p)=f(x,-\infty,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$; likewise for
    ///   $f(x,\infty,p)$
    /// - $f(0,y,p)=f(x,0,p)=0$ if neither argument is `NaN` or infinite
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero or infinite
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_prec_round). Unlike that function, this function accepts
    /// arguments with any exponent.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::agm_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = BigExponentFloat::from(24).agm_prec(BigExponentFloat::from(6), 5);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::from(24).agm_prec(BigExponentFloat::from(6), 20);
    /// assert_eq!(agm.to_string(), "13.458176");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        agm_helper(Cow::Owned(self), Cow::Owned(other), prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`BigExponentFloat`]s, rounding the
    /// result to the nearest value of the specified precision. The first [`BigExponentFloat`] is
    /// taken by value and the second by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded AGM is less than, equal to, or greater than the exact AGM. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(-\infty,y,p)=f(x,-\infty,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$; likewise for
    ///   $f(x,\infty,p)$
    /// - $f(0,y,p)=f(x,0,p)=0$ if neither argument is `NaN` or infinite
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero or infinite
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_prec_round). Unlike that function, this function accepts
    /// arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = BigExponentFloat::from(24).agm_prec_val_ref(&BigExponentFloat::from(6), 5);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::from(24).agm_prec_val_ref(&BigExponentFloat::from(6), 20);
    /// assert_eq!(agm.to_string(), "13.458176");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        agm_helper(Cow::Owned(self), Cow::Borrowed(other), prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`BigExponentFloat`]s, rounding the
    /// result to the nearest value of the specified precision. The first [`BigExponentFloat`] is
    /// taken by reference and the second by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded AGM is less than, equal to, or greater than the exact AGM. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(-\infty,y,p)=f(x,-\infty,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$; likewise for
    ///   $f(x,\infty,p)$
    /// - $f(0,y,p)=f(x,0,p)=0$ if neither argument is `NaN` or infinite
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero or infinite
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_prec_round). Unlike that function, this function accepts
    /// arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24)).agm_prec_ref_val(BigExponentFloat::from(6), 5);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) =
    ///     (&BigExponentFloat::from(24)).agm_prec_ref_val(BigExponentFloat::from(6), 20);
    /// assert_eq!(agm.to_string(), "13.458176");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        agm_helper(Cow::Borrowed(self), Cow::Owned(other), prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`BigExponentFloat`]s, rounding the
    /// result to the nearest value of the specified precision. The first [`BigExponentFloat`] is
    /// taken by reference and the second by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded AGM is less than, equal to, or greater than the exact AGM. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(-\infty,y,p)=f(x,-\infty,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$; likewise for
    ///   $f(x,\infty,p)$
    /// - $f(0,y,p)=f(x,0,p)=0$ if neither argument is `NaN` or infinite
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero or infinite
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_prec_round). Unlike that function, this function accepts
    /// arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) =
    ///     (&BigExponentFloat::from(24)).agm_prec_ref_ref(&BigExponentFloat::from(6), 5);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) =
    ///     (&BigExponentFloat::from(24)).agm_prec_ref_ref(&BigExponentFloat::from(6), 20);
    /// assert_eq!(agm.to_string(), "13.458176");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        agm_helper(Cow::Borrowed(self), Cow::Borrowed(other), prec, Nearest)
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the arithmetic-geometric mean (AGM) of $x$ and a
    /// [`BigExponentFloat`] $y$, rounding the result to the nearest value of the specified
    /// precision. The [`BigExponentFloat`] on the right-hand side is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded AGM is less than, equal to, or
    /// greater than the exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(-\infty,y,p)=f(x,-\infty,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$; likewise for
    ///   $f(x,\infty,p)$
    /// - $f(0,y,p)=f(x,0,p)=0$ if neither argument is `NaN` or infinite
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero or infinite
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_prec_round). Unlike that function, this function accepts
    /// arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(x.agm_prec_assign(BigExponentFloat::from(6), 5), Greater);
    /// assert_eq!(x.to_string(), "13.5");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(x.agm_prec_assign(BigExponentFloat::from(6), 20), Greater);
    /// assert_eq!(x.to_string(), "13.458176");
    /// ```
    #[inline]
    pub fn agm_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = agm_helper(Cow::Owned(x), Cow::Owned(other), prec, Nearest);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the arithmetic-geometric mean (AGM) of $x$ and a
    /// [`BigExponentFloat`] $y$, rounding the result to the nearest value of the specified
    /// precision. The [`BigExponentFloat`] on the right-hand side is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded AGM is less than, equal to, or
    /// greater than the exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(-\infty,y,p)=f(x,-\infty,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$; likewise for
    ///   $f(x,\infty,p)$
    /// - $f(0,y,p)=f(x,0,p)=0$ if neither argument is `NaN` or infinite
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero or infinite
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_prec_round). Unlike that function, this function accepts
    /// arguments with any exponent.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_prec_assign_ref(&BigExponentFloat::from(6), 5),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.5");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_prec_assign_ref(&BigExponentFloat::from(6), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.458176");
    /// ```
    #[inline]
    pub fn agm_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = agm_helper(Cow::Owned(x), Cow::Borrowed(other), prec, Nearest);
        *self = result;
        o
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, returning a
    /// [`BigExponentFloat`] and rounding the result to the specified precision and with the
    /// specified rounding mode. Both [`Rational`]s are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded AGM is less than, equal to, or greater than the
    /// exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::agm_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::agm_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_round(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985109");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_round(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851141");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_round(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851141");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn agm_rational_prec_round(
        x: Rational,
        y: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        agm_rational_helper(&x, &y, prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, returning a
    /// [`BigExponentFloat`] and rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`Rational`] is taken by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded AGM is less
    /// than, equal to, or greater than the exact AGM. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::agm_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985109");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851141");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851141");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn agm_rational_prec_round_val_ref(
        x: Rational,
        y: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        agm_rational_helper(&x, y, prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, returning a
    /// [`BigExponentFloat`] and rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`Rational`] is taken by reference and the second by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded AGM is less than,
    /// equal to, or greater than the exact AGM. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::agm_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985109");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851141");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851141");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn agm_rational_prec_round_ref_val(
        x: &Rational,
        y: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        agm_rational_helper(x, &y, prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, returning a
    /// [`BigExponentFloat`] and rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`Rational`] is taken by reference and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded AGM is less
    /// than, equal to, or greater than the exact AGM. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::agm_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985109");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851141");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851141");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_rational_prec_round_ref_ref(
        x: &Rational,
        y: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        agm_rational_helper(x, y, prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, returning a
    /// [`BigExponentFloat`] and rounding the result to the nearest value of the specified
    /// precision. Both [`Rational`]s are taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded AGM is less than, equal to, or greater than the exact AGM.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,y,p)=f(x,0,p)=0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::agm_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::agm_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), y.significant_bits())`.
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
    /// let (agm, o) = BigExponentFloat::agm_rational_prec(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851141");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     100,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851137022003454646968546577215");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn agm_rational_prec(x: Rational, y: Rational, prec: u64) -> (Self, Ordering) {
        agm_rational_helper(&x, &y, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, returning a
    /// [`BigExponentFloat`] and rounding the result to the nearest value of the specified
    /// precision. The first [`Rational`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded AGM is less than, equal to, or
    /// greater than the exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,y,p)=f(x,0,p)=0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::agm_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), y.significant_bits())`.
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
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_val_ref(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851141");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_val_ref(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     100,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851137022003454646968546577215");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn agm_rational_prec_val_ref(x: Rational, y: &Rational, prec: u64) -> (Self, Ordering) {
        agm_rational_helper(&x, y, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, returning a
    /// [`BigExponentFloat`] and rounding the result to the nearest value of the specified
    /// precision. The first [`Rational`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded AGM is less than, equal to, or
    /// greater than the exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,y,p)=f(x,0,p)=0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::agm_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), y.significant_bits())`.
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
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_ref_val(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851141");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_ref_val(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     100,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851137022003454646968546577215");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn agm_rational_prec_ref_val(x: &Rational, y: Rational, prec: u64) -> (Self, Ordering) {
        agm_rational_helper(x, &y, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, returning a
    /// [`BigExponentFloat`] and rounding the result to the nearest value of the specified
    /// precision. The first [`Rational`] is taken by reference and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded AGM is less than, equal to, or
    /// greater than the exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,y,p)=f(x,0,p)=0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// Unlike [its `Float` counterpart](crate::Float::agm_rational_prec_round), this function never
    /// overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), y.significant_bits())`.
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
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_ref_ref(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851141");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::agm_rational_prec_ref_ref(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     100,
    /// );
    /// assert_eq!(agm.to_string(), "0.39851137022003454646968546577215");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn agm_rational_prec_ref_ref(x: &Rational, y: &Rational, prec: u64) -> (Self, Ordering) {
        agm_rational_helper(x, y, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of a [`BigExponentFloat`] and a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode. Both
    /// arguments are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded AGM is less than, equal to, or greater than the exact AGM. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(-\infty,y,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$ if $x$ is not `NaN` or infinite
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_float_rational_prec_round). Unlike that function, this
    /// function accepts a [`BigExponentFloat`] with any exponent, and never overflows or
    /// underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::agm_big_exponent_float_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = BigExponentFloat::from(24).agm_big_exponent_float_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(agm.to_string(), "6.6568680");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = BigExponentFloat::from(24).agm_big_exponent_float_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(agm.to_string(), "6.656876");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::from(24).agm_big_exponent_float_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(agm.to_string(), "6.656876");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn agm_big_exponent_float_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        agm_big_exponent_float_rational_helper(Cow::Owned(self), &other, prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of a [`BigExponentFloat`] and a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded AGM is less than, equal to, or greater than
    /// the exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(-\infty,y,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$ if $x$ is not `NaN` or infinite
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_float_rational_prec_round). Unlike that function, this
    /// function accepts a [`BigExponentFloat`] with any exponent, and never overflows or
    /// underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = BigExponentFloat::from(24)
    ///     .agm_big_exponent_float_rational_prec_round_val_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(agm.to_string(), "6.6568680");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = BigExponentFloat::from(24)
    ///     .agm_big_exponent_float_rational_prec_round_val_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(agm.to_string(), "6.656876");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::from(24)
    ///     .agm_big_exponent_float_rational_prec_round_val_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(agm.to_string(), "6.656876");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_big_exponent_float_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        agm_big_exponent_float_rational_helper(Cow::Owned(self), other, prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of a [`BigExponentFloat`] and a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded AGM is less than, equal to, or greater than
    /// the exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(-\infty,y,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$ if $x$ is not `NaN` or infinite
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_float_rational_prec_round). Unlike that function, this
    /// function accepts a [`BigExponentFloat`] with any exponent, and never overflows or
    /// underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24))
    ///     .agm_big_exponent_float_rational_prec_round_ref_val(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(agm.to_string(), "6.6568680");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24))
    ///     .agm_big_exponent_float_rational_prec_round_ref_val(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(agm.to_string(), "6.656876");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24))
    ///     .agm_big_exponent_float_rational_prec_round_ref_val(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(agm.to_string(), "6.656876");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn agm_big_exponent_float_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        agm_big_exponent_float_rational_helper(Cow::Borrowed(self), &other, prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of a [`BigExponentFloat`] and a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference and the [`Rational`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded AGM is less than, equal to, or
    /// greater than the exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(-\infty,y,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$ if $x$ is not `NaN` or infinite
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_float_rational_prec_round). Unlike that function, this
    /// function accepts a [`BigExponentFloat`] with any exponent, and never overflows or
    /// underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24))
    ///     .agm_big_exponent_float_rational_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Floor,
    ///     );
    /// assert_eq!(agm.to_string(), "6.6568680");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24))
    ///     .agm_big_exponent_float_rational_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Ceiling,
    ///     );
    /// assert_eq!(agm.to_string(), "6.656876");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24))
    ///     .agm_big_exponent_float_rational_prec_round_ref_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Nearest,
    ///     );
    /// assert_eq!(agm.to_string(), "6.656876");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_big_exponent_float_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        agm_big_exponent_float_rational_helper(Cow::Borrowed(self), other, prec, rm)
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the arithmetic-geometric mean (AGM) of $x$ and a
    /// [`Rational`] $y$, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] on the right-hand side is taken by value. An [`Ordering`] is
    /// returned, indicating whether the rounded AGM is less than, equal to, or greater than the
    /// exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(-\infty,y,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$ if $x$ is not `NaN` or infinite
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_float_rational_prec_round). Unlike that function, this
    /// function accepts a [`BigExponentFloat`] with any exponent, and never overflows or
    /// underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_big_exponent_float_rational_prec_round_assign(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "6.6568680");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_big_exponent_float_rational_prec_round_assign(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "6.656876");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_big_exponent_float_rational_prec_round_assign(
    ///         Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "6.656876");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn agm_big_exponent_float_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = agm_big_exponent_float_rational_helper(Cow::Owned(x), &other, prec, rm);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the arithmetic-geometric mean (AGM) of $x$ and a
    /// [`Rational`] $y$, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] on the right-hand side is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded AGM is less than, equal to, or
    /// greater than the exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p,m)=f(-\infty,y,p,m)=\text{NaN}$
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$
    /// - $f(0,y,p,m)=f(x,0,p,m)=0$ if $x$ is not `NaN` or infinite
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_float_rational_prec_round). Unlike that function, this
    /// function accepts a [`BigExponentFloat`] with any exponent, and never overflows or
    /// underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the two arguments are positive and
    /// distinct (and the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_big_exponent_float_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Floor
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "6.6568680");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_big_exponent_float_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Ceiling
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "6.656876");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_big_exponent_float_rational_prec_round_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20,
    ///         Nearest
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "6.656876");
    /// ```
    #[inline]
    pub fn agm_big_exponent_float_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = agm_big_exponent_float_rational_helper(Cow::Owned(x), other, prec, rm);
        *self = result;
        o
    }

    /// Computes the arithmetic-geometric mean (AGM) of a [`BigExponentFloat`] and a [`Rational`],
    /// rounding the result to the nearest value of the specified precision. Both arguments are
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded AGM is less
    /// than, equal to, or greater than the exact AGM. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(-\infty,y,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$
    /// - $f(0,y,p)=f(x,0,p)=0$ if $x$ is not `NaN` or infinite
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_float_rational_prec_round). Unlike that function, this
    /// function accepts a [`BigExponentFloat`] with any exponent, and never overflows or
    /// underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::agm_big_exponent_float_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (agm, o) = BigExponentFloat::from(24)
    ///     .agm_big_exponent_float_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(agm.to_string(), "6.656876");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::from(24)
    ///     .agm_big_exponent_float_rational_prec(Rational::from_unsigneds(1u8, 3), 100);
    /// assert_eq!(agm.to_string(), "6.6568740624918905960055565225339");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn agm_big_exponent_float_rational_prec(
        self,
        other: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        agm_big_exponent_float_rational_helper(Cow::Owned(self), &other, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of a [`BigExponentFloat`] and a [`Rational`],
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded AGM is less than, equal to, or greater than
    /// the exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(-\infty,y,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$
    /// - $f(0,y,p)=f(x,0,p)=0$ if $x$ is not `NaN` or infinite
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_float_rational_prec_round). Unlike that function, this
    /// function accepts a [`BigExponentFloat`] with any exponent, and never overflows or
    /// underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (agm, o) = BigExponentFloat::from(24)
    ///     .agm_big_exponent_float_rational_prec_val_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(agm.to_string(), "6.656876");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = BigExponentFloat::from(24)
    ///     .agm_big_exponent_float_rational_prec_val_ref(&Rational::from_unsigneds(1u8, 3), 100);
    /// assert_eq!(agm.to_string(), "6.6568740624918905960055565225339");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn agm_big_exponent_float_rational_prec_val_ref(
        self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        agm_big_exponent_float_rational_helper(Cow::Owned(self), other, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of a [`BigExponentFloat`] and a [`Rational`],
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded AGM is less than, equal to, or greater than
    /// the exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(-\infty,y,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$
    /// - $f(0,y,p)=f(x,0,p)=0$ if $x$ is not `NaN` or infinite
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_float_rational_prec_round). Unlike that function, this
    /// function accepts a [`BigExponentFloat`] with any exponent, and never overflows or
    /// underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (agm, o) = (&BigExponentFloat::from(24))
    ///     .agm_big_exponent_float_rational_prec_ref_val(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(agm.to_string(), "6.656876");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24))
    ///     .agm_big_exponent_float_rational_prec_ref_val(Rational::from_unsigneds(1u8, 3), 100);
    /// assert_eq!(agm.to_string(), "6.6568740624918905960055565225339");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn agm_big_exponent_float_rational_prec_ref_val(
        &self,
        other: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        agm_big_exponent_float_rational_helper(Cow::Borrowed(self), &other, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of a [`BigExponentFloat`] and a [`Rational`],
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference and the [`Rational`] by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded AGM is less than, equal to, or
    /// greater than the exact AGM. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(-\infty,y,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$
    /// - $f(0,y,p)=f(x,0,p)=0$ if $x$ is not `NaN` or infinite
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_float_rational_prec_round). Unlike that function, this
    /// function accepts a [`BigExponentFloat`] with any exponent, and never overflows or
    /// underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (agm, o) = (&BigExponentFloat::from(24))
    ///     .agm_big_exponent_float_rational_prec_ref_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(agm.to_string(), "6.656876");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = (&BigExponentFloat::from(24))
    ///     .agm_big_exponent_float_rational_prec_ref_ref(&Rational::from_unsigneds(1u8, 3), 100);
    /// assert_eq!(agm.to_string(), "6.6568740624918905960055565225339");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn agm_big_exponent_float_rational_prec_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        agm_big_exponent_float_rational_helper(Cow::Borrowed(self), other, prec, Nearest)
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the arithmetic-geometric mean (AGM) of $x$ and a
    /// [`Rational`] $y$, rounding the result to the nearest value of the specified precision. The
    /// [`Rational`] on the right-hand side is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded AGM is less than, equal to, or greater than the exact AGM.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(-\infty,y,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$
    /// - $f(0,y,p)=f(x,0,p)=0$ if $x$ is not `NaN` or infinite
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_float_rational_prec_round). Unlike that function, this
    /// function accepts a [`BigExponentFloat`] with any exponent, and never overflows or
    /// underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_big_exponent_float_rational_prec_assign(Rational::from_unsigneds(1u8, 3), 20),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "6.656876");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_big_exponent_float_rational_prec_assign(Rational::from_unsigneds(1u8, 3), 100),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "6.6568740624918905960055565225339");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn agm_big_exponent_float_rational_prec_assign(
        &mut self,
        other: Rational,
        prec: u64,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) =
            agm_big_exponent_float_rational_helper(Cow::Owned(x), &other, prec, Nearest);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $x$ with the arithmetic-geometric mean (AGM) of $x$ and a
    /// [`Rational`] $y$, rounding the result to the nearest value of the specified precision. The
    /// [`Rational`] on the right-hand side is taken by reference. An [`Ordering`] is returned,
    /// indicating whether the rounded AGM is less than, equal to, or greater than the exact AGM.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(-\infty,y,p)=\text{NaN}$
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $\text{NaN}$ if $y\leq 0$
    /// - $f(0,y,p)=f(x,0,p)=0$ if $x$ is not `NaN` or infinite
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$, and neither argument is zero
    ///
    /// FLINT has no AGM of `arf_t`s, so the special cases are those of [its `Float`
    /// counterpart](crate::Float::agm_float_rational_prec_round). Unlike that function, this
    /// function accepts a [`BigExponentFloat`] with any exponent, and never overflows or
    /// underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_big_exponent_float_rational_prec_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         20
    ///     ),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "6.656876");
    ///
    /// let mut x = BigExponentFloat::from(24);
    /// assert_eq!(
    ///     x.agm_big_exponent_float_rational_prec_assign_ref(
    ///         &Rational::from_unsigneds(1u8, 3),
    ///         100
    ///     ),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "6.6568740624918905960055565225339");
    /// ```
    #[inline]
    pub fn agm_big_exponent_float_rational_prec_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) =
            agm_big_exponent_float_rational_helper(Cow::Owned(x), other, prec, Nearest);
        *self = result;
        o
    }
}
