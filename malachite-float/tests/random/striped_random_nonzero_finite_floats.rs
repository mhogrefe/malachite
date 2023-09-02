use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::random::striped_random_nonzero_finite_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn striped_random_nonzero_finite_floats_helper(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
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
        striped_random_nonzero_finite_floats(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
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
fn test_striped_random_nonzero_finite_floats() {
    // mean |sci_exponent| 1, mean precision 2
    let values = &[
        "-4.0", "-0.5", "-0.4", "2.0", "-1.8", "0.5", "-1.0", "-0.2", "-0.25", "1.0", "2.0e1",
        "0.2", "-0.5", "-1.5", "0.2", "-0.5", "-0.06", "-0.5", "-0.027", "-0.9",
    ];
    let values_hex = &[
        "-0x4.0#1",
        "-0x0.8#1",
        "-0x0.6#2",
        "0x2.0#1",
        "-0x1.c#3",
        "0x0.8#1",
        "-0x1.0#1",
        "-0x0.4#2",
        "-0x0.4000#12",
        "0x1.0#1",
        "0x1.0E+1#1",
        "0x0.4#1",
        "-0x0.8#1",
        "-0x1.8#2",
        "0x0.4#1",
        "-0x0.8#2",
        "-0x0.1#1",
        "-0x0.8#1",
        "-0x0.07#3",
        "-0x0.e#3",
    ];
    let common_values = &[
        ("0.5", 83167),
        ("-0.5", 83060),
        ("1.0", 41731),
        ("-1.0", 41688),
        ("0.2", 41643),
        ("-0.2", 41534),
        ("0.5", 21186),
        ("-2.0", 21185),
        ("-0.5", 21101),
        ("0.1", 21077),
    ];
    let common_values_hex = &[
        ("0x0.8#1", 83167),
        ("-0x0.8#1", 83060),
        ("0x1.0#1", 41731),
        ("-0x1.0#1", 41688),
        ("0x0.4#1", 41643),
        ("-0x0.4#1", 41534),
        ("0x0.8#2", 21186),
        ("-0x2.0#1", 21185),
        ("-0x0.8#2", 21101),
        ("0x0.2#1", 21077),
    ];
    let sample_median = ("0.001", None);
    let sample_median_hex = ("0x0.004#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.5590711455005939),
        standard_deviation: NiceFloat(827.1547441305203),
        skewness: NiceFloat(-398.68071157281685),
        excess_kurtosis: NiceFloat(306701.41416153224),
    };
    striped_random_nonzero_finite_floats_helper(
        1,
        1,
        8,
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
        "-7.99998474121093750001355087279484370237900894408952422003540121387514246086880724874263\
        75172310289",
        "-0.5",
        "-0.4999980928",
        "3.99999618536094136631021225",
        "-1.99951171875000002775557561562812773113",
        "0.999999981",
        "-1.9843750000000284217094304038",
        "-0.49999999999999822",
        "-0.25",
        "1.75",
        "31.96875",
        "0.25097656244",
        "-0.8",
        "-1.9",
        "0.25",
        "-0.5",
        "-0.117",
        "-0.8124084472656249999999995864",
        "-0.03112804889",
        "-0.99994",
    ];
    let values_hex = &[
        "-0x7.ffff0000000000003ffe0000000003ffffffc001ff00000000000001fff8001ffffffffc7f8000000\
        #324",
        "-0x0.8#1",
        "-0x0.7fffe0008#32",
        "0x3.ffffc0003fffcfc00007f#86",
        "-0x1.ffe00000000001fffffffffff00ffff#125",
        "0x0.ffffffb#28",
        "-0x1.fc0000000007fffffffffff#93",
        "-0x0.7fffffffffff80#55",
        "-0x0.4000#12",
        "0x1.c000#16",
        "0x1f.f80000000000000#65",
        "0x0.403fffffc#33",
        "-0x0.c#2",
        "-0x1.e#4",
        "0x0.4000000000#37",
        "-0x0.800#9",
        "-0x0.1e#4",
        "-0x0.cff9ffffffffffffffff800#90",
        "-0x0.07f801fff8#32",
        "-0x0.fffc#14",
    ];
    let common_values = &[
        ("-0.5", 2594),
        ("0.5", 2582),
        ("1.0", 1322),
        ("0.2", 1316),
        ("-1.0", 1310),
        ("-0.8", 1300),
        ("0.5", 1293),
        ("-0.2", 1282),
        ("0.8", 1281),
        ("-0.5", 1251),
    ];
    let common_values_hex = &[
        ("-0x0.8#1", 2594),
        ("0x0.8#1", 2582),
        ("0x1.0#1", 1322),
        ("0x0.4#1", 1316),
        ("-0x1.0#1", 1310),
        ("-0x0.c#2", 1300),
        ("0x0.8#2", 1293),
        ("-0x0.4#1", 1282),
        ("0x0.c#2", 1281),
        ("-0x0.8#2", 1251),
    ];
    let sample_median = (
        "0.00099134445",
        Some("0.00099170859994046622887253761291501"),
    );
    let sample_median_hex = (
        "0x0.0040f8000#26",
        Some("0x0.0040fe1c00ffffffffffffffffffe#106"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.7676690525044655),
        standard_deviation: NiceFloat(1309.828468878903),
        skewness: NiceFloat(-395.5135549279457),
        excess_kurtosis: NiceFloat(376873.9371275132),
    };
    striped_random_nonzero_finite_floats_helper(
        1,
        1,
        32,
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
        "-5.0e8", "-1.0e-6", "-1.0e1", "2.0e-7", "-0.055", "4.0e22", "-4.0e19", "-0.016",
        "-1.678e7", "9.0e37", "4.0e-9", "1.0e15", "-1.0e11", "-1.1e-8", "3.0e14", "-0.016",
        "-1.0e-34", "-3.0e14", "-2.2e30", "-1.4e57",
    ];
    let values_hex = &[
        "-0x2.0E+7#1",
        "-0x0.00001#1",
        "-0xc.0#2",
        "0x4.0E-6#1",
        "-0x0.0e#3",
        "0x8.0E+18#1",
        "-0x2.0E+16#1",
        "-0x0.04#2",
        "-0x1.000E+6#12",
        "0x4.0E+31#1",
        "0x1.0E-7#1",
        "0x4.0E+12#1",
        "-0x2.0E+9#1",
        "-0x3.0E-7#2",
        "0x1.0E+12#1",
        "-0x0.04#2",
        "-0x8.0E-29#1",
        "-0x1.0E+12#1",
        "-0x1.cE+25#3",
        "-0x3.8E+47#3",
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
    let sample_median = ("6.0e-123", None);
    let sample_median_hex = ("0x4.0E-102#2", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.118707184315476e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_nonzero_finite_floats_helper(
        64,
        1,
        8,
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
        "-1073739776.00000000000181876735894093184841877536737496941412363049476369677414021499616\
        766429030955",
        "-1.0e-6",
        "-15.999938969",
        "4.76836703462712927616859942e-7",
        "-0.062484741210937500867361737988378991598",
        "7.55578623e22",
        "-73210515542535831551.999999993",
        "-0.031249999999999889",
        "-1.678e7",
        "1.48874e38",
        "7.4433046393096446991e-9",
        "1.1302979531e15",
        "-2.0e11",
        "-1.4e-8",
        "2.81474976711e14",
        "-0.01562",
        "-1.8e-34",
        "-457345297547263.9999999997672",
        "-2.525407351e30",
        "-1.5692e57",
    ];
    let values_hex = &[
        "-0x3ffff800.0000000001fff0000000001ffffffe000ff80000000000000fffc000ffffffffe3fc000000\
        #324",
        "-0x0.00001#1",
        "-0xf.fffc001#32",
        "0x7.ffff80007fff9f80000feE-6#86",
        "-0x0.0fff00000000000fffffffffff807fff8#125",
        "0xf.fffffbE+18#28",
        "-0x3f8000000000fffff.ffffffe#93",
        "-0x0.07fffffffffff80#55",
        "-0x1.000E+6#12",
        "0x7.0000E+31#16",
        "0x1.ff80000000000000E-7#65",
        "0x4.03fffffcE+12#33",
        "-0x3.0E+9#2",
        "-0x3.cE-7#4",
        "0x1.000000000E+12#37",
        "-0x0.0400#9",
        "-0xf.0E-29#4",
        "-0x19ff3ffffffff.ffffffff000#90",
        "-0x1.fe007ffeE+25#32",
        "-0x3.fffE+47#14",
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
        "6.0509710318451379813553045518217e-123",
        Some("9.0763873905829e-123"),
    );
    let sample_median_hex = (
        "0x4.0001fffffffe00000000000000E-102#105",
        Some("0x6.000000fff80E-102#45"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.1192363579409163e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_nonzero_finite_floats_helper(
        64,
        1,
        32,
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
fn striped_random_nonzero_finite_floats_fail_1() {
    striped_random_nonzero_finite_floats(EXAMPLE_SEED, 1, 0, 2, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_finite_floats_fail_2() {
    striped_random_nonzero_finite_floats(EXAMPLE_SEED, 0, 1, 2, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_finite_floats_fail_3() {
    striped_random_nonzero_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_nonzero_finite_floats_fail_4() {
    striped_random_nonzero_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_finite_floats_fail_5() {
    striped_random_nonzero_finite_floats(EXAMPLE_SEED, 1, 1, 2, 3, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_finite_floats_fail_6() {
    striped_random_nonzero_finite_floats(EXAMPLE_SEED, 1, 1, 1, 0, 2, 1);
}
