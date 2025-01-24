// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::{assert_equal, Itertools};
use malachite_base::bools::random::random_bools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_unsigned_bit_chunks;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    uniform_primitive_int_assertions, CheckedToF64, MomentStats,
};
use std::panic::catch_unwind;

fn random_unsigned_bit_chunks_helper<T: CheckedToF64 + PrimitiveUnsigned>(
    chunk_size: u64,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    let xs = random_unsigned_bit_chunks(EXAMPLE_SEED, chunk_size);
    uniform_primitive_int_assertions(
        xs.clone(),
        T::ZERO,
        T::low_mask(chunk_size),
        expected_values,
        expected_common_values,
        expected_pop_median,
        expected_sample_median,
        expected_pop_moment_stats,
        expected_sample_moment_stats,
    );
    if chunk_size != 0 {
        assert_equal(
            random_bools(EXAMPLE_SEED)
                .chunks(usize::exact_from(chunk_size))
                .into_iter()
                .take(1000000)
                .map(T::from_bits_asc),
            xs.take(1000000),
        );
    }
}

#[test]
fn test_random_unsigned_bit_chunks() {
    // u16, chunk_size = 1
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
    random_unsigned_bit_chunks_helper::<u16>(
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u32, chunk_size = 2
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
    random_unsigned_bit_chunks_helper::<u32>(
        2,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u64, chunk_size = 3
    let values = &[1, 6, 5, 7, 6, 3, 1, 2, 4, 5, 1, 2, 6, 5, 4, 6, 0, 5, 6, 0];
    let common_values = &[
        (3, 125437),
        (6, 125372),
        (7, 125322),
        (4, 125116),
        (0, 125049),
        (5, 124759),
        (2, 124542),
        (1, 124403),
    ];
    let pop_median = (3, Some(4));
    let sample_median = (4, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(3.5),
        standard_deviation: NiceFloat(2.29128784747792),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.238095238095238),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.503543000000029),
        standard_deviation: NiceFloat(2.291658067611209),
        skewness: NiceFloat(-0.002350008016990765),
        excess_kurtosis: NiceFloat(-1.2376569368178467),
    };
    random_unsigned_bit_chunks_helper::<u64>(
        3,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u128, chunk_size = 7
    let values =
        &[113, 94, 23, 98, 70, 92, 52, 84, 33, 47, 1, 113, 54, 10, 47, 17, 89, 92, 119, 66];
    let common_values = &[
        (2, 8077),
        (121, 8039),
        (48, 8015),
        (113, 7966),
        (8, 7937),
        (77, 7933),
        (50, 7928),
        (91, 7927),
        (82, 7925),
        (102, 7924),
    ];
    let pop_median = (63, Some(64));
    let sample_median = (63, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(63.5),
        standard_deviation: NiceFloat(36.94928957368463),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2001464933162425),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(63.478088999999315),
        standard_deviation: NiceFloat(36.96113842989552),
        skewness: NiceFloat(-0.000454036075457304),
        excess_kurtosis: NiceFloat(-1.1998683031732713),
    };
    random_unsigned_bit_chunks_helper::<u128>(
        7,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u8, chunk_size = 8
    let values = &[
        113, 239, 69, 108, 228, 210, 168, 161, 87, 32, 110, 83, 188, 34, 89, 238, 93, 200, 149, 115,
    ];
    let common_values = &[
        (214, 4097),
        (86, 4078),
        (166, 4049),
        (22, 4048),
        (126, 4047),
        (55, 4040),
        (93, 4037),
        (191, 4036),
        (36, 4035),
        (42, 4032),
    ];
    let pop_median = (127, Some(128));
    let sample_median = (127, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(127.5),
        standard_deviation: NiceFloat(73.90027063549903),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.200036621652552),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(127.4588370000015),
        standard_deviation: NiceFloat(73.908735397844),
        skewness: NiceFloat(0.0004407839380447086),
        excess_kurtosis: NiceFloat(-1.200418003526934),
    };
    random_unsigned_bit_chunks_helper::<u8>(
        8,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}

fn random_unsigned_bit_chunks_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(random_unsigned_bit_chunks::<T>(EXAMPLE_SEED, 0));
    assert_panic!(random_unsigned_bit_chunks::<T>(EXAMPLE_SEED, T::WIDTH + 1));
}

#[test]
fn random_unsigned_bit_chunks_fail() {
    apply_fn_to_unsigneds!(random_unsigned_bit_chunks_fail_helper);
}
