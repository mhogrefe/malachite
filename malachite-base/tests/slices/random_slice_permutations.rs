// Copyright © 2024 Mikhail Hogrefe
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
            "ab", "ba", "ba", "ba", "ba", "ba", "ba", "ab", "ab", "ab", "ab", "ab", "ba", "ba",
            "ab", "ba", "ba", "ba", "ab", "ba",
        ],
        &[("ab", 500433), ("ba", 499567)],
        ("ab", None),
    );
    random_slice_permutations_helper(
        "abc",
        &[
            "acb", "bac", "bca", "bac", "bca", "acb", "acb", "cab", "acb", "bca", "cab", "bac",
            "abc", "acb", "cab", "cba", "bac", "bac", "cab", "cba",
        ],
        &[
            ("acb", 167689),
            ("cba", 167200),
            ("cab", 166428),
            ("bca", 166332),
            ("bac", 166279),
            ("abc", 166072),
        ],
        ("bac", None),
    );
    random_slice_permutations_helper(
        "abcd",
        &[
            "cadb", "cbad", "cadb", "badc", "acdb", "cbad", "dabc", "dbca", "cdba", "cdab", "bacd",
            "cabd", "adbc", "cdab", "dcab", "abcd", "abcd", "dacb", "bcad", "adcb",
        ],
        &[
            ("dabc", 41997),
            ("dbac", 41989),
            ("dcab", 41951),
            ("acbd", 41923),
            ("cdba", 41816),
            ("bcad", 41795),
            ("bdca", 41760),
            ("dacb", 41738),
            ("dbca", 41735),
            ("bdac", 41723),
        ],
        ("cabd", None),
    );
    random_slice_permutations_helper(
        "abcdefghij",
        &[
            "acjbhfdgie",
            "cbijehgfda",
            "fjageibdhc",
            "abcjefgdih",
            "gicjbfehda",
            "ahdjfciebg",
            "hfcdgeaibj",
            "gacdhjibfe",
            "cabejhdigf",
            "bicfdjhage",
            "jfhcgdbeai",
            "bifcedghja",
            "ifahdgebjc",
            "aefgicjhbd",
            "ebafcjhdgi",
            "eihjdcabgf",
            "ehiajdcfgb",
            "cgehadjibf",
            "hfcabdgije",
            "fjhiedgbac",
        ],
        &[
            ("hieafgbdjc", 6),
            ("agdcihfjeb", 5),
            ("aigjfdehcb", 5),
            ("aijdhcefgb", 5),
            ("aijgcdbehf", 5),
            ("badjcigfhe", 5),
            ("bfhadgcjie", 5),
            ("bgjfdcheia", 5),
            ("bhfigadjec", 5),
            ("bjigahdfec", 5),
        ],
        ("fabdjchgie", Some("fabdjchieg")),
    );
}
