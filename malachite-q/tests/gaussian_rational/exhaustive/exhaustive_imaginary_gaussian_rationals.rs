// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_q::gaussian_rational::exhaustive::exhaustive_imaginary_gaussian_rationals;

#[test]
fn test_exhaustive_imaginary_gaussian_rationals() {
    assert_eq!(
        exhaustive_imaginary_gaussian_rationals()
            .take(20)
            .map(|x| x.to_string())
            .collect_vec(),
        &[
            "0", "i", "-i", "i/2", "-i/2", "2i", "-2i", "i/3", "-i/3", "3i/2", "-3i/2", "2i/3",
            "-2i/3", "3i", "-3i", "i/4", "-i/4", "4i/3", "-4i/3", "3i/5"
        ][..]
    );
    assert!(
        exhaustive_imaginary_gaussian_rationals()
            .take(100)
            .all(|x| x.real == 0)
    );
}
