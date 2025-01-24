// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::comparison::traits::{Max, Min};
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, OneHalf, Two, Zero,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_gen_var_11;
use malachite_float::test_util::common::to_hex_string;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use rug::float::Special;

#[test]
fn test_min() {
    let min = Float::MIN;
    assert!(min.is_valid());
    assert_eq!(min, f64::NEGATIVE_INFINITY);
    assert_eq!(min.to_string(), "-Infinity");
    assert_eq!(
        Float::from(&rug::Float::with_val(1, Special::NegInfinity)),
        min
    );
}

#[test]
fn test_max() {
    let max = Float::MAX;
    assert!(max.is_valid());
    assert_eq!(max, f64::INFINITY);
    assert_eq!(max.to_string(), "Infinity");
    assert_eq!(
        Float::from(&rug::Float::with_val(1, Special::Infinity)),
        max
    );
}

#[test]
fn test_zero() {
    let zero = Float::ZERO;
    assert!(zero.is_valid());
    assert_eq!(zero, 0);
    assert_eq!(zero, 0.0);
    assert!(zero.is_positive_zero());
    assert!(zero.is_zero());
    assert_eq!(zero.to_string(), "0.0");
    assert_eq!(
        ComparableFloat(Float::from(&rug::Float::with_val(1, Special::Zero))),
        ComparableFloat(zero)
    );
}

#[test]
fn test_min_positive_value_prec() {
    let min_positive = Float::MIN_POSITIVE;
    assert!(min_positive.is_valid());
    assert_eq!(min_positive.to_string(), "too_small");
    assert_eq!(
        format!("{:#x}", ComparableFloatRef(&min_positive)),
        "0x1.0E-268435456#1"
    );
    assert_eq!(min_positive.get_prec(), Some(1));
    assert_eq!(
        ComparableFloatRef(&Float::from(
            &(rug::Float::with_val(1, 1.0) << (Float::MIN_EXPONENT - 1))
        )),
        ComparableFloatRef(&min_positive)
    );
    assert_eq!(
        ComparableFloat(Float::min_positive_value_prec(1)),
        ComparableFloat(min_positive)
    );

    let test = |p, out, out_hex| {
        let min_positive = Float::min_positive_value_prec(p);
        assert!(min_positive.is_valid());
        assert_eq!(min_positive.to_string(), out);
        assert_eq!(to_hex_string(&min_positive), out_hex);
        assert_eq!(
            ComparableFloat(Float::from(
                &(rug::Float::with_val(u32::exact_from(p), 1.0) << (Float::MIN_EXPONENT - 1))
            )),
            ComparableFloat(min_positive)
        );
    };
    test(1, "too_small", "0x1.0E-268435456#1");
    test(2, "too_small", "0x1.0E-268435456#2");
    test(3, "too_small", "0x1.0E-268435456#3");
    test(10, "too_small", "0x1.000E-268435456#10");
    test(
        100,
        "too_small",
        "0x1.0000000000000000000000000E-268435456#100",
    );
}

#[test]
#[should_panic]
fn min_positive_value_prec_fail() {
    Float::min_positive_value_prec(0);
}

#[test]
fn test_max_finite_value_with_prec() {
    let test = |p, out, out_hex| {
        let max_finite = Float::max_finite_value_with_prec(p);
        assert!(max_finite.is_valid());
        assert_eq!(max_finite.to_string(), out);
        assert_eq!(to_hex_string(&max_finite), out_hex);
    };
    test(1, "too_big", "0x4.0E+268435455#1");
    test(2, "too_big", "0x6.0E+268435455#2");
    test(3, "too_big", "0x7.0E+268435455#3");
    test(10, "too_big", "0x7.feE+268435455#10");
    test(
        100,
        "too_big",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
    );
}

#[test]
#[should_panic]
fn max_finite_value_with_prec_fail() {
    Float::max_finite_value_with_prec(0);
}

#[test]
fn test_one_prec() {
    let one = Float::ONE;
    assert!(one.is_valid());
    assert_eq!(one, 1);
    assert_eq!(one, 1.0);
    assert_eq!(one.to_string(), "1.0");
    assert_eq!(one.get_prec(), Some(1));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug::Float::with_val(1, 1.0))),
        ComparableFloatRef(&one)
    );
    assert_eq!(ComparableFloat(Float::one_prec(1)), ComparableFloat(one));

    let test = |p, out, out_hex| {
        let one = Float::one_prec(p);
        assert!(one.is_valid());
        assert_eq!(one.to_string(), out);
        assert_eq!(to_hex_string(&one), out_hex);
        assert_eq!(
            ComparableFloat(Float::from(&rug::Float::with_val(u32::exact_from(p), 1.0))),
            ComparableFloat(one)
        );
    };
    test(1, "1.0", "0x1.0#1");
    test(2, "1.0", "0x1.0#2");
    test(3, "1.0", "0x1.0#3");
    test(10, "1.0", "0x1.000#10");
    test(100, "1.0", "0x1.0000000000000000000000000#100");
}

#[test]
#[should_panic]
fn one_prec_fail() {
    Float::one_prec(0);
}

#[test]
fn one_prec_properties() {
    unsigned_gen_var_11().test_properties(|p| {
        let one = Float::one_prec(p);
        assert!(one.is_valid());
        assert_eq!(one, 1);
        assert_eq!(one, 1.0);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug::Float::with_val(u32::exact_from(p), 1.0))),
            ComparableFloatRef(&one)
        );

        assert_eq!(one.get_prec(), Some(p));
    });
}

#[test]
fn test_two_prec() {
    let two = Float::TWO;
    assert!(two.is_valid());
    assert_eq!(two, 2);
    assert_eq!(two, 2.0);
    assert_eq!(two.to_string(), "2.0");
    assert_eq!(two.get_prec(), Some(1));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug::Float::with_val(1, 2.0))),
        ComparableFloatRef(&two)
    );
    assert_eq!(ComparableFloat(Float::two_prec(1)), ComparableFloat(two));

    let test = |p, out, out_hex| {
        let two = Float::two_prec(p);
        assert!(two.is_valid());
        assert_eq!(two.to_string(), out);
        assert_eq!(to_hex_string(&two), out_hex);
        assert_eq!(
            ComparableFloat(Float::from(&rug::Float::with_val(u32::exact_from(p), 2.0))),
            ComparableFloat(two)
        );
    };
    test(1, "2.0", "0x2.0#1");
    test(2, "2.0", "0x2.0#2");
    test(3, "2.0", "0x2.0#3");
    test(10, "2.0", "0x2.00#10");
    test(100, "2.0", "0x2.0000000000000000000000000#100");
}

#[test]
#[should_panic]
fn two_prec_fail() {
    Float::two_prec(0);
}

#[test]
fn two_prec_properties() {
    unsigned_gen_var_11().test_properties(|p| {
        let two = Float::two_prec(p);
        assert!(two.is_valid());
        assert_eq!(two, 2);
        assert_eq!(two, 2.0);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug::Float::with_val(u32::exact_from(p), 2.0))),
            ComparableFloatRef(&two)
        );

        assert_eq!(two.get_prec(), Some(p));
    });
}

#[test]
fn test_negative_one_prec() {
    let negative_one = Float::NEGATIVE_ONE;
    assert!(negative_one.is_valid());
    assert_eq!(negative_one, -1);
    assert_eq!(negative_one, -1.0);
    assert_eq!(negative_one.to_string(), "-1.0");
    assert_eq!(negative_one.get_prec(), Some(1));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug::Float::with_val(1, -1.0))),
        ComparableFloatRef(&negative_one)
    );
    assert_eq!(
        ComparableFloat(Float::negative_one_prec(1)),
        ComparableFloat(negative_one)
    );

    let test = |p, out, out_hex| {
        let negative_one = Float::negative_one_prec(p);
        assert!(negative_one.is_valid());
        assert_eq!(negative_one.to_string(), out);
        assert_eq!(to_hex_string(&negative_one), out_hex);
        assert_eq!(
            ComparableFloat(Float::from(&rug::Float::with_val(u32::exact_from(p), -1.0))),
            ComparableFloat(negative_one)
        );
    };
    test(1, "-1.0", "-0x1.0#1");
    test(2, "-1.0", "-0x1.0#2");
    test(3, "-1.0", "-0x1.0#3");
    test(10, "-1.0", "-0x1.000#10");
    test(100, "-1.0", "-0x1.0000000000000000000000000#100");
}

#[test]
#[should_panic]
fn negative_one_prec_fail() {
    Float::negative_one_prec(0);
}

#[test]
fn negative_one_prec_properties() {
    unsigned_gen_var_11().test_properties(|p| {
        let negative_one = Float::negative_one_prec(p);
        assert!(negative_one.is_valid());
        assert_eq!(negative_one, -1);
        assert_eq!(negative_one, -1.0);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug::Float::with_val(
                u32::exact_from(p),
                -1.0
            ))),
            ComparableFloatRef(&negative_one)
        );

        assert_eq!(negative_one.get_prec(), Some(p));
    });
}

#[test]
fn test_one_half_prec() {
    let one_half = Float::ONE_HALF;
    assert!(one_half.is_valid());
    assert_eq!(one_half, 0.5);
    assert_eq!(one_half.to_string(), "0.5");
    assert_eq!(one_half.get_prec(), Some(1));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug::Float::with_val(1, 0.5))),
        ComparableFloatRef(&one_half)
    );
    assert_eq!(
        ComparableFloat(Float::one_half_prec(1)),
        ComparableFloat(one_half)
    );

    let test = |p, out, out_hex| {
        let one_half = Float::one_half_prec(p);
        assert!(one_half.is_valid());
        assert_eq!(one_half.to_string(), out);
        assert_eq!(to_hex_string(&one_half), out_hex);
        assert_eq!(
            ComparableFloat(Float::from(&rug::Float::with_val(u32::exact_from(p), 0.5))),
            ComparableFloat(one_half)
        );
    };
    test(1, "0.5", "0x0.8#1");
    test(2, "0.5", "0x0.8#2");
    test(3, "0.5", "0x0.8#3");
    test(10, "0.5", "0x0.800#10");
    test(100, "0.5", "0x0.8000000000000000000000000#100");
}

#[test]
#[should_panic]
fn one_half_prec_fail() {
    Float::one_half_prec(0);
}

#[test]
fn one_half_prec_properties() {
    unsigned_gen_var_11().test_properties(|p| {
        let one_half = Float::one_half_prec(p);
        assert!(one_half.is_valid());
        assert_eq!(one_half, 0.5);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug::Float::with_val(u32::exact_from(p), 0.5))),
            ComparableFloatRef(&one_half)
        );

        assert_eq!(one_half.get_prec(), Some(p));
    });
}

#[test]
fn test_negative_zero() {
    let negative_zero = Float::NEGATIVE_ZERO;
    assert!(negative_zero.is_valid());
    assert_eq!(negative_zero, 0);
    assert!(negative_zero.is_negative_zero());
    assert!(negative_zero.is_zero());
    assert_eq!(negative_zero.to_string(), "-0.0");
    assert_eq!(
        ComparableFloat(Float::from(&rug::Float::with_val(1, Special::NegZero))),
        ComparableFloat(negative_zero)
    );
}

#[test]
fn test_nan() {
    let nan = Float::NAN;
    assert!(nan.is_valid());
    assert!(nan.is_nan());
    assert_eq!(nan.to_string(), "NaN");
    assert_eq!(
        ComparableFloat(Float::from(&rug::Float::with_val(1, Special::Nan))),
        ComparableFloat(nan)
    );
}

#[test]
fn test_infinity() {
    let infinity = Float::INFINITY;
    assert!(infinity.is_valid());
    assert_eq!(infinity, f64::INFINITY);
    assert_eq!(infinity.to_string(), "Infinity");
    assert_eq!(
        Float::from(&rug::Float::with_val(1, Special::Infinity)),
        infinity
    );
}

#[test]
fn test_negative_infinity() {
    let negative_infinity = Float::NEGATIVE_INFINITY;
    assert!(negative_infinity.is_valid());
    assert_eq!(negative_infinity, f64::NEGATIVE_INFINITY);
    assert_eq!(negative_infinity.to_string(), "-Infinity");
    assert_eq!(
        Float::from(&rug::Float::with_val(1, Special::NegInfinity)),
        negative_infinity
    );
}

#[test]
fn test_default() {
    let d = Float::default();
    assert!(d.is_valid());
    assert!(d.is_nan());
    assert_eq!(d.to_string(), "NaN");
}
