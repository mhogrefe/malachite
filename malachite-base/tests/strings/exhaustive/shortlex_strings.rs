// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::exhaustive::shortlex_strings;

#[test]
fn test_shortlex_strings() {
    let ss = shortlex_strings().take(20).collect_vec();
    assert_eq!(
        ss.iter().map(String::as_str).collect_vec().as_slice(),
        &[
            "", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
            "q", "r", "s",
        ]
    );
}
