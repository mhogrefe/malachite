// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::platform::Limb;
use std::cmp::Ordering::{self, *};

pub fn pi_over_3_prec_round_simple(prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    const THREE: Float = Float::const_from_unsigned(3);
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let pi_lo = Float::pi_prec_round(working_prec, Floor).0;
        let mut pi_hi = pi_lo.clone();
        pi_hi.increment();
        let lo = pi_lo.div_round(THREE, Floor).0;
        let hi = pi_hi.div_round(THREE, Ceiling).0;
        let (pi_over_3_lo, mut o_lo) = Float::from_float_prec_round(lo, prec, rm);
        let (pi_over_3_hi, mut o_hi) = Float::from_float_prec_round(hi, prec, rm);
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        if o_lo == o_hi && pi_over_3_lo == pi_over_3_hi {
            return (pi_over_3_lo, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}
