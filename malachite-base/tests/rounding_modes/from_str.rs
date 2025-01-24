// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{string_gen, string_gen_var_2};
use std::str::FromStr;

#[test]
fn test_from_str() {
    let test = |s, out| {
        assert_eq!(RoundingMode::from_str(s), out);
    };
    test("Down", Ok(Down));
    test("Up", Ok(Up));
    test("Floor", Ok(Floor));
    test("Ceiling", Ok(Ceiling));
    test("Nearest", Ok(Nearest));
    test("Exact", Ok(Exact));

    test("", Err("".to_string()));
    test("abc", Err("abc".to_string()));
    test("Uptown", Err("Uptown".to_string()));
}

#[allow(clippy::needless_pass_by_value)]
fn from_str_helper(s: String) {
    let result = RoundingMode::from_str(&s);
    if let Ok(result) = result {
        assert_eq!(result.to_string(), s);
    }
}

#[test]
fn from_str_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 128);
    config.insert("mean_length_d", 1);
    string_gen().test_properties_with_config(&config, from_str_helper);
    string_gen_var_2().test_properties_with_config(&config, from_str_helper);
}
