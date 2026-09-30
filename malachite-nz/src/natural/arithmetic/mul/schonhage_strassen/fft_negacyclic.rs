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
use crate::natural::arithmetic::mul::schonhage_strassen::fft_radix2::*;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::swap;

// Applies a negacyclic transform of length `2 * n` to the residues `ii`: twists input `i` by
// $\sqrt{2}^{iw}$, a primitive `4 * n`th root of unity, and then applies `fft_radix2`, so that
// pointwise products of transforms give products modulo $x^{2n} + 1$.
//
// This is fft_negacyclic from fft/fft_negacylic.c, FLINT 3.6.0.
crate_test_fn! {fft_negacyclic(
    ii: &mut [Vec<Limb>],
    n: usize,
    w: u64,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
    temp: &mut [Limb],
) {
    let limbs = fft_limbs(n, w);
    // first apply twiddle factors corresponding to shifts of w*i/2 bits
    for i in 0..n {
        limbs_fft_adjust_sqrt2_power(t1, &ii[i], i, limbs, w, temp);
        swap(&mut ii[i], t1);
        limbs_fft_adjust_sqrt2_power(t2, &ii[n + i], n + i, limbs, w, temp);
        swap(&mut ii[n + i], t2);
        limbs_fft_butterfly(t1, t2, &ii[i], &ii[n + i], i, limbs, w);
        swap(&mut ii[i], t1);
        swap(&mut ii[n + i], t2);
    }
    let (ii_lo, ii_hi) = ii.split_at_mut(n);
    fft_radix2(ii_lo, n >> 1, w << 1, t1, t2);
    fft_radix2(ii_hi, n >> 1, w << 1, t1, t2);
}}
