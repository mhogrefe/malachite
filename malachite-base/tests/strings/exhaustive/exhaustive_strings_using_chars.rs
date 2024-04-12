// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::strings::exhaustive::exhaustive_strings_using_chars;
use std::iter::{empty, once};

fn exhaustive_strings_using_chars_helper<I: Clone + Iterator<Item = char>>(cs: I, out: &[&str]) {
    let ss = exhaustive_strings_using_chars(cs).take(20).collect_vec();
    assert_eq!(ss.iter().map(String::as_str).collect_vec().as_slice(), out);
}

#[test]
fn test_exhaustive_strings_using_chars() {
    exhaustive_strings_using_chars_helper(empty(), &[""]);
    exhaustive_strings_using_chars_helper(
        once('a'),
        &[
            "",
            "a",
            "aa",
            "aaaa",
            "aaa",
            "aaaaa",
            "aaaaaa",
            "aaaaaaaaa",
            "aaaaaaa",
            "aaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaaaa",
            "aaaaaaaaaaa",
            "aaaaaaaaaaaaa",
            "aaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaa",
        ],
    );
    exhaustive_strings_using_chars_helper(
        "ab".chars(),
        &[
            "", "a", "b", "aaa", "aa", "aab", "ab", "aaaaa", "ba", "aba", "bb", "aaaa", "abb",
            "aaab", "baa", "aaaaaaa", "bab", "aaba", "bba", "aaaab",
        ],
    );
    exhaustive_strings_using_chars_helper(
        "xyz".chars(),
        &[
            "", "x", "y", "xxx", "z", "xx", "xy", "xxxxx", "yx", "xxy", "yy", "xxxx", "xz", "xyx",
            "yz", "xxxxxx", "zx", "xyy", "zy", "xxxy",
        ],
    );
    exhaustive_strings_using_chars_helper(
        exhaustive_ascii_chars(),
        &[
            "", "a", "b", "aaa", "c", "aa", "d", "aaaa", "e", "ab", "f", "aab", "g", "ba", "h",
            "aaaaa", "i", "bb", "j", "aba",
        ],
    );
}
