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

use crate::natural::arithmetic::mul::schonhage_strassen::{
    limbs_add_signed_limb_mod_2expp1, limbs_sum_diff,
};
use crate::natural::arithmetic::neg::limbs_neg_in_place;
use crate::platform::Limb;

// Sets `t[..=limbs]` to $i_1 + i_2$ and `u[..=limbs]` to $i_1 - i_2$ modulo
// $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH`, after shifting `i1` right by `x`
// whole limbs and `i2` right by `y` whole limbs. The shifts may negate part of the inputs in place.
//
// This is butterfly_rshB from fft/butterfly_rshB.c, FLINT 3.6.0.
crate_test_fn! {limbs_butterfly_rsh_b(
    t: &mut [Limb],
    u: &mut [Limb],
    i1: &mut [Limb],
    i2: &mut [Limb],
    limbs: usize,
    x: usize,
    y: usize,
) {
    if x == 0 {
        if y == 0 {
            limbs_sum_diff(t, u, i1, i2, limbs + 1);
        } else {
            let cy = limbs_sum_diff(t, u, i1, &i2[y..], limbs - y);
            let cy1 = cy >> 1;
            let cy2 = (cy & 1).wrapping_neg();
            let cy =
                limbs_sum_diff(&mut u[limbs - y..], &mut t[limbs - y..], &i1[limbs - y..], i2, y);
            u[limbs] = (cy >> 1).wrapping_add(i1[limbs]);
            t[limbs] = i1[limbs].wrapping_sub(cy & 1);
            limbs_add_signed_limb_mod_2expp1(&mut t[limbs - y..], y, cy1.wrapping_add(i2[limbs]));
            limbs_add_signed_limb_mod_2expp1(&mut u[limbs - y..], y, cy2.wrapping_sub(i2[limbs]));
        }
    } else if y == 0 {
        let cy = limbs_sum_diff(t, u, &i1[x..], i2, limbs - x);
        let cy1 = cy >> 1;
        let cy2 = (cy & 1).wrapping_neg();
        let cy3 = Limb::from(limbs_neg_in_place(&mut i1[..x]));
        let cy = limbs_sum_diff(&mut t[limbs - x..], &mut u[limbs - x..], i1, &i2[limbs - x..], x);
        u[limbs] = cy3.wrapping_neg().wrapping_sub(cy & 1).wrapping_sub(i2[limbs]);
        t[limbs] = cy3.wrapping_neg().wrapping_add(i2[limbs]).wrapping_add(cy >> 1);
        limbs_add_signed_limb_mod_2expp1(&mut t[limbs - x..], x, cy1.wrapping_add(i1[limbs]));
        limbs_add_signed_limb_mod_2expp1(&mut u[limbs - x..], x, cy2.wrapping_add(i1[limbs]));
    } else if x == y {
        let cy = limbs_sum_diff(t, u, &i1[x..], &i2[x..], limbs - x);
        let cy1 = cy >> 1;
        let cy2 = (cy & 1).wrapping_neg();
        let cy = limbs_sum_diff(&mut t[limbs - x..], &mut u[limbs - x..], i2, i1, x);
        let cy3 = Limb::from(limbs_neg_in_place(&mut t[limbs - x..limbs]));
        u[limbs] = (cy & 1).wrapping_neg();
        t[limbs] = (cy >> 1).wrapping_neg().wrapping_sub(cy3);
        limbs_add_signed_limb_mod_2expp1(
            &mut t[limbs - x..],
            x,
            cy1.wrapping_add(i1[limbs]).wrapping_add(i2[limbs]),
        );
        limbs_add_signed_limb_mod_2expp1(
            &mut u[limbs - x..],
            x,
            cy2.wrapping_add(i1[limbs]).wrapping_sub(i2[limbs]),
        );
    } else if x > y {
        let cy = limbs_sum_diff(&mut t[limbs - y..], &mut u[limbs - y..], i2, &i1[x - y..], y);
        let cy3 = Limb::from(limbs_neg_in_place(&mut t[limbs - y..limbs]));
        t[limbs] = (cy >> 1).wrapping_neg().wrapping_sub(cy3);
        u[limbs] = (cy & 1).wrapping_neg();
        let cy3 = Limb::from(limbs_neg_in_place(&mut i1[..x - y]));
        let cy = limbs_sum_diff(
            &mut t[limbs - x..],
            &mut u[limbs - x..],
            i1,
            &i2[limbs - x + y..],
            x - y,
        );
        limbs_add_signed_limb_mod_2expp1(
            &mut t[limbs - y..],
            y,
            (cy >> 1).wrapping_add(i2[limbs]).wrapping_sub(cy3),
        );
        limbs_add_signed_limb_mod_2expp1(
            &mut u[limbs - y..],
            y,
            (cy & 1).wrapping_neg().wrapping_sub(i2[limbs]).wrapping_sub(cy3),
        );
        let cy = limbs_sum_diff(t, u, &i1[x..], &i2[y..], limbs - x);
        limbs_add_signed_limb_mod_2expp1(&mut t[limbs - x..], x, (cy >> 1).wrapping_add(i1[limbs]));
        limbs_add_signed_limb_mod_2expp1(
            &mut u[limbs - x..],
            x,
            (cy & 1).wrapping_neg().wrapping_add(i1[limbs]),
        );
    } else {
        // x < y
        let cy = limbs_sum_diff(&mut t[limbs - x..], &mut u[limbs - x..], &i2[y - x..], i1, x);
        let cy3 = Limb::from(limbs_neg_in_place(&mut t[limbs - x..limbs]));
        t[limbs] = (cy >> 1).wrapping_neg().wrapping_sub(cy3);
        u[limbs] = (cy & 1).wrapping_neg();
        let cy3 = Limb::from(limbs_neg_in_place(&mut i2[..y - x]));
        let cy = limbs_sum_diff(
            &mut t[limbs - y..],
            &mut u[limbs - y..],
            &i1[limbs - y + x..],
            i2,
            y - x,
        );
        limbs_add_signed_limb_mod_2expp1(
            &mut t[limbs - x..],
            x,
            (cy >> 1).wrapping_add(i1[limbs]).wrapping_sub(cy3),
        );
        limbs_add_signed_limb_mod_2expp1(
            &mut u[limbs - x..],
            x,
            (cy & 1).wrapping_neg().wrapping_add(i1[limbs]).wrapping_add(cy3),
        );
        let cy = limbs_sum_diff(t, u, &i1[x..], &i2[y..], limbs - y);
        limbs_add_signed_limb_mod_2expp1(&mut t[limbs - y..], y, (cy >> 1).wrapping_add(i2[limbs]));
        limbs_add_signed_limb_mod_2expp1(
            &mut u[limbs - y..],
            y,
            (cy & 1).wrapping_neg().wrapping_sub(i2[limbs]),
        );
    }
}}
