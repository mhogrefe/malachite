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
    rounding_possible, strict_bracket, ziv_loop_certified_from,
};
use crate::big_exponent_float::exponential::exp::power_of_2_integer;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::cmp::Ordering::{self, *};
use core::cmp::{max, min};
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    Abs, AddMulAssign, Parity, Pow, PowerOf2, SaturatingAddAssign, Sign,
};
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeOne, One, OneHalf, Two, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger, RoundingFrom, SaturatingFrom};
use malachite_base::num::exponential::traits::CeilingLogBase2;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::number_theory::traits::Factorial;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::number_theory::tangent_numbers::tangent_numbers_prefix;
use malachite_q::Rational;

// The threshold from which Stirling's series is used at Ziv level `w`, for an argument with
// `x_bits` significant bits: an argument below the threshold is shifted up to it. A larger
// threshold costs more in the rising product and saves terms of the series, and with them tangent
// numbers, whose cost grows with the cube of their number.
//
// The threshold is c * w / 16, with c chosen by timing (2026-11-20). Up to about 1250 bits c = 6 is
// best. Beyond, the tangent numbers come to dominate and a larger c pays: 8 from there on and 10
// from 2560 bits on; and 14 and 20 for an argument much shorter than the working precision, whose
// rising product is cheap, since its factors are short.
//
// This replaces `lnGammaShiftExp` from `AzFloat/LnGammaBracket.lean`, Azurite, where the threshold
// is the power of 2 in (w / 4, w / 2].
const fn ln_gamma_shift_threshold(w: u64, x_bits: u64) -> u64 {
    let c = if w < 1280 {
        6
    } else {
        let c = 6 + (((w / 640).ilog2() as u64) << 1);
        if x_bits <= w >> 2 { 3 * c - 10 } else { c }
    };
    (w * c) >> 4
}

// The number of significant bits of the lower end of an enclosure, if the enclosure is a single
// number, and otherwise as many as the Ziv level `w`: what `ln_gamma_shift_threshold` goes by.
fn enclosure_bits(x: &Bracket, w: u64) -> u64 {
    if x.0 == x.1 {
        x.0.get_prec().unwrap_or(1)
    } else {
        w
    }
}

// The shift d for a positive x with x < 2^e (`x_lo` is x, or the lower end of an enclosure of it):
// x + d >= t when 2^(e - 1) <= x, and d = 0 only when x >= 1.
//
// This is equivalent to `lnGammaShift` from `AzFloat/LnGammaBracket.lean`, Azurite, with a
// threshold that need not be a power of 2.
fn ln_gamma_shift(x_lo: &BigExponentFloat, e: &Integer, t: u64) -> u64 {
    let s = if *e > 64u32 {
        0
    } else if *e > 0u32 {
        t.saturating_sub(u64::power_of_2(u64::exact_from(e) - 1))
    } else {
        t
    };
    if s == 0 && *x_lo < 1u32 {
        // Every caller passes the exponent of x itself, or one more, so x >= 1 when there is no
        // shift.
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
pub(crate) fn ln_of_encl(y: &Bracket, prec: u64) -> Bracket {
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
// This is equivalent to `lnGammaBracketEncl` from `AzFloat/LnGammaBracket.lean`, Azurite. The gamma
// function uses `gamma_positive_approx` instead, which is faster; this one, where every step is an
// operation on enclosures, is kept for the tests and the benchmarks.
fn ln_gamma_bracket_enclosures(x: &Bracket, e: &Integer, w: u64) -> Bracket {
    let threshold = ln_gamma_shift_threshold(w, enclosure_bits(x, w));
    let prec = w + (threshold.ceiling_log_base_2() << 1) + 8 + (u64::from(w.ilog2()) << 1);
    let shift = ln_gamma_shift(&x.0, e, threshold);
    let terms = ln_gamma_terms(
        w,
        max(
            threshold.significant_bits(),
            if *e > 0u32 { u64::exact_from(e) } else { 0 },
        ),
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

// A positive real number v, known through a computed value x and a count k: v = x * (1 + t) for
// some t with |t| <= k * u, where u = 2^(-prec) and `prec` is the working precision of the
// computation that the value is part of. Values are combined by multiplication and division,
// rounded to nearest, and the counts add up: with z the exact product or quotient of the computed
// values and x its rounding, z = x * (1 + r) with |r| <= u, so the true product or quotient is x *
// (1 + r) * (1 + t_a) * (1 + t_b)^(+-1). With s = (k_a + k_b + 1) * u, that factor is at least 1 -
// s, and at most 1 + s + s^2 for a product and 1 + s + 4s^2 for a quotient (using 1 / (1 - b) <= 1
// + b + 2b^2 for b <= 1/2); and 4s^2 <= u as long as the counts stay below 2^(prec / 2 - 2), which
// `rel_count` checks. So the count of the result is k_a + k_b + 2.
struct Rel {
    x: BigExponentFloat,
    k: u64,
}

// Checks that a count is small enough for the first-order bounds to hold: less than 2^(prec / 2 -
// 2), and less than 2^62. In particular k * u <= 1/2.
fn rel_count(k: Option<u64>, prec: u64) -> Option<u64> {
    if let Some(k) = k
        && k.significant_bits() <= min((prec >> 1) - 2, 62)
    {
        Some(k)
    } else {
        // The counts are a few times the number of operations, which is far less than 2^(prec / 2).
        fail_on_untested_path("rel_count, the count is too large");
        None
    }
}

// The count of a value computed from values with the counts `k_a` and `k_b` by one operation whose
// rounding was `o`.
fn rel_combine(k_a: u64, k_b: u64, o: Ordering, prec: u64) -> Option<u64> {
    if k_a == 0 && k_b == 0 && o == Equal {
        Some(0)
    } else {
        rel_count(k_a.checked_add(k_b).and_then(|k| k.checked_add(2)), prec)
    }
}

fn rel_mul(a: &Rel, b: &Rel, prec: u64) -> Option<Rel> {
    let (x, o) = a.x.mul_prec_ref_ref(&b.x, prec);
    Some(Rel {
        x,
        k: rel_combine(a.k, b.k, o, prec)?,
    })
}

fn rel_div(a: &Rel, b: &Rel, prec: u64) -> Option<Rel> {
    let (x, o) = a.x.div_prec_ref_ref(&b.x, prec);
    Some(Rel {
        x,
        k: rel_combine(a.k, b.k, o, prec)?,
    })
}

// a + n for a natural number n. With a = x * (1 + t), a + n = (x + n) * (1 + t') with |t'| <= |t|,
// and x + n is rounded once.
fn rel_add_unsigned(a: &Rel, n: u64, prec: u64) -> Option<Rel> {
    let (x, o) = a.x.add_prec_ref_val(BigExponentFloat::from(n), prec);
    Some(Rel {
        x,
        k: rel_combine(a.k, 0, o, prec)?,
    })
}

// A positive real number, given by an enclosure with a positive lower end, as a `Rel` at precision
// `prec`: the lower end, with the relative width of the enclosure rounded up to a power of 2.
fn rel_from_bracket(x: &Bracket, prec: u64) -> Option<Rel> {
    if x.0 == x.1 {
        return Some(Rel {
            x: x.0.clone(),
            k: 0,
        });
    }
    let width =
        x.1.sub_prec_round_ref_ref(&x.0, 32, Ceiling)
            .0
            .div_prec_round_val_ref(&x.0, 32, Ceiling)
            .0;
    // The relative width is less than 2^e, which is 2^(e + prec) * u.
    let e = width.get_exponent().unwrap() + Integer::from(prec);
    let k = if e <= 0u32 {
        Some(1)
    } else if e <= 62u32 {
        Some(u64::power_of_2(u64::exact_from(&e)))
    } else {
        None
    };
    Some(Rel {
        x: x.0.clone(),
        k: rel_count(k, prec)?,
    })
}

// An upper bound for the absolute error of a computed value, as a sum of terms k * 2^e.
struct ErrorBound(Vec<(u64, Integer)>);

impl ErrorBound {
    fn push(&mut self, k: u64, e: Integer) {
        if k != 0 {
            self.0.push((k, e));
        }
    }

    // The error of a value x that was rounded to nearest at `prec` bits, with the result of the
    // rounding `o`: at most half a unit in the last place, which is less than 2^(e - prec) when |x|
    // < 2^e.
    fn push_rounding(&mut self, x: &BigExponentFloat, o: Ordering, prec: u64) {
        if o != Equal {
            self.push(1, x.get_exponent().unwrap() - Integer::from(prec));
        }
    }

    // A number that is at least the sum of the terms. With E the largest value of e + (the number
    // of bits of k), every term is less than 2^E; the terms are rounded up to multiples of 2^(E -
    // 40) and added.
    fn total(&self) -> BigExponentFloat {
        let Some(top) = self
            .0
            .iter()
            .map(|(k, e)| e + Integer::from(k.significant_bits()))
            .max()
        else {
            // The bracket of ln Gamma always has terms: 2 pi is not exact.
            fail_on_untested_path("ErrorBound::total, no terms");
            return BigExponentFloat::ZERO;
        };
        let unit = top - const { Integer::const_from_unsigned(40) };
        let mut count = Natural::ZERO;
        for (k, e) in &self.0 {
            // d <= 40 - (the number of bits of k)
            let d = i64::saturating_from(&(e - &unit));
            count += Natural::from(if d >= 0 {
                k << d
            } else {
                (k >> min(d.unsigned_abs(), 63)) + 1
            });
        }
        BigExponentFloat::from(count) * power_of_2_integer(unit)
    }
}

// The tangent numbers T_1, T_2, ..., computed one at a time, as they are asked for, and kept.
//
// Algorithm 4.3 of Brent and Zimmermann, _Modern Computer Arithmetic_ (see
// `tangent_numbers_prefix`), starts from T_j = (j - 1)! and makes passes k = 2, 3, ... over the
// table, replacing T_j by (j - k) T_(j - 1) + (j - k + 2) T_j for j >= k; T_j is final after pass
// j. Here the table is filled column by column instead of pass by pass: the value of T_j after pass
// k only depends on the value of T_(j - 1) after pass k and on that of T_j after pass k - 1, so the
// values that the last T_j took in each pass are all that is needed to compute the next one.
struct TangentNumbers {
    ts: Vec<Natural>,
    // The value of the last tangent number after each of the passes 1, 2, ..., in which it was
    // changed; the last of them is its final value.
    column: Vec<Natural>,
}

impl TangentNumbers {
    const fn new() -> Self {
        Self {
            ts: Vec::new(),
            column: Vec::new(),
        }
    }

    // T_j, for j >= 1.
    fn get(&mut self, j: u64) -> &Natural {
        let j = usize::exact_from(j);
        while self.ts.len() < j {
            // the index of the new tangent number
            let c = self.ts.len() + 1;
            if c == 1 {
                self.column.push(Natural::ONE);
            } else {
                self.column[0] *= Natural::from(c - 1);
                for k in 2..c {
                    let (lo, hi) = self.column.split_at_mut(k - 1);
                    let t = &mut hi[0];
                    *t *= Natural::from(c - k);
                    t.add_mul_assign(&lo[k - 2], Natural::from(c - k + 2));
                }
                let last = &self.column[c - 2] << 1u32;
                self.column.push(last);
            }
            self.ts.push(self.column[c - 1].clone());
        }
        &self.ts[j - 1]
    }
}

// With the `std` feature, the tangent numbers that this thread has computed are kept from one call
// to the next: `LnGammaState` takes the table when it is made and puts it back when it is dropped.
// The numbers are exact, so the results do not depend on the cache.
#[cfg(feature = "std")]
std::thread_local! {
    static TANGENT_NUMBERS: RefCell<TangentNumbers> = const { RefCell::new(TangentNumbers::new()) };
}

// The table of tangent numbers that this thread has kept, which is left empty.
fn take_cached_tangent_numbers() -> TangentNumbers {
    #[cfg(feature = "std")]
    {
        // `try_with` fails only while the thread is being torn down; then nothing is cached.
        TANGENT_NUMBERS
            .try_with(|cache| core::mem::replace(&mut *cache.borrow_mut(), TangentNumbers::new()))
            .unwrap_or_else(|_| TangentNumbers::new())
    }
    #[cfg(not(feature = "std"))]
    {
        TangentNumbers::new()
    }
}

// Keeps a table of tangent numbers for this thread, unless a longer one is kept already.
#[allow(clippy::needless_pass_by_value)]
fn cache_tangent_numbers(tangents: TangentNumbers) {
    #[cfg(feature = "std")]
    {
        let _ = TANGENT_NUMBERS.try_with(|cache| {
            let mut cache = cache.borrow_mut();
            if cache.ts.len() < tangents.ts.len() {
                *cache = tangents;
            }
        });
    }
    #[cfg(not(feature = "std"))]
    {
        let _ = tangents;
    }
}

// Frees the tangent numbers that this thread has kept.
pub(crate) fn clear_tangent_number_cache() {
    drop(take_cached_tangent_numbers());
}

crate_test_fn! {
    // The number of tangent numbers that this thread has kept.
    cached_tangent_number_count() -> usize {
        let tangents = take_cached_tangent_numbers();
        let count = tangents.ts.len();
        cache_tangent_numbers(tangents);
        count
    }
}

crate_test_fn! {
    // The first `len` tangent numbers, from `TangentNumbers`.
    tangent_numbers_one_at_a_time(len: u64) -> Vec<Natural> {
        let mut ts = TangentNumbers::new();
        (1..=len).map(|j| ts.get(j).clone()).collect()
    }
}

// The rising product x * (x + 1) * ... * (x + n - 1) of a positive x, at precision `prec`.
//
// (Multiplying the factors out in blocks, as polynomials in x that are evaluated from its powers,
// was tried and timed on 2026-11-20: it was slower up to 2000 bits, and for an argument shorter
// than the working precision at every precision, and 15 to 18% faster only from 3000 bits on.)
//
// This replaces `risingEncl` from `AzFloat/LnGammaBracket.lean`, Azurite.
fn rising_rel(prec: u64, x: &Rel, n: u64) -> Option<Rel> {
    let mut product = Rel {
        x: BigExponentFloat::ONE,
        k: 0,
    };
    for i in 0..n {
        product = rel_mul(&product, &rel_add_unsigned(x, i, prec)?, prec)?;
    }
    Some(product)
}

// An approximation of mu(y) = ln Gamma(y) - (y - 1/2) ln y + y - ln(2 pi) / 2 for y >= 1, by
// Stirling's series a_1 / y - a_2 / y^3 + a_3 / y^5 - ..., where a_k = T_k / ((2k - 1) * (4^(2k) -
// 2^(2k))), using at most `m` >= 1 tangent numbers. A bound for its error is added to `errors`. A
// term below `tol`, or the last one of the table, is not added: the remainder lies between 0 and
// it, so it only enters the error bound. So the bound is valid whatever the number of terms is.
//
// This replaces `stirlingMuEncl`, `stirlingSumLoop`, and `stirlingTermEncl` from
// `AzFloat/LnGammaBracket.lean`, Azurite.
fn stirling_mu_approx(
    prec: u64,
    m: u64,
    y: &Rel,
    tol: &BigExponentFloat,
    tangents: &RefCell<TangentNumbers>,
    errors: &mut ErrorBound,
) -> Option<BigExponentFloat> {
    let one = Rel {
        x: BigExponentFloat::ONE,
        k: 0,
    };
    // y^(-(2k - 1))
    let mut p = rel_div(&one, y, prec)?;
    let inverse_square = rel_mul(&p, &p, prec)?;
    let prec_integer = Integer::from(prec);
    let mut sum = BigExponentFloat::ZERO;
    for k in 1..=m {
        let numerator = Rel {
            x: BigExponentFloat::from(tangents.borrow_mut().get(k)),
            k: 0,
        };
        let denominator = Rel {
            x: BigExponentFloat::from(
                (Natural::power_of_2(k << 2) - Natural::power_of_2(k << 1))
                    * Natural::from((k << 1) - 1),
            ),
            k: 0,
        };
        let term = rel_div(&rel_mul(&p, &numerator, prec)?, &denominator, prec)?;
        let e = term.x.get_exponent().unwrap();
        if k == m || term.x <= *tol {
            // The term is less than 2^e, so its true value is less than 2^(e + 1).
            errors.push(1, e + Integer::ONE);
            break;
        }
        errors.push(term.k, e - &prec_integer);
        let o = if k.odd() {
            sum.add_prec_assign(term.x, prec)
        } else {
            sum.sub_prec_assign(term.x, prec)
        };
        errors.push_rounding(&sum, o, prec);
        p = rel_mul(&p, &inverse_square, prec)?;
    }
    Some(sum)
}

// The main part of Stirling's formula at Ziv level `w` >= 64, for an enclosure `x` of x > 0, with a
// positive lower end, where x < 2^e. With the shift d (see `ln_gamma_shift`), y = x + d, and the
// rising product P = x * (x + 1) * ... * (x + d - 1),
//
// ln Gamma(x) = S + ln(2 pi) / 2 - ln P, where S = (y - 1/2) ln y - y + mu(y).
//
// The formula is the one of `ln_gamma_bracket_enclosures`, with the same shift, number of terms,
// and precision, but it is evaluated once, with rounding to nearest, instead of twice, with
// rounding down and up. The factors and quotients (the argument, the rising product, the powers of
// 1 / y, and the terms of Stirling's series) are `Rel`s, which count their relative errors; S is
// formed from them with an `ErrorBound` for its absolute error.
//
// With y = y' * (1 + t), |t| <= k * u, and u = 2^(-prec):
// - ln y = ln y' + ln(1 + t), and |ln(1 + t)| <= 2 * k * u, since k * u <= 1/2. With L the computed
//   logarithm, |L| < 2^l, the error dL of L is at most 2 * k * u, plus that of its rounding.
// - y - 1/2 = h + dh, where h is the computed difference, |h| < 2^h, and |dh| <= k * u * y' plus
//   the rounding of h.
// - (y - 1/2) * ln y - h * L = dh * ln y + h * dL, and |ln y| < 2^(max(l, 0) + 1).
struct StirlingParts {
    // The working precision.
    prec: u64,
    // S, with a bound for its error.
    sum: BigExponentFloat,
    errors: ErrorBound,
    // P, if the argument was shifted.
    product: Option<Rel>,
}

// `None` when a count gets too large, which does not happen.
fn stirling_parts(
    x: &Bracket,
    e: &Integer,
    w: u64,
    tangents: &RefCell<TangentNumbers>,
) -> Option<StirlingParts> {
    let threshold = ln_gamma_shift_threshold(w, enclosure_bits(x, w));
    let prec = w + (threshold.ceiling_log_base_2() << 1) + 8 + (u64::from(w.ilog2()) << 1);
    let prec_integer = Integer::from(prec);
    let shift = ln_gamma_shift(&x.0, e, threshold);
    let terms = ln_gamma_terms(
        w,
        max(
            threshold.significant_bits(),
            if *e > 0u32 { u64::exact_from(e) } else { 0 },
        ),
    );
    let tol = power_of_2_integer(-Integer::from(w + 16));
    let x = rel_from_bracket(x, prec)?;
    let (y, product) = if shift == 0 {
        (x, None)
    } else {
        (
            rel_add_unsigned(&x, shift, prec)?,
            Some(rising_rel(prec, &x, shift)?),
        )
    };
    let mut errors = ErrorBound(Vec::new());
    let y_exponent = y.x.get_exponent().unwrap();
    // (y - 1/2) * ln y
    let (ln_y, ln_y_o) = y.x.ln_prec_ref(prec);
    let (h, h_o) = y.x.sub_prec_ref_val(BigExponentFloat::ONE_HALF, prec);
    if h <= 0u32 {
        // y >= 1
        fail_on_untested_path("stirling_parts, y - 1/2 is not positive");
        return None;
    }
    let h_exponent = h.get_exponent().unwrap();
    // |ln y| < 2^ln_y_bound
    let ln_y_bound = match ln_y.get_exponent() {
        Some(e) if *e > 0u32 => e + Integer::ONE,
        _ => Integer::ONE,
    };
    errors.push(y.k, y_exponent - &prec_integer + &ln_y_bound);
    if h_o != Equal {
        errors.push(1, h_exponent - &prec_integer + ln_y_bound);
    }
    errors.push(y.k << 1, h_exponent - &prec_integer);
    if ln_y_o != Equal {
        errors.push(1, h_exponent + ln_y.get_exponent().unwrap() - &prec_integer);
    }
    let (mut sum, o) = h.mul_prec(ln_y, prec);
    errors.push_rounding(&sum, o, prec);
    // - y
    errors.push(y.k, y_exponent - &prec_integer);
    let o = sum.sub_prec_assign_ref(&y.x, prec);
    errors.push_rounding(&sum, o, prec);
    // + mu(y)
    let mu = stirling_mu_approx(prec, terms, &y, &tol, tangents, &mut errors)?;
    let o = sum.add_prec_assign(mu, prec);
    errors.push_rounding(&sum, o, prec);
    Some(StirlingParts {
        prec,
        sum,
        errors,
        product,
    })
}

// The bracket of ln Gamma(x) at Ziv level `w` >= 64 from an enclosure `x` of x > 0, with a positive
// lower end, where x < 2^e: S + ln(2 pi / P^2) / 2, in the notation of `StirlingParts`. The
// constant and the rising product share one logarithm, as in `ln_gamma_bracket_enclosures`. `None`
// when a count gets too large, which does not happen.
//
// The gamma function does not use this bracket, but the enclosure of `gamma_positive_approx`, which
// takes one logarithm fewer; this one is for the logarithm of the gamma function.
//
// This replaces `lnGammaBracketEncl` and `lnGammaStirlingEncl` from `AzFloat/LnGammaBracket.lean`,
// Azurite.
fn ln_gamma_bracket_approx(
    x: &Bracket,
    e: &Integer,
    w: u64,
    tangents: &RefCell<TangentNumbers>,
) -> Option<Bracket> {
    let StirlingParts {
        prec,
        mut sum,
        mut errors,
        product,
    } = stirling_parts(x, e, w, tangents)?;
    let prec_integer = Integer::from(prec);
    let two_pi = Rel {
        x: BigExponentFloat::tau_prec(prec).0,
        k: 1,
    };
    let constant = if let Some(p) = product {
        rel_div(&two_pi, &rel_mul(&p, &p, prec)?, prec)?
    } else {
        two_pi
    };
    // The error of the logarithm is at most 2 * k * u, plus that of its rounding; and it is halved.
    let (ln_constant, o) = constant.x.ln_prec(prec);
    errors.push(constant.k, -&prec_integer);
    let half = ln_constant >> 1u32;
    errors.push_rounding(&half, o, prec);
    let o = sum.add_prec_assign(half, prec);
    errors.push_rounding(&sum, o, prec);
    let error = errors.total();
    Some((
        sum.sub_prec_round_ref_ref(&error, prec, Floor).0,
        sum.add_prec_round(error, prec, Ceiling).0,
    ))
}

// The enclosure of Gamma(x), or of 1 / Gamma(x) if `recip` is set, at Ziv level `w`, from an
// enclosure `x` of x > 0, with a positive lower end, where x < 2^e. In the notation of
// `StirlingParts`, which are taken at the level `w_ln`,
//
// Gamma(x) = e^S * sqrt(2 pi) / P.
//
// S is enclosed by its computed value and its error bound, e^S or e^(-S) by `gamma_exp_encl`, and P
// by its computed value P' and its count k: P' * (1 - k * u) <= P <= P' * (1 + k * u). Compared
// with the exponential of the bracket of ln Gamma, this takes one logarithm fewer; |S| is larger
// than |ln Gamma(x)| for a shifted argument, but the bracket of ln Gamma is formed from S too, so
// it is no more accurate.
//
// `None` when the enclosure of S is wider than 1 or a count gets too large, which does not happen.
fn gamma_positive_approx(
    recip: bool,
    x: &Bracket,
    e: &Integer,
    w_ln: u64,
    w: u64,
    tangents: &RefCell<TangentNumbers>,
) -> Option<Bracket> {
    let StirlingParts {
        prec: prec_ln,
        sum,
        errors,
        product,
    } = stirling_parts(x, e, w_ln, tangents)?;
    let error = errors.total();
    let exp = gamma_exp_encl(
        recip,
        (
            sum.sub_prec_round_ref_ref(&error, prec_ln, Floor).0,
            sum.add_prec_round(error, prec_ln, Ceiling).0,
        ),
        w,
    )?;
    let prec = w + 16;
    // P / sqrt(2 pi)
    let mut divisor = floor_bracket(
        BigExponentFloat::one_over_sqrt_tau_prec_round(prec, Floor).0,
        prec,
    );
    if let Some(p) = product {
        // k * u * P', rounded up
        let margin =
            p.x.mul_prec_round_ref_val(BigExponentFloat::from(p.k), 64, Ceiling)
                .0
                >> prec_ln;
        let p = (
            p.x.sub_prec_round_ref_ref(&margin, prec, Floor).0,
            p.x.add_prec_round(margin, prec, Ceiling).0,
        );
        divisor = encl_mul(prec, &divisor, &p);
    }
    Some(if recip {
        encl_mul(prec, &exp, &divisor)
    } else {
        encl_div(prec, &exp, &divisor)
    })
}

// How the enclosure of the gamma function of a positive number is computed, with what is kept from
// one Ziv level to the next: the tangent numbers, which are taken from this thread's cache and put
// back into it at the end.
pub(crate) struct LnGammaState {
    // Whether to compute with enclosures throughout, which is slower. Only the tests and the
    // benchmarks do.
    enclosures: bool,
    tangents: RefCell<TangentNumbers>,
}

impl LnGammaState {
    pub(crate) fn new(enclosures: bool) -> Self {
        Self {
            enclosures,
            tangents: RefCell::new(take_cached_tangent_numbers()),
        }
    }

    // The enclosure of Gamma(x), or of 1 / Gamma(x) if `recip` is set, at Ziv level `w`, from an
    // enclosure `x` of x > 0, with a positive lower end, where x < 2^e. The level `w_ln` is `w`
    // plus the bits of |ln Gamma(x)|, since the exponential turns an absolute error into a relative
    // one. `None` when there is no enclosure at this level.
    fn gamma_positive(
        &self,
        recip: bool,
        x: &Bracket,
        e: &Integer,
        w_ln: u64,
        w: u64,
    ) -> Option<Bracket> {
        if self.enclosures {
            gamma_exp_encl(recip, ln_gamma_bracket_enclosures(x, e, w_ln), w)
        } else {
            gamma_positive_approx(recip, x, e, w_ln, w, &self.tangents)
        }
    }

    // The bracket of ln Gamma(x) at Ziv level `w` >= 64, from an enclosure `x` of x > 0, with a
    // positive lower end, where x < 2^e. `None` when there is no bracket at this level.
    pub(crate) fn ln_gamma_bracket(&self, x: &Bracket, e: &Integer, w: u64) -> Option<Bracket> {
        if self.enclosures {
            Some(ln_gamma_bracket_enclosures(x, e, w))
        } else {
            ln_gamma_bracket_approx(x, e, w, &self.tangents)
        }
    }
}

impl Drop for LnGammaState {
    fn drop(&mut self) {
        cache_tangent_numbers(core::mem::replace(
            self.tangents.get_mut(),
            TangentNumbers::new(),
        ));
    }
}

crate_test_fn! {
    // The bracket of ln Gamma(x): the one computed with single values and error bounds (if
    // `enclosures` is not set), or the one computed with enclosures throughout.
    ln_gamma_bracket(
        enclosures: bool,
        x: &Bracket,
        e: &Integer,
        w: u64
    ) -> Option<Bracket> {
        LnGammaState::new(enclosures).ln_gamma_bracket(x, e, w)
    }
}

crate_test_fn! {
    // The enclosure of Gamma(x) or 1 / Gamma(x) for a positive x that the gamma function uses (if
    // `enclosures` is not set), or the one that is computed with enclosures throughout.
    gamma_positive_bracket(
        enclosures: bool,
        recip: bool,
        x: &Bracket,
        e: &Integer,
        w_ln: u64,
        w: u64
    ) -> Option<Bracket> {
        LnGammaState::new(enclosures).gamma_positive(recip, x, e, w_ln, w)
    }
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
    pub(crate) fn shifted_encl(&self, pos: bool, w: u64) -> Bracket {
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
    pub(crate) fn sin_encl(&self, prec: u64) -> Option<(bool, Bracket)> {
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
    state: &LnGammaState,
) -> Option<(bool, Bracket)> {
    if pos {
        let w_ln = w + gamma_ln_bits(e);
        return state
            .gamma_positive(recip, &target.shifted_encl(true, w_ln), e, w_ln, w)
            .map(|b| (false, b));
    }
    // 1 - x < 2^e
    let e = if *e > 0u32 {
        e + Integer::ONE
    } else {
        Integer::ONE
    };
    let w_ln = w + gamma_ln_bits(&e);
    let g = state.gamma_positive(false, &target.shifted_encl(false, w_ln), &e, w_ln, w)?;
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

// The bracket of Gamma(x) or 1 / Gamma(x) at Ziv level `w`, strict on both sides (see
// `strict_bracket`). When `gamma_core` gives nothing it is the dummy bracket [-1, 1], which Ziv's
// loop never accepts.
//
// This is equivalent to `gammaApprox` from `AzFloat/GammaBracket.lean` and `gammaRatApprox` from
// `AzFloat/GammaRatBracket.lean`, Azurite.
fn gamma_approx(
    recip: bool,
    target: &GammaTarget,
    e: &Integer,
    pos: bool,
    w: u64,
    state: &LnGammaState,
) -> Bracket {
    strict_bracket(
        gamma_core(recip, target, e, pos, w, state)
            .map(|(neg, (lo, hi))| if neg { (-hi, -lo) } else { (lo, hi) }),
        w,
    )
}

// The largest integer argument whose gamma function is taken as an exact factorial at precision
// `prec`. Any bound of at least `prec` + 65 would do for the loop beyond it (the odd part of (n -
// 1)! then has more than `prec` + 1 bits, so the value is not a rounding boundary); this one is
// about where the factorial stops being cheaper than the loop.
//
// This is equivalent to `gammaIntBound` from `AzFloat/GammaBracket.lean`, Azurite.
pub(crate) const fn gamma_int_bound(prec: u64) -> u64 {
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
// at the quarter-integers, and that Ziv's loop has no fuel. With `enclosures`, the gamma function
// of a positive number is enclosed through `ln_gamma_bracket_enclosures`; the result is the same.
fn gamma_helper_with(
    enclosures: bool,
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
            let state = LnGammaState::new(enclosures);
            ziv_loop_certified_from(
                &|w| gamma_approx(recip, &target, exponent, *sign, w, &state),
                prec,
                rm,
                max(64, prec + 32),
            )
        }
    }
}

// Gamma(x), or 1 / Gamma(x) if `recip` is set, rounded to `prec` bits with `rm`.
#[inline]
pub(crate) fn gamma_helper(
    recip: bool,
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    gamma_helper_with(false, recip, x, prec, rm)
}

crate_test_fn! {
    // `gamma_helper`, with the bracket of ln Gamma computed by `ln_gamma_bracket_enclosures`.
    gamma_prec_round_enclosures(
        recip: bool,
        x: &BigExponentFloat,
        prec: u64,
        rm: RoundingMode
    ) -> (BigExponentFloat, Ordering) {
        gamma_helper_with(true, recip, x, prec, rm)
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
// Gamma(1/3) for the denominators 2, 3, 4, and 6, and that Ziv's loop has no fuel. With
// `enclosures`, the gamma function of a positive number is enclosed through
// `ln_gamma_bracket_enclosures`; the result is the same.
fn gamma_rational_helper_with(
    enclosures: bool,
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
    let state = LnGammaState::new(enclosures);
    ziv_loop_certified_from(
        &|w| gamma_approx(recip, &target, &e, pos, w, &state),
        prec,
        rm,
        start,
    )
}

// Gamma(q), or 1 / Gamma(q) if `recip` is set, for a `Rational` q, rounded to `prec` bits with
// `rm`.
#[inline]
pub(crate) fn gamma_rational_helper(
    recip: bool,
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    gamma_rational_helper_with(false, recip, q, prec, rm)
}

crate_test_fn! {
    // `gamma_rational_helper`, with the bracket of ln Gamma computed by
    // `ln_gamma_bracket_enclosures`.
    gamma_rational_prec_round_enclosures(
        recip: bool,
        q: &Rational,
        prec: u64,
        rm: RoundingMode
    ) -> (BigExponentFloat, Ordering) {
        gamma_rational_helper_with(true, recip, q, prec, rm)
    }
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
