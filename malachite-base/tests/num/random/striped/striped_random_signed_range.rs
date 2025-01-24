// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::striped::striped_random_signed_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToBinaryString;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_base::test_util::stats::median;
use malachite_base::test_util::stats::moments::{moment_stats, CheckedToF64, MomentStats};
use std::panic::catch_unwind;

fn striped_random_signed_range_helper<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: CheckedToF64 + PrimitiveSigned + WrappingFrom<U>,
>(
    a: S,
    b: S,
    m_numerator: u64,
    m_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (S, Option<S>),
    expected_sample_moment_stats: MomentStats,
) {
    let xs = striped_random_signed_range::<U, S>(EXAMPLE_SEED, a, b, m_numerator, m_denominator);
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
fn test_striped_random_signed_range() {
    // i16, 50, 201, m = 4
    let values = &[
        "110010", "11000000", "10100110", "110111", "111111", "11001000", "1111000", "111111",
        "110010", "1111111", "111110", "11000001", "111101", "110111", "11001000", "110011",
        "111111", "11000011", "11000000", "11000000",
    ];
    let common_values = &[
        ("11001000", 187162),
        ("11000000", 79165),
        ("111111", 78685),
        ("110010", 70343),
        ("110011", 70101),
        ("110111", 26718),
        ("111100", 26467),
        ("111110", 26443),
        ("111000", 26309),
        ("11000001", 26222),
    ];
    let sample_median = (128, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(126.78841600000145),
        standard_deviation: NiceFloat(62.97286483710745),
        skewness: NiceFloat(-0.01241704115437865),
        excess_kurtosis: NiceFloat(-1.7551228249647437),
    };
    striped_random_signed_range_helper::<u16, i16>(
        50,
        201,
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // i16, -200, -49, m = 4
    let values = &[
        "1111111111001110",
        "1111111101000000",
        "1111111101011010",
        "1111111111001001",
        "1111111111000001",
        "1111111100111000",
        "1111111110001000",
        "1111111111000001",
        "1111111111001110",
        "1111111110000001",
        "1111111111000010",
        "1111111100111111",
        "1111111111000011",
        "1111111111001001",
        "1111111100111000",
        "1111111111001101",
        "1111111111000001",
        "1111111100111101",
        "1111111101000000",
        "1111111101000000",
    ];
    let common_values = &[
        ("1111111100111000", 187162),
        ("1111111101000000", 79165),
        ("1111111111000001", 78685),
        ("1111111111001110", 70343),
        ("1111111111001101", 70101),
        ("1111111111001001", 26718),
        ("1111111111000100", 26467),
        ("1111111111000010", 26443),
        ("1111111111001000", 26309),
        ("1111111100111111", 26222),
    ];
    let sample_median = (-128, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-126.78841600000145),
        standard_deviation: NiceFloat(62.97286483710745),
        skewness: NiceFloat(0.01241704115437865),
        excess_kurtosis: NiceFloat(-1.7551228249647437),
    };
    striped_random_signed_range_helper::<u16, i16>(
        -200,
        -49,
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // i16, -50, 201, m = 4
    let values = &[
        "1111111111001110",
        "1111111111001111",
        "11000111",
        "11111",
        "1111111111111111",
        "11000011",
        "1000000",
        "11000001",
        "1111111",
        "11000011",
        "11000100",
        "1111111111111111",
        "11001000",
        "1111111111010010",
        "1111111111110100",
        "1111111111011111",
        "1111111111111111",
        "1111111111001111",
        "1111111111111111",
        "1111111111010000",
    ];
    let common_values = &[
        ("1111111111001110", 93920),
        ("11001000", 93547),
        ("1111111111111111", 79262),
        ("1111111111010000", 70793),
        ("11000000", 39521),
        ("0", 33351),
        ("1111111111001111", 23264),
        ("1111111111111001", 19879),
        ("1111111111100000", 19842),
        ("1111111111110001", 19780),
    ];
    let sample_median = (-1, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(42.632988000001404),
        standard_deviation: NiceFloat(91.45288785939469),
        skewness: NiceFloat(0.7466366203634324),
        excess_kurtosis: NiceFloat(-1.0394231672402725),
    };
    striped_random_signed_range_helper::<u16, i16>(
        -50,
        201,
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

fn striped_random_signed_range_fail_helper<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() {
    assert_panic!(striped_random_signed_range::<U, S>(
        EXAMPLE_SEED,
        S::TWO,
        S::TWO,
        5,
        1,
    ));
    assert_panic!(striped_random_signed_range::<U, S>(
        EXAMPLE_SEED,
        S::NEGATIVE_ONE,
        S::NEGATIVE_ONE,
        5,
        1,
    ));
    assert_panic!(striped_random_signed_range::<U, S>(
        EXAMPLE_SEED,
        S::ONE,
        S::TWO,
        1,
        1,
    ));
}

#[test]
fn striped_random_signed_range_fail() {
    apply_fn_to_unsigned_signed_pairs!(striped_random_signed_range_fail_helper);
}
