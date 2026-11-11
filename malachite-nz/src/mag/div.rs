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
use malachite_base::num::conversion::traits::WrappingFrom;

impl Mag {
    // An upper bound for `x / y`, where `y` is positive.
    //
    // This is `mag_div` from `mag/div.c`, FLINT 3.6.0, for a positive divisor.
    fn div(x: Self, y: Self) -> Self {
        assert_ne!(y.man, 0);
        if x.man == 0 {
            return Self::ZERO;
        }
        let mut q = (x.man << MAG_BITS) / y.man + 1;
        let mut fix = q >> MAG_BITS;
        q = (q >> fix) + fix;
        // This could overflow to the next power of 2.
        if q >> MAG_BITS != 0 {
            q >>= 1;
            fix += 1;
        }
        Self {
            man: q,
            exp: x.exp - y.exp + i64::wrapping_from(fix),
        }
    }

    // An upper bound for `self / y`, where `y` is positive.
    //
    // This is `mag_div_ui` from `mag.h`, FLINT 3.6.0.
    #[inline]
    pub fn div_u64(self, y: u64) -> Self {
        Self::div(self, Self::from_u64_lower(y))
    }
}
