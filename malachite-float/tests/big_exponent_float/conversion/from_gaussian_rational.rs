// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_float::BigExponentFloat;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::str::FromStr;

#[test]
fn test_try_from_gaussian_rational() {
    let test = |s, out: Option<&str>| {
        let z = GaussianRational::from_str(s).unwrap();
        assert_eq!(
            BigExponentFloat::try_from(&z)
                .ok()
                .map(|x| format!("{x:#x}"))
                .as_deref(),
            out
        );
        assert_eq!(BigExponentFloat::convertible_from(&z), out.is_some());
        assert_eq!(BigExponentFloat::try_from(z).is_ok(), out.is_some());
    };
    test("0", Some("0x0.0"));
    test("-3/8", Some("-0x0.6"));
    test("1/3", None);
    test("i", None);
}

#[test]
fn try_from_gaussian_rational_properties() {
    gaussian_rational_gen().test_properties(|z| {
        let result = BigExponentFloat::try_from(&z);
        assert_eq!(result.is_ok(), BigExponentFloat::convertible_from(&z));
        if let Ok(x) = result {
            assert!(x.is_valid());
            assert_eq!(
                format!("{x:#x}"),
                format!("{:#x}", BigExponentFloat::try_from(&z.real).unwrap())
            );
        }
    });
}
