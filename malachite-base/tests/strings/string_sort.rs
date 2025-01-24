// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::comparison::is_weakly_ascending;
use malachite_base::strings::{string_is_subset, string_sort, string_unique};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::string_gen;

#[test]
fn test_string_sort() {
    let test = |s, out| {
        assert_eq!(string_sort(s), out);
    };
    test("", "");
    test("x", "x");
    test("Hello, world!", " !,Hdellloorw");
    test("Mississippi", "Miiiippssss");
    test(
        "A quick brown fox jumps over the lazy dog",
        "        Aabcdeefghijklmnoooopqrrstuuvwxyz",
    );
}

#[test]
fn string_sort_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 128);
    config.insert("mean_length_d", 1);
    string_gen().test_properties_with_config(&config, |s| {
        let t = string_sort(&s);
        assert!(is_weakly_ascending(t.chars()));
        assert_eq!(s.len(), t.len());
        assert_eq!(string_sort(&t), t);
        assert_eq!(string_unique(&t), string_sort(&string_unique(&s)));
        assert!(string_is_subset(&s, &t));
        assert!(string_is_subset(&t, &s));
    });
}
