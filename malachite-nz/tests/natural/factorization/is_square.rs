// Copyright © 2025 William Youmans
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Square;
use malachite_base::num::factorization::traits::IsSquare;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_pair_gen_var_3;
use std::str::FromStr;

#[test]
fn test_is_square_small() {
    // Test small perfect squares
    assert!(Natural::from(0u32).is_square());
    assert!(Natural::from(1u32).is_square());
    assert!(Natural::from(4u32).is_square());
    assert!(Natural::from(9u32).is_square());
    assert!(Natural::from(16u32).is_square());
    assert!(Natural::from(25u32).is_square());
    assert!(Natural::from(36u32).is_square());
    assert!(Natural::from(49u32).is_square());
    assert!(Natural::from(64u32).is_square());
    assert!(Natural::from(81u32).is_square());
    assert!(Natural::from(100u32).is_square());

    // Test small non-squares
    assert!(!Natural::from(2u32).is_square());
    assert!(!Natural::from(3u32).is_square());
    assert!(!Natural::from(5u32).is_square());
    assert!(!Natural::from(6u32).is_square());
    assert!(!Natural::from(7u32).is_square());
    assert!(!Natural::from(8u32).is_square());
    assert!(!Natural::from(10u32).is_square());
    assert!(!Natural::from(11u32).is_square());
    assert!(!Natural::from(12u32).is_square());
    assert!(!Natural::from(13u32).is_square());
    assert!(!Natural::from(14u32).is_square());
    assert!(!Natural::from(15u32).is_square());
}

#[test]
fn test_is_square_edge_cases() {
    // Test some specific edge cases

    // Powers of 2 that are perfect squares
    let power_of_2_square: Natural = Natural::from(1u64) << 0x1000;
    assert!(power_of_2_square.is_square());

    let power_of_2_non_square = power_of_2_square + Natural::from(1u64);
    assert!(!power_of_2_non_square.is_square());

    // Large squares
    let big_base = Natural::from_str("987654321098765432109876543210").unwrap();
    let big_square = big_base.square();
    assert!(big_square.is_square());

    let big_non_square = &big_square + Natural::from(1u32);
    assert!(!big_non_square.is_square());
}

#[test]
fn is_square_properties() {
    natural_pair_gen_var_3().test_properties(|(a, b)| {
        let sq = a.clone().square();
        assert!(sq.is_square());

        // test non-square in range (a^2, (a+1)^2)
        let non_sq = sq + (b % (Natural::from(2u64) * a)) + Natural::from(1u64);
        assert!(!non_sq.is_square());
    });
}
