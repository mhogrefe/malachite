// Copyright © 2025 Mikhail Hogrefe
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
use malachite_q::random::random_rational_with_denominator_range;
use malachite_q::test_util::random::random_rationals_helper_helper;
use malachite_q::Rational;
use std::str::FromStr;

fn random_rational_with_denominator_range_helper(
    d: &str,
    a: &str,
    b: &str,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_rationals_helper_helper(
        random_rational_with_denominator_range(
            EXAMPLE_SEED,
            Natural::from_str(d).unwrap(),
            Rational::from_str(a).unwrap(),
            Rational::from_str(b).unwrap(),
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
fn test_random_rational_with_denominator_range() {
    let values = &[
        "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0",
        "0", "0",
    ];
    let common_values = &[("0", 1000000)];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_rational_with_denominator_range_helper(
        "1",
        "0",
        "1",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "1/2", "1/2", "1/2", "1/2", "1/2", "1/2", "1/2", "1/2", "1/2", "1/2", "1/2", "1/2", "1/2",
        "1/2", "1/2", "1/2", "1/2", "1/2", "1/2", "1/2",
    ];
    let common_values = &[("1/2", 1000000)];
    let sample_median = ("1/2", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.5),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_rational_with_denominator_range_helper(
        "2",
        "0",
        "1",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "5/6", "1/6", "1/6", "1/6", "1/6", "5/6", "1/6", "1/6", "1/6", "1/6", "1/6", "1/6", "1/6",
        "1/6", "1/6", "1/6", "1/6", "1/6", "5/6", "1/6",
    ];
    let common_values = &[("1/6", 783614), ("5/6", 216386)];
    let sample_median = ("1/6", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.3109239999999932),
        standard_deviation: NiceFloat(0.2745204048819774),
        skewness: NiceFloat(1.3775012070185304),
        excess_kurtosis: NiceFloat(-0.10249042466258196),
    };
    random_rational_with_denominator_range_helper(
        "6",
        "0",
        "1",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "41/100", "43/100", "41/100", "41/100", "39/100", "41/100", "49/100", "41/100", "41/100",
        "39/100", "49/100", "37/100", "37/100", "49/100", "39/100", "37/100", "41/100", "41/100",
        "43/100", "37/100",
    ];
    let common_values = &[
        ("37/100", 167531),
        ("47/100", 167302),
        ("49/100", 166766),
        ("41/100", 166355),
        ("43/100", 166287),
        ("39/100", 165759),
    ];
    let sample_median = ("43/100", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.42668872000000546),
        standard_deviation: NiceFloat(0.042331383354523355),
        skewness: NiceFloat(0.2167262624312793),
        excess_kurtosis: NiceFloat(-1.345880916970383),
    };
    random_rational_with_denominator_range_helper(
        "100",
        "1/3",
        "1/2",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3", "3",
        "3", "3",
    ];
    let common_values = &[("3", 1000000)];
    let sample_median = ("3", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_rational_with_denominator_range_helper(
        "1",
        "268876667/98914198",
        "245850922/78256779",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "17/6", "17/6", "17/6", "17/6", "17/6", "17/6", "17/6", "17/6", "17/6", "17/6", "17/6",
        "17/6", "17/6", "17/6", "17/6", "17/6", "17/6", "17/6", "17/6", "17/6",
    ];
    let common_values = &[("17/6", 1000000)];
    let sample_median = ("17/6", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.8333333333333335),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_rational_with_denominator_range_helper(
        "6",
        "268876667/98914198",
        "245850922/78256779",
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "301/100", "279/100", "313/100", "299/100", "301/100", "273/100", "279/100", "301/100",
        "297/100", "311/100", "309/100", "301/100", "279/100", "289/100", "279/100", "279/100",
        "309/100", "293/100", "287/100", "299/100",
    ];
    let common_values = &[
        ("299/100", 59348),
        ("307/100", 59112),
        ("281/100", 59097),
        ("297/100", 58997),
        ("283/100", 58975),
        ("293/100", 58941),
        ("309/100", 58919),
        ("311/100", 58910),
        ("287/100", 58857),
        ("289/100", 58794),
    ];
    let sample_median = ("293/100", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.937086759999876),
        standard_deviation: NiceFloat(0.12202969771309406),
        skewness: NiceFloat(-0.025551558114942263),
        excess_kurtosis: NiceFloat(-1.2130746374242132),
    };
    random_rational_with_denominator_range_helper(
        "100",
        "268876667/98914198",
        "245850922/78256779",
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
fn random_rational_with_denominator_range_fail_1() {
    random_rational_with_denominator_range(
        EXAMPLE_SEED,
        Natural::ONE,
        Rational::from_unsigneds(1u32, 3),
        Rational::from_unsigneds(2u32, 3),
        10,
        0,
    );
}

#[test]
#[should_panic]
fn random_rational_with_denominator_range_fail_2() {
    random_rational_with_denominator_range(
        EXAMPLE_SEED,
        Natural::ONE,
        Rational::from_unsigneds(1u32, 3),
        Rational::from_unsigneds(2u32, 3),
        2,
        3,
    );
}

#[test]
#[should_panic]
fn random_rational_with_denominator_range_fail_3() {
    random_rational_with_denominator_range(
        EXAMPLE_SEED,
        Natural::ONE,
        Rational::from_unsigneds(1u32, 3),
        Rational::from_unsigneds(1u32, 3),
        2,
        3,
    );
}

#[test]
#[should_panic]
fn random_rational_with_denominator_range_fail_4() {
    random_rational_with_denominator_range(
        EXAMPLE_SEED,
        Natural::ONE,
        Rational::from_unsigneds(1u32, 2),
        Rational::from_unsigneds(1u32, 3),
        2,
        3,
    );
}
