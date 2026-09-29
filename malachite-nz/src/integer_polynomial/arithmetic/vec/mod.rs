// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::InnerNatural::Small;
use crate::natural::{LIMB_MAX_QUARTER, Natural};
use crate::platform::{Limb, SignedLimb};
use malachite_base::num::conversion::traits::WrappingFrom;

pub mod dot_general;
pub mod max_bits;

// The largest absolute value of a small FLINT `fmpz`, which is stored in a single word with two
// bits to spare; larger values are stored as GMP integers. Some of FLINT's algorithms take a fast
// path for small values that depends on the spare bits, so a faithful translation takes it for the
// same values.
pub(crate) const COEFF_MAX: Limb = LIMB_MAX_QUARTER;

// The value of `x` as a signed word, if FLINT would store `x` as a small `fmpz`; that is, if
// `COEFF_IS_MPZ` from `flint.h`, FLINT 3.6.0, would be false.
pub(crate) fn small_value(x: &Integer) -> Option<SignedLimb> {
    match x.abs {
        Natural(Small(small)) if small <= COEFF_MAX => {
            let value = SignedLimb::wrapping_from(small);
            Some(if x.sign { value } else { -value })
        }
        _ => None,
    }
}
