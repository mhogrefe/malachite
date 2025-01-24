// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::prefix_to_string;

#[test]
fn test_prefix_to_string() {
    let test = |xs: &[u32], max_len: usize, out: &str| {
        assert_eq!(prefix_to_string(xs.iter(), max_len), out);
    };
    test(&[], 1, "[]");
    test(&[1, 2, 3, 4], 1, "[1, ...]");
    test(&[1, 2, 3, 4], 2, "[1, 2, ...]");
    test(&[1, 2, 3, 4], 3, "[1, 2, 3, ...]");
    test(&[1, 2, 3, 4], 4, "[1, 2, 3, 4]");
    test(&[1, 2, 3, 4], 10, "[1, 2, 3, 4]");
}

#[test]
#[should_panic]
fn prefix_to_string_fail() {
    prefix_to_string([1, 2, 3].iter(), 0);
}
