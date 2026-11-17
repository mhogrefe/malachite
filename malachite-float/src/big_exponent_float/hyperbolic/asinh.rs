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
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{Abs, NegAssign, Reciprocal};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, One, Two, Zero as ZeroTrait,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

crate_test_enum! {
    // The six inverse hyperbolic functions, which share one implementation.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    InverseHyperbolicFunction {
        Asinh,
        Acosh,
        Atanh,
        Acoth,
        Asech,
        Acsch,
    }
}

// What a special value of an inverse hyperbolic function depends on: whether the argument is NaN,
// infinite, or zero, or else how its absolute value compares with 1.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ArgumentClass {
    NaN,
    Infinity,
    Zero,
    AbsOne,
    AbsLessThanOne,
    AbsGreaterThanOne,
}

impl InverseHyperbolicFunction {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Asinh => "asinh",
            Self::Acosh => "acosh",
            Self::Atanh => "atanh",
            Self::Acoth => "acoth",
            Self::Asech => "asech",
            Self::Acsch => "acsch",
        }
    }

    // Whether f(-x) = -f(x).
    const fn odd(self) -> bool {
        matches!(self, Self::Asinh | Self::Atanh | Self::Acoth | Self::Acsch)
    }

    // The value at a NaN, an infinity, zero, 1, -1, or an argument outside the domain: those of
    // `Float`'s functions, except that acsch has a pole at 0, where, as a `BigExponentFloat` has no
    // negative zero to tell the two one-sided limits apart, the result is NaN, as 1/0 is in
    // `arf_div`. `None` for any other argument, whose image is transcendental.
    const fn special(self, negative: bool, class: ArgumentClass) -> Option<BigExponentFloat> {
        Some(match (self, class) {
            (_, ArgumentClass::NaN)
            | (
                Self::Atanh | Self::Asech,
                ArgumentClass::Infinity | ArgumentClass::AbsGreaterThanOne,
            )
            | (Self::Acosh | Self::Acoth | Self::Acsch, ArgumentClass::Zero)
            | (Self::Acosh | Self::Acoth, ArgumentClass::AbsLessThanOne) => BigExponentFloat::NAN,
            (Self::Acosh, ArgumentClass::Infinity | ArgumentClass::AbsGreaterThanOne)
            | (Self::Asech, ArgumentClass::AbsLessThanOne)
                if negative =>
            {
                BigExponentFloat::NAN
            }
            (Self::Acosh | Self::Asech, ArgumentClass::AbsOne) if negative => BigExponentFloat::NAN,
            (Self::Asinh | Self::Acosh, ArgumentClass::Infinity)
            | (Self::Atanh | Self::Acoth, ArgumentClass::AbsOne)
                if negative =>
            {
                BigExponentFloat::NEGATIVE_INFINITY
            }
            (Self::Asinh | Self::Acosh, ArgumentClass::Infinity)
            | (Self::Atanh | Self::Acoth, ArgumentClass::AbsOne)
            | (Self::Asech, ArgumentClass::Zero) => BigExponentFloat::INFINITY,
            (Self::Acoth | Self::Acsch, ArgumentClass::Infinity)
            | (Self::Asinh | Self::Atanh, ArgumentClass::Zero)
            | (Self::Acosh | Self::Asech, ArgumentClass::AbsOne) => BigExponentFloat::ZERO,
            _ => return None,
        })
    }

    fn float(self, x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        match self {
            Self::Asinh => x.asinh_prec_round_ref(prec, rm),
            Self::Acosh => x.acosh_prec_round_ref(prec, rm),
            Self::Atanh => x.atanh_prec_round_ref(prec, rm),
            Self::Acoth => x.acoth_prec_round_ref(prec, rm),
            Self::Asech => x.asech_prec_round_ref(prec, rm),
            Self::Acsch => x.acsch_prec_round_ref(prec, rm),
        }
    }

    fn float_rational(self, x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        match self {
            Self::Asinh => Float::asinh_rational_prec_round_ref(x, prec, rm),
            Self::Acosh => Float::acosh_rational_prec_round_ref(x, prec, rm),
            Self::Atanh => Float::atanh_rational_prec_round_ref(x, prec, rm),
            Self::Acoth => Float::acoth_rational_prec_round_ref(x, prec, rm),
            Self::Asech => Float::asech_rational_prec_round_ref(x, prec, rm),
            Self::Acsch => Float::acsch_rational_prec_round_ref(x, prec, rm),
        }
    }

    // For an x close enough to 0 (for asinh and atanh) or large enough (for acoth and acsch) that
    // f(x) is within a tiny fraction of an ulp of a simple exact value v, the pair (v, up), where
    // f(x) is v plus (if `up`) or minus a positive amount small enough for `round_nudged` (which
    // needs it below 2^(EXP(v) - max(p, q) - 2), with q the precision of v). With s = x for asinh
    // and atanh and s = 1/x for acoth and acsch (when 1/x is exact, x a power of 2), and e = EXP(s)
    // with 2e <= -max(p, q) - 4: asinh s = s - s^3/6 + ... and atanh s = s + s^3/3 + ... differ
    // from s by less than |s|^3 < 2^(3e), and acsch x = asinh s, acoth x = atanh s.
    fn nudge(
        self,
        negative: bool,
        exact: Option<&BigExponentFloat>,
        prec: u64,
    ) -> Option<(BigExponentFloat, bool)> {
        let x = exact?;
        let v = match self {
            Self::Asinh | Self::Atanh => x.clone(),
            Self::Acoth | Self::Acsch if x.get_prec() == Some(1) => x.reciprocal_prec_ref(1).0,
            _ => return None,
        };
        // asinh and acsch move toward 0 from s, and atanh and acoth away from it
        let up = matches!(self, Self::Atanh | Self::Acoth) != negative;
        let e = v.get_exponent().unwrap();
        let q = v.get_prec().unwrap();
        (*e <= 0u32 && max(prec, q) + 4 <= -(e << 1u32)).then_some((v, up))
    }
}

fn bef_class(x: &BigExponentFloat) -> ArgumentClass {
    match x {
        BigExponentFloat(NaN) => ArgumentClass::NaN,
        BigExponentFloat(Infinity { .. }) => ArgumentClass::Infinity,
        BigExponentFloat(Zero) => ArgumentClass::Zero,
        _ => match x.partial_cmp_abs(&BigExponentFloat::ONE).unwrap() {
            Equal => ArgumentClass::AbsOne,
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
        Equal => ArgumentClass::AbsOne,
        Less => ArgumentClass::AbsLessThanOne,
        Greater => ArgumentClass::AbsGreaterThanOne,
    }
}

// The argument x of an inverse hyperbolic function, finite, nonzero, and in the function's domain.
#[derive(Clone, Copy)]
enum InverseHyperbolicArgument<'a> {
    BigExponentFloat(&'a BigExponentFloat),
    Rational(&'a Rational),
}

impl InverseHyperbolicArgument<'_> {
    fn negative(&self) -> bool {
        match self {
            InverseHyperbolicArgument::BigExponentFloat(x) => **x < 0u32,
            InverseHyperbolicArgument::Rational(q) => **q < 0u32,
        }
    }

    // x itself, if it is a `BigExponentFloat`.
    fn exact(&self) -> Option<Cow<'_, BigExponentFloat>> {
        match self {
            InverseHyperbolicArgument::BigExponentFloat(x) => Some(Cow::Borrowed(x)),
            InverseHyperbolicArgument::Rational(q) => {
                BigExponentFloat::try_from(*q).ok().map(Cow::Owned)
            }
        }
    }

    // |x|, exactly for a `BigExponentFloat` and with a relative error of at most 2^(-w) for a
    // `Rational`.
    fn abs(&self, w: u64) -> BigExponentFloat {
        match self {
            InverseHyperbolicArgument::BigExponentFloat(x) => (*x).abs(),
            InverseHyperbolicArgument::Rational(q) => {
                BigExponentFloat::from_rational_prec_ref(q, w).0.abs()
            }
        }
    }

    // 1/|x|, with a relative error of at most 2^(-w).
    fn reciprocal_abs(&self, w: u64) -> BigExponentFloat {
        match self {
            InverseHyperbolicArgument::BigExponentFloat(x) => (*x).abs().reciprocal_prec(w).0,
            InverseHyperbolicArgument::Rational(q) => {
                BigExponentFloat::from_rational_prec((*q).abs().reciprocal(), w).0
            }
        }
    }

    // 1 - |x| if `one_minus` and |x| - 1 otherwise, a positive number, with a relative error of at
    // most 2^(-w): the difference is formed exactly and rounded once.
    fn difference_from_one(&self, one_minus: bool, w: u64) -> BigExponentFloat {
        match self {
            InverseHyperbolicArgument::BigExponentFloat(x) => {
                let a = (*x).abs();
                if one_minus {
                    BigExponentFloat::ONE.sub_prec(a, w).0
                } else {
                    a.sub_prec(BigExponentFloat::ONE, w).0
                }
            }
            InverseHyperbolicArgument::Rational(q) => {
                let a = (*q).abs();
                BigExponentFloat::from_rational_prec(
                    if one_minus {
                        Rational::ONE - a
                    } else {
                        a - Rational::ONE
                    },
                    w,
                )
                .0
            }
        }
    }
}

// log(a + sqrt(a^2 + 1)) - 1 = a + a^2/(1 + sqrt(1 + a^2)), the argument t with asinh a = log(1 +
// t), for an a > 0 with a relative error of at most 3 * 2^(-w). Every operation is rounded to `w`
// bits and all the quantities are positive, so relative errors add up: a^2 has one below 7 *
// 2^(-w), 1 + a^2 below 8 * 2^(-w), its square root below 5 * 2^(-w), 1 + sqrt(1 + a^2) below 6 *
// 2^(-w), the quotient below 14 * 2^(-w), and t below 15 * 2^(-w).
fn asinh_minus_one_argument(a: &BigExponentFloat, w: u64) -> BigExponentFloat {
    let s = a.square_prec_ref(w).0;
    let r = BigExponentFloat::ONE
        .add_prec_val_ref(&s, w)
        .0
        .sqrt_prec(w)
        .0;
    let q = s.div_prec(BigExponentFloat::ONE.add_prec(r, w).0, w).0;
    a.add_prec_ref_val(q, w).0
}

// An approximation of f(x) for a finite nonzero x in the domain, and an `err` as in `ziv_loop`.
// With w = wp + 12, every function is log(1 + t)/k for k = 1 or 2 and a positive t, computed with
// `w`-bit operations from positive quantities, which keeps its relative error below 15 * 2^(-w):
// - asinh |x| and acsch |x| = asinh(1/|x|): t from `asinh_minus_one_argument`, with a = |x| (exact
//   for a `BigExponentFloat`) or a = 1/|x|;
// - acosh x: t = d + sqrt(d(d + 2)), with d = x - 1, for a relative error below 6 * 2^(-w);
// - asech x: t = (d + sqrt(d(1 + x)))/x, with d = 1 - x, below 9 * 2^(-w);
// - atanh |x|: k = 2 and t = 2|x|/(1 - |x|), below 5 * 2^(-w);
// - acoth |x|: k = 2 and t = 2/(|x| - 1), below 3 * 2^(-w).
// The relative condition number of log(1 + t), t/((1 + t) log(1 + t)), is less than 1 for t > 0, so
// log(1 + t) inherits a relative error below 15 * 2^(-w), to which its own rounding adds 2^(-w);
// with the sign of an odd function restored, the error is below 2^(EXP - wp - 7).
fn inverse_hyperbolic_approx(
    f: InverseHyperbolicFunction,
    x: InverseHyperbolicArgument<'_>,
    wp: u64,
) -> (BigExponentFloat, u64) {
    let w = wp + 12;
    let t = match f {
        InverseHyperbolicFunction::Asinh => asinh_minus_one_argument(&x.abs(w), w),
        InverseHyperbolicFunction::Acsch => asinh_minus_one_argument(&x.reciprocal_abs(w), w),
        InverseHyperbolicFunction::Acosh => {
            let d = x.difference_from_one(false, w);
            let r = d
                .add_prec_ref_val(BigExponentFloat::TWO, w)
                .0
                .mul_prec_val_ref(&d, w)
                .0
                .sqrt_prec(w)
                .0;
            d.add_prec(r, w).0
        }
        InverseHyperbolicFunction::Asech => {
            let a = x.abs(w);
            let d = x.difference_from_one(true, w);
            let r = a
                .add_prec_ref_val(BigExponentFloat::ONE, w)
                .0
                .mul_prec_val_ref(&d, w)
                .0
                .sqrt_prec(w)
                .0;
            d.add_prec(r, w).0.div_prec(a, w).0
        }
        InverseHyperbolicFunction::Atanh => {
            let d = x.difference_from_one(true, w);
            (x.abs(w) << 1u32).div_prec(d, w).0
        }
        InverseHyperbolicFunction::Acoth => {
            BigExponentFloat::TWO
                .div_prec(x.difference_from_one(false, w), w)
                .0
        }
    };
    let mut y = t.ln_1_plus_x_prec(w).0;
    if matches!(
        f,
        InverseHyperbolicFunction::Atanh | InverseHyperbolicFunction::Acoth
    ) {
        y >>= 1u32;
    }
    if f.odd() && x.negative() {
        y.neg_assign();
    }
    (y, wp)
}

// The result of one of `Float`'s functions, converted if it neither overflowed nor underflowed.
fn from_float_inverse_hyperbolic(
    result: (Float, Ordering),
) -> Option<(BigExponentFloat, Ordering)> {
    from_float_result(result, None)
}

// The exponent of the finite nonzero x, if it is within `Float`'s exponent range.
fn float_exponent(x: &BigExponentFloat) -> Option<i32> {
    let e = i32::try_from(x.get_exponent().unwrap()).ok()?;
    (Float::MIN_EXPONENT..=Float::MAX_EXPONENT)
        .contains(&e)
        .then_some(e)
}

// f(x) for a finite nonzero argument in the domain, after `Float`'s function: `round_nudged` when
// f(x) is that close to a simple value (see `InverseHyperbolicFunction::nudge`), and otherwise a
// Ziv loop.
fn round_inverse_hyperbolic(
    f: InverseHyperbolicFunction,
    x: InverseHyperbolicArgument<'_>,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let exact = x.exact();
    if let Some((v, up)) = f.nudge(x.negative(), exact.as_deref(), prec) {
        return round_nudged(&v, up, prec, rm);
    }
    ziv_loop(prec, rm, |wp| inverse_hyperbolic_approx(f, x, wp))
}

// f(x), rounded to `prec` bits with `rm`. The special values are those of
// `InverseHyperbolicFunction::special`; every other value is transcendental, so `Exact` is
// rejected. If `use_float` and x's exponent is within `Float`'s range, `Float`'s function is used
// unless its result underflows (possible only for an x with billions of bits); otherwise,
// `round_inverse_hyperbolic`.
pub(crate) fn inverse_hyperbolic_helper(
    f: InverseHyperbolicFunction,
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
    use_float: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if let Some(y) = f.special(*x < 0u32, bef_class(x)) {
        return (y, Equal);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat {}", f.name());
    if use_float
        && let Some(e) = float_exponent(x)
        && let Some(result) =
            from_float_inverse_hyperbolic(f.float(&finite_to_float(Cow::Borrowed(x), e), prec, rm))
    {
        return result;
    }
    round_inverse_hyperbolic(f, InverseHyperbolicArgument::BigExponentFloat(x), prec, rm)
}

// f(x) for a `Rational` x, rounded to `prec` bits with `rm`: as in `inverse_hyperbolic_helper`,
// with `Float`'s `Rational` functions, whose results underflow only for an x with billions of bits.
pub(crate) fn inverse_hyperbolic_rational_helper(
    f: InverseHyperbolicFunction,
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
    use_float: bool,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    if let Some(y) = f.special(*x < 0u32, rational_class(x)) {
        return (y, Equal);
    }
    assert_ne!(rm, Exact, "Inexact BigExponentFloat {}", f.name());
    if use_float && let Some(result) = from_float_inverse_hyperbolic(f.float_rational(x, prec, rm))
    {
        return result;
    }
    round_inverse_hyperbolic(f, InverseHyperbolicArgument::Rational(x), prec, rm)
}

crate_test_fn! {
    // f(x) rounded to `prec` bits with `rm`, without `Float`'s inverse hyperbolic functions: as
    // log(1 + t), as in `inverse_hyperbolic_approx`. Used to check `inverse_hyperbolic_helper`.
    inverse_hyperbolic_prec_round_generic(
        f: InverseHyperbolicFunction,
        x: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        inverse_hyperbolic_helper(f, x, prec, rm, false)
    }
}

crate_test_fn! {
    // f(x) for a `Rational` x rounded to `prec` bits with `rm`, without `Float`'s inverse
    // hyperbolic functions, as in `inverse_hyperbolic_prec_round_generic`. Used to check
    // `inverse_hyperbolic_rational_helper`.
    inverse_hyperbolic_rational_prec_round_generic(
        f: InverseHyperbolicFunction,
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (BigExponentFloat, Ordering) {
        inverse_hyperbolic_rational_helper(f, x, prec, rm, false)
    }
}

impl BigExponentFloat {
    /// Computes the inverse hyperbolic sine of a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic sine is less than, equal to, or greater than the exact inverse hyperbolic sine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asinh} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asinh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p+1}$.
    /// - If $\operatorname{asinh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm\infty$
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asinh_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::asinh_prec`]
    /// instead.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is finite and nonzero (when
    /// it is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::ONE.asinh_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "0.88");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.asinh_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.906");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ONE.asinh_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "0.88");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.asinh_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.asinh_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.8813744");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::ONE.asinh_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asinh_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_hyperbolic_helper(InverseHyperbolicFunction::Asinh, &self, prec, rm, true)
    }

    /// Computes the inverse hyperbolic sine of a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic sine is less than, equal to, or greater than the exact inverse hyperbolic sine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asinh} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asinh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p+1}$.
    /// - If $\operatorname{asinh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm\infty$
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asinh_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is finite and nonzero (when
    /// it is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).asinh_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "0.88");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).asinh_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.906");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).asinh_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "0.88");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).asinh_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).asinh_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.8813744");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).asinh_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asinh_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_hyperbolic_helper(InverseHyperbolicFunction::Asinh, self, prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with its inverse hyperbolic sine, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded inverse hyperbolic sine is less than, equal to, or greater
    /// than the exact inverse hyperbolic sine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asinh} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asinh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p+1}$.
    /// - If $\operatorname{asinh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm\infty$
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asinh_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is finite and nonzero (when
    /// it is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.asinh_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.88");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.asinh_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.906");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.asinh_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.88");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.asinh_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.88137341");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.asinh_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.8813744");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.asinh_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.88137341");
    /// ```
    #[inline]
    pub fn asinh_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) =
            inverse_hyperbolic_helper(InverseHyperbolicFunction::Asinh, self, prec, rm, true);
        *self = result;
        o
    }

    /// Computes the inverse hyperbolic sine of a [`BigExponentFloat`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic sine is
    /// less than, equal to, or greater than the exact inverse hyperbolic sine. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asinh} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asinh} x$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm\infty$
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asinh_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::asinh_prec_round`] instead.
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
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::ONE.asinh_prec(5);
    /// assert_eq!(y.to_string(), "0.88");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::ONE.asinh_prec(20);
    /// assert_eq!(y.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asinh_prec(self, prec: u64) -> (Self, Ordering) {
        inverse_hyperbolic_helper(InverseHyperbolicFunction::Asinh, &self, prec, Nearest, true)
    }

    /// Computes the inverse hyperbolic sine of a [`BigExponentFloat`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded inverse hyperbolic sine is
    /// less than, equal to, or greater than the exact inverse hyperbolic sine. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asinh} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asinh} x$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm\infty$
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asinh_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
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
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).asinh_prec_ref(5);
    /// assert_eq!(y.to_string(), "0.88");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::ONE).asinh_prec_ref(20);
    /// assert_eq!(y.to_string(), "0.88137341");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asinh_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        inverse_hyperbolic_helper(InverseHyperbolicFunction::Asinh, self, prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] with its inverse hyperbolic sine, rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is returned, indicating whether
    /// the rounded inverse hyperbolic sine is less than, equal to, or greater than the exact
    /// inverse hyperbolic sine. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asinh} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asinh} x$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm\infty$
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asinh_prec_round), this function accepts
    /// arguments with any exponent, and its result never overflows or underflows.
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
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.asinh_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.88");
    ///
    /// let mut x = BigExponentFloat::ONE;
    /// assert_eq!(x.asinh_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.88137341");
    /// ```
    #[inline]
    pub fn asinh_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) =
            inverse_hyperbolic_helper(InverseHyperbolicFunction::Asinh, self, prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes the inverse hyperbolic sine of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded inverse hyperbolic sine is less than, equal to, or greater than the exact inverse
    /// hyperbolic sine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asinh} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asinh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p+1}$.
    /// - If $\operatorname{asinh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asinh_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::asinh_rational_prec`] instead.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is finite and nonzero (when
    /// it is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::asinh_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(y.to_string(), "0.31");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asinh_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asinh_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asinh_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.3274498");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asinh_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.32745028");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asinh_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.32745028");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn asinh_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_hyperbolic_rational_helper(InverseHyperbolicFunction::Asinh, &x, prec, rm, true)
    }

    /// Computes the inverse hyperbolic sine of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded inverse hyperbolic sine is less than, equal to, or greater than the exact inverse
    /// hyperbolic sine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asinh} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asinh} x$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p+1}$.
    /// - If $\operatorname{asinh} x$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asinh_rational_prec_round), this function
    /// never overflows or underflows.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is finite and nonzero (when
    /// it is transcendental).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::asinh_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.31");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asinh_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asinh_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asinh_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.3274498");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asinh_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.32745028");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asinh_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.32745028");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asinh_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_hyperbolic_rational_helper(InverseHyperbolicFunction::Asinh, x, prec, rm, true)
    }

    /// Computes the inverse hyperbolic sine of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic sine is less than, equal to, or greater than the exact inverse hyperbolic sine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asinh} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asinh} x$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asinh_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::asinh_rational_prec_round`] instead.
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
    /// let (y, o) = BigExponentFloat::asinh_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asinh_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.32745028");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn asinh_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        inverse_hyperbolic_rational_helper(
            InverseHyperbolicFunction::Asinh,
            &x,
            prec,
            Nearest,
            true,
        )
    }

    /// Computes the inverse hyperbolic sine of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded inverse
    /// hyperbolic sine is less than, equal to, or greater than the exact inverse hyperbolic sine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \operatorname{asinh} x+\varepsilon.
    /// $$
    /// - If $\operatorname{asinh} x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\operatorname{asinh} x$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{asinh} x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asinh_rational_prec_round), this function
    /// never overflows or underflows.
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
    /// let (y, o) =
    ///     BigExponentFloat::asinh_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.328");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::asinh_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.32745028");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asinh_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        inverse_hyperbolic_rational_helper(InverseHyperbolicFunction::Asinh, x, prec, Nearest, true)
    }
}
