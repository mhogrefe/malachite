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

use crate::natural::arithmetic::shr::{limbs_shr_to_out, limbs_slice_shr_in_place};
use crate::platform::{Limb, SignedLimb};
use malachite_base::num::arithmetic::traits::XXSubYYToZZ;
use malachite_base::num::conversion::traits::WrappingFrom;

// Divides the residue `t[..=limbs]` in place by $2^d$ modulo $2^{\text{limbs}\cdot\text{W}} + 1$,
// where W is `Limb::WIDTH` and `d < W`.
//
// This is mpn_div_2expmod_2expp1 from fft/div_2expmod_2expp1.c, FLINT 3.6.0, where `t == i1`.
crate_test_fn! {limbs_div_2exp_mod_2expp1_in_place(t: &mut [Limb], limbs: usize, d: u64) {
    if d != 0 {
        let hi = SignedLimb::wrapping_from(t[limbs]);
        let lo = limbs_slice_shr_in_place(&mut t[..=limbs], d);
        t[limbs] = Limb::wrapping_from(hi >> d);
        (t[limbs], t[limbs - 1]) = Limb::xx_sub_yy_to_zz(t[limbs], t[limbs - 1], 0, lo);
    }
}}

// Sets `t[..=limbs]` to the residue `i1[..=limbs]` divided by $2^d$ modulo
// $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH` and `d < W`.
//
// This is mpn_div_2expmod_2expp1 from fft/div_2expmod_2expp1.c, FLINT 3.6.0, where `t != i1`.
crate_test_fn! {limbs_div_2exp_mod_2expp1_to_out(
    t: &mut [Limb],
    i1: &[Limb],
    limbs: usize,
    d: u64,
) {
    if d == 0 {
        t[..=limbs].copy_from_slice(&i1[..=limbs]);
    } else {
        let hi = SignedLimb::wrapping_from(i1[limbs]);
        let lo = limbs_shr_to_out(&mut t[..=limbs], &i1[..=limbs], d);
        t[limbs] = Limb::wrapping_from(hi >> d);
        (t[limbs], t[limbs - 1]) = Limb::xx_sub_yy_to_zz(t[limbs], t[limbs - 1], 0, lo);
    }
}}
