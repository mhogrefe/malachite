// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_q::gaussian_rational::exhaustive::exhaustive_gaussian_rationals;

#[test]
fn test_exhaustive_gaussian_rationals() {
    assert_eq!(
        exhaustive_gaussian_rationals()
            .take(20)
            .map(|x| x.to_string())
            .collect_vec(),
        &[
            "0", "1", "i", "1+i", "-1", "1/2", "-1+i", "1/2+i", "-i", "1-i", "i/2", "1+i/2",
            "-1-i", "1/2-i", "-1+i/2", "1/2+i/2", "-1/2", "2", "-1/2+i", "2+i"
        ][..]
    );
    // no repetitions
    assert!(exhaustive_gaussian_rationals().take(10000).all_unique());
}
