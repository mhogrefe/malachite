// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::mag::{MAG_BITS_I64, Mag};
use core::fmt::{Display, Formatter, Result};
use malachite_base::num::conversion::traits::WrappingFrom;

impl Display for Mag {
    // Writes the exact value of `self` as `m` or `m*2^k`, where `m` is odd (or `0`). For example,
    // `Mag::ONE` is written as `1`, and the `Mag` equal to 3/2 as `3*2^-1`.
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.man == 0 {
            return f.write_str("0");
        }
        let zeros = self.man.trailing_zeros();
        let pow = self.exp - MAG_BITS_I64 + i64::wrapping_from(zeros);
        let odd = self.man >> zeros;
        if pow == 0 {
            write!(f, "{odd}")
        } else {
            write!(f, "{odd}*2^{pow}")
        }
    }
}
