// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::NaN;
use malachite_base::num::conversion::from::SignedFromFloatError;
use malachite_base::num::conversion::traits::{ConvertibleFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, big_exponent_float_negate, big_exponent_float_power_of_2,
    big_exponent_float_to_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::big_exponent_float_gen_var_1;
use malachite_nz::integer::Integer;
use std::cmp::Ordering::{self, *};

const ROUNDING_MODES: [RoundingMode; 5] = [Floor, Ceiling, Down, Up, Nearest];

#[test]
fn test_integer_from_big_exponent_float() {
    let x = big_exponent_float_negate(big_exponent_float_power_of_2(Integer::from(1u64 << 24)));
    let n = Integer::try_from(&x).unwrap();
    assert_eq!(n, -(Integer::from(1u32) << (1u64 << 24)));
    assert!(Integer::convertible_from(&x));

    let test = |x: &str, x_hex: &str, rm: RoundingMode, out: &str, o: Ordering| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        let (n, o_alt) = Integer::rounding_from(x, rm);
        assert_eq!(n.to_string(), out);
        assert_eq!(o_alt, o);
    };
    let _ = big_exponent_float_negate(big_exponent_float_power_of_2(-Integer::from(1u64 << 40)));
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        Floor,
        "-1",
        Less,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        Ceiling,
        "0",
        Greater,
    );
    test(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        Nearest,
        "0",
        Greater,
    );

    let test_try_from = |x: &str, x_hex: &str, out: Result<&str, SignedFromFloatError>| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(
            Integer::try_from(x).map(|n| n.to_string()),
            out.map(ToString::to_string)
        );
        assert_eq!(Integer::convertible_from(x), out.is_ok());
    };
    test_try_from(
        "-1.2e-330985980542",
        "-0x1.0E-274877906944",
        Err(SignedFromFloatError::FloatNonIntegerOrOutOfRange),
    );
    test_try_from("NaN", "NaN", Err(SignedFromFloatError::FloatInfiniteOrNan));
}

#[test]
#[should_panic]
fn integer_rounding_from_big_exponent_float_fail() {
    Integer::rounding_from(BigExponentFloat::NAN, Floor);
}

#[test]
fn integer_from_big_exponent_float_properties() {
    big_exponent_float_gen_var_1().test_properties(|x| {
        if !big_exponent_float_exponent_is_small(&x) {
            return;
        }
        let f = big_exponent_float_to_float(&x).unwrap();
        assert_eq!(Integer::try_from(&x), Integer::try_from(&f));
        assert_eq!(Integer::convertible_from(&x), Integer::convertible_from(&f));
        assert_eq!(
            Integer::try_from(x.clone()).is_ok(),
            Integer::convertible_from(&x)
        );
        if x.is_finite() {
            for rm in ROUNDING_MODES {
                let result = Integer::rounding_from(&x, rm);
                assert_eq!(result, Integer::rounding_from(&f, rm));
                assert_eq!(Integer::rounding_from(x.clone(), rm), result);
            }
        }
    });
}
