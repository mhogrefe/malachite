use itertools::Itertools;
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::{moment_stats, MomentStats};
use malachite_nz::integer::random::random_integers;
use malachite_nz::integer::Integer;
use malachite_nz_test_util::integer::IntegerCheckedToF64Wrapper;

fn random_integers_helper(
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    let xs = random_integers(EXAMPLE_SEED, mean_bits_numerator, mean_bits_denominator);
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
fn test_random_integers() {
    // mean bits = 1/64
    let values = &["0"; 20];
    let common_values = &[
        ("0", 969830),
        ("1", 14858),
        ("-1", 14856),
        ("-3", 128),
        ("-2", 122),
        ("3", 101),
        ("2", 98),
        ("7", 2),
        ("-4", 2),
        ("5", 1),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.00012800000000000482),
        standard_deviation: NiceFloat(0.18133950617561467),
        skewness: NiceFloat(-0.11594058747329855),
        excess_kurtosis: NiceFloat(53.17726403139359),
    };
    random_integers_helper(
        1,
        64,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 1
    let values = &[
        "2", "-2", "-6", "0", "0", "-1", "-2", "-2", "2", "0", "0", "-1", "-7", "-2", "5", "0",
        "1", "1", "1", "-2",
    ];
    let common_values = &[
        ("0", 332922),
        ("-1", 166652),
        ("1", 166524),
        ("3", 42164),
        ("2", 41585),
        ("-3", 41436),
        ("-2", 41400),
        ("5", 10546),
        ("4", 10540),
        ("-6", 10475),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.4130599999999974),
        standard_deviation: NiceFloat(777.5605240878597),
        skewness: NiceFloat(-244.83259806631784),
        excess_kurtosis: NiceFloat(225482.22529172004),
    };
    random_integers_helper(
        1,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 32
    let values = &[
        "89270",
        "69403499476962893258904",
        "62",
        "-1848070042786",
        "-64671510460",
        "-696",
        "0",
        "-79",
        "70819",
        "7330",
        "215441",
        "-424643",
        "-11858",
        "-84146163512",
        "-7212822200",
        "1518",
        "23",
        "-909",
        "-60054",
        "-46",
    ];
    let common_values = &[
        ("0", 15405),
        ("1", 15074),
        ("-1", 14891),
        ("-2", 7292),
        ("2", 7217),
        ("-3", 7168),
        ("3", 7120),
        ("5", 3593),
        ("-6", 3558),
        ("-7", 3542),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.0417062616580636e155),
        standard_deviation: NiceFloat(f64::POSITIVE_INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_integers_helper(
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    // mean bits = 64
    let values = &[
        "15542",
        "204354108892664954266560767940941860034994328",
        "5282",
        "-323516",
        "-400812728",
        "-248570628312176883893327",
        "5606382754",
        "-63523217",
        "-15024295498724618356672330435",
        "25408382788335305673841323624499957642146385720",
        "70153184455655",
        "331577334953510974497668975717692627852954604565929960256566094891153641703901536975584071\
        2936487655650300919339856269",
        "-2179070834703641056854463566957970466590674233219693760530182904389383",
        "-5826316",
        "-8647284",
        "-1",
        "43088412843029635753589496830104451113312",
        "18608",
        "-3946823889925",
        "-114916707179919722397",
    ];
    let common_values = &[
        ("0", 7696),
        ("-1", 7685),
        ("1", 7575),
        ("-3", 3800),
        ("-2", 3773),
        ("3", 3717),
        ("2", 3679),
        ("7", 1889),
        ("6", 1862),
        ("-5", 1862),
    ];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.4757098576025357e248),
        standard_deviation: NiceFloat(f64::POSITIVE_INFINITY),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_integers_helper(
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
fn random_integers_fail_1() {
    random_integers(EXAMPLE_SEED, 0, 1);
}

#[test]
#[should_panic]
fn random_integers_fail_2() {
    random_integers(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn random_integers_fail_3() {
    random_integers(EXAMPLE_SEED, u64::MAX, u64::MAX - 1);
}
