// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Conjugate;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::test_util::generators::gaussian_integer_pair_gen;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::{gaussian_rational_gen, gaussian_rational_pair_gen};
use std::str::FromStr;

#[test]
fn test_sub() {
    let test = |s, t, out: &str| {
        let x = GaussianRational::from_str(s).unwrap();
        let y = GaussianRational::from_str(t).unwrap();

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
    test("2/3-5i/6", "0", "2/3-5i/6");
    test("0", "2/3-5i/6", "-2/3+5i/6");
    test("1/2+i/2", "1/3-i/3", "1/6+5i/6");
    test("1/2+i/2", "1/2+i/2", "0");
    test("2/3-5i/6", "-1/3-5i/6", "1");
}

#[test]
fn sub_properties() {
    gaussian_rational_pair_gen().test_properties(|(x, y)| {
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

    gaussian_rational_gen().test_properties(|x| {
        assert_eq!(&x - GaussianRational::ZERO, x);
        assert_eq!(GaussianRational::ZERO - &x, -&x);
        assert_eq!(&x - &x, GaussianRational::ZERO);
    });

    gaussian_integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            GaussianRational::from(&x) - GaussianRational::from(&y),
            GaussianRational::from(&x - &y)
        );
    });
}
