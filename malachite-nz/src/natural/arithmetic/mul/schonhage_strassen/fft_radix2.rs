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

use crate::natural::arithmetic::mul::schonhage_strassen::butterfly_lsh_b::limbs_butterfly_lsh_b;
use crate::natural::arithmetic::mul::schonhage_strassen::mul_2expmod_2expp1::*;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::swap;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;

// The number of limbs, besides the overflow limb, in each residue of a transform of length `2 * n`
// with twiddle-factor exponent `w`.
pub(crate) fn fft_limbs(n: usize, w: u64) -> usize {
    usize::exact_from((w * u64::exact_from(n)) >> Limb::LOG_WIDTH)
}

// Sets `s[..=limbs]` to $i_1 + i_2$ and `t[..=limbs]` to $2^{iw}(i_1 - i_2)$ modulo
// $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH`.
//
// This is fft_butterfly from fft/fft_radix2.c, FLINT 3.6.0.
crate_test_fn! {limbs_fft_butterfly(
    s: &mut [Limb],
    t: &mut [Limb],
    i1: &[Limb],
    i2: &[Limb],
    i: usize,
    limbs: usize,
    w: u64,
) {
    let b1 = u64::exact_from(i) * w;
    let y = usize::exact_from(b1 >> Limb::LOG_WIDTH);
    let b1 = b1 & Limb::WIDTH_MASK;
    limbs_butterfly_lsh_b(s, t, i1, i2, limbs, 0, y);
    limbs_mul_2exp_mod_2expp1_in_place(t, limbs, b1);
}}

// Applies a radix-2 decimation-in-frequency transform of length `2 * n` to the residues `ii`, with
// $2^w$ as a primitive `2 * n`th root of unity, leaving the output in bit-reversed order. `t1` and
// `t2` are scratch residues.
//
// This is fft_radix2 from fft/fft_radix2.c, FLINT 3.6.0.
crate_test_fn! {fft_radix2(
    ii: &mut [Vec<Limb>],
    n: usize,
    w: u64,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
) {
    let limbs = fft_limbs(n, w);
    if n == 1 {
        limbs_fft_butterfly(t1, t2, &ii[0], &ii[1], 0, limbs, w);
        swap(&mut ii[0], t1);
        swap(&mut ii[1], t2);
        return;
    }
    for i in 0..n {
        limbs_fft_butterfly(t1, t2, &ii[i], &ii[n + i], i, limbs, w);
        swap(&mut ii[i], t1);
        swap(&mut ii[n + i], t2);
    }
    let (ii_lo, ii_hi) = ii.split_at_mut(n);
    fft_radix2(ii_lo, n >> 1, w << 1, t1, t2);
    fft_radix2(ii_hi, n >> 1, w << 1, t1, t2);
}}
