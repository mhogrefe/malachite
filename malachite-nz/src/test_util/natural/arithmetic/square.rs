// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use crate::natural::arithmetic::square::limbs_square_diagonal_add_shl_1;
use crate::platform::{DoubleLimb, Limb};
use malachite_base::num::arithmetic::traits::Square;
use malachite_base::num::conversion::traits::SplitInHalf;

pub fn limbs_square_to_out_basecase_unrestricted(out: &mut [Limb], xs: &[Limb]) {
    let n = xs.len();
    let (xs_head, xs_tail) = xs.split_first().unwrap();
    (out[1], out[0]) = DoubleLimb::from(*xs_head).square().split_in_half();
    if n > 1 {
        let two_n = n << 1;
        let mut scratch = vec![0; two_n - 2];
        let (scratch_last, scratch_init) = scratch[..n].split_last_mut().unwrap();
        *scratch_last = limbs_mul_limb_to_out::<DoubleLimb, Limb>(scratch_init, xs_tail, *xs_head);
        for i in 1..n - 1 {
            let (scratch_last, scratch_init) = scratch[i..][i..n].split_last_mut().unwrap();
            let (xs_head, xs_tail) = xs[i..].split_first().unwrap();
            *scratch_last =
                limbs_slice_add_mul_limb_same_length_in_place_left(scratch_init, xs_tail, *xs_head);
        }
        limbs_square_diagonal_add_shl_1(&mut out[..two_n], &mut scratch, xs);
    }
}

// The squaring-dispatch thresholds, for building scratch-canary size sweeps (see
// `test_util::scratch`). The threshold constants are `pub(crate)`, so integration tests get the
// list through this function.
pub fn square_dispatch_thresholds() -> Vec<usize> {
    vec![
        crate::platform::SQR_BASECASE_THRESHOLD,
        crate::platform::SQR_TOOM2_THRESHOLD,
        crate::platform::SQR_TOOM3_THRESHOLD,
        crate::platform::SQR_TOOM4_THRESHOLD,
        crate::platform::SQR_TOOM6_THRESHOLD,
        crate::platform::SQR_TOOM8_THRESHOLD,
        crate::natural::arithmetic::square::SQR_FFT_THRESHOLD,
    ]
}
