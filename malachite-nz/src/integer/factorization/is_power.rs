// Copyright © 2025 William Youmans
//
// Uses code adopted from the FLINT Library.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::factorization::traits::{ExpressAsPower, IsPower};
use malachite_base::num::logic::traits::BitScan;

fn express_as_power_integer(n: &Integer) -> Option<(Integer, u32)> {
    use malachite_base::num::basic::traits::{One, Zero};

    // Special case: zero is considered a perfect square (0 = 0^2)
    if n == &Integer::ZERO {
        return Some((Integer::ZERO, 2));
    }

    // Special case: 1 is a perfect square (1 = 1^2), but -1 is not
    if n.abs == Natural::ONE {
        return if n.sign {
            Some((Integer::ONE, 2))
        } else {
            None
        };
    }

    // For positive integers, use the Natural implementation directly
    if n.sign {
        return n.abs.express_as_power().map(|(base, exp)| {
            (
                Integer {
                    sign: true,
                    abs: base,
                },
                exp as u32,
            )
        });
    }

    // For negative integers: Early check to avoid wasted computation A negative integer can only be
    // an odd power. Quick check: if the number of factors of 2 is even, it cannot be an odd power
    // overall.
    if let Some(pow_2) = n.abs.index_of_next_true_bit(0)
        && pow_2 > 0
        && pow_2.even()
    {
        // Even number of factors of 2 - cannot be an odd power
        return None;
    }

    // For negative integers, only odd exponents are valid
    if let Some((base, exp)) = n.abs.express_as_power() {
        // Check if the exponent is odd
        if exp % 2 == 1 {
            return Some((
                Integer {
                    sign: false,
                    abs: base,
                },
                exp as u32,
            ));
        }
    }

    None
}

fn is_power_integer(n: &Integer) -> bool {
    use malachite_base::num::basic::traits::One;

    // 0 and 1 are perfect powers, but -1 is not
    if n.abs <= Natural::ONE {
        return n.sign; // n >= 0
    }

    // For positive integers, use the Natural implementation
    if n.sign {
        return n.abs.is_power();
    }

    // For negative integers: Early check to avoid wasted computation A negative integer can only be
    // an odd power. Quick check: if the number of factors of 2 is even, it cannot be an odd power
    // overall.
    if let Some(pow_2) = n.abs.index_of_next_true_bit(0)
        && pow_2 > 0
        && pow_2.even()
    {
        // Even number of factors of 2 - cannot be an odd power
        return false;
    }

    // For negative integers, check if it's an odd power
    if let Some((_, exp)) = n.abs.express_as_power() {
        return exp % 2 == 1;
    }

    false
}

impl ExpressAsPower for Integer {
    /// Expresses an [`Integer`] as a perfect power if possible.
    ///
    /// Returns `Some((root, exponent))` where `root^exponent = self` and `exponent > 1`, or `None`
    /// if the number cannot be expressed as a perfect power.
    ///
    /// For negative integers, only odd exponents are valid.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::ExpressAsPower;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(8).express_as_power(), Some((Integer::from(2), 3)));
    /// assert_eq!(Integer::from(-8).express_as_power(), Some((Integer::from(-2), 3)));
    /// assert_eq!(Integer::from(6).express_as_power(), None);
    /// ```
    fn express_as_power(&self) -> Option<(Self, u64)> {
        express_as_power_integer(self).map(|(root, exp)| (root, u64::from(exp)))
    }
}

impl IsPower for Integer {
    /// Determines whether an [`Integer`] is a perfect power.
    ///
    /// A perfect power is any number of the form $a^x$ where $x > 1$, with $a$ and $x$ both
    /// integers. In particular, 0 and 1 are considered perfect powers, but -1 is not (since it
    /// cannot be expressed as an integer to an integer power > 1).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::factorization::traits::IsPower;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(0).is_power(), true);
    /// assert_eq!(Integer::from(1).is_power(), true);
    /// assert_eq!(Integer::from(-1).is_power(), false);
    /// assert_eq!(Integer::from(4).is_power(), true);
    /// assert_eq!(Integer::from(-8).is_power(), true);
    /// assert_eq!(Integer::from(6).is_power(), false);
    /// ```
    fn is_power(&self) -> bool {
        is_power_integer(self)
    }
}
