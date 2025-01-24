// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_nz::natural::random::random_naturals_less_than;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::natural::random::random_naturals_helper_helper;
use std::str::FromStr;

fn random_naturals_less_than_helper(
    limit: &str,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    random_naturals_helper_helper(
        random_naturals_less_than(EXAMPLE_SEED, Natural::from_str(limit).unwrap()),
        expected_values,
        expected_common_values,
        expected_sample_median,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_random_naturals_less_than() {
    let values = &["0"; 20];
    let common_values = &[("0", 1000000)];
    let sample_median = ("0", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(0.0),
        skewness: NiceFloat(f64::NAN),
        excess_kurtosis: NiceFloat(f64::NAN),
    };
    random_naturals_less_than_helper(
        "1",
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "1", "7", "13", "5", "7", "9", "2", "8", "2", "11", "4", "11", "14", "13", "6", "6", "11",
        "1", "3", "7",
    ];
    let common_values = &[
        ("10", 67077),
        ("7", 67071),
        ("2", 66902),
        ("6", 66802),
        ("3", 66769),
        ("9", 66740),
        ("11", 66724),
        ("14", 66705),
        ("0", 66612),
        ("13", 66590),
    ];
    let sample_median = ("7", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.002550000000202),
        standard_deviation: NiceFloat(4.318291113986895),
        skewness: NiceFloat(-0.0008733138647064903),
        excess_kurtosis: NiceFloat(-1.209254148711494),
    };
    random_naturals_less_than_helper(
        "15",
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "1", "7", "13", "5", "7", "9", "2", "8", "2", "11", "4", "11", "14", "13", "6", "6", "11",
        "1", "3", "7",
    ];
    let common_values = &[
        ("10", 62902),
        ("2", 62837),
        ("7", 62827),
        ("3", 62676),
        ("11", 62617),
        ("6", 62613),
        ("14", 62607),
        ("9", 62606),
        ("8", 62510),
        ("0", 62466),
    ];
    let sample_median = ("8", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.499876999999703),
        standard_deviation: NiceFloat(4.606495327759524),
        skewness: NiceFloat(-0.0003936178035348645),
        excess_kurtosis: NiceFloat(-1.207854285765452),
    };
    random_naturals_less_than_helper(
        "16",
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "7", "9", "8", "11", "14", "11", "1", "7", "1", "6", "6", "6", "7", "5", "12", "7", "15",
        "7", "7", "5",
    ];
    let common_values = &[
        ("13", 59283),
        ("10", 59171),
        ("3", 59142),
        ("6", 59131),
        ("16", 59052),
        ("0", 58906),
        ("9", 58849),
        ("14", 58816),
        ("2", 58806),
        ("7", 58744),
    ];
    let sample_median = ("8", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(8.003521000000225),
        standard_deviation: NiceFloat(4.899161622598552),
        skewness: NiceFloat(-0.0010892026016323982),
        excess_kurtosis: NiceFloat(-1.20819299609532),
    };
    random_naturals_less_than_helper(
        "17",
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "388977", "925783", "378973", "75271", "945129", "765554", "271720", "260338", "21995",
        "458836", "661659", "310190", "396637", "576534", "304342", "557803", "678529", "654451",
        "280711", "928029",
    ];
    let common_values = &[
        ("429869", 9),
        ("568287", 9),
        ("771880", 9),
        ("890", 8),
        ("18201", 8),
        ("61885", 8),
        ("163140", 8),
        ("173104", 8),
        ("214281", 8),
        ("340935", 8),
    ];
    let sample_median = ("500024", Some("500026"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(500269.9304960077),
        standard_deviation: NiceFloat(288405.96360756975),
        skewness: NiceFloat(-0.0010400381943526848),
        excess_kurtosis: NiceFloat(-1.1981474275491453),
    };
    random_naturals_less_than_helper(
        "1000000",
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
    let values = &[
        "3233271796909041147200401960861496742517",
        "6217357404646018754599684571795784707698",
        "9438016902699487422458396847802670929387",
        "7727353449345949782973180362335717735342",
        "534354137356207625017431174589301695702",
        "534877532602868824396077953846378055833",
        "2066581267912983630335063637372720045094",
        "1831715414082884162869589340142899207735",
        "2619564100767325027279529873701213301661",
        "6005405409180901613675532713129270331479",
        "8271966495851265353624356439908105167895",
        "2537046382263430904899281307939508702471",
        "7202939407624515890221097211474505126578",
        "5142762353061547853401995252125996224683",
        "1027218951536793906738056738325216303009",
        "9914727392521646960300300265785426013882",
        "1459386323443095819796283591928997970915",
        "5477318216232641272279240890043646394779",
        "6387837972601141117504319208136943264497",
        "5474635405681155657679090532822557929038",
    ];
    let common_values = &[
        ("8768725511813114574712047169606198", 1),
        ("9827974885359877076313510726004983", 1),
        ("12488944552955502737286653696783298", 1),
        ("22890668287803601945090476573028348", 1),
        ("24602492188456115932292147454123699", 1),
        ("32710913204967376519858724740864044", 1),
        ("36222387069235523377031863703777427", 1),
        ("49953398642815549142118724082696119", 1),
        ("51712706399518805574773981541840520", 1),
        ("58535354783119341230390576092841062", 1),
    ];
    let sample_median = (
        "5001510563009032264934634274159139768440",
        Some("5001521540293100102818635713617625902813"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(5.001250475448614e39),
        standard_deviation: NiceFloat(2.887524153222133e39),
        skewness: NiceFloat(-0.0006935347805930534),
        excess_kurtosis: NiceFloat(-1.2008620906428813),
    };
    random_naturals_less_than_helper(
        "10000000000000000000000000000000000000000",
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn random_naturals_less_than_fail() {
    random_naturals_less_than(EXAMPLE_SEED, Natural::ZERO);
}
