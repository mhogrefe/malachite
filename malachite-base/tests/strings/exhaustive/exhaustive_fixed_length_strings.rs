// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::exhaustive::exhaustive_fixed_length_strings;

fn exhaustive_fixed_length_strings_helper(len: u64, out: &[&str]) {
    let css = exhaustive_fixed_length_strings(len).take(20).collect_vec();
    assert_eq!(css.iter().map(String::as_str).collect_vec().as_slice(), out);
}

#[test]
fn test_exhaustive_fixed_length_strings() {
    exhaustive_fixed_length_strings_helper(0, &[""]);
    exhaustive_fixed_length_strings_helper(
        1,
        &[
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t",
        ],
    );
    exhaustive_fixed_length_strings_helper(
        2,
        &[
            "aa", "ab", "ba", "bb", "ac", "ad", "bc", "bd", "ca", "cb", "da", "db", "cc", "cd",
            "dc", "dd", "ae", "af", "be", "bf",
        ],
    );
    exhaustive_fixed_length_strings_helper(
        3,
        &[
            "aaa", "aab", "aba", "abb", "baa", "bab", "bba", "bbb", "aac", "aad", "abc", "abd",
            "bac", "bad", "bbc", "bbd", "aca", "acb", "ada", "adb",
        ],
    );
    exhaustive_fixed_length_strings_helper(
        10,
        &[
            "aaaaaaaaaa",
            "aaaaaaaaab",
            "aaaaaaaaba",
            "aaaaaaaabb",
            "aaaaaaabaa",
            "aaaaaaabab",
            "aaaaaaabba",
            "aaaaaaabbb",
            "aaaaaabaaa",
            "aaaaaabaab",
            "aaaaaababa",
            "aaaaaababb",
            "aaaaaabbaa",
            "aaaaaabbab",
            "aaaaaabbba",
            "aaaaaabbbb",
            "aaaaabaaaa",
            "aaaaabaaab",
            "aaaaabaaba",
            "aaaaabaabb",
        ],
    );
}
