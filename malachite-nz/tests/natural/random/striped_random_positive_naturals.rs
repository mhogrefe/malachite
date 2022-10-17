use itertools::Itertools;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_base::test_util::stats::median;
use malachite_base::test_util::stats::moments::{moment_stats, MomentStats};
use malachite_nz::natural::random::striped_random_positive_naturals;
use malachite_nz::natural::Natural;

fn striped_random_positive_naturals_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    let xs = striped_random_positive_naturals(
        EXAMPLE_SEED,
        mean_stripe_numerator,
        mean_stripe_denominator,
        mean_bits_numerator,
        mean_bits_denominator,
    );
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
    let actual_sample_moment_stats = moment_stats(
        xs.take(1000000)
            .map(|x| f64::rounding_from(&x, RoundingMode::Nearest)),
    );
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
fn test_striped_random_positive_naturals() {
    // mean bits = 65/64
    let values = &["1"; 20];
    let common_values = &[
        ("1", 984681),
        ("3", 7637),
        ("2", 7440),
        ("4", 97),
        ("7", 78),
        ("5", 33),
        ("6", 29),
        ("11", 2),
        ("8", 1),
        ("9", 1),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.023796999999977),
        standard_deviation: NiceFloat(0.20691241046333197),
        skewness: NiceFloat(10.752762867801868),
        excess_kurtosis: NiceFloat(162.40220891738076),
    };
    striped_random_positive_naturals_helper(
        4,
        1,
        65,
        64,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 2
    let values = &[
        "1", "16", "1", "16", "4", "15", "2", "1", "1", "1", "1", "1", "2", "2", "1", "1", "3",
        "2", "1", "1",
    ];
    let common_values = &[
        ("1", 500248),
        ("2", 124818),
        ("3", 124673),
        ("7", 47032),
        ("4", 46853),
        ("8", 17749),
        ("15", 17612),
        ("5", 15660),
        ("6", 15596),
        ("16", 6518),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(15.773014000000762),
        standard_deviation: NiceFloat(2128.8810534178506),
        skewness: NiceFloat(884.8410850537254),
        excess_kurtosis: NiceFloat(843254.2507640689),
    };
    striped_random_positive_naturals_helper(
        4,
        1,
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 32
    let values = &[
        "16",
        "4",
        "128",
        "34391195648",
        "75493376",
        "9007199120523391",
        "8",
        "8796094070783",
        "8",
        "950737950171027935941967741439",
        "1040391",
        "33554432",
        "84390026996392738938916902643112875376640",
        "30",
        "7",
        "554153860399104",
        "2199023255551",
        "16",
        "220784470296873664512",
        "4611685966886694919",
    ];
    let common_values = &[
        ("1", 31094),
        ("3", 15250),
        ("2", 15195),
        ("7", 13890),
        ("4", 13880),
        ("8", 12601),
        ("15", 12519),
        ("31", 11397),
        ("16", 11237),
        ("63", 10225),
    ];
    let sample_median = ("4194272", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.1641486095650758e130),
        standard_deviation: NiceFloat(1.1640776313097e133),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_positive_naturals_helper(
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
        "649037107316853596555600210165760",
        "822486237184",
        "196608",
        "141976867225561692967630759002112",
        "174223242635524708377374895198005052307456",
        "664594824829454142366461086851399679",
        "4503599627370496",
        "127",
        "2048",
        "147574233996470517759",
        "45660775794157599311165096735019350831520546815",
        "1048320",
        "1361212544433490269424560431578983940096",
        "3968",
        "536870911",
        "34359738368",
        "35184372088831",
        "9297159013149614014464",
        "768",
        "72040001986101247",
    ];
    let common_values = &[
        ("1", 15720),
        ("2", 7656),
        ("3", 7646),
        ("7", 7219),
        ("4", 7199),
        ("8", 7122),
        ("15", 6934),
        ("31", 6799),
        ("16", 6750),
        ("63", 6456),
    ];
    let sample_median = ("17592186044416", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.796251092974677e283),
        standard_deviation: NiceFloat(f64::POSITIVE_INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_positive_naturals_helper(
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
fn striped_random_positive_naturals_fail_1() {
    striped_random_positive_naturals(EXAMPLE_SEED, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_naturals_fail_2() {
    striped_random_positive_naturals(EXAMPLE_SEED, 2, 3, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_naturals_fail_3() {
    striped_random_positive_naturals(EXAMPLE_SEED, 4, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_positive_naturals_fail_4() {
    striped_random_positive_naturals(EXAMPLE_SEED, 4, 1, 2, 3);
}
