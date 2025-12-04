// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::basic::traits::NaN;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::primitive_float_pair_gen;
use malachite_float::test_util::common::{ORDERED_FLOAT_HEX_STRINGS, parse_hex_string};
use malachite_float::test_util::generators::{
    float_gen, float_pair_gen, float_pair_gen_var_1, float_pair_gen_var_10, float_triple_gen,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;

#[rustfmt::skip]
const MATRIX: [[u8; 21]; 21] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0], // -0x1.0#100
    [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0], // -0x1.0#2
    [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0], // -0x1.0#1
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0.0
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // NaN
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 0.0
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0], // 0x1.0#1
    [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0], // 0x1.0#2
    [0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0], // 0x1.0#100
    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
];

#[test]
fn test_eq_abs() {
    for (sx, row) in ORDERED_FLOAT_HEX_STRINGS.iter().zip(MATRIX.iter()) {
        let x = parse_hex_string(sx);
        for (sy, &e) in ORDERED_FLOAT_HEX_STRINGS.iter().zip(row.iter()) {
            let y = parse_hex_string(sy);
            assert_eq!(u8::from(x.eq_abs(&y)), e);
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn eq_abs_properties_helper(x: Float, y: Float) {
    let e = x.eq_abs(&y);
    assert_eq!((&x).abs() == (&y).abs(), e);
    assert_eq!(x.ne_abs(&y), !e);
    assert_eq!(y.eq_abs(&x), e);
}

#[allow(clippy::cmp_owned)]
#[test]
fn eq_abs_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        eq_abs_properties_helper(x, y);
    });

    float_pair_gen_var_10().test_properties(|(x, y)| {
        eq_abs_properties_helper(x, y);
    });

    float_gen().test_properties(|x| {
        if !x.is_nan() {
            assert!(x.eq_abs(&x));
        }
        assert!(x.ne_abs(&Float::NAN));
    });

    float_triple_gen().test_properties(|(x, y, z)| {
        if x.eq_abs(&y) && y.eq_abs(&z) {
            assert!(x.eq_abs(&z));
        }
    });

    primitive_float_pair_gen::<f64>().test_properties(|(x, y)| {
        assert_eq!(Float::from(x).eq_abs(&Float::from(y)), x.eq_abs(&y));
    });

    float_pair_gen_var_1().test_properties(|(x, y)| {
        assert_eq!(
            Rational::exact_from(&x).eq_abs(&Rational::exact_from(&y)),
            x.eq_abs(&y)
        );
    });
}

#[rustfmt::skip]
const ORDERED_FLOAT_MATRIX: [[u8; 21]; 21] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0], // -0x1.0#100
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0], // -0x1.0#2
    [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0], // -0x1.0#1
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0.0
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // NaN
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 0.0
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0], // 0x1.0#1
    [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0], // 0x1.0#2
    [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0], // 0x1.0#100
    [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
];

#[test]
fn test_comparable_float_eq_abs() {
    for (sx, row) in ORDERED_FLOAT_HEX_STRINGS
        .iter()
        .zip(ORDERED_FLOAT_MATRIX.iter())
    {
        let x = parse_hex_string(sx);
        for (sy, &e) in ORDERED_FLOAT_HEX_STRINGS.iter().zip(row.iter()) {
            let y = parse_hex_string(sy);
            assert_eq!(
                u8::from(ComparableFloat(x.clone()).eq_abs(&ComparableFloat(y.clone()))),
                e
            );
            assert_eq!(
                u8::from(ComparableFloatRef(&x).eq_abs(&ComparableFloatRef(&y))),
                e
            );
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn comparable_float_eq_abs_properties_helper(x: Float, y: Float) {
    let rx = ComparableFloatRef(&x);
    let ry = ComparableFloatRef(&y);
    let e = rx.eq_abs(&ry);
    assert_eq!(
        ComparableFloatRef(&(&x).abs()) == ComparableFloatRef(&(&y).abs()),
        e
    );
    assert_eq!(rx.ne_abs(&ry), !e);
    assert_eq!(ry.eq_abs(&rx), e);

    let x = ComparableFloat(x.clone());
    let y = ComparableFloat(y.clone());
    assert_eq!(
        ComparableFloat((&x.0).abs()) == ComparableFloat((&y.0).abs()),
        e
    );
    assert_eq!(x.eq_abs(&y), e);
    assert_eq!(x.ne_abs(&y), !e);
    assert_eq!(y.eq_abs(&x), e);

    if e && !x.is_nan() {
        assert!(x.eq_abs(&y));
    }
}

#[test]
fn comparable_float_eq_abs_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        comparable_float_eq_abs_properties_helper(x, y);
    });

    float_pair_gen_var_10().test_properties(|(x, y)| {
        comparable_float_eq_abs_properties_helper(x, y);
    });

    float_gen().test_properties(|x| {
        let x = ComparableFloat(x);
        assert!(x.eq_abs(&x));
    });

    float_triple_gen().test_properties(|(x, y, z)| {
        let x = ComparableFloat(x);
        let y = ComparableFloat(y);
        let z = ComparableFloat(z);
        if x.eq_abs(&y) && y.eq_abs(&z) {
            assert!(x.eq_abs(&z));
        }
    });

    primitive_float_pair_gen::<f64>().test_properties(|(x, y)| {
        assert_eq!(
            ComparableFloat(Float::from(x)).eq_abs(&ComparableFloat(Float::from(y))),
            NiceFloat(x).eq_abs(&NiceFloat(y))
        );
    });
}
