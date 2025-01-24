// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::NaN;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::primitive_float_pair_gen;
use malachite_float::test_util::common::{
    parse_hex_string, ORDERED_FLOAT_HEX_STRINGS, ORDERED_FLOAT_STRINGS,
};
use malachite_float::test_util::generators::{
    float_gen, float_pair_gen, float_pair_gen_var_1, float_pair_gen_var_10, float_triple_gen,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;

#[rustfmt::skip]
const MATRIX: [[u8; 21]; 21] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#100
    [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#2
    [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#1
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0.0
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // NaN
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 0.0
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0], // 0x1.0#1
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0], // 0x1.0#2
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0], // 0x1.0#100
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
];

#[test]
fn test_eq() {
    for (sx, row) in ORDERED_FLOAT_HEX_STRINGS.iter().zip(MATRIX.iter()) {
        let x = parse_hex_string(sx);
        for (sy, &e) in ORDERED_FLOAT_HEX_STRINGS.iter().zip(row.iter()) {
            let y = parse_hex_string(sy);
            assert_eq!(u8::from(x == y), e);
            assert_eq!(
                u8::from(rug::Float::exact_from(&x) == rug::Float::exact_from(&y)),
                e
            );
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn eq_properties_helper(x: Float, y: Float) {
    let e = x == y;
    assert_eq!(y == x, e);
    assert_eq!(rug::Float::exact_from(&x) == rug::Float::exact_from(&y), e);
}

#[allow(clippy::cmp_owned)]
#[test]
fn eq_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        eq_properties_helper(x, y);
    });

    float_pair_gen_var_10().test_properties(|(x, y)| {
        eq_properties_helper(x, y);
    });

    float_gen().test_properties(|x| {
        if !x.is_nan() {
            assert_eq!(x, x);
        }
        assert_ne!(x, Float::NAN);
    });

    float_triple_gen().test_properties(|(x, y, z)| {
        if x == y && y == z {
            assert_eq!(x, z);
        }
    });

    primitive_float_pair_gen::<f64>().test_properties(|(x, y)| {
        assert_eq!(Float::from(x) == Float::from(y), x == y);
    });

    float_pair_gen_var_1().test_properties(|(x, y)| {
        assert_eq!(Rational::exact_from(&x) == Rational::exact_from(&y), x == y);
    });
}

fn read_hex_strings(strings: &[&str]) -> Vec<Float> {
    strings.iter().map(|s| parse_hex_string(s)).collect()
}

#[test]
fn test_comparable_float_eq() {
    let xs = read_hex_strings(&ORDERED_FLOAT_HEX_STRINGS);
    let ys = read_hex_strings(&ORDERED_FLOAT_HEX_STRINGS);
    let xs_2: Vec<String> = xs.iter().map(Float::to_string).collect();
    assert_eq!(xs_2, ORDERED_FLOAT_STRINGS);
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            assert_eq!(
                i == j,
                ComparableFloat(x.clone()) == ComparableFloat(y.clone())
            );
            assert_eq!(i == j, ComparableFloatRef(x) == ComparableFloatRef(y));
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn comparable_float_eq_properties_helper(x: Float, y: Float) {
    let rx = ComparableFloatRef(&x);
    let ry = ComparableFloatRef(&y);
    let e = rx == ry;
    assert_eq!(ry == rx, e);

    let x = ComparableFloat(x.clone());
    let y = ComparableFloat(y.clone());
    assert_eq!(x == y, e);
    assert_eq!(y == x, e);

    if e && !x.is_nan() {
        assert_eq!(x, y);
    }
}

#[test]
fn comparable_float_eq_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        comparable_float_eq_properties_helper(x, y);
    });

    float_pair_gen_var_10().test_properties(|(x, y)| {
        comparable_float_eq_properties_helper(x, y);
    });

    float_gen().test_properties(|x| {
        let x = ComparableFloat(x);
        assert_eq!(x, x);
    });

    float_triple_gen().test_properties(|(x, y, z)| {
        let x = ComparableFloat(x);
        let y = ComparableFloat(y);
        let z = ComparableFloat(z);
        if x == y && y == z {
            assert_eq!(x, z);
        }
    });

    primitive_float_pair_gen::<f64>().test_properties(|(x, y)| {
        assert_eq!(
            ComparableFloat(Float::from(x)) == ComparableFloat(Float::from(y)),
            NiceFloat(x) == NiceFloat(y)
        );
    });
}
