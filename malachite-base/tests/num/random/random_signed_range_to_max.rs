use malachite_base_test_util::num::float::nice_float::NiceFloat;
use malachite_base_test_util::stats::moments::{
    uniform_primitive_integer_assertions, CheckedToF64, MomentStats,
};

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::random::random_signed_range_to_max;
use malachite_base::random::EXAMPLE_SEED;

fn random_signed_range_to_max_helper<T: CheckedToF64 + PrimitiveSigned>(
    a: T,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    uniform_primitive_integer_assertions(
        random_signed_range_to_max::<T>(EXAMPLE_SEED, a),
        a,
        T::MAX,
        expected_values,
        expected_common_values,
        expected_pop_median,
        expected_sample_median,
        expected_pop_moment_stats,
        expected_sample_moment_stats,
    );
}

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_random_signed_range_to_max() {
    // i8, 1
    let values = &[
        114, 95, 24, 99, 71, 93, 53, 85, 34, 48, 2, 114, 55, 11, 48, 18, 90, 93, 120, 67,
    ];
    let common_values = &[
        (3, 8154),
        (122, 8094),
        (49, 8062),
        (114, 8024),
        (79, 8003),
        (9, 7995),
        (92, 7995),
        (78, 7992),
        (51, 7989),
        (103, 7989),
    ];
    let pop_median = (64, None);
    let sample_median = (64, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(64.0),
        standard_deviation: NiceFloat(36.66060555964672),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2001488095238095),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(63.98849200000001),
        standard_deviation: NiceFloat(36.67596052855375),
        skewness: NiceFloat(-0.0006885262684998921),
        excess_kurtosis: NiceFloat(-1.199859926939298),
    };
    random_signed_range_to_max_helper::<i8>(
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, 10
    let values = &[
        123, 104, 33, 108, 80, 102, 62, 94, 43, 57, 11, 123, 64, 20, 57, 27, 99, 102, 76, 102,
    ];
    let common_values = &[
        (12, 8748),
        (58, 8680),
        (123, 8652),
        (101, 8640),
        (120, 8627),
        (87, 8624),
        (63, 8617),
        (51, 8612),
        (60, 8607),
        (103, 8603),
    ];
    let pop_median = (68, Some(69));
    let sample_median = (68, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(68.5),
        standard_deviation: NiceFloat(34.0624426605022),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2001723766429648),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(68.48982500000116),
        standard_deviation: NiceFloat(34.071363625884906),
        skewness: NiceFloat(-0.0006090540281705124),
        excess_kurtosis: NiceFloat(-1.2000002003229961),
    };
    random_signed_range_to_max_helper::<i8>(
        10,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i16, i16::MIN + 1
    let values = &[
        28530, -5050, 21221, 8617, -24488, -11409, -23875, 28250, 18526, -3178, 5566, 18906, 4726,
        -14304, 10648, -24145, -23032, 4178, -183, 114,
    ];
    let common_values = &[
        (-5320, 34),
        (17746, 33),
        (31871, 33),
        (-26931, 33),
        (-27161, 33),
        (7396, 32),
        (-5390, 32),
        (21597, 32),
        (25455, 32),
        (32572, 32),
    ];
    let pop_median = (0, None);
    let sample_median = (-4, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(18918.32494346861),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2000000005588105),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.9824120000000629),
        standard_deviation: NiceFloat(18919.548102352495),
        skewness: NiceFloat(0.00004707186894224452),
        excess_kurtosis: NiceFloat(-1.2002455454624814),
    };
    random_signed_range_to_max_helper::<i16>(
        i16::MIN + 1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}

macro_rules! random_signed_range_to_max_fail {
    ($t:ident, $random_signed_range_to_max_fail:ident) => {
        #[test]
        #[should_panic]
        fn $random_signed_range_to_max_fail() {
            random_signed_range_to_max::<$t>(EXAMPLE_SEED, $t::MIN);
        }
    };
}
random_signed_range_to_max_fail!(i8, random_signed_range_to_max_u8_fail);
random_signed_range_to_max_fail!(i16, random_signed_range_to_max_u16_fail);
random_signed_range_to_max_fail!(i32, random_signed_range_to_max_u32_fail);
random_signed_range_to_max_fail!(i64, random_signed_range_to_max_u64_fail);
random_signed_range_to_max_fail!(i128, random_signed_range_to_max_u128_fail);
random_signed_range_to_max_fail!(isize, random_signed_range_to_max_usize_fail);
