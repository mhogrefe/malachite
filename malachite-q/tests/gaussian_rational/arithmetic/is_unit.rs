// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Conjugate, IsUnit, MulI};
use malachite_base::num::basic::traits::Zero;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::str::FromStr;

#[test]
fn test_is_unit() {
    let test = |s, out| {
        assert_eq!(GaussianRational::from_str(s).unwrap().is_unit(), out);
    };
    test("0", false);
    test("1", true);
    test("-1", true);
    test("i", true);
    test("-i", true);
    test("2", true);
    test("-2", true);
    test("2i", true);
    test("1+i", true);
    test("1-i", true);
    test("3+4i", true);
    test("1/2", true);
    test("i/2", true);
    test("1/2+i/3", true);
    test("22/7-3i/5", true);
}

#[test]
fn is_unit_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let is_unit = x.is_unit();
        assert_eq!(x.is_unit(), x != 0u32);
        assert_eq!((-&x).is_unit(), is_unit);
        assert_eq!((&x).conjugate().is_unit(), is_unit);
        assert_eq!((&x).mul_i().is_unit(), is_unit);
        if is_unit {
            assert_ne!(x, GaussianRational::ZERO);
        }
    });
}
