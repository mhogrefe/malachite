// Copyright © 2026 Mikhail Hogrefe
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
    MUL_TOOM22_THRESHOLD, limbs_mul_limb_to_out, limbs_mul_to_out, limbs_mul_to_out_scratch_len,
};
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;

const RECURSIVE_PROD_THRESHOLD: usize = MUL_TOOM22_THRESHOLD;

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `factors.len()`.
//
// This is equivalent to `mpz_prodlimbs` from `mpz/prodlimbs.c`, GMP 6.3.0. If x is too small to
// contain the output, a newly-allocated vector containing the output is returned.
pub fn limbs_product(xs: &mut [Limb], factors: &mut [Limb]) -> (usize, Option<Vec<Limb>>) {
    let xs_len = xs.len();
    let factors_len = factors.len();
    assert_ne!(factors_len, 0);
    assert!(const { RECURSIVE_PROD_THRESHOLD > 3 });
    if factors_len < RECURSIVE_PROD_THRESHOLD {
        let j = factors_len - 1;
        let mut size = 1;
        for i in 1..j {
            let factor = factors[i];
            let carry = limbs_slice_mul_limb_in_place(&mut factors[..size], factor);
            factors[size] = carry;
            if carry != 0 {
                size += 1;
            }
        }
        let fj = factors[j];
        let factors = &factors[..size];
        let sp1 = size + 1;
        if sp1 > xs_len {
            let mut prod = vec![0; sp1];
            prod[..xs_len].copy_from_slice(xs);
            let carry = limbs_mul_limb_to_out::<DoubleLimb, Limb>(&mut prod, factors, fj);
            prod[size] = carry;
            (size + usize::from(carry != 0), Some(prod))
        } else {
            let carry = limbs_mul_limb_to_out::<DoubleLimb, Limb>(xs, factors, fj);
            xs[size] = carry;
            (size + usize::from(carry != 0), None)
        }
    } else {
        let mut i = factors_len >> 1;
        let mut j = factors_len - i;
        let mut x2 = vec![0; j];
        let (factors_lo, x1) = factors.split_at_mut(i);
        let ox2;
        (j, ox2) = limbs_product(&mut x2, x1);
        if let Some(new_x2) = ox2 {
            x2 = new_x2;
        }
        let x2 = &x2[..j];
        let new_x1;
        let ox1;
        (i, ox1) = limbs_product(x1, factors_lo);
        let x1 = if let Some(new_x1_temp) = ox1 {
            new_x1 = new_x1_temp;
            &new_x1[..i]
        } else {
            &x1[..i]
        };
        let size = i + j;
        let mut scratch = vec![0; limbs_mul_to_out_scratch_len(i, j)];
        if size > xs_len {
            let mut prod = vec![0; size];
            prod[..xs_len].copy_from_slice(xs);
            let carry = limbs_mul_to_out(&mut prod, x1, x2, &mut scratch);
            (size - usize::from(carry == 0), Some(prod))
        } else {
            let carry = limbs_mul_to_out(xs, x1, x2, &mut scratch);
            (size - usize::from(carry == 0), None)
        }
    }
}
