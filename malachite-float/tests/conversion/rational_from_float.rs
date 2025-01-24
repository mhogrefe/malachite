// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom};
use malachite_base::strings::ToDebugString;
use malachite_float::conversion::rational_from_float::RationalFromFloatError;
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::float_gen;
use malachite_float::Float;
use malachite_q::Rational;

#[test]
fn test_try_from_float() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let on = Rational::try_from(x.clone());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Rational::try_from(&x);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("NaN", "NaN", "Err(RationalFromFloatError)");
    test("Infinity", "Infinity", "Err(RationalFromFloatError)");
    test("-Infinity", "-Infinity", "Err(RationalFromFloatError)");
    test("0.0", "0x0.0", "Ok(0)");
    test("-0.0", "-0x0.0", "Ok(0)");

    test("1.0", "0x1.0#1", "Ok(1)");
    test("2.0", "0x2.0#1", "Ok(2)");
    test("0.5", "0x0.8#1", "Ok(1/2)");
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "Ok(6004799503160661/18014398509481984)",
    );
    test("123.0", "0x7b.0#7", "Ok(123)");
    test("1000000000000.0", "0xe8d4a51000.0#40", "Ok(1000000000000)");
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "Ok(6369051672525773/4503599627370496)",
    );
    test(
        "3.141592653589793",
        "0x3.243f6a8885a3#50",
        "Ok(884279719003555/281474976710656)",
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        "Ok(6121026514868073/2251799813685248)",
    );

    test("-1.0", "-0x1.0#1", "Ok(-1)");
    test("-2.0", "-0x2.0#1", "Ok(-2)");
    test("-0.5", "-0x0.8#1", "Ok(-1/2)");
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "Ok(-6004799503160661/18014398509481984)",
    );
    test("-123.0", "-0x7b.0#7", "Ok(-123)");
    test(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        "Ok(-1000000000000)",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "Ok(-6369051672525773/4503599627370496)",
    );
    test(
        "-3.141592653589793",
        "-0x3.243f6a8885a3#50",
        "Ok(-884279719003555/281474976710656)",
    );
    test(
        "-2.7182818284590451",
        "-0x2.b7e151628aed2#53",
        "Ok(-6121026514868073/2251799813685248)",
    );
}

#[test]
fn test_convertible_from_float() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(Rational::convertible_from(&x), out);
    };
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", false);
    test("-Infinity", "-Infinity", false);
    test("0.0", "0x0.0", true);
    test("-0.0", "-0x0.0", true);

    test("1.0", "0x1.0#1", true);
    test("2.0", "0x2.0#1", true);
    test("0.5", "0x0.8#1", true);
    test("0.33333333333333331", "0x0.55555555555554#53", true);
    test("123.0", "0x7b.0#7", true);
    test("1000000000000.0", "0xe8d4a51000.0#40", true);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", true);
    test("3.141592653589793", "0x3.243f6a8885a3#50", true);
    test("2.7182818284590451", "0x2.b7e151628aed2#53", true);

    test("-1.0", "-0x1.0#1", true);
    test("-2.0", "-0x2.0#1", true);
    test("-0.5", "-0x0.8#1", true);
    test("-0.33333333333333331", "-0x0.55555555555554#53", true);
    test("-123.0", "-0x7b.0#7", true);
    test("-1000000000000.0", "-0xe8d4a51000.0#40", true);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", true);
    test("-3.141592653589793", "-0x3.243f6a8885a3#50", true);
    test("-2.7182818284590451", "-0x2.b7e151628aed2#53", true);
}

#[test]
fn try_from_float_properties() {
    float_gen().test_properties(|x| {
        let rational_x = Rational::try_from(x.clone());
        assert!(rational_x.as_ref().map_or(true, Rational::is_valid));

        let rational_x_alt = Rational::try_from(&x);
        assert!(rational_x_alt.as_ref().map_or(true, Rational::is_valid));
        assert_eq!(rational_x, rational_x_alt);

        assert_eq!(
            rug::Rational::try_from(rug::Float::exact_from(&x))
                .map(|q| Rational::from(&q))
                .map_err(|_| RationalFromFloatError),
            rational_x
        );

        assert_eq!(rational_x.is_ok(), Rational::convertible_from(&x));
        if let Ok(n) = rational_x {
            assert_eq!(Rational::exact_from(&x), n);
            assert_eq!(n, x);
            assert_eq!(Float::exact_from(&n), x);
            assert_eq!(Float::exact_from(n.clone()), x);
            assert!(n.denominator_ref().is_power_of_2());
        }
    });
}

#[test]
fn convertible_from_float_properties() {
    float_gen().test_properties(|x| {
        assert_eq!(Rational::convertible_from(&x), x.is_finite());
    });
}
