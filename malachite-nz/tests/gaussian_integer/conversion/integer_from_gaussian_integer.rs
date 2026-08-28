// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, IsReal};
use malachite_base::strings::ToDebugString;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::{gaussian_integer_gen, gaussian_integer_gen_var_1};
use std::str::FromStr;

#[test]
fn test_try_from_gaussian_integer() {
    let test = |s, out| {
        let x = GaussianInteger::from_str(s).unwrap();

        let on = Integer::try_from(x.clone());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Integer::try_from(&x);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("0", "Ok(0)");
    test("123", "Ok(123)");
    test("-123", "Ok(-123)");
    test("1000000000000", "Ok(1000000000000)");
    test("-1000000000000", "Ok(-1000000000000)");
    test("i", "Err(IntegerFromGaussianIntegerError)");
    test("-i", "Err(IntegerFromGaussianIntegerError)");
    test("2-3i", "Err(IntegerFromGaussianIntegerError)");
}

#[test]
fn test_exact_from_gaussian_integer() {
    let test = |s, out| {
        let x = GaussianInteger::from_str(s).unwrap();

        let n = Integer::exact_from(x.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::exact_from(&x);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0");
    test("123", "123");
    test("-123", "-123");
    test("1000000000000", "1000000000000");
    test("-1000000000000", "-1000000000000");
}

#[test]
#[should_panic]
fn integer_exact_from_gaussian_integer_fail() {
    Integer::exact_from(GaussianInteger::from_str("2-3i").unwrap());
}

#[test]
#[should_panic]
fn integer_exact_from_gaussian_integer_ref_fail() {
    Integer::exact_from(&GaussianInteger::from_str("i").unwrap());
}

#[test]
fn test_convertible_from_gaussian_integer() {
    let test = |s, out| {
        let x = GaussianInteger::from_str(s).unwrap();
        assert_eq!(Integer::convertible_from(&x), out);
    };
    test("0", true);
    test("123", true);
    test("-123", true);
    test("1000000000000", true);
    test("-1000000000000", true);
    test("i", false);
    test("-i", false);
    test("2-3i", false);
}

#[test]
fn try_from_gaussian_integer_properties() {
    gaussian_integer_gen().test_properties(|x| {
        let on = Integer::try_from(x.clone());
        assert!(on.as_ref().map_or(true, Integer::is_valid));
        assert_eq!(Integer::try_from(&x), on);
        assert_eq!(on.is_ok(), x.is_real());
        assert_eq!(Integer::convertible_from(&x), on.is_ok());
        if let Ok(n) = on {
            assert_eq!(n, x.real);
            assert_eq!(GaussianInteger::from(n), x);
        }
    });

    gaussian_integer_gen_var_1().test_properties(|x| {
        assert!(Integer::convertible_from(&x));
        assert_eq!(GaussianInteger::from(Integer::exact_from(&x)), x);
    });
}
