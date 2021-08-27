use itertools::Itertools;
use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::{moment_stats, MomentStats};
use malachite_nz::natural::random::uniform_random_natural_inclusive_range;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::natural::NaturalCheckedToF64Wrapper;
use std::str::FromStr;

fn uniform_random_natural_inclusive_range_helper(
    a: &str,
    b: &str,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    let a = Natural::from_str(a).unwrap();
    let b = Natural::from_str(b).unwrap();
    let xs = uniform_random_natural_inclusive_range(EXAMPLE_SEED, a, b);
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
    let actual_sample_moment_stats = moment_stats(xs.take(1000000).map(NaturalCheckedToF64Wrapper));
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
fn test_uniform_random_natural_inclusive_range() {
    let values = &["0"; 20];
    let common_values = &[("0", 1000000)];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    uniform_random_natural_inclusive_range_helper(
        "0",
        "0",
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "2", "6", "6", "2", "3", "1", "3", "4", "5", "4", "6", "4", "2", "4", "6", "2", "1", "2",
        "6", "6",
    ];
    let common_values =
        &[("3", 167245), ("4", 166932), ("1", 166580), ("6", 166511), ("5", 166451), ("2", 166281)];
    let sample_median = ("3", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3.499925999999989),
        standard_deviation: NiceFloat(1.7070480100269305),
        skewness: NiceFloat(0.00002078867947249881),
        excess_kurtosis: NiceFloat(-1.2668800296473062),
    };
    uniform_random_natural_inclusive_range_helper(
        "1",
        "6",
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "1987", "1993", "1907", "1984", "1927", "1946", "1993", "1922", "1986", "1901", "1907",
        "1929", "1925", "1956", "1997", "1938", "1970", "1906", "1955", "1929",
    ];
    let common_values = &[
        ("1945", 10146),
        ("1987", 10096),
        ("1991", 10094),
        ("1982", 10056),
        ("1900", 10042),
        ("1973", 10033),
        ("1959", 10029),
        ("1967", 10026),
        ("1974", 10024),
        ("1946", 10023),
    ];
    let sample_median = ("1950", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1949.98699899998),
        standard_deviation: NiceFloat(29.18007161489914),
        skewness: NiceFloat(0.000791345316435403),
        excess_kurtosis: NiceFloat(-1.2020606886458867),
    };
    uniform_random_natural_inclusive_range_helper(
        "1900",
        "2000",
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "4233271796909041147200401960861496742517",
        "7217357404646018754599684571795784707698",
        "8727353449345949782973180362335717735342",
        "1534354137356207625017431174589301695702",
        "1534877532602868824396077953846378055833",
        "3066581267912983630335063637372720045094",
        "2831715414082884162869589340142899207735",
        "3619564100767325027279529873701213301661",
        "7005405409180901613675532713129270331479",
        "9271966495851265353624356439908105167895",
        "3537046382263430904899281307939508702471",
        "8202939407624515890221097211474505126578",
        "6142762353061547853401995252125996224683",
        "2027218951536793906738056738325216303009",
        "2459386323443095819796283591928997970915",
        "6477318216232641272279240890043646394779",
        "7387837972601141117504319208136943264497",
        "6474635405681155657679090532822557929038",
        "9135952782573375316643238824480434324207",
        "6103640323458129521087258887390847694928",
    ];
    let common_values = &[
        ("1000008513881061280823789640490226316271", 1),
        ("1000008768725511813114574712047169606198", 1),
        ("1000009827974885359877076313510726004983", 1),
        ("1000012488944552955502737286653696783298", 1),
        ("1000022890668287803601945090476573028348", 1),
        ("1000024602492188456115932292147454123699", 1),
        ("1000032710913204967376519858724740864044", 1),
        ("1000032757195298640822606970649697168394", 1),
        ("1000036222387069235523377031863703777427", 1),
        ("1000036429852801882310669972964558023474", 1),
    ];
    let sample_median = (
        "5500511672867651605813709882516812610647",
        Some("5500520043239248270285741751344805934001"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(5.501866091611912e39),
        standard_deviation: NiceFloat(2.5991591590322043e39),
        skewness: NiceFloat(0.00028444202202606493),
        excess_kurtosis: NiceFloat(-1.2007002735784507),
    };
    uniform_random_natural_inclusive_range_helper(
        "1000000000000000000000000000000000000000",
        "9999999999999999999999999999999999999999",
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn uniform_random_natural_inclusive_range_fail() {
    uniform_random_natural_inclusive_range(EXAMPLE_SEED, Natural::from(10u32), Natural::from(9u32));
}
