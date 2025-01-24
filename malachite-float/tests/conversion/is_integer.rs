// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::IsInteger;
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::{float_gen, float_gen_var_12};
use malachite_float::Float;
use malachite_q::Rational;

#[test]
fn test_is_integer() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.is_integer(), out);
    };
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", false);
    test("-Infinity", "-Infinity", false);
    test("0.0", "0x0.0", true);
    test("-0.0", "-0x0.0", true);

    test("1.0", "0x1.0#1", true);
    test("2.0", "0x2.0#1", true);
    test("0.5", "0x0.8#1", false);
    test("0.33333333333333331", "0x0.55555555555554#53", false);
    test("123.0", "0x7b.0#7", true);
    test("1000000000000.0", "0xe8d4a51000.0#40", true);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", false);
    test("3.141592653589793", "0x3.243f6a8885a3#50", false);
    test("2.7182818284590451", "0x2.b7e151628aed2#53", false);
    test("too_big", "0x4.0E+268435455#1", true);
    test("too_small", "0x1.0E-268435456#1", false);

    test("-1.0", "-0x1.0#1", true);
    test("-2.0", "-0x2.0#1", true);
    test("-0.5", "-0x0.8#1", false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", false);
    test("-123.0", "-0x7b.0#7", true);
    test("-1000000000000.0", "-0xe8d4a51000.0#40", true);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", false);
    test("-3.141592653589793", "-0x3.243f6a8885a3#50", false);
    test("-2.7182818284590451", "-0x2.b7e151628aed2#53", false);
    test("-too_big", "-0x4.0E+268435455#1", true);
    test("-too_small", "-0x1.0E-268435456#1", false);
}

#[allow(clippy::needless_pass_by_value)]
fn is_integer_properties_helper(x: Float, extreme: bool) {
    assert_eq!(x.is_integer(), (-&x).is_integer());
    if !extreme {
        if let Ok(q) = Rational::try_from(&x) {
            assert_eq!(q.is_integer(), x.is_integer());
        }
    }
}

#[test]
fn is_integer_properties() {
    float_gen().test_properties(|x| is_integer_properties_helper(x, false));

    float_gen_var_12().test_properties(|x| is_integer_properties_helper(x, true));
}
