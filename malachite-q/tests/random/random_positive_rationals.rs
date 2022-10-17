use itertools::Itertools;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_base::test_util::stats::median;
use malachite_base::test_util::stats::moments::{moment_stats, MomentStats};
use malachite_q::random::random_positive_rationals;
use malachite_q::Rational;

fn random_positive_rationals_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    let xs = random_positive_rationals(EXAMPLE_SEED, mean_bits_numerator, mean_bits_denominator);
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
fn test_random_positive_rationals() {
    // mean bits = 65/64
    let values = &["1"; 20];
    let common_values = &[
        ("1", 969573),
        ("3", 7488),
        ("2", 7484),
        ("1/3", 7459),
        ("1/2", 7391),
        ("5", 75),
        ("6", 67),
        ("1/7", 67),
        ("2/3", 64),
        ("1/5", 61),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.0147365511904034),
        standard_deviation: NiceFloat(0.21959420511046546),
        skewness: NiceFloat(8.77512842724579),
        excess_kurtosis: NiceFloat(133.15117857730885),
    };
    random_positive_rationals_helper(
        65,
        64,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 2
    let values = &[
        "1/24", "1/30", "1/2", "2", "1", "1", "1", "1", "1", "1", "1", "1", "1", "1/7", "2", "19",
        "1", "4", "3", "1/2",
    ];
    let common_values = &[
        ("1", 284707),
        ("2", 71334),
        ("1/2", 71222),
        ("3", 68255),
        ("1/3", 68142),
        ("5", 18020),
        ("4", 17749),
        ("1/5", 17746),
        ("1/4", 17705),
        ("1/6", 17245),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(8.527473854511467),
        standard_deviation: NiceFloat(439.80801149679047),
        skewness: NiceFloat(309.19213173617015),
        excess_kurtosis: NiceFloat(131113.5392046833),
    };
    random_positive_rationals_helper(
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 32
    let values = &[
        "11/2",
        "89/27922830575",
        "46627409/3788983764809694",
        "8/11316951483471",
        "11/1005760138411689342464923704482",
        "948931/42716754",
        "81013760999253680590984897748479904878392/23",
        "1/97645164585502",
        "1558028859598/29",
        "200127331174844881647/4058622214797175252",
        "155/1413",
        "24470495/285805200646849943",
        "18939240741294741985527157685848850947887212663091378627/3070040",
        "545942890259/414324415",
        "4/209925",
        "128959877500520349/1134718",
        "2/424578084893903",
        "1956237739171878131383877",
        "17054902546906498751130/7",
        "782845/239707736",
    ];
    let common_values = &[
        ("1", 1810),
        ("1/2", 922),
        ("2", 915),
        ("3", 809),
        ("1/3", 776),
        ("1/4", 470),
        ("4", 426),
        ("1/5", 412),
        ("5", 409),
        ("2/3", 386),
    ];
    let sample_median = ("1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.2380948358914507e127),
        standard_deviation: NiceFloat(2.2380948357494794e130),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_positive_rationals_helper(
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 64
    let values = &[
        "515152389601430248907720412245595/443042512729",
        "103663/41795133908582491293103906323025",
        "71296091098068019359037078314906341733724/243288956813821163751969634165018193",
        "5557920650918595/82",
        "487/32461763914959248855",
        "38511521798151392412656616617957654586378660839/637134",
        "2330568192653124764618470467652346596061/2516",
        "512663303/39317568409",
        "18536901993439/4959577657266999117207",
        "628/42485719907732979",
        "7403291719610544/1075307073896295169983034533112645563410195",
        "4797445/61",
        "127/13433407097045810",
        "30/1953914271219269",
        "37383453968917/610",
        "11479816781573453/2848901582",
        "2509812009985965380927298501595/13645002946929029896",
        "409735863893015988549887290441890365889795673/6863841",
        "359602127218795816494928857777/9159832300555",
        "142029094679916682/85936648268932530864438001",
    ];
    let common_values = &[
        ("1", 478),
        ("2", 241),
        ("3", 218),
        ("1/2", 218),
        ("1/3", 204),
        ("1/4", 115),
        ("1/7", 106),
        ("2/3", 106),
        ("4", 103),
        ("5", 103),
    ];
    let sample_median = ("1942164762009/1930873766009", Some("3568/3547"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(6.065823121451175e234),
        standard_deviation: NiceFloat(f64::POSITIVE_INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_positive_rationals_helper(
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
fn random_positive_rationals_fail_1() {
    random_positive_rationals(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_positive_rationals_fail_2() {
    random_positive_rationals(EXAMPLE_SEED, 2, 3);
}
