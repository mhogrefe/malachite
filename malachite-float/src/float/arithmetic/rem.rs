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
use crate::{Float, float_either_infinity, float_either_zero, float_nan, significand_bits};
use core::cmp::Ordering::{self, *};
use core::cmp::{max, min};
use core::ops::{Rem, RemAssign};
use malachite_base::num::arithmetic::traits::{
    DivMod, ModPow, ModPowerOf2, NegAssign, Parity, PowerOf2,
};
use malachite_base::num::basic::traits::{NegativeZero, Two, Zero as ZeroTrait};
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
        ) => {
            let signx = *x_sign;
            // To get rid of sign problems, we compute the result separately: quo(-x,-y) = quo(x,y),
            // rem(-x,-y) = -rem(x,y) quo(-x,y) = -quo(x,y), rem(-x,y) = -rem(x,y) thus quo =
            // sign(x/y)*quo(|x|,|y|), rem = sign(x)*rem(|x|,|y|)
            let sign = x_sign == y_sign;
            // x = mx*2^ex, y = my*2^ey
            let ex = i64::from(*x_exponent) - i64::exact_from(significand_bits(x_significand));
            let mut ey = i64::from(*y_exponent) - i64::exact_from(significand_bits(y_significand));
            // mx is only ever read, so it is borrowed rather than cloned
            let mx = x_significand;
            let mut q_is_odd = false;
            let mut quo = 0i64;
            let mut tiny = false;
            // Divide my by 2^k if possible to make operations mod my easier. Since my comes from a
            // regular float, due to the constraints on the exponent and the precision, there can be
            // no integer overflow below.
            let k = y_significand.trailing_zeros().unwrap();
            ey += i64::exact_from(k);
            let mut my = y_significand >> k;
            let mut r;
            if ex <= ey {
                // q = x/y = mx/(my*2^(ey-ex))
                //
                // First detect cases where q = 0, to avoid creating a huge number my*2^(ey-ex): if
                // sx = mx.significant_bits() and sy = my.significant_bits(), we have x < 2^(ex +
                // sx) and y >= 2^(ey + sy - 1), thus if ex + sx <= ey + sy - 1 the quotient is 0.
                let q;
                if ex + i64::exact_from(mx.significant_bits())
                    < ey + i64::exact_from(my.significant_bits())
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
                    // for the quotient-bits variants, to get the low 63 more bits of the quotient,
                    // we first compute R = X mod Y*2^63, where X and Y are defined below. Then the
                    // low 63 bits of the quotient are floor(R/Y).
                    my <<= 63u64;
                } else if nearest_quotient {
                    // remainder case: let X = mx*2^(ex-ey) and Y = my. Then both X and Y are
                    // integers. Assume X = R mod Y; then x = X*2^ey = R*2^ey mod (Y*2^ey=y). To be
                    // able to perform the rounding, we need the least significant bit of the
                    // quotient, i.e., one more bit in the remainder, which is obtained by dividing
                    // by 2Y.
                    my <<= 1u64;
                }
                let d = u64::exact_from(ex - ey);
                r = if d > 3 * my.significant_bits() {
                    // 2^(ex-ey) mod my. When 2^(ex-ey) is at least my^3, modular exponentiation is
                    // faster than the exact power and a single reduction.
                    (&(Natural::TWO % &my)).mod_pow(Natural::from(d), &my)
                } else {
                    Natural::power_of_2(d)
                };
                r = r * mx % &my;
                if want_quo {
                    // now 0 <= r < 2^63*Y
                    my >>= 63u64;
                    let q;
                    (q, r) = r.div_mod(&my);
                    // oldr = q*my + newr
                    quo = i64::exact_from(&q);
                    q_is_odd = quo.odd();
                } else if nearest_quotient {
                    // now 0 <= r < 2Y in the remainder case
                    my >>= 1u64;
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
                    // determine whether 2r is greater than my; both are nonnegative, so plain
                    // comparison mirrors mpz_cmpabs
                    let r2 = &r << 1u64;
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
                            // The C code increments a long here, which can overflow; we keep the
                            // documented low-63-bits contract instead.
                            quo = quo.wrapping_add(1) & i64::MAX;
                        }
                    }
                }
                // take into account sign of x
                if !signx {
                    r.neg_assign();
                }
                // The result is r*2^sh. Rounding r to prec bits gives an exponent of e or e + 1 (on
                // a rounding carry), so when e is strictly inside the representable range no
                // underflow or overflow is possible: round r once and shift exactly, avoiding the
                // Rational construction, whose denominator has |sh| bits when sh is negative. At
                // the range edges, fall back to the Rational conversion, whose single rounding
                // handles underflow. (Both paths are a single rounding of r*2^sh, so they agree
                // wherever both apply.)
                let sh = min(ex, ey);
                let e = i64::exact_from(r.significant_bits()) + sh;
                let (rem, o) = if e > Float::MIN_EXPONENT_I64 && e < Float::MAX_EXPONENT_I64 {
                    let (rem, o) = Float::from_integer_prec_round(r, prec, rm);
                    (rem << sh, o)
                } else {
                    Float::from_rational_prec_round(Rational::from(r) << sh, prec, rm)
                };
                (rem, o, if sign { quo } else { quo.wrapping_neg() })
            }
        }
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
