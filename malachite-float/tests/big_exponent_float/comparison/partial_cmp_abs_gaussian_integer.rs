// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_negate, big_exponent_float_power_of_2,
    float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, float_gaussian_integer_pair_gen,
};
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

// Compares |x| with |y| through x^2 and |y|^2, for an `x` with a small exponent, and otherwise by
// the size of the exponent.
fn gaussian_partial_cmp_abs_oracle(x: &BigExponentFloat, y: &GaussianInteger) -> Option<Ordering> {
    if x.is_nan() {
        None
    } else if !x.is_finite() {
        Some(Greater)
    } else if big_exponent_float_exponent_is_small(x) {
        Some(
            Rational::exact_from(x)
                .abs_squared()
                .cmp(&Rational::from(y.abs_squared())),
        )
    } else if *x.get_exponent().unwrap() > 0u32 {
        Some(Greater)
    } else {
        Some(if *y == 0u32 { Greater } else { Less })
    }
}

#[test]
fn test_partial_cmp_abs_gaussian_integer() {
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big);
    let neg_huge = big_exponent_float_negate(huge.clone());
    let neg_tiny = big_exponent_float_negate(tiny.clone());
    let test = |x: BigExponentFloat, y: &str| {
        let y = GaussianInteger::from_str(y).unwrap();
        let out = gaussian_partial_cmp_abs_oracle(&x, &y);
        assert_eq!(x.partial_cmp_abs(&y), out);
        assert_eq!(y.partial_cmp_abs(&x).map(Ordering::reverse), out);
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
}

#[test]
fn partial_cmp_abs_gaussian_integer_properties() {
    // - agreement with `Float`
    float_gaussian_integer_pair_gen().test_properties(|(x, y)| {
        let out = x.partial_cmp_abs(&y);
        let x = float_to_big_exponent_float(&x);
        assert_eq!(x.partial_cmp_abs(&y), out);
        assert_eq!(y.partial_cmp_abs(&x).map(Ordering::reverse), out);
        assert_eq!(
            x.partial_cmp_abs(&y),
            gaussian_partial_cmp_abs_oracle(&x, &y)
        );
    });

    big_exponent_float_gen().test_properties(|x| {
        for y in ["0", "1+i", "-3+4i", "1000000000000000000000000000000"] {
            let y = GaussianInteger::from_str(y).unwrap();
            assert_eq!(
                x.partial_cmp_abs(&y),
                gaussian_partial_cmp_abs_oracle(&x, &y)
            );
        }
    });
}
