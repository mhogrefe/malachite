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
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::{
    integer_matrix_gen_var_2, integer_polynomial_integer_matrix_pair_gen_var_1,
    integer_polynomial_integer_pair_gen,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::evaluate_at_matrix::*;

fn verify(p: &IntegerPolynomial, a: &IntegerMatrix) -> IntegerMatrix {
    let b = p.evaluate(a);
    assert!(b.is_valid());
    assert_eq!(p.evaluate(a.clone()), b);
    assert_eq!(evaluate_at_matrix_horner_integer(p, a), b);
    assert_eq!(evaluate_at_matrix_paterson_stockmeyer_integer(p, a), b);
    assert_eq!(evaluate_at_matrix_naive_integer(p, a), b);
    b
}

#[test]
fn test_evaluate_at_matrix() {
    let test = |p, a, out| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let a = IntegerMatrix::from_str(a).unwrap();
        assert_eq!(verify(&p, &a).to_string(), out);
    };
    // - Horner and Paterson–Stockmeyer: len == 0
    // - dispatch: len < 5
    test("0", "[]_{0,0}", "[]_{0,0}");
    // - Horner and Paterson–Stockmeyer: ring.is_zero_matrix(c)
    test("-3*x+1", "[]_{0,0}", "[]_{0,0}");
    test("0", "[1, 2; 3, 4]", "[0, 0; 0, 0]");
    // - Horner and Paterson–Stockmeyer: len == 1
    test("-5", "[1, 2; 3, 4]", "[-5, 0; 0, -5]");
    // The value is the constant term times the identity matrix.
    test("3*x^2-x+4", "[0, 0; 0, 0]", "[4, 0; 0, 4]");
    // - Horner and Paterson–Stockmeyer: len > 1 && !ring.is_zero_matrix(c)
    // - Paterson–Stockmeyer: lim == 1
    // - Paterson–Stockmeyer: top.is_empty()
    test("x", "[1, -2; 3, 4]", "[1, -2; 3, 4]");
    // The characteristic polynomial of [1, 2; 3, 4] is x^2 - 5x - 2, which vanishes at the matrix.
    test("x^2-5*x-2", "[1, 2; 3, 4]", "[0, 0; 0, 0]");
    // A rotation by a quarter turn squares to -1.
    test("x^2+1", "[0, -1; 1, 0]", "[0, 0; 0, 0]");
    // A 1 x 1 matrix holds the value of the polynomial at its element.
    test("5*x^2-3*x+7", "[-6]", "[205]");
    // - Paterson–Stockmeyer: lim >= 2
    test(
        "x^3-2*x+1",
        "[1, -2, 0; 0, 1, 1; -1, 0, 2]",
        "[2, -2, -8; -4, 2, 5; -5, 8, 7]",
    );
    test(
        "x^2+1",
        "[100000000000000000000, 1; 0, -100000000000000000000]",
        "[10000000000000000000000000000000000000001, 0; \
        0, 10000000000000000000000000000000000000001]",
    );
}

#[test]
fn test_evaluate_at_matrix_long() {
    let test = |p, a, out| {
        let p = IntegerPolynomial::from_str(p).unwrap();
        let a = IntegerMatrix::from_str(a).unwrap();
        assert_eq!(verify(&p, &a).to_string(), out);
    };
    // - Paterson–Stockmeyer: !top.is_empty()
    // - dispatch: len >= 5 && n * len < 425
    // A signed permutation matrix, whose powers have small entries.
    test(
        "3*x^69+2*x^68+x^67-x^65-2*x^64-3*x^63+3*x^62+2*x^61+x^60-x^58-2*x^57-3*x^56+3*x^55+2*x^54\
        +x^53-x^51-2*x^50-3*x^49+3*x^48+2*x^47+x^46-x^44-2*x^43-3*x^42+3*x^41+2*x^40+x^39-x^37-2*x\
        ^36-3*x^35+3*x^34+2*x^33+x^32-x^30-2*x^29-3*x^28+3*x^27+2*x^26+x^25-x^23-2*x^22-3*x^21+3*x\
        ^20+2*x^19+x^18-x^16-2*x^15-3*x^14+3*x^13+2*x^12+x^11-x^9-2*x^8-3*x^7+3*x^6+2*x^5+x^4-x^2-\
        2*x-3",
        "[0, 1, 0, 0, 0, 0; 0, 0, -1, 0, 0, 0; 1, 0, 0, 0, 0, 0; 0, 0, 0, 0, 1, 0; 0, 0, 0, 0, 0, \
        1; 0, 0, 0, -1, 0, 0]",
        "[6, 3, 3, 0, 0, 0; 3, 6, -3, 0, 0, 0; 3, -3, 6, 0, 0, 0; 0, 0, 0, 6, 3, -3; 0, 0, 0, 3, 6\
        , 3; 0, 0, 0, -3, 3, 6]",
    );
    // - dispatch: n * len >= 425
    test(
        "-3*x^70+3*x^69+2*x^68+x^67-x^65-2*x^64-3*x^63+3*x^62+2*x^61+x^60-x^58-2*x^57-3*x^56+3*x^5\
        5+2*x^54+x^53-x^51-2*x^50-3*x^49+3*x^48+2*x^47+x^46-x^44-2*x^43-3*x^42+3*x^41+2*x^40+x^39-\
        x^37-2*x^36-3*x^35+3*x^34+2*x^33+x^32-x^30-2*x^29-3*x^28+3*x^27+2*x^26+x^25-x^23-2*x^22-3*\
        x^21+3*x^20+2*x^19+x^18-x^16-2*x^15-3*x^14+3*x^13+2*x^12+x^11-x^9-2*x^8-3*x^7+3*x^6+2*x^5+\
        x^4-x^2-2*x-3",
        "[0, 1, 0, 0, 0, 0; 0, 0, -1, 0, 0, 0; 1, 0, 0, 0, 0, 0; 0, 0, 0, 0, 1, 0; 0, 0, 0, 0, 0, \
        1; 0, 0, 0, -1, 0, 0]",
        "[6, 6, 3, 0, 0, 0; 3, 6, -6, 0, 0, 0; 6, -3, 6, 0, 0, 0; 0, 0, 0, 6, 6, -3; 0, 0, 0, 3, 6\
        , 6; 0, 0, 0, -6, 3, 6]",
    );
}

#[test]
#[should_panic]
fn evaluate_at_matrix_fail() {
    // The matrix is not square.
    (&IntegerPolynomial::from_str("x+1").unwrap())
        .evaluate(&IntegerMatrix::from_str("[1, 2]").unwrap());
}

#[test]
fn evaluate_at_matrix_properties() {
    integer_polynomial_integer_matrix_pair_gen_var_1().test_properties(|(p, a)| {
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

    integer_polynomial_integer_pair_gen().test_properties(|(p, x)| {
        // At a 1 x 1 matrix, this is evaluation at the matrix's element.
        let y = (&p).evaluate(&x);
        assert_eq!(
            (&p).evaluate(IntegerMatrix::from_owned_elements(1, 1, vec![x])),
            IntegerMatrix::from_owned_elements(1, 1, vec![y])
        );
    });
}

#[test]
fn evaluate_at_matrix_cayley_hamilton_properties() {
    integer_matrix_gen_var_2().test_properties(|a| {
        // Every square matrix is a root of its characteristic polynomial.
        assert!((&(&a).characteristic_polynomial()).evaluate(&a).is_zero());
    });
}
