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

use crate::natural::arithmetic::mul::schonhage_strassen::butterfly_rsh_b::limbs_butterfly_rsh_b;
use crate::natural::arithmetic::mul::schonhage_strassen::div_2expmod_2expp1::*;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_radix2::fft_limbs;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::swap;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;

// Sets `s[..=limbs]` to $i_1 + 2^{-iw}i_2$ and `t[..=limbs]` to $i_1 - 2^{-iw}i_2$ modulo
// $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH`, overwriting `i2` with
// $2^{-iw}i_2$.
//
// This is ifft_butterfly from fft/ifft_radix2.c, FLINT 3.6.0.
crate_test_fn! {limbs_ifft_butterfly(
    s: &mut [Limb],
    t: &mut [Limb],
    i1: &mut [Limb],
    i2: &mut [Limb],
    i: usize,
    limbs: usize,
    w: u64,
) {
    let b1 = u64::exact_from(i) * w;
    let y = usize::exact_from(b1 >> Limb::LOG_WIDTH);
    let b1 = b1 & Limb::WIDTH_MASK;
    limbs_div_2exp_mod_2expp1_in_place(i2, limbs, b1);
    limbs_butterfly_rsh_b(s, t, i1, i2, limbs, 0, y);
}}

// Applies the inverse of `fft_radix2`, times `2 * n`, taking the input in bit-reversed order.
//
// This is ifft_radix2 from fft/ifft_radix2.c, FLINT 3.6.0.
crate_test_fn! {ifft_radix2(
    ii: &mut [Vec<Limb>],
    n: usize,
    w: u64,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
) {
    let limbs = fft_limbs(n, w);
    if n == 1 {
        let (ii_0, ii_1) = ii.split_at_mut(1);
        limbs_ifft_butterfly(t1, t2, &mut ii_0[0], &mut ii_1[0], 0, limbs, w);
        swap(&mut ii[0], t1);
        swap(&mut ii[1], t2);
        return;
    }
    {
        let (ii_lo, ii_hi) = ii.split_at_mut(n);
        ifft_radix2(ii_lo, n >> 1, w << 1, t1, t2);
        ifft_radix2(ii_hi, n >> 1, w << 1, t1, t2);
    }
    for i in 0..n {
        let (ii_lo, ii_hi) = ii.split_at_mut(n);
        limbs_ifft_butterfly(t1, t2, &mut ii_lo[i], &mut ii_hi[i], i, limbs, w);
        swap(&mut ii_lo[i], t1);
        swap(&mut ii_hi[i], t2);
    }
}}
