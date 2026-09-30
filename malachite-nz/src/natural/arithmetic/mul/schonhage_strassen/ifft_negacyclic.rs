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

use crate::natural::arithmetic::mul::schonhage_strassen::adjust_sqrt2::*;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_radix2::fft_limbs;
use crate::natural::arithmetic::mul::schonhage_strassen::ifft_radix2::*;
use crate::natural::arithmetic::neg::limbs_neg_in_place;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::swap;

// The inverse of `fft_negacyclic`, times `2 * n`.
//
// This is ifft_negacyclic from fft/ifft_negacyclic.c, FLINT 3.6.0.
crate_test_fn! {ifft_negacyclic(
    ii: &mut [Vec<Limb>],
    n: usize,
    w: u64,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
    temp: &mut [Limb],
) {
    let limbs = fft_limbs(n, w);
    {
        let (ii_lo, ii_hi) = ii.split_at_mut(n);
        ifft_radix2(ii_lo, n >> 1, w << 1, t1, t2);
        ifft_radix2(ii_hi, n >> 1, w << 1, t1, t2);
    }
    let (ii_lo, ii_hi) = ii.split_at_mut(n);
    for i in 0..n {
        limbs_ifft_butterfly(t1, t2, &mut ii_lo[i], &mut ii_hi[i], i, limbs, w);
        swap(&mut ii_lo[i], t1);
        swap(&mut ii_hi[i], t2);
        limbs_fft_adjust_sqrt2_power(t1, &ii_lo[i], (n << 1) - i, limbs, w, temp);
        limbs_neg_in_place(&mut t1[..=limbs]);
        swap(&mut ii_lo[i], t1);
        limbs_fft_adjust_sqrt2_power(t2, &ii_hi[i], n - i, limbs, w, temp);
        limbs_neg_in_place(&mut t2[..=limbs]);
        swap(&mut ii_hi[i], t2);
    }
}}
