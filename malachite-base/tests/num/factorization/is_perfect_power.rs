// Copyright © 2025 William Youmans
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::CheckedSquare;
use malachite_base::num::factorization::traits::{Factor, IsPerfectPower};
use malachite_base::num::random::random_unsigned_bit_chunks;
use malachite_base::random::EXAMPLE_SEED;

const NUM_TESTS: usize = 1000;
const BITS: u64 = 64;

fn test_perfect_squares() {
    // Test that squares pass the test
    let iter = random_unsigned_bit_chunks::<u64>(EXAMPLE_SEED, BITS / 2).take(NUM_TESTS);
    for d in iter {
        if let Some(d_squared) = d.checked_square() {
            let res = d_squared.is_perfect_power();
            assert!(res.is_some());

            let (base, exp) = res.unwrap();
            assert_eq!(base.pow(exp), d_squared);
        }
    }
}

fn test_perfect_cubes() {
    let iter = random_unsigned_bit_chunks::<u32>(EXAMPLE_SEED, BITS / 3).take(NUM_TESTS);
    for d in iter {
        if let Some(d_pow) = d.checked_pow(3) {
            let res = d_pow.is_perfect_power();
            assert!(res.is_some());

            let (base, exp) = res.unwrap();
            assert_eq!(base.pow(exp), d_pow);
        }
    }
}

fn test_perfect_fifth_powers() {
    let iter = random_unsigned_bit_chunks::<u32>(EXAMPLE_SEED, BITS / 5).take(NUM_TESTS);
    for d in iter {
        if let Some(d_pow) = d.checked_pow(5) {
            let res = d_pow.is_perfect_power();
            assert!(res.is_some());

            let (base, exp) = res.unwrap();
            assert_eq!(base.pow(exp), d_pow);
        }
    }
}

fn test_exhaustive_other_powers() {
    // Exhaustively test all other powers
    // This tests all bases from 2 up to 2^(WORD_BITS/5) and all their powers
    let max_base = 1u64 << (64 / 5); // Limit to prevent excessive test time

    for d in 2..max_base {
        let mut n = d * d; // Start with d^2

        // Keep multiplying by d until we overflow
        loop {
            let result = n.is_perfect_power();

            if let Some((base, exp)) = result {
                assert_eq!(base.pow(exp), n);
            }

            // Try to multiply by d, break if overflow
            match n.checked_mul(d) {
                Some(next_n) => n = next_n,
                None => break, // Overflow occurred
            }
        }
    }
}

fn test_non_perfect_powers() {
    let iter = random_unsigned_bit_chunks::<u64>(EXAMPLE_SEED, 64).take(NUM_TESTS);
    for d in iter {
        // naive perfect power testing by factoring
        if d.factor().into_iter().count() != 1 {
            assert!(d.is_perfect_power().is_none());
        }
    }
}

fn test_edge_cases() {
    // non-perfect powers
    let non_pows: [u64; 5] = [2, 3, 6, 11, 15];
    for x in non_pows {
        assert_eq!(x.is_perfect_power(), None);
    }

    let pows: [u64; 12] = [0, 1, 4, 8, 9, 16, 25, 32, 64, 81, 100, 128];
    for x in pows {
        let (base, exp) = x.is_perfect_power().unwrap();
        assert_eq!(x, base.pow(exp));
    }
}

#[test]
fn test_is_perfect_power() {
    test_perfect_squares();
    test_perfect_cubes();
    test_perfect_fifth_powers();
    test_exhaustive_other_powers();
    test_non_perfect_powers();
    test_edge_cases();
}
