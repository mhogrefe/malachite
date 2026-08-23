// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2021-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Infinity, NaN, Zero};
use crate::float::arithmetic::exp::{exp_overflow, exp_underflow};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, Compound, CompoundAssign, Sign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;

// Rounds (1+x)^n to `prec` bits, assuming |(1+x)^n - 1| < (1/4)ulp(1) = 2^(-prec-2), where `s_pos`
// is the sign of n*log2(1+x) (true if positive; that quantity is nonzero here).
//
// This is mpfr_compound_near_one from compound.c, MPFR 4.2.2.
fn compound_near_one(prec: u64, s_pos: bool, rm: RoundingMode) -> (Float, Ordering) {
    let mut y = Float::one_prec(prec);
    match rm {
        Exact => panic!("compound: Exact rounding was requested, but the result is inexact"),
        // round toward 1
        Nearest => (y, if s_pos { Less } else { Greater }),
        Down | Floor if s_pos => (y, Less),
        Up | Ceiling if !s_pos => (y, Greater),
        // round toward +Inf
        Up | Ceiling => {
            y.increment();
            (y, Greater)
        }
        // necessarily Down or Floor with a negative sign; round toward 0
        _ => {
            y.decrement();
            (y, Less)
        }
    }
}

// A shortcut for cases where Ziv's strategy may take too much memory and be too long, i.e. when x^n
// fits in the target precision (+ 1 additional bit for rounding to nearest) and the exact result
// (1+x)^n is very close to x^n. Necessarily, x is a large even integer and n > 1. The kx < ex test
// checks that x is an even integer (iff its least bit 1 has exponent >= 1), and the test after it
// is a simple condition that implies that x^n fits in the target precision. Here are the details:
// let k be the minimum length of the significand of x, and x' the odd (integer) significand of x.
// This means that 2^(k-1) <= x' < 2^k. Thus 2^(n*(k-1)) <= (x')^n < 2^(k*n), and x^n has between
// n*(k-1)+1 and k*n bits. So x^n can fit into p bits only if p >= n*(k-1)+1, i.e. n*(k-1) <= p-1.
//
// This is the "check if x^n fits" portion of mpfr_compound_si from compound.c, MPFR 4.2.2.
fn compound_x_n_fits(
    x: &Float,
    n: i64,
    prec: u64,
    rm: RoundingMode,
    wprec: u64,
) -> Option<(Float, Ordering)> {
    let ex = i64::from(x.get_exponent().unwrap());
    if ex < 17 {
        return None;
    }
    let kx = x.get_min_prec().unwrap();
    let p = prec + u64::from(rm == Nearest);
    if kx >= u64::exact_from(ex)
        || u128::from(n.unsigned_abs()) * u128::from(kx - 1) > u128::from(p - 1)
    {
        return None;
    }
    // Check whether x^n really fits into p bits.
    let (v, o_v) = x.pow_u_prec_round_ref(u64::exact_from(n), p, Down);
    if o_v != Equal {
        return None;
    }
    // (x+1)^n = x^n * (1 + 1/x)^n For directed rounding, we can round when (1 + 1/x)^n < 1 + 2^-p,
    // and then the result is x^n, except for rounding up. Indeed, if (1 + 1/x)^n < 1 + 2^-p, 1 <=
    // (x+1)^n < x^n * (1 + 2^-p) = x^n + x^n/2^p < x^n + ulp(x^n). For rounding to nearest, we can
    // round when (1 + 1/x)^n < 1 + 2^-p, and then the result is x^n when x^n fits into p-1 bits,
    // and nextabove(x^n) otherwise.
    let mut r = x.reciprocal_prec_round_ref(wprec, Up).0;
    r.add_prec_round_assign(Float::ONE, wprec, Up);
    r.pow_u_round_assign(u64::exact_from(n), Up);
    r.sub_prec_round_assign(Float::ONE, wprec, Up);
    // r cannot be zero
    if i64::from(r.get_exponent().unwrap()) >= -i64::exact_from(prec) {
        return None;
    }
    let v_min_prec = v.get_min_prec().unwrap();
    let mut y = Float::from_float_prec_round(v, prec, Down).0;
    Some(
        if (rm == Nearest && v_min_prec == p) || rm == Up || rm == Ceiling {
            // round up
            y.increment();
            (y, Greater)
        } else {
            (y, Less)
        },
    )
}

// This is mpfr_compound_si from compound.c, MPFR 4.2.2, with two corrections taken from the MPFR
// development sources: log2p1 is rounded toward zero unconditionally (4.2.2 chooses the direction
// from the signs of x and n, which is backwards for negative n and can yield a result off by one
// ulp in the min_prec escape below -- confirmed against 4.2.2 via rug and against exact rational
// arithmetic), and the rounding tests are skipped when e >= precu (when the error bound on u is too
// large to say anything). MPFR also runs the computation in its extended exponent range and maps
// back at the end via mpfr_check_range; we instead cut overflow and underflow against the real
// exponent range up front. This is safe because u is rounded toward zero (making the cuts sound),
// and because 2^u is rounded toward 1, which keeps the intermediate t representable whenever u
// survives the cuts.
fn compound_prec_round_helper(x: &Float, n: i64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    // Special cases
    match x {
        // compound(-Inf, n) is NaN, even for n == 0
        Float(Infinity { sign: false }) => return (Float::NAN, Equal),
        // compound(NaN, 0) = 1, like compound(x, 0) for any x >= -1; otherwise NaN propagates
        Float(NaN) => {
            return if n == 0 {
                (Float::one_prec(prec), Equal)
            } else {
                (Float::NAN, Equal)
            };
        }
        // compound(0, n) = 1
        Float(Zero { .. }) => return (Float::one_prec(prec), Equal),
        // compound(+Inf, 0) = 1, and otherwise (1 + Inf)^n is +0 for n < 0 and +Inf for n > 0
        Float(Infinity { .. }) => {
            return match n.sign() {
                Equal => (Float::one_prec(prec), Equal),
                Less => (Float::ZERO, Equal),
                Greater => (Float::INFINITY, Equal),
            };
        }
        _ => {}
    }
    // (1+x)^n = NaN for x < -1
    let compared = x.partial_cmp(&-1i32).unwrap();
    if compared == Less {
        return (Float::NAN, Equal);
    }
    // compound(x, 0) gives 1 for x >= -1
    if n == 0 {
        return (Float::one_prec(prec), Equal);
    }
    if compared == Equal {
        return if n < 0 {
            // compound(-1, n) = +Inf (MPFR also raises the divide-by-zero exception)
            (Float::INFINITY, Equal)
        } else {
            // compound(-1, n) = +0
            (Float::ZERO, Equal)
        };
    }
    if n == 1 {
        return x.add_prec_round_ref_val(Float::ONE, prec, rm);
    }
    let mut wprec = prec + prec.ceiling_log_base_2() + 6;
    // |n| <= 2^k
    let k = i64::exact_from(n.unsigned_abs().ceiling_log_base_2());
    let nf = Float::from(n);
    // We compute u = log2p1(x) with wprec + extra bits, since we lose some bits in 2^u.
    let mut extra = 0u64;
    let mut increment = Limb::WIDTH;
    let mut nloop = 0u32;
    let t = loop {
        let precu = wprec + extra;
        // We compute (1+x)^n as 2^(n*log2p1(x)), and we round toward 1, thus we round n*log2p1(x)
        // toward 0, which implies we round log2p1(x) toward 0. lg is nonzero and cannot underflow:
        // |log2(1+x)| > |x| >= 2^(MIN_EXPONENT-1), and toward-zero rounding cannot take it below
        // the minimum positive Float.
        let (lg, o_lg) = x.log_base_2_1_plus_x_prec_round_ref(precu, Down);
        let mut inex = o_lg != Equal;
        let mut e = i64::from(lg.get_exponent().unwrap());
        // |lg - log2(1+x)| <= ulp(lg) = 2^(e-precu)
        let (u, o_mul) = lg.mul_prec_round_val_ref(&nf, precu, Down);
        inex |= o_mul != Equal;
        // u is nonzero: |lg| >= 2^(MIN_EXPONENT-1) and |n| >= 1, and the toward-zero rounding of
        // the product cannot reach below the minimum positive Float.
        let e2 = i64::from(u.get_exponent().unwrap());
        // ```
        // |u - n*log2(1+x)| <= 2^(e2-precu) + |n|*2^(e-precu)
        //                   <= 2^(e2-precu) + 2^(e+k-precu) <= 2^(e+k+1-precu)
        // ``` where |n| <= 2^k, and e2 is the new exponent of u.
        debug_assert!(e2 <= e + k);
        e += k + 1;
        let new_extra = if e2 > 0 { u64::exact_from(e2) } else { 0 };
        // |u - n*log2(1+x)| <= 2^(e-precu) detect overflow: since we rounded n*log2p1(x) toward 0,
        // if n*log2p1(x) >= MAX_EXPONENT, we are sure there is overflow.
        if u >= Float::MAX_EXPONENT {
            return exp_overflow(prec, rm);
        }
        // detect underflow: similarly, since we rounded n*log2p1(x) toward 0, if n*log2p1(x) <
        // MIN_EXPONENT - 1, we are sure there is underflow.
        if u < const { Float::MIN_EXPONENT - 1 } {
            return exp_underflow(prec, if rm == Nearest { Down } else { rm });
        }
        // Detect cases where the result is 1 or 1+ulp(1) or 1-(1/2)ulp(1): |2^u - 1| =
        // |exp(u*log(2)) - 1| <= |u|*log(2) < |u|
        if nloop == 0 && e2 < -i64::exact_from(prec) {
            // since ulp(1) = 2^(1-prec), we have |u| < (1/4)ulp(1)
            return compound_near_one(prec, u.is_sign_positive(), rm);
        }
        // round 2^u toward 1
        let rnd2 = if u.is_sign_positive() { Floor } else { Ceiling };
        let (mut t, o_exp2) = Float::power_of_2_of_float_prec_round(u, wprec, rnd2);
        inex |= o_exp2 != Equal;
        // we had |u - n*log2(1+x)| < 2^(e-precu), thus u = n*log2(1+x) + delta with |delta| <
        // 2^(e-precu), then 2^u = (1+x)^n * 2^delta. For |delta| < 0.5, |2^delta - 1| <= |delta|
        // thus |t - (1+x)^n| <= ulp(t) + |t|*2^(e-precu) < 2^(EXP(t)-wprec) + 2^(EXP(t)+e-precu) If
        // e >= precu, the rounding error on u is too large, and we have to loop again (though the
        // escapes below may still exit the loop).
        if e < i64::exact_from(precu) {
            let extra_i = i64::exact_from(precu - wprec);
            let err = if extra_i >= e { 1 } else { e + 1 - extra_i };
            // now |t - (1+x)^n| < 2^(EXP(t)+err-wprec)
            if !inex
                || (rm != Exact
                    && i64::exact_from(wprec) > err
                    && float_can_round(
                        t.significand_ref().unwrap(),
                        wprec - u64::exact_from(err),
                        prec,
                        rm,
                    ))
            {
                break t;
            }
            // If t fits in the target precision (or with 1 more bit), then we can round, assuming
            // the working precision is large enough, but the above float_can_round will fail
            // because we cannot determine the ternary value. However, since we rounded t toward 1,
            // we can determine it. Since the error in the approximation t is at most 2^err ulp(t),
            // this error should be less than (1/2)ulp(y), thus we should have wprec - prec >= err +
            // 1. (For Exact rounding we skip this escape, since nudging t would turn an
            // exactly-representable result into a spurious panic; the exact-1+x escape below
            // decides exactness instead.)
            if rm != Exact
                && t.get_min_prec().unwrap() <= prec + 1
                && i64::exact_from(wprec - prec) > err
            {
                // we step t one place away from 1 to get the correct rounding
                if rnd2 == Floor {
                    // t was rounded downwards. t cannot be the largest finite significand (its
                    // min_prec is at most prec + 1 < wprec), so this cannot overflow.
                    t.increment();
                    break t;
                }
                if t.get_min_prec() != Some(1) || t.get_exponent() != Some(Float::MIN_EXPONENT) {
                    t.decrement();
                    break t;
                }
                // Otherwise t is the minimum positive Float, and stepping below it would leave the
                // representable exponent range. (In MPFR's extended exponent range the step and the
                // final rounding happen normally, and mpfr_check_range then maps the result back;
                // the following resolution is equivalent.) The true result lies strictly below t --
                // t was rounded toward 1 and inex holds, so some rounding was strict -- but within
                // half an ulp of the target precision, so the rounding resolves directly.
                return match rm {
                    Floor | Down => (Float::ZERO, Less),
                    // Ceiling, Up, or Nearest; rm is not Exact here
                    _ => (Float::min_positive_value_prec(prec), Greater),
                };
            }
        }
        // Detect particular cases where Ziv's strategy may take too much memory and be too long.
        // Since this does not depend on the working precision, we only check this at the first
        // iteration.
        debug_assert!(!(0..=1).contains(&n));
        if nloop == 0
            && n > 1
            && let Some(result) = compound_x_n_fits(x, n, prec, rm, wprec)
        {
            return result;
        }
        // Exact cases like compound(0.5, 2) = 9/4 must be detected, since except for 1+x a power of
        // 2, the log2p1 above will be inexact, so that in the Ziv test, inex != 0 and
        // float_can_round will fail (even for Nearest, as the ternary value cannot be determined),
        // yielding an infinite loop. For an exact case in precision prec, 1+x will necessarily be
        // exact in precision prec, thus also in wprec, where wprec >= prec, and we can use pow_s
        // under this condition (which will also evaluate some non-exact cases).
        let (s, o_s) = x.add_prec_round_ref_val(Float::ONE, wprec, Down);
        if o_s == Equal {
            return s.pow_s_prec_round(n, prec, rm);
        }
        wprec += increment;
        increment = wprec >> 1;
        extra = new_extra;
        nloop += 1;
    };
    Float::from_float_prec_round(t, prec, rm)
}

impl Float {
    /// Computes the compound function $(1+x)^n$ of a [`Float`] $x$ and an [`i64`] $n$, rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The compound function is defined in IEEE 754 and is useful for computing compound interest:
    /// if $x$ is an interest rate, then $(1+x)^n$ is the factor by which a principal grows after
    /// $n$ compounding periods.
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
    /// - $f(\text{NaN},n)=\text{NaN}$ if $n\neq 0$, and $1.0$ if $n=0$
    /// - $f(-\infty,n)=\text{NaN}$, even if $n=0$
    /// - $f(\infty,0)=1.0$
    /// - $f(\infty,n)=\infty$ if $n>0$, and $0.0$ if $n<0$
    /// - $f(\pm 0.0,n)=1.0$
    /// - $f(x,n)=\text{NaN}$ if $x<-1$, even if $n=0$
    /// - $f(-1.0,n)=1.0$ if $n=0$, $0.0$ if $n>0$, and $\infty$ if $n<0$
    /// - $f(x,0)=1.0$ if $x\geq -1$
    ///
    /// The result is never negative, and a zero result is always positive.
    ///
    /// Overflow and underflow:
    /// - If $f(x,n,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,n,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,n,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,n,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,n,p,m)\leq 2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,n,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, either $0.0$ or
    ///   $2^{-2^{30}}$ may be returned. This matches the behavior of MPFR's compound function,
    ///   whose underflow test rounds to nearest as if it were rounding toward zero, except for
    ///   inputs that it resolves by exact powering.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::compound_prec`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::compound_round`] instead. If both of these things are true, consider using the
    /// [`Compound`] trait instead.
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
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(3).compound_prec_round(2, 10, Nearest);
    /// assert_eq!(c.to_string(), "16.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::TWO.compound_prec_round(-2, 10, Floor);
    /// assert_eq!(c.to_string(), "0.11108");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::TWO.compound_prec_round(-2, 10, Ceiling);
    /// assert_eq!(c.to_string(), "0.11121");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn compound_prec_round(self, n: i64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        compound_prec_round_helper(&self, n, prec, rm)
    }

    /// Computes the compound function $(1+x)^n$ of a [`Float`] $x$ and an [`i64`] $n$, rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
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
    /// See the [`Float::compound_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::compound_prec_ref`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::compound_round_ref`] instead. If both of these things are true, consider using the
    /// [`Compound`] trait instead.
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
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(3).compound_prec_round_ref(2, 10, Nearest);
    /// assert_eq!(c.to_string(), "16.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::TWO.compound_prec_round_ref(-2, 10, Ceiling);
    /// assert_eq!(c.to_string(), "0.11121");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn compound_prec_round_ref(&self, n: i64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        compound_prec_round_helper(self, n, prec, rm)
    }

    /// Computes the compound function $(1+x)^n$ of a [`Float`] $x$ and an [`i64`] $n$, rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the compound value is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,n,p) = (1+x)^n+\varepsilon.
    /// $$
    /// - If $(1+x)^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $(1+x)^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   (1+x)^n\rfloor-p}$.
    ///
    /// See the [`Float::compound_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::compound_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using the [`Compound`] trait instead.
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
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(3).compound_prec(2, 10);
    /// assert_eq!(c.to_string(), "16.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::TWO.compound_prec(-2, 10);
    /// assert_eq!(c.to_string(), "0.11108");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn compound_prec(self, n: i64, prec: u64) -> (Self, Ordering) {
        self.compound_prec_round(n, prec, Nearest)
    }

    /// Computes the compound function $(1+x)^n$ of a [`Float`] $x$ and an [`i64`] $n$, rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is taken by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the compound value is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See the [`Float::compound_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::compound_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using the [`Compound`] trait instead.
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
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(3).compound_prec_ref(2, 10);
    /// assert_eq!(c.to_string(), "16.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::TWO.compound_prec_ref(-2, 10);
    /// assert_eq!(c.to_string(), "0.11108");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn compound_prec_ref(&self, n: i64, prec: u64) -> (Self, Ordering) {
        self.compound_prec_round_ref(n, prec, Nearest)
    }

    /// Computes the compound function $(1+x)^n$ of a [`Float`] $x$ and an [`i64`] $n$, rounding the
    /// result to the precision of the input with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,m) = (1+x)^n+\varepsilon.
    /// $$
    /// - If $(1+x)^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $(1+x)^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 (1+x)^n\rfloor-p+1}$, where $p$ is the precision of the input. Similarly,
    ///   $p$ is the precision of the input in the `Nearest` bullet below.
    /// - If $(1+x)^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 (1+x)^n\rfloor-p}$.
    ///
    /// See the [`Float::compound_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using the [`Compound`] trait instead. If you
    /// want to specify an output precision, consider using [`Float::compound_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// the number of significant bits of the exponent `n`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(1.5).compound_round(2, Floor);
    /// assert_eq!(c.to_string(), "6.0");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from(1.5).compound_round(2, Ceiling);
    /// assert_eq!(c.to_string(), "8.0");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn compound_round(self, n: i64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.compound_prec_round(n, prec, rm)
    }

    /// Computes the compound function $(1+x)^n$ of a [`Float`] $x$ and an [`i64`] $n$, rounding the
    /// result to the precision of the input with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See the [`Float::compound_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using the [`Compound`] trait instead. If you
    /// want to specify an output precision, consider using [`Float::compound_prec_round_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// the number of significant bits of the exponent `n`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(1.5).compound_round_ref(2, Floor);
    /// assert_eq!(c.to_string(), "6.0");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from(1.5).compound_round_ref(2, Ceiling);
    /// assert_eq!(c.to_string(), "8.0");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn compound_round_ref(&self, n: i64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.compound_prec_round_ref(n, prec, rm)
    }

    /// Computes the compound function $(1+x)^n$ of a [`Float`] $x$ and an [`i64`] $n$ in place,
    /// rounding the result to the specified precision and with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See the [`Float::compound_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::compound_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::compound_round_assign`] instead. If both of these things are true, consider
    /// using the [`CompoundAssign`] trait instead.
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
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.compound_prec_round_assign(2, 10, Floor), Equal);
    /// assert_eq!(x.to_string(), "16.000");
    /// ```
    pub fn compound_prec_round_assign(&mut self, n: i64, prec: u64, rm: RoundingMode) -> Ordering {
        let (y, o) = self.compound_prec_round_ref(n, prec, rm);
        *self = y;
        o
    }

    /// Computes the compound function $(1+x)^n$ of a [`Float`] $x$ and an [`i64`] $n$ in place,
    /// rounding the result to the nearest value of the specified precision. An [`Ordering`] is
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value.
    ///
    /// If the compound value is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See the [`Float::compound_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::compound_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using the [`CompoundAssign`] trait instead.
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
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::TWO;
    /// assert_eq!(x.compound_prec_assign(-2, 10), Less);
    /// assert_eq!(x.to_string(), "0.11108");
    /// ```
    #[inline]
    pub fn compound_prec_assign(&mut self, n: i64, prec: u64) -> Ordering {
        self.compound_prec_round_assign(n, prec, Nearest)
    }

    /// Computes the compound function $(1+x)^n$ of a [`Float`] $x$ and an [`i64`] $n$ in place,
    /// rounding the result to the precision of the input with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See the [`Float::compound_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using the [`CompoundAssign`] trait instead.
    /// If you want to specify an output precision, consider using
    /// [`Float::compound_prec_round_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// the number of significant bits of the exponent `n`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(1.5);
    /// assert_eq!(x.compound_round_assign(2, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "8.0");
    /// ```
    #[inline]
    pub fn compound_round_assign(&mut self, n: i64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.compound_prec_round_assign(n, prec, rm)
    }
}

impl Compound<i64> for Float {
    type Output = Self;

    /// Computes the compound function $(1+x)^n$ of a [`Float`] $x$ and an [`i64`] $n$, rounding the
    /// result to the nearest value with the precision of the input. The [`Float`] is taken by
    /// value.
    ///
    /// The compound function is defined in IEEE 754 and is useful for computing compound interest:
    /// if $x$ is an interest rate, then $(1+x)^n$ is the factor by which a principal grows after
    /// $n$ compounding periods.
    ///
    /// If the compound value is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See the [`Float::compound_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// the number of significant bits of the exponent `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Compound;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     Float::from(0.1).compound(10).to_string(),
    ///     "2.5937424601000005"
    /// );
    /// assert_eq!(Float::from(3).compound(2).to_string(), "16.0");
    /// assert_eq!(Float::TWO.compound(-2).to_string(), "0.12");
    /// ```
    #[inline]
    fn compound(self, n: i64) -> Self {
        let prec = self.significant_bits();
        self.compound_prec_round(n, prec, Nearest).0
    }
}

impl Compound<i64> for &Float {
    type Output = Float;

    /// Computes the compound function $(1+x)^n$ of a [`Float`] $x$ and an [`i64`] $n$, rounding the
    /// result to the nearest value with the precision of the input. The [`Float`] is taken by
    /// reference.
    ///
    /// The compound function is defined in IEEE 754 and is useful for computing compound interest:
    /// if $x$ is an interest rate, then $(1+x)^n$ is the factor by which a principal grows after
    /// $n$ compounding periods.
    ///
    /// If the compound value is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See the [`Float::compound_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// the number of significant bits of the exponent `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Compound;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     (&Float::from(0.1)).compound(10).to_string(),
    ///     "2.5937424601000005"
    /// );
    /// assert_eq!((&Float::from(3)).compound(2).to_string(), "16.0");
    /// assert_eq!((&Float::TWO).compound(-2).to_string(), "0.12");
    /// ```
    #[inline]
    fn compound(self, n: i64) -> Float {
        let prec = self.significant_bits();
        self.compound_prec_round_ref(n, prec, Nearest).0
    }
}

impl CompoundAssign<i64> for Float {
    /// Computes the compound function $(1+x)^n$ of a [`Float`] $x$ and an [`i64`] $n$ in place,
    /// rounding the result to the nearest value with the precision of the input.
    ///
    /// The compound function is defined in IEEE 754 and is useful for computing compound interest:
    /// if $x$ is an interest rate, then $(1+x)^n$ is the factor by which a principal grows after
    /// $n$ compounding periods.
    ///
    /// If the compound value is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// See the [`Float::compound_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(mn^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// the number of significant bits of the exponent `n`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CompoundAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(0.1);
    /// x.compound_assign(10);
    /// assert_eq!(x.to_string(), "2.5937424601000005");
    /// ```
    #[inline]
    fn compound_assign(&mut self, n: i64) {
        let prec = self.significant_bits();
        self.compound_prec_round_assign(n, prec, Nearest);
    }
}
