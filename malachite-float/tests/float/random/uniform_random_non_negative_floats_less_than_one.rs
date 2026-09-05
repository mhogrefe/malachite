// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::Float;
use malachite_float::float::random::uniform_random_non_negative_floats_less_than_one;
use malachite_float::test_util::float::random::{
    MalachiteRandGen, random_floats_helper_helper, random_floats_helper_helper_no_common_values,
};
use malachite_q::Rational;

fn uniform_random_non_negative_floats_less_than_one_helper(
    prec: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper(
        uniform_random_non_negative_floats_less_than_one(EXAMPLE_SEED, prec),
        expected_values,
        expected_values_hex,
        expected_common_values,
        expected_common_values_hex,
        expected_median,
        expected_median_hex,
        expected_moment_stats,
    );
}

fn uniform_random_non_negative_floats_less_than_one_helper_no_common_values(
    prec: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper_no_common_values(
        uniform_random_non_negative_floats_less_than_one(EXAMPLE_SEED, prec),
        expected_values,
        expected_values_hex,
        expected_median,
        expected_median_hex,
        expected_moment_stats,
    );
}

// Observed branch coverage: the zero and normal arms of the iterator are each hit millions of times
// by these cases (precision 1 gives ~50% zeros). The NaN arm (exponent below the minimum) is
// unreachable by sampling; see the comment in the iterator.
#[test]
fn test_uniform_random_non_negative_floats_less_than_one() {
    // precision 1
    let values = &[
        "0.50", "0.50", "0.50", "0.50", "0.50", "0.50", "0.0", "0.0", "0.0", "0.50", "0.0", "0.50",
        "0.0", "0.50", "0.0", "0.0", "0.50", "0.50", "0.50", "0.50",
    ];
    let values_hex = &[
        "0x0.8#1", "0x0.8#1", "0x0.8#1", "0x0.8#1", "0x0.8#1", "0x0.8#1", "0x0.0", "0x0.0",
        "0x0.0", "0x0.8#1", "0x0.0", "0x0.8#1", "0x0.0", "0x0.8#1", "0x0.0", "0x0.0", "0x0.8#1",
        "0x0.8#1", "0x0.8#1", "0x0.8#1",
    ];
    let common_values = &[("0.0", 500399), ("0.50", 499601)];
    let common_values_hex = &[("0x0.0", 500399), ("0x0.8#1", 499601)];
    let sample_median = ("0.0", None);
    let sample_median_hex = ("0x0.0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.24980049999999746),
        standard_deviation: NiceFloat(0.2500000453995434),
        skewness: NiceFloat(0.0015960005081696526),
        excess_kurtosis: NiceFloat(-1.9999974527824076),
    };
    uniform_random_non_negative_floats_less_than_one_helper(
        1,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 2
    let values = &[
        "0.25", "0.75", "0.25", "0.25", "0.75", "0.25", "0.50", "0.0", "0.50", "0.75", "0.0",
        "0.75", "0.50", "0.25", "0.50", "0.50", "0.75", "0.25", "0.75", "0.75",
    ];
    let values_hex = &[
        "0x0.4#2", "0x0.c#2", "0x0.4#2", "0x0.4#2", "0x0.c#2", "0x0.4#2", "0x0.8#2", "0x0.0",
        "0x0.8#2", "0x0.c#2", "0x0.0", "0x0.c#2", "0x0.8#2", "0x0.4#2", "0x0.8#2", "0x0.8#2",
        "0x0.c#2", "0x0.4#2", "0x0.c#2", "0x0.c#2",
    ];
    let common_values = &[("0.50", 250959), ("0.75", 250309), ("0.0", 249440), ("0.25", 249292)];
    let common_values_hex =
        &[("0x0.8#2", 250959), ("0x0.c#2", 250309), ("0x0.0", 249440), ("0x0.4#2", 249292)];
    let sample_median = ("0.50", None);
    let sample_median_hex = ("0x0.8#2", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.3755342499999979),
        standard_deviation: NiceFloat(0.2794519952878359),
        skewness: NiceFloat(-0.0034862794057751857),
        excess_kurtosis: NiceFloat(-1.3594690811609154),
    };
    uniform_random_non_negative_floats_less_than_one_helper(
        2,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 10
    let values = &[
        "0.86035", "0.084961", "0.090820", "0.61426", "0.50684", "0.97754", "0.61133", "0.35156",
        "0.23633", "0.47949", "0.082031", "0.15137", "0.91992", "0.34082", "0.021484", "0.20898",
        "0.72949", "0.62598", "0.11230", "0.13184",
    ];
    let values_hex = &[
        "0x0.dc4#10",
        "0x0.15c0#10",
        "0x0.1740#10",
        "0x0.9d4#10",
        "0x0.81c#10",
        "0x0.fa4#10",
        "0x0.9c8#10",
        "0x0.5a0#10",
        "0x0.3c8#10",
        "0x0.7ac#10",
        "0x0.1500#10",
        "0x0.26c#10",
        "0x0.eb8#10",
        "0x0.574#10",
        "0x0.0580#10",
        "0x0.358#10",
        "0x0.bac#10",
        "0x0.a04#10",
        "0x0.1cc0#10",
        "0x0.21c#10",
    ];
    let common_values = &[
        ("0.031250", 1067),
        ("0.66602", 1065),
        ("0.53320", 1061),
        ("0.83887", 1060),
        ("0.15332", 1058),
        ("0.36426", 1056),
        ("0.12695", 1053),
        ("0.32910", 1053),
        ("0.54883", 1053),
        ("0.28906", 1050),
    ];
    let common_values_hex = &[
        ("0x0.0800#10", 1067),
        ("0x0.aa8#10", 1065),
        ("0x0.888#10", 1061),
        ("0x0.d6c#10", 1060),
        ("0x0.274#10", 1058),
        ("0x0.5d4#10", 1056),
        ("0x0.208#10", 1053),
        ("0x0.544#10", 1053),
        ("0x0.8c8#10", 1053),
        ("0x0.4a0#10", 1050),
    ];
    let sample_median = ("0.49902", None);
    let sample_median_hex = ("0x0.7fc#10", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.4993825986327951),
        standard_deviation: NiceFloat(0.2886247998431472),
        skewness: NiceFloat(-0.00030153835683919454),
        excess_kurtosis: NiceFloat(-1.2006761472034735),
    };
    uniform_random_non_negative_floats_less_than_one_helper(
        10,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 64
    let values = &[
        "0.631482296721763473066",
        "0.931047602632525177766",
        "0.788476332428749709426",
        "0.131571387704865017901",
        "0.501731829849010852287",
        "0.951722267192218472772",
        "0.721201786763443242830",
        "0.271171265511285306069",
        "0.739225979791356637501",
        "0.698140786967770792220",
        "0.735838880221276530171",
        "0.0820429003057728895780",
        "0.134710633829075494545",
        "0.708650816283203440805",
        "0.235540684453755478993",
        "0.176334585027478297158",
        "0.570325674501846831526",
        "0.671112711767514559579",
        "0.360763574055944453284",
        "0.569524256359344900993",
    ];
    let values_hex = &[
        "0x0.a1a8d2e46c45ef71#64",
        "0x0.ee5922bc536e2057#64",
        "0x0.c9d995bd7395c85d#64",
        "0x0.21aea997481f92750#64",
        "0x0.80717f4890512607#64",
        "0x0.f3a4120c772e6be9#64",
        "0x0.b8a0ae27f73bae72#64",
        "0x0.456b7ae4fc6425680#64",
        "0x0.bd3de9ef8eb3f8f2#64",
        "0x0.b2b95ac807c055eb#64",
        "0x0.bc5fefd5acf70054#64",
        "0x0.1500c375aeaa189b0#64",
        "0x0.227c6566b824bbae0#64",
        "0x0.b56a23d038560d5d#64",
        "0x0.3c4c64f09b38cc160#64",
        "0x0.2d24436bd8c4a4d60#64",
        "0x0.9200dd080df882eb#64",
        "0x0.abce0aecf8aa5a81#64",
        "0x0.5c5b00682889fc730#64",
        "0x0.91cc577757944887#64",
    ];
    let sample_median = ("0.500294003891628108840", Some("0.500294250109741193207"));
    let sample_median_hex = ("0x0.801344911974dd3a#64", Some("0x0.801348b2992282a5#64"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.49998805918205524),
        standard_deviation: NiceFloat(0.2885853354232703),
        skewness: NiceFloat(-0.0007960342667556958),
        excess_kurtosis: NiceFloat(-1.2005689675667335),
    };
    uniform_random_non_negative_floats_less_than_one_helper_no_common_values(
        64,
        values,
        values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 100
    let values = &[
        "0.77036869653430013999927561196187",
        "0.45510823451024009165745155346559",
        "0.77909700539751403018961734563762",
        "0.31161894429127571910450123312810",
        "0.50189240992388260172143573636278",
        "0.35514268505701095098951593580089",
        "0.013753940775273231466840304995208",
        "0.74042190923116199969741509236327",
        "0.81070933679541231939950152928252",
        "0.45888164836448797103448224447906",
        "0.38670090064059910645448915684163",
        "0.45725756839342449958917991208034",
        "0.71862258492781138976433494642284",
        "0.91628133585990678021657119655330",
        "0.66851851677904673566694464055424",
        "0.81577780675121738231842074949812",
        "0.13968303927598752283293768687548",
        "0.55579117059312266548257895276120",
        "0.87511018180867291597108642315513",
        "0.11672000970897449493753465716670",
    ];
    let values_hex = &[
        "0x0.c536e2057a1a8d2e46c45ef71#100",
        "0x0.7481f9275c9d995bd7395c85d0#100",
        "0x0.c772e6be980717f4890512607#100",
        "0x0.4fc642568b8a0ae27f73bae720#100",
        "0x0.807c055ebbd3de9ef8eb3f8f2#100",
        "0x0.5aeaa189bbc5fefd5acf700540#100",
        "0x0.038560d5d227c6566b824bbae00#100",
        "0x0.bd8c4a4d63c4c64f09b38cc16#100",
        "0x0.cf8aa5a819200dd080df882eb#100",
        "0x0.7579448875c5b00682889fc730#100",
        "0x0.62fed48995ca55571ddbe291d0#100",
        "0x0.750ed4fe19265aa68d2dd10b80#100",
        "0x0.b7f7a6546e931fd76d406dc26#100",
        "0x0.ea9169e374180782dcd24dc86#100",
        "0x0.ab24078e562078e7a923edb67#100",
        "0x0.d0d6d078cc90f195e4e7feb9d#100",
        "0x0.23c244857f0a3c3e4958f8f070#100",
        "0x0.8e4854851a5f702b4e3c4016f#100",
        "0x0.e007388b43ce25a05d9c1a417#100",
        "0x0.1de15cd07d25235ee7da487580#100",
    ];
    let sample_median = (
        "0.49984019687923860625587865593521",
        Some("0.49984027231390296887935847861644"),
    );
    let sample_median_hex = (
        "0x0.7ff586f2d2916584240d94b430#100",
        Some("0x0.7ff58836cfdbca8ba2766a6330#100"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.5000146785339863),
        standard_deviation: NiceFloat(0.288717520061518),
        skewness: NiceFloat(0.0003500375100175877),
        excess_kurtosis: NiceFloat(-1.200435788949654),
    };
    uniform_random_non_negative_floats_less_than_one_helper_no_common_values(
        100,
        values,
        values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );
}

#[test]
fn uniform_random_non_negative_floats_less_than_one_properties() {
    for prec in [1, 3, 10, 63, 64, 65, 100] {
        for x in uniform_random_non_negative_floats_less_than_one(EXAMPLE_SEED, prec).take(10000) {
            assert!(x.is_valid());
            // - the value is in [0, 1)
            assert!(x >= 0u32);
            assert!(x < 1u32);
            if x == 0u32 {
                // - a zero draw is a positive zero
                assert!(!x.is_sign_negative());
            } else {
                // - every nonzero output has precision `prec`
                assert_eq!(x.get_prec(), Some(prec));
            }
            // - the value is k / 2^prec for an integer k
            let k = Rational::exact_from(&x) * Rational::power_of_2(prec);
            assert_eq!(k.denominator_ref(), &1u32);
        }
    }
}

#[test]
#[should_panic]
fn uniform_random_non_negative_floats_less_than_one_fail() {
    uniform_random_non_negative_floats_less_than_one(EXAMPLE_SEED, 0);
}

#[test]
fn test_uniform_random_non_negative_floats_less_than_one_vs_rug() {
    use malachite_base::num::random::random_primitive_ints;
    use malachite_float::ComparableFloat;
    for prec in [1u64, 2, 10, 31, 32, 33, 63, 64, 65, 100, 128, 200] {
        for i in 0..100u32 {
            let seed = EXAMPLE_SEED.fork(&i.to_string());
            let ours = uniform_random_non_negative_floats_less_than_one(seed, prec)
                .next()
                .unwrap();
            let mut bit_source = MalachiteRandGen::new(random_primitive_ints(seed));
            let mut state = rug::rand::RandState::new_custom(&mut bit_source);
            let theirs =
                rug::Float::with_val(u32::exact_from(prec), rug::Float::random_bits(&mut state));
            assert_eq!(
                ComparableFloat(Float::from(&theirs)),
                ComparableFloat(ours),
                "prec {prec} seed fork {i}"
            );
        }
    }
}
