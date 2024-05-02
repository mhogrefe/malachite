// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::test_util::stats::moments::CheckedToF64;

pub const fn uniform_bool_median(min: bool, max: bool) -> (bool, Option<bool>) {
    if min == max {
        (min, None)
    } else {
        (false, Some(true))
    }
}

pub fn uniform_primitive_int_median<T: PrimitiveInt>(min: T, max: T) -> (T, Option<T>) {
    let mut mean = min.wrapping_add(max);
    mean >>= 1;
    if mean < min || mean > max {
        mean.flip_bit(T::WIDTH - 1);
    }
    if min.even() == max.even() {
        (mean, None)
    } else {
        (mean, Some(mean + T::ONE))
    }
}

pub fn deleted_uniform_primitive_int_median<T: PrimitiveInt>(
    min: T,
    max: T,
    deleted: T,
) -> (T, Option<T>) {
    let (mut lo, mut hi) = uniform_primitive_int_median(min, max - T::ONE);
    if lo >= deleted {
        lo += T::ONE;
    }
    if let Some(hi) = hi.as_mut() {
        if *hi >= deleted {
            *hi += T::ONE;
        }
    }
    (lo, hi)
}

fn binary_search_median<T: PrimitiveInt, P: Fn(T) -> f64, C: Fn(T) -> f64>(
    mut min: T,
    mut max: T,
    pmf: P,
    cdf: C,
) -> (T, Option<T>) {
    let initial_median;
    loop {
        if min == max {
            initial_median = Some(min);
            break;
        }
        let mut mean = min.wrapping_add(max);
        mean >>= 1;
        if mean < min || mean > max {
            mean.flip_bit(T::WIDTH - 1);
        }
        if 1.0 - cdf(mean) > 0.5 {
            min = mean + T::ONE;
        } else if cdf(max) - pmf(max) > 0.5 {
            max = mean;
        } else {
            initial_median = Some(mean);
            break;
        }
    }
    let mut first_median = initial_median.unwrap();
    let mut first_good = false;
    while 1.0 - cdf(first_median) <= 0.5 && cdf(first_median) - pmf(first_median) <= 0.5 {
        first_good = true;
        first_median.wrapping_sub_assign(T::ONE);
    }
    assert!(first_good, "could not find median");
    first_median.wrapping_add_assign(T::ONE);
    let mut last_median = first_median.wrapping_add(T::ONE);
    while 1.0 - cdf(last_median) <= 0.5 && cdf(last_median) - pmf(last_median) <= 0.5 {
        last_median.wrapping_add_assign(T::ONE);
    }
    last_median.wrapping_sub_assign(T::ONE);
    if first_median == last_median {
        (first_median, None)
    } else {
        (first_median, Some(last_median))
    }
}

fn truncated_geometric_pmf(m: f64, unadjusted_mean: f64, n: f64) -> f64 {
    if n >= 0.0 && m >= n {
        let p = 1.0 / (unadjusted_mean + 1.0);
        let q = 1.0 - p;
        (q.powf(n) * p) / (1.0 - q.powf(1.0 + m))
    } else {
        0.0
    }
}

fn truncated_geometric_cdf(m: f64, unadjusted_mean: f64, n: f64) -> f64 {
    let p = 1.0 / (unadjusted_mean + 1.0);
    if n < 0.0 {
        0.0
    } else if n <= m {
        let q = 1.0 - p;
        (1.0 - q.powf(1.0 + n)) / (1.0 - q.powf(1.0 + m))
    } else {
        1.0
    }
}

pub fn truncated_geometric_median<T: CheckedToF64 + PrimitiveInt>(
    min: T,
    max: T,
    unadjusted_mean: f64,
) -> (T, Option<T>) {
    assert!(min >= T::ZERO);
    assert!(min <= max);
    let min_64 = min.checked_to_f64();
    let max_64 = max.checked_to_f64() - min_64;
    let unadjusted_mean = unadjusted_mean - min_64;
    binary_search_median(
        min,
        max,
        |n| truncated_geometric_pmf(max_64, unadjusted_mean, n.checked_to_f64() - min_64),
        |n| truncated_geometric_cdf(max_64, unadjusted_mean, n.checked_to_f64() - min_64),
    )
}

fn double_nonzero_geometric_pmf(a: f64, b: f64, unadjusted_mean: f64, n: f64) -> f64 {
    if n == 0.0 || n > a || n < -b {
        0.0
    } else {
        let p = 1.0 / (unadjusted_mean + 1.0);
        let q = 1.0 - p;
        q.powf(-1.0 + n.abs()) * p / (2.0 - q.powf(a) - q.powf(b))
    }
}

fn double_nonzero_geometric_cdf(a: f64, b: f64, unadjusted_mean: f64, n: f64) -> f64 {
    if n < -b {
        return 0.0;
    } else if n >= a {
        return 1.0;
    }
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    let d = 2.0 - q.powf(a) - q.powf(b);
    if n == -b {
        q.powf(-1.0 + b) * p / d
    } else if n < 0.0 {
        (1.0 - q.powf(1.0 + b + n)) * q.powf(-1.0 - n) / d
    } else {
        (2.0 - q.powf(b) - q.powf(n)) / d
    }
}

pub fn double_nonzero_geometric_median<T: CheckedToF64 + PrimitiveSigned>(
    min: T,
    max: T,
    unadjusted_mean: f64,
) -> (T, Option<T>) {
    assert!(min < T::ZERO);
    assert!(max > T::ZERO);
    let min_64 = -min.checked_to_f64();
    let max_64 = max.checked_to_f64();
    binary_search_median(
        min,
        max,
        |n| double_nonzero_geometric_pmf(max_64, min_64, unadjusted_mean, n.checked_to_f64()),
        |n| double_nonzero_geometric_cdf(max_64, min_64, unadjusted_mean, n.checked_to_f64()),
    )
}

fn double_geometric_pmf(a: f64, b: f64, unadjusted_mean: f64, n: f64) -> f64 {
    if n > a || n < -b {
        0.0
    } else {
        let p = 1.0 / (unadjusted_mean + 1.0);
        let q = 1.0 - p;
        let qpa = q.powf(a);
        q.powf(n.abs()) * p / (2.0 - qpa - q.powf(1.0 + b) - p + qpa * p)
    }
}

fn double_geometric_cdf(a: f64, b: f64, unadjusted_mean: f64, n: f64) -> f64 {
    if n < -b {
        return 0.0;
    } else if n >= a {
        return 1.0;
    }
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    let qpa = q.powf(a);
    let qpb = q.powf(b);
    let d = 2.0 - qpa - q.powf(1.0 + b) - p + qpa * p;
    if n == -b {
        qpb * p / d
    } else if n <= 0.0 {
        (1.0 - q.powf(1.0 + b + n)) * q.powf(-n) / d
    } else {
        (2.0 - qpb - q.powf(1.0 + n) - p + qpb * p) / d
    }
}

pub fn double_geometric_median<T: CheckedToF64 + PrimitiveSigned>(
    min: T,
    max: T,
    unadjusted_mean: f64,
) -> (T, Option<T>) {
    assert!(min < T::ZERO);
    assert!(max > T::ZERO);
    let min_64 = -min.checked_to_f64();
    let max_64 = max.checked_to_f64();
    binary_search_median(
        min,
        max,
        |n| double_geometric_pmf(max_64, min_64, unadjusted_mean, n.checked_to_f64()),
        |n| double_geometric_cdf(max_64, min_64, unadjusted_mean, n.checked_to_f64()),
    )
}
