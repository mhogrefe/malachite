// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Square;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{BitPack, Evaluate, Polynomial};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::mul::classical::mul_to_out_classical;
use malachite_nz::integer_polynomial::arithmetic::mul::karatsuba::mul_to_out_karatsuba;
use malachite_nz::integer_polynomial::arithmetic::mul::kronecker::mul_to_out_kronecker;
use malachite_nz::integer_polynomial::arithmetic::mul::mul_greater_to_out;
use malachite_nz::integer_polynomial::arithmetic::mul::schonhage_strassen::*;
use malachite_nz::integer_polynomial::arithmetic::mul::tiny::{
    mul_to_out_tiny_1, mul_to_out_tiny_2,
};
use malachite_nz::integer_polynomial::arithmetic::mul_middle::fft::mul_middle_to_out_fft;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_polynomial_gen, natural_polynomial_natural_pair_gen, natural_polynomial_pair_gen,
    natural_polynomial_triple_gen,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mul::{
    mul_naive, natural_generated_coefficients, naturals_mul_naive,
};

#[test]
fn test_mul() {
    let test = |s, t, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let r = &p * &q;
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(&p * q.clone(), r);
        assert_eq!(p.clone() * &q, r);
        assert_eq!(p.clone() * q.clone(), r);
        let mut s = p.clone();
        s *= &q;
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s *= q.clone();
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mul_naive(&p, &q), r);
    };
    test("0", "x+1", "0");
    test("1", "x^2+3*x+5", "x^2+3*x+5");
    // A constant on the right, which the forms taking the left by value multiply in place.
    test("x^2+3*x+5", "2", "2*x^2+6*x+10");
    test("x+1", "x+2", "x^2+3*x+2");
    test("2*x^2+3", "x+4", "2*x^3+8*x^2+3*x+12");
    test("x^3+x", "x^3+x", "x^6+2*x^4+x^2");
    test(
        "18446744073709551616*x+1",
        "18446744073709551616*x+1",
        "340282366920938463463374607431768211456*x^2+36893488147419103232*x+1",
    );
}

// Every multiplication kernel and every branch of the dispatcher, on polynomials with non-negative
// coefficients of chosen sizes.
#[test]
fn test_mul_algorithms() {
    let test = |len1: usize, len2: usize, bits1: u64, bits2: u64| {
        let xs = natural_generated_coefficients(len1, bits1);
        let ys = natural_generated_coefficients(len2, bits2);
        let expected = naturals_mul_naive(&xs, &ys);
        let n = len1 + len2 - 1;
        let mut out = vec![Natural::ZERO; n];
        mul_greater_to_out(&mut out, &xs, &ys);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        mul_to_out_classical(&mut out, &xs, &ys);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        mul_to_out_karatsuba(&mut out, &xs, &ys);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        mul_to_out_kronecker(&mut out, &xs, &ys);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        mul_to_out_schonhage_strassen(&mut out, &xs, &ys);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        if mul_middle_to_out_fft(&mut out, &xs, &ys, 0, n) {
            assert_eq!(out, expected);
        }
        // The tiny kernels need small coefficients, and every partial sum to fit in one or two
        // signed words.
        let small = bits1 <= Limb::WIDTH - 2 && bits2 <= Limb::WIDTH - 2;
        let rbits = bits1 + bits2 + u64::from(usize::BITS - len2.leading_zeros());
        if small && rbits <= Limb::WIDTH - 2 {
            let mut out = vec![Natural::ZERO; n];
            mul_to_out_tiny_1(&mut out, &xs, &ys);
            assert_eq!(out, expected);
        }
        if small && rbits < Limb::WIDTH << 1 {
            let mut out = vec![Natural::ZERO; n];
            mul_to_out_tiny_2(&mut out, &xs, &ys);
            assert_eq!(out, expected);
        }
    };
    // - one-word tiny kernel
    test(5, 4, 3, 5);
    // - two-word tiny kernel
    test(6, 5, 40, 30);
    // - classical
    test(3, 2, 500, 400);
    // - Karatsuba
    test(8, 7, 1000, 900);
    // - Kronecker substitution
    test(30, 20, 100, 90);
    // - Schönhage–Strassen, short
    test(40, 30, 600, 500);
    // - Schönhage–Strassen, long
    test(150, 120, 600, 500);
    // - small-prime FFT
    test(120, 110, 20, 20);
    test(300, 200, 64, 64);
}

#[test]
fn mul_properties() {
    natural_polynomial_pair_gen().test_properties(|(p, q)| {
        let r = &p * &q;
        assert!(r.is_valid());
        // The forms agree.
        assert_eq!(&p * q.clone(), r);
        assert_eq!(p.clone() * &q, r);
        assert_eq!(p.clone() * q.clone(), r);
        let mut s = p.clone();
        s *= &q;
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s *= q.clone();
        assert!(s.is_valid());
        assert_eq!(s, r);

        assert_eq!(mul_naive(&p, &q), r);
        // The product agrees with the product of the same polynomials as `IntegerPolynomial`s.
        assert_eq!(
            IntegerPolynomial::from(p.clone()) * IntegerPolynomial::from(q.clone()),
            IntegerPolynomial::from(r.clone())
        );
        // Multiplication is commutative.
        assert_eq!(&q * &p, r);
        // Degrees add, since the natural numbers have no zero divisors.
        assert_eq!(
            r.degree(),
            p.degree().and_then(|d| q.degree().map(|e| d + e))
        );
        // Evaluation is a ring homomorphism.
        for x in [Natural::from(3u32), Natural::from(10u32)] {
            assert_eq!((&r).evaluate(&x), (&p).evaluate(&x) * (&q).evaluate(&x));
        }
        assert_eq!((&r).bit_pack(100), (&p).bit_pack(100) * (&q).bit_pack(100));
    });

    natural_polynomial_gen().test_properties(|p| {
        assert_eq!(&p * &p, (&p).square());
        assert_eq!(&p * NaturalPolynomial::ZERO, NaturalPolynomial::ZERO);
        assert_eq!(NaturalPolynomial::ZERO * &p, NaturalPolynomial::ZERO);
        assert_eq!(&p * NaturalPolynomial::one(), p);
        assert_eq!(NaturalPolynomial::one() * &p, p);
    });

    natural_polynomial_natural_pair_gen().test_properties(|(p, c)| {
        // Multiplying by a constant polynomial is scalar multiplication.
        assert_eq!(
            &p * NaturalPolynomial::from(c.clone()),
            NaturalPolynomial::from_coefficients_asc(
                p.coefficients_asc().iter().map(|x| x * &c).collect()
            )
        );
    });

    natural_polynomial_triple_gen().test_properties(|(p, q, r)| {
        // Multiplication is associative and distributes over addition.
        assert_eq!(&(&p * &q) * &r, &p * &(&q * &r));
        assert_eq!(&p * &(&q + &r), &p * &q + &p * &r);
    });
}
