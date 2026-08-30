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
    float_gaussian_rational_pair_gen, float_rational_pair_gen,
};
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::str::FromStr;

#[test]
fn test_partial_eq_gaussian_rational() {
    let test = |s, t, out| {
        let x = Float::from_str(s).unwrap();
        let y = GaussianRational::from_str(t).unwrap();
        assert_eq!(x == y, out);
        assert_eq!(y == x, out);
    };
    test("0.0", "0", true);
    test("-0.0", "0", true);
    test("123.0", "123", true);
    test("0.5", "1/2", true);
    test("-0.5", "1/2", false);
    test("0.5", "1/3", false);
    test("123.0", "123+i", false);
    test("0.5", "1/2+i", false);
    test("0.0", "i", false);
    test("NaN", "0", false);
    test("Infinity", "0", false);
}

#[allow(clippy::cmp_owned)]
#[test]
fn partial_eq_gaussian_rational_properties() {
    float_gaussian_rational_pair_gen().test_properties(|(x, y)| {
        let eq = x == y;
        assert_eq!(y == x, eq);
    });

    gaussian_rational_gen().test_properties(|x| {
        assert_ne!(x, Float::NAN);
        assert_ne!(Float::NAN, x);
        assert_ne!(x, Float::INFINITY);
        assert_ne!(x, Float::NEGATIVE_INFINITY);
    });

    float_rational_pair_gen().test_properties(|(x, y)| {
        assert_eq!(x == GaussianRational::from(y.clone()), x == y);
        assert_eq!(GaussianRational::from(y.clone()) == x, y == x);
    });
}
