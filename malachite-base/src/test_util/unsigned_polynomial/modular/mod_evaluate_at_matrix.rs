// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::evaluate_at_matrix::{
    MatrixEvaluationRing, evaluate_at_matrix_horner, evaluate_at_matrix_paterson_stockmeyer,
};
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::modular::mod_evaluate_at_matrix::ModMatrixRing;
use crate::unsigned_polynomial::modular::mod_power_of_2_evaluate_at_matrix::ModPowerOf2MatrixRing;
use core::marker::PhantomData;

// The value at the n x n matrix `c` of the polynomial with coefficients `coefficients`, as the sum
// of each coefficient times the corresponding power of `c`, the powers being formed one by one.
pub fn evaluate_at_matrix_naive<R: MatrixEvaluationRing>(
    ring: &R,
    coefficients: &[R::Scalar],
    c: &R::Matrix,
    n: u64,
) -> R::Matrix {
    let mut sum = ring.zero_matrix(n);
    let mut power = ring.identity_matrix(n);
    for (i, coefficient) in coefficients.iter().enumerate() {
        if i != 0 {
            power = ring.mul(&power, c);
        }
        ring.add_scaled_assign(&mut sum, &power, coefficient);
    }
    sum
}

pub fn mod_evaluate_at_matrix_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    a: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    evaluate_at_matrix_naive(&ModMatrixRing { m }, p.coefficients_asc(), a, a.height())
}

pub fn mod_evaluate_at_matrix_horner<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    a: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    evaluate_at_matrix_horner(&ModMatrixRing { m }, p.coefficients_asc(), a, a.height())
}

pub fn mod_evaluate_at_matrix_paterson_stockmeyer<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    a: &UnsignedMatrix<T>,
    m: T,
) -> UnsignedMatrix<T> {
    evaluate_at_matrix_paterson_stockmeyer(
        &ModMatrixRing { m },
        p.coefficients_asc(),
        a,
        a.height(),
    )
}

const fn pow2_ring<T: PrimitiveUnsigned>(pow: u64) -> ModPowerOf2MatrixRing<T> {
    ModPowerOf2MatrixRing {
        pow,
        phantom: PhantomData,
    }
}

pub fn mod_power_of_2_evaluate_at_matrix_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    a: &UnsignedMatrix<T>,
    pow: u64,
) -> UnsignedMatrix<T> {
    evaluate_at_matrix_naive(&pow2_ring::<T>(pow), p.coefficients_asc(), a, a.height())
}

pub fn mod_power_of_2_evaluate_at_matrix_horner<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    a: &UnsignedMatrix<T>,
    pow: u64,
) -> UnsignedMatrix<T> {
    evaluate_at_matrix_horner(&pow2_ring::<T>(pow), p.coefficients_asc(), a, a.height())
}

pub fn mod_power_of_2_evaluate_at_matrix_paterson_stockmeyer<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    a: &UnsignedMatrix<T>,
    pow: u64,
) -> UnsignedMatrix<T> {
    evaluate_at_matrix_paterson_stockmeyer(
        &pow2_ring::<T>(pow),
        p.coefficients_asc(),
        a,
        a.height(),
    )
}
