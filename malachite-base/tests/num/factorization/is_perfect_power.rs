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

#[test]
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

#[test]
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

#[test]
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

#[test]
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

#[test]
fn test_non_perfect_powers() {
    let iter = random_unsigned_bit_chunks::<u64>(EXAMPLE_SEED, 64).take(NUM_TESTS);
    for d in iter {
        // naive perfect power testing by factoring
        if d.factor().into_iter().count() != 1 {
            assert!(d.is_perfect_power().is_none());
        }
    }
}

#[test]
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
fn test_signed() {
    assert_eq!(0i8.is_perfect_power().unwrap(), (0, 2));
    assert_eq!(1i16.is_perfect_power().unwrap(), (1, 2));
    assert_eq!(4i32.is_perfect_power().unwrap(), (2, 2));
    assert_eq!(8i64.is_perfect_power().unwrap(), (2, 3));

    // 64 = 2^6 = 4^3 but 3 is the largest odd exponent, so -64 = (-4)^3
    // where in the unsigned case we expect 2^6. etc.
    assert_eq!((-64i32).is_perfect_power().unwrap(), (-4, 3));
    assert_eq!((-4096i64).is_perfect_power().unwrap(), (-16, 3));
    assert_eq!((-3486784401i64).is_perfect_power().unwrap(), (-81, 5));
}
