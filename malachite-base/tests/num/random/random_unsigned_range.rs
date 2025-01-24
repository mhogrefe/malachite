// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_unsigned_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    uniform_primitive_int_assertions, CheckedToF64, MomentStats,
};
use std::panic::catch_unwind;

fn random_unsigned_range_helper<T: CheckedToF64 + PrimitiveUnsigned>(
    a: T,
    b: T,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    uniform_primitive_int_assertions(
        random_unsigned_range::<T>(EXAMPLE_SEED, a, b),
        a,
        b - T::ONE,
        expected_values,
        expected_common_values,
        expected_pop_median,
        expected_sample_median,
        expected_pop_moment_stats,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_random_unsigned_range() {
    // u8, 5, 6
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
    random_unsigned_range_helper::<u8>(
        5,
        6,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u16, 1, 7
    let values = &[2, 6, 4, 2, 3, 5, 6, 2, 3, 6, 5, 1, 6, 1, 3, 6, 3, 1, 5, 1];
    let common_values =
        &[(4, 167408), (1, 167104), (5, 166935), (6, 166549), (3, 166068), (2, 165936)];
    let pop_median = (3, Some(4));
    let sample_median = (4, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(3.5),
        standard_deviation: NiceFloat(1.707825127659933),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2685714285714285),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.5007809999998676),
        standard_deviation: NiceFloat(1.7081165966354217),
        skewness: NiceFloat(-0.0024015945144963404),
        excess_kurtosis: NiceFloat(-1.2685533575767198),
    };
    random_unsigned_range_helper::<u16>(
        1,
        7,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u32, 10, 20
    let values = &[11, 17, 15, 14, 16, 14, 12, 18, 11, 17, 15, 10, 12, 16, 13, 15, 12, 12, 19, 15];
    let common_values = &[
        (15, 100442),
        (12, 100144),
        (16, 100118),
        (18, 100047),
        (11, 100023),
        (14, 100011),
        (10, 99996),
        (19, 99936),
        (17, 99715),
        (13, 99568),
    ];
    let pop_median = (14, Some(15));
    let sample_median = (15, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(14.5),
        standard_deviation: NiceFloat(2.8722813232690143),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2242424242424241),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(14.49978199999957),
        standard_deviation: NiceFloat(2.8719356191409076),
        skewness: NiceFloat(-0.00016199543215732692),
        excess_kurtosis: NiceFloat(-1.2237431734377722),
    };
    random_unsigned_range_helper::<u32>(
        10,
        20,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u8, 0, u8::MAX
    let values = &[
        113, 239, 69, 108, 228, 210, 168, 161, 87, 32, 110, 83, 188, 34, 89, 238, 93, 200, 149, 115,
    ];
    let common_values = &[
        (214, 4112),
        (86, 4092),
        (166, 4063),
        (22, 4061),
        (126, 4061),
        (55, 4054),
        (93, 4054),
        (191, 4052),
        (36, 4049),
        (42, 4047),
    ];
    let pop_median = (127, None);
    let sample_median = (127, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(127.0),
        standard_deviation: NiceFloat(73.6115932898254),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2000369094488188),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(126.95183099999818),
        standard_deviation: NiceFloat(73.61930013623306),
        skewness: NiceFloat(0.0005380412105163868),
        excess_kurtosis: NiceFloat(-1.200353163514298),
    };
    random_unsigned_range_helper::<u8>(
        0,
        u8::MAX,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}

fn random_unsigned_range_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(random_unsigned_range::<T>(EXAMPLE_SEED, T::TWO, T::TWO));
}

#[test]
fn random_unsigned_range_fail() {
    apply_fn_to_unsigneds!(random_unsigned_range_fail_helper);
}
