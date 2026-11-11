// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, OneHalf, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_negate, big_exponent_float_power_of_2, float_to_big_exponent_float,
    parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, float_signed_pair_gen, float_unsigned_pair_gen,
};
use malachite_float::{BigExponentFloat, Float};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;

#[test]
fn test_partial_cmp_primitive_int() {
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
        BigExponentFloat::from(255u8),
        BigExponentFloat::from(-128i8),
        BigExponentFloat::from(u64::MAX),
        BigExponentFloat::from(i64::MIN),
        BigExponentFloat::from(u128::MAX),
        parse_big_exponent_float_hex("0x3.243f6a8885a30#53"),
        huge,
        tiny,
        neg_huge,
        neg_tiny,
    ];
    for x in &xs {
        let x = x.clone();
        for y in [0u8, 1, 3, 128, 255] {
            assert_eq!(x.partial_cmp(&y), x.partial_cmp(&Natural::from(y)));
            assert_eq!(
                y.partial_cmp(&x).map(Ordering::reverse),
                x.partial_cmp(&Natural::from(y))
            );
        }
        for y in [0i8, 1, -1, 3, -3, 127, -128] {
            assert_eq!(x.partial_cmp(&y), x.partial_cmp(&Integer::from(y)));
            assert_eq!(
                y.partial_cmp(&x).map(Ordering::reverse),
                x.partial_cmp(&Integer::from(y))
            );
        }
        for y in [0u64, 1, u64::MAX] {
            assert_eq!(x.partial_cmp(&y), x.partial_cmp(&Natural::from(y)));
        }
        for y in [0i128, i128::MIN, i128::MAX] {
            assert_eq!(x.partial_cmp(&y), x.partial_cmp(&Integer::from(y)));
        }
    }
}

fn partial_cmp_primitive_int_properties_helper_unsigned<
    T: PrimitiveUnsigned + PartialOrd<BigExponentFloat>,
>()
where
    BigExponentFloat: From<T> + PartialOrd<T> + PartialOrd<Natural>,
    Natural: From<T>,
    Float: PartialOrd<T>,
{
    big_exponent_float_gen().test_properties(|x| {
        for y in [T::ZERO, T::ONE, T::MAX] {
            assert_eq!(x.partial_cmp(&y), x.partial_cmp(&Natural::from(y)));
            assert_eq!(
                y.partial_cmp(&x).map(Ordering::reverse),
                x.partial_cmp(&Natural::from(y))
            );
        }
    });

    unsigned_gen::<T>().test_properties(|y| {
        let x = BigExponentFloat::from(y);
        assert_eq!(x.partial_cmp(&y), x.partial_cmp(&Natural::from(y)));
    });

    // - agreement with `Float`
    float_unsigned_pair_gen::<T>().test_properties(|(x, y)| {
        let out = x.partial_cmp(&y);
        let x = float_to_big_exponent_float(&x);
        assert_eq!(x.partial_cmp(&y), out);
    });
}

fn partial_cmp_primitive_int_properties_helper_signed<
    T: PrimitiveSigned + PartialOrd<BigExponentFloat>,
>()
where
    BigExponentFloat: From<T> + PartialOrd<T> + PartialOrd<Integer>,
    Integer: From<T>,
    Float: PartialOrd<T>,
{
    big_exponent_float_gen().test_properties(|x| {
        for y in [T::ZERO, T::ONE, T::NEGATIVE_ONE, T::MIN, T::MAX] {
            assert_eq!(x.partial_cmp(&y), x.partial_cmp(&Integer::from(y)));
            assert_eq!(
                y.partial_cmp(&x).map(Ordering::reverse),
                x.partial_cmp(&Integer::from(y))
            );
        }
    });

    signed_gen::<T>().test_properties(|y| {
        let x = BigExponentFloat::from(y);
        assert_eq!(x.partial_cmp(&y), x.partial_cmp(&Integer::from(y)));
    });

    // - agreement with `Float`
    float_signed_pair_gen::<T>().test_properties(|(x, y)| {
        let out = x.partial_cmp(&y);
        let x = float_to_big_exponent_float(&x);
        assert_eq!(x.partial_cmp(&y), out);
    });
}

#[test]
fn partial_cmp_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_cmp_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(partial_cmp_primitive_int_properties_helper_signed);
}
