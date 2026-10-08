// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_vector::random::random_unsigned_vectors_with_dimension;

fn random_vectors_with_dimension_helper(dimension: u64, expected_values: &[&str]) {
    assert_eq!(
        random_unsigned_vectors_with_dimension::<u8>(EXAMPLE_SEED, dimension)
            .take(20)
            .map(|v| v.to_string())
            .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    random_vectors_with_dimension_helper(
        0,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1
    random_vectors_with_dimension_helper(
        1,
        &[
            "(113)", "(239)", "(69)", "(108)", "(228)", "(210)", "(168)", "(161)", "(87)", "(32)",
            "(110)", "(83)", "(188)", "(34)", "(89)", "(238)", "(93)", "(200)", "(149)", "(115)",
        ],
    );
    // dimension 3
    random_vectors_with_dimension_helper(
        3,
        &[
            "(113, 239, 69)",
            "(108, 228, 210)",
            "(168, 161, 87)",
            "(32, 110, 83)",
            "(188, 34, 89)",
            "(238, 93, 200)",
            "(149, 115, 189)",
            "(149, 217, 201)",
            "(117, 146, 31)",
            "(72, 151, 169)",
            "(174, 33, 7)",
            "(38, 81, 144)",
            "(72, 127, 113)",
            "(128, 233, 107)",
            "(46, 119, 12)",
            "(18, 164, 243)",
            "(114, 174, 59)",
            "(247, 39, 174)",
            "(160, 184, 104)",
            "(37, 100, 252)",
        ],
    );
}
