// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2012 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::{ModMulOp, ProductShape};
use crate::unsigned_vector::arithmetic::mod_dot::{ModData, dot_sum, dot_sum_strided};
use alloc::vec;

// Below this dimension, the classical algorithm reads the second matrix's columns in place rather
// than transposing it first.
//
// This is `NMOD_MAT_MUL_TRANSPOSE_CUTOFF` from `nmod_mat.h`, FLINT 3.6.0.
const NMOD_MAT_MUL_TRANSPOSE_CUTOFF: usize = 20;

// Writes the reduced dot product `c` to `z`, adding it to the value already there if `ADD`. Making
// `ADD` a constant parameter compiles each kernel once for each case, so the inner loops do not
// test it.
#[inline]
fn write<T: PrimitiveUnsigned, const ADD: bool>(z: &mut T, c: T, m: T) {
    *z = if ADD { z.mod_add(c, m) } else { c };
}

// Applies `op` to `out` and the product of the matrices with elements `xs` and `ys` modulo `m`,
// reading each column of the second matrix in place, at a stride.
//
// This is equivalent to `_nmod_mat_addmul_basic_op` from `nmod_mat/mul_classical.c`, FLINT 3.6.0,
// with the output the same as the matrix that is added to.
fn mod_mul_basic<T: PrimitiveUnsigned, const ADD: bool>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    shape: ProductShape,
    m: T,
    d: &ModData<T>,
) {
    let ProductShape { br: k, bc: n, .. } = shape;
    for (row, out_row) in xs.chunks_exact(k).zip(out.chunks_exact_mut(n)) {
        for (j, z) in out_row.iter_mut().enumerate() {
            write::<T, ADD>(z, d.reduce_sum(dot_sum_strided(row, &ys[j..], n, d)), m);
        }
    }
}

// Applies `op` to `out` and the product of the matrices with elements `xs` and `ys` modulo `m`,
// transposing the second matrix first so that each dot product reads contiguous words.
//
// This is equivalent to `_nmod_mat_addmul_transpose_op` from `nmod_mat/mul_classical.c`, FLINT
// 3.6.0, with the output the same as the matrix that is added to.
fn mod_mul_transposed<T: PrimitiveUnsigned, const ADD: bool>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    shape: ProductShape,
    m: T,
    d: &ModData<T>,
) {
    let ProductShape { br: k, bc: n, .. } = shape;
    let mut tmp = vec![T::ZERO; k * n];
    for i in 0..k {
        for j in 0..n {
            tmp[j * k + i] = ys[i * n + j];
        }
    }
    for (row, out_row) in xs.chunks_exact(k).zip(out.chunks_exact_mut(n)) {
        for (column, z) in tmp.chunks_exact(k).zip(out_row.iter_mut()) {
            write::<T, ADD>(z, d.reduce_sum(dot_sum(row, column, d)), m);
        }
    }
}

// Applies `op` to `out` and the product of the matrices with elements `xs` and `ys` modulo `m`,
// when each dot product fits in one word unreduced. Several elements of each row of the second
// matrix are packed into one word, in fields wide enough for a whole dot product, so that one
// multiplication and addition of words computes several dot products at once.
//
// This is equivalent to `_nmod_mat_addmul_packed_op` from `nmod_mat/mul_classical.c`, FLINT 3.6.0,
// with the output the same as the matrix that is added to.
fn mod_mul_packed<T: PrimitiveUnsigned, const ADD: bool>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    shape: ProductShape,
    m: T,
) {
    // FLINT names the dimensions `M`, `N`, and `K`.
    let ProductShape {
        ar: big_m,
        br: big_n,
        bc: big_k,
    } = shape;
    // A bound on an unreduced dot product.
    let c = T::wrapping_from(big_n) * (m - T::ONE) * (m - T::ONE);
    let pack_bits = c.significant_bits();
    let pack = usize::exact_from(T::WIDTH / pack_bits);
    let k_pack = big_k.div_ceil(pack);
    let mask = T::low_mask(pack_bits);
    // Pack and transpose the second matrix.
    let mut tmp = vec![T::ZERO; k_pack * big_n];
    for i in 0..k_pack {
        for k in 0..big_n {
            let mut c = ys[k * big_k + i * pack];
            for j in 1..pack {
                if i * pack + j >= big_k {
                    break;
                }
                c |= ys[k * big_k + i * pack + j] << (pack_bits * u64::exact_from(j));
            }
            tmp[i * big_n + k] = c;
        }
    }
    // Multiply.
    for i in 0..big_m {
        let a_row = &xs[i * big_n..(i + 1) * big_n];
        for j in 0..k_pack {
            let t_row = &tmp[j * big_n..(j + 1) * big_n];
            let mut c = T::ZERO;
            for (&x, &y) in a_row.iter().zip(t_row) {
                c += x * y;
            }
            // Unpack and reduce.
            for k in 0..pack {
                if j * pack + k >= big_k {
                    break;
                }
                let d = ((c >> (u64::exact_from(k) * pack_bits)) & mask) % m;
                write::<T, ADD>(&mut out[i * big_k + j * pack + k], d, m);
            }
        }
    }
}

// Applies `op` to `out` and the product of the matrices with elements `xs` and `ys`, all reduced
// modulo `m`: with `ModMulOp::Set`, sets `out` to the product, and with `ModMulOp::Add`, adds the
// product to the elements already in `out`. The algorithm reads the second matrix's columns in
// place for small dimensions, transposes the second matrix first for larger ones, and packs several
// of its elements into each word when the dot products are small enough.
//
// This is equivalent to `_nmod_mat_mul_classical_op` from `nmod_mat/mul_classical.c`, FLINT 3.6.0,
// with the output the same as the matrix that is added to.
crate_test_fn! {matrix_mod_mul_classical_op<T: PrimitiveUnsigned>(
    out: &mut [T],
    op: ModMulOp,
    xs: &[T],
    ys: &[T],
    shape: ProductShape,
    m: T,
) {
    let ProductShape { ar, br: k, bc: n } = shape;
    if ar == 0 || n == 0 {
        // The product has no elements.
        return;
    }
    if k == 0 || m == T::ONE {
        if op == ModMulOp::Set {
            out.fill(T::ZERO);
        }
        return;
    }
    match op {
        ModMulOp::Set => mod_mul_classical::<T, false>(out, xs, ys, shape, m),
        ModMulOp::Add => mod_mul_classical::<T, true>(out, xs, ys, shape, m),
    }
}}

// The body of `matrix_mod_mul_classical_op`, for nonempty products with a positive inner dimension
// and a modulus greater than 1, adding to `out` if `ADD`.
fn mod_mul_classical<T: PrimitiveUnsigned, const ADD: bool>(
    out: &mut [T],
    xs: &[T],
    ys: &[T],
    shape: ProductShape,
    m: T,
) {
    let ProductShape { ar, br: k, bc: n } = shape;
    let d = ModData::new(m, k);
    if d.words == 1 && ar > 10 && k > 10 && n > 10 {
        mod_mul_packed::<T, ADD>(out, xs, ys, shape, m);
    } else if ar < NMOD_MAT_MUL_TRANSPOSE_CUTOFF
        || n < NMOD_MAT_MUL_TRANSPOSE_CUTOFF
        || k < NMOD_MAT_MUL_TRANSPOSE_CUTOFF
    {
        mod_mul_basic::<T, ADD>(out, xs, ys, shape, m, &d);
    } else {
        mod_mul_transposed::<T, ADD>(out, xs, ys, shape, m, &d);
    }
}
