// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2022 Daniel Schultz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use crate::natural::logic::not::limbs_not_to_out;
use crate::platform::Limb;

// Sets `z[..=limbs]` to the negation of the normalized residue `a[..=limbs]` modulo
// $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH`, normalized.
//
// This is mpn_negmod_2expp1 from fft/negmod_2expp1.c, FLINT 3.6.0.
crate_test_fn! {limbs_neg_mod_2expp1_to_out(z: &mut [Limb], a: &[Limb], limbs: usize) {
    if a[limbs] != 0 {
        assert_eq!(a[limbs], 1);
        z[0] = 1;
        z[1..=limbs].fill(0);
    } else {
        limbs_not_to_out(&mut z[..limbs], &a[..limbs]);
        z[limbs] = Limb::from(limbs_slice_add_limb_in_place(&mut z[..limbs], 2));
        if z[limbs] != 0 && z[0] != 0 {
            assert_eq!(z[0], 1);
            z[0] = 0;
            z[limbs] = 0;
        }
    }
}}
