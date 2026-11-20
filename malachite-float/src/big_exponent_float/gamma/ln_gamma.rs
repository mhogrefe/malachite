// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::bracket::{
    Bracket, encl_div, encl_sub, floor_bracket, strict_bracket, ziv_loop_certified_from,
};
use crate::big_exponent_float::gamma::gamma::{
    GammaTarget, LnGammaState, gamma_int_bound, ln_of_encl,
};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{Parity, Sign};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, Two, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger, RoundingFrom, SaturatingFrom};
use malachite_base::num::number_theory::traits::Factorial;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// The bits lost near the zeros x = 1 and x = 2 of ln Gamma, for an x that is neither: k when x - 1
// or x - 2 is about 2^(-k).
//
// This is equivalent to `lnGammaCancelBits` from `AzFloat/LnGamma.lean`, Azurite.
fn ln_gamma_cancel_bits(x: &BigExponentFloat) -> u64 {
    let f = |d: BigExponentFloat| match d.get_exponent() {
        Some(e) if *e < 0u32 => u64::saturating_from(&-e),
        _ => 0,
    };
    max(
        f(x.sub_prec_ref_val(BigExponentFloat::ONE, 2).0),
        f(x.sub_prec_ref_val(BigExponentFloat::TWO, 2).0),
    )
}

// The bits lost near the zeros q = 1 and q = 2 of ln Gamma, for a q that is neither: k when q - 1
// or q - 2 is about 2^(-k).
//
// This is equivalent to `lnGammaRatCancelBits` from `AzFloat/LnGammaRat.lean`, Azurite.
fn ln_gamma_rational_cancel_bits(q: &Rational) -> u64 {
    // q is neither 1 nor 2
    let f = |d: Rational| u64::saturating_from(-d.floor_log_base_2_abs());
    max(f(q - Rational::ONE), f(q - Rational::TWO))
}

// An enclosure of ln |Gamma(x)| at Ziv level `w` for an x < 0 with |x| < 2^e that is not an
// integer, by the reflection formula Gamma(x) Gamma(1 - x) = pi / sin(pi x):
//
// ln |Gamma(x)| = ln(pi / |sin(pi x)|) - ln Gamma(1 - x).
//
// The sine keeps its relative accuracy next to the poles and next to 0. `None` when the pieces do
// not give an enclosure at this level.
//
// This is equivalent to `lnAbsGammaReflect` and `lnAbsGammaNegEncl` from `AzFloat/LnGamma.lean` and
// `lnAbsGammaRatNegEncl` from `AzFloat/LnGammaRat.lean`, Azurite.
fn ln_abs_gamma_negative_encl(
    target: &GammaTarget,
    e: &Integer,
    w: u64,
    state: &LnGammaState,
) -> Option<Bracket> {
    // 1 - x < 2^e
    let e = if *e > 0u32 {
        e + Integer::ONE
    } else {
        Integer::ONE
    };
    let l = state.ln_gamma_bracket(&target.shifted_encl(false, w), &e, w)?;
    let prec = w + 16;
    let s = target.sin_encl(prec)?.1;
    let pi = floor_bracket(BigExponentFloat::pi_prec_round(prec, Floor).0, prec);
    Some(encl_sub(
        prec,
        &ln_of_encl(&encl_div(prec, &pi, &s), prec),
        &l,
    ))
}

// The bracket of ln Gamma(x), for a positive x (`pos`), or of ln |Gamma(x)|, for a negative x that
// is not an integer, at Ziv level `w`, where |x| < 2^e. It is strict on both sides (see
// `strict_bracket`).
//
// This is equivalent to `lnGammaApprox` and `lnGammaRatApprox` from `AzFloat/LnGammaBracket.lean`,
// `lnAbsGammaApprox` from `AzFloat/LnGamma.lean`, and `lnAbsGammaRatApprox` from
// `AzFloat/LnGammaRat.lean`, Azurite, where only the lower end is made strict.
fn ln_gamma_approx(
    target: &GammaTarget,
    e: &Integer,
    pos: bool,
    w: u64,
    state: &LnGammaState,
) -> Bracket {
    strict_bracket(
        if pos {
            state.ln_gamma_bracket(&target.shifted_encl(true, w), e, w)
        } else {
            ln_abs_gamma_negative_encl(target, e, w, state)
        },
        w,
    )
}

// ln Gamma(n) = ln((n - 1)!) for an integer n >= 3, as the logarithm of the exact factorial, which
// is correctly rounded. This is used up to `gamma_int_bound`, the bound up to which the gamma
// function takes the factorial.
//
// Azurite has no such path; the result is the same.
fn ln_gamma_of_small_integer(
    n: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, bool, Ordering) {
    let (y, o) = BigExponentFloat::from(Natural::factorial(n - 1)).ln_prec_round(prec, rm);
    (y, true, o)
}

fn assert_inexact_allowed(abs: bool, rm: RoundingMode) {
    if abs {
        assert_ne!(
            rm, Exact,
            "Inexact logarithm of the absolute value of the gamma function"
        );
    } else {
        assert_ne!(rm, Exact, "Inexact logarithm of the gamma function");
    }
}

// ln Gamma(x), or ln |Gamma(x)| if `abs` is set, rounded to `prec` bits with `rm`, and whether
// Gamma(x) is positive (`true` where the sign is not defined). With `enclosures`, the logarithm of
// the gamma function of a positive number is enclosed through `ln_gamma_bracket_enclosures`; the
// result is the same.
//
// This is equivalent to `lnGammaPrecRound`, `lnAbsGammaPrecRound`, and `gammaSign` from
// `AzFloat/LnGamma.lean`, Azurite, except that Ziv's loop has no fuel and that a small positive
// integer takes the logarithm of a factorial (unless `enclosures` is set).
fn ln_gamma_helper_with(
    enclosures: bool,
    abs: bool,
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, bool, Ordering) {
    assert_ne!(prec, 0);
    match x {
        BigExponentFloat(NaN) => (BigExponentFloat::NAN, true, Equal),
        BigExponentFloat(Infinity { sign: false }) if !abs => (BigExponentFloat::NAN, true, Equal),
        BigExponentFloat(Infinity { .. } | Zero) => (BigExponentFloat::INFINITY, true, Equal),
        BigExponentFloat(Finite { sign: false, .. }) if !abs => {
            (BigExponentFloat::NAN, true, Equal)
        }
        BigExponentFloat(Finite { sign, exponent, .. }) => {
            let start = if *sign {
                if *x == 1u32 || *x == 2u32 {
                    return (BigExponentFloat::ZERO, true, Equal);
                }
                assert_inexact_allowed(abs, rm);
                if !enclosures && x.is_integer() && *x <= gamma_int_bound(prec) {
                    let n = u64::exact_from(&Natural::rounding_from(x, Exact).0);
                    return ln_gamma_of_small_integer(n, prec, rm);
                }
                max(64, (prec + 32).saturating_add(ln_gamma_cancel_bits(x)))
            } else {
                if x.is_integer() {
                    return (BigExponentFloat::INFINITY, true, Equal);
                }
                assert_inexact_allowed(abs, rm);
                max(64, prec + 32)
            };
            let target = GammaTarget::BigExponentFloat(x);
            let state = LnGammaState::new(enclosures);
            let (y, o) = ziv_loop_certified_from(
                &|w| ln_gamma_approx(&target, exponent, *sign, w, &state),
                prec,
                rm,
                start,
            );
            (y, *sign || Integer::rounding_from(x, Down).0.odd(), o)
        }
    }
}

// ln Gamma(x), or ln |Gamma(x)| if `abs` is set, rounded to `prec` bits with `rm`, and whether
// Gamma(x) is positive.
#[inline]
pub(crate) fn ln_gamma_helper(
    abs: bool,
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, bool, Ordering) {
    ln_gamma_helper_with(false, abs, x, prec, rm)
}

crate_test_fn! {
    // `ln_gamma_helper`, with the bracket of ln Gamma computed by `ln_gamma_bracket_enclosures`.
    ln_gamma_prec_round_enclosures(
        abs: bool,
        x: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode
    ) -> (BigExponentFloat, bool, Ordering) {
        ln_gamma_helper_with(true, abs, x, prec, rm)
    }
}

// ln Gamma(q), or ln |Gamma(q)| if `abs` is set, for a `Rational` q, rounded to `prec` bits with
// `rm`, and whether Gamma(q) is positive (`true` where the sign is not defined). With `enclosures`,
// the logarithm of the gamma function of a positive number is enclosed through
// `ln_gamma_bracket_enclosures`; the result is the same.
//
// This is equivalent to `lnGammaRatPrecRound`, `lnAbsGammaRatPrecRound`, and `gammaRatSign` from
// `AzFloat/LnGammaRat.lean`, Azurite, except that Ziv's loop has no fuel and that a small positive
// integer takes the logarithm of a factorial (unless `enclosures` is set).
fn ln_gamma_rational_helper_with(
    enclosures: bool,
    abs: bool,
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, bool, Ordering) {
    assert_ne!(prec, 0);
    let pos = match q.sign() {
        Equal => return (BigExponentFloat::INFINITY, true, Equal),
        Greater => true,
        Less => false,
    };
    let start = if pos {
        if *q == 1u32 || *q == 2u32 {
            return (BigExponentFloat::ZERO, true, Equal);
        }
        assert_inexact_allowed(abs, rm);
        if !enclosures && q.is_integer() && *q <= gamma_int_bound(prec) {
            return ln_gamma_of_small_integer(u64::exact_from(q.numerator_ref()), prec, rm);
        }
        max(
            64,
            (prec + 32).saturating_add(ln_gamma_rational_cancel_bits(q)),
        )
    } else {
        if !abs {
            return (BigExponentFloat::NAN, true, Equal);
        }
        if q.is_integer() {
            return (BigExponentFloat::INFINITY, true, Equal);
        }
        assert_inexact_allowed(abs, rm);
        max(64, prec + 32)
    };
    // |q| < 2^e, from the rounding of q away from zero.
    let e = BigExponentFloat::from_rational_prec_round_ref(q, 32, Up)
        .0
        .get_exponent()
        .unwrap()
        .clone();
    let target = GammaTarget::Rational(q);
    let state = LnGammaState::new(enclosures);
    let (y, o) = ziv_loop_certified_from(
        &|w| ln_gamma_approx(&target, &e, pos, w, &state),
        prec,
        rm,
        start,
    );
    (y, pos || Integer::rounding_from(q, Down).0.odd(), o)
}

// ln Gamma(q), or ln |Gamma(q)| if `abs` is set, for a `Rational` q, rounded to `prec` bits with
// `rm`, and whether Gamma(q) is positive.
#[inline]
pub(crate) fn ln_gamma_rational_helper(
    abs: bool,
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, bool, Ordering) {
    ln_gamma_rational_helper_with(false, abs, q, prec, rm)
}

crate_test_fn! {
    // `ln_gamma_rational_helper`, with the bracket of ln Gamma computed by
    // `ln_gamma_bracket_enclosures`.
    ln_gamma_rational_prec_round_enclosures(
        abs: bool,
        q: &Rational,
        prec: u64,
        rm: RoundingMode
    ) -> (BigExponentFloat, bool, Ordering) {
        ln_gamma_rational_helper_with(true, abs, q, prec, rm)
    }
}

impl BigExponentFloat {
    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`BigExponentFloat::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p+1}$.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    /// - $f(1,p,m)=f(2,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_gamma_prec_round), this function never
    /// overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::ln_gamma_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is 1 or 2).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec_round(5, Floor);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec_round(5, Ceiling);
    /// assert_eq!(g.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec_round(5, Nearest);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec_round(20, Floor);
    /// assert_eq!(g.to_string(), "0.5723648");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec_round(20, Ceiling);
    /// assert_eq!(g.to_string(), "0.57236576");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec_round(20, Nearest);
    /// assert_eq!(g.to_string(), "0.5723648");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (y, _, o) = ln_gamma_helper(false, &self, prec, rm);
        (y, o)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`BigExponentFloat::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p+1}$.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    /// - $f(1,p,m)=f(2,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_gamma_prec_round), this function never
    /// overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::ln_gamma_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is 1 or 2).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec_round_ref(5, Floor);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec_round_ref(5, Ceiling);
    /// assert_eq!(g.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec_round_ref(5, Nearest);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec_round_ref(20, Floor);
    /// assert_eq!(g.to_string(), "0.5723648");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec_round_ref(20, Ceiling);
    /// assert_eq!(g.to_string(), "0.57236576");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec_round_ref(20, Nearest);
    /// assert_eq!(g.to_string(), "0.5723648");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (y, _, o) = ln_gamma_helper(false, self, prec, rm);
        (y, o)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`BigExponentFloat`] in
    /// place, rounding the result to the specified precision and with the specified rounding mode.
    /// An [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`BigExponentFloat::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p+1}$.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::ln_gamma_prec_round`] documentation for information on special
    /// cases and termination.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::ln_gamma_prec_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is 1 or 2).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.ln_gamma_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.ln_gamma_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.594");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.ln_gamma_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.ln_gamma_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.5723648");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.ln_gamma_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.57236576");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.ln_gamma_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.5723648");
    /// ```
    #[inline]
    pub fn ln_gamma_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (mut y, _, o) = ln_gamma_helper(false, self, prec, rm);
        swap(self, &mut y);
        o
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`BigExponentFloat::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   \Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    /// - $f(1,p)=f(2,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_gamma_prec_round), this function never
    /// overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::ln_gamma_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec(5);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec(20);
    /// assert_eq!(g.to_string(), "0.5723648");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::power_of_2(1u64 << 40).ln_gamma_prec(20);
    /// assert_eq!(g.to_string(), "6.1406009e330985980553");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_prec(self, prec: u64) -> (Self, Ordering) {
        self.ln_gamma_prec_round(prec, Nearest)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`BigExponentFloat::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   \Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    /// - $f(1,p)=f(2,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_gamma_prec_round), this function never
    /// overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every $x$,
    /// and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::ln_gamma_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec_ref(5);
    /// assert_eq!(g.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.ln_gamma_prec_ref(20);
    /// assert_eq!(g.to_string(), "0.5723648");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::power_of_2(1u64 << 40).ln_gamma_prec_ref(20);
    /// assert_eq!(g.to_string(), "6.1406009e330985980553");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.ln_gamma_prec_round_ref(prec, Nearest)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`BigExponentFloat`] in
    /// place, rounding the result to the nearest value of the specified precision. An [`Ordering`]
    /// is returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`BigExponentFloat::ln_abs_gamma_prec_round`] gives $\ln |\Gamma(x)|$ for every $x$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   \Gamma(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::ln_gamma_prec_round`] documentation for information on special
    /// cases and termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::ln_gamma_prec_round_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.ln_gamma_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.ln_gamma_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.5723648");
    ///
    /// let mut x = BigExponentFloat::power_of_2(1u64 << 40);
    /// assert_eq!(x.ln_gamma_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "6.1406009e330985980553");
    /// ```
    #[inline]
    pub fn ln_gamma_prec_assign(&mut self, prec: u64) -> Ordering {
        self.ln_gamma_prec_round_assign(prec, Nearest)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Rational`], rounding
    /// the result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`BigExponentFloat::ln_abs_gamma_rational_prec_round`] gives $\ln |\Gamma(x)|$ for every
    /// $x$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p+1}$.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    /// - $f(1,p,m)=f(2,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_gamma_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::ln_gamma_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is 1 or 2).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::ln_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "0.969");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ln_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "1.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ln_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "1.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ln_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "0.9854202");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ln_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "0.98542118");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ln_gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "0.9854202");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn ln_gamma_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (y, _, o) = ln_gamma_rational_helper(false, &x, prec, rm);
        (y, o)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Rational`], rounding
    /// the result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`BigExponentFloat::ln_abs_gamma_rational_prec_round`] gives $\ln |\Gamma(x)|$ for every
    /// $x$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p+1}$.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\ln \Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p,m)=\infty$
    /// - $f(x,p,m)=\text{NaN}$ if $x<0$
    /// - $f(1,p,m)=f(2,p,m)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_gamma_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::ln_gamma_rational_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is 1 or 2).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::ln_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "0.969");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ln_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "1.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ln_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "1.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ln_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "0.9854202");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ln_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "0.98542118");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ln_gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "0.9854202");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (y, _, o) = ln_gamma_rational_helper(false, x, prec, rm);
        (y, o)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Rational`], rounding
    /// the result to the nearest value of the specified precision and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`BigExponentFloat::ln_abs_gamma_rational_prec_round`] gives $\ln |\Gamma(x)|$ for every
    /// $x$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   \Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    /// - $f(1,p)=f(2,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_gamma_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::ln_gamma_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
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
    /// let (g, o) = BigExponentFloat::ln_gamma_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(g.to_string(), "1.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ln_gamma_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(g.to_string(), "0.9854202");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::ln_gamma_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\ln \Gamma(x)$, the logarithm of the gamma function, of a [`Rational`], rounding
    /// the result to the nearest value of the specified precision and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$. Its logarithm is 0 at $x = 1$
    /// and $x = 2$, negative between them, and positive on the rest of the positive reals. This
    /// function is not defined for a negative $x$, even where $\Gamma(x)$ is positive;
    /// [`BigExponentFloat::ln_abs_gamma_rational_prec_round`] gives $\ln |\Gamma(x)|$ for every
    /// $x$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \ln \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\ln \Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\ln \Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\ln
    ///   \Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,p)=\infty$
    /// - $f(x,p)=\text{NaN}$ if $x<0$
    /// - $f(1,p)=f(2,p)=0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::ln_gamma_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// The result is correctly rounded. Finding it takes $\ln \Gamma(x)$ not to be a rounding
    /// boundary, which is a number with at most `prec + 1` significant bits. This is known when $x$
    /// is a positive integer (then the value is the logarithm of an integer, which is 0 or
    /// irrational); nothing suggests that it ever fails, but it has not been proven for every
    /// rational $x$, and this function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::ln_gamma_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3 \log n)$
    ///
    /// $M(n) = O(n^2 \log n)$
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
    /// let (g, o) =
    ///     BigExponentFloat::ln_gamma_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(g.to_string(), "1.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) =
    ///     BigExponentFloat::ln_gamma_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(g.to_string(), "0.9854202");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn ln_gamma_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::ln_gamma_rational_prec_round_ref(x, prec, Nearest)
    }
}
