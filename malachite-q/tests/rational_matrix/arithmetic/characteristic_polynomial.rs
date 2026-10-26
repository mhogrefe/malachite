// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    CharacteristicPolynomial, Determinant, Parity, Pow, SimilarityTransform, Trace,
};
use malachite_base::num::basic::traits::One;
use malachite_base::polynomial::Polynomial;
use malachite_nz::test_util::generators::integer_matrix_gen_var_1;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_matrix_gen_var_2, rational_matrix_rational_pair_gen,
    rational_matrix_unsigned_rational_triple_gen_var_2,
};
use malachite_q::test_util::rational_matrix::arithmetic::characteristic_polynomial::*;
use std::panic::catch_unwind;

// Checks the characteristic polynomial by value and by reference, and against the principal minors
// and the Cayley-Hamilton theorem for small matrices, and returns it.
fn verify(a: &RationalMatrix) -> RationalPolynomial {
    let p = a.characteristic_polynomial();
    assert!(p.is_valid());
    assert_eq!(a.clone().characteristic_polynomial(), p);
    let n = a.height();
    if n <= 7 {
        assert_eq!(rational_matrix_characteristic_polynomial_naive(a), p);
    }
    if n <= 8 {
        assert!(rational_polynomial_evaluate_at_matrix(&p, a).is_zero());
    }
    p
}

#[test]
fn test_characteristic_polynomial() {
    let test = |s, out| {
        let a = RationalMatrix::from_str(s).unwrap();
        assert_eq!(verify(&a).to_string(), out);
    };
    // - dimension 0
    test("[]_{0,0}", "1");
    // - dimension 1
    test("[1/2]", "x-1/2");
    // - dimension 2
    test("[1/2, 1/3; -2, 3/4]", "x^2-5/4*x+25/24");
    // - an integer matrix
    test("[1, 2; 3, 4]", "x^2-5*x-2");
    // - dimension 3
    test(
        "[1/2, 1/3, 1; 2, 3/4, 5; -1, 0, 2/3]",
        "x^3-23/12*x^2+37/24*x+10/9",
    );
    // - dimension 5
    test(
        "[0, 5/3, 5/6, -7/4, -1/2; 3/4, -3, 1, 0, 8; -1, -1, -7/5, -5/3, -1/2; 3, 1/3, 3, -1/4, -1\
        /8; -9/4, 1/2, 7/2, -2/3, -2/3]",
        "x^5+319/60*x^4+631/40*x^3+586271/5760*x^2+2985161/34560*x+5126479/8640",
    );
    // - dimension 9: the multimodular algorithm for the numerators
    test(
        "[1/3, 5, 5/4, 1/2, -1, -1, -1, -5/2, 5; 3/2, -3/2, 1, 0, 5, 5/3, -1/2, 1, 1; -1/2, 0, 0, \
        0, -1/3, 2, -3/4, -1, 5/3; 1/2, -1, 2, 1/2, -1, -5, -1/2, 1, 5/2; -1/2, 1, -5/3, -5, 1/4, \
        -3, 3, 3/4, 0; 0, -1, 0, 2/3, 0, -3, 1/2, 3/4, -2; 1/2, 1/4, 1/4, 3, 1, 3/2, -3/2, 5/3, -1\
        /2; -1/2, -3/2, -1, 5/3, -1, 1, 1/4, 1, 5/3; 1/4, 1, -2, 1/4, 1, 3, -2, -4/3, -1/3]",
        "x^9+17/4*x^8-1189/144*x^7-123131/1728*x^6+4009931/10368*x^5+74352353/124416*x^4+19098455/\
        41472*x^3+4361972063/2985984*x^2-45305346289/4478976*x-175779906893/8957952",
    );
}

#[test]
fn characteristic_polynomial_fail() {
    let a = RationalMatrix::from_str("[1/2, 2]").unwrap();
    assert_panic!(a.characteristic_polynomial());
    let a = RationalMatrix::from_str("[1/2, 2]").unwrap();
    assert_panic!((&a).characteristic_polynomial());
}

#[test]
fn characteristic_polynomial_properties() {
    rational_matrix_gen_var_2().test_properties(|a| {
        let p = verify(&a);
        let n = a.height();
        // The polynomial is monic of degree n.
        assert_eq!(p.degree(), Some(n));
        assert_eq!(p.leading_coefficient(), Rational::ONE);
        // The constant coefficient is (-1)^n times the determinant, and the next-to-leading one is
        // the negative of the trace.
        let det = (&a).determinant();
        assert_eq!(p.coefficient(0), if n.even() { det } else { -det });
        if n > 0 {
            assert_eq!(p.coefficient(n - 1), -a.to_trace());
        }
        // A matrix and its transpose have the same characteristic polynomial.
        assert_eq!(a.transpose().characteristic_polynomial(), p);
    });

    rational_matrix_unsigned_rational_triple_gen_var_2().test_properties(|(a, r, d)| {
        // Similar matrices have the same characteristic polynomial.
        assert_eq!(
            (&a).similarity_transform(r, d).characteristic_polynomial(),
            a.characteristic_polynomial()
        );
    });

    rational_matrix_rational_pair_gen().test_properties(|(a, c)| {
        if !a.is_square() {
            return;
        }
        // Scaling the matrix by c multiplies the coefficient of x^(n - k) by c^k.
        let n = a.height();
        let p = (&a).characteristic_polynomial();
        let q = (&a * &c).characteristic_polynomial();
        for k in 0..=n {
            assert_eq!(q.coefficient(n - k), p.coefficient(n - k) * (&c).pow(k));
        }
    });

    integer_matrix_gen_var_1().test_properties(|a| {
        // The polynomial agrees with that of the IntegerMatrix.
        assert_eq!(
            RationalMatrix::from(&a).characteristic_polynomial(),
            RationalPolynomial::from(a.characteristic_polynomial())
        );
    });
}
