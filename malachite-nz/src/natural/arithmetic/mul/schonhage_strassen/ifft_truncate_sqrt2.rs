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

use crate::natural::arithmetic::mul::schonhage_strassen::adjust::limbs_fft_adjust;
use crate::natural::arithmetic::mul::schonhage_strassen::adjust_sqrt2::*;
use crate::natural::arithmetic::mul::schonhage_strassen::butterfly_rsh_b::limbs_butterfly_rsh_b;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_radix2::fft_limbs;
use crate::natural::arithmetic::mul::schonhage_strassen::ifft_radix2::*;
use crate::natural::arithmetic::mul::schonhage_strassen::ifft_truncate::*;
use crate::natural::arithmetic::mul::schonhage_strassen::mul_2expmod_2expp1::*;
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;

// Sets `s[..=limbs]` to $i_1 + \sqrt{2}^{-iw}i_2$ and `t[..=limbs]` to $i_1 - \sqrt{2}^{-iw}i_2$
// modulo $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH` and `i` is odd, using
// `temp[..=limbs]` as scratch and overwriting `i2`.
//
// This is ifft_butterfly_sqrt2 from fft/ifft_truncate_sqrt2.c, FLINT 3.6.0.
crate_test_fn! {limbs_ifft_butterfly_sqrt2(
    s: &mut [Limb],
    t: &mut [Limb],
    i1: &mut [Limb],
    i2: &mut [Limb],
    i: usize,
    limbs: usize,
    w: u64,
    temp: &mut [Limb],
) {
    let wn = u64::exact_from(limbs) << Limb::LOG_WIDTH;
    let j = u64::exact_from(i >> 1);
    let k = w >> 1;
    let mut b1 = wn + (wn >> 2) - j - u64::exact_from(i) * k - 1;
    let negate = b1 < wn;
    if !negate {
        b1 -= wn;
    }
    let y2 = usize::exact_from(b1 >> Limb::LOG_WIDTH);
    let b1 = b1 & Limb::WIDTH_MASK;
    // multiply by small part of 2^{2*wn - j - ik - 1 + wn/4}
    if b1 != 0 {
        limbs_mul_2exp_mod_2expp1_in_place(i2, limbs, b1);
    }
    // multiply by 2^{wn/2}, then subtract and negate...
    limbs_mul_sqrt2_and_sub(i2, temp, limbs, !negate);
    // ...negate and shift **left** by y2 limbs (i.e. shift right by (size - y2) limbs) and sumdiff
    limbs_butterfly_rsh_b(s, t, i1, i2, limbs, 0, limbs - y2);
}}

// The inverse of `fft_truncate_sqrt2`: from the first `trunc` outputs of a transform whose inputs
// past the first `trunc` were zero, recovers the first `trunc` inputs, times `4 * n`.
//
// This is ifft_truncate_sqrt2 from fft/ifft_truncate_sqrt2.c, FLINT 3.6.0.
crate_test_fn! {ifft_truncate_sqrt2(
    ii: &mut [Vec<Limb>],
    n: usize,
    w: u64,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
    temp: &mut [Limb],
    trunc: usize,
) {
    let limbs = fft_limbs(n, w);
    if w.even() {
        ifft_truncate(ii, n << 1, w >> 1, t1, t2, trunc);
        return;
    }
    let n2 = n << 1;
    ifft_radix2(&mut ii[..n2], n, w, t1, t2);
    for i in (trunc - n2..n2).step_by(2) {
        let (ii_lo, ii_hi) = ii.split_at_mut(n2);
        limbs_fft_adjust(&mut ii_hi[i], &ii_lo[i], i >> 1, limbs, w);
        let i = i + 1;
        limbs_fft_adjust_sqrt2(&mut ii_hi[i], &ii_lo[i], i, limbs, w, temp);
    }
    ifft_truncate1(&mut ii[n2..], n, w, t1, t2, trunc - n2);
    for i in (0..trunc - n2).step_by(2) {
        let (ii_lo, ii_hi) = ii.split_at_mut(n2);
        limbs_ifft_butterfly(t1, t2, &mut ii_lo[i], &mut ii_hi[i], i >> 1, limbs, w);
        swap(&mut ii_lo[i], t1);
        swap(&mut ii_hi[i], t2);
        let i = i + 1;
        limbs_ifft_butterfly_sqrt2(t1, t2, &mut ii_lo[i], &mut ii_hi[i], i, limbs, w, temp);
        swap(&mut ii_lo[i], t1);
        swap(&mut ii_hi[i], t2);
    }
    for x in &mut ii[trunc - n2..n2] {
        limbs_slice_shl_in_place(&mut x[..=limbs], 1);
    }
}}
