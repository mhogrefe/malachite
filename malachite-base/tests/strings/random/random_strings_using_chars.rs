// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::chars::random::{random_ascii_chars, random_char_inclusive_range};
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::strings::random::random_strings_using_chars;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::iter::repeat;

fn random_strings_using_chars_helper<I: Clone + Iterator<Item = char>>(
    cs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
) {
    let ss = random_strings_using_chars(
        EXAMPLE_SEED,
        cs_gen,
        mean_length_numerator,
        mean_length_denominator,
    );
    let values = ss.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, ss.clone());
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
fn test_random_strings_using_chars() {
    random_strings_using_chars_helper(
        &|_| repeat('a'),
        4,
        1,
        &[
            "",
            "aaaaaaaaaaaaaa",
            "aaaa",
            "aaaa",
            "a",
            "",
            "aaaaa",
            "aa",
            "aaaa",
            "",
            "aaaaaa",
            "",
            "",
            "aaaaaaaaaaa",
            "aaaaaaaa",
            "",
            "aaa",
            "",
            "aaaaa",
            "aaaaaaaaa",
        ],
        &[
            ("", 199913),
            ("a", 160173),
            ("aa", 128173),
            ("aaa", 102460),
            ("aaaa", 81463),
            ("aaaaa", 65695),
            ("aaaaaa", 52495),
            ("aaaaaaa", 41943),
            ("aaaaaaaa", 33396),
            ("aaaaaaaaa", 27035),
        ],
        ("aaa", None),
    );
    random_strings_using_chars_helper(
        &|seed| random_char_inclusive_range(seed, 'a', 'c'),
        4,
        1,
        &[
            "",
            "bbbbcaaacacaca",
            "cccb",
            "acca",
            "b",
            "",
            "cbcac",
            "cb",
            "cbba",
            "",
            "ccacca",
            "",
            "",
            "acacbcccccc",
            "baccbccb",
            "",
            "bcc",
            "",
            "cbbca",
            "aacbaabbc",
        ],
        &[
            ("", 199913),
            ("c", 53448),
            ("a", 53374),
            ("b", 53351),
            ("ab", 14396),
            ("aa", 14370),
            ("cb", 14345),
            ("cc", 14252),
            ("ba", 14240),
            ("ac", 14237),
        ],
        ("b", None),
    );
    random_strings_using_chars_helper(
        &random_ascii_chars,
        4,
        1,
        &[
            "",
            "U\u{16} D<]ae_M,O\u{1d}V",
            "(\u{10}&U",
            "{P-K",
            "Z",
            "",
            "\u{4}X\u{19}_,",
            "\u{1d},",
            "?\'[N",
            "",
            "|}*\u{15}zt",
            "",
            "",
            "\u{2}FXHmX\\\u{8} ZJ",
            "\u{b}\u{14}OwV\u{19} R",
            "",
            "|4\u{e}",
            "",
            "M$E\u{12}n",
            "J)\u{16}\u{1c}\u{11}_T\u{1d}-",
        ],
        &[
            ("", 199913),
            ("\u{c}", 1334),
            ("~", 1322),
            ("R", 1318),
            ("o", 1312),
            ("=", 1310),
            ("\u{1d}", 1310),
            ("}", 1308),
            ("\u{10}", 1306),
            ("\u{1e}", 1306),
        ],
        ("/x\u{b}", Some("/x\u{f}")),
    );
    random_strings_using_chars_helper(
        &random_ascii_chars,
        1,
        4,
        &[
            "", "", "U", "\u{16}", " D", "", "", "", "", "", "", "", "<]", "a", "", "e_", "", "",
            "", "",
        ],
        &[
            ("", 800023),
            ("<", 1334),
            ("$", 1329),
            ("\u{b}", 1319),
            ("%", 1313),
            ("v", 1309),
            ("A", 1308),
            ("\u{1a}", 1308),
            ("d", 1307),
            ("B", 1306),
        ],
        ("", None),
    );
}

#[test]
#[should_panic]
fn random_strings_using_chars_fail_1() {
    random_strings_using_chars(EXAMPLE_SEED, &random_ascii_chars, 0, 1);
}

#[test]
#[should_panic]
fn random_strings_using_chars_fail_2() {
    random_strings_using_chars(EXAMPLE_SEED, &random_ascii_chars, 1, 0);
}

#[test]
#[should_panic]
fn random_strings_using_chars_fail_3() {
    random_strings_using_chars(EXAMPLE_SEED, &random_ascii_chars, u64::MAX, u64::MAX - 1);
}
