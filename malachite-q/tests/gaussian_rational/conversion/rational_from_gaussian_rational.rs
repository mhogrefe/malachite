// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, IsReal};
use malachite_base::strings::ToDebugString;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{gaussian_rational_gen, gaussian_rational_gen_var_1};
use std::str::FromStr;

#[test]
fn test_try_from_gaussian_rational() {
    let test = |s, out| {
        let x = GaussianRational::from_str(s).unwrap();

        let on = Rational::try_from(x.clone());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Rational::try_from(&x);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("0", "Ok(0)");
    test("123", "Ok(123)");
    test("-123", "Ok(-123)");
    test("22/7", "Ok(22/7)");
    test("-22/7", "Ok(-22/7)");
    test("i", "Err(RationalFromGaussianRationalError)");
    test("i/2", "Err(RationalFromGaussianRationalError)");
    test("2-3i", "Err(RationalFromGaussianRationalError)");
    test("2/3-5i/6", "Err(RationalFromGaussianRationalError)");
}

#[test]
fn test_exact_from_gaussian_rational() {
    let test = |s, out| {
        let x = GaussianRational::from_str(s).unwrap();

        let n = Rational::exact_from(x.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Rational::exact_from(&x);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0");
    test("123", "123");
    test("-123", "-123");
    test("22/7", "22/7");
    test("-22/7", "-22/7");
}

#[test]
#[should_panic]
fn rational_exact_from_gaussian_rational_fail() {
    Rational::exact_from(GaussianRational::from_str("2-3i").unwrap());
}

#[test]
#[should_panic]
fn rational_exact_from_gaussian_rational_ref_fail() {
    Rational::exact_from(&GaussianRational::from_str("i/2").unwrap());
}

#[test]
fn test_convertible_from_gaussian_rational() {
    let test = |s, out| {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(Rational::convertible_from(&x), out);
    };
    test("0", true);
    test("123", true);
    test("-123", true);
    test("22/7", true);
    test("-22/7", true);
    test("i", false);
    test("i/2", false);
    test("2-3i", false);
    test("2/3-5i/6", false);
}

#[test]
fn try_from_gaussian_rational_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let on = Rational::try_from(x.clone());
        assert!(on.as_ref().map_or(true, Rational::is_valid));
        assert_eq!(Rational::try_from(&x), on);
        assert_eq!(on.is_ok(), x.is_real());
        assert_eq!(Rational::convertible_from(&x), on.is_ok());
        if let Ok(n) = on {
            assert_eq!(n, x.real);
            assert_eq!(GaussianRational::from(n), x);
        }
    });

    gaussian_rational_gen_var_1().test_properties(|x| {
        assert!(Rational::convertible_from(&x));
        assert_eq!(GaussianRational::from(Rational::exact_from(&x)), x);
    });
}
