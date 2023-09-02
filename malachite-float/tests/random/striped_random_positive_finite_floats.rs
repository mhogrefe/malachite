use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::random::striped_random_positive_finite_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn striped_random_positive_finite_floats_helper(
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
        striped_random_positive_finite_floats(
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
fn test_striped_random_positive_finite_floats() {
    // mean |sci_exponent| 1, mean precision 2
    let values = &[
        "0.8", "0.5", "1.0", "0.5", "0.5", "1.0", "1.0", "0.9", "1.0", "0.5", "1.0", "1.0", "2.0",
        "0.5", "0.5", "1.0", "0.06", "0.5", "0.1", "0.5",
    ];
    let values_hex = &[
        "0x0.c#2", "0x0.8#1", "0x1.0#2", "0x0.8#2", "0x0.8#1", "0x1.0#1", "0x1.0#1", "0x0.e#3",
        "0x1.0#1", "0x0.8#1", "0x1.0#1", "0x1.0#1", "0x2.0#1", "0x0.8#1", "0x0.8#3", "0x1.0#4",
        "0x0.1#1", "0x0.8#1", "0x0.2#1", "0x0.8#3",
    ];
    let common_values = &[
        ("0.5", 166114),
        ("0.2", 83464),
        ("1.0", 83434),
        ("0.5", 42010),
        ("0.1", 41531),
        ("0.8", 41521),
        ("2.0", 41483),
        ("1.5", 21125),
        ("0.06", 20888),
        ("4.0", 20800),
    ];
    let common_values_hex = &[
        ("0x0.8#1", 166114),
        ("0x0.4#1", 83464),
        ("0x1.0#1", 83434),
        ("0x0.8#2", 42010),
        ("0x0.2#1", 41531),
        ("0x0.c#2", 41521),
        ("0x2.0#1", 41483),
        ("0x1.8#2", 21125),
        ("0x0.1#1", 20888),
        ("0x4.0#1", 20800),
    ];
    let sample_median = ("0.5", None);
    let sample_median_hex = ("0x0.8#2", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.551406442622566),
        standard_deviation: NiceFloat(175.77708643350974),
        skewness: NiceFloat(274.1565025440714),
        excess_kurtosis: NiceFloat(92676.98806519402),
    };
    striped_random_positive_finite_floats_helper(
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
        "0.9",
        "0.9999961256982032864467",
        "1.0",
        "0.500003814697151939896999",
        "0.5",
        "1.9999999999999999930611061477915599069157972491",
        "1.00024414062499999999661186901877636521388460647575948049212",
        "0.50003046",
        "1.992195248600296508811879489",
        "0.99609375",
        "1.998046875931315",
        "1.99609375",
        "2.0",
        "0.9990234375",
        "0.50000004470348358154466281589450860049005755",
        "1.9999998807907104",
        "0.12499952404",
        "0.5000000000000000017347232691817032599608",
        "0.24999999999272",
        "0.99264526",
    ];
    let values_hex = &[
        "0x0.e#3",
        "0x0.ffffbf00001ffffffe#71",
        "0x1.0000000000#38",
        "0x0.80003fffffe0001ffffc#79",
        "0x0.80000#19",
        "0x1.ffffffffffffff8000000fffffffffffc003ff#153",
        "0x1.000ffffffffffffff000003fffffff8000000000000000ffc#195",
        "0x0.8001ff#24",
        "0x1.fe0081fffc1fffffffff80#89",
        "0x0.ff0000000000000#57",
        "0x1.ff800003fffe#48",
        "0x1.ff000000#31",
        "0x2.00000000000000#57",
        "0x0.ffc000000000#48",
        "0x0.800000c00000000007ffffffffffffc000000#145",
        "0x1.fffffe00000000#54",
        "0x0.1ffff803c#31",
        "0x0.800000000000001fffffc00000ffffffc#132",
        "0x0.3ffffffff800#43",
        "0x0.fe1e00#24",
    ];
    let common_values = &[
        ("0.5", 5069),
        ("0.2", 2623),
        ("1.0", 2623),
        ("0.5", 2567),
        ("0.8", 2547),
        ("0.9", 2419),
        ("0.5", 2408),
        ("0.94", 2381),
        ("0.5", 2336),
        ("0.5", 2240),
    ];
    let common_values_hex = &[
        ("0x0.8#1", 5069),
        ("0x0.4#1", 2623),
        ("0x1.0#1", 2623),
        ("0x0.8#2", 2567),
        ("0x0.c#2", 2547),
        ("0x0.e#3", 2419),
        ("0x0.8#3", 2408),
        ("0x0.f#4", 2381),
        ("0x0.8#4", 2336),
        ("0x0.80#6", 2240),
    ];
    let sample_median = ("0.7499998808", Some("0.7499998808"));
    let sample_median_hex = ("0x0.bffffe00#32", Some("0x0.bffffe000#33"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(4.651672278298667),
        standard_deviation: NiceFloat(258.23481359728345),
        skewness: NiceFloat(332.951705809113),
        excess_kurtosis: NiceFloat(144341.98602578667),
    };
    striped_random_positive_finite_floats_helper(
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
        "1.0e-37", "4.0e-34", "2.0e-6", "1.0e-45", "1.0e15", "2.0e-10", "8.0e3", "0.055",
        "0.00006", "7.0e13", "1.0e34", "5.0e-13", "0.00006", "4.0e9", "6.0e32", "0.06", "0.00002",
        "2.0e18", "0.03", "2.7e8",
    ];
    let values_hex = &[
        "0x3.0E-31#2",
        "0x2.0E-28#1",
        "0x0.00002#2",
        "0x8.0E-38#2",
        "0x4.0E+12#1",
        "0x1.0E-8#1",
        "0x2.0E+3#1",
        "0x0.0e#3",
        "0x0.0004#1",
        "0x4.0E+11#1",
        "0x2.0E+28#1",
        "0x8.0E-11#1",
        "0x0.0004#1",
        "0x1.0E+8#1",
        "0x2.0E+27#3",
        "0x0.10#4",
        "0x0.0001#1",
        "0x2.0E+15#1",
        "0x0.08#1",
        "0x1.0E+7#3",
    ];
    let common_values = &[
        ("0.5", 3831),
        ("0.1", 3830),
        ("0.2", 3826),
        ("1.0", 3753),
        ("2.0", 3716),
        ("4.0", 3653),
        ("0.06", 3639),
        ("0.03", 3637),
        ("2.0e1", 3632),
        ("8.0", 3617),
    ];
    let common_values_hex = &[
        ("0x0.8#1", 3831),
        ("0x0.2#1", 3830),
        ("0x0.4#1", 3826),
        ("0x1.0#1", 3753),
        ("0x2.0#1", 3716),
        ("0x4.0#1", 3653),
        ("0x0.1#1", 3639),
        ("0x0.08#1", 3637),
        ("0x1.0E+1#1", 3632),
        ("0x8.0#1", 3617),
    ];
    let sample_median = ("0.5", None);
    let sample_median_hex = ("0x0.8#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.4760378686777754e255),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_positive_finite_floats_helper(
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
        "1.6e-37",
        "7.703689931013567876617e-34",
        "1.90734863281e-6",
        "1.401309155383338826185809e-45",
        "1.1259e15",
        "4.6566128730773925619691287814442997493683296468e-10",
        "8193.9999999999999999722444310018159838321426962494216641914",
        "0.031251904",
        "0.0001215939482788266912116625665",
        "140187732541440.0",
        "2.0748905034207e34",
        "9.077183449e-13",
        "0.00006103515625",
        "8581545984.0",
        "649037165345292795070711450304511.99975585938",
        "0.124999992549419403",
        "0.00003051746192",
        "2305843009213693959.999999046325910967422",
        "0.06249999999818",
        "5.3292237e8",
    ];
    let values_hex = &[
        "0x3.8E-31#3",
        "0x3.fffefc00007ffffff8E-28#71",
        "0x0.00002000000000#38",
        "0x8.0003fffffe0001ffffcE-38#79",
        "0x4.0000E+12#19",
        "0x1.ffffffffffffff8000000fffffffffffc003ffE-8#153",
        "0x2001.fffffffffffffe000007fffffff0000000000000001ff8#195",
        "0x0.08001ff#24",
        "0x0.0007f80207fff07ffffffffe00#89",
        "0x7f8000000000.000#57",
        "0x3.ff000007fffcE+28#48",
        "0xf.f800000E-11#31",
        "0x0.000400000000000000#57",
        "0x1ff800000.0000#48",
        "0x200000300000000001ffffffffff.fff000000#145",
        "0x0.1fffffe00000000#54",
        "0x0.0001ffff803c#31",
        "0x2000000000000007.fffff000003ffffff0#132",
        "0x0.0ffffffffe00#43",
        "0x1.fc3c00E+7#24",
    ];
    let common_values = &[
        ("1.0", 130),
        ("8.0", 129),
        ("0.1", 122),
        ("2.0e1", 117),
        ("0.002", 116),
        ("7.0e4", 114),
        ("0.008", 112),
        ("8.0e-6", 112),
        ("0.06", 109),
        ("4.0", 108),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 130),
        ("0x8.0#1", 129),
        ("0x0.2#1", 122),
        ("0x1.0E+1#1", 117),
        ("0x0.008#1", 116),
        ("0x1.0E+4#1", 114),
        ("0x0.02#1", 112),
        ("0x0.00008#1", 112),
        ("0x0.1#1", 109),
        ("0x4.0#1", 108),
    ];
    let sample_median = ("0.562499940395355", Some("0.56249994039535611273"));
    let sample_median_hex = ("0x0.8fffff0000000#49", Some("0x0.8fffff0000003fff#64"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.9027050650923932e255),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_positive_finite_floats_helper(
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
fn striped_random_positive_finite_floats_fail_1() {
    striped_random_positive_finite_floats(EXAMPLE_SEED, 1, 0, 2, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_finite_floats_fail_2() {
    striped_random_positive_finite_floats(EXAMPLE_SEED, 0, 1, 2, 1, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_finite_floats_fail_3() {
    striped_random_positive_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_positive_finite_floats_fail_4() {
    striped_random_positive_finite_floats(EXAMPLE_SEED, 1, 1, 2, 1, 1, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_finite_floats_fail_5() {
    striped_random_positive_finite_floats(EXAMPLE_SEED, 1, 1, 2, 3, 2, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_finite_floats_fail_6() {
    striped_random_positive_finite_floats(EXAMPLE_SEED, 1, 1, 1, 0, 2, 1);
}
