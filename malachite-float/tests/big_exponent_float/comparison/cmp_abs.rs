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
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::{
    ComparableBigExponentFloat, ComparableBigExponentFloatRef,
};
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_negate, big_exponent_float_power_of_2, float_to_big_exponent_float,
    parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_pair_gen, big_exponent_float_triple_gen,
    float_pair_gen,
};
use malachite_nz::integer::Integer;
use std::cmp::Ordering::{self, *};

fn abs(x: &BigExponentFloat) -> BigExponentFloat {
    if x.is_sign_negative() {
        big_exponent_float_negate(x.clone())
    } else {
        x.clone()
    }
}

#[test]
fn test_partial_cmp_abs() {
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
            let out = abs(x).partial_cmp(&abs(y));
            assert_eq!(x.partial_cmp_abs(y), out);
            assert_eq!(
                ComparableBigExponentFloatRef(x).cmp_abs(&ComparableBigExponentFloatRef(y)),
                out.unwrap()
            );
        }
        assert_eq!(x.partial_cmp_abs(&BigExponentFloat::NAN), None);
        // - NaN is below everything in absolute value
        assert_eq!(
            ComparableBigExponentFloat(BigExponentFloat::NAN)
                .cmp_abs(&ComparableBigExponentFloat(x.clone())),
            Less
        );
    }
}

#[test]
fn partial_cmp_abs_properties() {
    big_exponent_float_pair_gen().test_properties(|(x, y)| {
        let ord = x.partial_cmp_abs(&y);
        assert_eq!(ord, abs(&x).partial_cmp(&abs(&y)));
        assert_eq!(y.partial_cmp_abs(&x).map(Ordering::reverse), ord);
        let c_ord = ComparableBigExponentFloatRef(&x).cmp_abs(&ComparableBigExponentFloatRef(&y));
        assert_eq!(
            ComparableBigExponentFloat(x.clone()).cmp_abs(&ComparableBigExponentFloat(y.clone())),
            c_ord
        );
        assert_eq!(
            ComparableBigExponentFloat(x.clone())
                .partial_cmp_abs(&ComparableBigExponentFloat(y.clone())),
            Some(c_ord)
        );
        if let Some(ord) = ord {
            assert_eq!(c_ord, ord);
        }
    });

    // - agreement with `Float`
    float_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            float_to_big_exponent_float(&x).partial_cmp_abs(&float_to_big_exponent_float(&y)),
            x.partial_cmp_abs(&y)
        );
    });

    big_exponent_float_gen().test_properties(|x| {
        assert_eq!(
            ComparableBigExponentFloatRef(&x).cmp_abs(&ComparableBigExponentFloatRef(&x)),
            Equal
        );
    });

    big_exponent_float_triple_gen().test_properties(|(x, y, z)| {
        let cx = ComparableBigExponentFloatRef(&x);
        let cy = ComparableBigExponentFloatRef(&y);
        let cz = ComparableBigExponentFloatRef(&z);
        if cx.lt_abs(&cy) && cy.lt_abs(&cz) {
            assert!(cx.lt_abs(&cz));
        } else if cx.gt_abs(&cy) && cy.gt_abs(&cz) {
            assert!(cx.gt_abs(&cz));
        }
    });
}
