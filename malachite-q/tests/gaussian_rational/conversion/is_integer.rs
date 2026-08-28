// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{IsGaussianInteger, IsInteger, IsReal};
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::str::FromStr;

#[test]
fn test_is_integer() {
    let test = |s, out| {
        assert_eq!(GaussianRational::from_str(s).unwrap().is_integer(), out);
    };
    test("0", true);
    test("1", true);
    test("100", true);
    test("-100", true);
    test("1/2", false);
    test("-22/7", false);
    test("i", false);
    test("i/2", false);
    test("2/3-5i/6", false);
    test("2+3i", false);
}

#[test]
fn is_integer_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let is_integer = x.is_integer();
        assert_eq!(is_integer, x.imaginary == 0u32 && (&x.real).is_integer());
        assert_eq!(is_integer, x.is_gaussian_integer() && x.is_real());
    });
}
