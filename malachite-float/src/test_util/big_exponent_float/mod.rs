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
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_nz::integer::Integer;

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
) {
    let hexes = xs
        .take(limit)
        .map(|x| {
            assert!(x.is_valid());
            format!("{x:#x}")
        })
        .collect_vec();
    assert!(hexes.iter().all_unique());
    assert_eq!(hexes, out);
}

// Checks that the first million values of `xs` are valid, that the first 20 are written in
// hexadecimal as `expected_values`, and that the 10 most common values among the first million,
// with their frequencies, are `expected_common_values`.
pub fn random_big_exponent_floats_helper_helper<I: Clone + Iterator<Item = BigExponentFloat>>(
    xs: I,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
) {
    let values = xs.clone().take(20).map(|x| format!("{x:#x}")).collect_vec();
    for x in xs.clone().take(1000000) {
        assert!(x.is_valid());
    }
    let common_values = common_values_map(1000000, 10, xs.map(|x| format!("{x:#x}")));
    let common_values = common_values
        .iter()
        .map(|(x, freq)| (x.as_str(), *freq))
        .collect_vec();
    let values = values.iter().map(String::as_str).collect_vec();
    assert_eq!(
        (values.as_slice(), common_values.as_slice()),
        (expected_values, expected_common_values)
    );
}

// Parses `Float`'s hexadecimal format, as `parse_hex_string` does, into the `BigExponentFloat`
// with the same value.
pub fn parse_big_exponent_float_hex(s: &str) -> BigExponentFloat {
    float_to_big_exponent_float(&crate::test_util::common::parse_hex_string(s))
}

// The positive power of 2 with sci-exponent `sci_exponent`: 2^`sci_exponent`.
pub fn big_exponent_float_power_of_2(sci_exponent: Integer) -> BigExponentFloat {
    exhaustive_positive_big_exponent_floats_with_sci_exponent_and_precision(sci_exponent, 1)
        .next()
        .unwrap()
}
