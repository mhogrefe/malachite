// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{gaussian_integer_gen, integer_gen};
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::rational_gen;

#[test]
fn test_from() {
    let test = |g: GaussianRational, out| {
        assert_eq!(g.to_string(), out);
        assert_eq!(g.imaginary, 0);
    };
    test(GaussianRational::from(0u8), "0");
    test(GaussianRational::from(123u8), "123");
    test(GaussianRational::from(-123i64), "-123");
    test(GaussianRational::from(false), "0");
    test(GaussianRational::from(true), "1");
    test(GaussianRational::from(Natural::from(123u32)), "123");
    test(GaussianRational::from(Integer::from(-123)), "-123");
    test(
        GaussianRational::from(Rational::from_signeds(-5, 6)),
        "-5/6",
    );
}

#[test]
fn test_from_gaussian_integer() {
    let test = |s| {
        let g = GaussianInteger::from_str(s).unwrap();
        assert_eq!(GaussianRational::from(&g).to_string(), s);
        assert_eq!(GaussianRational::from(g).to_string(), s);
    };
    test("0");
    test("1");
    test("-1");
    test("i");
    test("-i");
    test("2-3i");
    test("-2+3i");
    test("123456789012345678901234567890-987654321098765432109876543210i");
}

#[test]
fn from_properties() {
    rational_gen().test_properties(|x| {
        let g = GaussianRational::from(x.clone());
        assert_eq!(g.real, x);
        assert_eq!(g.imaginary, 0);
        assert_eq!(g.to_string(), x.to_string());
    });

    integer_gen().test_properties(|x| {
        let g = GaussianRational::from(x.clone());
        assert_eq!(g.real, x);
        assert_eq!(g.imaginary, Rational::ZERO);
    });

    gaussian_integer_gen().test_properties(|x| {
        let g = GaussianRational::from(&x);
        assert_eq!(g.real, x.real);
        assert_eq!(g.imaginary, x.imaginary);
        // integer-valued Gaussian rationals display exactly like Gaussian integers
        assert_eq!(g.to_string(), x.to_string());
        assert_eq!(GaussianRational::from(x), g);
    });
}
