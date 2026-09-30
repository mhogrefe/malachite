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

use crate::natural::arithmetic::mul::schonhage_strassen::mul_2expmod_2expp1::*;
use crate::natural::arithmetic::mul::schonhage_strassen::{
    limbs_add_signed_limb_mod_2expp1, limbs_neg_to_out,
};
use crate::natural::arithmetic::sub::limbs_sub_limb_in_place;
use crate::platform::Limb;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;

// Sets `r[..=limbs]` to the residue `i1[..=limbs]` times $2^{iw}$ modulo
// $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH`: the twiddle factor of a transform
// of length `2 * limbs * W / w`.
//
// This is fft_adjust from fft/adjust.c, FLINT 3.6.0.
crate_test_fn! {limbs_fft_adjust(r: &mut [Limb], i1: &[Limb], i: usize, limbs: usize, w: u64) {
    let b1 = u64::exact_from(i) * w;
    let x = usize::exact_from(b1 >> Limb::LOG_WIDTH);
    let b1 = b1 & Limb::WIDTH_MASK;
    if x != 0 {
        r[x..limbs].copy_from_slice(&i1[..limbs - x]);
        r[limbs] = 0;
        let cy = limbs_neg_to_out(&mut r[..x], &i1[limbs - x..limbs]);
        limbs_add_signed_limb_mod_2expp1(&mut r[x..], limbs - x, i1[limbs].wrapping_neg());
        limbs_sub_limb_in_place(&mut r[x..=limbs], cy);
        limbs_mul_2exp_mod_2expp1_in_place(r, limbs, b1);
    } else {
        limbs_mul_2exp_mod_2expp1_to_out(r, i1, limbs, b1);
    }
}}
