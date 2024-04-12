// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, AbsAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_float::test_util::common::{parse_hex_string, to_hex_string};
use malachite_float::test_util::generators::{float_gen, float_gen_var_4};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;

#[test]
fn test_abs_negative_zero() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let abs = x.clone().abs_negative_zero();
        assert!(abs.is_valid());

        assert_eq!(abs.to_string(), out);
        assert_eq!(to_hex_string(&abs), out_hex);

        let abs_alt = x.abs_negative_zero_ref();
        assert!(abs_alt.is_valid());
        assert_eq!(ComparableFloatRef(&abs), ComparableFloatRef(&abs_alt));

        let mut abs_alt = x;
        abs_alt.abs_negative_zero_assign();
        assert!(abs_alt.is_valid());
        assert_eq!(ComparableFloatRef(&abs), ComparableFloatRef(&abs_alt));
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "-Infinity", "-Infinity");
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "0.0", "0x0.0");

    test("1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("2.0", "0x2.0#1", "2.0", "0x2.0#1");
    test("0.5", "0x0.8#1", "0.5", "0x0.8#1");
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "0.33333333333333331",
        "0x0.55555555555554#53",
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
    );

    test("-1.0", "-0x1.0#1", "-1.0", "-0x1.0#1");
    test("-2.0", "-0x2.0#1", "-2.0", "-0x2.0#1");
    test("-0.5", "-0x0.8#1", "-0.5", "-0x0.8#1");
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
    );
}

#[test]
fn abs_negative_zero_properties() {
    float_gen().test_properties(|x| {
        let abs = x.clone().abs_negative_zero();
        assert!(abs.is_valid());

        let abs_alt = x.abs_negative_zero_ref();
        assert!(abs_alt.is_valid());
        assert_eq!(ComparableFloatRef(&abs), ComparableFloatRef(&abs_alt));

        let mut abs_alt = x.clone();
        abs_alt.abs_negative_zero_assign();
        assert!(abs_alt.is_valid());
        assert_eq!(ComparableFloatRef(&abs), ComparableFloatRef(&abs_alt));

        if x.is_negative_zero() {
            assert_eq!(ComparableFloatRef(&abs), ComparableFloatRef(&Float::ZERO));
        } else {
            assert_eq!(ComparableFloatRef(&abs), ComparableFloatRef(&x));
        }
        assert_eq!(
            ComparableFloat(abs.abs_negative_zero_ref()),
            ComparableFloat(abs)
        );
    });
}

#[test]
fn test_abs() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let abs = x.clone().abs();
        assert!(abs.is_valid());

        assert_eq!(abs.to_string(), out);
        assert_eq!(to_hex_string(&abs), out_hex);

        let abs_alt = (&x).abs();
        assert!(abs_alt.is_valid());
        assert_eq!(ComparableFloatRef(&abs), ComparableFloatRef(&abs_alt));

        let mut abs_alt = x.clone();
        abs_alt.abs_assign();
        assert!(abs_alt.is_valid());
        assert_eq!(ComparableFloatRef(&abs), ComparableFloatRef(&abs_alt));

        assert_eq!(
            ComparableFloat(Float::from(&rug::Float::exact_from(&x).abs())),
            ComparableFloat(abs)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "Infinity", "Infinity");
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "0.0", "0x0.0");

    test("1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("2.0", "0x2.0#1", "2.0", "0x2.0#1");
    test("0.5", "0x0.8#1", "0.5", "0x0.8#1");
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "0.33333333333333331",
        "0x0.55555555555554#53",
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
    );

    test("-1.0", "-0x1.0#1", "1.0", "0x1.0#1");
    test("-2.0", "-0x2.0#1", "2.0", "0x2.0#1");
    test("-0.5", "-0x0.8#1", "0.5", "0x0.8#1");
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "0.33333333333333331",
        "0x0.55555555555554#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
    );
}

#[test]
fn abs_properties() {
    float_gen().test_properties(|x| {
        let abs = x.clone().abs();
        assert!(abs.is_valid());

        let abs_alt = (&x).abs();
        assert!(abs_alt.is_valid());
        assert_eq!(ComparableFloatRef(&abs), ComparableFloatRef(&abs_alt));

        let mut abs_alt = x.clone();
        abs_alt.abs_assign();
        assert!(abs_alt.is_valid());
        assert_eq!(ComparableFloatRef(&abs), ComparableFloatRef(&abs_alt));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug::Float::exact_from(&x).abs())),
            ComparableFloatRef(&abs)
        );

        if x.is_sign_negative() {
            assert_eq!(ComparableFloatRef(&abs), ComparableFloatRef(&-&x));
        } else {
            assert_eq!(ComparableFloatRef(&abs), ComparableFloatRef(&x));
        }
        assert_eq!(ComparableFloatRef(&(&abs).abs()), ComparableFloatRef(&abs));
        assert_eq!(ComparableFloat((-x).abs()), ComparableFloat(abs));
    });

    float_gen_var_4().test_properties(|x| {
        assert_eq!(
            Rational::exact_from((&x).abs()),
            Rational::exact_from(x).abs()
        );
    });

    primitive_float_gen::<f64>().test_properties(|x| {
        assert_eq!(
            ComparableFloat(Float::from(x.abs())),
            ComparableFloat(Float::from(x).abs())
        );
    });
}
