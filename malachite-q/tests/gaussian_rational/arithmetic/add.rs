// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Conjugate;
use malachite_base::num::basic::traits::Zero;
use malachite_base::vecs::vec_from_str;
use malachite_nz::test_util::generators::gaussian_integer_pair_gen;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::gaussian_rational::arithmetic::add::gaussian_rational_sum_naive;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_pair_gen, gaussian_rational_triple_gen,
    gaussian_rational_vec_gen, rational_vec_gen,
};
use std::iter::{Sum, once};
use std::str::FromStr;

#[test]
fn test_add() {
    let test = |s, t, out: &str| {
        let x = GaussianRational::from_str(s).unwrap();
        let y = GaussianRational::from_str(t).unwrap();

        let result = x.clone() + y.clone();
        assert!(result.real.is_valid());
        assert!(result.imaginary.is_valid());
        assert_eq!(result.to_string(), out);

        let result = x.clone() + &y;
        assert!(result.real.is_valid());
        assert!(result.imaginary.is_valid());
        assert_eq!(result.to_string(), out);

        let result = &x + y.clone();
        assert!(result.real.is_valid());
        assert!(result.imaginary.is_valid());
        assert_eq!(result.to_string(), out);

        let result = &x + &y;
        assert!(result.real.is_valid());
        assert!(result.imaginary.is_valid());
        assert_eq!(result.to_string(), out);

        let mut result = x.clone();
        result += y.clone();
        assert_eq!(result.to_string(), out);

        let mut result = x;
        result += &y;
        assert_eq!(result.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "2/3-5i/6", "2/3-5i/6");
    test("1/2", "i/2", "1/2+i/2");
    test("1/2+i/2", "1/3-i/3", "5/6+i/6");
    test("1/2+i/2", "-1/2-i/2", "0");
    test("2/3-5i/6", "1/3+5i/6", "1");
}

#[test]
fn add_properties() {
    gaussian_rational_pair_gen().test_properties(|(x, y)| {
        let sum = x.clone() + y.clone();
        assert!(sum.real.is_valid());
        assert!(sum.imaginary.is_valid());
        assert_eq!(x.clone() + &y, sum);
        assert_eq!(&x + y.clone(), sum);
        assert_eq!(&x + &y, sum);
        let mut sum_alt = x.clone();
        sum_alt += y.clone();
        assert_eq!(sum_alt, sum);
        let mut sum_alt = x.clone();
        sum_alt += &y;
        assert_eq!(sum_alt, sum);

        assert_eq!(sum.real, &x.real + &y.real);
        assert_eq!(sum.imaginary, &x.imaginary + &y.imaginary);
        assert_eq!(&y + &x, sum);
        assert_eq!(&sum - &y, x);
        assert_eq!(&sum - &x, y);
        assert_eq!(-&x + -&y, -&sum);
        assert_eq!((&x).conjugate() + (&y).conjugate(), (&sum).conjugate());
    });

    gaussian_rational_gen().test_properties(|x| {
        assert_eq!(&x + GaussianRational::ZERO, x);
        assert_eq!(GaussianRational::ZERO + &x, x);
        assert_eq!(&x + -&x, GaussianRational::ZERO);
    });

    gaussian_rational_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!((&x + &y) + &z, &x + (&y + &z));
    });

    gaussian_integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            GaussianRational::from(&x) + GaussianRational::from(&y),
            GaussianRational::from(&x + &y)
        );
    });
}

#[test]
fn test_sum() {
    let test = |xs, out: &str| {
        let xs = vec_from_str::<GaussianRational>(xs).unwrap();
        let sum = GaussianRational::sum(xs.iter().cloned());
        assert!(sum.real.is_valid());
        assert!(sum.imaginary.is_valid());
        assert_eq!(sum.to_string(), out);

        let sum_alt = GaussianRational::sum(xs.iter());
        assert!(sum_alt.real.is_valid());
        assert!(sum_alt.imaginary.is_valid());
        assert_eq!(sum_alt, sum);

        let sum_alt = gaussian_rational_sum_naive(xs.into_iter());
        assert!(sum_alt.real.is_valid());
        assert!(sum_alt.imaginary.is_valid());
        assert_eq!(sum_alt, sum);
    };
    test("[]", "0");
    test("[10]", "10");
    test("[i/2]", "i/2");
    test("[1/2+i/2, 1/3-i/3]", "5/6+i/6");
    test("[2, -3i, 5/3+i, 7/2-i/2]", "43/6-5i/2");
    test("[2/3-5i/6, i, 1/3-i/6]", "1");
    test("[22/7+i/3, -22/7+1000000000000i, i/3]", "3000000000002i/3");
}

#[test]
fn sum_properties() {
    gaussian_rational_vec_gen().test_properties(|xs| {
        let sum = GaussianRational::sum(xs.iter().cloned());
        assert!(sum.real.is_valid());
        assert!(sum.imaginary.is_valid());

        let sum_alt = GaussianRational::sum(xs.iter());
        assert!(sum_alt.real.is_valid());
        assert!(sum_alt.imaginary.is_valid());
        assert_eq!(sum_alt, sum);

        assert_eq!(sum.real, Rational::sum(xs.iter().map(|x| &x.real)));
        assert_eq!(
            sum.imaginary,
            Rational::sum(xs.iter().map(|x| &x.imaginary))
        );

        let sum_alt = gaussian_rational_sum_naive(xs.into_iter());
        assert!(sum_alt.real.is_valid());
        assert!(sum_alt.imaginary.is_valid());
        assert_eq!(sum_alt, sum);
    });

    gaussian_rational_gen().test_properties(|x| {
        assert_eq!(GaussianRational::sum(once(&x)), x);
        assert_eq!(GaussianRational::sum(once(x.clone())), x);
    });

    gaussian_rational_pair_gen().test_properties(|(x, y)| {
        let sum = &x + &y;
        assert_eq!(GaussianRational::sum([&x, &y].into_iter()), sum);
        assert_eq!(GaussianRational::sum([x, y].into_iter()), sum);
    });

    rational_vec_gen().test_properties(|xs| {
        assert_eq!(
            GaussianRational::sum(xs.iter().cloned().map(GaussianRational::from)),
            GaussianRational::from(Rational::sum(xs.into_iter()))
        );
    });
}
