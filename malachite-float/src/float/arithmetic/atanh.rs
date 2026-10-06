// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2001-2026 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::acsch::RECIPROCAL_SAFE_EXPONENT;
use crate::float::arithmetic::asinh::round_with_error;
use crate::float::arithmetic::cosh::same_rounding;
use crate::float::arithmetic::round_near_x::{
    LEADING_TERM_MIN_EXPONENT, round_rational_leading_term, small_input_shortcut,
};
use crate::float::arithmetic::sin::{TINY_UNDERFLOW_EXPONENT, underflowed};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    Abs, Atanh, AtanhAssign, CeilingLogBase2, IsPowerOf2, Ln, Ln1PlusX, Square,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeZero, One, Two,
    Zero as ZeroTrait,
};
use malachite_base::num::comparison::traits::OrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// This is mpfr_atanh_small from atanh.c, MPFR 4.2.2. It returns an approximation y of atanh(x), for
// 0 < x <= 1/2, at precision p, together with k such that the error is bounded by 2^k ulp(y). MPFR
// uses faithful rounding throughout; `Nearest` is used instead, which only improves the bounds.
//
// In the following, theta represents a value with |theta| <= 2^(1-p) (might be a different value
// each time).
fn atanh_small(x: &Float, p: u64) -> (Float, u64) {
    // t = x * (1 + theta)
    let mut t = Float::from_float_prec_ref(x, p).0;
    // exact
    let mut y = t.clone();
    // x2 = x^2 * (1 + theta)
    let x2 = x.square_prec_ref(p).0;
    let p_i64 = i64::exact_from(p);
    let mut i = 3u64;
    // i as a `Float`, kept at 64 bits so that every odd i stays exact
    let mut i_float = Float::from_unsigned_prec(3u32, 64).0;
    loop {
        // t = x^i * (1 + theta)^i
        t *= &x2;
        // u = x^i/i * (1 + theta)^(i+1)
        let u = t.div_prec_ref_ref(&i_float, p).0;
        if u == 0u32 {
            // MPFR's extended exponent range keeps u from underflowing. Here a u that underflows to
            // zero is negligible too; this needs x^i to underflow before |u| < ulp(y), and so a
            // working precision above 2^30.
            fail_on_untested_path("atanh_small, u underflows");
            break;
        }
        // |u| < ulp(y)
        if i64::from(u.get_exponent().unwrap()) <= i64::from(y.get_exponent().unwrap()) - p_i64 {
            break;
        }
        // error <= ulp(y)
        y += u;
        i += 2;
        i_float += Float::TWO;
    }
    // See algorithms.tex: the total error is bounded by (i+7)/2 ulp(y).
    let err = (i + 8) >> 1; // ceil((i+7)/2)
    let k = err.ceiling_log_base_2();
    // if k + 2 < p, since k = ceil(log2(err)), we have err <= 2^k <= 2^(p-3), thus i+7 <= 2*err <=
    // 2^(p-2), thus (i+7)*epsilon <= 1/2, which implies our assumption (i+1)*epsilon <= 1/2.
    assert!(k + 2 < p);
    (y, k)
}

// This is mpfr_atanh from atanh.c, MPFR 4.2.2, where the input is finite, nonzero, and less than 1
// in absolute value. atanh = ln((1+x)/(1-x)) / 2, except when x is very small, in which case atanh
// = x + tiny error, and when x is small, where the Taylor expansion is used directly.
//
// Outside the small case this departs from MPFR, which computes ln((1+x)/(1-x)): taking the
// logarithm of a quotient near 1 loses about -EXP(atanh x) bits, which MPFR's error estimate
// charges to the working precision and which often forces a retry. ln(1 + 2x/(1-x)), with
// `ln_1_plus_x`, has a constant error bound instead; it was benchmarked as never slower and 2 to 4
// times faster for |x| < 1/4.
//
// MPFR computes (1+x)/(1-x) in an extended exponent range. Here 2x/(1-x) overflows once 1 - x is
// below about 2^(1 - MAX_EXPONENT), which needs an x with a precision above 2^30; for such an x the
// logarithm is instead computed as ln(1+x) - ln(1-x).
fn atanh_prec_round_normal_ref(xt: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact atanh");
    let exp_xt = i64::from(xt.get_exponent().unwrap());
    // atanh(x) = x + x^3/3 + ... so the error is < 2^(3*EXP(x)-1)
    if let Some(result) = small_input_shortcut(xt, -(exp_xt << 1), 1, true, prec, rm) {
        return result;
    }
    let negative = *xt < 0u32;
    let x = xt.abs();
    // Compute initial precision
    let nt = max(x.get_prec().unwrap(), prec);
    let mut working_prec = nt + nt.ceiling_log_base_2() + 4;
    let mut increment = Limb::WIDTH;
    // small case: assuming the AGM algorithm used by ln uses log2(p) steps for a precision of p
    // bits, try the special variant whenever EXP(x) <= -p/log2(p). The +1 avoids a division by 0
    // when prec = 1. This implies EXP(x) <= -1, thus x < 1/2.
    let k = 1 + prec.ceiling_log_base_2();
    let small = exp_xt <= -1 - i64::exact_from(prec / k);
    loop {
        let (t, err) = if small {
            let (t, k) = atanh_small(&x, working_prec);
            (t, i64::exact_from(k))
        } else {
            // (1-x) with x = |xt|
            let te = Float::ONE
                .sub_prec_round_val_ref(&x, working_prec, Ceiling)
                .0;
            if i64::from(te.get_exponent().unwrap()) < RECIPROCAL_SAFE_EXPONENT {
                fail_on_untested_path("atanh_prec_round_normal_ref, (1+x)/(1-x) may overflow");
                // ln(1+x) - ln(1-x), halved
                let t = (x
                    .add_prec_round_ref_val(Float::ONE, working_prec, Floor)
                    .0
                    .ln()
                    - te.ln())
                    >> 1u32;
                // error estimate: see algorithms.tex
                let err = max(4 - i64::from(t.get_exponent().unwrap()), 0) + 1;
                (t, err)
            } else {
                // ln(1 + u) with u = 2x/(1-x), halved. u has relative error below 3 * 2^-wp
                // (2^(1-wp) from 1 - x rounded up, 2^-wp from the division). Since ln(1+u) >=
                // u/(1+u), that moves ln(1+u) by less than 3.02 * 2^-wp ln(1+u), about 3 ulps, and
                // its own rounding adds 1/2 ulp: under 2^3 ulps with room for an exponent boundary.
                let t = (&x << 1u32).div_prec(te, working_prec).0.ln_1_plus_x() >> 1u32;
                (t, 3)
            }
        };
        // MPFR also stops on a zero t, which cannot occur here: |x| is at least about
        // 2^(-prec/log2(prec)) outside the small case, and atanh_small returns at least x.
        if let Some(result) =
            round_with_error(if negative { -t } else { t }, working_prec, err, prec, rm)
        {
            return result;
        }
        // reactualisation of the precision
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes atanh(x) for a nonzero `Rational` x with |x| <= 1/2 from the series x + x^3/3 + x^5/5 +
// ..., whose terms have the sign of x. With S the sum of the terms through x^k/k, the remaining
// terms sum to less than |x|^(k+2)/((k+2)(1 - x^2)) in magnitude, so S and S plus that bound
// bracket atanh(x); terms are added until the two ends round alike.
fn atanh_series(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let x2 = x.square();
    let one_minus_x2 = Rational::ONE - &x2;
    let mut power = x.clone(); // x^k
    let mut sum = x.clone();
    let mut k = 1u64;
    loop {
        power *= &x2;
        k += 2;
        let bound = &power / (Rational::from(k) * &one_minus_x2);
        if let Some(result) = same_rounding(
            Float::from_rational_prec_round_ref(&sum, prec, rm),
            Float::from_rational_prec_round(&sum + bound, prec, rm),
        ) {
            return result;
        }
        sum += &power / Rational::from(k);
    }
}

// Computes atanh(x) for a `Rational` x with 0 < |x| < 1, rounded to precision `prec` with rounding
// mode `rm`. The result is never exactly representable, so `rm` must not be `Exact`.
fn atanh_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact atanh");
    let positive = *x > 0u32;
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    if exp_x < TINY_UNDERFLOW_EXPONENT {
        // |x| < 2^(MIN_EXPONENT - 3), so |atanh(x)| < |x| (1 + x^2) is below 2^(MIN_EXPONENT - 2),
        // half the smallest positive Float, and the result is zero or that Float, by the rounding
        // mode alone
        return underflowed(positive, prec, rm);
    }
    // atanh(x) = x(1 + x^2/3 + ...), so |x| falls short of |atanh x| by less than 2^(3 EXP(x) - 1),
    // as for `tan_rational`; once that is below the distance from x to the nearest (prec + 1)-bit
    // dyadic other than x itself, x's own rounding, nudged away from zero, is the answer.
    if exp_x > LEADING_TERM_MIN_EXPONENT
        && -(exp_x << 1) > i64::exact_from(prec + x.denominator_ref().significant_bits()) + 4
    {
        return round_rational_leading_term(x.abs(), positive, true, prec, rm);
    }
    // For a small x the series converges by a factor of x^2 per term. It needs about T = prec/(2
    // |EXP(x)|) terms, whose exact `Rational`s grow by about the D bits of x's denominator each, so
    // it costs about T^2 (D + 64) against about prec log2(prec) for the logarithm. Benchmarks put
    // the crossover at EXP(x)^2 * 12 (1 + log2(prec)) = prec (D + 64). The series also takes the
    // inputs at the bottom of the exponent range, which the halving below could push out of it.
    if exp_x < 0
        && (u128::from(exp_x.unsigned_abs()).pow(2)
            * 12
            * u128::from(1 + prec.ceiling_log_base_2())
            > u128::from(prec)
                .saturating_mul(u128::from(x.denominator_ref().significant_bits()) + 64)
            || exp_x <= LEADING_TERM_MIN_EXPONENT)
    {
        return atanh_series(x, prec, rm);
    }
    atanh_rational_via_ln(x, prec, rm)
}

// Computes atanh(x) for a `Rational` x with 0 < |x| < 1 whose result is far above the bottom of the
// exponent range. For x = n/d, atanh(x) = ln((1 + x)/(1 - x))/2 = ln((d + n)/(d - n))/2, and the
// quotient is an exact `Rational`, built straight from the numerator and denominator. Halving the
// correctly rounded logarithm is then exact, so it gives the correctly rounded result.
fn atanh_rational_via_ln(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let n = x.numerator_ref();
    let d = x.denominator_ref();
    let (sum, difference) = (d + n, d - n);
    let q = if *x > 0u32 {
        Rational::from_naturals(sum, difference)
    } else {
        Rational::from_naturals(difference, sum)
    };
    let (y, o) = Float::ln_rational_prec_round(q, prec, rm);
    (y >> 1u32, o)
}

impl Float {
    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic tangent is less than, equal to, or greater than the exact inverse hyperbolic
    /// tangent. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p+1}$.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(\pm1,p,m)=\pm\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|>1$
    ///
    /// The result never overflows or underflows: for a nonzero $x$ with $|x|<1$, $|x| \leq
    /// |\operatorname{atanh} x| \leq \frac{q+1}{2}\ln 2$, where $q$ is the precision of $x$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::atanh_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::atanh_round`] instead. If both of these things are true, consider using
    /// [`Float::atanh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic tangent of such a [`Float`] is never exactly representable, or
    /// if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.54930687");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atanh_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.atanh_prec_round_ref(prec, rm)
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic tangent is less than, equal to, or greater than the exact inverse hyperbolic
    /// tangent. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p+1}$.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(\pm1,p,m)=\pm\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|>1$
    ///
    /// The result never overflows or underflows: for a nonzero $x$ with $|x|<1$, $|x| \leq
    /// |\operatorname{atanh} x| \leq \frac{q+1}{2}\ln 2$, where $q$ is the precision of $x$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::atanh_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::atanh_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).atanh()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic tangent of such a [`Float`] is never exactly representable, or
    /// if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.54930687");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    pub fn atanh_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // atanh(NaN) = NaN, and atanh(+/-Inf) = NaN since tanh gives a result between -1 and 1
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // atanh(+/-0) = +/-0
            Zero { sign } => (
                if *sign {
                    Self::ZERO
                } else {
                    Self::NEGATIVE_ZERO
                },
                Equal,
            ),
            // atanh(x) = NaN as soon as |x| > 1, and atanh(+/-1) = +/-Inf
            Finite {
                sign,
                exponent,
                significand,
                ..
            } if *exponent > 0 => {
                if *exponent == 1 && significand.is_power_of_2() {
                    (
                        if *sign {
                            Self::INFINITY
                        } else {
                            Self::NEGATIVE_INFINITY
                        },
                        Equal,
                    )
                } else {
                    (Self::NAN, Equal)
                }
            }
            Finite { .. } => atanh_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic tangent
    /// is less than, equal to, or greater than the exact inverse hyperbolic tangent. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// If the inverse hyperbolic tangent is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atanh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm0.0$
    /// - $f(\pm1,p)=\pm\infty$
    /// - $f(x,p)=\text{NaN}$ if $|x|>1$
    ///
    /// The result never overflows or underflows: for a nonzero $x$ with $|x|<1$, $|x| \leq
    /// |\operatorname{atanh} x| \leq \frac{q+1}{2}\ln 2$, where $q$ is the precision of $x$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atanh_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::atanh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec(5);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec(20);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atanh_prec(self, prec: u64) -> (Self, Ordering) {
        self.atanh_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic tangent is less than, equal to, or greater than the exact inverse hyperbolic
    /// tangent. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic tangent is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atanh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm0.0$
    /// - $f(\pm1,p)=\pm\infty$
    /// - $f(x,p)=\text{NaN}$ if $|x|>1$
    ///
    /// The result never overflows or underflows: for a nonzero $x$ with $|x|<1$, $|x| \leq
    /// |\operatorname{atanh} x| \leq \frac{q+1}{2}\ln 2$, where $q$ is the precision of $x$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atanh_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).atanh()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_ref(5);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_prec_ref(20);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atanh_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.atanh_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic tangent is
    /// less than, equal to, or greater than the exact inverse hyperbolic tangent. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm0.0$
    /// - $f(\pm1,m)=\pm\infty$
    /// - $f(x,m)=\text{NaN}$ if $|x|>1$
    ///
    /// The result never overflows or underflows: for a nonzero $x$ with $|x|<1$, $|x| \leq
    /// |\operatorname{atanh} x| \leq \frac{q+1}{2}\ln 2$, where $q$ is the precision of $x$.
    ///
    /// If you want to specify an output precision, consider using [`Float::atanh_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::atanh`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic tangent of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_round(Floor);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846113");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_round(Ceiling);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846192");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_round(Nearest);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846113");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atanh_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.atanh_prec_round(prec, rm)
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic tangent is
    /// less than, equal to, or greater than the exact inverse hyperbolic tangent. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm0.0$
    /// - $f(\pm1,m)=\pm\infty$
    /// - $f(x,m)=\text{NaN}$ if $|x|>1$
    ///
    /// The result never overflows or underflows: for a nonzero $x$ with $|x|<1$, $|x| \leq
    /// |\operatorname{atanh} x| \leq \frac{q+1}{2}\ln 2$, where $q$ is the precision of $x$.
    ///
    /// If you want to specify an output precision, consider using [`Float::atanh_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).atanh()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic tangent of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_round_ref(Floor);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846113");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846192");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::one_prec(100) >> 1u32).atanh_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "0.54930614433405484569762261846113");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atanh_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.atanh_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// replaced by the result, and an [`Ordering`] is returned, indicating whether the rounded
    /// inverse hyperbolic tangent is less than, equal to, or greater than the exact inverse
    /// hyperbolic tangent. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p+1}$.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::atanh_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::atanh_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::atanh_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::atanh_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic tangent of such a [`Float`] is never exactly representable, or
    /// if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.531");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.54930592");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.54930687");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.54930592");
    /// ```
    #[inline]
    pub fn atanh_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.atanh_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is replaced by the
    /// result, and an [`Ordering`] is returned, indicating whether the rounded inverse hyperbolic
    /// tangent is less than, equal to, or greater than the exact inverse hyperbolic tangent.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets a `NaN` it
    /// also returns `Equal`.
    ///
    /// If the inverse hyperbolic tangent is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atanh} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::atanh_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atanh_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::atanh_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.54930592");
    /// ```
    #[inline]
    pub fn atanh_prec_assign(&mut self, prec: u64) -> Ordering {
        self.atanh_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], rounding
    /// the result with the specified rounding mode. The [`Float`] is replaced by the result, and an
    /// [`Ordering`] is returned, indicating whether the rounded inverse hyperbolic tangent is less
    /// than, equal to, or greater than the exact inverse hyperbolic tangent. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p+1}$, where $p$ is the
    ///   precision of the input.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::atanh_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::atanh_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::atanh_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite, nonzero, and less than 1 in absolute value,
    /// since the inverse hyperbolic tangent of such a [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.54930614433405484569762261846113");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.54930614433405484569762261846192");
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// assert_eq!(x.atanh_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "0.54930614433405484569762261846113");
    /// ```
    #[inline]
    pub fn atanh_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.atanh_prec_round_assign(prec, rm)
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded inverse hyperbolic tangent is less than, equal
    /// to, or greater than the exact inverse hyperbolic tangent. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p+1}$.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0.0$
    /// - $f(\pm1,p,m)=\pm\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|>1$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $|x|<1$ with denominator $d$, $|\operatorname{atanh} x|
    ///   \leq \frac{1}{2}\ln 2d$.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires an $x$ of magnitude below $2^{-2^{30}}$, the smallest positive [`Float`],
    /// since $|\operatorname{atanh} x| > |x|$ for nonzero $x$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::atanh_rational_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every nonzero $x$ with $|x|<1$).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::atanh_rational_prec_round(Rational::ONE_HALF, 5, Floor);
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::atanh_rational_prec_round(Rational::ONE_HALF, 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::atanh_rational_prec_round(Rational::from_signeds(-1i8, 2), 20, Floor);
    /// assert_eq!(c.to_string(), "-0.54930687");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::atanh_rational_prec_round(Rational::from_signeds(-1i8, 2), 20, Ceiling);
    /// assert_eq!(c.to_string(), "-0.54930592");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atanh_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::atanh_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded inverse hyperbolic tangent is less than,
    /// equal to, or greater than the exact inverse hyperbolic tangent. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p+1}$.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0.0$
    /// - $f(\pm1,p,m)=\pm\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|>1$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $|x|<1$ with denominator $d$, $|\operatorname{atanh} x|
    ///   \leq \frac{1}{2}\ln 2d$.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires an $x$ of magnitude below $2^{-2^{30}}$, the smallest positive [`Float`],
    /// since $|\operatorname{atanh} x| > |x|$ for nonzero $x$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::atanh_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every nonzero $x$ with $|x|<1$).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::atanh_rational_prec_round_ref(&Rational::ONE_HALF, 5, Floor);
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::atanh_rational_prec_round_ref(&Rational::ONE_HALF, 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::atanh_rational_prec_round_ref(&Rational::from_signeds(-1i8, 2), 20, Floor);
    /// assert_eq!(c.to_string(), "-0.54930687");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::atanh_rational_prec_round_ref(&Rational::from_signeds(-1i8, 2), 20, Ceiling);
    /// assert_eq!(c.to_string(), "-0.54930592");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn atanh_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // atanh(0) = 0, exactly
            return (Self::ZERO, Equal);
        }
        match x.cmp_abs(&Rational::ONE) {
            Less => atanh_rational_helper(x, prec, rm),
            // atanh(+/-1) = +/-Inf
            Equal => (
                if *x > 0u32 {
                    Self::INFINITY
                } else {
                    Self::NEGATIVE_INFINITY
                },
                Equal,
            ),
            Greater => (Self::NAN, Equal),
        }
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic tangent is less than, equal to, or greater
    /// than the exact inverse hyperbolic tangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic tangent is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{atanh} x+\varepsilon,
    /// $$
    /// where, if $\operatorname{atanh} x$ is finite and nonzero, $|\varepsilon| \leq
    /// 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$ (unless the result underflows; see
    /// below).
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0.0$
    /// - $f(\pm1,p)=\pm\infty$
    /// - $f(x,p)=\text{NaN}$ if $|x|>1$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $|x|<1$ with denominator $d$, $|\operatorname{atanh} x|
    ///   \leq \frac{1}{2}\ln 2d$.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an $x$ of magnitude below $2^{-2^{30}}$, the smallest positive [`Float`],
    /// since $|\operatorname{atanh} x| > |x|$ for nonzero $x$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atanh_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf, Two};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::atanh_rational_prec(Rational::ONE_HALF, 5);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::atanh_rational_prec(Rational::ONE_HALF, 20);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::atanh_rational_prec(Rational::ONE, 10);
    /// assert_eq!(c.to_string(), "Infinity");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::atanh_rational_prec(Rational::TWO, 10);
    /// assert!(c.is_nan());
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atanh_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::atanh_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded inverse hyperbolic tangent is less than, equal to, or greater
    /// than the exact inverse hyperbolic tangent. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the inverse hyperbolic tangent is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{atanh} x+\varepsilon,
    /// $$
    /// where, if $\operatorname{atanh} x$ is finite and nonzero, $|\varepsilon| \leq
    /// 2^{\lfloor\log_2 |\operatorname{atanh} x|\rfloor-p}$ (unless the result underflows; see
    /// below).
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0.0$
    /// - $f(\pm1,p)=\pm\infty$
    /// - $f(x,p)=\text{NaN}$ if $|x|>1$
    ///
    /// Overflow and underflow:
    /// - The result never overflows: for $|x|<1$ with denominator $d$, $|\operatorname{atanh} x|
    ///   \leq \frac{1}{2}\ln 2d$.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an $x$ of magnitude below $2^{-2^{30}}$, the smallest positive [`Float`],
    /// since $|\operatorname{atanh} x| > |x|$ for nonzero $x$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atanh_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the logarithm is computed at a working precision of about $n$, and
    /// the input is handled with `Rational` arithmetic.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf, Two};
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::atanh_rational_prec_ref(&Rational::ONE_HALF, 5);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::atanh_rational_prec_ref(&Rational::ONE_HALF, 20);
    /// assert_eq!(c.to_string(), "0.54930592");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::atanh_rational_prec_ref(&Rational::ONE, 10);
    /// assert_eq!(c.to_string(), "Infinity");
    /// assert_eq!(o, Equal);
    ///
    /// let (c, o) = Float::atanh_rational_prec_ref(&Rational::TWO, 10);
    /// assert!(c.is_nan());
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn atanh_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::atanh_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Atanh for Float {
    type Output = Self;

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], taking it
    /// by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// tangent is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atanh} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(\pm1)=\pm\infty$
    /// - $f(x)=\text{NaN}$ if $|x|>1$
    ///
    /// See the [`Float::atanh_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atanh_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::atanh_prec`]. If you want both of these things, consider using
    /// [`Float::atanh_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atanh;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.atanh().is_nan());
    /// assert!(Float::INFINITY.atanh().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.atanh().is_nan());
    /// assert_eq!(Float::ZERO.atanh().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.atanh().to_string(), "-0.0");
    /// assert_eq!(Float::ONE.atanh().to_string(), "Infinity");
    /// assert_eq!(Float::NEGATIVE_ONE.atanh().to_string(), "-Infinity");
    /// assert!(Float::TWO.atanh().is_nan());
    /// assert_eq!(
    ///     (Float::one_prec(100) >> 1u32).atanh().to_string(),
    ///     "0.54930614433405484569762261846113"
    /// );
    /// assert_eq!(
    ///     (-(Float::from_unsigned_prec(3u32, 100).0 >> 2u32))
    ///         .atanh()
    ///         .to_string(),
    ///     "-0.97295507452765665255267637172144"
    /// );
    /// ```
    #[inline]
    fn atanh(self) -> Self {
        let prec = self.significant_bits();
        self.atanh_prec_round(prec, Nearest).0
    }
}

impl Atanh for &Float {
    type Output = Float;

    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], taking it
    /// by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// tangent is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atanh} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(\pm1)=\pm\infty$
    /// - $f(x)=\text{NaN}$ if $|x|>1$
    ///
    /// See the [`Float::atanh_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atanh_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::atanh_prec_ref`]. If you want both of these things, consider using
    /// [`Float::atanh_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atanh;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).atanh().is_nan());
    /// assert!((&Float::INFINITY).atanh().is_nan());
    /// assert!((&Float::NEGATIVE_INFINITY).atanh().is_nan());
    /// assert_eq!((&Float::ZERO).atanh().to_string(), "0.0");
    /// assert_eq!((&Float::NEGATIVE_ZERO).atanh().to_string(), "-0.0");
    /// assert_eq!((&Float::ONE).atanh().to_string(), "Infinity");
    /// assert_eq!((&Float::NEGATIVE_ONE).atanh().to_string(), "-Infinity");
    /// assert!((&Float::TWO).atanh().is_nan());
    /// assert_eq!(
    ///     (&(Float::one_prec(100) >> 1u32)).atanh().to_string(),
    ///     "0.54930614433405484569762261846113"
    /// );
    /// assert_eq!(
    ///     (&-(Float::from_unsigned_prec(3u32, 100).0 >> 2u32))
    ///         .atanh()
    ///         .to_string(),
    ///     "-0.97295507452765665255267637172144"
    /// );
    /// ```
    #[inline]
    fn atanh(self) -> Float {
        self.atanh_prec_round_ref(self.significant_bits(), Nearest)
            .0
    }
}

impl AtanhAssign for Float {
    /// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the inverse hyperbolic
    /// tangent is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{atanh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{atanh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atanh} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::atanh`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atanh_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::atanh_prec_assign`]. If you want both of these things, consider using
    /// [`Float::atanh_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AtanhAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.atanh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.atanh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.atanh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.atanh_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.atanh_assign();
    /// assert_eq!(x.to_string(), "-0.0");
    ///
    /// let mut x = Float::ONE;
    /// x.atanh_assign();
    /// assert_eq!(x.to_string(), "Infinity");
    ///
    /// let mut x = Float::NEGATIVE_ONE;
    /// x.atanh_assign();
    /// assert_eq!(x.to_string(), "-Infinity");
    ///
    /// let mut x = Float::TWO;
    /// x.atanh_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::one_prec(100) >> 1u32;
    /// x.atanh_assign();
    /// assert_eq!(x.to_string(), "0.54930614433405484569762261846113");
    ///
    /// let mut x = -(Float::from_unsigned_prec(3u32, 100).0 >> 2u32);
    /// x.atanh_assign();
    /// assert_eq!(x.to_string(), "-0.97295507452765665255267637172144");
    /// ```
    #[inline]
    fn atanh_assign(&mut self) {
        let prec = self.significant_bits();
        self.atanh_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a primitive float. Using
/// this function is more accurate than using the default `atanh` function or the one provided by
/// `libm`.
///
/// $$
/// f(x) = \operatorname{atanh} x+\varepsilon.
/// $$
/// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed
///   to be 0.
/// - If $\operatorname{atanh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\operatorname{atanh} x|\rfloor-p}$, where $p$ is the precision of the output (24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=\text{NaN}$
/// - $f(\pm0.0)=\pm0.0$
/// - $f(\pm1)=\pm\infty$
/// - $f(x)=\text{NaN}$ if $|x|>1$
///
/// Overflow is not possible. The result is subnormal only when $x$ is, and then it is $x$ itself,
/// since $|\operatorname{atanh} x - x| < |x|^3/2$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::atanh::primitive_float_atanh;
///
/// assert!(primitive_float_atanh(f32::NAN).is_nan());
/// assert!(primitive_float_atanh(f32::INFINITY).is_nan());
/// assert!(primitive_float_atanh(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(NiceFloat(primitive_float_atanh(0.0f32)), NiceFloat(0.0));
/// assert_eq!(NiceFloat(primitive_float_atanh(-0.0f32)), NiceFloat(-0.0));
/// assert_eq!(
///     NiceFloat(primitive_float_atanh(1.0f32)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert!(primitive_float_atanh(2.0f32).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_atanh(0.5f32)),
///     NiceFloat(0.54930615)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_atanh(0.5f64)),
///     NiceFloat(0.5493061443340549)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_atanh(-0.75f64)),
///     NiceFloat(-0.9729550745276566)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_atanh<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::atanh_prec, x)
}

/// Computes $\operatorname{atanh} x$, the inverse hyperbolic tangent of a [`Rational`], returning
/// the result as a primitive float. The result is correctly rounded.
///
/// $$
/// f(x) = \operatorname{atanh} x+\varepsilon.
/// $$
/// - If $\operatorname{atanh} x$ is infinite, zero, or NaN, $\varepsilon$ may be ignored or assumed
///   to be 0.
/// - If $\operatorname{atanh} x$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\operatorname{atanh} x|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if
///   `T` is a [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=0.0$
/// - $f(\pm1)=\pm\infty$
/// - $f(x)=\text{NaN}$ if $|x|>1$
///
/// Overflow is not possible for $|x|<1$. Underflow is: an `x` of small enough magnitude gives `0.0`
/// or `-0.0`.
///
/// # Worst-case complexity
/// $T(m) = O(m (\log m)^2 \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{One, OneHalf, Two, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::atanh::primitive_float_atanh_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_atanh_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_atanh_rational::<f64>(&Rational::ONE)),
///     NiceFloat(f64::INFINITY)
/// );
/// assert!(primitive_float_atanh_rational::<f64>(&Rational::TWO).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_atanh_rational::<f64>(&Rational::ONE_HALF)),
///     NiceFloat(0.5493061443340549)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_atanh_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(0.34657359027997264)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_atanh_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::atanh_rational_prec_ref, x)
}
