// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Mul};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    ModPowerOf2MulTruncated, ModPowerOf2MulTruncatedAssign, MulTruncated, Polynomial,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::natural_polynomial::arithmetic::mod_power_of_2_mul_truncated::{
    mod_power_of_2_mul_truncated_low_classical, mod_power_of_2_mul_truncated_low_karatsuba,
};
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mul::naturals_mul_naive;

#[test]
fn test_mod_power_of_2_mul_truncated() {
    let test = |s, t, len, pow, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let r = (&p).mod_power_of_2_mul_truncated(&q, len, pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mod_power_of_2_mul_truncated(q.clone(), len, pow), r);
        assert_eq!(p.clone().mod_power_of_2_mul_truncated(&q, len, pow), r);
        assert_eq!(
            p.clone().mod_power_of_2_mul_truncated(q.clone(), len, pow),
            r
        );
        let mut s = p.clone();
        s.mod_power_of_2_mul_truncated_assign(&q, len, pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_power_of_2_mul_truncated_assign(q.clone(), len, pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_power_of_2_mul_truncated_naive(&p, &q, len, pow), r);
    };
    test("x^2+3*x+2", "2*x+5", 0, 4, "0");
    test("x^2+3*x+2", "2*x+5", 2, 4, "3*x+10");
    test("x^2+3*x+2", "2*x+5", 10, 4, "2*x^3+11*x^2+3*x+10");
    // A constant, which the forms taking the other polynomial by value multiply in place.
    test("x^2+3*x+2", "6", 2, 4, "2*x+12");
    // The linear coefficient vanishes, and is trimmed.
    test("x+15", "x+1", 2, 4, "15");
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_truncated_fail() {
    NaturalPolynomial::from_str("x+16")
        .unwrap()
        .mod_power_of_2_mul_truncated(NaturalPolynomial::from_str("x+1").unwrap(), 2, 4);
}

#[test]
fn mod_power_of_2_mul_truncated_properties() {
    natural_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1().test_properties(
        |(p, q, len, pow)| {
            let r = (&p).mod_power_of_2_mul_truncated(&q, len, pow);
            assert!(r.is_valid());
            assert!(r.mod_power_of_2_is_reduced(pow));
            // The forms agree.
            assert_eq!((&p).mod_power_of_2_mul_truncated(q.clone(), len, pow), r);
            assert_eq!(p.clone().mod_power_of_2_mul_truncated(&q, len, pow), r);
            assert_eq!(
                p.clone().mod_power_of_2_mul_truncated(q.clone(), len, pow),
                r
            );
            let mut s = p.clone();
            s.mod_power_of_2_mul_truncated_assign(&q, len, pow);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_power_of_2_mul_truncated_assign(q.clone(), len, pow);
            assert!(s.is_valid());
            assert_eq!(s, r);

            assert_eq!(mod_power_of_2_mul_truncated_naive(&p, &q, len, pow), r);
            assert_eq!((&p).mul_truncated(&q, len).mod_power_of_2(pow), r);
            assert_eq!((&p).mod_power_of_2_mul(&q, pow).truncate(len), r);
            assert_eq!((&q).mod_power_of_2_mul_truncated(&p, len, pow), r);
            assert!(r.len() <= len);
        },
    );
}

// The kernels that multiply with low halves of products and truncate, at lengths around the
// Karatsuba threshold and with coefficients of one, two, and many limbs.
#[test]
fn test_mod_power_of_2_mul_truncated_low_algorithms() {
    for &(n, m, len) in &[
        (1, 1, 1),
        (3, 2, 2),
        (8, 8, 8),
        (9, 8, 16),
        (20, 7, 30),
        (33, 33, 33),
        (33, 33, 65),
        (70, 41, 60),
        (70, 41, 200),
    ] {
        for pow in [1, 31, 32, 33, 63, 64, 65, 100, 128, 129, 300, 1000] {
            let xs = natural_mod_power_of_2_generated_coefficients(n, pow);
            let ys = natural_mod_power_of_2_generated_coefficients(m, pow);
            let mut expected: Vec<_> = naturals_mul_naive(&xs, &ys)
                .into_iter()
                .map(|x| x.mod_power_of_2(pow))
                .collect();
            expected.resize(len, Natural::ZERO);
            assert_eq!(
                mod_power_of_2_mul_truncated_low_classical(&xs, &ys, len, pow),
                expected
            );
            assert_eq!(
                mod_power_of_2_mul_truncated_low_karatsuba(&xs, &ys, len, pow),
                expected
            );
            assert_eq!(
                mod_power_of_2_mul_truncated_low_karatsuba(&ys, &xs, len, pow),
                expected
            );
        }
    }
}

#[test]
fn mod_power_of_2_mul_truncated_low_properties() {
    natural_polynomial_pair_unsigned_unsigned_quadruple_gen_var_1().test_properties(
        |(p, q, len, pow)| {
            let xs = p.coefficients_asc();
            let ys = q.coefficients_asc();
            if xs.is_empty() || ys.is_empty() || len == 0 {
                return;
            }
            let r = (&p).mod_power_of_2_mul_truncated(&q, len, pow);
            let len = usize::try_from(len).unwrap();
            assert_eq!(
                NaturalPolynomial::from_coefficients_asc(
                    mod_power_of_2_mul_truncated_low_classical(xs, ys, len, pow)
                ),
                r
            );
            assert_eq!(
                NaturalPolynomial::from_coefficients_asc(
                    mod_power_of_2_mul_truncated_low_karatsuba(xs, ys, len, pow)
                ),
                r
            );
        },
    );
}

// The dispatcher's choice between the low-half kernels and the full truncated product.
#[test]
fn test_mod_power_of_2_mul_truncated_dispatch() {
    let test = |len1: usize, len2: usize, len: u64, pow: u64| {
        let p = NaturalPolynomial::from_coefficients_asc(
            natural_mod_power_of_2_generated_coefficients(len1, pow),
        );
        let q = NaturalPolynomial::from_coefficients_asc(
            natural_mod_power_of_2_generated_coefficients(len2, pow),
        );
        let r = (&p).mod_power_of_2_mul_truncated(&q, len, pow);
        assert_eq!(mod_power_of_2_mul_truncated_naive(&p, &q, len, pow), r);
        assert_eq!(
            p.clone().mod_power_of_2_mul_truncated(q.clone(), len, pow),
            r
        );
    };
    // - low-half kernels, with word arithmetic
    test(10, 9, 12, 64);
    // - low-half kernels, with slots
    test(10, 9, 12, 1000);
    // - low-half kernels, a length past the end of the product
    test(10, 9, u64::MAX, 1000);
    // - full truncated product, too long for the window
    test(1200, 1100, 1500, 64);
    // - full truncated product, coefficients too large for the window
    test(10, 9, 12, 30000);
    // - full truncated product, a constant factor
    test(10, 1, 5, 64);
}
