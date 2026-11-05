// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2014 Ashish Kedia
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::FloorSqrt;
use crate::num::basic::traits::Zero;
use crate::num::conversion::traits::WrappingFrom;
use alloc::vec::Vec;

/// The arithmetic that evaluating a polynomial at a square matrix needs: square matrices over a
/// commutative ring, and the scalars of that ring.
///
/// This is not part of the public API; it is public so that `malachite-nz` and `malachite-q` can
/// evaluate their polynomials at matrices.
#[doc(hidden)]
pub trait MatrixEvaluationRing {
    type Scalar: PartialEq + Zero;
    type Matrix: Clone;

    /// The $n \times n$ zero matrix.
    fn zero_matrix(&self, n: u64) -> Self::Matrix;

    /// The $n \times n$ identity matrix.
    fn identity_matrix(&self, n: u64) -> Self::Matrix;

    fn is_zero_matrix(&self, a: &Self::Matrix) -> bool;

    fn mul(&self, a: &Self::Matrix, b: &Self::Matrix) -> Self::Matrix;

    /// Adds `c` times the identity matrix to `a`.
    fn add_scalar_assign(&self, a: &mut Self::Matrix, c: &Self::Scalar);

    /// Adds `c` times `b` to `a`.
    fn add_scaled_assign(&self, a: &mut Self::Matrix, b: &Self::Matrix, c: &Self::Scalar);
}

/// The value at the $n \times n$ matrix `c` of the polynomial with coefficients `coefficients`, in
/// ascending order, by Horner's rule.
///
/// This is not part of the public API; it is public so that `malachite-nz` and `malachite-q` can
/// evaluate their polynomials at matrices.
///
/// This is equivalent to `_nmod_poly_evaluate_mat_horner` from `nmod_poly/evaluate_mat.c`, FLINT
/// 3.6.0, except that it starts from the leading coefficient times `c`, saving a matrix
/// multiplication, and skips zero coefficients.
// Adds `c` times the identity matrix to `a`, skipping the pass when `c` is 0.
#[inline]
fn add_scalar_assign<R: MatrixEvaluationRing>(ring: &R, a: &mut R::Matrix, c: &R::Scalar) {
    if *c != R::Scalar::ZERO {
        ring.add_scalar_assign(a, c);
    }
}

// Adds `c` times `b` to `a`, skipping the pass when `c` is 0.
#[inline]
fn add_scaled_assign<R: MatrixEvaluationRing>(
    ring: &R,
    a: &mut R::Matrix,
    b: &R::Matrix,
    c: &R::Scalar,
) {
    if *c != R::Scalar::ZERO {
        ring.add_scaled_assign(a, b, c);
    }
}

#[doc(hidden)]
pub fn evaluate_at_matrix_horner<R: MatrixEvaluationRing>(
    ring: &R,
    coefficients: &[R::Scalar],
    c: &R::Matrix,
    n: u64,
) -> R::Matrix {
    let mut dest = ring.zero_matrix(n);
    let len = coefficients.len();
    if len == 0 {
        return dest;
    }
    if len == 1 || ring.is_zero_matrix(c) {
        ring.add_scalar_assign(&mut dest, &coefficients[0]);
        return dest;
    }
    // Starting from the leading coefficient times `c`, rather than times the identity matrix, saves
    // a matrix multiplication.
    add_scaled_assign(ring, &mut dest, c, &coefficients[len - 1]);
    add_scalar_assign(ring, &mut dest, &coefficients[len - 2]);
    for coefficient in coefficients[..len - 2].iter().rev() {
        dest = ring.mul(&dest, c);
        add_scalar_assign(ring, &mut dest, coefficient);
    }
    dest
}

/// The value at the $n \times n$ matrix `c` of the polynomial with coefficients `coefficients`, in
/// ascending order, by the method of Paterson and Stockmeyer, which needs about $2\sqrt{d}$ matrix
/// multiplications for a polynomial of degree $d$.
///
/// This is not part of the public API; it is public so that `malachite-nz` and `malachite-q` can
/// evaluate their polynomials at matrices.
///
/// This is equivalent to `nmod_poly_evaluate_mat_paterson_stockmeyer` from
/// `nmod_poly/evaluate_mat.c`, FLINT 3.6.0, except that when the number of coefficients is a
/// multiple of the block size it does not multiply the zero matrix by the largest power, and that
/// it skips zero coefficients.
#[doc(hidden)]
pub fn evaluate_at_matrix_paterson_stockmeyer<R: MatrixEvaluationRing>(
    ring: &R,
    coefficients: &[R::Scalar],
    c: &R::Matrix,
    n: u64,
) -> R::Matrix {
    let len = coefficients.len();
    let mut dest = ring.zero_matrix(n);
    if len == 0 {
        return dest;
    }
    if len == 1 || ring.is_zero_matrix(c) {
        ring.add_scalar_assign(&mut dest, &coefficients[0]);
        return dest;
    }
    let lim = len.floor_sqrt();
    // temp[i] = c^i for i <= lim
    let mut temp: Vec<R::Matrix> = Vec::with_capacity(lim + 1);
    temp.push(ring.identity_matrix(n));
    temp.push(c.clone());
    for i in 2..=lim {
        temp.push(ring.mul(&temp[i - 1], c));
    }
    // The coefficients split into blocks of `lim`, from the top, with the `len % lim` leftover
    // coefficients at the top; each block is evaluated with the powers in `temp`, and Horner's rule
    // in c^lim combines the blocks.
    let (blocks, top) = coefficients.split_at(len - len % lim);
    for (power, coefficient) in temp.iter().zip(top) {
        add_scaled_assign(ring, &mut dest, power, coefficient);
    }
    let mut blocks = blocks.rchunks_exact(lim);
    if top.is_empty() {
        // Multiplying the zero matrix by c^lim would be wasted, so the first block starts dest.
        for (power, coefficient) in temp.iter().zip(blocks.next().unwrap()) {
            add_scaled_assign(ring, &mut dest, power, coefficient);
        }
    }
    for block in blocks {
        dest = ring.mul(&dest, &temp[lim]);
        for (power, coefficient) in temp.iter().zip(block) {
            add_scaled_assign(ring, &mut dest, power, coefficient);
        }
    }
    dest
}

/// The value at the $n \times n$ matrix `c` of the polynomial with coefficients `coefficients`, in
/// ascending order: by Horner's rule for short polynomials and small matrices, and otherwise by the
/// method of Paterson and Stockmeyer.
///
/// This is not part of the public API; it is public so that `malachite-nz` and `malachite-q` can
/// evaluate their polynomials at matrices.
///
/// This is equivalent to `nmod_poly_evaluate_mat` from `nmod_poly.h`, FLINT 3.6.0.
#[doc(hidden)]
pub fn evaluate_at_matrix<R: MatrixEvaluationRing>(
    ring: &R,
    coefficients: &[R::Scalar],
    c: &R::Matrix,
    n: u64,
) -> R::Matrix {
    let len = coefficients.len();
    if len < 5 || n.saturating_mul(u64::wrapping_from(len)) < 425 {
        evaluate_at_matrix_horner(ring, coefficients, c, n)
    } else {
        evaluate_at_matrix_paterson_stockmeyer(ring, coefficients, c, n)
    }
}
