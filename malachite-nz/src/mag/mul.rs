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

use crate::mag::{Mag, fix_mul};

impl Mag {
    // An upper bound for `x * y`.
    //
    // This is `mag_mul` from `mag/mul.c` and `mag_fast_mul` from `mag.h`, FLINT 3.6.0, which agree
    // on finite inputs.
    //
    // This is not an implementation of `Mul`: the product rounds up, and an operator would hide
    // that, leaving no natural place for a lower-bound sibling like FLINT's `mag_mul_lower`.
    #[allow(clippy::should_implement_trait)]
    pub fn mul(x: Self, y: Self) -> Self {
        if x.man == 0 || y.man == 0 {
            Self::ZERO
        } else {
            let mut z = Self {
                man: fix_mul(x.man, y.man) + 1,
                exp: x.exp + y.exp,
            };
            z.adjust_one_too_small();
            z
        }
    }

    // An upper bound for `self * y`.
    //
    // This is `mag_mul_ui` from `mag.h`, FLINT 3.6.0.
    #[inline]
    pub fn mul_u64(self, y: u64) -> Self {
        Self::mul(self, Self::from(y))
    }
}
