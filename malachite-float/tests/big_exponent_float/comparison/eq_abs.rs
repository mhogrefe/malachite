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
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::{
    ComparableBigExponentFloat, ComparableBigExponentFloatRef,
};
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_negate, big_exponent_float_power_of_2, float_to_big_exponent_float,
    parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_pair_gen, float_pair_gen,
};
use malachite_nz::integer::Integer;

#[test]
fn test_eq_abs() {
    // - the values, in increasing order, with exponents far outside `Float`'s range
    let big = Integer::from_str("100000000000000000000").unwrap();
    let xs = [
        BigExponentFloat::NEGATIVE_INFINITY,
        big_exponent_float_negate(big_exponent_float_power_of_2(big.clone() + Integer::ONE)),
        big_exponent_float_negate(big_exponent_float_power_of_2(big.clone())),
        parse_big_exponent_float_hex("-0x3.243f6a8885a30#53"),
        parse_big_exponent_float_hex("-0x3.0#2"),
        BigExponentFloat::NEGATIVE_ONE,
        big_exponent_float_negate(big_exponent_float_power_of_2(-big.clone())),
        BigExponentFloat::ZERO,
        big_exponent_float_power_of_2(-big.clone() - Integer::ONE),
        big_exponent_float_power_of_2(-big.clone()),
        parse_big_exponent_float_hex("0x0.55555555555554#53"),
        BigExponentFloat::ONE_HALF,
        BigExponentFloat::ONE,
        parse_big_exponent_float_hex("0x1.0000000000000000000000000000000000000002#160"),
        parse_big_exponent_float_hex("0x3.243f6a8885a30#53"),
        parse_big_exponent_float_hex("0x3.243f6a8885a308d313198a2e037074#120"),
        big_exponent_float_power_of_2(big.clone()),
        big_exponent_float_power_of_2(big.clone() + Integer::ONE),
        BigExponentFloat::INFINITY,
    ];
    for x in &xs {
        for y in &xs {
            let out = x == y || *x == big_exponent_float_negate(y.clone());
            assert_eq!(x.eq_abs(y), out);
            assert_eq!(
                ComparableBigExponentFloatRef(x).eq_abs(&ComparableBigExponentFloatRef(y)),
                out
            );
        }
        assert!(x.ne_abs(&BigExponentFloat::NAN));
    }
    assert!(BigExponentFloat::NAN.ne_abs(&BigExponentFloat::NAN));
    assert!(
        ComparableBigExponentFloat(BigExponentFloat::NAN)
            .eq_abs(&ComparableBigExponentFloat(BigExponentFloat::NAN))
    );
}

#[test]
fn eq_abs_properties() {
    big_exponent_float_pair_gen().test_properties(|(x, y)| {
        let e = x.eq_abs(&y);
        assert_eq!(y.eq_abs(&x), e);
        assert_eq!(x.partial_cmp_abs(&y) == Some(core::cmp::Ordering::Equal), e);
        assert_eq!(big_exponent_float_negate(x.clone()).eq_abs(&y), e);
        let c = ComparableBigExponentFloatRef(&x).eq_abs(&ComparableBigExponentFloatRef(&y));
        assert_eq!(
            ComparableBigExponentFloat(x.clone()).eq_abs(&ComparableBigExponentFloat(y.clone())),
            c
        );
        assert_eq!(c, e || (x.is_nan() && y.is_nan()));
    });

    // - agreement with `Float`
    float_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            float_to_big_exponent_float(&x).eq_abs(&float_to_big_exponent_float(&y)),
            x.eq_abs(&y)
        );
    });

    big_exponent_float_gen().test_properties(|x| {
        assert_eq!(x.eq_abs(&x), !x.is_nan());
    });
}
