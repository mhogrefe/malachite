// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_q::random::random_nonzero_rationals;
use malachite_q::test_util::random::random_rationals_helper_helper;

fn random_nonzero_rationals_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_rationals_helper_helper(
        random_nonzero_rationals(EXAMPLE_SEED, mean_bits_numerator, mean_bits_denominator),
        expected_values,
        expected_common_values,
        expected_sample_median,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_random_nonzero_rationals() {
    // mean bits = 65/64
    let values = &[
        "-1", "-1", "1", "1", "-1", "1", "1", "1", "1", "1", "1", "-1", "1", "-1", "-1", "-1",
        "-1", "-1", "-1", "-1",
    ];
    let common_values = &[
        ("-1", 484924),
        ("1", 484658),
        ("3", 3815),
        ("1/3", 3811),
        ("-1/3", 3747),
        ("-2", 3743),
        ("1/2", 3740),
        ("-1/2", 3695),
        ("2", 3664),
        ("-3", 3591),
    ];
    let sample_median = ("-1/7", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.000309218589743558),
        standard_deviation: NiceFloat(1.0376177116118968),
        skewness: NiceFloat(0.006166050703241532),
        excess_kurtosis: NiceFloat(-1.2860190331216774),
    };
    random_nonzero_rationals_helper(
        65,
        64,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 2
    let values = &[
        "-8", "-1/7", "5", "1/3", "-1", "1/5", "1", "5", "1/26", "5/3", "1/2", "-3", "2", "-1/10",
        "-13/5", "-1/81", "-33", "-21", "-9/13", "-1",
    ];
    let common_values = &[
        ("1", 143164),
        ("-1", 142719),
        ("-2", 35933),
        ("1/2", 35874),
        ("-1/2", 35720),
        ("2", 35231),
        ("3", 34254),
        ("-1/3", 34198),
        ("-3", 34081),
        ("1/3", 34031),
    ];
    let sample_median = ("-1/14021", Some("-1/14079"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.22120311344442486),
        standard_deviation: NiceFloat(759.898347425165),
        skewness: NiceFloat(85.37622949885149),
        excess_kurtosis: NiceFloat(197537.4297586209),
    };
    random_nonzero_rationals_helper(
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 32
    let values = &[
        "-80861953616/9687130509484985",
        "-14557437513/313",
        "100721397389/392237929981",
        "713431423/1285",
        "-3887883364/889",
        "14185/969",
        "12609/11359517108746272468338071",
        "3443/4354945",
        "1/29",
        "5551/892095",
        "10/105173604567",
        "-19537757974145/22970361",
        "260/571",
        "-129425842/844581",
        "-11400586904841764775819697763820861082775967865/619503489",
        "-640859665273/199255145279143029",
        "-15530559035875594619912/1323170930793965328781",
        "-943/46106",
        "-1/113",
        "-58/1217",
    ];
    let common_values = &[
        ("1", 940),
        ("-1", 916),
        ("1/2", 463),
        ("-2", 458),
        ("2", 454),
        ("-1/2", 435),
        ("1/3", 391),
        ("-1/3", 389),
        ("3", 382),
        ("-3", 367),
    ];
    let sample_median = (
        "-56/1571996361141158873691832735034843333252522256904712087971186761383651552863627257507\
        9059423061",
        Some(
            "-322364/3099957057369745737816290559529344769064121899362905191001555856493272865037\
            24517150128896230616971",
        ),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-2.221976926004828e140),
        standard_deviation: NiceFloat(2.2219769258579568e143),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_nonzero_rationals_helper(
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 64
    let values = &[
        "-48553528129227068854128246488656/36214055281593228106459",
        "-22071489/37044301297",
        "255/542981",
        "9316/1012576106433925497",
        "-1664715/130027",
        "76455368490844554617/2980153660384121645",
        "80116271334543/93897057958828250698",
        "506747611059054706907067367541552559/130",
        "3325802517091555255861282341611/45079787663689587397296114",
        "2619/25010",
        "138524171232037/57",
        "-327554224783124113092/16778017845503481359721357185",
        "8702051241754639477/3724642828701481585672",
        "-1137052310060943639222669/10603652863466943873570746520875421824929696768184798584",
        "-273682512911798198562097/1410",
        "-19469521047938950017/8215425900326016767087299690926901706491788211",
        "-7582001356264120926498825684258819/24285551",
        "-858560112398451899225/359",
        "-649287191321577133137936520213146643215345718495684792790/7529",
        "-32944218852544/20849367593547441115716222574535971210798382851",
    ];
    let common_values = &[
        ("-1", 252),
        ("1", 230),
        ("1/2", 142),
        ("2", 127),
        ("3", 108),
        ("-2", 105),
        ("-3", 102),
        ("1/3", 101),
        ("-1/3", 94),
        ("-1/2", 91),
    ];
    let sample_median = (
        "-15668175538278751/5533317337571469966334347182083533508129248736899312345519768644568022\
        940401012659857427367497110049027936099115261493004685012826642110638291331832674475090998\
        50592299336680480845705995316128005975675",
        Some(
            "-52/545401271867319824074151466637070353812934757165212163414335060581060990607232033\
            27440868128671504241123333342761624433358162265204129676361103528857606120262656465655\
            47492357491745108315",
        ),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-5.683252966255455e252),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_nonzero_rationals_helper(
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
fn random_nonzero_rationals_fail_1() {
    random_nonzero_rationals(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_nonzero_rationals_fail_2() {
    random_nonzero_rationals(EXAMPLE_SEED, 2, 3);
}
