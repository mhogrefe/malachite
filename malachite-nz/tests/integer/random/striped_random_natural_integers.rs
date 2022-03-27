use itertools::Itertools;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::{moment_stats, MomentStats};
use malachite_nz::integer::random::striped_random_natural_integers;
use malachite_nz::integer::Integer;

fn striped_random_natural_integers_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    let xs = striped_random_natural_integers(
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
    let actual_sample_moment_stats = moment_stats(xs.take(1000000).map(|x| f64::from(&x)));
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
fn test_striped_random_natural_integers() {
    // mean bits = 1/64
    let values = &["0"; 20];
    let common_values =
        &[("0", 984681), ("1", 15077), ("3", 120), ("2", 117), ("4", 3), ("5", 1), ("7", 1)];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.015695000000000875),
        standard_deviation: NiceFloat(0.12845498618458842),
        skewness: NiceFloat(9.02636021695415),
        excess_kurtosis: NiceFloat(104.38317092740806),
    };
    striped_random_natural_integers_helper(
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
        "0", "8", "0", "8", "2", "4", "1", "0", "0", "0", "0", "0", "1", "1", "0", "0", "1", "1",
        "0", "0",
    ];
    let common_values = &[
        ("0", 500248),
        ("1", 249491),
        ("3", 62636),
        ("2", 62505),
        ("4", 23595),
        ("7", 23447),
        ("8", 8713),
        ("15", 8690),
        ("6", 7938),
        ("5", 7832),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.16843100000002),
        standard_deviation: NiceFloat(782.5565010647151),
        skewness: NiceFloat(800.2073401417995),
        excess_kurtosis: NiceFloat(728738.7203924827),
    };
    striped_random_natural_integers_helper(
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
        "18014656207519744",
        "2228160",
        "64",
        "17592184995840",
        "1179440951012584587264",
        "9007749010526207",
        "67108864",
        "5",
        "24",
        "34359738879",
        "2417851639228158863474687",
        "512",
        "9444737328601429442560",
        "8",
        "131071",
        "524032",
        "8388607",
        "34359738368",
        "60",
        "2147741695",
    ];
    let common_values = &[
        ("0", 30467),
        ("1", 29379),
        ("3", 14232),
        ("2", 14195),
        ("4", 13131),
        ("7", 13019),
        ("8", 11921),
        ("15", 11751),
        ("31", 10682),
        ("16", 10555),
    ];
    let sample_median = ("3670016", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.459178425232889e129),
        standard_deviation: NiceFloat(1.459178425232619e132),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_natural_integers_helper(
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
        "1473193827441715154886135497317777215948837626052608",
        "1152921504606846976",
        "16777216",
        "4128768",
        "1180591620717412351744",
        "127",
        "1",
        "1073741823",
        "4722366482869645209600",
        "1267650600226049676594364547199",
        "288230376151711743",
        "8192",
        "274869520368",
        "1152921504606846976",
        "5317074242107007699768820031345917967",
        "1024",
        "8191",
        "4398046511104",
        "11417981541647679048466288345891489974790914528",
        "2251799813685247",
    ];
    let common_values = &[
        ("0", 15386),
        ("1", 15062),
        ("2", 7584),
        ("3", 7467),
        ("4", 7110),
        ("7", 7017),
        ("8", 6866),
        ("15", 6763),
        ("31", 6505),
        ("16", 6460),
    ];
    let sample_median = ("17592169267200", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.6414828903095017e263),
        standard_deviation: NiceFloat(f64::POSITIVE_INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_natural_integers_helper(
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
fn striped_random_natural_integers_fail_1() {
    striped_random_natural_integers(EXAMPLE_SEED, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_integers_fail_2() {
    striped_random_natural_integers(EXAMPLE_SEED, 2, 3, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_integers_fail_3() {
    striped_random_natural_integers(EXAMPLE_SEED, 4, 1, 0, 1);
}

#[test]
#[should_panic]
fn striped_random_natural_integers_fail_4() {
    striped_random_natural_integers(EXAMPLE_SEED, 4, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_natural_integers_fail_5() {
    striped_random_natural_integers(EXAMPLE_SEED, 4, 1, u64::MAX, u64::MAX - 1);
}
