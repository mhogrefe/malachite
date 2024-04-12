// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::vecs::exhaustive::exhaustive_combined_k_compositions;

fn exhaustive_combined_k_compositions_helper(
    n_min: usize,
    n_max: usize,
    k: usize,
    out: &[&[usize]],
) {
    let xss = exhaustive_combined_k_compositions(n_min, n_max, k).collect_vec();
    assert_eq!(xss.iter().map(Vec::as_slice).collect_vec().as_slice(), out);
}

#[test]
fn test_exhaustive_combined_k_compositions() {
    exhaustive_combined_k_compositions_helper(
        3,
        5,
        3,
        &[
            &[1, 1, 1],
            &[1, 1, 2],
            &[1, 2, 1],
            &[2, 1, 1],
            &[1, 1, 3],
            &[1, 2, 2],
            &[1, 3, 1],
            &[2, 1, 2],
            &[2, 2, 1],
            &[3, 1, 1],
        ],
    );
    exhaustive_combined_k_compositions_helper(
        6,
        8,
        5,
        &[
            &[1, 1, 1, 1, 2],
            &[1, 1, 1, 1, 3],
            &[1, 1, 1, 2, 1],
            &[1, 1, 1, 1, 4],
            &[1, 1, 2, 1, 1],
            &[1, 1, 1, 2, 2],
            &[1, 2, 1, 1, 1],
            &[2, 1, 1, 1, 1],
            &[1, 1, 1, 3, 1],
            &[1, 1, 1, 2, 3],
            &[1, 1, 2, 1, 2],
            &[1, 1, 2, 2, 1],
            &[1, 1, 3, 1, 1],
            &[1, 1, 1, 3, 2],
            &[1, 2, 1, 1, 2],
            &[1, 2, 1, 2, 1],
            &[1, 2, 2, 1, 1],
            &[1, 1, 1, 4, 1],
            &[1, 3, 1, 1, 1],
            &[2, 1, 1, 1, 2],
            &[2, 1, 1, 2, 1],
            &[1, 1, 2, 1, 3],
            &[2, 1, 2, 1, 1],
            &[1, 1, 2, 2, 2],
            &[2, 2, 1, 1, 1],
            &[1, 1, 2, 3, 1],
            &[3, 1, 1, 1, 1],
            &[1, 1, 3, 1, 2],
            &[1, 1, 3, 2, 1],
            &[1, 1, 4, 1, 1],
            &[1, 2, 1, 1, 3],
            &[1, 2, 1, 2, 2],
            &[1, 2, 1, 3, 1],
            &[1, 2, 2, 1, 2],
            &[1, 2, 2, 2, 1],
            &[1, 2, 3, 1, 1],
            &[1, 3, 1, 1, 2],
            &[1, 3, 1, 2, 1],
            &[1, 3, 2, 1, 1],
            &[1, 4, 1, 1, 1],
            &[2, 1, 1, 1, 3],
            &[2, 1, 1, 2, 2],
            &[2, 1, 1, 3, 1],
            &[2, 1, 2, 1, 2],
            &[2, 1, 2, 2, 1],
            &[2, 1, 3, 1, 1],
            &[2, 2, 1, 1, 2],
            &[2, 2, 1, 2, 1],
            &[2, 2, 2, 1, 1],
            &[2, 3, 1, 1, 1],
            &[3, 1, 1, 1, 2],
            &[3, 1, 1, 2, 1],
            &[3, 1, 2, 1, 1],
            &[3, 2, 1, 1, 1],
            &[4, 1, 1, 1, 1],
        ],
    );
}

#[test]
#[should_panic]
fn exhaustive_combined_k_compositions_fail() {
    exhaustive_combined_k_compositions(2, 1, 3);
}
