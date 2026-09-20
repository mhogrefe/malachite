// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::vecs::exhaustive::restricted_growth_strings;

fn restricted_growth_strings_helper(len: usize, block_count: usize, out: &[&[usize]]) {
    let xss = restricted_growth_strings(len, block_count).collect_vec();
    assert_eq!(xss.iter().map(Vec::as_slice).collect_vec().as_slice(), out);
}

#[test]
fn test_restricted_growth_strings() {
    restricted_growth_strings_helper(0, 0, &[&[]]);
    restricted_growth_strings_helper(0, 1, &[]);
    restricted_growth_strings_helper(1, 0, &[]);
    restricted_growth_strings_helper(1, 1, &[&[0]]);
    restricted_growth_strings_helper(3, 2, &[&[0, 0, 1], &[0, 1, 0], &[0, 1, 1]]);
    restricted_growth_strings_helper(
        4,
        2,
        &[
            &[0, 0, 0, 1],
            &[0, 0, 1, 0],
            &[0, 0, 1, 1],
            &[0, 1, 0, 0],
            &[0, 1, 0, 1],
            &[0, 1, 1, 0],
            &[0, 1, 1, 1],
        ],
    );
    restricted_growth_strings_helper(
        4,
        3,
        &[&[0, 0, 1, 2], &[0, 1, 0, 2], &[0, 1, 1, 2], &[0, 1, 2, 0], &[0, 1, 2, 1], &[0, 1, 2, 2]],
    );
    restricted_growth_strings_helper(4, 4, &[&[0, 1, 2, 3]]);
    restricted_growth_strings_helper(4, 5, &[]);
    restricted_growth_strings_helper(
        5,
        2,
        &[
            &[0, 0, 0, 0, 1],
            &[0, 0, 0, 1, 0],
            &[0, 0, 0, 1, 1],
            &[0, 0, 1, 0, 0],
            &[0, 0, 1, 0, 1],
            &[0, 0, 1, 1, 0],
            &[0, 0, 1, 1, 1],
            &[0, 1, 0, 0, 0],
            &[0, 1, 0, 0, 1],
            &[0, 1, 0, 1, 0],
            &[0, 1, 0, 1, 1],
            &[0, 1, 1, 0, 0],
            &[0, 1, 1, 0, 1],
            &[0, 1, 1, 1, 0],
            &[0, 1, 1, 1, 1],
        ],
    );
}
