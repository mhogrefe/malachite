use itertools::Itertools;
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::{moment_stats, MomentStats};
use malachite_nz::integer::random::striped_random_integers;
use malachite_nz::integer::Integer;
use malachite_nz_test_util::integer::IntegerCheckedToF64Wrapper;

fn striped_random_integers_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    let xs = striped_random_integers(
        EXAMPLE_SEED,
        mean_stripe_numerator,
        mean_stripe_denominator,
        mean_bits_numerator,
        mean_bits_denominator,
    );
    let actual_values = xs
        .clone()
        .map(|x| Integer::to_string(&x))
        .take(20)
        .collect_vec();
    let actual_values = actual_values.iter().map(String::as_str).collect_vec();
    let actual_common_values = common_values_map(1000000, 10, xs.clone())
        .into_iter()
        .map(|(x, freq)| (x.to_string(), freq))
        .collect_vec();
    let actual_common_values = actual_common_values
        .iter()
        .map(|(x, freq)| (x.as_str(), *freq))
        .collect_vec();
    let (median_lo, median_hi) = median(xs.clone().take(1000000));
    let (median_lo, median_hi) = (
        median_lo.to_string(),
        median_hi.map(|x| Integer::to_string(&x)),
    );
    let actual_sample_median = (median_lo.as_str(), median_hi.as_deref());
    // Note that the population moments do not exist.
    let actual_sample_moment_stats = moment_stats(xs.take(1000000).map(IntegerCheckedToF64Wrapper));
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_common_values.as_slice(),
            actual_sample_median,
            actual_sample_moment_stats
        ),
        (
            expected_values,
            expected_common_values,
            expected_sample_median,
            expected_sample_moment_stats
        )
    );
}

#[test]
fn test_striped_random_integers() {
    // mean bits = 1/64
    let values = &["0"; 20];
    let common_values = &[
        ("0", 969830),
        ("1", 14858),
        ("-1", 14856),
        ("-3", 135),
        ("2", 115),
        ("-2", 115),
        ("3", 84),
        ("-7", 3),
        ("4", 2),
        ("7", 1),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.00016100000000001123),
        standard_deviation: NiceFloat(0.18124295000911678),
        skewness: NiceFloat(-0.332760005499994),
        excess_kurtosis: NiceFloat(53.997755862907425),
    };
    striped_random_integers_helper(
        4,
        1,
        1,
        64,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 1
    let values = &[
        "2", "-2", "-4", "0", "0", "-1", "-2", "-3", "2", "0", "0", "-1", "-4", "-3", "7", "0",
        "1", "1", "1", "-3",
    ];
    let common_values = &[
        ("0", 332922),
        ("-1", 166652),
        ("1", 166524),
        ("2", 42176),
        ("3", 41573),
        ("-2", 41508),
        ("-3", 41328),
        ("4", 15789),
        ("-7", 15751),
        ("7", 15560),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.8722809999999835),
        standard_deviation: NiceFloat(809.560511708087),
        skewness: NiceFloat(-339.00725481029485),
        excess_kurtosis: NiceFloat(238310.7740951809),
    };
    striped_random_integers_helper(
        4,
        1,
        1,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 32
    let values = &[
        "65536",
        "75521006248971741167616",
        "32",
        "-2199023255520",
        "-68719468544",
        "-527",
        "0",
        "-112",
        "131071",
        "4152",
        "262143",
        "-262145",
        "-8192",
        "-137405429760",
        "-4294967296",
        "1219",
        "16",
        "-1023",
        "-32768",
        "-32",
    ];
    let common_values = &[
        ("0", 15405),
        ("1", 15074),
        ("-1", 14891),
        ("-3", 7324),
        ("2", 7197),
        ("3", 7140),
        ("-2", 7136),
        ("7", 6709),
        ("-7", 6675),
        ("-4", 6660),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.1248652082766593e155),
        standard_deviation: NiceFloat(f64::POSITIVE_INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_integers_helper(
        16,
        1,
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 64
    let values = &[
        "8192",
        "178427569518544464724715670468776264076361728",
        "8176",
        "-262144",
        "-268435456",
        "-226655146685469074391039",
        "4294967296",
        "-67108863",
        "-19807040628566083848630173696",
        "45671926166590716193865150952632647489410830335",
        "43978334404607",
        "252172839656924666985926477663676528888687738185461429445660194859797887186474365257113263\
        9068666062843684114535546880",
        "-1728806579227565766676057273846916536097145074328900789155504620306432",
        "-4194304",
        "-16777215",
        "-1",
        "43556142803623322374103370143943282917375",
        "31742",
        "-4123168604160",
        "-129703669268270284799",
    ];
    let common_values = &[
        ("0", 7696),
        ("-1", 7685),
        ("1", 7575),
        ("-2", 3831),
        ("-3", 3742),
        ("3", 3735),
        ("2", 3661),
        ("-4", 3643),
        ("7", 3615),
        ("4", 3570),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.346385398054525e248),
        standard_deviation: NiceFloat(f64::POSITIVE_INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_integers_helper(
        32,
        1,
        64,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn striped_random_integers_fail_1() {
    striped_random_integers(EXAMPLE_SEED, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_integers_fail_2() {
    striped_random_integers(EXAMPLE_SEED, 2, 3, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_integers_fail_3() {
    striped_random_integers(EXAMPLE_SEED, 4, 1, 0, 1);
}

#[test]
#[should_panic]
fn striped_random_integers_fail_4() {
    striped_random_integers(EXAMPLE_SEED, 4, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_integers_fail_5() {
    striped_random_integers(EXAMPLE_SEED, 4, 1, u64::MAX, u64::MAX - 1);
}
