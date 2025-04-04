// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_float::Float;
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::{float_gen, float_gen_var_4, float_gen_var_12};
use malachite_q::Rational;

#[test]
fn test_is_power_of_2() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.is_power_of_2(), out);
    };
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", false);
    test("-Infinity", "-Infinity", false);
    test("0.0", "0x0.0", false);
    test("-0.0", "-0x0.0", false);

    test("1.0", "0x1.0#1", true);
    test("2.0", "0x2.0#1", true);
    test("0.5", "0x0.8#1", true);
    test("0.33333333333333331", "0x0.55555555555554#53", false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", false);
    test("too_big", "0x4.0E+268435455#1", true);
    test("too_small", "0x1.0E-268435456#1", true);

    test("-1.0", "-0x1.0#1", false);
    test("-2.0", "-0x2.0#1", false);
    test("-0.5", "-0x0.8#1", false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", false);
    test("-too_big", "-0x4.0E+268435455#1", false);
    test("-too_small", "-0x1.0E-268435456#1", false);
}

#[allow(clippy::needless_pass_by_value)]
fn is_power_of_2_properties_helper(x: Float) {
    let is_power_of_2 = x.is_power_of_2();
    if is_power_of_2 {
        assert!(x > 0u32);
    }
}

#[test]
fn is_power_of_2_properties() {
    float_gen().test_properties(|x| {
        is_power_of_2_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        is_power_of_2_properties_helper(x);
    });

    float_gen_var_4().test_properties(|x| {
        assert_eq!(x.is_power_of_2(), Rational::exact_from(x).is_power_of_2());
    });

    primitive_float_gen::<f64>().test_properties(|x| {
        assert_eq!(x.is_power_of_2(), Float::from(x).is_power_of_2());
    });
}
