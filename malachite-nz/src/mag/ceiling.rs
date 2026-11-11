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

use crate::mag::{MAG_BITS_I64, Mag};
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::ShrRound;
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode::*;

impl Mag {
    // The smallest integer that is at least `self`, which must be 0 or at least 1, so that its
    // exponent is positive.
    //
    // This is `mag_get_fmpz` from `mag/get_fmpz.c`, FLINT 3.6.0.
    pub fn ceiling(self) -> Natural {
        if self.man == 0 {
            return Natural::ZERO;
        }
        let shift = self.exp - MAG_BITS_I64;
        if shift >= 0 {
            Natural::from(self.man) << shift
        } else {
            Natural::from(self.man.shr_round(-shift, Ceiling).0)
        }
    }
}
