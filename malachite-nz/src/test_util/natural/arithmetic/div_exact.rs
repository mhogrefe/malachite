// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991-2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::div_exact::MAX_OVER_3;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{WrappingMulAssign, WrappingSubAssign};
use malachite_base::num::basic::integers::PrimitiveInt;

// This is equivalent to `MODLIMB_INVERSE_3` from `gmp-impl.h`, GMP 6.2.1.
const MODLIMB_INVERSE_3: Limb = (MAX_OVER_3 << 1) | 1;
const CEIL_MAX_OVER_3: Limb = MAX_OVER_3 + 1;
const CEIL_2_MAX_OVER_3: Limb = ((Limb::MAX >> 1) / 3 + 1) | (1 << (Limb::WIDTH - 1));

/// Benchmarks show that this algorithm is always worse than the default.
///
/// This is equivalent to `mpn_divexact_by3c` from `mpn/generic/diveby3.c`, GMP 6.2.1, with
/// `DIVEXACT_BY3_METHOD == 1`, no carry-in, and no return value.
pub fn limbs_div_exact_3_to_out_alt(out: &mut [Limb], ns: &[Limb]) {
    let len = ns.len();
    assert_ne!(len, 0);
    assert!(out.len() >= len);
    let (ns_last, ns_init) = ns.split_last().unwrap();
    let (out_last, out_init) = out[..len].split_last_mut().unwrap();
    let mut big_carry = 0;
    for (out_q, n) in out_init.iter_mut().zip(ns_init.iter()) {
        let (diff, carry) = n.overflowing_sub(big_carry);
        big_carry = Limb::from(carry);
        let q = diff.wrapping_mul(MODLIMB_INVERSE_3);
        *out_q = q;
        if q >= CEIL_MAX_OVER_3 {
            big_carry += 1;
            if q >= CEIL_2_MAX_OVER_3 {
                big_carry += 1;
            }
        }
    }
    *out_last = ns_last
        .wrapping_sub(big_carry)
        .wrapping_mul(MODLIMB_INVERSE_3);
}

/// Benchmarks show that this algorithm is always worse than the default.
///
/// This is equivalent to `mpn_divexact_by3c` from `mpn/generic/diveby3.c`, GMP 6.2.1, with
/// `DIVEXACT_BY3_METHOD == 1`, no carry-in, and no return value, where `rp == up`.
pub fn limbs_div_exact_3_in_place_alt(ns: &mut [Limb]) {
    let len = ns.len();
    assert_ne!(len, 0);
    let (ns_last, ns_init) = ns.split_last_mut().unwrap();
    let mut big_carry = 0;
    for n in &mut *ns_init {
        let (diff, carry) = n.overflowing_sub(big_carry);
        big_carry = Limb::from(carry);
        let q = diff.wrapping_mul(MODLIMB_INVERSE_3);
        *n = q;
        if q >= CEIL_MAX_OVER_3 {
            big_carry += 1;
            if q >= CEIL_2_MAX_OVER_3 {
                big_carry += 1;
            }
        }
    }
    ns_last.wrapping_sub_assign(big_carry);
    ns_last.wrapping_mul_assign(MODLIMB_INVERSE_3);
}
