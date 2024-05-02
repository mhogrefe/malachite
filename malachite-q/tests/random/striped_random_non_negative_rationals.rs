// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_q::random::striped_random_non_negative_rationals;
use malachite_q::test_util::random::random_rationals_helper_helper;

#[allow(clippy::too_many_arguments)]
fn striped_random_non_negative_rationals_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_rationals_helper_helper(
        striped_random_non_negative_rationals(
            EXAMPLE_SEED,
            mean_stripe_numerator,
            mean_stripe_denominator,
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
fn test_striped_random_non_negative_rationals() {
    // mean bits = 65/64
    let values = &[
        "0", "0", "0", "2", "0", "0", "1", "0", "0", "0", "0", "0", "0", "0", "16", "1", "4", "1",
        "1", "0",
    ];
    let common_values = &[
        ("0", 496048),
        ("1", 247664),
        ("2", 62226),
        ("3", 61847),
        ("7", 23467),
        ("4", 23204),
        ("8", 8889),
        ("15", 8801),
        ("5", 7917),
        ("6", 7733),
    ];
    let sample_median = ("2/3", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(8.241369898412524),
        standard_deviation: NiceFloat(1203.8320236802172),
        skewness: NiceFloat(650.4921628408604),
        excess_kurtosis: NiceFloat(487036.59115699964),
    };
    striped_random_non_negative_rationals_helper(
        4,
        1,
        65,
        64,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 2
    let values = &[
        "2/3", "4/3", "0", "1", "0", "1/2", "448", "0", "0", "1", "5/14", "11/3", "0", "1/3",
        "19/3", "1/3", "0", "1", "0", "15",
    ];
    let common_values = &[
        ("0", 333130),
        ("1", 134397),
        ("2", 44677),
        ("3", 40161),
        ("1/2", 33752),
        ("1/3", 30003),
        ("4", 22136),
        ("7", 19720),
        ("1/4", 12536),
        ("8", 11373),
    ];
    let sample_median = ("2/3", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(12771.729072597298),
        standard_deviation: NiceFloat(5028817.645526858),
        skewness: NiceFloat(587.1636750940664),
        excess_kurtosis: NiceFloat(372144.3890017075),
    };
    striped_random_non_negative_rationals_helper(
        4,
        1,
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 32
    let values = &[
        "8192/127",
        "16776704/4396972769407",
        "8796093005951/648518346332962816",
        "87381/2863267840",
        "1024/2043",
        "51/58408828928",
        "85/13521606402434254795714066382848",
        "270335/7",
        "59421159664630116152453890047/9444741445172838006656",
        "6291455/1154891846623166464",
        "4503599631564799/114177029184456441820717001177155938271778439152",
        "40247906632508999881205124923399/137438953471",
        "73/154619122249",
        "1024/39611663922002864317824761855",
        "32",
        "127/9",
        "2199023247360/287",
        "1/8257539",
        "590156181179127562240/131199",
        "1/85",
    ];
    let common_values = &[
        ("0", 30369),
        ("1", 3494),
        ("2", 1638),
        ("1/2", 1619),
        ("4", 1527),
        ("1/4", 1481),
        ("1/8", 1348),
        ("8", 1328),
        ("16", 1204),
        ("1/16", 1190),
    ];
    let sample_median = ("2/3", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.681561663446933e148),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_non_negative_rationals_helper(
        16,
        1,
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 64
    let values = &[
        "1464583847936/7981747608676504359847391117664870922673555168908629",
        "2422574005712127994617856/100041309094775311912751523786213765636294062424476459466751",
        "9671406556916483641901054/2047",
        "1/10141204801678261259383949230080",
        "1/10384593719487506031596923529461760",
        "166153499473114484112975882535075839/1073741824",
        "1073758207/2097152",
        "10889035740836205568492768571262465220607/31",
        "16225927683142697268042315648307/15474248646392859802468352",
        "211174952009727/4294836224",
        "1125625028999183/309485009533116616750923776",
        "160551237036734989468671/2146697215",
        "4325375/324527219843164634252394901798911",
        "5666839779310716881032/42255019850195730860877091089",
        "201487684640834221069648/46912675075413",
        "1365/52818778157753880297518486869",
        "17179869184/7",
        "2420212822470693171986431/34359738367",
        "274877382656/11150372599265311570767859136324172163055871",
        "181/10141204802612896292451899146325",
    ];
    let common_values = &[
        ("0", 15382),
        ("1", 1486),
        ("1/2", 807),
        ("1/4", 728),
        ("2", 725),
        ("4", 693),
        ("8", 691),
        ("1/16", 685),
        ("1/8", 655),
        ("16", 628),
    ];
    let sample_median = (
        "28334198898317382877184/42501298345826806923263",
        Some("17179869184/25769803775"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(5.67251933470839e272),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_non_negative_rationals_helper(
        32,
        1,
        64,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn striped_random_non_negative_rationals_fail_1() {
    striped_random_non_negative_rationals(EXAMPLE_SEED, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_non_negative_rationals_fail_2() {
    striped_random_non_negative_rationals(EXAMPLE_SEED, 2, 3, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_non_negative_rationals_fail_3() {
    striped_random_non_negative_rationals(EXAMPLE_SEED, 4, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_non_negative_rationals_fail_4() {
    striped_random_non_negative_rationals(EXAMPLE_SEED, 4, 1, 2, 3);
}
