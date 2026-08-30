// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{gaussian_rational_rational_pair_gen, rational_pair_gen};
use std::str::FromStr;

#[test]
fn test_partial_eq_rational() {
    let test = |s, t, out| {
        let x = GaussianRational::from_str(s).unwrap();
        let y = Rational::from_str(t).unwrap();
        assert_eq!(x == y, out);
        assert_eq!(y == x, out);
    };
    test("0", "0", true);
    test("123", "123", true);
    test("22/7", "22/7", true);
    test("22/7", "-22/7", false);
    test("22/7+i", "22/7", false);
    test("i", "0", false);
    test("1/2", "2", false);
}

#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_rational_properties() {
    gaussian_rational_rational_pair_gen().test_properties(|(x, y)| {
        let eq = x == y;
        assert_eq!(y == x, eq);
        assert_eq!(x == GaussianRational::from(y.clone()), eq);
    });

    rational_pair_gen().test_properties(|(x, y)| {
        assert_eq!(GaussianRational::from(x.clone()) == y, x == y);
        assert_eq!(x == GaussianRational::from(y.clone()), x == y);
    });
}
