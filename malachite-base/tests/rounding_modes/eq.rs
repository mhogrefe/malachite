// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::common::test_eq_helper;
use malachite_base::test_util::generators::{
    rounding_mode_gen, rounding_mode_pair_gen, rounding_mode_triple_gen,
};

#[test]
fn test_eq() {
    test_eq_helper::<RoundingMode>(&["Down", "Up", "Floor", "Ceiling", "Nearest", "Exact"]);
}

#[test]
#[allow(clippy::eq_op)]
fn eq_properties() {
    rounding_mode_pair_gen().test_properties(|(x, y)| {
        assert_eq!(x == y, y == x);
    });

    rounding_mode_gen().test_properties(|rm| {
        assert_eq!(rm, rm);
    });

    rounding_mode_triple_gen().test_properties(|(x, y, z)| {
        if x == y && x == z {
            assert_eq!(x, z);
        }
    });
}
