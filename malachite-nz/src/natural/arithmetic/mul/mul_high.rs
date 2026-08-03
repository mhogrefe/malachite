// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the MPFR Library.
//
//      Copyright © 2005-2024 Free Software Foundation, Inc. Contributed by the AriC and Caramba
//      projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Mulders' short product: an approximation of the high half of a product, computed in roughly half
// the work of the full product at basecase sizes. Ported from MPFR's mulders.c as part of the
// `Float` multiplication port, and moved here, outside the `float_helpers` gate, because
// `mul_shr_round` uses it too. The complementary low-half kernel is in [`mul_low`](super::mul_low).

use crate::natural::arithmetic::add::{
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out_basecase, limbs_mul_same_length_to_out,
    limbs_mul_same_length_to_out_scratch_len,
};
use crate::platform::{Limb, MUL_FFT_THRESHOLD};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::XMulYToZZ;
use malachite_base::num::conversion::traits::WrappingFrom;

pub(crate) const MPFR_MULHIGH_TAB: [i8; 17] =
    [-1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0];

// This is mpfr_mulhigh_n_basecase from mulders.c, MPFR 4.2.0.
fn limbs_mul_high_same_length_basecase(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let len = xs.len();
    assert_eq!(ys.len(), len);
    // We neglect xs[0..len - 2] * ys[0], which is less than B ^ len
    let out = &mut out[len - 1..];
    (out[1], out[0]) = Limb::x_mul_y_to_zz(*xs.last().unwrap(), ys[0]);
    // The loop starts at ys[1]: the initial statement is ys[0]'s entire surviving row, and
    // repeating it would push the approximation above the true product, breaking the promise that
    // it never exceeds the truncated full product. (An earlier version of this port did repeat it,
    // adding the low limb of xs[len - 1] * ys[0] a second time. The `Float` callers absorbed that
    // overshoot inside their symmetric rounding budget, but one-sided consumers like
    // `mul_shr_round` cannot.)
    for (i, y) in ys.iter().enumerate().skip(1) {
        // Here, we neglect xs[0..len - i - 2] * ys[i], which is less than B ^ len too
        let (out_lo, out_hi) = out.split_at_mut(i + 1);
        out_hi[0] =
            limbs_slice_add_mul_limb_same_length_in_place_left(out_lo, &xs[len - i - 1..], *y);
        // In total, we neglect less than n * B ^ len, i.e., n ulps of out[len].
    }
}

pub(crate) fn limbs_mul_high_same_length_scratch_len(len: usize) -> usize {
    if len > MUL_FFT_THRESHOLD {
        limbs_mul_same_length_to_out_scratch_len(len)
    } else {
        let k = MPFR_MULHIGH_TAB.get(len).map_or_else(
            || 3 * (len >> 2),
            |&m| if m == -1 { 0 } else { usize::wrapping_from(m) },
        );
        if k == 0 {
            0
        } else {
            // The recursive case in `limbs_mul_high_same_length` reuses `scratch` for a full
            // multiply of length `k` and two recursive mul-highs of length `l = len - k`, so the
            // requirement is the max of those, not `scratch_len(len)`. Because Toom/FFT scratch
            // requirements are not monotonic in the operand length, `scratch_len(len)` can be
            // smaller than `scratch_len(k)` and under-size the buffer (mirrors the already- correct
            // `limbs_float_square_high_scratch_len`).
            let l = len - k;
            max(
                limbs_mul_same_length_to_out_scratch_len(k),
                limbs_mul_high_same_length_scratch_len(l),
            )
        }
    }
}

// Put in out[n..2 * len - 1] an approximation of the n high limbs of xs * ys. The error is less
// than len ulps of out[len] (and the approximation is always less or equal to the truncated full
// product).
//
// Implements Algorithm ShortMul from:
//
// [1] Short Division of Long Integers, David Harvey and Paul Zimmermann, Proceedings of the 20th
// Symposium on Computer Arithmetic (ARITH-20), July 25-27, 2011, pages 7-14.
//
// This is mpfr_mulhigh_n from mulders.c, MPFR 4.2.0.
pub(crate) fn limbs_mul_high_same_length(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let len = xs.len();
    assert_eq!(ys.len(), len);
    const LENGTH_VALID: bool = MPFR_MULHIGH_TAB.len() >= 8;
    assert!(LENGTH_VALID); // so that 3 * (len / 4) > len / 2
    let k = MPFR_MULHIGH_TAB.get(len).map_or_else(
        || Some(3 * (len >> 2)),
        |&m| {
            if m == -1 {
                None
            } else {
                Some(usize::wrapping_from(m))
            }
        },
    );
    assert!(k.is_none() || k == Some(0) || (k.unwrap() >= (len + 4) >> 1 && k.unwrap() < len));
    if let Some(k) = k {
        if k == 0 {
            // basecase error < len ulps
            limbs_mul_high_same_length_basecase(out, xs, ys);
        } else if len > MUL_FFT_THRESHOLD {
            // result is exact, no error
            limbs_mul_same_length_to_out(out, xs, ys, scratch);
        } else {
            let l = len - k;
            let out = &mut out[..len << 1];
            let (out_lo, out_hi) = out.split_at_mut(l << 1);
            let (ys_lo, ys_hi) = ys.split_at(l);
            limbs_mul_same_length_to_out(out_hi, &xs[l..], ys_hi, scratch);
            limbs_mul_high_same_length(out_lo, &xs[k..], ys_lo, scratch);
            let out_hi = &mut out_hi[k - l - 1..k];
            let mut carry = Limb::from(limbs_slice_add_same_length_in_place_left(
                out_hi,
                &out_lo[l - 1..],
            ));
            limbs_mul_high_same_length(out_lo, &xs[..l], &ys[k..], scratch);
            if limbs_slice_add_same_length_in_place_left(out_hi, &out_lo[l - 1..]) {
                carry += 1;
            }
            limbs_slice_add_limb_in_place(&mut out[len + l..], carry);
        }
    } else {
        // result is exact, no error
        limbs_mul_greater_to_out_basecase(out, xs, ys);
    }
}
