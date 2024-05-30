// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::limbs_slice_add_greater_in_place_left;
use crate::natural::arithmetic::mul::limbs_mul_greater_to_out_basecase;
use crate::natural::Natural;
use crate::platform::{Limb, MUL_TOOM22_THRESHOLD};
use malachite_base::num::basic::traits::{One, Zero};

// In GMP this is hardcoded to 500
pub const MUL_BASECASE_MAX_UN: usize = 500;

// We must have 1 < ys.len() < MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN < xs.len().
fn limbs_mul_greater_to_out_basecase_mem_opt_helper(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(ys_len > 1);
    assert!(ys_len < MUL_TOOM22_THRESHOLD);
    assert!(MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN);
    assert!(xs_len > MUL_BASECASE_MAX_UN);
    let mut triangle_buffer = [0; MUL_TOOM22_THRESHOLD];
    let mut offset = 0;
    for chunk in xs.chunks(MUL_BASECASE_MAX_UN) {
        let out = &mut out[offset..];
        if chunk.len() >= ys_len {
            limbs_mul_greater_to_out_basecase(out, chunk, ys);
        } else {
            limbs_mul_greater_to_out_basecase(out, ys, chunk);
        }
        if offset != 0 {
            limbs_slice_add_greater_in_place_left(out, &triangle_buffer[..ys_len]);
        }
        offset += MUL_BASECASE_MAX_UN;
        if offset < xs_len {
            triangle_buffer[..ys_len]
                .copy_from_slice(&out[MUL_BASECASE_MAX_UN..MUL_BASECASE_MAX_UN + ys_len]);
        }
    }
}

/// A version of `limbs_mul_greater_to_out_basecase` that attempts to be more efficient by
/// increasing cache locality. It is currently not measurably better than ordinary basecase.
pub fn limbs_mul_greater_to_out_basecase_mem_opt(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    if ys_len > 1 && ys_len < MUL_TOOM22_THRESHOLD && xs.len() > MUL_BASECASE_MAX_UN {
        limbs_mul_greater_to_out_basecase_mem_opt_helper(out, xs, ys);
    } else {
        limbs_mul_greater_to_out_basecase(out, xs, ys);
    }
}

pub fn limbs_product_naive(out: &mut [Limb], factors: &[Limb]) -> usize {
    let mut n = Natural::ONE;
    for &f in factors {
        n *= Natural::from(f);
    }
    let xs = n.into_limbs_asc();
    let size = xs.len();
    out[..size].copy_from_slice(&xs);
    size
}

pub fn natural_product_naive<I: Iterator<Item = Natural>>(xs: I) -> Natural {
    let mut p = Natural::ONE;
    for x in xs {
        if x == 0 {
            return Natural::ZERO;
        }
        p *= x;
    }
    p
}
