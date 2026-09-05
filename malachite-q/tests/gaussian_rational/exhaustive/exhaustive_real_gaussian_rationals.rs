// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_q::gaussian_rational::exhaustive::exhaustive_real_gaussian_rationals;

#[test]
fn test_exhaustive_real_gaussian_rationals() {
    assert_eq!(
        exhaustive_real_gaussian_rationals()
            .take(20)
            .map(|x| x.to_string())
            .collect_vec(),
        &[
            "0", "1", "-1", "1/2", "-1/2", "2", "-2", "1/3", "-1/3", "3/2", "-3/2", "2/3", "-2/3",
            "3", "-3", "1/4", "-1/4", "4/3", "-4/3", "3/5"
        ][..]
    );
    assert!(
        exhaustive_real_gaussian_rationals()
            .take(100)
            .all(|x| x.imaginary == 0u32)
    );
}
