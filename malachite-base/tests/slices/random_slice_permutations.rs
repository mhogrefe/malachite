// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::slices::random_slice_permutations;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_base::test_util::stats::median;

fn random_slice_permutations_helper(
    cs: &str,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
) {
    let cs = cs.chars().collect_vec();
    let ss =
        random_slice_permutations(EXAMPLE_SEED, &cs).map(|ds| ds.into_iter().copied().collect());
    let values = ss.clone().take(20).collect_vec();
    let common_values = common_values_map(1000000, 10, ss.clone());
    let (median_lo, median_hi) = median(ss.take(1000000));
    assert_eq!(
        (
            values.iter().map(String::as_str).collect_vec().as_slice(),
            common_values
                .iter()
                .map(|(s, f)| (s.as_str(), *f))
                .collect_vec()
                .as_slice(),
            (median_lo.as_str(), median_hi.as_deref())
        ),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_slice_permutations() {
    random_slice_permutations_helper("", &[""; 20], &[("", 1000000)], ("", None));
    random_slice_permutations_helper("a", &["a"; 20], &[("a", 1000000)], ("a", None));
    random_slice_permutations_helper(
        "ab",
        &[
            "ba", "ab", "ab", "ab", "ab", "ba", "ba", "ba", "ba", "ab", "ba", "ab", "ab", "ba",
            "ab", "ab", "ba", "ab", "ba", "ab",
        ],
        &[("ba", 500291), ("ab", 499709)],
        ("ba", None),
    );
    random_slice_permutations_helper(
        "abc",
        &[
            "bac", "cba", "cba", "cba", "cba", "acb", "acb", "bca", "acb", "bac", "abc", "bac",
            "bac", "acb", "cba", "abc", "bca", "cab", "acb", "bac",
        ],
        &[
            ("abc", 167957),
            ("bac", 167073),
            ("cba", 166362),
            ("acb", 166331),
            ("cab", 166166),
            ("bca", 166111),
        ],
        ("bac", None),
    );
    random_slice_permutations_helper(
        "abcd",
        &[
            "dacb", "cbad", "cdab", "cbad", "cdab", "bcda", "bcda", "acbd", "bcda", "dbca", "bdac",
            "dbac", "dbca", "bcad", "cadb", "dacb", "acbd", "dbac", "bdca", "abdc",
        ],
        &[
            ("dbca", 41992),
            ("bcda", 41855),
            ("cdab", 41827),
            ("dcab", 41818),
            ("abcd", 41809),
            ("badc", 41804),
            ("cadb", 41803),
            ("adbc", 41763),
            ("cbad", 41706),
            ("dbac", 41697),
        ],
        ("bdca", None),
    );
    random_slice_permutations_helper(
        "abcdefghij",
        &[
            "daiehfcbjg",
            "bhagejicdf",
            "fdaibchgje",
            "bicdfhgaej",
            "bdfihjagec",
            "chdjbafeig",
            "cgajbfdieh",
            "jecbhafgid",
            "aedjgchfbi",
            "hacjiefdgb",
            "ahgdfcbije",
            "cibdehfjag",
            "fgdebchjai",
            "gieajbdfch",
            "deagfjihcb",
            "hjfgdbacie",
            "jdghacbfie",
            "cebaihjfdg",
            "jgdbchfeai",
            "cjahbegdif",
        ],
        &[
            ("afdhbciejg", 5),
            ("afegijbcdh", 5),
            ("afgcedhjbi", 5),
            ("agbijhcfde", 5),
            ("ahejcigbfd", 5),
            ("ahjcgfdebi", 5),
            ("aihecgdfbj", 5),
            ("ajfghdcbei", 5),
            ("badicjhgfe", 5),
            ("bfdacehijg", 5),
        ],
        ("ejicadfgbh", Some("ejicadgbfh")),
    );
}
