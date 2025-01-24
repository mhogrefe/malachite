// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::strings_from_char_vecs;
use std::iter::repeat;

fn strings_from_char_vecs_helper<I: Iterator<Item = Vec<char>>>(css: I, out: &[&str]) {
    let css = strings_from_char_vecs(css).take(20).collect_vec();
    assert_eq!(css.iter().map(String::as_str).collect_vec().as_slice(), out);
}

#[test]
fn test_strings_from_char_vecs() {
    strings_from_char_vecs_helper([].iter().cloned(), &[]);
    strings_from_char_vecs_helper([vec!['a']].iter().cloned(), &["a"]);
    strings_from_char_vecs_helper(
        [vec!['a', 'b'], vec!['c', 'd']].iter().cloned(),
        &["ab", "cd"],
    );
    strings_from_char_vecs_helper(
        repeat(vec!['c', 'a', 't']),
        &[
            "cat", "cat", "cat", "cat", "cat", "cat", "cat", "cat", "cat", "cat", "cat", "cat",
            "cat", "cat", "cat", "cat", "cat", "cat", "cat", "cat",
        ],
    );
}
