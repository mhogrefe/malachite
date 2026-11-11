// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, OneHalf, Zero};
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_negate, big_exponent_float_power_of_2, float_to_big_exponent_float,
    parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, float_primitive_float_pair_gen,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use std::cmp::Ordering;

#[test]
fn test_partial_cmp_primitive_float() {
    let big = Integer::from_str("100000000000000000000").unwrap();
    let huge = big_exponent_float_power_of_2(big.clone());
    let tiny = big_exponent_float_power_of_2(-big);
    let neg_huge = big_exponent_float_negate(huge.clone());
    let neg_tiny = big_exponent_float_negate(tiny.clone());
    let xs = [
        BigExponentFloat::NAN,
        BigExponentFloat::INFINITY,
        BigExponentFloat::NEGATIVE_INFINITY,
        BigExponentFloat::ZERO,
        BigExponentFloat::ONE,
        BigExponentFloat::ONE_HALF,
        BigExponentFloat::from(f64::MAX_FINITE),
        BigExponentFloat::from(f64::MIN_POSITIVE_SUBNORMAL),
        BigExponentFloat::from(-f32::MAX_FINITE),
        parse_big_exponent_float_hex("0x3.243f6a8885a30#53"),
        parse_big_exponent_float_hex("0x3.243f6a8885a308d4#64"),
        huge,
        tiny,
        neg_huge,
        neg_tiny,
    ];
    for x in &xs {
        let x = x.clone();
        for y in [
            f64::NAN,
            f64::INFINITY,
            f64::NEGATIVE_INFINITY,
            0.0,
            -0.0,
            1.0,
            0.5,
            -0.5,
            core::f64::consts::PI,
            f64::MAX_FINITE,
            f64::MIN_POSITIVE_SUBNORMAL,
        ] {
            assert_eq!(x.partial_cmp(&y), x.partial_cmp(&BigExponentFloat::from(y)));
            assert_eq!(
                y.partial_cmp(&x).map(Ordering::reverse),
                x.partial_cmp(&BigExponentFloat::from(y))
            );
        }
        for y in [f32::NAN, f32::INFINITY, -0.0f32, 1.5, -f32::MAX_FINITE] {
            assert_eq!(x.partial_cmp(&y), x.partial_cmp(&BigExponentFloat::from(y)));
            assert_eq!(
                y.partial_cmp(&x).map(Ordering::reverse),
                x.partial_cmp(&BigExponentFloat::from(y))
            );
        }
    }
}

fn partial_cmp_primitive_float_properties_helper<T: PrimitiveFloat + PartialOrd<BigExponentFloat>>()
where
    BigExponentFloat: From<T> + PartialOrd<T> + PartialOrd<BigExponentFloat>,
    Float: PartialOrd<T>,
{
    big_exponent_float_gen().test_properties(|x| {
        for y in [T::ZERO, T::ONE, T::NAN, T::INFINITY, T::MAX_FINITE, T::MIN_POSITIVE_SUBNORMAL] {
            assert_eq!(x.partial_cmp(&y), x.partial_cmp(&BigExponentFloat::from(y)));
            assert_eq!(
                y.partial_cmp(&x).map(Ordering::reverse),
                x.partial_cmp(&BigExponentFloat::from(y))
            );
        }
    });

    primitive_float_gen::<T>().test_properties(|y| {
        for x in [BigExponentFloat::ZERO, BigExponentFloat::from(y)] {
            assert_eq!(x.partial_cmp(&y), x.partial_cmp(&BigExponentFloat::from(y)));
        }
    });

    // - agreement with `Float`
    float_primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let out = x.partial_cmp(&y);
        let x = float_to_big_exponent_float(&x);
        assert_eq!(x.partial_cmp(&y), out);
    });
}

#[test]
fn partial_cmp_primitive_float_properties() {
    apply_fn_to_primitive_floats!(partial_cmp_primitive_float_properties_helper);
}
