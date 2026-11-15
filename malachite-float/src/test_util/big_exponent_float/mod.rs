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
use itertools::Itertools;
use malachite_base::num::basic::traits::{Infinity as _, NaN as _, NegativeInfinity, Zero as _};
use malachite_base::num::conversion::traits::FromStringBase;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

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
