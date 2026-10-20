// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Fredrik Johansson
//
//      Copyright © 2016 Marco Bodrato
//
//      Copyright © 2021 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::arithmetic::mul::double_word::{
    DoubleWordRows, matrix_mul_double_word_internal,
};
use crate::integer_matrix::arithmetic::mul::strassen::{
    BlockRef, add_assign_block, block, reverse_sub_assign, set_block, set_sum,
};
use crate::integer_matrix::arithmetic::mul::{ProductShape, matrix_mul};
use crate::integer_matrix::arithmetic::square::matrix_square;
use crate::integer_polynomial::arithmetic::coefficient::{PolynomialCoefficient, small_value};
use crate::integer_vector::arithmetic::add::vec_add_assign;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::integer_vector::arithmetic::sub::vec_sub_assign;
use crate::natural::TWICE_WIDTH;
use crate::platform::SignedDoubleLimb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::AddMulAssign;
use malachite_base::num::basic::traits::Zero;

// The values of the elements `xs` as signed double words, if FLINT would store all of them as small
// `fmpz`s.
fn small_double_values<C: PolynomialCoefficient, const N: usize>(
    xs: &[C],
) -> Option<[SignedDoubleLimb; N]> {
    let mut out = [0; N];
    for (y, x) in out.iter_mut().zip(xs) {
        *y = SignedDoubleLimb::from(small_value(x)?);
    }
    Some(out)
}

// Returns the elements of the square of the 2 by 2 matrix with elements `xs`.
fn square_2<C: PolynomialCoefficient>(xs: &[C]) -> Vec<C> {
    if let Some([a, b, c, d]) = small_double_values(xs) {
        // Every element is less than $2^{W-2}$ in absolute value, where $W$ is the limb width, so
        // every sum below fits in a signed double word.
        let u = b * c;
        let t = a + d;
        vec![
            C::exact_from(a * a + u),
            C::exact_from(t * b),
            C::exact_from(t * c),
            C::exact_from(d * d + u),
        ]
    } else {
        let [a, b, c, d] = xs else {
            unreachable!();
        };
        let t = b.mul_ref(c);
        let mut b_00 = a.square_ref();
        b_00 += &t;
        let mut b_11 = d.square_ref();
        b_11 += &t;
        let t = a.add_ref(d);
        vec![b_00, b.mul_ref(&t), c.mul_ref(&t), b_11]
    }
}

// Returns the elements of the square of the 3 by 3 matrix with elements `xs`.
fn square_3<C: PolynomialCoefficient>(xs: &[C]) -> Vec<C> {
    if let Some([a_00, a_01, a_02, a_10, a_11, a_12, a_20, a_21, a_22]) = small_double_values(xs) {
        // Every element is less than $2^{W-2}$ in absolute value, where $W$ is the limb width, so
        // every sum below fits in a signed double word.
        let mut s = a_02 * a_20;
        let mut u = a_01 * a_10;
        let b_00 = a_00 * a_00 + u + s;
        let j = a_12 * a_21;
        u += j;
        s += j;
        let b_11 = a_11 * a_11 + u;
        let b_22 = a_22 * a_22 + s;
        let s = a_00 + a_11;
        let t = a_00 + a_22;
        let u = a_11 + a_22;
        [
            b_00,
            s * a_01 + a_02 * a_21,
            t * a_02 + a_01 * a_12,
            s * a_10 + a_20 * a_12,
            b_11,
            u * a_12 + a_10 * a_02,
            t * a_20 + a_21 * a_10,
            u * a_21 + a_01 * a_20,
            b_22,
        ]
        .into_iter()
        .map(C::exact_from)
        .collect()
    } else {
        let (bits, sign) = vec_max_bits(xs);
        if bits + u64::from(sign) <= TWICE_WIDTH {
            let mut out = vec![C::ZERO; 9];
            matrix_mul_double_word_internal(
                &mut out,
                &DoubleWordRows::Elements(xs),
                xs,
                ProductShape {
                    ar: 3,
                    br: 3,
                    bc: 3,
                },
                sign,
                (bits << 1) + 2,
            );
            out
        } else {
            let [a_00, a_01, a_02, a_10, a_11, a_12, a_20, a_21, a_22] = xs else {
                unreachable!();
            };
            let mut b_22 = a_22.square_ref();
            let mut b_11 = a_11.square_ref();
            let mut b_00 = a_00.square_ref();
            let mut temp = a_01.mul_ref(a_10);
            b_00 += &temp;
            b_11 += &temp;
            temp = a_12.mul_ref(a_21);
            b_11 += &temp;
            b_22 += &temp;
            temp = a_02.mul_ref(a_20);
            b_22 += &temp;
            b_00 += &temp;
            temp = a_00.add_ref(a_11);
            let b_01 = temp.mul_ref(a_01).add_mul(a_02, a_21);
            let b_10 = temp.mul_ref(a_10).add_mul(a_20, a_12);
            temp = a_00.add_ref(a_22);
            let b_02 = temp.mul_ref(a_02).add_mul(a_01, a_12);
            let b_20 = temp.mul_ref(a_20).add_mul(a_21, a_10);
            temp = a_11.add_ref(a_22);
            let b_12 = temp.mul_ref(a_12).add_mul(a_10, a_02);
            let b_21 = temp.mul_ref(a_21).add_mul(a_01, a_20);
            vec![b_00, b_01, b_02, b_10, b_11, b_12, b_20, b_21, b_22]
        }
    }
}

// Returns the elements of the square of the `n` by `n` matrix with elements `xs`, where `n` is at
// least 4, with Bodrato's algorithm: the matrix is split into four blocks, whose square takes four
// squares and three products of blocks. An odd last row and column are handled separately.
fn square_bodrato_blocks(xs: &[Integer], n: usize) -> Vec<Integer> {
    let anr = n >> 1;
    let at = |row, column| BlockRef {
        xs,
        width: n,
        row,
        column,
    };
    let (a_11_ref, a_12_ref, a_21_ref, a_22_ref) = (at(0, 0), at(0, anr), at(anr, 0), at(anr, anr));
    let a_12 = block(xs, n, 0..anr, anr..anr << 1);
    let a_21 = block(xs, n, anr..anr << 1, 0..anr);
    let half_shape = ProductShape {
        ar: anr,
        br: anr,
        bc: anr,
    };
    let mut x_2 = vec![Integer::ZERO; anr * anr];
    set_sum(&mut x_2, anr, a_22_ref, a_12_ref, false);
    let mut c_21 = matrix_square(&x_2, anr);
    set_sum(&mut x_2, anr, a_22_ref, a_21_ref, true);
    let mut c_22 = matrix_square(&x_2, anr);
    add_assign_block(&mut x_2, anr, a_12_ref, false);
    let mut c_11 = matrix_square(&x_2, anr);
    add_assign_block(&mut x_2, anr, a_11_ref, true);
    let mut c_12 = matrix_mul(&x_2, &a_12, half_shape);
    let x_1 = matrix_mul(&a_12, &a_21, half_shape);
    vec_add_assign(&mut c_11, &x_1);
    // C12 = C11 - C12
    reverse_sub_assign(&mut c_12, &c_11);
    // C11 = C21 - C11
    reverse_sub_assign(&mut c_11, &c_21);
    c_21 = matrix_mul(&a_21, &x_2, half_shape);
    drop(x_2);
    // C21 = C11 - C21
    reverse_sub_assign(&mut c_21, &c_11);
    vec_sub_assign(&mut c_12, &c_22);
    vec_add_assign(&mut c_22, &c_11);
    // A11 is copied only now, once `x_2` has been freed.
    c_11 = matrix_square(&block(xs, n, 0..anr, 0..anr), anr);
    vec_add_assign(&mut c_11, &x_1);
    let mut out = vec![Integer::ZERO; n * n];
    set_block(&mut out, n, 0, 0, c_11, anr);
    set_block(&mut out, n, 0, anr, c_12, anr);
    set_block(&mut out, n, anr, 0, c_21, anr);
    set_block(&mut out, n, anr, anr, c_22, anr);
    if n > anr << 1 {
        let m = anr << 1;
        // The last column, except for its last element.
        let a_c = block(xs, n, 0..n, m..n);
        let c_c = matrix_mul(
            &xs[..m * n],
            &a_c,
            ProductShape {
                ar: m,
                br: n,
                bc: 1,
            },
        );
        set_block(&mut out, n, 0, m, c_c, 1);
        // The last row, the product of the last row of the matrix and the whole matrix. FLINT
        // multiplies by a window of the first `m` columns and gets the last element from the last
        // column; Malachite would have to copy those columns, so it computes the last element here
        // instead, at the cost of `n` products rather than `n * m` copies.
        let c_r = matrix_mul(
            &xs[m * n..],
            xs,
            ProductShape {
                ar: 1,
                br: n,
                bc: n,
            },
        );
        set_block(&mut out, n, m, 0, c_r, n);
        // The product of the last column and the last row, added to the top-left block.
        let last_row = &xs[m * n..m * n + m];
        for i in 0..m {
            let x = &xs[i * n + m];
            for (z, y) in out[i * n..i * n + m].iter_mut().zip(last_row) {
                z.add_mul_assign(x, y);
            }
        }
    }
    out
}

// Returns the elements of the square of the `n` by `n` matrix with elements `xs`, in row-major
// order. Dimensions up to 3 use explicit formulas, with fast paths for elements that FLINT would
// store as small `fmpz`s; larger dimensions use Bodrato's algorithm.
//
// This is equivalent to `fmpz_mat_sqr_bodrato` from `fmpz_mat/sqr_bodrato.c`, FLINT 3.6.0.
crate_test_fn! {matrix_square_bodrato<C: PolynomialCoefficient>(xs: &[C], n: usize) -> Vec<C> {
    match n {
        0 => Vec::new(),
        1 => vec![xs[0].square_ref()],
        2 => square_2(xs),
        3 => square_3(xs),
        _ => C::from_integers(square_bodrato_blocks(&C::as_integers(xs), n)),
    }
}}
