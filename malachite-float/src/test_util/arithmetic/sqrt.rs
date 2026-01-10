// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::arithmetic::sqrt::generic_sqrt_rational_ref;
use crate::malachite_base::num::basic::traits::NaN;
use crate::test_util::common::rug_float_significant_bits;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{CheckedSqrt, FloorLogBase2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

pub fn rug_sqrt_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut sqrt = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = sqrt.assign_round(x.sqrt_ref(), rm);
    (sqrt, o)
}

pub fn rug_sqrt_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_sqrt_prec_round(x, prec, Round::Nearest)
}

pub fn rug_sqrt_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_sqrt_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_sqrt(x: &rug::Float) -> rug::Float {
    rug_sqrt_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

pub fn sqrt_rational_prec_round_generic(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    if *x < 0u32 {
        return (Float::NAN, Equal);
    }
    if let Some(sqrt) = x.checked_sqrt() {
        return Float::from_rational_prec_round(sqrt, prec, rm);
    }
    generic_sqrt_rational_ref(x, prec, rm)
}

pub fn sqrt_rational_prec_round_simple(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    if *x < 0u32 {
        return (Float::NAN, Equal);
    }
    if let Some(sqrt) = x.checked_sqrt() {
        return Float::from_rational_prec_round(sqrt, prec, rm);
    }
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    let mut end_shift = x.floor_log_base_2();
    let x2;
    let reduced_x: &Rational;
    if end_shift.gt_abs(&0x3fff_0000) {
        end_shift &= !1;
        x2 = x >> end_shift;
        reduced_x = &x2;
    } else {
        end_shift = 0;
        reduced_x = x;
    }
    loop {
        let qx_lower_bound = Float::from_rational_prec_round_ref(reduced_x, working_prec, Floor).0;
        let mut qx_upper_bound = qx_lower_bound.clone();
        qx_upper_bound.increment();
        let lower_bound = qx_lower_bound.sqrt_round(Floor).0;
        let upper_bound = qx_upper_bound.sqrt_round(Ceiling).0;
        let (mut sqrt_1, mut o_1) = Float::from_float_prec_round(lower_bound, prec, rm);
        let (sqrt_2, mut o_2) = Float::from_float_prec_round(upper_bound, prec, rm);
        if o_1 == Equal {
            o_1 = o_2;
        }
        if o_2 == Equal {
            o_2 = o_1;
        }
        if o_1 == o_2 && sqrt_1 == sqrt_2 {
            if end_shift != 0 {
                o_1 = sqrt_1.shl_prec_round_assign_helper(end_shift >> 1, prec, rm, o_1);
            }
            return (sqrt_1, o_1);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}
