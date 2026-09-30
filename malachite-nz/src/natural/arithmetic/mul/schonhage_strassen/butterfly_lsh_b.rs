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

use crate::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use crate::natural::arithmetic::mul::schonhage_strassen::{
    limbs_add_signed_limb_mod_2expp1, limbs_sum_diff,
};
use crate::natural::arithmetic::neg::limbs_neg_in_place;
use crate::natural::arithmetic::sub::limbs_sub_limb_in_place;
use crate::platform::Limb;

// Sets `t[..=limbs]` to $2^{\text{Wx}}(i_1 + i_2)$ and `u[..=limbs]` to $2^{\text{Wy}}(i_1 - i_2)$
// modulo $2^{\text{limbs}\cdot\text{W}} + 1$, where W is `Limb::WIDTH`: a sum and difference
// shifted left by whole limbs.
//
// This is butterfly_lshB from fft/butterfly_lshB.c, FLINT 3.6.0.
crate_test_fn! {limbs_butterfly_lsh_b(
    t: &mut [Limb],
    u: &mut [Limb],
    i1: &[Limb],
    i2: &[Limb],
    limbs: usize,
    x: usize,
    y: usize,
) {
    if x == 0 {
        if y == 0 {
            limbs_sum_diff(t, u, i1, i2, limbs + 1);
        } else {
            let cy = limbs_sum_diff(t, &mut u[y..], i1, i2, limbs - y);
            u[limbs] = (cy & 1).wrapping_neg();
            let cy1 = cy >> 1;
            let cy = limbs_sum_diff(&mut t[limbs - y..], u, &i2[limbs - y..], &i1[limbs - y..], y);
            t[limbs] = cy >> 1;
            limbs_slice_add_limb_in_place(&mut t[limbs - y..=limbs], cy1);
            let cy1 = (cy & 1).wrapping_neg().wrapping_add(i2[limbs].wrapping_sub(i1[limbs]));
            limbs_add_signed_limb_mod_2expp1(&mut u[y..], limbs - y, cy1);
            let cy1 = i1[limbs].wrapping_add(i2[limbs]).wrapping_neg();
            limbs_add_signed_limb_mod_2expp1(t, limbs, cy1);
        }
    } else if y == 0 {
        let cy = limbs_sum_diff(&mut t[x..], u, i1, i2, limbs - x);
        t[limbs] = cy >> 1;
        let cy1 = cy & 1;
        let cy = limbs_sum_diff(t, &mut u[limbs - x..], &i1[limbs - x..], &i2[limbs - x..], x);
        let cy2 = Limb::from(limbs_neg_in_place(&mut t[..x]));
        u[limbs] = (cy & 1).wrapping_neg();
        limbs_sub_limb_in_place(&mut u[limbs - x..=limbs], cy1);
        let cy1 = (cy >> 1)
            .wrapping_neg()
            .wrapping_sub(cy2)
            .wrapping_sub(i1[limbs].wrapping_add(i2[limbs]));
        limbs_add_signed_limb_mod_2expp1(&mut t[x..], limbs - x, cy1);
        let cy1 = i2[limbs].wrapping_sub(i1[limbs]);
        limbs_add_signed_limb_mod_2expp1(u, limbs, cy1);
    } else if x > y {
        let cy = limbs_sum_diff(&mut t[x..], &mut u[y..], i1, i2, limbs - x);
        t[limbs] = cy >> 1;
        let cy1 = cy & 1;
        let cy = limbs_sum_diff(
            t,
            &mut u[y + limbs - x..],
            &i1[limbs - x..],
            &i2[limbs - x..],
            x - y,
        );
        let cy2 = Limb::from(limbs_neg_in_place(&mut t[..x - y]));
        u[limbs] = (cy & 1).wrapping_neg();
        limbs_sub_limb_in_place(&mut u[y + limbs - x..=limbs], cy1);
        let cy1 = (cy >> 1).wrapping_add(cy2);
        let cy = limbs_sum_diff(&mut t[x - y..], u, &i2[limbs - y..], &i1[limbs - y..], y);
        let cy2 = Limb::from(limbs_neg_in_place(&mut t[x - y..x]));
        let borrow = Limb::from(limbs_sub_limb_in_place(&mut t[x - y..x], cy1));
        let cy1 = (cy >> 1)
            .wrapping_neg()
            .wrapping_sub(borrow)
            .wrapping_sub(cy2)
            .wrapping_sub(i1[limbs].wrapping_add(i2[limbs]));
        limbs_add_signed_limb_mod_2expp1(&mut t[x..], limbs - x, cy1);
        let cy1 = (cy & 1).wrapping_neg().wrapping_add(i2[limbs].wrapping_sub(i1[limbs]));
        limbs_add_signed_limb_mod_2expp1(&mut u[y..], limbs - y, cy1);
    } else if x < y {
        let cy = limbs_sum_diff(&mut t[x..], &mut u[y..], i1, i2, limbs - y);
        u[limbs] = (cy & 1).wrapping_neg();
        let cy1 = cy >> 1;
        let cy = limbs_sum_diff(
            &mut t[x + limbs - y..],
            u,
            &i2[limbs - y..],
            &i1[limbs - y..],
            y - x,
        );
        t[limbs] = cy >> 1;
        limbs_slice_add_limb_in_place(&mut t[x + limbs - y..=limbs], cy1);
        let cy1 = cy & 1;
        let cy = limbs_sum_diff(t, &mut u[y - x..], &i2[limbs - x..], &i1[limbs - x..], x);
        let borrow = Limb::from(limbs_sub_limb_in_place(&mut u[y - x..y], cy1));
        let cy1 = (cy & 1)
            .wrapping_neg()
            .wrapping_sub(borrow)
            .wrapping_add(i2[limbs].wrapping_sub(i1[limbs]));
        limbs_add_signed_limb_mod_2expp1(&mut u[y..], limbs - y, cy1);
        let cy2 = Limb::from(limbs_neg_in_place(&mut t[..x]));
        let cy1 = (cy >> 1)
            .wrapping_neg()
            .wrapping_sub(i1[limbs].wrapping_add(i2[limbs]))
            .wrapping_sub(cy2);
        limbs_add_signed_limb_mod_2expp1(&mut t[x..], limbs - x, cy1);
    } else {
        // x == y
        let cy = limbs_sum_diff(&mut t[x..], &mut u[x..], i1, i2, limbs - x);
        t[limbs] = cy >> 1;
        u[limbs] = (cy & 1).wrapping_neg();
        let cy = limbs_sum_diff(t, u, &i2[limbs - x..], &i1[limbs - x..], x);
        let cy2 = Limb::from(limbs_neg_in_place(&mut t[..x]));
        let cy1 = (cy >> 1)
            .wrapping_neg()
            .wrapping_sub(i1[limbs].wrapping_add(i2[limbs]))
            .wrapping_sub(cy2);
        limbs_add_signed_limb_mod_2expp1(&mut t[x..], limbs - x, cy1);
        let cy1 = (cy & 1)
            .wrapping_neg()
            .wrapping_add(i2[limbs])
            .wrapping_sub(i1[limbs]);
        limbs_add_signed_limb_mod_2expp1(&mut u[x..], limbs - x, cy1);
    }
}}
