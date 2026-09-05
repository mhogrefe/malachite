// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_nz::gaussian_integer::exhaustive::exhaustive_imaginary_gaussian_integers;

#[test]
fn test_exhaustive_imaginary_gaussian_integers() {
    assert_eq!(
        exhaustive_imaginary_gaussian_integers()
            .take(20)
            .map(|x| x.to_string())
            .collect_vec(),
        &[
            "0", "i", "-i", "2i", "-2i", "3i", "-3i", "4i", "-4i", "5i", "-5i", "6i", "-6i", "7i",
            "-7i", "8i", "-8i", "9i", "-9i", "10i"
        ][..]
    );
    assert!(
        exhaustive_imaginary_gaussian_integers()
            .take(100)
            .all(|x| x.real == 0u32)
    );
}
