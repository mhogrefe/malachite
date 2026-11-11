// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2014, 2015 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::mag::{MAG_BITS, MAG_BITS_I64, Mag};
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{IsPowerOf2, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{LeadingZeros, SignificantBits};

// The number of limbs in 64 bits.
const LIMBS_PER_WORD: usize = (u64::WIDTH >> Limb::LOG_WIDTH) as usize;

// Bits 64k through 64k + 63 of `x`: the `k`th 64-bit limb, whatever the width of a `Limb`.
fn word(x: &Natural, k: u64) -> u64 {
    let limbs = match x {
        Natural(Small(small)) => core::slice::from_ref(small),
        Natural(Large(limbs)) => limbs.as_slice(),
    };
    let mut w = 0;
    for (i, &limb) in limbs
        .iter()
        .skip(usize::exact_from(k) * LIMBS_PER_WORD)
        .take(LIMBS_PER_WORD)
        .enumerate()
    {
        w |= u64::wrapping_from(limb) << (i << Limb::LOG_WIDTH);
    }
    w
}

// An upper bound m * 2^e for a positive `x`, where m has exactly `bits` bits.
//
// This is `fmpz_abs_ubound_ui_2exp` from `fmpz/abs_ubound_ui_2exp.c`, FLINT 3.6.0, with 64-bit
// limbs.
fn abs_ubound_ui_2exp(x: &Natural, bits: u64) -> (u64, i64) {
    let size = x.significant_bits().div_ceil(u64::WIDTH);
    if size == 1 {
        let mut m = word(x, 0);
        let mut e = i64::wrapping_from(u64::WIDTH - LeadingZeros::leading_zeros(m))
            - i64::wrapping_from(bits);
        if e >= 0 {
            m = (m >> e) + 1;
            // The increment overflowed to the next power of 2.
            if m.is_power_of_2() {
                m = u64::power_of_2(bits - 1);
                e += 1;
            }
        } else {
            m <<= -e;
        }
        (m, e)
    } else {
        let mut e = i64::wrapping_from((size - 1) << u64::LOG_WIDTH);
        let mut m = word(x, size - 1);
        let shift = i64::wrapping_from(u64::WIDTH - LeadingZeros::leading_zeros(m))
            - i64::wrapping_from(bits);
        e += shift;
        m = if shift >= 0 {
            (m >> shift) + 1
        } else {
            // Read a second limb to get an accurate value.
            let m2 = word(x, size - 2);
            ((m << -shift) | (m2 >> (const { u64::WIDTH as i64 } + shift))) + 1
        };
        // The increment overflowed to the next power of 2.
        if m.is_power_of_2() {
            m = u64::power_of_2(bits - 1);
            e += 1;
        }
        (m, e)
    }
}

impl From<&Natural> for Mag {
    // An upper bound for `x`.
    //
    // Unlike `From<u64>`, this rounds up even when `x` has exactly `MAG_BITS` bits, or when the
    // bits below the top `MAG_BITS` are all zero: the result is exact only when `x` has fewer than
    // `MAG_BITS` bits, and is otherwise one unit in the last place above the truncation of `x`.
    //
    // This is `mag_set_fmpz` from `mag.h` and `mag_set_fmpz_2exp_fmpz` from
    // `mag/set_fmpz_2exp_fmpz.c`, FLINT 3.6.0.
    fn from(x: &Natural) -> Self {
        if *x == 0u32 {
            Self::ZERO
        } else {
            let (man, exp) = abs_ubound_ui_2exp(x, MAG_BITS);
            Self {
                man,
                exp: exp + MAG_BITS_I64,
            }
        }
    }
}
