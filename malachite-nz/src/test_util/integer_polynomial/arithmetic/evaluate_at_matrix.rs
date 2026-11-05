// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::evaluate_at_matrix::IntegerMatrixRing;
use malachite_base::matrix::Matrix;
use malachite_base::polynomial::evaluate_at_matrix::{
    evaluate_at_matrix_horner, evaluate_at_matrix_paterson_stockmeyer,
};
use malachite_base::test_util::unsigned_polynomial::modular::mod_evaluate_at_matrix::*;

pub fn evaluate_at_matrix_naive_integer(p: &IntegerPolynomial, a: &IntegerMatrix) -> IntegerMatrix {
    evaluate_at_matrix_naive(&IntegerMatrixRing, p.coefficients_asc(), a, a.height())
}

pub fn evaluate_at_matrix_horner_integer(
    p: &IntegerPolynomial,
    a: &IntegerMatrix,
) -> IntegerMatrix {
    evaluate_at_matrix_horner(&IntegerMatrixRing, p.coefficients_asc(), a, a.height())
}

pub fn evaluate_at_matrix_paterson_stockmeyer_integer(
    p: &IntegerPolynomial,
    a: &IntegerMatrix,
) -> IntegerMatrix {
    evaluate_at_matrix_paterson_stockmeyer(&IntegerMatrixRing, p.coefficients_asc(), a, a.height())
}
