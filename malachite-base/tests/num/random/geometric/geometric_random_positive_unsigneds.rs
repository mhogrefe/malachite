// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::geometric::geometric_random_positive_unsigneds;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    truncated_geometric_dist_assertions, CheckedToF64, MomentStats,
};
use std::panic::catch_unwind;

fn geometric_random_positive_unsigneds_helper<T: CheckedToF64 + PrimitiveUnsigned>(
    um_numerator: u64,
    um_denominator: u64,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    truncated_geometric_dist_assertions(
        geometric_random_positive_unsigneds::<T>(EXAMPLE_SEED, um_numerator, um_denominator),
        T::ONE,
        T::MAX,
        um_numerator,
        um_denominator,
        expected_values,
        expected_common_values,
        expected_pop_median,
        expected_sample_median,
        expected_pop_moment_stats,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_geometric_random_positive_unsigneds() {
    // u64, um = 65 / 64
    let values = &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2];
    let common_values = &[(1, 984537), (2, 15210), (3, 253)];
    let pop_median = (1, None);
    let sample_median = (1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1.015625),
        standard_deviation: NiceFloat(0.12597277731716458),
        skewness: NiceFloat(8.186292482887549),
        excess_kurtosis: NiceFloat(69.01538461538773),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.0157160000000027),
        standard_deviation: NiceFloat(0.12639233884624257),
        skewness: NiceFloat(8.160460378454992),
        excess_kurtosis: NiceFloat(68.31033619043166),
    };
    geometric_random_positive_unsigneds_helper::<u64>(
        65,
        64,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u64, um = 12345/10000
    let values = &[1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1];
    let common_values = &[
        (1, 809395),
        (2, 154707),
        (3, 29037),
        (4, 5577),
        (5, 1053),
        (6, 185),
        (7, 40),
        (8, 5),
        (9, 1),
    ];
    let pop_median = (1, None);
    let sample_median = (1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1.2345),
        standard_deviation: NiceFloat(0.538042981554448),
        skewness: NiceFloat(2.730265146765687),
        excess_kurtosis: NiceFloat(9.45434777164341),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.2349320000000084),
        standard_deviation: NiceFloat(0.5376590410783139),
        skewness: NiceFloat(2.716807176148366),
        excess_kurtosis: NiceFloat(9.308727522629948),
    };
    geometric_random_positive_unsigneds_helper::<u64>(
        12345,
        10000,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u64, um = 2
    let values = &[2, 1, 1, 4, 5, 5, 2, 1, 1, 2, 1, 1, 3, 3, 1, 1, 2, 1, 4, 2];
    let common_values = &[
        (1, 500085),
        (2, 249510),
        (3, 125328),
        (4, 62428),
        (5, 31280),
        (6, 15676),
        (7, 7853),
        (8, 3994),
        (9, 1932),
        (10, 942),
    ];
    let pop_median = (1, Some(2));
    let sample_median = (1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(2.0),
        standard_deviation: NiceFloat(std::f64::consts::SQRT_2),
        skewness: NiceFloat(2.1213203435596424),
        excess_kurtosis: NiceFloat(6.5),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.0006159999998947),
        standard_deviation: NiceFloat(1.414547850547892),
        skewness: NiceFloat(2.114056944012543),
        excess_kurtosis: NiceFloat(6.4341815215340645),
    };
    geometric_random_positive_unsigneds_helper::<u64>(
        2,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u8, um = 64
    let values = &[53, 56, 71, 20, 113, 87, 55, 44, 4, 17, 44, 36, 5, 25, 55, 9, 114, 24, 15, 76];
    let common_values = &[
        (1, 16172),
        (2, 15695),
        (3, 15520),
        (4, 15241),
        (5, 15089),
        (6, 14676),
        (7, 14543),
        (8, 14211),
        (9, 13961),
        (10, 13836),
    ];
    let pop_median = (43, None);
    let sample_median = (43, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(59.318470051671945),
        standard_deviation: NiceFloat(53.06875861106282),
        skewness: NiceFloat(1.275790253790931),
        excess_kurtosis: NiceFloat(1.21378930742857),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(59.312081000000006),
        standard_deviation: NiceFloat(53.01872774251563),
        skewness: NiceFloat(1.2706951236419683),
        excess_kurtosis: NiceFloat(1.1982916278084028),
    };
    geometric_random_positive_unsigneds_helper::<u8>(
        64,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u8, um = 1000
    let values =
        &[27, 115, 1, 27, 107, 217, 210, 242, 42, 36, 197, 207, 42, 43, 250, 195, 255, 122, 5, 221];
    let common_values = &[
        (6, 4501),
        (2, 4490),
        (8, 4461),
        (21, 4453),
        (10, 4449),
        (3, 4446),
        (11, 4440),
        (13, 4422),
        (12, 4415),
        (25, 4407),
    ];
    let pop_median = (120, None);
    let sample_median = (120, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(122.58449448192029),
        standard_deviation: NiceFloat(73.49201840569208),
        skewness: NiceFloat(0.08835569448221604),
        excess_kurtosis: NiceFloat(-1.1892531716440173),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(122.63144799999775),
        standard_deviation: NiceFloat(73.49633071884523),
        skewness: NiceFloat(0.08793356388157667),
        excess_kurtosis: NiceFloat(-1.1900100973541539),
    };
    geometric_random_positive_unsigneds_helper::<u8>(
        1000,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}

fn geometric_random_positive_unsigneds_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(geometric_random_positive_unsigneds::<T>(EXAMPLE_SEED, 1, 0));
    assert_panic!(geometric_random_positive_unsigneds::<T>(EXAMPLE_SEED, 2, 3));
}

#[test]
fn geometric_random_positive_unsigneds_fail() {
    apply_fn_to_unsigneds!(geometric_random_positive_unsigneds_fail_helper);
}
