// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, IsInteger};
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::Natural;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::str::FromStr;

#[test]
fn test_try_from_gaussian_rational() {
    let test = |s, out| {
        let x = GaussianRational::from_str(s).unwrap();

        let on = Natural::try_from(x.clone());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Natural::try_from(&x);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("0", "Ok(0)");
    test("123", "Ok(123)");
    test("1000000000000", "Ok(1000000000000)");
    test("-123", "Err(NaturalFromGaussianRationalError)");
    test("22/7", "Err(NaturalFromGaussianRationalError)");
    test("-22/7", "Err(NaturalFromGaussianRationalError)");
    test("i", "Err(NaturalFromGaussianRationalError)");
    test("i/2", "Err(NaturalFromGaussianRationalError)");
    test("2-3i", "Err(NaturalFromGaussianRationalError)");
    test("2/3-5i/6", "Err(NaturalFromGaussianRationalError)");
}

#[test]
fn test_exact_from_gaussian_rational() {
    let test = |s, out| {
        let x = GaussianRational::from_str(s).unwrap();

        let n = Natural::exact_from(x.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::exact_from(&x);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0");
    test("123", "123");
    test("1000000000000", "1000000000000");
}

#[test]
#[should_panic]
fn natural_exact_from_gaussian_rational_fail() {
    Natural::exact_from(GaussianRational::from_str("-123").unwrap());
}

#[test]
#[should_panic]
fn natural_exact_from_gaussian_rational_ref_fail() {
    Natural::exact_from(&GaussianRational::from_str("22/7").unwrap());
}

#[test]
fn test_convertible_from_gaussian_rational() {
    let test = |s, out| {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(Natural::convertible_from(&x), out);
    };
    test("0", true);
    test("123", true);
    test("1000000000000", true);
    test("-123", false);
    test("22/7", false);
    test("-22/7", false);
    test("i", false);
    test("i/2", false);
    test("2-3i", false);
    test("2/3-5i/6", false);
}

#[test]
fn try_from_gaussian_rational_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let on = Natural::try_from(x.clone());
        assert!(on.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(Natural::try_from(&x), on);
        assert_eq!(on.is_ok(), x.is_integer() && x.real >= 0u32);
        assert_eq!(Natural::convertible_from(&x), on.is_ok());
        if let Ok(n) = on {
            assert_eq!(x.real, n);
            assert_eq!(GaussianRational::from(n), x);
        }
    });
}
