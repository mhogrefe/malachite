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

use crate::natural::HALF_WIDTH;
use crate::natural::arithmetic::mul::schonhage_strassen::mul_2expmod_2expp1::*;
use crate::natural::arithmetic::mul::schonhage_strassen::{
    limbs_add_signed_limb_mod_2expp1, limbs_neg_to_out,
};
use crate::natural::arithmetic::sub::{
    limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left,
    limbs_sub_same_length_in_place_right,
};
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;

// Multiplies the residue in `r[..=limbs]` by $2^{\text{wn}/2}$, a square root of 2 in the ring,
// using `temp[..=limbs]` as scratch and leaving the product there, where wn is `limbs *
// Limb::WIDTH`; then sets `r` to the product minus `r`, or, if `negate`, `r` minus the product.
// This is the step that the square-root-of-2 twiddle factors share.
pub(crate) fn limbs_mul_sqrt2_and_sub(
    r: &mut [Limb],
    temp: &mut [Limb],
    limbs: usize,
    negate: bool,
) {
    // multiply by 2^{wn/2}
    let y = limbs >> 1;
    temp[y..limbs].copy_from_slice(&r[..limbs - y]);
    temp[limbs] = 0;
    let cy = if y != 0 {
        limbs_neg_to_out(&mut temp[..y], &r[limbs - y..limbs])
    } else {
        0
    };
    limbs_add_signed_limb_mod_2expp1(&mut temp[y..], limbs - y, r[limbs].wrapping_neg());
    limbs_sub_limb_in_place(&mut temp[y..=limbs], cy);
    // shift by an additional half limb (rare)
    if limbs.odd() {
        limbs_mul_2exp_mod_2expp1_in_place(temp, limbs, HALF_WIDTH);
    }
    // subtract
    if negate {
        limbs_sub_same_length_in_place_left(&mut r[..=limbs], &temp[..=limbs]);
    } else {
        limbs_sub_same_length_in_place_right(&temp[..=limbs], &mut r[..=limbs]);
    }
}

// Sets `r[..=limbs]` to the residue `i1[..=limbs]` times $\sqrt{2}^{iw}$ modulo
// $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH` and `i` is odd, using
// `temp[..=limbs]` as scratch.
//
// This is fft_adjust_sqrt2 from fft/adjust_sqrt2.c, FLINT 3.6.0.
crate_test_fn! {limbs_fft_adjust_sqrt2(
    r: &mut [Limb],
    i1: &[Limb],
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
    // multiply by 2^{j + wn/4 + i*k}
    if y != 0 {
        temp[y..limbs].copy_from_slice(&i1[..limbs - y]);
        let cy = limbs_neg_to_out(&mut temp[..y], &i1[limbs - y..limbs]);
        temp[limbs] = 0;
        limbs_add_signed_limb_mod_2expp1(&mut temp[y..], limbs - y, i1[limbs].wrapping_neg());
        limbs_sub_limb_in_place(&mut temp[y..=limbs], cy);
        limbs_mul_2exp_mod_2expp1_to_out(r, temp, limbs, b1);
    } else {
        limbs_mul_2exp_mod_2expp1_to_out(r, i1, limbs, b1);
    }
    limbs_mul_sqrt2_and_sub(r, temp, limbs, negate);
}}
