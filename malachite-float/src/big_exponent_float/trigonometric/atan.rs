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
use crate::big_exponent_float::arithmetic::mul_rational::{
    finite_to_float, float_rm, from_float_result, shift_finite,
};
use crate::big_exponent_float::exponential::exp::{round_nudged, ziv_loop};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use core::cmp::{max, min};
use malachite_base::num::arithmetic::traits::{Parity, Reciprocal, Sign};
use malachite_base::num::basic::traits::{NaN as NaNTrait, One, Two, Zero as ZeroTrait};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::exponential::traits::CeilingLogBase2;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

crate_test_enum! {
    // The six inverse trigonometric functions of one argument, which share one implementation.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    InverseTrigFunction {
        Asin,
        Acos,
        Atan,
        Acot,
        Asec,
        Acsc,
    }
}

// A special value of an inverse function: NaN, or k quarter turns (k π/2, or k u/4 with a period).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SpecialAngle {
    NaN,
    Quarters(i64),
}

impl InverseTrigFunction {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Asin => "asin",
            Self::Acos => "acos",
            Self::Atan => "atan",
            Self::Acot => "acot",
            Self::Asec => "asec",
            Self::Acsc => "acsc",
        }
    }

    // The value at a NaN, an infinity, a zero, ±1, or an argument outside the domain, or `None`.
    // These are the special values of `Float`'s functions, with acot(0) = π/2: the unsigned zero
    // of a `BigExponentFloat` is taken as +0, as `arb_atan2` takes it.
    const fn special(self, negative: bool, class: ArgumentClass) -> Option<SpecialAngle> {
        use ArgumentClass as C;
        use SpecialAngle::*;
        let s = if negative { -1 } else { 1 };
        Some(match (self, class) {
            (_, C::NaN)
            | (Self::Asin | Self::Acos, C::Infinity | C::AbsGreaterThanOne)
            | (Self::Asec | Self::Acsc, C::Zero | C::AbsLessThanOne) => NaN,
            (Self::Atan, C::Infinity) | (Self::Asin | Self::Acsc, C::One) => Quarters(s),
            (Self::Acot | Self::Acsc, C::Infinity) | (Self::Asin | Self::Atan, C::Zero) => {
                Quarters(0)
            }
            (Self::Asec, C::Infinity) | (Self::Acos | Self::Acot, C::Zero) => Quarters(1),
            (Self::Acos | Self::Asec, C::One) => Quarters(if negative { 2 } else { 0 }),
            _ => return None,
        })
    }

    fn float(self, x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        match self {
            Self::Asin => x.asin_prec_round_ref(prec, rm),
            Self::Acos => x.acos_prec_round_ref(prec, rm),
            Self::Atan => x.atan_prec_round_ref(prec, rm),
            Self::Acot => x.acot_prec_round_ref(prec, rm),
            Self::Asec => x.asec_prec_round_ref(prec, rm),
            Self::Acsc => x.acsc_prec_round_ref(prec, rm),
        }
    }

    fn float_rational(self, x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        match self {
            Self::Asin => Float::asin_rational_prec_round_ref(x, prec, rm),
            Self::Acos => Float::acos_rational_prec_round_ref(x, prec, rm),
            Self::Atan => Float::atan_rational_prec_round_ref(x, prec, rm),
            Self::Acot => Float::acot_rational_prec_round_ref(x, prec, rm),
            Self::Asec => Float::asec_rational_prec_round_ref(x, prec, rm),
            Self::Acsc => Float::acsc_rational_prec_round_ref(x, prec, rm),
        }
    }

    fn float_with_period(
        self,
        x: &Float,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        match self {
            Self::Asin => x.asin_with_period_prec_round_ref(u, prec, rm),
            Self::Acos => x.acos_with_period_prec_round_ref(u, prec, rm),
            Self::Atan => x.atan_with_period_prec_round_ref(u, prec, rm),
            Self::Acot => x.acot_with_period_prec_round_ref(u, prec, rm),
            Self::Asec => x.asec_with_period_prec_round_ref(u, prec, rm),
            Self::Acsc => x.acsc_with_period_prec_round_ref(u, prec, rm),
        }
    }

    fn float_with_period_rational(
        self,
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        match self {
            Self::Asin => Float::asin_with_period_rational_prec_round_ref(x, u, prec, rm),
            Self::Acos => Float::acos_with_period_rational_prec_round_ref(x, u, prec, rm),
            Self::Atan => Float::atan_with_period_rational_prec_round_ref(x, u, prec, rm),
            Self::Acot => Float::acot_with_period_rational_prec_round_ref(x, u, prec, rm),
            Self::Asec => Float::asec_with_period_rational_prec_round_ref(x, u, prec, rm),
            Self::Acsc => Float::acsc_with_period_rational_prec_round_ref(x, u, prec, rm),
        }
    }

    // The decomposition of f(x) as k π/2 + σ g(s), with g = asin or atan of a tiny s (|s| <=
    // 2^(-10)), for a finite nonzero x in the domain whose exponent e satisfies e <= -10 (`tiny`)
    // or e >= 11: asin x = asin x, acos x = π/2 - asin x, atan x = atan x or ±π/2 - atan(1/x),
    // acot x = ±π/2 - atan x or atan(1/x), asec x = π/2 - asin(1/x), and acsc x = asin(1/x).
    // `None` for an argument outside the domain.
    const fn decomposition(self, negative: bool, tiny: bool) -> Option<Decomposition> {
        let s = if negative { -1 } else { 1 };
        let (quarters, sigma, asin, reciprocal) = match (self, tiny) {
            (Self::Asin, true) => (0, 1, true, false),
            (Self::Acos, true) => (1, -1, true, false),
            (Self::Atan, true) => (0, 1, false, false),
            (Self::Acot, true) => (s, -1, false, false),
            (Self::Atan, false) => (s, -1, false, true),
            (Self::Acot, false) => (0, 1, false, true),
            (Self::Asec, false) => (1, -1, true, true),
            (Self::Acsc, false) => (0, 1, true, true),
            _ => return None,
        };
        Some(Decomposition {
            quarters,
            sigma,
            asin,
            reciprocal,
        })
    }
}

// k π/2 + σ g(s), with g = asin (if `asin`) or atan, and s the argument or (if `reciprocal`) its
// reciprocal.
#[derive(Clone, Copy, Debug)]
struct Decomposition {
    quarters: i64,
    sigma: i64,
    asin: bool,
    reciprocal: bool,
}

// The class of an argument, for the special values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ArgumentClass {
    NaN,
    Infinity,
    Zero,
    One,
    AbsLessThanOne,
    AbsGreaterThanOne,
}

fn bef_class(x: &BigExponentFloat) -> ArgumentClass {
    match x {
        BigExponentFloat(NaN) => ArgumentClass::NaN,
        BigExponentFloat(Infinity { .. }) => ArgumentClass::Infinity,
        BigExponentFloat(Zero) => ArgumentClass::Zero,
        _ => match x.partial_cmp_abs(&BigExponentFloat::ONE).unwrap() {
            Equal => ArgumentClass::One,
            Less => ArgumentClass::AbsLessThanOne,
            Greater => ArgumentClass::AbsGreaterThanOne,
        },
    }
}

fn rational_class(x: &Rational) -> ArgumentClass {
    if *x == 0u32 {
        return ArgumentClass::Zero;
    }
    match x.partial_cmp_abs(&Rational::ONE).unwrap() {
        Equal => ArgumentClass::One,
        Less => ArgumentClass::AbsLessThanOne,
        Greater => ArgumentClass::AbsGreaterThanOne,
    }
}

// A tiny argument s of asin or atan, |s| < 2^`exponent` <= 2^(-9), computed from the inputs.
#[derive(Clone, Copy)]
enum TinySource<'a> {
    BigExponentFloat(&'a BigExponentFloat),
    Rational(&'a Rational),
    Reciprocal(&'a BigExponentFloat),
    RationalReciprocal(&'a Rational),
    Quotient(&'a BigExponentFloat, &'a BigExponentFloat),
}

impl TinySource<'_> {
    // s, with a relative error of at most 2^(-w).
    fn approx(&self, w: u64) -> BigExponentFloat {
        match *self {
            TinySource::BigExponentFloat(x) => x.clone(),
            TinySource::Rational(q) => BigExponentFloat::from_rational_prec_ref(q, w).0,
            TinySource::Reciprocal(x) => x.reciprocal_prec_ref(w).0,
            TinySource::RationalReciprocal(q) => {
                BigExponentFloat::from_rational_prec(q.reciprocal(), w).0
            }
            TinySource::Quotient(a, b) => a.div_prec_ref_ref(b, w).0,
        }
    }

    // s exactly, if it is a `BigExponentFloat`.
    fn exact(&self) -> Option<BigExponentFloat> {
        match *self {
            TinySource::BigExponentFloat(x) => Some(x.clone()),
            TinySource::Rational(q) => BigExponentFloat::try_from(q).ok(),
            TinySource::Reciprocal(x) => {
                (x.get_prec() == Some(1)).then(|| x.reciprocal_prec_ref(1).0)
            }
            TinySource::RationalReciprocal(q) => BigExponentFloat::try_from(q.reciprocal()).ok(),
            TinySource::Quotient(a, b) => {
                (b.get_prec() == Some(1)).then(|| a.div_prec_ref_ref(b, a.get_prec().unwrap()).0)
            }
        }
    }
}

// Approximations of atan s and asin s for a nonzero s with |s| <= 2^(-9), given with a relative
// error of at most 2^(-ks), and the bits of their relative errors. atan s = s - s^3/3 + s^5/5 - ...
// is summed with `w`-bit arithmetic until a term is below 2^(-w - 2)|s|; the terms decrease by a
// factor of at least 2^18, so with n terms the sum's relative error is below (n + 4) 2^(2 - w).
// asin s = atan(s / sqrt(1 - s^2)), whose argument adds a relative error below 2^(2 - w). Since |s
// g'(s) / g(s)| < 1.01 for both functions at such an s, the error of s carries over unmagnified.
fn tiny_atan_asin(s: &BigExponentFloat, ks: u64, w: u64, asin: bool) -> (BigExponentFloat, u64) {
    let t = if asin {
        let one_minus = BigExponentFloat::ONE
            .sub_prec_ref_ref(&s.square_prec_ref(w + 4).0, w + 4)
            .0;
        s.div_prec_ref_val(one_minus.sqrt_prec(w + 4).0, w).0
    } else {
        s.clone()
    };
    let t2 = t.square_prec_ref(w).0;
    let mut power = t.clone();
    let mut sum = t.clone();
    let limit = t.get_exponent().unwrap() - Integer::from(w + 2);
    let mut n: u64 = 1;
    loop {
        power.mul_prec_assign_ref(&t2, w);
        let term = power
            .div_prec_ref_val(BigExponentFloat::from((n << 1) + 1), w)
            .0;
        sum = if n.odd() {
            sum.sub_prec_ref_ref(&term, w).0
        } else {
            sum.add_prec_ref_ref(&term, w).0
        };
        n += 1;
        if *term.get_exponent().unwrap() < limit {
            break;
        }
    }
    let k = w.saturating_sub((n + 4).ceiling_log_base_2() + 3);
    (sum, min(k, ks.saturating_sub(1)))
}

// An approximation of k π/2 + σ g(s) (or, with a period u, of the same angle times u/(2π)), and
// an `err` as in `ziv_loop`. g(s) has a relative error below 2^(-k) for k from `tiny_atan_asin`; a
// period multiplies it by u/(2π), with 2π rounded to w bits, adding below 2^(1 - w). With k != 0,
// the exact k π/2 (with π rounded to w bits) or k u/4 is added, and since |g(s)| <= 2^(-8) is far
// smaller, there is no cancellation: the sum's relative error is below 2^(2 - k') for the term's
// k'. A relative error below 2^(-j) gives `err` j - 1.
fn decomposed_approx(
    d: Decomposition,
    s: TinySource<'_>,
    u: Option<u64>,
    wp: u64,
) -> (BigExponentFloat, u64) {
    let w = wp + 12;
    let sa = s.approx(w);
    let (g, k) = tiny_atan_asin(&sa, w, w, d.asin);
    let (term, k) = match u {
        None => (g, k),
        Some(u) => {
            let two_pi = BigExponentFloat::from(Float::pi_prec(w).0) << 1u32;
            (
                g.mul_prec_val_ref(&BigExponentFloat::from(u), w)
                    .0
                    .div_prec_val_ref(&two_pi, w)
                    .0,
                min(k, w).saturating_sub(2),
            )
        }
    };
    let term = if d.sigma < 0 { -term } else { term };
    if d.quarters == 0 {
        return (term, k.saturating_sub(1));
    }
    let base = quarter_turns(d.quarters, u, w);
    (
        base.add_prec_val_ref(&term, w).0,
        min(k, w).saturating_sub(4),
    )
}

// k π/2, with π rounded to `w` bits, or, with a period u, k u/4 exactly.
fn quarter_turns(k: i64, u: Option<u64>, w: u64) -> BigExponentFloat {
    match u {
        None => {
            let half_pi = BigExponentFloat::from(Float::pi_prec(w).0) >> 1u32;
            half_pi * BigExponentFloat::from(k)
        }
        Some(u) => BigExponentFloat::from(Integer::from(u) * Integer::from(k)) >> 2u32,
    }
}

// k π/2 rounded to `prec` bits with `rm`, or k u/4 with a period u.
fn round_quarter_turns(
    k: i64,
    u: Option<u64>,
    prec: u64,
    rm: RoundingMode,
    name: &str,
) -> (BigExponentFloat, Ordering) {
    if k == 0 {
        return (BigExponentFloat::ZERO, Equal);
    }
    if let Some(u) = u {
        BigExponentFloat::from_rational_prec_round(
            Rational::from(Integer::from(u) * Integer::from(k)) >> 2u32,
            prec,
            rm,
        )
    } else {
        assert_ne!(rm, Exact, "Inexact BigExponentFloat {name}");
        let negative = k < 0;
        let (pi, o) = BigExponentFloat::pi_prec_round(prec, if negative { -rm } else { rm });
        let pi = shift_finite(pi, &Integer::from(i64::from(k.unsigned_abs() == 2) - 1));
        if negative {
            (-pi, o.reverse())
        } else {
            (pi, o)
        }
    }
}

// For an angle k π/2 + σ g(s) that is within a tiny fraction of an ulp of a simple exact value v,
// the pair (v, up), where the angle is v plus (if `up`) or minus a positive amount small enough for
// `round_nudged`. With no period and k = 0, v = s, when s is exact: g(s) - s is below |s|^3/3 in
// absolute value, with the sign of s for asin and the opposite sign for atan. With a period and k
// != 0, v = k u/4: the term σ g(s) u/(2π) is below 2^(EXP(s) + bits(u) - 2), and v's exponent is
// at least bits(u) - 2. In each case the condition below makes the amount smaller than 2^(EXP(v) -
// max(p, q) - 2), with q the precision of v. (The other cases, π times a nonzero rational plus or
// minus a tiny amount, and a tiny multiple of 1/π, are irrational, and no simple value is close.)
fn decomposed_nudge(
    d: Decomposition,
    s: TinySource<'_>,
    s_negative: bool,
    s_exponent: &Integer,
    u: Option<u64>,
    prec: u64,
) -> Option<(BigExponentFloat, bool)> {
    let s_up = (d.sigma > 0) != s_negative;
    match u {
        None if d.quarters == 0 => {
            let v = s.exact()?;
            let q = v.get_prec().unwrap();
            let up = if d.asin { !s_negative } else { s_negative };
            (max(prec, q) + 4 <= -(s_exponent << 1u32)).then_some((v, up))
        }
        Some(u) if d.quarters != 0 => {
            let v = quarter_turns(d.quarters, Some(u), 0);
            let q = v.get_prec().unwrap();
            (max(prec, q) + 5 <= -s_exponent).then_some((v, s_up))
        }
        _ => None,
    }
}

// Whether an exponent e has |e| large enough for the decomposition: e <= -10 or e >= 11.
fn decomposable(e: &Integer) -> Option<bool> {
    if *e <= -10i32 {
        Some(true)
    } else if *e >= 11u32 {
        Some(false)
    } else {
        None
    }
}

// The angle k π/2 + σ g(s) for a decomposable argument, rounded to `prec` bits with `rm`, with an
// optional period: `round_nudged` when the angle is that close to a simple value, and otherwise a
// Ziv loop. The angle is irrational (or, with a period, a nonzero rational plus an irrational), so
// `Exact` is rejected.
fn round_decomposed(
    f: InverseTrigFunction,
    d: Decomposition,
    s: TinySource<'_>,
    s_negative: bool,
    s_exponent: &Integer,
    u: Option<u64>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(rm, Exact, "Inexact BigExponentFloat {}", f.name());
    if let Some((v, up)) = decomposed_nudge(d, s, s_negative, s_exponent, u, prec) {
        return round_nudged(&v, up, prec, rm);
    }
    ziv_loop(prec, rm, |wp| decomposed_approx(d, s, u, wp))
}

// The argument x of an inverse function, as a `BigExponentFloat` or a `Rational`.
#[derive(Clone, Copy)]
enum InverseArgument<'a> {
    BigExponentFloat(&'a BigExponentFloat),
    Rational(&'a Rational),
}

impl InverseArgument<'_> {
    fn exponent(&self) -> Integer {
        match self {
            InverseArgument::BigExponentFloat(x) => x.get_exponent().unwrap().clone(),
            InverseArgument::Rational(q) => Integer::from(q.floor_log_base_2_abs()) + Integer::ONE,
        }
    }

    // The tiny argument s: x itself, or its reciprocal, with s's exponent bound.
    fn tiny(&self, reciprocal: bool) -> (TinySource<'_>, Integer) {
        let e = self.exponent();
        match (self, reciprocal) {
            (InverseArgument::BigExponentFloat(x), false) => (TinySource::BigExponentFloat(x), e),
            (InverseArgument::Rational(q), false) => (TinySource::Rational(q), e),
            // |x| >= 2^(e - 1), so |1/x| <= 2^(1 - e) < 2^(2 - e)
            (InverseArgument::BigExponentFloat(x), true) => {
                (TinySource::Reciprocal(x), Integer::TWO - e)
            }
            (InverseArgument::Rational(q), true) => {
                (TinySource::RationalReciprocal(q), Integer::TWO - e)
            }
        }
    }
}

// The radicand r of `round_near_one`: (1 - x)/2 for acos and (x - 1)/(2x) for asec.
enum Radicand {
    Exact(BigExponentFloat),
    Quotient(BigExponentFloat, BigExponentFloat),
    Rational(Rational),
}

impl Radicand {
    fn of_big_exponent_float(f: InverseTrigFunction, x: &BigExponentFloat) -> Self {
        if f == InverseTrigFunction::Acos {
            Self::Exact((BigExponentFloat::ONE - x) >> 1u32)
        } else {
            Self::Quotient(x - BigExponentFloat::ONE, x << 1u32)
        }
    }

    fn of_rational(f: InverseTrigFunction, x: &Rational) -> Self {
        Self::Rational(if f == InverseTrigFunction::Acos {
            (Rational::ONE - x) >> 1u32
        } else {
            ((x - Rational::ONE) / x) >> 1u32
        })
    }

    // r, with a relative error of at most 2^(-w)
    fn approx(&self, w: u64) -> BigExponentFloat {
        match self {
            Self::Exact(r) => r.clone(),
            Self::Quotient(n, d) => n.div_prec_ref_ref(d, w).0,
            Self::Rational(r) => BigExponentFloat::from_rational_prec_ref(r, w).0,
        }
    }
}

// acos x or asec x for an x close to 1, where the result is tiny: acos x = 2 asin(sqrt((1 - x)/2)),
// and asec x = acos(1/x) = 2 asin(sqrt((x - 1)/(2x))). The radicand is formed exactly (from the
// exact difference x - 1) or with a relative error below 2^(-w - 2), its square root rounded to w
// bits (a relative error below 2^(1 - w) in all), and asin taken as in `tiny_atan_asin`; with a
// period, the result is multiplied by u/(2π) as in `decomposed_approx`. The result is irrational,
// so `Exact` is rejected.
fn round_near_one(
    name: &str,
    radicand: &Radicand,
    u: Option<u64>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(rm, Exact, "Inexact BigExponentFloat {name}");
    ziv_loop(prec, rm, |wp| {
        let w = wp + 12;
        let s = radicand.approx(w + 2).sqrt_prec(w).0;
        let (g, k) = tiny_atan_asin(&s, w - 1, w, true);
        let g = g << 1u32;
        match u {
            None => (g, k.saturating_sub(1)),
            Some(u) => {
                let two_pi = BigExponentFloat::from(Float::pi_prec(w).0) << 1u32;
                (
                    g.mul_prec_val_ref(&BigExponentFloat::from(u), w)
                        .0
                        .div_prec_val_ref(&two_pi, w)
                        .0,
                    min(k, w).saturating_sub(3),
                )
            }
        }
    })
}

// Whether acos x or asec x (for a positive x in the domain) is computed by `round_near_one`, when
// not using `Float`'s functions: |x - 1| <= 2^(-10). The exponent is checked first: x - 1 is
// computed exactly, which for an x far from 1 would take as many bits as the exponent difference.
fn near_one(f: InverseTrigFunction, x: &BigExponentFloat) -> bool {
    matches!(f, InverseTrigFunction::Acos | InverseTrigFunction::Asec)
        && *x > 0u32
        && x.get_exponent().is_some_and(|e| *e == 0u32 || *e == 1u32)
        && match (x - BigExponentFloat::ONE).get_exponent() {
            Some(e) => *e <= -10i32,
            None => false,
        }
}

// The exponent of the finite nonzero x, if it is within `Float`'s exponent range.
fn float_exponent(x: &BigExponentFloat) -> Option<i32> {
    let e = i32::try_from(x.get_exponent().unwrap()).ok()?;
    (Float::MIN_EXPONENT..=Float::MAX_EXPONENT)
        .contains(&e)
        .then_some(e)
}

// The result of one of `Float`'s functions, converted if it neither overflowed nor underflowed;
// with `Exact`, the `Float` function is called with `Floor`, and the result must then be exact.
fn from_float_inverse(
    result: (Float, Ordering),
    name: &str,
    rm: RoundingMode,
) -> Option<(BigExponentFloat, Ordering)> {
    let (y, o) = from_float_result(result, None)?;
    assert!(rm != Exact || o == Equal, "Inexact BigExponentFloat {name}");
    Some((y, o))
}

// f(x) (with a period u, f(x) u/(2π)), rounded to `prec` bits with `rm`.
//
// The special values are those of `Float`'s functions (see `InverseTrigFunction::special`); with a
// period of 0 every result in the domain is 0. If `use_float` and x's exponent is within `Float`'s
// range, `Float`'s function is used unless its result underflows (possible only for an x with
// billions of bits). Otherwise, an x close to 1 for acos or asec is handled by `round_near_one`,
// and any other x has an exponent of at least 2^30 in absolute value (or, if not `use_float`, of at
// least 10), and is handled by `round_decomposed`; without `use_float`, the remaining arguments
// still use `Float`'s functions.
pub(crate) fn inverse_trig_helper(
    f: InverseTrigFunction,
    x: &BigExponentFloat,
    u: Option<u64>,
    prec: u64,
    rm: RoundingMode,
    use_float: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    let negative = *x < 0u32;
    if let Some(special) = f.special(negative, bef_class(x)) {
        return match special {
            SpecialAngle::NaN => (BigExponentFloat::NAN, Equal),
            _ if u == Some(0) => (BigExponentFloat::ZERO, Equal),
            SpecialAngle::Quarters(k) => round_quarter_turns(k, u, prec, rm, f.name()),
        };
    }
    if u == Some(0) {
        return (BigExponentFloat::ZERO, Equal);
    }
    let e = x.get_exponent().unwrap();
    let decomposable = decomposable(e);
    if (use_float || decomposable.is_none() && !near_one(f, x))
        && let Some(ex) = float_exponent(x)
    {
        let xf = finite_to_float(Cow::Borrowed(x), ex);
        let frm = float_rm(rm);
        let result = match u {
            None => f.float(&xf, prec, frm),
            Some(u) => f.float_with_period(&xf, u, prec, frm),
        };
        if let Some(result) = from_float_inverse(result, f.name(), rm) {
            return result;
        }
    }
    if near_one(f, x) {
        return round_near_one(
            f.name(),
            &Radicand::of_big_exponent_float(f, x),
            u,
            prec,
            rm,
        );
    }
    let tiny = decomposable.unwrap();
    let d = f.decomposition(negative, tiny).unwrap();
    let arg = InverseArgument::BigExponentFloat(x);
    let (s, s_exponent) = arg.tiny(d.reciprocal);
    round_decomposed(f, d, s, negative, &s_exponent, u, prec, rm)
}

// f(x) for a `Rational` x (with a period u, f(x) u/(2π)), rounded to `prec` bits with `rm`: as in
// `inverse_trig_helper`, with `Float`'s `Rational` functions. Their results underflow only for an x
// with billions of bits; such an x is either tiny or huge, and handled by `round_decomposed`, or
// within a tiny distance of 1, for acos or asec, and handled by `round_near_one`, with the radicand
// formed exactly.
pub(crate) fn inverse_trig_rational_helper(
    f: InverseTrigFunction,
    x: &Rational,
    u: Option<u64>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    let negative = *x < 0u32;
    if let Some(special) = f.special(negative, rational_class(x)) {
        return match special {
            SpecialAngle::NaN => (BigExponentFloat::NAN, Equal),
            _ if u == Some(0) => (BigExponentFloat::ZERO, Equal),
            SpecialAngle::Quarters(k) => round_quarter_turns(k, u, prec, rm, f.name()),
        };
    }
    if u == Some(0) {
        return (BigExponentFloat::ZERO, Equal);
    }
    let frm = float_rm(rm);
    let result = match u {
        None => f.float_rational(x, prec, frm),
        Some(u) => f.float_with_period_rational(x, u, prec, frm),
    };
    if let Some(result) = from_float_inverse(result, f.name(), rm) {
        return result;
    }
    let arg = InverseArgument::Rational(x);
    if let Some(tiny) = decomposable(&arg.exponent()) {
        let d = f.decomposition(negative, tiny).unwrap();
        let (s, s_exponent) = arg.tiny(d.reciprocal);
        return round_decomposed(f, d, s, negative, &s_exponent, u, prec, rm);
    }
    // The only other arguments whose results underflow are those within a tiny distance of 1, for
    // acos and asec.
    round_near_one(f.name(), &Radicand::of_rational(f, x), u, prec, rm)
}

crate_test_fn! {
    // f(x) (with a period u, f(x) u/(2π)) rounded to `prec` bits with `rm`, using the
    // decomposition into a tiny asin or atan, and the near-1 formula, wherever they apply (for an
    // exponent of at least 10 in absolute value, or within 2^(-10) of 1), rather than only beyond
    // `Float`'s range. Used to check `inverse_trig_helper`.
    inverse_trig_prec_round_generic(
        f: InverseTrigFunction,
        x: &BigExponentFloat,
        u: Option<u64>,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        inverse_trig_helper(f, x, u, prec, rm, false)
    }
}

// The special value of atan2(y, x) (with a period u, scaled by u/(2π)), as k quarter turns or k/2
// quarter turns (for the diagonals), or `None` for finite nonzero y and x. These are the values of
// `arb_atan2`, the argument of x + iy, which takes the unsigned zero as +0 (so that atan2(0, x) =
// π for a negative x, and atan2(0, 0) = 0), and, for infinite arguments, those of `Float`'s atan2.
const fn atan2_special(y: &BigExponentFloat, x: &BigExponentFloat) -> Option<Option<i64>> {
    // the result is in eighths of a turn (k π/4), or NaN (None)
    let ys = y.is_sign_negative();
    let xs = x.is_sign_negative();
    let sy = if ys { -1 } else { 1 };
    Some(match (y, x) {
        (BigExponentFloat(NaN), _) | (_, BigExponentFloat(NaN)) => None,
        (BigExponentFloat(Infinity { .. }), BigExponentFloat(Infinity { .. })) => {
            Some(sy * if xs { 3 } else { 1 })
        }
        (BigExponentFloat(Infinity { .. }), _) => Some(sy << 1),
        (_, BigExponentFloat(Infinity { .. })) | (BigExponentFloat(Zero), _) => {
            Some(if xs { sy << 2 } else { 0 })
        }
        (_, BigExponentFloat(Zero)) => Some(sy << 1),
        _ => return None,
    })
}

// k π/4 rounded to `prec` bits with `rm`, or, with a period u, k u/8.
fn round_eighth_turns(
    k: i64,
    u: Option<u64>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    if k == 0 {
        return (BigExponentFloat::ZERO, Equal);
    }
    if let Some(u) = u {
        BigExponentFloat::from_rational_prec_round(
            Rational::from(Integer::from(u) * Integer::from(k)) >> 3u32,
            prec,
            rm,
        )
    } else {
        assert_ne!(rm, Exact, "Inexact BigExponentFloat atan2");
        let negative = k < 0;
        let m = k.unsigned_abs();
        let rm = if negative { -rm } else { rm };
        let (z, o) = if m == 3 {
            // 3π/4, which is irrational: a Ziv loop on 3π rounded to wp + 4 bits, a relative
            // error below 2^(-wp - 3)
            ziv_loop(prec, rm, |wp| {
                let pi = BigExponentFloat::from(Float::pi_prec(wp + 4).0);
                (
                    pi.mul_prec_val_ref(
                        &const { BigExponentFloat::const_from_unsigned(3) },
                        wp + 4,
                    )
                    .0 >> 2u32,
                    wp + 2,
                )
            })
        } else {
            // π/4, π/2, or π
            let (pi, o) = BigExponentFloat::pi_prec_round(prec, rm);
            (
                shift_finite(pi, &Integer::from(i64::from(m.trailing_zeros()) - 2)),
                o,
            )
        };
        if negative { (-z, o.reverse()) } else { (z, o) }
    }
}

// atan2(y, x) (with a period u, atan2(y, x) u/(2π)), rounded to `prec` bits with `rm`.
//
// The special values are those of `atan2_special`; with a period of 0 every finite result is 0. For
// finite nonzero y and x, both are scaled by the same power of 2, so that the larger exponent is 0,
// and if the smaller one is then within `Float`'s range (and `use_float`), `Float`'s atan2 is used,
// unless its result underflows. Otherwise the exponents differ by at least 2^30 (or, if not
// `use_float`, by at least 11), and atan2(y, x) = atan(y/x) or ±π + atan(y/x) when |y| is the
// smaller (for a positive or negative x), and ±π/2 - atan(x/y) otherwise, with the tiny
// quotient's atan taken by `round_decomposed`.
pub(crate) fn atan2_helper(
    y: &BigExponentFloat,
    x: &BigExponentFloat,
    u: Option<u64>,
    prec: u64,
    rm: RoundingMode,
    use_float: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if let Some(special) = atan2_special(y, x) {
        return match special {
            None => (BigExponentFloat::NAN, Equal),
            Some(_) if u == Some(0) => (BigExponentFloat::ZERO, Equal),
            Some(k) => round_eighth_turns(k, u, prec, rm),
        };
    }
    if u == Some(0) {
        return (BigExponentFloat::ZERO, Equal);
    }
    let ey = y.get_exponent().unwrap();
    let ex = x.get_exponent().unwrap();
    let gap = ey - ex;
    let far = gap <= -11i32 || gap >= 11u32;
    if use_float || !far {
        let e = max(ey, ex).clone();
        let ys = shift_finite(y.clone(), &-&e);
        let xs = shift_finite(x.clone(), &-&e);
        if let (Some(eys), Some(exs)) = (float_exponent(&ys), float_exponent(&xs)) {
            let yf = finite_to_float(Cow::Owned(ys), eys);
            let xf = finite_to_float(Cow::Owned(xs), exs);
            let frm = float_rm(rm);
            let result = match u {
                None => yf.atan2_prec_round_ref_ref(&xf, prec, frm),
                Some(u) => yf.atan2_with_period_prec_round_ref_ref(&xf, u, prec, frm),
            };
            if let Some(result) = from_float_inverse(result, "atan2", rm) {
                return result;
            }
        }
    }
    let y_negative = *y < 0u32;
    let x_negative = *x < 0u32;
    if gap < 0u32 {
        // |y| << |x|: atan(y/x), or ±π + atan(y/x) for a negative x
        let quarters = if x_negative {
            if y_negative { -2 } else { 2 }
        } else {
            0
        };
        let d = Decomposition {
            quarters,
            sigma: 1,
            asin: false,
            reciprocal: false,
        };
        let s_exponent = gap + Integer::ONE;
        round_decomposed(
            InverseTrigFunction::Atan,
            d,
            TinySource::Quotient(y, x),
            y_negative != x_negative,
            &s_exponent,
            u,
            prec,
            rm,
        )
    } else {
        // |y| >> |x|: ±π/2 - atan(x/y)
        let d = Decomposition {
            quarters: if y_negative { -1 } else { 1 },
            sigma: -1,
            asin: false,
            reciprocal: false,
        };
        let s_exponent = Integer::ONE - gap;
        round_decomposed(
            InverseTrigFunction::Atan,
            d,
            TinySource::Quotient(x, y),
            y_negative != x_negative,
            &s_exponent,
            u,
            prec,
            rm,
        )
    }
}

// atan2(y, x) for `Rational`s y and x (with a period u, scaled by u/(2π)), rounded to `prec` bits
// with `rm`: `Float`'s function, whose result underflows only when x is positive and |y/x| is below
// 2^(-2^30), possible only for arguments with billions of bits; the tiny quotient y/x is then
// formed exactly and its atan taken by `round_decomposed`.
pub(crate) fn atan2_rational_helper(
    y: &Rational,
    x: &Rational,
    u: Option<u64>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    let frm = float_rm(rm);
    let result = match u {
        None => Float::atan2_rational_prec_round_ref(y, x, prec, frm),
        Some(u) => Float::atan2_with_period_rational_prec_round_ref(y, x, u, prec, frm),
    };
    if let Some(result) = from_float_inverse(result, "atan2", rm) {
        return result;
    }
    let q = y / x;
    let arg = InverseArgument::Rational(&q);
    let d = Decomposition {
        quarters: 0,
        sigma: 1,
        asin: false,
        reciprocal: false,
    };
    round_decomposed(
        InverseTrigFunction::Atan,
        d,
        TinySource::Rational(&q),
        q.sign() == Less,
        &arg.exponent(),
        u,
        prec,
        rm,
    )
}

crate_test_fn! {
    // atan2(y, x) (with a period u) rounded to `prec` bits with `rm`, using the decomposition
    // wherever the exponents differ by at least 11, rather than only beyond `Float`'s range. Used
    // to check `atan2_helper`.
    atan2_prec_round_generic(
        y: &BigExponentFloat,
        x: &BigExponentFloat,
        u: Option<u64>,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        atan2_helper(y, x, u, prec, rm, false)
    }
}

impl BigExponentFloat {
    /// Computes $\arctan x$, the arctangent of a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded arctangent is
    /// less than, equal to, or greater than the exact arctangent. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arctan x+\varepsilon.
    /// $$
    /// - If $\arctan x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arctan x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arctan x|\rfloor-p+1}$.
    /// - If $\arctan x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arctan x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm\pi/2$, rounded
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::atan_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is nonzero (every nonzero
    /// result is irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "0.3587704");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.35877085");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "0.35877085");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Atan, &self, None, prec, rm, true)
    }

    /// Computes $\arctan x$, the arctangent of a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded arctangent is
    /// less than, equal to, or greater than the exact arctangent. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arctan x+\varepsilon.
    /// $$
    /// - If $\arctan x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arctan x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arctan x|\rfloor-p+1}$.
    /// - If $\arctan x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arctan x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm\pi/2$, rounded
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is nonzero (every nonzero
    /// result is irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "0.3587704");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.35877085");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "0.35877085");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Atan, self, None, prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with $\arctan x$, its arctangent, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded arctangent is less than, equal to, or greater than the exact
    /// arctangent. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arctan x+\varepsilon.
    /// $$
    /// - If $\arctan x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arctan x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arctan x|\rfloor-p+1}$.
    /// - If $\arctan x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arctan x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm\pi/2$, rounded
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is nonzero (every nonzero
    /// result is irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.344");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.359");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.359");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.3587704");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.35877085");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.35877085");
    /// ```
    #[inline]
    pub fn atan_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Atan, self, None, prec, rm, true);
        *self = result;
        o
    }

    /// Computes $\arctan x$, the arctangent of a [`BigExponentFloat`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded arctangent is less than, equal
    /// to, or greater than the exact arctangent. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arctan x+\varepsilon.
    /// $$
    /// - If $\arctan x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arctan x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arctan
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm\pi/2$, rounded
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::atan_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_prec(5);
    /// assert_eq!(y.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_prec(20);
    /// assert_eq!(y.to_string(), "0.35877085");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan_prec(self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Atan, &self, None, prec, Nearest, true)
    }

    /// Computes $\arctan x$, the arctangent of a [`BigExponentFloat`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded arctangent is less than, equal
    /// to, or greater than the exact arctangent. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arctan x+\varepsilon.
    /// $$
    /// - If $\arctan x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arctan x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arctan
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm\pi/2$, rounded
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_prec_ref(5);
    /// assert_eq!(y.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_prec_ref(20);
    /// assert_eq!(y.to_string(), "0.35877085");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Atan, self, None, prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] with $\arctan x$, its arctangent, rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is returned, indicating whether
    /// the rounded arctangent is less than, equal to, or greater than the exact arctangent.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets
    /// the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arctan x+\varepsilon.
    /// $$
    /// - If $\arctan x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arctan x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arctan
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm\pi/2$, rounded
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "0.359");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "0.35877085");
    /// ```
    #[inline]
    pub fn atan_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Atan, self, None, prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes $\arctan x$, the arctangent of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arctangent is less than, equal to, or greater than the exact arctangent. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arctan x+\varepsilon.
    /// $$
    /// - If $\arctan x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arctan x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arctan x|\rfloor-p+1}$.
    /// - If $\arctan x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arctan x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::atan_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is nonzero (every nonzero
    /// result is irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::atan_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(y.to_string(), "0.31");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::atan_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(y.to_string(), "0.32175016");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.3217506");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.3217506");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Atan, &x, None, prec, rm)
    }

    /// Computes $\arctan x$, the arctangent of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arctangent is less than, equal to, or greater than the exact arctangent. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arctan x+\varepsilon.
    /// $$
    /// - If $\arctan x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arctan x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arctan x|\rfloor-p+1}$.
    /// - If $\arctan x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arctan x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is nonzero (every nonzero
    /// result is irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::atan_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.31");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.32175016");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.3217506");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.3217506");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Atan, x, None, prec, rm)
    }

    /// Computes $\arctan x$, the arctangent of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded arctangent
    /// is less than, equal to, or greater than the exact arctangent. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arctan x+\varepsilon.
    /// $$
    /// - If $\arctan x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arctan x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arctan
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::atan_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
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
    /// let (y, o) = BigExponentFloat::atan_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.3217506");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Atan, &x, None, prec, Nearest)
    }

    /// Computes $\arctan x$, the arctangent of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// arctangent is less than, equal to, or greater than the exact arctangent. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arctan x+\varepsilon.
    /// $$
    /// - If $\arctan x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arctan x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arctan
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
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
    /// let (y, o) = BigExponentFloat::atan_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::atan_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.3217506");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Atan, x, None, prec, Nearest)
    }

    /// Computes $u\arctan x/(2\pi)$, the arctangent of a [`BigExponentFloat`] measured in $u$ths of
    /// a turn, rounding the result to the specified precision and with the specified rounding mode.
    /// The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arctangent is less than, equal to, or greater than the exact arctangent.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\arctan x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arctan x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arctan x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |u\arctan x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\arctan x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\arctan x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(\pm\infty,u,p,m)=\pmu/4$
    /// - $f(0,u,p,m)=0$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::atan_with_period_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_with_period_prec_round(7, 5, Floor);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_with_period_prec_round(7, 5, Ceiling);
    /// assert_eq!(y.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_with_period_prec_round(7, 5, Nearest);
    /// assert_eq!(y.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_with_period_prec_round(7, 20, Floor);
    /// assert_eq!(y.to_string(), "0.39970064");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_with_period_prec_round(7, 20, Ceiling);
    /// assert_eq!(y.to_string(), "0.3997011");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_with_period_prec_round(7, 20, Nearest);
    /// assert_eq!(y.to_string(), "0.39970064");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Atan, &self, Some(u), prec, rm, true)
    }

    /// Computes $u\arctan x/(2\pi)$, the arctangent of a [`BigExponentFloat`] measured in $u$ths of
    /// a turn, rounding the result to the specified precision and with the specified rounding mode.
    /// The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arctangent is less than, equal to, or greater than the exact arctangent.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\arctan x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arctan x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arctan x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |u\arctan x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\arctan x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\arctan x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(\pm\infty,u,p,m)=\pmu/4$
    /// - $f(0,u,p,m)=0$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_with_period_prec_round_ref(7, 5, Floor);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).atan_with_period_prec_round_ref(7, 5, Ceiling);
    /// assert_eq!(y.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).atan_with_period_prec_round_ref(7, 5, Nearest);
    /// assert_eq!(y.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_with_period_prec_round_ref(7, 20, Floor);
    /// assert_eq!(y.to_string(), "0.39970064");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).atan_with_period_prec_round_ref(7, 20, Ceiling);
    /// assert_eq!(y.to_string(), "0.3997011");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).atan_with_period_prec_round_ref(7, 20, Nearest);
    /// assert_eq!(y.to_string(), "0.39970064");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Atan, self, Some(u), prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with $u\arctan x/(2\pi)$, its arctangent measured in $u$ths
    /// of a turn, rounding the result to the specified precision and with the specified rounding
    /// mode. An [`Ordering`] is returned, indicating whether the rounded arctangent is less than,
    /// equal to, or greater than the exact arctangent. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\arctan x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arctan x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arctan x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |u\arctan x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\arctan x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\arctan x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(\pm\infty,u,p,m)=\pmu/4$
    /// - $f(0,u,p,m)=0$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_with_period_prec_round_assign(7, 5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.391");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_with_period_prec_round_assign(7, 5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.406");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_with_period_prec_round_assign(7, 5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.406");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_with_period_prec_round_assign(7, 20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.39970064");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(
    ///     x.atan_with_period_prec_round_assign(7, 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.3997011");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_with_period_prec_round_assign(7, 20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.39970064");
    /// ```
    #[inline]
    pub fn atan_with_period_prec_round_assign(
        &mut self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Atan, self, Some(u), prec, rm, true);
        *self = result;
        o
    }

    /// Computes $u\arctan x/(2\pi)$, the arctangent of a [`BigExponentFloat`] measured in $u$ths of
    /// a turn, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded arctangent is less than, equal to, or greater than the exact arctangent.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\arctan x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arctan x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arctan x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |u\arctan
    ///   x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=\text{NaN}$
    /// - $f(\pm\infty,u,p)=\pmu/4$
    /// - $f(0,u,p)=0$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::atan_with_period_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_with_period_prec(7, 5);
    /// assert_eq!(y.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_with_period_prec(7, 20);
    /// assert_eq!(y.to_string(), "0.39970064");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan_with_period_prec(self, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Atan,
            &self,
            Some(u),
            prec,
            Nearest,
            true,
        )
    }

    /// Computes $u\arctan x/(2\pi)$, the arctangent of a [`BigExponentFloat`] measured in $u$ths of
    /// a turn, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arctangent is less than, equal to, or greater than the exact arctangent.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\arctan x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arctan x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arctan x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |u\arctan
    ///   x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=\text{NaN}$
    /// - $f(\pm\infty,u,p)=\pmu/4$
    /// - $f(0,u,p)=0$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_with_period_prec_ref(7, 5);
    /// assert_eq!(y.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_with_period_prec_ref(7, 20);
    /// assert_eq!(y.to_string(), "0.39970064");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan_with_period_prec_ref(&self, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Atan,
            self,
            Some(u),
            prec,
            Nearest,
            true,
        )
    }

    /// Replaces a [`BigExponentFloat`] with $u\arctan x/(2\pi)$, its arctangent measured in $u$ths
    /// of a turn, rounding the result to the nearest value of the specified precision. An
    /// [`Ordering`] is returned, indicating whether the rounded arctangent is less than, equal to,
    /// or greater than the exact arctangent. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\arctan x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arctan x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arctan x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |u\arctan
    ///   x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=\text{NaN}$
    /// - $f(\pm\infty,u,p)=\pmu/4$
    /// - $f(0,u,p)=0$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_with_period_prec_assign(7, 5), Greater);
    /// assert_eq!(x.to_string(), "0.406");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_with_period_prec_assign(7, 20), Less);
    /// assert_eq!(x.to_string(), "0.39970064");
    /// ```
    #[inline]
    pub fn atan_with_period_prec_assign(&mut self, u: u64, prec: u64) -> Ordering {
        let (result, o) = inverse_trig_helper(
            InverseTrigFunction::Atan,
            self,
            Some(u),
            prec,
            Nearest,
            true,
        );
        *self = result;
        o
    }

    /// Computes $u\arctan x/(2\pi)$, the arctangent of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded arctangent is less than, equal to, or greater than
    /// the exact arctangent. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\arctan x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arctan x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arctan x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |u\arctan x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\arctan x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\arctan x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,u,p,m)=0$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::atan_with_period_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.35845709");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.3584576");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.3584576");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Atan, &x, Some(u), prec, rm)
    }

    /// Computes $u\arctan x/(2\pi)$, the arctangent of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded arctangent is less than, equal to, or greater than
    /// the exact arctangent. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\arctan x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arctan x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arctan x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |u\arctan x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\arctan x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\arctan x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,u,p,m)=0$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.35845709");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.3584576");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.3584576");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan_with_period_rational_prec_round_ref(
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Atan, x, Some(u), prec, rm)
    }

    /// Computes $u\arctan x/(2\pi)$, the arctangent of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the nearest value of the specified
    /// precision. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arctangent is less than, equal to, or greater than the exact arctangent.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\arctan x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arctan x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arctan x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |u\arctan
    ///   x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,u,p)=0$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::atan_with_period_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
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
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "0.3584576");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan_with_period_rational_prec(x: Rational, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Atan, &x, Some(u), prec, Nearest)
    }

    /// Computes $u\arctan x/(2\pi)$, the arctangent of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the nearest value of the specified
    /// precision. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arctangent is less than, equal to, or greater than the exact
    /// arctangent. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\arctan x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arctan x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arctan x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |u\arctan
    ///   x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,u,p)=0$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pmu/8$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
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
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "0.359");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_with_period_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "0.3584576");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan_with_period_rational_prec_ref(x: &Rational, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Atan, x, Some(u), prec, Nearest)
    }

    /// Computes $\arctan(x)/\pi$, the arctangent of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded arctangent is less than, equal to, or greater than the exact arctangent.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arctan(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arctan(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arctan(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arctan(x)/\pi|\rfloor-p+1}$.
    /// - If $\arctan(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\arctan(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm1/2$
    /// - $f(0,p,m)=0$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::atan_pi_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_pi_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "0.113");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_pi_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.117");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_pi_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "0.113");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_pi_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "0.11420023");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_pi_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.1142004");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_pi_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "0.11420023");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan_pi_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Atan, &self, Some(2), prec, rm, true)
    }

    /// Computes $\arctan(x)/\pi$, the arctangent of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arctangent is less than, equal to, or greater than the exact arctangent.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arctan(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arctan(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arctan(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arctan(x)/\pi|\rfloor-p+1}$.
    /// - If $\arctan(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\arctan(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm1/2$
    /// - $f(0,p,m)=0$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_pi_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "0.113");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_pi_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.117");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_pi_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "0.113");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_pi_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "0.11420023");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_pi_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.1142004");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_pi_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "0.11420023");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan_pi_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Atan, self, Some(2), prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with $\arctan(x)/\pi$, its arctangent measured in
    /// half-turns, rounding the result to the specified precision and with the specified rounding
    /// mode. An [`Ordering`] is returned, indicating whether the rounded arctangent is less than,
    /// equal to, or greater than the exact arctangent. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arctan(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arctan(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arctan(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arctan(x)/\pi|\rfloor-p+1}$.
    /// - If $\arctan(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\arctan(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm1/2$
    /// - $f(0,p,m)=0$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_pi_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.113");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_pi_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.117");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_pi_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.113");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_pi_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.11420023");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_pi_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.1142004");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_pi_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.11420023");
    /// ```
    #[inline]
    pub fn atan_pi_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Atan, self, Some(2), prec, rm, true);
        *self = result;
        o
    }

    /// Computes $\arctan(x)/\pi$, the arctangent of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded arctangent is less than, equal to, or greater than the exact arctangent.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arctan(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arctan(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arctan(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arctan(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm1/2$
    /// - $f(0,p)=0$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::atan_pi_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_pi_prec(5);
    /// assert_eq!(y.to_string(), "0.113");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).atan_pi_prec(20);
    /// assert_eq!(y.to_string(), "0.11420023");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan_pi_prec(self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Atan,
            &self,
            Some(2),
            prec,
            Nearest,
            true,
        )
    }

    /// Computes $\arctan(x)/\pi$, the arctangent of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arctangent is less than, equal to, or greater than the exact arctangent.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arctan(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arctan(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arctan(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arctan(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm1/2$
    /// - $f(0,p)=0$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_pi_prec_ref(5);
    /// assert_eq!(y.to_string(), "0.113");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).atan_pi_prec_ref(20);
    /// assert_eq!(y.to_string(), "0.11420023");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan_pi_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Atan,
            self,
            Some(2),
            prec,
            Nearest,
            true,
        )
    }

    /// Replaces a [`BigExponentFloat`] with $\arctan(x)/\pi$, its arctangent measured in
    /// half-turns, rounding the result to the nearest value of the specified precision. An
    /// [`Ordering`] is returned, indicating whether the rounded arctangent is less than, equal to,
    /// or greater than the exact arctangent. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arctan(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arctan(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arctan(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arctan(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm1/2$
    /// - $f(0,p)=0$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_pi_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.113");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.atan_pi_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.11420023");
    /// ```
    #[inline]
    pub fn atan_pi_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) = inverse_trig_helper(
            InverseTrigFunction::Atan,
            self,
            Some(2),
            prec,
            Nearest,
            true,
        );
        *self = result;
        o
    }

    /// Computes $\arctan(x)/\pi$, the arctangent of a [`Rational`] measured in half-turns,
    /// returning a [`BigExponentFloat`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded arctangent is less than, equal to, or greater than
    /// the exact arctangent. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arctan(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arctan(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arctan(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arctan(x)/\pi|\rfloor-p+1}$.
    /// - If $\arctan(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\arctan(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::atan_pi_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::atan_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.102");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.105");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.102");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.1024163");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.10241640");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.10241640");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan_pi_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Atan, &x, Some(2), prec, rm)
    }

    /// Computes $\arctan(x)/\pi$, the arctangent of a [`Rational`] measured in half-turns,
    /// returning a [`BigExponentFloat`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded arctangent is less than, equal to, or greater than
    /// the exact arctangent. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arctan(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arctan(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arctan(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arctan(x)/\pi|\rfloor-p+1}$.
    /// - If $\arctan(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\arctan(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::atan_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.102");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.105");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.102");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.1024163");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.10241640");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.10241640");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan_pi_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Atan, x, Some(2), prec, rm)
    }

    /// Computes $\arctan(x)/\pi$, the arctangent of a [`Rational`] measured in half-turns,
    /// returning a [`BigExponentFloat`], rounding the result to the nearest value of the specified
    /// precision. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arctangent is less than, equal to, or greater than the exact arctangent.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arctan(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arctan(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arctan(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arctan(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::atan_pi_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
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
    /// let (y, o) = BigExponentFloat::atan_pi_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.102");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan_pi_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.10241640");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan_pi_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Atan, &x, Some(2), prec, Nearest)
    }

    /// Computes $\arctan(x)/\pi$, the arctangent of a [`Rational`] measured in half-turns,
    /// returning a [`BigExponentFloat`], rounding the result to the nearest value of the specified
    /// precision. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arctangent is less than, equal to, or greater than the exact
    /// arctangent. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arctan(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arctan(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arctan(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arctan(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1,\ldots)=\pm1/4$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
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
    /// let (y, o) =
    ///     BigExponentFloat::atan_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.102");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::atan_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.10241640");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan_pi_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Atan, x, Some(2), prec, Nearest)
    }
}
