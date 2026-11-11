// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, OneHalf, Zero};
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_negate, big_exponent_float_power_of_2, float_to_big_exponent_float,
    parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_integer_pair_gen, float_integer_pair_gen,
};
use malachite_nz::integer::Integer;
use std::cmp::Ordering::{self, *};

#[test]
fn test_partial_cmp_integer() {
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big);
    let neg_huge = big_exponent_float_negate(huge.clone());
    let neg_tiny = big_exponent_float_negate(tiny.clone());
    let test = |x: BigExponentFloat, y: &str, out: Option<Ordering>| {
        let y = Integer::from_str(y).unwrap();
        assert_eq!(x.partial_cmp(&y), out);
        assert_eq!(y.partial_cmp(&x), out.map(Ordering::reverse));
        assert_eq!(x == y, out == Some(Equal));
        assert_eq!(y == x, out == Some(Equal));
    };
    test(BigExponentFloat::NAN, "0", None);
    test(BigExponentFloat::INFINITY, "0", Some(Greater));
    test(BigExponentFloat::NEGATIVE_INFINITY, "0", Some(Less));
    test(BigExponentFloat::ZERO, "0", Some(Equal));
    test(BigExponentFloat::ZERO, "1", Some(Less));
    test(BigExponentFloat::ONE, "1", Some(Equal));
    test(BigExponentFloat::ONE_HALF, "1", Some(Less));
    test(BigExponentFloat::ONE, "0", Some(Greater));
    test(
        parse_big_exponent_float_hex("0x3.243f6a8885a30#53"),
        "3",
        Some(Greater),
    );
    test(
        parse_big_exponent_float_hex("0x3.243f6a8885a30#53"),
        "4",
        Some(Less),
    );
    test(parse_big_exponent_float_hex("0xff.0#8"), "255", Some(Equal));
    test(
        parse_big_exponent_float_hex("0xff.8#9"),
        "255",
        Some(Greater),
    );
    test(
        parse_big_exponent_float_hex("0x1.0E+40#1"),
        "1461501637330902918203684832716283019655932542976",
        Some(Equal),
    );
    test(
        parse_big_exponent_float_hex("0x1.0E+40#1"),
        "1461501637330902918203684832716283019655932542977",
        Some(Less),
    );
    test(huge.clone(), "0", Some(Greater));
    test(
        huge.clone(),
        "1461501637330902918203684832716283019655932542976",
        Some(Greater),
    );
    test(tiny.clone(), "0", Some(Greater));
    test(tiny.clone(), "1", Some(Less));
    test(neg_huge.clone(), "0", Some(Less));
    test(neg_tiny.clone(), "0", Some(Less));
    test(BigExponentFloat::NEGATIVE_INFINITY, "-1000", Some(Less));
    test(BigExponentFloat::NAN, "-1", None);
    test(BigExponentFloat::ZERO, "-1", Some(Greater));
    test(
        parse_big_exponent_float_hex("-0x3.243f6a8885a30#53"),
        "-3",
        Some(Less),
    );
    test(
        parse_big_exponent_float_hex("-0x3.243f6a8885a30#53"),
        "-4",
        Some(Greater),
    );
    test(
        parse_big_exponent_float_hex("-0xff.0#8"),
        "-255",
        Some(Equal),
    );
    test(huge.clone(), "-1", Some(Greater));
    test(
        neg_huge.clone(),
        "-1000000000000000000000000000000",
        Some(Less),
    );
    test(neg_tiny.clone(), "-1", Some(Greater));
    test(tiny.clone(), "-1", Some(Greater));
}

#[test]
fn partial_cmp_integer_properties() {
    big_exponent_float_integer_pair_gen().test_properties(|(x, y)| {
        let ord = x.partial_cmp(&y);
        assert_eq!(y.partial_cmp(&x).map(Ordering::reverse), ord);
        assert_eq!(ord, x.partial_cmp(&BigExponentFloat::from(&y)));
    });

    // - agreement with `Float`
    float_integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            float_to_big_exponent_float(&x).partial_cmp(&y),
            x.partial_cmp(&y)
        );
    });
}
