// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::unsigned_rational_sequence_gen;
use malachite_base::test_util::hash::hash;

#[test]
fn hash_properties() {
    unsigned_rational_sequence_gen::<u8>().test_properties(|xs| {
        assert_eq!(hash(&xs), hash(&xs.clone()));
    });
}
