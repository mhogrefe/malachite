// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_float::BigExponentFloat;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use std::str::FromStr;

#[test]
fn test_try_from_gaussian_integer() {
    let test = |s, out: Option<&str>| {
        let z = GaussianInteger::from_str(s).unwrap();
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
    test("-3", Some("-0x3.0"));
    test("1+i", None);
    test("i", None);
}

#[test]
fn try_from_gaussian_integer_properties() {
    gaussian_integer_gen().test_properties(|z| {
        let result = BigExponentFloat::try_from(&z);
        assert_eq!(result.is_ok(), BigExponentFloat::convertible_from(&z));
        if let Ok(x) = result {
            assert!(x.is_valid());
            assert_eq!(
                format!("{x:#x}"),
                format!("{:#x}", BigExponentFloat::from(&z.real))
            );
        }
    });
}
