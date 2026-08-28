// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, IsGaussianInteger};
use malachite_base::strings::ToDebugString;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::str::FromStr;

#[test]
fn test_try_from_gaussian_rational() {
    let test = |s, out| {
        let x = GaussianRational::from_str(s).unwrap();

        let og = GaussianInteger::try_from(x.clone());
        assert_eq!(og.to_debug_string(), out);

        let og = GaussianInteger::try_from(&x);
        assert_eq!(og.to_debug_string(), out);
    };
    test("0", "Ok(GaussianInteger { real: 0, imaginary: 0 })");
    test("123", "Ok(GaussianInteger { real: 123, imaginary: 0 })");
    test("-123", "Ok(GaussianInteger { real: -123, imaginary: 0 })");
    test("i", "Ok(GaussianInteger { real: 0, imaginary: 1 })");
    test("2-3i", "Ok(GaussianInteger { real: 2, imaginary: -3 })");
    test("22/7", "Err(GaussianIntegerFromGaussianRationalError)");
    test("i/2", "Err(GaussianIntegerFromGaussianRationalError)");
    test("2/3-5i/6", "Err(GaussianIntegerFromGaussianRationalError)");
    test("1+i/2", "Err(GaussianIntegerFromGaussianRationalError)");
    test("1/2+i", "Err(GaussianIntegerFromGaussianRationalError)");
}

#[test]
fn test_exact_from_gaussian_rational() {
    let test = |s, out| {
        let x = GaussianRational::from_str(s).unwrap();

        let g = GaussianInteger::exact_from(x.clone());
        assert_eq!(g.to_string(), out);

        let g = GaussianInteger::exact_from(&x);
        assert_eq!(g.to_string(), out);
    };
    test("0", "0");
    test("123", "123");
    test("-123", "-123");
    test("i", "i");
    test("2-3i", "2-3i");
}

#[test]
#[should_panic]
fn gaussian_integer_exact_from_gaussian_rational_fail() {
    GaussianInteger::exact_from(GaussianRational::from_str("22/7").unwrap());
}

#[test]
#[should_panic]
fn gaussian_integer_exact_from_gaussian_rational_ref_fail() {
    GaussianInteger::exact_from(&GaussianRational::from_str("i/2").unwrap());
}

#[test]
fn test_convertible_from_gaussian_rational() {
    let test = |s, out| {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(GaussianInteger::convertible_from(&x), out);
    };
    test("0", true);
    test("123", true);
    test("-123", true);
    test("i", true);
    test("2-3i", true);
    test("22/7", false);
    test("i/2", false);
    test("2/3-5i/6", false);
    test("1+i/2", false);
    test("1/2+i", false);
}

#[test]
fn try_from_gaussian_rational_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let og = GaussianInteger::try_from(x.clone());
        assert_eq!(GaussianInteger::try_from(&x), og);
        assert_eq!(og.is_ok(), x.is_gaussian_integer());
        assert_eq!(GaussianInteger::convertible_from(&x), og.is_ok());
        if let Ok(g) = og {
            assert_eq!(g.real, Integer::exact_from(&x.real));
            assert_eq!(g.imaginary, Integer::exact_from(&x.imaginary));
            assert_eq!(g.to_string(), x.to_string());
            assert_eq!(GaussianRational::from(g), x);
        }
    });

    gaussian_integer_gen().test_properties(|g| {
        assert_eq!(GaussianInteger::exact_from(GaussianRational::from(&g)), g);
    });
}
