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

use crate::mag::Mag;
use core::cmp::Ordering::{self, *};

impl PartialOrd for Mag {
    // Compares `self` and `other`; see the `Ord` implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Mag {
    // Compares `self` and `other`. A nonzero `Mag` with the larger exponent is larger, and for
    // equal exponents, the mantissas decide.
    //
    // This is `mag_cmp` from `mag/cmp.c`, FLINT 3.6.0, for finite inputs.
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Equal
        } else if self.man == 0 {
            Less
        } else if other.man == 0 {
            Greater
        } else {
            (self.exp, self.man).cmp(&(other.exp, other.man))
        }
    }
}
