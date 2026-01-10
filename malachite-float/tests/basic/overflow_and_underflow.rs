// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::{
    float_ordering_pair_gen, float_ordering_pair_gen_var_1,
};
use malachite_float::{Float, test_overflow, test_underflow};

#[test]
fn test_test_overflow() {
    let test = |s, s_hex, o, out: bool| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(test_overflow(&x, o), out);
    };
    test("Infinity", "Infinity", Greater, true);
    // This input doesn't actually make sense, since Infinity can't be less than the actual value
    test("Infinity", "Infinity", Less, false);
    test("-Infinity", "-Infinity", Less, true);
    // Likewise
    test("-Infinity", "-Infinity", Greater, false);

    test("too_big", "0x4.0E+268435455#1", Equal, false);
    test("too_big", "0x4.0E+268435455#1", Less, true);
    test("too_big", "0x6.0E+268435455#2", Less, true);
    test("too_big", "0x7.0E+268435455#3", Less, true);
    test("too_big", "0x7.feE+268435455#10", Less, true);
    test(
        "too_big",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Less,
        true,
    );
    test("too_big", "0x4.0E+268435454#1", Less, false);
    test("too_big", "0x6.0E+268435454#2", Less, false);
    test("too_big", "0x7.0E+268435454#3", Less, false);
    test("too_big", "0x4.0E+268435455#1", Greater, false);
    test("too_big", "0x6.0E+268435455#2", Greater, false);
    test("too_big", "0x7.0E+268435455#3", Greater, false);
    test("too_big", "0x7.feE+268435455#10", Greater, false);
    test(
        "too_big",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Greater,
        false,
    );

    test("-too_big", "-0x4.0E+268435455#1", Equal, false);
    test("-too_big", "-0x4.0E+268435455#1", Greater, true);
    test("-too_big", "-0x6.0E+268435455#2", Greater, true);
    test("-too_big", "-0x7.0E+268435455#3", Greater, true);
    test("-too_big", "-0x7.feE+268435455#10", Greater, true);
    test(
        "-too_big",
        "-0x7.ffffffffffffffffffffffff8E+268435455#100",
        Greater,
        true,
    );
    test("-too_big", "-0x4.0E+268435454#1", Greater, false);
    test("-too_big", "-0x6.0E+268435454#2", Greater, false);
    test("-too_big", "-0x7.0E+268435454#3", Greater, false);
    test("-too_big", "-0x4.0E+268435455#1", Less, false);
    test("-too_big", "-0x6.0E+268435455#2", Less, false);
    test("-too_big", "-0x7.0E+268435455#3", Less, false);
    test("-too_big", "-0x7.feE+268435455#10", Less, false);
    test(
        "-too_big",
        "-0x7.ffffffffffffffffffffffff8E+268435455#100",
        Less,
        false,
    );

    test("0.0", "0x0.0", Equal, false);
    test("-0.0", "-0x0.0", Equal, false);
    test("1.0", "0x1.0#1", Equal, false);
    test("-1.0", "-0x1.0#1", Equal, false);
}

#[allow(clippy::needless_pass_by_value)]
fn test_overflow_properties_helper(x: Float, o: Ordering) {
    let overflow = test_overflow(&x, o);
    assert_eq!(test_overflow(&-x, o.reverse()), overflow);
}

#[test]
fn test_overflow_properties() {
    float_ordering_pair_gen().test_properties(|(x, o)| {
        test_overflow_properties_helper(x, o);
    });

    float_ordering_pair_gen_var_1().test_properties(|(x, o)| {
        test_overflow_properties_helper(x, o);
    });
}

#[test]
fn test_test_underflow() {
    let test = |s, s_hex, o, out: bool| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(test_underflow(&x, o), out);
    };
    test("0.0", "0x0.0", Less, true);
    test("0.0", "0x0.0", Greater, true);
    test("-0.0", "-0x0.0", Less, true);
    test("-0.0", "-0x0.0", Greater, true);

    test("too_small", "0x1.0E-268435456#1", Equal, false);
    test("too_small", "0x1.0E-268435456#1", Greater, true);
    test("too_small", "0x1.0E-268435456#2", Greater, true);
    test("too_small", "0x1.0E-268435456#3", Greater, true);
    test("too_small", "0x1.000E-268435456#10", Greater, true);
    test(
        "too_small",
        "0x1.0000000000000000000000000E-268435456#100",
        Greater,
        true,
    );
    test("too_small", "0x1.8E-268435456#2", Greater, false);
    test("too_small", "0x2.0E-268435456#1", Greater, false);
    test("too_small", "0x1.0E-268435455#1", Greater, false);
    test("too_small", "0x1.0E-268435456#1", Less, false);
    test("too_small", "0x1.0E-268435456#2", Less, false);
    test("too_small", "0x1.0E-268435456#3", Less, false);
    test("too_small", "0x1.000E-268435456#10", Less, false);
    test(
        "too_small",
        "0x1.0000000000000000000000000E-268435456#100",
        Less,
        false,
    );

    test("-too_small", "-0x1.0E-268435456#1", Equal, false);
    test("-too_small", "-0x1.0E-268435456#1", Less, true);
    test("-too_small", "-0x1.0E-268435456#2", Less, true);
    test("-too_small", "-0x1.0E-268435456#3", Less, true);
    test("-too_small", "-0x1.000E-268435456#10", Less, true);
    test(
        "-too_small",
        "-0x1.0000000000000000000000000E-268435456#100",
        Less,
        true,
    );
    test("-too_small", "-0x1.8E-268435456#2", Less, false);
    test("-too_small", "-0x2.0E-268435456#1", Less, false);
    test("-too_small", "-0x1.0E-268435455#1", Less, false);
    test("-too_small", "-0x1.0E-268435456#1", Greater, false);
    test("-too_small", "-0x1.0E-268435456#2", Greater, false);
    test("-too_small", "-0x1.0E-268435456#3", Greater, false);
    test("-too_small", "-0x1.000E-268435456#10", Greater, false);
    test(
        "-too_small",
        "-0x1.0000000000000000000000000E-268435456#100",
        Greater,
        false,
    );

    test("0.0", "0x0.0", Equal, false);
    test("-0.0", "-0x0.0", Equal, false);
    test("1.0", "0x1.0#1", Equal, false);
    test("-1.0", "-0x1.0#1", Equal, false);
}

#[allow(clippy::needless_pass_by_value)]
fn test_underflow_properties_helper(x: Float, o: Ordering) {
    let underflow = test_underflow(&x, o);
    assert_eq!(test_underflow(&-x, o.reverse()), underflow);
}

#[test]
fn test_underflow_properties() {
    float_ordering_pair_gen().test_properties(|(x, o)| {
        test_underflow_properties_helper(x, o);
    });

    float_ordering_pair_gen_var_1().test_properties(|(x, o)| {
        test_underflow_properties_helper(x, o);
    });
}
