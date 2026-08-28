// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, IsReal};
use malachite_float::test_util::common::to_hex_string;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::str::FromStr;

#[test]
fn test_try_from_gaussian_rational() {
    let test = |s, out, out_hex| {
        let x = GaussianRational::from_str(s).unwrap();

        let f = Float::try_from(x.clone()).unwrap();
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);

        let f = Float::try_from(&x).unwrap();
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
    };
    test("0", "0.0", "0x0.0");
    test("1", "1.0", "0x1.0#1");
    test("123", "123.0", "0x7b.0#7");
    test("-123", "-123.0", "-0x7b.0#7");
    test("1/2", "0.50", "0x0.8#1");
    test("-5/2", "-2.5", "-0x2.8#3");

    let test_err = |s| {
        let x = GaussianRational::from_str(s).unwrap();
        assert!(Float::try_from(x.clone()).is_err());
        assert!(Float::try_from(&x).is_err());
        assert!(!Float::convertible_from(&x));
    };
    test_err("1/3");
    test_err("-22/7");
    test_err("i");
    test_err("i/2");
    test_err("2-3i");
    test_err("2/3-5i/6");
}

#[test]
#[should_panic]
fn float_exact_from_gaussian_rational_fail() {
    Float::exact_from(GaussianRational::from_str("1/3").unwrap());
}

#[test]
#[should_panic]
fn float_exact_from_gaussian_rational_ref_fail() {
    Float::exact_from(&GaussianRational::from_str("i/2").unwrap());
}

#[test]
fn test_convertible_from_gaussian_rational() {
    let test = |s, out| {
        let x = GaussianRational::from_str(s).unwrap();
        assert_eq!(Float::convertible_from(&x), out);
    };
    test("0", true);
    test("123", true);
    test("-123", true);
    test("1/2", true);
    test("-5/2", true);
    test("1/3", false);
    test("-22/7", false);
    test("i", false);
    test("i/2", false);
    test("2-3i", false);
}

#[test]
fn try_from_gaussian_rational_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let of = Float::try_from(x.clone());
        assert!(of.as_ref().map_or(true, Float::is_valid));
        let of_ref = Float::try_from(&x);
        assert_eq!(
            of.as_ref().ok().map(ComparableFloatRef),
            of_ref.as_ref().ok().map(ComparableFloatRef)
        );
        assert_eq!(of.is_ok(), x.is_real() && Float::convertible_from(&x.real));
        assert_eq!(Float::convertible_from(&x), of.is_ok());
        if let Ok(f) = of {
            assert_eq!(
                ComparableFloat(f.clone()),
                ComparableFloat(Float::exact_from(&x.real))
            );
            assert_eq!(GaussianRational::exact_from(&f), x);
        }
    });
}
