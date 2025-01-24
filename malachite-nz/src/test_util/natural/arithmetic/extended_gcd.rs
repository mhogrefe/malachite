// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{DivExact, DivMod, DivRound, NegAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::rounding_modes::RoundingMode::*;
use std::mem::swap;

pub fn extended_gcd_euclidean_natural(
    mut a: Natural,
    mut b: Natural,
) -> (Natural, Integer, Integer) {
    let mut stack = Vec::new();
    let gcd;
    let mut x;
    let mut y;
    loop {
        if a == 0u32 && b == 0u32 {
            (gcd, x, y) = (a, Integer::ZERO, Integer::ZERO);
            break;
        } else if a == b || a == 0u32 {
            (gcd, x, y) = (b, Integer::ZERO, Integer::ONE);
            break;
        }
        let (q, r) = (&b).div_mod(&a);
        stack.push(q);
        swap(&mut a, &mut b);
        a = r;
    }
    for q in stack.into_iter().rev() {
        swap(&mut x, &mut y);
        x -= Integer::from(q) * &y;
    }
    (gcd, x, y)
}

// This is equivalent to `n_xgcd` from `ulong_extras/xgcd.c`, FLINT 2.7.1, extended to `Natural`s
// and with an adjustment to find the minimal cofactors.
pub fn extended_gcd_binary_natural(mut a: Natural, mut b: Natural) -> (Natural, Integer, Integer) {
    if a == 0u32 && b == 0u32 {
        return (Natural::ZERO, Integer::ZERO, Integer::ZERO);
    } else if a == b || a == 0u32 {
        return (b, Integer::ZERO, Integer::ONE);
    } else if b == 0u32 {
        return (a, Integer::ONE, Integer::ZERO);
    }
    let mut swapped = false;
    if a < b {
        swap(&mut a, &mut b);
        swapped = true;
    }
    let mut u1 = Integer::ONE;
    let mut v2 = Integer::ONE;
    let mut u2 = Integer::ZERO;
    let mut v1 = Integer::ZERO;
    let mut u3 = a.clone();
    let mut v3 = b.clone();
    let mut d;
    let mut t2;
    let mut t1;
    while v3 != 0u32 {
        d = &u3 - &v3;
        if u3 < (&v3 << 2) {
            if d < v3 {
                // quot = 1
                t2 = v2.clone();
                t1 = u2.clone();
                u2 -= u1;
                u2.neg_assign();
                u1 = t1;
                u3 = v3;
                v2 -= v1;
                v2.neg_assign();
                v1 = t2;
                v3 = d;
            } else if d < (&v3 << 1) {
                // quot = 2
                t1 = u2.clone();
                u2 = u1 - (&u2 << 1);
                u1 = t1;
                u3 = v3;
                t2 = v2.clone();
                v2 = v1 - (v2 << 1);
                v1 = t2;
                v3 = d - &u3;
            } else {
                // quot = 3
                t1 = u2.clone();
                u2 = u1 - Integer::from(3u32) * &u2;
                u1 = t1;
                u3 = v3;
                t2 = v2.clone();
                v2 = v1 - Integer::from(3u32) * &v2;
                v1 = t2;
                v3 = d - (&u3 << 1);
            }
        } else {
            let (quot, rem) = u3.div_mod(&v3);
            let quot = Integer::from(quot);
            t1 = u2.clone();
            u2 = u1 - &quot * &u2;
            u1 = t1;
            u3 = v3.clone();
            t2 = v2.clone();
            v2 = v1 - quot * &v2;
            v1 = t2;
            v3 = rem;
        }
    }
    // The cofactors at this point are not necessarily minimal, so we may need to adjust.
    let gcd = u3;
    let mut x = u1;
    let mut y = v1;
    let two_limit_a = Integer::from(a.div_exact(&gcd));
    let two_limit_b = Integer::from(b.div_exact(&gcd));
    let limit_b = &two_limit_b >> 1u32;
    if x > limit_b {
        let k = (&x - limit_b).div_round(&two_limit_b, Ceiling).0;
        x -= two_limit_b * &k;
        y += two_limit_a * k;
    }
    if swapped {
        swap(&mut x, &mut y);
    }
    (gcd, x, y)
}
