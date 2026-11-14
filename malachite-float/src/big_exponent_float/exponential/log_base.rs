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
use crate::big_exponent_float::exponential::ln::{
    LogBase, log_1_plus_x_helper, log_helper, log_rational_helper,
};
use crate::float::exponential::log_base::rational_log_base_of_rational;
use crate::float::exponential::log_base_rational_rational_base::*;
use alloc::borrow::Cow;
use core::cmp::{
    Ordering::{self, *},
    max,
};
use malachite_base::num::arithmetic::traits::{CheckedLogBase, DivisibleBy, Gcd, IsPowerOf2, Pow};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, One, Two, Zero as ZeroTrait,
};
use malachite_base::num::comparison::traits::OrdProduct;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::ExpressAsPower;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// The base b of a logarithm, other than the bases of `LogBase`: a `u64` at least 2, a
// `BigExponentFloat` (any value; the logarithm is then ln(x) / ln(b), with the special values of
// `quotient_special`), or a `Rational` greater than 1.
#[derive(Clone, Copy, Debug)]
pub(crate) enum GeneralBase<'a> {
    Unsigned(u64),
    BigExponentFloat(&'a BigExponentFloat),
    Rational(&'a Rational),
}

impl GeneralBase<'_> {
    // log_2(b), correctly rounded to `wp` bits.
    fn log_2(self, wp: u64) -> BigExponentFloat {
        match self {
            Self::Unsigned(b) => {
                log_helper(
                    Cow::Owned(BigExponentFloat::from(b)),
                    LogBase::Two,
                    wp,
                    Nearest,
                )
                .0
            }
            Self::BigExponentFloat(b) => log_helper(Cow::Borrowed(b), LogBase::Two, wp, Nearest).0,
            Self::Rational(b) => log_rational_helper(b, LogBase::Two, wp, Nearest).0,
        }
    }

    // An l with |ln(b)| >= 2^l. For b >= 3, ln(b) > 1; otherwise ln(b) is computed with 10 bits,
    // which is within a factor 1 + 2^(-9) of the true value, so ln(b) >= 2^(EXP - 2).
    fn ln_lower_exponent(self) -> Integer {
        let ln = match self {
            Self::Unsigned(_) => return Integer::ZERO,
            Self::BigExponentFloat(b) => log_helper(Cow::Borrowed(b), LogBase::E, 10, Nearest).0,
            Self::Rational(b) => log_rational_helper(b, LogBase::E, 10, Nearest).0,
        };
        ln.get_exponent().unwrap() - Integer::TWO
    }

    // Whether b > 1, so that log_b is increasing.
    fn increasing(self) -> bool {
        match self {
            Self::Unsigned(_) | Self::Rational(_) => true,
            Self::BigExponentFloat(b) => *b > 1u32,
        }
    }
}

// The odd part m and 2-adic exponent t of a positive dyadic value m 2^t.
fn dyadic_parts(x: &BigExponentFloat) -> (Natural, Integer) {
    finite_odd_part(x)
}

// For a positive dyadic base b = s_b 2^(t_b) other than 1, the primitive root: (z, h, e) with b =
// (2^z h)^e, h odd, and e maximal. This is `Float`'s `dyadic_primitive_root` with `Integer`
// exponents.
fn dyadic_primitive_root(s_b: &Natural, t_b: &Integer) -> (Integer, Natural, Natural) {
    if *s_b == 1u32 {
        let e = t_b.unsigned_abs_ref().clone();
        return (
            if *t_b > 0u32 {
                Integer::ONE
            } else {
                -Integer::ONE
            },
            Natural::ONE,
            e,
        );
    }
    let (h0, k) = s_b.express_as_power().unwrap_or_else(|| (s_b.clone(), 1));
    let k = Natural::from(k);
    let e = if *t_b == 0u32 {
        k.clone()
    } else {
        (&k).gcd(t_b.unsigned_abs_ref())
    };
    let z = t_b / Integer::from(&e);
    let h = h0.pow(u64::exact_from(&(k / &e)));
    (z, h, e)
}

// For a positive dyadic x = s 2^t and a root 2^z h (h odd, the root not 1), the integer m with x =
// (2^z h)^m, if there is one. This is `Float`'s `dyadic_log_of_root` with `Integer` exponents: for
// h = 1, x must be 2^t with z dividing t; for odd h >= 3, s = h^m and t = z m with m >= 0.
fn dyadic_log_of_root(s: &Natural, t: &Integer, z: &Integer, h: &Natural) -> Option<Integer> {
    if *h == 1u32 {
        return (*s == 1u32 && *z != 0u32 && t.divisible_by(z)).then(|| t / z);
    }
    if *s == 1u32 {
        return None;
    }
    let m = Integer::from(s.checked_log_base(h)?);
    (Integer::cmp_product(z, &m, t) == Equal).then_some(m)
}

// log_b(x) for a positive dyadic x and a positive dyadic base b other than 1, if it is rational:
// with b = (2^z h)^e its primitive root, x must be (2^z h)^m, and log_b(x) = m / e. Only odd parts
// and exponents are used, so nothing the size of 2^|t| is materialized.
fn dyadic_exact_log(x: &BigExponentFloat, b: &BigExponentFloat) -> Option<Rational> {
    let (s_b, t_b) = dyadic_parts(b);
    let (z, h, e) = dyadic_primitive_root(&s_b, &t_b);
    let (s, t) = dyadic_parts(x);
    let m = dyadic_log_of_root(&s, &t, &z, &h)?;
    Some(Rational::from(m) / Rational::from(e))
}

impl GeneralBase<'_> {
    // log_b(x) for a finite positive x other than 1, if it is rational. A `BigExponentFloat` x is
    // dyadic, so for a `Rational` base it requires a dyadic base.
    fn exact_log(self, x: &BigExponentFloat) -> Option<Rational> {
        match self {
            Self::Unsigned(b) => dyadic_exact_log(x, &BigExponentFloat::from(b)),
            Self::BigExponentFloat(b) => dyadic_exact_log(x, b),
            Self::Rational(b) => dyadic_exact_log(x, &BigExponentFloat::try_from(b).ok()?),
        }
    }

    // log_b(x) for a positive `Rational` x other than 1, if it is rational.
    fn exact_log_rational(self, x: &Rational) -> Option<Rational> {
        match self {
            Self::Unsigned(b) => rational_log_base_of_rational(x, b),
            Self::Rational(b) => rational_log_base_rational_rational_base(x, b),
            Self::BigExponentFloat(b) => dyadic_exact_log(&BigExponentFloat::try_from(x).ok()?, b),
        }
    }
}

// The class of ln(v) for a `BigExponentFloat` v, for IEEE division of logarithms: `None` for NaN (v
// NaN or negative), and otherwise the sign of ln(v) (-1, 0, or 1) and whether it is infinite.
fn ln_class(v: &BigExponentFloat) -> Option<(i8, bool)> {
    match v {
        BigExponentFloat(NaN | Infinity { sign: false } | Finite { sign: false, .. }) => None,
        BigExponentFloat(Infinity { sign: true }) => Some((1, true)),
        BigExponentFloat(Zero) => Some((-1, true)),
        v => Some((
            match v.partial_cmp(&1u32).unwrap() {
                Less => -1,
                Equal => 0,
                Greater => 1,
            },
            false,
        )),
    }
}

// The special value of the quotient ln(x) / ln(b) of two logarithms with classes `lx` and `lb`
// (from `ln_class`), or `None` if both logarithms are finite and nonzero, in which case the
// quotient must be computed. The special values follow FLINT's `arf_div`: a NaN, a zero divisor, or
// two special operands (zeros or infinities) give NaN, except that 0 divided by a nonzero value,
// and a finite value divided by an infinity, give 0; an infinity divided by a finite nonzero value
// is an infinity. In particular, a base of 1 gives NaN (ln(b) = 0, and it takes both signs near b =
// 1), where `Float`'s logarithms, which divide by a signed zero, give an infinity.
fn quotient_special(lx: Option<(i8, bool)>, lb: Option<(i8, bool)>) -> Option<BigExponentFloat> {
    let ((sx, ix), (sb, ib)) = match (lx, lb) {
        (Some(lx), Some(lb)) => (lx, lb),
        _ => return Some(BigExponentFloat::NAN),
    };
    let infinity = |s: i8| {
        if s > 0 {
            BigExponentFloat::INFINITY
        } else {
            BigExponentFloat::NEGATIVE_INFINITY
        }
    };
    Some(if (ix && ib) || sb == 0 {
        BigExponentFloat::NAN
    } else if ib {
        BigExponentFloat::ZERO
    } else if ix {
        infinity(sx * sb)
    } else if sx == 0 {
        BigExponentFloat::ZERO
    } else {
        return None;
    })
}

// The class of ln(b) for a `u64` or `Rational` base, which is finite and positive (b > 1).
const POSITIVE_FINITE: Option<(i8, bool)> = Some((1, false));

impl GeneralBase<'_> {
    fn ln_class(self) -> Option<(i8, bool)> {
        match self {
            Self::BigExponentFloat(b) => ln_class(b),
            _ => POSITIVE_FINITE,
        }
    }

    // Panics if the base is a `u64` less than 2 or a `Rational` at most 1; returns the equivalent
    // `LogBase` for a `u64` power of 2.
    fn validate(self) -> Option<LogBase> {
        match self {
            Self::Unsigned(b) => {
                assert!(b > 1, "Logarithm base must be greater than 1");
                b.is_power_of_2()
                    .then(|| LogBase::PowerOf2(i64::from(b.trailing_zeros())))
            }
            Self::Rational(b) => {
                assert!(*b > 1u32, "Logarithm base must be greater than 1");
                None
            }
            Self::BigExponentFloat(_) => None,
        }
    }
}

// The rounding of an exact rational logarithm to `prec` bits with `rm`.
fn round_exact(value: &Rational, prec: u64, rm: RoundingMode) -> (BigExponentFloat, Ordering) {
    BigExponentFloat::from_rational_prec_round_ref(value, prec, rm)
}

// The result of one of `Float`'s logarithms, converted, if it neither overflows nor underflows;
// with `Exact`, the `Float` function is called with `Floor`, and the result must then be exact.
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

// The `Float` equal to the finite nonzero x, if x's exponent is within `Float`'s exponent range.
fn to_float(x: &BigExponentFloat) -> Option<Float> {
    let e = i32::try_from(x.get_exponent()?).ok()?;
    (Float::MIN_EXPONENT..=Float::MAX_EXPONENT)
        .contains(&e)
        .then(|| finite_to_float(Cow::Borrowed(x), e))
}

// An approximation of log_b(v) = L / log_2(b), where `l` is L = log_2(v) or log_2(1 + v), correctly
// rounded to `wp` bits, and an `err` as in `ziv_loop`: the two correctly rounded logarithms and the
// rounded quotient have relative errors of at most 2^(-wp) each, so the quotient's error is below
// 3.01 * 2^(-wp) relatively, and below 2^(EXP - wp + 2).
fn quotient_approx(l: BigExponentFloat, base: GeneralBase<'_>, wp: u64) -> (BigExponentFloat, u64) {
    (l.div_prec(base.log_2(wp), wp).0, wp - 2)
}

// log_b(x) for a `BigExponentFloat` x and a general base b, rounded to `prec` bits with `rm`.
pub(crate) fn log_general_helper(
    x: Cow<'_, BigExponentFloat>,
    base: GeneralBase<'_>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if let Some(base) = base.validate() {
        return log_helper(x, base, prec, rm);
    }
    if let Some(y) = quotient_special(ln_class(&x), base.ln_class()) {
        return (y, Equal);
    }
    if let Some(xf) = to_float(&x) {
        let frm = float_rm(rm);
        let result = match base {
            GeneralBase::Unsigned(b) => Some(xf.log_base_prec_round(b, prec, frm)),
            GeneralBase::Rational(b) => Some(xf.log_base_rational_base_prec_round(b, prec, frm)),
            GeneralBase::BigExponentFloat(b) => {
                to_float(b).map(|bf| xf.log_base_float_base_prec_round(&bf, prec, frm))
            }
        };
        if let Some(result) = result.and_then(|result| from_float_log(result, rm)) {
            return result;
        }
    }
    if let Some(value) = base.exact_log(&x) {
        return round_exact(&value, prec, rm);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat logarithm");
    ziv_loop(prec, rm, |wp| {
        let l = log_helper(Cow::Borrowed(&x), LogBase::Two, wp, Nearest).0;
        quotient_approx(l, base, wp)
    })
}

// log_b(1 + x) for a `BigExponentFloat` x and a general base b, rounded to `prec` bits with `rm`.
pub(crate) fn log_general_1_plus_x_helper(
    x: &BigExponentFloat,
    base: GeneralBase<'_>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if let Some(base) = base.validate() {
        return log_1_plus_x_helper(x, base, prec, rm);
    }
    // The class of ln(1 + x).
    let lx = match x {
        BigExponentFloat(NaN | Infinity { sign: false }) => None,
        BigExponentFloat(Infinity { sign: true }) => Some((1, true)),
        BigExponentFloat(Zero) => Some((0, false)),
        x => match x.partial_cmp(&-1i32).unwrap() {
            Less => None,
            Equal => Some((-1, true)),
            Greater => Some((if *x > 0u32 { 1 } else { -1 }, false)),
        },
    };
    if let Some(y) = quotient_special(lx, base.ln_class()) {
        return (y, Equal);
    }
    let BigExponentFloat(Finite {
        exponent,
        significand,
        ..
    }) = x
    else {
        unreachable!()
    };
    // This check comes before `Float`'s functions, which cover only part of the exponent range. The
    // result is never exact, since d is irrational.
    //
    // If log_b(x) = v is rational and dyadic, log_b(1 + x) = v + d with d = log_b(1 + 1/x), where
    // |d| < 2^(1 - e - l) for x's exponent e and |ln(b)| >= 2^l; every approximation of log_2(1 +
    // x) may then give exactly v, so (as in `log_1_plus_x_helper`) v is nudged toward the sign of d
    // when |d| is small enough, and otherwise d is computed and added.
    if *x >= 2u32
        && let Some(v) = base.exact_log(x)
        && let Ok(v) = BigExponentFloat::try_from(&v)
    {
        let BigExponentFloat(Finite {
            exponent: v_exponent,
            significand: v_significand,
            ..
        }) = &v
        else {
            unreachable!()
        };
        let q = significand_prec(v_significand);
        if *exponent >= Integer::from(max(prec, q) + 3) - v_exponent - base.ln_lower_exponent() {
            assert_ne!(rm, Exact, "Inexact BigExponentFloat logarithm");
            return round_nudged(&v, base.increasing(), prec, rm);
        }
        assert_ne!(rm, Exact, "Inexact BigExponentFloat logarithm");
        return ziv_loop(prec, rm, |wp| {
            let r = x.reciprocal_prec_ref(wp + 4).0;
            let l = log_1_plus_x_helper(&r, LogBase::Two, wp, Nearest).0;
            let (d, err) = quotient_approx(l, base, wp);
            (v.add_prec_ref_val(d, wp).0, err - 2)
        });
    }
    if let Some(xf) = to_float(x) {
        let frm = float_rm(rm);
        let result = match base {
            GeneralBase::Unsigned(b) => Some(xf.log_base_1_plus_x_prec_round(b, prec, frm)),
            GeneralBase::Rational(b) => {
                Some(xf.log_base_rational_base_1_plus_x_prec_round(b, prec, frm))
            }
            GeneralBase::BigExponentFloat(b) => {
                to_float(b).map(|bf| xf.log_base_float_base_1_plus_x_prec_round(&bf, prec, frm))
            }
        };
        if let Some(result) = result.and_then(|result| from_float_log(result, rm)) {
            return result;
        }
    }
    // log_b(1 + x) is rational only if 1 + x is a rational power of b. 1 + x is formed exactly when
    // that is cheap: when x's exponent is moderate, or when x is an integer using all its bits (as
    // 1 + x must be, if it is a large power of an integer base and x's exponent is large).
    if exponent.unsigned_abs_ref() <= &Natural::from(1u32 << 16)
        || finite_is_integer(x) && *exponent <= significand_prec(significand) + 64
    {
        let z = x + BigExponentFloat::ONE;
        if z != 1u32
            && let Some(value) = base.exact_log(&z)
        {
            return round_exact(&value, prec, rm);
        }
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat logarithm");
    ziv_loop(prec, rm, |wp| {
        let l = log_1_plus_x_helper(x, LogBase::Two, wp, Nearest).0;
        quotient_approx(l, base, wp)
    })
}

// log_b(x) for a `Rational` x and a general base b, rounded to `prec` bits with `rm`.
pub(crate) fn log_general_rational_helper(
    x: &Rational,
    base: GeneralBase<'_>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if let Some(base) = base.validate() {
        return log_rational_helper(x, base, prec, rm);
    }
    let lx = match x.partial_cmp(&0u32).unwrap() {
        Less => None,
        Equal => Some((-1, true)),
        Greater => Some((
            match x.partial_cmp(&1u32).unwrap() {
                Less => -1,
                Equal => 0,
                Greater => 1,
            },
            false,
        )),
    };
    if let Some(y) = quotient_special(lx, base.ln_class()) {
        return (y, Equal);
    }
    let frm = float_rm(rm);
    let result = match base {
        GeneralBase::Unsigned(b) => Some(Float::log_base_rational_prec_round_ref(x, b, prec, frm)),
        GeneralBase::Rational(b) => Some(Float::log_base_rational_rational_base_prec_round_ref(
            x, b, prec, frm,
        )),
        GeneralBase::BigExponentFloat(b) => to_float(b)
            .map(|bf| Float::log_base_rational_float_base_prec_round_ref(x, &bf, prec, frm)),
    };
    if let Some(result) = result.and_then(|result| from_float_log(result, rm)) {
        return result;
    }
    if let Some(value) = base.exact_log_rational(x) {
        return round_exact(&value, prec, rm);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat logarithm");
    ziv_loop(prec, rm, |wp| {
        let l = log_rational_helper(x, LogBase::Two, wp, Nearest).0;
        quotient_approx(l, base, wp)
    })
}

impl BigExponentFloat {
    /// Computes the base-$b$ logarithm of a [`BigExponentFloat`], where $b$ is a [`u64`], rounding
    /// the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded logarithm is less than, equal to, or greater than the exact logarithm. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,b,p,m) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p+1}$.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},b,p,m)=\text{NaN}$
    /// - $f(\infty,b,p,m)=\infty$
    /// - $f(-\infty,b,p,m)=\text{NaN}$
    /// - $f(0,b,p,m)=-\infty$
    /// - $f(1,b,p,m)=0$
    /// - $f(x,b,p,m)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::log_base_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), 64)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than 2, or if `rm` is `Exact` but the result
    /// cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_prec_round(3, 5, Floor);
    /// assert_eq!(format!("{y:?}"), "0x1.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_prec_round(3, 5, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_prec_round(3, 5, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_prec_round(3, 20, Floor);
    /// assert_eq!(format!("{y:?}"), "0x1.0abf");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_prec_round(3, 20, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x1.0abf2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_prec_round(3, 20, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x1.0abf2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_prec_round(self, base: u64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        log_general_helper(Cow::Owned(self), GeneralBase::Unsigned(base), prec, rm)
    }

    /// Computes the base-$b$ logarithm of a [`BigExponentFloat`], where $b$ is a [`u64`], rounding
    /// the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded logarithm is less than, equal to, or greater than the exact logarithm.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,b,p,m) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p+1}$.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},b,p,m)=\text{NaN}$
    /// - $f(\infty,b,p,m)=\infty$
    /// - $f(-\infty,b,p,m)=\text{NaN}$
    /// - $f(0,b,p,m)=-\infty$
    /// - $f(1,b,p,m)=0$
    /// - $f(x,b,p,m)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), 64)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than 2, or if `rm` is `Exact` but the result
    /// cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_prec_round_ref(3, 5, Floor);
    /// assert_eq!(format!("{y:?}"), "0x1.0");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_prec_round_ref(3, 5, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_prec_round_ref(3, 5, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_prec_round_ref(3, 20, Floor);
    /// assert_eq!(format!("{y:?}"), "0x1.0abf");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_prec_round_ref(3, 20, Ceiling);
    /// assert_eq!(format!("{y:?}"), "0x1.0abf2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_prec_round_ref(3, 20, Nearest);
    /// assert_eq!(format!("{y:?}"), "0x1.0abf2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_prec_round_ref(
        &self,
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        log_general_helper(Cow::Borrowed(self), GeneralBase::Unsigned(base), prec, rm)
    }

    /// Computes the base-$b$ logarithm of a [`BigExponentFloat`], where $b$ is a [`u64`], in place,
    /// rounding the result to the specified precision and with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded logarithm is less than, equal to,
    /// or greater than the exact logarithm. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p+1}$.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), 64)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than 2, or if `rm` is `Exact` but the result
    /// cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_prec_round_assign(3, 5, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x1.0");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_prec_round_assign(3, 5, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x1.1");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_prec_round_assign(3, 5, Nearest), Greater);
    /// assert_eq!(format!("{x:?}"), "0x1.1");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_prec_round_assign(3, 20, Floor), Less);
    /// assert_eq!(format!("{x:?}"), "0x1.0abf");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_prec_round_assign(3, 20, Ceiling), Greater);
    /// assert_eq!(format!("{x:?}"), "0x1.0abf2");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_prec_round_assign(3, 20, Nearest), Greater);
    /// assert_eq!(format!("{x:?}"), "0x1.0abf2");
    /// ```
    #[inline]
    pub fn log_base_prec_round_assign(
        &mut self,
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) = log_general_helper(Cow::Owned(x), GeneralBase::Unsigned(base), prec, rm);
        *self = result;
        o
    }

    /// Computes the base-$b$ logarithm of a [`BigExponentFloat`], where $b$ is a [`u64`], rounding
    /// the result to the nearest value of the specified precision. The [`BigExponentFloat`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded logarithm
    /// is less than, equal to, or greater than the exact logarithm. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,b,p) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_b
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},b,p)=\text{NaN}$
    /// - $f(\infty,b,p)=\infty$
    /// - $f(-\infty,b,p)=\text{NaN}$
    /// - $f(0,b,p)=-\infty$
    /// - $f(1,b,p)=0$
    /// - $f(x,b,p)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::log_base_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), 64)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_prec(3, 5);
    /// assert_eq!(format!("{y:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(PI).log_base_prec(3, 20);
    /// assert_eq!(format!("{y:?}"), "0x1.0abf2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::power_of_2(1000000000000i64).log_base_prec(3, 10);
    /// assert_eq!(format!("{y:?}"), "0x9.3E+9");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_prec(self, base: u64, prec: u64) -> (Self, Ordering) {
        log_general_helper(Cow::Owned(self), GeneralBase::Unsigned(base), prec, Nearest)
    }

    /// Computes the base-$b$ logarithm of a [`BigExponentFloat`], where $b$ is a [`u64`], rounding
    /// the result to the nearest value of the specified precision. The [`BigExponentFloat`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// logarithm is less than, equal to, or greater than the exact logarithm. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,b,p) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_b
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},b,p)=\text{NaN}$
    /// - $f(\infty,b,p)=\infty$
    /// - $f(-\infty,b,p)=\text{NaN}$
    /// - $f(0,b,p)=-\infty$
    /// - $f(1,b,p)=0$
    /// - $f(x,b,p)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), 64)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_prec_ref(3, 5);
    /// assert_eq!(format!("{y:?}"), "0x1.1");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(PI)).log_base_prec_ref(3, 20);
    /// assert_eq!(format!("{y:?}"), "0x1.0abf2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_prec_ref(&self, base: u64, prec: u64) -> (Self, Ordering) {
        log_general_helper(
            Cow::Borrowed(self),
            GeneralBase::Unsigned(base),
            prec,
            Nearest,
        )
    }

    /// Computes the base-$b$ logarithm of a [`BigExponentFloat`], where $b$ is a [`u64`], in place,
    /// rounding the result to the nearest value of the specified precision. An [`Ordering`] is
    /// returned, indicating whether the rounded logarithm is less than, equal to, or greater than
    /// the exact logarithm. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// x \gets \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_b
    ///   x|\rfloor-p}$.
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), 64)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_prec_assign(3, 5), Greater);
    /// assert_eq!(format!("{x:?}"), "0x1.1");
    ///
    /// let mut x = BigExponentFloat::from(PI);
    /// assert_eq!(x.log_base_prec_assign(3, 20), Greater);
    /// assert_eq!(format!("{x:?}"), "0x1.0abf2");
    /// ```
    #[inline]
    pub fn log_base_prec_assign(&mut self, base: u64, prec: u64) -> Ordering {
        let x = core::mem::replace(self, Self::ZERO);
        let (result, o) =
            log_general_helper(Cow::Owned(x), GeneralBase::Unsigned(base), prec, Nearest);
        *self = result;
        o
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes the base-$b$ logarithm of a [`Rational`], where $b$ is a [`u64`], rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded logarithm is less than, equal to, or greater
    /// than the exact logarithm. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,b,p,m) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p+1}$.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,b,p,m)=-\infty$
    /// - $f(1,b,p,m)=0$
    /// - $f(x,b,p,m)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_rational_prec_round), this
    /// function's result never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::log_base_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), 64)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than 2, or if `rm` is `Exact` but the result
    /// cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     3,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.78");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     3,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.74");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     3,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.78");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     3,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.770888");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     3,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.77088");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     3,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.77088");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_rational_prec_round(
        x: Rational,
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        log_general_rational_helper(&x, GeneralBase::Unsigned(base), prec, rm)
    }

    /// Computes the base-$b$ logarithm of a [`Rational`], where $b$ is a [`u64`], rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded logarithm is less than, equal to, or greater
    /// than the exact logarithm. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,b,p,m) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p+1}$.
    /// - If $\log_b x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\log_b x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,b,p,m)=-\infty$
    /// - $f(1,b,p,m)=0$
    /// - $f(x,b,p,m)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_rational_prec_round), this
    /// function's result never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), 64)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `base` is less than 2, or if `rm` is `Exact` but the result
    /// cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     3,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.78");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     3,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.74");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     3,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.78");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     3,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.770888");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     3,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.77088");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::log_base_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     3,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(format!("{y:?}"), "-0x0.77088");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_rational_prec_round_ref(
        x: &Rational,
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        log_general_rational_helper(x, GeneralBase::Unsigned(base), prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes the base-$b$ logarithm of a [`Rational`], where $b$ is a [`u64`], rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded logarithm is less than, equal to, or greater than the exact
    /// logarithm. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,b,p) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_b
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,b,p)=-\infty$
    /// - $f(1,b,p)=0$
    /// - $f(x,b,p)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_rational_prec_round), this
    /// function's result never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::log_base_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), 64)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::log_base_rational_prec(Rational::from_unsigneds(3u8, 5), 3, 5);
    /// assert_eq!(format!("{y:?}"), "-0x0.78");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::log_base_rational_prec(Rational::from_unsigneds(3u8, 5), 3, 20);
    /// assert_eq!(format!("{y:?}"), "-0x0.77088");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_rational_prec(x: Rational, base: u64, prec: u64) -> (Self, Ordering) {
        log_general_rational_helper(&x, GeneralBase::Unsigned(base), prec, Nearest)
    }

    /// Computes the base-$b$ logarithm of a [`Rational`], where $b$ is a [`u64`], rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded logarithm is less than, equal to, or greater than
    /// the exact logarithm. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,b,p) = \log_b x+\varepsilon.
    /// $$
    /// - If $\log_b x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\log_b x$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\log_b
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,b,p)=-\infty$
    /// - $f(1,b,p)=0$
    /// - $f(x,b,p)=\text{NaN}$ if $x<0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::log_base_rational_prec_round), this
    /// function's result never overflows or underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^2 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(x.significant_bits(), 64)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `base` is less than 2.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::log_base_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 3, 5);
    /// assert_eq!(format!("{y:?}"), "-0x0.78");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::log_base_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 3, 20);
    /// assert_eq!(format!("{y:?}"), "-0x0.77088");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn log_base_rational_prec_ref(x: &Rational, base: u64, prec: u64) -> (Self, Ordering) {
        log_general_rational_helper(x, GeneralBase::Unsigned(base), prec, Nearest)
    }
}
