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

pub fn one_over_sqrt_tau_prec_round_simple(prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let tau_lo = Float::tau_prec_round(working_prec, Floor).0;
        let mut tau_hi = tau_lo.clone();
        tau_hi.increment();
        let lo = tau_hi.reciprocal_sqrt_round(Floor).0;
        let hi = tau_lo.reciprocal_sqrt_round(Ceiling).0;
        let (one_over_sqrt_tau_lo, mut o_lo) = Float::from_float_prec_round(lo, prec, rm);
        let (one_over_sqrt_tau_hi, mut o_hi) = Float::from_float_prec_round(hi, prec, rm);
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        if o_lo == o_hi && one_over_sqrt_tau_lo == one_over_sqrt_tau_hi {
            return (one_over_sqrt_tau_lo, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}
