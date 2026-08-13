// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::basic::traits::{Infinity, NegativeInfinity};
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs, PartialOrdAbsDouble};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::generators::primitive_float_pair_gen;
use malachite_float::test_util::common::{ORDERED_FLOAT_HEX_STRINGS, parse_hex_string};
use malachite_float::test_util::generators::{
    float_gen, float_pair_gen, float_pair_gen_var_1, float_pair_gen_var_10, float_triple_gen,
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
    [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
    [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0],
    [0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0],
    [0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 9, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0], // -0x1.0#100
    [0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 9, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0], // -0x1.0#2
    [0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 9, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0], // -0x1.0#1
    [0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 9, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 9, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 9, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0.0
    [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9], // NaN
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 9, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 0.0
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 9, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 9, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 9, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0], // 0x1.0#1
    [0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 9, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0], // 0x1.0#2
    [0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 9, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0], // 0x1.0#100
    [0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0],
    [0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0],
    [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0],
    [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 9, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
];

#[test]
fn test_partial_cmp_abs() {
    for (sx, row) in ORDERED_FLOAT_HEX_STRINGS.iter().zip(MATRIX.iter()) {
        let x = parse_hex_string(sx);
        for (sy, &e) in ORDERED_FLOAT_HEX_STRINGS.iter().zip(row.iter()) {
            let y = parse_hex_string(sy);
            assert_eq!(encode(x.partial_cmp_abs(&y)), e);
            assert_eq!(
                encode(rug::Float::exact_from(&x).cmp_abs(&rug::Float::exact_from(&y))),
                e
            );
        }
    }
}

fn partial_cmp_abs_properties_helper(x: Float, y: Float) {
    let ord = x.partial_cmp_abs(&y);
    assert_eq!(y.partial_cmp_abs(&x).map(Ordering::reverse), ord);
    assert_eq!(
        (&x).abs() == (&y).abs(),
        x.partial_cmp_abs(&y) == Some(Equal)
    );
    assert_eq!((-&x).partial_cmp_abs(&y), ord);
    assert_eq!(x.partial_cmp_abs(&-&y), ord);
    assert_eq!((-&x).partial_cmp_abs(&-&y), ord);
    assert_eq!(
        rug::Float::exact_from(&x).cmp_abs(&rug::Float::exact_from(&y)),
        ord
    );
    if (!x.is_zero() || !y.is_zero())
        && let Some(ord) = ord
        && ord != Equal
    {
        assert_eq!(ComparableFloat(x).cmp_abs(&ComparableFloat(y)), ord);
    }
}

#[test]
fn partial_cmp_abs_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        partial_cmp_abs_properties_helper(x, y);
    });

    float_pair_gen_var_10().test_properties(|(x, y)| {
        partial_cmp_abs_properties_helper(x, y);
    });

    float_gen().test_properties(|x| {
        if !x.is_nan() {
            assert_eq!(x.partial_cmp_abs(&x), Some(Equal));
            assert_eq!(x.partial_cmp_abs(&-&x), Some(Equal));
            assert_eq!((-&x).partial_cmp_abs(&x), Some(Equal));
            assert_eq!((-&x).partial_cmp_abs(&-&x), Some(Equal));
            assert!(x.le_abs(&Float::INFINITY));
            assert!(x.le_abs(&Float::NEGATIVE_INFINITY));
        }
    });

    float_triple_gen().test_properties(|(x, y, z)| {
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x.lt_abs(&z));
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x.gt_abs(&z));
        }
    });

    primitive_float_pair_gen::<f64>().test_properties(|(x, y)| {
        assert_eq!(
            Float::from(x).partial_cmp_abs(&Float::from(y)),
            x.abs().partial_cmp(&y.abs())
        );
    });

    float_pair_gen_var_1().test_properties(|(x, y)| {
        assert_eq!(
            Some(Rational::exact_from(&x).cmp_abs(&Rational::exact_from(&y))),
            x.partial_cmp_abs(&y)
        );
    });
}

#[rustfmt::skip]
const MATRIX_2: [[u8; 21]; 21] = [
    [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
    [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0],
    [0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0],
    [0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0],
    [0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0], // -0x1.0#100
    [0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0, 0], // -0x1.0#2
    [0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0, 0, 0], // -0x1.0#1
    [0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // -0.0
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // NaN
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0], // 0.0
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0, 0, 0], // 0x1.0#1
    [0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0, 0], // 0x1.0#2
    [0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0], // 0x1.0#100
    [0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0],
    [0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0],
    [0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0],
    [1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1],
];

#[test]
fn test_comparable_float_cmp_abs() {
    for (sx, row) in ORDERED_FLOAT_HEX_STRINGS.iter().zip(MATRIX_2.iter()) {
        let x = parse_hex_string(sx);
        for (sy, &e) in ORDERED_FLOAT_HEX_STRINGS.iter().zip(row.iter()) {
            assert_ne!(e, 9);
            let y = parse_hex_string(sy);
            assert_eq!(
                encode(Some(
                    ComparableFloat(x.clone()).cmp_abs(&ComparableFloat(y.clone()))
                )),
                e
            );
            assert_eq!(
                encode(Some(
                    ComparableFloatRef(&x).cmp_abs(&ComparableFloatRef(&y))
                )),
                e
            );
        }
    }
}

// The antisymmetry assertion below is the property under test; swapping its operands would restate
// the line above it rather than assert anything.
#[cfg_attr(dylint_lib = "malachite_lints", expect(redundant_cmp_reverse))]
fn comparable_float_cmp_abs_properties_helper(x: Float, y: Float) {
    let cx = ComparableFloatRef(&x);
    let cy = ComparableFloatRef(&y);
    let ord = cx.cmp_abs(&cy);
    assert_eq!(cy.cmp_abs(&cx).reverse(), ord);
    assert_eq!(
        ComparableFloat((&x).abs()) == ComparableFloat((&y).abs()),
        cx.cmp_abs(&cy) == Equal
    );
    assert_eq!(
        ComparableFloat(x.clone()).cmp_abs(&ComparableFloat(y.clone())),
        ord
    );
    assert_eq!(
        (ComparableFloatRef(&x)).cmp_abs(&ComparableFloatRef(&-&y)),
        ord
    );
    assert_eq!(
        (ComparableFloatRef(&-&x)).cmp_abs(&ComparableFloatRef(&y)),
        ord
    );
    assert_eq!((ComparableFloat(-x)).cmp_abs(&ComparableFloat(-y)), ord);
}

#[test]
fn comparable_float_cmp_abs_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        comparable_float_cmp_abs_properties_helper(x, y);
    });

    float_pair_gen_var_10().test_properties(|(x, y)| {
        comparable_float_cmp_abs_properties_helper(x, y);
    });

    float_gen().test_properties(|x| {
        let cx = ComparableFloatRef(&x);
        assert_eq!(cx.cmp_abs(&cx), Equal);
        assert_eq!(cx.cmp_abs(&ComparableFloatRef(&-&x)), Equal);
        assert_eq!(ComparableFloatRef(&-&x).cmp_abs(&cx), Equal);
        assert_eq!(
            ComparableFloatRef(&-&x).cmp_abs(&ComparableFloatRef(&-&x)),
            Equal
        );
        assert!(cx.le_abs(&ComparableFloatRef(&Float::INFINITY)));
        assert!(cx.le_abs(&ComparableFloatRef(&Float::NEGATIVE_INFINITY)));
    });

    float_triple_gen().test_properties(|(x, y, z)| {
        let cx = ComparableFloatRef(&x);
        let cy = ComparableFloatRef(&y);
        let cz = ComparableFloatRef(&z);
        if cx.lt_abs(&cy) && cy.lt_abs(&cz) {
            assert!(cx.lt_abs(&cz));
        } else if cx.gt_abs(&cy) && cy.gt_abs(&cz) {
            assert!(cx.gt_abs(&cz));
        }
    });

    primitive_float_pair_gen::<f64>().test_properties(|(x, y)| {
        assert_eq!(
            ComparableFloat(Float::from(x)).cmp_abs(&ComparableFloat(Float::from(y))),
            NiceFloat(x.abs()).cmp(&NiceFloat(y.abs()))
        );
    });
}

#[test]
fn test_partial_cmp_abs_double() {
    let test = |s, t, out: Option<Ordering>| {
        let x = parse_hex_string(s);
        let y = parse_hex_string(t);
        assert_eq!(x.partial_cmp_abs_double(&y), out);
    };
    test("NaN", "NaN", None);
    test("NaN", "0x1.0#1", None);
    test("0x1.0#1", "NaN", None);
    // - doubling leaves infinities and zeros alone
    test("Infinity", "Infinity", Some(Equal));
    test("0x0.0", "0x0.0", Some(Equal));
    test("-0x0.0", "0x0.0", Some(Equal));
    test("Infinity", "0x1.0#1", Some(Greater));
    test("0x1.0#1", "Infinity", Some(Less));
    test("0x1.0#1", "0x0.0", Some(Greater));
    test("0x0.0", "0x1.0#1", Some(Less));
    // - ordinary values
    test("0x1.0#1", "0x0.8#1", Some(Equal));
    test("0x0.8#1", "0x1.0#1", Some(Less));
    test("0x4.0#1", "0x1.0#1", Some(Greater));
    // - only magnitudes matter
    test("-0x1.0#1", "0x0.8#1", Some(Equal));
    test("0x1.0#1", "-0x0.8#1", Some(Equal));
    // - precision does not affect the value comparison
    test("0x1.000#10", "0x0.800#10", Some(Equal));
}

#[test]
fn test_partial_cmp_abs_double_overflow() {
    // Doubling the largest finite value overflows to an infinity, so forming it first would make an
    // infinite `self` compare equal rather than greater.
    let max = Float::max_finite_value_with_prec(10);
    assert_eq!(Float::INFINITY.partial_cmp_abs_double(&max), Some(Greater));
    assert_eq!((&max << 1u64), Float::INFINITY);
    assert_eq!(
        Float::INFINITY.partial_cmp_abs(&(&max << 1u64)),
        Some(Equal)
    );
    // a finite value is still less than the unrepresentable doubled one
    assert_eq!(max.partial_cmp_abs_double(&max), Some(Less));
}

#[test]
fn partial_cmp_abs_double_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        let c = x.partial_cmp_abs_double(&y);
        // sign changes are invisible
        assert_eq!((-&x).partial_cmp_abs_double(&y), c);
        assert_eq!(x.partial_cmp_abs_double(&-&y), c);
        if x.is_nan() || y.is_nan() {
            assert_eq!(c, None);
            return;
        }
        // the doubling form agrees except where it overflows, which is the point of this
        let doubled = &y << 1u64;
        if doubled.is_infinite() && y.is_finite() {
            // the true doubled value is past the range, so it exceeds every finite value
            assert_eq!(c, Some(if x.is_infinite() { Greater } else { Less }));
        } else {
            assert_eq!(c, x.partial_cmp_abs(&doubled));
        }
    });
}
