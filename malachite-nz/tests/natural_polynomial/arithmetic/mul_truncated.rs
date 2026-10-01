// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{MulTruncated, MulTruncatedAssign, Polynomial};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::fft::mul_middle_to_out_fft;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::classical::*;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::karatsuba::*;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::kronecker::*;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::mul_truncated_to_out;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::schonhage_strassen::*;
use malachite_nz::integer_polynomial::arithmetic::mul_truncated::tiny::{
    mul_truncated_to_out_tiny_1, mul_truncated_to_out_tiny_2,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mul::{
    natural_generated_coefficients, naturals_mul_naive,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mul_truncated::mul_truncated_naive;

#[test]
fn test_mul_truncated() {
    let test = |s, t, len: u64, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let r = (&p).mul_truncated(&q, len);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mul_truncated(q.clone(), len), r);
        assert_eq!(p.clone().mul_truncated(&q, len), r);
        assert_eq!(p.clone().mul_truncated(q.clone(), len), r);
        let mut s = p.clone();
        s.mul_truncated_assign(&q, len);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mul_truncated_assign(q.clone(), len);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mul_truncated_naive(&p, &q, len), r);
    };
    test("x^2+3*x+2", "2*x+5", 0, "0");
    test("x^2+3*x+2", "2*x+5", 1, "10");
    test("x^2+3*x+2", "2*x+5", 2, "19*x+10");
    test("x^2+3*x+2", "2*x+5", 10, "2*x^3+11*x^2+19*x+10");
    // A constant, which the forms taking the other polynomial by value multiply in place.
    test("x^2+3*x+2", "3", 2, "9*x+6");
    // Truncating below a zero coefficient trims it.
    test("x^2+1", "x^2+1", 3, "2*x^2+1");
    test("x^2+1", "1", 2, "1");
}

// Every truncated multiplication kernel and every branch of the dispatcher, on polynomials with
// non-negative coefficients of chosen sizes.
#[test]
fn test_mul_truncated_algorithms() {
    let test = |len1: usize, len2: usize, bits1: u64, bits2: u64, n: usize| {
        let xs = natural_generated_coefficients(len1, bits1);
        let ys = natural_generated_coefficients(len2, bits2);
        let expected = &naturals_mul_naive(&xs, &ys)[..n];
        let mut out = vec![Natural::ZERO; n];
        mul_truncated_to_out(&mut out, &xs, &ys);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        mul_truncated_to_out_classical(&mut out, &xs, &ys);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        mul_truncated_to_out_karatsuba(&mut out, &xs, &ys);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        mul_truncated_to_out_kronecker(&mut out, &xs, &ys);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        mul_truncated_to_out_schonhage_strassen(&mut out, &xs, &ys);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        if mul_middle_to_out_fft(&mut out, &xs, &ys, 0, n) {
            assert_eq!(out, expected);
        }
        let small = bits1 <= Limb::WIDTH - 2 && bits2 <= Limb::WIDTH - 2;
        let rbits = bits1 + bits2 + u64::from(usize::BITS - len2.leading_zeros());
        if small && rbits <= Limb::WIDTH - 2 {
            let mut out = vec![Natural::ZERO; n];
            mul_truncated_to_out_tiny_1(&mut out, &xs, &ys);
            assert_eq!(out, expected);
        }
        if small && rbits < Limb::WIDTH << 1 {
            let mut out = vec![Natural::ZERO; n];
            mul_truncated_to_out_tiny_2(&mut out, &xs, &ys);
            assert_eq!(out, expected);
        }
    };
    // - one-word tiny kernel
    test(5, 4, 3, 5, 6);
    // - two-word tiny kernel
    test(6, 5, 40, 30, 7);
    // - classical
    test(3, 2, 500, 400, 3);
    // - Karatsuba
    test(8, 7, 1000, 900, 12);
    // - Kronecker substitution
    test(60, 50, 100, 90, 60);
    // - Schönhage–Strassen
    test(40, 30, 600, 500, 50);
    test(150, 120, 600, 500, 200);
    // - small-prime FFT
    test(120, 110, 20, 20, 150);
    test(300, 200, 64, 64, 400);
}

#[test]
fn mul_truncated_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_2().test_properties(
        |(p, q, len)| {
            let r = (&p).mul_truncated(&q, len);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!((&p).mul_truncated(q.clone(), len), r);
            assert_eq!(p.clone().mul_truncated(&q, len), r);
            assert_eq!(p.clone().mul_truncated(q.clone(), len), r);
            let mut s = p.clone();
            s.mul_truncated_assign(&q, len);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mul_truncated_assign(q.clone(), len);
            assert!(s.is_valid());
            assert_eq!(s, r);

            assert_eq!(mul_truncated_naive(&p, &q, len), r);
            assert_eq!((&p * &q).truncate(len), r);
            assert_eq!((&q).mul_truncated(&p, len), r);
            assert_eq!(
                IntegerPolynomial::from(p.clone()).mul_truncated(IntegerPolynomial::from(q), len),
                IntegerPolynomial::from(r.clone())
            );
            assert!(r.len() <= len);
        },
    );
}
