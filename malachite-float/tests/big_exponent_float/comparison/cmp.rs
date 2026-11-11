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
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_float::big_exponent_float::{
    ComparableBigExponentFloat, ComparableBigExponentFloatRef,
};
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_negate, big_exponent_float_power_of_2,
    float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_2, big_exponent_float_pair_gen,
    big_exponent_float_triple_gen, float_pair_gen,
};
use malachite_float::{BigExponentFloat, ComparableFloatRef};
use malachite_nz::integer::Integer;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

#[test]
fn test_partial_cmp() {
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
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in xs.iter().enumerate() {
            assert_eq!(x.partial_cmp(y), Some(i.cmp(&j)));
            assert_eq!(
                ComparableBigExponentFloatRef(x).cmp(&ComparableBigExponentFloatRef(y)),
                i.cmp(&j)
            );
        }
        assert_eq!(x.partial_cmp(&BigExponentFloat::NAN), None);
        assert_eq!(BigExponentFloat::NAN.partial_cmp(x), None);
        // - NaN lies between the negative values and zero
        let nan_cmp = ComparableBigExponentFloat(BigExponentFloat::NAN)
            .cmp(&ComparableBigExponentFloat(x.clone()));
        assert_eq!(nan_cmp, if x.is_sign_negative() { Greater } else { Less });
    }
    assert_eq!(
        ComparableBigExponentFloat(BigExponentFloat::NAN)
            .cmp(&ComparableBigExponentFloat(BigExponentFloat::NAN)),
        Equal
    );
}

#[allow(clippy::needless_pass_by_value)]
fn partial_cmp_properties_helper(x: BigExponentFloat, y: BigExponentFloat) {
    let ord = x.partial_cmp(&y);
    assert_eq!(y.partial_cmp(&x).map(Ordering::reverse), ord);
    assert_eq!(x == y, ord == Some(Equal));
    let cx = ComparableBigExponentFloatRef(&x);
    let cy = ComparableBigExponentFloatRef(&y);
    let c_ord = cx.cmp(&cy);
    assert_eq!(cy.cmp(&cx), c_ord.reverse());
    assert_eq!(
        ComparableBigExponentFloat(x.clone()).cmp(&ComparableBigExponentFloat(y.clone())),
        c_ord
    );
    if let Some(ord) = ord {
        assert_eq!(c_ord, ord);
    }
    assert_eq!(
        big_exponent_float_negate(y.clone()).partial_cmp(&big_exponent_float_negate(x.clone())),
        ord
    );
    if x.is_finite()
        && y.is_finite()
        && big_exponent_float_exponent_is_small(&x)
        && big_exponent_float_exponent_is_small(&y)
    {
        assert_eq!(
            Some(Rational::exact_from(&x).cmp(&Rational::exact_from(&y))),
            ord
        );
    }
}

#[test]
fn partial_cmp_properties() {
    big_exponent_float_pair_gen().test_properties(|(x, y)| {
        partial_cmp_properties_helper(x, y);
    });

    // - nearby values with equal exponents
    big_exponent_float_gen().test_properties(|x| {
        if let Some(prec) = x.get_prec() {
            let y = x
                .round_to_prec_round_ref(prec.div_ceil(2), RoundingMode::Floor)
                .0;
            partial_cmp_properties_helper(x, y);
        }
    });

    // - agreement with `Float`
    float_pair_gen().test_properties(|(x, y)| {
        let bx = float_to_big_exponent_float(&x);
        let by = float_to_big_exponent_float(&y);
        assert_eq!(bx.partial_cmp(&by), x.partial_cmp(&y));
        if !x.is_nan() && !y.is_nan() && x != y {
            assert_eq!(
                ComparableBigExponentFloatRef(&bx).cmp(&ComparableBigExponentFloatRef(&by)),
                ComparableFloatRef(&x).cmp(&ComparableFloatRef(&y))
            );
        }
    });

    big_exponent_float_gen().test_properties(|x| {
        let cx = ComparableBigExponentFloatRef(&x);
        assert_eq!(cx.cmp(&cx), Equal);
        assert_eq!(
            x.partial_cmp(&x),
            if x.is_nan() { None } else { Some(Equal) }
        );
        assert!(ComparableBigExponentFloatRef(&BigExponentFloat::NEGATIVE_INFINITY) <= cx);
        assert!(ComparableBigExponentFloatRef(&BigExponentFloat::INFINITY) >= cx);
    });

    big_exponent_float_gen_var_2().test_properties(|x| {
        if x.is_finite() {
            assert!(x > BigExponentFloat::NEGATIVE_INFINITY);
            assert!(x < BigExponentFloat::INFINITY);
        }
    });

    big_exponent_float_triple_gen().test_properties(|(x, y, z)| {
        let cx = ComparableBigExponentFloatRef(&x);
        let cy = ComparableBigExponentFloatRef(&y);
        let cz = ComparableBigExponentFloatRef(&z);
        if cx < cy && cy < cz {
            assert!(cx < cz);
        } else if cx > cy && cy > cz {
            assert!(cx > cz);
        }
    });
}
