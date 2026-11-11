// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, OneHalf, Zero};
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_negate, big_exponent_float_power_of_2, float_to_big_exponent_float,
    parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, float_gaussian_rational_pair_gen,
};
use malachite_nz::integer::Integer;
use malachite_q::gaussian_rational::GaussianRational;

#[test]
fn test_partial_eq_gaussian_rational() {
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big);
    let neg_huge = big_exponent_float_negate(huge.clone());
    let neg_tiny = big_exponent_float_negate(tiny.clone());
    let test = |x: BigExponentFloat, y: &str| {
        let y = GaussianRational::from_str(y).unwrap();
        let out = y.imaginary == 0u32 && x == y.real;
        assert_eq!(x == y, out);
        assert_eq!(y == x, out);
    };
    test(BigExponentFloat::NAN, "3+4i");
    test(BigExponentFloat::INFINITY, "3+4i");
    test(BigExponentFloat::NEGATIVE_INFINITY, "0");
    test(BigExponentFloat::ZERO, "0");
    test(BigExponentFloat::ZERO, "i");
    test(BigExponentFloat::from(5), "3+4i");
    test(BigExponentFloat::from(-5), "-3-4i");
    test(BigExponentFloat::from(5), "-4i");
    test(BigExponentFloat::from(-4), "-4i");
    test(BigExponentFloat::from(3), "3");
    test(BigExponentFloat::from(-3), "3");
    test(BigExponentFloat::from(4.5), "3+4i");
    test(BigExponentFloat::from(5.5), "3+4i");
    test(
        parse_big_exponent_float_hex("0x5.0000000000000000000010#90"),
        "3+4i",
    );
    test(
        parse_big_exponent_float_hex("0x4.fffffffffffffffffffff0#90"),
        "3+4i",
    );
    test(huge.clone(), "3+4i");
    test(neg_huge.clone(), "-3+4i");
    test(tiny.clone(), "3+4i");
    test(tiny.clone(), "0");
    test(neg_tiny.clone(), "i");
    test(BigExponentFloat::from(2.5), "3/2+2i");
    test(BigExponentFloat::from(-2.5), "-3/2-2i");
    test(BigExponentFloat::ONE_HALF, "1/3+i/3");
    test(
        tiny.clone(),
        "1/1000000000000000000000000000000+i/1000000000000000000000000000000",
    );
    test(BigExponentFloat::from(3), "3+0i");
    test(huge.clone(), "0");
}

#[test]
fn partial_eq_gaussian_rational_properties() {
    // - agreement with `Float`
    float_gaussian_rational_pair_gen().test_properties(|(x, y)| {
        let out = x == y;
        let x = float_to_big_exponent_float(&x);
        assert_eq!(x == y, out);
        assert_eq!(y == x, out);
    });

    big_exponent_float_gen().test_properties(|x| {
        let y = GaussianRational::from_str("1000000000000000000000000000000/7").unwrap();
        assert_eq!(x == y, x == y.real);
    });
}
