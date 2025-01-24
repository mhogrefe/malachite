// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::strings::string_is_subset;
use malachite_base::test_util::generators::rounding_mode_gen;
use malachite_base::test_util::rounding_modes::ROUNDING_MODE_CHARS;
use std::str::FromStr;

#[test]
fn test_to_string() {
    let test = |rm: RoundingMode, out| {
        assert_eq!(rm.to_string(), out);
    };
    test(Down, "Down");
    test(Up, "Up");
    test(Floor, "Floor");
    test(Ceiling, "Ceiling");
    test(Nearest, "Nearest");
    test(Exact, "Exact");
}

#[test]
fn to_string_properties() {
    rounding_mode_gen().test_properties(|rm| {
        let s = rm.to_string();
        assert_eq!(RoundingMode::from_str(&s), Ok(rm));
        assert!(string_is_subset(&s, ROUNDING_MODE_CHARS));
    });
}
