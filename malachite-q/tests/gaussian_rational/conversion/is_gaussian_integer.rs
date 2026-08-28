// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{IsGaussianInteger, IsInteger};
use malachite_nz::test_util::generators::gaussian_integer_gen;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::str::FromStr;

#[test]
fn test_is_gaussian_integer() {
    let test = |s, out| {
        assert_eq!(
            GaussianRational::from_str(s).unwrap().is_gaussian_integer(),
            out
        );
    };
    test("0", true);
    test("1", true);
    test("-100", true);
    test("i", true);
    test("2+3i", true);
    test("1/2", false);
    test("i/2", false);
    test("2/3-5i/6", false);
    test("1+i/2", false);
    test("1/2+i", false);
}

#[test]
fn is_gaussian_integer_properties() {
    gaussian_rational_gen().test_properties(|x| {
        assert_eq!(
            x.is_gaussian_integer(),
            (&x.real).is_integer() && (&x.imaginary).is_integer()
        );
    });

    gaussian_integer_gen().test_properties(|x| {
        assert!(GaussianRational::from(x).is_gaussian_integer());
    });
}
