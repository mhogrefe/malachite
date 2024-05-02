// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_signed_inclusive_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    uniform_primitive_int_assertions, CheckedToF64, MomentStats,
};
use std::panic::catch_unwind;

fn random_signed_inclusive_range_helper<T: CheckedToF64 + PrimitiveSigned>(
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
        random_signed_inclusive_range::<T>(EXAMPLE_SEED, a, b),
        a,
        b,
        expected_values,
        expected_common_values,
        expected_pop_median,
        expected_sample_median,
        expected_pop_moment_stats,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_random_signed_inclusive_range() {
    // i8, 5, 5
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
    random_signed_inclusive_range_helper::<i8>(
        5,
        5,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i16, 1, 6
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
    random_signed_inclusive_range_helper::<i16>(
        1,
        6,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i32, 10, 19
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
    random_signed_inclusive_range_helper::<i32>(
        10,
        19,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i64, -20, -11
    let values = &[
        -19, -13, -15, -16, -14, -16, -18, -12, -19, -13, -15, -20, -18, -14, -17, -15, -18, -18,
        -11, -15,
    ];
    let common_values = &[
        (-15, 100442),
        (-18, 100144),
        (-14, 100118),
        (-12, 100047),
        (-19, 100023),
        (-16, 100011),
        (-20, 99996),
        (-11, 99936),
        (-13, 99715),
        (-17, 99568),
    ];
    let pop_median = (-16, Some(-15));
    let sample_median = (-15, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-15.5),
        standard_deviation: NiceFloat(2.8722813232690143),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2242424242424241),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-15.50021800000043),
        standard_deviation: NiceFloat(2.8719356191409076),
        skewness: NiceFloat(-0.00016199543215732692),
        excess_kurtosis: NiceFloat(-1.2237431734377722),
    };
    random_signed_inclusive_range_helper::<i64>(
        -20,
        -11,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, -100, 99
    let values =
        &[13, -31, 8, 68, 61, -13, -68, 10, -17, 88, -66, -11, -7, 49, 15, 89, 49, 17, 46, -69];
    let common_values = &[
        (-33, 5190),
        (66, 5182),
        (87, 5176),
        (-73, 5172),
        (-7, 5169),
        (91, 5167),
        (-84, 5136),
        (26, 5132),
        (-12, 5125),
        (-14, 5125),
    ];
    let pop_median = (-1, Some(0));
    let sample_median = (-1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.5),
        standard_deviation: NiceFloat(57.73430522661548),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2000600015000376),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.5228260000000045),
        standard_deviation: NiceFloat(57.74415076317745),
        skewness: NiceFloat(0.00018970091170297907),
        excess_kurtosis: NiceFloat(-1.2011004127418898),
    };
    random_signed_inclusive_range_helper::<i8>(
        -100,
        99,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, i8::MIN, i8::MAX
    let values = &[
        -15, 111, -59, -20, 100, 82, 40, 33, -41, -96, -18, -45, 60, -94, -39, 110, -35, 72, 21,
        -13,
    ];
    let common_values = &[
        (86, 4097),
        (-42, 4078),
        (38, 4049),
        (-106, 4048),
        (-2, 4047),
        (-73, 4040),
        (-35, 4037),
        (63, 4036),
        (-92, 4035),
        (-86, 4032),
    ];
    let pop_median = (-1, Some(0));
    let sample_median = (-1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.5),
        standard_deviation: NiceFloat(73.90027063549903),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.200036621652552),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.5411629999999936),
        standard_deviation: NiceFloat(73.90873539784404),
        skewness: NiceFloat(0.00044078393804460487),
        excess_kurtosis: NiceFloat(-1.200418003526936),
    };
    random_signed_inclusive_range_helper::<i8>(
        i8::MIN,
        i8::MAX,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}

fn random_signed_inclusive_range_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(random_signed_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::TWO,
        T::ONE
    ));
}

#[test]
fn random_signed_inclusive_range_fail() {
    apply_fn_to_signeds!(random_signed_inclusive_range_fail_helper);
}
