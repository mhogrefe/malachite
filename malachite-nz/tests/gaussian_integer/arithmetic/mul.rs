// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Conjugate, PowerOf2};
use malachite_base::num::basic::traits::{I, NegativeOne, One, Zero};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::gaussian_integer::arithmetic::mul::gaussian_integer_mul_naive;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, gaussian_integer_pair_gen, gaussian_integer_triple_gen,
};
use std::str::FromStr;

#[test]
fn test_mul() {
    let test = |s, t, out: &str| {
        let x = GaussianInteger::from_str(s).unwrap();
        let y = GaussianInteger::from_str(t).unwrap();

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
    test("0", "2-3i", "0");
    test("1", "2-3i", "2-3i");
    test("i", "i", "-1");
    test("i", "2-3i", "3+2i");
    test("2-3i", "-1+4i", "10+11i");
    test("2-3i", "2+3i", "13");
    test("1+i", "1-i", "2");
    test("1+i", "1+i", "2i");
    test("1000000000000", "i", "1000000000000i");
}

#[test]
fn mul_properties() {
    gaussian_integer_pair_gen().test_properties(|(x, y)| {
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
        assert_eq!(gaussian_integer_mul_naive(&x, &y), product);
        assert_eq!(&y * &x, product);
        assert_eq!(-&x * &y, -&product);
        assert_eq!((&x).conjugate() * (&y).conjugate(), (&product).conjugate());
        assert_eq!(
            (&product).abs_squared(),
            (&x).abs_squared() * (&y).abs_squared()
        );
    });

    gaussian_integer_gen().test_properties(|x| {
        assert_eq!(&x * GaussianInteger::ONE, x);
        assert_eq!(GaussianInteger::ONE * &x, x);
        assert_eq!(&x * GaussianInteger::ZERO, GaussianInteger::ZERO);
        assert_eq!(GaussianInteger::ZERO * &x, GaussianInteger::ZERO);
        assert_eq!(&x * GaussianInteger::NEGATIVE_ONE, -&x);
        // Multiplication by i rotates by a quarter turn.
        let rotated = &x * GaussianInteger::I;
        assert_eq!(rotated.real, -&x.imaginary);
        assert_eq!(rotated.imaginary, x.real.clone());
        // x times its conjugate is its squared absolute value.
        assert_eq!(
            &x * (&x).conjugate(),
            GaussianInteger::from((&x).abs_squared())
        );
    });

    gaussian_integer_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!((&x * &y) * &z, &x * (&y * &z));
        assert_eq!(&x * (&y + &z), &x * &y + &x * &z);
        assert_eq!((&x + &y) * &z, &x * &z + &y * &z);
    });
}

#[test]
fn mul_large_properties() {
    // Large inputs exercise the Karatsuba path, which the default configuration rarely reaches.
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    gaussian_integer_pair_gen().test_properties_with_config(&config, |(x, y)| {
        let product = &x * &y;
        assert_eq!(gaussian_integer_mul_naive(&x, &y), product);
        assert_eq!(x.clone() * y.clone(), product);
        assert_eq!(x.clone() * &y, product);
        assert_eq!(&x * y.clone(), product);
    });
}

#[test]
fn test_mul_branch_coverage() {
    #[allow(clippy::needless_pass_by_value)]
    fn check(x: GaussianInteger, y: GaussianInteger) {
        let product = &x * &y;
        assert!(product.real.is_valid());
        assert!(product.imaginary.is_valid());
        assert_eq!(gaussian_integer_mul_naive(&x, &y), product);
        assert_eq!(x.clone() * y.clone(), product);
        assert_eq!(x.clone() * &y, product);
        assert_eq!(&x * y.clone(), product);
        let mut product_alt = x.clone();
        product_alt *= y.clone();
        assert_eq!(product_alt, product);
        let mut product_alt = x.clone();
        product_alt *= &y;
        assert_eq!(product_alt, product);
        assert_eq!(&y * &x, product);
        assert_eq!(
            (&product).abs_squared(),
            (&x).abs_squared() * (&y).abs_squared()
        );
    }
    let gi = |real, imaginary| GaussianInteger { real, imaginary };
    let big = |bits: u64, tweak: i64| Integer::power_of_2(bits) + Integer::from(tweak);
    // - all four parts fit in a signed word
    check(
        gi(Integer::from(i64::MAX), Integer::from(i64::MIN)),
        gi(Integer::from(i64::MIN), Integer::from(i64::MAX)),
    );
    // - a part just exceeds a signed word, so the operands go to the fused path
    check(
        gi(Integer::from(i64::MAX) + Integer::ONE, Integer::from(3)),
        gi(Integer::from(-5), Integer::from(7)),
    );
    // - both operands are large and balanced, engaging the Karatsuba path
    check(
        gi(big(900, 3), -big(901, 17)),
        gi(big(950, -1), big(949, 12345)),
    );
    // - the first operand is large but unbalanced, so the fused path is used
    check(
        gi(big(900, 3), Integer::from(99)),
        gi(big(950, -1), big(949, 12345)),
    );
    // - the second operand is too small for the Karatsuba path
    check(
        gi(big(900, 3), -big(901, 17)),
        gi(big(100, -1), big(100, 5)),
    );
    // - the second operand is large but unbalanced
    check(
        gi(big(900, 3), -big(901, 17)),
        gi(big(950, -1), Integer::from(-2)),
    );
}
