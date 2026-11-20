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
    Bracket, ZIV_GUARD_BITS, encl_add, encl_div, encl_mul, encl_sub, floor_bracket,
    rounding_possible, ziv_loop_certified_from,
};
use crate::big_exponent_float::exponential::exp::power_of_2_integer;
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    Abs, Parity, Pow, PowerOf2, SaturatingAddAssign, Sign,
};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeOne, One, OneHalf, Two, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger, RoundingFrom, SaturatingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::number_theory::traits::Factorial;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::number_theory::tangent_numbers::tangent_numbers_prefix;
use malachite_q::Rational;

// The exponent L of the threshold 2^L from which Stirling's series is used at Ziv level `w`: 2^L is
// in (w / 4, w / 2]. A larger shift costs more in the rising product than it saves in the series.
//
// This is equivalent to `lnGammaShiftExp` from `AzFloat/LnGammaBracket.lean`, Azurite.
const fn ln_gamma_shift_exp(w: u64) -> u64 {
    (w >> 2).ilog2() as u64 + 1
}

// The shift d for a positive x with x < 2^e (`x_lo` is x, or the lower end of an enclosure of it):
// x + d >= 2^l, and d = 0 only when x >= 1.
//
// This is equivalent to `lnGammaShift` from `AzFloat/LnGammaBracket.lean`, Azurite.
fn ln_gamma_shift(x_lo: &BigExponentFloat, e: &Integer, l: u64) -> u64 {
    let s = if *e > l {
        0
    } else if *e > 0u32 {
        u64::power_of_2(l) - u64::power_of_2(u64::exact_from(e) - 1)
    } else {
        u64::power_of_2(l)
    };
    if s == 0 && *x_lo < 1u32 {
        // Every caller passes the exponent of x itself, or one more, so x >= 1 when e > l.
        fail_on_untested_path("ln_gamma_shift, no shift for an argument below 1");
        1
    } else {
        s
    }
}

// The number of tangent numbers at Ziv level `w` for an argument y with 2^(e - 1) <= y < 2^e. The
// term of index m of Stirling's series is about (m / (pi * exp(1) * y))^(2m), which is m times at
// least (4e + 7 - floor(lg(m^4))) / 2 bits; the result is the least m that reaches `w` + 24 bits,
// plus three. Too few terms only cost a wider bracket: the series is closed by its last term,
// whatever it is.
//
// This is equivalent to `lnGammaTerms` and `lnGammaTermsGo` from `AzFloat/LnGammaBracket.lean`,
// Azurite.
fn ln_gamma_terms(w: u64, e: u64) -> u64 {
    let target = w.saturating_add(24).saturating_mul(2);
    let bits = e.saturating_mul(4).saturating_add(7);
    let candidates = w >> 1;
    for m in 1..=candidates {
        let lg = Natural::from(m).pow(4).significant_bits() - 1;
        if target <= m.saturating_mul(bits.saturating_sub(lg)) {
            return m + 3;
        }
    }
    // e is at least about lg(w), which makes some m below w / 2 large enough.
    fail_on_untested_path("ln_gamma_terms, no candidate reaches the target");
    candidates + 4
}

// The rising product x * (x + 1) * ... * (x + n - 1) of an enclosure `x` of a positive number, at
// precision `prec`.
//
// This is equivalent to `risingEncl` from `AzFloat/LnGammaBracket.lean`, Azurite.
fn rising_encl(prec: u64, x: &Bracket, n: u64) -> Bracket {
    let mut product = (BigExponentFloat::ONE, BigExponentFloat::ONE);
    for i in 0..n {
        let i = BigExponentFloat::from(i);
        let factor = (
            x.0.add_prec_round_ref_ref(&i, prec, Floor).0,
            x.1.add_prec_round_ref_val(i, prec, Ceiling).0,
        );
        product = encl_mul(prec, &product, &factor);
    }
    product
}

// The enclosure of a_k / y^(2k - 1), where a_k = T_k / ((2k - 1) * (4^(2k) - 2^(2k))), from the
// tangent number T_k and an enclosure `p` of y^(-(2k - 1)).
//
// This is equivalent to `stirlingTermEncl` from `AzFloat/LnGammaBracket.lean`, Azurite.
fn stirling_term_encl(prec: u64, t: &Natural, k: u64, p: &Bracket) -> Bracket {
    let numerator = BigExponentFloat::from(t);
    let denominator = BigExponentFloat::from(
        (Natural::power_of_2(k << 2) - Natural::power_of_2(k << 1)) * Natural::from((k << 1) - 1),
    );
    let coefficient = (
        numerator
            .div_prec_round_ref_ref(&denominator, prec, Floor)
            .0,
        numerator.div_prec_round(denominator, prec, Ceiling).0,
    );
    encl_mul(prec, &coefficient, p)
}

// The enclosure of mu(y) = ln Gamma(y) - (y - 1/2) ln y + y - ln(2 pi) / 2 from an enclosure `y` of
// y >= 1, by Stirling's series a_1 / y - a_2 / y^3 + a_3 / y^5 - ..., using at most `m` >= 1
// tangent numbers. A term below `tol`, or the last one of the table, is not added: the remainder
// lies between 0 and it, with its sign. So the enclosure is valid whatever the number of terms is.
//
// This is equivalent to `stirlingMuEncl` and `stirlingSumLoop` from `AzFloat/LnGammaBracket.lean`,
// Azurite.
fn stirling_mu_encl(prec: u64, m: u64, y: &Bracket, tol: &BigExponentFloat) -> Bracket {
    let ts: Vec<Natural> = tangent_numbers_prefix(m);
    let one = (BigExponentFloat::ONE, BigExponentFloat::ONE);
    let mut p = encl_div(prec, &one, y);
    let inverse_square = encl_mul(prec, &p, &p);
    let mut sum = (BigExponentFloat::ZERO, BigExponentFloat::ZERO);
    for (k, t) in (1..).zip(&ts) {
        let term = stirling_term_encl(prec, t, k, &p);
        if k == m || term.1 <= *tol {
            if k.odd() {
                sum.1.add_prec_round_assign(term.1, prec, Ceiling);
            } else {
                sum.0.sub_prec_round_assign(term.1, prec, Floor);
            }
            break;
        }
        sum = if k.odd() {
            encl_add(prec, &sum, &term)
        } else {
            encl_sub(prec, &sum, &term)
        };
        p = encl_mul(prec, &p, &inverse_square);
    }
    sum
}

// The logarithm of an enclosure `y` of a positive number at precision `prec`: the logarithm at the
// lower end, with the upper end raised by (hi - lo) / lo >= ln(hi / lo).
//
// This is equivalent to `lnOfEncl` from `AzFloat/LnEncl.lean`, Azurite, with the correctly rounded
// logarithm in place of Azurite's bracket.
fn ln_of_encl(y: &Bracket, prec: u64) -> Bracket {
    let (lo, mut hi) = floor_bracket(y.0.ln_prec_round_ref(prec, Floor).0, prec);
    if y.0 != y.1 {
        let width =
            y.1.sub_prec_round_ref_ref(&y.0, 64, Ceiling)
                .0
                .div_prec_round_val_ref(&y.0, 64, Ceiling)
                .0;
        hi.add_prec_round_assign(width, prec, Ceiling);
    }
    (lo, hi)
}

// The enclosure of ln Gamma(y) - ln(2 pi) / 2 for y >= 1 from an enclosure `y` of y: (y - 1/2) ln y
// - y + mu(y).
//
// This is equivalent to `lnGammaStirlingEncl` from `AzFloat/LnGammaBracket.lean`, Azurite.
fn ln_gamma_stirling_encl(prec: u64, m: u64, y: &Bracket, tol: &BigExponentFloat) -> Bracket {
    let ln_y = ln_of_encl(y, prec);
    let half = (BigExponentFloat::ONE_HALF, BigExponentFloat::ONE_HALF);
    // y >= 1, so y - 1/2 and ln y are nonnegative.
    let a = encl_mul(prec, &encl_sub(prec, y, &half), &ln_y);
    encl_add(
        prec,
        &encl_sub(prec, &a, y),
        &stirling_mu_encl(prec, m, y, tol),
    )
}

// The bracket of ln Gamma(x) at Ziv level `w` >= 64 from an enclosure `x` of x > 0, with a positive
// lower end, where x < 2^e. The argument is shifted to y = x + d >= 2^L, where Stirling's series
// converges fast:
//
// ln Gamma(x) = ln Gamma(y) - ln(x * (x + 1) * ... * (x + d - 1)).
//
// The constant and the rising product P share one logarithm: ln(2 pi) / 2 - ln P = ln(2 pi / P^2) /
// 2. The result encloses ln Gamma(x) whatever `e` and `w` are; they only choose the shift, the
// number of terms, and the precision. Its width is not bounded: near the zeros x = 1 and x = 2 the
// terms cancel.
//
// This is equivalent to `lnGammaBracketEncl` from `AzFloat/LnGammaBracket.lean`, Azurite.
pub(crate) fn ln_gamma_bracket_encl(x: &Bracket, e: &Integer, w: u64) -> Bracket {
    let l = ln_gamma_shift_exp(w);
    let prec = w + (l << 1) + 8 + (u64::from(w.ilog2()) << 1);
    let shift = ln_gamma_shift(&x.0, e, l);
    let terms = ln_gamma_terms(
        w,
        max(l + 1, if *e > 0u32 { u64::exact_from(e) } else { 0 }),
    );
    let tol = power_of_2_integer(-Integer::from(w + 16));
    let two_pi = floor_bracket(BigExponentFloat::tau_prec_round(prec, Floor).0, prec);
    let (stirling, constant) = if shift == 0 {
        (ln_gamma_stirling_encl(prec, terms, x, &tol), two_pi)
    } else {
        let p = rising_encl(prec, x, shift);
        let shift = BigExponentFloat::from(shift);
        let y = (
            x.0.add_prec_round_ref_ref(&shift, prec, Floor).0,
            x.1.add_prec_round_ref_val(shift, prec, Ceiling).0,
        );
        (
            ln_gamma_stirling_encl(prec, terms, &y, &tol),
            encl_div(prec, &two_pi, &encl_mul(prec, &p, &p)),
        )
    };
    let (lo, hi) = ln_of_encl(&constant, prec);
    encl_add(prec, &stirling, &(lo >> 1u32, hi >> 1u32))
}

// The bits of |ln Gamma(y)| for y < 2^e: about e + lg e for a large y, lg |e| for a tiny one.
//
// This is equivalent to `gammaLnBits` from `AzFloat/GammaBracket.lean`, Azurite.
fn gamma_ln_bits(e: &Integer) -> u64 {
    match e.sign() {
        Greater => {
            let e = u64::exact_from(e);
            e + u64::from(e.ilog2()) + 2
        }
        Equal => 2,
        Less => e.significant_bits() + 1,
    }
}

// The enclosure of e^v (or of e^(-v), if `neg` is set) at Ziv level `w` from an enclosure `m` of v:
// the exponential at the lower end, with the upper end raised by 2d times itself, where d is the
// width of `m`, since e^d <= 1 + 2d for d <= 1. `None` when `m` is wider than 1.
//
// This is equivalent to `gammaExpEncl` and `expEnclPos?` from `AzFloat/GammaBracket.lean`, Azurite,
// with the correctly rounded exponential in place of Azurite's bracket.
fn gamma_exp_encl(neg: bool, m: Bracket, w: u64) -> Option<Bracket> {
    let m = if neg { (-m.1, -m.0) } else { m };
    let width = m.1.sub_prec_round_ref_ref(&m.0, 64, Ceiling).0;
    if width > 1u32 {
        // The bracket of ln Gamma is taken with as many more bits as |ln Gamma| has, so it is
        // narrow from the first Ziv level on.
        fail_on_untested_path("gamma_exp_encl, enclosure wider than 1");
        return None;
    }
    let prec = w + 16;
    let (lo, hi) = floor_bracket(m.0.exp_prec_round(prec, Floor).0, prec);
    let excess = hi.mul_prec_round_ref_val(width << 1u32, 64, Ceiling).0;
    Some((lo, hi.add_prec_round(excess, prec, Ceiling).0))
}

// The argument of the gamma function: a nonzero `BigExponentFloat` or a nonzero `Rational`, not an
// integer if negative.
pub(crate) enum GammaTarget<'a> {
    BigExponentFloat(&'a BigExponentFloat),
    Rational(&'a Rational),
}

impl GammaTarget<'_> {
    // An enclosure of x, for a positive x, or of 1 - x, for a negative x, at Ziv level `w`.
    //
    // This is equivalent to `lnGammaRatEncl` from `AzFloat/LnGammaBracket.lean`, Azurite, for a
    // `Rational`.
    fn shifted_encl(&self, pos: bool, w: u64) -> Bracket {
        let prec = w + 32;
        match self {
            Self::BigExponentFloat(x) => {
                if pos {
                    ((*x).clone(), (*x).clone())
                } else {
                    (
                        BigExponentFloat::ONE
                            .sub_prec_round_val_ref(x, prec, Floor)
                            .0,
                        BigExponentFloat::ONE
                            .sub_prec_round_val_ref(x, prec, Ceiling)
                            .0,
                    )
                }
            }
            Self::Rational(q) => {
                if pos {
                    (
                        BigExponentFloat::from_rational_prec_round_ref(q, prec, Floor).0,
                        BigExponentFloat::from_rational_prec_round_ref(q, prec, Ceiling).0,
                    )
                } else {
                    let q = Rational::ONE - *q;
                    (
                        BigExponentFloat::from_rational_prec_round_ref(&q, prec, Floor).0,
                        BigExponentFloat::from_rational_prec_round(q, prec, Ceiling).0,
                    )
                }
            }
        }
    }

    // For an x that is not an integer, and an integer k nearest to it: whether x - k is negative,
    // whether k is odd, and an enclosure of |x - k| at precision `prec`.
    //
    // This is equivalent to `gammaNearInt` and `gammaFracEncl` from `AzFloat/GammaBracket.lean` and
    // the first part of `gammaRatSinEncl` from `AzFloat/GammaRatBracket.lean`, Azurite.
    fn fractional_encl(&self, prec: u64) -> (bool, bool, Bracket) {
        match self {
            Self::BigExponentFloat(x) => {
                let k = Integer::rounding_from(*x, Nearest).0;
                let k_odd = k.odd();
                let d = *x - BigExponentFloat::from(k);
                let a = (&d).abs();
                (
                    d < 0u32,
                    k_odd,
                    (
                        a.round_to_prec_round_ref(prec, Floor).0,
                        a.round_to_prec_round_ref(prec, Ceiling).0,
                    ),
                )
            }
            Self::Rational(q) => {
                let k = Integer::rounding_from(*q, Nearest).0;
                let k_odd = k.odd();
                let d = *q - Rational::from(k);
                let a = (&d).abs();
                let prec = prec + 32;
                (
                    d < 0u32,
                    k_odd,
                    (
                        BigExponentFloat::from_rational_prec_round_ref(&a, prec, Floor).0,
                        BigExponentFloat::from_rational_prec_round(a, prec, Ceiling).0,
                    ),
                )
            }
        }
    }

    // Whether sin(pi x) is negative, and a positive enclosure of |sin(pi x)|, for an x that is not
    // an integer, at precision `prec`. With an integer k near x and d = x - k, sin(pi x) = (-1)^k
    // sin(pi d), which keeps its relative accuracy next to the integers; sin(pi |d|) is the sine at
    // the lower end of the enclosure of pi |d|, widened by the width of that enclosure, since the
    // sine is 1-Lipschitz. `None` when the enclosure is not positive.
    //
    // This is equivalent to `gammaSinEncl`, `gammaSinOfFrac`, and `sinOfEncl` from
    // `AzFloat/GammaBracket.lean` and `gammaRatSinEncl` from `AzFloat/GammaRatBracket.lean`,
    // Azurite, with the correctly rounded sine, whose accuracy is relative, in place of Azurite's
    // bracket, whose accuracy is absolute.
    fn sin_encl(&self, prec: u64) -> Option<(bool, Bracket)> {
        let (negative, k_odd, a) = self.fractional_encl(prec);
        let pi = floor_bracket(BigExponentFloat::pi_prec_round(prec, Floor).0, prec);
        let y = encl_mul(prec, &a, &pi);
        let width = y.1.sub_prec_round_ref_ref(&y.0, prec, Ceiling).0;
        let (mut lo, hi) = floor_bracket(y.0.sin_prec_round(prec, Floor).0, prec);
        lo.sub_prec_round_assign_ref(&width, prec, Floor);
        if lo > 0u32 {
            Some((
                negative != k_odd,
                (lo, hi.add_prec_round(width, prec, Ceiling).0),
            ))
        } else {
            // The correctly rounded sine has relative accuracy, and so has the enclosure of pi |d|,
            // so the widened enclosure stays positive.
            fail_on_untested_path("sin_encl, enclosure not positive");
            None
        }
    }
}

// Whether Gamma(x) is negative, and a positive enclosure of |Gamma(x)| (or of |1 / Gamma(x)|, if
// `recip` is set), at Ziv level `w`, for an x with |x| < 2^e that is positive (`pos`) or negative
// and not an integer. `None` when the pieces do not give one at this level.
//
// - x > 0: Gamma(x) = e^(ln Gamma(x)). The bracket of ln Gamma(x) is taken at `gamma_ln_bits` more
//   bits than the Ziv level, the size of |ln Gamma(x)|, since the exponential turns its absolute
//   error into a relative one.
// - x < 0: by the reflection formula Gamma(x) Gamma(1 - x) = pi / sin(pi x), with T = |sin(pi x)|
//   Gamma(1 - x), |Gamma(x)| = pi / T, and the sign is that of sin(pi x).
//
// This is equivalent to `gammaCore` and `gammaReflect` from `AzFloat/GammaBracket.lean` and
// `gammaRatCore` from `AzFloat/GammaRatBracket.lean`, Azurite.
fn gamma_core(
    recip: bool,
    target: &GammaTarget,
    e: &Integer,
    pos: bool,
    w: u64,
) -> Option<(bool, Bracket)> {
    if pos {
        let w_ln = w + gamma_ln_bits(e);
        let ln_gamma = ln_gamma_bracket_encl(&target.shifted_encl(true, w_ln), e, w_ln);
        return gamma_exp_encl(recip, ln_gamma, w).map(|b| (false, b));
    }
    // 1 - x < 2^e
    let e = if *e > 0u32 {
        e + Integer::ONE
    } else {
        Integer::ONE
    };
    let w_ln = w + gamma_ln_bits(&e);
    let ln_gamma = ln_gamma_bracket_encl(&target.shifted_encl(false, w_ln), &e, w_ln);
    let g = gamma_exp_encl(false, ln_gamma, w)?;
    let prec = w + 16;
    let (neg, s) = target.sin_encl(prec)?;
    let t = encl_mul(prec, &s, &g);
    let pi = floor_bracket(BigExponentFloat::pi_prec_round(prec, Floor).0, prec);
    Some((
        neg,
        if recip {
            encl_div(prec, &t, &pi)
        } else {
            encl_div(prec, &pi, &t)
        },
    ))
}

// The bracket of Gamma(x) or 1 / Gamma(x) at Ziv level `w`, strict on both sides: the end nearer to
// zero is moved towards zero, and the other one away from it, by a 2^(-2w)-th of themselves. When
// `gamma_core` gives nothing it is the dummy bracket [-1, 1], which Ziv's loop never accepts.
//
// This is equivalent to `gammaApprox`, `gammaApproxOf`, and `gammaStrict` from
// `AzFloat/GammaBracket.lean` and `gammaRatApprox` from `AzFloat/GammaRatBracket.lean`, Azurite.
fn gamma_approx(recip: bool, target: &GammaTarget, e: &Integer, pos: bool, w: u64) -> Bracket {
    let Some((neg, (mut lo, mut hi))) = gamma_core(recip, target, e, pos, w) else {
        return (BigExponentFloat::NEGATIVE_ONE, BigExponentFloat::ONE);
    };
    let prec = (w << 1) + 64;
    let shift = w << 1;
    let lo_margin = &lo >> shift;
    lo.sub_prec_round_assign(lo_margin, prec, Floor);
    let hi_margin = &hi >> shift;
    hi.add_prec_round_assign(hi_margin, prec, Ceiling);
    if neg { (-hi, -lo) } else { (lo, hi) }
}

// The largest integer argument whose gamma function is taken as an exact factorial at precision
// `prec`. Any bound of at least `prec` + 65 would do for the loop beyond it (the odd part of (n -
// 1)! then has more than `prec` + 1 bits, so the value is not a rounding boundary); this one is
// about where the factorial stops being cheaper than the loop.
//
// This is equivalent to `gammaIntBound` from `AzFloat/GammaBracket.lean`, Azurite.
const fn gamma_int_bound(prec: u64) -> u64 {
    prec.saturating_mul(8).saturating_add(2048)
}

// Gamma(n) = (n - 1)! or its reciprocal for a positive integer n, rounded once.
fn gamma_of_small_integer(
    recip: bool,
    n: u64,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    let f = Natural::factorial(n - 1);
    if recip {
        BigExponentFloat::from(f).reciprocal_prec_round(prec, rm)
    } else {
        BigExponentFloat::from_natural_prec_round(f, prec, rm)
    }
}

// The tiny-argument path of Gamma for a finite x with x < 2^e and precision `x_prec`: when e <=
// -(`prec` + 72), -Gamma(x) lies in (-1 / x, -1 / x + 2), so it is rounded (in the negated mode,
// the result negated) by one test of `rounding_possible`. The negation puts -1 / x, which is a
// float when x is a power of 2, at the open side of the half-open test: Ziv's loop would need about
// |e| bits to see on which side of it the value lies. `None` when the test fails.
//
// This is equivalent to `gammaTiny?` from `AzFloat/GammaBracket.lean`, Azurite.
fn gamma_tiny(
    x: &BigExponentFloat,
    x_prec: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(BigExponentFloat, Ordering)> {
    let w = prec + x_prec + ZIV_GUARD_BITS + 8;
    let lo = BigExponentFloat::NEGATIVE_ONE
        .div_prec_round_val_ref(x, w, Floor)
        .0;
    let hi = BigExponentFloat::NEGATIVE_ONE
        .div_prec_round_val_ref(x, w, Ceiling)
        .0
        .add_prec_round(BigExponentFloat::TWO, w, Ceiling)
        .0;
    rounding_possible(
        &|p, m| lo.round_to_prec_round_ref(p, m),
        &|p, m| hi.round_to_prec_round_ref(p, m),
        prec,
        -rm,
    )
    .map(|(r, o)| (-r, o.reverse()))
}

// The tiny-argument path of 1 / Gamma for a finite x with |x| < 2^e and precision `x_prec`: when e
// <= -(max(`prec`, `x_prec`) + 72), 1 / Gamma(x) lies in (x, x + x^2), so it is rounded by one test
// of `rounding_possible`, the exact end x at the open side. `None` when the argument is not tiny or
// the test fails.
//
// This is equivalent to `reciprocalGammaTiny?` from `AzFloat/GammaBracket.lean`, Azurite.
fn reciprocal_gamma_tiny(
    x: &BigExponentFloat,
    e: &Integer,
    x_prec: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(BigExponentFloat, Ordering)> {
    let w = max(prec, x_prec) + ZIV_GUARD_BITS + 8;
    if *e > -Integer::from(w) {
        return None;
    }
    let hi = x
        .add_prec_round_ref_val(x.abs() * power_of_2_integer(e.clone()), w, Ceiling)
        .0;
    rounding_possible(
        &|p, m| x.round_to_prec_round_ref(p, m),
        &|p, m| hi.round_to_prec_round_ref(p, m),
        prec,
        rm,
    )
}

// Gamma(x), or 1 / Gamma(x) if `recip` is set, rounded to `prec` bits with `rm`.
//
// This is equivalent to `gammaPrecRound` and `reciprocalGammaPrecRound` from `AzFloat/Gamma.lean`,
// Azurite, except that Gamma(0) is NaN, that there is no shortcut through sqrt(pi) and Gamma(1/4)
// at the quarter-integers, and that Ziv's loop has no fuel.
pub(crate) fn gamma_helper(
    recip: bool,
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    match x {
        BigExponentFloat(Infinity { sign: true }) => (
            if recip {
                BigExponentFloat::ZERO
            } else {
                BigExponentFloat::INFINITY
            },
            Equal,
        ),
        BigExponentFloat(NaN | Infinity { sign: false }) => (BigExponentFloat::NAN, Equal),
        BigExponentFloat(Zero) => (
            if recip {
                BigExponentFloat::ZERO
            } else {
                BigExponentFloat::NAN
            },
            Equal,
        ),
        BigExponentFloat(Finite {
            sign,
            exponent,
            significand,
        }) => {
            if x.is_integer() {
                if !*sign {
                    return (
                        if recip {
                            BigExponentFloat::ZERO
                        } else {
                            BigExponentFloat::NAN
                        },
                        Equal,
                    );
                }
                if *x <= gamma_int_bound(prec) {
                    let n = u64::exact_from(&Natural::rounding_from(x, Exact).0);
                    return gamma_of_small_integer(recip, n, prec, rm);
                }
            }
            if recip {
                assert_ne!(rm, Exact, "Inexact reciprocal gamma function");
            } else {
                assert_ne!(rm, Exact, "Inexact gamma function");
            }
            let x_prec = significand_prec(significand);
            let tiny = if recip {
                reciprocal_gamma_tiny(x, exponent, x_prec, prec, rm)
            } else if *exponent <= -Integer::from(prec + ZIV_GUARD_BITS + 8) {
                gamma_tiny(x, x_prec, prec, rm)
            } else {
                None
            };
            if let Some(r) = tiny {
                return r;
            }
            let target = GammaTarget::BigExponentFloat(x);
            ziv_loop_certified_from(
                &|w| gamma_approx(recip, &target, exponent, *sign, w),
                prec,
                rm,
                max(64, prec + 32),
            )
        }
    }
}

// Whether q is tiny at precision `prec`: |q| < 2^e with e <= -(`prec` + 72).
//
// This is equivalent to `gammaRatIsTiny` from `AzFloat/GammaRat.lean`, Azurite.
fn gamma_rational_is_tiny(e: &Integer, prec: u64) -> bool {
    *e <= -Integer::from(prec + ZIV_GUARD_BITS + 8)
}

// The tiny-argument paths of Gamma(q) and 1 / Gamma(q): -Gamma(q) lies in (-1 / q, -1 / q + 2),
// rounded in the negated mode from the outward roundings of the two rational ends, and 1 / Gamma(q)
// lies in (q, q + q^2). The test fails when a rounding boundary lies between the two ends; then the
// value has to be known to about |e| bits beyond the precision.
//
// This is equivalent to `gammaRatTiny?` and `reciprocalGammaRatTiny?` from `AzFloat/GammaRat.lean`,
// Azurite.
fn gamma_rational_tiny(
    recip: bool,
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> Option<(BigExponentFloat, Ordering)> {
    let w = prec + ZIV_GUARD_BITS + 8;
    if recip {
        let lo = BigExponentFloat::from_rational_prec_round_ref(q, w, Floor).0;
        let hi = BigExponentFloat::from_rational_prec_round(q * (q + Rational::ONE), w, Ceiling).0;
        rounding_possible(
            &|p, m| lo.round_to_prec_round_ref(p, m),
            &|p, m| hi.round_to_prec_round_ref(p, m),
            prec,
            rm,
        )
    } else {
        let r = -Rational::ONE / q;
        let lo = BigExponentFloat::from_rational_prec_round_ref(&r, w, Floor).0;
        let hi = BigExponentFloat::from_rational_prec_round(r + Rational::TWO, w, Ceiling).0;
        rounding_possible(
            &|p, m| lo.round_to_prec_round_ref(p, m),
            &|p, m| hi.round_to_prec_round_ref(p, m),
            prec,
            -rm,
        )
        .map(|(r, o)| (-r, o.reverse()))
    }
}

// Gamma(q), or 1 / Gamma(q) if `recip` is set, for a `Rational` q, rounded to `prec` bits with
// `rm`.
//
// This is equivalent to `gammaRatPrecRound`, `reciprocalGammaRatPrecRound`, and `gammaRatStart`
// from `AzFloat/GammaRat.lean` and `gammaRatExp` from `AzFloat/GammaRatBracket.lean`, Azurite,
// except that Gamma(0) is NaN, that there is no shortcut through sqrt(pi), Gamma(1/4), and
// Gamma(1/3) for the denominators 2, 3, 4, and 6, and that Ziv's loop has no fuel.
pub(crate) fn gamma_rational_helper(
    recip: bool,
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(prec, 0);
    let pos = *q > 0u32;
    let is_integer = q.is_integer();
    if !pos && is_integer {
        return (
            if recip {
                BigExponentFloat::ZERO
            } else {
                BigExponentFloat::NAN
            },
            Equal,
        );
    }
    if is_integer && *q <= gamma_int_bound(prec) {
        return gamma_of_small_integer(recip, u64::exact_from(q.numerator_ref()), prec, rm);
    }
    if recip {
        assert_ne!(rm, Exact, "Inexact reciprocal gamma function");
    } else {
        assert_ne!(rm, Exact, "Inexact gamma function");
    }
    // |q| < 2^e, from the rounding of q away from zero.
    let e = BigExponentFloat::from_rational_prec_round_ref(q, 32, Up)
        .0
        .get_exponent()
        .unwrap()
        .clone();
    let mut start = max(64, prec + 32);
    if !is_integer && gamma_rational_is_tiny(&e, prec) {
        if let Some(r) = gamma_rational_tiny(recip, q, prec, rm) {
            return r;
        }
        // The value has to be known to about |e| bits beyond the precision.
        start.saturating_add_assign(u64::saturating_from(&-&e));
    }
    let target = GammaTarget::Rational(q);
    ziv_loop_certified_from(
        &|w| gamma_approx(recip, &target, &e, pos, w),
        prec,
        rm,
        start,
    )
}

impl BigExponentFloat {
    /// Computes $\Gamma(x)$, the gamma function, of a [`BigExponentFloat`], rounding the result to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p+1}$.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gamma_prec_round), this function never
    /// overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every $x$, and this function
    /// would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::gamma_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^3 \log (n + m))$
    ///
    /// $M(n, m) = O((n + m)^2 \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the exponent of $x$ if that is positive, and 0 otherwise.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is a
    /// positive integer and $(x-1)!$ fits in the precision).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec_round(5, Floor);
    /// assert_eq!(g.to_string(), "1.8");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec_round(5, Ceiling);
    /// assert_eq!(g.to_string(), "1.81");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec_round(5, Nearest);
    /// assert_eq!(g.to_string(), "1.8");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec_round(20, Floor);
    /// assert_eq!(g.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec_round(20, Ceiling);
    /// assert_eq!(g.to_string(), "1.7724552");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec_round(20, Nearest);
    /// assert_eq!(g.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gamma_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        gamma_helper(false, &self, prec, rm)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`BigExponentFloat`], rounding the result to
    /// the specified precision and with the specified rounding mode. The [`BigExponentFloat`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p+1}$.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gamma_prec_round), this function never
    /// overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every $x$, and this function
    /// would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::gamma_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^3 \log (n + m))$
    ///
    /// $M(n, m) = O((n + m)^2 \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the exponent of $x$ if that is positive, and 0 otherwise.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is a
    /// positive integer and $(x-1)!$ fits in the precision).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec_round_ref(5, Floor);
    /// assert_eq!(g.to_string(), "1.8");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec_round_ref(5, Ceiling);
    /// assert_eq!(g.to_string(), "1.81");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec_round_ref(5, Nearest);
    /// assert_eq!(g.to_string(), "1.8");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec_round_ref(20, Floor);
    /// assert_eq!(g.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec_round_ref(20, Ceiling);
    /// assert_eq!(g.to_string(), "1.7724552");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec_round_ref(20, Nearest);
    /// assert_eq!(g.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gamma_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        gamma_helper(false, self, prec, rm)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`BigExponentFloat`] in place, rounding the
    /// result to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p+1}$.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::gamma_prec_round`] documentation for information on special
    /// cases, running time, and termination.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::gamma_prec_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^3 \log (n + m))$
    ///
    /// $M(n, m) = O((n + m)^2 \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the exponent of $x$ if that is positive, and 0 otherwise.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is a
    /// positive integer and $(x-1)!$ fits in the precision).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.gamma_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.8");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.gamma_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.81");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.gamma_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.8");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.gamma_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.772453");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.gamma_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.7724552");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.gamma_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.772453");
    /// ```
    #[inline]
    pub fn gamma_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (mut g, o) = gamma_helper(false, self, prec, rm);
        swap(self, &mut g);
        o
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`BigExponentFloat`], rounding the result to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gamma_prec_round), this function never
    /// overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every $x$, and this function
    /// would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::gamma_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^3 \log (n + m))$
    ///
    /// $M(n, m) = O((n + m)^2 \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the exponent of $x$ if that is positive, and 0 otherwise.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec(5);
    /// assert_eq!(g.to_string(), "1.8");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec(20);
    /// assert_eq!(g.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::from(1.0e9).gamma_prec(20);
    /// assert_eq!(g.to_string(), "9.9046270e8565705513");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn gamma_prec(self, prec: u64) -> (Self, Ordering) {
        self.gamma_prec_round(prec, Nearest)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`BigExponentFloat`], rounding the result to
    /// the nearest value of the specified precision. The [`BigExponentFloat`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gamma_prec_round), this function never
    /// overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when it is another multiple of $1/4$ (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every $x$, and this function
    /// would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::gamma_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^3 \log (n + m))$
    ///
    /// $M(n, m) = O((n + m)^2 \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the exponent of $x$ if that is positive, and 0 otherwise.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec_ref(5);
    /// assert_eq!(g.to_string(), "1.8");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::ONE_HALF.gamma_prec_ref(20);
    /// assert_eq!(g.to_string(), "1.772453");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::from(1.0e9).gamma_prec_ref(20);
    /// assert_eq!(g.to_string(), "9.9046270e8565705513");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn gamma_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.gamma_prec_round_ref(prec, Nearest)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`BigExponentFloat`] in place, rounding the
    /// result to the nearest value of the specified precision. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\Gamma(x)|\rfloor-p}$.
    ///
    /// See the [`BigExponentFloat::gamma_prec_round`] documentation for information on special
    /// cases, running time, and termination.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::gamma_prec_round_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n + m)^3 \log (n + m))$
    ///
    /// $M(n, m) = O((n + m)^2 \log (n + m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(prec, self.significant_bits())`,
    /// and $m$ is the exponent of $x$ if that is positive, and 0 otherwise.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.gamma_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "1.8");
    ///
    /// let mut x = BigExponentFloat::ONE_HALF;
    /// assert_eq!(x.gamma_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.772453");
    ///
    /// let mut x = BigExponentFloat::from(1.0e9);
    /// assert_eq!(x.gamma_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "9.9046270e8565705513");
    /// ```
    #[inline]
    pub fn gamma_prec_assign(&mut self, prec: u64) -> Ordering {
        self.gamma_prec_round_assign(prec, Nearest)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p+1}$.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gamma_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every rational $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::gamma_rational_prec`] instead.
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
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is a
    /// positive integer and $(x-1)!$ fits in the precision).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) =
    ///     BigExponentFloat::gamma_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(g.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "2.6789360");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "2.678940");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::gamma_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "2.678940");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn gamma_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        gamma_rational_helper(false, &x, prec, rm)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode and returning the result as a
    /// [`BigExponentFloat`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p+1}$.
    /// - If $\Gamma(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gamma_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every rational $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::gamma_rational_prec_ref`] instead.
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
    /// with the given precision (it can only when it is infinite or `NaN`, or when $x$ is a
    /// positive integer and $(x-1)!$ fits in the precision).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (g, o) = BigExponentFloat::gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(g.to_string(), "2.6789360");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(g.to_string(), "2.678940");
    /// assert_eq!(o, Greater);
    ///
    /// let (g, o) = BigExponentFloat::gamma_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(g.to_string(), "2.678940");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn gamma_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        gamma_rational_helper(false, x, prec, rm)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision and returning the result as a [`BigExponentFloat`].
    /// The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=\text{NaN}$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gamma_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every rational $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::gamma_rational_prec_round`] instead.
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
    /// let (g, o) = BigExponentFloat::gamma_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(g.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) = BigExponentFloat::gamma_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(g.to_string(), "2.678940");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn gamma_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::gamma_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\Gamma(x)$, the gamma function, of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision and returning the result as a [`BigExponentFloat`].
    /// The [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded value is less than, equal to, or greater than the exact value. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $\Gamma(x) = \int_0^\infty t^{x-1} e^{-t}\,dt$ for $x > 0$, and the recurrence $\Gamma(x) =
    /// \Gamma(x+1)/x$ extends it to every real $x$ other than $0, -1, -2, \ldots$, where it has
    /// poles. If $n$ is a positive integer, $\Gamma(n) = (n-1)!$.
    ///
    /// If the value is equidistant from two [`BigExponentFloat`]s with the specified precision, the
    /// [`BigExponentFloat`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`]
    /// for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \Gamma(x)+\varepsilon.
    /// $$
    /// - If $\Gamma(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\Gamma(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\Gamma(x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=\text{NaN}$ if $x$ is 0 or a negative integer
    ///
    /// Unlike [its `Float` counterpart](crate::Float::gamma_rational_prec_round), this function
    /// never overflows or underflows.
    ///
    /// Warning: the absolute value of $\ln \Gamma(x)$ is about $|x| \ln |x|$ for a large $|x|$, so
    /// an argument with exponent $e > 0$ takes about $e$ bits of working precision beyond `prec`,
    /// and gives a result whose exponent has about $e$ bits. The time and memory needed therefore
    /// grow with the exponent, without any limit: an argument near $2^{2^{40}}$ needs more than
    /// $2^{40}$ bits, or 128 GiB.
    ///
    /// The result is correctly rounded. Finding it takes $\Gamma(x)$ not to be a rounding boundary,
    /// which is a number with at most `prec + 1` significant bits. This is known when $x$ is an
    /// integer, and when its denominator is 2, 3, 4, or 6 (then $\Gamma(x)$ is irrational); nothing
    /// suggests that it ever fails, but it has not been proven for every rational $x$, and this
    /// function would not terminate if it did fail.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::gamma_rational_prec_round_ref`] instead.
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
    ///     BigExponentFloat::gamma_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(g.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (g, o) =
    ///     BigExponentFloat::gamma_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(g.to_string(), "2.678940");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn gamma_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::gamma_rational_prec_round_ref(x, prec, Nearest)
    }
}
