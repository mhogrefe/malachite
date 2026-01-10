// Copyright © 2026 William Youmans
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedSqrt, Square};
use malachite_base::num::factorization::traits::IsSquare;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{natural_gen, natural_pair_gen_var_3};
use std::str::FromStr;

#[test]
fn test_is_square() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().is_square(), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().is_perfect_square(), out);
    };
    // Test small perfect squares
    test("0", true);
    test("1", true);
    test("4", true);
    test("9", true);
    test("16", true);
    test("25", true);
    test("36", true);
    test("49", true);
    test("64", true);
    test("81", true);
    test("100", true);

    // Test small non-squares
    test("2", false);
    test("3", false);
    test("5", false);
    test("6", false);
    test("7", false);
    test("8", false);
    test("10", false);
    test("11", false);
    test("12", false);
    test("13", false);
    test("14", false);
    test("15", false);
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
    natural_gen().test_properties(|x| {
        let is_square = x.is_square();
        assert_eq!(is_square, (&x).checked_sqrt().is_some());
        assert!((&x).square().is_square());
        assert_eq!(rug::Integer::from(&x).is_perfect_square(), is_square);
    });

    natural_pair_gen_var_3().test_properties(|(a, b)| {
        let sq = (&a).square();
        // test non-square in range (a^2, (a+1)^2)
        let non_sq = sq + (b % (Natural::from(2u64) * a)) + Natural::from(1u64);
        assert!(!non_sq.is_square());
    });
}
