// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate, DivI, MulI, MulIAssign};
use malachite_base::num::basic::traits::I;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::str::FromStr;

#[test]
fn test_mul_i() {
    let test = |s, out| {
        let x = GaussianRational::from_str(s).unwrap();

        let y = x.clone().mul_i();
        assert!(y.real.is_valid());
        assert!(y.imaginary.is_valid());
        assert_eq!(y.to_string(), out);

        let y = (&x).mul_i();
        assert!(y.real.is_valid());
        assert!(y.imaginary.is_valid());
        assert_eq!(y.to_string(), out);

        let mut y = x;
        y.mul_i_assign();
        assert_eq!(y.to_string(), out);
    };
    test("0", "0");
    test("1", "i");
    test("i", "-1");
    test("1/2", "i/2");
    test("i/2", "-1/2");
    test("1+i", "-1+i");
    test("1/2-2i/3", "2/3+i/2");
    test("-22/7+3i/5", "-3/5-22i/7");
}

#[test]
fn mul_i_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let y = x.clone().mul_i();
        assert!(y.real.is_valid());
        assert!(y.imaginary.is_valid());
        assert_eq!((&x).mul_i(), y);
        let mut x_alt = x.clone();
        x_alt.mul_i_assign();
        assert_eq!(x_alt, y);

        assert_eq!(y.real, -&x.imaginary);
        assert_eq!(y.imaginary, x.real);
        assert_eq!(y, &x * GaussianRational::I);
        assert_eq!((&y).div_i(), x);
        assert_eq!((&y).mul_i(), -&x);
        assert_eq!((&y).abs_squared(), (&x).abs_squared());
        assert_eq!((&y).conjugate(), (&x).conjugate().div_i());
        assert_eq!((-&x).mul_i(), -&y);
    });
}
