// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_nz::mag::exhaustive::exhaustive_positive_mags;

#[test]
fn test_exhaustive_positive_mags() {
    assert_eq!(
        exhaustive_positive_mags()
            .take(20)
            .map(|x| x.to_string())
            .collect_vec(),
        &[
            "1*2^-1", "1", "3*2^-2", "3*2^-1", "1*2^-2", "1*2^1", "3*2^-3", "3", "5*2^-3",
            "5*2^-2", "7*2^-3", "7*2^-2", "5*2^-4", "5*2^-1", "7*2^-4", "7*2^-1", "1*2^-3",
            "1*2^2", "3*2^-4", "3*2^1"
        ]
    );
}
