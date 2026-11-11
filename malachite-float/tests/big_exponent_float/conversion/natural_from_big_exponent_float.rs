// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
use malachite_base::num::conversion::from::UnsignedFromFloatError;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_negate, big_exponent_float_power_of_2,
    big_exponent_float_to_float,
};
use malachite_float::test_util::generators::big_exponent_float_gen_var_1;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::cmp::Ordering::*;

const ROUNDING_MODES: [RoundingMode; 5] = [Floor, Ceiling, Down, Up, Nearest];

#[test]
fn test_natural_from_big_exponent_float() {
    // - a huge integer, beyond `Float`'s range
    let x = big_exponent_float_power_of_2(Integer::from(1u64 << 24));
    let n = Natural::try_from(&x).unwrap();
    assert_eq!(n, Natural::from(1u32) << (1u64 << 24));
    assert!(Natural::convertible_from(&x));
    assert_eq!(Natural::rounding_from(&x, Exact), (n, Equal));
    // - a tiny positive value, beyond `Float`'s range
    let x = big_exponent_float_power_of_2(-Integer::from(1u64 << 40));
    assert_eq!(
        Natural::rounding_from(&x, Floor),
        (Natural::from(0u32), Less)
    );
    assert_eq!(
        Natural::rounding_from(&x, Nearest),
        (Natural::from(0u32), Less)
    );
    assert_eq!(
        Natural::rounding_from(&x, Up),
        (Natural::from(1u32), Greater)
    );
    assert_eq!(
        Natural::try_from(&x),
        Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange)
    );
    // - negative values
    let x = big_exponent_float_negate(big_exponent_float_power_of_2(Integer::from(1u64 << 40)));
    assert_eq!(
        Natural::rounding_from(&x, Ceiling),
        (Natural::from(0u32), Greater)
    );
    assert_eq!(
        Natural::try_from(&x),
        Err(UnsignedFromFloatError::FloatNegative)
    );
    assert_eq!(
        Natural::rounding_from(&BigExponentFloat::NEGATIVE_INFINITY, Down),
        (Natural::from(0u32), Greater)
    );
    assert_eq!(
        Natural::try_from(&BigExponentFloat::NAN),
        Err(UnsignedFromFloatError::FloatInfiniteOrNan)
    );
    assert_eq!(
        Natural::try_from(BigExponentFloat::INFINITY),
        Err(UnsignedFromFloatError::FloatInfiniteOrNan)
    );
}

#[test]
#[should_panic]
fn natural_rounding_from_big_exponent_float_fail_1() {
    Natural::rounding_from(BigExponentFloat::NAN, Floor);
}

#[test]
#[should_panic]
fn natural_rounding_from_big_exponent_float_fail_2() {
    Natural::rounding_from(BigExponentFloat::NEGATIVE_INFINITY, Floor);
}

#[test]
#[should_panic]
fn natural_rounding_from_big_exponent_float_fail_3() {
    let x = big_exponent_float_power_of_2(Integer::from(-1));
    Natural::rounding_from(x, Exact);
}

#[test]
fn natural_from_big_exponent_float_properties() {
    big_exponent_float_gen_var_1().test_properties(|x| {
        if !big_exponent_float_exponent_is_small(&x) {
            return;
        }
        let f = big_exponent_float_to_float(&x).unwrap();
        // - agreement with `Float`
        assert_eq!(Natural::try_from(&x), Natural::try_from(&f));
        assert_eq!(Natural::convertible_from(&x), Natural::convertible_from(&f));
        assert_eq!(
            Natural::try_from(x.clone()).is_ok(),
            Natural::convertible_from(&x)
        );
        for rm in ROUNDING_MODES {
            let panics = x.is_nan()
                || x.is_infinite() && x.is_sign_positive()
                || x.is_sign_negative() && !matches!(rm, Ceiling | Down | Nearest);
            if !panics {
                let result = Natural::rounding_from(&x, rm);
                assert_eq!(result, Natural::rounding_from(&f, rm));
                assert_eq!(Natural::rounding_from(x.clone(), rm), result);
            }
        }
    });
}
