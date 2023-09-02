use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::random::random_nonzero_finite_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn random_nonzero_finite_floats_helper(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper(
        random_nonzero_finite_floats(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
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
fn test_random_nonzero_finite_floats() {
    // mean |sci_exponent| 1, mean precision 2
    let values = &[
        "-4.0", "-0.5", "-0.4", "2.0", "-1.0", "0.5", "-1.0", "-0.4", "-0.4768", "1.0", "2.0e1",
        "0.2", "-0.5", "-1.0", "0.2", "-0.8", "-0.06", "-0.5", "-0.023", "-0.5",
    ];
    let values_hex = &[
        "-0x4.0#1",
        "-0x0.8#1",
        "-0x0.6#2",
        "0x2.0#1",
        "-0x1.0#3",
        "0x0.8#1",
        "-0x1.0#1",
        "-0x0.6#2",
        "-0x0.7a10#12",
        "0x1.0#1",
        "0x1.0E+1#1",
        "0x0.4#1",
        "-0x0.8#1",
        "-0x1.0#2",
        "0x0.4#1",
        "-0x0.c#2",
        "-0x0.1#1",
        "-0x0.8#1",
        "-0x0.06#3",
        "-0x0.8#3",
    ];
    let common_values = &[
        ("0.5", 83167),
        ("-0.5", 83060),
        ("1.0", 41731),
        ("-1.0", 41688),
        ("0.2", 41643),
        ("-0.2", 41534),
        ("-2.0", 21185),
        ("0.5", 21166),
        ("0.1", 21077),
        ("-0.5", 20914),
    ];
    let common_values_hex = &[
        ("0x0.8#1", 83167),
        ("-0x0.8#1", 83060),
        ("0x1.0#1", 41731),
        ("-0x1.0#1", 41688),
        ("0x0.4#1", 41643),
        ("-0x0.4#1", 41534),
        ("-0x2.0#1", 21185),
        ("0x0.8#2", 21166),
        ("0x0.2#1", 21077),
        ("-0x0.8#2", 20914),
    ];
    let sample_median = ("0.001", None);
    let sample_median_hex = ("0x0.004#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.568720696402829),
        standard_deviation: NiceFloat(876.7001430745316),
        skewness: NiceFloat(-540.308156545252),
        excess_kurtosis: NiceFloat(486100.3517515351),
    };
    random_nonzero_finite_floats_helper(
        1,
        1,
        2,
        1,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 1, mean precision 64
    let values = &[
        "-4.14430807774440355061614115037031746862111513773654797362803064629233365606384697676260\
        9516719551",
        "-0.5",
        "-0.3325796124",
        "2.11726214369233826954129204",
        "-1.19160392845269975891223055911503394268",
        "0.518426906",
        "-1.2013513335896632820283496412",
        "-0.26333202931220086",
        "-0.2683",
        "1.71088",
        "30.850540057245481219",
        "0.41683542187",
        "-0.5",
        "-1.2",
        "0.329013682192",
        "-0.623",
        "-0.07",
        "-0.6228611764781841916979294095",
        "-0.019814045103",
        "-0.50055",
    ];
    let values_hex = &[
        "-0x4.24f15fca75f9865c038bd55844fff40d21205eaa8642524cd23a9f1837937e9c89911612c6c4819d8\
        #324",
        "-0x0.8#1",
        "-0x0.5523effe8#32",
        "0x2.1e04e45037aa8304b9742#86",
        "-0x1.310cf47e7d4ddcf7a0d6eca9afb1430#125",
        "0x0.84b7a03#28",
        "-0x1.338bc2d0c609ffddf4593fa#93",
        "-0x0.4369ba55e29d85#55",
        "-0x0.44b0#12",
        "0x1.b5fc#16",
        "0x1e.d9bcfe41ceabd8e#65",
        "0x0.6ab5b9e8c#33",
        "-0x0.8#2",
        "-0x1.4#4",
        "0x0.543a3d9cf4#37",
        "-0x0.9f8#9",
        "-0x0.12#4",
        "-0x0.9f73d47eec00b12e51e0814#90",
        "-0x0.05128883b8#32",
        "-0x0.8024#14",
    ];
    let common_values = &[
        ("-0.5", 2594),
        ("0.5", 2582),
        ("1.0", 1322),
        ("0.2", 1316),
        ("-1.0", 1310),
        ("-0.8", 1295),
        ("0.8", 1288),
        ("0.5", 1286),
        ("-0.2", 1282),
        ("-0.5", 1256),
    ];
    let common_values_hex = &[
        ("-0x0.8#1", 2594),
        ("0x0.8#1", 2582),
        ("0x1.0#1", 1322),
        ("0x0.4#1", 1316),
        ("-0x1.0#1", 1310),
        ("-0x0.c#2", 1295),
        ("0x0.c#2", 1288),
        ("0x0.8#2", 1286),
        ("-0x0.4#1", 1282),
        ("-0x0.8#2", 1256),
    ];
    let sample_median = (
        "0.0013945155288752493439",
        Some("0.0013945681317486564931957701"),
    );
    let sample_median_hex = (
        "0x0.005b6416971ccf16f0#63",
        Some("0x0.005b64f88495604a5cf29320#84"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.3471294382388627),
        standard_deviation: NiceFloat(946.3262474932196),
        skewness: NiceFloat(-231.427091034788),
        excess_kurtosis: NiceFloat(286498.2528287583),
    };
    random_nonzero_finite_floats_helper(
        1,
        1,
        64,
        1,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 64, mean precision 2
    let values = &[
        "-5.0e8", "-1.0e-6", "-1.0e1", "2.0e-7", "-0.03", "4.0e22", "-4.0e19", "-0.023", "-3.2e7",
        "9.0e37", "4.0e-9", "1.0e15", "-1.0e11", "-7.0e-9", "3.0e14", "-0.023", "-1.0e-34",
        "-3.0e14", "-1.9e30", "-8.0e56",
    ];
    let values_hex = &[
        "-0x2.0E+7#1",
        "-0x0.00001#1",
        "-0xc.0#2",
        "0x4.0E-6#1",
        "-0x0.08#3",
        "0x8.0E+18#1",
        "-0x2.0E+16#1",
        "-0x0.06#2",
        "-0x1.e84E+6#12",
        "0x4.0E+31#1",
        "0x1.0E-7#1",
        "0x4.0E+12#1",
        "-0x2.0E+9#1",
        "-0x2.0E-7#2",
        "0x1.0E+12#1",
        "-0x0.06#2",
        "-0x8.0E-29#1",
        "-0x1.0E+12#1",
        "-0x1.8E+25#3",
        "-0x2.0E+47#3",
    ];
    let common_values = &[
        ("0.5", 2018),
        ("-1.0", 2004),
        ("-0.5", 1938),
        ("2.0", 1933),
        ("0.2", 1901),
        ("0.06", 1900),
        ("-0.1", 1883),
        ("-2.0", 1875),
        ("0.1", 1874),
        ("1.0", 1873),
    ];
    let common_values_hex = &[
        ("0x0.8#1", 2018),
        ("-0x1.0#1", 2004),
        ("-0x0.8#1", 1938),
        ("0x2.0#1", 1933),
        ("0x0.4#1", 1901),
        ("0x0.1#1", 1900),
        ("-0x0.2#1", 1883),
        ("-0x2.0#1", 1875),
        ("0x0.2#1", 1874),
        ("0x1.0#1", 1873),
    ];
    let sample_median = ("6.0e-123", Some("6.0e-123"));
    let sample_median_hex = ("0x4.0E-102#2", Some("0x4.0E-102#3"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.1187071843154923e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_nonzero_finite_floats_helper(
        64,
        1,
        2,
        1,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // mean |sci_exponent| 64, mean precision 64
    let values = &[
        "-556239614.326901209278831465330010369277037366613406531583358190459728647134822964160746\
        24468527615",
        "-1.0e-6",
        "-10.642547596",
        "2.52397315942327769940053468e-7",
        "-0.037237622764146867466007204972344810709",
        "3.91712295e22",
        "-44322041186676375483.90897365",
        "-0.0164582518320125536",
        "-1.8006e7",
        "1.45545e38",
        "7.1829510985979533799e-9",
        "1.8772598506e15",
        "-1.0e11",
        "-9.0e-9",
        "3.7043647413e14",
        "-0.01947",
        "-1.1e-34",
        "-350639670286337.3842260690894",
        "-1.6075063147e30",
        "-7.855e56",
    ];
    let values_hex = &[
        "-0x21278afe.53afcc32e01c5eaac227ffa0690902f5543212926691d4f8c1bc9bf4e44c88b09636240cec\
        #324",
        "-0x0.00001#1",
        "-0xa.a47dffd#32",
        "0x4.3c09c8a06f55060972e84E-6#86",
        "-0x0.098867a3f3ea6ee7bd06b7654d7d8a180#125",
        "0x8.4b7a03E+18#28",
        "-0x2671785a18c13ffbb.e8b27f4#93",
        "-0x0.04369ba55e29d85#55",
        "-0x1.12cE+6#12",
        "0x6.d7f0E+31#16",
        "0x1.ed9bcfe41ceabd8eE-7#65",
        "0x6.ab5b9e8cE+12#33",
        "-0x2.0E+9#2",
        "-0x2.8E-7#4",
        "0x1.50e8f673dE+12#37",
        "-0x0.04fc#9",
        "-0x9.0E-29#4",
        "-0x13ee7a8fdd801.625ca3c1028#90",
        "-0x1.44a220eeE+25#32",
        "-0x2.009E+47#14",
    ];
    let common_values = &[
        ("-4.0", 73),
        ("5.0e2", 70),
        ("2.0", 69),
        ("1.0", 68),
        ("4.0", 68),
        ("-6.0e1", 67),
        ("-0.008", 65),
        ("8.0", 64),
        ("-0.2", 64),
        ("-2.0", 62),
    ];
    let common_values_hex = &[
        ("-0x4.0#1", 73),
        ("0x2.0E+2#1", 70),
        ("0x2.0#1", 69),
        ("0x1.0#1", 68),
        ("0x4.0#1", 68),
        ("-0x4.0E+1#1", 67),
        ("-0x0.02#1", 65),
        ("0x8.0#1", 64),
        ("-0x0.4#1", 64),
        ("-0x2.0#1", 62),
    ];
    let sample_median = (
        "1.11041133486928099130129119629511e-122",
        Some("1.17736340791157e-122"),
    );
    let sample_median_hex = (
        "0x7.57271a3f7db391dd454e0ccda4E-102#105",
        Some("0x7.c874b99a73cE-102#45"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.8473451262482198e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_nonzero_finite_floats_helper(
        64,
        1,
        64,
        1,
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
fn random_nonzero_finite_floats_fail_1() {
    random_nonzero_finite_floats(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_nonzero_finite_floats_fail_2() {
    random_nonzero_finite_floats(EXAMPLE_SEED, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn random_nonzero_finite_floats_fail_3() {
    random_nonzero_finite_floats(EXAMPLE_SEED, 1, 1, 1, 0);
}

#[test]
#[should_panic]
fn random_nonzero_finite_floats_fail_4() {
    random_nonzero_finite_floats(EXAMPLE_SEED, 1, 1, 1, 1);
}
