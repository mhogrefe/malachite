// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate, DivI, DivIAssign, MulI};
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use std::str::FromStr;

#[test]
fn test_div_i() {
    let test = |s, out| {
        let x = GaussianInteger::from_str(s).unwrap();

        let y = x.clone().div_i();
        assert!(y.real.is_valid());
        assert!(y.imaginary.is_valid());
        assert_eq!(y.to_string(), out);

        let y = (&x).div_i();
        assert!(y.real.is_valid());
        assert!(y.imaginary.is_valid());
        assert_eq!(y.to_string(), out);

        let mut y = x;
        y.div_i_assign();
        assert_eq!(y.to_string(), out);
    };
    test("0", "0");
    test("1", "-i");
    test("i", "1");
    test("-1", "i");
    test("-i", "-1");
    test("1+i", "1-i");
    test("2-3i", "-3-2i");
    test("-123+456i", "456+123i");
}

#[test]
fn div_i_properties() {
    gaussian_integer_gen().test_properties(|x| {
        let y = x.clone().div_i();
        assert!(y.real.is_valid());
        assert!(y.imaginary.is_valid());
        assert_eq!((&x).div_i(), y);
        let mut x_alt = x.clone();
        x_alt.div_i_assign();
        assert_eq!(x_alt, y);

        assert_eq!(y.real, x.imaginary);
        assert_eq!(y.imaginary, -&x.real);
        assert_eq!(y, -(&x).mul_i());
        assert_eq!((&y).mul_i(), x);
        assert_eq!((&y).div_i(), -&x);
        assert_eq!((&y).abs_squared(), (&x).abs_squared());
        assert_eq!((&y).conjugate(), (&x).conjugate().mul_i());
        assert_eq!((-&x).div_i(), -&y);
    });
}
