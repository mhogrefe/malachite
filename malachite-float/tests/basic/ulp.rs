// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, SciMantissaAndExponent};
use malachite_base::test_util::generators::primitive_float_gen_var_12;
use malachite_float::test_util::common::{parse_hex_string, to_hex_string};
use malachite_float::test_util::generators::{float_gen, float_gen_var_3};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::panic::catch_unwind;

#[allow(clippy::redundant_closure_for_method_calls)]
#[test]
fn test_ulp() {
    let test = |s, s_hex, out: Option<&str>, out_hex: Option<&str>| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let actual_out = x.ulp();
        assert!(actual_out.as_ref().map_or(true, Float::is_valid));

        let s = actual_out.as_ref().map(|x| x.to_string());
        assert_eq!(s.as_deref(), out);
        let s = actual_out.map(|x| to_hex_string(&x));
        assert_eq!(s.as_deref(), out_hex);
    };
    test("NaN", "NaN", None, None);
    test("Infinity", "Infinity", None, None);
    test("-Infinity", "-Infinity", None, None);
    test("0.0", "0x0.0", None, None);
    test("-0.0", "-0x0.0", None, None);

    test("1.0", "0x1.0#1", Some("1.0"), Some("0x1.0#1"));
    test("2.0", "0x2.0#1", Some("2.0"), Some("0x2.0#1"));
    test("0.5", "0x0.8#1", Some("0.5"), Some("0x0.8#1"));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Some("6.0e-17"),
        Some("0x4.0E-14#1"),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Some("2.0e-16"),
        Some("0x1.0E-13#1"),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Some("4.0e-16"),
        Some("0x2.0E-13#1"),
    );

    test("-1.0", "-0x1.0#1", Some("1.0"), Some("0x1.0#1"));
    test("-2.0", "-0x2.0#1", Some("2.0"), Some("0x2.0#1"));
    test("-0.5", "-0x0.8#1", Some("0.5"), Some("0x0.8#1"));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Some("6.0e-17"),
        Some("0x4.0E-14#1"),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        Some("2.0e-16"),
        Some("0x1.0E-13#1"),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Some("4.0e-16"),
        Some("0x2.0E-13#1"),
    );
}

#[test]
fn ulp_properties() {
    float_gen().test_properties(|x| {
        let ulp = x.ulp();
        ulp.as_ref().map_or_else(
            || {
                assert!(!x.is_normal());
            },
            |ulp| {
                assert!(ulp.is_valid());
                assert!(ulp.is_power_of_2());
            },
        );
        assert_eq!((-x).ulp().map(ComparableFloat), ulp.map(ComparableFloat));
    });
}

#[test]
fn test_increment() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let mut x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        x.increment();
        assert!(x.is_valid());

        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
    };
    test("1.0", "0x1.0#1", "2.0", "0x2.0#2");
    test("2.0", "0x2.0#1", "4.0", "0x4.0#2");
    test("0.5", "0x0.8#1", "1.0", "0x1.0#2");
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "0.33333333333333337",
        "0x0.55555555555558#53",
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "1.4142135623730954",
        "0x1.6a09e667f3bce#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3.1415926535897936",
        "0x3.243f6a8885a32#53",
    );

    test("-1.0", "-0x1.0#1", "-0.0", "-0x0.0");
    test("-2.0", "-0x2.0#1", "-0.0", "-0x0.0");
    test("-0.5", "-0x0.8#1", "-0.0", "-0x0.0");
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        "-0.999999999999999999999999999998",
        "-0x0.ffffffffffffffffffffffffe#99",
    );
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "-0.33333333333333326",
        "-0x0.55555555555550#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-1.4142135623730949",
        "-0x1.6a09e667f3bcc#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3.1415926535897927",
        "-0x3.243f6a8885a2e#53",
    );
}

#[test]
fn increment_fail() {
    assert_panic!({
        let mut x = Float::NAN;
        x.increment();
    });
    assert_panic!({
        let mut x = Float::INFINITY;
        x.increment();
    });
    assert_panic!({
        let mut x = Float::NEGATIVE_INFINITY;
        x.increment();
    });
    assert_panic!({
        let mut x = Float::ZERO;
        x.increment();
    });
    assert_panic!({
        let mut x = Float::NEGATIVE_ZERO;
        x.increment();
    });
}

#[test]
fn increment_properties() {
    float_gen_var_3().test_properties(|mut x| {
        let old_x = x.clone();
        x.increment();
        let final_x = x.clone();
        assert!(x.is_valid());
        assert_eq!(
            Rational::exact_from(&old_x) + Rational::exact_from(old_x.ulp().unwrap()),
            Rational::exact_from(&x)
        );

        if x != 0u32 {
            x.decrement();
            assert_eq!(ComparableFloatRef(&x), ComparableFloatRef(&old_x));
        }
        let mut x = -old_x;
        x.decrement();
        assert_eq!(ComparableFloat(x), ComparableFloat(-final_x));
    });

    primitive_float_gen_var_12::<f64>().test_properties(|x| {
        let next_x = x.next_higher();
        if next_x.is_finite() && next_x != 0.0 && x.sci_exponent() == next_x.sci_exponent() {
            let mut big_x = Float::from(x);
            big_x.increment();
            assert_eq!(ComparableFloat(big_x), ComparableFloat(Float::from(next_x)));
        }
    });
}

#[test]
fn test_decrement() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let mut x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        x.decrement();
        assert!(x.is_valid());

        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
    };
    test("1.0", "0x1.0#1", "0.0", "0x0.0");
    test("2.0", "0x2.0#1", "0.0", "0x0.0");
    test("0.5", "0x0.8#1", "0.0", "0x0.0");
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "0.33333333333333326",
        "0x0.55555555555550#53",
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "1.4142135623730949",
        "0x1.6a09e667f3bcc#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "3.1415926535897927",
        "0x3.243f6a8885a2e#53",
    );

    test("-1.0", "-0x1.0#1", "-2.0", "-0x2.0#2");
    test("-2.0", "-0x2.0#1", "-4.0", "-0x4.0#2");
    test("-0.5", "-0x0.8#1", "-1.0", "-0x1.0#2");
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        "-1.000000000000000000000000000002",
        "-0x1.0000000000000000000000002#100",
    );
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "-0.33333333333333337",
        "-0x0.55555555555558#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-1.4142135623730954",
        "-0x1.6a09e667f3bce#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-3.1415926535897936",
        "-0x3.243f6a8885a32#53",
    );
}

#[test]
fn decrement_fail() {
    assert_panic!({
        let mut x = Float::NAN;
        x.decrement();
    });
    assert_panic!({
        let mut x = Float::INFINITY;
        x.decrement();
    });
    assert_panic!({
        let mut x = Float::NEGATIVE_INFINITY;
        x.decrement();
    });
    assert_panic!({
        let mut x = Float::ZERO;
        x.decrement();
    });
    assert_panic!({
        let mut x = Float::NEGATIVE_ZERO;
        x.decrement();
    });
}

#[test]
fn decrement_properties() {
    float_gen_var_3().test_properties(|mut x| {
        let old_x = x.clone();
        x.decrement();
        let final_x = x.clone();
        assert!(x.is_valid());
        assert_eq!(
            Rational::exact_from(&old_x) - Rational::exact_from(old_x.ulp().unwrap()),
            Rational::exact_from(&x)
        );

        if x != 0u32 {
            x.increment();
            assert_eq!(ComparableFloatRef(&x), ComparableFloatRef(&old_x));
        }
        let mut x = -old_x;
        x.increment();
        assert_eq!(ComparableFloat(x), ComparableFloat(-final_x));
    });

    primitive_float_gen_var_12::<f64>().test_properties(|x| {
        let next_x = x.next_lower();
        if next_x.is_finite() && next_x != 0.0 && x.sci_exponent() == next_x.sci_exponent() {
            let mut big_x = Float::from(x);
            big_x.decrement();
            assert_eq!(ComparableFloat(big_x), ComparableFloat(Float::from(next_x)));
        }
    });
}
