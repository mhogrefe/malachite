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
use crate::rational_polynomial::arithmetic::evaluate_at_matrix::RationalMatrixRing;
use malachite_base::matrix::Matrix;
use malachite_base::polynomial::evaluate_at_matrix::{
    evaluate_at_matrix_horner, evaluate_at_matrix_paterson_stockmeyer,
};
use malachite_base::test_util::unsigned_polynomial::modular::mod_evaluate_at_matrix::*;
use malachite_nz::integer::Integer;

type Kernel = fn(&RationalMatrixRing, &[Integer], &RationalMatrix, u64) -> RationalMatrix;

fn with_kernel(p: &RationalPolynomial, a: &RationalMatrix, kernel: Kernel) -> RationalMatrix {
    let mut y = kernel(
        &RationalMatrixRing,
        p.numerator_ref().coefficients_asc(),
        a,
        a.height(),
    );
    let d = Rational::from(p.denominator_ref());
    for e in &mut y.elements {
        *e /= &d;
    }
    y
}

pub fn evaluate_at_matrix_naive_rational(
    p: &RationalPolynomial,
    a: &RationalMatrix,
) -> RationalMatrix {
    with_kernel(p, a, evaluate_at_matrix_naive::<RationalMatrixRing>)
}

pub fn evaluate_at_matrix_horner_rational(
    p: &RationalPolynomial,
    a: &RationalMatrix,
) -> RationalMatrix {
    with_kernel(p, a, evaluate_at_matrix_horner::<RationalMatrixRing>)
}

pub fn evaluate_at_matrix_paterson_stockmeyer_rational(
    p: &RationalPolynomial,
    a: &RationalMatrix,
) -> RationalMatrix {
    with_kernel(
        p,
        a,
        evaluate_at_matrix_paterson_stockmeyer::<RationalMatrixRing>,
    )
}
