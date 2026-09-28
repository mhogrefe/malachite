// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{FloorSqrt, Height, Square};
use malachite_base::num::basic::traits::One;
use malachite_base::polynomial::{FloorL2Norm, L2NormSquared};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_polynomial_gen;

#[test]
fn test_floor_l2_norm() {
    let test = |s, out| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let n = (&p).floor_l2_norm();
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
    };
    test("0", "0");
    test("-5", "5");
    test("x^2-3*x+2", "3");
    // The norm is an integer.
    test("3*x-4", "5");
    test("-x^3+x", "1");
    // Coefficients of more than one limb.
    test("1000000000000000000000*x-1", "1000000000000000000000");
}

#[test]
fn floor_l2_norm_properties() {
    integer_polynomial_gen().test_properties(|p| {
        let f = (&p).floor_l2_norm();
        assert!(f.is_valid());
        let n = (&p).l2_norm_squared();
        // It is the floor of the square root of the square of the norm.
        assert_eq!((&n).floor_sqrt(), f);
        assert!((&f).square() <= n);
        assert!((&f + Natural::ONE).square() > n);
        // It is at least the height.
        assert!(p.to_height() <= f);
        assert_eq!((-&p).floor_l2_norm(), f);
    });
}
