// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Contributed to the GNU project by Marco Bodrato.
//
//      Copyright © 2010-2012 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::mul::limb::limbs_slice_mul_limb_in_place;
use crate::natural::arithmetic::mul::{
    limbs_mul_limb_to_out, limbs_mul_to_out, limbs_mul_to_out_scratch_len, MUL_TOOM22_THRESHOLD,
};
use crate::platform::Limb;

const RECURSIVE_PROD_THRESHOLD: usize = MUL_TOOM22_THRESHOLD;

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `factors.len()`.
//
// This is equivalent to `mpz_prodlimbs` from `mpz/prodlimbs.c`, GMP 6.2.1, except that the output
// buffer is provided.
pub fn limbs_product(out: &mut [Limb], factors: &mut [Limb]) -> usize {
    let mut factors_len = factors.len();
    assert!(factors_len > 1);
    assert!(RECURSIVE_PROD_THRESHOLD > 3);
    if factors_len < RECURSIVE_PROD_THRESHOLD {
        factors_len -= 1;
        let mut size = 1;
        for i in 1..factors_len {
            let factor = factors[i];
            let carry = limbs_slice_mul_limb_in_place(&mut factors[..size], factor);
            factors[size] = carry;
            if carry != 0 {
                size += 1;
            }
        }
        assert!(out.len() > size);
        let carry = limbs_mul_limb_to_out(out, &factors[..size], factors[factors_len]);
        out[size] = carry;
        if carry != 0 {
            size += 1;
        }
        size
    } else {
        let half_len = factors_len >> 1;
        let (factors, xs) = factors.split_at_mut(half_len);
        let mut ys = vec![0; xs.len()];
        let ys_len = limbs_product(&mut ys, xs);
        let xs_len = limbs_product(xs, &mut factors[..half_len]);
        let size = xs_len + ys_len;
        assert!(out.len() >= size);
        let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(xs_len, ys_len)];
        if limbs_mul_to_out(out, &xs[..xs_len], &ys[..ys_len], &mut mul_scratch) == 0 {
            size - 1
        } else {
            size
        }
    }
}
