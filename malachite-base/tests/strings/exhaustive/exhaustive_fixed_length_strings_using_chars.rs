// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::exhaustive::exhaustive_fixed_length_strings_using_chars;
use std::iter::{empty, once};

fn exhaustive_fixed_length_strings_using_chars_helper<I: Iterator<Item = char>>(
    len: u64,
    cs: I,
    out: &[&str],
) {
    let css = exhaustive_fixed_length_strings_using_chars(len, cs)
        .take(20)
        .collect_vec();
    assert_eq!(css.iter().map(String::as_str).collect_vec().as_slice(), out);
}

#[test]
fn test_exhaustive_fixed_length_strings_using_chars() {
    exhaustive_fixed_length_strings_using_chars_helper(0, empty(), &[""]);
    exhaustive_fixed_length_strings_using_chars_helper(1, empty(), &[]);
    exhaustive_fixed_length_strings_using_chars_helper(0, once('q'), &[""]);
    exhaustive_fixed_length_strings_using_chars_helper(1, once('q'), &["q"]);
    exhaustive_fixed_length_strings_using_chars_helper(2, once('q'), &["qq"]);
    exhaustive_fixed_length_strings_using_chars_helper(3, once('q'), &["qqq"]);
    exhaustive_fixed_length_strings_using_chars_helper(10, once('q'), &["qqqqqqqqqq"]);
    exhaustive_fixed_length_strings_using_chars_helper(0, ['x', 'y'].iter().copied(), &[""]);
    exhaustive_fixed_length_strings_using_chars_helper(1, ['x', 'y'].iter().copied(), &["x", "y"]);
    exhaustive_fixed_length_strings_using_chars_helper(
        2,
        ['x', 'y'].iter().copied(),
        &["xx", "xy", "yx", "yy"],
    );
    exhaustive_fixed_length_strings_using_chars_helper(
        3,
        ['x', 'y'].iter().copied(),
        &["xxx", "xxy", "xyx", "xyy", "yxx", "yxy", "yyx", "yyy"],
    );
    exhaustive_fixed_length_strings_using_chars_helper(
        10,
        ['x', 'y'].iter().copied(),
        &[
            "xxxxxxxxxx",
            "xxxxxxxxxy",
            "xxxxxxxxyx",
            "xxxxxxxxyy",
            "xxxxxxxyxx",
            "xxxxxxxyxy",
            "xxxxxxxyyx",
            "xxxxxxxyyy",
            "xxxxxxyxxx",
            "xxxxxxyxxy",
            "xxxxxxyxyx",
            "xxxxxxyxyy",
            "xxxxxxyyxx",
            "xxxxxxyyxy",
            "xxxxxxyyyx",
            "xxxxxxyyyy",
            "xxxxxyxxxx",
            "xxxxxyxxxy",
            "xxxxxyxxyx",
            "xxxxxyxxyy",
        ],
    );
    exhaustive_fixed_length_strings_using_chars_helper(0, ['c', 'a', 't'].iter().copied(), &[""]);
    exhaustive_fixed_length_strings_using_chars_helper(
        1,
        ['c', 'a', 't'].iter().copied(),
        &["c", "a", "t"],
    );
    exhaustive_fixed_length_strings_using_chars_helper(
        2,
        ['c', 'a', 't'].iter().copied(),
        &["cc", "ca", "ac", "aa", "ct", "at", "tc", "ta", "tt"],
    );
    exhaustive_fixed_length_strings_using_chars_helper(
        3,
        ['c', 'a', 't'].iter().copied(),
        &[
            "ccc", "cca", "cac", "caa", "acc", "aca", "aac", "aaa", "cct", "cat", "act", "aat",
            "ctc", "cta", "atc", "ata", "ctt", "att", "tcc", "tca",
        ],
    );
    exhaustive_fixed_length_strings_using_chars_helper(
        10,
        ['c', 'a', 't'].iter().copied(),
        &[
            "cccccccccc",
            "ccccccccca",
            "ccccccccac",
            "ccccccccaa",
            "cccccccacc",
            "cccccccaca",
            "cccccccaac",
            "cccccccaaa",
            "ccccccaccc",
            "ccccccacca",
            "ccccccacac",
            "ccccccacaa",
            "ccccccaacc",
            "ccccccaaca",
            "ccccccaaac",
            "ccccccaaaa",
            "cccccacccc",
            "cccccaccca",
            "cccccaccac",
            "cccccaccaa",
        ],
    );
}
