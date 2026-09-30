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
use crate::natural::arithmetic::mul::schonhage_strassen::butterfly_lsh_b::limbs_butterfly_lsh_b;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_radix2::*;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_truncate::*;
use crate::natural::arithmetic::mul::schonhage_strassen::mul_2expmod_2expp1::*;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;

// Sets `s[..=limbs]` to $i_1 + i_2$ and `t[..=limbs]` to $\sqrt{2}^{iw}(i_1 - i_2)$ modulo
// $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH` and `i` is odd, using
// `temp[..=limbs]` as scratch.
//
// This is fft_butterfly_sqrt2 from fft/fft_truncate_sqrt2.c, FLINT 3.6.0.
crate_test_fn! {limbs_fft_butterfly_sqrt2(
    s: &mut [Limb],
    t: &mut [Limb],
    i1: &[Limb],
    i2: &[Limb],
    i: usize,
    limbs: usize,
    w: u64,
    temp: &mut [Limb],
) {
    let wn = u64::exact_from(limbs) << Limb::LOG_WIDTH;
    let j = u64::exact_from(i >> 1);
    let k = w >> 1;
    let mut b1 = j + (wn >> 2) + u64::exact_from(i) * k;
    let negate = b1 >= wn;
    if negate {
        b1 -= wn;
    }
    let y = usize::exact_from(b1 >> Limb::LOG_WIDTH);
    let b1 = b1 & Limb::WIDTH_MASK;
    // sumdiff and multiply by 2^{j + wn/4 + i*k}
    limbs_butterfly_lsh_b(s, t, i1, i2, limbs, 0, y);
    limbs_mul_2exp_mod_2expp1_in_place(t, limbs, b1);
    limbs_mul_sqrt2_and_sub(t, temp, limbs, negate);
}}

// Applies a transform of length `4 * n` to the residues `ii`, whose inputs past the first `trunc`
// are zero, computing only the first `trunc` outputs, where `2 * n < trunc <= 4 * n` and `trunc` is
// even. The primitive `4 * n`th root of unity is $\sqrt{2}^w$, so `w` may be odd.
//
// This is fft_truncate_sqrt2 from fft/fft_truncate_sqrt2.c, FLINT 3.6.0.
crate_test_fn! {fft_truncate_sqrt2(
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
        fft_truncate(ii, n << 1, w >> 1, t1, t2, trunc);
        return;
    }
    let n2 = n << 1;
    for i in (0..trunc - n2).step_by(2) {
        limbs_fft_butterfly(t1, t2, &ii[i], &ii[n2 + i], i >> 1, limbs, w);
        swap(&mut ii[i], t1);
        swap(&mut ii[n2 + i], t2);
        let i = i + 1;
        limbs_fft_butterfly_sqrt2(t1, t2, &ii[i], &ii[n2 + i], i, limbs, w, temp);
        swap(&mut ii[i], t1);
        swap(&mut ii[n2 + i], t2);
    }
    for i in (trunc - n2..n2).step_by(2) {
        let (ii_lo, ii_hi) = ii.split_at_mut(n2);
        limbs_fft_adjust(&mut ii_hi[i], &ii_lo[i], i >> 1, limbs, w);
        let i = i + 1;
        limbs_fft_adjust_sqrt2(&mut ii_hi[i], &ii_lo[i], i, limbs, w, temp);
    }
    let (ii_lo, ii_hi) = ii.split_at_mut(n2);
    fft_radix2(ii_lo, n, w, t1, t2);
    fft_truncate1(ii_hi, n, w, t1, t2, trunc - n2);
}}
