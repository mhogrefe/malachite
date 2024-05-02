// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
use malachite_base::test_util::generators::unsigned_gen_var_1;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen_var_2, natural_unsigned_pair_gen_var_4};
use std::str::FromStr;

#[test]
fn test_integer_mantissa_and_exponent() {
    let test = |s: &str, mantissa: &str, exponent: u64| {
        let n = Natural::from_str(s).unwrap();
        let mantissa = Natural::from_str(mantissa).unwrap();
        let (actual_mantissa, actual_exponent) = n.integer_mantissa_and_exponent();
        assert_eq!(actual_mantissa, mantissa);
        assert_eq!(actual_exponent, exponent);
        assert_eq!(n.integer_mantissa(), mantissa);
        assert_eq!(n.integer_exponent(), exponent);
    };
    test("1", "1", 0);
    test("3", "3", 0);
    test("100", "25", 2);
    test("123", "123", 0);
}

#[test]
#[should_panic]
fn integer_mantissa_and_exponent_fail() {
    Natural::ZERO.integer_mantissa_and_exponent();
}

#[test]
fn test_from_integer_mantissa_and_exponent() {
    let test = |mantissa: &str, exponent: u64, out: Option<&str>| {
        let mantissa = Natural::from_str(mantissa).unwrap();
        assert_eq!(
            <&Natural as IntegerMantissaAndExponent<_, _, _>>::from_integer_mantissa_and_exponent(
                mantissa, exponent
            ),
            out.map(|s| Natural::from_str(s).unwrap())
        );
    };
    test("0", 5, Some("0"));
    test("1", 0, Some("1"));
    test("3", 0, Some("3"));
    test("25", 2, Some("100"));
}

#[test]
fn integer_mantissa_and_exponent_properties() {
    natural_gen_var_2().test_properties(|n| {
        let (mantissa, exponent) = n.integer_mantissa_and_exponent();
        assert_eq!(n.integer_mantissa(), mantissa);
        assert_eq!(n.integer_exponent(), exponent);
        assert!(mantissa.odd());
        let n_alt = <&Natural as IntegerMantissaAndExponent::<Natural, u64, Natural>>
            ::from_integer_mantissa_and_exponent(mantissa, exponent);
        assert_eq!(n_alt.unwrap(), n);
    });

    unsigned_gen_var_1::<Limb>().test_properties(|x| {
        let (mantissa_1, exponent_1) = x.integer_mantissa_and_exponent();
        let (mantissa_2, exponent_2) = Natural::from(x).integer_mantissa_and_exponent();
        assert_eq!(mantissa_1, mantissa_2);
        assert_eq!(exponent_1, exponent_2);
    });
}

#[test]
fn from_integer_mantissa_and_exponent_properties() {
    natural_unsigned_pair_gen_var_4::<u64>().test_properties(|(m, e)| {
        let n =
            <&Natural as IntegerMantissaAndExponent<_, _, _>>::from_integer_mantissa_and_exponent(
                m.clone(),
                e,
            )
            .unwrap();
        if m.odd() {
            assert_eq!(n.integer_mantissa_and_exponent(), (m, e));
        }
    });
}
