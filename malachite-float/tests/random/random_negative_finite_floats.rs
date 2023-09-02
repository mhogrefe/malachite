use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::random::random_negative_finite_floats;
use malachite_float::test_util::random::random_floats_helper_helper;

fn random_negative_finite_floats_helper(
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
        random_negative_finite_floats(
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
fn test_random_negative_finite_floats() {
    // mean |sci_exponent| 1, mean precision 2
    let values = &[
        "-0.8", "-0.5", "-1.0", "-0.5", "-0.5", "-1.0", "-1.0", "-0.6", "-1.0", "-0.5", "-1.0",
        "-1.0", "-2.0", "-0.5", "-0.9", "-1.2", "-0.06", "-0.5", "-0.1", "-0.6",
    ];
    let values_hex = &[
        "-0x0.c#2", "-0x0.8#1", "-0x1.0#2", "-0x0.8#2", "-0x0.8#1", "-0x1.0#1", "-0x1.0#1",
        "-0x0.a#3", "-0x1.0#1", "-0x0.8#1", "-0x1.0#1", "-0x1.0#1", "-0x2.0#1", "-0x0.8#1",
        "-0x0.e#3", "-0x1.4#4", "-0x0.1#1", "-0x0.8#1", "-0x0.2#1", "-0x0.a#3",
    ];
    let common_values = &[
        ("-0.5", 166114),
        ("-0.2", 83464),
        ("-1.0", 83434),
        ("-0.8", 42025),
        ("-0.1", 41531),
        ("-0.5", 41506),
        ("-2.0", 41483),
        ("-1.5", 21005),
        ("-1.0", 20892),
        ("-0.06", 20888),
    ];
    let common_values_hex = &[
        ("-0x0.8#1", 166114),
        ("-0x0.4#1", 83464),
        ("-0x1.0#1", 83434),
        ("-0x0.c#2", 42025),
        ("-0x0.2#1", 41531),
        ("-0x0.8#2", 41506),
        ("-0x2.0#1", 41483),
        ("-0x1.8#2", 21005),
        ("-0x1.0#2", 20892),
        ("-0x0.1#1", 20888),
    ];
    let sample_median = ("-0.5", None);
    let sample_median_hex = ("-0x0.8#2", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-3.6464237937138053),
        standard_deviation: NiceFloat(208.48505030002843),
        skewness: NiceFloat(-334.72226371600976),
        excess_kurtosis: NiceFloat(139648.08254659182),
    };
    random_negative_finite_floats_helper(
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
        "-0.9",
        "-0.6925035668890312650418",
        "-1.605881209063",
        "-0.990506747417030704628675",
        "-0.832636",
        "-1.5238754689275855490274534835768455582440285754",
        "-1.93323214541319290206845677531070805983540084390241660654293",
        "-0.56918591",
        "-1.502719129666093199125391422",
        "-0.657013864200053067",
        "-1.14774106784965",
        "-1.670535118",
        "-3.21417188991484648",
        "-0.527854400242237",
        "-0.54577074628712951508975537535646778830546403",
        "-1.3752106387416565",
        "-0.1109667999",
        "-0.5781797141893270384887263003861965780332",
        "-0.19798117053799",
        "-0.67483377",
    ];
    let values_hex = &[
        "-0x0.e#3",
        "-0x0.b147e9ec26d848ea18#71",
        "-0x1.9b1b07ea30#38",
        "-0x0.fd91d9a69f9c57b9281c#79",
        "-0x0.d527a#19",
        "-0x1.861cb3e6387c2226486fb609141d0afd231bed#153",
        "-0x1.eee84d48202601771e38afd0bdfc260a19aa6a3d6dc57f9e0#195",
        "-0x0.91b62b#24",
        "-0x1.80b2336cfd4cd648e16fda#89",
        "-0x0.a8320f83c2000a8#57",
        "-0x1.25d25bceb0bc#48",
        "-0x1.aba83084#31",
        "-0x3.36d3f80ee82298#57",
        "-0x0.8721774a1711#48",
        "-0x0.8bb7a1b26aaf1ff5a8ab8f12f79cf2945c350#145",
        "-0x1.600dcdee81b620#54",
        "-0x0.1c6851f88#31",
        "-0x0.940395f3a75e9211a45ac2954f251e317#132",
        "-0x0.32aee4dcaf38#43",
        "-0x0.acc1e8#24",
    ];
    let common_values = &[
        ("-0.5", 5069),
        ("-0.5", 2626),
        ("-0.2", 2623),
        ("-1.0", 2623),
        ("-0.8", 2488),
        ("-1.0", 1309),
        ("-0.1", 1301),
        ("-2.0", 1299),
        ("-0.6", 1293),
        ("-0.4", 1283),
    ];
    let common_values_hex = &[
        ("-0x0.8#1", 5069),
        ("-0x0.8#2", 2626),
        ("-0x0.4#1", 2623),
        ("-0x1.0#1", 2623),
        ("-0x0.c#2", 2488),
        ("-0x1.0#2", 1309),
        ("-0x0.2#1", 1301),
        ("-0x2.0#1", 1299),
        ("-0x0.a#3", 1293),
        ("-0x0.6#2", 1283),
    ];
    let sample_median = (
        "-0.746024386480478",
        Some("-0.746023112757506482497266106928906360272421050588543420130087"),
    );
    let sample_median_hex = (
        "-0x0.befb7445f3be0#49",
        Some("-0x0.befb5ee75a863b717ef7486b94b7d4da51661d718ae7467808#199"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-4.569313859457829),
        standard_deviation: NiceFloat(234.303398813093),
        skewness: NiceFloat(-309.74563573271297),
        excess_kurtosis: NiceFloat(127224.11304735676),
    };
    random_negative_finite_floats_helper(
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
        "-1.0e-37", "-4.0e-34", "-2.0e-6", "-1.0e-45", "-1.0e15", "-2.0e-10", "-8.0e3", "-0.04",
        "-0.00006", "-7.0e13", "-1.0e34", "-5.0e-13", "-0.00006", "-4.0e9", "-1.1e33", "-0.08",
        "-0.00002", "-2.0e18", "-0.03", "-3.4e8",
    ];
    let values_hex = &[
        "-0x3.0E-31#2",
        "-0x2.0E-28#1",
        "-0x0.00002#2",
        "-0x8.0E-38#2",
        "-0x4.0E+12#1",
        "-0x1.0E-8#1",
        "-0x2.0E+3#1",
        "-0x0.0a#3",
        "-0x0.0004#1",
        "-0x4.0E+11#1",
        "-0x2.0E+28#1",
        "-0x8.0E-11#1",
        "-0x0.0004#1",
        "-0x1.0E+8#1",
        "-0x3.8E+27#3",
        "-0x0.14#4",
        "-0x0.0001#1",
        "-0x2.0E+15#1",
        "-0x0.08#1",
        "-0x1.4E+7#3",
    ];
    let common_values = &[
        ("-0.5", 3831),
        ("-0.1", 3830),
        ("-0.2", 3826),
        ("-1.0", 3753),
        ("-2.0", 3716),
        ("-4.0", 3653),
        ("-0.06", 3639),
        ("-0.03", 3637),
        ("-2.0e1", 3632),
        ("-8.0", 3617),
    ];
    let common_values_hex = &[
        ("-0x0.8#1", 3831),
        ("-0x0.2#1", 3830),
        ("-0x0.4#1", 3826),
        ("-0x1.0#1", 3753),
        ("-0x2.0#1", 3716),
        ("-0x4.0#1", 3653),
        ("-0x0.1#1", 3639),
        ("-0x0.08#1", 3637),
        ("-0x1.0E+1#1", 3632),
        ("-0x8.0#1", 3617),
    ];
    let sample_median = ("-0.5", None);
    let sample_median_hex = ("-0x0.8#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-9.840252457852111e254),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_negative_finite_floats_helper(
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
        "-1.6e-37",
        "-5.334853424266217791859e-34",
        "-3.06297532857e-6",
        "-2.77599116811770918852638e-45",
        "-1.874929e15",
        "-3.548049062787521511845880848301680661374769582e-10",
        "-15837.0377352248762537447979033453204261716037132485968407997",
        "-0.03557412",
        "-0.0000917186968790340087356806288",
        "-92466481062144.0205",
        "-1.19188246820143e34",
        "-7.596714196e-13",
        "-0.000098088741757655227",
        "-4534234772.18021",
        "-708450932856717755249416271609319.23689407419",
        "-0.08595066492135353",
        "-0.00002709150388",
        "-2666383304065262724.410502472270114820267",
        "-0.049495292634496",
        "-3.6229862e8",
    ];
    let values_hex = &[
        "-0x3.8E-31#3",
        "-0x2.c51fa7b09b6123a860E-28#71",
        "-0x0.0000336360fd46#38",
        "-0xf.d91d9a69f9c57b9281cE-38#79",
        "-0x6.a93dE+12#19",
        "-0x1.861cb3e6387c2226486fb609141d0afd231bedE-8#153",
        "-0x3ddd.09a90404c02ee3c715fa17bf84c143354d47adb8aff3c0#195",
        "-0x0.091b62b#24",
        "-0x0.000602c8cdb3f533592385bf68#89",
        "-0x541907c1e100.054#57",
        "-0x2.4ba4b79d6178E+28#48",
        "-0xd.5d41842E-11#31",
        "-0x0.00066da7f01dd04530#57",
        "-0x10e42ee94.2e22#48",
        "-0x22ede86c9aabc7fd6a2ae3c4bde7.3ca5170d4#145",
        "-0x0.1600dcdee81b620#54",
        "-0x0.0001c6851f88#31",
        "-0x2500e57ce9d7a484.6916b0a553c9478c5c#132",
        "-0x0.0cabb9372bce#43",
        "-0x1.5983d0E+7#24",
    ];
    let common_values = &[
        ("-1.0", 130),
        ("-8.0", 129),
        ("-0.1", 122),
        ("-2.0e1", 117),
        ("-0.002", 116),
        ("-7.0e4", 114),
        ("-0.008", 112),
        ("-8.0e-6", 112),
        ("-0.06", 109),
        ("-4.0", 108),
    ];
    let common_values_hex = &[
        ("-0x1.0#1", 130),
        ("-0x8.0#1", 129),
        ("-0x0.2#1", 122),
        ("-0x1.0E+1#1", 117),
        ("-0x0.008#1", 116),
        ("-0x1.0E+4#1", 114),
        ("-0x0.02#1", 112),
        ("-0x0.00008#1", 112),
        ("-0x0.1#1", 109),
        ("-0x4.0#1", 108),
    ];
    let sample_median = (
        "-0.72638326182303358287114819073153992766993863954",
        Some("-0.726315"),
    );
    let sample_median_hex = (
        "-0x0.b9f440e1e448b7a0f85e7d4da4ac517428596d0#154",
        Some("-0x0.b9efc#18"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-9.955567916342348e254),
        standard_deviation: NiceFloat(f64::INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_negative_finite_floats_helper(
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
fn random_negative_finite_floats_fail_1() {
    random_negative_finite_floats(EXAMPLE_SEED, 1, 0, 2, 1);
}

#[test]
#[should_panic]
fn random_negative_finite_floats_fail_2() {
    random_negative_finite_floats(EXAMPLE_SEED, 0, 1, 2, 1);
}

#[test]
#[should_panic]
fn random_negative_finite_floats_fail_3() {
    random_negative_finite_floats(EXAMPLE_SEED, 1, 1, 1, 0);
}

#[test]
#[should_panic]
fn random_negative_finite_floats_fail_4() {
    random_negative_finite_floats(EXAMPLE_SEED, 1, 1, 1, 1);
}
