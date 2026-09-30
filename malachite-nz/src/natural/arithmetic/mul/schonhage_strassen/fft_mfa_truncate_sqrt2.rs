// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009, 2011, 2020 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::mul::karatsuba::revbin;
use crate::natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use crate::natural::arithmetic::mul::schonhage_strassen::adjust_sqrt2::*;
use crate::natural::arithmetic::mul::schonhage_strassen::butterfly_lsh_b::limbs_butterfly_lsh_b;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_radix2::*;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_truncate_sqrt2::*;
use crate::natural::arithmetic::mul::schonhage_strassen::mul_2expmod_2expp1::*;
use crate::natural::arithmetic::neg::limbs_neg_in_place;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::CeilingLogBase2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;

// Sets `u[..=limbs]` to $2^{b_1}(s + t)$ and `v[..=limbs]` to $2^{b_2}(s - t)$ modulo
// $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH`, and $b_1$ and $b_2$ are less than
// $2 \cdot \text{limbs}\cdot\text{W}$.
//
// This is fft_butterfly_twiddle from fft/fft_mfa_truncate_sqrt2.c, FLINT 3.6.0.
crate_test_fn! {limbs_fft_butterfly_twiddle(
    u: &mut [Limb],
    v: &mut [Limb],
    s: &[Limb],
    t: &[Limb],
    limbs: usize,
    mut b1: u64,
    mut b2: u64,
) {
    let nw = u64::exact_from(limbs) << Limb::LOG_WIDTH;
    let negate2 = b1 >= nw;
    if negate2 {
        b1 -= nw;
    }
    let x = usize::exact_from(b1 >> Limb::LOG_WIDTH);
    let b1 = b1 & Limb::WIDTH_MASK;
    let negate1 = b2 >= nw;
    if negate1 {
        b2 -= nw;
    }
    let y = usize::exact_from(b2 >> Limb::LOG_WIDTH);
    let b2 = b2 & Limb::WIDTH_MASK;
    limbs_butterfly_lsh_b(u, v, s, t, limbs, x, y);
    limbs_mul_2exp_mod_2expp1_in_place(u, limbs, b1);
    if negate2 {
        limbs_neg_in_place(&mut u[..=limbs]);
    }
    limbs_mul_2exp_mod_2expp1_in_place(v, limbs, b2);
    if negate1 {
        limbs_neg_in_place(&mut v[..=limbs]);
    }
}}

// Applies `fft_radix2` to the residues `ii[0]`, `ii[is]`, `ii[2 * is]`, ..., additionally
// multiplying output `k` by $2^{\text{ws}\cdot r_k c}$, where the rows $r_k$ start at `r` and go up
// in steps of `rs`, in bit-reversed order.
//
// This is fft_radix2_twiddle from fft/fft_mfa_truncate_sqrt2.c, FLINT 3.6.0.
crate_test_fn! {fft_radix2_twiddle(
    ii: &mut [Vec<Limb>],
    is: usize,
    n: usize,
    w: u64,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
    ws: u64,
    r: usize,
    c: usize,
    rs: usize,
) {
    let limbs = fft_limbs(n, w);
    if n == 1 {
        let tw1 = r * c;
        let tw2 = tw1 + rs * c;
        limbs_fft_butterfly_twiddle(
            t1,
            t2,
            &ii[0],
            &ii[is],
            limbs,
            u64::exact_from(tw1) * ws,
            u64::exact_from(tw2) * ws,
        );
        swap(&mut ii[0], t1);
        swap(&mut ii[is], t2);
        return;
    }
    for i in 0..n {
        limbs_fft_butterfly(t1, t2, &ii[i * is], &ii[(n + i) * is], i, limbs, w);
        swap(&mut ii[i * is], t1);
        swap(&mut ii[(n + i) * is], t2);
    }
    fft_radix2_twiddle(ii, is, n >> 1, w << 1, t1, t2, ws, r, c, rs << 1);
    fft_radix2_twiddle(&mut ii[n * is..], is, n >> 1, w << 1, t1, t2, ws, r + rs, c, rs << 1);
}}

// Like `fft_radix2_twiddle`, but computing only the first `trunc` outputs, where `trunc` is at most
// `2 * n`, as if the inputs past the first `trunc` were zero; the inputs past `n` are used as they
// are.
//
// This is fft_truncate1_twiddle from fft/fft_mfa_truncate_sqrt2.c, FLINT 3.6.0.
crate_test_fn! {fft_truncate1_twiddle(
    ii: &mut [Vec<Limb>],
    is: usize,
    n: usize,
    w: u64,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
    ws: u64,
    r: usize,
    c: usize,
    rs: usize,
    trunc: usize,
) {
    let limbs = fft_limbs(n, w);
    if trunc == n << 1 {
        fft_radix2_twiddle(ii, is, n, w, t1, t2, ws, r, c, rs);
    } else if trunc <= n {
        for i in 0..n {
            let (ii_lo, ii_hi) = ii.split_at_mut((i + n) * is);
            limbs_slice_add_same_length_in_place_left(
                &mut ii_lo[i * is][..=limbs],
                &ii_hi[0][..=limbs],
            );
        }
        fft_truncate1_twiddle(ii, is, n >> 1, w << 1, t1, t2, ws, r, c, rs << 1, trunc);
    } else {
        for i in 0..n {
            limbs_fft_butterfly(t1, t2, &ii[i * is], &ii[(n + i) * is], i, limbs, w);
            swap(&mut ii[i * is], t1);
            swap(&mut ii[(n + i) * is], t2);
        }
        fft_radix2_twiddle(ii, is, n >> 1, w << 1, t1, t2, ws, r, c, rs << 1);
        fft_truncate1_twiddle(
            &mut ii[n * is..],
            is,
            n >> 1,
            w << 1,
            t1,
            t2,
            ws,
            r + rs,
            c,
            rs << 1,
            trunc - n,
        );
    }
}}

// The column transforms of a matrix Fourier version of `fft_truncate_sqrt2`, for a transform of
// length `4 * n` whose inputs past the first `trunc` are zero. The residues are viewed as two
// matrices of `2 * n / n1` rows and `n1` columns; the row transforms are left to
// `fft_mfa_truncate_sqrt2_inner`.
//
// This is fft_mfa_truncate_sqrt2_outer from fft/fft_mfa_truncate_sqrt2_inner.c, FLINT 3.6.0,
// without the threading.
crate_test_fn! {fft_mfa_truncate_sqrt2_outer(
    ii: &mut [Vec<Limb>],
    n: usize,
    w: u64,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
    temp: &mut [Limb],
    n1: usize,
    trunc: usize,
) {
    let n2 = (n << 1) / n1;
    let trunc2 = (trunc - (n << 1)) / n1;
    let limbs = fft_limbs(n, w);
    let depth = n2.ceiling_log_base_2();
    let two_n = n << 1;
    // first half matrix fourier FFT : n2 rows, n1 cols
    //
    // FFTs on columns
    for i in 0..n1 {
        // relevant part of first layer of full sqrt2 FFT
        let mut j = i;
        while j < trunc - two_n {
            limbs_fft_butterfly_sqrt2_power(t1, t2, &ii[j], &ii[two_n + j], j, limbs, w, temp);
            swap(&mut ii[j], t1);
            swap(&mut ii[two_n + j], t2);
            j += n1;
        }
        let (ii_lo, ii_hi) = ii.split_at_mut(two_n);
        while j < two_n {
            limbs_fft_adjust_sqrt2_power(&mut ii_hi[j], &ii_lo[j], j, limbs, w, temp);
            j += n1;
        }
        // FFT of length n2 on column i, applying z^{r*i} for rows going up in steps of 1 starting
        // at row 0, where z => w bits
        fft_radix2_twiddle(&mut ii[i..], n1, n2 >> 1, w * u64::exact_from(n1), t1, t2, w, 0, i, 1);
        for j in 0..n2 {
            let s = revbin(j, depth);
            if j < s {
                ii.swap(i + j * n1, i + s * n1);
            }
        }
    }
    // second half matrix fourier FFT : n2 rows, n1 cols
    let ii = &mut ii[two_n..];
    // FFTs on columns
    for i in 0..n1 {
        // FFT of length n2 on column i, applying z^{r*i} for rows going up in steps of 1 starting
        // at row 0, where z => w bits
        fft_truncate1_twiddle(
            &mut ii[i..],
            n1,
            n2 >> 1,
            w * u64::exact_from(n1),
            t1,
            t2,
            w,
            0,
            i,
            1,
            trunc2,
        );
        for j in 0..n2 {
            let s = revbin(j, depth);
            if j < s {
                ii.swap(i + j * n1, i + s * n1);
            }
        }
    }
}}
