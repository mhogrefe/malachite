// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_nz::gaussian_integer::exhaustive::exhaustive_real_gaussian_integers;

#[test]
fn test_exhaustive_real_gaussian_integers() {
    assert_eq!(
        exhaustive_real_gaussian_integers()
            .take(20)
            .map(|x| x.to_string())
            .collect_vec(),
        &[
            "0", "1", "-1", "2", "-2", "3", "-3", "4", "-4", "5", "-5", "6", "-6", "7", "-7", "8",
            "-8", "9", "-9", "10"
        ][..]
    );
    assert!(
        exhaustive_real_gaussian_integers()
            .take(100)
            .all(|x| x.imaginary == 0)
    );
}
