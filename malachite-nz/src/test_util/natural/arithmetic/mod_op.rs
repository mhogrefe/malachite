// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991, 1993, 1994, 1996, 1998-2000, 2002, 2003 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::div_mod::limbs_invert_limb;
use crate::natural::arithmetic::mod_op::mod_by_preinversion;
use crate::platform::Limb;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::LeadingZeros;
use rug::ops::RemRounding;

pub fn rug_neg_mod(x: rug::Integer, y: rug::Integer) -> rug::Integer {
    -x.rem_ceil(y)
}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// This is equivalent to `mpn_divrem_1` from `mpn/generic/divrem_1.c`, GMP 6.2.1, where `qxn == 0`
// and `un > 1`, but not computing the quotient.
pub fn limbs_mod_limb_alt_3(ns: &[Limb], d: Limb) -> Limb {
    assert_ne!(d, 0);
    let len = ns.len();
    assert!(len > 1);
    let bits = LeadingZeros::leading_zeros(d);
    let (ns_last, ns_init) = ns.split_last().unwrap();
    if bits == 0 {
        // High quotient limb is 0 or 1, skip a divide step.
        let mut r = *ns_last;
        if r >= d {
            r -= d;
        }
        // Multiply-by-inverse, divisor already normalized.
        let d_inv = limbs_invert_limb(d);
        for n in ns_init.iter().rev() {
            r = mod_by_preinversion(r, *n, d, d_inv);
        }
        r
    } else {
        // Skip a division if high < divisor (high quotient 0). Testing here before normalizing will
        // still skip as often as possible.
        let (ns, mut r) = if *ns_last < d {
            (ns_init, *ns_last)
        } else {
            (ns, 0)
        };
        let d = d << bits;
        r <<= bits;
        let d_inv = limbs_invert_limb(d);
        let (ns_last, ns_init) = ns.split_last().unwrap();
        let mut previous_n = *ns_last;
        let cobits = Limb::WIDTH - bits;
        r |= previous_n >> cobits;
        for &n in ns_init.iter().rev() {
            let shifted_n = (previous_n << bits) | (n >> cobits);
            r = mod_by_preinversion(r, shifted_n, d, d_inv);
            previous_n = n;
        }
        mod_by_preinversion(r, previous_n << bits, d, d_inv) >> bits
    }
}
