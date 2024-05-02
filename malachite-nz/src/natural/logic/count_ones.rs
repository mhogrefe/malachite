// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::logic::traits::CountOnes;

// Interpreting a slice of `Limb`s, as the limbs (in ascending order) of a `Natural`, counts the
// number of ones in the binary expansion of the `Natural`.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
pub_crate_test! {limbs_count_ones(xs: &[Limb]) -> u64 {
    xs.iter().map(|&x| CountOnes::count_ones(x)).sum()
}}

impl CountOnes for &Natural {
    /// Counts the number of ones in the binary expansion of a [`Natural`].
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
    /// use malachite_base::num::logic::traits::CountOnes;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.count_ones(), 0);
    /// // 105 = 1101001b
    /// assert_eq!(Natural::from(105u32).count_ones(), 4);
    /// // 10^12 = 1110100011010100101001010001000000000000b
    /// assert_eq!(Natural::from(10u32).pow(12).count_ones(), 13);
    /// ```
    fn count_ones(self) -> u64 {
        match *self {
            Natural(Small(small)) => CountOnes::count_ones(small),
            Natural(Large(ref limbs)) => limbs_count_ones(limbs),
        }
    }
}
