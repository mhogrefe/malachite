// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};

pub fn float_partial_cmp_rational_alt(x: &Float, other: &Rational) -> Option<Ordering> {
    match (x, other) {
        (float_nan!(), _) => None,
        (float_infinity!(), _) => Some(Greater),
        (float_negative_infinity!(), _) => Some(Less),
        (float_either_zero!(), y) => 0u32.partial_cmp(y),
        (
            Float(Finite {
                sign: s_x,
                exponent: e_x,
                ..
            }),
            y,
        ) => Some(if *y == 0u32 {
            if *s_x {
                Greater
            } else {
                Less
            }
        } else {
            let s_cmp = s_x.cmp(&(*y > 0));
            if s_cmp != Equal {
                return Some(s_cmp);
            }
            let ord_cmp = (i64::from(*e_x) - 1).cmp(&other.floor_log_base_2_abs());
            if ord_cmp == Equal {
                Rational::try_from(x).unwrap().cmp(other)
            } else if *s_x {
                ord_cmp
            } else {
                ord_cmp.reverse()
            }
        }),
    }
}
