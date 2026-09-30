// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008-2011, 2020 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::mul::schonhage_strassen::div_2expmod_2expp1::*;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_mfa_truncate_sqrt2::*;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_mfa_truncate_sqrt2_inner::*;
use crate::natural::arithmetic::mul::schonhage_strassen::fft_truncate_sqrt2::fft_truncate_sqrt2;
use crate::natural::arithmetic::mul::schonhage_strassen::ifft_mfa_truncate_sqrt2::*;
use crate::natural::arithmetic::mul::schonhage_strassen::ifft_truncate_sqrt2::ifft_truncate_sqrt2;
use crate::natural::arithmetic::mul::schonhage_strassen::mulmod_2expp1::fft_mulmod_2expp1;
use crate::natural::arithmetic::mul::schonhage_strassen::normmod_2expp1::limbs_norm_mod_2expp1;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;

// Replaces the first `trunc` residues of `ii` (each of `limbs + 1` limbs) with the first `trunc`
// coefficients of the cyclic convolution of length `4 << depth` of `ii` and `jj` (or of `ii` with
// itself, if `jj` is `None`), modulo $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH`.
// The residues must be zero past the first `trunc`, and `ii` and `jj` must have `4 << depth`
// residues. `jj` is overwritten.
//
// This is fft_convolution from fft/convolution.c, FLINT 3.6.0.
crate_test_fn! {fft_convolution(
    ii: &mut [Vec<Limb>],
    mut jj: Option<&mut [Vec<Limb>]>,
    depth: u64,
    limbs: usize,
    trunc: usize,
    t1: &mut Vec<Limb>,
    t2: &mut Vec<Limb>,
    s1: &mut [Limb],
    tt: &mut [Limb],
) {
    let n = usize::power_of_2(depth);
    let w = (u64::exact_from(limbs) << Limb::LOG_WIDTH) / u64::exact_from(n);
    let sqrt = usize::power_of_2(depth >> 1);
    if depth <= 6 {
        let trunc = trunc.div_ceil(2) << 1;
        fft_truncate_sqrt2(ii, n, w, t1, t2, s1, trunc);
        if let Some(jj) = jj.as_deref_mut() {
            fft_truncate_sqrt2(jj, n, w, t1, t2, s1, trunc);
        }
        for j in 0..trunc {
            limbs_norm_mod_2expp1(&mut ii[j], limbs);
            if let Some(jj) = jj.as_deref_mut() {
                limbs_norm_mod_2expp1(&mut jj[j], limbs);
                fft_mulmod_2expp1(&mut ii[j], Some(&jj[j]), n, w, tt);
            } else {
                fft_mulmod_2expp1(&mut ii[j], None, n, w, tt);
            }
        }
        ifft_truncate_sqrt2(ii, n, w, t1, t2, s1, trunc);
        for x in &mut ii[..trunc] {
            limbs_div_2exp_mod_2expp1_in_place(x, limbs, depth + 2);
            limbs_norm_mod_2expp1(x, limbs);
        }
    } else {
        let sqrt2 = sqrt << 1;
        let trunc = sqrt2 * trunc.div_ceil(sqrt2);
        fft_mfa_truncate_sqrt2_outer(ii, n, w, t1, t2, s1, sqrt, trunc);
        if let Some(jj) = jj.as_deref_mut() {
            fft_mfa_truncate_sqrt2_outer(jj, n, w, t1, t2, s1, sqrt, trunc);
        }
        fft_mfa_truncate_sqrt2_inner(ii, jj, n, w, t1, t2, sqrt, trunc, tt);
        ifft_mfa_truncate_sqrt2_outer(ii, n, w, t1, t2, s1, sqrt, trunc);
    }
}}
