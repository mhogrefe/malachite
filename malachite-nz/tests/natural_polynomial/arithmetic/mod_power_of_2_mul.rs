// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2Mul, ModPowerOf2MulAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::natural_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_nz::test_util::generators::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mod_power_of_2_mul::*;
use malachite_nz::test_util::natural_polynomial::arithmetic::mul::naturals_mul_naive;

#[test]
fn test_mod_power_of_2_mul() {
    let test = |s, t, pow, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from_str(t).unwrap();
        let r = (&p).mod_power_of_2_mul(&q, pow);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!((&p).mod_power_of_2_mul(q.clone(), pow), r);
        assert_eq!(p.clone().mod_power_of_2_mul(&q, pow), r);
        assert_eq!(p.clone().mod_power_of_2_mul(q.clone(), pow), r);
        let mut s = p.clone();
        s.mod_power_of_2_mul_assign(&q, pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        let mut s = p.clone();
        s.mod_power_of_2_mul_assign(q.clone(), pow);
        assert!(s.is_valid());
        assert_eq!(s, r);
        assert_eq!(mod_power_of_2_mul_naive(&p, &q, pow), r);
    };
    test("0", "0", 0, "0");
    test("0", "x+1", 4, "0");
    test("x^2+3*x+2", "2*x+5", 4, "2*x^3+11*x^2+3*x+10");
    test("x^2+3*x+2", "2*x+5", 100, "2*x^3+11*x^2+19*x+10");
    // A constant, which the forms taking the other polynomial by value multiply in place.
    test("x^2+3*x+2", "6", 4, "6*x^2+2*x+12");
    // The leading coefficient vanishes, so the degree drops.
    test("8*x+1", "2*x+1", 4, "10*x+1");
    // Every coefficient vanishes.
    test("2*x+2", "8", 4, "0");
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_fail_1() {
    NaturalPolynomial::from_str("x+16")
        .unwrap()
        .mod_power_of_2_mul(NaturalPolynomial::from_str("x+1").unwrap(), 4);
}

#[test]
#[should_panic]
fn mod_power_of_2_mul_fail_2() {
    NaturalPolynomial::from_str("x+1")
        .unwrap()
        .mod_power_of_2_mul(NaturalPolynomial::from_str("16*x+1").unwrap(), 4);
}

#[test]
fn mod_power_of_2_mul_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, pow)| {
            let r = (&p).mod_power_of_2_mul(&q, pow);
            assert!(r.is_valid());
            assert!(r.mod_power_of_2_is_reduced(pow));
            // The forms agree.
            assert_eq!((&p).mod_power_of_2_mul(q.clone(), pow), r);
            assert_eq!(p.clone().mod_power_of_2_mul(&q, pow), r);
            assert_eq!(p.clone().mod_power_of_2_mul(q.clone(), pow), r);
            let mut s = p.clone();
            s.mod_power_of_2_mul_assign(&q, pow);
            assert!(s.is_valid());
            assert_eq!(s, r);
            let mut s = p.clone();
            s.mod_power_of_2_mul_assign(q.clone(), pow);
            assert!(s.is_valid());
            assert_eq!(s, r);

            assert_eq!(mod_power_of_2_mul_naive(&p, &q, pow), r);
            assert_eq!((&p * &q).mod_power_of_2(pow), r);
            // Multiplication is commutative.
            assert_eq!((&q).mod_power_of_2_mul(&p, pow), r);
            if pow == 0 {
                assert_eq!(r, NaturalPolynomial::ZERO);
            }
        },
    );
}

// The kernels that multiply with low halves of products, at lengths around the Karatsuba threshold
// and with coefficients of one, two, and many limbs.
#[test]
fn test_mod_power_of_2_mul_low_algorithms() {
    for &(n, m) in &[(1, 1), (3, 2), (8, 8), (9, 8), (20, 7), (33, 33), (70, 41)] {
        for pow in [1, 31, 32, 33, 63, 64, 65, 100, 128, 129, 300, 1000] {
            let xs = natural_mod_power_of_2_generated_coefficients(n, pow);
            let ys = natural_mod_power_of_2_generated_coefficients(m, pow);
            let expected: Vec<_> = naturals_mul_naive(&xs, &ys)
                .into_iter()
                .map(|x| x.mod_power_of_2(pow))
                .collect();
            assert_eq!(mod_power_of_2_mul_low_classical(&xs, &ys, pow), expected);
            assert_eq!(mod_power_of_2_mul_low_karatsuba(&xs, &ys, pow), expected);
            assert_eq!(mod_power_of_2_mul_low_karatsuba(&ys, &xs, pow), expected);
        }
    }
}

#[test]
fn mod_power_of_2_mul_low_properties() {
    natural_polynomial_natural_polynomial_unsigned_triple_gen_var_1().test_properties(
        |(p, q, pow)| {
            let xs = p.coefficients_asc();
            let ys = q.coefficients_asc();
            if xs.is_empty() || ys.is_empty() {
                return;
            }
            let r = (&p).mod_power_of_2_mul(&q, pow);
            assert_eq!(
                NaturalPolynomial::from_coefficients_asc(mod_power_of_2_mul_low_classical(
                    xs, ys, pow
                )),
                r
            );
            assert_eq!(
                NaturalPolynomial::from_coefficients_asc(mod_power_of_2_mul_low_karatsuba(
                    xs, ys, pow
                )),
                r
            );
        },
    );
}

// The dispatcher's choice between the low-half kernels and the full product.
#[test]
fn test_mod_power_of_2_mul_dispatch() {
    let test = |len1: usize, len2: usize, pow: u64| {
        let p = NaturalPolynomial::from_coefficients_asc(
            natural_mod_power_of_2_generated_coefficients(len1, pow),
        );
        let q = NaturalPolynomial::from_coefficients_asc(
            natural_mod_power_of_2_generated_coefficients(len2, pow),
        );
        let r = (&p).mod_power_of_2_mul(&q, pow);
        assert_eq!(mod_power_of_2_mul_naive(&p, &q, pow), r);
        assert_eq!(p.clone().mod_power_of_2_mul(q.clone(), pow), r);
    };
    // - low-half kernels, with word arithmetic
    test(10, 9, 64);
    // - low-half kernels, with slots
    test(10, 9, 1000);
    // - full product, too long for the window
    test(3000, 2100, 64);
    // - full product, coefficients too large for the window
    test(10, 9, 30000);
    // - full product, a constant factor
    test(10, 1, 64);
}

// The dispatcher at the edges of the low-half kernels' windows, against the full product, which
// shares nothing with the low-half kernels. Each length is the largest in a window, or one past it,
// for the largest power the window covers. The edges are those measured in 2026-10; if they move,
// these still check agreement, only less sharply.
#[test]
fn test_mod_power_of_2_mul_window_edges() {
    for &(pow, edge) in &[
        (16, 150),
        (32, 500),
        (48, 1000),
        (64, 2000),
        (96, 24),
        (128, 64),
        (300, 100),
        (500, 48),
        (2000, 32),
        (5000, 8),
    ] {
        for n in [edge, edge + 1] {
            let xs = natural_mod_power_of_2_generated_coefficients(n, pow);
            let mut ys = natural_mod_power_of_2_generated_coefficients(n, pow);
            ys.reverse();
            let p = NaturalPolynomial::from_coefficients_asc(xs.clone());
            let q = NaturalPolynomial::from_coefficients_asc(ys.clone());
            assert_eq!(
                (&p).mod_power_of_2_mul(&q, pow),
                NaturalPolynomial::from_coefficients_asc(mod_power_of_2_mul_full(&xs, &ys, pow))
            );
        }
    }
}
