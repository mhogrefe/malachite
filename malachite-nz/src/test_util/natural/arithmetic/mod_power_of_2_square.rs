// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::mod_power_of_2_square::limbs_square_diagonal_shl_add;
use crate::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use crate::platform::{DoubleLimb, Limb};
use malachite_base::num::arithmetic::traits::{Square, WrappingSquare};
use malachite_base::num::conversion::traits::SplitInHalf;

pub fn limbs_square_low_basecase_unrestricted(out: &mut [Limb], xs: &[Limb]) {
    let n = xs.len();
    let out = &mut out[..n];
    assert_ne!(n, 0);
    let xs_0 = xs[0];
    match n {
        1 => out[0] = xs_0.wrapping_square(),
        2 => {
            let p_hi;
            (p_hi, out[0]) = DoubleLimb::from(xs_0).square().split_in_half();
            out[1] = (xs_0.wrapping_mul(xs[1]) << 1).wrapping_add(p_hi);
        }
        _ => {
            let mut scratch = vec![0; n - 1];
            limbs_mul_limb_to_out(&mut scratch, &xs[1..], xs_0);
            for i in 1.. {
                let two_i = i << 1;
                if two_i >= n - 1 {
                    break;
                }
                limbs_slice_add_mul_limb_same_length_in_place_left(
                    &mut scratch[two_i..],
                    &xs[i + 1..n - i],
                    xs[i],
                );
            }
            limbs_square_diagonal_shl_add(out, &mut scratch, xs);
        }
    }
}
