// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::chars::random::{random_ascii_chars, random_char_inclusive_range};
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::random::random_fixed_length_strings_using_chars;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_base::test_util::stats::median;
use std::iter::repeat;

fn random_fixed_length_strings_using_chars_helper<I: Clone + Iterator<Item = char>>(
    len: u64,
    cs: I,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
) {
    let ss = random_fixed_length_strings_using_chars(len, cs);
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
fn test_random_fixed_length_strings_using_chars() {
    random_fixed_length_strings_using_chars_helper(
        0,
        repeat('a'),
        &[""; 20],
        &[("", 1000000)],
        ("", None),
    );
    random_fixed_length_strings_using_chars_helper(
        1,
        repeat('a'),
        &["a"; 20],
        &[("a", 1000000)],
        ("a", None),
    );
    random_fixed_length_strings_using_chars_helper(
        2,
        repeat('a'),
        &["aa"; 20],
        &[("aa", 1000000)],
        ("aa", None),
    );
    random_fixed_length_strings_using_chars_helper(
        3,
        repeat('a'),
        &["aaa"; 20],
        &[("aaa", 1000000)],
        ("aaa", None),
    );
    random_fixed_length_strings_using_chars_helper(
        10,
        repeat('a'),
        &["aaaaaaaaaa"; 20],
        &[("aaaaaaaaaa", 1000000)],
        ("aaaaaaaaaa", None),
    );
    random_fixed_length_strings_using_chars_helper(
        1,
        random_char_inclusive_range(EXAMPLE_SEED, 'a', 'c'),
        &[
            "b", "a", "b", "c", "b", "b", "a", "b", "a", "c", "b", "a", "b", "c", "c", "a", "b",
            "a", "c", "c",
        ],
        &[("b", 333784), ("c", 333516), ("a", 332700)],
        ("b", None),
    );
    random_fixed_length_strings_using_chars_helper(
        2,
        random_char_inclusive_range(EXAMPLE_SEED, 'a', 'c'),
        &[
            "ba", "bc", "bb", "ab", "ac", "ba", "bc", "ca", "ba", "cc", "cb", "ac", "cb", "bb",
            "aa", "ca", "cc", "ba", "bb", "ac",
        ],
        &[
            ("cb", 111755),
            ("aa", 111436),
            ("cc", 111255),
            ("bb", 111161),
            ("ab", 111154),
            ("ba", 111089),
            ("ca", 110836),
            ("bc", 110731),
            ("ac", 110583),
        ],
        ("bb", None),
    );
    random_fixed_length_strings_using_chars_helper(
        3,
        random_char_inclusive_range(EXAMPLE_SEED, 'a', 'c'),
        &[
            "bab", "cbb", "aba", "cba", "bcc", "aba", "ccc", "bac", "cbb", "baa", "cac", "cba",
            "bba", "cca", "cab", "cbb", "ccb", "bba", "cab", "bbc",
        ],
        &[
            ("bab", 37526),
            ("ccb", 37346),
            ("acb", 37337),
            ("bca", 37271),
            ("cbb", 37251),
            ("bba", 37211),
            ("aab", 37170),
            ("caa", 37142),
            ("bbb", 37096),
            ("abc", 37095),
        ],
        ("bbb", None),
    );
    random_fixed_length_strings_using_chars_helper(
        10,
        random_char_inclusive_range(EXAMPLE_SEED, 'a', 'c'),
        &[
            "babcbbabac",
            "babccabacc",
            "cbaccbbbaa",
            "caccbabbac",
            "cacabcbbcc",
            "bbbacabbbc",
            "abbcbbbcbc",
            "bbcabbbcab",
            "cbaacabbbc",
            "bccccccbac",
            "abaacbcaba",
            "bbaabcacab",
            "bbabaaacbc",
            "cccbccabba",
            "aacabaabcc",
            "acabccccab",
            "bcacccaacc",
            "accaccbbbc",
            "aabcbaabcc",
            "cbccbbbabc",
        ],
        &[
            ("caacabbbba", 38),
            ("cbaaacbaab", 37),
            ("bcacbbabca", 36),
            ("bcbacaabba", 36),
            ("aabaaccbaa", 35),
            ("abcbccaaab", 35),
            ("bcbcbbbbba", 35),
            ("aabccaaaca", 34),
            ("abacaaabba", 34),
            ("baccbabbcb", 34),
        ],
        ("bbbbbbbacc", None),
    );
    random_fixed_length_strings_using_chars_helper(
        1,
        random_ascii_chars(EXAMPLE_SEED),
        &[
            "q", "^", "\u{17}", "b", "F", "\\", "4", "T", "!", "/", "\u{1}", "q", "6", "\n", "/",
            "\u{11}", "Y", "\\", "w", "B",
        ],
        &[
            ("\u{2}", 8077),
            ("y", 8039),
            ("0", 8015),
            ("q", 7966),
            ("\u{8}", 7937),
            ("M", 7933),
            ("2", 7928),
            ("[", 7927),
            ("R", 7925),
            ("f", 7924),
        ],
        ("?", None),
    );
    random_fixed_length_strings_using_chars_helper(
        2,
        random_ascii_chars(EXAMPLE_SEED),
        &[
            "q^", "\u{17}b", "F\\", "4T", "!/", "\u{1}q", "6\n", "/\u{11}", "Y\\", "wB", "\\r",
            "\\^", "\u{15}3", "\'.", "\'r", "\u{7}$", "\u{17}S", ":\r", "r@", "I(",
        ],
        &[
            ("c5", 91),
            ("\u{1c} ", 90),
            ("GN", 90),
            ("a2", 90),
            ("\u{13}%", 89),
            ("o\u{14}", 89),
            ("(u", 88),
            ("X2", 88),
            ("\u{10}e", 87),
            ("\u{1e}f", 87),
        ],
        ("@\u{2}", None),
    );
    random_fixed_length_strings_using_chars_helper(
        3,
        random_ascii_chars(EXAMPLE_SEED),
        &[
            "q^\u{17}",
            "bF\\",
            "4T!",
            "/\u{1}q",
            "6\n/",
            "\u{11}Y\\",
            "wB\\",
            "r\\^",
            "\u{15}3\'",
            ".\'r",
            "\u{7}$\u{17}",
            "S:\r",
            "r@I",
            "(\u{10}\u{11}",
            "}\u{b}\u{7}",
            "0z5",
            ".n1",
            "\u{10}At",
            "<9.",
            "w\\?",
        ],
        &[
            ("$7\u{5}", 7),
            ("*\u{1c}\u{1e}", 7),
            ("\u{e}sb", 6),
            ("\u{10}+g", 6),
            ("\u{13}`\u{14}", 6),
            ("\u{13}oF", 6),
            ("\u{15}[[", 6),
            ("\u{1c}Ve", 6),
            ("\u{1e}<7", 6),
            ("\"*K", 6),
        ],
        ("?}^", None),
    );
    random_fixed_length_strings_using_chars_helper(
        10,
        random_ascii_chars(EXAMPLE_SEED),
        &[
            "q^\u{17}bF\\4T!/",
            "\u{1}q6\n/\u{11}Y\\wB",
            "\\r\\^\u{15}3\'.\'r",
            "\u{7}$\u{17}S:\rr@I(",
            "\u{10}\u{11}}\u{b}\u{7}0z5.n",
            "1\u{10}At<9.w\\?",
            "b\u{15}(\\hJ\u{10}cO\\",
            "^5Edc\u{1f}kq{t",
            "=z./\u{5}x\u{1}dZr",
            "J%\u{5}`=VU_\u{7f}b",
            ";\u{13}\u{6}U.k\r\u{6}PB",
            "k]$p\u{1a}+FOH.",
            "\r,a\u{1}=DZZ\u{16}\u{18}",
            "cY\t\u{1e}\u{19}&<,\u{13}%",
            "\u{c}{Z!$Z,\u{17}\u{8}?",
            "\u{3}\u{4}]\u{1}H\u{c}(K*|",
            "l\u{15}8^:\u{e}\u{7f}D(P",
            "\u{1}XEk!$\u{14}/];",
            "E9d\u{e})|v\u{e}W*",
            ").\u{19}\u{11}5\u{7f}b8\u{18}:",
        ],
        &[
            ("\u{0}\u{0}\u{1})\u{19}\\\u{10};bj", 1),
            ("\u{0}\u{0}\u{3}\u{7};sV\u{e}2}", 1),
            ("\u{0}\u{0}\u{5}\tmrh\u{1f}{E", 1),
            ("\u{0}\u{0}\n\n\u{2}\"\u{13}ftF", 1),
            ("\u{0}\u{0}\r+I^a\u{6}>R", 1),
            ("\u{0}\u{0}\ry#hUV8+", 1),
            ("\u{0}\u{0}\u{e}\"^\u{6},+\u{10}[", 1),
            ("\u{0}\u{0}\u{11}J\u{0}\u{11}5kiy", 1),
            ("\u{0}\u{0}\u{12}\u{4}txo}{7", 1),
            ("\u{0}\u{0}\u{13}Gb;}ex|", 1),
        ],
        (
            "@\u{b}\u{c}t\u{11}\u{b}\u{e}NV&",
            Some("@\u{b}\u{e}FN+o}X\u{4}"),
        ),
    );
}
