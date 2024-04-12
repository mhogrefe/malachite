// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::striped::striped_random_unsigned_inclusive_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToBinaryString;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_base::test_util::stats::median;
use malachite_base::test_util::stats::moments::{moment_stats, CheckedToF64, MomentStats};
use std::panic::catch_unwind;

fn striped_random_unsigned_inclusive_range_helper<T: CheckedToF64 + PrimitiveUnsigned>(
    a: T,
    b: T,
    m_numerator: u64,
    m_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (T, Option<T>),
    expected_sample_moment_stats: MomentStats,
) {
    let xs = striped_random_unsigned_inclusive_range::<T>(
        EXAMPLE_SEED,
        a,
        b,
        m_numerator,
        m_denominator,
    );
    let actual_values = xs
        .clone()
        .map(|x| x.to_binary_string())
        .take(20)
        .collect_vec();
    let actual_common_values = common_values_map(1000000, 10, xs.clone())
        .iter()
        .map(|(x, frequency)| (x.to_binary_string(), *frequency))
        .collect_vec();
    let actual_sample_median = median(xs.clone().take(1000000));
    let actual_sample_moment_stats = moment_stats(xs.take(1000000));
    assert_eq!(
        (
            actual_values,
            actual_common_values,
            actual_sample_median,
            actual_sample_moment_stats
        ),
        (
            expected_values
                .iter()
                .map(ToString::to_string)
                .collect_vec(),
            expected_common_values
                .iter()
                .map(|(x, frequency)| (x.to_string(), *frequency))
                .collect_vec(),
            expected_sample_median,
            expected_sample_moment_stats
        )
    );
}

#[test]
fn test_striped_random_unsigned_inclusive_range() {
    // u8, 5, 5, m = 4
    let values = &["101"; 20];
    let common_values = &[("101", 1000000)];
    let sample_median = (5, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(5.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_unsigned_inclusive_range_helper::<u8>(
        5,
        5,
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // u8, 0, 7, m = 4
    let values = &[
        "0", "0", "0", "101", "11", "100", "11", "11", "0", "111", "111", "100", "0", "11", "111",
        "0", "0", "1", "0", "0",
    ];
    let common_values = &[
        ("111", 281415),
        ("0", 280832),
        ("110", 94370),
        ("11", 93804),
        ("100", 93374),
        ("1", 93351),
        ("10", 31559),
        ("101", 31295),
    ];
    let sample_median = (4, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.5039770000000354),
        standard_deviation: NiceFloat(2.8721055747415143),
        skewness: NiceFloat(-0.0024668908296986798),
        excess_kurtosis: NiceFloat(-1.6474519863189017),
    };
    striped_random_unsigned_inclusive_range_helper::<u8>(
        0,
        7,
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // u8, 1, 6, m = 4
    let values = &[
        "1", "1", "1", "110", "1", "110", "10", "11", "11", "100", "100", "110", "1", "1", "110",
        "1", "1", "11", "1", "1",
    ];
    let common_values = &[
        ("110", 375034),
        ("1", 374765),
        ("100", 93832),
        ("11", 93634),
        ("101", 31588),
        ("10", 31147),
    ];
    let sample_median = (4, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.5014329999999694),
        standard_deviation: NiceFloat(2.207774177939804),
        skewness: NiceFloat(-0.0014160236801414635),
        excess_kurtosis: NiceFloat(-1.753353488243057),
    };
    striped_random_unsigned_inclusive_range_helper::<u8>(
        1,
        6,
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // u16, 10000, 20000, m = 4
    let values = &[
        "10011100011111",
        "11010011001111",
        "10011100011100",
        "100111000011111",
        "11111100000000",
        "10011100111100",
        "10011110110000",
        "100000000000111",
        "11111111111111",
        "10101111001111",
        "100111000000000",
        "11000000000111",
        "11111111000000",
        "100000000001111",
        "100000000101100",
        "100011110000000",
        "100100011110010",
        "100011000000111",
        "100110111111111",
        "11111111110100",
    ];
    let common_values = &[
        ("100111000100000", 70337),
        ("100111000000000", 16785),
        ("10011111111111", 12552),
        ("10011100010000", 11029),
        ("10011100011111", 11017),
        ("100000000000000", 10528),
        ("11111111111111", 8012),
        ("100111000000111", 5466),
        ("100111000000001", 5462),
        ("100111000001111", 5448),
    ];
    let sample_median = (16383, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(15546.241523000184),
        standard_deviation: NiceFloat(3460.441923849274),
        skewness: NiceFloat(-0.39481033379336405),
        excess_kurtosis: NiceFloat(-1.1600583805474893),
    };
    striped_random_unsigned_inclusive_range_helper::<u16>(
        10000,
        20000,
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

fn striped_random_unsigned_inclusive_range_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(striped_random_unsigned_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::TWO,
        T::ONE,
        5,
        1,
    ));
    assert_panic!(striped_random_unsigned_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::ONE,
        T::TWO,
        1,
        1,
    ));
}

#[test]
fn striped_random_unsigned_inclusive_range_fail() {
    apply_fn_to_unsigneds!(striped_random_unsigned_inclusive_range_fail_helper);
}
