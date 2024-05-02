// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_unsigneds_less_than;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    uniform_primitive_int_assertions, CheckedToF64, MomentStats,
};
use std::panic::catch_unwind;

fn random_unsigneds_less_than_helper<T: CheckedToF64 + PrimitiveUnsigned>(
    limit: T,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    uniform_primitive_int_assertions(
        random_unsigneds_less_than(EXAMPLE_SEED, limit),
        T::ZERO,
        limit.wrapping_sub(T::ONE),
        expected_values,
        expected_common_values,
        expected_pop_median,
        expected_sample_median,
        expected_pop_moment_stats,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_random_unsigneds_less_than() {
    // u8, limit = 1
    let values = &[0; 20];
    let common_values = &[(0, 1000000)];
    let pop_median = (0, None);
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_unsigneds_less_than_helper::<u8>(
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u16, limit = 2
    let values = &[1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0];
    let common_values = &[(1, 500473), (0, 499527)];
    let pop_median = (0, Some(1));
    let sample_median = (1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(0.5),
        standard_deviation: NiceFloat(0.5),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.9999999999999998),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.5004730000000077),
        standard_deviation: NiceFloat(0.5000000262710417),
        skewness: NiceFloat(-0.0018920008465908307),
        excess_kurtosis: NiceFloat(-1.999996420332894),
    };
    random_unsigneds_less_than_helper::<u16>(
        2,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u32, limit = 3
    let values = &[1, 0, 1, 2, 1, 1, 0, 1, 0, 2, 1, 0, 1, 2, 2, 0, 1, 0, 2, 2];
    let common_values = &[(1, 333784), (2, 333516), (0, 332700)];
    let pop_median = (1, None);
    let sample_median = (1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1.0),
        standard_deviation: NiceFloat(0.816496580927726),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.5),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.0008159999999369),
        standard_deviation: NiceFloat(0.8162205586482172),
        skewness: NiceFloat(-0.0014985805078073927),
        excess_kurtosis: NiceFloat(-1.498982317720307),
    };
    random_unsigneds_less_than_helper::<u32>(
        3,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u64, limit = 4
    let values = &[1, 0, 3, 1, 3, 3, 2, 3, 1, 1, 0, 1, 0, 3, 2, 1, 0, 1, 2, 3];
    let common_values = &[(1, 250314), (3, 250015), (2, 249955), (0, 249716)];
    let pop_median = (1, Some(2));
    let sample_median = (1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1.5),
        standard_deviation: NiceFloat(1.118033988749895),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.3599999999999999),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.5002690000000294),
        standard_deviation: NiceFloat(1.117793888470597),
        skewness: NiceFloat(-0.00003155128630032229),
        excess_kurtosis: NiceFloat(-1.3594490446207492),
    };
    random_unsigneds_less_than_helper::<u64>(
        4,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u128, limit = 10
    let values = &[1, 7, 5, 4, 6, 4, 2, 8, 1, 7, 5, 0, 2, 6, 3, 5, 2, 2, 9, 5];
    let common_values = &[
        (5, 100442),
        (2, 100144),
        (6, 100118),
        (8, 100047),
        (1, 100023),
        (4, 100011),
        (0, 99996),
        (9, 99936),
        (7, 99715),
        (3, 99568),
    ];
    let pop_median = (4, Some(5));
    let sample_median = (5, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(4.5),
        standard_deviation: NiceFloat(2.8722813232690143),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2242424242424241),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(4.499781999999947),
        standard_deviation: NiceFloat(2.8719356191409156),
        skewness: NiceFloat(-0.00016199543215745585),
        excess_kurtosis: NiceFloat(-1.2237431734377897),
    };
    random_unsigneds_less_than_helper::<u128>(
        10,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}

fn random_unsigneds_less_than_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(random_unsigneds_less_than::<T>(EXAMPLE_SEED, T::ZERO));
}

#[test]
fn random_unsigneds_less_than_fail() {
    apply_fn_to_unsigneds!(random_unsigneds_less_than_fail_helper);
}
