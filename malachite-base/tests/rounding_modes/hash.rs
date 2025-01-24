// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::rounding_mode_gen;
use malachite_base::test_util::hash::hash;

#[test]
fn hash_properties() {
    rounding_mode_gen().test_properties(|rm| {
        assert_eq!(hash(&rm), hash(&rm.clone()));
    });
}
