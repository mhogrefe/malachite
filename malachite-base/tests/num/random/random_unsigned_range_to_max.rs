use malachite_base_test_util::num::float::nice_float::NiceFloat;
use malachite_base_test_util::stats::moments::{
    uniform_primitive_integer_assertions, CheckedToF64, MomentStats,
};

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::random_unsigned_range_to_max;
use malachite_base::random::EXAMPLE_SEED;

fn random_unsigned_range_to_max_helper<T: CheckedToF64 + PrimitiveUnsigned>(
    a: T,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    uniform_primitive_integer_assertions(
        random_unsigned_range_to_max::<T>(EXAMPLE_SEED, a),
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
fn test_random_unsigned_range_to_max() {
    // u8, 1
    let values = &[
        114, 240, 70, 109, 229, 211, 169, 162, 88, 33, 111, 84, 189, 35, 90, 239, 94, 201, 150, 116,
    ];
    let common_values = &[
        (215, 4112),
        (87, 4092),
        (167, 4063),
        (23, 4061),
        (127, 4061),
        (56, 4054),
        (94, 4054),
        (192, 4052),
        (37, 4049),
        (43, 4047),
    ];
    let pop_median = (128, None);
    let sample_median = (128, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(128.0),
        standard_deviation: NiceFloat(73.6115932898254),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2000369094488188),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(127.9518310000005),
        standard_deviation: NiceFloat(73.61930013623305),
        skewness: NiceFloat(0.0005380412105164573),
        excess_kurtosis: NiceFloat(-1.2003531635142988),
    };
    random_unsigned_range_to_max_helper::<u8>(
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u8, 10
    let values = &[
        123, 249, 79, 118, 238, 220, 178, 171, 97, 42, 120, 93, 198, 44, 99, 248, 103, 210, 159,
        125,
    ];
    let common_values = &[
        (224, 4247),
        (136, 4230),
        (176, 4226),
        (96, 4222),
        (201, 4203),
        (32, 4200),
        (52, 4199),
        (197, 4199),
        (77, 4191),
        (26, 4189),
    ];
    let pop_median = (132, Some(133));
    let sample_median = (133, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(132.5),
        standard_deviation: NiceFloat(71.01349636982161),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2000396595885316),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(132.47981299999978),
        standard_deviation: NiceFloat(71.04259887937542),
        skewness: NiceFloat(0.0002550704757036722),
        excess_kurtosis: NiceFloat(-1.2006769540452082),
    };
    random_unsigned_range_to_max_helper::<u8>(
        10,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u16, u16::MAX - 10
    let values = &[
        65526, 65532, 65530, 65529, 65531, 65529, 65527, 65533, 65535, 65526, 65535, 65532, 65530,
        65525, 65527, 65531, 65528, 65530, 65527, 65527,
    ];
    let common_values = &[
        (65530, 91265),
        (65531, 91078),
        (65527, 91064),
        (65525, 91029),
        (65533, 90964),
        (65526, 90885),
        (65529, 90884),
        (65534, 90848),
        (65535, 90786),
        (65532, 90670),
    ];
    let pop_median = (65530, None);
    let sample_median = (65530, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(65530.0),
        standard_deviation: NiceFloat(3.1622776601683795),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.22),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(65529.998817001615),
        standard_deviation: NiceFloat(3.1619793798547056),
        skewness: NiceFloat(0.00004359716853275025),
        excess_kurtosis: NiceFloat(-1.2194914772569132),
    };
    random_unsigned_range_to_max_helper::<u16>(
        u16::MAX - 10,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u8, u8::MAX
    let values = &[255; 20];
    let common_values = &[(255, 1000000)];
    let pop_median = (255, None);
    let sample_median = (255, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(255.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(255.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_unsigned_range_to_max_helper::<u8>(
        u8::MAX,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}

macro_rules! random_unsigned_range_to_max_fail {
    ($t:ident, $random_unsigned_range_to_max_fail:ident) => {
        #[test]
        #[should_panic]
        fn $random_unsigned_range_to_max_fail() {
            random_unsigned_range_to_max::<$t>(EXAMPLE_SEED, 0);
        }
    };
}
random_unsigned_range_to_max_fail!(u8, random_unsigned_range_to_max_u8_fail);
random_unsigned_range_to_max_fail!(u16, random_unsigned_range_to_max_u16_fail);
random_unsigned_range_to_max_fail!(u32, random_unsigned_range_to_max_u32_fail);
random_unsigned_range_to_max_fail!(u64, random_unsigned_range_to_max_u64_fail);
random_unsigned_range_to_max_fail!(u128, random_unsigned_range_to_max_u128_fail);
random_unsigned_range_to_max_fail!(usize, random_unsigned_range_to_max_usize_fail);
