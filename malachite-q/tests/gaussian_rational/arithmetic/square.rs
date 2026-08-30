// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate, Square, SquareAssign};
use malachite_nz::test_util::generators::gaussian_integer_gen;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::gaussian_rational::arithmetic::square::gaussian_rational_square_naive;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_gen_var_1, gaussian_rational_gen_var_2,
};
use std::str::FromStr;

#[test]
fn test_square() {
    let test = |s, out: &str| {
        let x = GaussianRational::from_str(s).unwrap();

        let square = x.clone().square();
        assert!(square.real.is_valid());
        assert!(square.imaginary.is_valid());
        assert_eq!(square.to_string(), out);

        let square = (&x).square();
        assert!(square.real.is_valid());
        assert!(square.imaginary.is_valid());
        assert_eq!(square.to_string(), out);

        let mut square = x;
        square.square_assign();
        assert_eq!(square.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("i", "-1");
    test("1/2", "1/4");
    test("i/2", "-1/4");
    test("1/2+i/2", "i/2");
    test("2/3-5i/6", "-1/4-10i/9");
}

#[test]
fn square_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let square = x.clone().square();
        assert!(square.real.is_valid());
        assert!(square.imaginary.is_valid());
        assert_eq!((&x).square(), square);
        let mut x_alt = x.clone();
        x_alt.square_assign();
        assert_eq!(x_alt, square);

        assert_eq!(&x * &x, square);
        assert_eq!(gaussian_rational_square_naive(&x), square);
        assert_eq!((-&x).square(), square);
        assert_eq!((&x).conjugate().square(), (&square).conjugate());
        assert_eq!((&square).abs_squared(), (&x).abs_squared().square());
    });

    // Purely real and purely imaginary values take the early-out paths.
    gaussian_rational_gen_var_1().test_properties(|x| {
        assert_eq!(gaussian_rational_square_naive(&x), (&x).square());
    });

    gaussian_rational_gen_var_2().test_properties(|x| {
        assert_eq!(gaussian_rational_square_naive(&x), (&x).square());
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(
            GaussianRational::from(&x).square(),
            GaussianRational::from((&x).square())
        );
    });
}
