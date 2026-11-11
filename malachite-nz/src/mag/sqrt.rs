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
use malachite_base::num::arithmetic::traits::{Parity, PowerOf2};
use malachite_base::num::conversion::traits::WrappingFrom;

impl Mag {
    // An upper bound for the square root of `self`.
    //
    // This is `mag_sqrt` from `mag/sqrt.c`, FLINT 3.6.0, for an exponent of less than 2^61 in
    // absolute value (FLINT's "lagom" exponents), the only ones an `i64` exponent can reach here.
    pub fn sqrt(self) -> Self {
        if self.man == 0 {
            return self;
        }
        // The mantissa has 30 bits, so this conversion and scaling are exact.
        let mut t = self.man as f64 / (u64::power_of_2(MAG_BITS) as f64);
        let mut e = self.exp;
        if e.odd() {
            e = (e - 1) >> 1;
            t *= 2.0;
        } else {
            e >>= 1;
        }
        t = libm::sqrt(t) * (1.0 + 1e-13);
        // This is `MAG_SET_D_2EXP` from `mag.h`, FLINT 3.6.0.
        let (x, x_exp) = libm::frexp(t);
        let mut man = (x * u64::power_of_2(MAG_BITS) as f64) as u64 + 1;
        let fix = man >> MAG_BITS;
        man = (man >> fix) + fix;
        Self {
            man,
            exp: e + i64::from(x_exp) + i64::wrapping_from(fix),
        }
    }
}
