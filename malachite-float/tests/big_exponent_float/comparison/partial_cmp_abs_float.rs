// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, One, OneHalf, Zero,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_negate, big_exponent_float_power_of_2, big_exponent_float_to_float,
    float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::{big_exponent_float_pair_gen, float_pair_gen};
use malachite_nz::integer::Integer;
use std::cmp::Ordering;

#[test]
fn test_partial_cmp_abs_float() {
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big);
    let neg_huge = big_exponent_float_negate(huge.clone());
    let neg_tiny = big_exponent_float_negate(tiny.clone());
    let test = |x: BigExponentFloat, y: &str| {
        let y = parse_hex_string(y);
        let out = x.partial_cmp_abs(&float_to_big_exponent_float(&y));
        assert_eq!(x.partial_cmp_abs(&y), out);
        assert_eq!(y.partial_cmp_abs(&x), out.map(Ordering::reverse));
    };
    test(BigExponentFloat::NAN, "NaN");
    test(BigExponentFloat::ZERO, "0x0.0");
    test(BigExponentFloat::ZERO, "-0x0.0");
    test(BigExponentFloat::ONE, "0x1.0#1");
    test(BigExponentFloat::ONE, "0x1.0000000000000000000000000#100");
    test(BigExponentFloat::ONE, "-0x1.0#1");
    test(BigExponentFloat::ONE_HALF, "0x1.0#1");
    test(BigExponentFloat::INFINITY, "Infinity");
    test(BigExponentFloat::NEGATIVE_INFINITY, "Infinity");
    test(BigExponentFloat::NEGATIVE_INFINITY, "-Infinity");
    test(BigExponentFloat::NEGATIVE_ONE, "-Infinity");
    test(
        parse_big_exponent_float_hex("0x3.243f6a8885a30#53"),
        "0x3.243f6a8885a30#53",
    );
    test(
        parse_big_exponent_float_hex("0x3.243f6a8885a30#53"),
        "0x3.243f6a8885a308d4#64",
    );
    test(
        parse_big_exponent_float_hex("-0x3.243f6a8885a30#53"),
        "-0x3.243f6a8885a20#52",
    );
    test(huge.clone(), "0x4.0E+268435455#1");
    test(huge.clone(), "Infinity");
    test(neg_huge.clone(), "-0x4.0E+268435455#1");
    test(tiny.clone(), "0x1.0E-268435456#1");
    test(tiny.clone(), "0x0.0");
    test(neg_tiny.clone(), "-0x0.0");
    test(neg_tiny.clone(), "-0x1.0E-268435456#1");
}

#[test]
fn partial_cmp_abs_float_properties() {
    // - the comparison agrees with the one between `BigExponentFloat`s
    big_exponent_float_pair_gen().test_properties(|(x, y)| {
        if let Some(f) = big_exponent_float_to_float(&y) {
            assert_eq!(x.partial_cmp_abs(&f), x.partial_cmp_abs(&y));
            assert_eq!(f.partial_cmp_abs(&x), y.partial_cmp_abs(&x));
        }
    });

    // - agreement with `Float`
    float_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            float_to_big_exponent_float(&x).partial_cmp_abs(&y),
            x.partial_cmp_abs(&y)
        );
    });
}
