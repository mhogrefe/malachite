// Copyright © 2025 William Youmans
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::factorization::traits::{ExpressAsPower, IsPower};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use std::str::FromStr;

#[test]
fn test_is_power_small() {
    // Test small perfect powers
    assert!(Natural::from(0u32).is_power());
    assert!(Natural::from(1u32).is_power());
    assert!(Natural::from(4u32).is_power());
    assert!(Natural::from(8u32).is_power());
    assert!(Natural::from(9u32).is_power());
    assert!(Natural::from(16u32).is_power());
    assert!(Natural::from(25u32).is_power());
    assert!(Natural::from(27u32).is_power());
    assert!(Natural::from(32u32).is_power());
    assert!(Natural::from(64u32).is_power());
    assert!(Natural::from(81u32).is_power());
    assert!(Natural::from(100u32).is_power());
    assert!(Natural::from(125u32).is_power());
    assert!(Natural::from(243u32).is_power());
    assert!(Natural::from(1024u32).is_power());
    assert!(Natural::from(1296u32).is_power());

    // Test small non-powers
    assert!(!Natural::from(2u32).is_power());
    assert!(!Natural::from(3u32).is_power());
    assert!(!Natural::from(5u32).is_power());
    assert!(!Natural::from(6u32).is_power());
    assert!(!Natural::from(7u32).is_power());
    assert!(!Natural::from(10u32).is_power());
    assert!(!Natural::from(12u32).is_power());
    assert!(!Natural::from(15u32).is_power());
}

#[test]
fn test_is_power_edge_cases() {
    // Test some specific edge cases

    // Powers of 2 that are perfect powers
    let power_of_2_power: Natural = Natural::from(1u64) << 0x1000;
    assert!(power_of_2_power.is_power());

    let power_of_2_non_power = power_of_2_power + Natural::from(1u64);
    assert!(!power_of_2_non_power.is_power());

    // Large powers
    let big_base = Natural::from_str("987654321098765432109876543210").unwrap();
    let big_power = (&big_base).pow(3);
    assert!(big_power.is_power());

    let big_non_power = &big_power + Natural::from(1u32);
    assert!(!big_non_power.is_power());

    // Test prime 1009 (SMALLEST_OMITTED_PRIME) - ensures termination
    assert!(!Natural::from(1009u32).is_power());
}

#[test]
fn is_power_properties() {
    natural_gen().test_properties(|x| {
        // Consistency: is_power() should match express_as_power()
        assert_eq!(
            x.is_power(),
            x.express_as_power().is_some(),
            "is_power() and express_as_power() inconsistent for {x}",
        );

        // Any number raised to a power >= 2 should be a perfect power
        if x > 1u32 {
            let power_2 = (&x).pow(2);
            assert!(power_2.is_power(), "{x}^2 should be a perfect power");

            let power_3 = (&x).pow(3);
            assert!(power_3.is_power(), "{x}^3 should be a perfect power");
        }
    });
}
