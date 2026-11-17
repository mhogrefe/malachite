// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::mul_rational::{finite_to_float, from_float_result};
use crate::big_exponent_float::exponential::exp::{round_nudged, ziv_loop};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use core::cmp::{max, min};
use malachite_base::num::arithmetic::traits::{Abs, PowerOf2};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeOne, One,
    Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::exponential::traits::CeilingLogBase2;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

crate_test_enum! {
    // The six forward hyperbolic functions, which share one implementation.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    HyperbolicFunction {
        Sinh,
        Cosh,
        Tanh,
        Coth,
        Sech,
        Csch,
    }
}

impl HyperbolicFunction {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Sinh => "sinh",
            Self::Cosh => "cosh",
            Self::Tanh => "tanh",
            Self::Coth => "coth",
            Self::Sech => "sech",
            Self::Csch => "csch",
        }
    }

    // The value at a NaN, an infinity, or zero: those of `Float`'s functions, except that coth and
    // csch have a pole at 0, where, as a `BigExponentFloat` has no negative zero to tell the two
    // one-sided limits apart, the result is NaN, as 1/0 is in `arf_div`.
    const fn special(self, x: &BigExponentFloat) -> Option<BigExponentFloat> {
        Some(match (self, x) {
            (_, BigExponentFloat(NaN)) | (Self::Coth | Self::Csch, BigExponentFloat(Zero)) => {
                BigExponentFloat::NAN
            }
            (Self::Sinh, BigExponentFloat(Infinity { sign: true }))
            | (Self::Cosh, BigExponentFloat(Infinity { .. })) => BigExponentFloat::INFINITY,
            (Self::Sinh, BigExponentFloat(Infinity { sign: false })) => {
                BigExponentFloat::NEGATIVE_INFINITY
            }
            (Self::Tanh | Self::Coth, BigExponentFloat(Infinity { sign: true })) => {
                BigExponentFloat::ONE
            }
            (Self::Tanh | Self::Coth, BigExponentFloat(Infinity { sign: false })) => {
                BigExponentFloat::NEGATIVE_ONE
            }
            (Self::Sech | Self::Csch, BigExponentFloat(Infinity { .. }))
            | (Self::Sinh | Self::Tanh, BigExponentFloat(Zero)) => BigExponentFloat::ZERO,
            (Self::Cosh | Self::Sech, BigExponentFloat(Zero)) => BigExponentFloat::ONE,
            _ => return None,
        })
    }

    fn rational_at_zero(self) -> BigExponentFloat {
        self.special(&BigExponentFloat::ZERO).unwrap()
    }

    fn float(self, x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        match self {
            Self::Sinh => x.sinh_prec_round_ref(prec, rm),
            Self::Cosh => x.cosh_prec_round_ref(prec, rm),
            Self::Tanh => x.tanh_prec_round_ref(prec, rm),
            Self::Coth => x.coth_prec_round_ref(prec, rm),
            Self::Sech => x.sech_prec_round_ref(prec, rm),
            Self::Csch => x.csch_prec_round_ref(prec, rm),
        }
    }

    fn float_rational(self, x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        match self {
            Self::Sinh => Float::sinh_rational_prec_round_ref(x, prec, rm),
            Self::Cosh => Float::cosh_rational_prec_round_ref(x, prec, rm),
            Self::Tanh => Float::tanh_rational_prec_round_ref(x, prec, rm),
            Self::Coth => Float::coth_rational_prec_round_ref(x, prec, rm),
            Self::Sech => Float::sech_rational_prec_round_ref(x, prec, rm),
            Self::Csch => Float::csch_rational_prec_round_ref(x, prec, rm),
        }
    }

    // The function from s ≈ sinh |x| and c ≈ cosh |x|, with relative errors below 2^(-ks) and
    // 2^(-kc), and the sign of x, as an approximation and an `err` as in `ziv_loop`; quotients and
    // reciprocals as in `TrigFunction::combine`.
    fn combine(
        self,
        negative: bool,
        s: BigExponentFloat,
        ks: u64,
        c: BigExponentFloat,
        kc: u64,
        wp: u64,
    ) -> (BigExponentFloat, u64) {
        let quotient_bits =
            |ka: u64, kb: u64| min(min(ka, kb.saturating_sub(1)), wp).saturating_sub(2);
        let (y, k, odd) = match self {
            Self::Sinh => (s, ks, true),
            Self::Cosh => (c, kc, false),
            Self::Tanh => (s.div_prec_val_ref(&c, wp).0, quotient_bits(ks, kc), true),
            Self::Coth => (c.div_prec_val_ref(&s, wp).0, quotient_bits(kc, ks), true),
            Self::Sech => (c.reciprocal_prec(wp).0, quotient_bits(u64::MAX, kc), false),
            Self::Csch => (s.reciprocal_prec(wp).0, quotient_bits(u64::MAX, ks), true),
        };
        (if odd && negative { -y } else { y }, k.saturating_sub(1))
    }

    // For an x close enough to 0, or (for tanh and coth) large enough, that f(x) is within a tiny
    // fraction of an ulp of a simple exact value v, the pair (v, up), where f(x) is v plus (if
    // `up`) or minus a positive amount small enough for `round_nudged` (which needs it below
    // 2^(EXP(v) - max(p, q) - 2), with q the precision of v):
    // - near 0, with e = EXP(x) and 2e <= -max(p, q) - 4: sinh x = x + x^3/6 + ... and tanh x = x -
    //   x^3/3 + ... differ from x by less than 2^(3e); cosh x = 1 + x^2/2 + ... and sech x = 1 -
    //   x^2/2 + ... differ from 1 by less than 2^(2e - 1); and coth x = 1/x + x/3 - ... and csch x
    //   = 1/x - x/6 + ... differ from 1/x by less than |x|/2 < 2^(e - 1), when 1/x is exact (x a
    //   power of 2);
    // - for |x| >= p + 4: tanh |x| = 1 - 2/(e^(2|x|) + 1) and coth |x| = 1 + 2/(e^(2|x|) - 1)
    //   differ from 1 by less than 4 e^(-2|x|) < 2^(2 - 2.8(p + 4)), well below 2^(-p - 1).
    fn nudge(
        self,
        negative: bool,
        exponent: &Integer,
        exact: Option<&BigExponentFloat>,
        prec: u64,
    ) -> Option<(BigExponentFloat, bool)> {
        if *exponent <= 0u32 {
            let (v, up) = match self {
                Self::Sinh => (exact?.clone(), !negative),
                Self::Tanh => (exact?.clone(), negative),
                Self::Cosh => (BigExponentFloat::ONE, true),
                Self::Sech => (BigExponentFloat::ONE, false),
                Self::Coth | Self::Csch => {
                    let x = exact?;
                    if x.get_prec() != Some(1) {
                        return None;
                    }
                    (x.reciprocal_prec_ref(1).0, (self == Self::Coth) != negative)
                }
            };
            let q = v.get_prec().unwrap();
            return (max(prec, q) + 4 <= -(exponent << 1u32)).then_some((v, up));
        }
        if !matches!(self, Self::Tanh | Self::Coth) {
            return None;
        }
        // |x| >= 2^(e - 1) >= p + 4
        let big = *exponent > 64u32 || u64::power_of_2(u64::exact_from(exponent) - 1) >= prec + 4;
        if !big {
            return None;
        }
        let v = if negative {
            BigExponentFloat::NEGATIVE_ONE
        } else {
            BigExponentFloat::ONE
        };
        // tanh moves toward 0 from ±1, and coth away from it
        Some((v, (self == Self::Coth) != negative))
    }
}

// The argument x of a forward hyperbolic function.
#[derive(Clone, Copy)]
enum HyperbolicArgument<'a> {
    BigExponentFloat(&'a BigExponentFloat),
    Rational(&'a Rational),
}

impl HyperbolicArgument<'_> {
    fn exponent(&self) -> Integer {
        match self {
            HyperbolicArgument::BigExponentFloat(x) => x.get_exponent().unwrap().clone(),
            HyperbolicArgument::Rational(q) => {
                Integer::from(q.floor_log_base_2_abs()) + Integer::ONE
            }
        }
    }

    fn negative(&self) -> bool {
        match self {
            HyperbolicArgument::BigExponentFloat(x) => **x < 0u32,
            HyperbolicArgument::Rational(q) => **q < 0u32,
        }
    }

    // |x|, exactly for a `BigExponentFloat` and with a relative error of at most 2^(-w) for a
    // `Rational`.
    fn abs_approx(&self, w: u64) -> BigExponentFloat {
        match self {
            HyperbolicArgument::BigExponentFloat(x) => (*x).abs(),
            HyperbolicArgument::Rational(q) => {
                BigExponentFloat::from_rational_prec_ref(q, w).0.abs()
            }
        }
    }

    // e^|x|, with a relative error of at most 2^(-w).
    fn exp_abs(&self, w: u64) -> BigExponentFloat {
        match self {
            HyperbolicArgument::BigExponentFloat(x) => (*x).abs().exp_prec(w).0,
            HyperbolicArgument::Rational(q) => BigExponentFloat::exp_rational_prec((*q).abs(), w).0,
        }
    }

    // x itself, if it is a `BigExponentFloat`.
    fn exact(&self) -> Option<Cow<'_, BigExponentFloat>> {
        match self {
            HyperbolicArgument::BigExponentFloat(x) => Some(Cow::Borrowed(x)),
            HyperbolicArgument::Rational(q) => BigExponentFloat::try_from(*q).ok().map(Cow::Owned),
        }
    }
}

// Approximations of sinh t and cosh t for 0 < t < 1, from their Taylor series, whose terms are all
// positive, with every operation rounded to `w` bits, and a k such that both relative errors are
// below 2^(-k): each of the n terms carries at most 3n roundings, and with all terms positive the
// sums' relative errors are below (4n + 4) 2^(-w). The sums stop once a term is below 2^(-w - 2)
// times the sum.
fn sinh_cosh_series(t: &BigExponentFloat, w: u64) -> (BigExponentFloat, BigExponentFloat, u64) {
    let t2 = t.square_prec_ref(w).0;
    let mut cosh_sum = BigExponentFloat::ONE;
    let mut sinh_sum = BigExponentFloat::ONE;
    let mut term = BigExponentFloat::ONE;
    let limit = -Integer::from(w + 2);
    let mut k: u64 = 1;
    loop {
        term = term
            .mul_prec_ref_ref(&t2, w)
            .0
            .div_prec_val_ref(&BigExponentFloat::from(((k << 1) - 1) * (k << 1)), w)
            .0;
        let sinh_term = term
            .div_prec_ref_ref(&BigExponentFloat::from((k << 1) + 1), w)
            .0;
        cosh_sum.add_prec_assign_ref(&term, w);
        sinh_sum.add_prec_assign_ref(&sinh_term, w);
        if *term.get_exponent().unwrap() < limit {
            break;
        }
        k += 1;
    }
    let s = t.mul_prec_ref_ref(&sinh_sum, w).0;
    (
        s,
        cosh_sum,
        w.saturating_sub(((k << 2) + 4).ceiling_log_base_2() + 1),
    )
}

// An approximation of f(x) for a finite nonzero x, and an `err` as in `ziv_loop`. With w = wp + 12:
// - for |x| < 1, sinh |x| and cosh |x| come from `sinh_cosh_series` on |x| (exact, or for a
//   `Rational` with a relative error of at most 2^(-w); sinh t / t and cosh t change by less than
//   that relative amount);
// - otherwise E = e^|x| has a relative error of at most 2^(-w), and E^(-1) one of at most 2^(1 -
//   w); sinh |x| = (E - E^(-1))/2, where (E + E^(-1))/(E - E^(-1)) <= (e + 1/e)/(e - 1/e) < 1.32
//   magnifies these to below 2^(2 - w), and cosh |x| = (E + E^(-1))/2, with a relative error below
//   2^(2 - w). For a huge |x|, e^|x| is computed without overflow, and E^(-1) is negligible but
//   harmless.
fn hyperbolic_approx(
    f: HyperbolicFunction,
    x: HyperbolicArgument<'_>,
    wp: u64,
) -> (BigExponentFloat, u64) {
    let w = wp + 12;
    let (s, c, k) = if x.exponent() <= 0u32 {
        sinh_cosh_series(&x.abs_approx(w), w)
    } else {
        let e = x.exp_abs(w);
        let em = e.reciprocal_prec_ref(w).0;
        let s = e.sub_prec_ref_ref(&em, w).0 >> 1u32;
        let c = e.add_prec_val_ref(&em, w).0 >> 1u32;
        (s, c, w - 4)
    };
    f.combine(x.negative(), s, k, c, k, wp)
}

// The result of one of `Float`'s functions, converted if it neither overflowed nor underflowed;
// with `Exact`, the `Float` function is called with `Floor`, and the result must then be exact.
fn from_float_hyperbolic(
    result: (Float, Ordering),
    f: HyperbolicFunction,
    rm: RoundingMode,
) -> Option<(BigExponentFloat, Ordering)> {
    let (y, o) = from_float_result(result, None)?;
    assert!(
        rm != Exact || o == Equal,
        "Inexact BigExponentFloat {}",
        f.name()
    );
    Some((y, o))
}

// Whether the exponent of the finite nonzero x is within `Float`'s exponent range, and if so, the
// exponent.
fn float_exponent(x: &BigExponentFloat) -> Option<i32> {
    let e = i32::try_from(x.get_exponent().unwrap()).ok()?;
    (Float::MIN_EXPONENT..=Float::MAX_EXPONENT)
        .contains(&e)
        .then_some(e)
}

// f(x) for a nonzero argument, after `Float`'s function: `round_nudged` when f(x) is that close to
// a simple value (see `HyperbolicFunction::nudge`), and otherwise a Ziv loop. The value is
// transcendental, so `Exact` is rejected.
fn round_hyperbolic(
    f: HyperbolicFunction,
    x: HyperbolicArgument<'_>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(rm, Exact, "Inexact BigExponentFloat {}", f.name());
    let exact = x.exact();
    if let Some((v, up)) = f.nudge(x.negative(), &x.exponent(), exact.as_deref(), prec) {
        return round_nudged(&v, up, prec, rm);
    }
    ziv_loop(prec, rm, |wp| hyperbolic_approx(f, x, wp))
}

// f(x), rounded to `prec` bits with `rm`. The special values are those of
// `HyperbolicFunction::special`; every other value is transcendental, so `Exact` is rejected. If
// `use_float` and x's exponent is within `Float`'s range, `Float`'s function is used unless its
// result overflows or underflows (as it does for |x| beyond about 7.4 * 10^8, or a tiny x);
// otherwise, `round_hyperbolic`.
pub(crate) fn hyperbolic_helper(
    f: HyperbolicFunction,
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
    use_float: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if let Some(y) = f.special(x) {
        return (y, Equal);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat {}", f.name());
    if use_float
        && let Some(e) = float_exponent(x)
        && let Some(result) = from_float_hyperbolic(
            f.float(&finite_to_float(Cow::Borrowed(x), e), prec, rm),
            f,
            rm,
        )
    {
        return result;
    }
    round_hyperbolic(f, HyperbolicArgument::BigExponentFloat(x), prec, rm)
}

// f(x) for a `Rational` x, rounded to `prec` bits with `rm`: as in `hyperbolic_helper`, with
// `Float`'s `Rational` functions.
pub(crate) fn hyperbolic_rational_helper(
    f: HyperbolicFunction,
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if *x == 0u32 {
        return (f.rational_at_zero(), Equal);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat {}", f.name());
    if let Some(result) = from_float_hyperbolic(f.float_rational(x, prec, rm), f, rm) {
        return result;
    }
    round_hyperbolic(f, HyperbolicArgument::Rational(x), prec, rm)
}

crate_test_fn! {
    // f(x) rounded to `prec` bits with `rm`, without `Float`'s hyperbolic functions: from the
    // series or from e^|x|, as in `hyperbolic_approx`. Used to check `hyperbolic_helper`.
    hyperbolic_prec_round_generic(
        f: HyperbolicFunction,
        x: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        hyperbolic_helper(f, x, prec, rm, false)
    }
}

impl BigExponentFloat {
    /// Computes the hyperbolic sine and cosine of a [`BigExponentFloat`], rounding the results to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is
    /// taken by value. Two [`Ordering`]s are also returned, indicating whether the rounded values
    /// are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sinh x+\varepsilon_1, \cosh x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sinh_prec_round`] and [`BigExponentFloat::cosh_prec_round`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=(\text{NaN},\text{NaN})$
    /// - $f(\pm\infty,p,m)=(\pm\infty,\infty)$
    /// - $f(0,p,m)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sinh_cosh_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: for an argument with exponent $e > 0$, $e^{|x|}$ is computed by reducing $|x|$
    /// modulo $\log 2$, which takes about $e$ bits of $\log 2$. The time and memory needed
    /// therefore grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs
    /// about $2^{40}$ bits, or 128 GiB, of $\log 2$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::sinh_cosh_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the argument is finite and nonzero (when
    /// the results are transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sinh_cosh_prec_round(5, Floor);
    /// assert_eq!(s.to_string(), "1.12");
    /// assert_eq!(c.to_string(), "1.5");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sinh_cosh_prec_round(5, Ceiling);
    /// assert_eq!(s.to_string(), "1.19");
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sinh_cosh_prec_round(5, Nearest);
    /// assert_eq!(s.to_string(), "1.19");
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sinh_cosh_prec_round(20, Floor);
    /// assert_eq!(s.to_string(), "1.1751995");
    /// assert_eq!(c.to_string(), "1.543079");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sinh_cosh_prec_round(20, Ceiling);
    /// assert_eq!(s.to_string(), "1.17520");
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sinh_cosh_prec_round(20, Nearest);
    /// assert_eq!(s.to_string(), "1.17520");
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sinh_cosh_prec_round(
        self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = hyperbolic_helper(HyperbolicFunction::Sinh, &self, prec, rm, true);
        let (c, o_c) = hyperbolic_helper(HyperbolicFunction::Cosh, &self, prec, rm, true);
        (s, c, o_s, o_c)
    }

    /// Computes the hyperbolic sine and cosine of a [`BigExponentFloat`], rounding the results to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is
    /// taken by reference. Two [`Ordering`]s are also returned, indicating whether the rounded
    /// values are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sinh x+\varepsilon_1, \cosh x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sinh_prec_round`] and [`BigExponentFloat::cosh_prec_round`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=(\text{NaN},\text{NaN})$
    /// - $f(\pm\infty,p,m)=(\pm\infty,\infty)$
    /// - $f(0,p,m)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sinh_cosh_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: for an argument with exponent $e > 0$, $e^{|x|}$ is computed by reducing $|x|$
    /// modulo $\log 2$, which takes about $e$ bits of $\log 2$. The time and memory needed
    /// therefore grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs
    /// about $2^{40}$ bits, or 128 GiB, of $\log 2$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the argument is finite and nonzero (when
    /// the results are transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sinh_cosh_prec_round_ref(5, Floor);
    /// assert_eq!(s.to_string(), "1.12");
    /// assert_eq!(c.to_string(), "1.5");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sinh_cosh_prec_round_ref(5, Ceiling);
    /// assert_eq!(s.to_string(), "1.19");
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sinh_cosh_prec_round_ref(5, Nearest);
    /// assert_eq!(s.to_string(), "1.19");
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sinh_cosh_prec_round_ref(20, Floor);
    /// assert_eq!(s.to_string(), "1.1751995");
    /// assert_eq!(c.to_string(), "1.543079");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sinh_cosh_prec_round_ref(20, Ceiling);
    /// assert_eq!(s.to_string(), "1.17520");
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sinh_cosh_prec_round_ref(20, Nearest);
    /// assert_eq!(s.to_string(), "1.17520");
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sinh_cosh_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = hyperbolic_helper(HyperbolicFunction::Sinh, self, prec, rm, true);
        let (c, o_c) = hyperbolic_helper(HyperbolicFunction::Cosh, self, prec, rm, true);
        (s, c, o_s, o_c)
    }

    /// Replaces a [`BigExponentFloat`] with its hyperbolic sine and writes its hyperbolic cosine to
    /// `cosh`, rounding both results to the specified precision and with the specified rounding
    /// mode. The previous value of `cosh` is discarded. Two [`Ordering`]s are returned, indicating
    /// whether the rounded values are less than, equal to, or greater than the exact values.
    ///
    /// See [`BigExponentFloat::sinh_cosh_prec_round`] for the error bounds, the special cases, and
    /// the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the argument is finite and nonzero (when
    /// the results are transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sinh_cosh_prec_round_assign(&mut c, 5, Floor),
    ///     (Less, Less)
    /// );
    /// assert_eq!(x.to_string(), "1.12");
    /// assert_eq!(c.to_string(), "1.5");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sinh_cosh_prec_round_assign(&mut c, 5, Ceiling),
    ///     (Greater, Greater)
    /// );
    /// assert_eq!(x.to_string(), "1.19");
    /// assert_eq!(c.to_string(), "1.56");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sinh_cosh_prec_round_assign(&mut c, 5, Nearest),
    ///     (Greater, Greater)
    /// );
    /// assert_eq!(x.to_string(), "1.19");
    /// assert_eq!(c.to_string(), "1.56");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sinh_cosh_prec_round_assign(&mut c, 20, Floor),
    ///     (Less, Less)
    /// );
    /// assert_eq!(x.to_string(), "1.1751995");
    /// assert_eq!(c.to_string(), "1.543079");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sinh_cosh_prec_round_assign(&mut c, 20, Ceiling),
    ///     (Greater, Greater)
    /// );
    /// assert_eq!(x.to_string(), "1.17520");
    /// assert_eq!(c.to_string(), "1.5430813");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(
    ///     x.sinh_cosh_prec_round_assign(&mut c, 20, Nearest),
    ///     (Greater, Greater)
    /// );
    /// assert_eq!(x.to_string(), "1.17520");
    /// assert_eq!(c.to_string(), "1.5430813");
    /// ```
    #[inline]
    pub fn sinh_cosh_prec_round_assign(
        &mut self,
        cosh: &mut Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        let (s, o_s) = hyperbolic_helper(HyperbolicFunction::Sinh, self, prec, rm, true);
        let (c, o_c) = hyperbolic_helper(HyperbolicFunction::Cosh, self, prec, rm, true);
        *self = s;
        *cosh = c;
        (o_s, o_c)
    }

    /// Computes the hyperbolic sine and cosine of a [`BigExponentFloat`], rounding the results to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] is taken by value.
    /// Two [`Ordering`]s are also returned, indicating whether the rounded values are less than,
    /// equal to, or greater than the exact values. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal` for it.
    ///
    /// $$
    /// f(x,p) = (\sinh x+\varepsilon_1, \cosh x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sinh_prec`] and [`BigExponentFloat::cosh_prec`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=(\text{NaN},\text{NaN})$
    /// - $f(\pm\infty,p)=(\pm\infty,\infty)$
    /// - $f(0,p)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sinh_cosh_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: for an argument with exponent $e > 0$, $e^{|x|}$ is computed by reducing $|x|$
    /// modulo $\log 2$, which takes about $e$ bits of $\log 2$. The time and memory needed
    /// therefore grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs
    /// about $2^{40}$ bits, or 128 GiB, of $\log 2$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::sinh_cosh_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of the argument.
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
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sinh_cosh_prec(5);
    /// assert_eq!(s.to_string(), "1.19");
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::ONE.sinh_cosh_prec(20);
    /// assert_eq!(s.to_string(), "1.17520");
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sinh_cosh_prec(self, prec: u64) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = hyperbolic_helper(HyperbolicFunction::Sinh, &self, prec, Nearest, true);
        let (c, o_c) = hyperbolic_helper(HyperbolicFunction::Cosh, &self, prec, Nearest, true);
        (s, c, o_s, o_c)
    }

    /// Computes the hyperbolic sine and cosine of a [`BigExponentFloat`], rounding the results to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// reference. Two [`Ordering`]s are also returned, indicating whether the rounded values are
    /// less than, equal to, or greater than the exact values. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal` for
    /// it.
    ///
    /// $$
    /// f(x,p) = (\sinh x+\varepsilon_1, \cosh x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sinh_prec`] and [`BigExponentFloat::cosh_prec`].
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=(\text{NaN},\text{NaN})$
    /// - $f(\pm\infty,p)=(\pm\infty,\infty)$
    /// - $f(0,p)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sinh_cosh_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// Warning: for an argument with exponent $e > 0$, $e^{|x|}$ is computed by reducing $|x|$
    /// modulo $\log 2$, which takes about $e$ bits of $\log 2$. The time and memory needed
    /// therefore grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs
    /// about $2^{40}$ bits, or 128 GiB, of $\log 2$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the number of significant bits of the integer part of the argument.
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
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sinh_cosh_prec_ref(5);
    /// assert_eq!(s.to_string(), "1.19");
    /// assert_eq!(c.to_string(), "1.56");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = (&BigExponentFloat::ONE).sinh_cosh_prec_ref(20);
    /// assert_eq!(s.to_string(), "1.17520");
    /// assert_eq!(c.to_string(), "1.5430813");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sinh_cosh_prec_ref(&self, prec: u64) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = hyperbolic_helper(HyperbolicFunction::Sinh, self, prec, Nearest, true);
        let (c, o_c) = hyperbolic_helper(HyperbolicFunction::Cosh, self, prec, Nearest, true);
        (s, c, o_s, o_c)
    }

    /// Replaces a [`BigExponentFloat`] with its hyperbolic sine and writes its hyperbolic cosine to
    /// `cosh`, rounding both results to the nearest value of the specified precision. The previous
    /// value of `cosh` is discarded. Two [`Ordering`]s are returned, indicating whether the rounded
    /// values are less than, equal to, or greater than the exact values.
    ///
    /// See [`BigExponentFloat::sinh_cosh_prec`] for the error bounds, the special cases, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(x.sinh_cosh_prec_assign(&mut c, 5), (Greater, Greater));
    /// assert_eq!(x.to_string(), "1.19");
    /// assert_eq!(c.to_string(), "1.56");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// let mut c = BigExponentFloat::NAN;
    /// assert_eq!(x.sinh_cosh_prec_assign(&mut c, 20), (Greater, Greater));
    /// assert_eq!(x.to_string(), "1.17520");
    /// assert_eq!(c.to_string(), "1.5430813");
    /// ```
    #[inline]
    pub fn sinh_cosh_prec_assign(&mut self, cosh: &mut Self, prec: u64) -> (Ordering, Ordering) {
        let (s, o_s) = hyperbolic_helper(HyperbolicFunction::Sinh, self, prec, Nearest, true);
        let (c, o_c) = hyperbolic_helper(HyperbolicFunction::Cosh, self, prec, Nearest, true);
        *self = s;
        *cosh = c;
        (o_s, o_c)
    }

    /// Computes the hyperbolic sine and cosine of a [`Rational`], returning [`BigExponentFloat`]s,
    /// rounding the results to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by value. Two [`Ordering`]s are also returned, indicating whether the
    /// rounded values are less than, equal to, or greater than the exact values. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sinh x+\varepsilon_1, \cosh x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sinh_rational_prec_round`] and
    /// [`BigExponentFloat::cosh_rational_prec_round`].
    ///
    /// Special cases:
    /// - $f(0,p,m)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sinh_cosh_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::sinh_cosh_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the argument is finite and nonzero (when
    /// the results are transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sinh_cosh_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.328");
    /// assert_eq!(c.to_string(), "1.0");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sinh_cosh_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.344");
    /// assert_eq!(c.to_string(), "1.06");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sinh_cosh_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.344");
    /// assert_eq!(c.to_string(), "1.06");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sinh_cosh_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.3395405");
    /// assert_eq!(c.to_string(), "1.0560703");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sinh_cosh_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.33954096");
    /// assert_eq!(c.to_string(), "1.056072");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sinh_cosh_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.3395405");
    /// assert_eq!(c.to_string(), "1.056072");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sinh_cosh_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = hyperbolic_rational_helper(HyperbolicFunction::Sinh, &x, prec, rm);
        let (c, o_c) = hyperbolic_rational_helper(HyperbolicFunction::Cosh, &x, prec, rm);
        (s, c, o_s, o_c)
    }

    /// Computes the hyperbolic sine and cosine of a [`Rational`], returning [`BigExponentFloat`]s,
    /// rounding the results to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by reference. Two [`Ordering`]s are also returned, indicating whether
    /// the rounded values are less than, equal to, or greater than the exact values. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal` for it.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sinh x+\varepsilon_1, \cosh x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sinh_rational_prec_round`] and
    /// [`BigExponentFloat::cosh_rational_prec_round`].
    ///
    /// Special cases:
    /// - $f(0,p,m)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sinh_cosh_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of the argument.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the argument is finite and nonzero (when
    /// the results are transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sinh_cosh_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.328");
    /// assert_eq!(c.to_string(), "1.0");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sinh_cosh_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.344");
    /// assert_eq!(c.to_string(), "1.06");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sinh_cosh_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.344");
    /// assert_eq!(c.to_string(), "1.06");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sinh_cosh_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(s.to_string(), "0.3395405");
    /// assert_eq!(c.to_string(), "1.0560703");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sinh_cosh_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(s.to_string(), "0.33954096");
    /// assert_eq!(c.to_string(), "1.056072");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = BigExponentFloat::sinh_cosh_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.3395405");
    /// assert_eq!(c.to_string(), "1.056072");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sinh_cosh_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = hyperbolic_rational_helper(HyperbolicFunction::Sinh, x, prec, rm);
        let (c, o_c) = hyperbolic_rational_helper(HyperbolicFunction::Cosh, x, prec, rm);
        (s, c, o_s, o_c)
    }

    /// Computes the hyperbolic sine and cosine of a [`Rational`], returning [`BigExponentFloat`]s,
    /// rounding the results to the nearest value of the specified precision. The [`Rational`] is
    /// taken by value. Two [`Ordering`]s are also returned, indicating whether the rounded values
    /// are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal` for it.
    ///
    /// $$
    /// f(x,p) = (\sinh x+\varepsilon_1, \cosh x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sinh_rational_prec`] and [`BigExponentFloat::cosh_rational_prec`].
    ///
    /// Special cases:
    /// - $f(0,p)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sinh_cosh_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::sinh_cosh_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of the argument.
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
    /// let (s, c, o_s, o_c) =
    ///     BigExponentFloat::sinh_cosh_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(s.to_string(), "0.344");
    /// assert_eq!(c.to_string(), "1.06");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) =
    ///     BigExponentFloat::sinh_cosh_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(s.to_string(), "0.3395405");
    /// assert_eq!(c.to_string(), "1.056072");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn sinh_cosh_rational_prec(x: Rational, prec: u64) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = hyperbolic_rational_helper(HyperbolicFunction::Sinh, &x, prec, Nearest);
        let (c, o_c) = hyperbolic_rational_helper(HyperbolicFunction::Cosh, &x, prec, Nearest);
        (s, c, o_s, o_c)
    }

    /// Computes the hyperbolic sine and cosine of a [`Rational`], returning [`BigExponentFloat`]s,
    /// rounding the results to the nearest value of the specified precision. The [`Rational`] is
    /// taken by reference. Two [`Ordering`]s are also returned, indicating whether the rounded
    /// values are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal` for it.
    ///
    /// $$
    /// f(x,p) = (\sinh x+\varepsilon_1, \cosh x+\varepsilon_2).
    /// $$
    /// The errors $\varepsilon_1$ and $\varepsilon_2$ are bounded as for
    /// [`BigExponentFloat::sinh_rational_prec`] and [`BigExponentFloat::cosh_rational_prec`].
    ///
    /// Special cases:
    /// - $f(0,p)=(0,1)$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::sinh_cosh_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, x.significant_bits())`, and
    /// $m$ is the number of significant bits of the integer part of the argument.
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
    /// let (s, c, o_s, o_c) =
    ///     BigExponentFloat::sinh_cosh_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(s.to_string(), "0.344");
    /// assert_eq!(c.to_string(), "1.06");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) =
    ///     BigExponentFloat::sinh_cosh_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(s.to_string(), "0.3395405");
    /// assert_eq!(c.to_string(), "1.056072");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sinh_cosh_rational_prec_ref(
        x: &Rational,
        prec: u64,
    ) -> (Self, Self, Ordering, Ordering) {
        let (s, o_s) = hyperbolic_rational_helper(HyperbolicFunction::Sinh, x, prec, Nearest);
        let (c, o_c) = hyperbolic_rational_helper(HyperbolicFunction::Cosh, x, prec, Nearest);
        (s, c, o_s, o_c)
    }
}
