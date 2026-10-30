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
use malachite_base::polynomial::{
    DivPowerOfX, MulMiddle, MulMiddleAssign, MulPowerOfX, MulTruncated, Polynomial,
};
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::classical::*;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::fft::mul_middle_to_out_fft;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::kronecker::*;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::mul_middle_to_out;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::schonhage_strassen::*;
use malachite_nz::integer_polynomial::arithmetic::mul_middle::tiny::{
    mul_middle_to_out_tiny_1, mul_middle_to_out_tiny_2,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_polynomial_natural_polynomial_unsigned_unsigned_quadruple_gen_var_1,
    natural_polynomial_unsigned_unsigned_triple_gen_var_5,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mul::{
    natural_generated_coefficients, naturals_mul_naive,
};
use malachite_nz::test_util::natural_polynomial::arithmetic::mul_middle::mul_middle_naive;

#[test]
fn test_mul_middle() {
    let test = |s, t, low, high, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let r = (&p).mul_middle(&q, low, high);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mul_middle(q.clone(), low, high), r);
        assert_eq!(p.clone().mul_middle(&q, low, high), r);
        assert_eq!(p.clone().mul_middle(q.clone(), low, high), r);
        let mut s = p.clone();
        s.mul_middle_assign(&q, low, high);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mul_middle_assign(q.clone(), low, high);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mul_middle_naive(&p, &q, low, high), r);
    };
    test("0", "x+1", 0, 3, "0");
    test("x+1", "0", 1, 2, "0");
    test("x+1", "x+2", 0, 0, "0");
    test("x+1", "x+2", 1, 1, "0");
    test("x+1", "x+2", 0, 3, "x^2+3*x+2");
    test("x+1", "x+2", 1, 3, "x+3");
    test("x^2+3*x+2", "2*x+5", 1, 3, "11*x+19");
    test("3*x^2+2*x+1", "6*x^2+5*x+4", 1, 4, "27*x^2+28*x+13");
    // The range extends past the end of the product.
    test("x+1", "x+2", 2, 100, "1");
    test("x+1", "x+2", 0, u64::MAX, "x^2+3*x+2");
    // The range starts past the end of the product.
    test("x+1", "x+2", 3, 100, "0");
    test("x+1", "x+2", u64::MAX, u64::MAX, "0");
    // Constants on either side, which the forms taking the other by value multiply in place.
    test("3", "x^3+x^2+x+1", 1, 3, "3*x+3");
    test("x^3+x^2+x+1", "2", 2, 4, "2*x+2");
    test("x^3+x^2+x+1", "2", 5, 7, "0");
    test("2*x^2+3", "x+4", 1, 2, "3");
    // Both factors are the same.
    test("x^2+1", "x^2+1", 1, 3, "2*x");
    test(
        "18446744073709551616*x+1",
        "18446744073709551616*x+1",
        1,
        3,
        "340282366920938463463374607431768211456*x+36893488147419103232",
    );
}

#[test]
#[should_panic]
fn mul_middle_fail() {
    let _ = NaturalPolynomial::from_str("x+1").unwrap().mul_middle(
        NaturalPolynomial::from_str("x+2").unwrap(),
        2,
        1,
    );
}

#[test]
#[should_panic]
fn mul_middle_ref_ref_fail() {
    let _ = (&NaturalPolynomial::from_str("x+1").unwrap()).mul_middle(
        &NaturalPolynomial::from_str("x+2").unwrap(),
        2,
        1,
    );
}

// A constant factor takes the in-place path.
#[test]
#[should_panic]
fn mul_middle_constant_fail() {
    let _ = NaturalPolynomial::from_str("x+1").unwrap().mul_middle(
        NaturalPolynomial::from_str("3").unwrap(),
        2,
        1,
    );
}

// The zero polynomial has no coefficients, so only the explicit check makes this panic rather than
// quietly returning zero.
#[test]
#[should_panic]
fn mul_middle_zero_polynomial_fail() {
    let _ = NaturalPolynomial::ZERO.mul_middle(NaturalPolynomial::ZERO, 2, 1);
}

#[test]
#[should_panic]
fn mul_middle_assign_fail() {
    let mut p = NaturalPolynomial::from_str("x+1").unwrap();
    p.mul_middle_assign(NaturalPolynomial::from_str("x+2").unwrap(), 2, 1);
}

// Every middle-product kernel and every branch of the dispatcher, on polynomials with non-negative
// coefficients of chosen sizes.
#[test]
fn test_mul_middle_algorithms() {
    let test = |len1: usize, len2: usize, bits1: u64, bits2: u64, nlo: usize, nhi: usize| {
        let xs = natural_generated_coefficients(len1, bits1);
        let ys = natural_generated_coefficients(len2, bits2);
        let expected = &naturals_mul_naive(&xs, &ys)[nlo..nhi];
        let mut out = vec![Natural::ZERO; nhi - nlo];
        mul_middle_to_out(&mut out, &xs, &ys, nlo, nhi);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; nhi - nlo];
        mul_middle_to_out_classical(&mut out, &xs, &ys, nlo, nhi);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; nhi - nlo];
        mul_middle_to_out_kronecker(&mut out, &xs, &ys, nlo, nhi);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; nhi - nlo];
        mul_middle_to_out_schonhage_strassen(&mut out, &xs, &ys, nlo, nhi);
        assert_eq!(out, expected);
        let mut out = vec![Natural::ZERO; nhi - nlo];
        if mul_middle_to_out_fft(&mut out, &xs, &ys, nlo, nhi) {
            assert_eq!(out, expected);
        }
        let small = bits1 <= Limb::WIDTH - 2 && bits2 <= Limb::WIDTH - 2;
        let rbits = bits1 + bits2 + u64::from(usize::BITS - len2.leading_zeros());
        if small && rbits <= Limb::WIDTH - 2 {
            let mut out = vec![Natural::ZERO; nhi - nlo];
            mul_middle_to_out_tiny_1(&mut out, &xs, &ys, nlo, nhi);
            assert_eq!(out, expected);
        }
        if small && rbits < Limb::WIDTH << 1 {
            let mut out = vec![Natural::ZERO; nhi - nlo];
            mul_middle_to_out_tiny_2(&mut out, &xs, &ys, nlo, nhi);
            assert_eq!(out, expected);
        }
    };
    // - one-word tiny kernel
    test(5, 4, 3, 5, 1, 6);
    // - two-word tiny kernel
    test(6, 5, 40, 30, 2, 7);
    // - nhi <= 8
    test(8, 7, 100, 90, 2, 8);
    // - nhi > 8 && !classical_preferred(len2, bits1, bits2) && len <= 3
    test(12, 12, 100, 90, 9, 12);
    // - nlo == 0 && karatsuba_preferred(len2, bits1, bits2)
    test(8, 7, 1000, 900, 0, 12);
    // - nlo != 0, so Karatsuba is not used
    test(8, 7, 1000, 900, 2, 12);
    // - fft_preferred(len2, bits1, bits2, 100, 200), and the FFT computes the product
    test(120, 110, 20, 20, 30, 150);
    // - fft_preferred(len2, bits1, bits2, 100, 200), but the FFT declines
    test(120, 110, 250, 250, 30, 150);
    // - schonhage_strassen_preferred(len1, len2, bits1, bits2, 3071)
    test(39, 20, 600, 500, 19, 39);
    test(23, 12, 600, 500, 11, 23);
    // - Kronecker substitution
    test(23, 12, 500, 400, 11, 23);
}

#[test]
fn mul_middle_properties() {
    natural_polynomial_natural_polynomial_unsigned_unsigned_quadruple_gen_var_1().test_properties(
        |(p, q, low, high)| {
            let r = (&p).mul_middle(&q, low, high);
            assert!(r.is_valid());
            // The forms agree.
            assert_eq!((&p).mul_middle(q.clone(), low, high), r);
            assert_eq!(p.clone().mul_middle(&q, low, high), r);
            assert_eq!(p.clone().mul_middle(q.clone(), low, high), r);
            let mut s = p.clone();
            s.mul_middle_assign(&q, low, high);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mul_middle_assign(q.clone(), low, high);
            assert!(s.is_valid());
            assert_eq!(s, r);

            // This is a window of the whole product.
            assert_eq!(mul_middle_naive(&p, &q, low, high), r);
            assert_eq!((&p * &q).truncate(high).div_power_of_x(low), r);
            assert!(r.len() <= high - low);
            // A window from the bottom is a truncated product.
            assert_eq!((&p).mul_middle(&q, 0, high), (&p).mul_truncated(&q, high));
            // A window splits into two adjacent windows.
            let mid = low + ((high - low) >> 1);
            assert_eq!(
                (&p).mul_middle(&q, low, mid)
                    + (&p).mul_middle(&q, mid, high).mul_power_of_x(mid - low),
                r
            );
            // Multiplication is commutative.
            assert_eq!((&q).mul_middle(&p, low, high), r);
            assert_eq!(
                IntegerPolynomial::from(&p).mul_middle(IntegerPolynomial::from(&q), low, high),
                IntegerPolynomial::from(&r)
            );
        },
    );

    natural_polynomial_unsigned_unsigned_triple_gen_var_5().test_properties(|(p, low, high)| {
        let window = p.clone().truncate(high).div_power_of_x(low);
        assert_eq!((&p).mul_middle(NaturalPolynomial::one(), low, high), window);
        assert_eq!(NaturalPolynomial::one().mul_middle(&p, low, high), window);
        assert_eq!(
            (&p).mul_middle(NaturalPolynomial::ZERO, low, high),
            NaturalPolynomial::ZERO
        );
        assert_eq!(
            NaturalPolynomial::ZERO.mul_middle(&p, low, high),
            NaturalPolynomial::ZERO
        );
        // The square path.
        assert_eq!(
            (&p).mul_middle(&p, low, high),
            (&p * &p).truncate(high).div_power_of_x(low)
        );
    });
}
