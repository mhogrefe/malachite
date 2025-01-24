// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::strings::{string_is_subset, string_sort, string_unique};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::string_pair_gen;

#[test]
fn test_string_is_subset() {
    let test = |s, t, out| {
        assert_eq!(string_is_subset(s, t), out);
    };
    test("", "Hello, world!", true);
    test("o, well", "Hello, world!", true);
    test("MMM", "Mississippi", true);
    test("Hello, World!", "Hello, world!", false);
    test("j", "Mississippi", false);
    test(
        "abcdefghijklmnopqrstuvwxyz",
        "A quick brown fox jumps over the lazy dog",
        true,
    );
}

#[test]
fn string_is_subset_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 128);
    config.insert("mean_length_d", 1);
    string_pair_gen().test_properties_with_config(&config, |(s, t)| {
        let is_subset = string_is_subset(&s, &t);
        assert_eq!(
            string_is_subset(&string_sort(&s), &string_sort(&t)),
            is_subset
        );
        assert_eq!(
            string_is_subset(&string_unique(&s), &string_unique(&t)),
            is_subset
        );
        assert_eq!(
            is_subset && string_is_subset(&t, &s),
            string_sort(&string_unique(&s)) == string_sort(&string_unique(&t))
        );
    });
}
