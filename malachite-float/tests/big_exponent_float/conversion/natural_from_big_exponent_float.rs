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
    big_exponent_float_exponent_is_small, big_exponent_float_power_of_2,
    big_exponent_float_to_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::big_exponent_float_gen_var_1;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::cmp::Ordering::{self, *};

const ROUNDING_MODES: [RoundingMode; 5] = [Floor, Ceiling, Down, Up, Nearest];

#[test]
fn test_natural_from_big_exponent_float() {
    // - a huge integer, beyond `Float`'s range
    let x = big_exponent_float_power_of_2(Integer::from(1u64 << 24));
    let n = Natural::try_from(&x).unwrap();
    assert_eq!(n, Natural::from(1u32) << (1u64 << 24));
    assert!(Natural::convertible_from(&x));
    assert_eq!(Natural::rounding_from(&x, Exact), (n, Equal));

    let test = |x: &str, x_hex: &str, rm: RoundingMode, out: &str, o: Ordering| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        let (n, o_alt) = Natural::rounding_from(x, rm);
        assert_eq!(n.to_string(), out);
        assert_eq!(o_alt, o);
    };
    // - a tiny positive value, beyond `Float`'s range
    test("1.2e-330985980542", "0x1.0E-274877906944", Floor, "0", Less);
    test(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        Nearest,
        "0",
        Less,
    );
    test("1.2e-330985980542", "0x1.0E-274877906944", Up, "1", Greater);
    // - negative values
    test(
        "-8.1e330985980541",
        "-0x1.0E+274877906944",
        Ceiling,
        "0",
        Greater,
    );
    test("-Infinity", "-Infinity", Down, "0", Greater);

    let test_try_from = |x: &str, x_hex: &str, out: Result<&str, UnsignedFromFloatError>| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(
            Natural::try_from(x).map(|n| n.to_string()),
            out.map(ToString::to_string)
        );
        assert_eq!(Natural::convertible_from(x), out.is_ok());
    };
    test_try_from(
        "1.2e-330985980542",
        "0x1.0E-274877906944",
        Err(UnsignedFromFloatError::FloatNonIntegerOrOutOfRange),
    );
    test_try_from(
        "-8.1e330985980541",
        "-0x1.0E+274877906944",
        Err(UnsignedFromFloatError::FloatNegative),
    );
    test_try_from(
        "NaN",
        "NaN",
        Err(UnsignedFromFloatError::FloatInfiniteOrNan),
    );
    test_try_from(
        "Infinity",
        "Infinity",
        Err(UnsignedFromFloatError::FloatInfiniteOrNan),
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
