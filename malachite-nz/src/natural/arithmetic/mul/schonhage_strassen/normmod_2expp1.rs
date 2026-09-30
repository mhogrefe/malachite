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
use crate::platform::Limb;

// Normalizes the residue `t[..=limbs]` modulo $2^{\text{limbs}\cdot\text{W}} + 1$, where W is
// `Limb::WIDTH`, so that the top limb is 0, or 1 when the rest is zero.
//
// This is mpn_normmod_2expp1 from fft/normmod_2expp1.c, FLINT 3.6.0.
crate_test_fn! {limbs_norm_mod_2expp1(t: &mut [Limb], limbs: usize) {
    let hi = t[limbs];
    if hi != 0 {
        t[limbs] = 0;
        limbs_add_signed_limb_mod_2expp1(t, limbs, hi.wrapping_neg());
        // hi will now be in [-1, 1]
        let hi = t[limbs];
        if hi != 0 {
            t[limbs] = 0;
            limbs_add_signed_limb_mod_2expp1(t, limbs, hi.wrapping_neg());
            // if we now have -1 (very unlikely)
            if t[limbs] == Limb::MAX {
                t[limbs] = 0;
                limbs_add_signed_limb_mod_2expp1(t, limbs, 1);
            }
        }
    }
}}
