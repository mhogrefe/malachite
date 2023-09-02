use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::random::striped_random_non_positive_finite_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn striped_random_non_positive_finite_floats_helper(
    mean_sci_exponent_abs_numerator: u64,
    mean_sci_exponent_abs_denominator: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    zero_p_numerator: u64,
    zero_p_denominator: u64,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper(
        striped_random_non_positive_finite_floats(
            EXAMPLE_SEED,
            mean_sci_exponent_abs_numerator,
            mean_sci_exponent_abs_denominator,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_precision_numerator,
            mean_precision_denominator,
            zero_p_numerator,
            zero_p_denominator,
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
fn test_striped_random_non_positive_finite_floats() {
    // mean |sci_exponent| 1, mean precision 2
    let values = &[
        "-4.0", "-0.5", "-0.4", "-0.0", "-2.0", "-1.8", "-0.5", "-1.0", "-0.2", "-0.25", "-1.0",
        "-2.0e1", "-0.2", "-0.0", "-0.5", "-1.5", "-0.2", "-0.0", "-0.5", "-0.06",
    ];
    let values_hex = &[
        "-0x4.0#1",
        "-0x0.8#1",
        "-0x0.6#2",
        "-0x0.0",
        "-0x2.0#1",
        "-0x1.c#3",
        "-0x0.8#1",
        "-0x1.0#1",
        "-0x0.4#2",
        "-0x0.4000#12",
        "-0x1.0#1",
        "-0x1.0E+1#1",
        "-0x0.4#1",
        "-0x0.0",
        "-0x0.8#1",
        "-0x1.8#2",
        "-0x0.4#1",
        "-0x0.0",
        "-0x0.8#2",
        "-0x0.1#1",
    ];
    let common_values = &[
        ("-0.5", 149586),
        ("-0.0", 100224),
        ("-1.0", 74929),
        ("-0.2", 74827),
        ("-0.5", 38123),
        ("-2.0", 37612),
        ("-0.1", 37602),
        ("-0.8", 37379),
        ("-0.2", 18928),
        ("-0.4", 18840),
    ];
    let common_values_hex = &[
        ("-0x0.8#1", 149586),
        ("-0x0.0", 100224),
        ("-0x1.0#1", 74929),
        ("-0x0.4#1", 74827),
        ("-0x0.8#2", 38123),
        ("-0x2.0#1", 37612),
        ("-0x0.2#1", 37602),
        ("-0x0.c#2", 37379),
        ("-0x0.4#2", 18928),
        ("-0x0.6#2", 18840),
    ];
    let sample_median = ("-0.5", None);
    let sample_median_hex = ("-0x0.8#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-4.378260436174753),
        standard_deviation: NiceFloat(783.6252024383801),
        skewness: NiceFloat(-588.0046776041995),
        excess_kurtosis: NiceFloat(368204.8297165849),
    };
    striped_random_non_positive_finite_floats_helper(
        1,
        1,
        8,
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

    // mean |sci_exponent| 1, mean precision 64
    let values = &[
        "-7.99998474121093750001355087279484370237900894408952422003540121387514246086880724874263\
        75172310289",
        "-0.5",
        "-0.4999980928",
        "-0.0",
        "-3.99999618536094136631021225",
        "-1.99951171875000002775557561562812773113",
        "-0.999999981",
        "-1.9843750000000284217094304038",
        "-0.49999999999999822",
        "-0.25",
        "-1.75",
        "-31.96875",
        "-0.25097656244",
        "-0.0",
        "-0.8",
        "-1.9",
        "-0.25",
        "-0.0",
        "-0.5",
        "-0.117",
    ];
    let values_hex = &[
        "-0x7.ffff0000000000003ffe0000000003ffffffc001ff00000000000001fff8001ffffffffc7f8000000\
        #324",
        "-0x0.8#1",
        "-0x0.7fffe0008#32",
        "-0x0.0",
        "-0x3.ffffc0003fffcfc00007f#86",
        "-0x1.ffe00000000001fffffffffff00ffff#125",
        "-0x0.ffffffb#28",
        "-0x1.fc0000000007fffffffffff#93",
        "-0x0.7fffffffffff80#55",
        "-0x0.4000#12",
        "-0x1.c000#16",
        "-0x1f.f80000000000000#65",
        "-0x0.403fffffc#33",
        "-0x0.0",
        "-0x0.c#2",
        "-0x1.e#4",
        "-0x0.4000000000#37",
        "-0x0.0",
        "-0x0.800#9",
        "-0x0.1e#4",
    ];
    let common_values = &[
        ("-0.0", 100224),
        ("-0.5", 4643),
        ("-1.0", 2373),
        ("-0.2", 2353),
        ("-0.8", 2346),
        ("-0.5", 2275),
        ("-0.5", 2183),
        ("-0.9", 2131),
        ("-0.5", 2125),
        ("-0.94", 2082),
    ];
    let common_values_hex = &[
        ("-0x0.0", 100224),
        ("-0x0.8#1", 4643),
        ("-0x1.0#1", 2373),
        ("-0x0.4#1", 2353),
        ("-0x0.c#2", 2346),
        ("-0x0.8#2", 2275),
        ("-0x0.8#3", 2183),
        ("-0x0.e#3", 2131),
        ("-0x0.8#4", 2125),
        ("-0x0.f#4", 2082),
    ];
    let sample_median = (
        "-0.500003814697265624998",
        Some("-0.500003814697265624996823626346822177860204279529750086286"),
    );
    let sample_median_hex = (
        "-0x0.80003ffffffffffff8#69",
        Some("-0x0.80003ffffffffffff0fffff8000000000000000000000000#189"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-5.794641954736897),
        standard_deviation: NiceFloat(1199.3322364588257),
        skewness: NiceFloat(-671.6561608456597),
        excess_kurtosis: NiceFloat(499631.39656685427),
    };
    striped_random_non_positive_finite_floats_helper(
        1,
        1,
        32,
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

    // mean |sci_exponent| 64, mean precision 2
    let values = &[
        "-5.0e8", "-1.0e-6", "-1.0e1", "-0.0", "-2.0e-7", "-0.055", "-4.0e22", "-4.0e19", "-0.016",
        "-1.678e7", "-9.0e37", "-4.0e-9", "-1.0e15", "-0.0", "-1.0e11", "-1.1e-8", "-3.0e14",
        "-0.0", "-0.016", "-1.0e-34",
    ];
    let values_hex = &[
        "-0x2.0E+7#1",
        "-0x0.00001#1",
        "-0xc.0#2",
        "-0x0.0",
        "-0x4.0E-6#1",
        "-0x0.0e#3",
        "-0x8.0E+18#1",
        "-0x2.0E+16#1",
        "-0x0.04#2",
        "-0x1.000E+6#12",
        "-0x4.0E+31#1",
        "-0x1.0E-7#1",
        "-0x4.0E+12#1",
        "-0x0.0",
        "-0x2.0E+9#1",
        "-0x3.0E-7#2",
        "-0x1.0E+12#1",
        "-0x0.0",
        "-0x0.04#2",
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
    let sample_median = ("-0.0034", None);
    let sample_median_hex = ("-0x0.00e#3", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.1187071843154847e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_non_positive_finite_floats_helper(
        64,
        1,
        8,
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

    // mean |sci_exponent| 64, mean precision 64
    let values = &[
        "-1073739776.00000000000181876735894093184841877536737496941412363049476369677414021499616\
        766429030955",
        "-1.0e-6",
        "-15.999938969",
        "-0.0",
        "-4.76836703462712927616859942e-7",
        "-0.062484741210937500867361737988378991598",
        "-7.55578623e22",
        "-73210515542535831551.999999993",
        "-0.031249999999999889",
        "-1.678e7",
        "-1.48874e38",
        "-7.4433046393096446991e-9",
        "-1.1302979531e15",
        "-0.0",
        "-2.0e11",
        "-1.4e-8",
        "-2.81474976711e14",
        "-0.0",
        "-0.01562",
        "-1.8e-34",
    ];
    let values_hex = &[
        "-0x3ffff800.0000000001fff0000000001ffffffe000ff80000000000000fffc000ffffffffe3fc000000\
        #324",
        "-0x0.00001#1",
        "-0xf.fffc001#32",
        "-0x0.0",
        "-0x7.ffff80007fff9f80000feE-6#86",
        "-0x0.0fff00000000000fffffffffff807fff8#125",
        "-0xf.fffffbE+18#28",
        "-0x3f8000000000fffff.ffffffe#93",
        "-0x0.07fffffffffff80#55",
        "-0x1.000E+6#12",
        "-0x7.0000E+31#16",
        "-0x1.ff80000000000000E-7#65",
        "-0x4.03fffffcE+12#33",
        "-0x0.0",
        "-0x3.0E+9#2",
        "-0x3.cE-7#4",
        "-0x1.000000000E+12#37",
        "-0x0.0",
        "-0x0.0400#9",
        "-0xf.0E-29#4",
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
    let sample_median = (
        "-0.003906249999985790879585158165071727505",
        Some("-0.0039062499999857908796"),
    );
    let sample_median_hex = (
        "-0x0.00fffffffffc001ffe007c000000007f8#121",
        Some("-0x0.00fffffffffc001ffe#63"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1.119236357940914e242),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_non_positive_finite_floats_helper(
        64,
        1,
        32,
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
fn striped_random_non_positive_finite_floats_fail_1() {
    striped_random_non_positive_finite_floats(EXAMPLE_SEED, 1, 0, 2, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_positive_finite_floats_fail_2() {
    striped_random_non_positive_finite_floats(EXAMPLE_SEED, 0, 1, 2, 1, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_positive_finite_floats_fail_3() {
    striped_random_non_positive_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_positive_finite_floats_fail_4() {
    striped_random_non_positive_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_positive_finite_floats_fail_5() {
    striped_random_non_positive_finite_floats(EXAMPLE_SEED, 1, 1, 2, 3, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_positive_finite_floats_fail_6() {
    striped_random_non_positive_finite_floats(EXAMPLE_SEED, 1, 1, 1, 0, 2, 1, 1, 10);
}

#[test]
#[should_panic]
fn striped_random_non_positive_finite_floats_fail_7() {
    striped_random_non_positive_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 2, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_non_positive_finite_floats_fail_8() {
    striped_random_non_positive_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 2, 1, 1, 0);
}
