// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::floats::PrimitiveFloat;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::logic::traits::BitAccess;
use std::cmp::Ordering::*;

pub fn unadjusted_mean_to_adjusted_mean(unadjusted_mean: f64, limit: f64) -> f64 {
    let m = limit;
    assert!(unadjusted_mean > 0.0);
    assert!(m > 0.0);
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    let pow = q.powf(m + 1.0);
    let adjusted = ((m * p + 1.0) * pow - q) / (p * (pow - 1.0));
    adjusted.abs()
}

pub fn adjusted_mean_to_unadjusted_mean(adjusted_mean: f64, limit: f64) -> f64 {
    assert!(adjusted_mean > 0.0);
    assert!(adjusted_mean < limit / 2.0);
    let mut min = f64::MIN_POSITIVE_SUBNORMAL.to_ordered_representation();
    let mut max = f64::MAX_FINITE.to_ordered_representation();
    loop {
        if min == max {
            let f_max =
                unadjusted_mean_to_adjusted_mean(f64::from_ordered_representation(max), limit);
            assert!(!f_max.is_nan() && f_max <= limit && f_max >= adjusted_mean);
            return f64::from_ordered_representation(max);
        }
        let (sum, overflow) = min.overflowing_add(max);
        let mut mid = sum >> 1;
        if overflow {
            mid.set_bit(u64::WIDTH - 1);
        }
        let f_mid = unadjusted_mean_to_adjusted_mean(f64::from_ordered_representation(mid), limit);
        let compare = if f_mid.is_nan() {
            Greater
        } else {
            f_mid.partial_cmp(&adjusted_mean).unwrap()
        };
        match compare {
            Greater => max = mid,
            Less => min = mid + 1,
            Equal => return f64::from_ordered_representation(mid),
        }
    }
}
