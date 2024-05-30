// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::ToDebugString;
use malachite_q::exhaustive::exhaustive_rationals;

#[test]
fn test_exhaustive_rationals() {
    assert_eq!(
        exhaustive_rationals()
            .take(20)
            .collect_vec()
            .to_debug_string(),
        "[0, 1, -1, 1/2, -1/2, 2, -2, 1/3, -1/3, 3/2, -3/2, 2/3, -2/3, 3, -3, 1/4, -1/4, 4/3, \
        -4/3, 3/5]"
    );
}
