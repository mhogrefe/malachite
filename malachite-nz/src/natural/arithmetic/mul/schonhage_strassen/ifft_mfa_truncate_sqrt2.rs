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
use crate::natural::arithmetic::mul::schonhage_strassen::adjust::limbs_fft_adjust;
use crate::natural::arithmetic::mul::schonhage_strassen::adjust_sqrt2::*;
use crate::natural::arithmetic::mul::schonhage_strassen::butterfly_rsh_b::limbs_butterfly_rsh_b;
use crate::natural::arithmetic::mul::schonhage_strassen::div_2expmod_2expp1::*;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_radix2::fft_limbs;
use crate::natural::arithmetic::mul::schonhage_strassen::ifft_radix2::limbs_ifft_butterfly;
use crate::natural::arithmetic::mul::schonhage_strassen::ifft_truncate_sqrt2::*;
use crate::natural::arithmetic::mul::schonhage_strassen::normmod_2expp1::limbs_norm_mod_2expp1;
use crate::natural::arithmetic::neg::limbs_neg_in_place;
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural::arithmetic::sub::{
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
};
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::CeilingLogBase2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;

// Sets `u[..=limbs]` to $2^{-b_1}s + 2^{-b_2}t$ and `v[..=limbs]` to $2^{-b_1}s - 2^{-b_2}t$ modulo
// $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH`, and $b_1$ and $b_2$ are less than
// $2 \cdot \text{limbs}\cdot\text{W}$, overwriting `s` and `t`.
//
// This is ifft_butterfly_twiddle from fft/ifft_mfa_truncate_sqrt2.c, FLINT 3.6.0.
crate_test_fn! {limbs_ifft_butterfly_twiddle(
    u: &mut [Limb],
    v: &mut [Limb],
    s: &mut [Limb],
    t: &mut [Limb],
    limbs: usize,
    mut b1: u64,
    mut b2: u64,
) {
    let nw = u64::exact_from(limbs) << Limb::LOG_WIDTH;
    let negate1 = b1 >= nw;
    if negate1 {
        b1 -= nw;
    }
    let x = usize::exact_from(b1 >> Limb::LOG_WIDTH);
    let b1 = b1 & Limb::WIDTH_MASK;
    let negate2 = b2 >= nw;
    if negate2 {
        b2 -= nw;
    }
    let y = usize::exact_from(b2 >> Limb::LOG_WIDTH);
    let b2 = b2 & Limb::WIDTH_MASK;
    if negate1 {
        limbs_neg_in_place(&mut s[..=limbs]);
    }
    limbs_div_2exp_mod_2expp1_in_place(s, limbs, b1);
    if negate2 {
        limbs_neg_in_place(&mut t[..=limbs]);
    }
    limbs_div_2exp_mod_2expp1_in_place(t, limbs, b2);
    limbs_butterfly_rsh_b(u, v, s, t, limbs, x, y);
}}

// The inverse of `fft_radix2_twiddle`, times `2 * n`.
//
// This is ifft_radix2_twiddle from fft/ifft_mfa_truncate_sqrt2.c, FLINT 3.6.0.
crate_test_fn! {ifft_radix2_twiddle(
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
        let (ii_lo, ii_hi) = ii.split_at_mut(is);
        limbs_ifft_butterfly_twiddle(
            t1,
            t2,
            &mut ii_lo[0],
            &mut ii_hi[0],
            limbs,
            u64::exact_from(tw1) * ws,
            u64::exact_from(tw2) * ws,
        );
        swap(&mut ii[0], t1);
        swap(&mut ii[is], t2);
        return;
    }
    ifft_radix2_twiddle(ii, is, n >> 1, w << 1, t1, t2, ws, r, c, rs << 1);
    ifft_radix2_twiddle(&mut ii[n * is..], is, n >> 1, w << 1, t1, t2, ws, r + rs, c, rs << 1);
    for i in 0..n {
        let (ii_lo, ii_hi) = ii.split_at_mut((n + i) * is);
        limbs_ifft_butterfly(t1, t2, &mut ii_lo[i * is], &mut ii_hi[0], i, limbs, w);
        swap(&mut ii_lo[i * is], t1);
        swap(&mut ii_hi[0], t2);
    }
}}

// The inverse of `fft_truncate1_twiddle`: from the first `trunc` outputs of a transform and the
// inputs past the first `trunc`, recovers the first `trunc` inputs, times `2 * n`.
//
// This is ifft_truncate1_twiddle from fft/ifft_mfa_truncate_sqrt2.c, FLINT 3.6.0.
crate_test_fn! {ifft_truncate1_twiddle(
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
        ifft_radix2_twiddle(ii, is, n, w, t1, t2, ws, r, c, rs);
    } else if trunc <= n {
        for i in trunc..n {
            let (ii_lo, ii_hi) = ii.split_at_mut((i + n) * is);
            limbs_slice_add_same_length_in_place_left(
                &mut ii_lo[i * is][..=limbs],
                &ii_hi[0][..=limbs],
            );
            limbs_div_2exp_mod_2expp1_in_place(&mut ii_lo[i * is], limbs, 1);
        }
        ifft_truncate1_twiddle(ii, is, n >> 1, w << 1, t1, t2, ws, r, c, rs << 1, trunc);
        for i in 0..trunc {
            let (ii_lo, ii_hi) = ii.split_at_mut((n + i) * is);
            let x = &mut ii_lo[i * is][..=limbs];
            limbs_slice_shl_in_place(x, 1);
            limbs_sub_same_length_in_place_left(x, &ii_hi[0][..=limbs]);
        }
    } else {
        ifft_radix2_twiddle(ii, is, n >> 1, w << 1, t1, t2, ws, r, c, rs << 1);
        for i in trunc - n..n {
            let (ii_lo, ii_hi) = ii.split_at_mut((i + n) * is);
            limbs_sub_same_length_in_place_right(&ii_lo[i * is][..=limbs], &mut ii_hi[0][..=limbs]);
            limbs_fft_adjust(t1, &ii_hi[0], i, limbs, w);
            limbs_slice_add_same_length_in_place_left(
                &mut ii_lo[i * is][..=limbs],
                &ii_hi[0][..=limbs],
            );
            swap(&mut ii_hi[0], t1);
        }
        ifft_truncate1_twiddle(
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
        for i in 0..trunc - n {
            let (ii_lo, ii_hi) = ii.split_at_mut((n + i) * is);
            limbs_ifft_butterfly(t1, t2, &mut ii_lo[i * is], &mut ii_hi[0], i, limbs, w);
            swap(&mut ii_lo[i * is], t1);
            swap(&mut ii_hi[0], t2);
        }
    }
}}

// The inverse of the matrix Fourier transform that `fft_mfa_truncate_sqrt2_outer` and the row
// transforms of `fft_mfa_truncate_sqrt2_inner` make up, whose inverse row transforms
// `fft_mfa_truncate_sqrt2_inner` has already applied. The first `trunc` residues are left
// normalized and divided by the transform length.
//
// This is ifft_mfa_truncate_sqrt2_outer from fft/ifft_mfa_truncate_sqrt2.c, FLINT 3.6.0, without
// the threading.
crate_test_fn! {ifft_mfa_truncate_sqrt2_outer(
    ii: &mut [Vec<Limb>],
    n: usize,
    w: u64,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
    temp: &mut [Limb],
    n1: usize,
    trunc: usize,
) {
    let two_n = n << 1;
    let n2 = two_n / n1;
    let trunc2 = (trunc - two_n) / n1;
    let depth = n2.ceiling_log_base_2();
    let depth2 = n1.ceiling_log_base_2();
    let limbs = fft_limbs(n, w);
    let wn1 = w * u64::exact_from(n1);
    // first half mfa IFFT : n2 rows, n1 cols
    //
    // column IFFTs
    for i in 0..n1 {
        for j in 0..n2 {
            let s = revbin(j, depth);
            if j < s {
                ii.swap(i + j * n1, i + s * n1);
            }
        }
        // IFFT of length n2 on column i, applying z^{r*i} for rows going up in steps of 1 starting
        // at row 0, where z => w bits
        ifft_radix2_twiddle(&mut ii[i..], n1, n2 >> 1, wn1, t1, t2, w, 0, i, 1);
    }
    // second half IFFT : n2 rows, n1 cols
    //
    // column IFFTs with relevant sqrt2 layer butterflies combined; the second half starts at
    // `ii[two_n]`
    for i in 0..n1 {
        for j in 0..trunc2 {
            let s = revbin(j, depth);
            if j < s {
                ii.swap(two_n + i + j * n1, two_n + i + s * n1);
            }
        }
        let (ii_lo, ii_hi) = ii.split_at_mut(two_n);
        for j in trunc2..n2 {
            let u = i + j * n1;
            limbs_fft_adjust_sqrt2_power(&mut ii_hi[u], &ii_lo[u], u, limbs, w, temp);
        }
        // IFFT of length n2 on column i, applying z^{r*i} for rows going up in steps of 1 starting
        // at row 0, where z => w bits
        ifft_truncate1_twiddle(&mut ii[two_n + i..], n1, n2 >> 1, wn1, t1, t2, w, 0, i, 1, trunc2);
        // relevant components of final sqrt2 layer of IFFT
        let mut j = i;
        let (ii_lo, ii_hi) = ii.split_at_mut(two_n);
        while j < trunc - two_n {
            limbs_ifft_butterfly_sqrt2_power(
                t1,
                t2,
                &mut ii_lo[j],
                &mut ii_hi[j],
                j,
                limbs,
                w,
                temp,
            );
            swap(&mut ii_lo[j], t1);
            swap(&mut ii_hi[j], t2);
            j += n1;
        }
        let mut j = trunc + i - two_n;
        while j < two_n {
            limbs_slice_shl_in_place(&mut ii[j][..=limbs], 1);
            j += n1;
        }
        for j in 0..trunc2 {
            let t = two_n + j * n1 + i;
            limbs_div_2exp_mod_2expp1_in_place(&mut ii[t], limbs, depth + depth2 + 1);
            limbs_norm_mod_2expp1(&mut ii[t], limbs);
        }
        for j in 0..n2 {
            let t = j * n1 + i;
            limbs_div_2exp_mod_2expp1_in_place(&mut ii[t], limbs, depth + depth2 + 1);
            limbs_norm_mod_2expp1(&mut ii[t], limbs);
        }
    }
}}
