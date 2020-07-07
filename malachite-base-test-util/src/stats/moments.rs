use std::fmt::{self, Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::Sub;

use num::float::nice_float::NiceFloat;
use stats::common_values_map::common_values_map;
use stats::median;

// Panics if the input exceeds the finite range of f64.
pub trait CheckedToF64 {
    fn checked_to_f64(&self) -> f64;
}

macro_rules! impl_checked_to_f64_for_primitive_integers {
    ($t: ident) => {
        impl CheckedToF64 for $t {
            #[inline]
            fn checked_to_f64(&self) -> f64 {
                // No primitive integer type, not even u128 and i128, exceeds the finite range of
                // f64.
                *self as f64
            }
        }
    };
}
apply_to_primitive_ints!(impl_checked_to_f64_for_primitive_integers);

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
pub fn pop_disc_uniform_dist_median<T: CheckedToF64>(a: &T, b: &T) -> NiceFloat<f64> {
    NiceFloat((a.checked_to_f64() + b.checked_to_f64()) / 2.0)
}

fn pop_disc_uniform_dist_mean<T: CheckedToF64>(a: &T, b: &T) -> f64 {
    pop_disc_uniform_dist_median(a, b).unwrap()
}

fn pop_disc_uniform_dist_standard_deviation<T: CheckedToF64>(a: &T, b: &T) -> f64 {
    let a = a.checked_to_f64();
    let b = b.checked_to_f64();
    let n = b - a + 1.0;
    ((n * n - 1.0) / 12.0).sqrt()
}

fn pop_disc_uniform_dist_skewness<T: CheckedToF64>(a: &T, b: &T) -> f64 {
    let a = a.checked_to_f64();
    let b = b.checked_to_f64();
    if a == b {
        f64::NAN
    } else {
        0.0
    }
}

fn pop_disc_uniform_dist_excess_kurtosis<T: CheckedToF64>(a: &T, b: &T) -> f64 {
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
        mean: NiceFloat(pop_disc_uniform_dist_mean(a, b)),
        standard_deviation: NiceFloat(pop_disc_uniform_dist_standard_deviation(a, b)),
        skewness: NiceFloat(pop_disc_uniform_dist_skewness(a, b)),
        excess_kurtosis: NiceFloat(pop_disc_uniform_dist_excess_kurtosis(a, b)),
    }
}

pub fn disc_uniform_dist_assertions<I: Clone + Iterator>(
    xs: I,
    a: &I::Item,
    b: &I::Item,
    expected_values: &[I::Item],
    expected_common_values: &[(I::Item, usize)],
    expected_pop_median: NiceFloat<f64>,
    expected_sample_median: (I::Item, Option<I::Item>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) where
    I::Item: CheckedToF64 + Debug + Display + Eq + Hash + Ord,
{
    let actual_values = xs.clone().take(20).collect::<Vec<I::Item>>();
    let actual_common_values = common_values_map(1_000_000, 10, xs.clone());
    let actual_sample_median = median(xs.clone().take(1_000_000));
    let actual_sample_moment_stats = moment_stats(xs.take(1_000_000));
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_common_values.as_slice(),
            pop_disc_uniform_dist_median(a, b),
            actual_sample_median,
            pop_disc_uniform_dist_moment_stats(a, b),
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

pub fn pop_deleted_disc_uniform_dist_median<T: CheckedToF64>(
    a: &T,
    b: &T,
    c: &T,
) -> NiceFloat<f64> {
    let undeleted_median = pop_disc_uniform_dist_median(a, b).0;
    let c = c.checked_to_f64();
    NiceFloat(if c >= undeleted_median {
        undeleted_median
    } else {
        undeleted_median + 0.5
    })
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

fn pop_deleted_disc_uniform_dist_moment_stats<T: CheckedToF64 + Ord>(
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

pub fn deleted_disc_uniform_dist_assertions<I: Clone + Iterator>(
    xs: I,
    a: &I::Item,
    b: &I::Item,
    c: &I::Item,
    expected_values: &[I::Item],
    expected_common_values: &[(I::Item, usize)],
    expected_pop_median: NiceFloat<f64>,
    expected_sample_median: (I::Item, Option<I::Item>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) where
    I::Item: CheckedToF64 + Debug + Display + Eq + Hash + Ord,
{
    let actual_values = xs.clone().take(20).collect::<Vec<I::Item>>();
    let actual_common_values = common_values_map(1_000_000, 10, xs.clone());
    let actual_sample_median = median(xs.clone().take(1_000_000));
    let actual_sample_moment_stats = moment_stats(xs.take(1_000_000));
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_common_values.as_slice(),
            pop_deleted_disc_uniform_dist_median(a, b, c),
            actual_sample_median,
            pop_deleted_disc_uniform_dist_moment_stats(a, b, c),
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

pub fn pop_truncated_geometric_dist_median(
    unadjusted_mean: f64,
    min: f64,
    max: f64,
) -> NiceFloat<f64> {
    let m = max - min;
    let p = 1.0 / ((unadjusted_mean - min) + 1.0);
    let c = 1.0 - (1.0 - p).powf(m + 1.0);
    let q = 1.0 - p;
    NiceFloat(((1.0 - c / 2.0).log(q) - 1.0).ceil() + min)
}

// unadjusted_mean is what the mean would be if the distribution were not truncated.
fn pop_truncated_geometric_dist_mean(unadjusted_mean: f64, max: f64) -> f64 {
    let m = max;
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    (q - q.powf(m) * q * (1.0 + m * p)) / (p - q.powf(1.0 + m) * p)
}

fn pop_truncated_geometric_dist_2_mean(unadjusted_mean: f64, max: f64) -> f64 {
    let m = max;
    let p = 1.0 / (unadjusted_mean + 1.0);
    let q = 1.0 - p;
    let p_2 = p * p;
    (2.0 - 3.0 * p + p_2 - q.powf(m) * q * (2.0 + p * (-1.0 + m * (2.0 + m * p))))
        / ((1.0 - q.powf(1.0 + m)) * p_2)
}

fn pop_truncated_geometric_dist_3_mean(unadjusted_mean: f64, max: f64) -> f64 {
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

fn pop_truncated_geometric_dist_4_mean(unadjusted_mean: f64, max: f64) -> f64 {
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
    unadjusted_mean: f64,
    min: f64,
    max: f64,
) -> MomentStats {
    assert!(min <= max);
    assert!(min <= unadjusted_mean); // unadjusted_mean may be arbitrarily large
    let unadjusted_mean = unadjusted_mean - min;
    let max = max - min;
    let mut stats = moment_stats_from_raw_moments(
        pop_truncated_geometric_dist_mean(unadjusted_mean, max),
        pop_truncated_geometric_dist_2_mean(unadjusted_mean, max),
        pop_truncated_geometric_dist_3_mean(unadjusted_mean, max),
        pop_truncated_geometric_dist_4_mean(unadjusted_mean, max),
    );
    stats.mean = NiceFloat(stats.mean.0 + min);
    stats
}

pub fn truncated_geometric_dist_assertions<I: Clone + Iterator>(
    xs: I,
    um_numerator: u64,
    um_denominator: u64,
    min: &I::Item,
    max: &I::Item,
    expected_values: &[I::Item],
    expected_common_values: &[(I::Item, usize)],
    expected_pop_median: NiceFloat<f64>,
    expected_sample_median: (I::Item, Option<I::Item>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) where
    I::Item:
        CheckedToF64 + Copy + Debug + Display + Eq + Hash + Ord + Sub<I::Item, Output = I::Item>,
{
    let min = min.checked_to_f64();
    let max = max.checked_to_f64();
    let unadjusted_mean = um_numerator as f64 / um_denominator as f64;
    let actual_values = xs.clone().take(20).collect::<Vec<I::Item>>();
    let actual_common_values = common_values_map(1_000_000, 10, xs.clone());
    let actual_sample_median = median(xs.clone().take(1_000_000));
    let actual_sample_moment_stats = moment_stats(xs.take(1_000_000));
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_common_values.as_slice(),
            pop_truncated_geometric_dist_median(unadjusted_mean, min, max),
            actual_sample_median,
            pop_truncated_geometric_dist_moment_stats(unadjusted_mean, min, max),
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

pub fn negative_truncated_geometric_dist_assertions<I: Clone + Iterator>(
    xs: I,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
    abs_min: &I::Item,
    abs_max: &I::Item,
    expected_values: &[I::Item],
    expected_common_values: &[(I::Item, usize)],
    expected_pop_median: NiceFloat<f64>,
    expected_sample_median: (I::Item, Option<I::Item>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) where
    I::Item:
        CheckedToF64 + Copy + Debug + Display + Eq + Hash + Ord + Sub<I::Item, Output = I::Item>,
{
    let abs_min = -abs_min.checked_to_f64();
    let abs_max = -abs_max.checked_to_f64();
    let abs_unadjusted_mean = abs_um_numerator as f64 / abs_um_denominator as f64;
    let actual_values = xs.clone().take(20).collect::<Vec<I::Item>>();
    let actual_common_values = common_values_map(1_000_000, 10, xs.clone());
    let actual_sample_median = median(xs.clone().take(1_000_000));
    let actual_sample_moment_stats = moment_stats(xs.take(1_000_000));
    let mut pop_sample_moment_stats =
        pop_truncated_geometric_dist_moment_stats(abs_unadjusted_mean, abs_min, abs_max);
    pop_sample_moment_stats.mean = NiceFloat(-pop_sample_moment_stats.mean.0);
    pop_sample_moment_stats.skewness = NiceFloat(-pop_sample_moment_stats.skewness.0);
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_common_values.as_slice(),
            NiceFloat(
                -pop_truncated_geometric_dist_median(abs_unadjusted_mean, abs_min, abs_max).0
            ),
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
