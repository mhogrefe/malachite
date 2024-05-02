// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::is_unique;
use malachite_base::strings::{string_is_subset, string_unique};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::string_gen;

#[test]
fn test_string_unique() {
    let test = |s, out| {
        assert_eq!(string_unique(s), out);
    };
    test("", "");
    test("x", "x");
    test("xxxxxxxxx", "x");
    test("Hello, world!", "Helo, wrd!");
    test("Mississippi", "Misp");
    test(
        "A quick brown fox jumps over the lazy dog",
        "A quickbrownfxjmpsvethlazydg",
    );
}

#[test]
fn string_unique_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 128);
    config.insert("mean_length_d", 1);
    string_gen().test_properties_with_config(&config, |s| {
        let t = string_unique(&s);
        assert!(is_unique(t.chars()));
        assert!(t.len() <= s.len());
        assert_eq!(string_unique(&t), t);
        assert!(string_is_subset(&s, &t));
        assert!(string_is_subset(&t, &s));
    });
}
