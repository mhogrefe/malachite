// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_characteristic_polynomial::{
    charpoly_danilevsky, charpoly_hessenberg,
};
use crate::natural_matrix::arithmetic::mod_determinant::charpoly_berkowitz;
use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModAddAssign, ModMul};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;

// The characteristic polynomial modulo `m` of a square matrix, by Berkowitz's division-free
// algorithm, which works for any modulus.
pub fn natural_matrix_mod_characteristic_polynomial_berkowitz(
    a: &NaturalMatrix,
    m: &Natural,
) -> NaturalPolynomial {
    let n = usize::exact_from(a.height());
    let mut cp = vec![Natural::ZERO; n + 1];
    charpoly_berkowitz(&mut cp, a.elements_ref(), n, m);
    NaturalPolynomial::from_coefficients_asc(cp)
}

// The characteristic polynomial modulo `m` of a square matrix, by Danilevsky's method on
// `Natural`s, or `None` if a pivot is not invertible modulo `m`.
pub fn natural_matrix_mod_characteristic_polynomial_danilevsky(
    a: &NaturalMatrix,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    let n = usize::exact_from(a.height());
    charpoly_danilevsky(a.elements_ref(), n, m).map(NaturalPolynomial::from_coefficients_asc)
}

// The characteristic polynomial modulo `m` of a square matrix, by reduction to Hessenberg form on
// `Natural`s, which works for any modulus.
pub fn natural_matrix_mod_characteristic_polynomial_hessenberg(
    a: &NaturalMatrix,
    m: &Natural,
) -> NaturalPolynomial {
    let n = usize::exact_from(a.height());
    NaturalPolynomial::from_coefficients_asc(charpoly_hessenberg(a.elements_ref(), n, m))
}

// The value modulo `m` of the polynomial `p`, whose coefficients are reduced modulo `m`, at the
// square matrix `a`, whose elements are reduced modulo `m`, by Horner's rule.
pub fn natural_polynomial_mod_evaluate_at_matrix(
    p: &NaturalPolynomial,
    a: &NaturalMatrix,
    m: &Natural,
) -> NaturalMatrix {
    assert!(a.is_square(), "the matrix must be square");
    let n = a.height();
    let k = usize::exact_from(n);
    let mut result = NaturalMatrix::zero(n, n);
    for c in p.coefficients_asc().iter().rev() {
        let mut elements = result.mod_mul(a, m).into_elements();
        for i in 0..k {
            elements[i * k + i].mod_add_assign(c, m);
        }
        result = NaturalMatrix::from_owned_elements(n, n, elements);
    }
    result
}
