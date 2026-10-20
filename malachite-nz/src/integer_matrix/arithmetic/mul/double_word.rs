// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011, 2018 Fredrik Johansson
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
use crate::integer_matrix::arithmetic::mul::small::sign_extension;
use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::arithmetic::neg::limbs_neg_in_place;
use crate::natural::{TWICE_WIDTH, WIDTH_MINUS_1};
use crate::platform::{DoubleLimb, Limb, SignedDoubleLimb};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    XMulYToZZ, XXAddYYToZZ, XXSubYYToZZ, XXXAddYYYToZZZ, XXXXAddYYYYToZZZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, SplitInHalf, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;

// The element `x`, which must be less than $2^{2W}$ in absolute value, where $W$ is the limb width,
// as its two low limbs: the high limb and then the low limb. For a negative `x`, these are the
// limbs of its absolute value.
//
// This is equivalent to `fmpz_get_uiui` from `fmpz.h`, FLINT 3.6.0, for non-negative `x`.
fn get_uiui<C: PolynomialCoefficient>(x: &C) -> (Limb, Limb) {
    match *x.unsigned_abs_ref().as_limbs_asc() {
        [] => (0, 0),
        [x_0] => (0, x_0),
        [x_0, x_1] => (x_1, x_0),
        _ => panic!("the element does not fit in two limbs"),
    }
}

// The element `x`, which must be less than $2^{2W-1}$ in absolute value, where $W$ is the limb
// width, as the high and low limbs of its two's complement.
//
// This is equivalent to `fmpz_get_signed_uiui` from `fmpz.h`, FLINT 3.6.0.
fn get_signed_uiui<C: PolynomialCoefficient>(x: &C) -> (Limb, Limb) {
    let (x_1, x_0) = get_uiui(x);
    if x.is_negative() {
        Limb::xx_sub_yy_to_zz(0, 0, x_1, x_0)
    } else {
        (x_1, x_0)
    }
}

// Sets `x` to the element whose two's complement is the limbs `xs`, least significant first,
// reusing the limb vector of `x` when it has one, as FLINT reuses an `fmpz`'s `mpz`. `xs` is
// negated in place if it is negative.
//
// This is equivalent to `fmpz_set_signed_ui_array` from `fmpz.h`, FLINT 3.6.0.
fn set_signed_limbs<C: PolynomialCoefficient>(x: &mut C, xs: &mut [Limb]) {
    if xs.last().unwrap() >> WIDTH_MINUS_1 == 0 {
        x.assign_sign_and_limbs(true, xs);
    } else {
        limbs_neg_in_place(xs);
        x.assign_sign_and_limbs(false, xs);
    }
}

// Sets `x` to the element whose absolute value is the limbs `xs`, least significant first, reusing
// the limb vector of `x` when it has one.
//
// This is equivalent to `fmpz_set_ui_array` from `fmpz.h`, FLINT 3.6.0.
fn set_limbs<C: PolynomialCoefficient>(x: &mut C, xs: &[Limb]) {
    x.assign_sign_and_limbs(true, xs);
}

// The product of two limbs read as signed words, as the high and low limbs of its two's complement.
//
// This is equivalent to `smul_ppmm` from `longlong.h`, FLINT 3.6.0.
#[inline]
fn signed_mul_limbs(x: Limb, y: Limb) -> (Limb, Limb) {
    DoubleLimb::wrapping_from(
        SignedDoubleLimb::from(x.cast_signed()) * SignedDoubleLimb::from(y.cast_signed()),
    )
    .split_in_half()
}

// Below this many columns in the first matrix, the four-word signed kernel checks whether each pair
// of elements fits in single words, since its inner loop is too short to warm up.
//
// This is `FMPZ_MAT_MUL_4_BRANCHLESS_CUTOFF` from `fmpz_mat/mul_double_word.c`, FLINT 3.6.0.
const FMPZ_MAT_MUL_4_BRANCHLESS_CUTOFF: usize = 16;

// The number of bits in four limbs.
const QUADRUPLE_WIDTH: u64 = Limb::WIDTH << 2;

// Sets `out` to the product of a row `ar` of signed double-word elements, each stored as its low
// limb and then its high limb, and the matrix whose transpose is `b` in the same format, where
// every element of the product fits in four signed words. Pairs of elements that both fit in single
// words are multiplied with one signed multiplication.
//
// This is equivalent to `_do_row_22_4_signed_branchy` from `fmpz_mat/mul_double_word.c`, FLINT
// 3.6.0.
fn do_row_22_4_signed_branchy<C: PolynomialCoefficient>(
    out: &mut [C],
    ar: &[Limb],
    b: &[Limb],
    br: usize,
    bc: usize,
) {
    for (out_j, b) in out.iter_mut().take(bc).zip(b.chunks_exact(br << 1)) {
        let (mut t_3, mut t_2, mut t_1): (Limb, Limb, Limb) = (0, 0, 0);
        let (mut u_2, mut u_1, mut u_0): (Limb, Limb, Limb) = (0, 0, 0);
        for (a, b) in ar[..br << 1]
            .as_chunks::<2>()
            .0
            .iter()
            .zip(b.as_chunks::<2>().0)
        {
            let (a_0, a_1) = (a[0], a[1]);
            let (b_0, b_1) = (b[0], b[1]);
            if sign_extension(a_0) == a_1 && sign_extension(b_0) == b_1 {
                let (w_1, w_0) = signed_mul_limbs(b_0, a_0);
                (u_2, u_1, u_0) =
                    Limb::xxx_add_yyy_to_zzz(u_2, u_1, u_0, sign_extension(w_1), w_1, w_0);
            } else {
                (t_3, t_2) = Limb::xx_sub_yy_to_zz(t_3, t_2, 0, sign_extension(a_1) & b_0);
                (t_3, t_2) = Limb::xx_sub_yy_to_zz(t_3, t_2, 0, sign_extension(b_1) & a_0);
                let (w_3, w_2) = signed_mul_limbs(b_1, a_1);
                (t_3, t_2) = Limb::xx_add_yy_to_zz(t_3, t_2, w_3, w_2);
                let (w_1, w_0) = Limb::x_mul_y_to_zz(b_0, a_0);
                (u_2, u_1, u_0) = Limb::xxx_add_yyy_to_zzz(u_2, u_1, u_0, 0, w_1, w_0);
                let (w_2, w_1) = Limb::x_mul_y_to_zz(a_1, b_0);
                (t_3, t_2, t_1) = Limb::xxx_add_yyy_to_zzz(t_3, t_2, t_1, 0, w_2, w_1);
                let (w_2, w_1) = Limb::x_mul_y_to_zz(b_1, a_0);
                (t_3, t_2, t_1) = Limb::xxx_add_yyy_to_zzz(t_3, t_2, t_1, 0, w_2, w_1);
            }
        }
        let (s_3, s_2, s_1, s_0) =
            Limb::xxxx_add_yyyy_to_zzzz(t_3, t_2, t_1, 0, sign_extension(u_2), u_2, u_1, u_0);
        set_signed_limbs(out_j, &mut [s_0, s_1, s_2, s_3]);
    }
}

// Sets `out` to the product of a row `ar` of signed double-word elements and the matrix whose
// transpose is `b`, where every element of the product fits in four signed words.
//
// This is equivalent to `_do_row_22_4_signed` from `fmpz_mat/mul_double_word.c`, FLINT 3.6.0.
fn do_row_22_4_signed<C: PolynomialCoefficient>(
    out: &mut [C],
    ar: &[Limb],
    b: &[Limb],
    br: usize,
    bc: usize,
) {
    for (out_j, b) in out.iter_mut().take(bc).zip(b.chunks_exact(br << 1)) {
        let (mut t_3, mut t_2, mut t_1, mut t_0): (Limb, Limb, Limb, Limb) = (0, 0, 0, 0);
        let (mut u_2, mut u_1, mut u_0): (Limb, Limb, Limb) = (0, 0, 0);
        let (mut v_3, mut v_2): (Limb, Limb) = (0, 0);
        for (a, b) in ar[..br << 1]
            .as_chunks::<2>()
            .0
            .iter()
            .zip(b.as_chunks::<2>().0)
        {
            let (a_0, a_1) = (a[0], a[1]);
            let (b_0, b_1) = (b[0], b[1]);
            (t_3, t_2) = Limb::xx_sub_yy_to_zz(t_3, t_2, 0, sign_extension(a_1) & b_0);
            (t_3, t_2) = Limb::xx_sub_yy_to_zz(t_3, t_2, 0, sign_extension(b_1) & a_0);
            let (w_3, w_2) = signed_mul_limbs(b_1, a_1);
            (t_3, t_2) = Limb::xx_add_yy_to_zz(t_3, t_2, w_3, w_2);
            let (w_1, w_0) = Limb::x_mul_y_to_zz(b_0, a_0);
            (u_2, u_1, u_0) = Limb::xxx_add_yyy_to_zzz(u_2, u_1, u_0, 0, w_1, w_0);
            let (w_2, w_1) = Limb::x_mul_y_to_zz(a_1, b_0);
            (t_3, t_2, t_1) = Limb::xxx_add_yyy_to_zzz(t_3, t_2, t_1, 0, w_2, w_1);
            let (w_2, w_1) = Limb::x_mul_y_to_zz(b_1, a_0);
            (v_3, v_2, t_1) = Limb::xxx_add_yyy_to_zzz(v_3, v_2, t_1, 0, w_2, w_1);
        }
        (t_3, t_2, t_1, t_0) =
            Limb::xxxx_add_yyyy_to_zzzz(t_3, t_2, t_1, t_0, sign_extension(u_2), u_2, u_1, u_0);
        (t_3, t_2) = Limb::xx_add_yy_to_zz(t_3, t_2, v_3, v_2);
        set_signed_limbs(out_j, &mut [t_0, t_1, t_2, t_3]);
    }
}

// Sets `out` to the product of a row `ar` of signed double-word elements and the matrix whose
// transpose is `b`, where every element of the product fits in five signed words.
//
// This is equivalent to `_do_row_22_5_signed` from `fmpz_mat/mul_double_word.c`, FLINT 3.6.0.
fn do_row_22_5_signed<C: PolynomialCoefficient>(
    out: &mut [C],
    ar: &[Limb],
    b: &[Limb],
    br: usize,
    bc: usize,
) {
    for (out_j, b) in out.iter_mut().take(bc).zip(b.chunks_exact(br << 1)) {
        let (mut s_4, mut s_3, mut s_2, mut s_1, mut s_0): (Limb, Limb, Limb, Limb, Limb) =
            (0, 0, 0, 0, 0);
        let (mut u_3, mut u_2): (Limb, Limb) = (0, 0);
        let mut v_2: Limb = 0;
        for (a, b) in ar[..br << 1]
            .as_chunks::<2>()
            .0
            .iter()
            .zip(b.as_chunks::<2>().0)
        {
            let (a_0, a_1) = (a[0], a[1]);
            let (b_0, b_1) = (b[0], b[1]);
            (u_3, u_2) = Limb::xx_sub_yy_to_zz(u_3, u_2, 0, sign_extension(a_1) & b_0);
            (u_3, u_2) = Limb::xx_sub_yy_to_zz(u_3, u_2, 0, sign_extension(b_1) & a_0);
            let (p_3, p_2) = signed_mul_limbs(b_1, a_1);
            (s_4, s_3, s_2) =
                Limb::xxx_add_yyy_to_zzz(s_4, s_3, s_2, sign_extension(p_3), p_3, p_2);
            let (p_2, p_1) = Limb::x_mul_y_to_zz(a_1, b_0);
            (u_3, u_2, s_1) = Limb::xxx_add_yyy_to_zzz(u_3, u_2, s_1, 0, p_2, p_1);
            let (p_1, p_0) = Limb::x_mul_y_to_zz(b_0, a_0);
            (v_2, s_1, s_0) = Limb::xxx_add_yyy_to_zzz(v_2, s_1, s_0, 0, p_1, p_0);
            let (p_2, p_1) = Limb::x_mul_y_to_zz(b_1, a_0);
            (u_3, u_2, s_1) = Limb::xxx_add_yyy_to_zzz(u_3, u_2, s_1, 0, p_2, p_1);
        }
        (s_4, s_3, s_2) = Limb::xxx_add_yyy_to_zzz(s_4, s_3, s_2, sign_extension(u_3), u_3, u_2);
        (s_4, s_3, s_2) = Limb::xxx_add_yyy_to_zzz(s_4, s_3, s_2, 0, 0, v_2);
        set_signed_limbs(out_j, &mut [s_0, s_1, s_2, s_3, s_4]);
    }
}

// Sets `out` to the product of a row `ar` of non-negative double-word elements and the matrix whose
// transpose is `b`, where every element of the product fits in four words.
//
// This is equivalent to `_do_row_22_4_unsigned` from `fmpz_mat/mul_double_word.c`, FLINT 3.6.0.
fn do_row_22_4_unsigned<C: PolynomialCoefficient>(
    out: &mut [C],
    ar: &[Limb],
    b: &[Limb],
    br: usize,
    bc: usize,
) {
    for (out_j, b) in out.iter_mut().take(bc).zip(b.chunks_exact(br << 1)) {
        let (mut s_3, mut s_2, mut s_1, mut s_0): (Limb, Limb, Limb, Limb) = (0, 0, 0, 0);
        let (mut u_3, mut u_2, mut u_1): (Limb, Limb, Limb) = (0, 0, 0);
        let (mut v_3, mut v_2): (Limb, Limb) = (0, 0);
        for (a, b) in ar[..br << 1]
            .as_chunks::<2>()
            .0
            .iter()
            .zip(b.as_chunks::<2>().0)
        {
            let (a_0, a_1) = (a[0], a[1]);
            let (b_0, b_1) = (b[0], b[1]);
            let (p_2, p_1) = Limb::x_mul_y_to_zz(a_1, b_0);
            (u_3, u_2, u_1) = Limb::xxx_add_yyy_to_zzz(u_3, u_2, u_1, 0, p_2, p_1);
            let (p_3, p_2) = Limb::x_mul_y_to_zz(a_1, b_1);
            let (p_1, p_0) = Limb::x_mul_y_to_zz(a_0, b_0);
            (s_3, s_2, s_1, s_0) =
                Limb::xxxx_add_yyyy_to_zzzz(s_3, s_2, s_1, s_0, p_3, p_2, p_1, p_0);
            let (p_2, p_1) = Limb::x_mul_y_to_zz(a_0, b_1);
            (v_3, v_2, u_1) = Limb::xxx_add_yyy_to_zzz(v_3, v_2, u_1, 0, p_2, p_1);
        }
        (u_3, u_2) = Limb::xx_add_yy_to_zz(u_3, u_2, v_3, v_2);
        (s_3, s_2, s_1) = Limb::xxx_add_yyy_to_zzz(s_3, s_2, s_1, u_3, u_2, u_1);
        set_limbs(out_j, &[s_0, s_1, s_2, s_3]);
    }
}

// Sets `out` to the product of a row `ar` of non-negative double-word elements and the matrix whose
// transpose is `b`, where every element of the product fits in five words.
//
// This is equivalent to `_do_row_22_5_unsigned` from `fmpz_mat/mul_double_word.c`, FLINT 3.6.0.
fn do_row_22_5_unsigned<C: PolynomialCoefficient>(
    out: &mut [C],
    ar: &[Limb],
    b: &[Limb],
    br: usize,
    bc: usize,
) {
    for (out_j, b) in out.iter_mut().take(bc).zip(b.chunks_exact(br << 1)) {
        let (mut s_4, mut s_3, mut s_2, mut s_1, mut s_0): (Limb, Limb, Limb, Limb, Limb) =
            (0, 0, 0, 0, 0);
        let (mut u_2, mut u_1): (Limb, Limb) = (0, 0);
        let (mut v_3, mut v_2): (Limb, Limb) = (0, 0);
        for (a, b) in ar[..br << 1]
            .as_chunks::<2>()
            .0
            .iter()
            .zip(b.as_chunks::<2>().0)
        {
            let (a_0, a_1) = (a[0], a[1]);
            let (b_0, b_1) = (b[0], b[1]);
            let (p_2, p_1) = Limb::x_mul_y_to_zz(a_1, b_0);
            (s_3, s_2, s_1) = Limb::xxx_add_yyy_to_zzz(s_3, s_2, s_1, 0, p_2, p_1);
            let (p_1, p_0) = Limb::x_mul_y_to_zz(b_0, a_0);
            (u_2, u_1, s_0) = Limb::xxx_add_yyy_to_zzz(u_2, u_1, s_0, 0, p_1, p_0);
            let (p_2, p_1) = Limb::x_mul_y_to_zz(b_1, a_0);
            (s_3, s_2, s_1) = Limb::xxx_add_yyy_to_zzz(s_3, s_2, s_1, 0, p_2, p_1);
            let (p_3, p_2) = Limb::x_mul_y_to_zz(b_1, a_1);
            (s_4, v_3, v_2) = Limb::xxx_add_yyy_to_zzz(s_4, v_3, v_2, 0, p_3, p_2);
        }
        // `s_3` is small, so this doesn't overflow.
        (s_3, s_2, s_1) = Limb::xxx_add_yyy_to_zzz(s_3, s_2, s_1, 0, u_2, u_1);
        (s_4, s_3, s_2) = Limb::xxx_add_yyy_to_zzz(s_4, s_3, s_2, 0, v_3, v_2);
        set_limbs(out_j, &[s_0, s_1, s_2, s_3, s_4]);
    }
}

// Appends the elements of `xs` to `out` as double words, each stored as its low limb and then its
// high limb: in two's complement if `sign`, and otherwise as the limbs of the non-negative element.
fn push_double_words<C: PolynomialCoefficient>(out: &mut Vec<Limb>, xs: &[C], sign: bool) {
    for x in xs {
        let (x_1, x_0) = if sign {
            get_signed_uiui(x)
        } else {
            get_uiui(x)
        };
        out.push(x_0);
        out.push(x_1);
    }
}

// The elements of `xs` as double words, in the format of `push_double_words`.
pub(crate) fn double_words<C: PolynomialCoefficient>(xs: &[C], sign: bool) -> Vec<Limb> {
    let mut out = Vec::with_capacity(xs.len() << 1);
    push_double_words(&mut out, xs, sign);
    out
}

// The first matrix of a product, as the double-word algorithm reads it.
crate_test_enum! {
DoubleWordRows<'a, C> {
    // The elements, which are converted to double words one row at a time, as FLINT does.
    Elements(&'a [C]),
    // The elements already converted to double words, in the format of `double_words`.
    Converted(Vec<Limb>),
}}

// Sets `out` to the product of the first matrix, given as `xs`, and the matrix with elements `ys`.
// If `sign`, the elements of both matrices are less than $2^{2W-1}$ in absolute value, where $W$ is
// the limb width; otherwise they are non-negative and less than $2^{2W}$. Every element of the
// product is less than $2^{\text{bits}}$ in absolute value, and `bits + sign` is at most $5W$. Each
// element of the product is a dot product of double words summed in four or five words.
//
// This is equivalent to `_fmpz_mat_mul_double_word_internal` from `fmpz_mat/mul_double_word.c`,
// FLINT 3.6.0, with one thread.
crate_test_fn! {matrix_mul_double_word_internal<C: PolynomialCoefficient>(
    out: &mut [C],
    xs: &DoubleWordRows<C>,
    ys: &[C],
    shape: ProductShape,
    sign: bool,
    bits: u64,
) {
    let ProductShape { ar, br, bc } = shape;
    assert!(ar > 0 && br > 0 && bc > 0);
    let words = if bits + u64::from(sign) <= QUADRUPLE_WIDTH {
        4
    } else {
        5
    };
    // Transpose the second matrix into double words: element (i, j) goes to position j * br + i.
    //
    // This is `_red_worker` from `fmpz_mat/mul_double_word.c`.
    let mut bl: Vec<Limb> = vec![0; (br * bc) << 1];
    for j in 0..bc {
        for i in 0..br {
            let y = &ys[i * bc + j];
            let (y_1, y_0) = if sign { get_signed_uiui(y) } else { get_uiui(y) };
            bl[(j * br + i) << 1] = y_0;
            bl[((j * br + i) << 1) + 1] = y_1;
        }
    }
    // This is `_mul_worker` from `fmpz_mat/mul_double_word.c`.
    let mut al: Vec<Limb> = Vec::new();
    for (i, out_row) in out.chunks_exact_mut(bc).take(ar).enumerate() {
        let row: &[Limb] = match xs {
            DoubleWordRows::Elements(xs) => {
                al.clear();
                push_double_words(&mut al, &xs[i * br..(i + 1) * br], sign);
                &al
            }
            DoubleWordRows::Converted(words) => &words[(i * br) << 1..((i + 1) * br) << 1],
        };
        if sign {
            if words == 4 && br < FMPZ_MAT_MUL_4_BRANCHLESS_CUTOFF {
                do_row_22_4_signed_branchy(out_row, row, &bl, br, bc);
            } else if words == 4 {
                do_row_22_4_signed(out_row, row, &bl, br, bc);
            } else {
                do_row_22_5_signed(out_row, row, &bl, br, bc);
            }
        } else if words == 4 {
            do_row_22_4_unsigned(out_row, row, &bl, br, bc);
        } else {
            do_row_22_5_unsigned(out_row, row, &bl, br, bc);
        }
    }
}}

// Returns the elements of the product of the matrices with elements `xs` and `ys`, with
// `matrix_mul_double_word_internal`. If some element is negative, every element must be less than
// $2^{2W-1}$ in absolute value, where $W$ is the limb width; otherwise every element must be less
// than $2^{2W}$.
//
// This is equivalent to `_fmpz_mat_mul_double_word` from `fmpz_mat/mul_double_word.c`, FLINT 3.6.0.
crate_test_fn! {matrix_mul_double_word<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    shape: ProductShape,
) -> Vec<C> {
    let ProductShape { ar, br, bc } = shape;
    let mut out = vec![C::ZERO; ar * bc];
    if ar == 0 || br == 0 || bc == 0 {
        return out;
    }
    let (a_bits, a_negative) = vec_max_bits(xs);
    let (b_bits, b_negative) = vec_max_bits(ys);
    let sign = a_negative || b_negative;
    assert!(a_bits + u64::from(sign) <= TWICE_WIDTH);
    assert!(b_bits + u64::from(sign) <= TWICE_WIDTH);
    let c_bits = u64::exact_from(br).significant_bits() + a_bits + b_bits;
    matrix_mul_double_word_internal(
        &mut out,
        &DoubleWordRows::Elements(xs),
        ys,
        shape,
        sign,
        c_bits,
    );
    out
}}
