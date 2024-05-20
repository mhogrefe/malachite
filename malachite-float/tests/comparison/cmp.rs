// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::primitive_float_pair_gen;
use malachite_float::test_util::common::{parse_hex_string, ORDERED_FLOAT_HEX_STRINGS};
use malachite_float::test_util::generators::{
    float_gen, float_pair_gen, float_pair_gen_var_1, float_triple_gen,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

const fn encode(oo: Option<Ordering>) -> u8 {
    match oo {
        None => 9,
        Some(Less) => 0,
        Some(Equal) => 1,
        Some(Greater) => 2,
    }
}

#[rustfmt::skip]
const MATRIX: [[u8; 21]; 21] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 1, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 1, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 1, 1, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#100
    [2, 2, 2, 2, 1, 1, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#2
    [2, 2, 2, 2, 1, 1, 1, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0x1.0#1
    [2, 2, 2, 2, 2, 2, 2, 1, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 9, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0.0
    [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9], // NaN
    [2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 9, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 0.0
    [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0], // 0x1.0#1
    [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0], // 0x1.0#2
    [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0], // 0x1.0#100
    [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0],
    [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
];

#[test]
fn test_partial_cmp() {
    for (sx, row) in ORDERED_FLOAT_HEX_STRINGS.iter().zip(MATRIX.iter()) {
        let x = parse_hex_string(sx);
        for (sy, &e) in ORDERED_FLOAT_HEX_STRINGS.iter().zip(row.iter()) {
            let y = parse_hex_string(sy);
            assert_eq!(encode(x.partial_cmp(&y)), e);
            assert_eq!(
                encode(rug::Float::exact_from(&x).partial_cmp(&rug::Float::exact_from(&y))),
                e
            );
        }
    }
}

#[test]
fn partial_cmp_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        let ord = x.partial_cmp(&y);
        assert_eq!(y.partial_cmp(&x).map(Ordering::reverse), ord);
        assert_eq!(x == y, x.partial_cmp(&y) == Some(Equal));
        assert_eq!((-&y).partial_cmp(&-&x), ord);
        assert_eq!(
            rug::Float::exact_from(&x).partial_cmp(&rug::Float::exact_from(&y)),
            ord
        );
        if !x.is_zero() || !y.is_zero() {
            if let Some(ord) = ord {
                if ord != Equal {
                    assert_eq!(ComparableFloat(x).cmp(&ComparableFloat(y)), ord);
                }
            }
        }
    });

    float_gen().test_properties(|x| {
        if !x.is_nan() {
            assert_eq!(x.partial_cmp(&x), Some(Equal));
            assert!(x <= Float::INFINITY);
            assert!(x >= Float::NEGATIVE_INFINITY);
        }
    });

    float_triple_gen().test_properties(|(x, y, z)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });

    primitive_float_pair_gen::<f64>().test_properties(|(x, y)| {
        assert_eq!(
            Float::from(x).partial_cmp(&Float::from(y)),
            x.partial_cmp(&y)
        );
    });

    float_pair_gen_var_1().test_properties(|(x, y)| {
        assert_eq!(
            Some(Rational::exact_from(&x).cmp(&Rational::exact_from(&y))),
            x.partial_cmp(&y)
        );
    });
}

fn read_hex_strings(strings: &[&str]) -> Vec<Float> {
    strings.iter().map(|s| parse_hex_string(s)).collect()
}

#[test]
fn test_comparable_float_cmp() {
    let xs = read_hex_strings(&ORDERED_FLOAT_HEX_STRINGS);
    let ys = read_hex_strings(&ORDERED_FLOAT_HEX_STRINGS);
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            assert_eq!(
                i.cmp(&j),
                ComparableFloat(x.clone()).cmp(&ComparableFloat(y.clone()))
            );
            assert_eq!(i.cmp(&j), ComparableFloatRef(x).cmp(&ComparableFloatRef(y)));
        }
    }
}

#[test]
fn comparable_float_cmp_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        let cx = ComparableFloatRef(&x);
        let cy = ComparableFloatRef(&y);
        let ord = cx.cmp(&cy);
        assert_eq!(cy.cmp(&cx).reverse(), ord);
        assert_eq!(cx == cy, cx.cmp(&cy) == Equal);
        assert_eq!(
            ComparableFloat(x.clone()).cmp(&ComparableFloat(y.clone())),
            ord
        );
        assert_eq!((ComparableFloat(-y)).cmp(&ComparableFloat(-x)), ord);
    });

    float_gen().test_properties(|x| {
        let cx = ComparableFloatRef(&x);
        assert_eq!(cx.cmp(&cx), Equal);
        assert!(cx <= ComparableFloatRef(&Float::INFINITY));
        assert!(cx >= ComparableFloatRef(&Float::NEGATIVE_INFINITY));
    });

    float_triple_gen().test_properties(|(x, y, z)| {
        let cx = ComparableFloatRef(&x);
        let cy = ComparableFloatRef(&y);
        let cz = ComparableFloatRef(&z);
        if cx < cy && cy < cz {
            assert!(cx < cz);
        } else if cx > cy && cy > cz {
            assert!(cx > cz);
        }
    });

    primitive_float_pair_gen::<f64>().test_properties(|(x, y)| {
        assert_eq!(
            ComparableFloat(Float::from(x)).cmp(&ComparableFloat(Float::from(y))),
            NiceFloat(x).cmp(&NiceFloat(y))
        );
    });
}
