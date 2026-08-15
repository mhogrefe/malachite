// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::float::random::{normal_random_floats, uniform_mod_from_u64s};
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::float::random::{
    MalachiteRandGen, random_floats_helper_helper, random_floats_helper_helper_no_common_values,
};
use malachite_float::{ComparableFloat, Float};

#[test]
fn test_normal_random_floats_vs_rug() {
    for rm in [Floor, Ceiling, Down, Up, Nearest] {
        let rug_rm = rug_round_try_from_rounding_mode(rm).unwrap();
        for prec in [1u64, 2, 10, 31, 32, 33, 63, 64, 65, 100] {
            let mut ours = normal_random_floats(EXAMPLE_SEED, prec, rm);
            let mut bit_source = MalachiteRandGen::new(random_primitive_ints(EXAMPLE_SEED));
            let mut state = rug::rand::RandState::new_custom(&mut bit_source);
            for i in 0..50u32 {
                let x = ours.next().unwrap();
                let (theirs, o) = rug::Float::with_val_round(
                    u32::exact_from(prec),
                    rug::Float::random_normal(&mut state),
                    rug_rm,
                );
                assert_ne!(o, core::cmp::Ordering::Equal);
                assert_eq!(
                    ComparableFloat(Float::from(&theirs)),
                    ComparableFloat(x),
                    "rm {rm} prec {prec} output {i}"
                );
            }
        }
    }
}

fn normal_random_floats_helper(
    prec: u64,
    rm: RoundingMode,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper(
        normal_random_floats(EXAMPLE_SEED, prec, rm),
        expected_values,
        expected_values_hex,
        expected_common_values,
        expected_common_values_hex,
        expected_median,
        expected_median_hex,
        expected_moment_stats,
    );
}

fn normal_random_floats_helper_no_common_values(
    prec: u64,
    rm: RoundingMode,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper_no_common_values(
        normal_random_floats(EXAMPLE_SEED, prec, rm),
        expected_values,
        expected_values_hex,
        expected_median,
        expected_median_hex,
        expected_moment_stats,
    );
}

#[test]
fn test_normal_random_floats() {
    // precision 1, rounding mode Floor
    let values = &[
        "-0.50", "-4.0", "-4.0", "-1.0", "0.12", "-1.0", "-1.0", "-0.25", "1.0", "-0.062", "-1.0",
        "-0.25", "0.50", "-1.0", "0.50", "1.0", "0.50", "1.0", "0.50", "-0.12",
    ];
    let values_hex = &[
        "-0x0.8#1", "-0x4.0#1", "-0x4.0#1", "-0x1.0#1", "0x0.2#1", "-0x1.0#1", "-0x1.0#1",
        "-0x0.4#1", "0x1.0#1", "-0x0.1#1", "-0x1.0#1", "-0x0.4#1", "0x0.8#1", "-0x1.0#1",
        "0x0.8#1", "0x1.0#1", "0x0.8#1", "0x1.0#1", "0x0.8#1", "-0x0.2#1",
    ];
    let common_values = &[
        ("0.50", 150221),
        ("-1.0", 150089),
        ("1.0", 135725),
        ("-2.0", 135591),
        ("-0.50", 92674),
        ("0.25", 92327),
        ("-0.25", 49273),
        ("0.12", 48870),
        ("0.062", 25018),
        ("-0.12", 24979),
    ];
    let common_values_hex = &[
        ("0x0.8#1", 150221),
        ("-0x1.0#1", 150089),
        ("0x1.0#1", 135725),
        ("-0x2.0#1", 135591),
        ("-0x0.8#1", 92674),
        ("0x0.4#1", 92327),
        ("-0x0.4#1", 49273),
        ("0x0.2#1", 48870),
        ("0x0.1#1", 25018),
        ("-0x0.2#1", 24979),
    ];
    let sample_median = ("-0.00049", None);
    let sample_median_hex = ("-0x0.002#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.2874899879455224),
        standard_deviation: NiceFloat(1.129022181351112),
        skewness: NiceFloat(-0.8750547365221832),
        excess_kurtosis: NiceFloat(1.1456896152321088),
    };
    normal_random_floats_helper(
        1,
        Floor,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 1, rounding mode Nearest
    let values = &[
        "-0.50", "-2.0", "-2.0", "-1.0", "0.25", "-0.50", "-1.0", "-0.12", "1.0", "-0.031", "-1.0",
        "-0.25", "0.50", "-0.50", "0.50", "1.0", "0.50", "1.0", "1.0", "-0.062",
    ];
    let values_hex = &[
        "-0x0.8#1",
        "-0x2.0#1",
        "-0x2.0#1",
        "-0x1.0#1",
        "0x0.4#1",
        "-0x0.8#1",
        "-0x1.0#1",
        "-0x0.2#1",
        "0x1.0#1",
        "-0x0.08#1",
        "-0x1.0#1",
        "-0x0.4#1",
        "0x0.8#1",
        "-0x0.8#1",
        "0x0.8#1",
        "0x1.0#1",
        "0x0.8#1",
        "0x1.0#1",
        "0x1.0#1",
        "-0x0.1#1",
    ];
    let common_values = &[
        ("1.0", 159154),
        ("-1.0", 159022),
        ("-0.50", 127727),
        ("0.50", 127422),
        ("-0.25", 71843),
        ("0.25", 71761),
        ("2.0", 65718),
        ("-2.0", 65646),
        ("-0.12", 37221),
        ("0.12", 37063),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 159154),
        ("-0x1.0#1", 159022),
        ("-0x0.8#1", 127727),
        ("0x0.8#1", 127422),
        ("-0x0.4#1", 71843),
        ("0x0.4#1", 71761),
        ("0x2.0#1", 65718),
        ("-0x2.0#1", 65646),
        ("-0x0.2#1", 37221),
        ("0x0.2#1", 37063),
    ];
    let sample_median = ("-0.00049", None);
    let sample_median_hex = ("-0x0.002#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.000019309259444502124),
        standard_deviation: NiceFloat(0.9808707667723517),
        skewness: NiceFloat(-0.0004359998807234094),
        excess_kurtosis: NiceFloat(0.3994211736535438),
    };
    normal_random_floats_helper(
        1,
        Nearest,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 2, rounding mode Nearest
    let values = &[
        "-0.50", "-2.0", "-2.0", "-0.75", "0.25", "-0.50", "-1.0", "-0.12", "1.5", "-0.047",
        "-0.75", "-0.19", "0.50", "-0.50", "0.50", "1.0", "0.50", "1.0", "1.0", "-0.062",
    ];
    let values_hex = &[
        "-0x0.8#2",
        "-0x2.0#2",
        "-0x2.0#2",
        "-0x0.c#2",
        "0x0.4#2",
        "-0x0.8#2",
        "-0x1.0#2",
        "-0x0.2#2",
        "0x1.8#2",
        "-0x0.0c#2",
        "-0x0.c#2",
        "-0x0.3#2",
        "0x0.8#2",
        "-0x0.8#2",
        "0x0.8#2",
        "0x1.0#2",
        "0x0.8#2",
        "0x1.0#2",
        "0x1.0#2",
        "-0x0.10#2",
    ];
    let common_values = &[
        ("-1.0", 84617),
        ("1.0", 84606),
        ("0.75", 75623),
        ("-0.75", 75402),
        ("1.5", 65833),
        ("-1.5", 65481),
        ("-0.50", 65216),
        ("0.50", 64951),
        ("-0.38", 46245),
        ("0.38", 46069),
    ];
    let common_values_hex = &[
        ("-0x1.0#2", 84617),
        ("0x1.0#2", 84606),
        ("0x0.c#2", 75623),
        ("-0x0.c#2", 75402),
        ("0x1.8#2", 65833),
        ("-0x1.8#2", 65481),
        ("-0x0.8#2", 65216),
        ("0x0.8#2", 64951),
        ("-0x0.6#2", 46245),
        ("0x0.6#2", 46069),
    ];
    let sample_median = ("-0.00037", None);
    let sample_median_hex = ("-0x0.0018#2", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.000037750405833101116),
        standard_deviation: NiceFloat(0.994129738627467),
        skewness: NiceFloat(-0.005766204939878577),
        excess_kurtosis: NiceFloat(0.1331247073597135),
    };
    normal_random_floats_helper(
        2,
        Nearest,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 10, rounding mode Floor
    let values = &[
        "-0.45166",
        "-2.2734",
        "-2.1602",
        "-0.78613",
        "0.23486",
        "-0.61328",
        "-0.91895",
        "-0.13696",
        "1.2871",
        "-0.045227",
        "-0.77051",
        "-0.21143",
        "0.61523",
        "-0.58691",
        "0.57520",
        "1.0098",
        "0.58008",
        "1.0176",
        "0.89355",
        "-0.069092",
    ];
    let values_hex = &[
        "-0x0.73a#10",
        "-0x2.46#10",
        "-0x2.29#10",
        "-0x0.c94#10",
        "0x0.3c2#10",
        "-0x0.9d0#10",
        "-0x0.eb4#10",
        "-0x0.231#10",
        "0x1.498#10",
        "-0x0.0b94#10",
        "-0x0.c54#10",
        "-0x0.362#10",
        "0x0.9d8#10",
        "-0x0.964#10",
        "0x0.934#10",
        "0x1.028#10",
        "0x0.948#10",
        "0x1.048#10",
        "0x0.e4c#10",
        "-0x0.11b0#10",
    ];
    let common_values = &[
        ("-1.0020", 501),
        ("-1.0430", 497),
        ("1.0059", 491),
        ("-1.0117", 489),
        ("1.0527", 488),
        ("1.0488", 486),
        ("-1.0352", 485),
        ("-1.0371", 485),
        ("-1.0391", 482),
        ("-1.0195", 480),
    ];
    let common_values_hex = &[
        ("-0x1.008#10", 501),
        ("-0x1.0b0#10", 497),
        ("0x1.018#10", 491),
        ("-0x1.030#10", 489),
        ("0x1.0d8#10", 488),
        ("0x1.0c8#10", 486),
        ("-0x1.090#10", 485),
        ("-0x1.098#10", 485),
        ("-0x1.0a0#10", 482),
        ("-0x1.050#10", 480),
    ];
    let sample_median = ("-0.00037956", Some("-0.00037909"));
    let sample_median_hex = ("-0x0.0018e0#10", Some("-0x0.0018d8#10"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.0003462511047368072),
        standard_deviation: NiceFloat(1.00025020037424),
        skewness: NiceFloat(-0.004296836680590008),
        excess_kurtosis: NiceFloat(0.0019571619561973286),
    };
    normal_random_floats_helper(
        10,
        Floor,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 10, rounding mode Nearest
    let values = &[
        "-0.45166",
        "-2.2695",
        "-2.1602",
        "-0.78516",
        "0.23486",
        "-0.61230",
        "-0.91797",
        "-0.13672",
        "1.2891",
        "-0.045227",
        "-0.77051",
        "-0.21143",
        "0.61621",
        "-0.58594",
        "0.57520",
        "1.0117",
        "0.58008",
        "1.0195",
        "0.89453",
        "-0.069092",
    ];
    let values_hex = &[
        "-0x0.73a#10",
        "-0x2.45#10",
        "-0x2.29#10",
        "-0x0.c90#10",
        "0x0.3c2#10",
        "-0x0.9cc#10",
        "-0x0.eb0#10",
        "-0x0.230#10",
        "0x1.4a0#10",
        "-0x0.0b94#10",
        "-0x0.c54#10",
        "-0x0.362#10",
        "0x0.9dc#10",
        "-0x0.960#10",
        "0x0.934#10",
        "0x1.030#10",
        "0x0.948#10",
        "0x1.050#10",
        "0x0.e50#10",
        "-0x0.11b0#10",
    ];
    let common_values = &[
        ("-1.0625", 516),
        ("1.0547", 515),
        ("-1.0117", 507),
        ("1.0078", 503),
        ("-1.0312", 502),
        ("-1.0020", 492),
        ("1.0137", 488),
        ("-1.0371", 487),
        ("-1.0430", 487),
        ("1.0176", 486),
    ];
    let common_values_hex = &[
        ("-0x1.100#10", 516),
        ("0x1.0e0#10", 515),
        ("-0x1.030#10", 507),
        ("0x1.020#10", 503),
        ("-0x1.080#10", 502),
        ("-0x1.008#10", 492),
        ("0x1.038#10", 488),
        ("-0x1.098#10", 487),
        ("-0x1.0b0#10", 487),
        ("0x1.048#10", 486),
    ];
    let sample_median = ("-0.00037956", Some("-0.00037909"));
    let sample_median_hex = ("-0x0.0018e0#10", Some("-0x0.0018d8#10"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.00021611807208891598),
        standard_deviation: NiceFloat(1.0002478152283445),
        skewness: NiceFloat(-0.002600868079023604),
        excess_kurtosis: NiceFloat(0.0019187467105070688),
    };
    normal_random_floats_helper(
        10,
        Nearest,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 64, rounding mode Nearest
    let values = &[
        "-0.451504252183151288832",
        "-2.27117126544428541953",
        "-2.15835550144924383383",
        "-0.785386644000701374644",
        "-0.200372054470627854818",
        "-0.136765099604330640651",
        "0.938217172547427922082",
        "-0.770200730592049569981",
        "0.283338642640889548852",
        "0.894472074710071516366",
        "-0.344896427655279184585",
        "1.89148687495895858224",
        "1.27623382926922098943",
        "-1.17295582976994172751",
        "-0.00509947749612093473502",
        "1.30843057062330567122",
        "-0.672805198362238965088",
        "2.32186220310629492527",
        "-1.74701701933510028942",
        "0.154644118250563320454",
    ];
    let values_hex = &[
        "-0x0.7395c85d21aea9978#64",
        "-0x2.456b7ae4b2b95ac8#64",
        "-0x2.2889fc73ddbe291c#64",
        "-0x0.c90f195eb2bdb1bd#64",
        "-0x0.334b953cfbd245588#64",
        "-0x0.2303099e08e783e40#64",
        "0x0.f02f0028a304cbd4#64",
        "-0x0.c52be0053f87638f#64",
        "0x0.4888e19bd5ed44d08#64",
        "0x0.e4fc1f3410a588a4#64",
        "-0x0.584b21dd496229130#64",
        "0x1.e4387bdcc3105d52#64",
        "0x1.46b7429ec2994dc0#64",
        "-0x1.2c46d55083b2880c#64",
        "-0x0.014e33091291278bd0#64",
        "0x1.4ef54e4de9e94df0#64",
        "-0x0.ac3cf6238b6b80c8#64",
        "0x2.52658fb428f8cb9c#64",
        "-0x1.bf3c81e39983044e#64",
        "0x0.2796c1c667a93e3c8#64",
    ];
    let sample_median = (
        "0.00183368940488509720181",
        Some("0.00183659798113458680264"),
    );
    let sample_median_hex = (
        "0x0.00782c3406669c01558#64",
        Some("0x0.00785d0043ceb215fb8#64"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.000556100536541963),
        standard_deviation: NiceFloat(0.9997333532630669),
        skewness: NiceFloat(-0.005667883734186153),
        excess_kurtosis: NiceFloat(-0.0006175255177205408),
    };
    normal_random_floats_helper_no_common_values(
        64,
        Nearest,
        values,
        values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 100, rounding mode Ceiling
    let values = &[
        "-0.45150425228377297254069995165490",
        "1.7212017865456357967998184327843",
        "-2.1583555012911591702721979709495",
        "-0.78538664397416212179058205169893",
        "1.6483003320176533677006994037746",
        "-0.75944985856164669331574790469235",
        "-0.13676509969761770633374888946771",
        "-1.2595940594301378193502605292710",
        "-0.77020073065319092689126436621384",
        "-0.042561029366083685099861927158142",
        "0.61611931105232143926122974546631",
        "1.1938020891263303749975625383734",
        "0.89447207474025712510172166366761",
        "0.19047492193293170601806926175064",
        "1.2762338292710201939714974679701",
        "-0.51444292335452243927945430917084",
        "1.3084305704635369746758188064370",
        "1.1030581945968969811351625668542",
        "-0.010084011766740794320085528609767",
        "1.4023526046430579424736146580938",
    ];
    let values_hex = &[
        "-0x0.7395c85d9051260721aea99748#100",
        "0x1.b8a0ae2707c055ebbd3de9efa#100",
        "-0x2.2889fc732fed48995ca55571c#100",
        "-0x0.c90f195e958f8f07b2bdb1bd0#100",
        "0x1.a5f702b4007388b43ce25a05e#100",
        "-0x0.c26b4e51795e7f0774a95c1b3#100",
        "-0x0.2303099e6f797d6608e783e3e0#100",
        "-0x1.4274c19b7d02b1390b935f100#100",
        "-0x0.c52be00582c126a23f87638ef#100",
        "-0x0.0ae54795361ff38c80d203773a#100",
        "0x0.9db9fec3675d98d3638efe19e#100",
        "0x1.319d0382b1ae521abffa9e6be#100",
        "0x0.e4fc1f3431d606fb10a588a48#100",
        "0x0.30c2f6e868fbf8c02ae6ba36e0#100",
        "0x1.46b7429ec493bc19c2994dbf4#100",
        "-0x0.83b2880b775ffe442136c0a08#100",
        "0x1.4ef54e4d3a3e6a0fe9e94df10#100",
        "0x1.1a62059760e3d4ef77053be36#100",
        "-0x0.0294dda4c029589cff07353fb04#100",
        "0x1.6700948e670083f9bf3c81e3a#100",
    ];
    let sample_median = (
        "0.00092374581850171232891756828977884",
        Some("0.00092481513255710251246088930283403"),
    );
    let sample_median_hex = (
        "0x0.003c89e2148d55d8fb6b0e9f8628#100",
        Some("0x0.003c9bd2bfca2c342d7717cdab50#100"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0010569543005151884),
        standard_deviation: NiceFloat(1.000429128447009),
        skewness: NiceFloat(-0.0024878552969430355),
        excess_kurtosis: NiceFloat(-0.008755675468828716),
    };
    normal_random_floats_helper_no_common_values(
        100,
        Ceiling,
        values,
        values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );
}

#[test]
fn normal_random_floats_properties() {
    for rm in [Floor, Ceiling, Down, Up, Nearest] {
        for prec in [1, 3, 10, 63, 64, 65, 100] {
            let mut seen_positive = false;
            let mut seen_negative = false;
            for x in normal_random_floats(EXAMPLE_SEED, prec, rm).take(1000) {
                assert!(x.is_valid());
                // - the result is nonzero; the value conversion always sets a trailing bit, and
                //   rounding to zero would require underflow, which is unreachable by sampling
                assert!(x != 0u32);
                // - every output has precision `prec`
                assert_eq!(x.get_prec(), Some(prec));
                if x > 0u32 {
                    seen_positive = true;
                } else {
                    seen_negative = true;
                }
            }
            // - the sign is a fair coin
            assert!(seen_positive && seen_negative);
        }
    }
}

// The gmp_urandomm_ui replica, tested directly against GMP through rug's `below`, including the
// degenerate-stream fallback after 80 rejections, which sampling cannot reach.
#[test]
fn test_uniform_mod_vs_gmp() {
    // - 3 bits are drawn for a modulus of 6; the value 5 is accepted immediately
    let s: Vec<u32> = vec![5];
    assert_eq!(run_uniform_mod_case(&s, 6), 5);
    // - 7 is rejected twice, then 3 is accepted
    let s: Vec<u32> = vec![7, 7, 3];
    assert_eq!(run_uniform_mod_case(&s, 6), 3);
    // - a modulus of 8 is a power of 2, so only 3 bits are drawn and no rejection can occur
    let s: Vec<u32> = vec![7];
    assert_eq!(run_uniform_mod_case(&s, 8), 7);
    // - 80 consecutive rejections trigger GMP's degenerate-stream fallback of ret - n
    let s: Vec<u32> = vec![7; 80];
    assert_eq!(run_uniform_mod_case(&s, 6), 1);
}

fn run_uniform_mod_case(u32s: &[u32], m: u64) -> u64 {
    let ours = uniform_mod_from_u64s(pack_u32s(u32s), m);
    let mut bit_source = MalachiteRandGen::new(pack_u32s(u32s));
    let mut state = rug::rand::RandState::new_custom(&mut bit_source);
    let theirs = state.below(u32::exact_from(m));
    assert_eq!(u64::from(theirs), ours, "m {m}");
    ours
}

fn pack_u32s(u32s: &[u32]) -> impl Iterator<Item = u64> + Send + Sync + use<'_> {
    let mut v: Vec<u32> = u32s.to_vec();
    v.extend_from_slice(&[0; 4]);
    v.chunks(2)
        .map(|c| u64::from(c[0]) | (u64::from(*c.get(1).unwrap_or(&0)) << 32))
        .collect::<Vec<u64>>()
        .into_iter()
}

#[test]
#[should_panic]
fn normal_random_floats_fail_1() {
    normal_random_floats(EXAMPLE_SEED, 0, Nearest);
}

#[test]
#[should_panic]
fn normal_random_floats_fail_2() {
    normal_random_floats(EXAMPLE_SEED, 10, Exact);
}
