// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::geometric::geometric_random_signed_inclusive_range;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    double_truncated_geometric_dist_assertions, negative_truncated_geometric_dist_assertions,
    truncated_geometric_dist_assertions, CheckedToF64, MomentStats,
};
use std::panic::catch_unwind;

fn geometric_random_signed_inclusive_range_helper<
    U: CheckedToF64 + PrimitiveUnsigned,
    S: CheckedToF64
        + PrimitiveSigned
        + UnsignedAbs<Output = U>
        + WrappingFrom<<S as UnsignedAbs>::Output>,
>(
    a: S,
    b: S,
    abs_um_numerator: u64,
    abs_um_denominator: u64,
    expected_values: &[S],
    expected_common_values: &[(S, usize)],
    expected_natural_mean: Option<NiceFloat<f64>>,
    expected_pop_median: (S, Option<S>),
    expected_sample_median: (S, Option<S>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    let xs = geometric_random_signed_inclusive_range::<S>(
        EXAMPLE_SEED,
        a,
        b,
        abs_um_numerator,
        abs_um_denominator,
    );
    if a >= S::ZERO {
        assert!(expected_natural_mean.is_none());
        truncated_geometric_dist_assertions(
            xs,
            a,
            b,
            abs_um_numerator,
            abs_um_denominator,
            expected_values,
            expected_common_values,
            expected_pop_median,
            expected_sample_median,
            expected_pop_moment_stats,
            expected_sample_moment_stats,
        );
    } else if b <= S::ONE {
        assert!(expected_natural_mean.is_none());
        negative_truncated_geometric_dist_assertions(
            xs,
            b,
            a,
            abs_um_numerator,
            abs_um_denominator,
            expected_values,
            expected_common_values,
            expected_pop_median,
            expected_sample_median,
            expected_pop_moment_stats,
            expected_sample_moment_stats,
        );
    } else {
        double_truncated_geometric_dist_assertions(
            xs,
            a,
            b,
            abs_um_numerator,
            abs_um_denominator,
            expected_values,
            expected_common_values,
            expected_natural_mean.unwrap(),
            expected_pop_median,
            expected_sample_median,
            expected_pop_moment_stats,
            expected_sample_moment_stats,
        );
    }
}

#[test]
fn test_geometric_random_signed_inclusive_range() {
    // i8, 5, 5, um = 10 (um is irrelevant here)
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
    geometric_random_signed_inclusive_range_helper::<_, i8>(
        5,
        5,
        10,
        1,
        values,
        common_values,
        None,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i16, 1, 6, um = 3
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
    geometric_random_signed_inclusive_range_helper::<_, i16>(
        1,
        6,
        3,
        1,
        values,
        common_values,
        None,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i32, 10, 19, um = 30
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
    geometric_random_signed_inclusive_range_helper::<_, i32>(
        10,
        19,
        30,
        1,
        values,
        common_values,
        None,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i64, -20, -11, um = 30
    let values = &[
        -17, -14, -12, -14, -12, -15, -16, -13, -12, -13, -19, -11, -11, -17, -11, -19, -13, -16,
        -20, -19,
    ];
    let common_values = &[
        (-11, 124528),
        (-12, 118758),
        (-13, 112479),
        (-14, 107215),
        (-15, 101400),
        (-16, 96120),
        (-17, 91622),
        (-18, 86347),
        (-19, 82852),
        (-20, 78679),
    ];
    let pop_median = (-15, None);
    let sample_median = (-15, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-15.078692813090253),
        standard_deviation: NiceFloat(2.853334884793182),
        skewness: NiceFloat(-0.18019334055480088),
        excess_kurtosis: NiceFloat(-1.1801134249293568),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-15.076649000000186),
        standard_deviation: NiceFloat(2.8539222266332365),
        skewness: NiceFloat(-0.18245914264956217),
        excess_kurtosis: NiceFloat(-1.1796684829530097),
    };
    geometric_random_signed_inclusive_range_helper::<_, i64>(
        -20,
        -11,
        30,
        1,
        values,
        common_values,
        None,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, -100, 99, um = 30
    let values =
        &[-32, -31, -88, 52, -40, 64, -36, -1, -7, 46, 14, 48, -61, -60, 21, -55, -69, -26, -44, 6];
    let common_values = &[
        (0, 16949),
        (-1, 16668),
        (1, 16340),
        (-2, 15893),
        (2, 15885),
        (-3, 15421),
        (3, 15350),
        (4, 14937),
        (-4, 14913),
        (5, 14576),
    ];
    let pop_median = (0, None);
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.06416536680865222),
        standard_deviation: NiceFloat(35.06981580711165),
        skewness: NiceFloat(-0.009387535787624381),
        excess_kurtosis: NiceFloat(0.331207115680745),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.031523999999998706),
        standard_deviation: NiceFloat(34.99870187802065),
        skewness: NiceFloat(-0.009330685977635587),
        excess_kurtosis: NiceFloat(0.33242667080464994),
    };
    geometric_random_signed_inclusive_range_helper::<_, i8>(
        -100,
        99,
        30,
        1,
        values,
        common_values,
        Some(NiceFloat(26.065423999999997)),
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8, i8::MIN, i8::MAX, um = 30
    let values =
        &[-32, -31, -88, 52, -40, 64, -36, -1, -7, 46, 14, 48, -61, -60, 21, -55, -69, -26, -44, 6];
    let common_values = &[
        (0, 16612),
        (1, 16170),
        (-1, 16142),
        (2, 15527),
        (-2, 15450),
        (-3, 15004),
        (3, 14998),
        (4, 14694),
        (-4, 14522),
        (5, 14226),
    ];
    let pop_median = (0, None);
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.03203944371479178),
        standard_deviation: NiceFloat(38.608914337623204),
        skewness: NiceFloat(-0.006631455473213565),
        excess_kurtosis: NiceFloat(0.885187645693085),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.006649000000000714),
        standard_deviation: NiceFloat(38.52435386760686),
        skewness: NiceFloat(-0.002824821494549616),
        excess_kurtosis: NiceFloat(0.8938770112620578),
    };
    geometric_random_signed_inclusive_range_helper::<_, i8>(
        i8::MIN,
        i8::MAX,
        30,
        1,
        values,
        common_values,
        Some(NiceFloat(28.03693899999907)),
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}

fn geometric_random_signed_inclusive_range_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(geometric_random_signed_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::TWO,
        T::ONE,
        1,
        1
    ));
    assert_panic!(geometric_random_signed_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::ONE,
        T::ZERO,
        1,
        1
    ));
    assert_panic!(geometric_random_signed_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::ZERO,
        T::exact_from(10),
        1,
        0
    ));
    assert_panic!(geometric_random_signed_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::ONE,
        T::exact_from(10),
        9,
        10
    ));
    assert_panic!(geometric_random_signed_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::exact_from(-10),
        T::NEGATIVE_ONE,
        9,
        10
    ));
    assert_panic!(geometric_random_signed_inclusive_range::<T>(
        EXAMPLE_SEED,
        T::ZERO,
        T::exact_from(10),
        u64::MAX,
        u64::MAX - 1
    ));
}

#[test]
fn geometric_random_signed_inclusive_range_fail() {
    apply_fn_to_signeds!(geometric_random_signed_inclusive_range_fail_helper);
}
