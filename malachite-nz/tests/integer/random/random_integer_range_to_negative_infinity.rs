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
use malachite_nz::integer::random::random_integer_range_to_negative_infinity;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::integer::random::random_integers_helper_helper;
use std::str::FromStr;

fn random_integer_range_to_negative_infinity_helper(
    a: &str,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_integers_helper_helper(
        random_integer_range_to_negative_infinity(
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
fn test_random_integer_range_to_negative_infinity() {
    let values = &[
        "0", "-14", "0", "-8", "-2", "-6", "-1", "0", "0", "0", "0", "0", "-1", "-1", "0", "0",
        "-1", "-1", "0", "0",
    ];
    let common_values = &[
        ("0", 500248),
        ("-1", 249491),
        ("-2", 62676),
        ("-3", 62465),
        ("-7", 15819),
        ("-5", 15781),
        ("-6", 15694),
        ("-4", 15518),
        ("-13", 3945),
        ("-8", 3895),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-7.289019000000012),
        standard_deviation: NiceFloat(811.503067487901),
        skewness: NiceFloat(-791.581366511165),
        excess_kurtosis: NiceFloat(717047.0759703598),
    };
    random_integer_range_to_negative_infinity_helper(
        "0",
        1,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "-20431208470830262",
        "-2777240",
        "-114",
        "-12184833305054",
        "-1121025855008623490210",
        "-13478874522577592",
        "-115311695",
        "-7",
        "-18",
        "-54522366353",
        "-2183264193236231773387459",
        "-824",
        "-18558864232439549193912",
        "-15",
        "-110989",
        "-453270",
        "-4307150",
        "-45388024541",
        "-47",
        "-3345913274",
    ];
    let common_values = &[
        ("0", 30467),
        ("-1", 29379),
        ("-3", 14233),
        ("-2", 14194),
        ("-7", 6984),
        ("-6", 6980),
        ("-4", 6964),
        ("-5", 6929),
        ("-10", 3479),
        ("-15", 3431),
    ];
    let sample_median = ("-3201522", Some("-3201388"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-2.480305129633914e129),
        standard_deviation: NiceFloat(2.4803051296331898e132),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_integer_range_to_negative_infinity_helper(
        "0",
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "2",
        "-24",
        "-2",
        "-1",
        "-1",
        "-8124",
        "-2",
        "-321835205711",
        "-99",
        "-10",
        "-145",
        "-1",
        "-814468690",
        "-20280",
        "-120",
        "1",
        "-2023",
        "10",
        "-909",
        "-10902",
    ];
    let common_values = &[
        ("0", 116331),
        ("1", 97337),
        ("-1", 96828),
        ("-3", 40657),
        ("2", 40328),
        ("-2", 40302),
        ("3", 40199),
        ("10", 18790),
        ("9", 18745),
        ("8", 18603),
    ];
    let sample_median = ("-1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-324066858236836.2),
        standard_deviation: NiceFloat(2.1808471647947933e17),
        skewness: NiceFloat(-782.8568751691934),
        excess_kurtosis: NiceFloat(641629.0527568299),
    };
    random_integer_range_to_negative_infinity_helper(
        "10",
        5,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "-2",
        "-69403499476962893258904",
        "-62",
        "-61363647650",
        "-64671510460",
        "0",
        "0",
        "-120",
        "-79",
        "-5283",
        "-171170",
        "-346513",
        "-15043",
        "-76271186",
        "-260083512",
        "-1720",
        "-1518",
        "1",
        "-3",
        "-49022969236123561123418405268118",
    ];
    let common_values = &[
        ("0", 27157),
        ("1", 26618),
        ("-1", 26439),
        ("-2", 12828),
        ("3", 12822),
        ("-3", 12808),
        ("2", 12788),
        ("10", 8081),
        ("9", 8077),
        ("8", 8044),
    ];
    let sample_median = ("-276826", Some("-276824"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.7536910581415426e155),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_integer_range_to_negative_infinity_helper(
        "10",
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "-13", "-182", "-13", "-152", "-50", "-94", "-18", "-14", "-12", "-10", "-11", "-15",
        "-28", "-24", "-15", "-13", "-31", "-19", "-15", "-11",
    ];
    let common_values = &[
        ("-13", 83731),
        ("-10", 83540),
        ("-12", 83478),
        ("-14", 83457),
        ("-11", 83187),
        ("-15", 82855),
        ("-20", 15837),
        ("-31", 15785),
        ("-19", 15779),
        ("-18", 15776),
    ];
    let sample_median = ("-15", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-128.80925900000398),
        standard_deviation: NiceFloat(15255.606046679814),
        skewness: NiceFloat(-845.8189981268482),
        excess_kurtosis: NiceFloat(789803.2223167756),
    };
    random_integer_range_to_negative_infinity_helper(
        "-10",
        5,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "-118",
        "-56",
        "-1714",
        "-55845661150",
        "-93254818",
        "-822568088563644",
        "-120",
        "-871591019599",
        "-99",
        "-1171796531603249384284396706",
        "-3570371",
        "-76271186",
        "-69092967935443594634663005648041578296",
        "-110",
        "-39",
        "-25543539470733",
        "-317538101910",
        "-206",
        "-14906804826461850333",
        "-95450125556931311",
    ];
    let common_values = &[
        ("-13", 5852),
        ("-12", 5824),
        ("-14", 5824),
        ("-11", 5734),
        ("-10", 5589),
        ("-15", 5564),
        ("-18", 2148),
        ("-27", 2143),
        ("-19", 2134),
        ("-23", 2134),
    ];
    let sample_median = ("-7289286", Some("-7289020"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.8276128186777812e120),
        standard_deviation: NiceFloat(1.8276117282901724e123),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_integer_range_to_negative_infinity_helper(
        "-10",
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
    random_integer_range_to_negative_infinity_helper(
        "1000000000000",
        1,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "89270",
        "18359148696",
        "-50",
        "-1189717027294",
        "-61363647650",
        "956",
        "0",
        "120",
        "99407",
        "5283",
        "-171170",
        "-346513",
        "-15043",
        "-119462940242",
        "6836752184",
        "1720",
        "-30",
        "-999",
        "-45453",
        "-54",
    ];
    let common_values = &[
        ("0", 18088),
        ("1", 17624),
        ("-1", 17275),
        ("3", 8602),
        ("2", 8437),
        ("-3", 8322),
        ("-2", 8240),
        ("7", 4233),
        ("-6", 4129),
        ("-4", 4119),
    ];
    let sample_median = ("-24", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-8.821612894174129e142),
        standard_deviation: NiceFloat(8.821611690087554e145),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_integer_range_to_negative_infinity_helper(
        "1000000000000",
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "-1098860169725",
        "-15476566285494",
        "-1039756450654",
        "-11872468885656",
        "-3085108281010",
        "-7786786793950",
        "-1848070042786",
        "-1001655869084",
        "-1066875920613",
        "-1005653510487",
        "-1039550426984",
        "-1016104838230",
        "-1232902614972",
        "-2160500927160",
        "-1077775969857",
        "-1032850377710",
        "-1421346833487",
        "-2100488049827",
        "-1095947428690",
        "-1014665028606",
    ];
    let common_values = &[
        ("-1002769937332", 2),
        ("-1073874696438", 2),
        ("-1000000188909", 1),
        ("-1000000496682", 1),
        ("-1000000510433", 1),
        ("-1000000739585", 1),
        ("-1000001292527", 1),
        ("-1000001626249", 1),
        ("-1000002315263", 1),
        ("-1000002353491", 1),
    ];
    let sample_median = ("-1099462180132", Some("-1099461808833"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-9002075457674.271),
        standard_deviation: NiceFloat(1175305974119130.0),
        skewness: NiceFloat(-894.6662458308058),
        excess_kurtosis: NiceFloat(856946.9244296869),
    };
    random_integer_range_to_negative_infinity_helper(
        "-1000000000000",
        41,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "-2282426752182",
        "-3076375863448",
        "-33871433858738",
        "-193926771358011304414",
        "-683117911635193788",
        "-95918191752968866622136",
        "-5399022933155",
        "-2679643424229893512354",
        "-7585953905347",
        "-17738519421136481929559726434799186",
        "-70600001019637432",
        "-77299482847566318",
        "-332646018266965594347466935183659688140188647",
        "-4856563742926",
        "-1110539913949",
        "-8169120771017371179759",
        "-142409202767618812372",
        "-3612730358681",
        "-468787385712310874935747551",
        "-9607440468294695468459788",
    ];
    let common_values = &[
        ("-1000001292527", 1),
        ("-1000003874527", 1),
        ("-1000008544047", 1),
        ("-1000010938009", 1),
        ("-1000029445751", 1),
        ("-1000041463842", 1),
        ("-1000043700145", 1),
        ("-1000044447812", 1),
        ("-1000047340863", 1),
        ("-1000049992535", 1),
    ];
    let sample_median = ("-70368772587252716", Some("-70366472614875784"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.0984547559553134e113),
        standard_deviation: NiceFloat(1.098369190533207e116),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_integer_range_to_negative_infinity_helper(
        "-1000000000000",
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
fn random_integer_range_to_negative_infinity_fail_1() {
    random_integer_range_to_negative_infinity(EXAMPLE_SEED, Integer::from(-10), 1, 0);
}

#[test]
#[should_panic]
fn random_integer_range_to_negative_infinity_fail_2() {
    random_integer_range_to_negative_infinity(EXAMPLE_SEED, Integer::from(-10), 4, 1);
}
