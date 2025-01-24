// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, SaturatingFrom};
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_gen;
use std::str::FromStr;

#[test]
fn test_try_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        let on = Natural::try_from(u.clone());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Natural::try_from(&u);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("0", "Ok(0)");
    test("123", "Ok(123)");
    test("-123", "Err(NaturalFromIntegerError)");
    test("1000000000000", "Ok(1000000000000)");
    test("-1000000000000", "Err(NaturalFromIntegerError)");
    test("2147483647", "Ok(2147483647)");
    test("2147483648", "Ok(2147483648)");
    test("-2147483648", "Err(NaturalFromIntegerError)");
    test("-2147483649", "Err(NaturalFromIntegerError)");
}

#[test]
fn test_exact_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        let n = Natural::exact_from(u.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::exact_from(&u);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0");
    test("123", "123");
    test("1000000000000", "1000000000000");
    test("2147483647", "2147483647");
    test("2147483648", "2147483648");
}

#[test]
#[should_panic]
fn natural_exact_from_integer_fail_1() {
    Natural::exact_from(Integer::from_str("-123").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_integer_fail_2() {
    Natural::exact_from(Integer::from_str("-1000000000000").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_integer_fail_3() {
    Natural::exact_from(Integer::from_str("-2147483648").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_integer_fail_4() {
    Natural::exact_from(Integer::from_str("-2147483649").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_integer_ref_fail_1() {
    Natural::exact_from(&Integer::from_str("-123").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_integer_ref_fail_2() {
    Natural::exact_from(&Integer::from_str("-1000000000000").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_integer_ref_fail_3() {
    Natural::exact_from(&Integer::from_str("-2147483648").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_integer_ref_fail_4() {
    Natural::exact_from(&Integer::from_str("-2147483649").unwrap());
}

#[test]
fn test_saturating_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        let n = Natural::saturating_from(u.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::saturating_from(&u);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0");
    test("123", "123");
    test("-123", "0");
    test("1000000000000", "1000000000000");
    test("-1000000000000", "0");
    test("2147483647", "2147483647");
    test("2147483648", "2147483648");
    test("-2147483648", "0");
    test("-2147483649", "0");
}

#[test]
fn test_convertible_from_integer() {
    let test = |s, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(Natural::convertible_from(u.clone()), out);
        assert_eq!(Natural::convertible_from(&u), out);
    };
    test("0", true);
    test("123", true);
    test("-123", false);
    test("1000000000000", true);
    test("-1000000000000", false);
    test("2147483647", true);
    test("2147483648", true);
    test("-2147483648", false);
    test("-2147483649", false);
}

#[test]
fn try_from_integer_properties() {
    integer_gen().test_properties(|x| {
        let natural_x = Natural::try_from(x.clone());
        assert!(natural_x.as_ref().map_or(true, Natural::is_valid));

        let natural_x_alt = Natural::try_from(&x);
        assert!(natural_x_alt.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(natural_x, natural_x_alt);

        assert_eq!(natural_x.is_ok(), x >= 0);
        assert_eq!(natural_x.is_ok(), Natural::convertible_from(&x));
        if let Ok(n) = natural_x {
            assert_eq!(n.to_string(), x.to_string());
            assert_eq!(Natural::exact_from(&x), n);
            assert_eq!(Integer::from(&n), x);
            assert_eq!(Integer::from(n), x);
        }
    });
}

#[test]
fn saturating_from_integer_properties() {
    integer_gen().test_properties(|x| {
        let natural_x = Natural::saturating_from(x.clone());
        assert!(natural_x.is_valid());

        let natural_x_alt = Natural::saturating_from(&x);
        assert!(natural_x_alt.is_valid());
        assert_eq!(natural_x, natural_x_alt);

        assert_eq!(natural_x == 0, x <= 0);
        assert!(natural_x >= x);
        assert_eq!(natural_x == x, Natural::convertible_from(x));
    });
}

#[test]
fn convertible_from_integer_properties() {
    integer_gen().test_properties(|x| {
        let convertible = Natural::convertible_from(x.clone());
        assert_eq!(Natural::convertible_from(&x), convertible);
        assert_eq!(convertible, x >= 0);
    });
}
