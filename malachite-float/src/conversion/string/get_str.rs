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
use crate::floor_and_ceiling;
use core::cmp::Ordering::{self, Equal};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, CheckedLogBase2, NegAssign, Sign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
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
    let p = Float::from_natural_prec(Natural::from(mantissa), 128).0;
    let p = p >> u64::exact_from(128 - i64::from(exp));
    // t = e * p, with e as a `Float` with the precision of an `mpfr_exp_t` minus one, both
    // roundings up.
    let t = Float::from_signed_prec_round(e, i64::WIDTH - 1, Ceiling)
        .0
        .mul_prec_round(p, i64::WIDTH - 1, Ceiling)
        .0;
    // ceil(t).
    i64::rounding_from(&t, Ceiling).0
}

// Returns at least `1 + ceil(bit_len * log(2) / log(base))` digits, where `bit_len` is the number
// of bits of the mantissa, ensuring that converting the output back gives the same `Float`.
//
// `base` must be between 2 and 62, inclusive.
//
// This is `mpfr_get_str_ndigits` from `get_str.c`, MPFR 4.2.2.
pub(crate) fn get_str_ndigits(base: u64, bit_len: u64) -> usize {
    assert!((2..=62).contains(&base));
    // Deal first with power-of-two bases, since even for those, `ceil_mul` might return a value too
    // large by 1. For `base = 2 ^ k`, this is `1 + ceil((bit_len - 1) / k) = 2 + floor((bit_len -
    // 2) / k)`.
    if let Some(k) = base.checked_log_base_2() {
        return usize::exact_from(1 + (bit_len + k - 2) / k);
    }
    // `ceil_mul` is guaranteed to give `1 + ceil(bit_len * log(2) / log(base))` for `bit_len` below
    // this bound (for `bit_len = 186564318007` and `base = 7` or `49` it returns one more).
    let ret = if bit_len < 186_564_318_007 {
        u64::exact_from(ceil_mul(i64::exact_from(bit_len), base, 1))
    } else {
        // `bit_len` is large and `base` is not a power of two, so `bit_len * log(2) / log(base)`
        // cannot be an integer and Ziv's loop terminates. `w` is the working precision; `ceil_mul`
        // used a 77-bit upper approximation to `log(2) / log(base)`. Reaching here needs a mantissa
        // of at least ~1.86e11 bits, far beyond any `Float` the test suite builds.
        fail_on_untested_path("get_str_ndigits, Ziv loop for huge bit_len");
        let mut w = 77;
        loop {
            w <<= 1;
            // lower (rounding down) and upper (rounding up) approximations to `log2(base)`
            let (log_lo, log_hi) = floor_and_ceiling(
                Float::from_unsigned_prec(base, w)
                    .0
                    .log_base_2_prec_round(w, Floor),
            );
            // lower (`bit_len / log_hi`, rounding down) and upper (`bit_len / log_lo`, rounding up)
            // bounds on `bit_len * log(2) / log(base)`, each rounded up to an integer
            let pf = Float::from_unsigned_prec(bit_len, w).0;
            let lo = pf.div_round_ref_val(log_hi, Floor).0;
            let hi = pf.div_round(log_lo, Ceiling).0;
            let lo = u64::rounding_from(&lo, Ceiling).0;
            let hi = u64::rounding_from(&hi, Ceiling).0;
            if lo == hi {
                break lo;
            }
        }
    };
    usize::exact_from(1 + ret)
}

/// Converts a [`Float`] to base-`base` mantissa digits and an exponent, rounding to `digit_len`
/// digits with the rounding mode `rm`.
///
/// The digits are returned as ASCII characters (`0`–`9`, then lowercase `a`–`z`, then uppercase
/// `A`–`Z`, supporting a `base` of up to 62; a negative `base` in `-36..=-2` uses base `|base|`
/// with `0`–`9` and uppercase `A`–`Z`), preceded by `-` when `x` is negative. With the returned
/// exponent $e$, the value represented is $0.d_1 d_2 \ldots \times \mathrm{base}^e$, where $d_1 d_2
/// \ldots$ are the digits. If `digit_len` is 0, the fewest digits that round-trip back to `x` are
/// used.
///
/// `base` must be in `2..=62` or `-36..=-2`; any other value returns `None`.
///
/// The returned [`Ordering`] reports whether the rounded result is less than, equal to, or greater
/// than the exact value of `x`. The special values NaN, $\infty$, and $-\infty$ produce the strings
/// `@NaN@`, `@Inf@`, and `-@Inf@`, each with exponent 0 and `Equal`.
///
/// # Worst-case complexity
/// $T(n) = O(n (\log n)^2 \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.complexity(), digit_len)`.
///
/// # Panics
/// Panics if `rm` is `Exact` but `x` cannot be represented exactly in `digit_len` base-`base`
/// digits.
///
/// # Examples
/// ```
/// use core::cmp::Ordering::*;
/// use malachite_base::rounding_modes::RoundingMode::{self, *};
/// use malachite_float::conversion::string::get_str::get_str;
/// use malachite_float::Float;
/// use malachite_q::Rational;
///
/// // Render the returned digit bytes as a `String` for readability.
/// let s = |x: &Float, base: i64, n: usize, rm: RoundingMode| {
///     get_str(x, base, n, rm)
///         .map(|(digits, exp, ord)| (String::from_utf8(digits).unwrap(), exp, ord))
/// };
///
/// // 1.25 to 3 digits: 0.125 * 10^1 in base 10, 0.101 * 2^1 in base 2, both exact.
/// assert_eq!(
///     s(&Float::from(1.25), 10, 3, Nearest),
///     Some(("125".to_string(), 1, Equal))
/// );
/// assert_eq!(
///     s(&Float::from(1.25), 2, 3, Nearest),
///     Some(("101".to_string(), 1, Equal))
/// );
///
/// // A negative value gets a leading `-`.
/// assert_eq!(
///     s(&Float::from(-1.25), 10, 3, Nearest),
///     Some(("-125".to_string(), 1, Equal))
/// );
///
/// // 1/3 (to 53 bits) has no finite base-10 expansion, so the result is rounded and the
/// // `Ordering` gives the direction.
/// let third = Float::from_rational_prec(Rational::from_unsigneds(1u32, 3u32), 53).0;
/// assert_eq!(s(&third, 10, 4, Floor), Some(("3333".to_string(), 0, Less)));
/// assert_eq!(
///     s(&third, 10, 4, Ceiling),
///     Some(("3334".to_string(), 0, Greater))
/// );
///
/// // Special values produce fixed strings; an invalid base gives `None`.
/// assert_eq!(
///     s(&Float::from(f64::NAN), 2, 0, Down),
///     Some(("@NaN@".to_string(), 0, Equal))
/// );
/// assert_eq!(
///     s(&Float::from(f64::INFINITY), 2, 0, Down),
///     Some(("@Inf@".to_string(), 0, Equal))
/// );
/// assert_eq!(s(&Float::from(1.25), 100, 0, Nearest), None);
/// ```
///
/// This is mpfr_get_str from get_str.c, MPFR 4.2.2.
pub fn get_str(
    x: &Float,
    base: i64,
    digit_len: usize,
    mut rm: RoundingMode,
) -> Option<(Vec<u8>, i64, Ordering)> {
    // valid bases are -36..=-2 and 2..=62
    if !(-36..=-2).contains(&base) && !(2..=62).contains(&base) {
        return None;
    }
    let b = base.unsigned_abs();
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
            // Malachite's zero carries no precision, so the digit_len == 0 default
            // (get_str_ndigits) does not apply; use a single digit.
            (
                !*sign,
                vec![b'0'; if digit_len == 0 { 1 } else { digit_len }],
                0,
                0,
            )
        }
        Finite {
            sign,
            exponent,
            precision,
            significand,
        } => {
            let m = if digit_len == 0 {
                get_str_ndigits(b, *precision)
            } else {
                digit_len
            };
            // For a negative x, reduce to the magnitude by inverting the rounding direction (the
            // mpfr_get_str MPFR_INVERT_RND step).
            let neg = !*sign;
            if neg {
                rm.neg_assign();
            }
            // Malachite's `exponent` is MPFR's EXP (the scientific exponent plus one).
            let xp = significand.to_limbs_asc();
            let x_exp = i64::from(*exponent);
            let (s, e, dir) = if b.is_power_of_two() {
                limbs_get_str_power_of_2(&xp, x_exp, *precision, b, base, m, rm)
            } else {
                let g = ceil_mul(x_exp - 1, b, 1);
                let exp = (i64::exact_from(m) - g).unsigned_abs();
                // radix-2 precision needed for m digits in base b, plus guard bits
                let mut prec = u64::exact_from(ceil_mul(i64::exact_from(m), b, 0)) + 1;
                prec += prec.ceiling_log_base_2();
                if exp != 0 {
                    // add the maximal exponentiation error
                    prec += 3 * exp.ceiling_log_base_2();
                }
                limbs_get_str(&xp, x_exp, b, base, m, rm, g, prec, i64::exact_from(exp))
            };
            (neg, s, e, dir)
        }
    };
    // `Exact` demands that the digits represent `x` exactly; a nonzero `dir` means rounding was
    // needed, which violates the contract. (For odd bases this is common, since a dyadic `Float`
    // rarely has a finite expansion there; and `digit_len == 0` picks the round-trip digit count,
    // which is generally fewer than the exact expansion needs.)
    assert!(
        rm != Exact || dir == 0,
        "get_str: Exact rounding was requested, but {x} is not exactly representable in the \
         requested number of base-{base} digits"
    );
    // `dir` orders the result's magnitude against `|x|`; negating both reverses the order.
    let o = dir.sign();
    Some(if neg {
        s.insert(0, b'-');
        (s, e, o.reverse())
    } else {
        (s, e, o)
    })
}
