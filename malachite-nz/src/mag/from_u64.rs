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

use crate::mag::{MAG_BITS, Mag};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::LeadingZeros;

impl From<u64> for Mag {
    // An upper bound for `x`.
    //
    // This is `mag_set_ui` from `mag/set_ui.c`, FLINT 3.6.0.
    fn from(x: u64) -> Self {
        if x == 0 {
            return Self::ZERO;
        }
        let mut bits = u64::WIDTH - LeadingZeros::leading_zeros(x);
        let mut x = x;
        if bits <= MAG_BITS {
            x <<= MAG_BITS - bits;
        } else {
            x = (x >> (bits - MAG_BITS)) + 1;
            let overflow = x >> MAG_BITS;
            bits += overflow;
            x >>= overflow;
        }
        Self {
            man: x,
            exp: i64::wrapping_from(bits),
        }
    }
}

impl Mag {
    // A lower bound for `x`.
    //
    // This is `mag_set_ui_lower` from `mag/set_ui.c`, FLINT 3.6.0.
    pub fn from_u64_lower(x: u64) -> Self {
        if x == 0 {
            return Self::ZERO;
        }
        let bits = u64::WIDTH - LeadingZeros::leading_zeros(x);
        let x = if bits <= MAG_BITS {
            x << (MAG_BITS - bits)
        } else {
            x >> (bits - MAG_BITS)
        };
        Self {
            man: x,
            exp: i64::wrapping_from(bits),
        }
    }
}
