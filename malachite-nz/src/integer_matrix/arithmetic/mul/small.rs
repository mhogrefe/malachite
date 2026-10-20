// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011, 2018, 2024 Fredrik Johansson
//
//      Copyright © 2016 Aaditya Thakkar
//
//      Copyright © 2021 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::mul::ProductShape;
use crate::integer_polynomial::arithmetic::coefficient::{
    PolynomialCoefficient, SMALL_FMPZ_BITCOUNT_MAX, small_value,
};
use crate::integer_vector::arithmetic::dot_general::coefficient_from_signed_triple;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::TWICE_WIDTH;
use crate::platform::{DoubleLimb, Limb, SignedDoubleLimb, SignedLimb};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{Parity, XXXAddYYYToZZZ};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, JoinHalves, SplitInHalf, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;

// The elements of `xs`, each of which FLINT would store as a small `fmpz`, as signed words.
pub(crate) fn small_words<C: PolynomialCoefficient>(xs: &[C]) -> Vec<SignedLimb> {
    xs.iter().map(|x| small_value(x).unwrap()).collect()
}

// `FLINT_SIGN_EXT` from `flint.h`, FLINT 3.6.0: all ones if `x` is negative as a signed word, and
// zero otherwise.
#[inline]
pub(crate) const fn sign_extension(x: Limb) -> Limb {
    if x >> (Limb::WIDTH - 1) == 0 {
        0
    } else {
        Limb::MAX
    }
}

// The product of two signed words, as the high and low words of its two's complement.
//
// This is equivalent to `smul_ppmm` from `longlong.h`, FLINT 3.6.0.
#[inline]
pub(crate) fn signed_mul(x: SignedLimb, y: SignedLimb) -> (Limb, Limb) {
    DoubleLimb::wrapping_from(SignedDoubleLimb::from(x) * SignedDoubleLimb::from(y)).split_in_half()
}

// The elements of the `br` by `bc` matrix `ys`, each of which FLINT would store as a small `fmpz`,
// as signed words, transposed: element (k, j) goes to position `j * br + k`, so each column is
// contiguous.
pub(crate) fn small_words_transposed<C: PolynomialCoefficient>(
    ys: &[C],
    shape: ProductShape,
) -> Vec<SignedLimb> {
    let ProductShape { br, bc, .. } = shape;
    let mut out = Vec::with_capacity(br * bc);
    for j in 0..bc {
        for k in 0..br {
            out.push(small_value(&ys[k * bc + j]).unwrap());
        }
    }
    out
}

// Sets `out` to the product of the matrix with signed-word elements `xs` and the matrix whose
// transpose has signed-word elements `yts`, as `small_words_transposed` gives them, when every
// element of the product fits in a small `fmpz`.
//
// This is equivalent to `_fmpz_mat_mul_small_1` from `fmpz_mat/mul.c`, FLINT 3.6.0, except that the
// second matrix is read through its transpose, so that each dot product reads contiguous words.
crate_test_fn! {matrix_mul_small_1<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[SignedLimb],
    yts: &[SignedLimb],
    shape: ProductShape,
) {
    let ProductShape { br, bc, .. } = shape;
    for (row, out_row) in xs.chunks_exact(br).zip(out.chunks_exact_mut(bc)) {
        for (column, z) in yts.chunks_exact(br).zip(out_row.iter_mut()) {
            let mut s: SignedLimb = 0;
            for (&x, &y) in row.iter().zip(column) {
                s += x * y;
            }
            *z = C::exact_from(s);
        }
    }
}}

// Sets `out` to the product of the matrix with signed-word elements `xs` and the matrix whose
// transpose has signed-word elements `yts`, when every element of the product fits in a signed
// double word.
//
// This is equivalent to `_fmpz_mat_mul_small_2a` from `fmpz_mat/mul.c`, FLINT 3.6.0, with the two
// words of the sum held as one double word, and with the second matrix read through its transpose.
crate_test_fn! {matrix_mul_small_2a<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[SignedLimb],
    yts: &[SignedLimb],
    shape: ProductShape,
) {
    let ProductShape { br, bc, .. } = shape;
    for (row, out_row) in xs.chunks_exact(br).zip(out.chunks_exact_mut(bc)) {
        for (column, z) in yts.chunks_exact(br).zip(out_row.iter_mut()) {
            let mut s: SignedDoubleLimb = 0;
            for (&x, &y) in row.iter().zip(column) {
                s += SignedDoubleLimb::from(x) * SignedDoubleLimb::from(y);
            }
            *z = C::exact_from(s);
        }
    }
}}

// Sets `out` to the product of the matrix with signed-word elements `xs` and the matrix whose
// transpose has signed-word elements `yts`, summing in three words.
//
// This is equivalent to `_fmpz_mat_mul_small_2b` from `fmpz_mat/mul.c`, FLINT 3.6.0, except that
// the second matrix is read through its transpose.
crate_test_fn! {matrix_mul_small_2b<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[SignedLimb],
    yts: &[SignedLimb],
    shape: ProductShape,
) {
    let ProductShape { br, bc, .. } = shape;
    for (row, out_row) in xs.chunks_exact(br).zip(out.chunks_exact_mut(bc)) {
        for (column, z) in yts.chunks_exact(br).zip(out_row.iter_mut()) {
            let (mut s_2, mut s_1, mut s_0): (Limb, Limb, Limb) = (0, 0, 0);
            for (&x, &y) in row.iter().zip(column) {
                let (p_1, p_0) = signed_mul(x, y);
                (s_2, s_1, s_0) =
                    Limb::xxx_add_yyy_to_zzz(s_2, s_1, s_0, sign_extension(p_1), p_1, p_0);
            }
            *z = coefficient_from_signed_triple(s_2, s_1, s_0);
        }
    }
}}

// The dot product of `xs` and `ys`, when it fits in a small `fmpz`.
//
// This is equivalent to `_dot1` from `fmpz_mat/mul_small.c`, FLINT 3.6.0.
fn dot_1<C: PolynomialCoefficient>(xs: &[SignedLimb], ys: &[SignedLimb]) -> C {
    let mut s: SignedLimb = 0;
    for (&x, &y) in xs.iter().zip(ys) {
        s += x * y;
    }
    C::exact_from(s)
}

// The dot product of `xs` and `ys`, when it fits in a signed double word.
//
// This is equivalent to `_dot2` from `fmpz_mat/mul_small.c`, FLINT 3.6.0, with the two words of the
// sum held as one double word.
fn dot_2<C: PolynomialCoefficient>(xs: &[SignedLimb], ys: &[SignedLimb]) -> C {
    let mut s: SignedDoubleLimb = 0;
    for (&x, &y) in xs.iter().zip(ys) {
        s += SignedDoubleLimb::from(x) * SignedDoubleLimb::from(y);
    }
    C::exact_from(s)
}

// The dot product of `xs` and `ys`, summed in three words.
//
// This is equivalent to `_dot3` from `fmpz_mat/mul_small.c`, FLINT 3.6.0.
fn dot_3<C: PolynomialCoefficient>(xs: &[SignedLimb], ys: &[SignedLimb]) -> C {
    let (mut s_2, mut s_1, mut s_0): (Limb, Limb, Limb) = (0, 0, 0);
    for (&x, &y) in xs.iter().zip(ys) {
        let (p_1, p_0) = signed_mul(x, y);
        (s_2, s_1, s_0) = Limb::xxx_add_yyy_to_zzz(s_2, s_1, s_0, sign_extension(p_1), p_1, p_0);
    }
    coefficient_from_signed_triple(s_2, s_1, s_0)
}

// Adds the dot product of `xs` and `ys` to the signed word `s[0]`.
//
// This is equivalent to `_dot_add1` from `fmpz_mat/mul_small.c`, FLINT 3.6.0.
fn dot_add_1(s: &mut [Limb], xs: &[SignedLimb], ys: &[SignedLimb]) {
    let mut s_0 = SignedLimb::wrapping_from(s[0]);
    for (&x, &y) in xs.iter().zip(ys) {
        s_0 += x * y;
    }
    s[0] = Limb::wrapping_from(s_0);
}

// Adds the dot product of `xs` and `ys` to the signed double word `(s[1], s[0])`, alternating
// between two accumulators so that consecutive additions are independent.
//
// This is equivalent to `_dot_add2` from `fmpz_mat/mul_small.c`, FLINT 3.6.0, with each pair of
// words held as one double word.
fn dot_add_2(s: &mut [Limb], mut xs: &[SignedLimb], mut ys: &[SignedLimb]) {
    let len = xs.len();
    let mut s_acc = SignedDoubleLimb::wrapping_from(DoubleLimb::join_halves(s[1], s[0]));
    let mut t_acc: SignedDoubleLimb = if len.odd() {
        let t = SignedDoubleLimb::from(xs[0]) * SignedDoubleLimb::from(ys[0]);
        xs = &xs[1..];
        ys = &ys[1..];
        t
    } else {
        0
    };
    for i in 0..len >> 1 {
        s_acc += SignedDoubleLimb::from(xs[i << 1]) * SignedDoubleLimb::from(ys[i << 1]);
        t_acc +=
            SignedDoubleLimb::from(xs[(i << 1) + 1]) * SignedDoubleLimb::from(ys[(i << 1) + 1]);
    }
    s_acc += t_acc;
    (s[1], s[0]) = DoubleLimb::wrapping_from(s_acc).split_in_half();
}

// Adds the dot product of `xs` and `ys` to the signed triple word `(s[2], s[1], s[0])`, alternating
// between two accumulators so that consecutive additions are independent.
//
// This is equivalent to `_dot_add3` from `fmpz_mat/mul_small.c`, FLINT 3.6.0.
fn dot_add_3(s: &mut [Limb], mut xs: &[SignedLimb], mut ys: &[SignedLimb]) {
    let len = xs.len();
    let (mut s_2, mut s_1, mut s_0) = (s[2], s[1], s[0]);
    let (mut t_2, mut t_1, mut t_0) = if len.odd() {
        let (t_1, t_0) = signed_mul(xs[0], ys[0]);
        xs = &xs[1..];
        ys = &ys[1..];
        (sign_extension(t_1), t_1, t_0)
    } else {
        (0, 0, 0)
    };
    for i in 0..len >> 1 {
        let (p_1, p_0) = signed_mul(xs[i << 1], ys[i << 1]);
        (s_2, s_1, s_0) = Limb::xxx_add_yyy_to_zzz(s_2, s_1, s_0, sign_extension(p_1), p_1, p_0);
        let (p_1, p_0) = signed_mul(xs[(i << 1) + 1], ys[(i << 1) + 1]);
        (t_2, t_1, t_0) = Limb::xxx_add_yyy_to_zzz(t_2, t_1, t_0, sign_extension(p_1), p_1, p_0);
    }
    (s[2], s[1], s[0]) = Limb::xxx_add_yyy_to_zzz(s_2, s_1, s_0, t_2, t_1, t_0);
}

// The number of rows of the first matrix in a block, `m_blk_sz` in `fmpz_mat/mul_small.c`, FLINT
// 3.6.0.
const M_BLOCK_SIZE: usize = 16;

// The largest number of columns of the first matrix in a block, the `k_blk_sz` limit in
// `fmpz_mat/mul_small.c`, FLINT 3.6.0.
const K_BLOCK_SIZE_MAX: usize = 128;

// Sets `out` to the product of the `m` by `k` matrix with signed-word elements `xs` and the `k` by
// `n` matrix with elements `ys`, where the shape is `ProductShape { ar: m, br: k, bc: n }`, each of
// which FLINT would store as a small `fmpz`, where every element of the product has at most
// `c_bits` significant bits, and `c_bits` is at most `3 * Limb::WIDTH - 1`. The dot products are
// summed in one, two, or three words, as `c_bits` requires. The second matrix is transposed into
// words, in blocks of at most 128 rows; when the first matrix is wider than that, the product is
// built in blocks of 16 rows, adding the contribution of each block of 128 columns of the first
// matrix.
//
// This is equivalent to `_fmpz_mat_mul_small_internal` from `fmpz_mat/mul_small.c`, FLINT 3.6.0,
// with one thread and with the first matrix already converted to words.
crate_test_fn! {matrix_mul_small_internal<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &[SignedLimb],
    ys: &[C],
    shape: ProductShape,
    c_bits: u64,
) {
    let ProductShape { ar: m, br: k, bc: n } = shape;
    assert!(m > 0 && k > 0 && n > 0);
    let (k_blk_sz, k_blk_ct) = if k <= K_BLOCK_SIZE_MAX {
        (k, 1)
    } else {
        (K_BLOCK_SIZE_MAX, k.div_ceil(K_BLOCK_SIZE_MAX))
    };
    let words = if c_bits <= SMALL_FMPZ_BITCOUNT_MAX {
        1
    } else if c_bits < TWICE_WIDTH {
        2
    } else {
        3
    };
    // Transpose the second matrix into blocks: element (i, j) goes to row j of block i / k_blk_sz,
    // at position i % k_blk_sz.
    //
    // This is `_tr_worker` from `fmpz_mat/mul_small.c`.
    let mut bl: Vec<SignedLimb> = vec![0; n * k_blk_ct * k_blk_sz];
    let mut iq = 0;
    let mut ir = 0;
    for i in 0..k {
        for j in 0..n {
            bl[iq * n * k_blk_sz + j * k_blk_sz + ir] = small_value(&ys[i * n + j]).unwrap();
        }
        ir += 1;
        if ir >= k_blk_sz {
            iq += 1;
            ir = 0;
        }
    }
    // This is `_mul_worker` from `fmpz_mat/mul_small.c`.
    if k <= k_blk_sz {
        // There is no blocking overhead: the second matrix is fully transposed in `bl`.
        for h in 0..m {
            let row = &xs[h * k..(h + 1) * k];
            for j in 0..n {
                let column = &bl[j * k_blk_sz..j * k_blk_sz + k];
                out[h * n + j] = match words {
                    1 => dot_1(row, column),
                    2 => dot_2(row, column),
                    _ => dot_3(row, column),
                };
            }
        }
        return;
    }
    let mut ta: Vec<SignedLimb> = vec![0; M_BLOCK_SIZE * k_blk_sz];
    let mut tc: Vec<Limb> = vec![0; n * M_BLOCK_SIZE * words];
    let mut h = 0;
    while h < m {
        let hstop = (m - h).min(M_BLOCK_SIZE);
        // `tc` holds the block of the product made of rows `h` through `h + hstop - 1`.
        tc[..n * hstop * words].fill(0);
        let mut i = 0;
        while i < k {
            let istop = k_blk_sz.min(k - i);
            // Copy the block of the first matrix made of rows `h` through `h + hstop - 1` and
            // columns `i` through `i + istop - 1` into `ta`.
            for hh in 0..hstop {
                ta[hh * k_blk_sz..hh * k_blk_sz + istop]
                    .copy_from_slice(&xs[(h + hh) * k + i..(h + hh) * k + i + istop]);
            }
            // Add the contribution of this block to the block of the product.
            for j in 0..n {
                let column = &bl[i * n + j * k_blk_sz..i * n + j * k_blk_sz + istop];
                for hh in 0..hstop {
                    let row = &ta[hh * k_blk_sz..hh * k_blk_sz + istop];
                    let s = &mut tc[words * (hh + hstop * j)..words * (hh + hstop * j + 1)];
                    match words {
                        1 => dot_add_1(s, row, column),
                        2 => dot_add_2(s, row, column),
                        _ => dot_add_3(s, row, column),
                    }
                }
            }
            i += k_blk_sz;
        }
        // Copy the block of the product out.
        for j in 0..n {
            for hh in 0..hstop {
                let s = &tc[words * (hh + hstop * j)..words * (hh + hstop * j + 1)];
                out[(h + hh) * n + j] = match words {
                    1 => C::exact_from(SignedLimb::wrapping_from(s[0])),
                    2 => C::exact_from(SignedDoubleLimb::wrapping_from(DoubleLimb::join_halves(
                        s[1], s[0],
                    ))),
                    _ => coefficient_from_signed_triple(s[2], s[1], s[0]),
                };
            }
        }
        h += M_BLOCK_SIZE;
    }
}}

// Returns the elements of the product of the matrices with elements `xs` and `ys`, each of which
// FLINT would store as a small `fmpz`, with `matrix_mul_small_internal`.
//
// This is equivalent to `_fmpz_mat_mul_small` from `fmpz_mat/mul_small.c`, FLINT 3.6.0.
crate_test_fn! {matrix_mul_small<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    shape: ProductShape,
) -> Vec<C> {
    let ProductShape { ar, br, bc } = shape;
    let mut out = vec![C::ZERO; ar * bc];
    if ar == 0 || br == 0 || bc == 0 {
        return out;
    }
    let c_bits = vec_max_bits(xs).0 + vec_max_bits(ys).0 + u64::exact_from(br).significant_bits();
    matrix_mul_small_internal(&mut out, &small_words(xs), ys, shape, c_bits);
    out
}}
