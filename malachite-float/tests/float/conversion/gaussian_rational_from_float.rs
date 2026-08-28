// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{Infinity, NaN};
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom};
use malachite_base::strings::ToDebugString;
use malachite_float::Float;
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::float_gen;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;

#[test]
fn test_try_from_float() {
    let test = |s, s_hex, out: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let og = GaussianRational::try_from(x.clone());
        assert_eq!(og.map(|g| g.to_string()).to_debug_string(), out);

        let og = GaussianRational::try_from(&x);
        assert_eq!(og.map(|g| g.to_string()).to_debug_string(), out);
    };
    test("0.0", "0x0.0", "Ok(\"0\")");
    test("-0.0", "-0x0.0", "Ok(\"0\")");
    test("1.0", "0x1.0#1", "Ok(\"1\")");
    test("-1.0", "-0x1.0#1", "Ok(\"-1\")");
    test("123.0", "0x7b.0#7", "Ok(\"123\")");
    test("0.50", "0x0.8#1", "Ok(\"1/2\")");
    test("-2.5", "-0x2.8#3", "Ok(\"-5/2\")");

    test("NaN", "NaN", "Err(GaussianRationalFromFloatError)");
    test(
        "Infinity",
        "Infinity",
        "Err(GaussianRationalFromFloatError)",
    );
    test(
        "-Infinity",
        "-Infinity",
        "Err(GaussianRationalFromFloatError)",
    );
}

#[test]
#[should_panic]
fn gaussian_rational_exact_from_float_fail() {
    GaussianRational::exact_from(Float::NAN);
}

#[test]
#[should_panic]
fn gaussian_rational_exact_from_float_ref_fail() {
    GaussianRational::exact_from(&Float::INFINITY);
}

#[test]
fn test_convertible_from_float() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        assert_eq!(GaussianRational::convertible_from(&x), out);
    };
    test("0.0", "0x0.0", true);
    test("1.0", "0x1.0#1", true);
    test("-123.0", "-0x7b.0#7", true);
    test("0.50", "0x0.8#1", true);
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", false);
    test("-Infinity", "-Infinity", false);
}

#[test]
fn try_from_float_properties() {
    float_gen().test_properties(|x| {
        let og = GaussianRational::try_from(x.clone());
        assert_eq!(GaussianRational::try_from(&x), og);
        assert_eq!(og.is_ok(), x.is_finite());
        assert_eq!(GaussianRational::convertible_from(&x), og.is_ok());
        if let Ok(g) = og {
            assert_eq!(g.imaginary, 0u32);
            assert_eq!(g.real, Rational::exact_from(&x));
            assert_eq!(Float::exact_from(&g), x);
        }
    });
}
