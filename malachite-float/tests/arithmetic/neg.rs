// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_float::test_util::common::{parse_hex_string, to_hex_string};
use malachite_float::test_util::generators::{float_gen, float_gen_var_12, float_gen_var_4};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;

#[test]
fn test_neg() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let neg = -x.clone();
        assert!(neg.is_valid());

        assert_eq!(neg.to_string(), out);
        assert_eq!(to_hex_string(&neg), out_hex);

        let neg_alt = -&x;
        assert!(neg_alt.is_valid());
        assert_eq!(ComparableFloatRef(&neg), ComparableFloatRef(&neg_alt));

        let mut neg_alt = x.clone();
        neg_alt.neg_assign();
        assert!(neg_alt.is_valid());
        assert_eq!(ComparableFloatRef(&neg), ComparableFloatRef(&neg_alt));

        assert_eq!(
            ComparableFloat(Float::from(&-rug::Float::exact_from(&x))),
            ComparableFloat(neg)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", "Infinity", "Infinity");
    test("0.0", "0x0.0", "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", "0.0", "0x0.0");

    test("1.0", "0x1.0#1", "-1.0", "-0x1.0#1");
    test("2.0", "0x2.0#1", "-2.0", "-0x2.0#1");
    test("0.5", "0x0.8#1", "-0.5", "-0x0.8#1");
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
    );
    test(
        "too_big",
        "0x4.0E+268435455#1",
        "-too_big",
        "-0x4.0E+268435455#1",
    );
    test(
        "too_small",
        "0x1.0E-268435456#1",
        "-too_small",
        "-0x1.0E-268435456#1",
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
    test(
        "-too_big",
        "-0x4.0E+268435455#1",
        "too_big",
        "0x4.0E+268435455#1",
    );
    test(
        "-too_small",
        "-0x1.0E-268435456#1",
        "too_small",
        "0x1.0E-268435456#1",
    );
}

fn neg_properties_helper(x: Float) {
    let neg = -x.clone();
    assert!(neg.is_valid());

    let neg_alt = -&x;
    assert!(neg_alt.is_valid());
    assert_eq!(ComparableFloatRef(&neg), ComparableFloatRef(&neg_alt));

    let mut neg_alt = x.clone();
    neg_alt.neg_assign();
    assert!(neg_alt.is_valid());
    assert_eq!(ComparableFloatRef(&neg), ComparableFloatRef(&neg_alt));

    assert_eq!(
        ComparableFloatRef(&-Float::from(&rug::Float::exact_from(&x))),
        ComparableFloatRef(&neg)
    );
    assert_eq!(
        ComparableFloatRef(&neg) == ComparableFloatRef(&x),
        x.is_nan()
    );

    assert_eq!(ComparableFloat(-neg), ComparableFloat(x));
}

#[test]
fn neg_properties() {
    float_gen().test_properties(|x| {
        neg_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        neg_properties_helper(x);
    });

    float_gen_var_4().test_properties(|x| {
        assert_eq!(Rational::exact_from(-&x), -Rational::exact_from(x));
    });

    primitive_float_gen::<f64>().test_properties(|x| {
        assert_eq!(
            ComparableFloat(Float::from(-x)),
            ComparableFloat(-Float::from(x))
        );
    });
}
