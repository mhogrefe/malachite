// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009, 2011 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use crate::natural::arithmetic::mul::schonhage_strassen::adjust::limbs_fft_adjust;
use crate::natural::arithmetic::mul::schonhage_strassen::div_2expmod_2expp1::*;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_radix2::fft_limbs;
use crate::natural::arithmetic::mul::schonhage_strassen::ifft_radix2::*;
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural::arithmetic::sub::{
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
};
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::swap;

// The inverse of `fft_truncate1`: from the first `trunc` outputs of a transform and the inputs past
// the first `trunc`, recovers the first `trunc` inputs, times `2 * n`.
//
// This is ifft_truncate1 from fft/ifft_truncate.c, FLINT 3.6.0.
crate_test_fn! {ifft_truncate1(
    ii: &mut [Vec<Limb>],
    n: usize,
    w: u64,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
    trunc: usize,
) {
    let limbs = fft_limbs(n, w);
    if trunc == n << 1 {
        ifft_radix2(ii, n, w, t1, t2);
    } else if trunc <= n {
        for i in trunc..n {
            let (ii_lo, ii_hi) = ii.split_at_mut(n);
            limbs_slice_add_same_length_in_place_left(&mut ii_lo[i][..=limbs], &ii_hi[i][..=limbs]);
            limbs_div_2exp_mod_2expp1_in_place(&mut ii_lo[i], limbs, 1);
        }
        ifft_truncate1(ii, n >> 1, w << 1, t1, t2, trunc);
        for i in 0..trunc {
            let (ii_lo, ii_hi) = ii.split_at_mut(n);
            limbs_slice_shl_in_place(&mut ii_lo[i][..=limbs], 1);
            limbs_sub_same_length_in_place_left(&mut ii_lo[i][..=limbs], &ii_hi[i][..=limbs]);
        }
    } else {
        ifft_radix2(&mut ii[..n], n >> 1, w << 1, t1, t2);
        for i in trunc - n..n {
            let (ii_lo, ii_hi) = ii.split_at_mut(n);
            limbs_sub_same_length_in_place_right(&ii_lo[i][..=limbs], &mut ii_hi[i][..=limbs]);
            limbs_fft_adjust(t1, &ii_hi[i], i, limbs, w);
            limbs_slice_add_same_length_in_place_left(&mut ii_lo[i][..=limbs], &ii_hi[i][..=limbs]);
            swap(&mut ii_hi[i], t1);
        }
        ifft_truncate1(&mut ii[n..], n >> 1, w << 1, t1, t2, trunc - n);
        for i in 0..trunc - n {
            let (ii_lo, ii_hi) = ii.split_at_mut(n);
            limbs_ifft_butterfly(t1, t2, &mut ii_lo[i], &mut ii_hi[i], i, limbs, w);
            swap(&mut ii_lo[i], t1);
            swap(&mut ii_hi[i], t2);
        }
    }
}}

// The inverse of `fft_truncate`: from the first `trunc` outputs of a transform whose inputs past
// the first `trunc` were zero, recovers the first `trunc` inputs, times `2 * n`.
//
// This is ifft_truncate from fft/ifft_truncate.c, FLINT 3.6.0.
crate_test_fn! {ifft_truncate(
    ii: &mut [Vec<Limb>],
    n: usize,
    w: u64,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
    trunc: usize,
) {
    let limbs = fft_limbs(n, w);
    if trunc == n << 1 {
        ifft_radix2(ii, n, w, t1, t2);
    } else if trunc <= n {
        ifft_truncate(ii, n >> 1, w << 1, t1, t2, trunc);
        for x in &mut ii[..trunc] {
            limbs_slice_shl_in_place(&mut x[..=limbs], 1);
        }
    } else {
        ifft_radix2(&mut ii[..n], n >> 1, w << 1, t1, t2);
        for i in trunc - n..n {
            let (ii_lo, ii_hi) = ii.split_at_mut(n);
            limbs_fft_adjust(&mut ii_hi[i], &ii_lo[i], i, limbs, w);
        }
        ifft_truncate1(&mut ii[n..], n >> 1, w << 1, t1, t2, trunc - n);
        for i in 0..trunc - n {
            let (ii_lo, ii_hi) = ii.split_at_mut(n);
            limbs_ifft_butterfly(t1, t2, &mut ii_lo[i], &mut ii_hi[i], i, limbs, w);
            swap(&mut ii_lo[i], t1);
            swap(&mut ii_hi[i], t2);
        }
        for x in &mut ii[trunc - n..n] {
            limbs_slice_shl_in_place(&mut x[..=limbs], 1);
        }
    }
}}
