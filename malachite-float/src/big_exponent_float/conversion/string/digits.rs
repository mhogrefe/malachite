// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// The two numeric cores of `BigExponentFloat`'s string conversions in any base: rounding a value to
// a given number of digits (the counterpart of `Float`'s `get_str`), and rounding a digit string to
// a given number of bits (the counterpart of `Float`'s `set_str_helper`). Both hand a value whose
// exponent is within `Float`'s range to the `Float` function, so that the two types agree wherever
// both can represent the result, and compute any other value themselves, with `Integer` exponents.
//
// Write the base as b = 2^a c with c odd. A value outside `Float`'s range has a binary exponent
// beyond 2^30 in absolute value, and the cost of computing its rounding exactly is the size of the
// numbers involved: the factor 2^(a j) of b^j is a shift, but c^j and the shifted value are not
// free. Exact computation is used whenever the result could be exact or a tie (a midpoint between
// two candidates), which happens only when the exact computation is no larger than the result
// itself; otherwise a Ziv loop, which then always terminates, computes the rounding from
// approximations of c^j.

use crate::Float;
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::Finite;
use crate::big_exponent_float::arithmetic::mul_rational::{finite_to_float, from_float_result};
use crate::big_exponent_float::arithmetic::round_to_integer::finite_odd_part;
use crate::big_exponent_float::exponential::exp::finite_can_round;
use crate::float::conversion::string::get_str::get_str;
use crate::float::conversion::string::set_str::set_str_helper;
use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{AddMul, Pow};
use malachite_base::num::basic::traits::One;
use malachite_base::num::comparison::traits::OrdProduct;
use malachite_base::num::conversion::traits::{Digits, ExactFrom, RoundingFrom, ToStringBase};
use malachite_base::num::exponential::traits::CeilingLogBase2;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// x * 2^s, for a finite nonzero x.
pub(crate) fn finite_shl_integer(x: &BigExponentFloat, s: &Integer) -> BigExponentFloat {
    let BigExponentFloat(Finite {
        sign,
        exponent,
        significand,
    }) = x
    else {
        unreachable!()
    };
    BigExponentFloat(Finite {
        sign: *sign,
        exponent: exponent + s,
        significand: significand.clone(),
    })
}

// The rounding mode that rounds the magnitude of a value with the given sign as `rm` rounds the
// value: `Floor` and `Ceiling` become `Down` or `Up`.
pub(crate) const fn magnitude_rounding_mode(rm: RoundingMode, positive: bool) -> RoundingMode {
    match rm {
        Floor => {
            if positive {
                Down
            } else {
                Up
            }
        }
        Ceiling => {
            if positive {
                Up
            } else {
                Down
            }
        }
        rm => rm,
    }
}

// floor(log_b x) for a finite positive x. The logarithm is computed with directed rounding at a
// precision that grows until both bounds have the same floor; when log_b x is an integer, both
// bounds are that integer.
fn floor_log_base_of_positive(x: &BigExponentFloat, base: u64) -> Integer {
    let mut prec = x.get_exponent().unwrap().significant_bits() + 16;
    loop {
        let lo = x.log_base_prec_round_ref(base, prec, Floor).0;
        let hi = x.log_base_prec_round_ref(base, prec, Ceiling).0;
        let lo = Integer::rounding_from(&lo, Floor).0;
        if lo == Integer::rounding_from(&hi, Floor).0 {
            return lo;
        }
        prec <<= 1;
    }
}

// The base-2 exponent `x`'s value would have in `Float`, if it is within `Float`'s range.
fn float_exponent(x: &BigExponentFloat) -> Option<i32> {
    let e = i32::try_from(x.get_exponent()?).ok()?;
    (Float::MIN_EXPONENT..=Float::MAX_EXPONENT)
        .contains(&e)
        .then_some(e)
}

crate_test_fn! {
// The digits of a finite nonzero `x` rounded to `digit_len` > 0 digits in base |`base`| (from 2 to
// 36; a negative `base` gives uppercase digits) with `rm`, as `Float`'s `get_str` (mpfr_get_str)
// computes them: the digit characters, preceded by `-` when `x` is negative; the exponent e for
// which the rounded value is 0.d_1 d_2 ... d_m * base^e; and whether the rounded value is less
// than, equal to, or greater than `x`. The exponent is an `Integer`.
//
// Panics if `rm` is `Exact` and `x` cannot be written exactly with `digit_len` digits.
big_exponent_float_get_str(
    x: &BigExponentFloat,
    base: i64,
    digit_len: usize,
    rm: RoundingMode,
) -> (Vec<u8>, Integer, Ordering) {
    assert!((2..=36).contains(&base.unsigned_abs()));
    assert_ne!(digit_len, 0);
    if let Some(e) = float_exponent(x) {
        let float = finite_to_float(Cow::Borrowed(x), e);
        let (s, e, o) = get_str(&float, base, digit_len, rm).unwrap();
        return (s, Integer::from(e), o);
    }
    let positive = *x > 0u32;
    let mrm = magnitude_rounding_mode(rm, positive);
    let b = base.unsigned_abs();
    let ax = if positive { x.clone() } else { x.clone().negate() };
    let m = u64::exact_from(digit_len);
    // |x| lies in [b^(e - 1), b^e), and the digits are the rounding of |x| b^j, which lies in [b^(m
    // - 1), b^m), to an integer.
    let mut e = floor_log_base_of_positive(&ax, b) + Integer::ONE;
    let j = Integer::from(m) - &e;
    let a = u64::from(b.trailing_zeros());
    let c = b >> a;
    let (mantissa, t) = finite_odd_part(&ax);
    let mantissa_bits = mantissa.significant_bits();
    // |x| b^j = mantissa * c^j * 2^shift.
    let shift = t.add_mul(Integer::from(a), &j);
    let c_bits = if c == 1 { 0 } else { c.ceiling_log_base_2() };
    // The size of the exact computation, in bits, against the size of the result: an exact or tie
    // result needs the first to be at most m log_2(b) + 3 * mantissa_bits + 2.
    let cost = (Natural::from(mantissa_bits) + shift.unsigned_abs_ref())
        .add_mul(Natural::from(c_bits), j.unsigned_abs_ref());
    let budget = Natural::from(3 * mantissa_bits + 64)
        .add_mul(Natural::from(m), Natural::from(b.ceiling_log_base_2()));
    let (mut digits, o) = if cost <= budget {
        let mut y = Rational::from(&mantissa) << i64::exact_from(&shift);
        // In a power-of-2 base c = 1, and j may be huge.
        if c != 1 {
            let c_power = Natural::from(c).pow(u64::exact_from(j.unsigned_abs_ref()));
            if j >= 0u32 {
                y *= Rational::from(c_power);
            } else {
                y /= Rational::from(c_power);
            }
        }
        Natural::rounding_from(&y, mrm)
    } else {
        assert_ne!(mrm, Exact, "Inexact BigExponentFloat digit conversion");
        let scaled = finite_shl_integer(&ax, &(Integer::from(a) * &j));
        let c = BigExponentFloat::from(c);
        let mut wp = m * b.ceiling_log_base_2() + 64;
        loop {
            // c^j and the product are each correctly rounded to wp bits, so the relative error is
            // below 2^(1 - wp) (1 + 2^(-wp)), and the error below 2^(EXP - wp + 2).
            let c_power = c.pow_integer_prec_ref_ref(&j, wp).0;
            let mut y = scaled.mul_prec_ref_val(c_power, wp).0;
            let integer_bits = u64::exact_from(y.get_exponent().unwrap());
            if finite_can_round(&y, wp - 2, wp, integer_bits, mrm) {
                let o = y.round_to_prec_round_assign(integer_bits, mrm);
                break (Natural::exact_from(&y), o);
            }
            wp += wp >> 1;
        }
    };
    // Rounding up may reach b^m, which is written with one more digit as 0.1 * b^(e + 1).
    let b_natural = Natural::from(b);
    if digits == (&b_natural).pow(m) {
        digits = b_natural.pow(m - 1);
        e += Integer::ONE;
    }
    let mut s = Vec::with_capacity(digit_len + 1);
    if !positive {
        s.push(b'-');
    }
    let ds = digits.to_string_base(u8::exact_from(b));
    debug_assert_eq!(ds.len(), digit_len);
    if base < 0 {
        s.extend(ds.bytes().map(|d| d.to_ascii_uppercase()));
    } else {
        s.extend(ds.bytes());
    }
    (
        s,
        e,
        if positive { o } else { o.reverse() },
    )
}}

crate_test_fn! {
// The value sign * 0.d_1 d_2 ... d_n * base^`exp_base`, rounded to `prec` bits with `rm`, where
// `digits` holds the digit values (not characters), most significant first, with leading and
// trailing zeros stripped; it must be nonempty. This is `Float`'s `set_str_helper` with an
// `Integer` exponent and no overflow or underflow.
big_exponent_float_set_str_helper(
    sign: bool,
    digits: &[u8],
    base: u8,
    exp_base: &Integer,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert!(!digits.is_empty());
    assert_ne!(prec, 0);
    // The value lies in [base^(exp_base - 1), base^exp_base). When its binary exponent is certainly
    // within `Float`'s range, `Float` computes it.
    let log_hi = base.ceiling_log_base_2();
    if let Ok(eb) = i64::try_from(exp_base)
        && eb.unsigned_abs() <= const { Float::MAX_EXPONENT_U64 - 128 } / log_hi
        && let Some(result) = from_float_result(
            set_str_helper(sign, digits, base, eb, 0, prec, rm),
            None,
        )
    {
        return result;
    }
    let mrm = magnitude_rounding_mode(rm, sign);
    let n = Natural::from_digits_desc(&u64::from(base), digits.iter().map(|&d| u64::from(d)))
        .unwrap();
    // The value is n * base^j = n * c^j * 2^(a j).
    let j = exp_base - Integer::from(digits.len());
    let a = u64::from(base.trailing_zeros());
    let c = u64::from(base >> a);
    let c_bits = if c == 1 { 0 } else { c.ceiling_log_base_2() };
    // The rounding of n c^j can be exact or a tie only if n c^j has at most prec + 1 significant
    // bits (for j >= 0) or c^|j| divides n (for j < 0); either bounds |j| log_2(c).
    let exact_bound = Natural::from(prec.max(n.significant_bits()) + 65);
    let (y, o) = if c == 1 {
        // A power-of-2 base, where j may be huge: the value is n * 2^(a j).
        BigExponentFloat::from_rational_prec_round(Rational::from(n), prec, mrm)
    } else if Natural::cmp_product(&Natural::from(c_bits), j.unsigned_abs_ref(), &exact_bound)
        != Greater
    {
        let c_power = Natural::from(c).pow(u64::exact_from(j.unsigned_abs_ref()));
        let q = if j >= 0u32 {
            Rational::from(n * c_power)
        } else {
            Rational::from_naturals(n, c_power)
        };
        BigExponentFloat::from_rational_prec_round(q, prec, mrm)
    } else {
        assert_ne!(mrm, Exact, "Inexact BigExponentFloat string conversion");
        let n = BigExponentFloat::from(n);
        let c = BigExponentFloat::from(c);
        let mut wp = prec + 64;
        loop {
            // As in `big_exponent_float_get_str`, the error is below 2^(EXP - wp + 2).
            let c_power = c.pow_integer_prec_ref_ref(&j, wp).0;
            let y = n.mul_prec_ref_val(c_power, wp).0;
            if finite_can_round(&y, wp - 2, wp, prec, mrm) {
                break y.round_to_prec_round(prec, mrm);
            }
            wp += wp >> 1;
        }
    };
    let y = finite_shl_integer(&y, &(Integer::from(a) * j));
    if sign {
        (y, o)
    } else {
        (y.negate(), o.reverse())
    }
}}
