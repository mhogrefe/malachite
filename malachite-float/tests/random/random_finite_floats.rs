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
use malachite_float::random::random_finite_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn random_finite_floats_helper(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_zero_p_numerator: u64,
    mean_zero_p_denominator: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper(
        random_finite_floats(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
            mean_zero_p_numerator,
            mean_zero_p_denominator,
        ),
        expected_values,
        expected_values_hex,
        expected_common_values,
        expected_common_values_hex,
        expected_median,
        expected_median_hex,
        expected_moment_stats,
    );
}

#[test]
fn test_random_finite_floats() {
    // mean |sci_exponent| 1, mean precision 2, special probability 1/10
    let values = &[
        "-3.0", "-1.0", "-2.0", "1.12", "-1.0", "1.5", "-2.0", "-0.3", "-1.0", "0.8", "0.0",
        "0.338", "-1.2", "-0.59", "8.0", "-6.0", "-3.0", "-0.2", "-0.8", "-0.5",
    ];
    let values_hex = &[
        "-0x3.0#3",
        "-0x1.0#3",
        "-0x2.0#2",
        "0x1.2#5",
        "-0x1.0#2",
        "0x1.8#3",
        "-0x2.0#1",
        "-0x0.5#3",
        "-0x1.0#1",
        "0x0.c#2",
        "0x0.0",
        "0x0.568#9",
        "-0x1.4#3",
        "-0x0.98#5",
        "0x8.0#1",
        "-0x6.0#2",
        "-0x3.0#2",
        "-0x0.4#1",
        "-0x0.c#2",
        "-0x0.8#2",
    ];
    let common_values = &[
        ("1.0", 75012),
        ("-1.0", 74560),
        ("0.0", 49991),
        ("-0.0", 49908),
        ("0.5", 37822),
        ("2.0", 37501),
        ("-0.5", 37342),
        ("-2.0", 37246),
        ("-0.2", 18908),
        ("-1.0", 18873),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 75012),
        ("-0x1.0#1", 74560),
        ("0x0.0", 49991),
        ("-0x0.0", 49908),
        ("0x0.8#1", 37822),
        ("0x2.0#1", 37501),
        ("-0x0.8#1", 37342),
        ("-0x2.0#1", 37246),
        ("-0x0.4#1", 18908),
        ("-0x1.0#2", 18873),
    ];
    let sample_median = ("0.0", None);
    let sample_median_hex = ("0x0.0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.7164152871702301),
        standard_deviation: NiceFloat(1678.2398853586644),
        skewness: NiceFloat(-81.29182513358477),
        excess_kurtosis: NiceFloat(334895.7379317797),
    };
    random_finite_floats_helper(
        1,
        1,
        2,
        1,
        1,
        10,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 1, mean precision 64, special probability 1/10
    let values = &[
        "-2.55291108846570214097387565888051871482698844535",
        "-1.42916620847224432",
        "-2.8",
        "1.627895434494957473077687019296248257966273",
        "-1.7636659376",
        "1.0673947",
        "-2.56646360702751047",
        "-0.26139226673975317791585341",
        "-1.1299945035408183423314884117645901537",
        "0.85631649",
        "0.0",
        "0.38895408170042664025227",
        "-1.12207115063093089756992765",
        "-0.934062407041679788538190170179503",
        "11.30718",
        "-5.0",
        "-3.2134957439331342",
        "-0.35078847666770333344764253961",
        "-0.8696",
        "-0.61703047474",
    ];
    let values_hex = &[
        "-0x2.8d8b94c28e52f82d1e50104590476568ffae14e#158",
        "-0x1.6dddd62defc582#57",
        "-0x2.c#5",
        "0x1.a0bdc15476ab627f1b98bc51ab2497603df6#145",
        "-0x1.c37f9c630#34",
        "0x1.1140c8#22",
        "-0x2.9103c24a8eaeb4#57",
        "-0x0.42ea9a8513159b517a2ee8#84",
        "-0x1.214751dd5e1921c008153fce88b687c#123",
        "0x0.db378ec#26",
        "0x0.0",
        "0x0.63927ea48c8e2dfcdd8#75",
        "-0x1.1f400e0fbeb4413f6af44#85",
        "-0x0.ef1eb6c2aac41a798ac240288da#107",
        "0xb.4ea38#21",
        "-0x5.0#4",
        "-0x3.36a7a836072b20#55",
        "-0x0.59cd460b19f84a9b4a9886c4#94",
        "-0x0.dea0#13",
        "-0x0.9df5b58da#36",
    ];
    let common_values = &[
        ("0.0", 49991),
        ("-0.0", 49908),
        ("1.0", 2360),
        ("-1.0", 2302),
        ("2.0", 1209),
        ("-1.0", 1197),
        ("-0.5", 1177),
        ("1.0", 1168),
        ("0.5", 1158),
        ("-2.0", 1158),
    ];
    let common_values_hex = &[
        ("0x0.0", 49991),
        ("-0x0.0", 49908),
        ("0x1.0#1", 2360),
        ("-0x1.0#1", 2302),
        ("0x2.0#1", 1209),
        ("-0x1.0#2", 1197),
        ("-0x0.8#1", 1177),
        ("0x1.0#2", 1168),
        ("0x0.8#1", 1158),
        ("-0x2.0#1", 1158),
    ];
    let sample_median = ("0.0", None);
    let sample_median_hex = ("0x0.0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.6054865970994369),
        standard_deviation: NiceFloat(1644.9473368833571),
        skewness: NiceFloat(-6.939646339338025),
        excess_kurtosis: NiceFloat(322737.3533747247),
    };
    random_finite_floats_helper(
        1,
        1,
        64,
        1,
        1,
        10,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 64, mean precision 2, special probability 1/10
    let values = &[
        "-5.0e13", "-2.2e-16", "-0.5", "3.0e8", "-9.5e-7", "0.38", "-1.0e-11", "-7.0e-17",
        "-3.0e10", "1.7e-13", "0.0", "3.99e20", "-1.6e-29", "-4.7e28", "1.0e-8", "-2.5e-21",
        "-1.0e10", "-1.0e43", "-0.19", "-0.1",
    ];
    let values_hex = &[
        "-0x3.0E+11#3",
        "-0x1.0E-13#3",
        "-0x0.8#2",
        "0x1.2E+7#5",
        "-0x0.000010#2",
        "0x0.6#3",
        "-0x1.0E-9#1",
        "-0x5.0E-14#3",
        "-0x8.0E+8#1",
        "0x3.0E-11#2",
        "0x0.0",
        "0x1.5aE+17#9",
        "-0x1.4E-24#3",
        "-0x9.8E+23#5",
        "0x4.0E-7#1",
        "-0xc.0E-18#2",
        "-0x3.0E+8#2",
        "-0x8.0E+35#1",
        "-0x0.3#2",
        "-0x0.2#2",
    ];
    let common_values = &[
        ("0.0", 49991),
        ("-0.0", 49908),
        ("1.0", 1791),
        ("0.5", 1788),
        ("-1.0", 1782),
        ("-4.0", 1778),
        ("-0.5", 1727),
        ("2.0", 1695),
        ("4.0", 1690),
        ("2.0e1", 1689),
    ];
    let common_values_hex = &[
        ("0x0.0", 49991),
        ("-0x0.0", 49908),
        ("0x1.0#1", 1791),
        ("0x0.8#1", 1788),
        ("-0x1.0#1", 1782),
        ("-0x4.0#1", 1778),
        ("-0x0.8#1", 1727),
        ("0x2.0#1", 1695),
        ("0x4.0#1", 1690),
        ("0x1.0E+1#1", 1689),
    ];
    let sample_median = ("0.0", None);
    let sample_median_hex = ("0x0.0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.7899297879002272e243),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_finite_floats_helper(
        64,
        1,
        2,
        1,
        1,
        10,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 64, mean precision 64, special probability 1/10
    let values = &[
        "-44911286823141.1855899032680253942982725544577883",
        "-3.17338646132424422e-16",
        "-0.69",
        "436984853.27897203898621663845006920021638213",
        "-1.6819629074e-6",
        "0.2668487",
        "-1.86734804230764746e-11",
        "-5.8040742598686895873572801e-17",
        "-38826315498.940567851125400956255170888",
        "1.94703828e-13",
        "0.0",
        "459195929699359096631.38",
        "-1.41625290177455750344133296e-29",
        "-74004048183563132066750152744.5532",
        "2.106127e-8",
        "-2.1e-21",
        "-13801859126.0280018",
        "-1.5645688873494313277150105805e43",
        "-0.21741",
        "-0.154257618684",
    ];
    let values_hex = &[
        "-0x28d8b94c28e5.2f82d1e50104590476568ffae14e#158",
        "-0x1.6dddd62defc582E-13#57",
        "-0x0.b0#5",
        "0x1a0bdc15.476ab627f1b98bc51ab2497603df6#145",
        "-0x0.00001c37f9c630#34",
        "0x0.445032#22",
        "-0x1.4881e12547575aE-9#57",
        "-0x4.2ea9a8513159b517a2ee8E-14#84",
        "-0x90a3a8eea.f0c90e0040a9fe7445b43e#123",
        "0x3.6cde3bE-11#26",
        "0x0.0",
        "0x18e49fa923238b7f37.60#75",
        "-0x1.1f400e0fbeb4413f6af44E-24#85",
        "-0xef1eb6c2aac41a798ac24028.8da#107",
        "0x5.a751cE-7#21",
        "-0xa.0E-18#4",
        "-0x336a7a836.072b20#55",
        "-0xb.39a8c1633f0953695310d88E+35#94",
        "-0x0.37a8#13",
        "-0x0.277d6d6368#36",
    ];
    let common_values = &[
        ("0.0", 49991),
        ("-0.0", 49908),
        ("-2.0e1", 77),
        ("2.0", 66),
        ("-2.0", 61),
        ("-8.0", 61),
        ("-3.0e1", 59),
        ("-0.2", 58),
        ("0.5", 57),
        ("4.0e3", 57),
    ];
    let common_values_hex = &[
        ("0x0.0", 49991),
        ("-0x0.0", 49908),
        ("-0x1.0E+1#1", 77),
        ("0x2.0#1", 66),
        ("-0x2.0#1", 61),
        ("-0x8.0#1", 61),
        ("-0x2.0E+1#1", 59),
        ("-0x0.4#1", 58),
        ("0x0.8#1", 57),
        ("0x1.0E+3#1", 57),
    ];
    let sample_median = ("0.0", None);
    let sample_median_hex = ("0x0.0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.2330178132397616e243),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_finite_floats_helper(
        64,
        1,
        64,
        1,
        1,
        10,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn random_finite_floats_fail_1() {
    random_finite_floats(EXAMPLE_SEED, 1, 0, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_finite_floats_fail_2() {
    random_finite_floats(EXAMPLE_SEED, 0, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_finite_floats_fail_3() {
    random_finite_floats(EXAMPLE_SEED, 1, 1, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn random_finite_floats_fail_4() {
    random_finite_floats(EXAMPLE_SEED, 1, 1, 1, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_finite_floats_fail_5() {
    random_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0);
}

#[test]
#[should_panic]
fn random_finite_floats_fail_6() {
    random_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 2, 1);
}
