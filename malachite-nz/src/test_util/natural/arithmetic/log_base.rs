// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBasePowerOf2, CheckedLogBase2, CheckedLogBasePowerOf2, FloorLogBasePowerOf2, Square,
};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use std::cmp::Ordering::*;

pub fn floor_log_base_naive(x: &Natural, base: &Natural) -> u64 {
    assert_ne!(*x, 0);
    assert!(*base > 1);
    let mut result = 0;
    let mut p = Natural::ONE;
    // loop always executes at least once
    while p <= *x {
        result += 1;
        p *= base;
    }
    result - 1
}

pub fn ceiling_log_base_naive(x: &Natural, base: &Natural) -> u64 {
    assert_ne!(*x, 0);
    assert!(*base > 1);
    let mut result = 0;
    let mut p = Natural::ONE;
    while p < *x {
        result += 1;
        p *= base;
    }
    result
}

pub fn checked_log_base_naive(x: &Natural, base: &Natural) -> Option<u64> {
    assert_ne!(*x, 0);
    assert!(*base > 1);
    let mut result = 0;
    let mut p = Natural::ONE;
    while p < *x {
        result += 1;
        p *= base;
    }
    if p == *x {
        Some(result)
    } else {
        None
    }
}

fn log_by_squaring_helper(x: &Natural, base: &Natural) -> (u64, bool) {
    assert_ne!(*x, 0);
    assert!(*base > 1);
    if *x == 1 {
        return (0, true);
    } else if x < base {
        return (0, false);
    }
    let x_bits = x.significant_bits();
    let mut powers = vec![base.clone()];
    for i in 0.. {
        let power = &powers[i];
        if ((power.significant_bits() - 1) << 1) | 1 > x_bits {
            break;
        }
        let next_power = power.square();
        powers.push(next_power);
    }
    // At this point, `powers[i]` is `base ^ (2 ^ i)`
    let mut log = 0;
    let mut test_power = Natural::ONE;
    for (i, power) in powers.into_iter().enumerate().rev() {
        let new_test_power = &test_power * power;
        match new_test_power.cmp(x) {
            Equal => {
                log.set_bit(u64::exact_from(i));
                return (log, true);
            }
            Less => {
                test_power = new_test_power;
                log.set_bit(u64::exact_from(i));
            }
            _ => {}
        }
    }
    (log, false)
}

pub fn floor_log_base_by_squaring(x: &Natural, base: &Natural) -> u64 {
    if let Some(log_base) = base.checked_log_base_2() {
        return x.floor_log_base_power_of_2(log_base);
    }
    log_by_squaring_helper(x, base).0
}

pub fn ceiling_log_base_by_squaring(x: &Natural, base: &Natural) -> u64 {
    if let Some(log_base) = base.checked_log_base_2() {
        return x.ceiling_log_base_power_of_2(log_base);
    }
    let (log, exact) = log_by_squaring_helper(x, base);
    if exact {
        log
    } else {
        log + 1
    }
}

pub fn checked_log_base_by_squaring(x: &Natural, base: &Natural) -> Option<u64> {
    if let Some(log_base) = base.checked_log_base_2() {
        return x.checked_log_base_power_of_2(log_base);
    }
    let (log, exact) = log_by_squaring_helper(x, base);
    if exact {
        Some(log)
    } else {
        None
    }
}
