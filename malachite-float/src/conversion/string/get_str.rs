// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 1999-2024 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::conversion::string::get_str_data::MPFR_L2B;
use core::cmp::Ordering::{self, Equal};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::CeilingLogBase2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Ceiling, Exact, Floor};
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float_extras::{limbs_get_str, limbs_get_str_power_of_2};

// Returns `ceil(e * log2(beta) ^ ((-1) ^ i))`, or that plus 1. For `i == 0` it uses a 23-bit upper
// approximation to `log(beta) / log(2)`; for `i == 1` a 77-bit upper approximation to `log(2) /
// log(beta)`. Both approximations are entries of `MPFR_L2B`.
//
// This is `mpfr_ceil_mul` from `get_str.c`, MPFR 4.2.2.
pub(crate) fn ceil_mul(e: i64, beta: u64, i: usize) -> i64 {
    // p = mantissa * 2 ^ (exp - 128): the l2b approximation as an exact `Float`.
    let (mantissa, exp) = MPFR_L2B[usize::exact_from(beta) - 2][i];
    let (p, _) = Float::from_natural_prec(Natural::from(mantissa), 128);
    let p = p >> u64::exact_from(128 - i64::from(exp));
    // t = e, as a `Float` with the precision of an `mpfr_exp_t` minus one, rounded up.
    let (t, _) = Float::from_signed_prec_round(e, i64::WIDTH - 1, Ceiling);
    // t = t * p, rounded up.
    let (t, _) = t.mul_prec_round(p, i64::WIDTH - 1, Ceiling);
    // ceil(t).
    i64::rounding_from(&t, Ceiling).0
}

// Returns at least `1 + ceil(p * log(2) / log(b))` digits, where `p` is the number of bits of the
// mantissa, ensuring that converting the output back gives the same `Float`.
//
// `b` must be between 2 and 62, inclusive.
//
// This is `mpfr_get_str_ndigits` from `get_str.c`, MPFR 4.2.2.
pub(crate) fn get_str_ndigits(b: u64, p: u64) -> usize {
    assert!((2..=62).contains(&b));
    // Deal first with power-of-two bases, since even for those, `ceil_mul` might return a value too
    // large by 1. For `b = 2 ^ k`, this is `1 + ceil((p - 1) / k) = 2 + floor((p - 2) / k)`.
    if b.is_power_of_two() {
        let k = b.significant_bits() - 1;
        return usize::exact_from(1 + (p + k - 2) / k);
    }
    // `ceil_mul` is guaranteed to give `1 + ceil(p * log(2) / log(b))` for `p` below this bound
    // (for `p = 186564318007` and `b = 7` or `49` it returns one more).
    let ret = if p < 186_564_318_007 {
        u64::exact_from(ceil_mul(i64::exact_from(p), b, 1))
    } else {
        // `p` is large and `b` is not a power of two, so `p * log(2) / log(b)` cannot be an integer
        // and Ziv's loop terminates. `w` is the working precision; `ceil_mul` used a 77-bit upper
        // approximation to `log(2) / log(b)`. Reaching here needs a mantissa of at least ~1.86e11
        // bits, far beyond any `Float` the test suite builds.
        fail_on_untested_path("get_str_ndigits, Ziv loop for huge p");
        let mut w = 77;
        loop {
            w *= 2;
            // upper (rounding up) and lower (rounding down) approximations to `log2(b)`
            let log_hi = Float::from_unsigned_prec(b, w)
                .0
                .log_base_2_prec_round(w, Ceiling)
                .0;
            let log_lo = Float::from_unsigned_prec(b, w)
                .0
                .log_base_2_prec_round(w, Floor)
                .0;
            // lower (`p / log_hi`, rounding down) and upper (`p / log_lo`, rounding up) bounds on
            // `p * log(2) / log(b)`, each rounded up to an integer
            let lo = Float::from_unsigned_prec(p, w)
                .0
                .div_prec_round(log_hi, w, Floor)
                .0;
            let hi = Float::from_unsigned_prec(p, w)
                .0
                .div_prec_round(log_lo, w, Ceiling)
                .0;
            let lo = u64::rounding_from(&lo, Ceiling).0;
            let hi = u64::rounding_from(&hi, Ceiling).0;
            if lo == hi {
                break lo;
            }
        }
    };
    usize::exact_from(1 + ret)
}

// Computes the mantissa digit string and exponent of a `Float` `x` in base `b0` (`2 <= |b0| <= 62`,
// or a negative base in `-36..=-2`), with `m` digits (`m == 0` chooses the minimum that
// round-trips, via `get_str_ndigits`), rounding with `rnd`. Returns the digit characters (with a
// leading `-` for a negative `x`) and the exponent, or `None` if the base is invalid. Special
// values produce the strings `@NaN@`, `@Inf@`, and `-@Inf@`.
//
// The third return value is an [`Ordering`] indicating whether the returned (rounded) value is less
// than, equal to, or greater than `x`.
//
// This is `mpfr_get_str` from `get_str.c`, MPFR 4.2.2.
pub fn get_str(
    x: &Float,
    b0: i64,
    m: usize,
    rnd: RoundingMode,
) -> Option<(Vec<u8>, i64, Ordering)> {
    // valid bases are -36..=-2 and 2..=62
    if b0 < -36 || (-2 < b0 && b0 < 2) || 62 < b0 {
        return None;
    }
    let b = b0.unsigned_abs();
    // `dir` is the direction in which the magnitude of `x` was rounded to the result (-1, 0, or 1).
    let (neg, mut s, e, dir) = match &x.0 {
        NaN => return Some((b"@NaN@".to_vec(), 0, Equal)),
        Infinity { sign } => {
            let s = if *sign {
                b"@Inf@".to_vec()
            } else {
                b"-@Inf@".to_vec()
            };
            return Some((s, 0, Equal));
        }
        Zero { sign } => {
            // Malachite's zero carries no precision, so the m == 0 default (get_str_ndigits) does
            // not apply; use a single digit.
            (!*sign, vec![b'0'; if m == 0 { 1 } else { m }], 0, 0)
        }
        Finite {
            sign,
            exponent,
            precision,
            significand,
        } => {
            let m = if m == 0 {
                get_str_ndigits(b, *precision)
            } else {
                m
            };
            // For a negative x, reduce to the magnitude by inverting the rounding direction (the
            // mpfr_get_str MPFR_INVERT_RND step).
            let neg = !*sign;
            let rnd = if neg {
                match rnd {
                    Floor => Ceiling,
                    Ceiling => Floor,
                    rnd => rnd,
                }
            } else {
                rnd
            };
            // Malachite's `exponent` is MPFR's EXP (the scientific exponent plus one).
            let xp = significand.to_limbs_asc();
            let x_exp = i64::from(*exponent);
            let (s, e, dir) = if b.is_power_of_two() {
                limbs_get_str_power_of_2(&xp, x_exp, *precision, b, b0, m, rnd)
            } else {
                let g = ceil_mul(x_exp - 1, b, 1);
                let exp = (i64::exact_from(m) - g).abs();
                // radix-2 precision needed for m digits in base b, plus guard bits
                let mut prec = ceil_mul(i64::exact_from(m), b, 0) + 1;
                prec += i64::exact_from(u64::exact_from(prec).ceiling_log_base_2());
                if exp != 0 {
                    // add the maximal exponentiation error
                    prec += 3 * i64::exact_from(u64::exact_from(exp).ceiling_log_base_2());
                }
                limbs_get_str(&xp, x_exp, b, b0, m, rnd, g, u64::exact_from(prec), exp)
            };
            (neg, s, e, dir)
        }
    };
    // `Exact` demands that the digits represent `x` exactly; a nonzero `dir` means rounding was
    // needed, which violates the contract. (For odd bases this is common, since a dyadic `Float`
    // rarely has a finite expansion there; and `m == 0` picks the round-trip digit count, which is
    // generally fewer than the exact expansion needs.)
    assert!(
        rnd != Exact || dir == 0,
        "get_str: Exact rounding was requested, but {x} is not exactly representable in the \
         requested number of base-{b0} digits"
    );
    // `dir` orders the result's magnitude against `|x|`; negating both reverses the order.
    let o = dir.cmp(&0);
    let o = if neg { o.reverse() } else { o };
    if neg {
        s.insert(0, b'-');
    }
    Some((s, e, o))
}
