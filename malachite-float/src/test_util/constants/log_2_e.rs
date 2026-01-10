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

pub fn log_2_e_prec_round_simple(prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let ln_2_lower_bound = Float::ln_2_prec_round(working_prec, Floor).0;
        let mut ln_2_upper_bound = ln_2_lower_bound.clone();
        ln_2_upper_bound.increment();
        let lower_bound = ln_2_upper_bound.reciprocal_round(Floor).0;
        let upper_bound = ln_2_lower_bound.reciprocal_round(Ceiling).0;
        let (log_2_e_1, mut o_1) = Float::from_float_prec_round(lower_bound, prec, rm);
        let (log_2_e_2, mut o_2) = Float::from_float_prec_round(upper_bound, prec, rm);
        if o_1 == Equal {
            o_1 = o_2;
        }
        if o_2 == Equal {
            o_2 = o_1;
        }
        if o_1 == o_2 && log_2_e_1 == log_2_e_2 {
            return (log_2_e_1, o_1);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}
