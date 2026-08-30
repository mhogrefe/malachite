// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, AbsSquaredAssign, Square};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::{gaussian_integer_gen, gaussian_integer_gen_var_1};
use std::str::FromStr;

#[test]
fn test_abs_squared() {
    let test = |s, out| {
        let x = GaussianInteger::from_str(s).unwrap();

        let squared = x.clone().abs_squared();
        assert!(squared.is_valid());
        assert_eq!(squared.to_string(), out);

        let squared = (&x).abs_squared();
        assert!(squared.is_valid());
        assert_eq!(squared.to_string(), out);

        // The assign form leaves the purely real squared absolute value in place, which displays
        // identically.
        let mut squared = x;
        squared.abs_squared_assign();
        assert!(squared.real.is_valid());
        assert!(squared.imaginary.is_valid());
        assert_eq!(squared.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("-1", "1");
    test("i", "1");
    test("-i", "1");
    test("1+i", "2");
    test("2-3i", "13");
    test("-123", "15129");
    test("1000000000000i", "1000000000000000000000000");
}

#[test]
fn abs_squared_properties() {
    gaussian_integer_gen().test_properties(|x| {
        let abs_squared = x.clone().abs_squared();
        assert!(abs_squared.is_valid());
        assert_eq!((&x).abs_squared(), abs_squared);
        let mut x_alt = x.clone();
        x_alt.abs_squared_assign();
        assert_eq!(x_alt.real, abs_squared);
        assert_eq!(x_alt.imaginary, 0u32);
        assert_eq!((&x.real).square() + (&x.imaginary).square(), abs_squared);
        let conjugate = GaussianInteger {
            real: x.real.clone(),
            imaginary: -&x.imaginary,
        };
        assert_eq!(conjugate.abs_squared(), abs_squared);
        assert!(abs_squared >= 0u32);
        assert_eq!(abs_squared == 0u32, x == GaussianInteger::ZERO);
    });

    gaussian_integer_gen_var_1().test_properties(|x| {
        assert_eq!((&x).abs_squared(), (&x.real).abs_squared());
    });
}
