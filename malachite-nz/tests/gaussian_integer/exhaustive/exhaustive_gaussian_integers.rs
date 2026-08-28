// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_nz::gaussian_integer::exhaustive::exhaustive_gaussian_integers;

#[test]
fn test_exhaustive_gaussian_integers() {
    assert_eq!(
        exhaustive_gaussian_integers()
            .take(20)
            .map(|x| x.to_string())
            .collect_vec(),
        &[
            "0", "1", "i", "1+i", "-1", "2", "-1+i", "2+i", "-i", "1-i", "2i", "1+2i", "-1-i",
            "2-i", "-1+2i", "2+2i", "-2", "3", "-2+i", "3+i"
        ][..]
    );
    // no repetitions
    assert!(exhaustive_gaussian_integers().take(10000).all_unique());
}
