use itertools::Itertools;
use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::{moment_stats, MomentStats};
use malachite_nz::integer::random::random_integer_inclusive_range;
use malachite_nz::integer::Integer;
use malachite_nz_test_util::integer::IntegerCheckedToF64Wrapper;
use std::str::FromStr;

fn random_integer_inclusive_range_helper(
    a: &str,
    b: &str,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    let a = Integer::from_str(a).unwrap();
    let b = Integer::from_str(b).unwrap();
    let xs = random_integer_inclusive_range(
        EXAMPLE_SEED,
        a,
        b,
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
fn test_random_integer_inclusive_range() {
    let values = &["0"; 20];
    let common_values = &[("0", 1000000)];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_integer_inclusive_range_helper(
        "0",
        "0",
        1,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "1", "0", "1", "-1", "-1", "-4", "-2", "-2", "-1", "-1", "0", "1", "2", "-4", "0", "1",
        "0", "0", "1", "0",
    ];
    let common_values = &[
        ("0", 284116),
        ("1", 189679),
        ("-1", 189332),
        ("-4", 84500),
        ("3", 63397),
        ("2", 63173),
        ("-3", 62961),
        ("-2", 62842),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.3356830000000005),
        standard_deviation: NiceFloat(1.8054398863225456),
        skewness: NiceFloat(-0.35221934475763134),
        excess_kurtosis: NiceFloat(-0.2458978296075136),
    };
    random_integer_inclusive_range_helper(
        "-4",
        "3",
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "1023", "1022", "1023", "1023", "1023", "1022", "1023", "1024", "1024", "1025", "1025",
        "1022", "1023", "1023", "1023", "1022", "1025", "1024", "1024", "1024",
    ];
    let common_values = &[("1023", 300404), ("1022", 299811), ("1025", 200144), ("1024", 199641)];
    let sample_median = ("1023", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1023.3001179999817),
        standard_deviation: NiceFloat(1.0999810889439465),
        skewness: NiceFloat(0.2889412070926685),
        excess_kurtosis: NiceFloat(-1.2389995110068848),
    };
    random_integer_inclusive_range_helper(
        "1022",
        "1025",
        12,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "-1023", "-1023", "-1023", "-1023", "-1023", "-1023", "-1023", "-1025", "-1024", "-1025",
        "-1024", "-1023", "-1023", "-1023", "-1023", "-1023", "-1025", "-1026", "-1024", "-1024",
    ];
    let common_values =
        &[("-1023", 600215), ("-1024", 133294), ("-1026", 133261), ("-1025", 133230)];
    let sample_median = ("-1023", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1023.7995370000281),
        standard_deviation: NiceFloat(1.1073864781257785),
        skewness: NiceFloat(-0.990171399672332),
        excess_kurtosis: NiceFloat(-0.5751529612720772),
    };
    random_integer_inclusive_range_helper(
        "-1026",
        "-1023",
        12,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "2", "152", "1", "0", "-62", "5282", "0", "28", "-4", "-79", "-11", "2", "1", "-1", "82",
        "-1", "696", "-6", "-39", "1421",
    ];
    let common_values = &[
        ("0", 118542),
        ("1", 95287),
        ("-1", 95269),
        ("-3", 38248),
        ("3", 38202),
        ("2", 38150),
        ("-2", 38078),
        ("-4", 15429),
        ("7", 15423),
        ("-5", 15382),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(130.2935679999988),
        standard_deviation: NiceFloat(901.4375229872913),
        skewness: NiceFloat(7.858028656993447),
        excess_kurtosis: NiceFloat(67.9560213744922),
    };
    random_integer_inclusive_range_helper(
        "-1000",
        "9999",
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn random_integer_inclusive_range_fail_1() {
    random_integer_inclusive_range(EXAMPLE_SEED, Integer::from(-100), Integer::from(-10), 1, 0);
}

#[test]
#[should_panic]
fn random_integer_inclusive_range_fail_2() {
    random_integer_inclusive_range(EXAMPLE_SEED, Integer::from(-100), Integer::from(-10), 4, 1);
}

#[test]
#[should_panic]
fn random_integer_inclusive_range_fail_3() {
    random_integer_inclusive_range(EXAMPLE_SEED, Integer::from(-9), Integer::from(-10), 10, 1);
}
