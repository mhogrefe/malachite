// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::striped::striped_random_unsigned_bit_chunks;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToBinaryString;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_base::test_util::stats::median;
use malachite_base::test_util::stats::moments::{moment_stats, CheckedToF64, MomentStats};
use std::panic::catch_unwind;

fn striped_random_unsigned_bit_chunks_helper<T: CheckedToF64 + PrimitiveUnsigned>(
    chunk_size: u64,
    m_numerator: u64,
    m_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (T, Option<T>),
    expected_sample_moment_stats: MomentStats,
) {
    let xs = striped_random_unsigned_bit_chunks::<T>(
        EXAMPLE_SEED,
        chunk_size,
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
fn test_striped_random_unsigned_bit_chunks() {
    // u8, chunk_size = 0, m = 4
    let values = &["0"; 20];
    let common_values = &[("0", 1000000)];
    let sample_median = (0, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_unsigned_bit_chunks_helper::<u8>(
        0,
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // u8, chunk_size = 3, m = 4
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
    striped_random_unsigned_bit_chunks_helper::<u8>(
        3,
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // chunk_size = 1, u8, m = 2
    let values = &[
        "0", "0", "0", "1", "0", "1", "0", "0", "0", "1", "1", "1", "0", "0", "1", "0", "0", "0",
        "0", "0",
    ];
    let common_values = &[("1", 500454), ("0", 499546)];
    let sample_median = (1, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.500454000000004),
        standard_deviation: NiceFloat(0.5000000438840368),
        skewness: NiceFloat(-0.0018160007486136712),
        excess_kurtosis: NiceFloat(-1.9999967021412284),
    };
    striped_random_unsigned_bit_chunks_helper::<u8>(
        1,
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // chunk_size = 6, u8, m = 5/4
    let values = &[
        "10100", "10110", "10100", "100110", "10010", "101101", "10101", "10001", "10101",
        "101001", "101101", "101010", "11011", "1101", "100001", "11110", "111", "101", "10110",
        "10010",
    ];
    let common_values = &[
        ("101010", 164180),
        ("10101", 164004),
        ("101011", 41169),
        ("101101", 41127),
        ("10010", 41114),
        ("100101", 40919),
        ("1010", 40893),
        ("110101", 40851),
        ("10110", 40844),
        ("101001", 40812),
    ];
    let sample_median = (32, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(31.51096499999948),
        standard_deviation: NiceFloat(13.555827954398234),
        skewness: NiceFloat(-0.0016679039370573313),
        excess_kurtosis: NiceFloat(-1.0970963271830174),
    };
    striped_random_unsigned_bit_chunks_helper::<u8>(
        6,
        5,
        4,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // chunk_size = 45, u64, m = 32
    let values = &[
        "11111111111",
        "111111111111111111111",
        "0",
        "111111111111111111111111111000000000000000000",
        "111111111",
        "111111111111111111111111111111111100000000000",
        "0",
        "1111111111111111111111111111111111",
        "1100000000111111111111111111111111111111",
        "111111111111111111100000000000000011111111111",
        "111111111111111111111111111111111111111100000",
        "111111111111111111111111111111111111111111111",
        "11111111111111111111111111",
        "1111111",
        "111111111111111110000000000000000000000000000",
        "11111111111111",
        "0",
        "11111111111111111111111111111",
        "11111111111111111111111111111111111111111111",
        "1000000011111111111111111111111110000000",
    ];
    let common_values = &[
        ("0", 123843),
        ("111111111111111111111111111111111111111111111", 123659),
        ("111111111111111100000000000000000000000000000", 4110),
        ("111111110000000000000000000000000000000000000", 4093),
        ("111111111111111111111111111111111", 4092),
        ("1111111111111111111111111111111111", 4081),
        ("11111111111111111111111111111111111111", 4078),
        ("1111111", 4072),
        ("111111111111111111111000000000000000000000000", 4057),
        ("111111111111111111111111111111111111111110000", 4051),
    ];
    let sample_median = (17592186044416, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(17607908125347.086),
        standard_deviation: NiceFloat(16886338134364.322),
        skewness: NiceFloat(-0.0018748788166730555),
        excess_kurtosis: NiceFloat(-1.9429442153094645),
    };
    striped_random_unsigned_bit_chunks_helper::<u64>(
        45,
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // chunk_size = 4, u64, m = 2
    let values = &[
        "11", "110", "11", "1100", "101", "1010", "110", "100", "101", "1101", "1111", "1100",
        "111", "11", "1101", "100", "1", "10", "0", "0",
    ];
    let common_values = &[
        ("1110", 62873),
        ("1010", 62820),
        ("1000", 62739),
        ("100", 62694),
        ("1001", 62689),
        ("110", 62619),
        ("1011", 62601),
        ("11", 62507),
        ("1100", 62467),
        ("101", 62457),
    ];
    let sample_median = (8, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.5009610000000775),
        standard_deviation: NiceFloat(4.606733799393635),
        skewness: NiceFloat(-0.0007329768450960305),
        excess_kurtosis: NiceFloat(-1.20818908668768),
    };
    striped_random_unsigned_bit_chunks_helper::<u64>(
        4,
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // chunk_size = 10, u64, m = 33/32
    let values = &[
        "101010101",
        "101010101",
        "101011010",
        "1010101010",
        "101010101",
        "1010101010",
        "101010101",
        "101010101",
        "101010001",
        "1010101001",
        "1010110101",
        "1010101010",
        "101101010",
        "101101010",
        "1010101101",
        "101011010",
        "101010101",
        "101010010",
        "101010101",
        "101010101",
    ];
    let common_values = &[
        ("1010101010", 379066),
        ("101010101", 378152),
        ("1010110101", 12071),
        ("101011010", 12069),
        ("101001010", 12042),
        ("1011010101", 11977),
        ("101010110", 11960),
        ("1001010101", 11941),
        ("1010010101", 11934),
        ("100101010", 11903),
    ];
    let sample_median = (565, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(511.639053000003),
        standard_deviation: NiceFloat(177.48758065877436),
        skewness: NiceFloat(-0.0017256448985827214),
        excess_kurtosis: NiceFloat(-1.734924505103183),
    };
    striped_random_unsigned_bit_chunks_helper::<u64>(
        10,
        33,
        32,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

fn striped_random_unsigned_bit_chunks_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(striped_random_unsigned_bit_chunks::<T>(
        EXAMPLE_SEED,
        4,
        1,
        0
    ));
    assert_panic!(striped_random_unsigned_bit_chunks::<T>(
        EXAMPLE_SEED,
        4,
        2,
        3
    ));
    assert_panic!(striped_random_unsigned_bit_chunks::<T>(
        EXAMPLE_SEED,
        200,
        4,
        1
    ));
}

#[test]
fn striped_random_unsigned_bit_chunks_fail() {
    apply_fn_to_unsigneds!(striped_random_unsigned_bit_chunks_fail_helper);
}
