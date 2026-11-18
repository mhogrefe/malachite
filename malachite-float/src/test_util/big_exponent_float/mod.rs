// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::exhaustive::*;
use crate::{Float, InnerFloat};
use core::cmp::Ordering::{self, *};
use itertools::Itertools;
use malachite_base::num::arithmetic::traits::{Square, UnsignedAbs};
use malachite_base::num::basic::traits::{
    Infinity as _, NaN as _, NegativeInfinity, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// The `Float` with the same value as `x` and with `x`'s precision, if `x`'s exponent is in
// `Float`'s range. NaN, infinities, and zero map to the corresponding `Float`s (zero to positive
// zero).
pub fn big_exponent_float_to_float(x: &BigExponentFloat) -> Option<Float> {
    match x {
        BigExponentFloat(NaN) => Some(Float::NAN),
        BigExponentFloat(Infinity { sign: true }) => Some(Float::INFINITY),
        BigExponentFloat(Infinity { sign: false }) => Some(Float::NEGATIVE_INFINITY),
        BigExponentFloat(Zero) => Some(Float::ZERO),
        BigExponentFloat(Finite {
            sign,
            exponent,
            significand,
        }) => {
            let e = i32::try_from(exponent).ok()?;
            if !(Float::MIN_EXPONENT..=Float::MAX_EXPONENT).contains(&e) {
                return None;
            }
            Some(Float(InnerFloat::Finite {
                sign: *sign,
                exponent: e,
                precision: x.get_prec().unwrap(),
                significand: significand.clone(),
            }))
        }
    }
}

// The `BigExponentFloat` with the same value as `x`. Both zeros map to zero.
pub fn float_to_big_exponent_float(x: &Float) -> BigExponentFloat {
    match x {
        Float(InnerFloat::NaN) => BigExponentFloat::NAN,
        Float(InnerFloat::Infinity { sign: true }) => BigExponentFloat::INFINITY,
        Float(InnerFloat::Infinity { sign: false }) => BigExponentFloat::NEGATIVE_INFINITY,
        Float(InnerFloat::Zero { .. }) => BigExponentFloat::ZERO,
        Float(InnerFloat::Finite {
            sign,
            exponent,
            significand,
            ..
        }) => BigExponentFloat::from_parts(*sign, Integer::from(*exponent), significand.clone()),
    }
}

// Checks that the first `limit` values of `xs` are valid, distinct, and written in hexadecimal as
// `out`.
pub fn exhaustive_big_exponent_floats_helper_helper<I: Iterator<Item = BigExponentFloat>>(
    xs: I,
    limit: usize,
    out: &[&str],
    out_hex: &[&str],
) {
    let xs = xs
        .take(limit)
        .inspect(|x| assert!(x.is_valid()))
        .collect_vec();
    let hexes = xs.iter().map(|x| format!("{x:#x}")).collect_vec();
    assert!(hexes.iter().all_unique());
    assert_eq!(hexes, out_hex);
    assert_eq!(xs.iter().map(ToString::to_string).collect_vec(), out);
}

// Checks that the first million values of `xs` are valid, that the first 20 are written in
// hexadecimal as `expected_values`, and that the 10 most common values among the first million,
// with their frequencies, are `expected_common_values`.
pub fn random_big_exponent_floats_helper_helper<I: Clone + Iterator<Item = BigExponentFloat>>(
    xs: I,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
) {
    let first = xs.clone().take(20).collect_vec();
    for x in xs.clone().take(1000000) {
        assert!(x.is_valid());
    }
    // The common values are counted by their exact (hexadecimal) strings; the decimal list writes
    // the same values, in the same order.
    let common_values = common_values_map(1000000, 10, xs.map(|x| format!("{x:#x}")));
    let common_values_dec = common_values
        .iter()
        .map(|(x, freq)| (parse_big_exponent_float_hex(x).to_string(), *freq))
        .collect_vec();
    let values_hex = first.iter().map(|x| format!("{x:#x}")).collect_vec();
    let values = first.iter().map(ToString::to_string).collect_vec();
    let common_values_hex = common_values
        .iter()
        .map(|(x, freq)| (x.as_str(), *freq))
        .collect_vec();
    let common_values_dec = common_values_dec
        .iter()
        .map(|(x, freq)| (x.as_str(), *freq))
        .collect_vec();
    let values_hex = values_hex.iter().map(String::as_str).collect_vec();
    let values = values.iter().map(String::as_str).collect_vec();
    assert_eq!(
        (values_hex.as_slice(), common_values_hex.as_slice()),
        (expected_values_hex, expected_common_values_hex)
    );
    assert_eq!(
        (values.as_slice(), common_values_dec.as_slice()),
        (expected_values, expected_common_values)
    );
}

// Parses a hexadecimal string into a `BigExponentFloat`: `Float`'s format, with its `#` precision
// suffix, as `parse_hex_string` does, into the `BigExponentFloat` with the same value; and, without
// a suffix, `BigExponentFloat`'s own (what `Debug` writes), which reaches any exponent.
pub fn parse_big_exponent_float_hex(s: &str) -> BigExponentFloat {
    if s.contains('#') {
        float_to_big_exponent_float(&crate::test_util::common::parse_hex_string(s))
    } else {
        BigExponentFloat::from_string_base(16, s).unwrap()
    }
}

// The positive power of 2 with sci-exponent `sci_exponent`: 2^`sci_exponent`.
pub fn big_exponent_float_power_of_2(sci_exponent: Integer) -> BigExponentFloat {
    exhaustive_positive_big_exponent_floats_with_sci_exponent_and_precision(sci_exponent, 1)
        .next()
        .unwrap()
}

// The negative of `x`. This stands in for `Neg`, which comes with the arithmetic module.
pub fn big_exponent_float_negate(x: BigExponentFloat) -> BigExponentFloat {
    x.negate()
}

// Whether converting `x` to an exact `Natural`, `Integer`, or `Rational` is cheap: whether its
// exponent is less than 2^16 in absolute value. An exact conversion of a value with a huge exponent
// would need a huge amount of memory, so property tests skip those.
pub fn big_exponent_float_exponent_is_small(x: &BigExponentFloat) -> bool {
    x.get_exponent()
        .is_none_or(|e| e.unsigned_abs_ref() < &Natural::from(1u32 << 16))
}

// The exact value of `x`, written independently of the power-of-2-base formatters: `NaN`,
// `Infinity`, `-Infinity`, `0x0.0`, or `±m*2^k` with `m` odd.
pub fn big_exponent_float_exact_string(x: &BigExponentFloat) -> String {
    let Some(significand) = x.significand_ref() else {
        return format!("{x:#x}");
    };
    let zeros = significand.trailing_zeros().unwrap();
    let m = significand >> zeros;
    let k = x.get_exponent().unwrap() - Integer::from(m.significant_bits());
    let sign = if x.is_sign_negative() { "-" } else { "" };
    format!("{sign}{m}*2^{k}")
}

// Whether `t` is less than W(x), where W is the principal branch W_0 of the Lambert W function if
// `branch0` and the branch W_{-1} otherwise, and x is in the open domain of the branch (x > -1/e,
// and x < 0 for W_{-1}), is nonzero, and is not t * e^t. The answer comes from comparing t * e^t
// with x at growing precision, using that t * e^t increases on [-1, ∞) and decreases on (-∞,
// -1]: a check of the Lambert W functions that shares nothing with their algorithm but the
// exponential function. x is given by `cmp_x`, which compares its argument with x.
//
// Deciding a tiny `t` that way would take about as many bits as its exponent is negative, so when x
// is a `BigExponentFloat`, also given as `x_float`, a tiny `t` close to it is first compared using
// t + t^2 / 2 < t * e^t < t + 2 * t^2, which holds for 0 < |t| < 1.
pub fn lambert_w_exceeds(
    branch0: bool,
    cmp_x: &dyn Fn(&BigExponentFloat) -> Ordering,
    x_float: Option<&BigExponentFloat>,
    t: &BigExponentFloat,
) -> bool {
    if branch0 {
        if *t <= -1i32 {
            return true;
        }
    } else if *t >= -1i32 {
        return false;
    }
    if t.is_zero() {
        return cmp_x(t) == Less;
    }
    if let Some(x) = x_float
        && let (Some(e_x), Some(e_t)) = (x.get_exponent(), t.get_exponent())
        && *e_t <= -32i32
        && (e_x - e_t).unsigned_abs() <= 2u32
    {
        let d = x - t;
        let t_squared = t.square();
        if d < &t_squared >> 1u32 {
            return false;
        } else if d > t_squared << 1u32 {
            return true;
        }
    }
    let mut prec = t.get_prec().unwrap() + 64;
    loop {
        let e_lo = t.exp_prec_round_ref(prec, Floor).0;
        let e_hi = t.exp_prec_round_ref(prec, Ceiling).0;
        let (f_lo, f_hi) = if t.is_sign_positive() {
            (
                t.mul_prec_round_ref_val(e_lo, prec, Floor).0,
                t.mul_prec_round_ref_val(e_hi, prec, Ceiling).0,
            )
        } else {
            (
                t.mul_prec_round_ref_val(e_hi, prec, Floor).0,
                t.mul_prec_round_ref_val(e_lo, prec, Ceiling).0,
            )
        };
        if cmp_x(&f_hi) == Less {
            return branch0;
        } else if cmp_x(&f_lo) == Greater {
            return !branch0;
        }
        prec <<= 1;
    }
}

// Asserts that `w`, with the `Ordering` `o`, is W(x) correctly rounded to `prec` bits with `rm`,
// for a finite nonzero `w`, an x as in `lambert_w_exceeds`, and `rm` not `Exact`: that `o` tells
// the side of W(x) that `w` is on, and that W(x) is closer to `w` than the next number with `prec`
// bits on that side is (or, for `Nearest`, than the midpoint is).
pub fn assert_lambert_w_correctly_rounded(
    branch0: bool,
    cmp_x: &dyn Fn(&BigExponentFloat) -> Ordering,
    x_float: Option<&BigExponentFloat>,
    w: &BigExponentFloat,
    o: Ordering,
    prec: u64,
    rm: RoundingMode,
) {
    assert!(w.is_finite() && !w.is_zero());
    assert!(w.get_prec().unwrap() <= prec);
    assert_ne!(o, Equal);
    assert_ne!(rm, Exact);
    let below = o == Less;
    assert_eq!(lambert_w_exceeds(branch0, cmp_x, x_float, w), below);
    let positive = w.is_sign_positive();
    match rm {
        Floor => assert!(below),
        Ceiling => assert!(!below),
        Down => assert_eq!(below, positive),
        Up => assert_eq!(below, !positive),
        _ => {}
    }
    // The gap between w and the next number with `prec` bits on the side of W(x): halved when that
    // side is toward zero and |w| is a power of 2.
    let mut gap_exponent = w.get_exponent().unwrap() - Integer::from(prec);
    if below != positive && w.get_prec() == Some(1) {
        gap_exponent -= Integer::ONE;
    }
    if rm == Nearest {
        gap_exponent -= Integer::ONE;
    }
    let gap = big_exponent_float_power_of_2(gap_exponent);
    let bound = if below { w + gap } else { w - gap };
    assert_eq!(lambert_w_exceeds(branch0, cmp_x, x_float, &bound), !below);
}

// Rounds a positive value lying strictly between `lo` and `hi` to `prec` bits with `rm`, when the
// two ends determine the result and its `Ordering`.
fn round_between(
    lo: &BigExponentFloat,
    hi: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> Option<(BigExponentFloat, Ordering)> {
    let l = lo.round_to_prec_round_ref(prec, rm).0;
    if l != hi.round_to_prec_round_ref(prec, rm).0 {
        None
    } else if l <= *lo {
        Some((l, Less))
    } else if l >= *hi {
        Some((l, Greater))
    } else {
        None
    }
}

// Gives a finite nonzero result of a function that is odd the sign of its argument: `f` computes
// the function of the absolute value, given the rounding mode to use.
fn odd_function_result<F: Fn(RoundingMode) -> (BigExponentFloat, Ordering)>(
    negative: bool,
    rm: RoundingMode,
    f: F,
) -> (BigExponentFloat, Ordering) {
    if negative {
        let (g, o) = f(-rm);
        (-g, o.reverse())
    } else {
        f(rm)
    }
}

// The Gudermannian function of a finite nonzero `BigExponentFloat`, from the identity gd(x) =
// arctan(sinh(x)): the hyperbolic sine and the arctangent are rounded down and up at a working
// precision that doubles until the two results round alike. The time taken grows with |x|.
pub fn gudermannian_prec_round_naive(
    x: &BigExponentFloat,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert!(x.is_finite() && !x.is_zero());
    assert_ne!(rm, Exact);
    let a = if *x < 0u32 { -x } else { x.clone() };
    odd_function_result(*x < 0u32, rm, |rm| {
        let mut working_prec = prec + 64;
        loop {
            let lo = a
                .sinh_prec_round_ref(working_prec, Floor)
                .0
                .atan_prec_round(working_prec, Floor)
                .0;
            let hi = a
                .sinh_prec_round_ref(working_prec, Ceiling)
                .0
                .atan_prec_round(working_prec, Ceiling)
                .0;
            if let Some(r) = round_between(&lo, &hi, prec, rm) {
                return r;
            }
            working_prec <<= 1;
        }
    })
}

// The Gudermannian function of a nonzero `Rational`, computed like `gudermannian_prec_round_naive`.
pub fn gudermannian_rational_prec_round_naive(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_ne!(*x, 0u32);
    assert_ne!(rm, Exact);
    let a = if *x < 0u32 { -x } else { x.clone() };
    odd_function_result(*x < 0u32, rm, |rm| {
        let mut working_prec = prec + 64;
        loop {
            let lo = BigExponentFloat::sinh_rational_prec_round_ref(&a, working_prec, Floor)
                .0
                .atan_prec_round(working_prec, Floor)
                .0;
            let hi = BigExponentFloat::sinh_rational_prec_round_ref(&a, working_prec, Ceiling)
                .0
                .atan_prec_round(working_prec, Ceiling)
                .0;
            if let Some(r) = round_between(&lo, &hi, prec, rm) {
                return r;
            }
            working_prec <<= 1;
        }
    })
}
