// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2007-2025 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{
    Float, emulate_float_float_to_float_and_i64_fn, emulate_float_float_to_float_fn,
    emulate_float_to_float_and_i64_fn, emulate_float_to_float_fn, float_either_infinity,
    float_either_zero, float_nan, significand_bits,
};
use core::cmp::Ordering::{self, *};
use core::cmp::{max, min};
use core::ops::{Rem, RemAssign};
use malachite_base::num::arithmetic::traits::{
    DivMod, ModPow, ModPowerOf2, NegAssign, Parity, PowerOf2,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{NegativeZero, One, Two, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// This is mpfr_rem1 from rem1.c, MPFR 4.2.2, with the result's precision passed explicitly, the
// first rounding mode `rnd_q` (which is always `MPFR_RNDZ` for the fmod family and `MPFR_RNDN` for
// the remainder family) represented by the `nearest_quotient` flag, and the optional `quo` output
// always returned (the callers that don't want it discard it).
//
// rem1 works as follows: let q = x/y rounded to an integer toward zero if `nearest_quotient` is
// false, and to the nearest integer (ties to even) if it is true. Put x - q*y in the returned
// `Float`, rounded to `prec` bits according to `rm`. The returned `i64` has the sign of q, and
// agrees with q in its 63 low order bits; in other words, quo = q (mod 2^63) and quo * q >= 0. If
// the remainder is zero, it has the sign of x. The returned `Ordering` gives the place of the
// rounded remainder relative to x - q*y.
//
// If x or y is NaN, or x is infinite, or y is zero: quo is 0 (unspecified in MPFR), and the
// remainder is NaN. If y is infinite and x is finite, or x is zero and y is nonzero: quo is 0 and
// the remainder is x rounded to `prec`.
//
// Since |x - q*y| <= y/2, no overflow is possible. Only an underflow is possible when y is very
// small.

fn rem1_helper(
    x: &Float,
    y: &Float,
    nearest_quotient: bool,
    want_quo: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering, i64) {
    assert_ne!(prec, 0);
    match (x, y) {
        (Float(NaN | Infinity { .. }), _) | (_, Float(NaN | Zero { .. })) => {
            (float_nan!(), Equal, 0)
        }
        (_, float_either_infinity!()) | (float_either_zero!(), _) => {
            // either y is infinite and x is zero or finite, or x is zero and y is not special; in
            // both cases the quotient is zero and the remainder is x.
            let (rem, o) = Float::from_float_prec_round_ref(x, prec, rm);
            (rem, o, 0)
        }
        (
            Float(Finite {
                sign: x_sign,
                exponent: x_exponent,
                significand: x_significand,
                ..
            }),
            Float(Finite {
                sign: y_sign,
                exponent: y_exponent,
                significand: y_significand,
                ..
            }),
        ) => rem1_core(
            *x_sign,
            x_significand,
            i64::from(*x_exponent) - i64::exact_from(significand_bits(x_significand)),
            *y_sign,
            y_significand,
            i64::from(*y_exponent) - i64::exact_from(significand_bits(y_significand)),
            &Natural::ONE,
            nearest_quotient,
            want_quo,
            prec,
            rm,
        ),
    }
}

// The integer-level core shared by the Float-Float and mixed Float-Rational remainder functions:
// computes the remainder of A by B rounded to `prec` bits with `rm`, where A = a*2^ea with sign
// `x_sign` and B = b*2^eb with sign `y_sign`, a and b positive integers, dividing the result by the
// positive integer `den`. The identity rem(x, n/d) = rem(xd, n)/d, which preserves the quotient
// (and so its parity and low bits), reduces a Rational operand on either side to this form; `den`
// is 1 in the Float-Float case.
#[allow(clippy::too_many_arguments)]
fn rem1_core(
    x_sign: bool,
    mx: &Natural,
    ex: i64,
    y_sign: bool,
    b: &Natural,
    eb: i64,
    den: &Natural,
    nearest_quotient: bool,
    want_quo: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering, i64) {
    let signx = x_sign;
    // To get rid of sign problems, we compute the result separately: quo(-x,-y) = quo(x,y),
    // rem(-x,-y) = -rem(x,y) quo(-x,y) = -quo(x,y), rem(-x,y) = -rem(x,y) thus quo =
    // sign(x/y)*quo(|x|,|y|), rem = sign(x)*rem(|x|,|y|)
    let sign = x_sign == y_sign;
    // A = mx*2^ex, B = my*2^ey
    let mut ey = eb;
    let mut q_is_odd = false;
    let mut quo = 0i64;
    let mut tiny = false;
    // Divide my by 2^k if possible to make operations mod my easier. Since the exponents come from
    // regular floats, due to the constraints on the exponent and the precision, there can be no
    // integer overflow below.
    let k = b.trailing_zeros().unwrap();
    ey += i64::exact_from(k);
    let mut my = b >> k;
    let mut r;
    if ex <= ey {
        // q = x/y = mx/(my*2^(ey-ex))
        //
        // First detect cases where q = 0, to avoid creating a huge number my*2^(ey-ex): if sx =
        // mx.significant_bits() and sy = my.significant_bits(), we have x < 2^(ex + sx) and y >=
        // 2^(ey + sy - 1), thus if ex + sx <= ey + sy - 1 the quotient is 0.
        let q;
        if ex + i64::exact_from(mx.significant_bits()) < ey + i64::exact_from(my.significant_bits())
        {
            tiny = true;
            q = Natural::ZERO;
            r = mx.clone();
        } else {
            // divide mx by my*2^(ey-ex)
            my <<= u64::exact_from(ey - ex);
            // since mx > 0 and my > 0, truncating division is the same as floor division
            (q, r) = mx.div_mod(&my);
            // 0 <= r < my
        }
        if nearest_quotient {
            q_is_odd = q.odd();
        }
        if want_quo {
            quo = i64::exact_from(&(&q).mod_power_of_2(63));
        }
    } else {
        // ex > ey
        if want_quo {
            // for the quotient-bits variants, to get the low 63 more bits of the quotient, we first
            // compute R = X mod Y*2^63, where X and Y are defined below. Then the low 63 bits of
            // the quotient are floor(R/Y).
            my <<= 63u32;
        } else if nearest_quotient {
            // remainder case: let X = mx*2^(ex-ey) and Y = my. Then both X and Y are integers.
            // Assume X = R mod Y; then x = X*2^ey = R*2^ey mod (Y*2^ey=y). To be able to perform
            // the rounding, we need the least significant bit of the quotient, i.e., one more bit
            // in the remainder, which is obtained by dividing by 2Y.
            my <<= 1u32;
        }
        let d = u64::exact_from(ex - ey);
        r = if d > 3 * my.significant_bits() {
            // 2^(ex-ey) mod my. When 2^(ex-ey) is at least my^3, modular exponentiation is faster
            // than the exact power and a single reduction.
            (&(Natural::TWO % &my)).mod_pow(Natural::from(d), &my)
        } else {
            Natural::power_of_2(d)
        };
        r = r * mx % &my;
        if want_quo {
            // now 0 <= r < 2^63*Y
            my >>= 63u32;
            let q;
            (q, r) = r.div_mod(&my);
            // oldr = q*my + newr
            quo = i64::exact_from(&q);
            q_is_odd = quo.odd();
        } else if nearest_quotient {
            // now 0 <= r < 2Y in the remainder case
            my >>= 1u32;
            // least significant bit of q
            q_is_odd = r >= my;
            if q_is_odd {
                r -= &my;
            }
        }
        // now 0 <= r < my, and if needed, q_is_odd is the least significant bit of q
    }
    if r == 0u32 {
        // a zero remainder takes the sign of x, and is always exact
        (
            if signx {
                Float::ZERO
            } else {
                Float::NEGATIVE_ZERO
            },
            Equal,
            if sign { quo } else { quo.wrapping_neg() },
        )
    } else {
        let mut my = Integer::from(my);
        let mut r = Integer::from(r);
        if nearest_quotient {
            // determine whether 2r is greater than my; both are nonnegative, so plain comparison
            // mirrors mpz_cmpabs
            let r2 = &r << 1u32;
            let c = if tiny {
                // if tiny, we should compare r with my*2^(ey-ex)
                if ex + i64::exact_from(r2.significant_bits())
                    < ey + i64::exact_from(my.significant_bits())
                {
                    // r*2^ex < my*2^ey
                    Less
                } else {
                    my <<= u64::exact_from(ey - ex);
                    r2.cmp(&my)
                }
            } else {
                r2.cmp(&my)
            };
            // if the quotient rounds away, we need to subtract my from r, and add 1 to quo
            if c == Greater || c == Equal && q_is_odd {
                r -= &my;
                if want_quo {
                    // The C code increments a long here, which can overflow; we keep the documented
                    // low-63-bits contract instead.
                    quo = quo.wrapping_add(1) & i64::MAX;
                }
            }
        }
        // take into account sign of x
        if !signx {
            r.neg_assign();
        }
        // The result is r*2^sh/den. In the den = 1 case, rounding r to prec bits gives an exponent
        // of e or e + 1 (on a rounding carry), so when e is strictly inside the representable range
        // no underflow or overflow is possible: round r once and shift exactly, avoiding the
        // Rational construction, whose denominator has |sh| bits when sh is negative. At the range
        // edges, and whenever den is not 1, fall back to the Rational conversion, whose single
        // rounding handles underflow. (Both paths are a single rounding of the same value, so they
        // agree wherever both apply.)
        let sh = min(ex, ey);
        let (rem, o) = if *den == 1u32 {
            let e = i64::exact_from(r.significant_bits()) + sh;
            if e > Float::MIN_EXPONENT_I64 && e < Float::MAX_EXPONENT_I64 {
                let (rem, o) = Float::from_integer_prec_round(r, prec, rm);
                (rem << sh, o)
            } else {
                Float::from_rational_prec_round(Rational::from(r) << sh, prec, rm)
            }
        } else {
            Float::from_rational_prec_round(
                Rational::from_integers(r, Integer::from(den)) << sh,
                prec,
                rm,
            )
        };
        (rem, o, if sign { quo } else { quo.wrapping_neg() })
    }
}

// This is mpfr_fmod_ui from fmod_ui.c, MPFR 4.2.2, generalized over `nearest_quotient` like the
// helper it wraps. The conversion of `other` to a `Float` is exact, and `rem1_helper` depends only
// on its arguments' values, so this is a pure thin wrapper. A zero modulus yields NaN, matching
// mpfr_fmod_ui.
fn rem_unsigned_helper(
    x: &Float,
    other: u64,
    nearest_quotient: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    if other == 0 {
        (float_nan!(), Equal)
    } else {
        let (r, o, _) = rem1_helper(x, &Float::from(other), nearest_quotient, false, prec, rm);
        (r, o)
    }
}

// Shared special-case handling and scaling for the Float-mod-Rational functions. A Rational modulus
// keeps the reduction exact: converting it to a Float first would perturb the remainder by the
// quotient times the conversion error. A zero modulus yields NaN, as with a zero Float modulus.
fn rem_rational_helper(
    x: &Float,
    y: &Rational,
    nearest_quotient: bool,
    want_quo: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering, i64) {
    assert_ne!(prec, 0);
    match x {
        _ if *y == 0u32 => (float_nan!(), Equal, 0),
        Float(NaN | Infinity { .. }) => (float_nan!(), Equal, 0),
        float_either_zero!() => {
            // the quotient is zero and the remainder is x
            let (rem, o) = Float::from_float_prec_round_ref(x, prec, rm);
            (rem, o, 0)
        }
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            let d = y.denominator_ref();
            rem1_core(
                *sign,
                &(significand * d),
                i64::from(*exponent) - i64::exact_from(significand_bits(significand)),
                *y > 0u32,
                y.numerator_ref(),
                0,
                d,
                nearest_quotient,
                want_quo,
                prec,
                rm,
            )
        }
    }
}

// The reversed direction: the remainder of a Rational by a Float. A zero Rational gives a positive
// zero (a Rational zero has no sign), an infinite Float modulus returns the Rational rounded, and a
// NaN or zero Float modulus gives NaN.
fn rational_rem_float_helper(
    x: &Rational,
    y: &Float,
    nearest_quotient: bool,
    want_quo: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering, i64) {
    assert_ne!(prec, 0);
    match y {
        Float(NaN | Zero { .. }) => (float_nan!(), Equal, 0),
        float_either_infinity!() => {
            // the quotient is zero and the remainder is x
            let (rem, o) = Float::from_rational_prec_round_ref(x, prec, rm);
            (rem, o, 0)
        }
        Float(Finite {
            sign,
            exponent,
            significand,
            ..
        }) => {
            if *x == 0u32 {
                (Float::ZERO, Equal, 0)
            } else {
                let d = x.denominator_ref();
                rem1_core(
                    *x > 0u32,
                    x.numerator_ref(),
                    0,
                    *sign,
                    &(significand * d),
                    i64::from(*exponent) - i64::exact_from(significand_bits(significand)),
                    d,
                    nearest_quotient,
                    want_quo,
                    prec,
                    rm,
                )
            }
        }
    }
}

impl Float {
    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the specified
    /// precision and with the specified rounding mode. Both [`Float`]s are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal
    /// to, or greater than the exact remainder. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::rem_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::rem_round`] instead. If both of these things are true, consider using `%`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_prec_round(Float::from(7u32), 1, Floor);
    /// assert_eq!(r.to_string(), "2.0");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = Float::from(10u32).rem_prec_round(Float::from(7u32), 1, Ceiling);
    /// assert_eq!(r.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (r, o, _) = rem1_helper(&self, &other, false, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the specified
    /// precision and with the specified rounding mode. The first [`Float`] is taken by value and
    /// the second by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::rem_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::rem_round`] instead. If both of these things are true, consider using `%`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_prec_round_val_ref(&Float::from(7u32), 1, Floor);
    /// assert_eq!(r.to_string(), "2.0");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = Float::from(10u32).rem_prec_round_val_ref(&Float::from(7u32), 1, Ceiling);
    /// assert_eq!(r.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn rem_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem1_helper(&self, other, false, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the specified
    /// precision and with the specified rounding mode. The first [`Float`] is taken by reference
    /// and the second by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::rem_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::rem_round`] instead. If both of these things are true, consider using `%`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_prec_round_ref_val(Float::from(7u32), 1, Floor);
    /// assert_eq!(r.to_string(), "2.0");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = Float::from(10u32).rem_prec_round_ref_val(Float::from(7u32), 1, Ceiling);
    /// assert_eq!(r.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem1_helper(self, &other, false, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the specified
    /// precision and with the specified rounding mode. Both [`Float`]s are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal
    /// to, or greater than the exact remainder. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::rem_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::rem_round`] instead. If both of these things are true, consider using `%`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_prec_round_ref_ref(&Float::from(7u32), 1, Floor);
    /// assert_eq!(r.to_string(), "2.0");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = Float::from(10u32).rem_prec_round_ref_ref(&Float::from(7u32), 1, Ceiling);
    /// assert_eq!(r.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn rem_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem1_helper(self, other, false, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the nearest value
    /// of the specified precision. Both [`Float`]s are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rem_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using `%` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_prec(Float::from(7u32), 1);
    /// assert_eq!(r.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = Float::from(10u32).rem_prec(Float::from(7u32), 2);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn rem_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        self.rem_prec_round(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the nearest value
    /// of the specified precision. The first [`Float`] is taken by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is
    /// less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rem_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using `%` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_prec_val_ref(&Float::from(7u32), 1);
    /// assert_eq!(r.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = Float::from(10u32).rem_prec_val_ref(&Float::from(7u32), 2);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn rem_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.rem_prec_round_val_ref(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the nearest value
    /// of the specified precision. The first [`Float`] is taken by reference and the second by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less
    /// than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rem_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using `%` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_prec_ref_val(Float::from(7u32), 1);
    /// assert_eq!(r.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = Float::from(10u32).rem_prec_ref_val(Float::from(7u32), 2);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn rem_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        self.rem_prec_round_ref_val(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the nearest value
    /// of the specified precision. Both [`Float`]s are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rem_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using `%` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_prec_ref_ref(&Float::from(7u32), 1);
    /// assert_eq!(r.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (r, o) = Float::from(10u32).rem_prec_ref_ref(&Float::from(7u32), 2);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn rem_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.rem_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the maximum of
    /// the precisions of the inputs, with the specified rounding mode. Both [`Float`]s are taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less
    /// than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::rem_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `%`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_round(Float::from(7u32), Floor);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (r, o) = (-Float::from(10u32)).rem_round(Float::from(7u32), Floor);
    /// assert_eq!(r.to_string(), "-3.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn rem_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_prec_round(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the maximum of
    /// the precisions of the inputs, with the specified rounding mode. The first [`Float`] is taken
    /// by value and the second by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::rem_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `%`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_round_val_ref(&Float::from(7u32), Floor);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (r, o) = (-Float::from(10u32)).rem_round_val_ref(&Float::from(7u32), Floor);
    /// assert_eq!(r.to_string(), "-3.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn rem_round_val_ref(self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_prec_round_val_ref(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the maximum of
    /// the precisions of the inputs, with the specified rounding mode. The first [`Float`] is taken
    /// by reference and the second by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::rem_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `%`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_round_ref_val(Float::from(7u32), Floor);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (r, o) = (-Float::from(10u32)).rem_round_ref_val(Float::from(7u32), Floor);
    /// assert_eq!(r.to_string(), "-3.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn rem_round_ref_val(&self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_prec_round_ref_val(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the maximum of
    /// the precisions of the inputs, with the specified rounding mode. Both [`Float`]s are taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is
    /// less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::rem_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using `%`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_round_ref_ref(&Float::from(7u32), Floor);
    /// assert_eq!(r.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (r, o) = (-Float::from(10u32)).rem_round_ref_ref(&Float::from(7u32), Floor);
    /// assert_eq!(r.to_string(), "-3.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn rem_round_ref_ref(&self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s in place, with the quotient rounded toward zero, as
    /// for the `%` operator on primitive floats and C's `fmod`, rounding the remainder to the
    /// specified precision and with the specified rounding mode. The [`Float`] on the right-hand
    /// side is taken by value. An [`Ordering`] is returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(10u32);
    /// assert_eq!(x.rem_prec_round_assign(Float::from(7u32), 1, Floor), Less);
    /// assert_eq!(x.to_string(), "2.0");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_prec_round_assign(&mut self, other: Self, prec: u64, rm: RoundingMode) -> Ordering {
        let (r, o, _) = rem1_helper(self, &other, false, false, prec, rm);
        *self = r;
        o
    }

    /// Computes the remainder of two [`Float`]s in place, with the quotient rounded toward zero, as
    /// for the `%` operator on primitive floats and C's `fmod`, rounding the remainder to the
    /// specified precision and with the specified rounding mode. The [`Float`] on the right-hand
    /// side is taken by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(10u32);
    /// assert_eq!(
    ///     x.rem_prec_round_assign_ref(&Float::from(7u32), 1, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "2.0");
    /// ```
    pub fn rem_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (r, o, _) = rem1_helper(self, other, false, false, prec, rm);
        *self = r;
        o
    }

    /// Computes the remainder of two [`Float`]s in place, with the quotient rounded toward zero, as
    /// for the `%` operator on primitive floats and C's `fmod`, rounding the remainder to the
    /// nearest value of the specified precision. The [`Float`] on the right-hand side is taken by
    /// value. An [`Ordering`] is returned, indicating whether the rounded remainder is less than,
    /// equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(10u32);
    /// assert_eq!(x.rem_prec_assign(Float::from(7u32), 2), Equal);
    /// assert_eq!(x.to_string(), "3.0");
    /// ```
    #[inline]
    pub fn rem_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        self.rem_prec_round_assign(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s in place, with the quotient rounded toward zero, as
    /// for the `%` operator on primitive floats and C's `fmod`, rounding the remainder to the
    /// nearest value of the specified precision. The [`Float`] on the right-hand side is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded remainder is less
    /// than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(10u32);
    /// assert_eq!(x.rem_prec_assign_ref(&Float::from(7u32), 2), Equal);
    /// assert_eq!(x.to_string(), "3.0");
    /// ```
    #[inline]
    pub fn rem_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        self.rem_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s in place, with the quotient rounded toward zero, as
    /// for the `%` operator on primitive floats and C's `fmod`, rounding the remainder to the
    /// maximum of the precisions of the inputs, with the specified rounding mode. The [`Float`] on
    /// the right-hand side is taken by value. An [`Ordering`] is returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(10u32);
    /// assert_eq!(x.rem_round_assign(Float::from(7u32), Floor), Equal);
    /// assert_eq!(x.to_string(), "3.0");
    /// ```
    pub fn rem_round_assign(&mut self, other: Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_prec_round_assign(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s in place, with the quotient rounded toward zero, as
    /// for the `%` operator on primitive floats and C's `fmod`, rounding the remainder to the
    /// maximum of the precisions of the inputs, with the specified rounding mode. The [`Float`] on
    /// the right-hand side is taken by reference. An [`Ordering`] is returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(10u32);
    /// assert_eq!(x.rem_round_assign_ref(&Float::from(7u32), Floor), Equal);
    /// assert_eq!(x.to_string(), "3.0");
    /// ```
    pub fn rem_round_assign_ref(&mut self, other: &Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_prec_round_assign_ref(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the specified
    /// precision and with the specified rounding mode. Both [`Float`]s are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal
    /// to, or greater than the exact remainder, along with the low bits of the quotient as an
    /// `i64`. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. (MPFR documents the same contract for its `quo` output,
    /// but its C implementation can overflow a `long` when the low 63 bits are all ones and the
    /// quotient rounds away from zero; this implementation always keeps the modular contract.)
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::rem_and_quotient_bits_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::rem_and_quotient_bits_round`] instead. If both of these
    /// things are true, consider using [`Float::rem_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) =
    ///     Float::from(100u32).rem_and_quotient_bits_prec_round(Float::from(7u32), 5, Floor);
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rem_and_quotient_bits_prec_round(
        self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem1_helper(&self, &other, false, true, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the specified
    /// precision and with the specified rounding mode. The first [`Float`] is taken by value and
    /// the second by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. (MPFR documents the same contract for its `quo` output,
    /// but its C implementation can overflow a `long` when the low 63 bits are all ones and the
    /// quotient rounds away from zero; this implementation always keeps the modular contract.)
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::rem_and_quotient_bits_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::rem_and_quotient_bits_round`] instead. If both of these
    /// things are true, consider using [`Float::rem_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(100u32);
    /// let y = Float::from(7u32);
    /// let (r, o, q) = x.rem_and_quotient_bits_prec_round_val_ref(&y, 5, Floor);
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem1_helper(&self, other, false, true, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the specified
    /// precision and with the specified rounding mode. The first [`Float`] is taken by reference
    /// and the second by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. (MPFR documents the same contract for its `quo` output,
    /// but its C implementation can overflow a `long` when the low 63 bits are all ones and the
    /// quotient rounds away from zero; this implementation always keeps the modular contract.)
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::rem_and_quotient_bits_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::rem_and_quotient_bits_round`] instead. If both of these
    /// things are true, consider using [`Float::rem_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(100u32);
    /// let y = Float::from(7u32);
    /// let (r, o, q) = x.rem_and_quotient_bits_prec_round_ref_val(y, 5, Floor);
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rem_and_quotient_bits_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem1_helper(self, &other, false, true, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the specified
    /// precision and with the specified rounding mode. Both [`Float`]s are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal
    /// to, or greater than the exact remainder, along with the low bits of the quotient as an
    /// `i64`. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. (MPFR documents the same contract for its `quo` output,
    /// but its C implementation can overflow a `long` when the low 63 bits are all ones and the
    /// quotient rounds away from zero; this implementation always keeps the modular contract.)
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::rem_and_quotient_bits_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::rem_and_quotient_bits_round`] instead. If both of these
    /// things are true, consider using [`Float::rem_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(100u32);
    /// let y = Float::from(7u32);
    /// let (r, o, q) = x.rem_and_quotient_bits_prec_round_ref_ref(&y, 5, Floor);
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem1_helper(self, other, false, true, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the nearest value
    /// of the specified precision. Both [`Float`]s are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rem_and_quotient_bits_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the two inputs, consider using
    /// [`Float::rem_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) = Float::from(100u32).rem_and_quotient_bits_prec(Float::from(7u32), 5);
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits_prec(self, other: Self, prec: u64) -> (Self, Ordering, i64) {
        self.rem_and_quotient_bits_prec_round(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the nearest value
    /// of the specified precision. The first [`Float`] is taken by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is
    /// less than, equal to, or greater than the exact remainder, along with the low bits of the
    /// quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rem_and_quotient_bits_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the two inputs, consider using
    /// [`Float::rem_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) =
    ///     Float::from(100u32).rem_and_quotient_bits_prec_val_ref(&Float::from(7u32), 5);
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits_prec_val_ref(
        self,
        other: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        self.rem_and_quotient_bits_prec_round_val_ref(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the nearest value
    /// of the specified precision. The first [`Float`] is taken by reference and the second by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less
    /// than, equal to, or greater than the exact remainder, along with the low bits of the quotient
    /// as an `i64`. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rem_and_quotient_bits_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the two inputs, consider using
    /// [`Float::rem_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) =
    ///     Float::from(100u32).rem_and_quotient_bits_prec_ref_val(Float::from(7u32), 5);
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits_prec_ref_val(
        &self,
        other: Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        self.rem_and_quotient_bits_prec_round_ref_val(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the nearest value
    /// of the specified precision. Both [`Float`]s are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rem_and_quotient_bits_prec_round`] instead. If you know that your target precision
    /// is the maximum of the precisions of the two inputs, consider using
    /// [`Float::rem_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) =
    ///     Float::from(100u32).rem_and_quotient_bits_prec_ref_ref(&Float::from(7u32), 5);
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits_prec_ref_ref(
        &self,
        other: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        self.rem_and_quotient_bits_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the maximum of
    /// the precisions of the inputs, with the specified rounding mode. Both [`Float`]s are taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less
    /// than, equal to, or greater than the exact remainder, along with the low bits of the quotient
    /// as an `i64`. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::rem_and_quotient_bits_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using [`Float::rem_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) = Float::from(100u32).rem_and_quotient_bits_round(Float::from(7u32), Floor);
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    pub fn rem_and_quotient_bits_round(
        self,
        other: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_and_quotient_bits_prec_round(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the maximum of
    /// the precisions of the inputs, with the specified rounding mode. The first [`Float`] is taken
    /// by value and the second by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder, along
    /// with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::rem_and_quotient_bits_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using [`Float::rem_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) =
    ///     Float::from(100u32).rem_and_quotient_bits_round_val_ref(&Float::from(7u32), Floor);
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    pub fn rem_and_quotient_bits_round_val_ref(
        self,
        other: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_and_quotient_bits_prec_round_val_ref(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the maximum of
    /// the precisions of the inputs, with the specified rounding mode. The first [`Float`] is taken
    /// by reference and the second by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder, along
    /// with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::rem_and_quotient_bits_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using [`Float::rem_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) =
    ///     Float::from(100u32).rem_and_quotient_bits_round_ref_val(Float::from(7u32), Floor);
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    pub fn rem_and_quotient_bits_round_ref_val(
        &self,
        other: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_and_quotient_bits_prec_round_ref_val(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the maximum of
    /// the precisions of the inputs, with the specified rounding mode. Both [`Float`]s are taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is
    /// less than, equal to, or greater than the exact remainder, along with the low bits of the
    /// quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::rem_and_quotient_bits_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using [`Float::rem_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) =
    ///     Float::from(100u32).rem_and_quotient_bits_round_ref_ref(&Float::from(7u32), Floor);
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    pub fn rem_and_quotient_bits_round_ref_ref(
        &self,
        other: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_and_quotient_bits_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the nearest value
    /// of the maximum of the precisions of the inputs. Both [`Float`]s are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal
    /// to, or greater than the exact remainder, along with the low bits of the quotient as an
    /// `i64`. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::rem_and_quotient_bits_prec`] instead. If you want to use a rounding mode other than
    /// `Nearest`, consider using [`Float::rem_and_quotient_bits_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) = Float::from(100u32).rem_and_quotient_bits(Float::from(7u32));
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits(self, other: Self) -> (Self, Ordering, i64) {
        self.rem_and_quotient_bits_round(other, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the nearest value
    /// of the maximum of the precisions of the inputs. The first [`Float`] is taken by value and
    /// the second by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::rem_and_quotient_bits_prec`] instead. If you want to use a rounding mode other than
    /// `Nearest`, consider using [`Float::rem_and_quotient_bits_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) = Float::from(100u32).rem_and_quotient_bits_val_ref(&Float::from(7u32));
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits_val_ref(self, other: &Self) -> (Self, Ordering, i64) {
        self.rem_and_quotient_bits_round_val_ref(other, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the nearest value
    /// of the maximum of the precisions of the inputs. The first [`Float`] is taken by reference
    /// and the second by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::rem_and_quotient_bits_prec`] instead. If you want to use a rounding mode other than
    /// `Nearest`, consider using [`Float::rem_and_quotient_bits_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) = Float::from(100u32).rem_and_quotient_bits_ref_val(Float::from(7u32));
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits_ref_val(&self, other: Self) -> (Self, Ordering, i64) {
        self.rem_and_quotient_bits_round_ref_val(other, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded toward zero, as for the
    /// `%` operator on primitive floats and C's `fmod`, rounding the remainder to the nearest value
    /// of the maximum of the precisions of the inputs. Both [`Float`]s are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded remainder is less than, equal
    /// to, or greater than the exact remainder, along with the low bits of the quotient as an
    /// `i64`. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::rem_and_quotient_bits_prec`] instead. If you want to use a rounding mode other than
    /// `Nearest`, consider using [`Float::rem_and_quotient_bits_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) = Float::from(100u32).rem_and_quotient_bits_ref_ref(&Float::from(7u32));
    /// assert_eq!(r.to_string(), "2.00");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 14);
    /// ```
    #[inline]
    pub fn rem_and_quotient_bits_ref_ref(&self, other: &Self) -> (Self, Ordering, i64) {
        self.rem_and_quotient_bits_round_ref_ref(other, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the specified precision and with the specified rounding mode. Both [`Float`]s
    /// are taken by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ieee_remainder_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::ieee_remainder_round`] instead. If both of these things are
    /// true, consider using [`Float::ieee_remainder`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(14u32).ieee_remainder_prec_round(Float::from(3u32), 10, Nearest);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    ///
    /// let (r, o) = Float::from(10u32).ieee_remainder_prec_round(Float::from(7u32), 1, Floor);
    /// assert_eq!(r.to_string(), "2.0");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn ieee_remainder_prec_round(
        self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem1_helper(&self, &other, true, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the specified precision and with the specified rounding mode. The first
    /// [`Float`] is taken by value and the second by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ieee_remainder_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::ieee_remainder_round`] instead. If both of these things are
    /// true, consider using [`Float::ieee_remainder`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) =
    ///     Float::from(14u32).ieee_remainder_prec_round_val_ref(&Float::from(3u32), 10, Nearest);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    ///
    /// let (r, o) =
    ///     Float::from(10u32).ieee_remainder_prec_round_val_ref(&Float::from(7u32), 1, Floor);
    /// assert_eq!(r.to_string(), "2.0");
    /// assert_eq!(o, Less);
    /// ```
    pub fn ieee_remainder_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem1_helper(&self, other, true, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the specified precision and with the specified rounding mode. The first
    /// [`Float`] is taken by reference and the second by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ieee_remainder_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::ieee_remainder_round`] instead. If both of these things are
    /// true, consider using [`Float::ieee_remainder`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) =
    ///     Float::from(14u32).ieee_remainder_prec_round_ref_val(Float::from(3u32), 10, Nearest);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    ///
    /// let (r, o) =
    ///     Float::from(10u32).ieee_remainder_prec_round_ref_val(Float::from(7u32), 1, Floor);
    /// assert_eq!(r.to_string(), "2.0");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn ieee_remainder_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem1_helper(self, &other, true, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the specified precision and with the specified rounding mode. Both [`Float`]s
    /// are taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::ieee_remainder_prec`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::ieee_remainder_round`] instead. If both of these things are
    /// true, consider using [`Float::ieee_remainder`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) =
    ///     Float::from(14u32).ieee_remainder_prec_round_ref_ref(&Float::from(3u32), 10, Nearest);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    ///
    /// let (r, o) =
    ///     Float::from(10u32).ieee_remainder_prec_round_ref_ref(&Float::from(7u32), 1, Floor);
    /// assert_eq!(r.to_string(), "2.0");
    /// assert_eq!(o, Less);
    /// ```
    pub fn ieee_remainder_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem1_helper(self, other, true, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the specified precision. Both [`Float`]s are taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less
    /// than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ieee_remainder_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Float::ieee_remainder`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(14u32).ieee_remainder_prec(Float::from(3u32), 10);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn ieee_remainder_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        self.ieee_remainder_prec_round(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the specified precision. The first [`Float`] is taken by
    /// value and the second by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ieee_remainder_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Float::ieee_remainder`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(14u32).ieee_remainder_prec_val_ref(&Float::from(3u32), 10);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn ieee_remainder_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.ieee_remainder_prec_round_val_ref(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the specified precision. The first [`Float`] is taken by
    /// reference and the second by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ieee_remainder_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Float::ieee_remainder`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(14u32).ieee_remainder_prec_ref_val(Float::from(3u32), 10);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn ieee_remainder_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        self.ieee_remainder_prec_round_ref_val(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the specified precision. Both [`Float`]s are taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is
    /// less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ieee_remainder_prec_round`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Float::ieee_remainder`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(14u32).ieee_remainder_prec_ref_ref(&Float::from(3u32), 10);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn ieee_remainder_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.ieee_remainder_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the maximum of the precisions of the inputs, with the specified rounding mode.
    /// Both [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ieee_remainder_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::ieee_remainder`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(14u32).ieee_remainder_round(Float::from(3u32), Floor);
    /// assert_eq!(r.to_string(), "-1.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn ieee_remainder_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.ieee_remainder_prec_round(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the maximum of the precisions of the inputs, with the specified rounding mode.
    /// The first [`Float`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ieee_remainder_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::ieee_remainder`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(14u32).ieee_remainder_round_val_ref(&Float::from(3u32), Floor);
    /// assert_eq!(r.to_string(), "-1.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn ieee_remainder_round_val_ref(self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.ieee_remainder_prec_round_val_ref(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the maximum of the precisions of the inputs, with the specified rounding mode.
    /// The first [`Float`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ieee_remainder_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::ieee_remainder`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(14u32).ieee_remainder_round_ref_val(Float::from(3u32), Floor);
    /// assert_eq!(r.to_string(), "-1.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn ieee_remainder_round_ref_val(&self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.ieee_remainder_prec_round_ref_val(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the maximum of the precisions of the inputs, with the specified rounding mode.
    /// Both [`Float`]s are taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ieee_remainder_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::ieee_remainder`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(14u32).ieee_remainder_round_ref_ref(&Float::from(3u32), Floor);
    /// assert_eq!(r.to_string(), "-1.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn ieee_remainder_round_ref_ref(&self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.ieee_remainder_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the maximum of the precisions of the inputs. Both
    /// [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::ieee_remainder_prec`]
    /// instead. If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ieee_remainder_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     Float::from(14u32)
    ///         .ieee_remainder(Float::from(3u32))
    ///         .to_string(),
    ///     "-1.0"
    /// );
    ///
    /// assert_eq!(
    ///     Float::from(10u32)
    ///         .ieee_remainder(Float::from(3u32))
    ///         .to_string(),
    ///     "1.0"
    /// );
    /// ```
    #[inline]
    pub fn ieee_remainder(self, other: Self) -> Self {
        self.ieee_remainder_round(other, Nearest).0
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the maximum of the precisions of the inputs. The first
    /// [`Float`] is taken by value and the second by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::ieee_remainder_prec`]
    /// instead. If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ieee_remainder_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let r = Float::from(14u32).ieee_remainder_val_ref(&Float::from(3u32));
    /// assert_eq!(r.to_string(), "-1.0");
    ///
    /// let r = Float::from(10u32).ieee_remainder_val_ref(&Float::from(3u32));
    /// assert_eq!(r.to_string(), "1.0");
    /// ```
    #[inline]
    pub fn ieee_remainder_val_ref(self, other: &Self) -> Self {
        self.ieee_remainder_round_val_ref(other, Nearest).0
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the maximum of the precisions of the inputs. The first
    /// [`Float`] is taken by reference and the second by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::ieee_remainder_prec`]
    /// instead. If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ieee_remainder_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let r = Float::from(14u32).ieee_remainder_ref_val(Float::from(3u32));
    /// assert_eq!(r.to_string(), "-1.0");
    ///
    /// assert_eq!(
    ///     Float::from(10u32)
    ///         .ieee_remainder_ref_val(Float::from(3u32))
    ///         .to_string(),
    ///     "1.0"
    /// );
    /// ```
    #[inline]
    pub fn ieee_remainder_ref_val(&self, other: Self) -> Self {
        self.ieee_remainder_round_ref_val(other, Nearest).0
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the maximum of the precisions of the inputs. Both
    /// [`Float`]s are taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::ieee_remainder_prec`]
    /// instead. If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ieee_remainder_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let r = Float::from(14u32).ieee_remainder_ref_ref(&Float::from(3u32));
    /// assert_eq!(r.to_string(), "-1.0");
    ///
    /// let r = Float::from(10u32).ieee_remainder_ref_ref(&Float::from(3u32));
    /// assert_eq!(r.to_string(), "1.0");
    /// ```
    #[inline]
    pub fn ieee_remainder_ref_ref(&self, other: &Self) -> Self {
        self.ieee_remainder_round_ref_ref(other, Nearest).0
    }

    /// Computes the remainder of two [`Float`]s in place, with the quotient rounded to the nearest
    /// integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding
    /// the remainder to the specified precision and with the specified rounding mode. The [`Float`]
    /// on the right-hand side is taken by value. An [`Ordering`] is returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(14u32);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_round_assign(Float::from(3u32), 10, Nearest),
    ///     Equal
    /// );
    /// assert_eq!(x.to_string(), "-1.0000");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn ieee_remainder_prec_round_assign(
        &mut self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (r, o, _) = rem1_helper(self, &other, true, false, prec, rm);
        *self = r;
        o
    }

    /// Computes the remainder of two [`Float`]s in place, with the quotient rounded to the nearest
    /// integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding
    /// the remainder to the specified precision and with the specified rounding mode. The [`Float`]
    /// on the right-hand side is taken by reference. An [`Ordering`] is returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(14u32);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_round_assign_ref(&Float::from(3u32), 10, Nearest),
    ///     Equal
    /// );
    /// assert_eq!(x.to_string(), "-1.0000");
    /// ```
    pub fn ieee_remainder_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (r, o, _) = rem1_helper(self, other, true, false, prec, rm);
        *self = r;
        o
    }

    /// Computes the remainder of two [`Float`]s in place, with the quotient rounded to the nearest
    /// integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding
    /// the remainder to the nearest value of the specified precision. The [`Float`] on the
    /// right-hand side is taken by value. An [`Ordering`] is returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(14u32);
    /// assert_eq!(x.ieee_remainder_prec_assign(Float::from(3u32), 10), Equal);
    /// assert_eq!(x.to_string(), "-1.0000");
    /// ```
    #[inline]
    pub fn ieee_remainder_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        self.ieee_remainder_prec_round_assign(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s in place, with the quotient rounded to the nearest
    /// integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding
    /// the remainder to the nearest value of the specified precision. The [`Float`] on the
    /// right-hand side is taken by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(14u32);
    /// assert_eq!(
    ///     x.ieee_remainder_prec_assign_ref(&Float::from(3u32), 10),
    ///     Equal
    /// );
    /// assert_eq!(x.to_string(), "-1.0000");
    /// ```
    #[inline]
    pub fn ieee_remainder_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        self.ieee_remainder_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s in place, with the quotient rounded to the nearest
    /// integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding
    /// the remainder to the maximum of the precisions of the inputs, with the specified rounding
    /// mode. The [`Float`] on the right-hand side is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(14u32);
    /// assert_eq!(
    ///     x.ieee_remainder_round_assign(Float::from(3u32), Floor),
    ///     Equal
    /// );
    /// assert_eq!(x.to_string(), "-1.0");
    /// ```
    pub fn ieee_remainder_round_assign(&mut self, other: Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.ieee_remainder_prec_round_assign(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s in place, with the quotient rounded to the nearest
    /// integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding
    /// the remainder to the maximum of the precisions of the inputs, with the specified rounding
    /// mode. The [`Float`] on the right-hand side is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(14u32);
    /// assert_eq!(
    ///     x.ieee_remainder_round_assign_ref(&Float::from(3u32), Floor),
    ///     Equal
    /// );
    /// assert_eq!(x.to_string(), "-1.0");
    /// ```
    pub fn ieee_remainder_round_assign_ref(&mut self, other: &Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.ieee_remainder_prec_round_assign_ref(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s in place, with the quotient rounded to the nearest
    /// integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding
    /// the remainder to the nearest value of the maximum of the precisions of the inputs. The
    /// [`Float`] on the right-hand side is taken by value. An [`Ordering`] is returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(14u32);
    /// x.ieee_remainder_assign(Float::from(3u32));
    /// assert_eq!(x.to_string(), "-1.0");
    /// ```
    #[inline]
    pub fn ieee_remainder_assign(&mut self, other: Self) {
        self.ieee_remainder_round_assign(other, Nearest);
    }

    /// Computes the remainder of two [`Float`]s in place, with the quotient rounded to the nearest
    /// integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding
    /// the remainder to the nearest value of the maximum of the precisions of the inputs. The
    /// [`Float`] on the right-hand side is taken by reference. An [`Ordering`] is returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(14u32);
    /// x.ieee_remainder_assign_ref(&Float::from(3u32));
    /// assert_eq!(x.to_string(), "-1.0");
    /// ```
    #[inline]
    pub fn ieee_remainder_assign_ref(&mut self, other: &Self) {
        self.ieee_remainder_round_assign_ref(other, Nearest);
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the specified precision and with the specified rounding mode. Both [`Float`]s
    /// are taken by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. (MPFR documents the same contract for its `quo` output,
    /// but its C implementation can overflow a `long` when the low 63 bits are all ones and the
    /// quotient rounds away from zero; this implementation always keeps the modular contract.)
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec`] instead. If you know that your target
    /// precision is the maximum of the precisions of the two inputs, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_round`] instead. If both of these things are true,
    /// consider using [`Float::ieee_remainder_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(14u32);
    /// let y = Float::from(3u32);
    /// let (r, o, q) = x.ieee_remainder_and_quotient_bits_prec_round(y, 10, Floor);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec_round(
        self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem1_helper(&self, &other, true, true, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the specified precision and with the specified rounding mode. The first
    /// [`Float`] is taken by value and the second by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. (MPFR documents the same contract for its `quo` output,
    /// but its C implementation can overflow a `long` when the low 63 bits are all ones and the
    /// quotient rounds away from zero; this implementation always keeps the modular contract.)
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec`] instead. If you know that your target
    /// precision is the maximum of the precisions of the two inputs, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_round`] instead. If both of these things are true,
    /// consider using [`Float::ieee_remainder_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(14u32);
    /// let y = Float::from(3u32);
    /// let (r, o, q) = x.ieee_remainder_and_quotient_bits_prec_round_val_ref(&y, 10, Floor);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem1_helper(&self, other, true, true, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the specified precision and with the specified rounding mode. The first
    /// [`Float`] is taken by reference and the second by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. (MPFR documents the same contract for its `quo` output,
    /// but its C implementation can overflow a `long` when the low 63 bits are all ones and the
    /// quotient rounds away from zero; this implementation always keeps the modular contract.)
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec`] instead. If you know that your target
    /// precision is the maximum of the precisions of the two inputs, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_round`] instead. If both of these things are true,
    /// consider using [`Float::ieee_remainder_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(14u32);
    /// let y = Float::from(3u32);
    /// let (r, o, q) = x.ieee_remainder_and_quotient_bits_prec_round_ref_val(y, 10, Floor);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem1_helper(self, &other, true, true, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the specified precision and with the specified rounding mode. Both [`Float`]s
    /// are taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. (MPFR documents the same contract for its `quo` output,
    /// but its C implementation can overflow a `long` when the low 63 bits are all ones and the
    /// quotient rounds away from zero; this implementation always keeps the modular contract.)
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec`] instead. If you know that your target
    /// precision is the maximum of the precisions of the two inputs, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_round`] instead. If both of these things are true,
    /// consider using [`Float::ieee_remainder_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(14u32);
    /// let y = Float::from(3u32);
    /// let (r, o, q) = x.ieee_remainder_and_quotient_bits_prec_round_ref_ref(&y, 10, Floor);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem1_helper(self, other, true, true, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the specified precision. Both [`Float`]s are taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less
    /// than, equal to, or greater than the exact remainder, along with the low bits of the quotient
    /// as an `i64`. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec_round`] instead. If you know that your target
    /// precision is the maximum of the precisions of the two inputs, consider using
    /// [`Float::ieee_remainder_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) =
    ///     Float::from(14u32).ieee_remainder_and_quotient_bits_prec(Float::from(3u32), 10);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec(
        self,
        other: Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        self.ieee_remainder_and_quotient_bits_prec_round(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the specified precision. The first [`Float`] is taken by
    /// value and the second by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder, along with
    /// the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec_round`] instead. If you know that your target
    /// precision is the maximum of the precisions of the two inputs, consider using
    /// [`Float::ieee_remainder_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(14u32);
    /// let y = Float::from(3u32);
    /// let (r, o, q) = x.ieee_remainder_and_quotient_bits_prec_val_ref(&y, 10);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec_val_ref(
        self,
        other: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        self.ieee_remainder_and_quotient_bits_prec_round_val_ref(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the specified precision. The first [`Float`] is taken by
    /// reference and the second by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder, along with
    /// the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec_round`] instead. If you know that your target
    /// precision is the maximum of the precisions of the two inputs, consider using
    /// [`Float::ieee_remainder_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) =
    ///     Float::from(14u32).ieee_remainder_and_quotient_bits_prec_ref_val(Float::from(3u32), 10);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec_ref_val(
        &self,
        other: Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        self.ieee_remainder_and_quotient_bits_prec_round_ref_val(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the specified precision. Both [`Float`]s are taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is
    /// less than, equal to, or greater than the exact remainder, along with the low bits of the
    /// quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec_round`] instead. If you know that your target
    /// precision is the maximum of the precisions of the two inputs, consider using
    /// [`Float::ieee_remainder_and_quotient_bits`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(14u32);
    /// let y = Float::from(3u32);
    /// let (r, o, q) = x.ieee_remainder_and_quotient_bits_prec_ref_ref(&y, 10);
    /// assert_eq!(r.to_string(), "-1.0000");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_prec_ref_ref(
        &self,
        other: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        self.ieee_remainder_and_quotient_bits_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the maximum of the precisions of the inputs, with the specified rounding mode.
    /// Both [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder, along with
    /// the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec_round`] instead. If you know you'll be using
    /// the `Nearest` rounding mode, consider using [`Float::ieee_remainder_and_quotient_bits`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) =
    ///     Float::from(14u32).ieee_remainder_and_quotient_bits_round(Float::from(3u32), Floor);
    /// assert_eq!(r.to_string(), "-1.0");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    pub fn ieee_remainder_and_quotient_bits_round(
        self,
        other: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.ieee_remainder_and_quotient_bits_prec_round(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the maximum of the precisions of the inputs, with the specified rounding mode.
    /// The first [`Float`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec_round`] instead. If you know you'll be using
    /// the `Nearest` rounding mode, consider using [`Float::ieee_remainder_and_quotient_bits`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(14u32);
    /// let y = Float::from(3u32);
    /// let (r, o, q) = x.ieee_remainder_and_quotient_bits_round_val_ref(&y, Floor);
    /// assert_eq!(r.to_string(), "-1.0");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    pub fn ieee_remainder_and_quotient_bits_round_val_ref(
        self,
        other: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.ieee_remainder_and_quotient_bits_prec_round_val_ref(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the maximum of the precisions of the inputs, with the specified rounding mode.
    /// The first [`Float`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec_round`] instead. If you know you'll be using
    /// the `Nearest` rounding mode, consider using [`Float::ieee_remainder_and_quotient_bits`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(14u32);
    /// let y = Float::from(3u32);
    /// let (r, o, q) = x.ieee_remainder_and_quotient_bits_round_ref_val(y, Floor);
    /// assert_eq!(r.to_string(), "-1.0");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    pub fn ieee_remainder_and_quotient_bits_round_ref_val(
        &self,
        other: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.ieee_remainder_and_quotient_bits_prec_round_ref_val(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the maximum of the precisions of the inputs, with the specified rounding mode.
    /// Both [`Float`]s are taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder, along
    /// with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec_round`] instead. If you know you'll be using
    /// the `Nearest` rounding mode, consider using [`Float::ieee_remainder_and_quotient_bits`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let x = Float::from(14u32);
    /// let y = Float::from(3u32);
    /// let (r, o, q) = x.ieee_remainder_and_quotient_bits_round_ref_ref(&y, Floor);
    /// assert_eq!(r.to_string(), "-1.0");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    pub fn ieee_remainder_and_quotient_bits_round_ref_ref(
        &self,
        other: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.ieee_remainder_and_quotient_bits_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the maximum of the precisions of the inputs. Both
    /// [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder, along with
    /// the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec`] instead. If you want to use a rounding mode
    /// other than `Nearest`, consider using [`Float::ieee_remainder_and_quotient_bits_round`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) = Float::from(14u32).ieee_remainder_and_quotient_bits(Float::from(3u32));
    /// assert_eq!(r.to_string(), "-1.0");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits(self, other: Self) -> (Self, Ordering, i64) {
        self.ieee_remainder_and_quotient_bits_round(other, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the maximum of the precisions of the inputs. The first
    /// [`Float`] is taken by value and the second by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec`] instead. If you want to use a rounding mode
    /// other than `Nearest`, consider using [`Float::ieee_remainder_and_quotient_bits_round`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) =
    ///     Float::from(14u32).ieee_remainder_and_quotient_bits_val_ref(&Float::from(3u32));
    /// assert_eq!(r.to_string(), "-1.0");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_val_ref(self, other: &Self) -> (Self, Ordering, i64) {
        self.ieee_remainder_and_quotient_bits_round_val_ref(other, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the maximum of the precisions of the inputs. The first
    /// [`Float`] is taken by reference and the second by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec`] instead. If you want to use a rounding mode
    /// other than `Nearest`, consider using [`Float::ieee_remainder_and_quotient_bits_round`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) =
    ///     Float::from(14u32).ieee_remainder_and_quotient_bits_ref_val(Float::from(3u32));
    /// assert_eq!(r.to_string(), "-1.0");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_ref_val(&self, other: Self) -> (Self, Ordering, i64) {
        self.ieee_remainder_and_quotient_bits_round_ref_val(other, Nearest)
    }

    /// Computes the remainder of two [`Float`]s, with the quotient rounded to the nearest integer,
    /// ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`, rounding the
    /// remainder to the nearest value of the maximum of the precisions of the inputs. Both
    /// [`Float`]s are taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder, along with
    /// the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::ieee_remainder_and_quotient_bits_prec`] instead. If you want to use a rounding mode
    /// other than `Nearest`, consider using [`Float::ieee_remainder_and_quotient_bits_round`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o, q) =
    ///     Float::from(14u32).ieee_remainder_and_quotient_bits_ref_ref(&Float::from(3u32));
    /// assert_eq!(r.to_string(), "-1.0");
    /// assert_eq!(o, Equal);
    /// assert_eq!(q, 5);
    /// ```
    #[inline]
    pub fn ieee_remainder_and_quotient_bits_ref_ref(&self, other: &Self) -> (Self, Ordering, i64) {
        self.ieee_remainder_and_quotient_bits_round_ref_ref(other, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a `u64`, with the quotient rounded toward zero,
    /// rounding the remainder to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The conversion of the modulus to a [`Float`] is exact, so this behaves exactly like the
    /// corresponding `rem` function with `Float::from(other)`, except that a zero modulus yields
    /// `NaN` (matching `mpfr_fmod_ui`) rather than following the `Float` special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_unsigned_prec_round(7, 1, Floor);
    /// assert_eq!(r.to_string(), "2.0");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = Float::from(10u32).rem_unsigned_prec_round(7, 1, Ceiling);
    /// assert_eq!(r.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rem_unsigned_prec_round(
        self,
        other: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rem_unsigned_helper(&self, other, false, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a `u64`, with the quotient rounded toward zero,
    /// rounding the remainder to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The conversion of the modulus to a [`Float`] is exact, so this behaves exactly like the
    /// corresponding `rem` function with `Float::from(other)`, except that a zero modulus yields
    /// `NaN` (matching `mpfr_fmod_ui`) rather than following the `Float` special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_unsigned_prec_round_ref(7, 1, Floor);
    /// assert_eq!(r.to_string(), "2.0");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) = Float::from(10u32).rem_unsigned_prec_round_ref(7, 1, Ceiling);
    /// assert_eq!(r.to_string(), "4.0");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rem_unsigned_prec_round_ref(
        &self,
        other: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rem_unsigned_helper(self, other, false, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a `u64`, with the quotient rounded toward zero,
    /// rounding the remainder to the nearest value of the specified precision. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder
    /// is less than, equal to, or greater than the exact remainder. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The conversion of the modulus to a [`Float`] is exact, so this behaves exactly like the
    /// corresponding `rem` function with `Float::from(other)`, except that a zero modulus yields
    /// `NaN` (matching `mpfr_fmod_ui`) rather than following the `Float` special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_unsigned_prec(3, 10);
    /// assert_eq!(r.to_string(), "1.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn rem_unsigned_prec(self, other: u64, prec: u64) -> (Self, Ordering) {
        rem_unsigned_helper(&self, other, false, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a `u64`, with the quotient rounded toward zero,
    /// rounding the remainder to the nearest value of the specified precision. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The conversion of the modulus to a [`Float`] is exact, so this behaves exactly like the
    /// corresponding `rem` function with `Float::from(other)`, except that a zero modulus yields
    /// `NaN` (matching `mpfr_fmod_ui`) rather than following the `Float` special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_unsigned_prec_ref(3, 10);
    /// assert_eq!(r.to_string(), "1.0000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn rem_unsigned_prec_ref(&self, other: u64, prec: u64) -> (Self, Ordering) {
        rem_unsigned_helper(self, other, false, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a `u64`, with the quotient rounded toward zero,
    /// rounding the remainder to `self.significant_bits()` bits, with the specified rounding mode.
    /// The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The conversion of the modulus to a [`Float`] is exact, so this behaves exactly like the
    /// corresponding `rem` function with `Float::from(other)`, except that a zero modulus yields
    /// `NaN` (matching `mpfr_fmod_ui`) rather than following the `Float` special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_unsigned_round(3, Floor);
    /// assert_eq!(r.to_string(), "1.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn rem_unsigned_round(self, other: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        rem_unsigned_helper(&self, other, false, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a `u64`, with the quotient rounded toward zero,
    /// rounding the remainder to `self.significant_bits()` bits, with the specified rounding mode.
    /// The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The conversion of the modulus to a [`Float`] is exact, so this behaves exactly like the
    /// corresponding `rem` function with `Float::from(other)`, except that a zero modulus yields
    /// `NaN` (matching `mpfr_fmod_ui`) rather than following the `Float` special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    ///
    /// let (r, o) = Float::from(10u32).rem_unsigned_round_ref(3, Floor);
    /// assert_eq!(r.to_string(), "1.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn rem_unsigned_round_ref(&self, other: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        rem_unsigned_helper(self, other, false, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a `u64`, with the quotient rounded toward zero,
    /// rounding the remainder to the nearest value of `self.significant_bits()` bits. The [`Float`]
    /// is taken by value.
    ///
    /// The conversion of the modulus to a [`Float`] is exact, so this behaves exactly like the
    /// corresponding `rem` function with `Float::from(other)`, except that a zero modulus yields
    /// `NaN` (matching `mpfr_fmod_ui`) rather than following the `Float` special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(10u32).rem_unsigned(3).to_string(), "1.0");
    ///
    /// assert_eq!(Float::from(10u32).rem_unsigned(0).to_string(), "NaN");
    /// ```
    #[inline]
    pub fn rem_unsigned(self, other: u64) -> Self {
        self.rem_unsigned_round(other, Nearest).0
    }

    /// Computes the remainder of a [`Float`] by a `u64`, with the quotient rounded toward zero,
    /// rounding the remainder to the nearest value of `self.significant_bits()` bits. The [`Float`]
    /// is taken by reference.
    ///
    /// The conversion of the modulus to a [`Float`] is exact, so this behaves exactly like the
    /// corresponding `rem` function with `Float::from(other)`, except that a zero modulus yields
    /// `NaN` (matching `mpfr_fmod_ui`) rather than following the `Float` special cases.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(10u32).rem_unsigned_ref(3).to_string(), "1.0");
    ///
    /// assert_eq!(Float::from(10u32).rem_unsigned_ref(0).to_string(), "NaN");
    /// ```
    #[inline]
    pub fn rem_unsigned_ref(&self, other: u64) -> Self {
        self.rem_unsigned_round_ref(other, Nearest).0
    }
}

impl Float {
    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] and the [`Rational`]
    /// are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(10u32).rem_rational_prec_round(Rational::from_signeds(22, 7), 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (r, o) =
    ///     Float::from(10u32).rem_rational_prec_round(Rational::from_signeds(22, 7), 5, Ceiling);
    /// assert_eq!(r.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem_rational_helper(&self, &other, false, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by value
    /// and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.rem_rational_prec_round_val_ref(&y, 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.rem_rational_prec_round_val_ref(&y, 5, Ceiling);
    /// assert_eq!(r.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn rem_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem_rational_helper(&self, other, false, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.rem_rational_prec_round_ref_val(y, 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.rem_rational_prec_round_ref_val(y, 5, Ceiling);
    /// assert_eq!(r.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem_rational_helper(self, &other, false, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] and the [`Rational`]
    /// are both taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.rem_rational_prec_round_ref_ref(&y, 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.rem_rational_prec_round_ref_ref(&y, 5, Ceiling);
    /// assert_eq!(r.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn rem_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem_rational_helper(self, other, false, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Float`] and the [`Rational`] are both taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less
    /// than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(10u32).rem_rational_prec(Rational::from_signeds(22, 7), 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rem_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        self.rem_rational_prec_round(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by value and the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(10u32).rem_rational_prec_val_ref(&Rational::from_signeds(22, 7), 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rem_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Self, Ordering) {
        self.rem_rational_prec_round_val_ref(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by reference and the
    /// [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(10u32).rem_rational_prec_ref_val(Rational::from_signeds(22, 7), 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rem_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Self, Ordering) {
        self.rem_rational_prec_round_ref_val(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Float`] and the [`Rational`] are both taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is
    /// less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(10u32).rem_rational_prec_ref_ref(&Rational::from_signeds(22, 7), 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rem_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Self, Ordering) {
        self.rem_rational_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] and the [`Rational`]
    /// are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) = Float::from(10u32).rem_rational_round(Rational::from_signeds(22, 7), Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rem_rational_round(self, other: Rational, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.rem_rational_prec_round(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] is taken by value and
    /// the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(10u32).rem_rational_round_val_ref(&Rational::from_signeds(22, 7), Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rem_rational_round_val_ref(
        self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.rem_rational_prec_round_val_ref(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] is taken by reference
    /// and the [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(10u32).rem_rational_round_ref_val(Rational::from_signeds(22, 7), Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rem_rational_round_ref_val(
        &self,
        other: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.rem_rational_prec_round_ref_val(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] and the [`Rational`]
    /// are both taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(10u32).rem_rational_round_ref_ref(&Rational::from_signeds(22, 7), Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rem_rational_round_ref_ref(
        &self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.rem_rational_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// toward zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Rational`] is taken
    /// by value. An [`Ordering`] is returned, indicating whether the rounded remainder is less
    /// than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// assert_eq!(
    ///     x.rem_rational_prec_round_assign(Rational::from_signeds(22, 7), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.562");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn rem_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (r, o, _) = rem_rational_helper(self, &other, false, false, prec, rm);
        *self = r;
        o
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// toward zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Rational`] is taken
    /// by reference. An [`Ordering`] is returned, indicating whether the rounded remainder is less
    /// than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// assert_eq!(x.rem_rational_prec_round_assign_ref(&y, 5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.562");
    /// ```
    pub fn rem_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (r, o, _) = rem_rational_helper(self, other, false, false, prec, rm);
        *self = r;
        o
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// toward zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result
    /// to the nearest value of the specified precision. The [`Rational`] is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded remainder is less than, equal to,
    /// or greater than the exact remainder. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// assert_eq!(
    ///     x.rem_rational_prec_assign(Rational::from_signeds(22, 7), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.562");
    /// ```
    #[inline]
    pub fn rem_rational_prec_assign(&mut self, other: Rational, prec: u64) -> Ordering {
        self.rem_rational_prec_round_assign(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// toward zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result
    /// to the nearest value of the specified precision. The [`Rational`] is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded remainder is less than, equal to,
    /// or greater than the exact remainder. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// assert_eq!(
    ///     x.rem_rational_prec_assign_ref(&Rational::from_signeds(22, 7), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.562");
    /// ```
    #[inline]
    pub fn rem_rational_prec_assign_ref(&mut self, other: &Rational, prec: u64) -> Ordering {
        self.rem_rational_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// toward zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result
    /// to the [`Float`]'s precision, with the specified rounding mode. The [`Rational`] is taken by
    /// value. An [`Ordering`] is returned, indicating whether the rounded remainder is less than,
    /// equal to, or greater than the exact remainder. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// assert_eq!(
    ///     x.rem_rational_round_assign(Rational::from_signeds(22, 7), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.50");
    /// ```
    #[inline]
    pub fn rem_rational_round_assign(&mut self, other: Rational, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.rem_rational_prec_round_assign(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// toward zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result
    /// to the [`Float`]'s precision, with the specified rounding mode. The [`Rational`] is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded remainder is less
    /// than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// assert_eq!(
    ///     x.rem_rational_round_assign_ref(&Rational::from_signeds(22, 7), Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.50");
    /// ```
    #[inline]
    pub fn rem_rational_round_assign_ref(
        &mut self,
        other: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = self.significant_bits();
        self.rem_rational_prec_round_assign_ref(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] and the [`Rational`]
    /// are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.rem_rational_and_quotient_bits_prec_round(y, 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_helper(&self, &other, false, true, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by value
    /// and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder, along with
    /// the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.rem_rational_and_quotient_bits_prec_round_val_ref(&y, 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_helper(&self, other, false, true, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder,
    /// along with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.rem_rational_and_quotient_bits_prec_round_ref_val(y, 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_helper(self, &other, false, true, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] and the [`Rational`]
    /// are both taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder, along with
    /// the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.rem_rational_and_quotient_bits_prec_round_ref_ref(&y, 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_helper(self, other, false, true, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Float`] and the [`Rational`] are both taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less
    /// than, equal to, or greater than the exact remainder, along with the low bits of the quotient
    /// as an `i64`. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.rem_rational_and_quotient_bits_prec(y, 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec(
        self,
        other: Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        self.rem_rational_and_quotient_bits_prec_round(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by value and the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.rem_rational_and_quotient_bits_prec_val_ref(&y, 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec_val_ref(
        self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        self.rem_rational_and_quotient_bits_prec_round_val_ref(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by reference and the
    /// [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.rem_rational_and_quotient_bits_prec_ref_val(y, 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec_ref_val(
        &self,
        other: Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        self.rem_rational_and_quotient_bits_prec_round_ref_val(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Float`] and the [`Rational`] are both taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is
    /// less than, equal to, or greater than the exact remainder, along with the low bits of the
    /// quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.rem_rational_and_quotient_bits_prec_ref_ref(&y, 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_prec_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        self.rem_rational_and_quotient_bits_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] and the [`Rational`]
    /// are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.rem_rational_and_quotient_bits_round(y, Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_round(
        self,
        other: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = self.significant_bits();
        self.rem_rational_and_quotient_bits_prec_round(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] is taken by value and
    /// the [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder, along with
    /// the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.rem_rational_and_quotient_bits_round_val_ref(&y, Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_round_val_ref(
        self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = self.significant_bits();
        self.rem_rational_and_quotient_bits_prec_round_val_ref(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] is taken by reference
    /// and the [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder, along with
    /// the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.rem_rational_and_quotient_bits_round_ref_val(y, Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_round_ref_val(
        &self,
        other: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = self.significant_bits();
        self.rem_rational_and_quotient_bits_prec_round_ref_val(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`]'s precision, with the specified rounding mode. The [`Float`] and the [`Rational`]
    /// are both taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder, along with
    /// the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.rem_rational_and_quotient_bits_round_ref_ref(&y, Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_round_ref_ref(
        &self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = self.significant_bits();
        self.rem_rational_and_quotient_bits_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the [`Float`]'s precision. The [`Float`] and the [`Rational`] are both
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded remainder
    /// is less than, equal to, or greater than the exact remainder, along with the low bits of the
    /// quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o, q) =
    ///     Float::from(10u32).rem_rational_and_quotient_bits(Rational::from_signeds(22, 7));
    /// assert_eq!(r.to_string(), "0.62");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits(self, other: Rational) -> (Self, Ordering, i64) {
        self.rem_rational_and_quotient_bits_round(other, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the [`Float`]'s precision. The [`Float`] is taken by value and the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.rem_rational_and_quotient_bits_val_ref(&y);
    /// assert_eq!(r.to_string(), "0.62");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_val_ref(self, other: &Rational) -> (Self, Ordering, i64) {
        self.rem_rational_and_quotient_bits_round_val_ref(other, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the [`Float`]'s precision. The [`Float`] is taken by reference and the
    /// [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.rem_rational_and_quotient_bits_ref_val(y);
    /// assert_eq!(r.to_string(), "0.62");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_ref_val(&self, other: Rational) -> (Self, Ordering, i64) {
        self.rem_rational_and_quotient_bits_round_ref_val(other, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the [`Float`]'s precision. The [`Float`] and the [`Rational`] are both
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.rem_rational_and_quotient_bits_ref_ref(&y);
    /// assert_eq!(r.to_string(), "0.62");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn rem_rational_and_quotient_bits_ref_ref(
        &self,
        other: &Rational,
    ) -> (Self, Ordering, i64) {
        self.rem_rational_and_quotient_bits_round_ref_ref(other, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] and the [`Rational`] are both taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.ieee_remainder_rational_prec_round(y, 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn ieee_remainder_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem_rational_helper(&self, &other, true, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.ieee_remainder_rational_prec_round_val_ref(&y, 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// ```
    pub fn ieee_remainder_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem_rational_helper(&self, other, true, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.ieee_remainder_rational_prec_round_ref_val(y, 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn ieee_remainder_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem_rational_helper(self, &other, true, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] and the [`Rational`] are both taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.ieee_remainder_rational_prec_round_ref_ref(&y, 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// ```
    pub fn ieee_remainder_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rem_rational_helper(self, other, true, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] and the
    /// [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(10u32).ieee_remainder_rational_prec(Rational::from_signeds(22, 7), 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        self.ieee_remainder_rational_prec_round(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by value and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.ieee_remainder_rational_prec_val_ref(&y, 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_prec_val_ref(
        self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.ieee_remainder_rational_prec_round_val_ref(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.ieee_remainder_rational_prec_ref_val(y, 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_prec_ref_val(
        &self,
        other: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.ieee_remainder_rational_prec_round_ref_val(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] and the
    /// [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.ieee_remainder_rational_prec_ref_ref(&y, 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_prec_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        self.ieee_remainder_rational_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`]'s precision, with the specified rounding mode. The
    /// [`Float`] and the [`Rational`] are both taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::from(10u32).ieee_remainder_rational_round(Rational::from_signeds(22, 7), Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_round(
        self,
        other: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.ieee_remainder_rational_prec_round(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`]'s precision, with the specified rounding mode. The
    /// [`Float`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.ieee_remainder_rational_round_val_ref(&y, Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_round_val_ref(
        self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.ieee_remainder_rational_prec_round_val_ref(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`]'s precision, with the specified rounding mode. The
    /// [`Float`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.ieee_remainder_rational_round_ref_val(y, Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_round_ref_val(
        &self,
        other: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.ieee_remainder_rational_prec_round_ref_val(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`]'s precision, with the specified rounding mode. The
    /// [`Float`] and the [`Rational`] are both taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o) = x.ieee_remainder_rational_round_ref_ref(&y, Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_round_ref_ref(
        &self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.ieee_remainder_rational_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`]'s precision. The [`Float`] and the
    /// [`Rational`] are both taken by value.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let r = Float::from(10u32).ieee_remainder_rational(Rational::from_signeds(22, 7));
    /// assert_eq!(r.to_string(), "0.62");
    /// ```
    #[inline]
    pub fn ieee_remainder_rational(self, other: Rational) -> Self {
        self.ieee_remainder_rational_round(other, Nearest).0
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`]'s precision. The [`Float`] is
    /// taken by value and the [`Rational`] by reference.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let r = Float::from(10u32).ieee_remainder_rational_val_ref(&Rational::from_signeds(22, 7));
    /// assert_eq!(r.to_string(), "0.62");
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_val_ref(self, other: &Rational) -> Self {
        self.ieee_remainder_rational_round_val_ref(other, Nearest).0
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`]'s precision. The [`Float`] is
    /// taken by reference and the [`Rational`] by value.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let r = Float::from(10u32).ieee_remainder_rational_ref_val(Rational::from_signeds(22, 7));
    /// assert_eq!(r.to_string(), "0.62");
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_ref_val(&self, other: Rational) -> Self {
        self.ieee_remainder_rational_round_ref_val(other, Nearest).0
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`]'s precision. The [`Float`] and the
    /// [`Rational`] are both taken by reference.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let r = Float::from(10u32).ieee_remainder_rational_ref_ref(&Rational::from_signeds(22, 7));
    /// assert_eq!(r.to_string(), "0.62");
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_ref_ref(&self, other: &Rational) -> Self {
        self.ieee_remainder_rational_round_ref_ref(other, Nearest).0
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// to the nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's
    /// `remainder`, rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Rational`] is taken by value. An [`Ordering`] is returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_round_assign(y, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.562");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn ieee_remainder_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (r, o, _) = rem_rational_helper(self, &other, true, false, prec, rm);
        *self = r;
        o
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// to the nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's
    /// `remainder`, rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Rational`] is taken by reference. An [`Ordering`] is returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_round_assign_ref(&y, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.562");
    /// ```
    pub fn ieee_remainder_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (r, o, _) = rem_rational_helper(self, other, true, false, prec, rm);
        *self = r;
        o
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// to the nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's
    /// `remainder`, rounding the result to the nearest value of the specified precision. The
    /// [`Rational`] is taken by value. An [`Ordering`] is returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// assert_eq!(
    ///     x.ieee_remainder_rational_prec_assign(Rational::from_signeds(22, 7), 5),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "0.562");
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_prec_assign(&mut self, other: Rational, prec: u64) -> Ordering {
        self.ieee_remainder_rational_prec_round_assign(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// to the nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's
    /// `remainder`, rounding the result to the nearest value of the specified precision. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// assert_eq!(x.ieee_remainder_rational_prec_assign_ref(&y, 5), Less);
    /// assert_eq!(x.to_string(), "0.562");
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_prec_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
    ) -> Ordering {
        self.ieee_remainder_rational_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// to the nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's
    /// `remainder`, rounding the result to the [`Float`]'s precision, with the specified rounding
    /// mode. The [`Rational`] is taken by value. An [`Ordering`] is returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// assert_eq!(x.ieee_remainder_rational_round_assign(y, Floor), Less);
    /// assert_eq!(x.to_string(), "0.50");
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_round_assign(
        &mut self,
        other: Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = self.significant_bits();
        self.ieee_remainder_rational_prec_round_assign(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// to the nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's
    /// `remainder`, rounding the result to the [`Float`]'s precision, with the specified rounding
    /// mode. The [`Rational`] is taken by reference. An [`Ordering`] is returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// assert_eq!(x.ieee_remainder_rational_round_assign_ref(&y, Floor), Less);
    /// assert_eq!(x.to_string(), "0.50");
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_round_assign_ref(
        &mut self,
        other: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = self.significant_bits();
        self.ieee_remainder_rational_prec_round_assign_ref(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// to the nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's
    /// `remainder`, rounding the result to the nearest value of the [`Float`]'s precision. The
    /// [`Rational`] is taken by value. An [`Ordering`] is returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// x.ieee_remainder_rational_assign(Rational::from_signeds(22, 7));
    /// assert_eq!(x.to_string(), "0.62");
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_assign(&mut self, other: Rational) {
        self.ieee_remainder_rational_round_assign(other, Nearest);
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// to the nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's
    /// `remainder`, rounding the result to the nearest value of the [`Float`]'s precision. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// x.ieee_remainder_rational_assign_ref(&Rational::from_signeds(22, 7));
    /// assert_eq!(x.to_string(), "0.62");
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_assign_ref(&mut self, other: &Rational) {
        self.ieee_remainder_rational_round_assign_ref(other, Nearest);
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] and the [`Rational`] are both taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.ieee_remainder_rational_and_quotient_bits_prec_round(y, 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_helper(&self, &other, true, true, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) =
    ///     x.ieee_remainder_rational_and_quotient_bits_prec_round_val_ref(&y, 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_helper(&self, other, true, true, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.ieee_remainder_rational_and_quotient_bits_prec_round_ref_val(y, 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_helper(self, &other, true, true, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] and the [`Rational`] are both taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) =
    ///     x.ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(&y, 5, Floor);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rem_rational_helper(self, other, true, true, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] and the
    /// [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder, along
    /// with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.ieee_remainder_rational_and_quotient_bits_prec(y, 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec(
        self,
        other: Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        self.ieee_remainder_rational_and_quotient_bits_prec_round(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by value and the [`Rational`] by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder,
    /// along with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.ieee_remainder_rational_and_quotient_bits_prec_val_ref(&y, 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec_val_ref(
        self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        self.ieee_remainder_rational_and_quotient_bits_prec_round_val_ref(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by reference and the [`Rational`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder,
    /// along with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.ieee_remainder_rational_and_quotient_bits_prec_ref_val(y, 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec_ref_val(
        &self,
        other: Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        self.ieee_remainder_rational_and_quotient_bits_prec_round_ref_val(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Float`] and the
    /// [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder,
    /// along with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.ieee_remainder_rational_and_quotient_bits_prec_ref_ref(&y, 5);
    /// assert_eq!(r.to_string(), "0.562");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_prec_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        self.ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`]'s precision, with the specified rounding mode. The
    /// [`Float`] and the [`Rational`] are both taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.ieee_remainder_rational_and_quotient_bits_round(y, Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_round(
        self,
        other: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = self.significant_bits();
        self.ieee_remainder_rational_and_quotient_bits_prec_round(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`]'s precision, with the specified rounding mode. The
    /// [`Float`] is taken by value and the [`Rational`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.ieee_remainder_rational_and_quotient_bits_round_val_ref(&y, Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_round_val_ref(
        self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = self.significant_bits();
        self.ieee_remainder_rational_and_quotient_bits_prec_round_val_ref(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`]'s precision, with the specified rounding mode. The
    /// [`Float`] is taken by reference and the [`Rational`] by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.ieee_remainder_rational_and_quotient_bits_round_ref_val(y, Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_round_ref_val(
        &self,
        other: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = self.significant_bits();
        self.ieee_remainder_rational_and_quotient_bits_prec_round_ref_val(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`]'s precision, with the specified rounding mode. The
    /// [`Float`] and the [`Rational`] are both taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.ieee_remainder_rational_and_quotient_bits_round_ref_ref(&y, Floor);
    /// assert_eq!(r.to_string(), "0.50");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_round_ref_ref(
        &self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = self.significant_bits();
        self.ieee_remainder_rational_and_quotient_bits_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`]'s precision. The [`Float`] and the
    /// [`Rational`] are both taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder, along
    /// with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.ieee_remainder_rational_and_quotient_bits(y);
    /// assert_eq!(r.to_string(), "0.62");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits(
        self,
        other: Rational,
    ) -> (Self, Ordering, i64) {
        self.ieee_remainder_rational_and_quotient_bits_round(other, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`]'s precision. The [`Float`] is
    /// taken by value and the [`Rational`] by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.ieee_remainder_rational_and_quotient_bits_val_ref(&y);
    /// assert_eq!(r.to_string(), "0.62");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_val_ref(
        self,
        other: &Rational,
    ) -> (Self, Ordering, i64) {
        self.ieee_remainder_rational_and_quotient_bits_round_val_ref(other, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`]'s precision. The [`Float`] is
    /// taken by reference and the [`Rational`] by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.ieee_remainder_rational_and_quotient_bits_ref_val(y);
    /// assert_eq!(r.to_string(), "0.62");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_ref_val(
        &self,
        other: Rational,
    ) -> (Self, Ordering, i64) {
        self.ieee_remainder_rational_and_quotient_bits_round_ref_val(other, Nearest)
    }

    /// Computes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`]'s precision. The [`Float`] and the
    /// [`Rational`] are both taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder,
    /// along with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] modulus is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values. Converting the modulus to a [`Float`] first would perturb the
    /// remainder by up to the quotient times the conversion error.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(\pm\infty,y,p)=f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let x = Float::from(10u32);
    /// let y = Rational::from_signeds(22, 7);
    /// let (r, o, q) = x.ieee_remainder_rational_and_quotient_bits_ref_ref(&y);
    /// assert_eq!(r.to_string(), "0.62");
    /// assert_eq!(o, Greater);
    /// assert_eq!(q, 3);
    /// ```
    #[inline]
    pub fn ieee_remainder_rational_and_quotient_bits_ref_ref(
        &self,
        other: &Rational,
    ) -> (Self, Ordering, i64) {
        self.ieee_remainder_rational_and_quotient_bits_round_ref_ref(other, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] and the [`Float`]
    /// are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_rem_float_prec_round(a, b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn rational_rem_float_prec_round(
        x: Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_float_helper(&x, &y, false, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] is taken by value
    /// and the [`Float`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_rem_float_prec_round_val_ref(a, &b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn rational_rem_float_prec_round_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_float_helper(&x, y, false, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] is taken by
    /// reference and the [`Float`] by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_rem_float_prec_round_ref_val(&a, b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn rational_rem_float_prec_round_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_float_helper(x, &y, false, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] and the [`Float`]
    /// are both taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_rem_float_prec_round_ref_ref(&a, &b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    pub fn rational_rem_float_prec_round_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_float_helper(x, y, false, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Rational`] and the [`Float`] are both taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less
    /// than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let (r, o) =
    ///     Float::rational_rem_float_prec(Rational::from_signeds(22, 7), Float::from(3u32), 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_rem_float_prec(x: Rational, y: Self, prec: u64) -> (Self, Ordering) {
        Self::rational_rem_float_prec_round(x, y, prec, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Rational`] is taken by value and the
    /// [`Float`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_rem_float_prec_val_ref(a, &b, 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_rem_float_prec_val_ref(x: Rational, y: &Self, prec: u64) -> (Self, Ordering) {
        Self::rational_rem_float_prec_round_val_ref(x, y, prec, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Rational`] is taken by reference and the
    /// [`Float`] by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_rem_float_prec_ref_val(&a, b, 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_rem_float_prec_ref_val(x: &Rational, y: Self, prec: u64) -> (Self, Ordering) {
        Self::rational_rem_float_prec_round_ref_val(x, y, prec, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Rational`] and the [`Float`] are both taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is
    /// less than, equal to, or greater than the exact remainder. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_rem_float_prec_ref_ref(&a, &b, 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_rem_float_prec_ref_ref(x: &Rational, y: &Self, prec: u64) -> (Self, Ordering) {
        Self::rational_rem_float_prec_round_ref_ref(x, y, prec, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`] modulus's precision, with the specified rounding mode. The [`Rational`] and the
    /// [`Float`] are both taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_rem_float_round(a, b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_rem_float_round(x: Rational, y: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = y.significant_bits();
        Self::rational_rem_float_prec_round(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`] modulus's precision, with the specified rounding mode. The [`Rational`] is taken
    /// by value and the [`Float`] by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_rem_float_round_val_ref(a, &b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_rem_float_round_val_ref(
        x: Rational,
        y: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = y.significant_bits();
        Self::rational_rem_float_prec_round_val_ref(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`] modulus's precision, with the specified rounding mode. The [`Rational`] is taken
    /// by reference and the [`Float`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_rem_float_round_ref_val(&a, b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_rem_float_round_ref_val(
        x: &Rational,
        y: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = y.significant_bits();
        Self::rational_rem_float_prec_round_ref_val(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`] modulus's precision, with the specified rounding mode. The [`Rational`] and the
    /// [`Float`] are both taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_rem_float_round_ref_ref(&a, &b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_rem_float_round_ref_ref(
        x: &Rational,
        y: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = y.significant_bits();
        Self::rational_rem_float_prec_round_ref_ref(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] and the [`Float`]
    /// are both taken by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_rem_float_and_quotient_bits_prec_round(a, b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_rem_float_and_quotient_bits_prec_round(
        x: Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_float_helper(&x, &y, false, true, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] is taken by value
    /// and the [`Float`] by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder, along with
    /// the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) =
    ///     Float::rational_rem_float_and_quotient_bits_prec_round_val_ref(a, &b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_rem_float_and_quotient_bits_prec_round_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_float_helper(&x, y, false, true, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] is taken by
    /// reference and the [`Float`] by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder, along
    /// with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) =
    ///     Float::rational_rem_float_and_quotient_bits_prec_round_ref_val(&a, b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_rem_float_and_quotient_bits_prec_round_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_float_helper(x, &y, false, true, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Rational`] and the [`Float`]
    /// are both taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder, along with
    /// the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) =
    ///     Float::rational_rem_float_and_quotient_bits_prec_round_ref_ref(&a, &b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_rem_float_and_quotient_bits_prec_round_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_float_helper(x, y, false, true, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Rational`] and the [`Float`] are both taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded remainder is less
    /// than, equal to, or greater than the exact remainder, along with the low bits of the quotient
    /// as an `i64`. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_rem_float_and_quotient_bits_prec(a, b, 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_rem_float_and_quotient_bits_prec(
        x: Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        Self::rational_rem_float_and_quotient_bits_prec_round(x, y, prec, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Rational`] is taken by value and the
    /// [`Float`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_rem_float_and_quotient_bits_prec_val_ref(a, &b, 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_rem_float_and_quotient_bits_prec_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        Self::rational_rem_float_and_quotient_bits_prec_round_val_ref(x, y, prec, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Rational`] is taken by reference and the
    /// [`Float`] by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_rem_float_and_quotient_bits_prec_ref_val(&a, b, 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_rem_float_and_quotient_bits_prec_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        Self::rational_rem_float_and_quotient_bits_prec_round_ref_val(x, y, prec, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the specified precision. The [`Rational`] and the [`Float`] are both taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded remainder is
    /// less than, equal to, or greater than the exact remainder, along with the low bits of the
    /// quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_rem_float_and_quotient_bits_prec_ref_ref(&a, &b, 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_rem_float_and_quotient_bits_prec_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        Self::rational_rem_float_and_quotient_bits_prec_round_ref_ref(x, y, prec, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`] modulus's precision, with the specified rounding mode. The [`Rational`] and the
    /// [`Float`] are both taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded remainder is less than, equal to, or greater than the exact remainder, along with
    /// the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_rem_float_and_quotient_bits_round(a, b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_rem_float_and_quotient_bits_round(
        x: Rational,
        y: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = y.significant_bits();
        Self::rational_rem_float_and_quotient_bits_prec_round(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`] modulus's precision, with the specified rounding mode. The [`Rational`] is taken
    /// by value and the [`Float`] by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder,
    /// along with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_rem_float_and_quotient_bits_round_val_ref(a, &b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_rem_float_and_quotient_bits_round_val_ref(
        x: Rational,
        y: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = y.significant_bits();
        Self::rational_rem_float_and_quotient_bits_prec_round_val_ref(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`] modulus's precision, with the specified rounding mode. The [`Rational`] is taken
    /// by reference and the [`Float`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder,
    /// along with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_rem_float_and_quotient_bits_round_ref_val(&a, b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_rem_float_and_quotient_bits_round_ref_val(
        x: &Rational,
        y: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = y.significant_bits();
        Self::rational_rem_float_and_quotient_bits_prec_round_ref_val(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// [`Float`] modulus's precision, with the specified rounding mode. The [`Rational`] and the
    /// [`Float`] are both taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder, along
    /// with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_rem_float_and_quotient_bits_round_ref_ref(&a, &b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_rem_float_and_quotient_bits_round_ref_ref(
        x: &Rational,
        y: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = y.significant_bits();
        Self::rational_rem_float_and_quotient_bits_prec_round_ref_ref(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the [`Float`] modulus's precision. The [`Rational`] and the [`Float`] are
    /// both taken by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_rem_float_and_quotient_bits(a, b);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_rem_float_and_quotient_bits(x: Rational, y: Self) -> (Self, Ordering, i64) {
        Self::rational_rem_float_and_quotient_bits_round(x, y, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the [`Float`] modulus's precision. The [`Rational`] is taken by value and
    /// the [`Float`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_rem_float_and_quotient_bits_val_ref(a, &b);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_rem_float_and_quotient_bits_val_ref(
        x: Rational,
        y: &Self,
    ) -> (Self, Ordering, i64) {
        Self::rational_rem_float_and_quotient_bits_round_val_ref(x, y, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the [`Float`] modulus's precision. The [`Rational`] is taken by reference
    /// and the [`Float`] by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_rem_float_and_quotient_bits_ref_val(&a, b);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_rem_float_and_quotient_bits_ref_val(
        x: &Rational,
        y: Self,
    ) -> (Self, Ordering, i64) {
        Self::rational_rem_float_and_quotient_bits_round_ref_val(x, y, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward
    /// zero, as for the `%` operator on primitive floats and C's `fmod`, rounding the result to the
    /// nearest value of the [`Float`] modulus's precision. The [`Rational`] and the [`Float`] are
    /// both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// remainder is less than, equal to, or greater than the exact remainder, along with the low
    /// bits of the quotient as an `i64`. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_rem_float_and_quotient_bits_ref_ref(&a, &b);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_rem_float_and_quotient_bits_ref_ref(
        x: &Rational,
        y: &Self,
    ) -> (Self, Ordering, i64) {
        Self::rational_rem_float_and_quotient_bits_round_ref_ref(x, y, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] and the [`Float`] are both taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_ieee_remainder_float_prec_round(a, b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn rational_ieee_remainder_float_prec_round(
        x: Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_float_helper(&x, &y, true, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by value and the [`Float`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_ieee_remainder_float_prec_round_val_ref(a, &b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn rational_ieee_remainder_float_prec_round_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_float_helper(&x, y, true, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by reference and the [`Float`] by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_ieee_remainder_float_prec_round_ref_val(&a, b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn rational_ieee_remainder_float_prec_round_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_float_helper(x, &y, true, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] and the [`Float`] are both taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_ieee_remainder_float_prec_round_ref_ref(&a, &b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    pub fn rational_ieee_remainder_float_prec_round_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (r, o, _) = rational_rem_float_helper(x, y, true, false, prec, rm);
        (r, o)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] and
    /// the [`Float`] are both taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_ieee_remainder_float_prec(a, b, 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_prec(x: Rational, y: Self, prec: u64) -> (Self, Ordering) {
        Self::rational_ieee_remainder_float_prec_round(x, y, prec, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] is
    /// taken by value and the [`Float`] by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_ieee_remainder_float_prec_val_ref(a, &b, 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_prec_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        Self::rational_ieee_remainder_float_prec_round_val_ref(x, y, prec, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] is
    /// taken by reference and the [`Float`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_ieee_remainder_float_prec_ref_val(&a, b, 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_prec_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering) {
        Self::rational_ieee_remainder_float_prec_round_ref_val(x, y, prec, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] and
    /// the [`Float`] are both taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_ieee_remainder_float_prec_ref_ref(&a, &b, 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_prec_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering) {
        Self::rational_ieee_remainder_float_prec_round_ref_ref(x, y, prec, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`] modulus's precision, with the specified rounding mode.
    /// The [`Rational`] and the [`Float`] are both taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_ieee_remainder_float_round(a, b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_round(
        x: Rational,
        y: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = y.significant_bits();
        Self::rational_ieee_remainder_float_prec_round(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`] modulus's precision, with the specified rounding mode.
    /// The [`Rational`] is taken by value and the [`Float`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_ieee_remainder_float_round_val_ref(a, &b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_round_val_ref(
        x: Rational,
        y: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = y.significant_bits();
        Self::rational_ieee_remainder_float_prec_round_val_ref(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`] modulus's precision, with the specified rounding mode.
    /// The [`Rational`] is taken by reference and the [`Float`] by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_ieee_remainder_float_round_ref_val(&a, b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_round_ref_val(
        x: &Rational,
        y: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = y.significant_bits();
        Self::rational_ieee_remainder_float_prec_round_ref_val(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`] modulus's precision, with the specified rounding mode.
    /// The [`Rational`] and the [`Float`] are both taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o) = Float::rational_ieee_remainder_float_round_ref_ref(&a, &b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_round_ref_ref(
        x: &Rational,
        y: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = y.significant_bits();
        Self::rational_ieee_remainder_float_prec_round_ref_ref(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`] modulus's precision. The
    /// [`Rational`] and the [`Float`] are both taken by value.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let q = Rational::from_signeds(22, 7);
    /// let f = Float::from(3u32);
    /// let r = Float::rational_ieee_remainder_float(q, f);
    /// assert_eq!(r.to_string(), "0.12");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_ieee_remainder_float(x: Rational, y: Self) -> Self {
        Self::rational_ieee_remainder_float_round(x, y, Nearest).0
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`] modulus's precision. The
    /// [`Rational`] is taken by value and the [`Float`] by reference.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let q = Rational::from_signeds(22, 7);
    /// let f = Float::from(3u32);
    /// let r = Float::rational_ieee_remainder_float_val_ref(q, &f);
    /// assert_eq!(r.to_string(), "0.12");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_ieee_remainder_float_val_ref(x: Rational, y: &Self) -> Self {
        Self::rational_ieee_remainder_float_round_val_ref(x, y, Nearest).0
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`] modulus's precision. The
    /// [`Rational`] is taken by reference and the [`Float`] by value.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let q = Rational::from_signeds(22, 7);
    /// let f = Float::from(3u32);
    /// let r = Float::rational_ieee_remainder_float_ref_val(&q, f);
    /// assert_eq!(r.to_string(), "0.12");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_ieee_remainder_float_ref_val(x: &Rational, y: Self) -> Self {
        Self::rational_ieee_remainder_float_round_ref_val(x, y, Nearest).0
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`] modulus's precision. The
    /// [`Rational`] and the [`Float`] are both taken by reference.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let q = Rational::from_signeds(22, 7);
    /// let f = Float::from(3u32);
    /// let r = Float::rational_ieee_remainder_float_ref_ref(&q, &f);
    /// assert_eq!(r.to_string(), "0.12");
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_ref_ref(x: &Rational, y: &Self) -> Self {
        Self::rational_ieee_remainder_float_round_ref_ref(x, y, Nearest).0
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] and the [`Float`] are both taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) =
    ///     Float::rational_ieee_remainder_float_and_quotient_bits_prec_round(a, b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits_prec_round(
        x: Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_float_helper(&x, &y, true, true, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by value and the [`Float`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let f = Float::rational_ieee_remainder_float_and_quotient_bits_prec_round_val_ref;
    /// let (r, o, q) = f(a, &b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits_prec_round_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_float_helper(&x, y, true, true, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by reference and the [`Float`] by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let f = Float::rational_ieee_remainder_float_and_quotient_bits_prec_round_ref_val;
    /// let (r, o, q) = f(&a, b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits_prec_round_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_float_helper(x, &y, true, true, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] and the [`Float`] are both taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$. This is the same contract as the corresponding
    /// [`Float`]-[`Float`] functions.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the exact remainder is not representable
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let f = Float::rational_ieee_remainder_float_and_quotient_bits_prec_round_ref_ref;
    /// let (r, o, q) = f(&a, &b, 5, Floor);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits_prec_round_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        rational_rem_float_helper(x, y, true, true, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] and
    /// the [`Float`] are both taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded remainder is less than, equal to, or greater than the exact remainder, along
    /// with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_ieee_remainder_float_and_quotient_bits_prec(a, b, 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits_prec(
        x: Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        Self::rational_ieee_remainder_float_and_quotient_bits_prec_round(x, y, prec, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] is
    /// taken by value and the [`Float`] by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder,
    /// along with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) =
    ///     Float::rational_ieee_remainder_float_and_quotient_bits_prec_val_ref(a, &b, 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits_prec_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        Self::rational_ieee_remainder_float_and_quotient_bits_prec_round_val_ref(
            x, y, prec, Nearest,
        )
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] is
    /// taken by reference and the [`Float`] by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder,
    /// along with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) =
    ///     Float::rational_ieee_remainder_float_and_quotient_bits_prec_ref_val(&a, b, 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits_prec_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        Self::rational_ieee_remainder_float_and_quotient_bits_prec_round_ref_val(
            x, y, prec, Nearest,
        )
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] and
    /// the [`Float`] are both taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded remainder is less than, equal to, or greater than the exact remainder,
    /// along with the low bits of the quotient as an `i64`. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) =
    ///     Float::rational_ieee_remainder_float_and_quotient_bits_prec_ref_ref(&a, &b, 5);
    /// assert_eq!(r.to_string(), "0.141");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits_prec_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
    ) -> (Self, Ordering, i64) {
        Self::rational_ieee_remainder_float_and_quotient_bits_prec_round_ref_ref(
            x, y, prec, Nearest,
        )
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`] modulus's precision, with the specified rounding mode.
    /// The [`Rational`] and the [`Float`] are both taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_ieee_remainder_float_and_quotient_bits_round(a, b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits_round(
        x: Rational,
        y: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = y.significant_bits();
        Self::rational_ieee_remainder_float_and_quotient_bits_prec_round(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`] modulus's precision, with the specified rounding mode.
    /// The [`Rational`] is taken by value and the [`Float`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) =
    ///     Float::rational_ieee_remainder_float_and_quotient_bits_round_val_ref(a, &b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits_round_val_ref(
        x: Rational,
        y: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = y.significant_bits();
        Self::rational_ieee_remainder_float_and_quotient_bits_prec_round_val_ref(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`] modulus's precision, with the specified rounding mode.
    /// The [`Rational`] is taken by reference and the [`Float`] by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) =
    ///     Float::rational_ieee_remainder_float_and_quotient_bits_round_ref_val(&a, b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits_round_ref_val(
        x: &Rational,
        y: Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = y.significant_bits();
        Self::rational_ieee_remainder_float_and_quotient_bits_prec_round_ref_val(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the [`Float`] modulus's precision, with the specified rounding mode.
    /// The [`Rational`] and the [`Float`] are both taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p+1}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the exact remainder is not representable with the output
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) =
    ///     Float::rational_ieee_remainder_float_and_quotient_bits_round_ref_ref(&a, &b, Floor);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits_round_ref_ref(
        x: &Rational,
        y: &Self,
        rm: RoundingMode,
    ) -> (Self, Ordering, i64) {
        let prec = y.significant_bits();
        Self::rational_ieee_remainder_float_and_quotient_bits_prec_round_ref_ref(x, y, prec, rm)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`] modulus's precision. The
    /// [`Rational`] and the [`Float`] are both taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded remainder is less than, equal to, or greater than the exact
    /// remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_ieee_remainder_float_and_quotient_bits(a, b);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits(
        x: Rational,
        y: Self,
    ) -> (Self, Ordering, i64) {
        Self::rational_ieee_remainder_float_and_quotient_bits_round(x, y, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`] modulus's precision. The
    /// [`Rational`] is taken by value and the [`Float`] by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_ieee_remainder_float_and_quotient_bits_val_ref(a, &b);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits_val_ref(
        x: Rational,
        y: &Self,
    ) -> (Self, Ordering, i64) {
        Self::rational_ieee_remainder_float_and_quotient_bits_round_val_ref(x, y, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`] modulus's precision. The
    /// [`Rational`] is taken by reference and the [`Float`] by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_ieee_remainder_float_and_quotient_bits_ref_val(&a, b);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits_ref_val(
        x: &Rational,
        y: Self,
    ) -> (Self, Ordering, i64) {
        Self::rational_ieee_remainder_float_and_quotient_bits_round_ref_val(x, y, Nearest)
    }

    /// Computes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded to the
    /// nearest integer, ties to even; this is the IEEE 754 `remainder` operation, C's `remainder`,
    /// rounding the result to the nearest value of the [`Float`] modulus's precision. The
    /// [`Rational`] and the [`Float`] are both taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded remainder is less than, equal to, or greater than
    /// the exact remainder, along with the low bits of the quotient as an `i64`. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The [`Rational`] dividend is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// The returned `i64` agrees with the exact quotient $q$ in its low 63 bits and has $q$'s sign:
    /// it equals $\pm(|q|\bmod 2^{63})$.
    ///
    /// $$
    /// f(x,y,p) = x - y\operatorname{roundeven}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or an input is special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2
    ///   |x-y\operatorname{roundeven}(x/y)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN},p)=f(x,\pm0.0,p)=\text{NaN}$
    /// - $f(x,\pm\infty,p)=x$
    /// - $f(0,y,p)=0.0$ if $y$ is not `NaN` and $y\neq 0$ (a zero [`Rational`] has no sign, so the
    ///   result is a positive zero)
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    /// - The quotient bits are 0 in all of the above special cases.
    ///
    /// The remainder never overflows, but it can underflow, since its granularity may lie far below
    /// the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(),
    /// y.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let a = Rational::from_signeds(22, 7);
    /// let b = Float::from(3u32);
    /// let (r, o, q) = Float::rational_ieee_remainder_float_and_quotient_bits_ref_ref(&a, &b);
    /// assert_eq!(r.to_string(), "0.12");
    /// assert_eq!(o, Less);
    /// assert_eq!(q, 1);
    /// ```
    #[inline]
    pub fn rational_ieee_remainder_float_and_quotient_bits_ref_ref(
        x: &Rational,
        y: &Self,
    ) -> (Self, Ordering, i64) {
        Self::rational_ieee_remainder_float_and_quotient_bits_round_ref_ref(x, y, Nearest)
    }
}

impl Rem<Self> for Float {
    type Output = Self;

    /// Takes the remainder of two [`Float`]s, with the quotient rounded toward zero (as for the `%`
    /// operator on primitive floats and C's `fmod`), taking both by value. The result is rounded to
    /// the nearest value of the maximum of the precisions of the inputs.
    ///
    /// $$
    /// x\%y = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$,
    ///   where $p$ is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::rem_prec`] instead. If
    /// you want to use a rounding mode other than `Nearest`, consider using [`Float::rem_round`]
    /// instead. If you want both, consider using [`Float::rem_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!((Float::from(10u32) % Float::from(7u32)).to_string(), "3.0");
    /// assert_eq!(
    ///     (-Float::from(10u32) % Float::from(7u32)).to_string(),
    ///     "-3.0"
    /// );
    /// ```
    #[inline]
    fn rem(self, other: Self) -> Self {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_prec_round(other, prec, Nearest).0
    }
}

impl Rem<&Self> for Float {
    type Output = Self;

    /// Takes the remainder of two [`Float`]s, with the quotient rounded toward zero (as for the `%`
    /// operator on primitive floats and C's `fmod`), taking the first by value and the second by
    /// reference. The result is rounded to the nearest value of the maximum of the precisions of
    /// the inputs.
    ///
    /// $$
    /// x\%y = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$,
    ///   where $p$ is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::rem_prec`] instead. If
    /// you want to use a rounding mode other than `Nearest`, consider using [`Float::rem_round`]
    /// instead. If you want both, consider using [`Float::rem_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!((Float::from(10u32) % &Float::from(7u32)).to_string(), "3.0");
    /// assert_eq!(
    ///     (-Float::from(10u32) % &Float::from(7u32)).to_string(),
    ///     "-3.0"
    /// );
    /// ```
    #[inline]
    fn rem(self, other: &Self) -> Self {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_prec_round_val_ref(other, prec, Nearest).0
    }
}

impl Rem<Float> for &Float {
    type Output = Float;

    /// Takes the remainder of two [`Float`]s, with the quotient rounded toward zero (as for the `%`
    /// operator on primitive floats and C's `fmod`), taking the first by reference and the second
    /// by value. The result is rounded to the nearest value of the maximum of the precisions of the
    /// inputs.
    ///
    /// $$
    /// x\%y = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$,
    ///   where $p$ is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::rem_prec`] instead. If
    /// you want to use a rounding mode other than `Nearest`, consider using [`Float::rem_round`]
    /// instead. If you want both, consider using [`Float::rem_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::from(10u32) % Float::from(7u32)).to_string(), "3.0");
    /// assert_eq!(
    ///     (-Float::from(10u32) % Float::from(7u32)).to_string(),
    ///     "-3.0"
    /// );
    /// ```
    #[inline]
    fn rem(self, other: Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_prec_round_ref_val(other, prec, Nearest).0
    }
}

impl Rem<&Float> for &Float {
    type Output = Float;

    /// Takes the remainder of two [`Float`]s, with the quotient rounded toward zero (as for the `%`
    /// operator on primitive floats and C's `fmod`), taking both by reference. The result is
    /// rounded to the nearest value of the maximum of the precisions of the inputs.
    ///
    /// $$
    /// x\%y = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$,
    ///   where $p$ is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::rem_prec`] instead. If
    /// you want to use a rounding mode other than `Nearest`, consider using [`Float::rem_round`]
    /// instead. If you want both, consider using [`Float::rem_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     (&Float::from(10u32) % &Float::from(7u32)).to_string(),
    ///     "3.0"
    /// );
    /// assert_eq!(
    ///     (-Float::from(10u32) % &Float::from(7u32)).to_string(),
    ///     "-3.0"
    /// );
    /// ```
    #[inline]
    fn rem(self, other: &Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_prec_round_ref_ref(other, prec, Nearest).0
    }
}

impl RemAssign<Self> for Float {
    /// Takes the remainder of two [`Float`]s in place, with the quotient rounded toward zero (as
    /// for the `%` operator on primitive floats and C's `fmod`); the [`Float`] on the right-hand
    /// side is taken by value. The result is rounded to the nearest value of the maximum of the
    /// precisions of the inputs.
    ///
    /// $$
    /// x\%y = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$,
    ///   where $p$ is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::rem_prec`] instead. If
    /// you want to use a rounding mode other than `Nearest`, consider using [`Float::rem_round`]
    /// instead. If you want both, consider using [`Float::rem_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(10u32);
    /// x %= Float::from(7u32);
    /// assert_eq!(x.to_string(), "3.0");
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: Self) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_prec_round_assign(other, prec, Nearest);
    }
}

impl RemAssign<&Self> for Float {
    /// Takes the remainder of two [`Float`]s in place, with the quotient rounded toward zero (as
    /// for the `%` operator on primitive floats and C's `fmod`); the [`Float`] on the right-hand
    /// side is taken by reference. The result is rounded to the nearest value of the maximum of the
    /// precisions of the inputs.
    ///
    /// $$
    /// x\%y = x - y\operatorname{trunc}(x/y) + \varepsilon.
    /// $$
    /// - If the exact remainder is zero or the inputs are special, $\varepsilon$ is 0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |x-y\operatorname{trunc}(x/y)|\rfloor-p}$,
    ///   where $p$ is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=f(\pm\infty,y,p)=f(x,\pm0.0,p) = \text{NaN}$
    /// - $f(x,\pm\infty,p)=x$ if $x$ is finite
    /// - $f(\pm0.0,y,p)=\pm0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - If the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// The remainder never overflows, since it is smaller than $y$ in magnitude, but it can
    /// underflow, since its granularity may lie far below the minimum positive [`Float`]:
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::rem_prec`] instead. If
    /// you want to use a rounding mode other than `Nearest`, consider using [`Float::rem_round`]
    /// instead. If you want both, consider using [`Float::rem_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(10u32);
    /// x %= &Float::from(7u32);
    /// assert_eq!(x.to_string(), "3.0");
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: &Self) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.rem_prec_round_assign_ref(other, prec, Nearest);
    }
}

impl Rem<Rational> for Float {
    type Output = Self;

    /// Takes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward zero
    /// (as for the `%` operator on primitive floats and C's `fmod`), taking both by value. The
    /// result is rounded to the nearest value of the [`Float`]'s precision.
    ///
    /// The [`Rational`] operand is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(\pm\infty,y)=f(x,0)=\text{NaN}$
    /// - $f(\pm0.0,y)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum of the operands'
    /// complexities.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let r = Float::from(10u32) % Rational::from_signeds(22, 7);
    /// assert_eq!(r.to_string(), "0.62");
    /// ```
    #[inline]
    fn rem(self, other: Rational) -> Self {
        let prec = self.significant_bits();
        self.rem_rational_prec_round(other, prec, Nearest).0
    }
}

impl Rem<&Rational> for Float {
    type Output = Self;

    /// Takes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward zero
    /// (as for the `%` operator on primitive floats and C's `fmod`), taking the [`Float`] by value
    /// and the [`Rational`] by reference. The result is rounded to the nearest value of the
    /// [`Float`]'s precision.
    ///
    /// The [`Rational`] operand is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(\pm\infty,y)=f(x,0)=\text{NaN}$
    /// - $f(\pm0.0,y)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum of the operands'
    /// complexities.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let r = Float::from(10u32) % &Rational::from_signeds(22, 7);
    /// assert_eq!(r.to_string(), "0.62");
    /// ```
    #[inline]
    fn rem(self, other: &Rational) -> Self {
        let prec = self.significant_bits();
        self.rem_rational_prec_round_val_ref(other, prec, Nearest).0
    }
}

impl Rem<Rational> for &Float {
    type Output = Float;

    /// Takes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward zero
    /// (as for the `%` operator on primitive floats and C's `fmod`), taking the [`Float`] by
    /// reference and the [`Rational`] by value. The result is rounded to the nearest value of the
    /// [`Float`]'s precision.
    ///
    /// The [`Rational`] operand is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(\pm\infty,y)=f(x,0)=\text{NaN}$
    /// - $f(\pm0.0,y)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum of the operands'
    /// complexities.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let r = &Float::from(10u32) % Rational::from_signeds(22, 7);
    /// assert_eq!(r.to_string(), "0.62");
    /// ```
    #[inline]
    fn rem(self, other: Rational) -> Float {
        let prec = self.significant_bits();
        self.rem_rational_prec_round_ref_val(other, prec, Nearest).0
    }
}

impl Rem<&Rational> for &Float {
    type Output = Float;

    /// Takes the remainder of a [`Float`] by a [`Rational`], with the quotient rounded toward zero
    /// (as for the `%` operator on primitive floats and C's `fmod`), taking both by reference. The
    /// result is rounded to the nearest value of the [`Float`]'s precision.
    ///
    /// The [`Rational`] operand is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(\pm\infty,y)=f(x,0)=\text{NaN}$
    /// - $f(\pm0.0,y)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum of the operands'
    /// complexities.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let r = &Float::from(10u32) % &Rational::from_signeds(22, 7);
    /// assert_eq!(r.to_string(), "0.62");
    /// ```
    #[inline]
    fn rem(self, other: &Rational) -> Float {
        let prec = self.significant_bits();
        self.rem_rational_prec_round_ref_ref(other, prec, Nearest).0
    }
}

impl RemAssign<Rational> for Float {
    /// Takes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// toward zero (as for the `%` operator on primitive floats and C's `fmod`); the [`Rational`]
    /// is taken by value. The result is rounded to the nearest value of the [`Float`]'s precision.
    ///
    /// The [`Rational`] operand is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(\pm\infty,y)=f(x,0)=\text{NaN}$
    /// - $f(\pm0.0,y)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum of the operands'
    /// complexities.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// x %= Rational::from_signeds(22, 7);
    /// assert_eq!(x.to_string(), "0.62");
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: Rational) {
        let prec = self.significant_bits();
        self.rem_rational_prec_round_assign(other, prec, Nearest);
    }
}

impl RemAssign<&Rational> for Float {
    /// Takes the remainder of a [`Float`] by a [`Rational`] in place, with the quotient rounded
    /// toward zero (as for the `%` operator on primitive floats and C's `fmod`); the [`Rational`]
    /// is taken by reference. The result is rounded to the nearest value of the [`Float`]'s
    /// precision.
    ///
    /// The [`Rational`] operand is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// Special cases:
    /// - $f(\text{NaN},y)=f(\pm\infty,y)=f(x,0)=\text{NaN}$
    /// - $f(\pm0.0,y)=\pm0.0$ if $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum of the operands'
    /// complexities.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Float::from(10u32);
    /// x %= &Rational::from_signeds(22, 7);
    /// assert_eq!(x.to_string(), "0.62");
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: &Rational) {
        let prec = self.significant_bits();
        self.rem_rational_prec_round_assign_ref(other, prec, Nearest);
    }
}

impl Rem<Float> for Rational {
    type Output = Float;

    /// Takes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward zero
    /// (as for the `%` operator on primitive floats and C's `fmod`), taking both by value. The
    /// result is rounded to the nearest value of the [`Float`]'s precision.
    ///
    /// The [`Rational`] operand is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=f(x,\pm0.0)=\text{NaN}$
    /// - $f(x,\pm\infty)=x$
    /// - $f(0,y)=0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum of the operands'
    /// complexities.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let r = Rational::from_signeds(22, 7) % Float::from(3u32);
    /// assert_eq!(r.to_string(), "0.12");
    /// ```
    #[inline]
    fn rem(self, other: Float) -> Float {
        let prec = other.significant_bits();
        Float::rational_rem_float_prec_round(self, other, prec, Nearest).0
    }
}

impl Rem<&Float> for Rational {
    type Output = Float;

    /// Takes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward zero
    /// (as for the `%` operator on primitive floats and C's `fmod`), taking the [`Rational`] by
    /// value and the [`Float`] by reference. The result is rounded to the nearest value of the
    /// [`Float`]'s precision.
    ///
    /// The [`Rational`] operand is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=f(x,\pm0.0)=\text{NaN}$
    /// - $f(x,\pm\infty)=x$
    /// - $f(0,y)=0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum of the operands'
    /// complexities.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let r = Rational::from_signeds(22, 7) % &Float::from(3u32);
    /// assert_eq!(r.to_string(), "0.12");
    /// ```
    #[inline]
    fn rem(self, other: &Float) -> Float {
        let prec = other.significant_bits();
        Float::rational_rem_float_prec_round_val_ref(self, other, prec, Nearest).0
    }
}

impl Rem<Float> for &Rational {
    type Output = Float;

    /// Takes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward zero
    /// (as for the `%` operator on primitive floats and C's `fmod`), taking the [`Rational`] by
    /// reference and the [`Float`] by value. The result is rounded to the nearest value of the
    /// [`Float`]'s precision.
    ///
    /// The [`Rational`] operand is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=f(x,\pm0.0)=\text{NaN}$
    /// - $f(x,\pm\infty)=x$
    /// - $f(0,y)=0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum of the operands'
    /// complexities.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let r = &Rational::from_signeds(22, 7) % Float::from(3u32);
    /// assert_eq!(r.to_string(), "0.12");
    /// ```
    #[inline]
    fn rem(self, other: Float) -> Float {
        let prec = other.significant_bits();
        Float::rational_rem_float_prec_round_ref_val(self, other, prec, Nearest).0
    }
}

impl Rem<&Float> for &Rational {
    type Output = Float;

    /// Takes the remainder of a [`Rational`] by a [`Float`], with the quotient rounded toward zero
    /// (as for the `%` operator on primitive floats and C's `fmod`), taking both by reference. The
    /// result is rounded to the nearest value of the [`Float`]'s precision.
    ///
    /// The [`Rational`] operand is used exactly, so the result is the correctly-rounded remainder
    /// of the exact input values.
    ///
    /// Special cases:
    /// - $f(x,\text{NaN})=f(x,\pm0.0)=\text{NaN}$
    /// - $f(x,\pm\infty)=x$
    /// - $f(0,y)=0.0$ if $y$ is not `NaN` and $y\neq 0$
    /// - Otherwise, if the exact remainder is zero, a zero with the sign of $x$ is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum of the operands'
    /// complexities.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// let r = &Rational::from_signeds(22, 7) % &Float::from(3u32);
    /// assert_eq!(r.to_string(), "0.12");
    /// ```
    #[inline]
    fn rem(self, other: &Float) -> Float {
        let prec = other.significant_bits();
        Float::rational_rem_float_prec_round_ref_ref(self, other, prec, Nearest).0
    }
}

/// Computes the remainder of two primitive floats, with the quotient rounded toward zero, using
/// emulated [`Float`] arithmetic.
///
/// The floating-point remainder of two values of the same format is always exactly representable,
/// so this function returns the same values as the `%` operator on primitive floats; it serves as a
/// reference implementation. NaN, infinite `x`, or zero `y` gives NaN; a zero remainder has the
/// sign of `x`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::rem::primitive_float_rem;
///
/// assert_eq!(NiceFloat(primitive_float_rem(10.0, 7.0)), NiceFloat(3.0));
/// assert_eq!(NiceFloat(primitive_float_rem(10.5, 3.25)), NiceFloat(0.75));
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_rem<T: PrimitiveFloat>(x: T, y: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_float_to_float_fn(Float::rem_prec, x, y)
}

/// Computes the IEEE 754 `remainder` of two primitive floats, with the quotient rounded to the
/// nearest integer (ties to even), using emulated [`Float`] arithmetic.
///
/// Like the truncated-quotient remainder, this value is always exactly representable. NaN, infinite
/// `x`, or zero `y` gives NaN; a zero remainder has the sign of `x`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::rem::primitive_float_ieee_remainder;
///
/// assert_eq!(
///     NiceFloat(primitive_float_ieee_remainder(14.0, 3.0)),
///     NiceFloat(-1.0)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_ieee_remainder<T: PrimitiveFloat>(x: T, y: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_float_to_float_fn(Float::ieee_remainder_prec, x, y)
}

/// Computes the remainder of a primitive float by a [`Rational`], with the quotient rounded toward
/// zero, correctly rounding the result to the nearest value.
///
/// The [`Rational`] modulus is used exactly. A remainder is unusually sensitive to its modulus —
/// perturbing it by $\varepsilon$ moves the result by up to the quotient times $\varepsilon$ — so
/// no primitive-float approximation of the modulus could produce these values. NaN or infinite `x`,
/// or zero `y`, gives NaN.
///
/// # Worst-case complexity
/// $T(n) = O(n \log n \log\log n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `y.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::rem::primitive_float_rem_rational;
/// use malachite_q::Rational;
///
/// // 10 mod 22/7 = 4/7
/// assert_eq!(
///     NiceFloat(primitive_float_rem_rational(
///         10.0,
///         &Rational::from_signeds(22, 7)
///     )),
///     NiceFloat(0.5714285714285714)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_rem_rational<T: PrimitiveFloat>(x: T, y: &Rational) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| Float::rem_rational_prec_val_ref(x, y, prec), x)
}

/// Computes the IEEE 754 `remainder` of a primitive float by a [`Rational`], with the quotient
/// rounded to the nearest integer (ties to even), correctly rounding the result to the nearest
/// value.
///
/// The [`Rational`] modulus is used exactly; see [`primitive_float_rem_rational`] for why this
/// matters.
///
/// # Worst-case complexity
/// $T(n) = O(n \log n \log\log n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `y.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::rem::primitive_float_ieee_remainder_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_ieee_remainder_rational(
///         10.0,
///         &Rational::from_signeds(22, 7)
///     )),
///     NiceFloat(0.5714285714285714)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_ieee_remainder_rational<T: PrimitiveFloat>(x: T, y: &Rational) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_to_float_fn(
        |x, prec| Float::ieee_remainder_rational_prec_val_ref(x, y, prec),
        x,
    )
}

/// Computes the remainder of a [`Rational`] by a primitive float, with the quotient rounded toward
/// zero, correctly rounding the result to the nearest value.
///
/// The [`Rational`] dividend is used exactly. NaN or zero `y` gives NaN; an infinite `y` returns
/// the dividend, rounded.
///
/// # Worst-case complexity
/// $T(n) = O(n \log n \log\log n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::rem::primitive_float_rational_rem_float;
/// use malachite_q::Rational;
///
/// // 22/7 mod 3 = 1/7
/// assert_eq!(
///     NiceFloat(primitive_float_rational_rem_float(
///         &Rational::from_signeds(22, 7),
///         3.0
///     )),
///     NiceFloat(0.14285714285714285)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_rational_rem_float<T: PrimitiveFloat>(x: &Rational, y: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_to_float_fn(
        |y, prec| Float::rational_rem_float_prec_ref_val(x, y, prec),
        y,
    )
}

/// Computes the IEEE 754 `remainder` of a [`Rational`] by a primitive float, with the quotient
/// rounded to the nearest integer (ties to even), correctly rounding the result to the nearest
/// value.
///
/// The [`Rational`] dividend is used exactly.
///
/// # Worst-case complexity
/// $T(n) = O(n \log n \log\log n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::rem::primitive_float_rational_ieee_remainder_float;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_rational_ieee_remainder_float(
///         &Rational::from_signeds(22, 7),
///         3.0
///     )),
///     NiceFloat(0.14285714285714285)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_rational_ieee_remainder_float<T: PrimitiveFloat>(x: &Rational, y: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_to_float_fn(
        |y, prec| Float::rational_ieee_remainder_float_prec_ref_val(x, y, prec),
        y,
    )
}

/// Computes the remainder of a primitive float by a `u64`, with the quotient rounded toward zero,
/// correctly rounding the result to the nearest value.
///
/// The modulus is used exactly, even when it is not representable in the primitive float type (any
/// `u64` above $2^{T::MANTISSA\\_WIDTH+1}$ has neighbors that round to the same float). A zero
/// modulus gives NaN, matching `mpfr_fmod_ui`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::rem::primitive_float_rem_unsigned;
///
/// assert_eq!(
///     NiceFloat(primitive_float_rem_unsigned(10.5, 3)),
///     NiceFloat(1.5)
/// );
/// // u64::MAX is not exactly representable as an f64, but the remainder is taken exactly
/// assert_eq!(
///     NiceFloat(primitive_float_rem_unsigned(1.0e30, u64::MAX)),
///     NiceFloat(5.076964209140211e18)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_rem_unsigned<T: PrimitiveFloat>(x: T, y: u64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| x.rem_unsigned_prec(y, prec), x)
}

/// Computes the remainder of two primitive floats along with the low bits of the quotient, with the
/// quotient rounded toward zero, using emulated [`Float`] arithmetic.
///
/// This is the analog of C's `fmodquo`-style functions: the `i64` agrees with the exact quotient
/// $q$ in its low 63 bits and has $q$'s sign.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::rem::primitive_float_rem_and_quotient_bits;
///
/// let (r, q) = primitive_float_rem_and_quotient_bits(100.0, 7.0);
/// assert_eq!(NiceFloat(r), NiceFloat(2.0));
/// assert_eq!(q, 14);
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_rem_and_quotient_bits<T: PrimitiveFloat>(x: T, y: T) -> (T, i64)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_float_to_float_and_i64_fn(Float::rem_and_quotient_bits_prec, x, y)
}

/// Computes the IEEE 754 `remainder` of two primitive floats along with the low bits of the
/// quotient, with the quotient rounded to the nearest integer (ties to even), using emulated
/// [`Float`] arithmetic.
///
/// This is the analog of C's `remquo`: the `i64` agrees with the exact quotient $q$ in its low 63
/// bits and has $q$'s sign.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::rem::primitive_float_ieee_remainder_and_quotient_bits;
///
/// let (r, q) = primitive_float_ieee_remainder_and_quotient_bits(14.0, 3.0);
/// assert_eq!(NiceFloat(r), NiceFloat(-1.0));
/// assert_eq!(q, 5);
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_ieee_remainder_and_quotient_bits<T: PrimitiveFloat>(x: T, y: T) -> (T, i64)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_float_to_float_and_i64_fn(Float::ieee_remainder_and_quotient_bits_prec, x, y)
}

/// Computes the remainder of a primitive float by a [`Rational`] along with the low bits of the
/// quotient, with the quotient rounded toward zero, correctly rounding the remainder to the nearest
/// value.
///
/// The [`Rational`] modulus is used exactly.
///
/// # Worst-case complexity
/// $T(n) = O(n \log n \log\log n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `y.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::rem::primitive_float_rem_rational_and_quotient_bits;
/// use malachite_q::Rational;
///
/// let (r, q) =
///     primitive_float_rem_rational_and_quotient_bits(10.0, &Rational::from_signeds(22, 7));
/// assert_eq!(NiceFloat(r), NiceFloat(0.5714285714285714));
/// assert_eq!(q, 3);
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_rem_rational_and_quotient_bits<T: PrimitiveFloat>(
    x: T,
    y: &Rational,
) -> (T, i64)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_to_float_and_i64_fn(
        |x, prec| Float::rem_rational_and_quotient_bits_prec_val_ref(x, y, prec),
        x,
    )
}

/// Computes the IEEE 754 `remainder` of a primitive float by a [`Rational`] along with the low bits
/// of the quotient, with the quotient rounded to the nearest integer (ties to even), correctly
/// rounding the remainder to the nearest value.
///
/// The [`Rational`] modulus is used exactly. This is the natural tool for additive argument
/// reduction against a non-dyadic constant: reducing against a [`Rational`] approximation of, say,
/// $\pi/2$ yields the reduced argument and the quadrant bits in one call.
///
/// # Worst-case complexity
/// $T(n) = O(n \log n \log\log n)$
///
/// $M(n) = O(n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `y.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::rem::*;
/// use malachite_q::Rational;
///
/// let (r, q) = primitive_float_ieee_remainder_rational_and_quotient_bits(
///     10.0,
///     &Rational::from_signeds(22, 7),
/// );
/// assert_eq!(NiceFloat(r), NiceFloat(0.5714285714285714));
/// assert_eq!(q, 3);
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_ieee_remainder_rational_and_quotient_bits<T: PrimitiveFloat>(
    x: T,
    y: &Rational,
) -> (T, i64)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_to_float_and_i64_fn(
        |x, prec| Float::ieee_remainder_rational_and_quotient_bits_prec_val_ref(x, y, prec),
        x,
    )
}
