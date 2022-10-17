use itertools::Itertools;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_base::test_util::stats::median;
use malachite_base::test_util::stats::moments::{moment_stats, MomentStats};
use malachite_q::random::striped_random_positive_rationals;
use malachite_q::Rational;

#[allow(clippy::too_many_arguments)]
fn striped_random_positive_rationals_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    let xs = striped_random_positive_rationals(
        EXAMPLE_SEED,
        mean_stripe_numerator,
        mean_stripe_denominator,
        mean_bits_numerator,
        mean_bits_denominator,
    );
    let actual_values = xs
        .clone()
        .map(|x| Rational::to_string(&x))
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
        median_hi.map(|x| Rational::to_string(&x)),
    );
    let actual_sample_median = (median_lo.as_str(), median_hi.as_deref());
    // Note that the population moments do not exist.
    let actual_sample_moment_stats = moment_stats(
        xs.take(1000000)
            .map(|x| f64::rounding_from(x, RoundingMode::Nearest)),
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
fn test_striped_random_positive_rationals() {
    // mean bits = 65/64
    let values = &["1"; 20];
    let common_values = &[
        ("1", 969574),
        ("3", 7614),
        ("1/3", 7540),
        ("2", 7358),
        ("1/2", 7310),
        ("4", 98),
        ("7", 88),
        ("1/7", 87),
        ("1/4", 79),
        ("2/3", 59),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.0148534444805162),
        standard_deviation: NiceFloat(0.22114620623320444),
        skewness: NiceFloat(9.149432588003647),
        excess_kurtosis: NiceFloat(159.01272255850165),
    };
    striped_random_positive_rationals_helper(
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
        "1/16", "1/16", "4/15", "2", "1", "1", "1", "1", "3/2", "1", "1", "2/3", "1", "1/7", "2",
        "16", "1", "4", "2", "1/2",
    ];
    let common_values = &[
        ("1", 286322),
        ("2", 71839),
        ("1/2", 71792),
        ("1/3", 65670),
        ("3", 65630),
        ("1/4", 27230),
        ("4", 26986),
        ("7", 24307),
        ("1/7", 24217),
        ("2/3", 16638),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(8.675884348144963),
        standard_deviation: NiceFloat(452.3475394447746),
        skewness: NiceFloat(244.55477994852157),
        excess_kurtosis: NiceFloat(71058.94920001029),
    };
    striped_random_positive_rationals_helper(
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
        "4",
        "1/268681216",
        "75493376/9007199120523391",
        "8/8796094070783",
        "8/950737950171027935941967741439",
        "1040391/33554432",
        "2813000899879757964630563421437095845888",
        "1/79164837199872",
        "2199023255551/16",
        "220784470296873664512/4611685966886694919",
        "33/256",
        "16809472/144255925429997319",
        "6129981798088146185736712229649530847599712363400396804/786431",
        "1099511578623/1073741761",
        "2/65791",
        "18014398509490175/266208",
        "1/140752654954496",
        "2417842415927590238812160",
        "9444732965755934466048/7",
        "4194303/1073709056",
    ];
    let common_values = &[
        ("1", 3591),
        ("1/2", 1841),
        ("2", 1732),
        ("1/4", 1579),
        ("4", 1555),
        ("1/8", 1495),
        ("8", 1470),
        ("1/16", 1391),
        ("16", 1304),
        ("1/32", 1231),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.394348586566577e127),
        standard_deviation: NiceFloat(2.3943485865002743e130),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_positive_rationals_helper(
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
        "302231454903657360261120/383",
        "3/2166395068749415481073467392",
        "174223242635524708377374895198005052307456/664594824829454142366461086851399679",
        "4503599627370496/127",
        "2048/147574233996470517759",
        "9132155158831519862233019347003870166304109363/209664",
        "10634473003386642729879378371710812032/31",
        "536870911/34359738368",
        "5026338869833/1328165573307087716352",
        "768/72040001986101247",
        "18014261070561279/2786912585102768425368689128829376599687168",
        "133152",
        "3/545357767376900",
        "31/2251799813685247",
        "4398046511135/64",
        "8796093046784/4194303",
        "38685626236675332845338112/562949953420767",
        "4459452226323108777095472045064328031949030396/50331647",
        "2535298782614042945771878219776/70366596710399",
        "144115188075855871/154740143727431099539783680",
    ];
    let common_values = &[
        ("1", 1591),
        ("2", 794),
        ("1/2", 762),
        ("8", 757),
        ("16", 718),
        ("4", 691),
        ("1/4", 691),
        ("1/8", 689),
        ("1/16", 659),
        ("32", 650),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.3341225920157865e234),
        standard_deviation: NiceFloat(f64::POSITIVE_INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    striped_random_positive_rationals_helper(
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
fn striped_random_positive_rationals_fail_1() {
    striped_random_positive_rationals(EXAMPLE_SEED, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_rationals_fail_2() {
    striped_random_positive_rationals(EXAMPLE_SEED, 2, 3, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_positive_rationals_fail_3() {
    striped_random_positive_rationals(EXAMPLE_SEED, 4, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_positive_rationals_fail_4() {
    striped_random_positive_rationals(EXAMPLE_SEED, 4, 1, 2, 3);
}
