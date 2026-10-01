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
use malachite_base::polynomial::{
    MulTruncated, Polynomial, SquareTruncated, SquareTruncatedAssign,
};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::fft::mul_middle_to_out_fft;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::classical::*;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::karatsuba::*;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::kronecker::*;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::schonhage_strassen::*;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::square_truncated_to_out;
use malachite_nz::integer_polynomial::arithmetic::square_truncated::tiny::{
    square_truncated_to_out_tiny_1, square_truncated_to_out_tiny_2,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_polynomial_unsigned_pair_gen_var_3;
use malachite_nz::test_util::natural_polynomial::arithmetic::mul::{
    natural_generated_coefficients, naturals_mul_naive,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::square_truncated::*;

#[test]
fn test_square_truncated() {
    let test = |s, len: u64, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let r = (&p).square_truncated(len);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(p.clone().square_truncated(len), r);
        let mut s = p.clone();
        s.square_truncated_assign(len);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(square_truncated_naive(&p, len), r);
    };
    test("x^2+3*x+2", 0, "0");
    test("x^2+3*x+2", 1, "4");
    test("x^2+3*x+2", 3, "13*x^2+12*x+4");
    test("x^2+3*x+2", 10, "x^4+6*x^3+13*x^2+12*x+4");
    // A constant, which `square_truncated_assign` squares in place.
    test("3", 1, "9");
    // Truncating below a zero coefficient trims it.
    test("x^2+1", 2, "1");
}

// Every truncated squaring kernel and every branch of the dispatcher, on polynomials with
// non-negative coefficients of chosen sizes.
#[test]
fn test_square_truncated_algorithms() {
    let test = |len: usize, bits: u64, n: usize| {
        let xs = natural_generated_coefficients(len, bits);
        let expected = &naturals_mul_naive(&xs, &xs)[..n];
        let mut out = vec![Natural::ZERO; n];
        square_truncated_to_out(&mut out, &xs);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        square_truncated_to_out_classical(&mut out, &xs);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        square_truncated_to_out_karatsuba(&mut out, &xs);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        square_truncated_to_out_kronecker(&mut out, &xs);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        square_truncated_to_out_schonhage_strassen(&mut out, &xs);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; n];
        if mul_middle_to_out_fft(&mut out, &xs, &xs, 0, n) {
            assert_eq!(out, expected);
        }
        let small = bits <= Limb::WIDTH - 2;
        let rbits = (bits << 1) + u64::from(usize::BITS - len.leading_zeros());
        if small && rbits <= Limb::WIDTH - 2 {
            let mut out = vec![Natural::ZERO; n];
            square_truncated_to_out_tiny_1(&mut out, &xs);
            assert_eq!(out, expected);
        }
        if small && rbits < Limb::WIDTH << 1 {
            let mut out = vec![Natural::ZERO; n];
            square_truncated_to_out_tiny_2(&mut out, &xs);
            assert_eq!(out, expected);
        }
    };
    // - one-word tiny kernel
    test(5, 4, 6);
    // - two-word tiny kernel
    test(6, 40, 7);
    // - classical
    test(3, 500, 3);
    // - Karatsuba
    test(8, 1000, 12);
    // - Kronecker substitution
    test(60, 100, 60);
    // - Schönhage–Strassen
    test(40, 600, 60);
    test(150, 600, 200);
    // - small-prime FFT
    test(250, 20, 300);
    test(300, 64, 400);
}

#[test]
fn square_truncated_properties() {
    natural_polynomial_unsigned_pair_gen_var_3::<u64>().test_properties(|(p, len)| {
        let r = (&p).square_truncated(len);
        assert!(r.is_valid());
        // The forms agree.
        assert_eq!(p.clone().square_truncated(len), r);
        let mut s = p.clone();
        s.square_truncated_assign(len);
        assert!(s.is_valid());
        assert_eq!(s, r);

        assert_eq!(square_truncated_naive(&p, len), r);
        assert_eq!((&p).square().truncate(len), r);
        assert_eq!((&p).mul_truncated(&p, len), r);
        assert_eq!(
            IntegerPolynomial::from(p.clone()).square_truncated(len),
            IntegerPolynomial::from(r.clone())
        );
        assert!(r.len() <= len);
    });
}
