// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate, IsUnit, MulI};
use malachite_base::num::basic::traits::{I, NegativeI, Zero};
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use std::str::FromStr;

#[test]
fn test_is_unit() {
    let test = |s, out| {
        assert_eq!(GaussianInteger::from_str(s).unwrap().is_unit(), out);
    };
    test("0", false);
    test("1", true);
    test("-1", true);
    test("i", true);
    test("-i", true);
    test("2", false);
    test("-2", false);
    test("2i", false);
    test("1+i", false);
    test("1-i", false);
    test("3+4i", false);
}

#[test]
fn is_unit_properties() {
    gaussian_integer_gen().test_properties(|x| {
        let is_unit = x.is_unit();
        assert_eq!(x.is_unit(), (&x).abs_squared() == 1u32);
        assert_eq!(
            x.is_unit(),
            x == 1u32 || x == -1i32 || x == GaussianInteger::I || x == GaussianInteger::NEGATIVE_I
        );
        assert_eq!((-&x).is_unit(), is_unit);
        assert_eq!((&x).conjugate().is_unit(), is_unit);
        assert_eq!((&x).mul_i().is_unit(), is_unit);
        if is_unit {
            assert_ne!(x, GaussianInteger::ZERO);
        }
    });
}
