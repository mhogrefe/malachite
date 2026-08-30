// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
use malachite_float::Float;
use malachite_float::test_util::generators::{
    float_gaussian_integer_pair_gen, float_integer_pair_gen,
};
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use malachite_q::gaussian_rational::GaussianRational;
use std::str::FromStr;

#[test]
fn test_partial_eq_gaussian_integer() {
    let test = |s, t, out| {
        let x = Float::from_str(s).unwrap();
        let y = GaussianInteger::from_str(t).unwrap();
        assert_eq!(x == y, out);
        assert_eq!(y == x, out);
    };
    test("0.0", "0", true);
    test("-0.0", "0", true);
    test("123.0", "123", true);
    test("-123.0", "-123", true);
    test("-123.0", "123", false);
    test("0.5", "0", false);
    test("123.0", "123+i", false);
    test("0.0", "i", false);
    test("NaN", "0", false);
    test("Infinity", "0", false);
    test("-Infinity", "0", false);
}

#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_gaussian_integer_properties() {
    float_gaussian_integer_pair_gen().test_properties(|(x, y)| {
        let eq = x == y;
        assert_eq!(y == x, eq);
        assert_eq!(x == GaussianRational::from(&y), eq);
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_ne!(x, Float::NAN);
        assert_ne!(Float::NAN, x);
        assert_ne!(x, Float::INFINITY);
        assert_ne!(x, Float::NEGATIVE_INFINITY);
    });

    float_integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(x == GaussianInteger::from(y.clone()), x == y);
        assert_eq!(GaussianInteger::from(y.clone()) == x, y == x);
    });
}
