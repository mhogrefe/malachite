// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero};
use malachite_base::strings::ToDebugString;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_to_float, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1, float_gen,
};
use std::num::FpCategory;

const fn predicates(x: &BigExponentFloat) -> [bool; 7] {
    [
        x.is_nan(),
        x.is_finite(),
        x.is_infinite(),
        x.is_zero(),
        x.is_normal(),
        x.is_sign_positive(),
        x.is_sign_negative(),
    ]
}

#[test]
fn test_classification() {
    // - out: is_nan, is_finite, is_infinite, is_zero, is_normal, is_sign_positive, is_sign_negative
    let test = |s, s_hex, out| {
        let x = parse_big_exponent_float_hex(s_hex);
        assert_eq!(x.to_string(), s);
        assert_eq!(predicates(&x), out, "{s_hex}");
    };
    test(
        "NaN",
        "NaN",
        [true, false, false, false, false, false, false],
    );
    test(
        "Infinity",
        "Infinity",
        [false, false, true, false, false, true, false],
    );
    test(
        "-Infinity",
        "-Infinity",
        [false, false, true, false, false, false, true],
    );
    test(
        "0.0",
        "0x0.0",
        [false, true, false, true, false, true, false],
    );
    test(
        "1.0",
        "0x1.0#1",
        [false, true, false, false, true, true, false],
    );
    test(
        "-1.0",
        "-0x1.0#1",
        [false, true, false, false, true, false, true],
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        [false, true, false, false, true, true, false],
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        [false, true, false, false, true, false, true],
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        [false, true, false, false, true, true, false],
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        [false, true, false, false, true, true, false],
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        [false, true, false, false, true, true, false],
    );
    assert_eq!(BigExponentFloat::NAN.classify(), FpCategory::Nan);
    assert_eq!(BigExponentFloat::INFINITY.classify(), FpCategory::Infinite);
    assert_eq!(
        BigExponentFloat::NEGATIVE_INFINITY.classify(),
        FpCategory::Infinite
    );
    assert_eq!(BigExponentFloat::ZERO.classify(), FpCategory::Zero);
    assert_eq!(BigExponentFloat::ONE.classify(), FpCategory::Normal);
    assert_eq!(
        BigExponentFloat::NEGATIVE_ONE.classify(),
        FpCategory::Normal
    );
    assert_eq!(
        parse_big_exponent_float_hex("0x1.0E+25000000000000000000").classify(),
        FpCategory::Normal
    );
}

#[test]
fn test_into_and_to_finite_and_non_nan() {
    let test = |s,
                s_hex,
                non_nan: Option<&str>,
                non_nan_hex: Option<&str>,
                finite: Option<&str>,
                finite_hex: Option<&str>| {
        let x = parse_big_exponent_float_hex(s_hex);
        assert_eq!(x.to_string(), s);
        let check = |y: Option<BigExponentFloat>, out: Option<&str>, out_hex: Option<&str>| {
            assert!(y.as_ref().is_none_or(BigExponentFloat::is_valid));
            assert_eq!(y.as_ref().map(ToString::to_string).as_deref(), out);
            assert_eq!(
                y.as_ref().map(ToDebugString::to_debug_string).as_deref(),
                out_hex
            );
        };
        check(x.to_non_nan(), non_nan, non_nan_hex);
        check(x.clone().into_non_nan(), non_nan, non_nan_hex);
        check(x.to_finite(), finite, finite_hex);
        check(x.into_finite(), finite, finite_hex);
    };
    test("NaN", "NaN", None, None, None, None);
    test(
        "Infinity",
        "Infinity",
        Some("Infinity"),
        Some("Infinity"),
        None,
        None,
    );
    test(
        "-Infinity",
        "-Infinity",
        Some("-Infinity"),
        Some("-Infinity"),
        None,
        None,
    );
    test(
        "0.0",
        "0x0.0",
        Some("0.0"),
        Some("0x0.0"),
        Some("0.0"),
        Some("0x0.0"),
    );
    test(
        "-1.5",
        "-0x1.8#2",
        Some("-1.5"),
        Some("-0x1.8"),
        Some("-1.5"),
        Some("-0x1.8"),
    );
}

#[test]
fn classification_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let [nan, finite, infinite, zero, normal, positive, negative] = predicates(&x);
        assert_eq!(nan, !finite && !infinite);
        assert!(!(finite && infinite));
        assert_eq!(normal, finite && !zero);
        assert!(!(positive && negative));
        assert_eq!(nan, !positive && !negative);
        assert_eq!(
            x.classify(),
            if nan {
                FpCategory::Nan
            } else if infinite {
                FpCategory::Infinite
            } else if zero {
                FpCategory::Zero
            } else {
                FpCategory::Normal
            }
        );
        assert_eq!(x.to_non_nan().is_some(), !nan);
        assert_eq!(x.to_finite().is_some(), finite);
        assert_eq!(
            format!("{:?}", x.to_non_nan()),
            format!("{:?}", x.clone().into_non_nan())
        );
        assert_eq!(
            format!("{:?}", x.to_finite()),
            format!("{:?}", x.clone().into_finite())
        );
    });

    // - agreement with `Float`
    big_exponent_float_gen_var_1().test_properties(|x| {
        let f = big_exponent_float_to_float(&x).unwrap();
        assert_eq!(
            predicates(&x),
            [
                f.is_nan(),
                f.is_finite(),
                f.is_infinite(),
                f.is_zero(),
                f.is_normal(),
                f.is_sign_positive(),
                f.is_sign_negative(),
            ]
        );
        assert_eq!(x.classify(), f.classify());
    });

    // - agreement with `Float`, except for the sign of negative zero, which is lost
    float_gen().test_properties(|f| {
        let x = float_to_big_exponent_float(&f);
        assert!(x.is_valid());
        assert_eq!(x.is_nan(), f.is_nan());
        assert_eq!(x.is_finite(), f.is_finite());
        assert_eq!(x.is_zero(), f.is_zero());
        assert_eq!(x.is_normal(), f.is_normal());
        if !f.is_zero() {
            assert_eq!(x.is_sign_negative(), f.is_sign_negative());
        }
        assert_eq!(x.classify(), f.classify());
    });
}
