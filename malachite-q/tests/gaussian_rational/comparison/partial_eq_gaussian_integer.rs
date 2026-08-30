// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::gaussian_integer_pair_gen;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gaussian_integer_pair_gen;
use std::str::FromStr;

#[test]
fn test_partial_eq_gaussian_integer() {
    let test = |s, t, out| {
        let x = GaussianRational::from_str(s).unwrap();
        let y = GaussianInteger::from_str(t).unwrap();
        assert_eq!(x == y, out);
        assert_eq!(y == x, out);
    };
    test("0", "0", true);
    test("123", "123", true);
    test("123+i", "123+i", true);
    test("123+i", "123", false);
    test("123+i/2", "123+i", false);
    test("22/7", "3", false);
    test("-2-3i", "-2-3i", true);
    test("-2-3i", "-2+3i", false);
}

#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_gaussian_integer_properties() {
    gaussian_rational_gaussian_integer_pair_gen().test_properties(|(x, y)| {
        let eq = x == y;
        assert_eq!(y == x, eq);
        assert_eq!(x == GaussianRational::from(&y), eq);
        assert_eq!(eq, x.real == y.real && x.imaginary == y.imaginary);
    });

    gaussian_integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(GaussianRational::from(&x) == y, x == y);
        assert_eq!(x == GaussianRational::from(&y), x == y);
    });
}
