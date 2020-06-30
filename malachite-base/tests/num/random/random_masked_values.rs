use malachite_base_test_util::num::float::nice_float::NiceFloat;
use malachite_base_test_util::stats::moments::{
    disc_uniform_dist_assertions, CheckedToF64, MomentStats,
};

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::random::random_masked_values;
use malachite_base::random::{standard_random_values, EXAMPLE_SEED};

fn random_masked_values_helper<T: CheckedToF64 + PrimitiveInteger>(
    pow: u64,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: NiceFloat<f64>,
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    let min = if pow < T::WIDTH { T::ZERO } else { T::MIN };
    disc_uniform_dist_assertions(
        random_masked_values(standard_random_values::<T>(EXAMPLE_SEED), pow),
        &min,
        &T::low_mask(pow),
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
fn test_random_masked_values() {
    // u8, pow = 0
    let values = &[0; 20];
    let common_values = &[(0, 1_000_000)];
    let pop_median = NiceFloat(0.0);
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        stdev: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        kurtosis: NiceFloat(f64::NAN),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        stdev: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        kurtosis: NiceFloat(f64::NAN),
    };
    random_masked_values_helper::<u8>(
        0,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u8, pow = 1
    let values = &[1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0];
    let common_values = &[(0, 500187), (1, 499813)];
    let pop_median = NiceFloat(0.5);
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(0.5),
        stdev: NiceFloat(0.5),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-2.0),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.49981300000001017),
        stdev: NiceFloat(0.5000002150311751),
        skewness: NiceFloat(0.0007480000523136409),
        kurtosis: NiceFloat(-1.999999440496001),
    };
    random_masked_values_helper::<u8>(
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u8, pow = 2
    let values = &[1, 0, 3, 0, 1, 1, 1, 3, 3, 0, 1, 0, 2, 3, 0, 0, 2, 3, 3, 0];
    let common_values = &[(3, 250600), (0, 250417), (2, 249770), (1, 249213)];
    let pop_median = NiceFloat(1.5);
    let sample_median = (2, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1.5),
        stdev: NiceFloat(1.118033988749895),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.3599999999999999),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.5005529999999707),
        stdev: NiceFloat(1.1189436742865808),
        skewness: NiceFloat(-0.0009920908109427875),
        kurtosis: NiceFloat(-1.3620790865995875),
    };
    random_masked_values_helper::<u8>(
        2,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u8, pow = 3
    let values = &[1, 4, 7, 4, 5, 5, 5, 7, 7, 0, 1, 4, 2, 7, 0, 4, 2, 7, 3, 0];
    let common_values = &[
        (7, 125446),
        (0, 125424),
        (2, 125277),
        (3, 125154),
        (4, 124993),
        (1, 124724),
        (6, 124493),
        (5, 124489),
    ];
    let pop_median = NiceFloat(3.5);
    let sample_median = (3, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(3.5),
        stdev: NiceFloat(2.29128784747792),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2380952380952381),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.498237000000012),
        stdev: NiceFloat(2.2924389080424907),
        skewness: NiceFloat(0.0018634242915326487),
        kurtosis: NiceFloat(-1.2380484256988056),
    };
    random_masked_values_helper::<u8>(
        3,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u8, pow = 7
    let values = &[
        113, 100, 87, 60, 93, 61, 117, 23, 7, 72, 105, 12, 114, 39, 104, 100, 114, 111, 107, 72,
    ];
    let common_values = &[
        (121, 8065),
        (55, 8045),
        (88, 8031),
        (80, 8005),
        (27, 8004),
        (45, 7997),
        (74, 7997),
        (63, 7966),
        (2, 7958),
        (68, 7954),
    ];
    let pop_median = NiceFloat(63.5);
    let sample_median = (63, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(63.5),
        stdev: NiceFloat(36.94928957368463),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2001464933162425),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(63.48145299999957),
        stdev: NiceFloat(36.93806598117268),
        skewness: NiceFloat(-0.0001294633581290759),
        kurtosis: NiceFloat(-1.1988830398738437),
    };
    random_masked_values_helper::<u8>(
        7,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u8, pow = 8
    let values = &[
        113, 228, 87, 188, 93, 189, 117, 151, 7, 72, 233, 12, 114, 39, 104, 228, 242, 239, 235, 200,
    ];
    let common_values = &[
        (88, 4062),
        (121, 4052),
        (173, 4045),
        (47, 4041),
        (27, 4036),
        (123, 4034),
        (74, 4032),
        (183, 4030),
        (16, 4021),
        (55, 4015),
    ];
    let pop_median = NiceFloat(127.5);
    let sample_median = (127, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(127.5),
        stdev: NiceFloat(73.90027063549903),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.200036621652552),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(127.45649300000248),
        stdev: NiceFloat(73.9152992323804),
        skewness: NiceFloat(-0.0004998735838741764),
        kurtosis: NiceFloat(-1.200958676605836),
    };
    random_masked_values_helper::<u8>(
        8,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, pow = 0
    let values = &[0; 20];
    let common_values = &[(0, 1_000_000)];
    let pop_median = NiceFloat(0.0);
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        stdev: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        kurtosis: NiceFloat(f64::NAN),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        stdev: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        kurtosis: NiceFloat(f64::NAN),
    };
    random_masked_values_helper::<i8>(
        0,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, pow = 1
    let values = &[1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0];
    let common_values = &[(0, 500187), (1, 499813)];
    let pop_median = NiceFloat(0.5);
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(0.5),
        stdev: NiceFloat(0.5),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-2.0),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.49981300000001017),
        stdev: NiceFloat(0.5000002150311751),
        skewness: NiceFloat(0.0007480000523136409),
        kurtosis: NiceFloat(-1.999999440496001),
    };
    random_masked_values_helper::<i8>(
        1,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, pow = 2
    let values = &[1, 0, 3, 0, 1, 1, 1, 3, 3, 0, 1, 0, 2, 3, 0, 0, 2, 3, 3, 0];
    let common_values = &[(3, 250600), (0, 250417), (2, 249770), (1, 249213)];
    let pop_median = NiceFloat(1.5);
    let sample_median = (2, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1.5),
        stdev: NiceFloat(1.118033988749895),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.3599999999999999),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.5005529999999707),
        stdev: NiceFloat(1.1189436742865808),
        skewness: NiceFloat(-0.0009920908109427875),
        kurtosis: NiceFloat(-1.3620790865995875),
    };
    random_masked_values_helper::<i8>(
        2,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, pow = 3
    let values = &[1, 4, 7, 4, 5, 5, 5, 7, 7, 0, 1, 4, 2, 7, 0, 4, 2, 7, 3, 0];
    let common_values = &[
        (7, 125446),
        (0, 125424),
        (2, 125277),
        (3, 125154),
        (4, 124993),
        (1, 124724),
        (6, 124493),
        (5, 124489),
    ];
    let pop_median = NiceFloat(3.5);
    let sample_median = (3, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(3.5),
        stdev: NiceFloat(2.29128784747792),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2380952380952381),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.498237000000012),
        stdev: NiceFloat(2.2924389080424907),
        skewness: NiceFloat(0.0018634242915326487),
        kurtosis: NiceFloat(-1.2380484256988056),
    };
    random_masked_values_helper::<i8>(
        3,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, pow = 7
    let values = &[
        113, 100, 87, 60, 93, 61, 117, 23, 7, 72, 105, 12, 114, 39, 104, 100, 114, 111, 107, 72,
    ];
    let common_values = &[
        (121, 8065),
        (55, 8045),
        (88, 8031),
        (80, 8005),
        (27, 8004),
        (45, 7997),
        (74, 7997),
        (63, 7966),
        (2, 7958),
        (68, 7954),
    ];
    let pop_median = NiceFloat(63.5);
    let sample_median = (63, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(63.5),
        stdev: NiceFloat(36.94928957368463),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2001464933162425),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(63.48145299999957),
        stdev: NiceFloat(36.93806598117268),
        skewness: NiceFloat(-0.0001294633581290759),
        kurtosis: NiceFloat(-1.1988830398738437),
    };
    random_masked_values_helper::<i8>(
        7,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, pow = 8
    let values = &[
        113, -28, 87, -68, 93, -67, 117, -105, 7, 72, -23, 12, 114, 39, 104, -28, -14, -17, -21,
        -56,
    ];
    let common_values = &[
        (88, 4062),
        (121, 4052),
        (-83, 4045),
        (47, 4041),
        (27, 4036),
        (123, 4034),
        (74, 4032),
        (-73, 4030),
        (16, 4021),
        (55, 4015),
    ];
    let pop_median = NiceFloat(-64.5);
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-64.5),
        stdev: NiceFloat(36.94928957368463),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2001464933162425),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.4935870000000081),
        stdev: NiceFloat(73.87406122754155),
        skewness: NiceFloat(0.00046947456841985925),
        kurtosis: NiceFloat(-1.199505878879556),
    };
    random_masked_values_helper::<i8>(
        8,
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}
