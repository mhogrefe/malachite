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

use crate::natural::arithmetic::mul::schonhage_strassen::limbs_add_signed_limb_mod_2expp1;
use crate::natural::arithmetic::shl::{limbs_shl_to_out, limbs_slice_shl_in_place};
use crate::natural::arithmetic::sub::limbs_sub_limb_in_place;
use crate::platform::{Limb, SignedLimb};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;

// Multiplies the residue `t[..=limbs]` in place by $2^d$ modulo $2^{\text{limbs}\cdot\text{W}} +
// 1$, where W is `Limb::WIDTH` and `d < W`.
//
// This is mpn_mul_2expmod_2expp1 from fft/mul_2expmod_2expp1.c, FLINT 3.6.0, where `t == i1`.
crate_test_fn! {limbs_mul_2exp_mod_2expp1_in_place(t: &mut [Limb], limbs: usize, d: u64) {
    if d != 0 {
        let hi1 = Limb::wrapping_from(SignedLimb::wrapping_from(t[limbs]) >> (Limb::WIDTH - d));
        limbs_slice_shl_in_place(&mut t[..=limbs], d);
        let hi2 = t[limbs];
        t[limbs] = 0;
        limbs_sub_limb_in_place(&mut t[..=limbs], hi2);
        limbs_add_signed_limb_mod_2expp1(&mut t[1..], limbs - 1, hi1.wrapping_neg());
    }
}}

// Sets `t[..=limbs]` to the residue `i1[..=limbs]` times $2^d$ modulo
// $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH` and `d < W`.
//
// This is mpn_mul_2expmod_2expp1 from fft/mul_2expmod_2expp1.c, FLINT 3.6.0, where `t != i1`.
crate_test_fn! {limbs_mul_2exp_mod_2expp1_to_out(
    t: &mut [Limb],
    i1: &[Limb],
    limbs: usize,
    d: u64,
) {
    if d == 0 {
        t[..=limbs].copy_from_slice(&i1[..=limbs]);
    } else {
        let hi1 = Limb::wrapping_from(SignedLimb::wrapping_from(i1[limbs]) >> (Limb::WIDTH - d));
        limbs_shl_to_out(&mut t[..=limbs], &i1[..=limbs], d);
        let hi2 = t[limbs];
        t[limbs] = 0;
        limbs_sub_limb_in_place(&mut t[..=limbs], hi2);
        limbs_add_signed_limb_mod_2expp1(&mut t[1..], limbs - 1, hi1.wrapping_neg());
    }
}}
