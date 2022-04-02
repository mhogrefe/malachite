use itertools::Itertools;
use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_base::test_util::stats::median;
use malachite_base::test_util::stats::moments::{moment_stats, MomentStats};
use malachite_nz::integer::random::uniform_random_integer_inclusive_range;
use malachite_nz::integer::Integer;
use std::str::FromStr;

fn uniform_random_integer_inclusive_range_helper(
    a: &str,
    b: &str,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    let a = Integer::from_str(a).unwrap();
    let b = Integer::from_str(b).unwrap();
    let xs = uniform_random_integer_inclusive_range(EXAMPLE_SEED, a, b);
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
fn test_uniform_random_integer_inclusive_range() {
    let values = &["0"; 20];
    let common_values = &[("0", 1000000)];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    uniform_random_integer_inclusive_range_helper(
        "0",
        "0",
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "-3", "3", "1", "1", "3", "-3", "-2", "-4", "-2", "-1", "0", "-1", "2", "1", "2", "2",
        "-1", "-3", "-1", "3",
    ];
    let common_values = &[
        ("-2", 125739),
        ("-1", 125293),
        ("2", 125220),
        ("3", 125016),
        ("-4", 124976),
        ("1", 124665),
        ("-3", 124627),
        ("0", 124464),
    ];
    let sample_median = ("-1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.5004029999999962),
        standard_deviation: NiceFloat(2.291244004332587),
        skewness: NiceFloat(0.0011305808123400524),
        excess_kurtosis: NiceFloat(-1.2381429974564255),
    };
    uniform_random_integer_inclusive_range_helper(
        "-4",
        "3",
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "1023", "1025", "1023", "1023", "1025", "1023", "1024", "1022", "1024", "1025", "1022",
        "1025", "1024", "1023", "1024", "1024", "1025", "1023", "1025", "1025",
    ];
    let common_values = &[("1024", 250959), ("1025", 250309), ("1022", 249440), ("1023", 249292)];
    let sample_median = ("1024", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1023.5021369999873),
        standard_deviation: NiceFloat(1.1178079811513417),
        skewness: NiceFloat(-0.003486279405775989),
        excess_kurtosis: NiceFloat(-1.3594690811608392),
    };
    uniform_random_integer_inclusive_range_helper(
        "1022",
        "1025",
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "-1025", "-1023", "-1025", "-1025", "-1023", "-1025", "-1024", "-1026", "-1024", "-1023",
        "-1026", "-1023", "-1024", "-1025", "-1024", "-1024", "-1023", "-1025", "-1023", "-1023",
    ];
    let common_values =
        &[("-1024", 250959), ("-1023", 250309), ("-1026", 249440), ("-1025", 249292)];
    let sample_median = ("-1024", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1024.4978629999973),
        standard_deviation: NiceFloat(1.1178079811512938),
        skewness: NiceFloat(-0.0034862794057047334),
        excess_kurtosis: NiceFloat(-1.3594690811609873),
    };
    uniform_random_integer_inclusive_range_helper(
        "-1026",
        "-1023",
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "7279", "1141", "3725", "8735", "8576", "4611", "-916", "5299", "2421", "2094", "8430",
        "-253", "5785", "1183", "9525", "1201", "3280", "3065", "6206", "8542",
    ];
    let common_values = &[
        ("3214", 127),
        ("7954", 126),
        ("2592", 125),
        ("6885", 125),
        ("7656", 125),
        ("2344", 124),
        ("6392", 124),
        ("4426", 123),
        ("-312", 122),
        ("1519", 122),
    ];
    let sample_median = ("4509", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(4503.707855999995),
        standard_deviation: NiceFloat(3172.2131846202396),
        skewness: NiceFloat(-0.0021819995553274013),
        excess_kurtosis: NiceFloat(-1.196969134950911),
    };
    uniform_random_integer_inclusive_range_helper(
        "-1000",
        "9999",
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn uniform_random_integer_inclusive_range_fail() {
    uniform_random_integer_inclusive_range(EXAMPLE_SEED, Integer::from(-9), Integer::from(-10));
}
