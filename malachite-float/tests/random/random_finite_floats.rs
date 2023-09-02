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
    mean_special_p_numerator: u64,
    mean_special_p_denominator: u64,
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
            mean_special_p_numerator,
            mean_special_p_denominator,
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
fn test_random_finite_floats() {
    // mean |sci_exponent| 1, mean precision 2, special probability 1/10
    let values = &[
        "-1.5", "-0.5", "-1.0", "0.56", "-0.5", "0.8", "-1.0", "-0.16", "-0.5", "0.4", "0.0",
        "0.1689", "-0.6", "-0.3", "4.0", "-3.0", "-1.5", "-0.1", "-0.4", "-0.2",
    ];
    let values_hex = &[
        "-0x1.8#3",
        "-0x0.8#3",
        "-0x1.0#2",
        "0x0.90#5",
        "-0x0.8#2",
        "0x0.c#3",
        "-0x1.0#1",
        "-0x0.28#3",
        "-0x0.8#1",
        "0x0.6#2",
        "0x0.0",
        "0x0.2b4#9",
        "-0x0.a#3",
        "-0x0.4c#5",
        "0x4.0#1",
        "-0x3.0#2",
        "-0x1.8#2",
        "-0x0.2#1",
        "-0x0.6#2",
        "-0x0.4#2",
    ];
    let common_values = &[
        ("0.5", 75012),
        ("-0.5", 74560),
        ("0.0", 49991),
        ("-0.0", 49908),
        ("0.2", 37822),
        ("1.0", 37501),
        ("-0.2", 37342),
        ("-1.0", 37246),
        ("-0.1", 18908),
        ("-0.5", 18873),
    ];
    let common_values_hex = &[
        ("0x0.8#1", 75012),
        ("-0x0.8#1", 74560),
        ("0x0.0", 49991),
        ("-0x0.0", 49908),
        ("0x0.4#1", 37822),
        ("0x1.0#1", 37501),
        ("-0x0.4#1", 37342),
        ("-0x1.0#1", 37246),
        ("-0x0.2#1", 18908),
        ("-0x0.8#2", 18873),
    ];
    let sample_median = ("0.0", None);
    let sample_median_hex = ("0x0.0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.35820764358511503),
        standard_deviation: NiceFloat(839.1199426793322),
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
        "-1.276455544232851070486937829440259357413494222677",
        "-0.71458310423612216",
        "-1.38",
        "0.8139477172474787365388435096481241289831365",
        "-0.8818329688",
        "0.5336974",
        "-1.28323180351375524",
        "-0.1306961333698765889579267",
        "-0.5649972517704091711657442058822950769",
        "0.428158246",
        "0.0",
        "0.194477040850213320126136",
        "-0.56103557531546544878496383",
        "-0.467031203520839894269095085089752",
        "5.653591",
        "-2.5",
        "-1.6067478719665671",
        "-0.1753942383338516667238212698",
        "-0.43481",
        "-0.30851523737",
    ];
    let values_hex = &[
        "-0x1.46c5ca6147297c168f280822c823b2b47fd70a70#158",
        "-0x0.b6eeeb16f7e2c10#57",
        "-0x1.6#5",
        "0x0.d05ee0aa3b55b13f8dcc5e28d5924bb01efb0#145",
        "-0x0.e1bfce318#34",
        "0x0.88a064#22",
        "-0x1.4881e12547575a#57",
        "-0x0.21754d42898acda8bd1774#84",
        "-0x0.90a3a8eeaf0c90e0040a9fe7445b43e#123",
        "0x0.6d9bc76#26",
        "0x0.0",
        "0x0.31c93f52464716fe6ec0#75",
        "-0x0.8fa00707df5a209fb57a20#85",
        "-0x0.778f5b6155620d3cc561201446d#107",
        "0x5.a751c#21",
        "-0x2.8#4",
        "-0x1.9b53d41b039590#55",
        "-0x0.2ce6a3058cfc254da54c4362#94",
        "-0x0.6f50#13",
        "-0x0.4efadac6d0#36",
    ];
    let common_values = &[
        ("0.0", 49991),
        ("-0.0", 49908),
        ("0.5", 2360),
        ("-0.5", 2302),
        ("1.0", 1209),
        ("-0.5", 1197),
        ("-0.2", 1177),
        ("0.5", 1168),
        ("0.2", 1158),
        ("-1.0", 1158),
    ];
    let common_values_hex = &[
        ("0x0.0", 49991),
        ("-0x0.0", 49908),
        ("0x0.8#1", 2360),
        ("-0x0.8#1", 2302),
        ("0x1.0#1", 1209),
        ("-0x0.8#2", 1197),
        ("-0x0.4#1", 1177),
        ("0x0.8#2", 1168),
        ("0x0.4#1", 1158),
        ("-0x1.0#1", 1158),
    ];
    let sample_median = ("0.0", None);
    let sample_median_hex = ("0x0.0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.30274329854971843),
        standard_deviation: NiceFloat(822.4736684416786),
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
        "-2.6e13", "-1.1e-16", "-0.2", "1.5e8", "-5.0e-7", "0.19", "-7.0e-12", "-3.5e-17",
        "-2.0e10", "9.0e-14", "0.0", "1.995e20", "-8.0e-30", "-2.4e28", "7.0e-9", "-1.0e-21",
        "-6.0e9", "-6.0e42", "-0.09", "-0.06",
    ];
    let values_hex = &[
        "-0x1.8E+11#3",
        "-0x8.0E-14#3",
        "-0x0.4#2",
        "0x9.0E+6#5",
        "-0x8.0E-6#2",
        "0x0.30#3",
        "-0x8.0E-10#1",
        "-0x2.8E-14#3",
        "-0x4.0E+8#1",
        "0x1.8E-11#2",
        "0x0.0",
        "0xa.d0E+16#9",
        "-0xa.0E-25#3",
        "-0x4.cE+23#5",
        "0x2.0E-7#1",
        "-0x6.0E-18#2",
        "-0x1.8E+8#2",
        "-0x4.0E+35#1",
        "-0x0.18#2",
        "-0x0.10#2",
    ];
    let common_values = &[
        ("0.0", 49991),
        ("-0.0", 49908),
        ("0.5", 1791),
        ("0.2", 1788),
        ("-0.5", 1782),
        ("-2.0", 1778),
        ("-0.2", 1727),
        ("1.0", 1695),
        ("2.0", 1690),
        ("8.0", 1689),
    ];
    let common_values_hex = &[
        ("0x0.0", 49991),
        ("-0x0.0", 49908),
        ("0x0.8#1", 1791),
        ("0x0.4#1", 1788),
        ("-0x0.8#1", 1782),
        ("-0x2.0#1", 1778),
        ("-0x0.4#1", 1727),
        ("0x1.0#1", 1695),
        ("0x2.0#1", 1690),
        ("0x8.0#1", 1689),
    ];
    let sample_median = ("0.0", None);
    let sample_median_hex = ("0x0.0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(8.949648939501136e242),
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
        "-22455643411570.5927949516340126971491362772288941",
        "-1.58669323066212211e-16",
        "-0.34",
        "218492426.639486019493108319225034600108191063",
        "-8.409814537e-7",
        "0.13342434",
        "-9.3367402115382373e-12",
        "-2.9020371299343447936786401e-17",
        "-19413157749.470283925562700478127585444",
        "9.7351914e-14",
        "0.0",
        "229597964849679548315.69",
        "-7.0812645088727875172066648e-30",
        "-37002024091781566033375076372.2766",
        "1.0530634e-8",
        "-1.1e-21",
        "-6900929563.0140009",
        "-7.8228444367471566385750529024e42",
        "-0.1087",
        "-0.077128809342",
    ];
    let values_hex = &[
        "-0x146c5ca61472.97c168f280822c823b2b47fd70a70#158",
        "-0xb.6eeeb16f7e2c10E-14#57",
        "-0x0.58#5",
        "0xd05ee0a.a3b55b13f8dcc5e28d5924bb01efb0#145",
        "-0xe.1bfce318E-6#34",
        "0x0.222819#22",
        "-0xa.440f092a3abad0E-10#57",
        "-0x2.1754d42898acda8bd1774E-14#84",
        "-0x4851d4775.786487002054ff3a22da1f#123",
        "0x1.b66f1d8E-11#26",
        "0x0.0",
        "0xc724fd49191c5bf9b.b0#75",
        "-0x8.fa00707df5a209fb57a20E-25#85",
        "-0x778f5b6155620d3cc5612014.46d#107",
        "0x2.d3a8eE-7#21",
        "-0x5.0E-18#4",
        "-0x19b53d41b.039590#55",
        "-0x5.9cd460b19f84a9b4a9886c4E+35#94",
        "-0x0.1bd4#13",
        "-0x0.13beb6b1b4#36",
    ];
    let common_values = &[
        ("0.0", 49991),
        ("-0.0", 49908),
        ("-8.0", 77),
        ("1.0", 66),
        ("-1.0", 61),
        ("-4.0", 61),
        ("-2.0e1", 59),
        ("-0.1", 58),
        ("0.2", 57),
        ("2.0e3", 57),
    ];
    let common_values_hex = &[
        ("0x0.0", 49991),
        ("-0x0.0", 49908),
        ("-0x8.0#1", 77),
        ("0x1.0#1", 66),
        ("-0x1.0#1", 61),
        ("-0x4.0#1", 61),
        ("-0x1.0E+1#1", 59),
        ("-0x0.2#1", 58),
        ("0x0.4#1", 57),
        ("0x8.0E+2#1", 57),
    ];
    let sample_median = ("0.0", None);
    let sample_median_hex = ("0x0.0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.6165089066198808e243),
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
