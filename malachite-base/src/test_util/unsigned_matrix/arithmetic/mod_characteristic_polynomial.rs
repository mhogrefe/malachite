// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::ModMul;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::polynomial::Polynomial;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_characteristic_polynomial::{
    charpoly_danilevsky, charpoly_hessenberg,
};
use crate::unsigned_matrix::arithmetic::mod_determinant::charpoly_berkowitz;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec;

// The characteristic polynomial modulo `m` of a square matrix, by Berkowitz's division-free
// algorithm, which works for any modulus.
pub fn unsigned_matrix_mod_characteristic_polynomial_berkowitz<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    let n = usize::exact_from(a.height());
    let mut cp = vec![T::ZERO; n + 1];
    charpoly_berkowitz(&mut cp, a.elements_ref(), n, m);
    UnsignedPolynomial::from_coefficients_asc(cp)
}

// The characteristic polynomial modulo `m` of a square matrix, by Danilevsky's method, or `None` if
// a pivot is not invertible modulo `m`.
pub fn unsigned_matrix_mod_characteristic_polynomial_danilevsky<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    let n = usize::exact_from(a.height());
    charpoly_danilevsky(a.elements_ref(), n, m).map(UnsignedPolynomial::from_coefficients_asc)
}

// The characteristic polynomial modulo `m` of a square matrix, by reduction to Hessenberg form,
// which works for any modulus.
pub fn unsigned_matrix_mod_characteristic_polynomial_hessenberg<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    let n = usize::exact_from(a.height());
    UnsignedPolynomial::from_coefficients_asc(charpoly_hessenberg(a.elements_ref(), n, m))
}

// The value modulo `m` of the polynomial `p`, whose coefficients are reduced modulo `m`, at the
// square matrix `a`, whose elements are reduced modulo `m`, by Horner's rule.
pub fn unsigned_polynomial_mod_evaluate_at_matrix<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    a: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    assert!(a.is_square(), "the matrix must be square");
    let n = a.height();
    let k = usize::exact_from(n);
    let mut result = UnsignedMatrix::zero(n, n);
    for &c in p.coefficients_asc().iter().rev() {
        let mut elements = result.mod_mul(a, m).into_elements();
        for i in 0..k {
            elements[i * k + i].mod_add_assign(c, m);
        }
        result = UnsignedMatrix::from_owned_elements(n, n, elements);
    }
    result
}
