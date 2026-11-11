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

use crate::mag::{MAG_BITS_I64, Mag, fix_mul};

impl Mag {
    // Replaces `self` with an upper bound for `self + x * y`.
    //
    // This is `mag_fast_addmul` from `mag.h`, FLINT 3.6.0.
    pub fn add_mul_assign(&mut self, x: Self, y: Self) {
        if self.man == 0 {
            *self = Self::mul(x, y);
        } else if x.man != 0 && y.man != 0 {
            // x * y < 2^e
            let e = x.exp + y.exp;
            let shift = self.exp - e;
            let product = fix_mul(x.man, y.man);
            if shift >= 0 {
                if shift >= MAG_BITS_I64 {
                    self.man += 1;
                } else {
                    self.man += (product >> shift) + 1;
                }
            } else {
                let shift = -shift;
                self.exp = e;
                self.man = if shift >= MAG_BITS_I64 {
                    product + 2
                } else {
                    product + (self.man >> shift) + 2
                };
                self.adjust_one_too_small();
            }
            self.adjust_one_too_large();
        }
    }
}
