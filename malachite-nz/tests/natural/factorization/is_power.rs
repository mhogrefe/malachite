// Copyright © 2025 William Youmans
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::factorization::traits::{ExpressAsPower, IsPower};
use malachite_nz::natural::Natural;

#[test]
fn test_natural_express_as_power_basic() {
    // Zero case
    assert_eq!(Natural::ZERO.express_as_power(), Some((Natural::ZERO, 2)));

    // One is a perfect square (1 = 1^2)
    assert_eq!(Natural::ONE.express_as_power(), Some((Natural::ONE, 2)));

    // Small perfect powers
    assert_eq!(
        Natural::from(4u32).express_as_power(),
        Some((Natural::from(2u32), 2))
    );
    assert_eq!(
        Natural::from(8u32).express_as_power(),
        Some((Natural::from(2u32), 3))
    );
    assert_eq!(
        Natural::from(9u32).express_as_power(),
        Some((Natural::from(3u32), 2))
    );
    assert_eq!(
        Natural::from(16u32).express_as_power(),
        Some((Natural::from(2u32), 4))
    );
    assert_eq!(
        Natural::from(27u32).express_as_power(),
        Some((Natural::from(3u32), 3))
    );
    assert_eq!(
        Natural::from(32u32).express_as_power(),
        Some((Natural::from(2u32), 5))
    );
    assert_eq!(
        Natural::from(64u32).express_as_power(),
        Some((Natural::from(2u32), 6))
    );
    assert_eq!(
        Natural::from(81u32).express_as_power(),
        Some((Natural::from(3u32), 4))
    );
    assert_eq!(
        Natural::from(100u32).express_as_power(),
        Some((Natural::from(10u32), 2))
    );
    assert_eq!(
        Natural::from(125u32).express_as_power(),
        Some((Natural::from(5u32), 3))
    );
    assert_eq!(
        Natural::from(128u32).express_as_power(),
        Some((Natural::from(2u32), 7))
    );
}

#[test]
fn test_natural_express_as_power_non_powers() {
    // Non-perfect powers
    assert_eq!(Natural::from(2u32).express_as_power(), None);
    assert_eq!(Natural::from(3u32).express_as_power(), None);
    assert_eq!(Natural::from(5u32).express_as_power(), None);
    assert_eq!(Natural::from(6u32).express_as_power(), None);
    assert_eq!(Natural::from(7u32).express_as_power(), None);
    assert_eq!(Natural::from(10u32).express_as_power(), None);
    assert_eq!(Natural::from(12u32).express_as_power(), None);
    assert_eq!(Natural::from(15u32).express_as_power(), None);
    assert_eq!(Natural::from(18u32).express_as_power(), None);
    assert_eq!(Natural::from(20u32).express_as_power(), None);
}

#[test]
fn test_natural_express_as_power_large() {
    // Larger perfect powers
    assert_eq!(
        Natural::from(1024u32).express_as_power(),
        Some((Natural::from(2u32), 10))
    );
    assert_eq!(
        Natural::from(4096u32).express_as_power(),
        Some((Natural::from(2u32), 12))
    );
    // 10000 = 100^2 = 10^4, but 100 = 10^2, so recursively we get 10^4
    assert_eq!(
        Natural::from(10000u32).express_as_power(),
        Some((Natural::from(10u32), 4))
    );

    // Test with very large numbers
    let large_base = Natural::from(12345u32);
    let large_power = &large_base * &large_base * &large_base; // 12345^3
    assert_eq!(large_power.express_as_power(), Some((large_base, 3)));
}

#[test]
fn test_natural_express_as_power_edge_cases() {
    // Powers where 2 divides exactly once (should reject)
    assert_eq!(Natural::from(2u32).express_as_power(), None);
    assert_eq!(Natural::from(6u32).express_as_power(), None);
    assert_eq!(Natural::from(10u32).express_as_power(), None);
    assert_eq!(Natural::from(14u32).express_as_power(), None);

    // 243 = 3^5
    assert_eq!(
        Natural::from(243u32).express_as_power(),
        Some((Natural::from(3u32), 5))
    );

    // 1296 = 6^4 = (2·3)^4 = 2^4 · 3^4
    assert_eq!(
        Natural::from(1296u32).express_as_power(),
        Some((Natural::from(6u32), 4))
    );
}

#[test]
fn test_natural_express_as_power_highest_exponent() {
    // Test that we get the highest possible exponent 64 = 2^6 (not 8^2 or 4^3)
    assert_eq!(
        Natural::from(64u32).express_as_power(),
        Some((Natural::from(2u32), 6))
    );

    // 4096 = 2^12 (not 64^2 or 16^3 or 8^4)
    assert_eq!(
        Natural::from(4096u32).express_as_power(),
        Some((Natural::from(2u32), 12))
    );
}

#[test]
fn test_natural_is_power_basic() {
    // Zero and one are perfect powers
    assert_eq!(Natural::ZERO.is_power(), true);
    assert_eq!(Natural::ONE.is_power(), true);

    // Perfect powers
    assert_eq!(Natural::from(4u32).is_power(), true);
    assert_eq!(Natural::from(8u32).is_power(), true);
    assert_eq!(Natural::from(9u32).is_power(), true);
    assert_eq!(Natural::from(16u32).is_power(), true);
    assert_eq!(Natural::from(27u32).is_power(), true);
    assert_eq!(Natural::from(32u32).is_power(), true);
    assert_eq!(Natural::from(64u32).is_power(), true);
    assert_eq!(Natural::from(81u32).is_power(), true);
    assert_eq!(Natural::from(100u32).is_power(), true);
    assert_eq!(Natural::from(125u32).is_power(), true);
    assert_eq!(Natural::from(128u32).is_power(), true);
    assert_eq!(Natural::from(243u32).is_power(), true);
    assert_eq!(Natural::from(1024u32).is_power(), true);
    assert_eq!(Natural::from(1296u32).is_power(), true);

    // Non-perfect powers
    assert_eq!(Natural::from(2u32).is_power(), false);
    assert_eq!(Natural::from(3u32).is_power(), false);
    assert_eq!(Natural::from(5u32).is_power(), false);
    assert_eq!(Natural::from(6u32).is_power(), false);
    assert_eq!(Natural::from(7u32).is_power(), false);
    assert_eq!(Natural::from(10u32).is_power(), false);
    assert_eq!(Natural::from(12u32).is_power(), false);
    assert_eq!(Natural::from(15u32).is_power(), false);
    assert_eq!(Natural::from(18u32).is_power(), false);
    assert_eq!(Natural::from(20u32).is_power(), false);
}

#[test]
fn test_natural_is_power_consistency() {
    // For all numbers that express_as_power returns Some, is_power should return true
    for i in 0u32..=200 {
        let n = Natural::from(i);
        let has_power = n.express_as_power().is_some();
        assert_eq!(n.is_power(), has_power, "Failed for Natural {i}");
    }
}
