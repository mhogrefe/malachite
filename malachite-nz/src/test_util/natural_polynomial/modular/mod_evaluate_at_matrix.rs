// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::modular::mod_evaluate_at_matrix::NaturalModMatrixRing;
use crate::natural_polynomial::modular::mod_power_of_2_evaluate_at_matrix::*;
use malachite_base::matrix::Matrix;
use malachite_base::polynomial::evaluate_at_matrix::{
    evaluate_at_matrix_horner, evaluate_at_matrix_paterson_stockmeyer,
};
use malachite_base::test_util::unsigned_polynomial::modular::mod_evaluate_at_matrix::*;

pub fn mod_evaluate_at_matrix_naive(
    p: &NaturalPolynomial,
    a: &NaturalMatrix,
    m: &Natural,
) -> NaturalMatrix {
    evaluate_at_matrix_naive(
        &NaturalModMatrixRing { m },
        p.coefficients_asc(),
        a,
        a.height(),
    )
}

pub fn mod_evaluate_at_matrix_horner(
    p: &NaturalPolynomial,
    a: &NaturalMatrix,
    m: &Natural,
) -> NaturalMatrix {
    evaluate_at_matrix_horner(
        &NaturalModMatrixRing { m },
        p.coefficients_asc(),
        a,
        a.height(),
    )
}

pub fn mod_evaluate_at_matrix_paterson_stockmeyer(
    p: &NaturalPolynomial,
    a: &NaturalMatrix,
    m: &Natural,
) -> NaturalMatrix {
    evaluate_at_matrix_paterson_stockmeyer(
        &NaturalModMatrixRing { m },
        p.coefficients_asc(),
        a,
        a.height(),
    )
}

pub fn mod_power_of_2_evaluate_at_matrix_naive(
    p: &NaturalPolynomial,
    a: &NaturalMatrix,
    pow: u64,
) -> NaturalMatrix {
    evaluate_at_matrix_naive(
        &NaturalModPowerOf2MatrixRing { pow },
        p.coefficients_asc(),
        a,
        a.height(),
    )
}

pub fn mod_power_of_2_evaluate_at_matrix_horner(
    p: &NaturalPolynomial,
    a: &NaturalMatrix,
    pow: u64,
) -> NaturalMatrix {
    evaluate_at_matrix_horner(
        &NaturalModPowerOf2MatrixRing { pow },
        p.coefficients_asc(),
        a,
        a.height(),
    )
}

pub fn mod_power_of_2_evaluate_at_matrix_paterson_stockmeyer(
    p: &NaturalPolynomial,
    a: &NaturalMatrix,
    pow: u64,
) -> NaturalMatrix {
    evaluate_at_matrix_paterson_stockmeyer(
        &NaturalModPowerOf2MatrixRing { pow },
        p.coefficients_asc(),
        a,
        a.height(),
    )
}
