// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use crate::rational_polynomial::RationalPolynomial;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Determinant, Parity, PowerOf2};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;

// The characteristic polynomial of a square matrix from its principal minors: the coefficient of
// $x^{n-k}$ is $(-1)^k$ times the sum of the determinants of the $k \times k$ principal
// submatrices, so this takes time exponential in the dimension.
pub fn rational_matrix_characteristic_polynomial_naive(a: &RationalMatrix) -> RationalPolynomial {
    assert!(a.is_square(), "the matrix must be square");
    let n = usize::exact_from(a.height());
    let mut cp = vec![Rational::ZERO; n + 1];
    for subset in 0..u64::power_of_2(u64::exact_from(n)) {
        let indices: Vec<u64> = (0..u64::exact_from(n))
            .filter(|&i| (subset >> i).odd())
            .collect();
        let k = indices.len();
        let elements = indices
            .iter()
            .flat_map(|&i| indices.iter().map(move |&j| a.get(i, j).unwrap().clone()))
            .collect();
        let k_u64 = u64::exact_from(k);
        let minor = RationalMatrix::from_owned_elements(k_u64, k_u64, elements).determinant();
        if k_u64.odd() {
            cp[n - k] -= minor;
        } else {
            cp[n - k] += minor;
        }
    }
    RationalPolynomial::from_coefficients_asc(cp)
}

// The polynomial `p` evaluated at the square matrix `a`, by Horner's rule.
pub fn rational_polynomial_evaluate_at_matrix(
    p: &RationalPolynomial,
    a: &RationalMatrix,
) -> RationalMatrix {
    let n = a.height();
    let mut result = RationalMatrix::zero(n, n);
    for c in p.to_coefficients_asc().iter().rev() {
        result = &result * a;
        for i in 0..n {
            *result.get_mut(i, i).unwrap() += c;
        }
    }
    result
}
