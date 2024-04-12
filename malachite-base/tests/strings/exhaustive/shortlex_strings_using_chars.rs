// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::strings::exhaustive::shortlex_strings_using_chars;
use std::iter::{empty, once};

fn shortlex_strings_using_chars_helper<I: Clone + Iterator<Item = char>>(cs: I, out: &[&str]) {
    let ss = shortlex_strings_using_chars(cs).take(20).collect_vec();
    assert_eq!(ss.iter().map(String::as_str).collect_vec().as_slice(), out);
}

#[test]
fn test_shortlex_strings_using_chars() {
    shortlex_strings_using_chars_helper(empty(), &[""]);
    shortlex_strings_using_chars_helper(
        once('a'),
        &[
            "",
            "a",
            "aa",
            "aaa",
            "aaaa",
            "aaaaa",
            "aaaaaa",
            "aaaaaaa",
            "aaaaaaaa",
            "aaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaaa",
            "aaaaaaaaaaaa",
            "aaaaaaaaaaaaa",
            "aaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaa",
        ],
    );
    shortlex_strings_using_chars_helper(
        "ab".chars(),
        &[
            "", "a", "b", "aa", "ab", "ba", "bb", "aaa", "aab", "aba", "abb", "baa", "bab", "bba",
            "bbb", "aaaa", "aaab", "aaba", "aabb", "abaa",
        ],
    );
    shortlex_strings_using_chars_helper(
        "xyz".chars(),
        &[
            "", "x", "y", "z", "xx", "xy", "xz", "yx", "yy", "yz", "zx", "zy", "zz", "xxx", "xxy",
            "xxz", "xyx", "xyy", "xyz", "xzx",
        ],
    );
    shortlex_strings_using_chars_helper(
        exhaustive_ascii_chars(),
        &[
            "", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
            "q", "r", "s",
        ],
    );
}
