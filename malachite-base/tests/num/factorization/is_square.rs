// Copyright © 2025 William Youmans
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL as published by the Free Software Foundation; either version
// 3 of the License, or (at your option any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::traits::IsSquare;
use malachite_base::num::random::random_unsigned_bit_chunks;
use malachite_base::random::EXAMPLE_SEED;

const NUM_TESTS: usize = 1000;

#[test]
fn test_is_square() {
    // Randomly generate integers in the range (a^2, (a+1)^2) which are guaranteed
    // to not be square.
    let iter_a = random_unsigned_bit_chunks::<u64>(EXAMPLE_SEED, 32).take(NUM_TESTS);
    let iter_b = random_unsigned_bit_chunks::<u64>(EXAMPLE_SEED, 32).take(NUM_TESTS);
    for (a, b) in iter_a.zip(iter_b) {
        let s = a * a + (b % (2 * a)) + 1;
        assert!(!s.is_square());
    }

    // Randomly generate squares
    let iter_a = random_unsigned_bit_chunks::<u64>(EXAMPLE_SEED, 32).take(NUM_TESTS);
    for a in iter_a {
        let s = a * a;
        assert!(s.is_square());
    }
}
