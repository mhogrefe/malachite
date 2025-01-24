// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::geometric::geometric_random_unsigned_inclusive_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    truncated_geometric_dist_assertions, CheckedToF64, MomentStats,
};
use std::panic::catch_unwind;

fn geometric_random_unsigned_inclusive_range_helper<T: CheckedToF64 + PrimitiveUnsigned>(
    a: T,
    b: T,
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
        geometric_random_unsigned_inclusive_range::<T>(
            EXAMPLE_SEED,
            a,
            b,
            um_numerator,
            um_denominator,
        ),
        a,
        b,
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
fn test_geometric_random_unsigned_inclusive_range() {
    // u8, 5, 5, um = 10 (um is irrelevant here)
    let values = &[5; 20];
    let common_values = &[(5, 1000000)];
    let pop_median = (5, None);
    let sample_median = (5, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(5.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(5.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    geometric_random_unsigned_inclusive_range_helper::<u8>(
        5,
        5,
        10,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u16, 1, 6, um = 3
    let values = &[2, 5, 2, 3, 4, 2, 5, 6, 1, 2, 4, 3, 3, 2, 4, 2, 5, 1, 5, 4];
    let common_values =
        &[(1, 365286), (2, 243368), (3, 162008), (4, 108422), (5, 72522), (6, 48394)];
    let pop_median = (2, None);
    let sample_median = (2, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(2.4225563909774435),
        standard_deviation: NiceFloat(1.4838791137635392),
        skewness: NiceFloat(0.8646161570662343),
        excess_kurtosis: NiceFloat(-0.2650814667635877),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.4247080000000336),
        standard_deviation: NiceFloat(1.4858025852532333),
        skewness: NiceFloat(0.8627980859656847),
        excess_kurtosis: NiceFloat(-0.27185507049284263),
    };
    geometric_random_unsigned_inclusive_range_helper::<u16>(
        1,
        6,
        3,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u32, 10, 19, um = 30
    let values = &[18, 13, 12, 14, 11, 14, 16, 12, 12, 12, 11, 11, 12, 16, 10, 12, 13, 17, 19, 18];
    let common_values = &[
        (10, 123089),
        (11, 117792),
        (12, 111339),
        (13, 106927),
        (14, 102009),
        (15, 96319),
        (16, 92170),
        (17, 86830),
        (18, 83895),
        (19, 79630),
    ];
    let pop_median = (14, None);
    let sample_median = (14, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(14.099085006908673),
        standard_deviation: NiceFloat(2.8551273967321666),
        skewness: NiceFloat(0.17141556508548922),
        excess_kurtosis: NiceFloat(-1.1843121480092598),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(14.099542000000508),
        standard_deviation: NiceFloat(2.8550785526337443),
        skewness: NiceFloat(0.17230969046968866),
        excess_kurtosis: NiceFloat(-1.183362703825652),
    };
    geometric_random_unsigned_inclusive_range_helper::<u32>(
        10,
        19,
        30,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u8, 0, u8::MAX, um = 1
    let values = &[1, 0, 0, 3, 4, 4, 1, 0, 0, 1, 0, 0, 2, 2, 0, 0, 1, 0, 3, 1];
    let common_values = &[
        (0, 500085),
        (1, 249510),
        (2, 125328),
        (3, 62428),
        (4, 31280),
        (5, 15676),
        (6, 7853),
        (7, 3994),
        (8, 1932),
        (9, 942),
    ];
    let pop_median = (0, Some(1));
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1.0),
        standard_deviation: NiceFloat(std::f64::consts::SQRT_2),
        skewness: NiceFloat(2.1213203435596424),
        excess_kurtosis: NiceFloat(6.5),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.0006159999999573),
        standard_deviation: NiceFloat(1.414547850547892),
        skewness: NiceFloat(2.1140569440125403),
        excess_kurtosis: NiceFloat(6.4341815215340805),
    };
    geometric_random_unsigned_inclusive_range_helper::<u8>(
        0,
        u8::MAX,
        1,
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}

fn geometric_random_unsigned_inclusive_range_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(geometric_random_unsigned_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::TWO,
        T::ONE,
        1,
        1
    ));
    assert_panic!(geometric_random_unsigned_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::ZERO,
        T::exact_from(10),
        1,
        0
    ));
    assert_panic!(geometric_random_unsigned_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::ONE,
        T::exact_from(10),
        9,
        10
    ));
    assert_panic!(geometric_random_unsigned_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::ZERO,
        T::exact_from(10),
        u64::MAX,
        u64::MAX - 1
    ));
}

#[test]
fn geometric_random_unsigned_inclusive_range_fail() {
    apply_fn_to_unsigneds!(geometric_random_unsigned_inclusive_range_fail_helper);
}
