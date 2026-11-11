// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero};
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, big_exponent_float_to_float, float_to_big_exponent_float,
    parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1, float_gen,
};
use malachite_nz::integer::Integer;
use std::num::FpCategory;
use std::str::FromStr;

// (hex, is_nan, is_finite, is_infinite, is_zero, is_normal, is_sign_positive, is_sign_negative)
const CASES: [(&str, [bool; 7]); 9] = [
    ("NaN", [true, false, false, false, false, false, false]),
    ("Infinity", [false, false, true, false, false, true, false]),
    ("-Infinity", [false, false, true, false, false, false, true]),
    ("0x0.0", [false, true, false, true, false, true, false]),
    ("0x1.0#1", [false, true, false, false, true, true, false]),
    ("-0x1.0#1", [false, true, false, false, true, false, true]),
    (
        "0x0.55555555555554#53",
        [false, true, false, false, true, true, false],
    ),
    (
        "-0x3.243f6a8885a30#53",
        [false, true, false, false, true, false, true],
    ),
    (
        "0x4.0E+268435455#1",
        [false, true, false, false, true, true, false],
    ),
];

const fn predicates(x: &BigExponentFloat) -> [bool; 7] {
    [
        x.is_nan(),
        x.is_finite(),
        x.is_infinite(),
        x.is_zero(),
        x.is_normal(),
        x.is_sign_positive(),
        x.is_sign_negative(),
    ]
}

#[test]
fn test_classification() {
    for (s, out) in CASES {
        let x = parse_big_exponent_float_hex(s);
        assert_eq!(predicates(&x), out, "{s}");
    }
    // - an exponent far outside `Float`'s range
    for e in ["1000000000000000000000", "-1000000000000000000000"] {
        let x = big_exponent_float_power_of_2(Integer::from_str(e).unwrap());
        assert_eq!(
            predicates(&x),
            [false, true, false, false, true, true, false]
        );
        assert_eq!(x.classify(), FpCategory::Normal);
    }
    assert_eq!(BigExponentFloat::NAN.classify(), FpCategory::Nan);
    assert_eq!(BigExponentFloat::INFINITY.classify(), FpCategory::Infinite);
    assert_eq!(
        BigExponentFloat::NEGATIVE_INFINITY.classify(),
        FpCategory::Infinite
    );
    assert_eq!(BigExponentFloat::ZERO.classify(), FpCategory::Zero);
    assert_eq!(BigExponentFloat::ONE.classify(), FpCategory::Normal);
    assert_eq!(
        BigExponentFloat::NEGATIVE_ONE.classify(),
        FpCategory::Normal
    );
}

#[test]
fn test_into_and_to_finite_and_non_nan() {
    let test = |s, non_nan: &str, finite: &str| {
        let x = parse_big_exponent_float_hex(s);
        assert_eq!(format!("{:?}", x.to_non_nan()), non_nan);
        assert_eq!(format!("{:?}", x.clone().into_non_nan()), non_nan);
        assert_eq!(format!("{:?}", x.to_finite()), finite);
        assert_eq!(format!("{:?}", x.into_finite()), finite);
    };
    test("NaN", "None", "None");
    test("Infinity", "Some(Infinity)", "None");
    test("-Infinity", "Some(-Infinity)", "None");
    test("0x0.0", "Some(0x0.0)", "Some(0x0.0)");
    test("-0x1.8#2", "Some(-0x1.8)", "Some(-0x1.8)");
}

#[test]
fn classification_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let [nan, finite, infinite, zero, normal, positive, negative] = predicates(&x);
        assert_eq!(nan, !finite && !infinite);
        assert!(!(finite && infinite));
        assert_eq!(normal, finite && !zero);
        assert!(!(positive && negative));
        assert_eq!(nan, !positive && !negative);
        assert_eq!(
            x.classify(),
            if nan {
                FpCategory::Nan
            } else if infinite {
                FpCategory::Infinite
            } else if zero {
                FpCategory::Zero
            } else {
                FpCategory::Normal
            }
        );
        assert_eq!(x.to_non_nan().is_some(), !nan);
        assert_eq!(x.to_finite().is_some(), finite);
        assert_eq!(
            format!("{:?}", x.to_non_nan()),
            format!("{:?}", x.clone().into_non_nan())
        );
        assert_eq!(
            format!("{:?}", x.to_finite()),
            format!("{:?}", x.clone().into_finite())
        );
    });

    // - agreement with `Float`
    big_exponent_float_gen_var_1().test_properties(|x| {
        let f = big_exponent_float_to_float(&x).unwrap();
        assert_eq!(
            predicates(&x),
            [
                f.is_nan(),
                f.is_finite(),
                f.is_infinite(),
                f.is_zero(),
                f.is_normal(),
                f.is_sign_positive(),
                f.is_sign_negative(),
            ]
        );
        assert_eq!(x.classify(), f.classify());
    });

    // - agreement with `Float`, except for the sign of negative zero, which is lost
    float_gen().test_properties(|f| {
        let x = float_to_big_exponent_float(&f);
        assert!(x.is_valid());
        assert_eq!(x.is_nan(), f.is_nan());
        assert_eq!(x.is_finite(), f.is_finite());
        assert_eq!(x.is_zero(), f.is_zero());
        assert_eq!(x.is_normal(), f.is_normal());
        if !f.is_zero() {
            assert_eq!(x.is_sign_negative(), f.is_sign_negative());
        }
        assert_eq!(x.classify(), f.classify());
    });
}
