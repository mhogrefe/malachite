// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::add::significand_prec;
use crate::big_exponent_float::bracket::{
    Bracket, ZIV_GUARD_BITS, floor_bracket, rational_exponent_bound, rounding_possible,
    ziv_loop_certified,
};
use crate::big_exponent_float::exponential::exp::power_of_2_integer;
use core::cmp::Ordering::{self, *};
use core::cmp::{max, min};
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{Abs, FloorSqrt, Parity, PowerOf2};
use malachite_base::num::basic::traits::{NaN as NaNTrait, One, Zero as ZeroTrait};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// The number of terms after which the alternating series gives up. The series is only tried when
// fewer terms are expected: at most about 18 for a small argument and 60 for a large one.
//
// This is equivalent to `gdSeriesTerms` from `AzFloat/GdBracket.lean`, Azurite, where it is 40.
const GD_SERIES_TERMS: u64 = 64;

// An enclosure of pi / 2 at precision `prec`.
//
// This is equivalent to half of `piBracket` from `AzFloat/PiBracket.lean`, Azurite, with the
// correctly rounded constant in place of Azurite's bracket.
fn pi_over_2_bracket(prec: u64) -> Bracket {
    floor_bracket(BigExponentFloat::pi_over_2_prec_round(prec, Floor).0, prec)
}

// The product of two enclosures of nonnegative numbers, rounded outward at precision `prec`.
//
// This is equivalent to `Encl.mul` from `AzFloat/Interval.lean`, Azurite.
fn encl_mul(prec: u64, a: &Bracket, b: &Bracket) -> Bracket {
    (
        a.0.mul_prec_round_ref_ref(&b.0, prec, Floor).0,
        a.1.mul_prec_round_ref_ref(&b.1, prec, Ceiling).0,
    )
}

// An enclosure of a nonnegative number divided by a positive integer, rounded outward at precision
// `prec`.
//
// This is equivalent to `Encl.div` by `Encl.nat` from `AzFloat/Interval.lean`, Azurite.
fn encl_div_unsigned(prec: u64, a: &Bracket, n: u64) -> Bracket {
    let n = BigExponentFloat::from(n);
    (
        a.0.div_prec_round_ref_ref(&n, prec, Floor).0,
        a.1.div_prec_round_ref_val(n, prec, Ceiling).0,
    )
}

// The sum of two enclosures, rounded outward at precision `prec`.
//
// This is equivalent to `Encl.add` from `AzFloat/Interval.lean`, Azurite.
fn encl_add(prec: u64, a: &Bracket, b: &Bracket) -> Bracket {
    (
        a.0.add_prec_round_ref_ref(&b.0, prec, Floor).0,
        a.1.add_prec_round_ref_ref(&b.1, prec, Ceiling).0,
    )
}

// The difference of two enclosures, rounded outward at precision `prec`.
//
// This is equivalent to `Encl.sub` from `AzFloat/Interval.lean`, Azurite.
fn encl_sub(prec: u64, a: &Bracket, b: &Bracket) -> Bracket {
    (
        a.0.sub_prec_round_ref_ref(&b.1, prec, Floor).0,
        a.1.sub_prec_round_ref_ref(&b.0, prec, Ceiling).0,
    )
}

// An enclosure of arctan z for an enclosure `z` of a number in [0, 1], by the alternating series z
// - z^3 / 3 + z^5 / 5 - ... at precision `prec`, stopped at terms below 2^(-k - 2) or after
// `GD_SERIES_TERMS` terms. The partial sums are alternately below and above the value, so the sum
// of any number of terms and the next term enclose it: the result is valid however many terms are
// taken, and narrow only when the series has converged.
//
// This is equivalent to `atanAlt`, `atanAltLoop`, and `atanAltClose` from `AzFloat/GdBracket.lean`,
// Azurite.
fn atan_alt(z: &Bracket, k: u64, prec: u64) -> Bracket {
    let z_squared = encl_mul(prec, z, z);
    let tau = power_of_2_integer(-Integer::from(k + 2));
    // `power` encloses z^(2i + 1), and `acc` the sum of the first i terms.
    let mut power = z.clone();
    let mut acc = (BigExponentFloat::ZERO, BigExponentFloat::ZERO);
    let mut i = 0;
    loop {
        let term = encl_div_unsigned(prec, &power, (i << 1) | 1);
        if i == GD_SERIES_TERMS || term.1 < tau {
            if i == GD_SERIES_TERMS {
                // The series is only tried when fewer terms than this are expected.
                fail_on_untested_path("atan_alt, out of terms");
            }
            return if i.even() {
                let hi = acc.1.add_prec_round(term.1, prec, Ceiling).0;
                (acc.0, hi)
            } else {
                let lo = acc.0.sub_prec_round(term.1, prec, Floor).0;
                (lo, acc.1)
            };
        }
        power = encl_mul(prec, &power, &z_squared);
        acc = if i.even() {
            encl_add(prec, &acc, &term)
        } else {
            encl_sub(prec, &acc, &term)
        };
        i += 1;
    }
}

// `Some(b)` when the width of `b` is at most 2^(-j), and `None` otherwise.
//
// This is equivalent to `enclNarrow?` from `AzFloat/GdBracket.lean`, Azurite.
fn encl_narrow(b: Bracket, j: u64) -> Option<Bracket> {
    if b.1.sub_prec_round_ref_ref(&b.0, 64, Ceiling).0 <= power_of_2_integer(-Integer::from(j)) {
        Some(b)
    } else {
        // The series is only tried when it is expected to converge.
        fail_on_untested_path("encl_narrow, series too wide");
        None
    }
}

// The series path for a small argument a, from an enclosure `t` of tanh(a / 2) at the level `k`: 2
// arctan t by the alternating series, accepted when its width is at most 2^(8 - k).
//
// This is equivalent to `gdSmallOf` from `AzFloat/GdBracket.lean`, Azurite.
fn gd_small_of(t: &Bracket, k: u64) -> Option<Bracket> {
    let a = atan_alt(t, k, k + 32);
    encl_narrow((a.0 << 1u32, a.1 << 1u32), k - 8)
}

// The series path for a large argument a, from an enclosure `z` of e^(-a) at Ziv level `w`: pi / 2
// - 2 arctan z by the alternating series, accepted when its width is at most 2^(-w).
//
// This is equivalent to `gdLargeOf` from `AzFloat/GdBracket.lean`, Azurite.
fn gd_large_of(z: &Bracket, w: u64) -> Option<Bracket> {
    let k = w + 8;
    let prec = k + 56;
    let a = atan_alt(z, k, prec);
    let p = pi_over_2_bracket(prec);
    encl_narrow(
        (
            p.0.sub_prec_round(a.1 << 1u32, prec, Floor).0,
            p.1.sub_prec_round(a.0 << 1u32, prec, Ceiling).0,
        ),
        w,
    )
}

// The general path, from an enclosure `t` of tanh(a / 2) and at Ziv level `w`: 2 arctan t, with the
// arctangent taken at the lower end and its upper end widened by the width of `t`, which is valid
// because the derivative of the arctangent is at most 1.
//
// This is equivalent to `gdCoreOf` from `AzFloat/GdBracket.lean` and `atanWiden` from
// `AzFloat/AtanRatBracket.lean`, Azurite, with the correctly rounded arctangent in place of
// Azurite's bracket.
fn gd_core_of(t: &Bracket, w: u64) -> Bracket {
    let prec = w + 16;
    let (lo, mut hi) = floor_bracket(t.0.atan_prec_round_ref(prec, Floor).0, prec);
    let width = t.1.sub_prec_round_ref_ref(&t.0, 64, Ceiling).0;
    hi.add_prec_round_assign(width, prec, Ceiling);
    (lo << 1u32, hi << 1u32)
}

// The level of accuracy for an argument below 2^e at Ziv level `w`: for a small argument the
// accuracy is relative.
//
// This is equivalent to `atanWork` from `AzFloat/AtanBracket.lean`, Azurite.
fn atan_work(e: &Integer, w: u64) -> u64 {
    if *e < 0u32 {
        w + 8 + u64::exact_from(&-e)
    } else {
        w + 8
    }
}

// Whether the alternating series of tanh(a / 2) is tried for a small argument a < 2^e, at the level
// `k` = `w` + 8 - e: when 32 * (1 - e) >= `k`. Each term gains at least 2 * (1 - e) bits, so the
// series then takes at most about 18 terms.
//
// Azurite's threshold is 64 * (1 - e) >= `k`. This one was measured with the arctangent used here:
// at precisions from 53 to 3000 bits the series is the faster path up to 12 to 22 terms.
const fn small_series_is_tried(k: u64, w: u64) -> bool {
    (k - (w + 8) + 1) << 5 >= k
}

// Whether the alternating series of e^(-a) is tried for a large argument a, at Ziv level `w`: when
// min(170, 4 * sqrt(`w`)) * floor(a) >= `w` + 8. Each term gains about 2.9 * floor(a) bits, so the
// series then takes at most about 60 terms.
//
// Azurite's threshold is 92 * floor(a) >= `w` + 8. This one was measured with the arctangent used
// here: the number of terms up to which the series is the faster path grows with the precision,
// from 13 at 53 bits to about 60 at 3000 bits and beyond.
fn large_series_is_tried(floor: u64, w: u64) -> bool {
    min(170, w.floor_sqrt() << 2) * floor >= w + 8
}

// The bracket of gd(a) for a huge a >= `n`: [pi / 2 - 2^(1 - n), pi / 2].
//
// This is equivalent to `gdHuge` from `AzFloat/GdBracket.lean`, Azurite.
fn gd_huge(n: Integer, w: u64) -> Bracket {
    let prec = w + 72;
    let p = pi_over_2_bracket(prec);
    (
        p.0.sub_prec_round(power_of_2_integer(Integer::ONE - n), prec, Floor)
            .0,
        p.1,
    )
}

// The positive argument a of gd or of its inverse: a `BigExponentFloat` with its exponent, or a
// `Rational` with an exponent bound. In either case a < 2^e for the second field e.
pub(crate) enum GdTarget<'a> {
    BigExponentFloat(&'a BigExponentFloat, &'a Integer),
    Rational(&'a Rational, &'a Integer),
}

impl GdTarget<'_> {
    pub(crate) const fn exponent_bound(&self) -> &Integer {
        match self {
            Self::BigExponentFloat(_, e) | Self::Rational(_, e) => e,
        }
    }

    // A natural number that is at most a. For a `BigExponentFloat` with exponent e it is 2^(e - 1),
    // capped at 2^61, when e >= 1, and 0 otherwise; for a `Rational` it is the floor of a.
    //
    // This is equivalent to `floatAbsFloorN` and `ratAbsFloorN` from `AzFloat/ExpApprox.lean`,
    // Azurite.
    fn lower_bound(&self) -> Natural {
        match self {
            Self::BigExponentFloat(_, e) => {
                if **e >= 62u32 {
                    Natural::power_of_2(61)
                } else if **e >= 1u32 {
                    Natural::power_of_2(u64::exact_from(*e) - 1)
                } else {
                    Natural::ZERO
                }
            }
            Self::Rational(q, _) => Natural::rounding_from(*q, Floor).0,
        }
    }

    // The floor of a, which is less than 2^64, given `self.lower_bound()`.
    fn floor(&self, lower_bound: &Natural) -> u64 {
        match self {
            Self::BigExponentFloat(a, _) => u64::exact_from(&Natural::rounding_from(*a, Floor).0),
            Self::Rational(..) => u64::exact_from(lower_bound),
        }
    }

    // An enclosure of tanh(a / 2) at precision `prec`.
    //
    // This is equivalent to `tanhBracket` and `tanhRatBracket` from `AzFloat/TanhBracket.lean`,
    // Azurite, with the correctly rounded hyperbolic tangent in place of Azurite's bracket.
    fn tanh_of_half(&self, prec: u64) -> Bracket {
        floor_bracket(
            match self {
                Self::BigExponentFloat(a, _) => (*a >> 1u32).tanh_prec_round(prec, Floor).0,
                Self::Rational(q, _) => {
                    BigExponentFloat::tanh_rational_prec_round(*q >> 1u32, prec, Floor).0
                }
            },
            prec,
        )
    }

    // Enclosures of sin(a) and cos(a) at precision `prec`, for a < pi / 2, where both are positive.
    //
    // This is equivalent to `sinCosBracket` from `AzFloat/SinCosBracket.lean` and
    // `sinCosRatBracket` from `AzFloat/SinCosRatBracket.lean`, Azurite, with the correctly rounded
    // sine and cosine in place of Azurite's brackets.
    pub(crate) fn sin_cos(&self, prec: u64) -> (Bracket, Bracket) {
        let (s, c, _, _) = match self {
            Self::BigExponentFloat(a, _) => a.sin_cos_prec_round_ref(prec, Floor),
            Self::Rational(q, _) => {
                BigExponentFloat::sin_cos_rational_prec_round_ref(q, prec, Floor)
            }
        };
        (floor_bracket(s, prec), floor_bracket(c, prec))
    }

    // An enclosure of e^(-a) at precision `prec`.
    //
    // This is equivalent to `expBracket` from `AzFloat/ExpApprox.lean` and `expRatBracket` from
    // `AzFloat/ExpRatApprox.lean`, Azurite, with the correctly rounded exponential in place of
    // Azurite's bracket.
    fn exp_of_negative(&self, prec: u64) -> Bracket {
        floor_bracket(
            match self {
                Self::BigExponentFloat(a, _) => (-*a).exp_prec_round(prec, Floor).0,
                Self::Rational(q, _) => {
                    BigExponentFloat::exp_rational_prec_round(-*q, prec, Floor).0
                }
            },
            prec,
        )
    }
}

// The bracket of gd(a) at Ziv level `w` for a positive a.
//
// This is equivalent to `gdBracket`, `gdSmallSeries?`, `gdLargeSeries?`, `gdCore`, and
// `gdRatBracket` from `AzFloat/GdBracket.lean`, Azurite.
fn gd_bracket(target: &GdTarget, w: u64) -> Bracket {
    let lower_bound = target.lower_bound();
    if lower_bound >= w + 8 {
        return gd_huge(Integer::from(lower_bound), w);
    }
    // a < 2 * (w + 8) here
    let floor = target.floor(&lower_bound);
    let series = if floor != 0 {
        if large_series_is_tried(floor, w) {
            // e^(-a) <= 2^(-floor) is only needed to about w - floor bits.
            gd_large_of(
                &target.exp_of_negative(max(64, (w + 32).saturating_sub(floor))),
                w,
            )
        } else {
            None
        }
    } else {
        let k = atan_work(target.exponent_bound(), w);
        if small_series_is_tried(k, w) {
            gd_small_of(&target.tanh_of_half(w + 64), k)
        } else {
            None
        }
    };
    series.unwrap_or_else(|| gd_core_of(&target.tanh_of_half(w + 32), w))
}

// The tiny-argument path of an odd function f with f(a) in (a, a + a^3] (when `above` is set) or in
// [a - a^3, a) for a tiny positive a, where a < 2^e with e <= -(w + 8): the rounding of f(a) is
// decided from the two ends of that interval when they round alike.
//
// This is equivalent to `sinTiny?` from `AzFloat/SinBracket.lean`, `tanTiny?` from
// `AzFloat/TanBracket.lean`, `sinTinyRat?` and `sinTinyCore?` from `AzFloat/SinRatBracket.lean`,
// and `tanTinyRat?` and `tanTinyCore?` from `AzFloat/TanRatBracket.lean`, Azurite.
fn tiny_argument(
    target: &GdTarget,
    above: bool,
    w: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(BigExponentFloat, Ordering)> {
    // With s = a when `above` is set and s = -a otherwise, f(a) or -f(a) lies in (s, s + a^3].
    let rm = if above { rm } else { -rm };
    let r = match target {
        GdTarget::BigExponentFloat(a, e) => {
            let s = if above { (*a).clone() } else { -*a };
            // a^3 <= a * 2^(2e)
            let cube_bound = *a * power_of_2_integer(*e << 1u32);
            let hi = s.add_prec_round_ref_val(cube_bound, w + 8, Ceiling).0;
            rounding_possible(
                &|p, m| s.round_to_prec_round_ref(p, m),
                &|p, m| hi.round_to_prec_round_ref(p, m),
                prec,
                rm,
            )
        }
        GdTarget::Rational(q, e) => {
            let s = if above { (*q).clone() } else { -*q };
            // a^3 < 2^(3e)
            let cube_bound = power_of_2_integer(const { Integer::const_from_unsigned(3) } * *e);
            let hi = BigExponentFloat::from_rational_prec_round_ref(&s, w + 8, Ceiling)
                .0
                .add_prec_round(cube_bound, w + 8, Ceiling)
                .0;
            rounding_possible(
                &|p, m| BigExponentFloat::from_rational_prec_round_ref(&s, p, m),
                &|p, m| hi.round_to_prec_round_ref(p, m),
                prec,
                rm,
            )
        }
    };
    if above {
        r
    } else {
        r.map(|(r, o)| (-r, o.reverse()))
    }
}

// The value, rounded to `prec` bits with `rm`, of an odd function f at the argument whose absolute
// value a is given by `target` and which is positive when `sign` is set: f(a) is computed, with the
// rounding mode negated for a negative argument. A tiny a takes the tiny-argument path, at the
// level `w` and in the direction `above`; otherwise `general` computes f(a) with a given rounding
// mode.
pub(crate) fn odd_function_near_identity<F: Fn(RoundingMode) -> (BigExponentFloat, Ordering)>(
    sign: bool,
    target: &GdTarget,
    above: bool,
    w: u64,
    prec: u64,
    rm: RoundingMode,
    general: &F,
) -> (BigExponentFloat, Ordering) {
    let abs_rm = if sign { rm } else { -rm };
    let (f, o) = if *target.exponent_bound() <= -Integer::from(w + 8)
        && let Some(r) = tiny_argument(target, above, w, prec, abs_rm)
    {
        r
    } else {
        general(abs_rm)
    };
    if sign { (f, o) } else { (-f, o.reverse()) }
}

// gd(x), rounded to `prec` bits with `rm`.
//
// This is equivalent to `gdPrecRound` from `AzFloat/Gd.lean`, Azurite.
pub(crate) fn gudermannian_helper(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match x {
        BigExponentFloat(NaN) => (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Infinity { sign: true }) => {
            BigExponentFloat::pi_over_2_prec_round(prec, rm)
        }
        BigExponentFloat(Infinity { sign: false }) => {
            let (p, o) = BigExponentFloat::pi_over_2_prec_round(prec, -rm);
            (-p, o.reverse())
        }
        BigExponentFloat(Zero) => (BigExponentFloat::ZERO, Equal),
        BigExponentFloat(Finite {
            sign,
            exponent,
            significand,
        }) => {
            assert_ne!(rm, Exact, "Inexact Gudermannian function");
            let target = GdTarget::BigExponentFloat(&x.abs(), exponent);
            odd_function_near_identity(
                *sign,
                &target,
                false,
                max(prec, significand_prec(significand)) + ZIV_GUARD_BITS,
                prec,
                rm,
                &|rm| ziv_loop_certified(&|w| gd_bracket(&target, w), prec, rm),
            )
        }
    }
}

// gd(q) for a `Rational` q, rounded to `prec` bits with `rm`.
//
// This is equivalent to `gdRatPrecRound` from `AzFloat/GdRat.lean`, Azurite.
pub(crate) fn gudermannian_rational_helper(
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if *q == 0u32 {
        return (BigExponentFloat::ZERO, Equal);
    }
    assert_ne!(rm, Exact, "Inexact Gudermannian function");
    let a = q.abs();
    let target = GdTarget::Rational(&a, &rational_exponent_bound(&a));
    odd_function_near_identity(
        *q > 0u32,
        &target,
        false,
        prec + ZIV_GUARD_BITS,
        prec,
        rm,
        &|rm| ziv_loop_certified(&|w| gd_bracket(&target, w), prec, rm),
    )
}

impl BigExponentFloat {
    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm\pi/2$, correctly rounded
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gudermannian_prec_round), this function never
    /// underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::gudermannian_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec_round(5, Floor);
    /// assert_eq!(w.to_string(), "0.844");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec_round(5, Ceiling);
    /// assert_eq!(w.to_string(), "0.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec_round(5, Nearest);
    /// assert_eq!(w.to_string(), "0.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec_round(20, Floor);
    /// assert_eq!(w.to_string(), "0.86576939");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec_round(20, Ceiling);
    /// assert_eq!(w.to_string(), "0.8657703");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec_round(20, Nearest);
    /// assert_eq!(w.to_string(), "0.86576939");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gudermannian_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        gudermannian_helper(&self, prec, rm)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm\pi/2$, correctly rounded
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gudermannian_prec_round), this function never
    /// underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::gudermannian_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec_round_ref(5, Floor);
    /// assert_eq!(w.to_string(), "0.844");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec_round_ref(5, Ceiling);
    /// assert_eq!(w.to_string(), "0.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec_round_ref(5, Nearest);
    /// assert_eq!(w.to_string(), "0.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec_round_ref(20, Floor);
    /// assert_eq!(w.to_string(), "0.86576939");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec_round_ref(20, Ceiling);
    /// assert_eq!(w.to_string(), "0.8657703");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec_round_ref(20, Nearest);
    /// assert_eq!(w.to_string(), "0.86576939");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gudermannian_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        gudermannian_helper(self, prec, rm)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`BigExponentFloat`] in
    /// place, rounding the result to the specified precision and with the specified rounding mode.
    /// An [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::gudermannian_prec_round`] documentation for information on
    /// special cases.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::gudermannian_prec_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.gudermannian_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.844");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.gudermannian_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.88");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.gudermannian_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.88");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.gudermannian_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.86576939");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.gudermannian_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.8657703");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.gudermannian_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.86576939");
    /// ```
    #[inline]
    pub fn gudermannian_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (mut g, o) = gudermannian_helper(self, prec, rm);
        swap(self, &mut g);
        o
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm\pi/2$, correctly rounded
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gudermannian_prec_round), this function never
    /// underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::gudermannian_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec(5);
    /// assert_eq!(w.to_string(), "0.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec(20);
    /// assert_eq!(w.to_string(), "0.86576939");
    /// assert_eq!(o, Less);
    ///
    /// let x = BigExponentFloat::from(1.0e10).exp_prec(10).0;
    /// let (w, o) = x.gudermannian_prec(20);
    /// assert_eq!(w.to_string(), "1.570797");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn gudermannian_prec(self, prec: u64) -> (Self, Ordering) {
        self.gudermannian_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm\pi/2$, correctly rounded
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gudermannian_prec_round), this function never
    /// underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::gudermannian_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec_ref(5);
    /// assert_eq!(w.to_string(), "0.88");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::ONE.gudermannian_prec_ref(20);
    /// assert_eq!(w.to_string(), "0.86576939");
    /// assert_eq!(o, Less);
    ///
    /// let x = BigExponentFloat::from(1.0e10).exp_prec(10).0;
    /// let (w, o) = x.gudermannian_prec_ref(20);
    /// assert_eq!(w.to_string(), "1.570797");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn gudermannian_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.gudermannian_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`BigExponentFloat`] in
    /// place, rounding the result to the nearest value of the specified precision. An [`Ordering`]
    /// is returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::gudermannian_prec_round`] documentation for information on
    /// special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::gudermannian_prec_round_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^{3/2} \log (n + m) \log\log (n + m))$
    ///
    /// $M(n, m) = O((n + m) \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the exponent of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.gudermannian_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "0.88");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.gudermannian_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.86576939");
    ///
    /// let mut x = BigExponentFloat::from(1.0e10).exp_prec(10).0;
    /// assert_eq!(x.gudermannian_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "1.570797");
    /// ```
    #[inline]
    pub fn gudermannian_prec_assign(&mut self, prec: u64) -> Ordering {
        self.gudermannian_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Rational`], rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gudermannian_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::gudermannian_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.56693554");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.5669365");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.56693554");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn gudermannian_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        gudermannian_rational_helper(&x, prec, rm)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Rational`], rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gudermannian_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::gudermannian_rational_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = BigExponentFloat::gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.56693554");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = BigExponentFloat::gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.5669365");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = BigExponentFloat::gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.56693554");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gudermannian_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        gudermannian_rational_helper(x, prec, rm)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Rational`], rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gudermannian_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::gudermannian_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
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
    /// let (w, o) =
    ///     BigExponentFloat::gudermannian_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) =
    ///     BigExponentFloat::gudermannian_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(w.to_string(), "0.56693554");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gudermannian_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::gudermannian_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Rational`], rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gudermannian_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::gudermannian_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
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
    /// let (w, o) =
    ///     BigExponentFloat::gudermannian_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) =
    ///     BigExponentFloat::gudermannian_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(w.to_string(), "0.56693554");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gudermannian_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::gudermannian_rational_prec_round_ref(x, prec, Nearest)
    }
}
