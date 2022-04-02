use itertools::Itertools;
use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_base::test_util::stats::median;
use malachite_base::test_util::stats::moments::{moment_stats, MomentStats};
use malachite_nz::integer::random::striped_random_nonzero_integers;
use malachite_nz::integer::Integer;

fn striped_random_nonzero_integers_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: Option<MomentStats>,
) {
    let xs = striped_random_nonzero_integers(
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
    let actual_sample_moment_stats = if expected_sample_moment_stats.is_some() {
        Some(moment_stats(xs.take(1000000).map(|x| f64::from(&x))))
    } else {
        None
    };
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
fn test_striped_random_nonzero_integers() {
    // mean bits = 65/64
    let values = &[
        "1", "1", "1", "-1", "-1", "-1", "1", "-1", "-1", "1", "1", "1", "-1", "-1", "-1", "-1",
        "1", "1", "-1", "-1",
    ];
    let common_values = &[
        ("1", 492842),
        ("-1", 491818),
        ("2", 3848),
        ("3", 3791),
        ("-3", 3753),
        ("-2", 3709),
        ("7", 50),
        ("4", 49),
        ("-4", 41),
        ("-7", 36),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0015040000000000353),
        standard_deviation: NiceFloat(1.0443710206123127),
        skewness: NiceFloat(0.0008346150701385402),
        excess_kurtosis: NiceFloat(-1.2794877665824687),
    };
    striped_random_nonzero_integers_helper(
        4,
        1,
        65,
        64,
        values,
        common_values,
        sample_median,
        Some(sample_moment_stats),
    );
    // mean bits = 2
    let values = &[
        "4", "1", "4", "-8", "-1", "-1", "1", "-1", "-2", "7", "7", "6", "-1", "-1", "-3", "-14",
        "1", "4", "-8", "-1",
    ];
    let common_values = &[
        ("1", 249934),
        ("-1", 249480),
        ("3", 62605),
        ("-3", 62544),
        ("2", 62495),
        ("-2", 62282),
        ("4", 23545),
        ("-7", 23428),
        ("7", 23343),
        ("-4", 23304),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.9232179999999961),
        standard_deviation: NiceFloat(942.1853934867996),
        skewness: NiceFloat(-30.544317259845204),
        excess_kurtosis: NiceFloat(179726.72807613286),
    };
    striped_random_nonzero_integers_helper(
        4,
        1,
        2,
        1,
        values,
        common_values,
        sample_median,
        Some(sample_moment_stats),
    );
    // mean bits = 32
    let values = &[
        "4",
        "268435456",
        "84405977732342160290572740160760316144",
        "-133169152",
        "-131064",
        "-2251834173421823",
        "1577058304",
        "-126100789566374399",
        "-76",
        "270335",
        "33554431",
        "262144",
        "-4398046511104",
        "-20352",
        "-1023",
        "-1",
        "63",
        "72057589742960640",
        "-1",
        "-8388607",
    ];
    let common_values = &[
        ("1", 15709),
        ("-1", 15677),
        ("-3", 7646),
        ("-2", 7603),
        ("3", 7564),
        ("2", 7494),
        ("4", 6925),
        ("7", 6916),
        ("-7", 6903),
        ("-4", 6802),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-2.5217288846207e111),
        standard_deviation: NiceFloat(2.5217283396166554e114),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_nonzero_integers_helper(
        16,
        1,
        32,
        1,
        values,
        common_values,
        sample_median,
        Some(sample_moment_stats),
    );
    // mean bits = 64
    let values = &[
        "67108864",
        "47890485651710580317658107747553101683567604294221824",
        "512",
        "-311675034947891977256960",
        "-309485009821345068724780928",
        "-1179647",
        "1",
        "-131071",
        "-17179869184",
        "1056767",
        "273820942303",
        "137438952504",
        "-536870912",
        "-36749372959343247360",
        "-4722366482869645217791",
        "-786432",
        "1023",
        "8388608",
        "-274911460352",
        "-24575",
    ];
    let common_values = &[
        ("1", 7878),
        ("-1", 7830),
        ("-3", 3940),
        ("-2", 3893),
        ("2", 3885),
        ("3", 3806),
        ("4", 3707),
        ("-7", 3694),
        ("-4", 3689),
        ("7", 3608),
    ];
    let sample_median = ("1", None);
    // No moment calculation; abs of some generated `Integer`s exceed `f64::MAX`
    striped_random_nonzero_integers_helper(
        32,
        1,
        64,
        1,
        values,
        common_values,
        sample_median,
        None,
    );
}

#[test]
#[should_panic]
fn striped_random_nonzero_integers_fail_1() {
    striped_random_nonzero_integers(EXAMPLE_SEED, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_integers_fail_2() {
    striped_random_nonzero_integers(EXAMPLE_SEED, 2, 3, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_nonzero_integers_fail_3() {
    striped_random_nonzero_integers(EXAMPLE_SEED, 4, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_nonzero_integers_fail_4() {
    striped_random_nonzero_integers(EXAMPLE_SEED, 4, 1, 2, 3);
}
