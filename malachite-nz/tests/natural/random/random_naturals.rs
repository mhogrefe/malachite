use itertools::Itertools;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::{moment_stats, MomentStats};
use malachite_nz::natural::random::random_naturals;
use malachite_nz::natural::Natural;

fn random_naturals_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    let xs = random_naturals(EXAMPLE_SEED, mean_bits_numerator, mean_bits_denominator);
    let actual_values = xs
        .clone()
        .map(|x| Natural::to_string(&x))
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
        median_hi.map(|x| Natural::to_string(&x)),
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
fn test_random_naturals() {
    // mean bits = 1/64
    let values = &["0"; 20];
    let common_values = &[
        ("0", 984681),
        ("1", 15077),
        ("2", 121),
        ("3", 116),
        ("6", 2),
        ("4", 1),
        ("5", 1),
        ("7", 1),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.015695000000000257),
        standard_deviation: NiceFloat(0.12853281096935348),
        skewness: NiceFloat(9.11690327111834),
        excess_kurtosis: NiceFloat(110.73931175909136),
    };
    random_naturals_helper(
        1,
        64,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 1
    let values = &[
        "0", "14", "0", "8", "2", "6", "1", "0", "0", "0", "0", "0", "1", "1", "0", "0", "1", "1",
        "0", "0",
    ];
    let common_values = &[
        ("0", 500248),
        ("1", 249491),
        ("2", 62676),
        ("3", 62465),
        ("7", 15819),
        ("5", 15781),
        ("6", 15694),
        ("4", 15518),
        ("13", 3945),
        ("8", 3895),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.289019000000012),
        standard_deviation: NiceFloat(811.503067487901),
        skewness: NiceFloat(791.581366511165),
        excess_kurtosis: NiceFloat(717047.0759703598),
    };
    random_naturals_helper(
        1,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 32
    let values = &[
        "20431208470830262",
        "2777240",
        "114",
        "12184833305054",
        "1121025855008623490210",
        "13478874522577592",
        "115311695",
        "7",
        "18",
        "54522366353",
        "2183264193236231773387459",
        "824",
        "18558864232439549193912",
        "15",
        "110989",
        "453270",
        "4307150",
        "45388024541",
        "47",
        "3345913274",
    ];
    let common_values = &[
        ("0", 30467),
        ("1", 29379),
        ("3", 14233),
        ("2", 14194),
        ("7", 6984),
        ("6", 6980),
        ("4", 6964),
        ("5", 6929),
        ("10", 3479),
        ("15", 3431),
    ];
    let sample_median = ("3201388", Some("3201522"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.480305129633914e129),
        standard_deviation: NiceFloat(2.4803051296331898e132),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_naturals_helper(
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 64
    let values = &[
        "1049807948069596877906281043152861735368289016372406",
        "1388880088667859422",
        "26145954",
        "3731388",
        "1470862095575962348216",
        "99",
        "1",
        "835275153",
        "3892061391890507266755",
        "925334710331614885833504493368",
        "221414670923422190",
        "11239",
        "254772031885",
        "1351005164080654998",
        "9136414433496904064275246960259217614",
        "1775",
        "5562",
        "8137327159764",
        "19744859531291384657393101375027010425831988999",
        "2078424122508695",
    ];
    let common_values = &[
        ("0", 15386),
        ("1", 15062),
        ("2", 7592),
        ("3", 7459),
        ("4", 3719),
        ("5", 3707),
        ("6", 3685),
        ("7", 3508),
        ("12", 1906),
        ("11", 1865),
    ];
    let sample_median = ("15157534309527", Some("15157859817105"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.8099447055615434e263),
        standard_deviation: NiceFloat(f64::POSITIVE_INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_naturals_helper(
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
fn random_naturals_fail_1() {
    random_naturals(EXAMPLE_SEED, 0, 1);
}

#[test]
#[should_panic]
fn random_naturals_fail_2() {
    random_naturals(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_naturals_fail_3() {
    random_naturals(EXAMPLE_SEED, u64::MAX, u64::MAX - 1);
}
