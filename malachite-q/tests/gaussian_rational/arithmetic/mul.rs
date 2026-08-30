// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate};
use malachite_base::num::basic::traits::{I, NegativeOne, One, Zero};
use malachite_base::vecs::vec_from_str;
use malachite_nz::test_util::generators::gaussian_integer_pair_gen;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::test_util::gaussian_rational::arithmetic::mul::gaussian_rational_product_naive;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, gaussian_rational_pair_gen, gaussian_rational_triple_gen,
    gaussian_rational_vec_gen, rational_vec_gen,
};
use std::iter::{Product, once};
use std::str::FromStr;

#[test]
fn test_mul() {
    let test = |s, t, out: &str| {
        let x = GaussianRational::from_str(s).unwrap();
        let y = GaussianRational::from_str(t).unwrap();

        let product = x.clone() * y.clone();
        assert!(product.real.is_valid());
        assert!(product.imaginary.is_valid());
        assert_eq!(product.to_string(), out);

        let product = x.clone() * &y;
        assert!(product.real.is_valid());
        assert!(product.imaginary.is_valid());
        assert_eq!(product.to_string(), out);

        let product = &x * y.clone();
        assert!(product.real.is_valid());
        assert!(product.imaginary.is_valid());
        assert_eq!(product.to_string(), out);

        let product = &x * &y;
        assert!(product.real.is_valid());
        assert!(product.imaginary.is_valid());
        assert_eq!(product.to_string(), out);

        let mut product = x.clone();
        product *= y.clone();
        assert_eq!(product.to_string(), out);

        let mut product = x;
        product *= &y;
        assert_eq!(product.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "2/3-5i/6", "0");
    test("1", "2/3-5i/6", "2/3-5i/6");
    test("i", "i", "-1");
    test("1/2+i/2", "1/3-i/3", "1/3");
    test("1/2+i/2", "1/2-i/2", "1/2");
    test("2/3-5i/6", "2/3+5i/6", "41/36");
    test("i/2", "i/2", "-1/4");
}

#[test]
fn mul_properties() {
    gaussian_rational_pair_gen().test_properties(|(x, y)| {
        let product = x.clone() * y.clone();
        assert!(product.real.is_valid());
        assert!(product.imaginary.is_valid());
        assert_eq!(x.clone() * &y, product);
        assert_eq!(&x * y.clone(), product);
        assert_eq!(&x * &y, product);
        let mut product_alt = x.clone();
        product_alt *= y.clone();
        assert_eq!(product_alt, product);
        let mut product_alt = x.clone();
        product_alt *= &y;
        assert_eq!(product_alt, product);

        assert_eq!(
            product.real,
            &x.real * &y.real - &x.imaginary * &y.imaginary
        );
        assert_eq!(
            product.imaginary,
            &x.real * &y.imaginary + &x.imaginary * &y.real
        );
        assert_eq!(&y * &x, product);
        assert_eq!(-&x * &y, -&product);
        assert_eq!((&x).conjugate() * (&y).conjugate(), (&product).conjugate());
        assert_eq!(
            (&product).abs_squared(),
            (&x).abs_squared() * (&y).abs_squared()
        );
    });

    gaussian_rational_gen().test_properties(|x| {
        // Aliased references are detected and routed through the squaring algorithm.
        assert_eq!(&x * &x, x.clone() * x.clone());
        assert_eq!(&x * GaussianRational::ONE, x);
        assert_eq!(GaussianRational::ONE * &x, x);
        assert_eq!(&x * GaussianRational::ZERO, GaussianRational::ZERO);
        assert_eq!(GaussianRational::ZERO * &x, GaussianRational::ZERO);
        assert_eq!(&x * GaussianRational::NEGATIVE_ONE, -&x);
        // Multiplication by i rotates by a quarter turn.
        let rotated = &x * GaussianRational::I;
        assert_eq!(rotated.real, -&x.imaginary);
        assert_eq!(rotated.imaginary, x.real.clone());
        // x times its conjugate is its squared absolute value.
        assert_eq!(
            &x * (&x).conjugate(),
            GaussianRational::from((&x).abs_squared())
        );
    });

    gaussian_rational_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!((&x * &y) * &z, &x * (&y * &z));
        assert_eq!(&x * (&y + &z), &x * &y + &x * &z);
        assert_eq!((&x + &y) * &z, &x * &z + &y * &z);
    });

    gaussian_integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            GaussianRational::from(&x) * GaussianRational::from(&y),
            GaussianRational::from(&x * &y)
        );
    });
}

#[test]
fn test_product() {
    let test = |xs, out: &str| {
        let xs = vec_from_str::<GaussianRational>(xs).unwrap();
        let product = GaussianRational::product(xs.iter().cloned());
        assert!(product.real.is_valid());
        assert!(product.imaginary.is_valid());
        assert_eq!(product.to_string(), out);

        let product_alt = GaussianRational::product(xs.iter());
        assert!(product_alt.real.is_valid());
        assert!(product_alt.imaginary.is_valid());
        assert_eq!(product_alt, product);

        let product_alt = gaussian_rational_product_naive(xs.into_iter());
        assert!(product_alt.real.is_valid());
        assert!(product_alt.imaginary.is_valid());
        assert_eq!(product_alt, product);
    };
    test("[]", "1");
    test("[10]", "10");
    test("[i/2]", "i/2");
    test("[i/2, i/2]", "-1/4");
    test("[i/2, i/2, 2i]", "-i/2");
    test("[1/2+i/2, 1/3-i/3]", "1/3");
    test("[2, -3i, 5/3+i, 7/2-i/2]", "16-38i");
    test("[2/3-5i/6, 0, i]", "0");
}

#[test]
fn product_properties() {
    gaussian_rational_vec_gen().test_properties(|xs| {
        let product = GaussianRational::product(xs.iter().cloned());
        assert!(product.real.is_valid());
        assert!(product.imaginary.is_valid());

        let product_alt = GaussianRational::product(xs.iter());
        assert!(product_alt.real.is_valid());
        assert!(product_alt.imaginary.is_valid());
        assert_eq!(product_alt, product);

        // Conjugation distributes over products.
        assert_eq!(
            GaussianRational::product(xs.iter().map(Conjugate::conjugate)),
            (&product).conjugate()
        );
        // The norm is multiplicative.
        assert_eq!(
            Rational::product(xs.iter().map(AbsSquared::abs_squared)),
            (&product).abs_squared()
        );

        let product_alt = gaussian_rational_product_naive(xs.into_iter());
        assert!(product_alt.real.is_valid());
        assert!(product_alt.imaginary.is_valid());
        assert_eq!(product_alt, product);
    });

    gaussian_rational_gen().test_properties(|x| {
        assert_eq!(GaussianRational::product(once(&x)), x);
        assert_eq!(GaussianRational::product(once(x.clone())), x);
    });

    gaussian_rational_pair_gen().test_properties(|(x, y)| {
        let product = &x * &y;
        assert_eq!(GaussianRational::product([&x, &y].into_iter()), product);
        assert_eq!(GaussianRational::product([x, y].into_iter()), product);
    });

    rational_vec_gen().test_properties(|xs| {
        assert_eq!(
            GaussianRational::product(xs.iter().cloned().map(GaussianRational::from)),
            GaussianRational::from(Rational::product(xs.into_iter()))
        );
    });
}
