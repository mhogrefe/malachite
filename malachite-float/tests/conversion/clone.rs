// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_pair_gen, float_pair_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::test_util::generators::integer_pair_gen;
use rug;

#[test]
#[allow(clippy::redundant_clone)]
fn test_clone() {
    let test = |s, s_hex| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let x_alt = x.clone();
        assert!(x.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));

        let r_x = rug::Float::exact_from(&x).clone();
        assert_eq!(ComparableFloat(Float::from(&r_x)), ComparableFloat(x));
    };
    test("NaN", "NaN");
    test("Infinity", "Infinity");
    test("-Infinity", "-Infinity");
    test("0.0", "0x0.0");
    test("-0.0", "-0x0.0");

    test("1.0", "0x1.0#1");
    test("2.0", "0x2.0#1");
    test("0.5", "0x0.8#1");
    test("0.33333333333333331", "0x0.55555555555554#53");
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53");
    test("3.1415926535897931", "0x3.243f6a8885a30#53");

    test("-1.0", "-0x1.0#1");
    test("-2.0", "-0x2.0#1");
    test("-0.5", "-0x0.8#1");
    test("-0.33333333333333331", "-0x0.55555555555554#53");
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53");
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53");
}

#[test]
fn test_clone_from() {
    let test = |s, s_hex, t, t_hex| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let mut x_alt = x.clone();
        x_alt.clone_from(&y);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&y));

        let mut rug_x = rug::Float::exact_from(&x);
        let rug_y = rug::Float::exact_from(&y);
        rug_x.clone_from(&rug_y);
        assert_eq!(
            ComparableFloat(Float::from(&rug_x)),
            ComparableFloat(Float::from(&rug_y))
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("NaN", "NaN", "0.0", "0x0.0");
    test("NaN", "NaN", "Infinity", "Infinity");
    test("NaN", "NaN", "-Infinity", "-Infinity");
    test("NaN", "NaN", "0.33333333333333331", "0x0.55555555555554#53");

    test("0.0", "0x0.0", "NaN", "NaN");
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("0.0", "0x0.0", "Infinity", "Infinity");
    test("0.0", "0x0.0", "-Infinity", "-Infinity");
    test(
        "0.0",
        "0x0.0",
        "0.33333333333333331",
        "0x0.55555555555554#53",
    );

    test("Infinity", "Infinity", "NaN", "NaN");
    test("Infinity", "Infinity", "0.0", "0x0.0");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("Infinity", "Infinity", "-Infinity", "-Infinity");
    test(
        "Infinity",
        "Infinity",
        "0.33333333333333331",
        "0x0.55555555555554#53",
    );

    test("-Infinity", "-Infinity", "NaN", "NaN");
    test("-Infinity", "-Infinity", "0.0", "0x0.0");
    test("-Infinity", "-Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "-Infinity", "-Infinity");
    test(
        "-Infinity",
        "-Infinity",
        "0.33333333333333331",
        "0x0.55555555555554#53",
    );

    test("0.33333333333333331", "0x0.55555555555554#53", "NaN", "NaN");
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "0.0",
        "0x0.0",
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "Infinity",
        "Infinity",
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "-Infinity",
        "-Infinity",
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "0.33333333333333331",
        "0x0.55555555555554#53",
    );
}

fn clone_and_clone_from_properties_helper_1(x: Float) {
    let mut_x = x.clone();
    assert!(mut_x.is_valid());
    assert_eq!(ComparableFloatRef(&mut_x), ComparableFloatRef(&x));

    assert_eq!(
        ComparableFloat(Float::from(&rug::Float::exact_from(&x).clone())),
        ComparableFloat(x)
    );
}

#[allow(clippy::needless_pass_by_value)]
fn clone_and_clone_from_properties_helper_2(x: Float, y: Float) {
    let mut mut_x = x.clone();
    mut_x.clone_from(&y);
    assert!(mut_x.is_valid());
    assert_eq!(ComparableFloatRef(&mut_x), ComparableFloatRef(&y));

    let mut rug_x = rug::Float::exact_from(&x);
    rug_x.clone_from(&rug::Float::exact_from(&y));
    assert_eq!(ComparableFloat(Float::from(&rug_x)), ComparableFloat(y));
}

#[allow(clippy::redundant_clone)]
#[test]
fn clone_and_clone_from_properties() {
    float_gen().test_properties(|x| {
        clone_and_clone_from_properties_helper_1(x);
    });

    float_gen_var_12().test_properties(|x| {
        clone_and_clone_from_properties_helper_1(x);
    });

    float_pair_gen().test_properties(|(x, y)| {
        clone_and_clone_from_properties_helper_2(x, y);
    });

    float_pair_gen_var_10().test_properties(|(x, y)| {
        clone_and_clone_from_properties_helper_2(x, y);
    });

    integer_pair_gen().test_properties(|(i, j)| {
        let x = Float::exact_from(&i);
        let y = Float::exact_from(&j);

        let mut mut_i = i.clone();
        let mut mut_x = x.clone();
        mut_i.clone_from(&j);
        mut_x.clone_from(&y);
        assert_eq!(mut_x, mut_i);
    });
}
