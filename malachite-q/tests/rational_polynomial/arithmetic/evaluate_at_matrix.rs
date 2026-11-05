// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::linear_algebra::traits::CharacteristicPolynomial;
use malachite_base::polynomial::arithmetic::traits::{Evaluate, MulPowerOfX};
use malachite_nz::test_util::generators::integer_polynomial_integer_matrix_pair_gen_var_1;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    rational_matrix_gen_var_4, rational_polynomial_rational_matrix_pair_gen_var_1,
    rational_polynomial_rational_pair_gen,
};
use malachite_q::test_util::rational_polynomial::arithmetic::evaluate_at_matrix::*;

fn verify(p: &RationalPolynomial, a: &RationalMatrix) -> RationalMatrix {
    let b = p.evaluate(a);
    assert!(b.is_valid());
    assert_eq!(p.evaluate(a.clone()), b);
    assert_eq!(evaluate_at_matrix_horner_rational(p, a), b);
    assert_eq!(evaluate_at_matrix_paterson_stockmeyer_rational(p, a), b);
    assert_eq!(evaluate_at_matrix_naive_rational(p, a), b);
    b
}

#[test]
fn test_evaluate_at_matrix() {
    let test = |p, a, out| {
        let p = RationalPolynomial::from_str(p).unwrap();
        let a = RationalMatrix::from_str(a).unwrap();
        assert_eq!(verify(&p, &a).to_string(), out);
    };
    // - Horner and Paterson–Stockmeyer: len == 0
    // - dispatch: len < 5
    // - p.denominator == 1
    test("0", "[]_{0,0}", "[]_{0,0}");
    // - Horner and Paterson–Stockmeyer: ring.is_zero_matrix(c)
    // - p.denominator != 1
    test("-1/3*x+1", "[]_{0,0}", "[]_{0,0}");
    test("0", "[1, 2; 3, 4]", "[0, 0; 0, 0]");
    // - Horner and Paterson–Stockmeyer: len == 1
    test("-5/2", "[1, 2; 3, 4]", "[-5/2, 0; 0, -5/2]");
    // The value is the constant term times the identity matrix.
    test("3/4*x^2-x+4", "[0, 0; 0, 0]", "[4, 0; 0, 4]");
    // - Horner and Paterson–Stockmeyer: len > 1 && !ring.is_zero_matrix(c)
    // - Paterson–Stockmeyer: lim == 1
    // - Paterson–Stockmeyer: top.is_empty()
    test("x", "[1/2, -2; 3, 4]", "[1/2, -2; 3, 4]");
    // The characteristic polynomial of [1, 2; 3, 4] is x^2 - 5x - 2, which vanishes at the matrix.
    test("x^2-5*x-2", "[1, 2; 3, 4]", "[0, 0; 0, 0]");
    test("1/2*x^2+1/2", "[1, 1; 0, 1]", "[1, 1; 0, 1]");
    // (x - 1/2)^2 vanishes at a Jordan block with eigenvalue 1/2.
    test("x^2-x+1/4", "[1/2, 1; 0, 1/2]", "[0, 0; 0, 0]");
    // (x - 1/2)(x - 1/3) vanishes at a triangular matrix with those eigenvalues.
    test("x^2-5/6*x+1/6", "[1/2, 1/3; 0, 1/3]", "[0, 0; 0, 0]");
    // A 1 x 1 matrix holds the value of the polynomial at its element.
    test("2/3*x^2-1/5*x+7", "[-1/6]", "[952/135]");
    // - Paterson–Stockmeyer: lim >= 2
    test(
        "x^3+1/2*x",
        "[1/2, -1/3, 0; 0, 1, 1/7; -1, 0, 2]",
        "[71/168, -3/4, -1/6; -1/2, 65/42, 15/14; -23/4, 7/6, 190/21]",
    );
}

#[test]
fn test_evaluate_at_matrix_long() {
    let test = |p, a, out| {
        let p = RationalPolynomial::from_str(p).unwrap();
        let a = RationalMatrix::from_str(a).unwrap();
        assert_eq!(verify(&p, &a).to_string(), out);
    };
    // - Paterson–Stockmeyer: !top.is_empty()
    // - dispatch: len >= 5 && n * len < 425
    // A scaled signed permutation matrix, whose powers have small entries.
    test(
        "2*x^69+1/3*x^68-x^66-2/3*x^65+x^64+x^63-1/2*x^61-2*x^60+2/3*x^59+1/2*x^58-1/3*x^56-x^55+2\
        *x^54+1/3*x^53-x^51-2/3*x^50+x^49+x^48-1/2*x^46-2*x^45+2/3*x^44+1/2*x^43-1/3*x^41-x^40+2*x\
        ^39+1/3*x^38-x^36-2/3*x^35+x^34+x^33-1/2*x^31-2*x^30+2/3*x^29+1/2*x^28-1/3*x^26-x^25+2*x^2\
        4+1/3*x^23-x^21-2/3*x^20+x^19+x^18-1/2*x^16-2*x^15+2/3*x^14+1/2*x^13-1/3*x^11-x^10+2*x^9+1\
        /3*x^8-x^6-2/3*x^5+x^4+x^3-1/2*x-2",
        "[0, 1/2, 0, 0, 0, 0; 0, 0, -2, 0, 0, 0; 1, 0, 0, 0, 0, 0; 0, 0, 0, 0, 1, 0; 0, 0, 0, 0, 0\
        , 1; 0, 0, 0, -1, 0, 0]",
        "[-6, -3/4, -1, 0, 0, 0; -2, -6, 3, 0, 0, 0; -3/2, 1/2, -6, 0, 0, 0; 0, 0, 0, -6, -3/2, 1\
        ; 0, 0, 0, -1, -6, -3/2; 0, 0, 0, 3/2, -1, -6]",
    );
    // - dispatch: n * len >= 425
    test(
        "-x^70+2*x^69+1/3*x^68-x^66-2/3*x^65+x^64+x^63-1/2*x^61-2*x^60+2/3*x^59+1/2*x^58-1/3*x^56-\
        x^55+2*x^54+1/3*x^53-x^51-2/3*x^50+x^49+x^48-1/2*x^46-2*x^45+2/3*x^44+1/2*x^43-1/3*x^41-x^\
        40+2*x^39+1/3*x^38-x^36-2/3*x^35+x^34+x^33-1/2*x^31-2*x^30+2/3*x^29+1/2*x^28-1/3*x^26-x^25\
        +2*x^24+1/3*x^23-x^21-2/3*x^20+x^19+x^18-1/2*x^16-2*x^15+2/3*x^14+1/2*x^13-1/3*x^11-x^10+2\
        *x^9+1/3*x^8-x^6-2/3*x^5+x^4+x^3-1/2*x-2",
        "[0, 1/2, 0, 0, 0, 0; 0, 0, -2, 0, 0, 0; 1, 0, 0, 0, 0, 0; 0, 0, 0, 0, 1, 0; 0, 0, 0, 0, 0\
        , 1; 0, 0, 0, -1, 0, 0]",
        "[-6, -1/4, -1, 0, 0, 0; -2, -6, 1, 0, 0, 0; -1/2, 1/2, -6, 0, 0, 0; 0, 0, 0, -6, -1/2, 1\
        ; 0, 0, 0, -1, -6, -1/2; 0, 0, 0, 1/2, -1, -6]",
    );
}

#[test]
#[should_panic]
fn evaluate_at_matrix_fail() {
    // The matrix is not square.
    (&RationalPolynomial::from_str("x+1").unwrap())
        .evaluate(&RationalMatrix::from_str("[1, 2]").unwrap());
}

#[test]
fn evaluate_at_matrix_properties() {
    rational_polynomial_rational_matrix_pair_gen_var_1().test_properties(|(p, a)| {
        let b = verify(&p, &a);
        assert_eq!(b.height(), a.height());
        assert_eq!(b.width(), a.height());

        // p(A) commutes with A.
        assert_eq!(&b * &a, &a * &b);
        // (x * p)(A) = A * p(A).
        assert_eq!((&(&p).mul_power_of_x(1)).evaluate(&a), &a * &b);
        // (-p)(A) = -p(A).
        assert_eq!((&-&p).evaluate(&a), -b);
    });

    rational_polynomial_rational_pair_gen().test_properties(|(p, x)| {
        // At a 1 x 1 matrix, this is evaluation at the matrix's element.
        let y = (&p).evaluate(&x);
        assert_eq!(
            (&p).evaluate(RationalMatrix::from_owned_elements(1, 1, vec![x])),
            RationalMatrix::from_owned_elements(1, 1, vec![y])
        );
    });

    integer_polynomial_integer_matrix_pair_gen_var_1().test_properties(|(p, a)| {
        // For integer coefficients and elements, this agrees with the `IntegerPolynomial` version.
        assert_eq!(
            (&RationalPolynomial::from(&p)).evaluate(RationalMatrix::from(&a)),
            RationalMatrix::from((&p).evaluate(a))
        );
    });
}

#[test]
fn evaluate_at_matrix_cayley_hamilton_properties() {
    rational_matrix_gen_var_4().test_properties(|a| {
        // Every square matrix is a root of its characteristic polynomial.
        assert!((&(&a).characteristic_polynomial()).evaluate(&a).is_zero());
    });
}
