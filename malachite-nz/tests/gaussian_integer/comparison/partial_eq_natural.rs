// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{gaussian_integer_natural_pair_gen, natural_pair_gen};
use std::str::FromStr;

#[test]
fn test_partial_eq_natural() {
    let test = |s, t, out| {
        let x = GaussianInteger::from_str(s).unwrap();
        let y = Natural::from_str(t).unwrap();
        assert_eq!(x == y, out);
        assert_eq!(y == x, out);
    };
    test("0", "0", true);
    test("0", "5", false);
    test("123", "123", true);
    test("-123", "123", false);
    test("1000000000000", "1000000000000", true);
    test("1000000000000", "1000000000001", false);
    test("123+i", "123", false);
    test("i", "0", false);
}

#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_natural_properties() {
    gaussian_integer_natural_pair_gen().test_properties(|(x, y)| {
        let eq = x == y;
        assert_eq!(y == x, eq);
        assert_eq!(x == GaussianInteger::from(&y), eq);
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(GaussianInteger::from(&x) == y, x == y);
        assert_eq!(x == GaussianInteger::from(&y), x == y);
    });
}
