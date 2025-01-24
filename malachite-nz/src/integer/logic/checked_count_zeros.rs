// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::logic::traits::{CountOnes, CountZeros};

// Interpreting a slice of `Limb`s, as the limbs (in ascending order) of a `Natural`, counts the
// number of zeros in the binary expansion of the negative (two's complement) of the `Natural`.
// `limbs` cannot be empty.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_crate_test! {limbs_count_zeros_neg(xs: &[Limb]) -> u64 {
    let mut sum = 0;
    let mut nonzero_seen = false;
    for &x in xs {
        sum += if nonzero_seen {
            CountOnes::count_ones(x)
        } else if x == 0 {
            Limb::WIDTH
        } else {
            nonzero_seen = true;
            CountZeros::count_zeros(x.wrapping_neg())
        };
    }
    sum
}}

impl Natural {
    fn count_zeros_neg(&self) -> u64 {
        match *self {
            Natural(Small(small)) => CountZeros::count_zeros(small.wrapping_neg()),
            Natural(Large(ref limbs)) => limbs_count_zeros_neg(limbs),
        }
    }
}

impl Integer {
    /// Counts the number of zeros in the binary expansion of an [`Integer`]. If the [`Integer`] is
    /// non-negative, then the number of zeros is infinite, so `None` is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.checked_count_zeros(), None);
    /// // -105 = 10010111 in two's complement
    /// assert_eq!(Integer::from(-105).checked_count_zeros(), Some(3));
    /// assert_eq!(Integer::from(105).checked_count_zeros(), None);
    /// // -10^12 = 10001011100101011010110101111000000000000 in two's complement
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).checked_count_zeros(),
    ///     Some(24)
    /// );
    /// ```
    pub fn checked_count_zeros(&self) -> Option<u64> {
        if self.sign {
            None
        } else {
            Some(self.abs.count_zeros_neg())
        }
    }
}
