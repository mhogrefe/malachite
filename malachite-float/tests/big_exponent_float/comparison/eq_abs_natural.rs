// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, OneHalf, Zero};
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_negate, big_exponent_float_power_of_2, float_to_big_exponent_float,
    parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_natural_pair_gen, float_natural_pair_gen,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::cmp::Ordering::*;

#[test]
fn test_eq_abs_natural() {
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big);
    let neg_huge = big_exponent_float_negate(huge.clone());
    let neg_tiny = big_exponent_float_negate(tiny.clone());
    let test = |x: BigExponentFloat, y: &str| {
        let y = Natural::from_str(y).unwrap();
        let out = x.partial_cmp_abs(&y) == Some(Equal);
        assert_eq!(x.eq_abs(&y), out);
        assert_eq!(y.eq_abs(&x), out);
    };
    test(BigExponentFloat::NAN, "0");
    test(BigExponentFloat::INFINITY, "0");
    test(BigExponentFloat::NEGATIVE_INFINITY, "0");
    test(BigExponentFloat::ZERO, "0");
    test(BigExponentFloat::ZERO, "1");
    test(BigExponentFloat::ONE, "1");
    test(BigExponentFloat::ONE_HALF, "1");
    test(BigExponentFloat::ONE, "0");
    test(parse_big_exponent_float_hex("0x3.243f6a8885a30#53"), "3");
    test(parse_big_exponent_float_hex("0x3.243f6a8885a30#53"), "4");
    test(parse_big_exponent_float_hex("0xff.0#8"), "255");
    test(parse_big_exponent_float_hex("0xff.8#9"), "255");
    test(
        parse_big_exponent_float_hex("0x1.0E+40#1"),
        "1461501637330902918203684832716283019655932542976",
    );
    test(
        parse_big_exponent_float_hex("0x1.0E+40#1"),
        "1461501637330902918203684832716283019655932542977",
    );
    test(huge.clone(), "0");
    test(
        huge.clone(),
        "1461501637330902918203684832716283019655932542976",
    );
    test(tiny.clone(), "0");
    test(tiny.clone(), "1");
    test(neg_huge.clone(), "0");
    test(neg_tiny.clone(), "0");
}

#[test]
fn eq_abs_natural_properties() {
    big_exponent_float_natural_pair_gen().test_properties(|(x, y)| {
        let e = x.eq_abs(&y);
        assert_eq!(y.eq_abs(&x), e);
        assert_eq!(x.partial_cmp_abs(&y) == Some(Equal), e);
        assert_eq!(big_exponent_float_negate(x).eq_abs(&y), e);
    });

    // - agreement with `Float`
    float_natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(float_to_big_exponent_float(&x).eq_abs(&y), x.eq_abs(&y));
    });
}
