// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::One;
use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_nz::natural::Natural;
use malachite_q::random::random_rational_with_denominator_range_to_negative_infinity;
use malachite_q::test_util::random::random_rationals_helper_helper;
use malachite_q::Rational;
use std::str::FromStr;

fn random_rational_with_denominator_range_to_negative_infinity_helper(
    d: &str,
    a: &str,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_rationals_helper_helper(
        random_rational_with_denominator_range_to_negative_infinity(
            EXAMPLE_SEED,
            Natural::from_str(d).unwrap(),
            Rational::from_str(a).unwrap(),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        expected_values,
        expected_common_values,
        expected_sample_median,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_random_rational_with_denominator_range_to_negative_infinity() {
    let values = &[
        "-2",
        "-4",
        "-178",
        "-1084828126",
        "-6",
        "-12",
        "-56",
        "-591",
        "-5283",
        "-5606382754",
        "-3",
        "-35",
        "-1",
        "-1",
        "-65728184",
        "-6",
        "0",
        "-7",
        "-110989",
        "-774544411777231510",
    ];
    let common_values = &[
        ("0", 90859),
        ("-1", 82901),
        ("-2", 37557),
        ("-3", 37534),
        ("-6", 17244),
        ("-5", 17221),
        ("-7", 17166),
        ("-4", 16881),
        ("-10", 7792),
        ("-8", 7781),
    ];
    let sample_median = ("-81", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-8.520127903486984e34),
        standard_deviation: NiceFloat(7.329977056427779e37),
        skewness: NiceFloat(-961.5590415384418),
        excess_kurtosis: NiceFloat(943383.4745426066),
    };
    random_rational_with_denominator_range_to_negative_infinity_helper(
        "1",
        "0",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "-591/2",
        "-5283/2",
        "-3/2",
        "-35/2",
        "-1/2",
        "-1/2",
        "-7/2",
        "-110989/2",
        "-1/2",
        "-5/2",
        "-751/2",
        "-7943/2",
        "-7065/2",
        "-8413078495/2",
        "-3/2",
        "-1/2",
        "-1/2",
        "-69/2",
        "-13/2",
        "-7749/2",
    ];
    let common_values = &[
        ("-1/2", 167084),
        ("-3/2", 75468),
        ("-7/2", 34473),
        ("-5/2", 34378),
        ("-11/2", 15785),
        ("-13/2", 15606),
        ("-15/2", 15565),
        ("-9/2", 15464),
        ("-29/2", 7220),
        ("-27/2", 7166),
    ];
    let sample_median = ("-89/2", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.0730191792105077e40),
        standard_deviation: NiceFloat(1.072277458027706e43),
        skewness: NiceFloat(-999.9978064950928),
        excess_kurtosis: NiceFloat(999994.0749119784),
    };
    random_rational_with_denominator_range_to_negative_infinity_helper(
        "2",
        "0",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "-35/6",
        "-1/6",
        "-1/6",
        "-7/6",
        "-110989/6",
        "-1/6",
        "-5/6",
        "-751/6",
        "-7943/6",
        "-8413078495/6",
        "-1/6",
        "-1/6",
        "-13/6",
        "-7/6",
        "-744559/6",
        "-707/6",
        "-751063343911/6",
        "-28206975458359/6",
        "-971/6",
        "-3925/6",
    ];
    let common_values = &[
        ("-1/6", 241941),
        ("-5/6", 50133),
        ("-7/6", 50049),
        ("-11/6", 22890),
        ("-13/6", 22726),
        ("-29/6", 10488),
        ("-23/6", 10390),
        ("-19/6", 10358),
        ("-25/6", 10358),
        ("-31/6", 10291),
    ];
    let sample_median = ("-67/6", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-2.4338034128378777e36),
        standard_deviation: NiceFloat(2.4305548034225437e39),
        skewness: NiceFloat(-999.9972159387221),
        excess_kurtosis: NiceFloat(999993.287033733),
    };
    random_rational_with_denominator_range_to_negative_infinity_helper(
        "6",
        "0",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "-99/100",
        "-913/100",
        "-11/100",
        "7/100",
        "1/100",
        "-10973/100",
        "47/100",
        "-39/100",
        "57/100",
        "-3/100",
        "23/100",
        "-10522099/100",
        "-1/100",
        "-29701/100",
        "1/100",
        "1/100",
        "-7743/100",
        "1/100",
        "-7/100",
        "-89199/100",
    ];
    let common_values = &[
        ("-1/100", 123615),
        ("1/100", 123443),
        ("3/100", 56191),
        ("-3/100", 56020),
        ("7/100", 25883),
        ("-7/100", 25543),
        ("-9/100", 11654),
        ("11/100", 11646),
        ("13/100", 11586),
        ("9/100", 11583),
    ];
    let sample_median = ("-1/100", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-9.380763184802002e48),
        standard_deviation: NiceFloat(9.380763184802352e51),
        skewness: NiceFloat(-999.9984999993959),
        excess_kurtosis: NiceFloat(999995.00000096),
    };
    random_rational_with_denominator_range_to_negative_infinity_helper(
        "100",
        "245850922/78256779",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "-509/100",
        "-427/100",
        "-407/100",
        "-343/100",
        "-591/100",
        "-913/100",
        "-321/100",
        "-1731/100",
        "-373/100",
        "-999/100",
        "-361/100",
        "-353/100",
        "-437/100",
        "-733/100",
        "-751/100",
        "-40711/100",
        "-921/100",
        "-5087/100",
        "-919/100",
        "-323/100",
    ];
    let common_values = &[
        ("-469/100", 6500),
        ("-497/100", 6483),
        ("-437/100", 6473),
        ("-473/100", 6460),
        ("-387/100", 6455),
        ("-421/100", 6454),
        ("-493/100", 6454),
        ("-379/100", 6447),
        ("-361/100", 6438),
        ("-347/100", 6432),
    ];
    let sample_median = ("-511/100", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-45.46337932000087),
        standard_deviation: NiceFloat(5322.134805564187),
        skewness: NiceFloat(-618.5045180626813),
        excess_kurtosis: NiceFloat(438771.04416568665),
    };
    random_rational_with_denominator_range_to_negative_infinity_helper(
        "100",
        "-245850922/78256779",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn random_rational_with_denominator_range_to_negative_infinity_fail_1() {
    random_rational_with_denominator_range_to_negative_infinity(
        EXAMPLE_SEED,
        Natural::ONE,
        Rational::from_unsigneds(1u32, 3),
        10,
        0,
    );
}

#[test]
#[should_panic]
fn random_rational_with_denominator_range_to_negative_infinity_fail_2() {
    random_rational_with_denominator_range_to_negative_infinity(
        EXAMPLE_SEED,
        Natural::ONE,
        Rational::from_signeds(-1i32, 3),
        2,
        3,
    );
}
