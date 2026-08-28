// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom};
use malachite_base::strings::ToDebugString;
use malachite_float::Float;
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::float_gen;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;

#[test]
fn test_try_from_float() {
    let test = |s, s_hex, out: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let og = GaussianInteger::try_from(x.clone());
        assert_eq!(og.map(|g| g.to_string()).to_debug_string(), out);

        let og = GaussianInteger::try_from(&x);
        assert_eq!(og.map(|g| g.to_string()).to_debug_string(), out);
    };
    test("0.0", "0x0.0", "Ok(\"0\")");
    test("-0.0", "-0x0.0", "Ok(\"0\")");
    test("1.0", "0x1.0#1", "Ok(\"1\")");
    test("-1.0", "-0x1.0#1", "Ok(\"-1\")");
    test("123.0", "0x7b.0#7", "Ok(\"123\")");
    test(
        "1000000000000.0",
        "0xe8d4a51000.0#40",
        "Ok(\"1000000000000\")",
    );

    test("NaN", "NaN", "Err(GaussianIntegerFromFloatError)");
    test("Infinity", "Infinity", "Err(GaussianIntegerFromFloatError)");
    test(
        "-Infinity",
        "-Infinity",
        "Err(GaussianIntegerFromFloatError)",
    );
    test("0.50", "0x0.8#1", "Err(GaussianIntegerFromFloatError)");
    test("-2.5", "-0x2.8#3", "Err(GaussianIntegerFromFloatError)");
}

#[test]
#[should_panic]
fn gaussian_integer_exact_from_float_fail() {
    GaussianInteger::exact_from(Float::from(1.5));
}

#[test]
#[should_panic]
fn gaussian_integer_exact_from_float_ref_fail() {
    GaussianInteger::exact_from(&Float::from(1.5));
}

#[test]
fn test_convertible_from_float() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        assert_eq!(GaussianInteger::convertible_from(&x), out);
    };
    test("0.0", "0x0.0", true);
    test("1.0", "0x1.0#1", true);
    test("-123.0", "-0x7b.0#7", true);
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", false);
    test("0.50", "0x0.8#1", false);
}

#[test]
fn try_from_float_properties() {
    float_gen().test_properties(|x| {
        let og = GaussianInteger::try_from(x.clone());
        assert_eq!(GaussianInteger::try_from(&x), og);
        assert_eq!(og.is_ok(), Integer::convertible_from(&x));
        assert_eq!(GaussianInteger::convertible_from(&x), og.is_ok());
        if let Ok(g) = og {
            assert_eq!(g.imaginary, 0u32);
            assert_eq!(g.real, Integer::exact_from(&x));
            assert_eq!(Float::exact_from(&g), x);
        }
    });
}
