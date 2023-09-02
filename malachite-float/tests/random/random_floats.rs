use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::random::random_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn random_floats_helper(
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
        random_floats(
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
fn test_random_floats() {
    // mean |sci_exponent| 1, mean precision 2, zero probability 1/10
    let values = &[
        "0.5", "0.1", "0.0", "NaN", "-0.1", "-0.5", "-0.5", "-0.8", "1.0", "0.1", "-0.5", "-0.1",
        "-1.0", "Infinity", "-0.2", "-0.5", "-0.19", "NaN", "0.2", "-0.0",
    ];
    let values_hex = &[
        "0x0.8#1", "0x0.2#1", "0x0.0", "NaN", "-0x0.2#1", "-0x0.8#2", "-0x0.8#1", "-0x0.c#2",
        "0x1.0#1", "0x0.2#1", "-0x0.8#3", "-0x0.2#2", "-0x1.0#1", "Infinity", "-0x0.4#1",
        "-0x0.8#2", "-0x0.3#2", "NaN", "0x0.4#1", "-0x0.0",
    ];
    let common_values = &[
        ("-0.5", 67626),
        ("0.5", 67293),
        ("0.0", 45033),
        ("-0.0", 44760),
        ("-1.0", 33918),
        ("1.0", 33760),
        ("0.2", 33658),
        ("Infinity", 33640),
        ("-0.2", 33629),
        ("NaN", 33393),
    ];
    let common_values_hex = &[
        ("-0x0.8#1", 67626),
        ("0x0.8#1", 67293),
        ("0x0.0", 45033),
        ("-0x0.0", 44760),
        ("-0x1.0#1", 33918),
        ("0x1.0#1", 33760),
        ("0x0.4#1", 33658),
        ("Infinity", 33640),
        ("-0x0.4#1", 33629),
        ("NaN", 33393),
    ];
    let sample_median = ("NaN", None);
    let sample_median_hex = ("NaN", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(f64::NAN),
        standard_deviation: NiceFloat(f64::NAN),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_floats_helper(
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
        "0.802345",
        "0.1844510959411",
        "0.0",
        "NaN",
        "-0.140425435608983174491205863215",
        "-0.8768255",
        "-0.88317510717823917616548455100967593266949044",
        "-0.71626192860674951702598151669390201151406708405120628436058629",
        "1.0",
        "0.1253012513284905102457716447254996354",
        "-0.9480358649144438967",
        "-0.12929",
        "-1.3179114423679677421065009032698059638051316095512",
        "Infinity",
        "-0.4770511755265128",
        "-0.8",
        "-0.2321615",
        "NaN",
        "0.362821734871449",
        "-0.0",
    ];
    let values_hex = &[
        "0x0.cd668#18",
        "0x0.2f382fe0c74#40",
        "0x0.0",
        "NaN",
        "-0x0.23f2ebdd779643465dc18b5f0#95",
        "-0x0.e077a2d0#30",
        "-0x0.e217c389f8c9fd22042f5ed70da20cfb9f1ec#146",
        "-0x0.b75cf116bc625ef1eab58f3c99505c2492852d5fb6817443c180#205",
        "0x1.00#7",
        "0x0.2013be289a8eeba947a6a3693540ab#118",
        "-0x0.f2b27a7b3e1bb6a8#62",
        "-0x0.2119#14",
        "-0x1.5162a4effe9ec7016f7470154ad80659d1b6420f0#162",
        "Infinity",
        "-0x0.7a20069d679c4#51",
        "-0x0.c#2",
        "-0x0.3b6ef0#20",
        "NaN",
        "0x0.5ce1e29d8d050#48",
        "-0x0.0",
    ];
    let common_values = &[
        ("0.0", 45033),
        ("-0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0.5", 2133),
        ("-0.5", 2122),
        ("-1.0", 1103),
        ("0.2", 1094),
        ("0.8", 1091),
    ];
    let common_values_hex = &[
        ("0x0.0", 45033),
        ("-0x0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0x0.8#1", 2133),
        ("-0x0.8#1", 2122),
        ("-0x1.0#1", 1103),
        ("0x0.4#1", 1094),
        ("0x0.c#2", 1091),
    ];
    let sample_median = ("NaN", None);
    let sample_median_hex = ("NaN", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(f64::NAN),
        standard_deviation: NiceFloat(f64::NAN),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_floats_helper(
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
        "3.0e11", "3.0e122", "0.0", "NaN", "-1.0e25", "-1.0e21", "-9.0e-10", "-2.0e33", "2.0e-9",
        "3.0e2", "-2.2e-16", "-3.0e-18", "-1.0e-19", "Infinity", "-9.0e49", "-5.0e11", "-1.5e-28",
        "NaN", "0.008", "-0.0",
    ];
    let values_hex = &[
        "0x4.0E+9#1",
        "0x8.0E+101#1",
        "0x0.0",
        "NaN",
        "-0x8.0E+20#1",
        "-0x4.0E+17#2",
        "-0x4.0E-8#1",
        "-0x6.0E+27#2",
        "0x8.0E-8#1",
        "0x1.0E+2#1",
        "-0x1.0E-13#3",
        "-0x4.0E-15#2",
        "-0x2.0E-16#1",
        "Infinity",
        "-0x4.0E+41#1",
        "-0x8.0E+9#2",
        "-0xc.0E-24#2",
        "NaN",
        "0x0.02#1",
        "-0x0.0",
    ];
    let common_values = &[
        ("0.0", 45033),
        ("-0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0.2", 1583),
        ("-0.5", 1560),
        ("1.0", 1542),
        ("-1.0", 1523),
        ("0.5", 1518),
    ];
    let common_values_hex = &[
        ("0x0.0", 45033),
        ("-0x0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0x0.4#1", 1583),
        ("-0x0.8#1", 1560),
        ("0x1.0#1", 1542),
        ("-0x1.0#1", 1523),
        ("0x0.8#1", 1518),
    ];
    let sample_median = ("NaN", None);
    let sample_median_hex = ("NaN", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(f64::NAN),
        standard_deviation: NiceFloat(f64::NAN),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_floats_helper(
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
        "4.41094e11",
        "4.87729991687e122",
        "0.0",
        "NaN",
        "-10864891829653204324806754.8428",
        "-2.070345677e21",
        "-1.64504182930707778067418290490224199866781094e-9",
        "-1859522280896461251765224423181480.1799665248460650575751417497",
        "1.86e-9",
        "256.616962720748564983340328397823253",
        "-4.210124981593760903e-16",
        "-3.5885e-18",
        "-1.4288824489588552981166100645509463424081540299818e-19",
        "Infinity",
        "-1.784860349753934e50",
        "-8.0e11",
        "-1.875386e-28",
        "NaN",
        "0.01133817921473279",
        "-0.0",
    ];
    let values_hex = &[
        "0x6.6b34E+9#18",
        "0xb.ce0bf831dE+101#40",
        "0x0.0",
        "NaN",
        "-0x8fcbaf75de590d1977062.d7c#95",
        "-0x7.03bd168E+17#30",
        "-0x7.10be1c4fc64fe910217af6b86d1067dcf8f6E-8#146",
        "-0x5bae788b5e312f78f55ac79e4ca8.2e12494296afdb40ba21e0c0#205",
        "0x8.0E-8#7",
        "0x100.9df144d4775d4a3d351b49aa0558#118",
        "-0x1.e564f4f67c376d50E-13#62",
        "-0x4.232E-15#14",
        "-0x2.a2c549dffd3d8e02dee8e02a95b00cb3a36c841eE-16#162",
        "Infinity",
        "-0x7.a20069d679c4E+41#51",
        "-0xc.0E+9#2",
        "-0xe.dbbcE-24#20",
        "NaN",
        "0x0.02e70f14ec6828#48",
        "-0x0.0",
    ];
    let common_values = &[
        ("0.0", 45033),
        ("-0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0.00002", 60),
        ("-0.2", 57),
        ("-3.0e1", 56),
        ("-3.0e2", 55),
        ("4.0", 54),
    ];
    let common_values_hex = &[
        ("0x0.0", 45033),
        ("-0x0.0", 44760),
        ("Infinity", 33640),
        ("NaN", 33393),
        ("-Infinity", 33191),
        ("0x0.0001#1", 60),
        ("-0x0.4#1", 57),
        ("-0x2.0E+1#1", 56),
        ("-0x1.0E+2#1", 55),
        ("0x4.0#1", 54),
    ];
    let sample_median = ("NaN", None);
    let sample_median_hex = ("NaN", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(f64::NAN),
        standard_deviation: NiceFloat(f64::NAN),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_floats_helper(
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
fn random_floats_fail_1() {
    random_floats(EXAMPLE_SEED, 1, 0, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_floats_fail_2() {
    random_floats(EXAMPLE_SEED, 0, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_floats_fail_3() {
    random_floats(EXAMPLE_SEED, 1, 1, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn random_floats_fail_4() {
    random_floats(EXAMPLE_SEED, 1, 1, 1, 1, 1, 10);
}

#[test]
#[should_panic]
fn random_floats_fail_5() {
    random_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0);
}

#[test]
#[should_panic]
fn random_floats_fail_6() {
    random_floats(EXAMPLE_SEED, 1, 1, 2, 1, 2, 1);
}
