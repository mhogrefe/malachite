// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, NegAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::test_util::generators::gaussian_integer_gen;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::generators::gaussian_rational_gen;
use std::str::FromStr;

#[test]
fn test_neg() {
    let test = |s, out| {
        let x = GaussianRational::from_str(s).unwrap();

        let neg = -x.clone();
        assert!(neg.real.is_valid());
        assert!(neg.imaginary.is_valid());
        assert_eq!(neg.to_string(), out);

        let neg = -&x;
        assert!(neg.real.is_valid());
        assert!(neg.imaginary.is_valid());
        assert_eq!(neg.to_string(), out);

        let mut neg = x;
        neg.neg_assign();
        assert_eq!(neg.to_string(), out);
    };
    test("0", "0");
    test("1", "-1");
    test("-1", "1");
    test("i", "-i");
    test("-i", "i");
    test("1+i", "-1-i");
    test("2-3i", "-2+3i");
    test("-2+3i", "2-3i");
    test("-123", "123");
    test("i/2", "-i/2");
    test("2/3-5i/6", "-2/3+5i/6");
}

#[test]
fn neg_properties() {
    gaussian_rational_gen().test_properties(|x| {
        let neg = -x.clone();
        assert!(neg.real.is_valid());
        assert!(neg.imaginary.is_valid());
        assert_eq!(-&x, neg);
        let mut neg_alt = x.clone();
        neg_alt.neg_assign();
        assert_eq!(neg_alt, neg);

        assert_eq!(neg.real, -&x.real);
        assert_eq!(neg.imaginary, -&x.imaginary);
        assert_eq!(-&neg, x);
        assert_eq!(neg == x, x == GaussianRational::ZERO);
        assert_eq!((&neg).abs_squared(), (&x).abs_squared());
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(GaussianRational::from(-&x), -GaussianRational::from(&x));
    });
}
