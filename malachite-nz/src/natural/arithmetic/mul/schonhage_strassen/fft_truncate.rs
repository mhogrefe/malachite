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
use crate::natural::arithmetic::mul::schonhage_strassen::fft_radix2::*;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::swap;

// Applies `fft_radix2` to `ii`, computing only the first `trunc` outputs, where `trunc` is at most
// `2 * n`, as if the inputs past the first `trunc` were zero; the inputs past `n` are used as they
// are.
//
// This is fft_truncate1 from fft/fft_truncate.c, FLINT 3.6.0.
crate_test_fn! {fft_truncate1(
    ii: &mut [Vec<Limb>],
    n: usize,
    w: u64,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
    trunc: usize,
) {
    let limbs = fft_limbs(n, w);
    if trunc == n << 1 {
        fft_radix2(ii, n, w, t1, t2);
    } else if trunc <= n {
        let (ii_lo, ii_hi) = ii.split_at_mut(n);
        for (x, y) in ii_lo.iter_mut().zip(ii_hi.iter()) {
            limbs_slice_add_same_length_in_place_left(&mut x[..=limbs], &y[..=limbs]);
        }
        fft_truncate1(ii, n >> 1, w << 1, t1, t2, trunc);
    } else {
        for i in 0..n {
            limbs_fft_butterfly(t1, t2, &ii[i], &ii[n + i], i, limbs, w);
            swap(&mut ii[i], t1);
            swap(&mut ii[n + i], t2);
        }
        let (ii_lo, ii_hi) = ii.split_at_mut(n);
        fft_radix2(ii_lo, n >> 1, w << 1, t1, t2);
        fft_truncate1(ii_hi, n >> 1, w << 1, t1, t2, trunc - n);
    }
}}

// Applies `fft_radix2` to `ii`, whose inputs past the first `trunc` are zero, computing only the
// first `trunc` outputs, where `trunc` is at most `2 * n`.
//
// This is fft_truncate from fft/fft_truncate.c, FLINT 3.6.0.
crate_test_fn! {fft_truncate(
    ii: &mut [Vec<Limb>],
    n: usize,
    w: u64,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
    trunc: usize,
) {
    let limbs = fft_limbs(n, w);
    if trunc == n << 1 {
        fft_radix2(ii, n, w, t1, t2);
    } else if trunc <= n {
        fft_truncate(ii, n >> 1, w << 1, t1, t2, trunc);
    } else {
        for i in 0..trunc - n {
            limbs_fft_butterfly(t1, t2, &ii[i], &ii[n + i], i, limbs, w);
            swap(&mut ii[i], t1);
            swap(&mut ii[n + i], t2);
        }
        for i in trunc - n..n {
            let (ii_lo, ii_hi) = ii.split_at_mut(n);
            limbs_fft_adjust(&mut ii_hi[i], &ii_lo[i], i, limbs, w);
        }
        let (ii_lo, ii_hi) = ii.split_at_mut(n);
        fft_radix2(ii_lo, n >> 1, w << 1, t1, t2);
        fft_truncate1(ii_hi, n >> 1, w << 1, t1, t2, trunc - n);
    }
}}
