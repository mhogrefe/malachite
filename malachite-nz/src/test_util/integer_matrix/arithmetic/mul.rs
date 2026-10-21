// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::mul::ProductShape;
use crate::integer_matrix::arithmetic::mul::small::{
    matrix_mul_small_1, matrix_mul_small_2a, matrix_mul_small_2b, sign_extension, signed_mul,
    small_words, small_words_transposed,
};
use crate::integer_matrix::arithmetic::mul::strassen::matrix_mul_strassen;
use crate::integer_matrix::arithmetic::mul::waksman::matrix_mul_waksman;
use crate::integer_polynomial::arithmetic::coefficient::{
    PolynomialCoefficient, SMALL_FMPZ_BITCOUNT_MAX,
};
use crate::integer_vector::arithmetic::dot_general::coefficient_from_signed_triple;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::TWICE_WIDTH;
use crate::platform::{Limb, SignedDoubleLimb, SignedLimb};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{AddMulAssign, XXXAddYYYToZZZ};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

// Computes each element of the product as a sum of products, adding one product at a time.
pub fn integer_matrix_mul_naive(a: &IntegerMatrix, b: &IntegerMatrix) -> IntegerMatrix {
    let height = usize::exact_from(a.height());
    let inner = usize::exact_from(a.width());
    let width = usize::exact_from(b.width());
    assert_eq!(a.width(), b.height());
    let mut elements = Vec::with_capacity(height * width);
    for i in 0..height {
        for j in 0..width {
            let mut sum = Integer::ZERO;
            for k in 0..inner {
                sum.add_mul_assign(&a.elements[i * inner + k], &b.elements[k * width + j]);
            }
            elements.push(sum);
        }
    }
    IntegerMatrix::from_owned_elements(a.height(), b.width(), elements)
}

// Multiplies with whichever of `matrix_mul_small_1`, `matrix_mul_small_2a`, and
// `matrix_mul_small_2b` the bit count of the product calls for, as `fmpz_mat_mul` does for small
// dimensions. Every element of both matrices must fit in a small `fmpz`. If `column_reads`, FLINT's
// versions, which read the second matrix down its columns, are used instead.
fn small_unblocked<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    shape: ProductShape,
    column_reads: bool,
) -> Vec<C> {
    let ProductShape { ar, br, bc } = shape;
    let mut out = vec![C::ZERO; ar * bc];
    if ar == 0 || br == 0 || bc == 0 {
        return out;
    }
    let c_bits = vec_max_bits(xs).0 + vec_max_bits(ys).0 + u64::exact_from(br).significant_bits();
    let a_words = small_words(xs);
    if column_reads {
        let b_words = small_words(ys);
        if c_bits <= SMALL_FMPZ_BITCOUNT_MAX {
            matrix_mul_small_1_column_reads(&mut out, &a_words, &b_words, shape);
        } else if c_bits < TWICE_WIDTH {
            matrix_mul_small_2a_column_reads(&mut out, &a_words, &b_words, shape);
        } else {
            matrix_mul_small_2b_column_reads(&mut out, &a_words, &b_words, shape);
        }
    } else {
        let b_words = small_words_transposed(ys, shape);
        if c_bits <= SMALL_FMPZ_BITCOUNT_MAX {
            matrix_mul_small_1(&mut out, &a_words, &b_words, shape);
        } else if c_bits < TWICE_WIDTH {
            matrix_mul_small_2a(&mut out, &a_words, &b_words, shape);
        } else {
            matrix_mul_small_2b(&mut out, &a_words, &b_words, shape);
        }
    }
    out
}

// The library's unblocked kernels for small elements, chosen as `fmpz_mat_mul` chooses them.
pub fn matrix_mul_small_unblocked<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    shape: ProductShape,
) -> Vec<C> {
    small_unblocked(xs, ys, shape, false)
}

// FLINT's unblocked kernels for small elements, which read the second matrix down its columns.
pub fn matrix_mul_small_unblocked_column_reads<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    shape: ProductShape,
) -> Vec<C> {
    small_unblocked(xs, ys, shape, true)
}

// Sets `out` to the product of the `ar` by `br` matrix with signed-word elements `xs` and the `br`
// by `bc` matrix with signed-word elements `ys`, when every element of the product fits in a small
// `fmpz`.
//
// This is `_fmpz_mat_mul_small_1` from `fmpz_mat/mul.c`, FLINT 3.6.0, which reads the second matrix
// down its columns; the library reads a transposed copy instead.
pub fn matrix_mul_small_1_column_reads<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[SignedLimb],
    ys: &[SignedLimb],
    shape: ProductShape,
) {
    let ProductShape { ar, br, bc } = shape;
    for i in 0..ar {
        for j in 0..bc {
            let mut s: SignedLimb = 0;
            for k in 0..br {
                s += xs[i * br + k] * ys[k * bc + j];
            }
            out[i * bc + j] = C::exact_from(s);
        }
    }
}

// Sets `out` to the product of the `ar` by `br` matrix with signed-word elements `xs` and the `br`
// by `bc` matrix with signed-word elements `ys`, when every element of the product fits in a signed
// double word.
//
// This is `_fmpz_mat_mul_small_2a` from `fmpz_mat/mul.c`, FLINT 3.6.0, with the two words of the
// sum held as one double word, reading the second matrix down its columns.
pub fn matrix_mul_small_2a_column_reads<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[SignedLimb],
    ys: &[SignedLimb],
    shape: ProductShape,
) {
    let ProductShape { ar, br, bc } = shape;
    for i in 0..ar {
        for j in 0..bc {
            let mut s: SignedDoubleLimb = 0;
            for k in 0..br {
                s +=
                    SignedDoubleLimb::from(xs[i * br + k]) * SignedDoubleLimb::from(ys[k * bc + j]);
            }
            out[i * bc + j] = C::exact_from(s);
        }
    }
}

// Sets `out` to the product of the `ar` by `br` matrix with signed-word elements `xs` and the `br`
// by `bc` matrix with signed-word elements `ys`, summing in three words.
//
// This is `_fmpz_mat_mul_small_2b` from `fmpz_mat/mul.c`, FLINT 3.6.0, reading the second matrix
// down its columns.
pub fn matrix_mul_small_2b_column_reads<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[SignedLimb],
    ys: &[SignedLimb],
    shape: ProductShape,
) {
    let ProductShape { ar, br, bc } = shape;
    for i in 0..ar {
        for j in 0..bc {
            let (mut s_2, mut s_1, mut s_0): (Limb, Limb, Limb) = (0, 0, 0);
            for k in 0..br {
                let (p_1, p_0) = signed_mul(xs[i * br + k], ys[k * bc + j]);
                (s_2, s_1, s_0) =
                    Limb::xxx_add_yyy_to_zzz(s_2, s_1, s_0, sign_extension(p_1), p_1, p_0);
            }
            out[i * bc + j] = coefficient_from_signed_triple(s_2, s_1, s_0);
        }
    }
}

// Multiplies two `IntegerMatrix`s with the given kernel, which takes the elements and the shape.
pub fn integer_matrix_mul_with(
    a: &IntegerMatrix,
    b: &IntegerMatrix,
    f: impl Fn(&[Integer], &[Integer], ProductShape) -> Vec<Integer>,
) -> IntegerMatrix {
    assert_eq!(a.width(), b.height());
    let shape = ProductShape {
        ar: usize::exact_from(a.height()),
        br: usize::exact_from(a.width()),
        bc: usize::exact_from(b.width()),
    };
    IntegerMatrix::from_owned_elements(a.height(), b.width(), f(&a.elements, &b.elements, shape))
}

// Waksman's algorithm for any coefficients, through `Integer`s.
pub fn matrix_mul_waksman_any<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    shape: ProductShape,
) -> Vec<C> {
    C::from_integers(matrix_mul_waksman(
        &C::as_integers(xs),
        &C::as_integers(ys),
        shape,
    ))
}

// Strassen's algorithm for any coefficients, through `Integer`s.
pub fn matrix_mul_strassen_any<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    shape: ProductShape,
) -> Vec<C> {
    C::from_integers(matrix_mul_strassen(
        &C::as_integers(xs),
        &C::as_integers(ys),
        shape,
    ))
}
