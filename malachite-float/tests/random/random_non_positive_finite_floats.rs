use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::random::random_non_positive_finite_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn random_non_positive_finite_floats_helper(
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
        random_non_positive_finite_floats(
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
    )
}

#[test]
fn test_random_non_positive_finite_floats() {
    // mean |sci_exponent| 1, mean precision 2, zero probability 1/10
    let values = &[
        "-4.0", "-0.5", "-0.4", "-0.0", "-2.0", "-1.0", "-0.5", "-1.0", "-0.4", "-0.4768", "-1.0",
        "-2.0e1", "-0.2", "-0.0", "-0.5", "-1.0", "-0.2", "-0.0", "-0.8", "-0.06",
    ];
    let values_hex = &[
        "-0x4.0#1",
        "-0x0.8#1",
        "-0x0.6#2",
        "-0x0.0",
        "-0x2.0#1",
        "-0x1.0#3",
        "-0x0.8#1",
        "-0x1.0#1",
        "-0x0.6#2",
        "-0x0.7a10#12",
        "-0x1.0#1",
        "-0x1.0E+1#1",
        "-0x0.4#1",
        "-0x0.0",
        "-0x0.8#1",
        "-0x1.0#2",
        "-0x0.4#1",
        "-0x0.0",
        "-0x0.c#2",
        "-0x0.1#1",
    ];
    let common_values = &[
        ("-0.5", 149586),
        ("-0.0", 100224),
        ("-1.0", 74929),
        ("-0.2", 74827),
        ("-0.5", 37902),
        ("-2.0", 37612),
        ("-0.1", 37602),
        ("-0.8", 37600),
        ("-0.2", 18934),
        ("-0.4", 18834),
    ];
    let common_values_hex = &[
        ("-0x0.8#1", 149586),
        ("-0x0.0", 100224),
        ("-0x1.0#1", 74929),
        ("-0x0.4#1", 74827),
        ("-0x0.8#2", 37902),
        ("-0x2.0#1", 37612),
        ("-0x0.2#1", 37602),
        ("-0x0.c#2", 37600),
        ("-0x0.4#2", 18934),
        ("-0x0.6#2", 18834),
    ];
    let sample_median = ("-0.5", None);
    let sample_median_hex = ("-0x0.8#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-4.3466859684673675),
        standard_deviation: NiceFloat(835.8920312692726),
        skewness: NiceFloat(-718.2460436055444),
        excess_kurtosis: NiceFloat(578520.0792964109),
    };
    random_non_positive_finite_floats_helper(
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

    // mean |sci_exponent| 1, mean precision 64, zero probability 1/10
    let values = &[
        "-4.14430807774440355061614115037031746862111513773654797362803064629233365606384697676260\
        9516719551",
        "-0.5",
        "-0.3325796124",
        "-0.0",
        "-2.11726214369233826954129204",
        "-1.19160392845269975891223055911503394268",
        "-0.518426906",
        "-1.2013513335896632820283496412",
        "-0.26333202931220086",
        "-0.2683",
        "-1.71088",
        "-30.850540057245481219",
        "-0.41683542187",
        "-0.0",
        "-0.5",
        "-1.2",
        "-0.329013682192",
        "-0.0",
        "-0.623",
        "-0.07",
    ];
    let values_hex = &[
        "-0x4.24f15fca75f9865c038bd55844fff40d21205eaa8642524cd23a9f1837937e9c89911612c6c4819d8\
        #324",
        "-0x0.8#1",
        "-0x0.5523effe8#32",
        "-0x0.0",
        "-0x2.1e04e45037aa8304b9742#86",
        "-0x1.310cf47e7d4ddcf7a0d6eca9afb1430#125",
        "-0x0.84b7a03#28",
        "-0x1.338bc2d0c609ffddf4593fa#93",
        "-0x0.4369ba55e29d85#55",
        "-0x0.44b0#12",
        "-0x1.b5fc#16",
        "-0x1e.d9bcfe41ceabd8e#65",
        "-0x0.6ab5b9e8c#33",
        "-0x0.0",
        "-0x0.8#2",
        "-0x1.4#4",
        "-0x0.543a3d9cf4#37",
        "-0x0.0",
        "-0x0.9f8#9",
        "-0x0.12#4",
    ];
    let common_values = &[
        ("-0.0", 100224),
        ("-0.5", 4643),
        ("-1.0", 2373),
        ("-0.2", 2353),
        ("-0.8", 2327),
        ("-0.5", 2294),
        ("-0.2", 1219),
        ("-2.0", 1204),
        ("-1.0", 1195),
        ("-0.4", 1191),
    ];
    let common_values_hex = &[
        ("-0x0.0", 100224),
        ("-0x0.8#1", 4643),
        ("-0x1.0#1", 2373),
        ("-0x0.4#1", 2353),
        ("-0x0.c#2", 2327),
        ("-0x0.8#2", 2294),
        ("-0x0.4#2", 1219),
        ("-0x2.0#1", 1204),
        ("-0x1.0#2", 1195),
        ("-0x0.6#2", 1191),
    ];
    let sample_median = (
        "-0.65634095145149450716439926007140230894810329586001204",
        Some("-0.656335890592859991893"),
    );
    let sample_median_hex = (
        "-0x0.a805f5e9827b377dc6bd240f77b2cd68b8ac0f0a6f54#175",
        Some("-0x0.a805a1014990ee3f6#67"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-5.255249017308858),
        standard_deviation: NiceFloat(844.3711946031435),
        skewness: NiceFloat(-594.3533044311691),
        excess_kurtosis: NiceFloat(387838.0226770939),
    };
    random_non_positive_finite_floats_helper(
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

    // mean |sci_exponent| 64, mean precision 2, zero probability 1/10
    let values = &[
        "-5.0e8", "-1.0e-6", "-1.0e1", "-0.0", "-2.0e-7", "-0.03", "-4.0e22", "-4.0e19", "-0.023",
        "-3.2e7", "-9.0e37", "-4.0e-9", "-1.0e15", "-0.0", "-1.0e11", "-7.0e-9", "-3.0e14", "-0.0",
        "-0.023", "-1.0e-34",
    ];
    let values_hex = &[
        "-0x2.0E+7#1",
        "-0x0.00001#1",
        "-0xc.0#2",
        "-0x0.0",
        "-0x4.0E-6#1",
        "-0x0.08#3",
        "-0x8.0E+18#1",
        "-0x2.0E+16#1",
        "-0x0.06#2",
        "-0x1.e84E+6#12",
        "-0x4.0E+31#1",
        "-0x1.0E-7#1",
        "-0x4.0E+12#1",
        "-0x0.0",
        "-0x2.0E+9#1",
        "-0x2.0E-7#2",
        "-0x1.0E+12#1",
        "-0x0.0",
        "-0x0.06#2",
        "-0x8.0E-29#1",
    ];
    let common_values = &[
        ("-0.0", 100224),
        ("-0.5", 3531),
        ("-1.0", 3503),
        ("-2.0", 3399),
        ("-0.2", 3381),
        ("-0.06", 3375),
        ("-0.1", 3369),
        ("-0.03", 3330),
        ("-4.0", 3283),
        ("-8.0", 3250),
    ];
    let common_values_hex = &[
        ("-0x0.0", 100224),
        ("-0x0.8#1", 3531),
        ("-0x1.0#1", 3503),
        ("-0x2.0#1", 3399),
        ("-0x0.4#1", 3381),
        ("-0x0.1#1", 3375),
        ("-0x0.2#1", 3369),
        ("-0x0.08#1", 3330),
        ("-0x4.0#1", 3283),
        ("-0x8.0#1", 3250),
    ];
    let sample_median = ("-0.0029", Some("-0.0029"));
    let sample_median_hex = ("-0x0.00c#4", Some("-0x0.00c#3"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.1187071843154994e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_non_positive_finite_floats_helper(
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

    // mean |sci_exponent| 64, mean precision 64, zero probability 1/10
    let values = &[
        "-556239614.326901209278831465330010369277037366613406531583358190459728647134822964160746\
        24468527615",
        "-1.0e-6",
        "-10.642547596",
        "-0.0",
        "-2.52397315942327769940053468e-7",
        "-0.037237622764146867466007204972344810709",
        "-3.91712295e22",
        "-44322041186676375483.90897365",
        "-0.0164582518320125536",
        "-1.8006e7",
        "-1.45545e38",
        "-7.1829510985979533799e-9",
        "-1.8772598506e15",
        "-0.0",
        "-1.0e11",
        "-9.0e-9",
        "-3.7043647413e14",
        "-0.0",
        "-0.01947",
        "-1.1e-34",
    ];
    let values_hex = &[
        "-0x21278afe.53afcc32e01c5eaac227ffa0690902f5543212926691d4f8c1bc9bf4e44c88b09636240cec\
        #324",
        "-0x0.00001#1",
        "-0xa.a47dffd#32",
        "-0x0.0",
        "-0x4.3c09c8a06f55060972e84E-6#86",
        "-0x0.098867a3f3ea6ee7bd06b7654d7d8a180#125",
        "-0x8.4b7a03E+18#28",
        "-0x2671785a18c13ffbb.e8b27f4#93",
        "-0x0.04369ba55e29d85#55",
        "-0x1.12cE+6#12",
        "-0x6.d7f0E+31#16",
        "-0x1.ed9bcfe41ceabd8eE-7#65",
        "-0x6.ab5b9e8cE+12#33",
        "-0x0.0",
        "-0x2.0E+9#2",
        "-0x2.8E-7#4",
        "-0x1.50e8f673dE+12#37",
        "-0x0.0",
        "-0x0.04fc#9",
        "-0x9.0E-29#4",
    ];
    let common_values = &[
        ("-0.0", 100224),
        ("-4.0", 129),
        ("-2.0", 120),
        ("-1.0", 107),
        ("-0.03", 107),
        ("-5.0e2", 106),
        ("-2.0e1", 105),
        ("-2.0e3", 104),
        ("-0.2", 102),
        ("-3.0e2", 102),
    ];
    let common_values_hex = &[
        ("-0x0.0", 100224),
        ("-0x4.0#1", 129),
        ("-0x2.0#1", 120),
        ("-0x1.0#1", 107),
        ("-0x0.08#1", 107),
        ("-0x2.0E+2#1", 106),
        ("-0x1.0E+1#1", 105),
        ("-0x8.0E+2#1", 104),
        ("-0x0.4#1", 102),
        ("-0x1.0E+2#1", 102),
    ];
    let sample_median = ("-0.0037239916472", Some("-0.003723741"));
    let sample_median_hex = ("-0x0.00f40e3655e#35", Some("-0x0.00f40a00#21"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.8473451262482141e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_non_positive_finite_floats_helper(
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
fn random_non_positive_finite_floats_fail_1() {
    random_non_positive_finite_floats(EXAMPLE_SEED, 1, 0, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_non_positive_finite_floats_fail_2() {
    random_non_positive_finite_floats(EXAMPLE_SEED, 0, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_non_positive_finite_floats_fail_3() {
    random_non_positive_finite_floats(EXAMPLE_SEED, 1, 1, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn random_non_positive_finite_floats_fail_4() {
    random_non_positive_finite_floats(EXAMPLE_SEED, 1, 1, 1, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_non_positive_finite_floats_fail_5() {
    random_non_positive_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0);
}

#[test]
#[should_panic]
fn random_non_positive_finite_floats_fail_6() {
    random_non_positive_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 2, 1);
}
