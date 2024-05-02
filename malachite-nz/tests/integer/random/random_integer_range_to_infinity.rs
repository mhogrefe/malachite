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
use malachite_nz::integer::random::random_integer_range_to_infinity;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::integer::random::random_integers_helper_helper;
use std::str::FromStr;

fn random_integer_range_to_infinity_helper(
    a: &str,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_integers_helper_helper(
        random_integer_range_to_infinity(
            EXAMPLE_SEED,
            Integer::from_str(a).unwrap(),
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
fn test_random_integer_range_to_infinity() {
    let values = &[
        "0", "14", "0", "8", "2", "6", "1", "0", "0", "0", "0", "0", "1", "1", "0", "0", "1", "1",
        "0", "0",
    ];
    let common_values = &[
        ("0", 500248),
        ("1", 249491),
        ("2", 62676),
        ("3", 62465),
        ("7", 15819),
        ("5", 15781),
        ("6", 15694),
        ("4", 15518),
        ("13", 3945),
        ("8", 3895),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.289019000000012),
        standard_deviation: NiceFloat(811.503067487901),
        skewness: NiceFloat(791.581366511165),
        excess_kurtosis: NiceFloat(717047.0759703598),
    };
    random_integer_range_to_infinity_helper(
        "0",
        1,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "20431208470830262",
        "2777240",
        "114",
        "12184833305054",
        "1121025855008623490210",
        "13478874522577592",
        "115311695",
        "7",
        "18",
        "54522366353",
        "2183264193236231773387459",
        "824",
        "18558864232439549193912",
        "15",
        "110989",
        "453270",
        "4307150",
        "45388024541",
        "47",
        "3345913274",
    ];
    let common_values = &[
        ("0", 30467),
        ("1", 29379),
        ("3", 14233),
        ("2", 14194),
        ("7", 6984),
        ("6", 6980),
        ("4", 6964),
        ("5", 6929),
        ("10", 3479),
        ("15", 3431),
    ];
    let sample_median = ("3201388", Some("3201522"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.480305129633914e129),
        standard_deviation: NiceFloat(2.4803051296331898e132),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_integer_range_to_infinity_helper(
        "0",
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "11", "182", "12", "152", "50", "94", "18", "11", "13", "15", "14", "13", "28", "24", "13",
        "13", "31", "19", "12", "13",
    ];
    let common_values = &[
        ("13", 83637),
        ("11", 83622),
        ("15", 83440),
        ("14", 83364),
        ("10", 83305),
        ("12", 82880),
        ("20", 15837),
        ("31", 15785),
        ("19", 15779),
        ("18", 15776),
    ];
    let sample_median = ("15", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(128.81076899999977),
        standard_deviation: NiceFloat(15255.606035258177),
        skewness: NiceFloat(845.8189997295934),
        excess_kurtosis: NiceFloat(789803.2243471228),
    };
    random_integer_range_to_infinity_helper(
        "10",
        5,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "118",
        "56",
        "1714",
        "55845661150",
        "93254818",
        "822568088563644",
        "120",
        "871591019599",
        "99",
        "1171796531603249384284396706",
        "3570371",
        "76271186",
        "69092967935443594634663005648041578296",
        "110",
        "39",
        "25543539470733",
        "317538101910",
        "206",
        "14906804826461850333",
        "95450125556931311",
    ];
    let common_values = &[
        ("13", 5882),
        ("14", 5840),
        ("12", 5734),
        ("15", 5645),
        ("11", 5644),
        ("10", 5642),
        ("18", 2148),
        ("27", 2143),
        ("19", 2134),
        ("23", 2134),
    ];
    let sample_median = ("7289020", Some("7289286"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.8276128186777812e120),
        standard_deviation: NiceFloat(1.8276117282901724e123),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_integer_range_to_infinity_helper(
        "10",
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "2", "-2", "-6", "0", "0", "-1", "-2", "-2", "2", "0", "0", "-1", "-7", "-2", "5", "0",
        "1", "1", "1", "-2",
    ];
    let common_values = &[
        ("0", 340125),
        ("-1", 170288),
        ("1", 170110),
        ("3", 42793),
        ("2", 42482),
        ("-2", 42480),
        ("-3", 42401),
        ("7", 10842),
        ("-6", 10756),
        ("-5", 10753),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(4.031789999999887),
        standard_deviation: NiceFloat(549.0621114688183),
        skewness: NiceFloat(481.1013689046789),
        excess_kurtosis: NiceFloat(256425.40976596656),
    };
    random_integer_range_to_infinity_helper(
        "-10",
        1,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "89270",
        "69403499476962893258904",
        "62",
        "59700386",
        "45674428",
        "696",
        "0",
        "-3",
        "3235",
        "7330",
        "-5",
        "15043",
        "11858",
        "2541784888",
        "7007928",
        "1518",
        "23",
        "909",
        "60054",
        "1",
    ];
    let common_values = &[
        ("0", 27154),
        ("1", 26497),
        ("-1", 26298),
        ("-2", 12852),
        ("2", 12781),
        ("3", 12707),
        ("-3", 12664),
        ("-8", 8085),
        ("-10", 8066),
        ("-9", 7928),
    ];
    let sample_median = ("293781", Some("293783"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(6.04296612240715e153),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_integer_range_to_infinity_helper(
        "-10",
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "1032867295426",
        "15476566285494",
        "1005058503561",
        "11872468885656",
        "3085108281010",
        "7786786793950",
        "1848070042786",
        "1008384510771",
        "1035939113223",
        "1034091049134",
        "1097997002237",
        "1066780473347",
        "1232902614972",
        "2160500927160",
        "1039676158979",
        "1075044604283",
        "1421346833487",
        "2100488049827",
        "1090935342918",
        "1033099299962",
    ];
    let common_values = &[
        ("1012318490312", 2),
        ("1020804407546", 2),
        ("1040579317197", 2),
        ("1041361099759", 2),
        ("1099357770481", 2),
        ("1000000358874", 1),
        ("1000000635467", 1),
        ("1000000743391", 1),
        ("1000001041678", 1),
        ("1000001124568", 1),
    ];
    let sample_median = ("1099468717392", Some("1099468761569"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(9002084314682.418),
        standard_deviation: NiceFloat(1175305974058995.2),
        skewness: NiceFloat(894.6662459454856),
        excess_kurtosis: NiceFloat(856946.924578041),
    };
    random_integer_range_to_infinity_helper(
        "1000000000000",
        41,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "2282426752182",
        "3076375863448",
        "33871433858738",
        "193926771358011304414",
        "683117911635193788",
        "95918191752968866622136",
        "5399022933155",
        "2679643424229893512354",
        "7585953905347",
        "17738519421136481929559726434799186",
        "70600001019637432",
        "77299482847566318",
        "332646018266965594347466935183659688140188647",
        "4856563742926",
        "1110539913949",
        "8169120771017371179759",
        "142409202767618812372",
        "3612730358681",
        "468787385712310874935747551",
        "9607440468294695468459788",
    ];
    let common_values = &[
        ("1000006091267", 1),
        ("1000006483280", 1),
        ("1000008421992", 1),
        ("1000009071089", 1),
        ("1000011436758", 1),
        ("1000013492649", 1),
        ("1000014387323", 1),
        ("1000020641917", 1),
        ("1000020859147", 1),
        ("1000020971497", 1),
    ];
    let sample_median = ("70366472614875784", Some("70368772587252716"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.0984547559553134e113),
        standard_deviation: NiceFloat(1.098369190533207e116),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_integer_range_to_infinity_helper(
        "1000000000000",
        64,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "2", "-2", "-6", "0", "0", "-1", "-2", "-2", "2", "0", "0", "-1", "-7", "-2", "5", "0",
        "1", "1", "1", "-2",
    ];
    let common_values = &[
        ("0", 332922),
        ("-1", 166652),
        ("1", 166524),
        ("3", 42164),
        ("2", 41585),
        ("-3", 41436),
        ("-2", 41400),
        ("5", 10546),
        ("4", 10540),
        ("-6", 10475),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.4130599999999974),
        standard_deviation: NiceFloat(777.5605240878597),
        skewness: NiceFloat(-244.83259806631784),
        excess_kurtosis: NiceFloat(225482.22529172004),
    };
    random_integer_range_to_infinity_helper(
        "-1000000000000",
        1,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "89270",
        "69403499476962893258904",
        "62",
        "0",
        "61363647650",
        "-956",
        "120",
        "99407",
        "-5283",
        "-171170",
        "-346513",
        "-15043",
        "119462940242",
        "6836752184",
        "-1720",
        "-30",
        "-999",
        "-45453",
        "-54",
        "78",
    ];
    let common_values = &[
        ("0", 17993),
        ("1", 17632),
        ("-1", 17419),
        ("-2", 8480),
        ("-3", 8468),
        ("3", 8420),
        ("2", 8265),
        ("6", 4225),
        ("4", 4223),
        ("-4", 4199),
    ];
    let sample_median = ("24", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.2795558914963709e155),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_integer_range_to_infinity_helper(
        "-1000000000000",
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn random_integer_range_to_infinity_fail_1() {
    random_integer_range_to_infinity(EXAMPLE_SEED, Integer::from(10), 1, 0);
}

#[test]
#[should_panic]
fn random_integer_range_to_infinity_fail_2() {
    random_integer_range_to_infinity(EXAMPLE_SEED, Integer::from(10), 4, 1);
}
