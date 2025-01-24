// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{SaturatingAbs, UnsignedAbs};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::num::float::NiceFloat;
use crate::test_util::stats::common_values_map::common_values_map;
use crate::test_util::stats::median;
use crate::test_util::stats::median::{
    deleted_uniform_primitive_int_median, double_geometric_median, double_nonzero_geometric_median,
    truncated_geometric_median, uniform_bool_median, uniform_primitive_int_median,
};
use itertools::Itertools;
use std::fmt::{self, Debug, Formatter};

// Panics if the input exceeds the finite range of f64.
pub trait CheckedToF64 {
    fn checked_to_f64(&self) -> f64;
}

macro_rules! impl_checked_to_f64_for_primitive_ints {
    ($t: ident) => {
        impl CheckedToF64 for $t {
            #[allow(clippy::cast_lossless)]
            #[inline]
            fn checked_to_f64(&self) -> f64 {
                // No primitive integer type, not even u128 and i128, exceeds the finite range of
                // f64.
                *self as f64
            }
        }
    };
}
apply_to_primitive_ints!(impl_checked_to_f64_for_primitive_ints);

impl CheckedToF64 for bool {
    #[inline]
    fn checked_to_f64(&self) -> f64 {
        if *self {
            1.0
        } else {
            0.0
        }
    }
}

impl CheckedToF64 for f32 {
    #[inline]
    fn checked_to_f64(&self) -> f64 {
        f64::from(*self)
    }
}

impl CheckedToF64 for f64 {
    #[inline]
    fn checked_to_f64(&self) -> f64 {
        *self
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MomentStats {
    pub mean: NiceFloat<f64>,
    pub standard_deviation: NiceFloat<f64>,
    pub skewness: NiceFloat<f64>,
    pub excess_kurtosis: NiceFloat<f64>,
}

impl Debug for MomentStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "MomentStats {{ mean: NiceFloat({}), standard_deviation: NiceFloat({}), \
                skewness: NiceFloat({}), excess_kurtosis: NiceFloat({}) }}",
            self.mean, self.standard_deviation, self.skewness, self.excess_kurtosis
        ))
    }
}

// From https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance
#[allow(clippy::suspicious_operation_groupings)]
pub fn moment_stats<I: Iterator>(xs: I) -> MomentStats
where
    I::Item: CheckedToF64,
{
    let mut n: usize = 1;
    let mut m = 0.0;
    let mut m_2 = 0.0;
    let mut m_3 = 0.0;
    let mut m_4 = 0.0;
    for x in xs {
        let d = x.checked_to_f64() - m;
        let nf = n.checked_to_f64();
        let d_n = d / nf;
        let d_n_2 = d_n * d_n;
        let d_n_3 = d_n_2 * d_n;
        let m_4_1 = (nf - 1.0) * (nf * nf - 3.0 * nf + 3.0) * d_n_3 * d;
        let m_4_2 = m_2 * d_n_2 * 6.0;
        let m_4_3 = m_3 * d_n * 4.0;
        m_4 += m_4_1 + m_4_2 - m_4_3;
        let m_3_1 = (nf - 1.0) * (nf - 2.0) * d_n_2 * d;
        let m_3_2 = m_2 * d_n * 3.0;
        m_3 += m_3_1 - m_3_2;
        m_2 += (nf - 1.0) * d_n * d;
        m += d_n;
        n += 1;
    }
    n -= 1;
    let nf = n.checked_to_f64();
    let mean = m;
    let standard_deviation = (m_2 / (nf - 1.0)).sqrt();
    let sqrt_m_2 = m_2.sqrt();
    let skewness = m_3 * nf.sqrt() / (sqrt_m_2 * sqrt_m_2 * sqrt_m_2);
    let excess_kurtosis = m_4 * nf / (m_2 * m_2) - 3.0;
    MomentStats {
        mean: NiceFloat(mean),
        standard_deviation: NiceFloat(standard_deviation),
        skewness: NiceFloat(skewness),
        excess_kurtosis: NiceFloat(excess_kurtosis),
    }
}

pub const NAN_MOMENT_STATS: MomentStats = MomentStats {
    mean: NiceFloat(f64::NAN),
    standard_deviation: NiceFloat(f64::NAN),
    skewness: NiceFloat(f64::NAN),
    excess_kurtosis: NiceFloat(f64::NAN),
};

pub fn mean<I: Iterator>(xs: I) -> NiceFloat<f64>
where
    I::Item: CheckedToF64,
{
    let mut n: usize = 1;
    let mut m = 0.0;
    for x in xs {
        let d_n = (x.checked_to_f64() - m) / n.checked_to_f64();
        m += d_n;
        n += 1;
    }
    NiceFloat(m)
}

fn moment_stats_from_raw_moments(
    mean: f64,
    p_2_mean: f64,
    p_3_mean: f64,
    p_4_mean: f64,
) -> MomentStats {
    let mean_2 = mean * mean;
    let mean_3 = mean_2 * mean;
    let mean_4 = mean_2 * mean_2;
    let variance = p_2_mean - mean_2;
    let standard_deviation = variance.sqrt();
    let mu_3 = p_3_mean - 3.0 * mean * p_2_mean + 2.0 * mean_3;
    let mu_4 = p_4_mean - 4.0 * mean * p_3_mean + 6.0 * mean_2 * p_2_mean - 3.0 * mean_4;
    MomentStats {
        mean: NiceFloat(mean),
        standard_deviation: NiceFloat(standard_deviation),
        skewness: NiceFloat(mu_3 / (variance * standard_deviation)),
        excess_kurtosis: NiceFloat(mu_4 / (variance * variance) - 3.0),
    }
}

#[inline]
fn pop_uniform_mean<T: CheckedToF64>(a: &T, b: &T) -> f64 {
    (a.checked_to_f64() + b.checked_to_f64()) / 2.0
}

fn pop_uniform_standard_deviation<T: CheckedToF64>(a: &T, b: &T) -> f64 {
    let a = a.checked_to_f64();
    let b = b.checked_to_f64();
    let n = b - a + 1.0;
    ((n * n - 1.0) / 12.0).sqrt()
}

fn pop_uniform_skewness<T: CheckedToF64>(a: &T, b: &T) -> f64 {
    let a = a.checked_to_f64();
    let b = b.checked_to_f64();
    if a == b {
        f64::NAN
    } else {
        0.0
    }
}

fn pop_uniform_excess_kurtosis<T: CheckedToF64>(a: &T, b: &T) -> f64 {
    let a = a.checked_to_f64();
    let b = b.checked_to_f64();
    if a == b {
        f64::NAN
    } else {
        let n = b - a + 1.0;
        let n_squared = n * n;
        6.0 / 5.0 * (-1.0 - 2.0 / (-1.0 + n_squared))
    }
}

pub fn pop_disc_uniform_dist_moment_stats<T: CheckedToF64>(a: &T, b: &T) -> MomentStats {
    MomentStats {
        mean: NiceFloat(pop_uniform_mean(a, b)),
        standard_deviation: NiceFloat(pop_uniform_standard_deviation(a, b)),
        skewness: NiceFloat(pop_uniform_skewness(a, b)),
        excess_kurtosis: NiceFloat(pop_uniform_excess_kurtosis(a, b)),
    }
}

pub fn uniform_bool_assertions<I: Clone + Iterator<Item = bool>>(
    xs: I,
    a: bool,
    b: bool,
    expected_values: &[bool],
    expected_common_values: &[(bool, usize)],
    expected_pop_median: (bool, Option<bool>),
    expected_sample_median: (bool, Option<bool>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    let actual_values = xs.clone().take(20).collect_vec();
    let actual_common_values = common_values_map(1000000, 10, xs.clone());
    let actual_sample_median = median(xs.clone().take(1000000));
    let actual_sample_moment_stats = moment_stats(xs.take(1000000));
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_common_values.as_slice(),
            uniform_bool_median(a, b),
            actual_sample_median,
            pop_disc_uniform_dist_moment_stats(&a, &b),
            actual_sample_moment_stats
        ),
        (
            expected_values,
            expected_common_values,
            expected_pop_median,
            expected_sample_median,
            expected_pop_moment_stats,
            expected_sample_moment_stats
        )
    );
}

pub fn uniform_primitive_int_assertions<I: Clone + Iterator>(
    xs: I,
    a: I::Item,
    b: I::Item,
    expected_values: &[I::Item],
    expected_common_values: &[(I::Item, usize)],
    expected_pop_median: (I::Item, Option<I::Item>),
    expected_sample_median: (I::Item, Option<I::Item>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) where
    I::Item: CheckedToF64 + PrimitiveInt,
{
    let actual_values = xs.clone().take(20).collect_vec();
    let actual_common_values = common_values_map(1000000, 10, xs.clone());
    let actual_sample_median = median(xs.clone().take(1000000));
    let actual_sample_moment_stats = moment_stats(xs.take(1000000));
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_common_values.as_slice(),
            uniform_primitive_int_median(a, b),
            actual_sample_median,
            pop_disc_uniform_dist_moment_stats(&a, &b),
            actual_sample_moment_stats
        ),
        (
            expected_values,
            expected_common_values,
            expected_pop_median,
            expected_sample_median,
            expected_pop_moment_stats,
            expected_sample_moment_stats
        )
    );
}

fn pop_deleted_disc_uniform_dist_mean<T: CheckedToF64>(a: &T, b: &T, c: &T) -> f64 {
    let a = a.checked_to_f64();
    let b = b.checked_to_f64();
    let c = c.checked_to_f64();
    (a - a * a + b + b * b - 2.0 * c) / (2.0 * (b - a))
}

fn pop_deleted_disc_uniform_dist_2_mean<T: CheckedToF64>(a: &T, b: &T, c: &T) -> f64 {
    let a = a.checked_to_f64();
    let b = b.checked_to_f64();
    let c = c.checked_to_f64();
    let a_2 = a * a;
    let a_3 = a_2 * a;
    (a - 3.0 * a_2 + 2.0 * a_3 - b * (1.0 + b) * (1.0 + 2.0 * b) + 6.0 * c * c) / (6.0 * (a - b))
}

fn pop_deleted_disc_uniform_dist_3_mean<T: CheckedToF64>(a: &T, b: &T, c: &T) -> f64 {
    let a = a.checked_to_f64();
    let b = b.checked_to_f64();
    let c = c.checked_to_f64();
    let a_2 = a * a;
    let b_2 = b * b;
    let a_m_1 = a - 1.0;
    let b_p_1 = b + 1.0;
    (a_m_1 * a_m_1 * a_2 - b_2 * b_p_1 * b_p_1 + 4.0 * c * c * c) / (4.0 * (a - b))
}

fn pop_deleted_disc_uniform_dist_4_mean<T: CheckedToF64>(a: &T, b: &T, c: &T) -> f64 {
    let a = a.checked_to_f64();
    let b = b.checked_to_f64();
    let c = c.checked_to_f64();
    let a_2 = a * a;
    let a_3 = a_2 * a;
    let a_4 = a_2 * a_2;
    let a_5 = a_3 * a_2;
    let b_2 = b * b;
    let b_3 = b_2 * b;
    let b_4 = b_2 * b_2;
    let b_5 = b_3 * b_2;
    let c_2 = c * c;
    (-a + 10.0 * a_3 - 15.0 * a_4 + 6.0 * a_5 + b - 10.0 * b_3 - 15.0 * b_4 - 6.0 * b_5
        + 30.0 * c_2 * c_2)
        / (30.0 * (a - b))
}

fn deleted_uniform_primitive_int_moment_stats<T: CheckedToF64 + Ord>(
    a: &T,
    b: &T,
    c: &T,
) -> MomentStats {
    assert!(a <= c);
    assert!(c <= b);
    moment_stats_from_raw_moments(
        pop_deleted_disc_uniform_dist_mean(a, b, c),
        pop_deleted_disc_uniform_dist_2_mean(a, b, c),
        pop_deleted_disc_uniform_dist_3_mean(a, b, c),
        pop_deleted_disc_uniform_dist_4_mean(a, b, c),
    )
}

pub fn deleted_uniform_primitive_int_assertions<I: Clone + Iterator>(
    xs: I,
    a: I::Item,
    b: I::Item,
    c: I::Item,
    expected_values: &[I::Item],
    expected_common_values: &[(I::Item, usize)],
    expected_pop_median: (I::Item, Option<I::Item>),
    expected_sample_median: (I::Item, Option<I::Item>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) where
    I::Item: CheckedToF64 + PrimitiveInt,
{
    let actual_values = xs.clone().take(20).collect_vec();
    let actual_common_values = common_values_map(1000000, 10, xs.clone());
    let actual_sample_median = median(xs.clone().take(1000000));
    let actual_sample_moment_stats = moment_stats(xs.take(1000000));
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_common_values.as_slice(),
            deleted_uniform_primitive_int_median(a, b, c),
            actual_sample_median,
            deleted_uniform_primitive_int_moment_stats(&a, &b, &c),
            actual_sample_moment_stats
        ),
        (
            expected_values,
            expected_common_values,
            expected_pop_median,
            expected_sample_median,
            expected_pop_moment_stats,
            expected_sample_moment_stats
        )
    );
}

// unadjusted_mean is what the mean would be if the distribution were not truncated.
fn pop_truncated_geometric_dist_mean(max: f64, unadjusted_mean: f64) -> f64 {
    let m = max;
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    (q - q.powf(m) * q * (1.0 + m * p)) / (p - q.powf(1.0 + m) * p)
}

fn pop_truncated_geometric_dist_2_mean(max: f64, unadjusted_mean: f64) -> f64 {
    let m = max;
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    let p_2 = p * p;
    (2.0 - 3.0 * p + p_2 - q.powf(m) * q * (2.0 + p * (-1.0 + m * (2.0 + m * p))))
        / ((1.0 - q.powf(1.0 + m)) * p_2)
}

fn pop_truncated_geometric_dist_3_mean(max: f64, unadjusted_mean: f64) -> f64 {
    let m = max;
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    let p_2 = p * p;
    let p_3 = p_2 * p;
    (6.0 - 12.0 * p + 7.0 * p_2
        - p_3
        - q.powf(m) * q * (6.0 + p * (-6.0 + p + m * (6.0 + p * (-3.0 + m * (3.0 + m * p))))))
        / ((1.0 - q.powf(1.0 + m)) * p_3)
}

fn pop_truncated_geometric_dist_4_mean(max: f64, unadjusted_mean: f64) -> f64 {
    let m = max;
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    let m_2 = m * m;
    let m_4 = m_2 * m_2;
    let p_2 = p * p;
    let p_3 = p_2 * p;
    let p_4 = p_2 * p_2;
    (1.0 / ((1.0 - q.powf(1.0 + m)) * p_4))
        * (24.0 - 60.0 * p + 50.0 * p_2 - 15.0 * p_3 + p_4
            - q.powf(m)
                * q
                * (24.0
                    + p * (-36.0
                        + 24.0 * m
                        + 2.0 * (7.0 + 6.0 * (-2.0 + m) * m) * p
                        + (-1.0 + 2.0 * m * (2.0 + m * (-3.0 + 2.0 * m))) * p_2
                        + m_4 * p_3)))
}

fn pop_truncated_geometric_dist_moment_stats(
    min: f64,
    max: f64,
    unadjusted_mean: f64,
) -> MomentStats {
    assert!(min <= max);
    assert!(min <= unadjusted_mean); // unadjusted_mean may be arbitrarily large
    let unadjusted_mean = unadjusted_mean - min;
    let max = max - min;
    let mut stats = moment_stats_from_raw_moments(
        pop_truncated_geometric_dist_mean(max, unadjusted_mean),
        pop_truncated_geometric_dist_2_mean(max, unadjusted_mean),
        pop_truncated_geometric_dist_3_mean(max, unadjusted_mean),
        pop_truncated_geometric_dist_4_mean(max, unadjusted_mean),
    );
    stats.mean = NiceFloat(stats.mean.0 + min);
    stats
}

pub fn truncated_geometric_dist_assertions<I: Clone + Iterator>(
    xs: I,
    min: I::Item,
    max: I::Item,
    um_numerator: u64,
    um_denominator: u64,
    expected_values: &[I::Item],
    expected_common_values: &[(I::Item, usize)],
    expected_pop_median: (I::Item, Option<I::Item>),
    expected_sample_median: (I::Item, Option<I::Item>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) where
    I::Item: CheckedToF64 + PrimitiveInt,
{
    let min_64 = min.checked_to_f64();
    let max_64 = max.checked_to_f64();
    let unadjusted_mean = um_numerator as f64 / um_denominator as f64;
    let actual_values = xs.clone().take(20).collect_vec();
    let actual_common_values = common_values_map(1000000, 10, xs.clone());
    let actual_sample_median = median(xs.clone().take(1000000));
    let actual_sample_moment_stats = moment_stats(xs.take(1000000));
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_common_values.as_slice(),
            truncated_geometric_median(min, max, unadjusted_mean),
            actual_sample_median,
            pop_truncated_geometric_dist_moment_stats(min_64, max_64, unadjusted_mean),
            actual_sample_moment_stats
        ),
        (
            expected_values,
            expected_common_values,
            expected_pop_median,
            expected_sample_median,
            expected_pop_moment_stats,
            expected_sample_moment_stats
        )
    );
}

pub fn negative_truncated_geometric_dist_assertions<
    I: Clone + Iterator<Item = S>,
    S: CheckedToF64 + PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
    U: CheckedToF64 + PrimitiveUnsigned,
>(
    xs: I,
    abs_min: S,
    abs_max: S,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
    expected_values: &[S],
    expected_common_values: &[(S, usize)],
    expected_pop_median: (S, Option<S>),
    expected_sample_median: (S, Option<S>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    let abs_min_64 = -abs_min.checked_to_f64();
    let abs_max_64 = -abs_max.checked_to_f64();
    let abs_unadjusted_mean = abs_um_numerator as f64 / abs_um_denominator as f64;
    let actual_values = xs.clone().take(20).collect_vec();
    let actual_common_values = common_values_map(1000000, 10, xs.clone());
    let actual_sample_median = median(xs.clone().take(1000000));
    let actual_sample_moment_stats = moment_stats(xs.take(1000000));
    let mut pop_sample_moment_stats =
        pop_truncated_geometric_dist_moment_stats(abs_min_64, abs_max_64, abs_unadjusted_mean);
    pop_sample_moment_stats.mean = NiceFloat(-pop_sample_moment_stats.mean.0);
    pop_sample_moment_stats.skewness = NiceFloat(-pop_sample_moment_stats.skewness.0);
    let (x, y) = truncated_geometric_median(
        abs_min.unsigned_abs(),
        abs_max.unsigned_abs(),
        abs_unadjusted_mean,
    );
    let (x, y) = y.map_or((S::wrapping_from(x.wrapping_neg()), None), |y| {
        (
            S::wrapping_from(y.wrapping_neg()),
            Some(S::wrapping_from(x.wrapping_neg())),
        )
    });
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_common_values.as_slice(),
            (x, y),
            actual_sample_median,
            pop_sample_moment_stats,
            actual_sample_moment_stats
        ),
        (
            expected_values,
            expected_common_values,
            expected_pop_median,
            expected_sample_median,
            expected_pop_moment_stats,
            expected_sample_moment_stats
        )
    );
}

fn pop_double_nonzero_truncated_geometric_dist_mean(b: f64, a: f64, unadjusted_mean: f64) -> f64 {
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    let qpa = q.powf(a);
    let qpb = q.powf(b);
    (qpa * (1.0 + a * p) - qpb * (1.0 + b * p)) / ((-2.0 + qpa + qpb) * p)
}

fn pop_double_nonzero_truncated_geometric_dist_2_mean(b: f64, a: f64, unadjusted_mean: f64) -> f64 {
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    let qpa = q.powf(a);
    let qpb = q.powf(b);
    (2.0 + (p * (2.0 - qpa + a * qpa * (2.0 + a * p) + qpb * (-1.0 + b * (2.0 + b * p))))
        / (-2.0 + qpa + qpb))
        / (p * p)
}

fn pop_double_nonzero_truncated_geometric_dist_3_mean(b: f64, a: f64, unadjusted_mean: f64) -> f64 {
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    let qpa = q.powf(a);
    let qpb = q.powf(b);
    (qpa * (6.0 + p * (-6.0 + p + a * (6.0 + p * (-3.0 + a * (3.0 + a * p)))))
        - qpb * (6.0 + p * (-6.0 + p + b * (6.0 + p * (-3.0 + b * (3.0 + b * p))))))
        / ((-2.0 + qpa + qpb) * p * p * p)
}

fn pop_double_nonzero_truncated_geometric_dist_4_mean(b: f64, a: f64, unadjusted_mean: f64) -> f64 {
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    let a_2 = a * a;
    let a_3 = a_2 * a;
    let a_4 = a_2 * a_2;
    let b_2 = b * b;
    let b_4 = b_2 * b_2;
    let p_2 = p * p;
    let p_3 = p_2 * p;
    let p_4 = p_2 * p_2;
    let qpa = q.powf(a);
    let qpb = q.powf(b);
    1.0 / p_4
        * (24.0
            + 1.0 / (-2.0 + qpa + qpb)
                * p
                * (-6.0 * a_2 * qpa * (-2.0 + p) * p + 4.0 * a_3 * qpa * p_2 + a_4 * qpa * p_3
                    - (-2.0 + qpa) * (36.0 + (-14.0 + p) * p)
                    + 4.0 * a * qpa * (6.0 + (-6.0 + p) * p)
                    + qpb
                        * (-36.0
                            + 24.0 * b
                            + 2.0 * (7.0 + 6.0 * (-2.0 + b) * b) * p
                            + (-1.0 + 2.0 * b * (2.0 + b * (-3.0 + 2.0 * b))) * p_2
                            + b_4 * p_3)))
}

fn pop_double_nonzero_truncated_geometric_dist_moment_stats(
    min: f64,
    max: f64,
    unadjusted_mean: f64,
) -> MomentStats {
    assert!(min < 0.0);
    assert!(max > 0.0);
    assert!(unadjusted_mean > 1.0); // unadjusted_mean may be arbitrarily large
    moment_stats_from_raw_moments(
        pop_double_nonzero_truncated_geometric_dist_mean(-min, max, unadjusted_mean - 1.0),
        pop_double_nonzero_truncated_geometric_dist_2_mean(-min, max, unadjusted_mean - 1.0),
        pop_double_nonzero_truncated_geometric_dist_3_mean(-min, max, unadjusted_mean - 1.0),
        pop_double_nonzero_truncated_geometric_dist_4_mean(-min, max, unadjusted_mean - 1.0),
    )
}

pub fn double_nonzero_truncated_geometric_dist_assertions<I: Clone + Iterator>(
    xs: I,
    min: I::Item,
    max: I::Item,
    um_numerator: u64,
    um_denominator: u64,
    expected_values: &[I::Item],
    expected_common_values: &[(I::Item, usize)],
    expected_abs_mean: NiceFloat<f64>,
    expected_pop_median: (I::Item, Option<I::Item>),
    expected_sample_median: (I::Item, Option<I::Item>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) where
    I::Item: CheckedToF64 + PrimitiveSigned,
{
    let min_64 = min.checked_to_f64();
    let max_64 = max.checked_to_f64();
    let unadjusted_mean = um_numerator as f64 / um_denominator as f64;
    let actual_values = xs.clone().take(20).collect_vec();
    let actual_common_values = common_values_map(1000000, 10, xs.clone());
    let actual_abs_mean = mean(xs.clone().map(SaturatingAbs::saturating_abs).take(1000000));
    let actual_sample_median = median(xs.clone().take(1000000));
    let actual_sample_moment_stats = moment_stats(xs.take(1000000));
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_common_values.as_slice(),
            actual_abs_mean,
            double_nonzero_geometric_median(min, max, unadjusted_mean),
            actual_sample_median,
            pop_double_nonzero_truncated_geometric_dist_moment_stats(
                min_64,
                max_64,
                unadjusted_mean,
            ),
            actual_sample_moment_stats
        ),
        (
            expected_values,
            expected_common_values,
            expected_abs_mean,
            expected_pop_median,
            expected_sample_median,
            expected_pop_moment_stats,
            expected_sample_moment_stats
        )
    );
}

fn pop_double_truncated_geometric_dist_mean(b: f64, a: f64, unadjusted_mean: f64) -> f64 {
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    let qpa = q.powf(a);
    q * (qpa * (1.0 + a * p) - q.powf(b) * (1.0 + b * p))
        / (p * (-2.0 + qpa + q.powf(1.0 + b) + p - qpa * p))
}

#[allow(clippy::suspicious_operation_groupings)]
fn pop_double_truncated_geometric_dist_2_mean(b: f64, a: f64, unadjusted_mean: f64) -> f64 {
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    let qpa = q.powf(a);
    let qpb = q.powf(b);
    q * (2.0 * (-2.0 + qpa + qpb)
        + p * (2.0 - qpa + a * qpa * (2.0 + a * p) + qpb * (-1.0 + b * (2.0 + b * p))))
        / (p * p * (-2.0 + qpa + q.powf(1.0 + b) + p - qpa * p))
}

fn pop_double_truncated_geometric_dist_3_mean(b: f64, a: f64, unadjusted_mean: f64) -> f64 {
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    let qpa = q.powf(a);
    let qpb = q.powf(b);
    q * (qpa * (6.0 + p * (-6.0 + p + a * (6.0 + p * (-3.0 + a * (3.0 + a * p)))))
        - qpb * (6.0 + p * (-6.0 + p + b * (6.0 + p * (-3.0 + b * (3.0 + b * p))))))
        / (p * p * p * (-2.0 + qpa + q.powf(1.0 + b) + p - qpa * p))
}

fn pop_double_truncated_geometric_dist_4_mean(b: f64, a: f64, unadjusted_mean: f64) -> f64 {
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    let qpa = q.powf(a);
    let qpb = q.powf(b);
    let a_2 = a * a;
    let a_3 = a_2 * a;
    let a_4 = a_2 * a_2;
    let b_2 = b * b;
    let b_4 = b_2 * b_2;
    let p_2 = p * p;
    let p_3 = p_2 * p;
    let p_4 = p_2 * p_2;
    q * (24.0 * (-2.0 + qpa + qpb)
        + p * (-6.0 * a_2 * qpa * (-2.0 + p) * p + 4.0 * a_3 * qpa * p_2 + a_4 * qpa * p_3
            - (-2.0 + qpa) * (36.0 + (-14.0 + p) * p)
            + 4.0 * a * qpa * (6.0 + (-6.0 + p) * p)
            + qpb
                * (-36.0
                    + 24.0 * b
                    + 2.0 * (7.0 + 6.0 * (-2.0 + b) * b) * p
                    + (-1.0 + 2.0 * b * (2.0 + b * (-3.0 + 2.0 * b))) * p_2
                    + b_4 * p_3)))
        / (p_4 * (-2.0 + qpa + q.powf(1.0 + b) + p - qpa * p))
}

fn pop_double_truncated_geometric_dist_moment_stats(
    min: f64,
    max: f64,
    unadjusted_mean: f64,
) -> MomentStats {
    assert!(min < 0.0);
    assert!(max > 0.0);
    assert!(unadjusted_mean > 0.0); // unadjusted_mean may be arbitrarily large
    moment_stats_from_raw_moments(
        pop_double_truncated_geometric_dist_mean(-min, max, unadjusted_mean),
        pop_double_truncated_geometric_dist_2_mean(-min, max, unadjusted_mean),
        pop_double_truncated_geometric_dist_3_mean(-min, max, unadjusted_mean),
        pop_double_truncated_geometric_dist_4_mean(-min, max, unadjusted_mean),
    )
}

pub fn double_truncated_geometric_dist_assertions<I: Clone + Iterator>(
    xs: I,
    min: I::Item,
    max: I::Item,
    um_numerator: u64,
    um_denominator: u64,
    expected_values: &[I::Item],
    expected_common_values: &[(I::Item, usize)],
    expected_natural_mean: NiceFloat<f64>,
    expected_pop_median: (I::Item, Option<I::Item>),
    expected_sample_median: (I::Item, Option<I::Item>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) where
    I::Item: CheckedToF64 + PrimitiveSigned,
{
    let min_64 = min.checked_to_f64();
    let max_64 = max.checked_to_f64();
    let unadjusted_mean = um_numerator as f64 / um_denominator as f64;
    let actual_values = xs.clone().take(20).collect_vec();
    let actual_common_values = common_values_map(1000000, 10, xs.clone());
    let actual_natural_mean = mean(xs.clone().filter(|&x| x >= I::Item::ZERO).take(1000000));
    let actual_sample_median = median(xs.clone().take(1000000));
    let actual_sample_moment_stats = moment_stats(xs.take(1000000));
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_common_values.as_slice(),
            actual_natural_mean,
            double_geometric_median(min, max, unadjusted_mean),
            actual_sample_median,
            pop_double_truncated_geometric_dist_moment_stats(min_64, max_64, unadjusted_mean),
            actual_sample_moment_stats
        ),
        (
            expected_values,
            expected_common_values,
            expected_natural_mean,
            expected_pop_median,
            expected_sample_median,
            expected_pop_moment_stats,
            expected_sample_moment_stats
        )
    );
}
