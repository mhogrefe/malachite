// Copyright © 2025 William Youmans
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::factorization::traits::{ExpressAsPower, IsPower};
use malachite_nz::integer::Integer;

#[test]
fn test_integer_express_as_power_positive() {
    // Positive integers should behave like naturals
    assert_eq!(
        Integer::from(8).express_as_power(),
        Some((Integer::from(2), 3))
    );
    assert_eq!(
        Integer::from(16).express_as_power(),
        Some((Integer::from(2), 4))
    );
    assert_eq!(
        Integer::from(27).express_as_power(),
        Some((Integer::from(3), 3))
    );
    assert_eq!(Integer::from(6).express_as_power(), None);
}

#[test]
fn test_integer_express_as_power_negative() {
    // Negative perfect powers (odd exponents only)
    assert_eq!(
        Integer::from(-8).express_as_power(),
        Some((Integer::from(-2), 3))
    );
    assert_eq!(
        Integer::from(-27).express_as_power(),
        Some((Integer::from(-3), 3))
    );
    assert_eq!(
        Integer::from(-125).express_as_power(),
        Some((Integer::from(-5), 3))
    );
    assert_eq!(
        Integer::from(-1000).express_as_power(),
        Some((Integer::from(-10), 3))
    );

    // Negative numbers that are perfect even powers should be rejected (because you can't have an
    // even root of a negative number)
    assert_eq!(Integer::from(-4).express_as_power(), None);
    assert_eq!(Integer::from(-16).express_as_power(), None);
    assert_eq!(Integer::from(-100).express_as_power(), None);

    // Negative non-powers
    assert_eq!(Integer::from(-2).express_as_power(), None);
    assert_eq!(Integer::from(-6).express_as_power(), None);
    assert_eq!(Integer::from(-10).express_as_power(), None);
}

#[test]
fn test_integer_express_as_power_special() {
    // Zero and ±1
    assert_eq!(Integer::ZERO.express_as_power(), Some((Integer::ZERO, 2)));
    // 1 is a perfect square (1 = 1^2)
    assert_eq!(Integer::ONE.express_as_power(), Some((Integer::ONE, 2)));
    // -1 is not a perfect power (no even roots of negative numbers)
    assert_eq!(Integer::from(-1).express_as_power(), None);
}

#[test]
fn test_integer_express_as_power_highest_exponent() {
    // Test that we get the highest possible exponent
    //
    // 64 = 2^6 (not 8^2 or 4^3)
    assert_eq!(
        Integer::from(64).express_as_power(),
        Some((Integer::from(2), 6))
    );

    // 4096 = 2^12 (not 64^2 or 16^3 or 8^4)
    assert_eq!(
        Integer::from(4096).express_as_power(),
        Some((Integer::from(2), 12))
    );
}

#[test]
fn test_integer_is_power_basic() {
    // Zero and 1 are perfect powers, but -1 is not
    assert_eq!(Integer::ZERO.is_power(), true);
    assert_eq!(Integer::ONE.is_power(), true);
    assert_eq!(Integer::from(-1).is_power(), false);

    // Positive perfect powers
    assert_eq!(Integer::from(4).is_power(), true);
    assert_eq!(Integer::from(8).is_power(), true);
    assert_eq!(Integer::from(16).is_power(), true);
    assert_eq!(Integer::from(27).is_power(), true);

    // Negative perfect powers (odd exponents only)
    assert_eq!(Integer::from(-8).is_power(), true);
    assert_eq!(Integer::from(-27).is_power(), true);
    assert_eq!(Integer::from(-125).is_power(), true);
    assert_eq!(Integer::from(-1000).is_power(), true);

    // Negative numbers that would be perfect even powers
    assert_eq!(Integer::from(-4).is_power(), false);
    assert_eq!(Integer::from(-16).is_power(), false);
    assert_eq!(Integer::from(-100).is_power(), false);

    // Non-perfect powers
    assert_eq!(Integer::from(2).is_power(), false);
    assert_eq!(Integer::from(3).is_power(), false);
    assert_eq!(Integer::from(6).is_power(), false);
    assert_eq!(Integer::from(-2).is_power(), false);
    assert_eq!(Integer::from(-6).is_power(), false);
}

#[test]
fn test_integer_is_power_consistency() {
    // For all numbers that express_as_power returns Some, is_power should return true
    for i in -100i32..=100 {
        let n = Integer::from(i);
        let has_power = n.express_as_power().is_some();
        assert_eq!(n.is_power(), has_power, "Failed for Integer {i}");
    }
}
