// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Conjugate;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::test_util::generators::{gaussian_integer_gen, gaussian_integer_pair_gen};
use std::str::FromStr;

#[test]
fn test_sub() {
    let test = |s, t, out: &str| {
        let x = GaussianInteger::from_str(s).unwrap();
        let y = GaussianInteger::from_str(t).unwrap();

        let result = x.clone() - y.clone();
        assert!(result.real.is_valid());
        assert!(result.imaginary.is_valid());
        assert_eq!(result.to_string(), out);

        let result = x.clone() - &y;
        assert!(result.real.is_valid());
        assert!(result.imaginary.is_valid());
        assert_eq!(result.to_string(), out);

        let result = &x - y.clone();
        assert!(result.real.is_valid());
        assert!(result.imaginary.is_valid());
        assert_eq!(result.to_string(), out);

        let result = &x - &y;
        assert!(result.real.is_valid());
        assert!(result.imaginary.is_valid());
        assert_eq!(result.to_string(), out);

        let mut result = x.clone();
        result -= y.clone();
        assert_eq!(result.to_string(), out);

        let mut result = x;
        result -= &y;
        assert_eq!(result.to_string(), out);
    };
    test("0", "0", "0");
    test("2-3i", "0", "2-3i");
    test("0", "2-3i", "-2+3i");
    test("1", "i", "1-i");
    test("2-3i", "-1+4i", "3-7i");
    test("2-3i", "2-3i", "0");
    test("1000000000000+i", "i", "1000000000000");
    test("123+456i", "23+56i", "100+400i");
}

#[test]
fn sub_properties() {
    gaussian_integer_pair_gen().test_properties(|(x, y)| {
        let diff = x.clone() - y.clone();
        assert!(diff.real.is_valid());
        assert!(diff.imaginary.is_valid());
        assert_eq!(x.clone() - &y, diff);
        assert_eq!(&x - y.clone(), diff);
        assert_eq!(&x - &y, diff);
        let mut diff_alt = x.clone();
        diff_alt -= y.clone();
        assert_eq!(diff_alt, diff);
        let mut diff_alt = x.clone();
        diff_alt -= &y;
        assert_eq!(diff_alt, diff);

        assert_eq!(diff.real, &x.real - &y.real);
        assert_eq!(diff.imaginary, &x.imaginary - &y.imaginary);
        assert_eq!(-(&y - &x), diff);
        assert_eq!(&x + -&y, diff);
        assert_eq!(&diff + &y, x);
        assert_eq!((&x).conjugate() - (&y).conjugate(), (&diff).conjugate());
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(&x - GaussianInteger::ZERO, x);
        assert_eq!(GaussianInteger::ZERO - &x, -&x);
        assert_eq!(&x - &x, GaussianInteger::ZERO);
    });
}
