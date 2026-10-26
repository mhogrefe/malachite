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
    CharacteristicPolynomial, Determinant, Parity, Trace,
};
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_gen_var_1;
use std::panic::catch_unwind;

// Checks the characteristic polynomial by value and by reference, and against that of the
// `IntegerMatrix` with the same elements, and returns it.
fn verify(a: &NaturalMatrix) -> IntegerPolynomial {
    let p = a.characteristic_polynomial();
    assert!(p.is_valid());
    assert_eq!(a.clone().characteristic_polynomial(), p);
    assert_eq!(IntegerMatrix::from(a).characteristic_polynomial(), p);
    p
}

#[test]
fn test_characteristic_polynomial() {
    let test = |s, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        assert_eq!(verify(&a).to_string(), out);
    };
    // - dimension 0
    test("[]_{0,0}", "1");
    // - dimension 1
    test("[5]", "x-5");
    // - dimension 2
    test("[1, 2; 3, 4]", "x^2-5*x-2");
    // - a diagonal matrix
    test("[2, 0; 0, 3]", "x^2-5*x+6");
    // - dimension 3
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", "x^3-16*x^2-12*x+3");
    // - dimension 4: Berkowitz's algorithm
    test(
        "[2, 1, 0, 3; 1, 0, 4, 2; 0, 3, 1, 1; 1, 2, 0, 5]",
        "x^4-8*x^3-3*x^2+79*x-77",
    );
    // - dimension 9: the multimodular algorithm
    test(
        "[84, 91, 68, 84, 3, 52, 47, 72, 18; 77, 64, 55, 84, 37, 60, 44, 40, 38; 24, 7, 40, 1, 39, \
        96, 67, 82, 90; 91, 85, 82, 69, 1, 61, 0, 75, 29; 79, 67, 29, 76, 87, 65, 40, 40, 15; 63, \
        36, 42, 4, 1, 14, 61, 53, 13; 65, 9, 50, 61, 94, 8, 61, 90, 11; 90, 59, 81, 71, 80, 7, 27, \
        21, 61; 93, 22, 45, 62, 2, 2, 50, 29, 65]",
        "x^9-505*x^8+24500*x^7-1525904*x^6-25439738*x^5+1617218684*x^4-155092598039*x^3+2123540540\
        8578*x^2+1381457572139812*x+25955835699646415",
    );
}

#[test]
fn characteristic_polynomial_fail() {
    let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(a.characteristic_polynomial());
    let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    assert_panic!((&a).characteristic_polynomial());
}

#[test]
fn characteristic_polynomial_properties() {
    natural_matrix_gen_var_1().test_properties(|a| {
        let p = verify(&a);
        let n = a.height();
        // The polynomial is monic of degree n.
        assert_eq!(p.degree(), Some(n));
        assert_eq!(*p.leading_coefficient(), 1u32);
        // The constant coefficient is (-1)^n times the determinant, and the next-to-leading one is
        // the negative of the trace.
        let det = (&a).determinant();
        assert_eq!(*p.coefficient(0), if n.even() { det } else { -det });
        if n > 0 {
            assert_eq!(*p.coefficient(n - 1), -Integer::from(a.to_trace()));
        }
        // A matrix and its transpose have the same characteristic polynomial.
        assert_eq!(a.transpose().characteristic_polynomial(), p);
    });
}
